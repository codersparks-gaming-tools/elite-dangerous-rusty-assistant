
use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(from = "FSSSignalDiscoveredEventSchema", into = "FSSSignalDiscoveredEventSchema")]
pub struct FSSSignalDiscoveredEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The internal system address reference for the system
    pub system_address: u64,
    /// The Name of the signal
    pub signal_name: LocalisedValue,
    /// The type of the signal
    pub signal_type: FssSignalType,
    /// Is the signal a station
    pub is_station: Option<bool>,
}

impl From<FSSSignalDiscoveredEventSchema> for FSSSignalDiscoveredEvent {
    fn from(value: FSSSignalDiscoveredEventSchema) -> Self {
        let signal_name = LocalisedValue {
            value: value.signal_name,
            localised_value: value.localised_signal_name,
        };

        Self {
            event_meta: value.event_meta,
            system_address: value.system_address,
            signal_name,
            signal_type: value.signal_type,
            is_station: value.is_station,
        }
    }
}


/// Schema struct for when a Full Spectrum Signal (FSS) is discovered
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FSSSignalDiscoveredEventSchema {
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

impl From<FSSSignalDiscoveredEvent> for FSSSignalDiscoveredEventSchema {
    fn from(value: FSSSignalDiscoveredEvent) -> Self {
        Self {
            event_meta: value.event_meta,
            system_address: value.system_address,
            signal_name: value.signal_name.value,
            localised_signal_name: value.signal_name.localised_value,
            signal_type: value.signal_type,
            is_station: value.is_station,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::exploration::fss_signal_discovered::{FSSSignalDiscoveredEvent, FssSignalType};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_fss_signal_discovered() {
        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"REIDEN LAKE V5M-WTH", "SignalType":"FleetCarrier", "IsStation":true }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name.value, String::from("REIDEN LAKE V5M-WTH"));
        assert_eq!(event.signal_type, FssSignalType::FleetCarrier);
        assert_eq!(event.signal_name.localised_value, None);
        assert_eq!(event.is_station, Some(true));
    }

    #[test]
    fn test_deserialize_fss_signal_discovered_example2() {

        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"$MULTIPLAYER_SCENARIO78_TITLE;", "SignalName_Localised":"Resource Extraction Site [High]", "SignalType":"ResourceExtraction" }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name.value, "$MULTIPLAYER_SCENARIO78_TITLE;");
        assert_eq!(event.signal_type, FssSignalType::ResourceExtraction);
        assert_eq!(event.is_station, None);
        assert_eq!(event.signal_name.localised_value, Some(String::from("Resource Extraction Site [High]")));
    }

    #[test]
    fn test_deserialize_fss_signal_discovered_unknown_signal_type() {
        let timestamp_str = "2025-01-04T19:27:38Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FSSSignalDiscovered", "SystemAddress":3107241104074, "SignalName":"REIDEN LAKE V5M-WTH", "SignalType":"WibbleFish", "IsStation":true }}"#);

        let event: FSSSignalDiscoveredEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.signal_name.value, String::from("REIDEN LAKE V5M-WTH"));
        assert_eq!(event.signal_type, FssSignalType::Unknown(String::from("WibbleFish")));
        assert_eq!(event.is_station, Some(true));
        assert_eq!(event.signal_name.localised_value, None);
    }
}