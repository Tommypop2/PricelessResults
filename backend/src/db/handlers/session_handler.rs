use surrealdb::{Surreal, engine::remote::ws::Client};

pub use crate::db::interfaces::session_interface::*;

pub async fn is_session_id_valid(db: &Surreal<Client>, session_id: &String) -> bool {
    let session = get_session(session_id, db).await;
    match session {
        Some(_) => true,
        None => false,
    }
}
