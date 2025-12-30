use crate::part_1::v3_1::attributes::extension::HasExtensions;
use crate::part_1::v3_1::primitives::{Identifier, MultiLanguageNameType};
use crate::utilities::deserialize_empty_identifier_as_none;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
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
