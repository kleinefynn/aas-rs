use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::data_type_def_xs::DataXsd;
use crate::part1::v3_1::reference::Reference;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Qualifiable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiers: Option<Vec<Qualifier>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "xml", serde(
    from = "xml::QualifierXMLProxy",
    into = "xml::QualifierXMLProxy"
))]
pub enum Qualifier {
    ConceptQualifier(QualifierInner),
    TemplateQualifier(QualifierInner),
    ValueQualifier(QualifierInner),

    /// unknown values (kind = null!)
    #[serde(untagged)]
    Unknown(QualifierInner),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct QualifierInner {
    #[serde(flatten)]
    pub semantics: HasSemantics,

    // TODO: Text parsing
    #[serde(rename = "type")]
    pub ty: String,

    #[serde(flatten)]
    pub value: DataXsd,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueId")]
    pub value_id: Option<Reference>,
}

pub(crate) mod xml {
    use crate::part1::v3_1::attributes::qualifiable::{Qualifier, QualifierInner};
    use crate::part1::v3_1::attributes::semantics::HasSemantics;
    use crate::part1::v3_1::primitives::data_type_def_xs::DataTypeXSDef;
    use crate::part1::v3_1::reference::Reference;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    enum QualifierKind {
        ConceptQualifier,
        TemplateQualifier,
        ValueQualifier,
        Unknown,
    }

    #[derive(Serialize, Deserialize)]
    pub struct QualifierXMLProxy {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "semanticId")]
        pub semantic_id: Option<Reference>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "supplementalSemanticIds")]
        pub supplemental_semantic_ids: Option<Vec<Reference>>,
        // end HasSemantics

        // enum tag
        kind: QualifierKind,

        // TODO: Text parsing
        #[serde(rename = "type")]
        pub ty: String,

        // needs to be parsed into fitting type+value
        #[serde(rename = "valueType")]
        pub value_type: DataTypeXSDef,
        pub value: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "valueId")]
        pub value_id: Option<Reference>,
    }

    impl From<QualifierXMLProxy> for Qualifier {
        fn from(value: QualifierXMLProxy) -> Self {
            let inner = QualifierInner {
                semantics: HasSemantics {
                    semantic_id: value.semantic_id,
                    supplemental_semantic_ids: value.supplemental_semantic_ids,
                },
                ty: value.ty,
                value: (value.value_type, value.value).try_into().unwrap(),
                value_id: value.value_id,
            };
            match value.kind {
                QualifierKind::ConceptQualifier => Self::ConceptQualifier(inner),
                QualifierKind::TemplateQualifier => Self::TemplateQualifier(inner),
                QualifierKind::ValueQualifier => Self::ValueQualifier(inner),
                QualifierKind::Unknown => Self::Unknown(inner),
            }
        }
    }
    impl From<Qualifier> for QualifierXMLProxy {
        fn from(value: Qualifier) -> Self {
            let (kind, inner) = match value {
                Qualifier::ConceptQualifier(inner) => (QualifierKind::ConceptQualifier, inner),
                Qualifier::TemplateQualifier(inner) => (QualifierKind::TemplateQualifier, inner),
                Qualifier::ValueQualifier(inner) => (QualifierKind::ValueQualifier, inner),
                Qualifier::Unknown(inner) => (QualifierKind::Unknown, inner),
            };

            Self {
                semantic_id: inner.semantics.semantic_id,
                supplemental_semantic_ids: inner.semantics.supplemental_semantic_ids,
                kind,
                ty: inner.ty,
                value_type: inner.value.clone().into(),
                value: inner.value.clone().into(),
                value_id: inner.value_id,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_deserialize() {
        let expected = Qualifier::Unknown(QualifierInner {
            semantics: Default::default(),
            ty: "Test".to_string(),
            value: DataXsd::Boolean(Some(true)),
            value_id: None,
        });

        let json = r#"
        {
            "kind":"Test",
            "type": "Test",
            "valueType":"xs:boolean",
            "value": true
        }"#;

        let actual: Qualifier = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_concept_qualifier_deserialize() {
        let expected = Qualifier::ConceptQualifier(QualifierInner {
            semantics: Default::default(),
            ty: "Test".to_string(),
            value: DataXsd::Boolean(Some(true)),
            value_id: None,
        });

        let json = r#"
        {
            "kind":"ConceptQualifier",
            "type": "Test",
            "valueType":"xs:boolean",
            "value": true
        }"#;

        let actual: Qualifier = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }
}
