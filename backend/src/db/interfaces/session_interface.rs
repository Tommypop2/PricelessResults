use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::{opt::RecordId, Surreal};

use super::user_interface::User;

#[derive(Serialize, Deserialize)]
pub struct Session<T = RecordId> {
    pub id: Option<RecordId>,
    pub user: T,
    pub user_agent: Option<String>,
    pub creation_date: DateTime<Local>,
}
impl<T> Session<T> {
    pub fn new(user: T, user_agent: Option<String>) -> Session<T> {
        Self {
            id: None,
            user,
            user_agent,
            creation_date: Local::now(),
        }
    }
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
) -> Option<Session<User>> {
    let session: Option<Session<User>> = db
        .query("SELECT *, user.* FROM $session_id")
        .bind((
            "session_id",
            RecordId {
                tb: "session".to_owned(),
                id: session_id.into(),
            },
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
) -> surrealdb::Result<Session> {
    let session_id = generate_random_string(64);
    let session = Session::new(
        RecordId {
            id: google_id.into(),
            tb: "user".to_string(),
        },
        user_agent,
    );
    let created: Session = db.create(("session", &session_id)).content(session).await?;
    Ok(created)
}
pub async fn delete_session(
    session_id: &str,
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
) -> surrealdb::Result<()> {
    // Don't need to validate, db just won't delete anything if session doesn't exist
    let _: Session = db.delete(("session", session_id)).await?;
    Ok(())
}
