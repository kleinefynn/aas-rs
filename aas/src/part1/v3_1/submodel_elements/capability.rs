use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Capability {
    // Inherited from DataElement
    pub referable: Referable,
    pub semantics: HasSemantics,
    pub qualifiable: Qualifiable,
    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
}

impl Capability {
    pub fn new() -> Self {
        Self {
            referable: Referable::default(),
            semantics: HasSemantics::default(),
            qualifiable: Qualifiable::default(),
            embedded_data_specifications: HasDataSpecification::default(),
        }
    }
}
