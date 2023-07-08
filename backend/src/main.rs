use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
mod classes;
mod db;
mod score;
mod statistics;
mod user;
mod user_tests;

use actix_cors::Cors;
use db::connection::surrealdb_connection::SurrealDBRepo;
use dotenv::dotenv;
use serde::Deserialize;
use surrealdb::sql::Thing;
use user::routes::user_routes;
use user_tests::routes::test_routes;

use crate::{
    classes::routes::class_routes, score::routes::score_routes,
    statistics::routes::statistics_routes,
};
struct AppState {
    surreal: SurrealDBRepo,
    oauth_clientid: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

async fn index() -> String {
    format!("This is base route for the Priceless Results API")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let client_id = std::env::var("client_id").expect("client_id variable should be set");
    let db_url = std::env::var("db_url").expect("db_url variable should be set");
    println!("Connecting to db at url: {}", db_url);
    let surreal = SurrealDBRepo::init(&db_url)
        .await
        .expect("Error connecting to SurrealDB!");

    println!("Connected to db");
    let surreal_data = Data::new(AppState {
        surreal,
        oauth_clientid: client_id,
    });
    HttpServer::new(move || {
        // let cors = Cors::default().allowed_origin("http://localhost:5173");
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            // This normalises the path for requests. However, if POST requests are normalised, they are transformed into get requests
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(surreal_data.clone())
            .route("/", web::get().to(index))
            .service(web::scope("/user").configure(user_routes))
            .service(web::scope("/tests").configure(test_routes))
            .service(web::scope("/class").configure(class_routes))
            .service(web::scope("/score").configure(score_routes))
            .service(web::scope("/statistics").configure(statistics_routes))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_index_ok() {
        let res = index().await;
        assert_eq!(res, "This is base route for the Priceless Results API")
    }
}
