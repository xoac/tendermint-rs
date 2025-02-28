use tendermint::{abci, evidence};
use tendermint_proto::v0_37 as raw;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub struct Dialect;

impl crate::dialect::Dialect for Dialect {
    type Event = Event;
    type Evidence = Evidence;
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub kind: String,
    pub attributes: Vec<EventAttribute>,
}

impl From<Event> for abci::Event {
    fn from(msg: Event) -> Self {
        Self {
            kind: msg.kind,
            attributes: msg.attributes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<abci::Event> for Event {
    fn from(msg: abci::Event) -> Self {
        Self {
            kind: msg.kind,
            attributes: msg.attributes.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EventAttribute {
    /// The event key.
    pub key: String,
    /// The event value.
    pub value: String,
    /// Whether Tendermint's indexer should index this event.
    ///
    /// **This field is nondeterministic**.
    pub index: bool,
}

impl From<EventAttribute> for abci::EventAttribute {
    fn from(msg: EventAttribute) -> Self {
        Self {
            key: msg.key,
            value: msg.value,
            index: msg.index,
        }
    }
}

impl From<abci::EventAttribute> for EventAttribute {
    fn from(msg: abci::EventAttribute) -> Self {
        Self {
            key: msg.key,
            value: msg.value,
            index: msg.index,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(into = "raw::types::Evidence", try_from = "raw::types::Evidence")]
pub struct Evidence(evidence::Evidence);

impl From<Evidence> for raw::types::Evidence {
    fn from(evidence: Evidence) -> Self {
        evidence.0.into()
    }
}

impl TryFrom<raw::types::Evidence> for Evidence {
    type Error = <evidence::Evidence as TryFrom<raw::types::Evidence>>::Error;

    fn try_from(value: raw::types::Evidence) -> Result<Self, Self::Error> {
        Ok(Self(evidence::Evidence::try_from(value)?))
    }
}

impl From<evidence::Evidence> for Evidence {
    fn from(evidence: evidence::Evidence) -> Self {
        Self(evidence)
    }
}
