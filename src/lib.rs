use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use std::{net::TcpListener, sync::RwLock};

use tokio;

use tera::{Context, Tera};

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch(tera: Arc<RwLock<Tera>>) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(100))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates/"),
        RecursiveMode::Recursive,
    )?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(_) = event {
                    if let Err(e) = tera.write().unwrap().full_reload() {
                        println!("failed to reload: {}", e);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

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

    let tera_clone = Arc::clone(&tera);

    tokio::spawn(async move {
        if let Err(e) = watch(tera_clone) {
            println!("error: {:?}", e)
        }
    });

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
