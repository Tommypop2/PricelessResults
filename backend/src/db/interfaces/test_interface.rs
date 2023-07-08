use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::Record;

use super::{
    common::{
        add_membership, clear_memberships, generate_id, CountRecord, Membership, MembershipType,
    },
    score_interface::Score,
    user_interface::User,
};
#[derive(Serialize, Deserialize, Debug)]
struct AssigneesCount {
    pub assignees: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Test<T = RecordId> {
    pub id: Option<RecordId>,
    pub name: String,
    pub max_score: u32,
    pub creation_date: DateTime<Local>,
    pub creator: T,
    pub assignees: u32,
}

impl Test {
    pub fn new(
        name: String,
        max_score: u32,
        creation_date: DateTime<Local>,
        creator_id: String,
    ) -> Test {
        Test {
            id: None,
            name,
            max_score,
            creation_date,
            creator: RecordId {
                tb: "user".to_owned(),
                id: creator_id.into(),
            },
            assignees: 0,
        }
    }
}
pub async fn create_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    test: &Test,
) -> surrealdb::Result<Test> {
    let new_test: Test = db.create("test").content(test).await?;
    Ok(new_test)
}
pub async fn read_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: &str,
) -> surrealdb::Result<Option<Test<User>>> {
    // let test: Test<User> = db.select(("test", id)).await.unwrap();
    let test: Option<Test<User>> = db
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
    Ok(test)
}
pub async fn read_tests_fuzzy_name(
    db: &Surreal<Client>,
    name: &str,
    creator_id: &str,
) -> surrealdb::Result<Vec<Test>> {
    let tests: Vec<Test> = db
        // Should only be able to search through classes that the user has created
        .query("SELECT * FROM test WHERE name ~ $name AND creator = $creator")
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
    Ok(tests)
}
pub async fn read_owned(db: &Surreal<Client>, creator_id: &str) -> surrealdb::Result<Vec<Test>> {
    let tests: Vec<Test> = db
        .query("SELECT * FROM test WHERE creator.user_id = $creator_id")
        .bind(("creator_id", &creator_id))
        .await?
        .take(0)?;
    Ok(tests)
}

pub async fn update_test<T: Serialize>(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    update: T,
    test_id: &str,
) -> surrealdb::Result<Test> {
    let updated: Test = db.update(("test", test_id)).merge(update).await?;

    Ok(updated)
}
pub async fn delete_test(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    id: &str,
) -> surrealdb::Result<Test> {
    clear_memberships(db, "test", id).await?;
    let test_opt: Option<Test> = db.delete(("test", id)).await?;
    match test_opt {
        Some(test) => Ok(test),
        None => Err(surrealdb::Error::Api(surrealdb::error::Api::InvalidParams(
            "Test doesn't have a present id".into(),
        ))),
    }
}

#[derive(Serialize, Deserialize)]
struct TestMembership<T = RecordId, U = RecordId, S = RecordId> {
    id: Option<RecordId>,
    test: T,
    user: U,
    score: Option<S>,
    creation_date: DateTime<Local>,
}
impl TestMembership {
    fn new(test: RecordId, user: RecordId, score: Option<RecordId>) -> Self {
        TestMembership {
            id: None,
            test,
            user,
            score,
            creation_date: Local::now(),
        }
    }
}
impl Membership for TestMembership {
    fn create_membership(record: Self) -> MembershipType<Self>
    where
        Self: std::marker::Sized,
    {
        let user_id = &record.user.id.to_string();
        let group_id = &record.test.id.to_string();
        MembershipType::new(
            record,
            generate_id(user_id, group_id),
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
    if let Some(_) = result {
        return Ok(());
    }
    // Add membership
    let membership = TestMembership::create_membership(TestMembership::new(
        RecordId {
            tb: "user".to_owned(),
            id: user_id.into(),
        },
        RecordId {
            tb: "class".to_owned(),
            id: test_id.into(),
        },
        // TODO: Fix this
        None,
        // RecordId{
        //     tb: "score".to_owned(),
        //     id: generate_id(user_id, test_id).into(),
        // }
    ));
    add_membership(db, membership).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMembershipRecord<U = RecordId, S = RecordId> {
    id: RecordId,
    test: Test,
    user: U,
    score: Option<S>,
    creation_date: DateTime<Local>,
}
pub async fn read_test_memberships(
    db: &Surreal<Client>,
    user_id: &str,
) -> surrealdb::Result<Vec<TestMembershipRecord<RecordId, Score>>> {
    let memberships: Vec<TestMembershipRecord<_, Score>> = db
        .query("SELECT *, test.*, score.* FROM test_membership WHERE user = $user ORDER BY creation_date ASC")
        .bind((
            "user",
            RecordId {
                tb: "user".to_owned(),
                id: user_id.into(),
            },
        ))
        .await.unwrap()
        .take(0).unwrap();
    Ok(memberships)
}
pub async fn read_test_memberships_by_class(
    db: &Surreal<Client>,
    class_id: &str,
    test_id: &str,
) -> surrealdb::Result<Vec<TestMembershipRecord<User>>> {
    let memberships: Vec<TestMembershipRecord<User>> = db
        .query(
            "SELECT *, test.*, user.* FROM test_membership WHERE test=$test AND (SELECT user FROM class_membership WHERE class=$class).user CONTAINS user",
        )
        .bind((
            "class",
            RecordId {
                tb: "class".to_owned(),
                id: class_id.into(),
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
        .take(0)?;
    Ok(memberships)
}
pub async fn add_test_to_class(
    db: &Surreal<Client>,
    class_id: &str,
    test_id: &str,
) -> surrealdb::Result<Test> {
    // Don't like doing this, but it's necessary to avoid 2 db queries for now. Ideally, it'd be possible to extract the pure id, without any special characters from the record
    // It seems for now hashing is the only reasonable method to not include those characters in the id

   db.query("INSERT INTO test_membership SELECT crypto::sha1(string::concat(type::string(user.id), type::string(($test).id))) as id, user.id AS user, $test AS test, type::thing('score', crypto::sha1(string::concat(type::string(user.id), type::string(($test).id)))) AS score, $creation_date as creation_date FROM (SELECT user.id, class.id FROM class_membership WHERE class.id = $class)")
    .bind(("test", RecordId{
        tb: "test".to_owned(),
        id: test_id.into()
    })).bind(("class", RecordId{
        tb: "class".to_owned(),
        id: class_id.into(),
    })).bind(("creation_date", &Local::now())).await.unwrap();
    let count = count_members(db, test_id).await?;
    let tst = update_test(db, AssigneesCount { assignees: count }, test_id).await?;
    Ok(tst)
}

pub async fn count_members(db: &Surreal<Client>, test_id: &str) -> surrealdb::Result<u32> {
    let count: Option<CountRecord> = db
        .query("SELECT count FROM SELECT count(), test FROM test_membership WHERE test=$test GROUP BY test")
        .bind((
            "test",
            RecordId {
                tb: "test".to_owned(),
                id: test_id.into(),
            },
        ))
        .await?
        .take(0)?;
    if let Some(count) = count {
        return Ok(count.count);
    }
    Ok(0)
}
