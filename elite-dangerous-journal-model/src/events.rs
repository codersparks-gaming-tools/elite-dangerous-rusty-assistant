//! This module contains the structs that are used to represent the events written to the relevant files in
//! the comander data folder - Normally:
//! ```text
//! C:\Users\%userprofile%\Saved Games\Frontier Developments\Elite Dangerous
//! ```
//!
//! The events try to follow the structure defined in [Elite Dangerous Player Journal](https://elite-journal.readthedocs.io/en/latest/) docs
//! however the locations may be tweaked if considered more sensible (or they are not structured in the docs)

use serde::{Deserialize, Serialize};
use crate::events::common::EmptyEvent;
use crate::events::exploration::fss_signal_discovered::FSSSignalDiscoveredEvent;
use crate::events::exploration::material_collected::MaterialCollectedEvent;
use crate::events::exploration::scan::event::ScanEvent;
use crate::events::odyssey::ship_locker::ShipLockerEvent;
use crate::events::other::drone::LaunchDroneEvent;
use crate::events::other::fuel_scoop::FuelScoopEvent;
use crate::events::other::music::MusicEvent;
use crate::events::other::receive_text::ReceiveTextEvent;
use crate::events::startup::cargo::CargoEvent;
use crate::events::startup::commander::{CommanderEvent, CommanderProgressEvent, CommanderRankEvent, CommanderReputationEvent};
use crate::events::startup::game::{FileHeaderEvent, LoadGameEvent};
use crate::events::startup::material::MaterialsEvent;
use crate::events::station_services::engineer::EngineerProgressEvent;
use crate::events::trade::mining::MiningRefinedEvent;
use crate::events::travel::fsd_jump::fsd_jump_event::FSDJumpEvent;
use crate::events::travel::fsd_target::FSDTargetEvent;
use crate::events::travel::start_jump::StartJumpEvent;

/// A module for common structures for example the serde processing of timestamps
pub mod common;

/// Events that are emitted at start up - https://elite-journal.readthedocs.io/en/latest/Startup/
pub mod startup;

/// Events that are emitted related to travel - https://elite-journal.readthedocs.io/en/latest/Travel/
pub mod travel;

/// Events that are emitted related to combat - https://elite-journal.readthedocs.io/en/latest/Combat/
pub mod combat;

/// Events that are emitted related to exploration - https://elite-journal.readthedocs.io/en/latest/Exploration/
pub mod exploration;

/// Events that are emitted related to trade - https://elite-journal.readthedocs.io/en/latest/Trade/
pub mod trade;

/// Events that are emitted related to station services - https://elite-journal.readthedocs.io/en/latest/Station%20Services/
pub mod station_services;

/// Events that are emitted related to powerplay - https://elite-journal.readthedocs.io/en/latest/Powerplay/
pub mod powerplay;

/// Events that are emitted related to squadrons - https://elite-journal.readthedocs.io/en/latest/Squadrons/
pub mod squadrons;

/// Events that are emitted related to fleet carriers - https://elite-journal.readthedocs.io/en/latest/Fleet%20Carriers/
pub mod fleet_carriers;

/// Events that are emitted that have been added in odyssey - https://elite-journal.readthedocs.io/en/latest/New%20in%20Odyssey/
pub mod odyssey;

/// Events that are emitted that have no other home - https://elite-journal.readthedocs.io/en/latest/Other%20Events/
pub mod other;

/// As added in E::D 3 there is a status file that is updated regularly - https://elite-journal.readthedocs.io/en/latest/Status%20File/
pub mod status;

/// The EliteDangerousEvent is the event that consumers will receive, it consist of variants based on the 
/// source of the event
#[derive(Debug)]
pub enum EliteDangerousEvent {
    /// Event has come from the journal log i.e. file pattern: Journal.%Y-%M-%dT%H%M%S.(cc).log
    /// All the %_ initials can be interpreted from https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    /// The (cc) represents the file number
    /// for example:
    /// 
    /// ```text
    /// Journal.2025-01-24T172047.01.log
    /// ```
    JournalEvent(JournalEvent),
}


