use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::data_type_def_xs::DataXsd;
use crate::part1::v3_1::reference::Reference;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]

#[cfg_attr(feature = "xml", serde(rename = "qualifiers"))]
pub struct Qualifiable {
    #[serde(skip_serializing_if = "Option::is_none")]

    pub qualifiers: Option<Vec<Qualifier>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]

#[cfg_attr(
    feature = "xml",
    serde(from = "xml::QualifierXMLProxy", into = "xml::QualifierXMLProxy")
)]
pub enum Qualifier {
    ConceptQualifier(QualifierInner),
    TemplateQualifier(QualifierInner),
    ValueQualifier(QualifierInner),

    /// unknown values (kind = null!)
    #[serde(untagged)]
    Unknown(QualifierInner),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct QualifierInner {
    #[serde(flatten)]
    pub semantics: HasSemantics,

    // TODO: Text parsing
    #[serde(rename = "type")]
    pub ty: String,

    #[serde(flatten)]
    pub value: DataXsd,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueId")]
    pub value_id: Option<Reference>,
}

#[cfg(feature = "xml")]

