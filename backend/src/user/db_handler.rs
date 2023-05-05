use serde::{Deserialize, Serialize};
use surrealdb::{Response, Surreal};
// Users and sessions are colocated, as that makes sense for this case, as sessions will only be used with users, and mostly vice versa
// This uses generics, which is kinda awesome :)
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
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
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
