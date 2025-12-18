pub mod data_type_def_xs;
mod identifier;
mod label;
pub mod lang_string;
mod message_topic;

pub use identifier::*;
pub use label::*;
pub use message_topic::*;

use crate::part_1::v3_1::LangString;

// TODO: Base64 parsing
pub type BlobType = Vec<u8>;

// TODO: Mime Parsing?
pub type ContentType = String;

pub type DateTimeUTC = iso8601::DateTime;

pub type LangStringSet = Vec<LangString>;

pub type MultiLanguageNameType = LangStringSet;

// UriBuf/IriBuf or UriRefBuf/IriRefBuf?

pub type Uri = iref::UriRefBuf;
pub type Iri = iref::IriRefBuf;
