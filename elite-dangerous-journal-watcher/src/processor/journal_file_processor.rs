use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::processor::NotifierProcessor;
use lazy_regex::regex_is_match;
use notify::EventKind;
use notify_debouncer_full::DebounceEventResult;

use tracing::{debug, error, info, trace, warn};
use elite_dangerous_journal_model::JournalEvent;
use crate::config::JournalWatcherConfig;

pub struct JournalFileProcessor {}


impl NotifierProcessor for JournalFileProcessor {
    fn process(&self, event_list: DebounceEventResult, config: Arc<RwLock<JournalWatcherConfig>>) {
        for debounced_event in event_list.expect("Failed to get event list") {
            let event = debounced_event.event;
            trace!("Processing event: {:?}", event);

            match event.kind {
                EventKind::Modify(_) => {
                    for path in event.paths {
                        let filename = path.file_name().unwrap().to_str().unwrap();

                        if regex_is_match!(
                            r"^Journal.*\.log$",
                            filename
                        ) {
                            debug!("Detected Journal file modified: {:?}", path);
                            self.process_log_file(path, Arc::clone(&config));
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

    fn process_log_file(&self, file_path: PathBuf, config_lock: Arc<RwLock<JournalWatcherConfig>>) {

        let mut file_pos: u64;
        match config_lock.read() {
            Ok(config) => { 
                match config.data.file_positions.get(&file_path) {
                    None => { file_pos = 0; }
                    Some(pos) => { file_pos = *pos;}
                }
                
            }
            Err(e) => {
                error!("Failed to read config: {}", e);
                return
            }
        }

        debug!("Processing log file: {:?} from position {file_pos}", file_path);
        
        let mut f = File::open(&file_path).expect("Failed to open journal file");
        
        f.seek(SeekFrom::Start(file_pos)).expect("Failed to seek to file position");
        
        file_pos = f.metadata().expect("Failed to get file metadata").len();
        
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            if line.is_empty() {
                trace!("Skipping empty line");
                continue;
            }
            trace!("Processing line: {}", line);
            let event   = serde_json::from_str::<JournalEvent>(&line);
            match event {
                Ok(event) => {
                    info!("Event: {:?}", event);
                    
                    match event {
                        JournalEvent::Unknown => { warn!("Unknown event: {}", line);}
                        _ => {}
                    }
                }
                Err(e) => {
                    error!("Failed to parse event: {}", e);
                }
            }
        }
        
        match config_lock.write() {
            Ok(mut config) => {
                debug!("Updating config for position for file: {:?}...", file_path);
                config.data.file_positions.insert(file_path, file_pos);
                config.save();
                debug!("...updated config");
            }
            Err(_) => {
                error!("Failed to update config");
            }
        }
    }
}
