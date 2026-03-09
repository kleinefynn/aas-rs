use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::identifiable::Identifiable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::{ContentType, Identifier, Label, Uri};
use crate::part1::v3_1::reference::Reference;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use strum::{Display, EnumString};


use crate::part1::v3_1::reference::deserialize_optional_external_reference;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]

#[serde(rename = "assetAdministrationShell")]
#[cfg_attr(
    feature = "xml",
    serde(
        from = "xml::AssetAdministrationShellXML",
        into = "xml::AssetAdministrationShellXML"
    )
)]
pub struct AssetAdministrationShell {
    #[serde(rename = "assetInformation")]
    pub asset_information: AssetInformation,

    #[serde(flatten)]
    pub identifiable: Identifiable,

    #[serde(flatten)]
    pub data_specification: HasDataSpecification,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Reference>,

    // TODO: What kind of submodel keys are supported?
    // 1. Only one Key
    // Only key type "Submodel" allowed?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submodels: Option<Vec<Reference>>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "AssetAdministrationShellMeta")]
pub struct AssetAdministrationShellMetamodel {
    #[serde(flatten)]
    pub identifiable: Identifiable,

    #[serde(flatten)]
    pub data_specification: HasDataSpecification,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "derivedFrom")]
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

// Todo: Test
impl ToJsonMetamodel for AssetAdministrationShell {
    type Error = serde_json::Error;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        let meta = AssetAdministrationShellMetamodel::from(self.clone());

        serde_json::to_string(&meta)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, EnumString, Display)]
#[serde(tag = "assetKind")]

#[cfg_attr(
    feature = "xml",
    serde(from = "xml::AssetInformationXML", into = "xml::AssetInformationXML")
)]
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
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]

pub struct AssetInformationInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "globalAssetId")]
    pub global_asset_id: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specificAssetIds")]
    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetType")]
    pub asset_type: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultThumbnail")]
    pub default_thumbnail: Option<Resource>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]

#[cfg_attr(
    feature = "xml",
    serde(from = "xml::SpecificAssetIdXML", into = "xml::SpecificAssetIdXML")
)]
pub struct SpecificAssetId {
    #[serde(flatten)]
    pub has_semantics: HasSemantics,

    pub name: Label,

    pub value: Identifier,

    /// The unique ID of the (external) subject the specific asset ID value belongs to or
    /// has meaning to
    /// Needs to be an external reference!
    /// TODO: Typesafe with Newtype pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalSubjectId")]
    #[serde(deserialize_with = "deserialize_optional_external_reference")]
    pub external_subject_id: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]

pub struct Resource {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub path: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    pub content_type: Option<ContentType>,
}

#[cfg(feature = "xml")]

