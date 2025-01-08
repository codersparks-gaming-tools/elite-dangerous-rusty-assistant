
//! Provide model structs for use with parsing of the elite dangerous journal logs


/// The module containing the struct implementations for each of the events
pub mod events;

#[cfg(test)]
mod test_helper;

use serde::{Deserialize, Serialize};
use crate::events::commander::{CommanderEvent, CommanderProgressEvent, CommanderRankEvent, CommanderReputationEvent};
use crate::events::engineer::EngineerProgressEvent;
use crate::events::fss_signal_discovered::FSSSignalDiscoveredEvent;
use crate::events::game::{FileHeaderEvent, LoadGameEvent};
use crate::events::material::MaterialsEvent;

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
    /// This is a catch all for any other event in the file to allow iterative development and also should frontier add an event in the future
    #[serde(other)]
    Unknown,
}