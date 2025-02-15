use std::future::Future;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use crate::{EliteDangerousEventProcessor, EliteDangerousPlugin};
use crate::exobiology_assistant::event_processor::ExobiologyAssistantEventProcessor;

pub mod event_processor;
pub mod model;


pub struct ExobiologyAssistantPlugin {
    event_processor: ExobiologyAssistantEventProcessor,
}

impl ExobiologyAssistantPlugin {

    pub async fn new(db: Arc<Surreal<Db>>) -> Self {
        Self {
            event_processor: ExobiologyAssistantEventProcessor::new(db)
        }
    }
}

impl EliteDangerousEventProcessor for ExobiologyAssistantPlugin {
    async fn process_event(&self, event: Arc<EliteDangerousEvent>) -> Result<(), String> {
        self.event_processor.process_event(event).await
    }
}

impl EliteDangerousPlugin for ExobiologyAssistantPlugin {
    fn name(&self) -> String {
        "Exobiology".to_string()
    }

    fn description(&self) -> String {
        "A plugin to assist with exobiology-related activities in Elite Dangerous.".to_string()
    }
}