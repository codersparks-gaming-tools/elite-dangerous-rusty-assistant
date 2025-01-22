use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use crate::events::common::{EventMeta, LocalisedValue};

/// Emitted when mining fragments are converted
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from="MiningRefinedEventSchema", into="MiningRefinedEventSchema")]
pub struct MiningRefinedEvent {
    
    /// The event meta data
    pub event_meta: EventMeta,
    
    /// The type of resource refined
    pub resource_type: LocalisedValue
    
}

impl From<MiningRefinedEventSchema> for MiningRefinedEvent {
    fn from(value: MiningRefinedEventSchema) -> Self {
        Self {
            event_meta: value.event_meta,
            resource_type: LocalisedValue { 
                value: value.resource_type, 
                localised_value: value.resource_type_localised 
            },
        }
    }
}

/// Schema representation of mining refined as contains localised value
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct MiningRefinedEventSchema {
    
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    #[serde(rename="Type")]
    pub resource_type: String,
    
    #[serde(rename="Type_Localised")]
    pub resource_type_localised: Option<String>,
}

impl From<MiningRefinedEvent> for MiningRefinedEventSchema {
    fn from(value: MiningRefinedEvent) -> Self {
        Self {
            event_meta: value.event_meta,
            resource_type: value.resource_type.value,
            resource_type_localised: value.resource_type.localised_value,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_mining_refined_event_deserialization() {
        
        let timestamp_str = "2023-01-01T00:00:00Z";
        let timestamp = create_timestamp(timestamp_str);
        
        let json_data = json!({
            "timestamp": timestamp_str,
            "Type": "Gold",
            "Type_Localised": "Gold (Localised)"
        })
            .to_string();

        let deserialized_event: MiningRefinedEvent = serde_json::from_str(&json_data).expect("Failed to deserialize MiningRefinedEvent");

        assert_eq!(deserialized_event.event_meta.timestamp, timestamp);
        assert_eq!(deserialized_event.resource_type.value, "Gold");
        assert_eq!(
            deserialized_event.resource_type.localised_value,
            Some("Gold (Localised)".to_string())
        );
    }
}