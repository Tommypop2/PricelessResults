use actix_web::{
    get,
    post,
    web::{self, Data},
    App, HttpServer, Responder,
};
mod db;
mod user;
use actix_cors::Cors;
use db::surrealdb_connection::SurrealDBRepo;
use dotenv::dotenv;
use google_oauth::AsyncClient;
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{Thing},
    Response,
};
use user::user::create_routes;
struct AppState {
    surreal: SurrealDBRepo,
    oauth_clientid: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Serialize, Deserialize)]
struct Session {
    session_id: String,
    user_id: String,
}
const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
fn generate_random_string(length: i32) -> String {
    let mut rng = rand::thread_rng();
    let mut result: String = "".into();
    let chars_len = CHARACTERS.len();
    for _ in 0..length {
        result.push(CHARACTERS.chars().nth(rng.gen_range(0..chars_len)).unwrap());
    }
    result
}
#[get("/")]
async fn index() -> String {
    format!("This is base route for the Priceless Results API")
}
#[derive(Serialize, Deserialize)]
struct LoginInfo {
    id_token: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: String,
    username: String,
    email: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct LoginResult {
    session_id: Option<String>,
    error: Option<String>,
    user: Option<User>,
}
#[post("/login")]
async fn login(
    shared_data: web::Data<AppState>,
    json: web::Json<LoginInfo>,
) -> actix_web::Result<impl Responder> {
    let client = AsyncClient::new(&shared_data.oauth_clientid);
    let data_result = client.validate_id_token(&json.id_token).await;
    let data = match data_result {
        Ok(res) => res,
        Err(_) => {
            return Ok(web::Json(LoginResult {
                session_id: None,
                error: Some("yes".into()),
                user: None,
            }))
        }
    };
    let google_id: String = data.sub;
    let username = data.name;

    let email: Option<String> = if data.email_verified {
        Some(data.email)
    } else {
        None
    };
    let result: Option<i32> = shared_data
        .surreal
        .db
        .query(format!(
            "SELECT count(user_id = {}) AS total FROM user;",
            google_id
        ))
        .await
        .unwrap()
        .take((0, "total"))
        .unwrap_or(Some(0));
    let value = match result {
        Some(val) => val,
        None => 0,
    };
    let user = if value == 0 {
        // User doesn't already have account
        let usr: User = shared_data
            .surreal
            .db
            .create("user")
            .content(User {
                user_id: google_id.clone(),
                username,
                email,
            })
            .await
            .unwrap();
        Some(usr)
    } else {
        let mut user_response: Response = shared_data
            .surreal
            .db
            .query(format!(
                "SELECT * FROM user WHERE user_id = \"{}\"",
                google_id
            ))
            .await
            .unwrap();
        let usr: Option<User> = user_response.take(0).unwrap();
        usr
    };
    let session_id = generate_random_string(64);
    let _created: Record = shared_data
        .surreal
        .db
        .create("session")
        .content(Session {
            session_id: session_id.clone(),
            user_id: google_id,
        })
        .await
        .unwrap();
    Ok(web::Json(LoginResult {
        session_id: Some(session_id),
        error: None,
        user: user,
    }))
}
#[derive(Deserialize)]
struct GetUserParams {
    session_id: String,
}
#[derive(Deserialize, Serialize)]
struct GetUserResult {
    success: bool,
    error: Option<String>,
    user: Option<User>,
}
#[get("/getuser")]
async fn get_user(
    shared_data: web::Data<AppState>,
    query: web::Query<GetUserParams>,
) -> actix_web::Result<impl Responder> {
    let session_id = &query.session_id;
    let user_id_option: Option<String> = shared_data
        .surreal
        .db
        .query(format!(
            "SELECT user_id FROM session WHERE session_id = \"{}\"",
            session_id
        ))
        .await
        .unwrap()
        .take((0, "user_id"))
        .unwrap();
    let user_id = match user_id_option {
        Some(id) => id,
        None => {
            return Ok(web::Json(GetUserResult {
                success: false,
                error: Some("No session with this id".into()),
                user: None,
            }))
        }
    };
    let mut user_response: Response = shared_data
        .surreal
        .db
        .query(format!(
            "SELECT * FROM user WHERE user_id = \"{}\"",
            user_id
        ))
        .await
        .unwrap();
    let user: Option<User> = user_response.take(0).unwrap();
    Ok(web::Json(GetUserResult {
        error: None,
        success: true,
        user,
    }))
}
#[derive(Serialize)]
struct TestRouteResult {}
#[get("/test_route")]
async fn test_route(_shared_data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(TestRouteResult {}))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let client_id = std::env::var("client_id").expect("client_id variable should be set");
    let db_url = std::env::var("db_url").expect("db_url variable should be set");
    let surreal = SurrealDBRepo::init(&db_url)
        .await
        .expect("Error connecting to SurrealDB!");

    println!("Connected to db");
    let surreal_data = Data::new(AppState {
        surreal,
        oauth_clientid: client_id,
    });
    HttpServer::new(move || {
        // let cors = Cors::default().allowed_origin("http://localhost:5173");
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(surreal_data.clone())
            .service(index)
            .service(login)
            .service(get_user)
            .service(test_route).service(web::scope("/epic").configure(create_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
