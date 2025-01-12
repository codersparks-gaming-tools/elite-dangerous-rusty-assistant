use crate::events::scan::common::{
    AtmosphereType, Composition, CompositionPercentage, ParentType, Ring, ScanType,
};
use crate::events::scan::helper::ScanEventHelper;
use crate::events::EventMeta;
use serde::{Deserialize, Deserializer, Serialize};
use tracing::{debug, trace};

/// The root scan event types
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The type of scan
    pub scan_type: ScanType,

    /// Enum to hold the variant of this type of event
    #[serde(flatten)]
    pub scan_data: ScanData,
}

impl<'de> Deserialize<'de> for ScanEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ScanEventHelper::deserialize(deserializer)?;
        trace!("ScanEventHelper interim value: {:?}", helper);

        debug!("Scan Data type: {:?}", helper.scan_type);

        let scan_data: ScanData;

        // We work out the ScanDataType by following these steps:
        //  1. If the ScanType field is cargo then that is simple :)
        //  2. If the event has StarType then we treat it as a star
        //  3. If the event has TidalLock field then we treat it as a planet/moon
        //  4. If we get here, then we treat as a belt cluster
        if helper.scan_type == ScanType::Cargo {

            scan_data = ScanData::Cargo;
        } else {

            if helper.star_type.is_some() {
                scan_data = ScanData::Star(Star::from_helper(&helper));
            } else if helper.tidal_lock.is_some() {
                scan_data = ScanData::PlanetMoon(PlanetMoon::from_helper(&helper));
            } else {
                scan_data = ScanData::BeltCluster(BeltCluster::from_helper(&helper));
            }
        }

        let event_meta = helper.event_meta;
        let scan_type = helper.scan_type;

        Ok(Self {
            event_meta,
            scan_type,
            scan_data,
        })
    }
}

impl ScanEvent {

}

/// Enum to represent the different types of scan data
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum ScanData {
    /// Scan data represents a star
    Star(Star),
    /// Scan data represents either a planet or a moon
    PlanetMoon(PlanetMoon),
    /// Cargo data scan
    Cargo,
    /// Scan data is belt cluster
    BeltCluster(BeltCluster),
    /// Catch all for unknown
    #[serde(untagged)]
    Unknown(String),
}

/// Data holding struct for Star Data
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Star {
    /// Body data for the star
    #[serde(flatten)]
    pub body: Body,

    /// The star type
    pub star_type: String,
    /// The subclass of the star
    pub sub_class: u32,
    /// The stellar mass of the star
    pub stellar_mass: f32,

    /// The radius of the star
    pub radius: f32,

    /// The surface temperature of the star
    pub surface_temperature: f32,

    /// The absolute magnitude of the star
    pub absolute_magnitude: f32,

    /// The age of the star in millions of years
    #[serde(rename = "Age_MY")]
    pub age: u32,

    /// The luminosity of the star
    pub luminosity: String,

    /// It's rotational period
    pub rotation_period: f32,

    /// The axial tilt of the star
    pub axial_tilt: f32,

    // Optional fields
    /// References to parents (if any)
    pub parents: Option<Vec<ParentType>>,

    /// The orbital mechanics data of the star (if None, it does not orbit)
    #[serde(flatten)]
    pub orbital_mechanics_data: Option<OrbitalMechanicsData>,

    /// Any rings for the star
    pub rings: Option<Vec<Ring>>,
}

impl Star {

    /// Creates a star event from the helper interim struct
    pub(crate) fn from_helper(helper: &ScanEventHelper) -> Self{

        let orbital_mechanics_data = OrbitalMechanicsData::from_helper(helper);

        Self {
            body: Body::from_helper(helper),
            star_type: helper.star_type.clone().unwrap(),
            sub_class: helper.subclass.unwrap(),
            stellar_mass: helper.stellar_mass.unwrap(),
            radius: helper.radius.unwrap(),
            surface_temperature: helper.surface_temperature.unwrap(),
            absolute_magnitude: helper.absolute_magnitude.unwrap(),
            age: helper.age.unwrap(),
            luminosity: helper.luminosity.clone().unwrap(),
            rotation_period: helper.rotation_period.unwrap(),
            axial_tilt: helper.axial_tilt.unwrap(),
            parents: helper.parents.clone(),
            orbital_mechanics_data,
            rings: helper.rings.clone(),
        }
    }
}

