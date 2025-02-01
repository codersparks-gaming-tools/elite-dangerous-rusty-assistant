/// Event that occurs when commander accepts a mission
pub mod mission_accepted;

/// Event that occurs when commander abandons a mission
pub mod mission_abandoned;

/// Event when mission is redirected
pub mod mission_redirected;
/// Mission completed event
pub mod mission_completed;
/// Common code for mission events
pub mod common;

/// Mission failed event
pub mod mission_failed;