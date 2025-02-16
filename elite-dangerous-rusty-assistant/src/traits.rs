use std::future::Future;
use std::sync::Arc;
use elite_dangerous_journal_model::events::EliteDangerousEvent;

pub trait EDRAComponent {
    /// Used to display the name of the component in the visualisation
    fn name(&self) -> String;
    /// Used to describe the component when viewing in a visualisation
    fn description(&self) -> String;

    /// Used to process the events from the journal watcher and other sources
    fn process_event(&self, event: Arc<EliteDangerousEvent>) -> impl Future<Output= Result<(), String>> + Send;
    
    /// Display output
    fn render(&self) -> impl Future<Output=()> + Send;
}