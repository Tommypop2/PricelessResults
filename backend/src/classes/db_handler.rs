use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
// Classes will probably just be an alias for applying tests to many users at once, and for class averages. Other than that, they shouldn't actually need to have much functionality
#[derive(Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub creation_date: DateTime<Local>,
}
#[derive(Serialize, Deserialize)]
pub struct ClassRecord {
    pub name: String,
    pub id: String,
    pub creation_date: DateTime<Local>,
}
pub async fn create_class(db: &Surreal<Client>, class: &Class) -> surrealdb::Result<ClassRecord> {
    let new_class: ClassRecord = db.create("class").content(class).await?;
    Ok(new_class)
}
pub async fn read_class(db: &Surreal<Client>, id: String) -> surrealdb::Result<Class> {
    let class: Class = db.select(("class", id)).await?;
    Ok(class)
}
pub async fn update_class(db: &Surreal<Client>, class: ClassRecord) -> surrealdb::Result<Class> {
    let updated: Class = db.update(("class", &class.id)).content(class).await?;
    Ok(updated)
}
pub async fn delete_class(db: &Surreal<Client>, id: String) -> surrealdb::Result<()> {
    db.delete(("class", id)).await?;
    Ok(())
}