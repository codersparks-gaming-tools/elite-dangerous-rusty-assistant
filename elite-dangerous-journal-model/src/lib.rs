#![warn(missing_docs)]
//! Provide model structs for use with parsing of the elite dangerous journal logs


/// The module containing the struct implementations for each of the events
pub mod events;

#[cfg(test)]
mod test_helper;

use serde::{Deserialize, Serialize};
use events::startup::commander::{CommanderEvent, CommanderProgressEvent, CommanderRankEvent, CommanderReputationEvent};
use events::station_services::engineer::EngineerProgressEvent;
use events::exploration::fss_signal_discovered::FSSSignalDiscoveredEvent;
use events::startup::game::{FileHeaderEvent, LoadGameEvent};
use events::startup::material::MaterialsEvent;
use events::other::music::MusicEvent;
use events::exploration::scan::event::ScanEvent;
use events::travel::fsd_target::FSDTargetEvent;
use crate::events::common::EmptyEvent;
use crate::events::exploration::material_collected::MaterialCollectedEvent;
use crate::events::odyssey::ship_locker::ShipLockerEvent;
use crate::events::other::drone::LaunchDroneEvent;
use crate::events::other::fuel_scoop::FuelScoopEvent;
use crate::events::other::receive_text::ReceiveTextEvent;
use crate::events::startup::cargo::CargoEvent;
use crate::events::travel::fsd_jump::fsd_jump_event::FSDJumpEvent;
use crate::events::travel::start_jump::StartJumpEvent;

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