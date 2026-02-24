use crate::part1::v3_1::primitives::data_type_def_xs::DataXsd;
use crate::part1::v3_1::reference::Reference;

/// HasExtensions
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HasExtensions {
    pub extension: Option<Vec<Extension>>,
}

/// `Extension` represents AAS elements that can hold additional, user-defined data through extensions.
///
/// This structure contains an optional list of `Extension` objects, allowing elements to be augmented
/// with supplementary information that extends the standard AAS metamodel.
///
/// Each contained `Extension` must have a unique name within its context to avoid conflicts.
///
/// The `HasExtension` structure enables flexible data modeling and customization,
/// supporting evolving use cases without compromising core interoperability.
///
/// Constraints:
/// - If present, the extensions list must contain at least one element.
/// - Extension names must be unique within this container.
#[derive(Clone, PartialEq, Debug)]
pub struct Extension {
    pub name: String,

    /// semantic definition
    pub semantic_id: Option<Reference>,

    pub supplemental_semantic_ids: Option<Vec<Reference>>,

    pub value: DataXsd,

    pub refers_to: Option<Vec<Reference>>,
}

impl Extension {
    pub fn new(name: String) -> Self {
        Self {
            semantic_id: None,
            supplemental_semantic_ids: None,
            name,
            //value_type: None,
            value: DataXsd::default(),
            refers_to: None,
        }
    }
}