/// Data struct for holding data about Planet or Moon
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetMoon {
    /// Body data of the planet
    #[serde(flatten)]
    pub body: Body,

    /// References to parents
    pub parents: Vec<ParentType>,

    /// Is the planet tidally locked
    pub tidal_lock: bool,

    /// Is terraform state (all examples have been empty string so far)
    pub terraform_state: String,

    /// Atmosphere of the planet
    pub atmosphere: String,

    /// Volcanism on the planet
    pub volcanism: String,
    #[serde(rename = "MassEM")]

    /// Mass of the planet
    pub mass_em: f32,

    /// Gravity at the surface
    pub surface_gravity: f32,

    /// Pressure at the surface
    pub surface_pressure: f32,

    /// Temperature at the surface
    pub surface_temperature: f32,

    /// Radius of the planet
    pub radius: f32,

    /// Can the planet be landed on
    pub landable: bool,

    /// The axial tilt of the planet
    pub axial_tilt: f32,

    /// Orbital mechanic data (as a planet it's not optional)
    #[serde(flatten)]
    pub orbital_mechanics_data: OrbitalMechanicsData,

    // Optional fields
    /// The atmosphere type of the planet
    pub atmosphere_type: Option<AtmosphereType>,
    /// Composition of the planet
    pub composition: Option<Composition>,

    /// Materials of the planet
    pub materials: Option<Vec<CompositionPercentage>>,

    /// Composition of the atmosphere
    pub atmosphere_composition: Option<Vec<CompositionPercentage>>,

    /// Details of any rings that the planet has
    pub rings: Option<Vec<Ring>>,

    /// How pristine the resources are
    pub reserve_level: Option<String>,
}

impl PlanetMoon {
    /// Creates a planet_moon event from the helper interim struct
    pub(crate) fn from_helper(helper: &ScanEventHelper) -> Self{

        Self{
            body: Body::from_helper(helper),
            parents: helper.parents.clone().unwrap(),
            tidal_lock: helper.tidal_lock.unwrap(),
            terraform_state: helper.terraform_state.clone().unwrap(),
            atmosphere: helper.atmosphere.clone().unwrap(),
            volcanism: helper.volcanism.clone().unwrap(),
            mass_em: helper.mass_em.unwrap(),
            surface_gravity: helper.surface_gravity.unwrap(),
            surface_pressure: helper.surface_pressure.unwrap(),
            surface_temperature: helper.surface_temperature.unwrap(),
            radius: helper.radius.unwrap(),
            landable: helper.landable.unwrap(),
            axial_tilt: helper.axial_tilt.unwrap(),
            // We know for a planet the orbital data must be present
            orbital_mechanics_data: OrbitalMechanicsData::from_helper(helper).unwrap(),
            atmosphere_type: helper.atmosphere_type.clone(),
            composition: helper.composition.clone(),
            materials: helper.materials.clone(),
            atmosphere_composition: helper.atmosphere_composition.clone(),
            rings: helper.rings.clone(),
            reserve_level: helper.reserve_level.clone(),
        }
    }
}

/// Data for a belt cluster type
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BeltCluster {
    /// The body data of the belt
    #[serde(flatten)]
    pub body: Body,
    /// Parents of the belt cluster
    pub parents: Vec<ParentType>,
}

impl BeltCluster {

    /// Creates a belt event from the helper interim struct
    pub(crate) fn from_helper(helper: &ScanEventHelper) -> Self{

        Self{
            body: Body::from_helper(helper),
            parents: helper.parents.clone().unwrap(),
        }
    }
}

/// Body data shared for different event types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    /// The name of the body
    pub body_name: String,

    /// The id of the body
    #[serde(rename = "BodyID")]
    pub body_id: u32,

    /// What star system this is in
    pub star_system: String,

    /// The system address
    pub system_address: u64,

    /// Distance from the arrival point
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f32,

    /// Has the body already been discovered
    #[serde(rename = "WasDiscovered")]
    pub discovered: bool,

    /// Was the body already mapped
    #[serde(rename = "WasMapped")]
    pub mapped: bool,
}

