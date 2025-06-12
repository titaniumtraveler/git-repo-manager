use crate::{
    schemas::{directories::PROJECT_PATHS, utils::entry_map},
    utils,
};
use directories_next::ProjectDirs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub use self::{checkout::Checkout, host::Host, open::Open, storage::Storage};

pub mod checkout;
pub mod host;
pub mod open;
pub mod storage;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "config.toml")]
pub struct Config {
    // TODO: serialize schema-url as soon as there is a standard way to compute that
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
    pub fn default_config(env: &ProjectDirs) -> Self {
        let default = Self::default();
        Self {
            storage: {
                let mut storage = default.storage;
                storage.insert(
                    "default".to_owned(),
                    Storage {
                        path: PathBuf::from_iter([
                            env.data_dir(),
                            "storage/${host/name}/${repo/url/hash}.git".as_ref(),
                        ]),
                        default: Some(true),
                    },
                );
                storage
            },
            checkout: {
                let mut checkout = default.checkout;

                checkout.insert(
                    "default".to_owned(),
                    Checkout {
                        path: PathBuf::from_iter([
                            env.data_dir(),
                            "checkout/${host/name}/{$repo/name}".as_ref(),
                        ]),
                        default: Some(true),
                    },
                );

                checkout
            },
            ..default
        }
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
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
