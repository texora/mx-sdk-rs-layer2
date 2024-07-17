use super::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;
pub enum CheckDctMapRaw {
    Unspecified,
    Star,
    Equal(CheckDctMapContentsRaw),
}

impl CheckDctMapRaw {
    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckDctMapRaw::Unspecified)
    }

    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctMapRaw::Star)
    }
}

impl Default for CheckDctMapRaw {
    fn default() -> Self {
        CheckDctMapRaw::Unspecified
    }
}

impl Serialize for CheckDctMapRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckDctMapRaw::Unspecified => serializer.serialize_str(""),
            CheckDctMapRaw::Star => serializer.serialize_str("*"),
            CheckDctMapRaw::Equal(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CheckDctMapRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckDctMapRawVisitor)
    }
}
struct CheckDctMapRawVisitor;

impl<'de> Visitor<'de> for CheckDctMapRawVisitor {
    type Value = CheckDctMapRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of log check")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckDctMapRaw::Star)
        } else {
            Err(de::Error::custom("only '*' allowed as logs string value"))
        }
    }

    fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        Ok(CheckDctMapRaw::Equal(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}
