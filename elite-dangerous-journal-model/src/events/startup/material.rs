
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::events::common::EventMeta;

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
#[serde(rename_all = "PascalCase")]
pub struct LocalisedMaterial {
    /// The internal name of the material
    pub name: String,
    /// The number  in the commander's inventory
    pub count: u64,
    /// The localised string used in the game
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
}

#[cfg(test)]
mod tests {
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
            name: "conductivecomponents".to_string(),
            count: 7,
            name_localised: "Conductive Components".to_string(),
        };

        let manufactured2 = LocalisedMaterial {
            name: "mechanicalequipment".to_string(),
            count: 12,
            name_localised: "Mechanical Equipment".to_string(),
        };

        let encoded1 = LocalisedMaterial {
            name: "bulkscandata".to_string(),
            count: 21,
            name_localised: "Anomalous Bulk Scan Data".to_string(),
        };

        let encoded2 = LocalisedMaterial {
            name: "disruptedwakeechoes".to_string(),
            count: 294,
            name_localised: "Atypical Disrupted Wake Echoes".to_string(),
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
            name: "conductivecomponents".to_string(),
            count: 7,
            name_localised: "Conductive Components".to_string(),
        };

        let manufactured2 = LocalisedMaterial {
            name: "mechanicalequipment".to_string(),
            count: 12,
            name_localised: "Mechanical Equipment".to_string(),
        };

        let encoded1 = LocalisedMaterial {
            name: "bulkscandata".to_string(),
            count: 21,
            name_localised: "Anomalous Bulk Scan Data".to_string(),
        };

        let encoded2 = LocalisedMaterial {
            name: "disruptedwakeechoes".to_string(),
            count: 294,
            name_localised: "Atypical Disrupted Wake Echoes".to_string(),
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
            name: "conductivecomponents".to_string(),
            count: 7,
            name_localised: "Conductive Components".to_string(),
        };

        let manufactured2 = LocalisedMaterial {
            name: "mechanicalequipment".to_string(),
            count: 12,
            name_localised: "Mechanical Equipment".to_string(),
        };

        let encoded1 = LocalisedMaterial {
            name: "bulkscandata".to_string(),
            count: 21,
            name_localised: "Anomalous Bulk Scan Data".to_string(),
        };

        let encoded2 = LocalisedMaterial {
            name: "disruptedwakeechoes".to_string(),
            count: 294,
            name_localised: "Atypical Disrupted Wake Echoes".to_string(),
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

