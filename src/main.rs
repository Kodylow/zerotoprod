use zerotoprod::run;

// tokio::main writes boilerplate to run async main as sync on top of tokio's runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
