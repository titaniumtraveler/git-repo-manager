use crate::schemas::{
    config::{Merge, merge::MergeEntry},
    utils::entry_map::VerboseEntry,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{mem, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "storage")]
#[schemars(transform = Self::transform_schema)]
pub struct Storage {
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Storage {
    type Short = PathBuf;

    fn from_short(path: Self::Short) -> Self {
        Self {
            path: Some(path),
            ..Default::default()
        }
    }
}

impl MergeEntry for Storage {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Storage {
            path,
            default,
            merge: _,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            path: path.or(s.path),
            default: default | s.default,
            merge: s.merge,
        }
    }
}
