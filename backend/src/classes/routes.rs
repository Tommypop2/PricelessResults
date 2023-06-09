use crate::db::handlers::class_handler::{ClassIdentifier, ClassMembershipRecord};
use crate::db::interfaces::user_interface::User;
use crate::db::shared::json_traits::JsonResult;
use crate::{
    db::handlers::{
        class_handler::{self, Class},
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
    class: Option<Class>,
}
impl JsonResult<Class> for ClassResult {
    fn success(data: Class) -> ClassResult {
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
    let new_class = Class::new(json.class.name.clone(), Local::now(), session.user.user_id);
    let class = match class_handler::create_class(&state.surreal.db, &new_class).await {
        Ok(class) => class,
        Err(_) => return Ok(ClassResult::failure_json("Couldn't create class")),
    };
    Ok(ClassResult::success_json(class))
}
#[derive(Deserialize)]
struct DeleteClassParams {
    session_id: String,
    class_id: String,
}
#[post("delete")]
async fn delete_class(
    state: web::Data<AppState>,
    json: web::Json<DeleteClassParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let user_session = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => return Ok(ClassResult::failure_json("No session with this id")),
    };
    let class_to_delete: Option<Class> =
        class_handler::read_class(&state.surreal.db, ClassIdentifier::Id(&json.class_id))
            .await
            .unwrap_or(None);
    if class_to_delete.is_none() {
        return Ok(ClassResult::failure_json("No class with this id"));
    }
    // TODO: Remove all of the unwrapping
    let creator_id = class_to_delete.unwrap().creator.id.to_string();
    let temp = session.user.id.unwrap().to_string();
    let user_id = temp.split(":").nth(1).unwrap();
    if creator_id != user_id {
        return Ok(ClassResult::failure_json(
            "You don't have permission to delete this class",
        ));
    }
    // The class can then be deleted, as the user has permission to do so, and the class is valid
    let deleted = class_handler::delete_class(&state.surreal.db, &json.class_id).await;
    if let Ok(class) = deleted {
        return Ok(ClassResult::success_json(class));
    }
    Ok(ClassResult::failure_json("Failed to delete class"))
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
    let class =
        match class_handler::read_class(&state.surreal.db, ClassIdentifier::Id(&query.id)).await {
            Ok(Some(class)) => class,
            Ok(None) => return Ok(ClassResult::failure_json("No class with this id")),
            Err(_) => return Ok(ClassResult::failure_json("Failed to read class")),
        };
    Ok(ClassResult::success_json(class))
}
#[derive(Serialize)]
struct ClassesResult {
    success: bool,
    error: Option<String>,
    classes: Option<Vec<Class<User>>>,
}
impl JsonResult<Vec<Class<User>>> for ClassesResult {
    fn success(data: Vec<Class<User>>) -> ClassesResult {
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
#[get("get_created")]
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
    let classes: Vec<Class<User>> = state
        .surreal
        .db
        .query("SELECT *, creator.* FROM class WHERE creator.user_id = $user_id")
        .bind(("user_id", &session.user.user_id))
        .await
        .unwrap()
        .take(0)
        .unwrap();
    Ok(web::Json(ClassesResult::success(classes)))
}
#[derive(Serialize)]
struct ClassMembershipResult {
    success: bool,
    error: Option<String>,
    memberships: Option<Vec<ClassMembershipRecord>>,
}
#[get("get_joined")]
async fn read_joined(
    state: web::Data<AppState>,
    query: web::Query<ReadClassesParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &query.session_id;
    let user_session = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match user_session {
        Some(session) => session,
        None => {
            return Ok(web::Json(ClassMembershipResult {
                success: false,
                error: Some("No session with this id".to_string()),
                memberships: None,
            }))
        }
    };
    let classes: Vec<ClassMembershipRecord> =
        class_handler::read_class_memberships(&state.surreal.db, &session.user.user_id)
            .await
            // Let's just hope the function won't fail for now
            .unwrap();
    Ok(web::Json(ClassMembershipResult {
        success: true,
        error: None,
        memberships: Some(classes),
    }))
}
#[derive(Deserialize)]
struct JoinClassParams {
    session_id: String,
    class_id: String,
}
#[derive(Serialize)]
struct JoinClassResult {
    success: bool,
    error: Option<String>,
}
#[post("join")]
async fn join_class(
    state: web::Data<AppState>,
    json: web::Json<JoinClassParams>,
) -> actix_web::Result<impl actix_web::Responder> {
    let session_id = &json.session_id;
    let session_result = session_handler::get_session(session_id, &state.surreal.db).await;
    let session = match session_result {
        Some(session) => session,
        None => {
            return Ok(web::Json(JoinClassResult {
                success: false,
                error: Some("No session with this id".to_string()),
            }))
        }
    };
    match class_handler::add_member(&state.surreal.db, &json.class_id, &session.user.user_id).await
    {
        Ok(_) => {
            return Ok(web::Json(JoinClassResult {
                success: true,
                error: None,
            }))
        }
        Err(_) => {
            return Ok(web::Json(JoinClassResult {
                success: false,
                error: Some("Failed to join class, you may already be a part of it.".to_string()),
            }))
        }
    };
}
pub fn class_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(create_class)
        .service(delete_class)
        .service(read_class)
        .service(read_classes)
        .service(join_class)
        .service(read_joined);
}
