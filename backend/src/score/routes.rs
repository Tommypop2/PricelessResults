use actix_web::{get, web};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        handlers::score_handler, interfaces::score_interface::Score,
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
#[derive(Deserialize)]
struct CreateScoreParams {
    user_id: String,
    test_id: String,
    score: u32,
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
#[get("create")]
async fn create_score(
    state: web::Data<AppState>,
    query: web::Query<CreateScoreParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let new_score = Score::new(&query.user_id, &query.test_id, query.score);
    let res = score_handler::create_score(&state.surreal.db, &new_score).await;
    if let Ok(score_res) = res {
        return Ok(ScoreResult::success_json(score_res));
    }
    return Ok(ScoreResult::failure_json("Failed to create score"));
}
pub fn score_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(create_score);
}
