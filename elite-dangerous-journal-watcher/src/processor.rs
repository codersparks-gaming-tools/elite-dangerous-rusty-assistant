use std::sync::{Arc, RwLock};
use notify_debouncer_full::DebounceEventResult;
use crate::config::JournalWatcherConfig;

pub mod log_event_processor;
pub mod journal_file_processor;

pub trait NotifierProcessor {
    fn process(&self, event: DebounceEventResult, config: Arc<RwLock<JournalWatcherConfig>>);
}
