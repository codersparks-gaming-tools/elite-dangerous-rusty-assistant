
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::events::common::{EventMeta, LocalisedValue};

/// The materials event give the current materials that the commander has in their inventory
///
/// Format example from logs
/// ```json
///{
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "Materials",
///   // Note, this is not included in the event as it tells the serde processor what Enum to use
///   "Raw": [
///     {
///       "Name": "lead",
///       "Count": 48
///     },
///     {
///       "Name": "nickel",
///       "Count": 299
///     },
///     {
///       "Name": "vanadium",
///       "Count": 102
///     }
///   ],
///   "Manufactured": [
///     {
///       "Name": "conductivecomponents",
///       "Name_Localised": "Conductive Components",
///       "Count": 7
///     },
///     {
///       "Name": "mechanicalequipment",
///       "Name_Localised": "Mechanical Equipment",
///       "Count": 12
///     }
///   ],
///   "Encoded": [
///     {
///       "Name": "bulkscandata",
///       "Name_Localised": "Anomalous Bulk Scan Data",
///       "Count": 21
///     },
///     {
///       "Name": "disruptedwakeechoes",
///       "Name_Localised": "Atypical Disrupted Wake Echoes",
///       "Count": 294
///     }
///   ]
/// }
/// ```
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialsEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// List of all the [Raw Materials](https://elite-dangerous.fandom.com/wiki/Raw_Materials) in the commanders inventory
    #[serde(default)]
    pub raw: Vec<RawMaterial>,
    /// List of all the [Manufactured Materials](https://elite-dangerous.fandom.com/wiki/Manufactured_Materials) in the commanders inventory
    #[serde(default)]
    pub manufactured: Vec<LocalisedMaterial>,
    /// List of all the [Encoded Materials](https://elite-dangerous.fandom.com/wiki/Encoded_Materials) in the commanders inventory
    #[serde(default)]
    pub encoded: Vec<LocalisedMaterial>,
}

/// A [Raw Material](https://elite-dangerous.fandom.com/wiki/Raw_Materials) inventory count
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RawMaterial {
    /// The name of the raw material
    pub name: String,
    /// The number in the commander's inventory
    pub count: u64,
}

/// Both [Encoded Material](https://elite-dangerous.fandom.com/wiki/Encoded_Materials) and [Manufactured Material](https://elite-dangerous.fandom.com/wiki/Manufactured_Materials) use a localised material inventory to provide a translation for in game.
///
/// **Note**: Localisation is not supported at present and only the raw value from the log is stored, there is no translation
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(from = "LocalisedMaterialSchema", into = "LocalisedMaterialSchema")]
pub struct LocalisedMaterial {
    /// The number  in the commander's inventory
    pub count: u64,
    /// The name of the material
    #[serde(rename = "Name_Localised")]
    pub name: LocalisedValue,
}

impl From<LocalisedMaterialSchema> for LocalisedMaterial {
    fn from(value: LocalisedMaterialSchema) -> Self {
        Self {
            count: value.count,
            name: LocalisedValue { value: value.name, localised_value: Some(value.name_localised) },
        }
    }
}

/// Schema for Localised Material
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocalisedMaterialSchema {
    /// The internal name of the material
    pub name: String,
    /// The number  in the commander's inventory
    pub count: u64,
    /// The localised string used in the game
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
}

impl From<LocalisedMaterial> for LocalisedMaterialSchema {
    fn from(value: LocalisedMaterial) -> Self {
        Self {
            name: value.name.value,
            count: value.count,
            name_localised: value.name.localised_value.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::common::LocalisedValue;
    use crate::events::startup::material::{LocalisedMaterial, MaterialsEvent, RawMaterial};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_commander_event() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Materials", "Raw":[ {{ "Name":"lead", "Count":48 }}, {{ "Name":"nickel", "Count":299 }} ], "Manufactured":[ {{ "Name":"conductivecomponents", "Name_Localised":"Conductive Components", "Count":7 }}, {{ "Name":"mechanicalequipment", "Name_Localised":"Mechanical Equipment", "Count":12 }} ], "Encoded":[ {{ "Name":"bulkscandata", "Name_Localised":"Anomalous Bulk Scan Data", "Count":21 }}, {{ "Name":"disruptedwakeechoes", "Name_Localised":"Atypical Disrupted Wake Echoes", "Count":294 }}] }}"#);

        let event: MaterialsEvent = serde_json::from_str(&json).unwrap();

        let raw1 = RawMaterial {
            name: "lead".to_string(),
            count: 48,
        };
        let raw2 = RawMaterial {
            name: "nickel".to_string(),
            count: 299,
        };

        let manufactured1 = LocalisedMaterial {
            count: 7,
            name: LocalisedValue {
                value: "conductivecomponents".to_string(),
                localised_value: Some("Conductive Components".to_string()),
            }
        };

        let manufactured2 = LocalisedMaterial {
            count: 12,
            name: LocalisedValue {
                value: "mechanicalequipment".to_string(),
                localised_value: Some("Mechanical Equipment".to_string()),
            }
        };

        let encoded1 = LocalisedMaterial {
            count: 21,
            name: LocalisedValue {
                value: "bulkscandata".to_string(),
                localised_value: Some("Anomalous Bulk Scan Data".to_string()),
            }
        };

        let encoded2 = LocalisedMaterial {
            count: 294,
            name: LocalisedValue {
                value: "disruptedwakeechoes".to_string(),
                localised_value: Some("Atypical Disrupted Wake Echoes".to_string()),
            }
        };

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.raw.len(), 2);
        assert!(event.raw.contains(&raw1));
        assert!(event.raw.contains(&raw2));
        assert_eq!(event.manufactured.len(), 2);
        assert!(event.manufactured.contains(&manufactured1));
        assert!(event.manufactured.contains(&manufactured2));
        assert_eq!(event.encoded.len(), 2);
        assert!(event.encoded.contains(&encoded1));
        assert!(event.encoded.contains(&encoded2));

    }

