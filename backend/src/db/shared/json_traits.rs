use actix_web::web;

pub trait JsonResult<T> {
    fn failure(message: String) -> Self;
    fn failure_json(message: &str) -> web::Json<Self>
    where
        Self: std::marker::Sized,
    {
        return web::Json(JsonResult::failure((*message).to_string()));
    }
    fn success(record: T) -> Self;
    fn success_json(record: T) -> web::Json<Self>
    where
        Self: std::marker::Sized,
    {
        return web::Json(JsonResult::success(record));
    }
}
