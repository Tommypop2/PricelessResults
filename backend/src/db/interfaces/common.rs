use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::Record;

pub fn generate_id(user_id: &String, group_id: &String) -> String {
    user_id.clone() + "-" + group_id
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
    fn create_membership(user: RecordId, group: RecordId) -> MembershipType<Self>
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
