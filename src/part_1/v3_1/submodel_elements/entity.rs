use crate::part_1::ToJsonMetamodel;
use crate::part_1::v3_1::core::SpecificAssetId;
use crate::part_1::v3_1::primitives::Identifier;
use crate::part_1::v3_1::submodel_elements::SubmodelElement;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;
use crate::part_1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part_1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part_1::v3_1::attributes::referable::Referable;
use crate::part_1::v3_1::attributes::semantics::HasSemantics;

/// The entity submodel element is designed to be used in submodels defining the relationship between the parts of the composite asset
/// it is composed of (e.g. bill of material).
/// These parts are called entities. Not all entities have a global asset ID.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(tag = "entityType")]
pub enum Entity {
    /// There is no separate Asset Administration Shell for co-managed entities.
    /// Co-managed entities need to be part of a self-managed entity.
    CoManagedEntity(EntityInner),

    /// Self-managed entities have their own Asset Administration Shell but can be part of another composite self-managed entity.
    /// The asset represented by an Asset Administration Shell is a self-managed entity per definition
    SelfManagedEntity(EntityInner),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct EntityInner {
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

    /// Statement applicable to the entity,
    /// each statement described by submodel element - typically with a qualified value
    pub statements: Option<Vec<SubmodelElement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "globalAssetId")]
    pub global_asset_id: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specificAssetId")]
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

#[cfg(test)]
mod tests {
    use crate::part_1::v3_1::attributes::referable::Referable;
    use crate::part_1::v3_1::primitives::Identifier;
    use crate::part_1::v3_1::submodel_elements::entity::{Entity, EntityInner};

    #[test]
    fn deserialize_simple() {
        let json = r#"
        {
            "idShort": "Example1",
            "entityType": "SelfManagedEntity",
            "globalAssetId": "https://example.com"
        }"#;
        
        let expected = Entity::SelfManagedEntity(EntityInner {
            referable: Referable {
                id_short: Some(Identifier::try_from("Example1").unwrap()),
                ..Default::default()
            },
            global_asset_id: Some(Identifier::try_from("https://example.com").unwrap()),
            ..Default::default()
        });
        
        let actual = serde_json::from_str::<Entity>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
