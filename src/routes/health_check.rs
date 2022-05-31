use actix_web::{web, HttpResponse, Responder};
// health_check is a 200 status check handler for development
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
