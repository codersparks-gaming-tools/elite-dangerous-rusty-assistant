use std::future::Future;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tracing::debug;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use elite_dangerous_journal_model::events::exploration::scan::event::ScanData;
use elite_dangerous_journal_model::events::JournalEvent::Scan;
use crate::EliteDangerousEventProcessor;

pub struct ExobiologyAssistantEventProcessor {
    db: Arc<Surreal<Db>>,
}

impl ExobiologyAssistantEventProcessor {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }
}

impl EliteDangerousEventProcessor for ExobiologyAssistantEventProcessor {
    async fn process_event(&self, event: Arc<EliteDangerousEvent>) -> Result<(), String> {
        
        match event.as_ref() {
            EliteDangerousEvent::JournalEvent(je) => match je {
                Scan(scan_event) => {
                    match &scan_event.scan_data {
                        ScanData::PlanetMoon(pm) => {
                            println!("{:?}", pm);
                        },
                        _ => {
                            debug!("Not interested in event: {:?}", event);
                        }
                    }
                },
                _ => {
                    debug!("Not interested in event: {:?}", event);
                }
            }
        }
        
        Ok(())
        
    }
}