    #[test]
    fn test_deserialize_commander_event_empty_raw() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Materials", "Raw":[], "Manufactured":[ {{ "Name":"conductivecomponents", "Name_Localised":"Conductive Components", "Count":7 }}, {{ "Name":"mechanicalequipment", "Name_Localised":"Mechanical Equipment", "Count":12 }} ], "Encoded":[ {{ "Name":"bulkscandata", "Name_Localised":"Anomalous Bulk Scan Data", "Count":21 }}, {{ "Name":"disruptedwakeechoes", "Name_Localised":"Atypical Disrupted Wake Echoes", "Count":294 }}] }}"#);

        let event: MaterialsEvent = serde_json::from_str(&json).unwrap();

        let manufactured1 = LocalisedMaterial {
            count: 7,
            name: LocalisedValue {
                value: "conductivecomponents".to_string(),
                localised_value: Some("Conductive Components".to_string()),
            }
        };

        let manufactured2 = LocalisedMaterial {
            count: 12,
            name: LocalisedValue {
                value: "mechanicalequipment".to_string(),
                localised_value: Some("Mechanical Equipment".to_string()),
            }
        };

        let encoded1 = LocalisedMaterial {
            count: 21,
            name: LocalisedValue {
                value: "bulkscandata".to_string(),
                localised_value: Some("Anomalous Bulk Scan Data".to_string()),
            }
        };

        let encoded2 = LocalisedMaterial {
            count: 294,
            name: LocalisedValue {
                value: "disruptedwakeechoes".to_string(),
                localised_value: Some("Atypical Disrupted Wake Echoes".to_string()),
            }
        };

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.raw.len(), 0);
        assert_eq!(event.manufactured.len(), 2);
        assert!(event.manufactured.contains(&manufactured1));
        assert!(event.manufactured.contains(&manufactured2));
        assert_eq!(event.encoded.len(), 2);
        assert!(event.encoded.contains(&encoded1));
        assert!(event.encoded.contains(&encoded2));

    }


    #[test]
    fn test_deserialize_commander_event_missing_raw() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Materials", "Manufactured":[ {{ "Name":"conductivecomponents", "Name_Localised":"Conductive Components", "Count":7 }}, {{ "Name":"mechanicalequipment", "Name_Localised":"Mechanical Equipment", "Count":12 }} ], "Encoded":[ {{ "Name":"bulkscandata", "Name_Localised":"Anomalous Bulk Scan Data", "Count":21 }}, {{ "Name":"disruptedwakeechoes", "Name_Localised":"Atypical Disrupted Wake Echoes", "Count":294 }}] }}"#);

        let event: MaterialsEvent = serde_json::from_str(&json).unwrap();

        let manufactured1 = LocalisedMaterial {
            count: 7,
            name: LocalisedValue {
                value: "conductivecomponents".to_string(),
                localised_value: Some("Conductive Components".to_string()),
            }
        };

        let manufactured2 = LocalisedMaterial {
            count: 12,
            name: LocalisedValue {
                value: "mechanicalequipment".to_string(),
                localised_value: Some("Mechanical Equipment".to_string()),
            }
        };

        let encoded1 = LocalisedMaterial {
            count: 21,
            name: LocalisedValue {
                value: "bulkscandata".to_string(),
                localised_value: Some("Anomalous Bulk Scan Data".to_string()),
            }
        };

        let encoded2 = LocalisedMaterial {
            count: 294,
            name: LocalisedValue {
                value: "disruptedwakeechoes".to_string(),
                localised_value: Some("Atypical Disrupted Wake Echoes".to_string()),
            }
        };

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.raw.len(), 0);
        assert_eq!(event.manufactured.len(), 2);
        assert!(event.manufactured.contains(&manufactured1));
        assert!(event.manufactured.contains(&manufactured2));
        assert_eq!(event.encoded.len(), 2);
        assert!(event.encoded.contains(&encoded1));
        assert!(event.encoded.contains(&encoded2));

    }
}

