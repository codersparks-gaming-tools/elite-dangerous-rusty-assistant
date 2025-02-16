use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tracing::{debug, error, trace};
use elite_dangerous_journal_model::events::EliteDangerousEvent;
use elite_dangerous_journal_model::events::JournalEvent::{MissionAbandoned, MissionAccepted, MissionCompleted, MissionFailed, MissionRedirected};
use crate::components::pirate_massacre::model::PirateMassacreMission;
use crate::traits::EDRAComponent;

pub const DATABASE_NAMESPACE: &str = "pirate_massacre";
pub const DATABASE_NAME: &str = "missions";
pub const RECORD_NAME: &str = "mission";

pub struct PirateMassacreComponent {
    db: Arc<Surreal<Db>>,
}

impl EDRAComponent for PirateMassacreComponent {
    fn name(&self) -> String {
        "Pirate Massacre Mission Helper".to_string()
    }

    fn description(&self) -> String {
        "Keeps track of pirate massacre missions".to_string()
    }

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

    async fn render(&self) {
        

        let missions = self.get_mission_summary_by_faction(true).await;

        match missions {
            Ok((missions, remaining_mission_count, total_mission_count)) => {
                println!("--------------- Remaining Missions: {} Total: {} ---------------------", remaining_mission_count, total_mission_count);

                missions.iter().for_each(|s| {
                    let ((faction, target_system, target_faction), count) = s;
                    println!("Target System: {}, Target Faction: {}, Faction: {}, Count {}", target_system, target_faction, faction, count );
                });
                println!("--------------------------------------------------------------------");
            }
            Err(e) => {  error!("Failed to get missions: {}", e); }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
    }
}
impl PirateMassacreComponent {
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

    pub async fn get_all_missions(&self) -> Result<Vec<PirateMassacreMission>, String> {
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

    pub async fn get_mission_summary_by_faction(&self, filter_redirected:bool) -> Result<(HashMap<(String, String, String), u64>, u64, u64), String> {

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