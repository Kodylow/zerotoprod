pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{web, HttpResponse, Responder};

#[derive(serde:: Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
