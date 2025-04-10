use crate::schemas::utils::overlay_visitor::Overlay;
use serde::{
    Deserializer,
    de::{Visitor, value::MapAccessDeserializer},
};
use std::marker::PhantomData;

pub struct MapOverlay<V, Fallback> {
    pub map_visitor: V,
    pub _marker: PhantomData<Fallback>,
}

impl<V, Fallback> MapOverlay<V, Fallback> {
    pub fn new(map_visitor: V) -> Self {
        Self {
            map_visitor,
            _marker: PhantomData,
        }
    }
}

impl<'de, V, Fallback> Overlay<'de> for MapOverlay<V, Fallback>
where
    V: Visitor<'de>,
    Fallback: Visitor<'de, Value = V::Value>,
{
    type Value = V::Value;
    type Fallback = Fallback;

    fn expecting(
        &self,
        fallback: &Self::Fallback,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        f.write_str("a map or ")?;
        fallback.expecting(f)
    }

    fn visit_map<A>(self, _: Self::Fallback, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        MapAccessDeserializer::new(map).deserialize_map(self.map_visitor)
    }
}
