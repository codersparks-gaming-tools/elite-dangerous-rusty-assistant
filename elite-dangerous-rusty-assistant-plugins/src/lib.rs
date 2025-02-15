use std::future::Future;
use std::sync::Arc;
use elite_dangerous_journal_model::events::EliteDangerousEvent;

pub mod pirate_massacre_plugin;
pub mod exobiology_assistant;

pub trait EliteDangerousPlugin {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub trait EliteDangerousEventProcessor {
    fn process_event(&self, event: Arc<EliteDangerousEvent>) -> impl Future<Output= Result<(), String>> + Send;
}
