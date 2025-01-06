use std::path::Path;
use notify::{recommended_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use futures::{channel::mpsc::{Receiver, channel}, SinkExt};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
}


impl FileWatcher {
    pub async fn new() -> notify::Result<(FileWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let watcher = recommended_watcher(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        })?;

        let file_watcher = FileWatcher { watcher };

        Ok((file_watcher, rx))
    }

    pub fn add_path<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)
    }
}
