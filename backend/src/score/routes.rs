use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        handlers::{score_handler, session_handler, test_handler},
        interfaces::score_interface::Score,
        shared::json_traits::JsonResult,
    },
    AppState,
};
#[get("")]
async fn index() -> actix_web::Result<impl actix_web::Responder> {
    Ok(web::Json(
        "Welcome to the base route for the score endpoint",
    ))
}
#[derive(Deserialize, Debug)]
struct CreateScoreParams {
    user_id: String,
    test_id: String,
    score: u32,
    session_id: String,
}
#[derive(Serialize)]
struct ScoreResult {
    success: bool,
    error: Option<String>,
    score: Option<Score>,
}
impl JsonResult<Score> for ScoreResult {
    fn failure(message: String) -> Self {
        Self {
            success: false,
            error: Some(message),
            score: None,
        }
    }

    fn success(record: Score) -> ScoreResult {
        ScoreResult {
            success: true,
            error: None,
            score: Some(record),
        }
    }
}
#[post("create")]
async fn create_score(
    state: web::Data<AppState>,
    json: web::Json<CreateScoreParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    dbg!("Running");
    let session_id = &json.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(ScoreResult::failure_json("No session with this id"));
        }
    };
    // Verify that test exists
    let test = match test_handler::read_test(&state.surreal.db, &json.test_id).await {
        Ok(Some(test)) => test,
        Err(_) | Ok(None) => {
            return Ok(ScoreResult::failure_json("No test with this id"));
        }
    };
    // Verify that user is owner of that test
    if !(test.creator.user_id == session.user.user_id) {
        return Ok(ScoreResult::failure_json(
            "You are not authorised to assign this test",
        ));
    }
    let new_score = Score::new(&json.user_id, &json.test_id, json.score);
    let res = score_handler::create_score(&state.surreal.db, &new_score).await;
    if let Ok(score_res) = res {
        return Ok(ScoreResult::success_json(score_res));
    }
    return Ok(ScoreResult::failure_json("Failed to create score"));
}
pub fn score_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(create_score);
}
