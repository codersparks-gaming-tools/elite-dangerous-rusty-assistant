use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;

/// FSSAllBodiesFound Event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSAllBodiesFoundEvent {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The name of the system
    pub system_name: String,
    /// The address of the system
    pub system_address: u64,
    /// The number of bodies
    pub count: u32,
}