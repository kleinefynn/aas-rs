use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::identifiable::Identifiable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::{ContentType, Identifier, Label, Uri};
use crate::part1::v3_1::reference::Reference;

use std::ops::Deref;
use strum::{Display, EnumString};

#[derive(Clone, PartialEq, Debug)]
pub struct AssetAdministrationShell {
    pub asset_information: AssetInformation,

    pub identifiable: Identifiable,

    pub data_specification: HasDataSpecification,

    pub derived_from: Option<Reference>,

    // TODO: What kind of submodel keys are supported?
    // 1. Only one Key
    // Only key type "Submodel" allowed?
    pub submodels: Option<Vec<Reference>>,
}

#[derive(Clone, PartialEq, Debug)]

pub struct AssetAdministrationShellMetamodel {
    pub identifiable: Identifiable,

    pub data_specification: HasDataSpecification,

    pub derived_from: Option<Reference>,
}

impl From<AssetAdministrationShell> for AssetAdministrationShellMetamodel {
    fn from(value: AssetAdministrationShell) -> Self {
        Self {
            identifiable: value.identifiable,
            data_specification: value.data_specification,
            derived_from: value.derived_from,
        }
    }
}

#[derive(Clone, PartialEq, Debug, EnumString, Display)]
pub enum AssetInformation {
    Instance(AssetInformationInner),
    NotApplicable(AssetInformationInner),
    Role(AssetInformationInner),
    Type(AssetInformationInner),
}

impl Deref for AssetInformation {
    type Target = AssetInformationInner;

    fn deref(&self) -> &Self::Target {
        match self {
            AssetInformation::Instance(i)
            | AssetInformation::NotApplicable(i)
            | AssetInformation::Role(i)
            | AssetInformation::Type(i) => i,
        }
    }
}

// TODO: Skip option serialization
#[derive(Clone, PartialEq, Debug, Default)]

pub struct AssetInformationInner {
    pub global_asset_id: Option<Identifier>,

    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,

    pub asset_type: Option<Identifier>,

    pub default_thumbnail: Option<Resource>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SpecificAssetId {
    pub has_semantics: HasSemantics,

    pub name: Label,

    pub value: Identifier,

    /// The unique ID of the (external) subject the specific asset ID value belongs to or
    /// has meaning to
    /// Needs to be an external reference!
    /// TODO: Typesafe with Newtype pattern
    pub external_subject_id: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Resource {
    pub path: Uri,
    pub content_type: Option<ContentType>,
}
