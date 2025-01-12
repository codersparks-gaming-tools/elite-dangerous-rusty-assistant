use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;

/// What this music event is for
#[derive(Debug,Serialize, Deserialize, Clone, PartialEq)]
pub enum Music {
    /// Music track for codex
    Codex,
    /// Music track for dogfighting
    #[serde(rename="Combat_Dogfight")]
    CombatDogfight,
    /// Arrival at destination from hyperspace jump
    DestinationFromHyperspace,
    /// Arrival at destination from being in supercruise
    DestinationFromSupercruise,
    /// Docking/launching with docking computer
    DockingComputer,
    /// Exploration track
    Exploration,
    /// In the galaxy map
    GalaxyMap,
    /// In the main menu
    MainMenu,
    /// No track has been set
    NoTrack,
    /// In starport
    Starport,
    /// In supercruise
    Supercruise,
    /// Scanning
    SystemAndSurfaceScanner,
    /// In system map
    SystemMap,
    /// Encounter with unknown
    #[serde(rename="Unknown_Encounter")]
    UnknownEncounter,
    /// Unknown in exploration
    #[serde(rename="Unknown_Exploration")]
    UnknownExploration,
    /// Catch all for other events
    #[serde(untagged)]
    Unknown(String),

}

/// A music event
#[derive(Debug,Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MusicEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The track that has been played
    pub music_track: Music,

}

#[cfg(test)]
mod tests {
    use crate::events::other::music::{Music, MusicEvent};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    pub fn test_music_combat_dogfight() {
        let timestamp_str = "2025-01-04T19:44:55Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Music", "MusicTrack":"Combat_Dogfight" }}"#);

        let event: MusicEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.music_track, Music::CombatDogfight);
        assert_eq!(event.event_meta.timestamp, timestamp);
    }

    #[test]
    pub fn test_music_main_menu() {
        let timestamp_str = "2025-01-04T19:44:55Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Music", "MusicTrack":"MainMenu" }}"#);

        let event: MusicEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.music_track, Music::MainMenu);
        assert_eq!(event.event_meta.timestamp, timestamp);
    }

    #[test]
    pub fn test_music_unknown_track() {

        let timestamp_str = "2025-01-04T19:44:55Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Music", "MusicTrack":"WibbleFish" }}"#);

        let event: MusicEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.music_track, Music::Unknown("WibbleFish".to_string()));
        assert_eq!(event.event_meta.timestamp, timestamp);
    }
}