use crate::part1::v3_1::concept_description::ConceptDescription;
use crate::part1::v3_1::core::{AssetAdministrationShell, Submodel};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "xml",
    serde(from = "xml::EnvironmentXMLProxy", into = "xml::EnvironmentXMLProxy")
)]
pub struct Environment {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetAdministrationShells")]
    pub asset_administration_shells: Option<Vec<AssetAdministrationShell>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submodels: Option<Vec<Submodel>>,

    #[serde(rename = "conceptDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concept_descriptions: Option<Vec<ConceptDescription>>,
}

#[cfg(feature = "xml")]

