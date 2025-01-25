use std::path::{Path};
use std::time::Duration;
use std::sync::{Arc, RwLock};
use futures::{SinkExt, StreamExt};
use futures::channel::oneshot::Receiver;
use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use tokio::task::JoinSet;
use tracing::{debug, error, info};
use crate::config::JournalWatcherConfig;
use crate::processor::NotifierProcessor;


/// Function that will start a file watcher for the files in directory journal_dir. It will then
/// use the process function of the supplied processor to process that Notify event
pub async fn start<D, P>(working_dir: D, journal_dir: D, processor: Arc<P>, terminate_rx: Receiver<()>)
where D: AsRef<Path>, P: NotifierProcessor + Send + Sync + 'static {
    let config = JournalWatcherConfig::new(working_dir.as_ref().to_path_buf());

    debug!("Config: {:?}", config);
    

    // The notify channel is used to passed events from the notify crate to the processor
    let (mut notify_tx, mut notify_rx) = futures::channel::mpsc::channel(50);

    let tick_rate = match config.data.tick_rate_milli {
        None => { None }
        Some(tr) => { Some(Duration::from_millis(tr)) }
    };

    let mut watcher = new_debouncer(
        Duration::from_millis(config.data.timeout_milli),
        tick_rate,
        move |event: DebounceEventResult| {
            futures::executor::block_on(async { notify_tx.send(event).await.expect("Failed to send event to processor") });
        }
    ).expect("Failed to create debouncer to watch journal files");

    watcher.watch(journal_dir, RecursiveMode::Recursive).expect("Failed to add journal dir to file watcher");

    let config_lock = Arc::new(RwLock::new(config));
    let mut join_set = JoinSet::new();

    let thread_processor = Arc::clone(&processor);
    join_set.spawn(async move {
        while let Some(res) = notify_rx.next().await {
            thread_processor.process(res, Arc::clone(&config_lock));
        }
    });

    let terminate_handle = join_set.spawn(async move {
        let _ = terminate_rx.await;
        info!("Terminate message received, stopping watcher...");
    });

    match join_set.join_next_with_id().await {
        None => { error!("Failed to join thread");}
        Some(res) => {
            let (id, _) = res.expect("Failed to join thread");

            if id == terminate_handle.id() {
                watcher.stop()
            }
        }
    }






}

