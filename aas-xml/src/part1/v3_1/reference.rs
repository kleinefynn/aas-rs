use crate::Xml;
use crate::part1::v3_1::key::Key;
use aas::part1::v3_1::reference::{Reference, ReferenceInner};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(EnumString, Clone, PartialEq, Debug, Deserialize, Serialize, Display)]
pub enum ReferenceType {
    ExternalReference,
    ModelReference,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "Reference")]
pub struct ReferenceXML {
    #[serde(rename = "type")]
    ty: ReferenceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referredSemanticId")]
    pub referred_semantic_id: Option<Box<ReferenceXML>>,
    /// needed for <keys><key>...</key></keys>
    pub keys: KeysXML,
}

impl From<ReferenceXML> for Reference {
    fn from(value: ReferenceXML) -> Self {
        match value.ty {
            ReferenceType::ExternalReference => Reference::ExternalReference(ReferenceInner {
                referred_semantic_id: value
                    .referred_semantic_id
                    .map(|v| *v)
                    .map(|v| Box::new(Reference::from(v))),
                keys: value.keys.key,
            }),
            ReferenceType::ModelReference => Reference::ModelReference(ReferenceInner {
                referred_semantic_id: value
                    .referred_semantic_id
                    .map(|v| *v)
                    .map(|v| Box::new(Reference::from(v))),
                keys: value.keys.key,
            }),
        }
    }
}

impl From<Reference> for ReferenceXML {
    fn from(value: Reference) -> Self {
        match value {
            Reference::ExternalReference(inner) => ReferenceXML {
                ty: ReferenceType::ExternalReference,
                keys: KeysXML { key: inner.keys },
                referred_semantic_id: inner
                    .referred_semantic_id
                    .map(|v| *v)
                    .map(|v| Box::new(ReferenceXML::from(v))),
            },
            Reference::ModelReference(inner) => ReferenceXML {
                ty: ReferenceType::ModelReference,
                keys: KeysXML { key: inner.keys },
                referred_semantic_id: inner
                    .referred_semantic_id
                    .map(|v| *v)
                    .map(|v| Box::new(ReferenceXML::from(v))),
            },
        }
    }
}

impl Xml for Reference {
    type Error = crate::XMLError;

    fn to_xml(&self) -> Result<String, Self::Error> {
        quick_xml::se::to_string(&ReferenceXML::from(self.clone()))
            .map_err(crate::XMLError::SerialiseError)
    }

    fn from_xml(json: &str) -> Result<Self, Self::Error> {
        quick_xml::de::from_str::<ReferenceXML>(json)
            .map_err(crate::XMLError::DeserializeError)
            .map(|v| v.into())
            .map(Into::into)
    }
}

/// needed for <key>...</key>
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
#[serde(rename = "key")]
pub struct KeysXML {
    pub key: Vec<Key>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1::v3_1::key::Key;
    use aas::part1::v3_1::reference::{Reference, ReferenceInner};

    #[test]
    fn test_xml_serialize() {
        let reference = ReferenceXML {
            ty: ReferenceType::ExternalReference,
            referred_semantic_id: None,
            keys: KeysXML {
                key: vec![Key::Blob("http://example/blob".into())],
            },
        };

        let xml = quick_xml::se::to_string(&reference).unwrap();

        println!("{}", xml);
    }

    #[test]
    fn deserialize_blob_reference_xml() {
        let xml = r#"
                <Reference>
                    <type>ExternalReference</type>
                    <keys>
                        <key>
                            <type>Blob</type>
                            <value>http://example/blob</value>
                        </key>
                        <key>
                            <type>Blob</type>
                            <value>http://example/blob2</value>
                        </key>
                    </keys>
                </Reference>"#;

        let expected = Reference::ExternalReference(ReferenceInner {
            referred_semantic_id: None,
            keys: vec![
                Key::Blob("http://example/blob".into()),
                Key::Blob("http://example/blob2".into()),
            ],
        });

        let actual = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(expected, actual);
    }
}
