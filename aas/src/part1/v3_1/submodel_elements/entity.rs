use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::core::SpecificAssetId;
use crate::part1::v3_1::primitives::Identifier;
use crate::part1::v3_1::submodel_elements::SubmodelElement;

use strum::{Display, EnumString};

/// The entity submodel element is designed to be used in submodels defining the relationship between the parts of the composite asset
/// it is composed of (e.g. bill of material).
/// These parts are called entities. Not all entities have a global asset ID.
#[derive(Clone, PartialEq, Debug, Display, EnumString)]
pub enum Entity {
    /// There is no separate Asset Administration Shell for co-managed entities.
    /// Co-managed entities need to be part of a self-managed entity.
    CoManagedEntity(EntityInner),

    /// Self-managed entities have their own Asset Administration Shell but can be part of another composite self-managed entity.
    /// The asset represented by an Asset Administration Shell is a self-managed entity per definition
    SelfManagedEntity(EntityInner),
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct EntityInner {
    // Inherited from DataElement
    pub referable: Referable,

    pub semantics: HasSemantics,

    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
    // ----- end inheritance
    /// Statement applicable to the entity,
    /// each statement described by submodel element - typically with a qualified value
    pub statements: Option<Vec<SubmodelElement>>,

    pub global_asset_id: Option<Identifier>,

    pub specific_asset_id: Option<Vec<SpecificAssetId>>,
}

impl ToJsonMetamodel for Entity {
    type Error = ();

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        // TODO: Add modelType
        match self {
            Entity::CoManagedEntity(_) => Ok(r#"{"entityType":"CoManagedEntity"}"#.into()),
            Entity::SelfManagedEntity(_) => Ok(r#"{"entityType":"SelfManagedEntity"}"#.into()),
        }
    }
}
