use futures::StreamExt;
use tokio::spawn;
use tracing::{error, info};
use elite_dangerous_journal_watcher::file_watcher::FileWatcher;

#[tokio::main]
async fn main() -> Result<(), ()> {

    tracing_subscriber::fmt::init();
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    info!("watching {}", path);

    let (mut watcher, mut rx) = FileWatcher::new().await.expect("Could not create FileWatcher");

    watcher.add_path(path).expect("Could not add path");

    let watcher_task = spawn(async move {
        while let Some(res) = rx.next().await {
            match res {
                Ok(event) => info!("changed: {:?}", event),
                Err(e) => error!("watch error: {:?}", e),
            }
        }
    });

    for i in 1..30 {
        info!("waiting...{i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    watcher_task.await.expect("watcher task failed");

    Ok(())
}