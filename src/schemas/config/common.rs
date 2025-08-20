use crate::schemas::config::{BString, Merge, merge::MergeEntry};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, mem};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Common {
    pub name: Option<BString>,
    #[serde(default)]
    pub alias: Vec<BString>,
    #[serde(default)]
    pub manifest: Option<BString>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl MergeEntry for Common {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Self {
            name,
            mut alias,
            manifest,
            default,
            merge: _,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            name: name.or(s.name),
            alias: {
                alias.extend(s.alias);
                alias
            },
            manifest: manifest.or(s.manifest),
            default: default | s.default,
            merge: s.merge,
        }
    }
}
