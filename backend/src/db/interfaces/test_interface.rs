use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
#[derive(Serialize, Deserialize)]
pub struct Test {
    pub name: String,
    pub max_score: u32,
    pub id: String,
}
pub async fn create_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    test: &Test,
) -> surrealdb::Result<Test> {
    let new_test: Test = db.create(("test", &test.id)).content(test).await?;
    Ok(new_test)
}
pub async fn read_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: String,
) -> surrealdb::Result<Test> {
    let test: Test = db.select(("test", id)).await?;
    Ok(test)
}
pub async fn update_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    test: Test,
) -> surrealdb::Result<Test> {
    let updated: Test = db.update(("test", &test.id)).content(test).await?;
    Ok(updated)
}
pub async fn delete_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: String,
) -> surrealdb::Result<()> {
    db.delete(("test", id)).await?;
    Ok(())
}
