use crate::part1::v3_1::attributes::extension::HasExtensions;
use crate::part1::v3_1::primitives::MultiLanguageNameType;
use aas::part1::v3_1::primitives::Identifier;
use serde::{Deserialize, Serialize};

use aas::utilities::deserialize_empty_identifier_as_none;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct Referable {
    #[serde(skip_serializing_if = "Option::is_none")]
    // use case where "" is needed or can this be ignored?
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_empty_identifier_as_none")]
    #[serde(rename = "idShort")]
    pub id_short: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<MultiLanguageNameType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MultiLanguageNameType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub category: Option<String>,

    /// HasExtensions
    #[serde(flatten)]
    pub extensions: HasExtensions,
}
