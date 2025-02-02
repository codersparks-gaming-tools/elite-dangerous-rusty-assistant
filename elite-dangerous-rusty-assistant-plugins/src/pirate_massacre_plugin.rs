mod model;

use std::collections::HashMap;
use elite_dangerous_journal_model::events::station_services::missions::mission_accepted::MissionAcceptedEvent;
use elite_dangerous_journal_model::events::station_services::missions::mission_completed::MissionCompletedEvent;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use elite_dangerous_journal_model::events::JournalEvent::{
    MissionAbandoned, MissionAccepted, MissionCompleted, MissionFailed, MissionRedirected,
};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb::Result;
use tracing::{debug, trace};
use crate::EliteDangerousEventProcessor;
use crate::pirate_massacre_plugin::model::PirateMassacreMission;




const DATABASE_NAMESPACE: &str = "pirate_massacre_plugin";
const DATABASE_NAME: &str = "data";
const RECORD_NAME: &str = "massacre_mission";

pub struct PirateMassacrePlugin {
    db: Arc<Surreal<Db>>,
}

impl PirateMassacrePlugin {
    pub async fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }

    async fn process_mission_completed_event(
        &self,
        event: &MissionCompletedEvent,
    ) -> std::result::Result<(), String> {
        let db = &self.db;
        match db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await {
            Ok(_) => {
                let mission_deleted_result: Result<Option<PirateMassacreMission>> =
                    db.delete((RECORD_NAME, event.mission_id.to_string())).await;
                match mission_deleted_result {
                    Ok(record) => {
                        if let Some(record) = record {
                            debug!("Deleted record: {:?}", record);
                        }
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn process_mission_accepted_event<'a>(
        &'a self,
        event: &'a MissionAcceptedEvent,
    ) -> std::result::Result<(), String> {
        let db = &self.db;
        match db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await {
            Ok(_) => {
                let pmm = PirateMassacreMission {
                    mission_id: event.mission_id.clone(),
                    faction: event.faction.clone(),
                    target_type: event.target_type.clone().unwrap().value,
                    target_faction: event.target_faction.clone().unwrap(),
                    target_system: event.destination_system.clone().unwrap(),
                    target_station: event.destination_station.clone().unwrap(),
                    count: event.kill_count.unwrap(),
                };

                let created: Result<Option<PirateMassacreMission>> = self
                    .db
                    .create((RECORD_NAME, pmm.mission_id.to_string()))
                    .content(pmm)
                    .await;

                match created {
                    Ok(record) => {
                        if let Some(record) = record {
                            debug!("Created record: {:?}", record);
                        }
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
    
    pub async fn get_all_missions(&self) -> std::result::Result<Vec<PirateMassacreMission>, String> {
        self.db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await.unwrap();
        
        let select_result :Result<Vec<PirateMassacreMission>> = self.db.select(RECORD_NAME).await;
        
        match select_result { 
            Ok(records) => {
                Ok(records)
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
    
    pub async fn get_mission_summary_by_faction(&self) -> std::result::Result<HashMap<(String, String, String), u64>, String> {
        
        let missions = self.get_all_missions().await?;
        
        let mut missions_summary: HashMap<(String, String, String), u64> = HashMap::new();
        
        missions.into_iter().for_each(|m| {
            let key = (m.faction, m.target_system, m.target_faction);
            let count = m.count;
            let current_count = missions_summary.entry(key).or_insert(0);
            *current_count += count;
        });
        
        Ok(missions_summary)
    }
}

impl EliteDangerousEventProcessor for PirateMassacrePlugin {
    async fn process_event(
        &self,
        event: Arc<EliteDangerousEvent>,
    ) -> std::result::Result<(), String> {
        match event.as_ref() {
            EliteDangerousEvent::JournalEvent(je) => match je {
                MissionAbandoned(ma) => {
                    trace!("Processing mission abandoned: {:?}", ma);
                    Ok(())
                }
                MissionAccepted(ma) => {
                    trace!("Processing mission accepted: {:?}", ma);
                    if ma.name.value.starts_with("Mission_Massacr") {
                        self.process_mission_accepted_event(ma).await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                MissionCompleted(mc) => {
                    trace!("Mission completed: {:?}", mc);
                    if mc.name.value.starts_with("Mission_Massacr") {
                        self.process_mission_completed_event(mc).await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                MissionFailed(mf) => {
                    trace!("Mission failed: {:?}", mf);
                    Ok(())
                }
                MissionRedirected(mr) => {
                    trace!("Mission redirected: {:?}", mr);
                    Ok(())
                }
                _ => {
                    trace!("Not interested in {:?}", je);
                    Ok(())
                }
            },
        }
    }
}
