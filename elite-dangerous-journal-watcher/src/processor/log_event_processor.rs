use std::sync::{Arc, RwLock};
use notify_debouncer_full::DebounceEventResult;
use tracing::{error, info};
use crate::config::JournalWatcherConfig;
use crate::processor::NotifierProcessor;

pub struct LogEventProcessor {}

impl NotifierProcessor for LogEventProcessor {
    async fn process(&self, event: DebounceEventResult, _ : Arc<RwLock<JournalWatcherConfig>>) -> Result<(), String> {
        match event {
            Ok(event) => info!("{:?}", event) ,
            Err(err) => { error!("{:?}", err);},
        }
        
        Ok(())
    }
}

impl LogEventProcessor {
    
    pub fn new() -> Self {
        Self {}
    }
}