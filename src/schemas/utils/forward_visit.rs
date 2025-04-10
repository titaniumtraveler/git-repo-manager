use crate::schemas::utils::with_visitor::{ErrorWrapper, WithVisitor};
use serde::{Deserialize, Deserializer, de::Visitor};
use std::{fmt, marker::PhantomData};

pub struct ForwardToVisitor<T>(pub PhantomData<T>);

impl<T> ForwardToVisitor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for ForwardToVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        struct Expecting<'a, 'b>(&'a mut fmt::Formatter<'b>);
        impl<'de> WithVisitor<'de, ErrorWrapper<fmt::Result>> for Expecting<'_, '_> {
            fn with_visitor<V: Visitor<'de>>(
                self,
                visitor: V,
            ) -> Result<V::Value, ErrorWrapper<fmt::Result>> {
                Err(ErrorWrapper(visitor.expecting(self.0)))
            }
        }
        match T::deserialize(Expecting(formatter).into_deserializer()) {
            Err(ErrorWrapper(result)) => result,
            _ => unreachable!(),
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitBool(bool);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitBool {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_bool(self.0)
            }
        }
        T::deserialize(VisitBool(v).into_deserializer())
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitI8(i8);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitI8 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_i8(self.0)
            }
        }
        T::deserialize(VisitI8(v).into_deserializer())
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitI16(i16);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitI16 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_i16(self.0)
            }
        }
        T::deserialize(VisitI16(v).into_deserializer())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitI32(i32);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitI32 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_i32(self.0)
            }
        }
        T::deserialize(VisitI32(v).into_deserializer())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitI64(i64);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitI64 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_i64(self.0)
            }
        }
        T::deserialize(VisitI64(v).into_deserializer())
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitI128(i128);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitI128 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_i128(self.0)
            }
        }
        T::deserialize(VisitI128(v).into_deserializer())
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitU8(u8);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitU8 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_u8(self.0)
            }
        }
        T::deserialize(VisitU8(v).into_deserializer())
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitU16(u16);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitU16 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_u16(self.0)
            }
        }
        T::deserialize(VisitU16(v).into_deserializer())
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitU32(u32);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitU32 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_u32(self.0)
            }
        }
        T::deserialize(VisitU32(v).into_deserializer())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitU64(u64);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitU64 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_u64(self.0)
            }
        }
        T::deserialize(VisitU64(v).into_deserializer())
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitU128(u128);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitU128 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_u128(self.0)
            }
        }
        T::deserialize(VisitU128(v).into_deserializer())
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitF32(f32);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitF32 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_f32(self.0)
            }
        }
        T::deserialize(VisitF32(v).into_deserializer())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitF64(f64);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitF64 {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_f64(self.0)
            }
        }
        T::deserialize(VisitF64(v).into_deserializer())
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitChar(char);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitChar {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_char(self.0)
            }
        }
        T::deserialize(VisitChar(v).into_deserializer())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitStr<'a>(&'a str);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitStr<'_> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_str(self.0)
            }
        }
        T::deserialize(VisitStr(v).into_deserializer())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitBorrowedStr<'de>(&'de str);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitBorrowedStr<'de> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_borrowed_str(self.0)
            }
        }
        T::deserialize(VisitBorrowedStr(v).into_deserializer())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitString(String);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitString {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_string(self.0)
            }
        }
        T::deserialize(VisitString(v).into_deserializer())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitBytes<'a>(&'a [u8]);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitBytes<'_> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_bytes(self.0)
            }
        }
        T::deserialize(VisitBytes(v).into_deserializer())
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitBorrowedBytes<'de>(&'de [u8]);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitBorrowedBytes<'de> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_borrowed_bytes(self.0)
            }
        }
        T::deserialize(VisitBorrowedBytes(v).into_deserializer())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitByteBuf(Vec<u8>);
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitByteBuf {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_byte_buf(self.0)
            }
        }
        T::deserialize(VisitByteBuf(v).into_deserializer())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitNone();
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitNone {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_none()
            }
        }
        T::deserialize(VisitNone().into_deserializer())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitSome<D>(D);
        impl<'de, D: Deserializer<'de>> WithVisitor<'de, D::Error> for VisitSome<D> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, D::Error> {
                visitor.visit_some(self.0)
            }
        }
        T::deserialize(VisitSome(deserializer).into_deserializer())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        struct VisitUnit;
        impl<'de, E: serde::de::Error> WithVisitor<'de, E> for VisitUnit {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, E> {
                visitor.visit_unit()
            }
        }
        T::deserialize(VisitUnit.into_deserializer())
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitNewtypeStruct<D>(D);
        impl<'de, D: serde::Deserializer<'de>> WithVisitor<'de, D::Error> for VisitNewtypeStruct<D> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, D::Error> {
                visitor.visit_newtype_struct(self.0)
            }
        }
        T::deserialize(VisitNewtypeStruct(deserializer).into_deserializer())
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        struct VisitSeq<A>(A);
        impl<'de, A: serde::de::SeqAccess<'de>> WithVisitor<'de, A::Error> for VisitSeq<A> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, A::Error> {
                visitor.visit_seq(self.0)
            }
        }
        T::deserialize(VisitSeq(seq).into_deserializer())
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        struct VisitMap<A>(A);
        impl<'de, A: serde::de::MapAccess<'de>> WithVisitor<'de, A::Error> for VisitMap<A> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, A::Error> {
                visitor.visit_map(self.0)
            }
        }
        T::deserialize(VisitMap(map).into_deserializer())
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        struct VisitEnum<A>(A);
        impl<'de, A: serde::de::EnumAccess<'de>> WithVisitor<'de, A::Error> for VisitEnum<A> {
            fn with_visitor<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, A::Error> {
                visitor.visit_enum(self.0)
            }
        }
        T::deserialize(VisitEnum(data).into_deserializer())
    }
}
