use actix_web::{get, web};
use serde::Deserialize;

use crate::{db::handlers::test_statistics_handler, AppState};
#[derive(Deserialize)]
struct ClassAverageParams {
    // TODO: Use session_id for authentication, where the user should also only be able to view the averages if they are the member of the desired class
    session_id: String,
    class_id: String,
}
#[get("/class_average")]
async fn class_average(
    state: web::Data<AppState>,
    query: web::Query<ClassAverageParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    match test_statistics_handler::class_averages_all_tests(&state.surreal.db, &query.class_id)
        .await
    {
        Ok(averages) => Ok(actix_web::HttpResponse::Ok().json(averages)),
        Err(_) => Ok(actix_web::HttpResponse::InternalServerError().finish()),
    }
}
pub fn statistics_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(class_average);
}
