use mvp::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Maybe use pico args to get port from environment.

    run(Some("8080"))?.await
}
