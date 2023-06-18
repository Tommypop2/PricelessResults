use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use super::user_interface::User;

// Classes will probably just be an alias for applying tests to many users at once, and for class averages. Other than that, they shouldn't actually need to have much functionality
#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub name: String,
    pub creation_date: DateTime<Local>,
    pub creator: RecordId,
}
impl Class {
    pub fn create(name: String, creation_date: DateTime<Local>, creator_id: String) -> Class {
        Class {
            name,
            creation_date,
            creator: RecordId {
                tb: "user".to_owned(),
                id: creator_id.into(),
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassRecord<T = RecordId> {
    pub name: String,
    pub id: T,
    pub creation_date: DateTime<Local>,
    pub creator: RecordId,
}
// Classes Themselves
pub async fn create_class(db: &Surreal<Client>, class: &Class) -> surrealdb::Result<ClassRecord> {
    let new_class: ClassRecord = db.create("class").content(class).await?;
    Ok(new_class)
}
pub async fn read_class(db: &Surreal<Client>, id: String) -> surrealdb::Result<ClassRecord> {
    let class: ClassRecord = db.select(("class", id)).await?;
    Ok(class)
}
pub async fn update_class(
    db: &Surreal<Client>,
    class: ClassRecord,
) -> surrealdb::Result<ClassRecord> {
    let updated: ClassRecord = db
        .update(("class", &class.id.id.to_string()))
        .content(class)
        .await?;
    Ok(updated)
}
pub async fn delete_class(db: &Surreal<Client>, id: String) -> surrealdb::Result<()> {
    db.delete(("class", id)).await?;
    Ok(())
}

// Handling memberships. This might be split into another interface at some point
#[derive(Deserialize, Debug)]
struct Record {
    #[allow(dead_code)]
    id: RecordId,
}
fn generate_id(user_id: &String, class_id: &String) -> String {
    user_id.clone() + "-" + class_id
}
#[derive(Serialize, Deserialize)]
struct ClassMembership {
    class: RecordId,
    user: RecordId,
}
// Way too many responsibilities. Will be extracted later
pub async fn add_member(
    db: &Surreal<Client>,
    class_id: &String,
    user_id: &String,
) -> surrealdb::Result<()> {
    let generated_id = generate_id(user_id, class_id);
    // Check if class exists
    read_class(db, class_id.to_string()).await?;
    // Check if they're already a member
    let result: Option<Record> = db
        .select(("class_membership", &generated_id))
        .await
        .unwrap();
    match result {
        Some(_) => return Ok(()),
        None => {}
    }
    // Add membership
    db.create(("class_membership", generated_id))
        .content(ClassMembership {
            class: RecordId {
                tb: "class".to_owned(),
                id: class_id.into(),
            },
            user: RecordId {
                tb: "user".to_owned(),
                id: user_id.into(),
            },
        })
        .await?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassMembershipRecord {
    id: RecordId,
    class: Class,
    user: RecordId,
}
pub async fn read_memberships(
    db: &Surreal<Client>,
    user_id: &String,
) -> surrealdb::Result<Vec<ClassMembershipRecord>> {
    let memberships: Vec<ClassMembershipRecord> = db
        .query("SELECT *, class.* FROM class_membership WHERE user = $user")
        .bind((
            "user",
            RecordId {
                tb: "user".to_owned(),
                id: user_id.into(),
            },
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(memberships)
}
