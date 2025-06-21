use crate::schemas::utils::{
    forward_visit::ForwardToVisitor,
    overlay_visitor::{Overlay, OverlayVisitor},
    verbose::VerboseEntry,
};
use serde::{Deserializer, de::Visitor};
use std::{fmt::Formatter, marker::PhantomData};

#[allow(dead_code)]
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: VerboseEntry<'de, Short = bool>,
{
    deserializer.deserialize_any(OverlayVisitor {
        overlay: FromBoolOverlay::new(),
        fallback: ForwardToVisitor::<T>::new(),
    })
}

struct FromBoolOverlay<T, Fallback> {
    _marker: PhantomData<(T, Fallback)>,
}

impl<T, Fallback> FromBoolOverlay<T, Fallback> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, T, Fallback> Overlay<'de> for FromBoolOverlay<T, Fallback>
where
    T: VerboseEntry<'de, Short = bool>,
    Fallback: Visitor<'de, Value = T>,
{
    type Value = T;
    type Fallback = Fallback;

    fn expecting(&self, fallback: &Self::Fallback, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("a bool or ")?;
        fallback.expecting(f)
    }

    fn visit_bool<E>(self, _: Self::Fallback, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T::from_short(v))
    }
}
