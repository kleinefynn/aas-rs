use crate::part1::v3_1::primitives::Iri;
use bigdecimal::BigDecimal;
use strum::{Display, EnumString};
use thiserror::Error;

/// Represents the valueType/value pair typesafe. Used i.e., by Extension or Property.
/// ValueType has to be always present, value can be optional.
/// Default: String(None)
#[derive(Clone, PartialEq, Debug, Display, EnumString)]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
pub enum DataXsd {
    // basic types
    Int(Option<i32>),

    Long(Option<i64>),

    Integer(Option<BigDecimal>),

    NegativeInteger(Option<BigDecimal>),

    NonNegativeInteger(Option<BigDecimal>),

    NonPositiveInteger(Option<BigDecimal>),

    PositiveInteger(Option<BigDecimal>),

    Short(Option<u16>),

    String(Option<String>),

    Boolean(Option<bool>),

    Byte(Option<i8>),

    UnsignedByte(Option<u8>),

    UnsignedInt(Option<u32>),

    UnsignedLong(Option<u64>),

    UnsignedShort(Option<u16>),

    Decimal(Option<BigDecimal>),

    Float(Option<f32>),

    Double(Option<f64>),

    Time(Option<iso8601::Time>),

    Date(Option<iso8601::Date>),

    DateTime(Option<iso8601::DateTime>),

    Duration(Option<iso8601::Duration>),

    /// TODO: using proper type or parsing
    GDay(Option<String>),

    /// TODO: using proper type or parsing
    GMonth(Option<String>),

    /// TODO: using proper type or parsing
    GMonthDay(Option<String>),

    /// TODO: using proper type or parsing
    GYear(Option<String>),

    /// TODO: using proper type or parsing
    GYearMonth(Option<String>),

    // binary
    Base64Binary(Option<Vec<u8>>),

    HexBinary(Option<Vec<u8>>),

    // Miscellaneous types
    /// URI and IRI possible
    AnyURI(Option<Iri>),
}

/// Type mapping of XSDef types.
#[derive(Clone, PartialEq, Debug, Display)]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
pub enum DataTypeXSDef {
    // basic types
    Int,
    Long,
    Integer,
    NegativeInteger,
    NonNegativeInteger,
    NonPositiveInteger,
    PositiveInteger,
    Short,
    String,
    Boolean,
    Byte,
    UnsignedByte,
    UnsignedInt,
    UnsignedLong,
    UnsignedShort,
    Decimal,
    Float,
    Double,
    // Date Time related
    Time,
    Date,
    DateTime,
    Duration,
    GDay,
    GMonth,
    GMonthDay,
    GYear,
    GYearMonth,
    // binary
    Base64Binary,
    HexBinary,
    // Miscellaneous types
    /// URI and IRI possible
    AnyURI,
}

#[derive(Debug, Clone, Error)]
pub enum ConversionError {
    #[error("Value is not parseable")]
    ParseError,
}

impl TryFrom<(DataTypeXSDef, Option<String>)> for DataXsd {
    type Error = ConversionError;

