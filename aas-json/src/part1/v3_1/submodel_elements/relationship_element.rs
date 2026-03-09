use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::submodel_elements::data_element::DataElement;
use serde::{Deserialize, Serialize};

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]

#[cfg_attr(
    feature = "xml",
    serde(
        from = "xml::RelationshipElementXMLProxy",
        into = "xml::RelationshipElementXMLProxy"
    )
)]
pub struct RelationshipElement {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    second: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]

#[cfg_attr(
    feature = "xml",
    serde(
        from = "xml::AnnotatedRelationshipElementXMLProxy",
        into = "xml::AnnotatedRelationshipElementXMLProxy"
    )
)]
pub struct AnnotatedRelationshipElement {
    // Inherited from RelationshipElement
    #[serde(flatten)]
    pub referable: Referable,

    #[serde(flatten)]
    pub semantics: HasSemantics,

    #[serde(flatten)]
    pub qualifiable: Qualifiable,

    #[serde(flatten)]
    pub embedded_data_specifications: HasDataSpecification,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Reference>,
    // --- end inheritance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<DataElement>>,
}


pub mod json {
    use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
    use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
    use crate::part1::v3_1::attributes::referable::Referable;
    use crate::part1::v3_1::attributes::semantics::HasSemantics;
    use crate::part1::v3_1::submodel_elements::relationship_element::{
        AnnotatedRelationshipElement, RelationshipElement,
    };
    use crate::part1::{MetamodelError, ToJsonMetamodel};
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "openapi")]
    use utoipa::ToSchema;

    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]

    pub struct RelationshipElementMeta {
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
    }

    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]

    #[allow(unused)]
    pub struct AnnotatedRelationshipElementMeta {
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
    }

    impl From<RelationshipElement> for RelationshipElementMeta {
        fn from(element: RelationshipElement) -> Self {
            Self {
                referable: element.referable,
                semantics: element.semantics,
                qualifiable: element.qualifiable,
                embedded_data_specifications: element.embedded_data_specifications,
            }
        }
    }

    impl From<AnnotatedRelationshipElement> for AnnotatedRelationshipElementMeta {
        fn from(element: AnnotatedRelationshipElement) -> Self {
            Self {
                referable: element.referable,
                semantics: element.semantics,
                qualifiable: element.qualifiable,
                embedded_data_specifications: element.embedded_data_specifications,
            }
        }
    }

    impl From<&RelationshipElement> for RelationshipElementMeta {
        fn from(element: &RelationshipElement) -> Self {
            let element = element.clone();
            Self {
                referable: element.referable,
                semantics: element.semantics,
                qualifiable: element.qualifiable,
                embedded_data_specifications: element.embedded_data_specifications,
            }
        }
    }
    impl From<&AnnotatedRelationshipElement> for AnnotatedRelationshipElementMeta {
        fn from(element: &AnnotatedRelationshipElement) -> Self {
            let element = element.clone();
            Self {
                referable: element.referable,
                semantics: element.semantics,
                qualifiable: element.qualifiable,
                embedded_data_specifications: element.embedded_data_specifications,
            }
        }
    }

    impl ToJsonMetamodel for RelationshipElement {
        type Error = MetamodelError;

        fn to_json_metamodel(&self) -> Result<String, Self::Error> {
            serde_json::to_string::<RelationshipElementMeta>(&self.into())
                .map_err(|e| MetamodelError::FailedSerialisation(e))
        }
    }

    impl ToJsonMetamodel for AnnotatedRelationshipElement {
        type Error = MetamodelError;

        fn to_json_metamodel(&self) -> Result<String, Self::Error> {
            serde_json::to_string::<AnnotatedRelationshipElementMeta>(&self.into())
                .map_err(|e| MetamodelError::FailedSerialisation(e))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
        use crate::part1::v3_1::attributes::referable::Referable;
        use crate::part1::v3_1::attributes::semantics::HasSemantics;
        use crate::part1::v3_1::key::Key;
        use crate::part1::v3_1::primitives::Identifier;
        use crate::part1::v3_1::reference::{Reference, ReferenceInner};

        #[test]
        fn test_relationship_element_to_metamodel() {
            // expect to remove "first" & "second" fields.
            let expected = r#"{"idShort":"relationship_test"}"#;
            let actual = RelationshipElement {
                referable: Referable {
                    id_short: Some(Identifier::try_from("relationship_test").unwrap()),
                    display_name: None,
                    description: None,
                    #[allow(deprecated)]
                    category: None,
                    extensions: Default::default(),
                },
                semantics: HasSemantics {
                    semantic_id: None,
                    supplemental_semantic_ids: None,
                },
                qualifiable: Qualifiable { qualifiers: None },
                embedded_data_specifications: Default::default(),

                first: Some(Reference::ExternalReference(ReferenceInner {
                    referred_semantic_id: None,
                    keys: vec![Key::RelationshipElement("https://example.com/1".into())],
                })),
                second: Some(Reference::ExternalReference(ReferenceInner {
                    referred_semantic_id: None,
                    keys: vec![Key::RelationshipElement("https://example.com/2".into())],
                })),
            };

            let actual = actual.to_json_metamodel().expect("Serialize to metamodel");

            assert_eq!(expected, actual);
        }
    }
}

#[cfg(feature = "xml")]

