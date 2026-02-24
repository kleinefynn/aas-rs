use crate::part1::v3_1::attributes::administrative_information::AdministrativeInformation;
use crate::part1::v3_1::attributes::referable::Referable;
use crate::part1::v3_1::primitives::Identifier;

#[derive(Clone, PartialEq, Debug)]
pub struct Identifiable {
    pub id: Identifier,

    pub administration: Option<AdministrativeInformation>,

    pub referable: Referable,
}
