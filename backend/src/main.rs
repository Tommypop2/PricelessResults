use actix_web::{
    cookie::Cookie,
    get,
    http::StatusCode,
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
mod db;
use actix_cors::Cors;
use db::surrealdb_connection::SurrealDBRepo;
use dotenv::dotenv;
use google_oauth::AsyncClient;
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
struct AppState {
    surreal: SurrealDBRepo,
    oauth_clientid: String,
    oauth_client_secret: String,
}
#[derive(Debug, Serialize)]
struct EpicThing<'a> {
    faxNoCap: bool,
    yes: &'a str,
}
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
fn generateRandomString(length: i32) -> String {
    let mut rng = rand::thread_rng();
    let mut result: String = "".into();
    let chars_len = CHARACTERS.len();
    for _ in 0..length {
        result.push(CHARACTERS.chars().nth(rng.gen_range(0..chars_len)).unwrap());
    }
    result
}
#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    println!("About to create");
    let created: Record = data
        .surreal
        .db
        .create("coolthing")
        .content(EpicThing {
            faxNoCap: true,
            yes: "asd",
        })
        .await
        .unwrap();
    dbg!(created);
    // println!("Done");
    // dbg!(created);
    // let app_name = &data.surreal.db.query("SELECT * FROM test").await;
    // dbg!(app_name);
    format!("Hello There") // <- response with app_name
}
#[derive(Serialize, Deserialize)]
struct LoginInfo {
    id_token: String,
}
#[post("/login")]
async fn login(data: web::Data<AppState>, json: web::Json<LoginInfo>) -> HttpResponse {
    let client = AsyncClient::new(&data.oauth_clientid);
    let data = client.validate_id_token(&json.id_token).await;
    if data.is_err() {
        return HttpResponse::Ok().body("Invalid Token");
    }
    println!("{}", generateRandomString(64));
    let id = data.unwrap().sub;
    println!("{}, {}", json.id_token, id);
    HttpResponse::Ok()
        .cookie(Cookie::new("token", &json.id_token))
        .body("yes")
    // return "".into();
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let client_id = std::env::var("clientId").expect("clientId variable should be set");
    let client_secret = std::env::var("clientSecret").expect("clientSecret variable should be set");
    let db_url = std::env::var("db_url").expect("db_url variable should be set");
    let surreal = SurrealDBRepo::init(&db_url)
        .await
        .expect("Error connecting to SurrealDB!");

    println!("Connected to db");
    let surreal_data = Data::new(AppState {
        surreal,
        oauth_clientid: client_id,
        oauth_client_secret: client_secret,
    });
    HttpServer::new(move || {
        // let cors = Cors::default().allowed_origin("http://localhost:5173");
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(surreal_data.clone())
            .service(index)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
