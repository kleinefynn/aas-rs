use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::submodel_elements::SubmodelElement;
use crate::part1::{MetamodelError, ToJsonMetamodel};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Operation {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    pub input_variable: Option<Box<SubmodelElement>>,

    pub output_variable: Option<Box<SubmodelElement>>,

    pub inoutput_variable: Option<Box<SubmodelElement>>,
}