impl Body {
    /// Creates a body from the helper interim struct
    pub(crate) fn from_helper(helper: &ScanEventHelper) -> Self{
        Self{
            body_name: helper.body_name.clone().unwrap(),
            body_id: helper.body_id.unwrap(),
            star_system: helper.star_system.clone().unwrap(),
            system_address: helper.system_address.unwrap(),
            distance_from_arrival_ls: helper.distance_from_arrival_ls.unwrap(),
            discovered: helper.discovered.unwrap(),
            mapped: helper.mapped.unwrap(),
        }
    }
}


/// Represent orbital mechanics data for the event
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OrbitalMechanicsData {
    /// Semi major axis
    pub semi_major_axis: f64,
    /// The eccentricity of the orbit
    pub eccentricity: f32,
    /// The orbital inclination
    pub orbital_inclination: f32,
    /// The periapsis of the orbit
    pub periapsis: f32,
    /// The period of the orbit
    pub orbital_period: f32,
    /// The ascending node
    pub ascending_node: f32,
    /// The mean anomaly of the orbit
    pub mean_anomaly: f32,
}

impl OrbitalMechanicsData {

    /// Creates an orbital mechanics data from the helper interim struct
    pub(crate) fn from_helper(helper: &ScanEventHelper) -> Option<Self>{
        // We're going to assume that if one field is present all are present
        if helper.semi_major_axis.is_some() {
            Some(Self {
                semi_major_axis: helper.semi_major_axis.unwrap(),
                eccentricity: helper.eccentricity.unwrap(),
                orbital_inclination: helper.orbital_inclination.unwrap(),
                periapsis: helper.periapsis.unwrap(),
                orbital_period: helper.orbital_period.unwrap(),
                ascending_node: helper.ascending_node.unwrap(),
                mean_anomaly: helper.mean_anomaly.unwrap(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::scan::common::{AtmosphereType, Composition, CompositionPercentage, ParentType, Ring, ScanType};
    use crate::events::scan::event::{OrbitalMechanicsData, ScanData, ScanEvent};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_scan_star_event_all_fields_present() {

        let timestamp_str = "2025-01-05T20:42:16Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"LHS 2522 3", "BodyID":8, "Parents":[ {{ "Null":7 }}, {{"Star":0}} ], "StarSystem":"LHS 2522", "SystemAddress":2106421430635, "DistanceFromArrivalLS":2739.775246, "StarType":"Y", "Subclass":4, "StellarMass":0.011719, "Radius":41898096.000000, "AbsoluteMagnitude":22.788177, "Age_MY":6672, "SurfaceTemperature":378.000000, "Luminosity":"V", "SemiMajorAxis":16398852467.536926, "Eccentricity":0.143218, "OrbitalInclination":7.094896, "Periapsis":284.520761, "OrbitalPeriod":41841028.332710, "AscendingNode":141.756867, "MeanAnomaly":7.950981, "RotationPeriod":119017.027353, "AxialTilt":-1.425449, "Rings":[ {{ "Name":"LHS 2522 3 A Ring", "RingClass":"eRingClass_Rocky", "MassMT":3.4128e+11, "InnerRad":9.0118e+07, "OuterRad":1.3795e+08 }}, {{ "Name":"LHS 2522 3 B Ring", "RingClass":"eRingClass_Rocky", "MassMT":9.1944e+12, "InnerRad":1.3805e+08, "OuterRad":5.5941e+08 }} ], "WasDiscovered":true, "WasMapped":false }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);
        match event.scan_data {
            ScanData::Star(star) => {

                assert_eq!(star.body.body_name, "LHS 2522 3");
                assert_eq!(star.body.body_id, 8);
                assert_eq!(star.body.star_system, "LHS 2522");
                assert_eq!(star.body.discovered, true);
                assert_eq!(star.body.mapped, false);
                assert_eq!(star.body.distance_from_arrival_ls, 2739.775246);
                assert_eq!(star.body.system_address, 2106421430635);
                assert_eq!(star.star_type, "Y");
                assert_eq!(star.stellar_mass, 0.011719);
                assert_eq!(star.radius, 41898096.000000);
                assert_eq!(star.surface_temperature, 378.000000);
                assert_eq!(star.absolute_magnitude, 22.788177);
                assert_eq!(star.age, 6672);
                assert_eq!(star.luminosity, "V");
                assert_eq!(star.rotation_period, 119017.027353);
                assert_eq!(star.axial_tilt, -1.425449);

                assert_eq!(star.parents.unwrap(), vec![ParentType::Null(7), ParentType::Star(0)]);

                let ring1 = Ring {
                    name: "LHS 2522 3 A Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 341280000000.0_f32,
                    inner_rad: 90118000.0_f32,
                    outer_rad: 137950000.0_f32,
                };

                let ring2 = Ring {
                    name: "LHS 2522 3 B Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 9194400000000.0_f32,
                    inner_rad: 138050000.0_f32,
                    outer_rad: 559410000.0_f32,
                };

                let rings = star.rings.unwrap();
                assert_eq!(rings.len(), 2);
                assert!(rings.contains(&ring1));
                assert!(rings.contains(&ring2));

                let orbit_data = star.orbital_mechanics_data.unwrap();

                assert_eq!(orbit_data.orbital_inclination, 7.094896);
                assert_eq!(orbit_data.orbital_period, 41841028.332710);
                assert_eq!(orbit_data.semi_major_axis, 16398852467.536926);
                assert_eq!(orbit_data.mean_anomaly, 7.950981);
                assert_eq!(orbit_data.eccentricity, 0.143218);
                assert_eq!(orbit_data.periapsis, 284.520761);
                assert_eq!(orbit_data.ascending_node, 141.756867);


            }
            _ => { panic!("Expected Star, got: {:?}", event.scan_data); }
        }
    }

    #[test]
    fn test_scan_star_event_no_rings() {

        let timestamp_str = "2025-01-05T20:42:16Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"LHS 2522 3", "BodyID":8, "Parents":[ {{ "Null":7 }}, {{"Star":0}} ], "StarSystem":"LHS 2522", "SystemAddress":2106421430635, "DistanceFromArrivalLS":2739.775246, "StarType":"Y", "Subclass":4, "StellarMass":0.011719, "Radius":41898096.000000, "AbsoluteMagnitude":22.788177, "Age_MY":6672, "SurfaceTemperature":378.000000, "Luminosity":"V", "SemiMajorAxis":16398852467.536926, "Eccentricity":0.143218, "OrbitalInclination":7.094896, "Periapsis":284.520761, "OrbitalPeriod":41841028.332710, "AscendingNode":141.756867, "MeanAnomaly":7.950981, "RotationPeriod":119017.027353, "AxialTilt":-1.425449, "WasDiscovered":true, "WasMapped":false }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);
        match event.scan_data {
            ScanData::Star(star) => {

                assert_eq!(star.body.body_name, "LHS 2522 3");
                assert_eq!(star.body.body_id, 8);
                assert_eq!(star.body.star_system, "LHS 2522");
                assert_eq!(star.body.discovered, true);
                assert_eq!(star.body.mapped, false);
                assert_eq!(star.body.distance_from_arrival_ls, 2739.775246);
                assert_eq!(star.body.system_address, 2106421430635);
                assert_eq!(star.star_type, "Y");
                assert_eq!(star.stellar_mass, 0.011719);
                assert_eq!(star.radius, 41898096.000000);
                assert_eq!(star.surface_temperature, 378.000000);
                assert_eq!(star.absolute_magnitude, 22.788177);
                assert_eq!(star.age, 6672);
                assert_eq!(star.luminosity, "V");
                assert_eq!(star.rotation_period, 119017.027353);
                assert_eq!(star.axial_tilt, -1.425449);

                assert_eq!(star.parents.unwrap(), vec![ParentType::Null(7), ParentType::Star(0)]);
                assert!(star.rings.is_none());

                let orbit_data = star.orbital_mechanics_data.unwrap();

                assert_eq!(orbit_data.orbital_inclination, 7.094896);
                assert_eq!(orbit_data.orbital_period, 41841028.332710);
                assert_eq!(orbit_data.semi_major_axis, 16398852467.536926);
                assert_eq!(orbit_data.mean_anomaly, 7.950981);
                assert_eq!(orbit_data.eccentricity, 0.143218);
                assert_eq!(orbit_data.periapsis, 284.520761);
                assert_eq!(orbit_data.ascending_node, 141.756867);


            }
            _ => { panic!("Expected Star, got: {:?}", event.scan_data); }
        }
    }

    #[test]
    fn test_scan_star_event_no_orbital_data() {

        let timestamp_str = "2025-01-05T20:42:16Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"LHS 2522 3", "BodyID":8, "Parents":[ {{ "Null":7 }}, {{"Star":0}} ], "StarSystem":"LHS 2522", "SystemAddress":2106421430635, "DistanceFromArrivalLS":2739.775246, "StarType":"Y", "Subclass":4, "StellarMass":0.011719, "Radius":41898096.000000, "AbsoluteMagnitude":22.788177, "Age_MY":6672, "SurfaceTemperature":378.000000, "Luminosity":"V", "RotationPeriod":119017.027353, "AxialTilt":-1.425449, "Rings":[ {{ "Name":"LHS 2522 3 A Ring", "RingClass":"eRingClass_Rocky", "MassMT":3.4128e+11, "InnerRad":9.0118e+07, "OuterRad":1.3795e+08 }}, {{ "Name":"LHS 2522 3 B Ring", "RingClass":"eRingClass_Rocky", "MassMT":9.1944e+12, "InnerRad":1.3805e+08, "OuterRad":5.5941e+08 }} ], "WasDiscovered":true, "WasMapped":false }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);
        match event.scan_data {
            ScanData::Star(star) => {

                assert_eq!(star.body.body_name, "LHS 2522 3");
                assert_eq!(star.body.body_id, 8);
                assert_eq!(star.body.star_system, "LHS 2522");
                assert_eq!(star.body.discovered, true);
                assert_eq!(star.body.mapped, false);
                assert_eq!(star.body.distance_from_arrival_ls, 2739.775246);
                assert_eq!(star.body.system_address, 2106421430635);
                assert_eq!(star.star_type, "Y");
                assert_eq!(star.stellar_mass, 0.011719);
                assert_eq!(star.radius, 41898096.000000);
                assert_eq!(star.surface_temperature, 378.000000);
                assert_eq!(star.absolute_magnitude, 22.788177);
                assert_eq!(star.age, 6672);
                assert_eq!(star.luminosity, "V");
                assert_eq!(star.rotation_period, 119017.027353);
                assert_eq!(star.axial_tilt, -1.425449);

                assert_eq!(star.parents.unwrap(), vec![ParentType::Null(7), ParentType::Star(0)]);

                let ring1 = Ring {
                    name: "LHS 2522 3 A Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 341280000000.0_f32,
                    inner_rad: 90118000.0_f32,
                    outer_rad: 137950000.0_f32,
                };

                let ring2 = Ring {
                    name: "LHS 2522 3 B Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 9194400000000.0_f32,
                    inner_rad: 138050000.0_f32,
                    outer_rad: 559410000.0_f32,
                };

                let rings = star.rings.unwrap();
                assert_eq!(rings.len(), 2);
                assert!(rings.contains(&ring1));
                assert!(rings.contains(&ring2));

                assert_eq!(star.orbital_mechanics_data, None);


            }
            _ => { panic!("Expected Star, got: {:?}", event.scan_data); }
        }
    }

    #[test]
    fn test_scan_star_event_no_parent_data() {

        let timestamp_str = "2025-01-05T20:42:16Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"LHS 2522 3", "BodyID":8, "StarSystem":"LHS 2522", "SystemAddress":2106421430635, "DistanceFromArrivalLS":2739.775246, "StarType":"Y", "Subclass":4, "StellarMass":0.011719, "Radius":41898096.000000, "AbsoluteMagnitude":22.788177, "Age_MY":6672, "SurfaceTemperature":378.000000, "Luminosity":"V", "SemiMajorAxis":16398852467.536926, "Eccentricity":0.143218, "OrbitalInclination":7.094896, "Periapsis":284.520761, "OrbitalPeriod":41841028.332710, "AscendingNode":141.756867, "MeanAnomaly":7.950981, "RotationPeriod":119017.027353, "AxialTilt":-1.425449, "Rings":[ {{ "Name":"LHS 2522 3 A Ring", "RingClass":"eRingClass_Rocky", "MassMT":3.4128e+11, "InnerRad":9.0118e+07, "OuterRad":1.3795e+08 }}, {{ "Name":"LHS 2522 3 B Ring", "RingClass":"eRingClass_Rocky", "MassMT":9.1944e+12, "InnerRad":1.3805e+08, "OuterRad":5.5941e+08 }} ], "WasDiscovered":true, "WasMapped":false }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);
        match event.scan_data {
            ScanData::Star(star) => {

                assert_eq!(star.body.body_name, "LHS 2522 3");
                assert_eq!(star.body.body_id, 8);
                assert_eq!(star.body.star_system, "LHS 2522");
                assert_eq!(star.body.discovered, true);
                assert_eq!(star.body.mapped, false);
                assert_eq!(star.body.distance_from_arrival_ls, 2739.775246);
                assert_eq!(star.body.system_address, 2106421430635);
                assert_eq!(star.star_type, "Y");
                assert_eq!(star.stellar_mass, 0.011719);
                assert_eq!(star.radius, 41898096.000000);
                assert_eq!(star.surface_temperature, 378.000000);
                assert_eq!(star.absolute_magnitude, 22.788177);
                assert_eq!(star.age, 6672);
                assert_eq!(star.luminosity, "V");
                assert_eq!(star.rotation_period, 119017.027353);
                assert_eq!(star.axial_tilt, -1.425449);

                assert_eq!(star.parents, None);

                let ring1 = Ring {
                    name: "LHS 2522 3 A Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 341280000000.0_f32,
                    inner_rad: 90118000.0_f32,
                    outer_rad: 137950000.0_f32,
                };

                let ring2 = Ring {
                    name: "LHS 2522 3 B Ring".to_string(),
                    ring_class: "eRingClass_Rocky".to_string(),
                    mass_mt: 9194400000000.0_f32,
                    inner_rad: 138050000.0_f32,
                    outer_rad: 559410000.0_f32,
                };

                let rings = star.rings.unwrap();
                assert_eq!(rings.len(), 2);
                assert!(rings.contains(&ring1));
                assert!(rings.contains(&ring2));

                let orbit_data = star.orbital_mechanics_data.unwrap();

                assert_eq!(orbit_data.orbital_inclination, 7.094896);
                assert_eq!(orbit_data.orbital_period, 41841028.332710);
                assert_eq!(orbit_data.semi_major_axis, 16398852467.536926);
                assert_eq!(orbit_data.mean_anomaly, 7.950981);
                assert_eq!(orbit_data.eccentricity, 0.143218);
                assert_eq!(orbit_data.periapsis, 284.520761);
                assert_eq!(orbit_data.ascending_node, 141.756867);


            }
            _ => { panic!("Expected Star, got: {:?}", event.scan_data); }
        }
    }

    #[test]
    fn test_scan_event_planet_example_1() {

        let timestamp_str = "2025-01-05T20:44:18Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"LHS 2522 4 a", "BodyID":20, "Parents":[ {{"Planet":18}}, {{"Null":7}}, {{"Star":0}} ], "StarSystem":"LHS 2522", "SystemAddress":2106421430635, "DistanceFromArrivalLS":2683.850460, "TidalLock":true, "TerraformState":"", "PlanetClass":"Icy body", "Atmosphere":"thin argon atmosphere", "AtmosphereType":"Argon", "AtmosphereComposition":[ {{ "Name":"Argon", "Percent":100.000000 }} ], "Volcanism":"major water geysers volcanism", "MassEM":0.055796, "Radius":3101817.250000, "SurfaceGravity":2.311420, "SurfaceTemperature":100.500710, "SurfacePressure":8061.750488, "Landable":true, "Materials":[ {{ "Name":"sulphur", "Percent":24.456606 }}, {{ "Name":"carbon", "Percent":20.565472 }} ], "Composition":{{ "Ice":0.571810, "Rock":0.390847, "Metal":0.037343 }}, "SemiMajorAxis":627286136.150360, "Eccentricity":0.003676, "OrbitalInclination":0.000573, "Periapsis":22.055149, "OrbitalPeriod":106721.353531, "AscendingNode":-146.743804, "MeanAnomaly":120.248385, "RotationPeriod":106729.171879, "AxialTilt":0.464257, "WasDiscovered":true, "WasMapped":true }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        println!("{:#?}", event);

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);

        match event.scan_data {
            ScanData::PlanetMoon(planet_moon) => {

                assert_eq!(planet_moon.body.body_name, "LHS 2522 4 a");
                assert_eq!(planet_moon.body.body_id, 20);
                assert_eq!(planet_moon.body.star_system, "LHS 2522");
                assert_eq!(planet_moon.body.discovered, true);
                assert_eq!(planet_moon.body.mapped, true);
                assert_eq!(planet_moon.body.distance_from_arrival_ls, 2683.850460);
                assert_eq!(planet_moon.body.system_address, 2106421430635);

                assert_eq!(planet_moon.parents, vec![ParentType::Planet(18), ParentType::Null(7), ParentType::Star(0)]);

                assert_eq!(planet_moon.tidal_lock, true);
                assert_eq!(planet_moon.terraform_state, "");
                assert_eq!(planet_moon.atmosphere, "thin argon atmosphere");
                assert_eq!(planet_moon.volcanism, "major water geysers volcanism");
                assert_eq!(planet_moon.mass_em, 0.055796);
                assert_eq!(planet_moon.surface_gravity, 2.311420);
                assert_eq!(planet_moon.surface_pressure, 8061.750488);
                assert_eq!(planet_moon.surface_temperature, 100.500710);
                assert_eq!(planet_moon.radius, 3101817.250000);
                assert_eq!(planet_moon.landable, true);
                assert_eq!(planet_moon.axial_tilt, 0.464257);

                let orbital = OrbitalMechanicsData{
                    semi_major_axis: 627286136.150360,
                    eccentricity: 0.003676,
                    orbital_inclination: 0.000573,
                    periapsis: 22.055149,
                    orbital_period: 106721.353531,
                    ascending_node: -146.743804,
                    mean_anomaly: 120.248385,
                };

                assert_eq!(planet_moon.orbital_mechanics_data, orbital);
                assert_eq!(planet_moon.atmosphere_type, Some(AtmosphereType::Argon));
                assert_eq!(planet_moon.composition, Some(Composition { ice: 0.571810, rock: 0.390847, metal: 0.037343}));
                assert_eq!(planet_moon.atmosphere_composition, Some(vec![CompositionPercentage{ name: "Argon".to_string(), percent: 100.000000 }]));
                assert_eq!(planet_moon.materials, Some(vec![CompositionPercentage{ name:"sulphur".to_string(), percent:24.456606 }, CompositionPercentage{ name:"carbon".to_string(), percent:20.565472 }]));

                assert_eq!(planet_moon.rings, None);
                assert_eq!(planet_moon.reserve_level, None);


            }
            _ => { panic!("Expected PlanetMoon, got: {:?}", event.scan_data); }
        }
    }

    #[test]
    fn test_scan_event_belt() {

        let timestamp_str = "2025-01-06T22:31:16Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Scan", "ScanType":"AutoScan", "BodyName":"Bota Ili A Belt Cluster 3", "BodyID":4, "Parents":[ {{"Ring":1}}, {{"Star":0}} ], "StarSystem":"Bota Ili", "SystemAddress":6406044390082, "DistanceFromArrivalLS":6.035235, "WasDiscovered":true, "WasMapped":false }}"#);

        let event: ScanEvent = serde_json::from_str(&json).unwrap();

        println!("{:#?}", event);
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.scan_type, ScanType::AutoScan);

        match event.scan_data {
            ScanData::BeltCluster(belt) => {
                assert_eq!(belt.body.body_name, "Bota Ili A Belt Cluster 3");
                assert_eq!(belt.body.body_id, 4);
                assert_eq!(belt.body.star_system, "Bota Ili");
                assert_eq!(belt.body.discovered, true);
                assert_eq!(belt.body.mapped, false);
                assert_eq!(belt.body.distance_from_arrival_ls, 6.035235);
                assert_eq!(belt.body.system_address, 6406044390082);

                assert_eq!(belt.parents, vec![ParentType::Ring(1), ParentType::Star(0)]);


            },
            _ => { panic!("Expected BeltCluster, got: {:?}", event.scan_data); }
        }

    }

    #[test]
    fn test_cargo_scan_event() {

    }

}