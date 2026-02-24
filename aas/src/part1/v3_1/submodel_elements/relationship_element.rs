use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::submodel_elements::data_element::DataElement;

#[derive(Clone, PartialEq, Debug)]
pub struct RelationshipElement {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    first: Option<Reference>,

    second: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnnotatedRelationshipElement {
    // Inherited from RelationshipElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,

    pub first: Option<Reference>,

    pub second: Option<Reference>,
    // --- end inheritance
    pub annotations: Option<Vec<DataElement>>,
}
