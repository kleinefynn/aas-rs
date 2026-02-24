use crate::part1::ToJsonMetamodel;
use bigdecimal::BigDecimal;
use iref::IriRefBuf;

use strum::{Display, EnumString};

// TODO: If the min value is missing, the value is assumed to be negative infinite.
// TODO: If the max value is missing, the value is assumed to be positive infinite.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RangeInner<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

// TODO: update to big decimal
// TODO: Only allow xsd atomic types.
#[derive(Clone, PartialEq, Debug, Display, EnumString)]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
pub enum Range {
    // basic types
    Int(RangeInner<i32>),

    Integer(RangeInner<i32>),

    Long(RangeInner<i64>),

    NegativeInteger(RangeInner<i32>),

    NonNegativeInteger(RangeInner<u32>),

    NonPositiveInteger(RangeInner<i32>),

    PositiveInteger(RangeInner<u32>),

    Short(RangeInner<u16>),

    String(RangeInner<String>),

    Boolean(RangeInner<bool>),

    Byte(RangeInner<i8>),

    UnsignedByte(RangeInner<u8>),

    UnsignedInt(RangeInner<u32>),

    UnsignedLong(RangeInner<u64>),

    UnsignedShort(RangeInner<u16>),
    Decimal(RangeInner<BigDecimal>),

    Float(RangeInner<f32>),

    Double(RangeInner<f64>),

    // Date Time related
    Time(RangeInner<iso8601::Time>),
    Date(RangeInner<iso8601::Date>),
    DateTime(RangeInner<iso8601::DateTime>),

    /// TODO: using proper type
    Duration(RangeInner<String>),

    /// TODO: using proper type or parsing
    GDay(RangeInner<String>),

    /// TODO: using proper type or parsing
    GMonth(RangeInner<String>),

    /// TODO: using proper type or parsing
    GMonthDay(RangeInner<String>),

    /// TODO: using proper type or parsing
    GYear(RangeInner<String>),

    /// TODO: using proper type or parsing
    GYearMonth(RangeInner<String>),

    // binary
    Base64Binary(RangeInner<Vec<u8>>),

    HexBinary(RangeInner<Vec<u8>>),

    // string related
    // TODO: is this supported??
    AnyURI(RangeInner<IriRefBuf>),
}

impl ToJsonMetamodel for Range {
    type Error = ();

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        Ok(format!(r#"{{"valueType":"{}"}}"#, self.to_string()))
    }
}
