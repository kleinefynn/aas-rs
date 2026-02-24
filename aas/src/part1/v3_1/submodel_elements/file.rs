use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::{ContentType, Uri};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct File {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    /// Path and name of the file (with file extension)
    /// The path can be absolute or relative.
    pub value: Option<Uri>,

    pub content_type: Option<ContentType>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct FileMeta {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
}

impl From<File> for FileMeta {
    fn from(file: File) -> Self {
        Self {
            referable: file.referable,
            semantics: file.semantics,
            qualifiable: file.qualifiable,
            embedded_data_specifications: file.embedded_data_specifications,
        }
    }
}

impl From<&File> for FileMeta {
    fn from(file: &File) -> Self {
        file.clone().into()
    }
}
