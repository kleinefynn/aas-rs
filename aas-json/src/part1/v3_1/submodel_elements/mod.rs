mod basic_event;
mod blob;
mod capability;
mod data_element;
mod entity;
mod file;
mod multi_language_property;
mod operation;
mod property;
mod range;
mod reference_element;
mod relationship_element;
mod submodel_element_collection;

use crate::utilities::deserialize_empty_identifier_as_none;
use std::collections::HashMap;
pub use submodel_element_collection::*;
mod submodel_element_list;
pub use submodel_element_list::*;

use crate::part1::v3_1::attributes::data_specification::{
    EmbeddedDataSpecification, HasDataSpecification,
};
use crate::part1::v3_1::attributes::extension::Extension;
use crate::part1::v3_1::attributes::qualifiable::{Qualifiable, Qualifier};
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::Identifier;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::submodel_elements::basic_event::BasicEventElement;
pub use crate::part1::v3_1::submodel_elements::blob::Blob;
pub use crate::part1::v3_1::submodel_elements::capability::Capability;
use crate::part1::v3_1::submodel_elements::data_element::DataElement;
use crate::part1::v3_1::submodel_elements::entity::Entity;
use crate::part1::v3_1::submodel_elements::file::File;
use crate::part1::v3_1::submodel_elements::multi_language_property::MultiLanguageProperty;
use crate::part1::v3_1::submodel_elements::operation::Operation;
use crate::part1::v3_1::submodel_elements::property::Property;
use crate::part1::v3_1::submodel_elements::range::Range;
use crate::part1::v3_1::submodel_elements::reference_element::ReferenceElement;
use crate::part1::v3_1::submodel_elements::relationship_element::{
    AnnotatedRelationshipElement, RelationshipElement,
};
use crate::part1::{MetamodelError, ToJsonMetamodel};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::part1::v3_1::primitives::xml::LangStringTextType;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// alias are made to support for camelCase, PascalCase and lowercase.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(tag = "modelType")]
pub enum SubmodelElement {
    #[serde(
        alias = "RelationshipElement",
        alias = "relationshipElement",
        alias = "relationshipelement"
    )]
    RelationshipElement(RelationshipElement),
    #[serde(
        alias = "AnnotatedRelationshipElement",
        alias = "annotatedRelationshipElement",
        alias = "annotatedrelationshipelement"
    )]
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    #[serde(
        alias = "BasicEventElement",
        alias = "basicEventElement",
        alias = "basiceventelement"
    )]
    BasicEventElement(BasicEventElement),
    #[serde(alias = "Blob", alias = "blob", alias = "blob")]
    Blob(Blob),
    #[serde(alias = "Capability", alias = "capability", alias = "capability")]
    Capability(Capability),
    // TODO: is this needed? Deserializes??
    #[serde(alias = "DataElement", alias = "dataElement", alias = "dataelement")]
    DataElement(DataElement),
    #[serde(alias = "Entity", alias = "entity", alias = "entity")]
    Entity(Entity),
    #[serde(alias = "File", alias = "file", alias = "file")]
    File(File),
    #[serde(
        alias = "MultiLanguageProperty",
        alias = "multiLanguageProperty",
        alias = "multilanguageproperty"
    )]
    MultiLanguageProperty(MultiLanguageProperty),
    #[serde(alias = "Operation", alias = "operation", alias = "operation")]
    Operation(Operation),
    #[serde(alias = "Property", alias = "property", alias = "property")]
    Property(Property),
    #[serde(alias = "Range", alias = "range", alias = "range")]
    Range(Range),
    #[serde(
        alias = "ReferenceElement",
        alias = "referenceElement",
        alias = "referenceelement"
    )]
    ReferenceElement(ReferenceElement),
    #[serde(
        alias = "SubmodelElementCollection",
        alias = "submodelElementCollection",
        alias = "submodelelementcollection"
    )]
    SubmodelElementCollection(SubmodelElementCollection),
    #[serde(
        alias = "SubmodelElementList",
        alias = "submodelElementList",
        alias = "submodelelementlist"
    )]
    SubmodelElementList(SubmodelElementList),
}

/// Every SubmodelElement has these
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]

pub struct SubmodelElementFields {
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
}

// fields for submodel elements
#[derive(Serialize, Deserialize)]
pub struct SubmodelElementFieldsXML {
    // Inherited from DataElement
    #[serde(skip_serializing_if = "Option::is_none")]
    // use case where "" is needed or can this be ignored?
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_empty_identifier_as_none")]
    #[serde(rename = "idShort")]
    pub id_short: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<LangStringTextType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<LangStringTextType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extensions")]
    pub extension: Option<Vec<Extension>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "semanticId")]
    pub semantic_id: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supplementalSemanticIds")]
    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
}

// maybe without variants?
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Display)]

pub enum AasSubmodelElements {
    RelationshipElement,
    AnnotatedRelationshipElement,
    BasicEventElement,
    Blob,
    Capability,
    // TODO: is this needed? Deserializes??
    DataElement,
    Entity,
    File,
    MultiLanguageProperty,
    Operation,
    Property,
    Range,
    ReferenceElement,
    #[serde(alias = "SubmodelElementCollection")]
    SubmodelElementCollection,
    SubmodelElementList,
}

impl ToJsonMetamodel for SubmodelElement {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        let data = match self {
            SubmodelElement::RelationshipElement(_) => Err(MetamodelError::MetamodelNotSupported),
            SubmodelElement::AnnotatedRelationshipElement(_) => {
                Err(MetamodelError::MetamodelNotSupported)
            }
            SubmodelElement::BasicEventElement(elm) => elm.to_json_metamodel(),
            SubmodelElement::Blob(elm) => elm.to_json_metamodel(),
            SubmodelElement::Capability(elm) => elm.to_json_metamodel(),
            SubmodelElement::DataElement(elm) => elm.to_json_metamodel(),
            SubmodelElement::Entity(elm) => Ok(elm.to_json_metamodel().unwrap()),
            SubmodelElement::File(elm) => elm.to_json_metamodel(),
            SubmodelElement::MultiLanguageProperty(elm) => elm.to_json_metamodel(),
            SubmodelElement::Operation(elm) => elm.to_json_metamodel(),
            SubmodelElement::Property(elm) => elm.to_json_metamodel(),
            SubmodelElement::Range(elm) => Ok(elm.to_json_metamodel().unwrap()),
            SubmodelElement::ReferenceElement(elm) => Ok(elm.to_json_metamodel().unwrap()),
            SubmodelElement::SubmodelElementCollection(elm) => Ok(elm.to_json_metamodel().unwrap()),
            SubmodelElement::SubmodelElementList(elm) => elm.to_json_metamodel(),
        };

        data.and_then(|str| {
            let mut map: HashMap<&str, serde_json::Value> = serde_json::from_str(&str).unwrap();
            map.insert("modelType", serde_json::Value::String(self.to_string()));
            Ok(serde_json::to_string(&map).unwrap())
        })
    }
}
