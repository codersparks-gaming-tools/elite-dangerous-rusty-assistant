
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Struct to represent the x, y, z co-ords of a star
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq)]
pub struct StarPosition {
    /// X position of star
    pub x: f32,
    /// Y postion of star
    pub y: f32,
    /// Z position of star
    pub z: f32,
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_star_position_serialization() {
        let position = StarPosition { x: -22.84375, y: 36.53125, z: -1.1875 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        assert_eq!(serialized, "[-22.84375,36.53125,-1.1875]");
    }

    #[test]
    fn test_star_position_deserialization() {
        let serialized = "[-22.84375,36.53125,-1.1875]";
        let deserialized: StarPosition = serde_json::from_str(serialized).expect("Failed to deserialize");
        let expected = StarPosition { x: -22.84375, y: 36.53125, z: -1.1875 };
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_star_position_roundtrip() {
        let position = StarPosition { x: -22.84375, y: 36.53125, z: -1.1875 };
        let serialized = serde_json::to_string(&position).expect("Failed to serialize");
        let deserialized: StarPosition = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(position, deserialized);
    }
}
