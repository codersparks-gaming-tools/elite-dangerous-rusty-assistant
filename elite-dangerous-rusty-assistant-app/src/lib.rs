use std::future::Future;
use std::sync::Arc;
use elite_dangerous_journal_model::events::EliteDangerousEvent;

pub mod pirate_massacre_helper;

pub trait EliteDangerousEventProcessor {
    fn process_event(&self, event: Arc<EliteDangerousEvent>) -> impl Future<Output= Result<(), String>> + Send;
}