use crate::{XMLError, Xml};
use aas::part1::v3_1::attributes::administrative_information::{
    AdministrativeInformation, Version,
};
use aas::part1::v3_1::attributes::data_specification::{
    EmbeddedDataSpecification, HasDataSpecification,
};
use aas::part1::v3_1::primitives::Identifier;
use aas::part1::v3_1::reference::Reference;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AdministrativeInformationXML {
    version: Option<String>,
    revision: Option<String>,

    /// The subject ID of the subject responsible for making the element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Reference>,

    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "embeddedDataSpecifications")]
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
}

impl From<AdministrativeInformationXML> for AdministrativeInformation {
    fn from(value: AdministrativeInformationXML) -> Self {
        Self {
            version: Version {
                version: value.version,
                revision: value.revision,
            },
            creator: value.creator,
            template_id: value.template_id,
            data_specification: HasDataSpecification {
                embedded_data_specifications: value.embedded_data_specifications,
            },
        }
    }
}

impl From<AdministrativeInformation> for AdministrativeInformationXML {
    fn from(value: AdministrativeInformation) -> Self {
        Self {
            version: value.version.version,
            revision: value.version.revision,
            creator: value.creator,
            template_id: value.template_id,
            embedded_data_specifications: value.data_specification.embedded_data_specifications,
        }
    }
}

impl Xml for AdministrativeInformation {
    type Error = XMLError;

    fn to_xml(&self) -> Result<String, Self::Error> {
        quick_xml::se::to_string(self).map_err(XMLError::SerialiseError)
    }

    fn from_xml(json: &str) -> Result<Self, Self::Error> {
        quick_xml::de::from_str(json).map_err(XMLError::DeserializeError)
    }
}
