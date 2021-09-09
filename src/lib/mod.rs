mod routes;
mod watcher;

use routes::{index, refresh, say_hello, AppData};
use watcher::watch_template;

use actix_web::{dev::Server, App, HttpServer};
use std::{net::TcpListener, sync::Arc, sync::RwLock};
use tera::Tera;
use tokio;

#[tokio::main]
pub async fn run(addr: Option<&str>) -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", addr.unwrap_or("0")))
        .expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let addr = format!("127.0.0.1:{}", port);

    println!("Listening on: http://{}", addr);

    let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let tera = Arc::new(RwLock::new(tera));

    watch_template(Arc::clone(&tera));

    let server = HttpServer::new(move || {
        App::new()
            .data(AppData {
                tera: Arc::clone(&tera),
            })
            .service(index)
            .service(say_hello)
            .service(refresh)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
