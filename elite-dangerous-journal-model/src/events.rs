//! This module contains the structs that are used to represent the events written to the relevant files in
//! the comander data folder - Normally:
//! ```text
//! C:\Users\%userprofile%\Saved Games\Frontier Developments\Elite Dangerous
//! ```
//!
//! The events try to follow the structure defined in [Elite Dangerous Player Journal](https://elite-journal.readthedocs.io/en/latest/) docs
//! however the locations may be tweaked if considered more sensible (or they are not structured in the docs)


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


