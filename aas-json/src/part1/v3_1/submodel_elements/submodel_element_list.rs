use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::data_type_def_xs::DataTypeXSDef;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::submodel_elements::{AasSubmodelElements, SubmodelElement};
use crate::part1::{MetamodelError, ToJsonMetamodel};
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;
// TODO: TYPING
// We could make the pair value / type_value_list_element one enum
// Deserialize check for constraints.

/// A submodel element list is an ordered list of submodel elements.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SubmodelElementList {
    #[serde(flatten)]
    pub referable: Referable,

    // HasSemantics
    #[serde(flatten)]
    pub semantics: HasSemantics,

    // Qualifiable
    #[serde(flatten)]
    pub qualifiable: Qualifiable,

    #[serde(flatten)]
    pub embedded_data_specifications: HasDataSpecification,

    /// Defines whether order in list is relevant. If orderRelevant = false, the list represents a set or a bag.
    #[serde(rename = "orderRelevant")]
    #[serde(default = "ordering_default")]
    is_order_relevant: bool,

    /// Semantic ID which the submodel elements contained in the list match
    #[serde(rename = "semanticIdListElement")]
    semantic_id_list_element: Option<Reference>,

    // Question: can value, type_value_list_element be merged into an enum?
    // maybe together with value_type_list_element?
    // newtype or something for type safety.
    /// Submodel elements contained in the list
    value: Option<Vec<SubmodelElement>>,

    /// The submodel element type of the submodel elements contained in the list
    #[serde(rename = "typeValueListElement")]
    type_value_list_element: AasSubmodelElements,

    /// The value type of the submodel element contained in the list
    #[serde(rename = "valueTypeListElement")]
    value_type_list_element: Option<DataTypeXSDef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SubmodelElementListMeta {
    /// Defines whether order in list is relevant. If orderRelevant = false, the list represents a set or a bag.
    #[serde(rename = "orderRelevant")]
    #[serde(default = "ordering_default")]
    is_order_relevant: bool,

    /// Semantic ID which the submodel elements contained in the list match
    #[serde(rename = "semanticIdListElement")]
    semantic_id_list_element: Option<Reference>,

    /// The submodel element type of the submodel elements contained in the list
    #[serde(rename = "typeValueListElement")]
    type_value_list_element: AasSubmodelElements,

    /// The value type of the submodel element contained in the list
    #[serde(rename = "valueTypeListElement")]
    value_type_list_element: Option<DataTypeXSDef>,
}

fn ordering_default() -> bool {
    true
}

impl From<SubmodelElementList> for SubmodelElementListMeta {
    fn from(element: SubmodelElementList) -> Self {
        Self {
            is_order_relevant: element.is_order_relevant,
            semantic_id_list_element: element.semantic_id_list_element,
            type_value_list_element: element.type_value_list_element,
            value_type_list_element: element.value_type_list_element,
        }
    }
}

impl From<&SubmodelElementList> for SubmodelElementListMeta {
    fn from(element: &SubmodelElementList) -> Self {
        element.clone().into()
    }
}

impl ToJsonMetamodel for SubmodelElementList {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        serde_json::to_string::<SubmodelElementListMeta>(&self.into())
            .map_err(|e| MetamodelError::FailedSerialisation(e))
    }
}
