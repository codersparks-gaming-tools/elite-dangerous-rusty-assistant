use serde::{Deserialize, Serialize};

/// Meta data about the ship
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ShipMeta {
    /// The ship being described (type)
    pub ship: String,
    /// The ID of the ship being described
    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    /// The name of the ship
    pub ship_name: String,
    /// The in-game identity (e.g. "NCC-1701")
    pub ship_ident: String
}