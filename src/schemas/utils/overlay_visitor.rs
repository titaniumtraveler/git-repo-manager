use serde::de::Visitor;

pub use self::map_overlay::MapOverlay;

mod map_overlay;

pub struct OverlayVisitor<'de, O: Overlay<'de>> {
    pub overlay: O,
    pub fallback: O::Fallback,
}

pub trait Overlay<'de>: Sized {
    type Value;
    type Fallback: Visitor<'de, Value = Self::Value>;

    fn expecting(
        &self,
        fallback: &Self::Fallback,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result;

    fn visit_bool<E>(self, fallback: Self::Fallback, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_bool(v)
    }

    fn visit_i8<E>(self, fallback: Self::Fallback, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_i8(v)
    }

    fn visit_i16<E>(self, fallback: Self::Fallback, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_i16(v)
    }

    fn visit_i32<E>(self, fallback: Self::Fallback, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_i32(v)
    }

    fn visit_i64<E>(self, fallback: Self::Fallback, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_i64(v)
    }

    fn visit_i128<E>(self, fallback: Self::Fallback, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_i128(v)
    }

    fn visit_u8<E>(self, fallback: Self::Fallback, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_u8(v)
    }

    fn visit_u16<E>(self, fallback: Self::Fallback, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_u16(v)
    }

    fn visit_u32<E>(self, fallback: Self::Fallback, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_u32(v)
    }

    fn visit_u64<E>(self, fallback: Self::Fallback, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_u64(v)
    }

    fn visit_u128<E>(self, fallback: Self::Fallback, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_u128(v)
    }

    fn visit_f32<E>(self, fallback: Self::Fallback, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_f32(v)
    }

    fn visit_f64<E>(self, fallback: Self::Fallback, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_f64(v)
    }

    fn visit_char<E>(self, fallback: Self::Fallback, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_char(v)
    }

    fn visit_str<E>(self, fallback: Self::Fallback, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, fallback: Self::Fallback, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, fallback: Self::Fallback, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_string(v)
    }

    fn visit_bytes<E>(self, fallback: Self::Fallback, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(
        self,
        fallback: Self::Fallback,
        v: &'de [u8],
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, fallback: Self::Fallback, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_byte_buf(v)
    }

    fn visit_none<E>(self, fallback: Self::Fallback) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_none()
    }

    fn visit_some<D>(
        self,
        fallback: Self::Fallback,
        deserializer: D,
    ) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fallback.visit_some(deserializer)
    }

    fn visit_unit<E>(self, fallback: Self::Fallback) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        fallback.visit_unit()
    }

    fn visit_newtype_struct<D>(
        self,
        fallback: Self::Fallback,
        deserializer: D,
    ) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fallback.visit_newtype_struct(deserializer)
    }

    fn visit_seq<A>(self, fallback: Self::Fallback, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        fallback.visit_seq(seq)
    }

    fn visit_map<A>(self, fallback: Self::Fallback, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        fallback.visit_map(map)
    }

    fn visit_enum<A>(self, fallback: Self::Fallback, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        fallback.visit_enum(data)
    }
}

impl<'de, O: Overlay<'de>> Visitor<'de> for OverlayVisitor<'de, O> {
    type Value = O::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.overlay.expecting(&self.fallback, formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_bool(self.fallback, v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_i8(self.fallback, v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_i16(self.fallback, v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_i32(self.fallback, v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_i64(self.fallback, v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_i128(self.fallback, v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_u8(self.fallback, v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_u16(self.fallback, v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_u32(self.fallback, v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_u64(self.fallback, v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_u128(self.fallback, v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_f32(self.fallback, v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_f64(self.fallback, v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_char(self.fallback, v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_str(self.fallback, v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_borrowed_str(self.fallback, v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_string(self.fallback, v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_bytes(self.fallback, v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_borrowed_bytes(self.fallback, v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_byte_buf(self.fallback, v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_none(self.fallback)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.overlay.visit_some(self.fallback, deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.overlay.visit_unit(self.fallback)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.overlay
            .visit_newtype_struct(self.fallback, deserializer)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.overlay.visit_seq(self.fallback, seq)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.overlay.visit_map(self.fallback, map)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        self.overlay.visit_enum(self.fallback, data)
    }
}
