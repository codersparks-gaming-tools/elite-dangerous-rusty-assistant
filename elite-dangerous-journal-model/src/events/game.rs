
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::events::EventMeta;
use crate::events::ship::ShipMeta;

/// A struct to provide game meta data
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GameMeta {
    pub(crate) language: String,
    #[serde(rename = "gameversion")]
    pub(crate) game_version: String,
    pub(crate) build: String,
}

/// Game Mode Type
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum GameMode {
    /// User has started a solo session
    Solo,
    /// User has started/joined a group session - Group will be named in ```Group``` field
    Group,
    /// User has joined an open session
    Open,
    /// Catch all for unknown types
    #[serde(other)]
    Other,
}

/// A game load event
///
/// Exmaple:
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "LoadGame",
///   "FID": "F00000000",
///   "Commander": "ANON",
///   "Horizons": true,
///   "Odyssey": true,
///   "Ship": "Anaconda",
///   "ShipID": 5,
///   "ShipName": "",
///   "ShipIdent": "",
///   "FuelLevel": 32.000000,
///   "FuelCapacity": 32.000000,
///   "GameMode": "Group",
///   "Group": "group_name",
///   "Credits": 53038302,
///   "Loan": 0,
///   "language": "English/UK",
///   "gameversion": "4.0.0.1904",
///   "build": "r308767/r0 "
/// }
/// ```
///
/// **note**: Group is only specified if GameMode is ```Group```. Not currently validated
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {

    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// Is horizons enabled for this game
    horizons: bool,

    /// Is Odyssey enabled for this game
    odyssey: bool,

    /// The unique internal identifier of the commander
    #[serde(rename = "FID")]
    pub fid: String,
    /// The commanders name
    pub commander: String,

    /// Default ship meta being loaded
    #[serde(flatten)]
    pub ship: ShipMeta,

    /// The current fuel level of the ship
    pub fuel_level: f32,

    /// The fuel capacity of the ship
    pub fuel_capacity: f32,

    /// Game mode type started
    pub game_mode: GameMode,

    /// Group name (only specified if game mode is ```Group```
    pub group: Option<String>,

    /// Credit user has
    pub credits: i64,

    /// Any loan user has
    pub loan: u64,

    /// Game meta data
    #[serde(flatten)]
    pub game_meta: GameMeta,

}


/// Each log file has a header, this is the representation of that event
///
/// Example:
/// ```json
/// {
///   "timestamp": "2025-01-04T18:57:24Z",
///   "event": "Fileheader",
///   "part": 1,
///   "language": "English/UK",
///   "Odyssey": true,
///   "gameversion": "4.0.0.1904",
///   "build": "r308767/r0 "
/// }
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FileHeaderEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// Part field - Not sure what this is used for yet
    pub part: u8,
    /// Does the game have odyssey enabled
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,

    /// Game client meta dat
    #[serde(flatten)]
    pub game_meta: GameMeta,
}

