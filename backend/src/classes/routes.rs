use crate::{
    db::handlers::{
        class_handler::{self, Class, ClassRecord},
        user_handler, session_handler,
    },
    AppState,
};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};

#[get("")]
async fn index() -> String {
    format!("Hi, welcome to the base route for the class endpoint")
}
#[derive(Deserialize)]
struct CreateClassParams {
    session_id: String,
    class: Class,
}
#[derive(Serialize)]
struct ClassResult {
    success: bool,
    error: Option<String>,
    class: Option<ClassRecord>,
}
impl ClassResult {
    fn failure(message: String) -> ClassResult {
        return ClassResult {
            success: false,
            error: Some(message),
            class: None,
        };
    }
    pub fn failure_json(message: &'static str) -> web::Json<ClassResult> {
        return web::Json(ClassResult::failure((*message).to_string()));
    }
    pub fn success(class: ClassRecord) -> ClassResult {
        return ClassResult {
            success: true,
            error: None,
            class: Some(class),
        };
    }
    pub fn success_json(class: ClassRecord) -> web::Json<ClassResult> {
        return web::Json(ClassResult::success(class));
    }
}
#[post("create")]
async fn create_class(
    state: web::Data<AppState>,
    json: web::Json<CreateClassParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session = session_handler::get_session(session_id, &state.surreal.db).await;
    let _ = match session {
        Some(session) => session,
        None => return Ok(ClassResult::failure_json("No session with this id")),
    };
    let class = match class_handler::create_class(&state.surreal.db, &json.class).await {
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
#[get("read")]
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
pub fn class_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(create_class).service(read_class);
}
