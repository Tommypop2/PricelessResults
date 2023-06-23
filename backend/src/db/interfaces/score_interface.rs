use std::future::IntoFuture;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize)]
pub struct Score<T = RecordId, U = RecordId> {
    pub id: Option<RecordId>,
    pub test: T,
    pub user: U,
    pub creation_date: DateTime<Local>,
}
impl Score {
    fn new(user_id: &str, test_id: &str) -> Score {
        Score {
            id: None,
            test: RecordId {
                tb: "test".to_owned(),
                id: test_id.into(),
            },
            user: RecordId {
                tb: "user".to_owned(),
                id: user_id.into(),
            },
            creation_date: Local::now(),
        }
    }
}
pub async fn create_score(db: &Surreal<Client>, score: &Score) -> surrealdb::Result<Score> {
    let new_score: Score = db.create("score").content(score).await?;
    Ok(new_score)
}
pub async fn read_score(db: &Surreal<Client>, score_id: &str) -> surrealdb::Result<Score> {
    let score: Score = db.select(("score", score_id)).await?;
    Ok(score)
}
