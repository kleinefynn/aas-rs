use serde_with::DisplayFromStr;
use crate::part_1::v3_1::primitives::Iri;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use strum::{Display, EnumString};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// TODO: Properly use "time"-crate for this? Proper support for Time/Date/DateTime without zones.


#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize)]
#[serde(untagged)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DateTimeXS {
    NaiveDateTime(NaiveDateTime),
    DateTime(DateTime<Utc>),
}

#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize)]
#[serde(untagged)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DateXS {
    NaiveDate(NaiveDate),
    Date(DateTime<Utc>),
}

#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize)]
#[serde(untagged)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum TimeXS {
    NaiveTime(NaiveTime),
    // TODO: Custom type to wrap with time with offset.
    Time(chrono::DateTime<Utc>),
}


/// Type mapping of XSDef types.
#[serde_as]
#[derive(Clone, PartialEq, Debug, Display, Deserialize, Serialize)]
#[strum(prefix = "xs:", serialize_all = "camelCase")]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DataTypeXSDef {
    // basic types
    #[serde(rename = "xs:int")]
    Int(i32),

    #[serde(rename = "xs:long")]
    Long(i64),

    #[serde(rename = "xs:integer")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    Integer(BigDecimal),

    #[serde(rename = "xs:negativeInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    NegativeInteger(BigDecimal),

    #[serde(rename = "xs:nonNegativeInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    NonNegativeInteger(BigDecimal),

    #[serde(rename = "xs:nonPositiveInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    NonPositiveInteger(BigDecimal),

    #[serde(rename = "xs:positiveInteger")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    PositiveInteger(BigDecimal),

    #[serde(rename = "xs:short")]
    Short(u16),

    #[serde(rename = "xs:string")]
    String(String),

    #[serde(rename = "xs:boolean")]
    Boolean(bool),
    #[serde(rename = "xs:byte")]
    Byte(i8),

    #[serde(rename = "xs:unsignedByte")]
    UnsignedByte(u8),

    #[serde(rename = "xs:unsignedInt")]
    UnsignedInt(u32),

    #[serde(rename = "xs:unsignedLong")]
    UnsignedLong(u64),

    #[serde(rename = "xs:unsignedShort")]
    UnsignedShort(u16),

    #[serde(rename = "xs:decimal")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    Decimal(BigDecimal),

    #[serde(rename = "xs:float")]
    Float(f32),

    #[serde(rename = "xs:double")]
    Double(#[serde_as(as = "DisplayFromStr")] f64),

    // Date Time related
    // TODO: TIMEZONES?
    #[serde(rename = "xs:time")]
    Time(TimeXS),

    // TODO: TIMEZONES?
    #[serde(rename = "xs:date")]
    Date(DateXS),

    #[serde(rename = "xs:dateTime")]
    DateTime(DateTimeXS),

    #[serde(rename = "xs:duration")]
    Duration(iso8601_duration::Duration),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gDay")]
    GDay(String),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonth")]
    GMonth(String),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gMonthDay")]
    GMonthDay(String),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYear")]
    GYear(String),

    /// TODO: using proper type or parsing
    #[serde(rename = "xs:gYearMonth")]
    GYearMonth(String),

    // binary
    #[serde(rename = "xs:base64Binary")]
    Base64Binary(Vec<u8>),

    #[serde(rename = "xs:hexBinary")]
    HexBinary(Vec<u8>),

    // Miscellaneous types
    /// URI and IRI possible
    #[serde(rename = "xs:anyURI")]
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    AnyURI(Iri),
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
    Double (#[serde_as(as = "Option<DisplayFromStr>")] Option<f64>),

    // Date Time related
    // TODO: TIMEZONES?
    #[serde(rename = "xs:time")]
    Time(Option<TimeXS>),

    #[serde(rename = "xs:date")]
    Date(Option<DateXS>),

    #[serde(rename = "xs:dateTime")]
    DateTime(Option<DateTimeXS>),

    /// TODO: using proper type
    #[serde(rename = "xs:duration")]
    Duration(Option<iso8601_duration::Duration>),

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

        let expected = DataXsd::Duration(Some(iso8601_duration::Duration::new(1f32,0f32,0f32,0f32,0f32,0f32)));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_naive_date_time() {
        let json = r#"{
            "valueType": "xs:dateTime",
            "value": "2001-10-26T21:32:52"
        }"#;

        let expected = DataXsd::DateTime(Some(
            DateTimeXS::NaiveDateTime(
                NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2001i32, 10, 26).unwrap(),
                NaiveTime::from_hms_opt(21, 32, 52).unwrap(),
        ))));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_naive_date_time_with_zone() {
        let json = r#"{
            "valueType": "xs:dateTime",
            "value": "2001-10-26T21:32:52Z"
        }"#;

        let expected = DataXsd::DateTime(Some(DateTimeXS::DateTime(chrono::DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2001i32, 10, 26).unwrap(),
            NaiveTime::from_hms_opt(21, 32, 52).unwrap(),
        ), Utc))));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_time_with_zone() {
        let json = r#"{
            "valueType": "xs:time",
            "value": "21:32:52T"
        }"#;

        let expected = DataXsd::Time(Some(TimeXS::Time(chrono::DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2001i32, 10, 26).unwrap(),
                NaiveTime::from_hms_opt(21, 32, 52).unwrap(),
            ), Utc))));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_date_with_zone() {
        let json = r#"{
            "valueType": "xs:date",
            "value": "2001-10-26Z"
        }"#;

        let expected = DataXsd::Date(Some(DateXS::Date(chrono::DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2001i32, 10, 26).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ), Utc))));
        let actual: DataXsd = serde_json::from_str(json).unwrap();

        assert_eq!(expected, actual);
    }
}
