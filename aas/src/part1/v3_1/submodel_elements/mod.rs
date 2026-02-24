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

use std::collections::HashMap;
pub use submodel_element_collection::*;
mod submodel_element_list;
pub use submodel_element_list::*;

use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
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

use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SubmodelElement {
    RelationshipElement(RelationshipElement),

    AnnotatedRelationshipElement(AnnotatedRelationshipElement),

    BasicEventElement(BasicEventElement),

    Blob(Blob),

    Capability(Capability),
    // TODO: is this needed? Deserializes??
    DataElement(DataElement),

    Entity(Entity),

    File(File),

    MultiLanguageProperty(MultiLanguageProperty),

    Operation(Operation),

    Property(Property),

    Range(Range),

    ReferenceElement(ReferenceElement),

    SubmodelElementCollection(SubmodelElementCollection),

    SubmodelElementList(SubmodelElementList),
}

/// Every SubmodelElement has these
#[derive(Debug, Clone, PartialEq, Default)]

pub struct SubmodelElementFields {
    pub referable: Referable,

    // HasSemantics
    pub semantics: HasSemantics,

    // Qualifiable
    pub qualifiable: Qualifiable,

    pub embedded_data_specifications: HasDataSpecification,
}

// maybe without variants?
#[derive(Debug, Clone, PartialEq, Display, EnumString)]
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

    SubmodelElementCollection,
    SubmodelElementList,
}
