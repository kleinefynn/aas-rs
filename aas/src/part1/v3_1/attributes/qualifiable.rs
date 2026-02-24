use crate::part1::v3_1::attributes::semantics::HasSemantics;
use crate::part1::v3_1::primitives::data_type_def_xs::DataXsd;
use crate::part1::v3_1::reference::Reference;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Qualifiable {
    pub qualifiers: Option<Vec<Qualifier>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Qualifier {
    ConceptQualifier(QualifierInner),
    TemplateQualifier(QualifierInner),
    ValueQualifier(QualifierInner),

    /// unknown values (kind = null!)
    Unknown(QualifierInner),
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifierInner {
    pub semantics: HasSemantics,

    // TODO: Text parsing
    pub ty: String,

    pub value: DataXsd,

    pub value_id: Option<Reference>,
}
