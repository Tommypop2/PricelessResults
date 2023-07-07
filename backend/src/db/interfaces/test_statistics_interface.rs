use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use super::test_interface::Test;
pub enum Average {
    Mean,
    Median,
}
impl Average {
    fn value(&self) -> &str {
        match self {
            Average::Mean => "mean",
            Average::Median => "median",
        }
    }
}
pub async fn average_test_score(
    db: &Surreal<Client>,
    test_id: &str,
    average_type: Average,
) -> surrealdb::Result<()> {
    let fn_name = average_type.value();
    db.query(format!(
        "SELECT * FROM math::{fn_name}((SELECT * FROM score WHERE test = $test).score)"
    ))
    .bind((
        "test",
        RecordId {
            tb: "test".to_owned(),
            id: test_id.into(),
        },
    ))
    .await?;
    Ok(())
}
#[derive(Deserialize, Serialize)]
pub struct ClassAverage {
    mean_score: String,
    test: Test,
}
pub async fn class_averages_all_tests(
    db: &Surreal<Client>,
    class_id: &str,
) -> surrealdb::Result<Vec<ClassAverage>> {
    let class_averages = db.query("SELECT test, math::mean(score) as mean_score FROM (SELECT *, test.* FROM score WHERE (SELECT user FROM class_membership WHERE class=$class).user CONTAINS user) GROUP BY test").bind(("class", RecordId{
        tb: "class".to_owned(),
        id: class_id.into(),
    })).await?.take(0)?;
    Ok(class_averages)
}
