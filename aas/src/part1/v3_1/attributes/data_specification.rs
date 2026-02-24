use crate::part1::v3_1::LangString;
use crate::part1::v3_1::level_type::LevelType;
use crate::part1::v3_1::reference::Reference;
use crate::part1::v3_1::value_list::ValueList;
use strum::{Display, EnumString};

/// HasDataSpecification
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HasDataSpecification {
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EmbeddedDataSpecification {
    pub data_specification: Reference,

    pub data_specification_content: DataSpecificationContent,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DataSpecificationContent {
    DataSpecificationIec61360(DataSpecificationIec61360),
}

#[derive(Clone, PartialEq, Debug)]
pub struct DataSpecificationIec61360 {
    pub preferred_name: Vec<LangString>,

    pub short_name: Option<Vec<LangString>>,

    pub unit: Option<String>,

    pub unit_id: Option<Reference>,

    pub source_of_definition: Option<String>,

    pub symbol: Option<String>,

    pub data_type: Option<DataTypeIec61360>,

    pub definition: Option<Vec<LangString>>,

    pub value_format: Option<String>,

    pub value_list: Option<Vec<ValueList>>,

    pub value: Option<String>,

    pub level_type: Option<LevelType>,
}

#[derive(EnumString, Display, Clone, PartialEq, Debug)]
pub enum DataTypeIec61360 {
    Blob,
    Boolean,
    Date,
    File,
    Html,
    IntegerCount,
    IntegerCurrency,
    IntegerMeasure,
    Irdi,
    Iri,
    Rational,
    RationalMeasure,
    RealCount,
    RealCurrency,
    RealMeasure,
    String,
    StringTranslatable,
    Time,
    Timestamp,
}
