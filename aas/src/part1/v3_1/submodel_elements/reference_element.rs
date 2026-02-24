use crate::part1::ToJsonMetamodel;
use crate::part1::v3_1::reference::Reference;

// ToJsonMetadata implemented from upper enum.
#[derive(Clone, PartialEq, Debug, Default)]

pub struct ReferenceElement {
    /// External reference to an external object or entity or a logical reference
    /// to another element within the same or another Asset Administration Shell
    /// (i.e. a model reference to a Referable)
    pub value: Option<Reference>,
}

impl ToJsonMetamodel for ReferenceElement {
    type Error = ();

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        Ok(format!(r#"{{"modelType":"ReferenceElement"}}"#))
    }
}
