use crate::part1::v3_1::key::Key;
use std::ops::Deref;
use strum::{Display, EnumString};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ReferenceInner {
    /// E.g. semantic id of a standard submodel
    pub referred_semantic_id: Option<Box<Reference>>,

    pub keys: Vec<Key>,
}

/// A `Reference` is a mechanism to unambiguously identify one or multiple elements within or across
/// Asset Administration Shells. It consists of an ordered list of `Keys`, where each `Key` defines
/// a single step in the reference path, targeting specific sub-elements or external resources.
///
/// The `ReferenceType` attribute of a `Reference` determines the scope and semantics of the reference:
/// - `GlobalReference` means the reference resolves to an element identifiable globally,
/// often outside the local AAS context.
/// - `LocalReference` restricts the reference scope to internal elements or fragments
/// within the current parent element or AAS.
///
/// This distinction affects how references are interpreted, resolved, and validated in distributed environments,
/// ensuring interoperability and correct addressing in digital twin ecosystems.
///
/// A `Reference` supports multi-level navigation through composite structures by chaining multiple keys,
/// enabling precise targeting of nested submodels, submodel elements, or fragments.
#[derive(EnumString, Clone, PartialEq, Debug, Display)]
pub enum Reference {
    ExternalReference(ReferenceInner),
    ModelReference(ReferenceInner),
}

impl ReferenceInner {
    pub fn new(key: Key) -> Self {
        Self {
            referred_semantic_id: None,
            keys: vec![key],
        }
    }

    pub fn from_vec(keys: Vec<Key>) -> Self {
        Self {
            referred_semantic_id: None,
            keys,
        }
    }
}

impl Deref for Reference {
    type Target = ReferenceInner;

    fn deref(&self) -> &Self::Target {
        match self {
            Reference::ExternalReference(i) | Reference::ModelReference(i) => i,
        }
    }
}
