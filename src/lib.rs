mod watcher;

use watcher::watch_template;

use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::{net::TcpListener, sync::Arc, sync::RwLock};

use tokio;

use tera::{Context, Tera};

struct AppData {
    tera: Arc<RwLock<Tera>>,
}

#[get("/hello/{name}")]
async fn say_hello(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("John");

    let mut ctx = Context::new();
    ctx.insert("name", name);

    let rendered = data
        .tera
        .read()
        .unwrap()
        .render("index.html", &ctx)
        .expect("Failed to Render!");

    HttpResponse::Ok().body(rendered)
}

#[get("/refresh")]
async fn refresh(data: web::Data<AppData>) -> impl Responder {
    data.tera.write().unwrap().full_reload().unwrap();

    HttpResponse::Ok()
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

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
