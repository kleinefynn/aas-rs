use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::submodel_elements::SubmodelElement;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "xml",
    serde(
        from = "xml::SubmodelElementCollectionXMLProxy",
        into = "xml::SubmodelElementCollectionXMLProxy"
    )
)]
pub struct SubmodelElementCollection {
    pub referable: Referable,

    // HasSemantics
    pub semantics: HasSemantics,

    // Qualifiable
    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,

    value: Option<Vec<SubmodelElement>>,
}

impl ToJsonMetamodel for SubmodelElementCollection {
    type Error = ();

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        Ok(format!(r#"{{"modelType":"SubmodelElementCollection"}}"#))
    }
}

pub(crate) mod xml {
    use crate::part1::v3_1::attributes::data_specification::{
        EmbeddedDataSpecification, HasDataSpecification,
    };
    use crate::part1::v3_1::attributes::extension::{Extension, HasExtensions};
    use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
    use crate::part1::v3_1::attributes::referable::Referable;
    use crate::part1::v3_1::attributes::semantics::HasSemantics;
    use crate::part1::v3_1::primitives::Identifier;
    use crate::part1::v3_1::primitives::xml::LangStringTextType;
    use crate::part1::v3_1::submodel_elements::{SubmodelElement, SubmodelElementCollection};
    use crate::utilities::deserialize_empty_identifier_as_none;

    #[derive(Debug)]
    pub(crate) struct SubmodelElementCollectionXMLProxy {
        // Inherited from DataElement

        // use case where "" is needed or can this be ignored?
        pub id_short: Option<Identifier>,

        pub display_name: Option<LangStringTextType>,

        pub description: Option<LangStringTextType>,

        #[deprecated]
        pub category: Option<String>,

        pub extension: Option<Vec<Extension>>,

        pub semantics: Option<HasSemantics>,

        pub qualifiers: Option<Qualifiable>,

        embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,

        value: Option<ValueXMLWrapper>,
    }

    #[derive(Debug)]
    struct ValueXMLWrapper {
        value: Vec<SubmodelElement>,
    }

    impl From<SubmodelElementCollectionXMLProxy> for SubmodelElementCollection {
        fn from(value: SubmodelElementCollectionXMLProxy) -> Self {
            Self {
                referable: Referable {
                    id_short: value.id_short,
                    display_name: value.display_name.map(LangStringTextType::into),
                    description: value.description.map(LangStringTextType::into),
                    #[allow(deprecated)]
                    category: value.category,
                    extensions: HasExtensions {
                        extension: value.extension,
                    },
                },
                semantics: value.semantics.unwrap_or_default(),
                qualifiable: Qualifiable {
                    qualifiers: value.qualifiers.and_then(|v| v.qualifiers),
                },
                embedded_data_specifications: HasDataSpecification {
                    embedded_data_specifications: value.embedded_data_specifications,
                },
                value: value.value.map(|v| v.value),
            }
        }
    }
    impl From<SubmodelElementCollection> for SubmodelElementCollectionXMLProxy {
        fn from(value: SubmodelElementCollection) -> Self {
            Self {
                id_short: value.referable.id_short,
                display_name: value
                    .referable
                    .display_name
                    .map(|values| LangStringTextType { values }),
                description: value
                    .referable
                    .description
                    .map(|values| LangStringTextType { values }),
                #[allow(deprecated)]
                category: value.referable.category,
                extension: value.referable.extensions.extension,
                semantics: Some(value.semantics),
                qualifiers: Some(value.qualifiable),
                embedded_data_specifications: value
                    .embedded_data_specifications
                    .embedded_data_specifications,
                value: value.value.map(|v| ValueXMLWrapper { value: v }),
            }
        }
    }
}
