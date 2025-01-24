use std::path::{Path};
use std::time::Duration;
use std::sync::Arc;
use futures::StreamExt;
use notify::event::CreateKind::File;
use notify_debouncer_full::DebounceEventResult;
use tracing::debug;
use crate::config::JournalWatcherConfig;
use crate::file_watcher::FileWatcher;

/// The main struct for interacting with the elite journal file watcher
pub struct EliteJournalWatcher {
    
    config: JournalWatcherConfig,
    file_watcher: FileWatcher,
}

impl EliteJournalWatcher {
    
    pub async fn new<P: AsRef<Path>> (working_dir: P, journal_dir: P, processor: Arc<dyn Fn(DebounceEventResult) + Send + Sync>) -> Self {
        
        let config = JournalWatcherConfig::new(working_dir.as_ref().to_path_buf());
        
        debug!("Config: {:?}", config);
        
        let (mut file_watcher, mut rx) = FileWatcher::new(Duration::from_millis(config.data.timeout_milli), None ).await.expect("Failed to create file watcher");
        file_watcher.add_path(journal_dir).expect("Failed to add journal dir to file watcher"); 
        
        let f = Arc::clone(&processor);
        tokio::spawn(async move {
            while let Some(res) = rx.next().await {
                f(res);
            }
        }).await.expect("Failed to spawn processor task");
        
        
        
        Self {
            config,
            file_watcher,
        }
        
    }
    
    pub fn persist_config(&self) {
        self.config.save();
    }
    
    
    pub fn terminate(self) {
        self.file_watcher.terminate();
    }
}