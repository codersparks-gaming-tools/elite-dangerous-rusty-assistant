use std::path::PathBuf;
use crate::processor::NotifierProcessor;
use lazy_regex::regex_is_match;
use notify::{Event, EventKind};
use notify_debouncer_full::{DebounceEventResult, DebouncedEvent};
use tracing::debug;

pub struct JournalFileProcessor {}

impl NotifierProcessor for JournalFileProcessor {
    fn process(&self, event_list: DebounceEventResult) {
        for debounced_event in event_list.expect("Failed to get event list") {
            let event = debounced_event.event;
            debug!("Processing event: {:?}", event);

            match event.kind {
                EventKind::Modify(_) => {
                    for path in event.paths {
                        let filename = path.file_name().unwrap().to_str().unwrap();

                        if regex_is_match!(
                            r"^Journal.*\.log$",
                            filename
                        ) {
                            debug!("Journal file modified: {:?}", path);
                            self.process_log_file(path);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl JournalFileProcessor {
    
    pub fn new() -> Self {
        Self {}
    }
    
    fn process_log_file(&self, file_path: PathBuf) {
        
        debug!("Processing log file: {:?}", file_path);
    }
}
