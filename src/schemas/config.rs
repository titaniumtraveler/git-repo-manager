use crate::schemas::utils::entry_map::{self, VerboseEntry};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "kebab-case")]
pub struct Config {
    #[serde(with = "entry_map", default)]
    hosts: BTreeMap<String, Host>,
    #[serde(with = "entry_map", default)]
    checkout_dirs: BTreeMap<String, CheckoutDir>,
    #[serde(with = "entry_map", default)]
    repo_storages: BTreeMap<String, RepoStorage>,
    #[serde(with = "entry_map", default)]
    openers: BTreeMap<String, Opener>,
}

impl Config {
    pub fn read_from_path(path: &Path) -> anyhow::Result<Self> {
        let str = std::fs::read_to_string(path)?;
        let config = toml::from_str(&str)?;
        Ok(config)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub host: String,
    pub aliases: BTreeSet<String>,
    pub default: Option<bool>,
}

impl VerboseEntry<'_> for Host {
    type Short = String;

    fn from_short(host: Self::Short) -> Self {
        Self {
            host,
            aliases: BTreeSet::new(),
            default: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Opener {
    cmd: Vec<String>,
    default: Option<bool>,
}

impl VerboseEntry<'_> for Opener {
    type Short = Vec<String>;

    fn from_short(cmd: Self::Short) -> Self {
        Self { cmd, default: None }
    }
}
