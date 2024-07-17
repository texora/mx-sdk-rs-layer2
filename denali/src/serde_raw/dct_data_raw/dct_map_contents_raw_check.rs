use super::*;
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{collections::BTreeMap, fmt};
pub struct CheckDctMapContentsRaw {
    pub contents: BTreeMap<String, CheckDctRaw>,
    pub other_dcts_allowed: bool,
}

impl Serialize for CheckDctMapContentsRaw {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.contents.len()))?;
        for (k, v) in self.contents.iter() {
            map.serialize_entry(k, v)?;
        }
        if self.other_dcts_allowed {
            map.serialize_entry("+", "")?;
        }
        map.end()
    }
}
impl<'de> Deserialize<'de> for CheckDctMapContentsRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckDctMapContentsRawVisitor)
    }
}

struct CheckDctMapContentsRawVisitor;

impl<'de> Visitor<'de> for CheckDctMapContentsRawVisitor {
    type Value = CheckDctMapContentsRaw;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("CheckAccountRaw or nothing")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut contents = BTreeMap::<String, CheckDctRaw>::new();

        // While there are entries remaining in the input, add them
        // into our map.
        let mut other_dcts_allowed = false;

        while let Some((key, value)) = access.next_entry()? {
            if key == "+" {
                other_dcts_allowed = true;
            } else {
                contents.insert(key, value);
            }
        }

        Ok(CheckDctMapContentsRaw {
            other_dcts_allowed,
            contents,
        })
    }
}