/// The journal event enum allows the deserialisation of the events from the elite dangerous journal log
/// It uses the ```event``` json field to determine what enum variant to load and then completes the data
/// for the relevant struct
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event")]
pub enum JournalEvent {
    Cargo(CargoEvent),
    Commander(CommanderEvent),
    EngineerProgress(EngineerProgressEvent),
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    FSDJump(FSDJumpEvent),
    FSDTarget(FSDTargetEvent),
    FSSSignalDiscovered(FSSSignalDiscoveredEvent),
    FuelScoop(FuelScoopEvent),
    LaunchDrone(LaunchDroneEvent),
    LoadGame(LoadGameEvent),
    MaterialCollected(MaterialCollectedEvent),
    Materials(MaterialsEvent),
    MiningRefined(MiningRefinedEvent),
    Music(MusicEvent),
    NavRoute(EmptyEvent),
    NavRouteClear(EmptyEvent),
    Progress(CommanderProgressEvent),
    Rank(CommanderRankEvent),
    ReceiveText(ReceiveTextEvent),
    Reputation(CommanderReputationEvent),
    Scan(ScanEvent),
    ShipLocker(ShipLockerEvent),
    StartJump(StartJumpEvent),
    #[serde(rename= "Shutdown")]
    ShutDown(EmptyEvent),
    /// This is a catch all for any other event in the file to allow iterative development and also should frontier add an event in the future
    #[serde(other)]
    Unknown,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::events::startup::cargo::CargoVessel;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_serialize_journal_event_cargo() {

        let timestamp_str = "2025-01-13T18:05:28Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Cargo", "Vessel":"Ship", "Count":29, "Inventory":[ {{ "Name":"damagedescapepod", "Name_Localised":"Damaged Escape Pod", "Count":1, "Stolen":0 }}, {{ "Name":"drones", "Name_Localised":"Limpet", "Count":28, "Stolen":0 }} ] }}"#);


        let event: JournalEvent = serde_json::from_str(&json).expect("Failed to deserialize CargoEvent");

        // We clone so that we can reuse event below
        match event.clone() {

            JournalEvent::Cargo(cargo_event) => {
                assert_eq!(cargo_event.event_meta.timestamp, timestamp);
                assert_eq!(cargo_event.count, 29);
                assert_eq!(cargo_event.vessel, CargoVessel::Ship);

                if let Some(inventory) = &cargo_event.inventory {
                    assert_eq!(inventory.len(), 2);

                    assert_eq!(inventory[0].name.value, "damagedescapepod");
                    assert_eq!(inventory[0].name.localised_value, Some("Damaged Escape Pod".to_string()));
                    assert_eq!(inventory[0].count, 1);
                    assert_eq!(inventory[0].stolen, 0);

                    assert_eq!(inventory[1].name.value, "drones");
                    assert_eq!(inventory[1].name.localised_value, Some("Limpet".to_string()));
                    assert_eq!(inventory[1].count, 28);
                    assert_eq!(inventory[1].stolen, 0);
                } else {
                    panic!("CargoEvent inventory was empty");
                }
            },
            _ => panic!("Expected CargoEvent but got a different event")

        }


        // Serialize the event to JSON
        let serialized = serde_json::to_string(&event).expect("Failed to serialize CargoEvent");
        assert!(serialized.contains("\"event\":\"Cargo\""));
        assert!(serialized.contains("\"Vessel\":\"Ship\""));
        assert!(serialized.contains("\"Count\":29"));
        assert!(serialized.contains("\"Inventory\":["));
        assert!(serialized.contains("\"Name\":\"damagedescapepod\""));
        assert!(serialized.contains("\"Name_Localised\":\"Damaged Escape Pod\""));
        assert!(serialized.contains("\"Count\":1"));
        assert!(serialized.contains("\"Stolen\":0"));
        assert!(serialized.contains("\"Name\":\"drones\""));
        assert!(serialized.contains("\"Name_Localised\":\"Limpet\""));
        assert!(serialized.contains("\"Count\":28"));
        assert!(serialized.contains("\"Stolen\":0"));
        assert!(serialized.contains("\"timestamp\":\"2025-01-13T18:05:28Z\""));
    }




}


