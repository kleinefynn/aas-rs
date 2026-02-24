use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::ContentType;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Blob {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance

    // TODO "contentEncoding": "base64"
    pub value: Option<String>,

    // TODO typing. Add constraints. New type..
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

#[derive(Clone, PartialEq, Debug, Default)]
pub struct BlobMeta {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

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
