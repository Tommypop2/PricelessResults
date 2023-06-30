use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use super::common::generate_id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Score<T = RecordId, U = RecordId> {
    pub id: Option<RecordId>,
    pub score: Option<u32>,
    pub test: T,
    pub user: U,
    pub creation_date: DateTime<Local>,
}
impl Score {
    pub fn new(user_id: &str, test_id: &str, score: u32) -> Score {
        Score {
            score: Some(score),
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
    let new_score: Score = db
        .create((
            "score",
            &generate_id(&score.user.id.to_string(), &score.test.id.to_string()),
        ))
        .content(score)
        .await?;
    Ok(new_score)
}
pub async fn read_score(db: &Surreal<Client>, score_id: &str) -> surrealdb::Result<Score> {
    let score: Score = db.select(("score", score_id)).await?;
    Ok(score)
}
pub async fn read_scores(db: &Surreal<Client>, test_id: &str) -> surrealdb::Result<Vec<Score>> {
    let scores: Vec<Score> = db
        .query("SELECT * FROM score WHERE test = $test")
        .bind((
            "test",
            RecordId {
                tb: "test".to_owned(),
                id: test_id.into(),
            },
        ))
        .await?
        .take(0)?;
    Ok(scores)
}
pub async fn update_score<T: Serialize>(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    update: T,
    score_id: &str,
) -> surrealdb::Result<Score> {
    let updated: Score = db.update(("score", score_id)).merge(update).await?;
    Ok(updated)
}
