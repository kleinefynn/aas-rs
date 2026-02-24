use crate::part1::v3_1::attributes::extension::HasExtensions;
use crate::part1::v3_1::primitives::{Identifier, MultiLanguageNameType};

#[cfg(feature = "json")]
use crate::utilities::deserialize_empty_identifier_as_none;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Referable {
    // use case where "" is needed or can this be ignored?
    pub id_short: Option<Identifier>,

    pub display_name: Option<MultiLanguageNameType>,

    pub description: Option<MultiLanguageNameType>,

    #[deprecated]
    pub category: Option<String>,

    /// HasExtensions
    pub extensions: HasExtensions,
}
