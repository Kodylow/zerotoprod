use std::net::TcpListener;

use zerotoprod::run;

// tokio::main writes boilerplate to run async main as sync on top of tokio's runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    run(listener)?.await
}
