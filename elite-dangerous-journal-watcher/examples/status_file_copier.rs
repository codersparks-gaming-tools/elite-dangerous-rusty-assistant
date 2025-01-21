use std::path::Path;
use elite_dangerous_journal_watcher::file_watcher::FileWatcher;
use futures::StreamExt;
use std::time::Duration;
use notify::EventKind::Modify;
use notify_debouncer_full::DebouncedEvent;
use tokio::{signal, spawn};
use tracing::{debug, error, info, trace};

#[tokio::main]
async fn main() -> Result<(), ()> {

    tracing_subscriber::fmt::init();
    let output_path = std::env::args().nth(2).expect("Output path required as 2nd argument");
    let path = std::env::args().nth(1).expect("Required path not supplied");
    info!("watching {}, output path: {}", path, output_path);

    let (mut watcher, mut rx) = FileWatcher::new(Duration::from_millis(1000), None)
        .await
        .expect("Could not create FileWatcher");

    watcher.add_path(path).expect("Could not add path");

    let watcher_task = spawn(async move {
        while let Some(res) = rx.next().await {
            match res {
                Ok(event) => {
                    trace!("Event received: {:?}", event);
                    process_file(event, output_path.as_ref());
                },
                Err(e) => error!("watch error: {:?}", e),
            }
        }
    });

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("CTRL-C pressed, shutting down");
            watcher.terminate();
        }
        Err(e) => {
            error!("Unable to listen for shutdown signal: {}", e);
        }
    }

    watcher_task.await.expect("watcher task failed");

    Ok(())
}

fn process_file(event: Vec<DebouncedEvent>, output_path: &Path) {

    event.iter().for_each(|event| {
        // For now we're only interested in .json files as these are the ones that are replaced
        match event.kind {
            Modify(_) => {
                debug!("processing event: {:?}", event);
                event.paths.iter().for_each(|f| {
                    let file_name = f.file_name().expect("file expected to exist").to_str().unwrap();
                    if file_name.ends_with("Market.json") {
                        info!("processing file: {}", file_name);

                        let now = chrono::Utc::now();
                        let new_file_name = format!("{}_{}", now.format("%Y-%m-%d_%H-%M-%S%.3f"), file_name);
                        let new_file_path = output_path.join(new_file_name);
                        std::fs::copy(f, new_file_path).expect("failed to copy file");




                    } else {
                        debug!("Ignoring file (does not end with Market.json): {}", f.display());
                    }

                })
            },
            _ => {}
        }

    })
}
