use crate::{
    user::db_handler as user_db_handler,
    user_tests::db_handler::{self, Test},
    AppState,
};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
#[get("/")]
async fn index() -> actix_web::Result<impl actix_web::Responder> {
    Ok(format!("Hello"))
}
#[derive(Deserialize)]
struct CreateTestData {
    test: Test,
    session_id: String,
}
#[derive(Serialize)]
struct CreateTestResult {
    success: bool,
    test: Option<Test>,
    error: Option<String>,
}
#[post("/create")]
async fn create_test(
    state: web::Data<AppState>,
    json: web::Json<CreateTestData>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session = user_db_handler::get_session(&json.session_id, &state.surreal.db).await;
    let _ = match session {
        Some(session) => session,
        None => {
            return Ok(web::Json(CreateTestResult {
                success: false,
                error: Some("No session with this id".into()),
                test: None,
            }))
        }
    };
    let test_result = db_handler::create_test(&state.surreal.db, &json.test).await;
    let test = match test_result {
        Ok(test) => test,
        Err(error) => {
            return Ok(web::Json(CreateTestResult {
                success: false,
                error: Some(error.to_string()),
                test: None,
            }))
        }
    };
    Ok(web::Json(CreateTestResult {
        success: true,
        test: Some(test),
        error: None,
    }))
}
pub fn test_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
