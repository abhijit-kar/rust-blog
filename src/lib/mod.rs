mod routes;
mod watcher;

use routes::{index, refresh, say_hello, AppData};
use watcher::watch_template;

use actix_web::{dev::Server, App, HttpServer};
use std::{collections::HashMap, net::TcpListener, sync::Arc, sync::RwLock};
use tokio;

use tera::{to_value, try_get_value, Tera, Value};

use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakOptions, ComrakPlugins,
};

pub fn markdown_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);

    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.description_lists = true;
    options.extension.footnotes = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.header_ids = Some("user-content-".to_owned());
    options.extension.front_matter_delimiter = Some("---".to_owned());

    let adapter = SyntectAdapter::new("base16-eighties.dark");

    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    Ok(to_value(markdown_to_html_with_plugins(
        s.as_str(),
        &options,
        &plugins,
    ))
    .unwrap())
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
