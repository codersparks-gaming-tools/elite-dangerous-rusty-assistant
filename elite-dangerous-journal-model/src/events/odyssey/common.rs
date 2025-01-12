use serde::{Deserialize, Serialize};
// TODO: Confirm actual field data types
/// Used in new odyssey events - Represents an item in a container (Backpack, ShipLocker etc)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    /// The name of the item
    pub name: String,
    /// The owner of the item - API Docs does not state if this is string or number, assuming it's commander id for now TODO
    #[serde(rename="OwnerID")]
    pub owner_id: String,
    /// The mission id (if relevant) - MissionID has been used in other fields and is an int so using that for now TODO
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u32>,
    /// The number in the container
    pub count: u32,

}