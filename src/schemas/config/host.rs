use crate::schemas::{
    config::{BString, Merge, merge::MergeEntry},
    utils::verbose::VerboseEntry,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "host")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Host {
    pub host: Option<BString>,
    #[serde(default)]
    pub alias: Vec<BString>,
    #[serde(default)]
    pub key: Option<BString>,
    #[serde(default)]
    pub pass_program: Vec<BString>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Host {
    type Short = BString;

    fn from_short(host: Self::Short) -> Self {
        Self {
            host: Some(host),
            ..Default::default()
        }
    }
}

impl MergeEntry for Host {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Self {
            host,
            mut alias,
            key,
            pass_program,
            default,
            merge: _,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            host: host.or(s.host),
            alias: {
                alias.extend(s.alias);
                alias
            },
            key: key.or(s.key),
            pass_program: if !pass_program.is_empty() {
                pass_program
            } else {
                s.pass_program
            },
            default: default | s.default,
            merge: s.merge,
        }
    }
}
