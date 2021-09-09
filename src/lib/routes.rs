use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use std::{fs, sync::Arc, sync::RwLock};
use tera::{Context, Tera};

pub struct AppData {
    pub tera: Arc<RwLock<Tera>>,
}

#[get("/hello/{name}")]
pub async fn say_hello(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("John");

    let contents = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/markdowns/test.md"))
        .expect("Something went wrong reading the file");

    let mut ctx = Context::new();
    ctx.insert("name", name);
    ctx.insert("content", &contents);

    let rendered = data
        .tera
        .read()
        .unwrap()
        .render("index.html", &ctx)
        .expect("Failed to Render!");

    HttpResponse::Ok().body(rendered)
}

#[get("/refresh")]
pub async fn refresh(data: web::Data<AppData>) -> impl Responder {
    data.tera.write().unwrap().full_reload().unwrap();

    HttpResponse::Ok()
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
