use crate::{
    schemas::utils::entry_map::{self, VerboseEntry},
    utils,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

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
    hosts: BTreeMap<String, Host>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    repo_storages: BTreeMap<String, RepoStorage>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    checkout_dirs: BTreeMap<String, CheckoutDir>,
    #[serde(deserialize_with = "entry_map::deserialize", default)]
    openers: BTreeMap<String, Opener>,
}

impl Config {
    pub fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        utils::read_toml_from_path(path)
    }

    pub fn resolve_defaults(&mut self) {
        for (k, v) in &mut self.hosts {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        for (k, v) in &mut self.checkout_dirs {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        for (k, v) in &mut self.repo_storages {
            if k == "default" {
                v.default = Some(true);
            } else {
                v.default = Some(v.default.unwrap_or(false));
            }
        }

        for (k, v) in &mut self.openers {
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
pub struct CheckoutDir {
    pub path: PathBuf,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for CheckoutDir {
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
#[schemars(rename = "repo-storage")]
#[schemars(transform = Self::transform_schema)]
pub struct RepoStorage {
    pub path: PathBuf,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for RepoStorage {
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
#[schemars(rename = "opener")]
#[schemars(transform = Self::transform_schema)]
pub struct Opener {
    command: Vec<String>,
    working_directory: Option<PathBuf>,
    default: Option<bool>,
}

impl VerboseEntry<'_> for Opener {
    type Short = Vec<String>;

    fn from_short(command: Self::Short) -> Self {
        Self {
            command,
            working_directory: None,
            default: None,
        }
    }
}
