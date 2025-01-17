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


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_serialize_deserialize_fsd_jump_event() {
        let event_meta = EventMeta {
            timestamp: create_timestamp("2023-11-05T12:00:00Z"),
        };

        let star_pos = StarPosition {
            x: 123.45,
            y: 678.90,
            z: -12.34,
        };

        let fsd_jump_event = FSDJumpEvent {
            event_meta,
            star_system: "Sol".to_string(),
            system_address: 1234567890123456789,
            star_pos,
            system_economy: "Agriculture".to_string(),
            system_economy_localised: "Agricultural".to_string(),
            system_second_economy: "Industrial".to_string(),
            system_second_ecomomy_localised: "Industrialised".to_string(),
            system_government: "Democracy".to_string(),
            system_government_localised: "Democratic".to_string(),
            system_security: "High".to_string(),
            system_security_localised: "Secure".to_string(),
            population: 7000000000,
            jump_dist: 12.34,
            fuel_used: 1.5,
            fuel_level: 15.0,
            taxi: false,
            multi_crew: true,
        };

        // Serialize the event to JSON
        let serialized = serde_json::to_string(&fsd_jump_event).expect("Failed to serialize FSDJumpEvent");

        // Deserialize the JSON back into an FSDJumpEvent
        let deserialized: FSDJumpEvent =
            serde_json::from_str(&serialized).expect("Failed to deserialize FSDJumpEvent");

        // Assert that the deserialized struct matches the original
        assert_eq!(fsd_jump_event, deserialized);
    }
}
