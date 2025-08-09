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
#[schemars(rename = "open")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Open {
    // TODO: add aliases
    pub command: Vec<BString>,
    pub working_directory: Option<BString>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Open {
    type Short = Vec<BString>;

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
