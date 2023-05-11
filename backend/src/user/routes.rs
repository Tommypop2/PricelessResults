use actix_web::{get, post, web, HttpRequest, Responder};
use google_oauth::AsyncClient;
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::Response;
// This is terrible structure: will be fixed in the future hopefully
use crate::{user::db_handler, AppState, Record};
// Structures
#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: String,
    username: String,
    email: Option<String>,
    picture: String,
}
#[derive(Serialize, Deserialize)]
struct LoginInfo {
    id_token: String,
}
#[derive(Serialize, Deserialize)]
struct LoginResult {
    session_id: Option<String>,
    error: Option<String>,
    user: Option<User>,
}
#[derive(Serialize, Deserialize)]
struct Session {
    session_id: String,
    user_id: String,
    user_agent: Option<String>,
}
// Helpers
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
fn generate_picture_url(username: &str) -> String {
    let yes = username.replace(" ", "+");
    format!("https://ui-avatars.com/api/?name={yes}")
}
// Routes
#[post("/login")]
async fn login_route(
    shared_data: web::Data<AppState>,
    json: web::Json<LoginInfo>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let client = AsyncClient::new(&shared_data.oauth_clientid);
    let data_result = client.validate_id_token(&json.id_token).await;
    let user_agent_opt = req
        .headers()
        .get(actix_web::http::header::USER_AGENT)
        .unwrap();
    let user_agent = match user_agent_opt.to_str() {
        Ok(val) => Some(val.to_string()),
        Err(_) => None,
    };
    // let user_agent = user_agent_opt.unwrap().to_str().unwrap();
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
    let username = match data.name {
        Some(usrname) => usrname,
        None => "".into(),
    };
    let email_verified = match data.email_verified {
        Some(val) => val,
        None => false,
    };
    let email = match data.email {
        Some(email) => {
            if email_verified {
                Some(email)
            } else {
                None
            }
        }
        None => None,
    };

    let result: Option<User> =
        db_handler::get_user(google_id.clone(), &shared_data.surreal.db).await;
    let value = match result {
        // Kind of an annoying hack
        Some(_) => 1,
        None => 0,
    };
    let user = if value == 0 {
        // User doesn't already have account
        let url = match data.picture {
            Some(url) => url,
            None => generate_picture_url(&username),
        };
        let usr: User = shared_data
            .surreal
            .db
            .create("user")
            .content(User {
                user_id: google_id.clone(),
                username,
                email,
                picture: url,
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
            user_agent,
        })
        .await
        .unwrap();
    Ok(web::Json(LoginResult {
        session_id: Some(session_id),
        error: None,
        user,
    }))
}
#[derive(Deserialize)]
struct LogoutParams {
    session_id: String,
}
#[derive(Serialize)]
struct LogoutResult {
    error: Option<String>,
}
#[get("/logout")]
async fn logout_route(
    shared_data: web::Data<AppState>,
    query: web::Query<LogoutParams>,
) -> actix_web::Result<impl Responder> {
    let session_id = &query.session_id;
    // Don't need to validate, db just won't delete anything if session doesn't exist
    shared_data
        .surreal
        .db
        .query(format!(
            "DELETE session WHERE session_id = \"{}\"",
            session_id
        ))
        .await
        .unwrap();
    Ok(web::Json(LogoutResult { error: None }))
}
#[derive(Deserialize, Serialize)]
struct GetUserResult {
    success: bool,
    error: Option<String>,
    user: Option<User>,
}
#[derive(Deserialize)]
struct GetUserParams {
    session_id: String,
}
#[get("/user")]
async fn user_route(
    shared_data: web::Data<AppState>,
    query: web::Query<GetUserParams>,
) -> actix_web::Result<impl Responder> {
    // Having multiple queries here isn't great, but should be solved in the future with graph queries
    let session_id = &query.session_id;
    let session = db_handler::get_session(session_id, &shared_data.surreal.db).await;
    let user_id = match session {
        Some(session) => session.user_id,
        None => {
            return Ok(web::Json(GetUserResult {
                success: false,
                error: Some("No session with this id".into()),
                user: None,
            }))
        }
    };
    let user: Option<User> = db_handler::get_user(user_id, &shared_data.surreal.db).await;
    Ok(web::Json(GetUserResult {
        error: None,
        success: true,
        user,
    }))
}
#[derive(Serialize)]
struct UserSessionResult {
    sessions: Option<Vec<Session>>,
}
#[get("/sessions")]
async fn user_sessions(
    shared_data: web::Data<AppState>,
    query: web::Query<GetUserParams>,
) -> actix_web::Result<impl Responder> {
    let session_id = &query.session_id;
    let session = match db_handler::get_session(session_id, &shared_data.surreal.db).await {
        Some(val) => val,
        None => return Ok(web::Json(UserSessionResult { sessions: None })),
    };
    let user_id = session.user_id;
    let sessions: Vec<Session> = shared_data
        .surreal
        .db
        .query(format!(
            "SELECT * FROM session WHERE user_id = \"{user_id}\"",
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(web::Json(UserSessionResult {
        sessions: Some(sessions),
    }))
}
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_route)
        .service(logout_route)
        .service(user_route)
        .service(user_sessions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pic_url_test() {
        let generated = generate_picture_url("Thomas Beer");
        let expected = "https://ui-avatars.com/api/?name=Thomas+Beer";
        assert_eq!(generated, expected)
    }
    
    #[test]
    fn generate_random_string_test() {
        assert_eq!(generate_random_string(64).len(), 64)
    }
}
