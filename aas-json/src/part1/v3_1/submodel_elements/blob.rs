use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::ContentType;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct Blob {
    // Inherited from DataElement
    #[serde(flatten)]
    pub referable: Referable,

    #[serde(flatten)]
    pub semantics: HasSemantics,

    #[serde(flatten)]
    pub qualifiable: Qualifiable,

    #[serde(flatten)]
    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO "contentEncoding": "base64"
    pub value: Option<String>,

    // TODO typing. Add constraints. New type..
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
}

impl Blob {
    pub fn new(value: Option<String>, content_type: String) -> Self {
        Self {
            referable: Referable::default(),
            semantics: HasSemantics::default(),
            qualifiable: Qualifiable::default(),
            embedded_data_specifications: HasDataSpecification::default(),
            value,
            content_type,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct BlobMeta {
    // Inherited from DataElement
    #[serde(flatten)]
    pub referable: Referable,

    #[serde(flatten)]
    pub semantics: HasSemantics,

    #[serde(flatten)]
    pub qualifiable: Qualifiable,

    #[serde(flatten)]
    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
}

impl From<Blob> for BlobMeta {
    fn from(blob: Blob) -> Self {
        Self {
            referable: blob.referable,
            semantics: blob.semantics,
            qualifiable: blob.qualifiable,
            embedded_data_specifications: blob.embedded_data_specifications,
        }
    }
}

impl From<&Blob> for BlobMeta {
    fn from(blob: &Blob) -> Self {
        blob.clone().into()
    }
}

impl ToJsonMetamodel for Blob {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        serde_json::to_string::<BlobMeta>(&self.into())
            .map_err(|e| MetamodelError::FailedSerialisation(e))
    }
}
