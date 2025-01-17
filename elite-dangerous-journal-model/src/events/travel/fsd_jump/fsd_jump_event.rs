use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;
use crate::events::travel::fsd_jump::star_position::StarPosition;

/// FSD Jump event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct FSDJumpEvent {

    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The name of the Star System
    pub star_system: String,

    /// The address of the system
    pub system_address: u64,

    /// The position of the star
    pub star_pos: StarPosition,

    /// The econompy of the system
    pub system_economy: String,

    /// The localised version of the economy
    #[serde(rename="SystemEconomy_Localised")]
    pub system_economy_localised: String,

    /// System second economy
    pub system_second_economy: String,

    /// The localised version of the economy
    #[serde(rename="SystemSecondEconomy_Localised")]
    pub system_second_ecomomy_localised: String,

    /// System government type
    pub system_government: String,

    /// The localised version of the economy
    #[serde(rename="SystemGovernment_Localised")]
    pub system_government_localised: String,

    /// System security
    pub system_security: String,

    /// Localised version of system security
    #[serde(rename="SystemSecurity_Localised")]
    pub system_security_localised: String,

    /// Population of the system
    pub population: u64,

    /// The distance of the jump
    pub jump_dist: f32,

    /// Amount of fuel used
    pub fuel_used: f32,

    /// Fuel level
    pub fuel_level: f32,

    /// Taxi
    pub taxi: bool,

    /// Multi Crew
    #[serde(rename="Multicrew")]
    pub multi_crew: bool,

}