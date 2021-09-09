use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    sync::{mpsc::channel, Arc, RwLock},
    time::Duration,
};

use tera::Tera;

pub fn watch_template(tera: Arc<RwLock<Tera>>) {
    tokio::spawn(async move {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = match Watcher::new(tx, Duration::from_millis(100)) {
            Ok(w) => w,
            Err(err) => {
                println!("Failed to create watcher!: {:?}", err);
                return;
            }
        };

        watcher
            .watch(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/"),
                RecursiveMode::Recursive,
            )
            .unwrap_or_else(|err| {
                println!("Failed to watch!: {:?}", err);
            });

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(_) = event {
                        tera.write().unwrap().full_reload().unwrap_or_else(|err| {
                            println!("Failed to reload!: {:?}", err);
                        });
                    }
                }
                Err(e) => println!("Error while watching: {:?}", e),
            }
        }
    });
}
