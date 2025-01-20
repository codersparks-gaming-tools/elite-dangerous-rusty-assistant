use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// Data for a recieve text event
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(from = "ReceiveTextEventSchema", into = "ReceiveTextEventSchema")]
pub struct ReceiveTextEvent {
    /// The event meta data
    pub event_meta: EventMeta,
    /// The sender of the message
    pub from: LocalisedValue,
    /// The message
    pub message: LocalisedValue,
    /// The channel that received this message
    pub channel: Channel,
}

impl From<ReceiveTextEventSchema> for ReceiveTextEvent {
    fn from(value: ReceiveTextEventSchema) -> Self {

        let from = LocalisedValue {
            value: value.from,
            localised_value: value.localised_from,
        };
        let message = LocalisedValue {
            value: value.message,
            localised_value: value.localised_message,
        };
        Self{
            event_meta: value.event_meta,
            from,
            message,
            channel: value.channel,
        }
    }
}

/// The schema for the ReceiveText event
///
/// Example:
///
/// ```json
/// {
///   "timestamp": "2025-01-12T20:11:12Z",
///   "event": "ReceiveText",
///   "From": "$ShipName_PassengerLiner_Cruise;",
///   "From_Localised": "Cruise Ship",
///   "Message": "$CruiseLiner_SCPatrol04;",
///   "Message_Localised": "I'd like to direct your attention to our flight attendants for a brief safety demonstration.",
///   "Channel": "npc"
/// }
///
/// Read the docs: https://elite-journal.readthedocs.io/en/latest/Other%20Events/#receivetext
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub(crate) struct ReceiveTextEventSchema {
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// Raw from string (could be a localised placeholder)
    pub from: String,
    /// If present gives the localised value for the from field
    #[serde(rename = "From_Localised")]
    pub localised_from: Option<String>,
    /// Raw message string (could be a localised placeholder)
    pub message: String,
    /// If present gives the localised value for the message field
    #[serde(rename = "Message_Localised")]
    pub localised_message: Option<String>,
    /// The channel that received this message
    pub channel: Channel,
}

impl From<ReceiveTextEvent> for ReceiveTextEventSchema {
    fn from(value: ReceiveTextEvent) -> Self {
        Self{
            event_meta: value.event_meta,
            from: value.from.value,
            localised_from: value.from.localised_value,
            message: value.message.value,
            localised_message: value.message.localised_value,
            channel: value.channel,
        }
    }
}



/// The channel that message was received on
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Channel {
    /// Message from wing
    #[serde(rename="wing")]
    Wing,
    /// Message from local
    #[serde(rename="local")]
    Local,
    /// Message from voice chat
    #[serde(rename="voicechat")]
    VoiceChat,
    /// Message from friend
    #[serde(rename="friend")]
    Friend,
    /// Message from player
    #[serde(rename="player")]
    Player,
    /// Message from NPC
    #[serde(rename="npc")]
    NPC,
    /// Message from squadron
    #[serde(rename="squadron")]
    Squadron,
    /// Message from Star System
    #[serde(rename="starsystem")]
    StarSystem
}

#[cfg(test)]
mod tests {
    use crate::events::other::receive_text::{Channel, ReceiveTextEvent};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_receive_text_event() {
        let timestamp_str = "2025-01-12T20:12:24Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"ReceiveText", "From":"$npc_name_decorate:#name=James Gavin;", "From_Localised":"James Gavin", "Message":"$Smuggler_NearDeath03;", "Message_Localised":"You almost had me there!", "Channel":"npc" }}"#);

        let event: ReceiveTextEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.channel, Channel::NPC);
        assert_eq!(event.message.value, "$Smuggler_NearDeath03;");
        assert_eq!(event.message.localised_value, Some("You almost had me there!".to_string()));
        assert_eq!(event.from.value, "$npc_name_decorate:#name=James Gavin;");
        assert_eq!(event.from.localised_value, Some("James Gavin".to_string()));

        assert_eq!(event.event_meta.timestamp, timestamp);
    }
}