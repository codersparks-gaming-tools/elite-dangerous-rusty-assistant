use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// Event for detected signals for body
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalsEvent {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// The name of the body
    pub body_name: String,
    
    /// The id of the body
    #[serde(rename = "BodyID")]
    pub body_id: u32,
    
    /// The address of the system
    pub system_address: u64,
    
    /// The signals in the system
    pub signals: Vec<Signal>,
}


/// The representation of the signal
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "SignalSchema", into = "SignalSchema")]
pub struct Signal {
    /// The type of signal enum
    signal_type: SignalType,
    /// Localised value of the signal type
    signal_type_value: LocalisedValue,
    /// Number of instances
    count: u32,
}

impl From<SignalSchema> for Signal {
    fn from(value: SignalSchema) -> Self {
        Self {
            signal_type: value.signal_type.clone().into(),
            signal_type_value: LocalisedValue::new(value.signal_type, Some(value.signal_type_localised)),
            count: value.count,
        }
    }
}

/// Schema structure for Signal Event due to localised value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SignalSchema {
    /// The signal type (placeholder)
    #[serde(rename = "Type")]
    pub signal_type: String,
    
    /// The signal type localised string
    #[serde(rename = "Type_Localised")]
    pub signal_type_localised: String ,
    
    /// The number of signals
    pub count: u32,
}

impl From<Signal> for SignalSchema {
    fn from(value: Signal) -> Self {
        Self {
            signal_type: value.signal_type_value.value,
            signal_type_localised: value.signal_type_value.localised_value.unwrap(),
            count: value.count,
        }
    }
}

/// Enum to match the signal type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SignalType {
    /// The signal type is biological
    Biological,
    /// The signal type is Geological
    Geological,
    /// Catch all for error handling
    Other(String)
}

impl From<String> for SignalType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "$SAA_SignalType_Biological;" => SignalType::Biological,
            "$SAA_SignalType_Geological;" => SignalType::Geological,
            _ => SignalType::Other(value)
        }
    }
}