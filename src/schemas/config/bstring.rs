use bstr::ByteSlice;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    os::unix::ffi::OsStringExt,
};

#[derive(Default, Clone, PartialEq)]
pub struct BString(pub Vec<u8>);

impl Debug for BString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0.as_bstr(), f)
    }
}

impl JsonSchema for BString {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        "string".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string"
        })
    }
}

impl Serialize for BString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        bstr::BStr::serialize(self.0.as_bstr(), serializer)
    }
}

impl<'de> Deserialize<'de> for BString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        bstr::BString::deserialize(deserializer)
            .map(Into::into)
            .map(BString)
    }
}

impl Deref for BString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<OsString> for BString {
    fn from(value: OsString) -> Self {
        BString(OsString::into_vec(value))
    }
}

impl From<&OsStr> for BString {
    fn from(value: &OsStr) -> Self {
        BString(OsString::into_vec(value.to_owned()))
    }
}
