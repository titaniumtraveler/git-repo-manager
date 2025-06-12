use crate::{
    schemas::{
        directories::PROJECT_PATHS,
        utils::entry_map::{self, VerboseEntry},
    },
    utils,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "config.toml")]
pub struct Config {
    #[allow(dead_code)]
    #[serde(rename = "$schema", skip_serializing, default)]
    #[schemars(with = "String", default)]
    schema: IgnoredAny,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    pub host: BTreeMap<String, Host>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    pub storage: BTreeMap<String, Storage>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    pub checkout: BTreeMap<String, Checkout>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    pub open: BTreeMap<String, Open>,
}

impl Config {
    pub fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        utils::read_toml_from_path(path)
    }

    pub fn resolve_defaults(&mut self) {
        for (k, v) in &mut self.host {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        for (k, v) in &mut self.storage {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        if self.storage.is_empty() {
            self.storage.insert(
                "DEFAULT".to_owned(),
                Storage {
                    path: PROJECT_PATHS.default_storage_dir().to_owned(),
                    default: Some(true),
                },
            );
        }

        for (k, v) in &mut self.checkout {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        for (k, v) in &mut self.open {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "storage")]
#[schemars(transform = Self::transform_schema)]
pub struct Storage {
    pub path: PathBuf,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for Storage {
    type Short = PathBuf;

    fn from_short(path: Self::Short) -> Self {
        Self {
            path,
            default: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "open")]
#[schemars(transform = Self::transform_schema)]
pub struct Open {
    pub command: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for Open {
    type Short = Vec<String>;

    fn from_short(command: Self::Short) -> Self {
        Self {
            command,
            working_directory: None,
            default: None,
        }
    }
}
