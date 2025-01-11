use serde::{Deserialize, Serialize};
use crate::events::EventMeta;
use crate::events::scan::common::{AtmosphereType, Composition, CompositionPercentage, ParentType, Ring, ScanType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all="PascalCase")]
pub struct ScanEventHelper {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The absolute magnitude of the body
    pub absolute_magnitude: Option<f32>,

    /// The ascending node
    pub ascending_node: Option<f32>,

    /// The atmosphere display text
    pub atmosphere: Option<String>,

    /// List of the percentage composition of the atmosphere
    pub atmosphere_composition: Option<Vec<CompositionPercentage>>,

    /// The type of atmosphere
    pub atmosphere_type: Option<AtmosphereType>,

    /// The age of the star in millons of years
    #[serde(rename="Age_MY")]
    pub age: Option<u32>,

    /// The axial tilt of the body
    pub axial_tilt: Option<f32>,

    /// The name of the scanned body
    pub body_name: Option<String>,

    /// Id of the body
    #[serde(rename = "BodyID")]
    pub body_id: Option<u32>,

    /// The composition of the body
    pub composition: Option<Composition>,

    /// The distance from the arrival point in Light Seconds
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: Option<f32>,

    /// The eccentricity of the body
    pub eccentricity: Option<f32>,

    /// Is the body landable on
    pub landable: Option<bool>,

    /// Luminosity
    pub luminosity: Option<String>,

    /// Mass ET ?
    #[serde(rename="MassET")]
    pub mass_et: Option<f32>,

    /// Mass EM ?
    #[serde(rename="MassEM")]
    pub mass_em: Option<f32>,

    /// Materials
    pub materials: Option<Vec<CompositionPercentage>>,

    /// The mean anomaly
    pub mean_anomaly: Option<f32>,

    /// The Orbital Inclination of the body
    pub orbital_inclination: Option<f32>,

    /// The orbital period
    pub orbital_period: Option<f32>,

    /// Parents of the body
    pub parents: Option<Vec<ParentType>>,

    /// The periapsis of the body
    pub periapsis: Option<f32>,

    /// The planet class of the body
    pub planet_class: Option<String>,

    /// The radius of the object
    pub radius: Option<f32>,

    /// The status of the reserves
    pub reserve_level: Option<String>,

    /// The rings around the body
    pub rings: Option<Vec<Ring>>,

    /// Period of rotation of the body
    pub rotation_period: Option<f32>,

    /// The type of scan that was performed
    pub scan_type: ScanType,

    /// The semi major axis of the body
    pub semi_major_axis: Option<f64>,

    /// The name of the star systems
    pub star_system: Option<String>,

    /// The type of the star
    pub star_type: Option<String>,

    /// The stellar mass of the object
    pub stellar_mass: Option<f32>,

    /// The subclass of the start
    pub subclass: Option<u32>,

    /// The surface Gravity
    pub surface_gravity: Option<f32>,

    /// The pressure at the surface
    pub surface_pressure: Option<f32>,

    /// The surface temparatue
    pub surface_temperature: Option<f32>,

    /// The internal address of the system
    pub system_address: Option<u64>,

    /// The terraform state of the body
    pub terraform_state: Option<String>,

    /// Is the body tidally locked?
    pub tidal_lock: Option<bool>,

    /// Volcanism on the body
    pub volcanism: Option<String>,

    /// Is this body already discovered
    #[serde(rename = "WasDiscovered")]
    pub discovered: Option<bool>,

    /// Is this body mapped
    #[serde(rename = "WasMapped")]
    pub mapped: Option<bool>,


}