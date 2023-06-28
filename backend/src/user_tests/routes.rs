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
use actix_web::{get, post, web, Responder};
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
        None => return Ok(TestsResult::failure_json("No session with this id")),
    };
    let creator_id = session.user.user_id;
    // let tests: Vec<Test> = state
    //     .surreal
    //     .db
    //     .query(format!(
    //         "SELECT * FROM test WHERE creator.user_id = {creator_id}"
    //     ))
    //     .await
    //     .unwrap()
    //     .take(0)
    //     .unwrap();
    let tests = match test_handler::read_owned(&state.surreal.db, &creator_id).await {
        Ok(tests) => tests,
        _ => return Ok(TestsResult::failure_json("Failed to read tests")),
    };
    Ok(TestsResult::success_json(tests))
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
        None => return Ok(TestResult::failure_json("No session with this id")),
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
            return Ok(TestResult::failure_json("No session with this id"));
        }
    };
    // Verify that test exists
    let test = match test_handler::read_test(&state.surreal.db, &json.test_id).await {
        Ok(Some(test)) => test,
        Err(_) | Ok(None) => {
            return Ok(TestResult::failure_json("No test with this id"));
        }
    };
    // Verify that user is owner of that test
    if !(test.creator.user_id == session.user.user_id) {
        return Ok(TestResult::failure_json(
            "You are not authorised to assign this test",
        ));
    }
    // Assigning test to class
    if let Some(class_id) = &json.class_id {
        let result =
            test_handler::add_test_to_class(&state.surreal.db, class_id, &json.test_id).await;
        match result {
            Ok(test) => return Ok(TestResult::success_json(test)),
            Err(_) => return Ok(TestResult::failure_json("There was an error somewhere")),
        };
    }
    // Assigning to specific user
    if let Some(user_id) = &json.user_id {}

    // Neither class_id, or user_id was provided
    Ok(TestResult::failure_json("No class_id or user_id provided"))
}
#[derive(Serialize, Deserialize)]
struct TestMembershipRecordsResult<U> {
    success: bool,
    error: Option<String>,
    memberships: Option<Vec<TestMembershipRecord<U>>>,
}
impl<U> JsonResult<Vec<TestMembershipRecord<U>>> for TestMembershipRecordsResult<U> {
    fn success(record: Vec<TestMembershipRecord<U>>) -> Self {
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
        None => {
            return Ok(TestMembershipRecordsResult::failure_json(
                "No session with this id",
            ))
        }
    };
    let memberships =
        match test_handler::read_test_memberships(&state.surreal.db, &session.user.user_id).await {
            Ok(memberships) => memberships,
            Err(_) => {
                return Ok(TestMembershipRecordsResult::failure_json(
                    "Failed to read test memberships",
                ))
            }
        };
    Ok(TestMembershipRecordsResult::success_json(memberships))
}
#[derive(Deserialize)]
struct GetAssignedInClassParams {
    session_id: String,
    class_id: String,
    test_id: String,
}
#[get("get_assigned_in_class")]
async fn get_assigned_tests_by_class(
    state: web::Data<AppState>,
    query: web::Query<GetAssignedInClassParams>,
) -> actix_web::Result<impl Responder> {
    let user_session = session_handler::get_session(&query.session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => {
            return Ok(TestMembershipRecordsResult::failure_json(
                "No session with this id",
            ))
        }
    };
    let test_res = test_handler::read_test(&state.surreal.db, &query.test_id).await;
    match test_res {
        Ok(Some(test)) => {
            if test.creator.user_id != session.user.user_id {
                return Ok(TestMembershipRecordsResult::failure_json(
                    "You are not authorised to view this test",
                ));
            }
        }
        _ => {
            return Ok(TestMembershipRecordsResult::failure_json(
                "No test with this id",
            ))
        }
    }
    let memberships = match test_handler::read_test_memberships_by_class(
        &state.surreal.db,
        &query.class_id,
        &query.test_id,
    )
    .await
    {
        Ok(memberships) => memberships,
        Err(_) => {
            return Ok(TestMembershipRecordsResult::failure_json(
                "Failed to read test memberships",
            ))
        }
    };
    Ok(TestMembershipRecordsResult::success_json(memberships))
}
#[derive(Deserialize)]
struct DeleteTestParams {
    test_id: String,
    session_id: String,
}
#[post("delete")]
async fn delete_test(
    state: web::Data<AppState>,
    json: web::Json<DeleteTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let tst = match test_handler::delete_test(&state.surreal.db, &json.test_id).await {
        Ok(deleted) => deleted,
        _ => return Ok(TestResult::failure_json("Failed to delete test")),
    };
    Ok(TestResult::success_json(tst))
}
#[derive(Deserialize)]
struct FuzzyFindTestParams {
    session_id: String,
    search: String,
}
#[get("fuzzy_find")]
async fn fuzzy_find_test(
    state: web::Data<AppState>,
    query: web::Query<FuzzyFindTestParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let user_session = session_handler::get_session(&query.session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => return Ok(TestsResult::failure_json("No session with this id")),
    };
    // If the search query is empty, return all tests owned by the user
    let tests_res = if query.search.len() == 0usize {
        test_handler::read_owned(&state.surreal.db, &session.user.user_id).await
    } else {
        test_handler::read_tests_fuzzy_name(&state.surreal.db, &query.search, &session.user.user_id)
            .await
    };
    let tests = match tests_res {
        Ok(tests) => tests,
        Err(_) => return Ok(TestsResult::failure_json("Failed to fuzzy find tests")),
    };
    Ok(TestsResult::success_json(tests))
}

pub fn test_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(create_test)
        .service(get_assigned_tests)
        .service(get_assigned_tests_by_class)
        .service(assign_test)
        .service(delete_test)
        .service(fuzzy_find_test);
}
