use serde::{Deserialize, Deserializer, Serialize};

/// Wrapper Struct to serialize/deserialize given struct to/from XML
pub struct XML<T: Xml>(T);

impl<'de, T: Xml> Deserialize<'de> for XML<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <&str>::deserialize(deserializer)?;
        let t = T::from_xml(str).map_err(serde::de::Error::custom)?;
        Ok(Self(t))
    }
}

impl<T: Xml> Serialize for XML<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0
            .to_xml()
            .map(|a| a.serialize(serializer))
            .map_err(serde::ser::Error::custom)?
    }
}

pub trait Xml: Sized {
    type Error: std::fmt::Display;

    fn to_xml(&self) -> Result<String, Self::Error>;

    fn from_xml(json: &str) -> Result<Self, Self::Error>;
}
