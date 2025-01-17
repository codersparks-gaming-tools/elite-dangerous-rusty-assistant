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
use crate::events::odyssey::ship_locker::ShipLockerEvent;
use crate::events::other::fuel_scoop::FuelScoopEvent;
use crate::events::other::receive_text::ReceiveTextEvent;
use crate::events::travel::start_jump::StartJumpEvent;

/// The journal event enum allows the deserialisation of the events from the elite dangerous journal log
/// It uses the ```event``` json field to determine what enum variant to load and then completes the data
/// for the relevant struct
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event")]
pub enum JournalEvent {
    /// Fileheader event
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    /// Commander event
    Commander(CommanderEvent),
    /// Materials event
    Materials(MaterialsEvent),
    /// Commander Rank event
    Rank(CommanderRankEvent),
    /// Progress Rank event
    Progress(CommanderProgressEvent),
    /// Reputation event,
    Reputation(CommanderReputationEvent),
    /// FSS Signal Discovered event
    FSSSignalDiscovered(FSSSignalDiscoveredEvent),
    /// Game Load event
    LoadGame(LoadGameEvent),
    /// Engineer event
    EngineerProgress(EngineerProgressEvent),
    /// Music event
    Music(MusicEvent),
    /// Scan event
    Scan(ScanEvent),
    /// NavRoute event - This is an empty event
    NavRoute(EmptyEvent),
    /// NavRouteClear event - This is an empty event
    NavRouteClear(EmptyEvent),
    /// FSDTarget event
    FSDTarget(FSDTargetEvent),
    /// ShipLocker event
    ShipLocker(ShipLockerEvent),
    /// Fuel Scoop event
    FuelScoop(FuelScoopEvent),
    /// Receive text event
    ReceiveText(ReceiveTextEvent),
    /// StartJump event
    StartJump(StartJumpEvent),
    /// Game shutdown event
    #[serde(rename= "Shutdown")]
    ShutDown(EmptyEvent),
    /// This is a catch all for any other event in the file to allow iterative development and also should frontier add an event in the future
    #[serde(other)]
    Unknown,
}