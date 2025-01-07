use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

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
    /// Time of the event
    #[serde_as(as = "chrono::DateTime<chrono::Utc>")]
    pub timestamp: NaiveDateTime,
    /// Part field - Not sure what this is used for yet
    pub part: u8,
    /// The language used
    pub language: String,
    /// Does the game have odyssey enabled
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    /// The version of the game
    #[serde(rename = "gameversion")]
    pub game_version: String,
    /// The build of the game
    pub build: String
}

#[cfg(test)]
mod tests {
    use crate::events::file_header::FileHeaderEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize(){
        let timestamp_str = "2025-01-04T18:57:24Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Fileheader", "part":1, "language":"English/UK", "Odyssey":true, "gameversion":"4.0.0.1904", "build":"r308767/r0 " }}"#);

        let event: FileHeaderEvent = serde_json::from_str(json.as_str()).unwrap();

        assert_eq!(event.part, 1);
        assert_eq!(event.language, "English/UK");
        assert_eq!(event.odyssey, true);
        assert_eq!(event.game_version, "4.0.0.1904");
        assert_eq!(event.build, "r308767/r0 ");
        assert_eq!(event.timestamp, timestamp);
    }
}