use serde::{Deserialize, Serialize};
use crate::events::EventMeta;

/// An event that show the target of a FSD Jump
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FSDTargetEvent {

    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The name of the system
    pub name: String,

    /// The System Address
    pub system_address: u64,

    /// The class of the star being jumped to
    pub star_class: String,

    /// Remaining jumps in the current route
    pub remaining_jumps_in_route: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::events::ship::fsd_target::FSDTargetEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_fsd_target() {

        let timestamp_str = "2025-01-06T21:10:52Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSDTarget", "Name":"LF 8 +16 41", "SystemAddress":251012319587, "StarClass":"F", "RemainingJumpsInRoute":3 }}"#);

        let event : FSDTargetEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.name, "LF 8 +16 41");
        assert_eq!(event.system_address, 251012319587);
        assert_eq!(event.star_class, String::from("F"));
        assert_eq!(event.remaining_jumps_in_route, Some(3));
    }
}