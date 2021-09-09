use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    sync::{mpsc::channel, Arc, RwLock},
    time::Duration,
};

use tera::Tera;

pub fn watch_template(tera: Arc<RwLock<Tera>>) {
    tokio::spawn(async move {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Duration::from_millis(100)).expect("Failed to create watcher!");

        watcher
            .watch(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/"),
                RecursiveMode::Recursive,
            )
            .expect("Failed to watch Templates directory!");

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(_) = event {
                        tera.write()
                            .unwrap()
                            .full_reload()
                            .expect("Failed to Reload!")
                    }
                }
                Err(e) => println!("Error while watching: {:?}", e),
            }
        }
    });
}
