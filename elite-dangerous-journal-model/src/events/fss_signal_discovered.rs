
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::events::EventMeta;

/// Enum to represent the signal type
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FssSignalType {
    /// The signal is a fleet carrier
    FleetCarrier,
    /// The signal is a Resource Extraction location
    ResourceExtraction,
    /// The signal is a Codex signal
    Codex,
    /// The signal is an Installation
    Installation,
    /// The signal is a nav beacon
    NavBeacon,
    /// The signal is a tourist beacon
    TouristBeacon,
    /// The signal is a ONeil Cylinder type station
    StationONeilCylinder,
    /// The signal is an ONeil Orbis type station
    StationONeilOrbis,
    /// The signal is a Coriolis Station
    StationCoriolis,
    /// The signal is a Bernal Sphere station
    StationBernalSphere,
    /// The signal is a MegaShip station
    StationMegaShip,
    /// The signal is a combat site
    Combat,
    /// The signal is a mega ship
    #[serde(rename="Megaship")]
    MagaShip,
    /// The signal is an outpost
    Outpost,
    /// The signal is a Generic type
    Generic,
    /// The signal type has not yet been implemented but is captured as a string
    #[serde(untagged)]
    Unknown(String)
}

/// Event when a Full Spectrum Signal (FSS) is discovered
///
/// Example:
/// ```json
///{
///   "timestamp": "2025-01-04T19:27:38Z",
///   "event": "FSSSignalDiscovered",
///   "SystemAddress": 3107241104074,
///   "SignalName": "REIDEN LAKE V5M-WTH",
///   "SignalType": "FleetCarrier",
///   "IsStation": true
/// }
///
/// Example 2:
///
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FSSSignalDiscoveredEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The internal system address reference for the system
    pub system_address: u64,
    /// The Name of the signal
    pub signal_name: String,
    /// Localised version of signal name
    #[serde(rename = "SignalName_Localised")]
    pub localised_signal_name: Option<String>,
    /// The type of the signal
    pub signal_type: FssSignalType,
    /// Is the signal a station
    pub is_station: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::events::fss_signal_discovered::{FSSSignalDiscoveredEvent, FssSignalType};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_fss_signal_discovered() {
        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"REIDEN LAKE V5M-WTH", "SignalType":"FleetCarrier", "IsStation":true }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name, String::from("REIDEN LAKE V5M-WTH"));
        assert_eq!(event.signal_type, FssSignalType::FleetCarrier);
        assert_eq!(event.localised_signal_name, None);
        assert_eq!(event.is_station, Some(true));
    }

    #[test]
    fn test_deserialize_fss_signal_discovered_example2() {

        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"$MULTIPLAYER_SCENARIO78_TITLE;", "SignalName_Localised":"Resource Extraction Site [High]", "SignalType":"ResourceExtraction" }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name, "$MULTIPLAYER_SCENARIO78_TITLE;");
        assert_eq!(event.signal_type, FssSignalType::ResourceExtraction);
        assert_eq!(event.is_station, None);
        assert_eq!(event.localised_signal_name, Some(String::from("Resource Extraction Site [High]")));
    }

    #[test]
    fn test_deserialize_fss_signal_discovered_unknown_signal_type() {
        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"REIDEN LAKE V5M-WTH", "SignalType":"WibbleFish", "IsStation":true }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name, String::from("REIDEN LAKE V5M-WTH"));
        assert_eq!(event.signal_type, FssSignalType::Unknown(String::from("WibbleFish")));
        assert_eq!(event.is_station, Some(true));
        assert_eq!(event.localised_signal_name, None);
    }
}