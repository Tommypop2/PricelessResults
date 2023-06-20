use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, method::Update, opt::RecordId, Surreal};

use crate::Record;

use super::common::{add_membership, generate_id, Membership, MembershipType};

// Classes will probably just be an alias for applying tests to many users at once, and for class averages. Other than that, they shouldn't actually need to have much functionality
#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub name: String,
    pub creation_date: DateTime<Local>,
    pub creator: RecordId,
    pub members: u32,
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
            members: 0,
        }
    }
}
// This is used to update the members_count field on the `Class` struct
#[derive(Serialize, Deserialize, Debug)]
struct MembersCount {
    pub members: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassRecord<T = RecordId> {
    pub name: String,
    pub id: RecordId,
    pub creation_date: DateTime<Local>,
    pub creator: T,
    pub members: u32,
}
// Classes Themselves
pub async fn create_class(db: &Surreal<Client>, class: &Class) -> surrealdb::Result<ClassRecord> {
    let new_class: ClassRecord = db.create("class").content(class).await?;
    Ok(new_class)
}
pub enum ClassIdentifier {
    Id(String),
    CreatorId(String),
}
pub async fn read_class(
    db: &Surreal<Client>,
    id: ClassIdentifier,
) -> surrealdb::Result<Option<ClassRecord>> {
    let class: Option<ClassRecord> = match id {
        ClassIdentifier::Id(id) => db.select(("class", id)).await?,
        ClassIdentifier::CreatorId(id) => db
            .query("SELECT *, creator.* FROM class WHERE creator.user_id = $creator_id")
            .bind(("creator_id", id))
            .await?
            .take(0)?,
    };
    Ok(class)
}
/**
 * Has the potential to return multiple classes
 */
pub async fn read_classes(
    db: &Surreal<Client>,
    id: ClassIdentifier,
) -> surrealdb::Result<Vec<ClassRecord>> {
    let classes: Vec<ClassRecord> = match id {
        ClassIdentifier::Id(id) => db.select(("class", id)).await?,
        ClassIdentifier::CreatorId(id) => db
            .query("SELECT *, creator.* FROM class WHERE creator.user_id = $creator_id")
            .bind(("creator_id", id))
            .await?
            .take(0)?,
    };
    Ok(classes)
}
pub async fn read_classes_fuzzy_name(
    db: &Surreal<Client>,
    name: &str,
    creator_id: &str,
) -> surrealdb::Result<Vec<ClassRecord>> {
    let classes: Vec<ClassRecord> = db
        // Should only be able to search through classes that the user has created
        .query("SELECT * FROM class WHERE name ~ $name AND creator = $creator")
        .bind(("name", name))
        .bind((
            "creator",
            RecordId {
                tb: "user".to_owned(),
                id: creator_id.into(),
            },
        ))
        .await?
        .take(0)?;
    Ok(classes)
}
pub async fn update_class<T: Serialize>(
    db: &Surreal<Client>,
    update: T,
    class_id: &str,
) -> surrealdb::Result<ClassRecord> {
    let updated: ClassRecord = db.update(("class", class_id)).merge(update).await?;
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
// But the records can be created as if they were
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
    read_class(db, ClassIdentifier::Id(class_id.to_string())).await?;
    // Check if they're already a member
    let result: Option<Record> = db.select(("class_membership", &generated_id)).await?;
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
    let count = count_members(db, class_id).await?;
    update_class(db, MembersCount { members: count }, class_id).await?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassMembershipRecord {
    id: RecordId,
    class: Class,
    user: RecordId,
}
pub async fn read_class_memberships(
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
#[derive(Debug, Deserialize)]
struct CountRecord {
    count: u32,
}
pub async fn count_members(db: &Surreal<Client>, class_id: &str) -> surrealdb::Result<u32> {
    let count: Option<CountRecord> = db
        .query("SELECT count FROM SELECT count(class=$class), class FROM class_membership GROUP BY class")
        .bind((
            "class",
            RecordId {
                tb: "class".to_owned(),
                id: class_id.into(),
            },
        ))
        .await?
        .take(0)?;
    if let Some(count) = count {
        return Ok(count.count);
    }
    return Ok(0);
}
