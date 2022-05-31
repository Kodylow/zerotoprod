use actix_web::{web, HttpResponse};

#[derive(serde:: Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
