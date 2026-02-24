use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::data_type_def_xs::DataXsd;

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    pub value: DataXsd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyMeta {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
}

impl From<Property> for PropertyMeta {
    fn from(prop: Property) -> Self {
        Self {
            referable: prop.referable,
            semantics: prop.semantics,
            qualifiable: prop.qualifiable,
            embedded_data_specifications: prop.embedded_data_specifications,
        }
    }
}

impl From<&Property> for PropertyMeta {
    fn from(prop: &Property) -> Self {
        let prop = prop.clone();
        Self {
            referable: prop.referable,
            semantics: prop.semantics,
            qualifiable: prop.qualifiable,
            embedded_data_specifications: prop.embedded_data_specifications,
        }
    }
}
