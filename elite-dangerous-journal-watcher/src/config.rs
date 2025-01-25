use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "journal_watcher.json";

#[derive(Debug)]
pub struct JournalWatcherConfig {
    metadata: JournalWatcherConfigMetadata,
    pub data: JournalWatcherConfigData
}

impl JournalWatcherConfig {
    pub fn new<P: AsRef<Path>>(location: P) -> Self {
        
        let config_file = location.as_ref().join(CONFIG_FILE_NAME);
        
        let data : JournalWatcherConfigData;
        if config_file.exists() {
            data = JournalWatcherConfigData::load(&config_file);
        } else {
            data = Default::default();
            data.save(&config_file);
        }

        let metadata = JournalWatcherConfigMetadata {
            config_file_location: config_file
        };
        
        Self {
            metadata,
            data,
        }
    }
    
    pub fn save(&self) {
        self.data.save(self.metadata.config_file_location.clone());
    }

}


/// Configuration data of the Journal watcher
#[derive(Serialize, Deserialize, Debug)]
pub struct JournalWatcherConfigData {
    pub file_positions: HashMap<PathBuf, u64>,
    pub tick_rate_milli: Option<u64>,
    pub timeout_milli: u64,
}

impl Default for JournalWatcherConfigData {
    fn default() -> Self {
        Self {
            file_positions: HashMap::new(),
            tick_rate_milli: None,
            timeout_milli: 500,
        }
    }
}

impl JournalWatcherConfigData {
    pub fn load<P: AsRef<Path>>(location: P) -> Self {

        let file = File::open(location).expect("Unable to open config file");
        let config: Self = serde_json::from_reader(file).expect("Unable to parse config file");
        config
    }
    
    pub fn save<P: AsRef<Path>>(&self, location: P) {
        let file = File::create(location).expect("Unable to create config file");
        serde_json::to_writer(file, self).expect("Unable to write config file");
    }
}

/// Configuration of metadata for the config to use
#[derive(Debug)]
pub struct JournalWatcherConfigMetadata {
    config_file_location: PathBuf,
}