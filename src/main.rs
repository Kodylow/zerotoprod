use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// health_check is a 200 status check handler for development
async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
// tokio::main writes boilerplate to run async main as sync on top of tokio's runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // backbone of application, sets up http server in single line
    // handles all TRANSPORT level concerns
    HttpServer::new(|| {
        // where all the application logic lives: routing, middlewares, request handlers, etc
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
