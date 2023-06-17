use crate::{
    db::handlers::{
        session_handler::{self, is_session_id_valid},
        test_handler::{self, Test},
    },
    AppState,
};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
use test_handler::TestRecord;
#[derive(Deserialize)]
struct GetTestsParams {
    session_id: String,
}
#[derive(Serialize)]
struct GetTestsResult {
    success: bool,
    tests: Option<Vec<TestRecord>>,
    error: Option<String>,
}
#[get("")]
async fn index(
    state: web::Data<AppState>,
    query: web::Query<GetTestsParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let user_session = session_handler::get_session(&query.session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => {
            return Ok(web::Json(GetTestsResult {
                success: false,
                error: Some("No session with this id".into()),
                tests: None,
            }))
        }
    };
    let creator_id = session.user.user_id;
    let tests: Vec<TestRecord> = state
        .surreal
        .db
        .query(format!(
            "SELECT * FROM test WHERE creator.user_id = {creator_id}"
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(web::Json(GetTestsResult {
        success: true,
        error: None,
        tests: Some(tests),
    }))
}
#[derive(Serialize)]
struct CreateTestResult {
    success: bool,
    test: Option<TestRecord>,
    error: Option<String>,
}
#[derive(Deserialize)]
struct CreateTestTest {
    pub max_score: u32,
    pub name: String,
}
#[derive(Deserialize)]
struct CreateTestParams {
    test: CreateTestTest,
    session_id: String,
}
#[post("/create")]
async fn create_test(
    state: web::Data<AppState>,
    json: web::Json<CreateTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session = session_handler::get_session(session_id, &state.surreal.db).await;
    let user_session = match session {
        Some(session) => session,
        None => {
            return Ok(web::Json(CreateTestResult {
                success: false,
                error: Some("No session with this id".into()),
                test: None,
            }));
        }
    };
    let created_test = test_handler::create_test(
        &state.surreal.db,
        &Test::create(
            json.test.name.clone(),
            json.test.max_score,
            user_session.user.user_id,
        ),
    )
    .await;
    let test = match created_test {
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
