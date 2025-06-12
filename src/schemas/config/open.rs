use crate::schemas::{config::Merge, utils::entry_map::VerboseEntry};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "open")]
#[schemars(transform = Self::transform_schema)]
pub struct Open {
    pub command: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub default: Option<bool>,
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
