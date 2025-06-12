use crate::schemas::utils::entry_map::VerboseEntry;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "checkout-dir")]
#[schemars(transform = Self::transform_schema)]
pub struct Checkout {
    pub path: PathBuf,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for Checkout {
    type Short = PathBuf;

    fn from_short(path: Self::Short) -> Self {
        Self {
            path,
            default: None,
        }
    }
}
