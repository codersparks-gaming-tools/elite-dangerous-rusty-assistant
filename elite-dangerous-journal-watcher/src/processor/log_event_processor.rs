use notify_debouncer_full::DebounceEventResult;
use tracing::{error, info};
use crate::processor::NotifierProcessor;

pub struct LogEventProcessor {}

impl NotifierProcessor for LogEventProcessor {
    fn process(&self, event: DebounceEventResult) {
        match event {
            Ok(event) => info!("{:?}", event) ,
            Err(err) => { error!("{:?}", err);},
        }
    }
}

impl LogEventProcessor {
    
    pub fn new() -> Self {
        Self {}
    }
}