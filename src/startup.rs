use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::routes::{health_check, subscribe};
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.

// return Server on the happy path and drop async
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // backbone of application, sets up http server in single line
    // handles all TRANSPORT level concerns
    let server = HttpServer::new(|| {
        // where all the application logic lives: routing, middlewares, request handlers, etc
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
