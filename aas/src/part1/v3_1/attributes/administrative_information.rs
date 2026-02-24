use crate::part1::v3_1::attributes::data_specification::HasDataSpecification;
use crate::part1::v3_1::primitives::Identifier;
use crate::part1::v3_1::reference::Reference;
use thiserror::Error;

/// Administrative metainformation for an element like version information
#[derive(Clone, PartialEq, Debug)]
pub struct AdministrativeInformation {
    pub version: Version,
    /// The subject ID of the subject responsible for making the element
    pub creator: Option<Reference>,
    pub template_id: Option<Identifier>,
    pub data_specification: HasDataSpecification,
}

/// Versioning
/// Constraint AASd-005:
/// If AdministrativeInformation/version is not specified,
/// AdministrativeInformation/revision shall also be unspecified.
/// This means that a revision requires a version.
/// If there is no version, there is no revision.
/// Revision is optional.
#[derive(Clone, PartialEq, Debug)]
pub struct Version {
    pub version: Option<String>,
    pub revision: Option<String>,
}

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Revision can not exist without version")]
    RevisionNotApplicable,
}
