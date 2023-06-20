use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::Record;

use super::{
    common::{add_membership, generate_id, Membership, MembershipType},
};

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
#[derive(Serialize, Deserialize)]
struct ClassMembership {
    class: RecordId,
    user: RecordId,
}
// This is completely over-engineered, it's just so the membership records have different group names, rather than just storing as:
// {user: RecordId, group: RecordId}
impl Membership for ClassMembership {
    fn create_membership(user: RecordId, group: RecordId) -> MembershipType<Self>
    where
        Self: std::marker::Sized,
    {
        let membership = ClassMembership {
            class: group.clone(),
            user: user.clone(),
        };
        MembershipType::new(
            membership,
            generate_id(&group.id.to_string(), &user.id.to_string()),
            "class_membership".to_owned(),
        )
    }
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
    let membership = ClassMembership::create_membership(
        RecordId {
            tb: "user".to_owned(),
            id: user_id.into(),
        },
        RecordId {
            tb: "class".to_owned(),
            id: class_id.into(),
        },
    );
    add_membership(db, membership).await?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassMembershipRecord {
    id: RecordId,
    class: Class,
    user: RecordId,
    // May be better to actually store this in the database with the class, as that'd speed the below process up
    members: Option<i32>,
}
pub async fn read_class_memberships(
    db: &Surreal<Client>,
    user_id: &String,
    include_count: bool,
) -> surrealdb::Result<Vec<ClassMembershipRecord>> {
    let mut query: &str = "SELECT *, class.* FROM class_membership WHERE user = $user";
    if include_count {
        // This takes about 120-160 microseconds to run. Probably better to use 2 separate queries
        query = "SELECT *, class.*, (SELECT count(class=class.id), class FROM class_membership GROUP BY class)[0].count as members FROM class_membership WHERE user = $user";
    }
    let memberships: Vec<ClassMembershipRecord> = db
        .query(query)
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
