use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// health_check is a 200 status check handler for development
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub async fn run() -> std::io::Result<()> {
    // backbone of application, sets up http server in single line
    // handles all TRANSPORT level concerns
    HttpServer::new(|| {
        // where all the application logic lives: routing, middlewares, request handlers, etc
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}