use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::{Response, Surreal};
// Users and sessions are colocated, as that makes sense for this case, as sessions will only be used with users, and mostly vice versa
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
// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub picture: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub user_agent: Option<String>,
    pub creation_date: DateTime<Local>,
}
pub async fn get_user<T>(
    id: String,
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
) -> Option<T>
where
    for<'a> T: Deserialize<'a>,
{
    let mut user_response: Response = db
        .query(format!("SELECT * FROM user WHERE user_id = \"{}\"", id))
        .await
        .unwrap();
    user_response.take(0).unwrap()
}

pub async fn get_session(
    session_id: &str,
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
) -> Option<Session> {
    let session: Option<Session> = db
        .query(format!(
            "SELECT * FROM session WHERE session_id = \"{}\"",
            session_id
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap_or(None);
    session
}
pub async fn create_session(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    google_id: &String,
    user_agent: Option<String>,
) -> surrealdb::Result<Session> {
    let session_id = generate_random_string(64);
    let created: Session = db
        .create("session")
        .content(Session {
            session_id: session_id.clone(),
            user_id: google_id.to_string(),
            user_agent,
            creation_date: Local::now(),
        })
        .await?;
    Ok(created)
}
pub async fn delete_session(session_id: &str, db: &Surreal<surrealdb::engine::remote::ws::Client>) {
    // Don't need to validate, db just won't delete anything if session doesn't exist
    db.query(format!(
        "DELETE session WHERE session_id = \"{}\"",
        session_id
    ))
    .await
    .unwrap();
}