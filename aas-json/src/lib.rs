use serde::{Deserialize, Deserializer, Serialize};

/// Wrapper Struct to serialize/deserialize given struct to/from JSON
pub struct JSON<T: Json>(T);

impl<'de, T: Json> Deserialize<'de> for JSON<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <&str>::deserialize(deserializer)?;
        let t = T::from_json(str).map_err(serde::de::Error::custom)?;
        Ok(Self(t))
    }
}

impl<T: Json> Serialize for JSON<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0
            .to_json()
            .map(|a| a.serialize(serializer))
            .map_err(serde::ser::Error::custom)?
    }
}

pub trait Json: Sized {
    type Error: std::fmt::Display;

    fn to_json(&self) -> Result<String, Self::Error>;

    fn from_json(json: &str) -> Result<Self, Self::Error>;
}
