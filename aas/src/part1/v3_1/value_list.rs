use crate::part1::v3_1::reference::Reference;

#[derive(Clone, PartialEq, Debug)]
pub struct ValueList {
    pub value_reference_pairs: Vec<ValueReferencePair>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ValueReferencePair {
    pub value: String,

    pub value_id: Reference,
}
