use crate::part1::ToJsonMetamodel;
use bigdecimal::BigDecimal;
use iref::IriRefBuf;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// TODO: If the min value is missing, the value is assumed to be negative infinite.
// TODO: If the max value is missing, the value is assumed to be positive infinite.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]

pub struct RangeInner<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<T>,
}

// TODO: update to big decimal
// TODO: Only allow xsd atomic types.
#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize, EnumString)]
#[serde(tag = "valueType")]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
pub enum Range {
    // basic types
    #[serde(rename = "xs:int")]
    Int(RangeInner<i32>),

    #[serde(rename = "xs:integer")]
    Integer(RangeInner<i32>),

    #[serde(rename = "xs:long")]
    Long(RangeInner<i64>),

    #[serde(rename = "xs:negativeInteger")]
    NegativeInteger(RangeInner<i32>),

    #[serde(rename = "xs:nonNegativeInteger")]
    NonNegativeInteger(RangeInner<u32>),

    #[serde(rename = "xs:nonPositiveInteger")]
    NonPositiveInteger(RangeInner<i32>),

    #[serde(rename = "xs:positiveInteger")]
    PositiveInteger(RangeInner<u32>),

    #[serde(rename = "xs:short")]
    Short(RangeInner<u16>),

    #[serde(rename = "xs:string")]
    String(RangeInner<String>),

    #[serde(rename = "xs:boolean")]
    Boolean(RangeInner<bool>),

    #[serde(rename = "xs:byte")]
    Byte(RangeInner<i8>),

    #[serde(rename = "xs:unsignedByte")]
    UnsignedByte(RangeInner<u8>),

    #[serde(rename = "xs:unsignedInt")]
    UnsignedInt(RangeInner<u32>),

    #[serde(rename = "xs:unsignedLong")]
    UnsignedLong(RangeInner<u64>),

    #[serde(rename = "xs:unsignedShort")]
    UnsignedShort(RangeInner<u16>),

    #[serde(rename = "xs:decimal")]
    #[cfg_attr(feature = "openapi", schema(value_type = RangeInner<String>))]
    Decimal(RangeInner<BigDecimal>),

    #[serde(rename = "xs:float")]
    Float(RangeInner<f32>),

    #[serde(rename = "xs:double")]
    Double(RangeInner<f64>),

    // Date Time related
    #[serde(rename = "xs:time")]
    #[cfg_attr(feature = "openapi", schema(value_type = RangeInner<String>))]
    Time(RangeInner<iso8601::Time>),

    #[serde(rename = "xs:date")]
    #[cfg_attr(feature = "openapi", schema(value_type = RangeInner<String>))]
    Date(RangeInner<iso8601::Date>),

    #[serde(rename = "xs:dateTime")]
    #[cfg_attr(feature = "openapi", schema(value_type = RangeInner<String>))]
    DateTime(RangeInner<iso8601::DateTime>),

    /// TODO: using proper type
    #[serde(rename = "xs:duration")]
    Duration(RangeInner<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gDay")]
    GDay(RangeInner<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonth")]
    GMonth(RangeInner<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonthDay")]
    GMonthDay(RangeInner<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYear")]
    GYear(RangeInner<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYearMonth")]
    GYearMonth(RangeInner<String>),

    // binary
    #[serde(rename = "xs:base64Binary")]
    Base64Binary(RangeInner<Vec<u8>>),

    #[serde(rename = "xs:hexBinary")]
    HexBinary(RangeInner<Vec<u8>>),

    // string related
    // TODO: is this supported??
    #[serde(rename = "xs:anyURI")]
    #[cfg_attr(feature = "openapi", schema(value_type = RangeInner<String>))]
    AnyURI(RangeInner<IriRefBuf>),
}

impl ToJsonMetamodel for Range {
    type Error = ();

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        Ok(format!(r#"{{"valueType":"{}"}}"#, self.to_string()))
    }
}
