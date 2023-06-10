use actix_web::{get, post, web, HttpRequest, Responder};
use google_oauth::AsyncClient;
use serde::{Deserialize, Serialize};
use surrealdb::Response;
// This is terrible structure: will be fixed in the future hopefully
use crate::{
    db::handlers::{user_handler::{self, Session, User}, session_handler},
    AppState,
};
// Stuctures
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
// Helpers
fn generate_picture_url(username: &str) -> String {
    let usr_str = username.replace(" ", "+");
    format!("https://ui-avatars.com/api/?name={usr_str}")
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
        user_handler::get_user(google_id.clone(), &shared_data.surreal.db).await;
    let value = match result {
        // Kind of an annoying hack
        Some(_) => 1,
        None => 0,
    };
    let user = if value == 0 {
        // User doesn't already have account
        let email_string = match email {
            Some(value) => value,
            None => {
                return Ok(web::Json(LoginResult {
                    session_id: None,
                    error: Some("This account has no email".to_string()),
                    user: None,
                }));
            }
        };
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
                email: email_string,
                picture: url,
                admin: false,
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
    let session_result =
        session_handler::create_session(&shared_data.surreal.db, &google_id, user_agent).await;
    let session = match session_result {
        Ok(session) => session,
        Err(err) => {
            return Ok(web::Json(LoginResult {
                session_id: None,
                error: Some(err.to_string()),
                user: None,
            }));
        }
    };
    Ok(web::Json(LoginResult {
        session_id: Some(session.session_id),
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
    session_handler::delete_session(session_id, &shared_data.surreal.db).await;
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
    let session = session_handler::get_session(session_id, &shared_data.surreal.db).await;
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
    let user_option: Option<User> = user_handler::get_user(user_id, &shared_data.surreal.db).await;
    let user = match user_option {
        Some(user) => user,
        None => {
            return Ok(web::Json(GetUserResult {
                success: false,
                error: Some("No user with this session".into()),
                user: None,
            }))
        }
    };
    Ok(web::Json(GetUserResult {
        error: None,
        success: true,
        user: Some(user),
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
    let session = match session_handler::get_session(session_id, &shared_data.surreal.db).await {
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
        let generated = generate_picture_url("Test User");
        let expected = "https://ui-avatars.com/api/?name=Test+User";
        assert_eq!(generated, expected)
    }
}
