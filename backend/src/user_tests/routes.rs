use crate::{
    db::handlers::{
        test_handler::{self, Test},
        session_handler::is_session_id_valid,
    },
    AppState,
};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
use test_handler::TestRecord;
#[get("")]
async fn index() -> actix_web::Result<impl actix_web::Responder> {
    Ok(format!("Hello"))
}
#[derive(Serialize)]
struct CreateTestResult {
    success: bool,
    test: Option<TestRecord>,
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
    if !is_session_id_valid(&state.surreal.db, session_id).await {
        return Ok(web::Json(CreateTestResult {
            success: false,
            error: Some("No session with this id".into()),
            test: None,
        }));
    }
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
