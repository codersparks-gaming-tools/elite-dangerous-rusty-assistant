use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_star_position_serialization() {
        let position = StarPosition { x: 42.0, y: -13.5, z: 7.25 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        assert_eq!(serialized, "[42.0,-13.5,7.25]");
    }

    #[test]
    fn test_star_position_deserialization() {
        let serialized = "[42.0,-13.5,7.25]";
        let deserialized: StarPosition = serde_json::from_str(serialized).expect("Failed to deserialize");
        let expected = StarPosition { x: 42.0, y: -13.5, z: 7.25 };
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_star_position_roundtrip() {
        let position = StarPosition { x: 10.5, y: -20.75, z: 15.0 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        let deserialized: StarPosition = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(position, deserialized);
    }

    #[test]
    fn test_star_position_edge_case_zeros() {
        let position = StarPosition { x: 0.0, y: 0.0, z: 0.0 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        assert_eq!(serialized, "[0.0,0.0,0.0]");
        let deserialized: StarPosition = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(position, deserialized);
    }

    #[test]
    fn test_star_position_negative_values() {
        let position = StarPosition { x: -1.0, y: -2.5, z: -3.75 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        assert_eq!(serialized, "[-1.0,-2.5,-3.75]");
        let deserialized: StarPosition = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(position, deserialized);
    }
}

/// Gives the x,y,z co-ords for a star
#[derive(Debug, Clone, PartialEq)]
pub struct StarPosition {
    /// X position of star
    pub x: f32,
    /// Y postion of star
    pub y: f32,
    /// Z position of star
    pub z: f32,
}

impl Serialize for StarPosition {
    fn serialize<S>(&self, serializer:  S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        (&self.x, self.y, self.z).serialize(serializer)
    }
}

impl <'de> Deserialize<'de> for StarPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Deserialize::deserialize(deserializer)
            .map(|(x, y, z)| StarPosition { x, y, z })
    }
}

