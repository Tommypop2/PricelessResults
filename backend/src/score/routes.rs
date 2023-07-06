use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        handlers::{score_handler, session_handler, test_handler},
        interfaces::{common::generate_id, score_interface::Score},
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
    user: String,
    test: String,
    score: u32,
    session_id: String,
}
#[derive(Serialize)]
struct ScoreResult<T = Score> {
    success: bool,
    error: Option<String>,
    scores: Option<T>,
}
impl<T> JsonResult<T> for ScoreResult<T> {
    fn failure(message: String) -> Self {
        Self {
            success: false,
            error: Some(message),
            scores: None,
        }
    }

    fn success(record: T) -> ScoreResult<T> {
        ScoreResult {
            success: true,
            error: None,
            scores: Some(record),
        }
    }
}
#[derive(Serialize)]
struct ScoreUpdate {
    score: u32,
}
#[post("create")]
async fn create_score(
    state: web::Data<AppState>,
    json: web::Json<CreateScoreParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(ScoreResult::failure_json("No session with this id"));
        }
    };
    // Verify that test exists
    let test = match test_handler::read_test(&state.surreal.db, &json.test).await {
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
    // Check if score exists
    let score_res = score_handler::read_score(
        &state.surreal.db,
        score_handler::ReadScores::Both(&json.user, &json.test),
    )
    .await;
    if let Ok(score) = score_res {
        let score_id = if let Some(id) = score.id{
            id.id.to_string()
        } else {
            return Ok(ScoreResult::failure_json("No score with this id"));
        };
        let update_res = score_handler::update_score(
            &state.surreal.db,
            ScoreUpdate { score: json.score },
            &score_id,
        )
        .await;
        match update_res {
            Ok(updated) => return Ok(ScoreResult::success_json(updated)),
            Err(err) => return Ok(ScoreResult::failure_json(&err.to_string())),
        }
    } else {
        let new_score = Score::new(&json.user, &json.test, json.score);
        let score = score_handler::create_score(&state.surreal.db, &new_score)
            .await
            .unwrap();
        return Ok(ScoreResult::success_json(score));
    }
}
#[derive(Deserialize)]
struct ReadScoreParams {
    session_id: String,
    test_id: String,
}
#[get("read")]
async fn read_score(
    state: web::Data<AppState>,
    query: web::Query<ReadScoreParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &query.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(ScoreResult::failure_json("No session with this id"));
        }
    };
    // Verify that test exists
    let test = match test_handler::read_test(&state.surreal.db, &query.test_id).await {
        Ok(Some(test)) => test,
        Err(_) | Ok(None) => {
            return Ok(ScoreResult::failure_json("No test with this id"));
        }
    };
    // Verify that user is owner of that test
    if !(test.creator.user_id == session.user.user_id) {
        return Ok(ScoreResult::failure_json(
            "You are not authorised to read the scores for this test",
        ));
    }
    let scores = match score_handler::read_scores(
        &state.surreal.db,
        score_handler::ReadScores::TestId(&query.test_id),
    )
    .await
    {
        Ok(scores) => scores,
        Err(_) => {
            return Ok(ScoreResult::failure_json("Failed to read scores"));
        }
    };
    Ok(ScoreResult::success_json(scores))
}
#[derive(Deserialize)]
struct ReadUserScoresParams {
    session_id: String,
}
#[get("read_user")]
async fn read_user_scores(
    state: web::Data<AppState>,
    query: web::Query<ReadUserScoresParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &query.session_id;
    let session_opt = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_opt {
        Some(session) => session,
        None => {
            return Ok(ScoreResult::failure_json("No session with this id"));
        }
    };
    let scores = match score_handler::read_scores(
        &state.surreal.db,
        score_handler::ReadScores::UserId(&session.user.user_id),
    )
    .await
    {
        Ok(scores) => scores,
        _ => return Ok(ScoreResult::failure_json("Failed to retrieve scores")),
    };

    Ok(ScoreResult::success_json(scores))
}
pub fn score_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(create_score)
        .service(read_score)
        .service(read_user_scores);
}
