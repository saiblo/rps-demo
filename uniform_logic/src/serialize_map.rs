use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize,
};
use std::{collections::HashMap, fmt, hash::Hash, marker::PhantomData};

/// Wrap std::collections::HashMap to impl Serialize and Deserialize
pub struct SerializableMap<K, V>(pub HashMap<K, V>);

impl<K, V> From<HashMap<K, V>> for SerializableMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

// A Visitor is a type that holds methods that a Deserializer can drive
// depending on what is contained in the input data.
struct SerializableMapVisitor<K, V> {
    marker: PhantomData<fn() -> HashMap<K, V>>,
}

impl<K, V> SerializableMapVisitor<K, V> {
    fn new() -> Self {
        SerializableMapVisitor {
            marker: PhantomData,
        }
    }
}

// This is the trait that Deserializers are going to be driving.
impl<'de, K, V> Visitor<'de> for SerializableMapVisitor<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    // The type that our Visitor is going to produce.
    type Value = SerializableMap<K, V>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("SerializableMap")
    }

    // Deserialize SerializableMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map.into())
    }
}

// This is the trait that informs Serde how to deserialize SerializableMap.
impl<'de, K, V> Deserialize<'de> for SerializableMap<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of SerializableMap.
        deserializer.deserialize_map(SerializableMapVisitor::new())
    }
}

// This is the trait that informs Serde how to deserialize SerializableMap.
impl<'de, K, V> Serialize for SerializableMap<K, V>
where
    K: std::fmt::Display,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}
