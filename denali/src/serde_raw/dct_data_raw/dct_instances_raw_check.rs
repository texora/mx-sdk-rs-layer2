use super::*;
use serde::{
    de::{self, Deserializer, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::fmt;

pub enum CheckDctInstancesRaw {
    Unspecified,
    Star,
    Equal(Vec<CheckDctInstanceRaw>),
}

impl CheckDctInstancesRaw {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctInstancesRaw::Star)
    }

    pub fn is_unspecified(&self) -> bool {
        matches!(self, CheckDctInstancesRaw::Unspecified)
    }
}

impl Default for CheckDctInstancesRaw {
    fn default() -> Self {
        CheckDctInstancesRaw::Unspecified
    }
}

impl Serialize for CheckDctInstancesRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CheckDctInstancesRaw::Unspecified => {
                let map = serializer.serialize_map(Some(0))?;
                map.end()
            },
            CheckDctInstancesRaw::Star => serializer.serialize_str("*"),
            CheckDctInstancesRaw::Equal(m) => {
                let mut map = serializer.serialize_seq(Some(m.len()))?;
                for v in m {
                    map.serialize_element(v)?;
                }
                map.end()
            },
        }
    }
}

struct CheckDctInstancesRawVisitor;

impl<'de> Visitor<'de> for CheckDctInstancesRawVisitor {
    type Value = CheckDctInstancesRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("serialized object JSON representation of an DCT instances list")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "*" {
            Ok(CheckDctInstancesRaw::Star)
        } else {
            Err(de::Error::custom(
                "only '*' allowed as DCT instances value",
            ))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::<CheckDctInstanceRaw>::new();

        while let Some(item) = seq.next_element()? {
            list.push(item);
        }

        Ok(CheckDctInstancesRaw::Equal(list))
    }
}

impl<'de> Deserialize<'de> for CheckDctInstancesRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckDctInstancesRawVisitor)
    }
}
