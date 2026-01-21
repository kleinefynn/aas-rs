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
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "xml", serde(
    from = "xml::SubmodelElementListXMLProxy",
    into = "xml::SubmodelElementListXMLProxy"
))]
pub struct SubmodelElementList {
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

mod xml {
    use super::SubmodelElementList;
    use crate::part1::v3_1::primitives::data_type_def_xs::DataTypeXSDef;
    use crate::part1::v3_1::reference::Reference;
    use crate::part1::v3_1::submodel_elements::submodel_element_list::ordering_default;
    use crate::part1::v3_1::submodel_elements::{AasSubmodelElements, SubmodelElement};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct SubmodelElementListXMLProxy {
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

    impl From<SubmodelElementList> for SubmodelElementListXMLProxy {
        fn from(value: SubmodelElementList) -> Self {
            Self {
                is_order_relevant: false,
                semantic_id_list_element: None,
                value: None,
                type_value_list_element: AasSubmodelElements::RelationshipElement,
                value_type_list_element: None,
            }
        }
    }
    impl From<SubmodelElementListXMLProxy> for SubmodelElementList {
        fn from(value: SubmodelElementListXMLProxy) -> Self {
            Self {
                is_order_relevant: false,
                semantic_id_list_element: None,
                value: None,
                type_value_list_element: AasSubmodelElements::RelationshipElement,
                value_type_list_element: None,
            }
        }
    }
}


#[cfg(not(feature = "xml"))]
#[cfg(test)]
mod tests {
    use crate::part1::v3_1::submodel_elements::SubmodelElementList;

    #[test]
    fn deserialize_complex() {
        let json = r#"
            {
              "idShort": "PcfCalculationMethods",
              "displayName": [
                {
                  "language": "en",
                  "text": "impact assessment methods"
                }
              ],
              "description": [
                {
                  "language": "en",
                  "text": "Standards, methods for determining the greenhouse gas emissions of a product."
                }
              ],
              "semanticId": {
                "type": "ExternalReference",
                "keys": [
                  {
                    "type": "GlobalReference",
                    "value": "https://admin-shell.io/idta/CarbonFootprint/PcfCalculationMethods/1/0"
                  }
                ]
              },
              "qualifiers": [
                {
                  "type": "SMT/Cardinality",
                  "valueType": "xs:string",
                  "value": "One"
                }
              ],
              "orderRelevant": true,
              "semanticIdListElement": {
                "type": "ExternalReference",
                "keys": [
                  {
                    "type": "GlobalReference",
                    "value": "0173-1#02-ABG854#003"
                  }
                ]
              },
              "typeValueListElement": "Property",
              "valueTypeListElement": "xs:string",
              "value": [
                {
                  "category": "PARAMETER",
                  "displayName": [
                    {
                      "language": "en",
                      "text": "impact assessment method / calculation method"
                    }
                  ],
                  "description": [
                    {
                      "language": "en",
                      "text": "Standard, method for determining the greenhouse gas emissions of a product."
                    }
                  ],
                  "semanticId": {
                    "type": "ExternalReference",
                    "keys": [
                      {
                        "type": "GlobalReference",
                        "value": "0173-1#02-ABG854#003"
                      }
                    ]
                  },
                  "qualifiers": [
                    {
                      "type": "SMT/Cardinality",
                      "valueType": "xs:string",
                      "value": "One"
                    }
                  ],
                  "valueType": "xs:string",
                  "value": "in-house method - see documentation",
                  "modelType": "Property"
                }
              ],
              "modelType": "SubmodelElementList"
            }
        "#;

        serde_json::from_str::<SubmodelElementList>(json).unwrap();
    }
}
