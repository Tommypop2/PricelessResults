use crate::{
    db::{
        handlers::{
            session_handler::{self},
            test_handler::{self, Test, TestMembershipRecord},
        },
        shared::json_traits::JsonResult,
    },
    AppState,
};
use actix_web::{get, post, web};
use chrono::Local;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct GetTestsParams {
    session_id: String,
}
#[derive(Serialize)]
struct TestsResult {
    success: bool,
    tests: Option<Vec<Test>>,
    error: Option<String>,
}
impl JsonResult<Vec<Test>> for TestsResult {
    fn success(record: Vec<Test>) -> Self {
        Self {
            success: true,
            tests: Some(record),
            error: None,
        }
    }
    fn failure(message: String) -> Self {
        Self {
            success: false,
            tests: None,
            error: Some(message),
        }
    }
}
#[get("get_created")]
async fn index(
    state: web::Data<AppState>,
    query: web::Query<GetTestsParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let user_session = session_handler::get_session(&query.session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => {
            return Ok(web::Json(TestsResult {
                success: false,
                error: Some("No session with this id".into()),
                tests: None,
            }))
        }
    };
    let creator_id = session.user.user_id;
    let tests: Vec<Test> = state
        .surreal
        .db
        .query(format!(
            "SELECT * FROM test WHERE creator.user_id = {creator_id}"
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(web::Json(TestsResult {
        success: true,
        error: None,
        tests: Some(tests),
    }))
}
#[derive(Serialize)]
struct TestResult {
    success: bool,
    test: Option<Test>,
    error: Option<String>,
}
impl JsonResult<Test> for TestResult {
    fn failure(message: String) -> Self {
        Self {
            success: false,
            test: None,
            error: Some(message),
        }
    }
    fn success(record: Test) -> Self {
        Self {
            success: true,
            test: Some(record),
            error: None,
        }
    }
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
#[post("create")]
async fn create_test(
    state: web::Data<AppState>,
    json: web::Json<CreateTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(web::Json(TestResult {
                success: false,
                error: Some("No session with this id".into()),
                test: None,
            }));
        }
    };
    let created_test = test_handler::create_test(
        &state.surreal.db,
        &Test::new(
            json.test.name.clone(),
            json.test.max_score,
            Local::now(),
            session.user.user_id,
        ),
    )
    .await;
    let test = match created_test {
        Ok(test) => test,
        Err(error) => {
            return Ok(web::Json(TestResult {
                success: false,
                error: Some(error.to_string()),
                test: None,
            }))
        }
    };
    Ok(web::Json(TestResult {
        success: true,
        test: Some(test),
        error: None,
    }))
}
#[derive(Deserialize)]
struct AssignTestParams {
    session_id: String,
    test_id: String,
    class_id: Option<String>,
    user_id: Option<String>,
}

#[post("assign")]
async fn assign_test(
    state: web::Data<AppState>,
    json: web::Json<AssignTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(TestResult::failure_json("No session with this id".into()));
        }
    };
    // Verify that test exists
    let test = match test_handler::read_test(&state.surreal.db, &json.test_id).await {
        Ok(Some(test)) => test,
        Err(_) | Ok(None) => {
            return Ok(TestResult::failure_json("No test with this id".into()));
        }
    };
    // Verify that user is owner of that test
    if !(test.creator.user_id == session.user.user_id) {
        return Ok(TestResult::failure_json(
            "You are not authorised to assign this test".into(),
        ));
    }
    // Assigning test to class
    if let Some(class_id) = &json.class_id {
        let result =
            test_handler::add_test_to_class(&state.surreal.db, class_id, &json.test_id).await;
        // Kinda annoying. Don't really actually need to return a TestResult.
        match result {
            Ok(_) => {
                return Ok(web::Json(TestResult {
                    success: true,
                    test: None,
                    error: None,
                }))
            }
            Err(_) => {
                return Ok(TestResult::failure_json(
                    "There was an error somewhere".into(),
                ))
            }
        };
    }
    // Assigning to specific user
    if let Some(user_id) = &json.user_id {}

    // Neither class_id, or user_id was provided
    Ok(TestResult::failure_json(
        "No class_id or user_id provided".into(),
    ))
}
#[derive(Serialize, Deserialize)]
struct TestMembershipRecordsResult {
    success: bool,
    error: Option<String>,
    memberships: Option<Vec<TestMembershipRecord>>,
}
impl JsonResult<Vec<TestMembershipRecord>> for TestMembershipRecordsResult {
    fn success(record: Vec<TestMembershipRecord>) -> Self {
        Self {
            success: true,
            error: None,
            memberships: Some(record),
        }
    }
    fn failure(message: String) -> Self {
        Self {
            success: false,
            error: Some(message),
            memberships: None,
        }
    }
}
#[get("get_assigned")]
async fn get_assigned_tests(
    state: web::Data<AppState>,
    query: web::Query<GetTestsParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let user_session = session_handler::get_session(&query.session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => return Ok(TestMembershipRecordsResult::failure_json("No session with this id".into())),
    };
    let memberships =
        match test_handler::read_test_memberships(&state.surreal.db, &session.user.user_id).await {
            Ok(memberships) => memberships,
            Err(_) => {
                return Ok(TestMembershipRecordsResult::failure_json(
                    "Failed to read test memberships".into(),
                ))
            }
        };
    Ok(TestMembershipRecordsResult::success_json(memberships))
}
pub fn test_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(create_test)
        .service(get_assigned_tests)
        .service(assign_test);
}
