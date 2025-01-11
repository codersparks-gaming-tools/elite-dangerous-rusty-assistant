use serde::{Deserialize, Serialize};

/// What type of scan was performed
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase")]
pub enum ScanType {
    /// Auto scan result
    AutoScan,
    /// Detailed scan result
    Detailed,
    /// Cargo scan result
    Cargo
}

/// An enum to capture the reference of parents
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase")]
pub enum ParentType {
    /// Not sure why this is a null parent, could be that it rotates around a mid point?
    Null(u32),
    /// Parent ring
    Ring(u32),
    /// Parent star
    Star(u32),
    /// Parent planet
    Planet(u32),
}


/// The atmosphere type
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase")]
pub enum AtmosphereType {
    /// Argon based atmosphere
    Argon,
    /// Argon rich atmosphere
    ArgonRich,
    /// Carbon Dioxide atmosphere
    CarbonDioxide,
    /// Carbon Dioxide rich atmosphere
    CarbonDioxideRich,
    /// No atmosphere type
    None,
    /// Silicate vapour type atmosphere
    SilicateVapour,
    /// Sulpher Dioxide based atmosphere
    SulphurDioxide,
    /// Water based atmosphere
    Water,
    /// Catch all for non represented atmospheres
    #[serde(untagged)]
    Other(String),
}

/// A struct to represent a composition value
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase", tag="Name")]
pub struct CompositionPercentage {
    /// The name of the composition element
    pub name: String,
    /// The percentage of this composition element
    pub percent: f32,
}

/// A composition of ice, metal, rock
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase", tag="Name")]
pub struct Composition {
    /// The ice composition
    pub ice: f32,
    /// The metal composition
    pub metal: f32,
    /// The rock composition
    pub rock: f32,
}

/// Struct to represent a ring in around a body
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase", tag="Name")]
pub struct Ring {
    /// Name of the Ring
    pub name: String,
    /// The class of the ring
    pub ring_class: String,
    /// The mass of the ring
    #[serde(rename = "MassMT")]
    pub mass_mt: f32,
    /// The radius of the inner part of the ring
    pub inner_rad: f32,
    /// The radius of the outer part of the ring
    pub outer_rad: f32,
}
