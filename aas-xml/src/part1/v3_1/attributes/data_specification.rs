use crate::part1::v3_1::level_type::LevelType;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::value_list::ValueList;
use serde::{Deserialize, Serialize};

use crate::part1::v3_1::attributes::data_specification::{
    DataSpecificationContent, DataSpecificationIec61360, DataTypeIec61360,
    EmbeddedDataSpecification,
};
use crate::part1::v3_1::level_type::LevelType;
use crate::part1::v3_1::primitives::xml::LangStringTextType;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::reference::deserialize_external_reference;
use crate::part1::v3_1::value_list::ValueList;
use aas::part1::v3_1::attributes::data_specification::{
    DataSpecificationContent, DataSpecificationIec61360, EmbeddedDataSpecification,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct EmbeddedDataSpecificationXMLProxy {
    #[serde(rename = "dataSpecification")]
    #[serde(deserialize_with = "deserialize_external_reference")]
    pub data_specification: Reference,

    #[serde(rename = "dataSpecificationContent")]
    pub data_specification_content: Wrapper,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Wrapper {
    #[serde(rename = "$value")]
    content: DataSpecificationContent,
}

impl From<EmbeddedDataSpecificationXMLProxy> for EmbeddedDataSpecification {
    fn from(value: EmbeddedDataSpecificationXMLProxy) -> Self {
        Self {
            data_specification: value.data_specification,
            data_specification_content: value.data_specification_content.content,
        }
    }
}

impl From<EmbeddedDataSpecification> for EmbeddedDataSpecificationXMLProxy {
    fn from(value: EmbeddedDataSpecification) -> Self {
        Self {
            data_specification: value.data_specification,
            data_specification_content: Wrapper {
                content: value.data_specification_content,
            },
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DataSpecificationIec61360XMLProxy {
    #[serde(rename = "preferredName")]
    pub preferred_name: LangStringTextType,

    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<LangStringTextType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(rename = "unitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<Reference>,

    #[serde(rename = "sourceOfDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_of_definition: Option<String>,

    #[serde(rename = "symbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataTypeIec61360>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LangStringTextType>,

    #[serde(rename = "valueFormat")]
    pub value_format: Option<String>,

    #[serde(rename = "valueList")]
    pub value_list: Option<WrapperList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "levelType")]
    pub level_type: Option<LevelType>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WrapperList {
    #[serde(rename = "$value")]
    pub value_list: Option<Vec<ValueList>>,
}

impl From<DataSpecificationIec61360XMLProxy> for DataSpecificationIec61360 {
    fn from(value: DataSpecificationIec61360XMLProxy) -> Self {
        Self {
            preferred_name: value.preferred_name.values,
            short_name: value.short_name.map(LangStringTextType::into),
            definition: value.definition.map(LangStringTextType::into),
            unit: value.unit,
            unit_id: value.unit_id,
            source_of_definition: value.source_of_definition,
            symbol: value.symbol,
            data_type: value.data_type,
            value_format: value.value_format,
            value_list: value.value_list.and_then(|v| v.value_list),
            value: value.value,
            level_type: value.level_type,
        }
    }
}
impl From<DataSpecificationIec61360> for DataSpecificationIec61360XMLProxy {
    fn from(value: DataSpecificationIec61360) -> Self {
        Self {
            preferred_name: LangStringTextType {
                values: value.preferred_name,
            },
            short_name: value.short_name.map(|values| LangStringTextType { values }),
            definition: value.definition.map(|values| LangStringTextType { values }),
            unit: value.unit,
            unit_id: value.unit_id,
            source_of_definition: value.source_of_definition,
            symbol: value.symbol,
            data_type: value.data_type,
            value_format: value.value_format,
            value_list: value.value_list.map(|v| WrapperList {
                value_list: Some(v),
            }),
            value: value.value,
            level_type: value.level_type,
        }
    }
}
