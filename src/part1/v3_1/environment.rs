use crate::part1::v3_1::concept_description::ConceptDescription;
use crate::part1::v3_1::core::{AssetAdministrationShell, Submodel};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Environment {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetAdministrationShells")]
    pub asset_administration_shells: Option<Vec<AssetAdministrationShell>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub submodels: Option<Vec<Submodel>>,

    #[serde(rename = "conceptDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concept_descriptions: Option<Vec<ConceptDescription>>,
}

#[cfg(feature = "xml")]
mod xml {

    #[cfg(test)]
    mod tests {
        use crate::part1::v3_1::environment::Environment;

        #[test]
        #[ignore]
        fn deserialize_xml() {
            let xml = include_str!("../../../tests/mvp-dpp-1.0.0.xml");

            let env: Environment = quick_xml::de::from_str(xml).expect("Deserialize works");
            println!("{:#?}", env);
        }
    }
}

#[cfg(not(feature = "xml"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = include_str!("../../../tests/env.json");

        let env: Environment = serde_json::from_str(json).expect("Deserialize works");

        println!("{:#?}", env);
    }
}
