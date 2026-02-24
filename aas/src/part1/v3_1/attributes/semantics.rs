use crate::part1::v3_1::reference::Reference;

// HasSemantics
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(
    feature = "xml",
    serde(from = "xml::HasSemanticsXMLProxy", into = "xml::HasSemanticsXMLProxy")
)]
pub struct HasSemantics {
    pub semantic_id: Option<Reference>,

    pub supplemental_semantic_ids: Option<Vec<Reference>>,
}

pub(crate) mod xml {
    use crate::part1::v3_1::attributes::semantics::HasSemantics;
    use crate::part1::v3_1::reference::Reference;

    #[derive(Debug)]
    pub struct HasSemanticsXMLProxy {
        pub semantic_id: Option<Reference>,

        pub supplemental_semantic_ids: SupplementalSemanticIdsWrapper,
    }

    #[derive(Debug)]
    pub struct SupplementalSemanticIdsWrapper {
        pub reference: Option<Vec<Reference>>,
    }

    impl From<HasSemantics> for HasSemanticsXMLProxy {
        fn from(value: HasSemantics) -> Self {
            Self {
                semantic_id: value.semantic_id,
                supplemental_semantic_ids: SupplementalSemanticIdsWrapper {
                    reference: value.supplemental_semantic_ids,
                },
            }
        }
    }

    impl From<HasSemanticsXMLProxy> for HasSemantics {
        fn from(value: HasSemanticsXMLProxy) -> Self {
            Self {
                semantic_id: value.semantic_id,
                supplemental_semantic_ids: value.supplemental_semantic_ids.reference,
            }
        }
    }
}
