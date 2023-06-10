use crate::db::shared::json_traits::JsonResult;
use crate::{
    db::handlers::{
        class_handler::{self, Class, ClassRecord},
        session_handler,
    },
    AppState,
};
use actix_web::{get, post, web};
use chrono::Local;
use serde::{Deserialize, Serialize};
#[get("")]
async fn index() -> String {
    format!("Hi, welcome to the base route for the class endpoint")
}
#[derive(Deserialize)]
struct CreateClassClass {
    name: String,
}
#[derive(Deserialize)]
struct CreateClassParams {
    session_id: String,
    class: CreateClassClass,
}
#[derive(Serialize)]
struct ClassResult {
    success: bool,
    error: Option<String>,
    class: Option<ClassRecord>,
}
impl JsonResult<ClassRecord> for ClassResult {
    fn success(data: ClassRecord) -> ClassResult {
        Self {
            success: true,
            error: None,
            class: Some(data),
        }
    }
    fn failure(message: String) -> ClassResult {
        Self {
            success: false,
            error: Some(message),
            class: None,
        }
    }
}
#[post("create")]
async fn create_class(
    state: web::Data<AppState>,
    json: web::Json<CreateClassParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let user_session = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => return Ok(ClassResult::failure_json("No session with this id")),
    };
    let new_class = Class::create(json.class.name.clone(), Local::now(), session.user.user_id);
    let class = match class_handler::create_class(&state.surreal.db, &new_class).await {
        Ok(class) => class,
        Err(_) => return Ok(ClassResult::failure_json("Couldn't create class")),
    };
    Ok(ClassResult::success_json(class))
}
#[derive(Deserialize)]
struct ReadClassParams {
    session_id: String,
    id: String,
}
#[get("get_single")]
async fn read_class(
    state: web::Data<AppState>,
    query: web::Query<ReadClassParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    // I'm still deciding how best to handle user authentication. I'll probably use middleware in the future to authenticate and authorize based
    // on the route. So, for now the following is just going to be copied and pasted everywhere:
    let session_id = &query.session_id;
    let session = session_handler::get_session(session_id, &state.surreal.db).await;
    let _ = match session {
        Some(session) => session,
        None => return Ok(ClassResult::failure_json("No session with this id")),
    };
    // Should only be able to read by id, as names are not necessarily unique
    let class = match class_handler::read_class(&state.surreal.db, query.id.clone()).await {
        Ok(class) => class,
        Err(_) => return Ok(ClassResult::failure_json("Failed to read class")),
    };
    Ok(ClassResult::success_json(class))
}
#[derive(Serialize)]
struct ClassesResult {
    success: bool,
    error: Option<String>,
    classes: Option<Vec<ClassRecord>>,
}
impl JsonResult<Vec<ClassRecord>> for ClassesResult {
    fn success(data: Vec<ClassRecord>) -> ClassesResult {
        Self {
            success: true,
            error: None,
            classes: Some(data),
        }
    }
    fn failure(message: String) -> ClassesResult {
        Self {
            success: false,
            error: Some(message),
            classes: None,
        }
    }
}
#[derive(Deserialize)]
struct ReadClassesParams {
    session_id: String,
}
#[get("get")]
async fn read_classes(
    state: web::Data<AppState>,
    query: web::Query<ReadClassesParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &query.session_id;
    let user_session = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => return Ok(ClassesResult::failure_json("No session with this id")),
    };
    let classes: Vec<ClassRecord> = state
        .surreal
        .db
        .query(format!(
            "SELECT * FROM classes WHERE class.creator.user_id = {}",
            session.user.user_id
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(web::Json(ClassesResult::success(classes)))
}
pub fn class_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(create_class)
        .service(read_class)
        .service(read_classes);
}
