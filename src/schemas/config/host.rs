use crate::schemas::utils::entry_map::VerboseEntry;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "host")]
#[schemars(transform = Self::transform_schema)]
pub struct Host {
    pub host: String,
    #[serde(default)]
    pub alias: BTreeSet<String>,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for Host {
    type Short = String;

    fn from_short(host: Self::Short) -> Self {
        Self {
            host,
            alias: BTreeSet::new(),
            default: None,
        }
    }
}
