use futures::{channel::mpsc::{channel, Receiver}, SinkExt};
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, RecommendedCache};
use std::path::Path;
use std::time::Duration;
use tracing::info;

/// File watcher struct
pub struct FileWatcher {
    watcher: Debouncer<RecommendedWatcher, RecommendedCache>,
}


impl FileWatcher {
    /// Create a new file watcher
    pub async fn new(timeout: Duration, tick_rate: Option<Duration>) -> notify::Result<(FileWatcher, Receiver<DebounceEventResult>)> {
        let (mut tx, rx) = channel(1);

        let watcher = new_debouncer(
            timeout,tick_rate, move |res| {
                futures::executor::block_on(async { tx.send(res).await.unwrap() })
            }
        ).expect("failed to create deboucer watcher");

        let file_watcher = FileWatcher { watcher };

        Ok((file_watcher, rx))
    }

    /// Add path for the file watcher to monitor
    pub fn add_path<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)
    }

    /// Terminate the file watcher
    pub fn terminate(self) {
        info!("File watcher terminating...");
        let  _ = &self.watcher.stop();
        info!("...File watcher terminated");
    }
}
