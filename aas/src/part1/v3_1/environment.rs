use crate::part1::v3_1::concept_description::ConceptDescription;
use crate::part1::v3_1::core::{AssetAdministrationShell, Submodel};

#[derive(Clone, PartialEq, Debug)]
pub struct Environment {
    pub asset_administration_shells: Option<Vec<AssetAdministrationShell>>,

    pub submodels: Option<Vec<Submodel>>,

    pub concept_descriptions: Option<Vec<ConceptDescription>>,
}
