use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    let new_score: Score = match db
        .query("INSERT INTO score SELECT crypto::sha1(test.id + user.id) as id, * FROM $content")
        .bind(("content", score))
        .await?
        .take(0)?
    {
        Some(score) => score,
        None => {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::InternalError(
                "Score not properly returned".to_owned(),
            )))
        }
    };
    dbg!("This has executed");
    Ok(new_score)
}
pub async fn read_score_by_id(db: &Surreal<Client>, score_id: &str) -> surrealdb::Result<Score> {
    let score: Score = db.select(("score", score_id)).await?;
    Ok(score)
}
pub async fn read_score<'a>(
    db: &Surreal<Client>,
    query: ReadScores<'a>,
) -> surrealdb::Result<Score> {
    let res = read_scores(db, query).await?;
    if res.is_empty() {
        return Err(surrealdb::Error::Api(surrealdb::error::Api::InternalError(
            "No scores present".to_owned(),
        )));
    }
    Ok(res[0].clone())
}
pub enum ReadScores<'a> {
    TestId(&'a str),
    UserId(&'a str),
    Both(&'a str, &'a str),
}
pub async fn read_scores<'a>(
    db: &Surreal<Client>,
    query: ReadScores<'a>,
) -> surrealdb::Result<Vec<Score>> {
    let scores: Vec<Score> = match query {
        ReadScores::TestId(test_id) => db
            .query("SELECT * FROM score WHERE test = $test")
            .bind((
                "test",
                RecordId {
                    tb: "test".to_owned(),
                    id: test_id.into(),
                },
            ))
            .await?
            .take(0)?,
        ReadScores::UserId(user_id) => db
            .query("SELECT * FROM score WHERE user = $user")
            .bind((
                "user",
                RecordId {
                    tb: "user".to_owned(),
                    id: user_id.into(),
                },
            ))
            .await?
            .take(0)?,
        ReadScores::Both(user_id, test_id) => db
            .query("SELECT * FROM score WHERE user = $user AND test = $test")
            .bind((
                "user",
                RecordId {
                    tb: "user".to_owned(),
                    id: user_id.into(),
                },
            ))
            .bind((
                "test",
                RecordId {
                    tb: "test".to_owned(),
                    id: test_id.into(),
                },
            ))
            .await?
            .take(0)?,
    };
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