#[cfg(test)]
mod tests {
    use crate::events::game::{GameMode, LoadGameEvent};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_load_game_event_solo() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let game_mode = "Solo";
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"LoadGame", "FID":"F00000000", "Commander":"ANON", "Horizons":true, "Odyssey":true, "Ship":"Anaconda", "ShipID":5, "ShipName":"", "ShipIdent":"", "FuelLevel":32.000000, "FuelCapacity":32.000000, "GameMode":"{game_mode}", "Credits":53038302, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event : LoadGameEvent  = serde_json::from_str(&json).unwrap();

        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.horizons, true);
        assert_eq!(event.odyssey, true);
        assert_eq!(event.fid, "F00000000");
        assert_eq!(event.commander, "ANON");
        assert_eq!(event.ship.ship, "Anaconda");
        assert_eq!(event.ship.ship_id, 5);
        assert_eq!(event.ship.ship_name, String::from(""));
        assert_eq!(event.ship.ship_ident, String::from(""));
        assert_eq!(event.fuel_capacity, 32.000000);
        assert_eq!(event.fuel_level, 32.000000);
        assert_eq!(event.game_mode, GameMode::Solo);
        assert!(event.group.is_none());
        assert_eq!(event.credits, 53038302);
        assert_eq!(event.loan, 0);
        assert_eq!(event.game_meta.game_version, "4.0.0.1904");
        assert_eq!(event.game_meta.build, "r308767/r0 ");
        assert_eq!(event.game_meta.language, "English/UK");
    }

    #[test]
    fn test_deserialize_load_game_event_open() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let game_mode = "Open";
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"LoadGame", "FID":"F00000000", "Commander":"ANON", "Horizons":true, "Odyssey":true, "Ship":"Anaconda", "ShipID":5, "ShipName":"", "ShipIdent":"", "FuelLevel":32.000000, "FuelCapacity":32.000000, "GameMode":"{game_mode}", "Credits":53038302, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event : LoadGameEvent  = serde_json::from_str(&json).unwrap();

        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.horizons, true);
        assert_eq!(event.odyssey, true);
        assert_eq!(event.fid, "F00000000");
        assert_eq!(event.commander, "ANON");
        assert_eq!(event.ship.ship, "Anaconda");
        assert_eq!(event.ship.ship_id, 5);
        assert_eq!(event.ship.ship_name, String::from(""));
        assert_eq!(event.ship.ship_ident, String::from(""));
        assert_eq!(event.fuel_capacity, 32.000000);
        assert_eq!(event.fuel_level, 32.000000);
        assert_eq!(event.game_mode, GameMode::Open);
        assert!(event.group.is_none());
        assert_eq!(event.credits, 53038302);
        assert_eq!(event.loan, 0);
        assert_eq!(event.game_meta.game_version, "4.0.0.1904");
        assert_eq!(event.game_meta.build, "r308767/r0 ");
        assert_eq!(event.game_meta.language, "English/UK");
    }

    #[test]
    fn test_deserialize_load_game_event_group() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let game_mode = "Solo";
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"LoadGame", "FID":"F00000000", "Commander":"ANON", "Horizons":true, "Odyssey":true, "Ship":"Anaconda", "ShipID":5, "ShipName":"", "ShipIdent":"", "FuelLevel":32.000000, "FuelCapacity":32.000000, "GameMode":"{game_mode}", "Group":"group_name", "Credits":53038302, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event : LoadGameEvent  = serde_json::from_str(&json).unwrap();

        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.horizons, true);
        assert_eq!(event.odyssey, true);
        assert_eq!(event.fid, "F00000000");
        assert_eq!(event.commander, "ANON");
        assert_eq!(event.ship.ship, "Anaconda");
        assert_eq!(event.ship.ship_id, 5);
        assert_eq!(event.ship.ship_name, String::from(""));
        assert_eq!(event.ship.ship_ident, String::from(""));
        assert_eq!(event.fuel_capacity, 32.000000);
        assert_eq!(event.fuel_level, 32.000000);
        assert_eq!(event.game_mode, GameMode::Solo);
        assert_eq!(event.group, Some(String::from("group_name")));
        assert_eq!(event.credits, 53038302);
        assert_eq!(event.loan, 0);
        assert_eq!(event.game_meta.game_version, "4.0.0.1904");
        assert_eq!(event.game_meta.build, "r308767/r0 ");
        assert_eq!(event.game_meta.language, "English/UK");
    }

    #[test]
    fn test_deserialize_load_game_event_wibble() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let game_mode = "Wibble";
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"LoadGame", "FID":"F00000000", "Commander":"ANON", "Horizons":true, "Odyssey":true, "Ship":"Anaconda", "ShipID":5, "ShipName":"", "ShipIdent":"", "FuelLevel":32.000000, "FuelCapacity":32.000000, "GameMode":"{game_mode}", "Credits":53038302, "Loan":0, "language":"English/UK", "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event : LoadGameEvent  = serde_json::from_str(&json).unwrap();

        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.horizons, true);
        assert_eq!(event.odyssey, true);
        assert_eq!(event.fid, "F00000000");
        assert_eq!(event.commander, "ANON");
        assert_eq!(event.ship.ship, "Anaconda");
        assert_eq!(event.ship.ship_id, 5);
        assert_eq!(event.ship.ship_name, String::from(""));
        assert_eq!(event.ship.ship_ident, String::from(""));
        assert_eq!(event.fuel_capacity, 32.000000);
        assert_eq!(event.fuel_level, 32.000000);
        assert_eq!(event.game_mode, GameMode::Other);
        assert!(event.group.is_none());
        assert_eq!(event.credits, 53038302);
        assert_eq!(event.loan, 0);
        assert_eq!(event.game_meta.game_version, "4.0.0.1904");
        assert_eq!(event.game_meta.build, "r308767/r0 ");
        assert_eq!(event.game_meta.language, "English/UK");
    }

    #[test]
    fn test_deserialize(){
        let timestamp_str = "2025-01-04T18:57:24Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Fileheader", "part":1, "language":"English/UK", "Odyssey":true, "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event: FileHeaderEvent = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(event.part, 1);
        assert_eq!(event.game_meta.language, "English/UK");
        assert_eq!(event.odyssey, true);
        assert_eq!(event.game_meta.game_version, "4.0.0.1904");
        assert_eq!(event.game_meta.build, "r308767/r0 ");
        assert_eq!(event.timestamp, timestamp);
    }
}