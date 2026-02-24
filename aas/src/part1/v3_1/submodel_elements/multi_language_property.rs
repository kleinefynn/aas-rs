use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::LangString;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::reference::Reference;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct MultiLanguageProperty {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    pub value: Option<Vec<LangString>>,

    pub value_id: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct MultiLanguagePropertyMeta {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
}

impl From<MultiLanguageProperty> for MultiLanguagePropertyMeta {
    fn from(m: MultiLanguageProperty) -> Self {
        Self {
            referable: m.referable,
            semantics: m.semantics,
            qualifiable: m.qualifiable,
            embedded_data_specifications: m.embedded_data_specifications,
        }
    }
}

impl From<&MultiLanguageProperty> for MultiLanguagePropertyMeta {
    fn from(m: &MultiLanguageProperty) -> Self {
        let m = m.clone();
        Self {
            referable: m.referable,
            semantics: m.semantics,
            qualifiable: m.qualifiable,
            embedded_data_specifications: m.embedded_data_specifications,
        }
    }
}
