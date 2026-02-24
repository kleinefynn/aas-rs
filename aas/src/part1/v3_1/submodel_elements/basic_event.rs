use crate::part1::v3_1::primitives::{DateTimeUTC, MessageTopic};
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::submodel_elements::SubmodelElementFields;

use strum::{Display, EnumString};

#[derive(Clone, PartialEq, Debug)]
pub struct BasicEventElement {
    submodel_element_fields: SubmodelElementFields,

    pub observed: Reference,

    pub direction: Direction,

    pub state: StateOfEvent,

    pub message_topic: Option<MessageTopic>,

    pub message_broker: Option<Reference>,

    pub last_update: Option<DateTimeUTC>,

    // TODO: duration type
    pub min_interval: Option<String>,

    // TODO: duration type
    pub max_interval: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]

pub struct BasicEventElementMeta {
    submodel_element_fields: SubmodelElementFields,

    pub direction: Direction,

    pub state: StateOfEvent,

    pub message_topic: Option<MessageTopic>,

    pub message_broker: Option<Reference>,

    pub last_update: Option<DateTimeUTC>,

    // TODO: duration type
    pub min_interval: Option<String>,

    // TODO: duration type
    pub max_interval: Option<String>,
}

impl From<BasicEventElement> for BasicEventElementMeta {
    fn from(element: BasicEventElement) -> Self {
        Self {
            submodel_element_fields: element.submodel_element_fields,
            direction: element.direction,
            state: element.state,
            message_topic: element.message_topic,
            message_broker: element.message_broker,
            last_update: element.last_update,
            min_interval: element.min_interval,
            max_interval: element.max_interval,
        }
    }
}

impl From<&BasicEventElement> for BasicEventElementMeta {
    fn from(element: &BasicEventElement) -> Self {
        element.clone().into()
    }
}

#[derive(Clone, PartialEq, Debug, Display, EnumString)]
pub enum StateOfEvent {
    On,

    Off,
}

#[derive(Clone, PartialEq, Debug, Display, EnumString)]

pub enum Direction {
    Input,

    Output,
}
