mod model;
mod event_processor;

use crate::pirate_massacre_plugin::event_processor::PirateMassacreEventProcessor;
use crate::pirate_massacre_plugin::model::PirateMassacreMission;
use crate::EliteDangerousEventProcessor;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tracing::debug;

const DATABASE_NAMESPACE: &str = "pirate_massacre_plugin";
const DATABASE_NAME: &str = "data";
const RECORD_NAME: &str = "massacre_mission";

pub struct PirateMassacrePlugin {
    db: Arc<Surreal<Db>>,
    event_processor: PirateMassacreEventProcessor
}

impl PirateMassacrePlugin {
    pub async fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db: db.clone(), event_processor: PirateMassacreEventProcessor::new(db.clone()) }
    }

    pub async fn get_all_missions(&self) -> std::result::Result<Vec<PirateMassacreMission>, String> {
        self.db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await.unwrap();

        let select_result : surrealdb::Result<Vec<PirateMassacreMission>> = self.db.select(RECORD_NAME).await;

        match select_result {
            Ok(records) => {
                Ok(records)
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
    
    pub async fn get_mission_summary_by_faction(&self, filter_redirected: bool) -> std::result::Result<(HashMap<(String, String, String), u64>, u64, u64), String> {
        
        let missions = self.get_all_missions().await?;
        let total_missions = missions.len() as u64;
        let mut remaining_active_misisons = 0;
        
        let mut missions_summary: HashMap<(String, String, String), u64> = HashMap::new();
        
        missions.into_iter().for_each(|m| {
            
            if filter_redirected && m.redirected {
                debug!("Skipping redirected mission");
            } else {
                let key = (m.faction, m.target_system, m.target_faction);
                let count = m.count;
                let current_count = missions_summary.entry(key).or_insert(0);
                *current_count += count;
                remaining_active_misisons += 1;
            }
        });
        
        Ok((missions_summary, remaining_active_misisons, total_missions))
    }
    
    
}

impl EliteDangerousEventProcessor for PirateMassacrePlugin {
    async fn process_event(
        &self,
        event: Arc<EliteDangerousEvent>,
    ) -> std::result::Result<(), String> {
        self.event_processor.process_event(event).await
    }
}
