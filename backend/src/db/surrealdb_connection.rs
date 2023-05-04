use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
use surrealdb::Surreal;
use surrealdb::{Error};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub db: Surreal<Client>,
}

impl SurrealDBRepo {
    pub async fn init(address: &str) -> Result<Self, Error> {
        let db = Surreal::new::<Ws>(address).await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("main").use_db("database").await?;
        Ok(SurrealDBRepo { db })
    }
}
