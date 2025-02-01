use serde::{Deserialize, Serialize};

/// The effect of the successful mission
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MissionEffect {
    None,
    VeryLow,
    Low,
    Med,
    High,
    VeryHigh,
}

impl From<String> for MissionEffect {
    fn from(value: String) -> Self {
        match value.as_str() {
            "None" => MissionEffect::None,
            "+" => MissionEffect::VeryLow,
            "++" => MissionEffect::Low,
            "+++" => MissionEffect::Med,
            "++++" => MissionEffect::High,
            "+++++" => MissionEffect::VeryHigh,
            _ => panic!("Unknown mission effect"),
        }
    }
}


impl MissionEffect {

    /// Convert the mission effect to the string representation that is present in events
    pub fn to_string(&self) -> String {
        match self {
            MissionEffect::None => String::from("None"),
            MissionEffect::VeryLow => String::from("+"),
            MissionEffect::Low => String::from("++"),
            MissionEffect::Med => String::from("+++"),
            MissionEffect::High => String::from("++++"),
            MissionEffect::VeryHigh => String::from("+++++"),
        }
    }
}