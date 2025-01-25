use notify_debouncer_full::DebounceEventResult;

pub mod log_event_processor;

pub trait NotifierProcessor {
    fn process(&self, event: DebounceEventResult);
}
