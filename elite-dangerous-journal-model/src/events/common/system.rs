use serde::{Deserialize, Serialize};

/// What is the allegiance of the system/faction etc
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all="PascalCase")]
pub enum Allegiance {
    /// Allegiance is with: Alliance
    Alliance,
    /// Allegiance is with: Empire
    Empire,
    /// Allegiance is with: Federation
    Federation,
    /// System is independant
    Independent,
}


/// Struct to represent faction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq )]
#[serde(rename_all="PascalCase")]
pub struct Faction {
    /// The name of the faction
    pub name: String,
    /// The state of the faction
    pub faction_state: String,
    /// The government type
    pub government: String,
    /// The influence of the faction,
    pub influence: f32,
    /// The allegiance of the faction
    pub allegiance: Allegiance,
    /// The happiness of the faction
    pub happiness: String,
    /// The localised happiness of the faction
    #[serde(rename="Happiness_Localised")]
    pub happiness_localised: String,
    /// Pilots reputation with faction
    pub my_reputation: f32,

}