use crate::{
    schemas::{
        config::merge::{Behavior, MergeEntry, OnlyIf},
        utils::entry_map,
    },
    utils,
};
use directories_next::ProjectDirs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::{
    collections::{BTreeMap, btree_map::Entry},
    mem,
    path::{Path, PathBuf},
};

pub use self::{checkout::Checkout, host::Host, merge::Merge, open::Open, storage::Storage};

pub mod checkout;
pub mod host;
pub mod merge;
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
    #[serde(default)]
    pub default: ConfigDefault,
}

impl Config {
    pub fn default_config(env: &ProjectDirs) -> Self {
        const MERGE_DEFAULT: Merge = Merge {
            only_if: OnlyIf::NotPresent,
            behavior: Behavior::Replace,
        };

        let default = Self::default();
        Self {
            storage: {
                let mut storage = default.storage;
                storage.insert(
                    "default".to_owned(),
                    Storage {
                        path: Some(PathBuf::from_iter([
                            env.data_dir(),
                            "storage/${host/name}/${repo/url/hash}.git".as_ref(),
                        ])),
                        default: true,
                        merge: MERGE_DEFAULT,
                    },
                );
                storage
            },
            checkout: {
                let mut checkout = default.checkout;

                checkout.insert(
                    "default".to_owned(),
                    Checkout {
                        path: Some(PathBuf::from_iter([
                            env.data_dir(),
                            "checkout/${host/name}/{$repo/name}".as_ref(),
                        ])),
                        default: true,
                        merge: MERGE_DEFAULT,
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
        fn find_default<T>(
            map: &BTreeMap<String, T>,
            mut is_default: impl FnMut(&&T) -> bool,
        ) -> Option<String> {
            map.iter()
                .find(|(_, v)| is_default(v))
                .map(|(k, _)| k.as_str())
                .or_else(|| map.contains_key("default").then_some("default"))
                .map(ToOwned::to_owned)
        }

        let ConfigDefault {
            host,
            storage,
            checkout,
            open,
        } = mem::take(&mut self.default);

        self.default = ConfigDefault {
            host: host.or_else(|| find_default(&self.host, |e| e.default)),
            storage: storage.or_else(|| find_default(&self.storage, |e| e.default)),
            checkout: checkout.or_else(|| find_default(&self.checkout, |e| e.default)),
            open: open.or_else(|| find_default(&self.open, |e| e.default)),
        };
    }

    pub fn merge(&mut self, other: Self) {
        let Self {
            schema: _,
            host,
            storage,
            checkout,
            open,
            default,
        } = other;
        merge_entries(&mut self.host, host);
        merge_entries(&mut self.storage, storage);
        merge_entries(&mut self.checkout, checkout);
        merge_entries(&mut self.open, open);

        pick_default(&mut self.default.host, default.host, &self.host);
        pick_default(&mut self.default.storage, default.storage, &self.storage);
        pick_default(&mut self.default.checkout, default.checkout, &self.checkout);
        pick_default(&mut self.default.open, default.open, &self.open);
    }
}

fn merge_entries<T: std::fmt::Debug + MergeEntry>(
    s: &mut BTreeMap<String, T>,
    o: BTreeMap<String, T>,
) {
    o.into_iter().for_each(|(key, value)| match s.entry(key) {
        Entry::Vacant(e) => {
            e.insert(value);
        }
        Entry::Occupied(mut e) => {
            e.get_mut().merge(value);
        }
    });
}

fn pick_default<T>(default: &mut Option<String>, other: Option<String>, map: &BTreeMap<String, T>) {
    if let Some(other) = other {
        if map.contains_key(&other) {
            *default = Some(other)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "default")]
pub struct ConfigDefault {
    pub host: Option<String>,
    pub storage: Option<String>,
    pub checkout: Option<String>,
    pub open: Option<String>,
}
