use crate::part1::MetamodelError;
use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::LangString;
use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::attributes::qualifiable::Qualifiable;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::reference::Reference;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct MultiLanguageProperty {
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
    pub value: Option<Vec<LangString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueId")]
    pub value_id: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]

pub struct MultiLanguagePropertyMeta {
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

impl From<MultiLanguageProperty> for MultiLanguagePropertyMeta {
    fn from(m: MultiLanguageProperty) -> Self {
        Self {
            referable: m.referable,
            semantics: m.semantics,
            qualifiable: m.qualifiable,
            embedded_data_specifications: m.embedded_data_specifications,
        }
    }
}

impl From<&MultiLanguageProperty> for MultiLanguagePropertyMeta {
    fn from(m: &MultiLanguageProperty) -> Self {
        let m = m.clone();
        Self {
            referable: m.referable,
            semantics: m.semantics,
            qualifiable: m.qualifiable,
            embedded_data_specifications: m.embedded_data_specifications,
        }
    }
}

impl ToJsonMetamodel for MultiLanguageProperty {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        let meta = MultiLanguagePropertyMeta::from(self);
        serde_json::to_string(&meta).map_err(|e| MetamodelError::FailedSerialisation(e))
    }
}
