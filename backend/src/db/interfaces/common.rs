use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::Record;

pub fn generate_id(user_id: &str, group_id: &str) -> String {
    user_id.to_owned() + "-" + group_id
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipType<T> {
    pub membership: T,
    pub id: String,
    pub table: String,
}
impl<T> MembershipType<T> {
    pub fn new(membership: T, id: String, table: String) -> Self {
        Self {
            membership,
            id,
            table,
        }
    }
}
pub trait Membership {
    fn create_membership(record: Self) -> MembershipType<Self>
    where
        Self: std::marker::Sized;
}
pub async fn add_membership<T>(
    db: &Surreal<Client>,
    membership: MembershipType<T>,
) -> surrealdb::Result<()>
where
    T: Serialize + Membership,
{
    let _: Record = db
        .create((membership.table, membership.id))
        .content(membership.membership)
        .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct CountRecord {
    pub count: u32,
}

pub async fn clear_memberships(
    db: &Surreal<Client>,
    membership_name: &str,
    membership_id: &str,
) -> surrealdb::Result<()> {
    db.query(format!(
        "DELETE {membership_name}_membership WHERE {membership_name}=$thing"
    ))
    .bind((
        "thing",
        RecordId {
            tb: membership_name.to_owned(),
            id: membership_id.into(),
        },
    ))
    .await?;
    Ok(())
}
