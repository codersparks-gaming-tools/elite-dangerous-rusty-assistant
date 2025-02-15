use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use crate::events::common::EventMeta;

/// Event for FSS Discovery Scan
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSDiscoveryScanEvent {
    /// Event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The progress of the scan (shows how completely the system has been scanned
    pub progress: f64,
    /// The number of stellar bodies
    pub body_count: u32,
    /// The number of non stellar bodies
    pub non_body_count: u32, 
}