use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::identifiable::Identifiable;
use crate::part1::v3_1::attributes::kind::ModellingKind;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::submodel_elements::SubmodelElement;

// make it an enum of ModellingKind?
#[derive(Clone, PartialEq, Debug)]
pub struct Submodel {
    pub identifiable: Identifiable,

    pub kind: Option<ModellingKind>,

    pub semantics: HasSemantics,

    pub qualifier: Qualifiable,

    pub data_specification: HasDataSpecification,

    pub submodel_elements: Option<Vec<SubmodelElement>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SubmodelMeta {
    pub identifiable: Identifiable,
    pub kind: Option<ModellingKind>,

    pub semantics: HasSemantics,

    pub qualifier: Qualifiable,

    pub data_specification: HasDataSpecification,
}

impl From<Submodel> for SubmodelMeta {
    fn from(value: Submodel) -> Self {
        Self {
            identifiable: value.identifiable,
            kind: value.kind,
            semantics: value.semantics,
            qualifier: value.qualifier,
            data_specification: value.data_specification,
        }
    }
}
