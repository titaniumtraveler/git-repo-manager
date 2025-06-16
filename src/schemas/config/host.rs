use crate::schemas::{
    config::{Merge, merge::MergeEntry},
    utils::entry_map::VerboseEntry,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, mem};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "host")]
#[schemars(transform = Self::transform_schema)]
pub struct Host {
    pub host: Option<String>,
    #[serde(default)]
    pub alias: BTreeSet<String>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Host {
    type Short = String;

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
            default: default | s.default,
            merge: s.merge,
        }
    }
}
