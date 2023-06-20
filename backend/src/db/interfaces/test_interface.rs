use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::Record;

use super::{
    common::{add_membership, generate_id, Membership, MembershipType},
    user_interface::User,
};
#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub name: String,
    pub max_score: u32,
    pub creation_date: DateTime<Local>,
    pub creator: RecordId,
}
impl Test {
    pub fn create(
        name: String,
        max_score: u32,
        creation_date: DateTime<Local>,
        creator_id: String,
    ) -> Test {
        Test {
            name,
            max_score,
            creation_date,
            creator: RecordId {
                tb: "user".to_owned(),
                id: creator_id.into(),
            },
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TestRecord<T = RecordId> {
    pub name: String,
    pub max_score: u32,
    pub creation_date: DateTime<Local>,
    pub creator: T,
    pub id: RecordId,
}
pub async fn create_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    test: &Test,
) -> surrealdb::Result<TestRecord> {
    let new_test: TestRecord = db.create("test").content(test).await?;
    Ok(new_test)
}
pub async fn read_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: &str,
) -> surrealdb::Result<Option<TestRecord<User>>> {
    // let test: TestRecord<User> = db.select(("test", id)).await.unwrap();
    dbg!(&id);
    let test: Option<TestRecord<User>> = db
        .query("SELECT *, creator.* FROM $test")
        .bind((
            "test",
            RecordId {
                tb: "test".to_owned(),
                id: id.into(),
            },
        ))
        .await?
        .take(0)?;
    dbg!(&test);
    Ok(test)
}
pub async fn update_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    test: TestRecord,
) -> surrealdb::Result<TestRecord> {
    let updated: TestRecord = db
        .update(("test", &test.id.id.to_string()))
        .content(test)
        .await?;

    Ok(updated)
}
pub async fn delete_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: String,
) -> surrealdb::Result<()> {
    db.delete(("test", id)).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TestMembership {
    test: RecordId,
    user: RecordId,
}
impl Membership for TestMembership {
    fn create_membership(user: RecordId, group: RecordId) -> MembershipType<Self>
    where
        Self: std::marker::Sized,
    {
        let membership = TestMembership {
            test: group.clone(),
            user: user.clone(),
        };
        MembershipType::new(
            membership,
            generate_id(&group.id.to_string(), &user.id.to_string()),
            "test_membership".to_owned(),
        )
    }
}

pub async fn add_test_member(
    db: &Surreal<Client>,
    test_id: &String,
    user_id: &String,
) -> surrealdb::Result<()> {
    let generated_id = generate_id(user_id, test_id);
    // Check if test exists
    read_test(db, test_id).await?;
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
    let membership = TestMembership::create_membership(
        RecordId {
            tb: "user".to_owned(),
            id: user_id.into(),
        },
        RecordId {
            tb: "class".to_owned(),
            id: test_id.into(),
        },
    );
    add_membership(db, membership).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMembershipRecord {
    id: RecordId,
    test: Test,
    user: RecordId,
}
pub async fn read_test_memberships(
    db: &Surreal<Client>,
    user_id: &str,
) -> surrealdb::Result<Vec<TestMembershipRecord>> {
    let memberships: Vec<TestMembershipRecord> = db
        .query("SELECT *, test.* FROM test_membership WHERE user = $user")
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

pub async fn add_test_to_class(
    db: &Surreal<Client>,
    class_id: &str,
    test_id: &str,
) -> surrealdb::Result<()> {
    db.query("INSERT INTO test_membership SELECT user.id AS user, $test AS test FROM (SELECT user.id, class.id FROM class_membership WHERE class.id = $class)").bind(("test", RecordId{
        tb: "test".to_owned(),
        id: test_id.into()
    })).bind(("class", RecordId{
        tb: "class".to_owned(),
        id: class_id.into(),
    })).await?;
    Ok(())
}
