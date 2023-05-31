use crate::{
    db::handlers::test_handler::{self, Test},
    db::handlers::user_handler,
    AppState,
};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
#[get("")]
async fn index() -> actix_web::Result<impl actix_web::Responder> {
    Ok(format!("Hello"))
}
#[derive(Serialize)]
struct CreateTestResult {
    success: bool,
    test: Option<Test>,
    error: Option<String>,
}
#[derive(Deserialize)]
struct CreateTestParams {
    test: Test,
    session_id: String,
}
#[post("/create")]
async fn create_test(
    state: web::Data<AppState>,
    json: web::Json<CreateTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session = user_handler::get_session(session_id, &state.surreal.db).await;
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
    let test_result = test_handler::create_test(&state.surreal.db, &json.test).await;
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
    cfg.service(index).service(create_test);
}