    fn try_from(value: (DataTypeXSDef, Option<String>)) -> Result<Self, Self::Error> {
        match value.0 {
            DataTypeXSDef::Int => Ok(DataXsd::Int(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Long => Ok(DataXsd::Long(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Integer => Ok(DataXsd::Integer(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::NegativeInteger => Ok(DataXsd::NegativeInteger(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::NonNegativeInteger => Ok(DataXsd::NonNegativeInteger(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::NonPositiveInteger => Ok(DataXsd::NonPositiveInteger(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::PositiveInteger => Ok(DataXsd::PositiveInteger(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Short => Ok(DataXsd::Short(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::String => Ok(DataXsd::String(value.1)),
            DataTypeXSDef::Boolean => Ok(DataXsd::Boolean(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Byte => Ok(DataXsd::Byte(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::UnsignedByte => Ok(DataXsd::UnsignedByte(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::UnsignedInt => Ok(DataXsd::UnsignedInt(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::UnsignedLong => Ok(DataXsd::UnsignedLong(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::UnsignedShort => Ok(DataXsd::UnsignedShort(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Decimal => Ok(DataXsd::Decimal(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Float => Ok(DataXsd::Float(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Double => Ok(DataXsd::Double(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Time => Ok(DataXsd::Time(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Date => Ok(DataXsd::Date(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::DateTime => Ok(DataXsd::DateTime(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Duration => Ok(DataXsd::Duration(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::GDay => Ok(DataXsd::GDay(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::GMonth => Ok(DataXsd::GMonth(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::GMonthDay => Ok(DataXsd::GMonthDay(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::GYear => Ok(DataXsd::GYear(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::GYearMonth => Ok(DataXsd::GYearMonth(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
            DataTypeXSDef::Base64Binary => {
                Ok(DataXsd::Base64Binary(value.1.map(|v| v.into_bytes())))
            }
            DataTypeXSDef::HexBinary => Ok(DataXsd::HexBinary(value.1.map(|v| v.into_bytes()))),
            DataTypeXSDef::AnyURI => Ok(DataXsd::AnyURI(
                value
                    .1
                    .map(|v| v.parse().map_err(|_| ConversionError::ParseError))
                    .transpose()?,
            )),
        }
    }
}

impl From<DataXsd> for Option<String> {
    fn from(value: DataXsd) -> Self {
        match value {
            DataXsd::Int(v) => v.map(|v| v.to_string()),
            DataXsd::Long(v) => v.map(|v| v.to_string()),
            DataXsd::Integer(v) => v.map(|v| v.to_string()),
            DataXsd::NegativeInteger(v) => v.map(|v| v.to_string()),
            DataXsd::NonNegativeInteger(v) => v.map(|v| v.to_string()),
            DataXsd::NonPositiveInteger(v) => v.map(|v| v.to_string()),
            DataXsd::PositiveInteger(v) => v.map(|v| v.to_string()),
            DataXsd::Short(v) => v.map(|v| v.to_string()),
            DataXsd::String(v) => v,
            DataXsd::Boolean(v) => v.map(|v| v.to_string()),
            DataXsd::Byte(v) => v.map(|v| v.to_string()),
            DataXsd::UnsignedByte(v) => v.map(|v| v.to_string()),
            DataXsd::UnsignedInt(v) => v.map(|v| v.to_string()),
            DataXsd::UnsignedLong(v) => v.map(|v| v.to_string()),
            DataXsd::UnsignedShort(v) => v.map(|v| v.to_string()),
            DataXsd::Decimal(v) => v.map(|v| v.to_string()),
            DataXsd::Float(v) => v.map(|v| v.to_string()),
            DataXsd::Double(v) => v.map(|v| v.to_string()),
            DataXsd::Time(v) => v.map(|v| v.to_string()),
            DataXsd::Date(v) => v.map(|v| v.to_string()),
            DataXsd::DateTime(v) => v.map(|v| v.to_string()),
            DataXsd::Duration(v) => v.map(|v| v.to_string()),
            DataXsd::GDay(v) => v.map(|v| v.to_string()),
            DataXsd::GMonth(v) => v.map(|v| v.to_string()),
            DataXsd::GMonthDay(v) => v.map(|v| v.to_string()),
            DataXsd::GYear(v) => v.map(|v| v.to_string()),
            DataXsd::GYearMonth(v) => v.map(|v| v.to_string()),
            DataXsd::Base64Binary(v) => v.map(|v| String::from_utf8_lossy(&v).to_string()),
            DataXsd::HexBinary(v) => v.map(|v| String::from_utf8_lossy(&v).to_string()),
            DataXsd::AnyURI(v) => v.map(|v| v.to_string()),
        }
    }
}

impl From<DataXsd> for DataTypeXSDef {
    fn from(value: DataXsd) -> Self {
        match value {
            DataXsd::Int(_) => DataTypeXSDef::Int,
            DataXsd::Long(_) => DataTypeXSDef::Long,
            DataXsd::Integer(_) => DataTypeXSDef::Integer,
            DataXsd::NegativeInteger(_) => DataTypeXSDef::NegativeInteger,
            DataXsd::NonNegativeInteger(_) => DataTypeXSDef::NonNegativeInteger,
            DataXsd::NonPositiveInteger(_) => DataTypeXSDef::NonPositiveInteger,
            DataXsd::PositiveInteger(_) => DataTypeXSDef::PositiveInteger,
            DataXsd::Short(_) => DataTypeXSDef::Short,
            DataXsd::String(_) => DataTypeXSDef::String,
            DataXsd::Boolean(_) => DataTypeXSDef::Boolean,
            DataXsd::Byte(_) => DataTypeXSDef::Byte,
            DataXsd::UnsignedByte(_) => DataTypeXSDef::UnsignedByte,
            DataXsd::UnsignedInt(_) => DataTypeXSDef::UnsignedInt,
            DataXsd::UnsignedLong(_) => DataTypeXSDef::UnsignedLong,
            DataXsd::UnsignedShort(_) => DataTypeXSDef::UnsignedShort,
            DataXsd::Decimal(_) => DataTypeXSDef::Decimal,
            DataXsd::Float(_) => DataTypeXSDef::Float,
            DataXsd::Double(_) => DataTypeXSDef::Double,
            DataXsd::Time(_) => DataTypeXSDef::Time,
            DataXsd::Date(_) => DataTypeXSDef::Date,
            DataXsd::DateTime(_) => DataTypeXSDef::DateTime,
            DataXsd::Duration(_) => DataTypeXSDef::Duration,
            DataXsd::GDay(_) => DataTypeXSDef::GDay,
            DataXsd::GMonth(_) => DataTypeXSDef::GMonth,
            DataXsd::GMonthDay(_) => DataTypeXSDef::GMonthDay,
            DataXsd::GYear(_) => DataTypeXSDef::GYear,
            DataXsd::GYearMonth(_) => DataTypeXSDef::GYearMonth,
            DataXsd::Base64Binary(_) => DataTypeXSDef::Base64Binary,
            DataXsd::HexBinary(_) => DataTypeXSDef::HexBinary,
            DataXsd::AnyURI(_) => DataTypeXSDef::AnyURI,
        }
    }
}

impl Default for DataXsd {
    fn default() -> Self {
        DataXsd::String(None)
    }
}
