use crate::part_1::v3_1::primitives::Iri;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use strum::{Display, EnumString};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Type mapping of XSDef types.
#[serde_as]
#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize)]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "openapi", schema(as = String))]
pub enum DataTypeXSDef {
    // basic types
    #[serde(rename = "xs:int")]
    Int,
    #[serde(rename = "xs:long")]
    Long,
    #[serde(rename = "xs:integer")]
    Integer,

    #[serde(rename = "xs:negativeInteger")]
    NegativeInteger,

    #[serde(rename = "xs:nonNegativeInteger")]
    NonNegativeInteger,

    #[serde(rename = "xs:nonPositiveInteger")]
    NonPositiveInteger,

    #[serde(rename = "xs:positiveInteger")]
    PositiveInteger,

    #[serde(rename = "xs:short")]
    Short,

    #[serde(rename = "xs:string")]
    String,

    #[serde(rename = "xs:boolean")]
    Boolean,
    #[serde(rename = "xs:byte")]
    Byte,

    #[serde(rename = "xs:unsignedByte")]
    UnsignedByte,

    #[serde(rename = "xs:unsignedInt")]
    UnsignedInt,

    #[serde(rename = "xs:unsignedLong")]
    UnsignedLong,

    #[serde(rename = "xs:unsignedShort")]
    UnsignedShort,

    #[serde(rename = "xs:decimal")]
    Decimal,

    #[serde(rename = "xs:float")]
    Float,

    #[serde(rename = "xs:double")]
    Double,

    // Date Time related
    #[serde(rename = "xs:time")]
    Time,

    #[serde(rename = "xs:date")]
    Date,

    #[serde(rename = "xs:dateTime")]
    DateTime,

    #[serde(rename = "xs:duration")]
    Duration,

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gDay")]
    GDay,

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonth")]
    GMonth,

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonthDay")]
    GMonthDay,

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYear")]
    GYear,

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYearMonth")]
    GYearMonth,

    // binary
    #[serde(rename = "xs:base64Binary")]
    Base64Binary,

    #[serde(rename = "xs:hexBinary")]
    HexBinary,

    // Miscellaneous types
    /// URI and IRI possible
    #[serde(rename = "xs:anyURI")]
    AnyURI,
}

/// represents the valueType/value pair typesafe. Used i.e. by Extension or Property.
/// ValueType has to be always present, value can be optional.
/// Default: String(None)
#[serde_as]
#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize, EnumString)]
#[serde(tag = "valueType", content = "value")]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DataXsd {
    // basic types
    #[serde(rename = "xs:int")]
    Int(Option<i32>),

    #[serde(rename = "xs:long")]
    Long(Option<i64>),

    #[serde(rename = "xs:integer")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    Integer(Option<BigDecimal>),

    #[serde(rename = "xs:negativeInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    NegativeInteger(Option<BigDecimal>),

    #[serde(rename = "xs:nonNegativeInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    NonNegativeInteger(Option<BigDecimal>),

    #[serde(rename = "xs:nonPositiveInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    NonPositiveInteger(Option<BigDecimal>),

    #[serde(rename = "xs:positiveInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    PositiveInteger(Option<BigDecimal>),

    #[serde(rename = "xs:short")]
    Short(Option<u16>),

    #[serde(rename = "xs:string")]
    String(Option<String>),

    #[serde(rename = "xs:boolean")]
    Boolean(Option<bool>),
    #[serde(rename = "xs:byte")]
    Byte(Option<i8>),

    #[serde(rename = "xs:unsignedByte")]
    UnsignedByte(Option<u8>),

    #[serde(rename = "xs:unsignedInt")]
    UnsignedInt(Option<u32>),

    #[serde(rename = "xs:unsignedLong")]
    UnsignedLong(Option<u64>),

    #[serde(rename = "xs:unsignedShort")]
    UnsignedShort(Option<u16>),

    #[serde(rename = "xs:decimal")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    Decimal(Option<BigDecimal>),

    #[serde(rename = "xs:float")]
    Float(#[serde_as(as = "Option<DisplayFromStr>")] Option<f32>),

    #[serde(rename = "xs:double")]
    Double(#[serde_as(as = "Option<DisplayFromStr>")] Option<f64>),

    #[serde(rename = "xs:time")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    Time(Option<iso8601::Time>),

    #[serde(rename = "xs:date")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    Date(Option<iso8601::Date>),

    #[serde(rename = "xs:dateTime")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    DateTime(Option<iso8601::DateTime>),

    #[serde(rename = "xs:duration")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    Duration(Option<iso8601::Duration>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gDay")]
    GDay(Option<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonth")]
    GMonth(Option<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonthDay")]
    GMonthDay(Option<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYear")]
    GYear(Option<String>),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYearMonth")]
    GYearMonth(Option<String>),

    // binary
    #[serde(rename = "xs:base64Binary")]
    Base64Binary(Option<Vec<u8>>),

    #[serde(rename = "xs:hexBinary")]
    HexBinary(Option<Vec<u8>>),

    // Miscellaneous types
    /// URI and IRI possible
    #[serde(rename = "xs:anyURI")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    AnyURI(Option<Iri>),
}

impl Default for DataXsd {
    fn default() -> Self {
        DataXsd::String(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_xs_string() {
        let json = r#""xs:string""#;

        serde_json::from_str::<DataTypeXSDef>(&json).unwrap();
    }

    #[test]
    fn deserialize_double_from_string() {
        let json = r#"{
            "valueType": "xs:double",
            "value": "1.2"
        }"#;

        let expected = DataXsd::Double(Some(1.2));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_duration() {
        let json = r#"{
            "valueType": "xs:duration",
            "value": "P1Y"
        }"#;

        let expected = DataXsd::Duration(Some(iso8601::Duration::YMDHMS {
            year: 1,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_naive_date_time() {
        let json = r#"{
            "valueType": "xs:dateTime",
            "value": "2001-10-26T21:32:52"
        }"#;

        let expected = DataXsd::DateTime(Some(iso8601::DateTime {
            date: iso8601::Date::YMD {
                year: 2001,
                month: 10,
                day: 26,
            },
            time: iso8601::Time {
                hour: 21,
                minute: 32,
                second: 52,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            },
        }));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_naive_date_time_with_zone() {
        let json = r#"{
            "valueType": "xs:dateTime",
            "value": "2001-10-26T21:32:52Z"
        }"#;

        let expected = DataXsd::DateTime(Some(iso8601::DateTime {
            date: iso8601::Date::YMD {
                year: 2001,
                month: 10,
                day: 26,
            },
            time: iso8601::Time {
                hour: 21,
                minute: 32,
                second: 52,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            },
        }));

        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_time_with_zone() {
        let json = r#"{
            "valueType": "xs:time",
            "value": "21:32:52Z"
        }"#;

        let expected = DataXsd::Time(Some(iso8601::Time {
            hour: 21,
            minute: 32,
            second: 52,
            millisecond: 0,
            tz_offset_hours: 0,
            tz_offset_minutes: 0,
        }));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_date_with_zone() {
        let json = r#"{
            "valueType": "xs:date",
            "value": "2001-10-26Z"
        }"#;

        let expected = DataXsd::Date(Some(iso8601::Date::YMD {
            year: 2001,
            month: 10,
            day: 26,
        }));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }
}
