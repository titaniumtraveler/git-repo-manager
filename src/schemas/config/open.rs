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
#[schemars(rename = "open")]
#[schemars(transform = Self::transform_schema)]
pub struct Open {
    pub command: Vec<String>,
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Open {
    type Short = Vec<String>;

    fn from_short(command: Self::Short) -> Self {
        Self {
            command,
            ..Default::default()
        }
    }
}

impl MergeEntry for Open {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Open {
            command,
            working_directory,
            default,
            merge: _,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            command: {
                if !command.is_empty() {
                    command
                } else {
                    s.command
                }
            },
            working_directory: working_directory.or(s.working_directory),
            default: default | s.default,
            merge: s.merge,
        }
    }
}
