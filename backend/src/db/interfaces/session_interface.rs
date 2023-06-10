use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::{opt::RecordId, Surreal};

use super::user_interface::User;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user: RecordId,
    pub user_agent: Option<String>,
    pub creation_date: DateTime<Local>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionRecord<T> {
    // pub id: String,
    pub session_id: String,
    pub user: T,
    pub user_agent: Option<String>,
    pub creation_date: DateTime<Local>,
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
pub async fn get_session(
    session_id: &str,
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
) -> Option<SessionRecord<User>> {
    let session: Option<SessionRecord<User>> = db
        .query(format!(
            "SELECT *, user.* FROM session WHERE session_id = \"{}\"",
            session_id
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap_or(None);
    session
}
pub async fn create_session<'a>(
    db: &'a Surreal<surrealdb::engine::remote::ws::Client>,
    google_id: &'a String,
    user_agent: Option<String>,
) -> surrealdb::Result<SessionRecord<RecordId>> {
    let session_id = generate_random_string(64);
    let created: SessionRecord<RecordId> = db
        .create(("session", session_id.clone()))
        .content(Session {
            session_id: session_id.clone(),
            user: RecordId {
                id: google_id.into(),
                tb: "user".to_string(),
            },
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
