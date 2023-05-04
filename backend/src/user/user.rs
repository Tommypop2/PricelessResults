use actix_web::{get, web};
// This is terrible structure: will be fixed in the future hopefully
use crate::AppState;
#[get("/cool_route")]
async fn cool_route(shared_data: web::Data<AppState>) -> String {
    shared_data.oauth_clientid.clone().into()
}
pub fn create_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(cool_route);
}
