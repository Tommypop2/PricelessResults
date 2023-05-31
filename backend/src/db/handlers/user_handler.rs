use surrealdb::{engine::remote::ws::Client, Surreal};

pub use crate::db::interfaces::user_interface::*;

pub async fn is_session_id_valid(db: &Surreal<Client>, session_id: &String) -> bool {
    let session = get_session(session_id, db).await;
    match session {
        Some(_) => true,
        None => false,
    }
}
