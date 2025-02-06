use crate::pirate_massacre_plugin::model::PirateMassacreMission;
use crate::pirate_massacre_plugin::{DATABASE_NAME, DATABASE_NAMESPACE, RECORD_NAME};
use crate::EliteDangerousEventProcessor;
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use elite_dangerous_journal_model::events::JournalEvent::{
    MissionAbandoned, MissionAccepted, MissionCompleted, MissionFailed, MissionRedirected,
};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tracing::{debug, trace};

pub struct PirateMassacreEventProcessor {
    db: Arc<Surreal<Db>>,
}

impl EliteDangerousEventProcessor for PirateMassacreEventProcessor {
    async fn process_event(&self, event: Arc<EliteDangerousEvent>) -> Result<(), String> {
        match event.as_ref() {
            EliteDangerousEvent::JournalEvent(je) => match je {
                MissionAbandoned(ma) => {
                    debug!("Processing mission abandoned: {:?}", ma);
                    Ok(())
                }
                MissionAccepted(ma) => {
                    debug!("Processing mission accepted: {:?}", ma);
                    if ma.name.value.starts_with("Mission_Massacr") {
                        let pmm = PirateMassacreMission {
                            mission_id: ma.mission_id.clone(),
                            faction: ma.faction.clone(),
                            target_type: ma.target_type.clone().unwrap().value,
                            target_faction: ma.target_faction.clone().unwrap(),
                            target_system: ma.destination_system.clone().unwrap(),
                            target_station: ma.destination_station.clone().unwrap(),
                            count: ma.kill_count.unwrap(),
                            redirected: false,
                        };
                        self.add_mission_to_store(pmm).await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                MissionCompleted(mc) => {
                    debug!("Mission completed: {:?}", mc);
                    if mc.name.value.starts_with("Mission_Massacr") {
                        self.remove_mission_from_store(mc.mission_id).await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                MissionFailed(mf) => {
                    debug!("Mission failed: {:?}", mf);
                    if mf.name.value.starts_with("Mission_Massacr") {
                        self.remove_mission_from_store(mf.mission_id).await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                MissionRedirected(mr) => {
                    debug!("Mission redirected: {:?}", mr);
                    if mr.name.value.starts_with("Mission_Massacr") {
                        self.update_destination_system(
                            mr.mission_id,
                            &*mr.new_destination_system,
                            &*mr.new_destination_station,
                        )
                        .await
                    } else {
                        trace!("Not a massacre mission");
                        Ok(())
                    }
                }
                _ => {
                    trace!("Not interested in {:?}", je);
                    Ok(())
                }
            },
        }
    }
}
impl PirateMassacreEventProcessor {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }

    async fn remove_mission_from_store(&self, mission_id: u64) -> std::result::Result<(), String> {
        let db = &self.db;
        match db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await {
            Ok(_) => {
                let mission_deleted_result: Result<
                    Option<PirateMassacreMission>,
                    surrealdb::Error,
                > = db.delete((RECORD_NAME, mission_id.to_string())).await;
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

    async fn add_mission_to_store(
        &self,
        pmm: PirateMassacreMission,
    ) -> std::result::Result<(), String> {
        let db = &self.db;
        match db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await {
            Ok(_) => {
                let created: surrealdb::Result<Option<PirateMassacreMission>> = self
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

    pub async fn update_destination_system(
        &self,
        mission_id: u64,
        new_destination_system: &str,
        new_destination_station: &str,
    ) -> std::result::Result<(), String> {
        let db = &self.db;
        match db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await {
            Ok(_) => {
                let res: Result<Option<PirateMassacreMission>, surrealdb::Error> =
                    self.db.select((RECORD_NAME, mission_id.to_string())).await;
                match res {
                    Ok(Some(mut pmm)) => {
                        pmm.target_system = new_destination_system.to_string();
                        pmm.target_station = new_destination_station.to_string();
                        pmm.redirected = true;
                        debug!("Updated record: {:?}", pmm);
                        let res: Result<Option<PirateMassacreMission>, surrealdb::Error> =
                            self.db.update((RECORD_NAME, mission_id.to_string())).content(pmm).await;
                        match res {
                            Ok(_) => {
                                debug!("Data store updated");
                                Ok(())
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    Ok(None) => return Err(format!("Mission {mission_id} not found")),
                    Err(e) => return Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
