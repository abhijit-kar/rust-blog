use mvp::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Maybe use pico args to get port from environment.

    // Use Some("8080") to specify the port explicitly.
    run(None)?.await
}
