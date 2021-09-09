mod routes;
mod watcher;

use routes::{index, refresh, say_hello, AppData};
use watcher::watch_template;

use actix_web::{dev::Server, App, HttpServer};
use std::{collections::HashMap, net::TcpListener, sync::Arc, sync::RwLock};
use tokio;

use pulldown_cmark::{html, Options, Parser};
use tera::{to_value, try_get_value, Tera, Value};

pub fn markdown_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    // options.insert(Options::ENABLE_TABLES);
    // options.insert(Options::ENABLE_FOOTNOTES);
    // options.insert(Options::ENABLE_TASKLISTS);
    // options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(&s, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(to_value(html_output).unwrap())
}

#[tokio::main]
pub async fn run(addr: Option<&str>) -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", addr.unwrap_or("0")))
        .expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let addr = format!("127.0.0.1:{}", port);

    println!("Listening on: http://{}", addr);

    let mut tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.register_filter("markdown", markdown_filter);

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
