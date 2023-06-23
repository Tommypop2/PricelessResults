use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Response, Surreal};

// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<RecordId>,
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub picture: String,
    pub admin: bool,
}
impl User {
    pub fn new(
        user_id: String,
        username: String,
        email: String,
        picture: String,
        admin: bool,
    ) -> User {
        User {
            id: None,
            user_id,
            username,
            email,
            picture,
            admin,
        }
    }
}
pub async fn get_user<T>(id: String, db: &Surreal<Client>) -> Option<T>
where
    for<'a> T: Deserialize<'a>,
{
    let mut user_response: Response = db
        .query(format!("SELECT * FROM user WHERE user_id = \"{}\"", id))
        .await
        .unwrap();
    user_response.take(0).unwrap()
}
pub async fn create_user(
    db: &Surreal<surrealdb::engine::remote::ws::Client>,
    user: &User,
) -> surrealdb::Result<User> {
    let new_usr: User = db.create(("user", &user.user_id)).content(user).await?;
    Ok(new_usr)
}
