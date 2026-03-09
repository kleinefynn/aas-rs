use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::identifiable::Identifiable;
use crate::part1::v3_1::attributes::kind::ModellingKind;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::submodel_elements::SubmodelElement;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// make it an enum of ModellingKind?
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]

#[cfg_attr(
    feature = "xml",
    serde(from = "xml::SubmodelXML", into = "xml::SubmodelXML")
)]
pub struct Submodel {
    #[serde(flatten)]
    pub identifiable: Identifiable,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModellingKind>,

    #[serde(flatten)]
    pub semantics: HasSemantics,

    #[serde(flatten)]
    pub qualifier: Qualifiable,

    #[serde(flatten)]
    pub data_specification: HasDataSpecification,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "submodelElements")]
    pub submodel_elements: Option<Vec<SubmodelElement>>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SubmodelMeta {
    #[serde(flatten)]
    pub identifiable: Identifiable,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModellingKind>,

    #[serde(flatten)]
    pub semantics: HasSemantics,

    #[serde(flatten)]
    pub qualifier: Qualifiable,

    #[serde(flatten)]
    pub data_specification: HasDataSpecification,
}

impl From<Submodel> for SubmodelMeta {
    fn from(value: Submodel) -> Self {
        Self {
            identifiable: value.identifiable,
            kind: value.kind,
            semantics: value.semantics,
            qualifier: value.qualifier,
            data_specification: value.data_specification,
        }
    }
}

// Todo: Test
impl ToJsonMetamodel for Submodel {
    type Error = serde_json::Error;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        let meta = SubmodelMeta::from(self.clone());

        serde_json::to_string(&meta)
    }
}
#[cfg(feature = "xml")]

