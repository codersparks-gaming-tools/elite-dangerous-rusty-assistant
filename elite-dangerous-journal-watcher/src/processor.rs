use notify_debouncer_full::DebounceEventResult;
use tracing::{error, info};

pub fn log_event_processor(result: DebounceEventResult) {
    
    match result {
        Ok(event) => info!("{:?}", event) ,
        Err(err) => { error!("{:?}", err);},
    }
}