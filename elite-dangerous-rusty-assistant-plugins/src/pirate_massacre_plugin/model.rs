use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PirateMassacreMission {
    
    pub mission_id: u64,
    pub faction: String,
    pub target_type: String,
    pub target_faction: String,
    pub target_system: String,
    pub target_station: String,
    pub count: u64,
    pub redirected: bool,
    
}