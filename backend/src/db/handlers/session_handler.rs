use chrono::{Duration, Local};
use surrealdb::Surreal;

use crate::db::interfaces::session_interface;
pub use crate::db::interfaces::session_interface::{create_session, delete_session, Session};
// This denotes how long a session will last
static SESSION_DURATION: i64 = 24 * 60 * 60;
use super::user_handler::User;
// pub async fn is_session_id_valid(db: &Surreal<Client>, session_id: &String) -> bool {
//     let session = get_session(session_id, db).await;
//     match session {
//         Some(_) => true,
//         None => false,
//     }
// }
pub async fn get_session(
    session_id: &str,
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
) -> Option<Session<User>> {
    let session = session_interface::get_session(session_id, db).await;
    // The session is invalid if older than the session_duration time
    if session.as_ref().is_some()
        && session.as_ref().unwrap().creation_date
            < Local::now() - Duration::seconds(SESSION_DURATION)
    {
        // Should delete the invalid session
        delete_session(session_id, db).await.unwrap_or_default();
        return None;
    }
    return session;
}
