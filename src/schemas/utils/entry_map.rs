use crate::schemas::utils::{
    forward_visit::ForwardToVisitor,
    map_visitor::MapVisitorExt,
    overlay_visitor::{MapOverlay, OverlayVisitor},
};
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, de::Visitor};
use std::{collections::BTreeMap, marker::PhantomData, mem};

pub trait VerboseEntry<'de>: Deserialize<'de> {
    type Short: Deserialize<'de> + JsonSchema;
    fn from_short(short: Self::Short) -> Self;
    fn transform_schema(schema: &mut Schema) {
        // This could lead to unnessary schema duplication,
        // but we don't really have a choice ...
        let mut generator = SchemaGenerator::default();

        let short = generator.subschema_for::<Self::Short>();
        let verbose = mem::take(schema);

        *schema = json_schema! {
            {
                "oneOf": [
                    short,
                    verbose,
                ]
            }

        };
    }
}

pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: VerboseEntry<'de>,
{
    deserializer.deserialize_map(EntriesVisitor(PhantomData))
}

struct EntriesVisitor<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> Visitor<'de> for EntriesVisitor<K, V>
where
    K: Deserialize<'de> + Ord,
    V: VerboseEntry<'de>,
{
    type Value = BTreeMap<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of entries")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut btree_map = BTreeMap::new();

        while let Some((key, val)) = map.next_entry::<_, Entry<V>>()? {
            btree_map.insert(key, val.0);
        }

        Ok(btree_map)
    }
}

struct Entry<T>(T);

impl<'de, T: VerboseEntry<'de>> Deserialize<'de> for Entry<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_any(OverlayVisitor {
                overlay: MapOverlay::new(ForwardToVisitor::<T>::new()),
                fallback: ForwardToVisitor::<T::Short>::new().map(T::from_short),
            })
            .map(Self)
    }
}
