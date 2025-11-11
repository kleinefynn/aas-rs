pub mod data_type_def_xs;
pub mod lang_string;

use crate::part_1::v3_1::LangString;
use crate::utilities::{
    deserialize_identifier, deserialize_label_type, deserialize_message_topic_type,
};
use serde::{Deserialize, Serialize};

// TODO: Base64 parsing
pub type BlobType = Vec<u8>;

// TODO: Mime Parsing?
pub type ContentType = String;

pub type DateTimeUTC = chrono::DateTime<chrono::Utc>;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Identifier(#[serde(deserialize_with = "deserialize_identifier")] pub String);

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LabelType(#[serde(deserialize_with = "deserialize_label_type")] pub String);

pub type LangStringSet = Vec<LangString>;

pub type MultiLanguageNameType = LangStringSet;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MessageTopicType(
    #[serde(deserialize_with = "deserialize_message_topic_type")] pub String,
);

pub type Uri = iref::UriBuf;
pub type Iri = iref::IriBuf;
