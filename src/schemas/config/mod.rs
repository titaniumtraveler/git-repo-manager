use crate::{
    schemas::{
        config::{
            common::Common,
            merge::{Behavior, MergeEntry, OnlyIf},
        },
        utils::verbose,
    },
    utils,
};
use anyhow::{Context, anyhow};
use directories_next::ProjectDirs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, btree_map::Entry},
    fmt::Debug,
    mem,
    path::{Path, PathBuf},
};

pub use self::{bstring::BString, host::Host, merge::Merge, open::Open, repo::Repo, tree::Tree};

pub mod bstring;
pub mod common;
pub mod host;
pub mod merge;
pub mod open;
pub mod repo;
pub mod tree;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "config.toml")]
pub struct Config {
    // TODO: serialize schema-url as soon as there is a standard way to compute that
    #[allow(dead_code)]
    #[serde(rename = "$schema", skip_serializing, default)]
    #[schemars(with = "String", default)]
    schema: IgnoredAny,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub host: BTreeMap<BString, Host>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub repo: BTreeMap<BString, Repo>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub tree: BTreeMap<BString, Tree>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub open: BTreeMap<BString, Open>,
    #[serde(default)]
    pub default: ConfigDefault,
}

impl Config {
    pub fn default_config(_env: &ProjectDirs) -> Self {
        const MERGE_DEFAULT: Merge = Merge {
            only_if: OnlyIf::NotPresent,
            behavior: Behavior::Replace,
        };

        let default = Self::default();
        Self {
            repo: {
                let mut storage = default.repo;
                storage.insert(
                    "default".into(),
                    Repo {
                        path: Some("$data-dir/repo/${host.name}/${repo.name}.git".into()),
                        url: Some("${repo.name}".into()),
                        common: Common {
                            name: None,
                            alias: Vec::new(),
                            manifest: Some("$data_dir/repo/repo-config.toml".into()),
                            default: true,
                            merge: MERGE_DEFAULT,
                        },
                    },
                );
                storage
            },
            tree: {
                let mut tree = default.tree;

                tree.insert(
                    "default".into(),
                    Tree {
                        path: Some("$data_dir/tree/${host.name}/${repo.name}/${tree.name}".into()),
                        branch: Some("${tree.name}".into()),
                        common: Common {
                            name: Some("${host.name}/${repo.name}/${tree.name}".into()),
                            alias: Vec::new(),
                            manifest: Some("$data_dir/tree/tree-config.toml".into()),
                            default: true,
                            merge: MERGE_DEFAULT,
                        },
                    },
                );

                tree
            },
            ..default
        }
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        utils::read_toml_from_path(path)
            .map(Option::unwrap_or_default)
            .with_context(|| {
                anyhow!(
                    "failed to read config file from `{path}`",
                    path = path.display()
                )
            })
    }

    pub fn resolve_defaults(&mut self) {
        fn find_default<T>(
            map: &BTreeMap<BString, T>,
            mut is_default: impl FnMut(&&T) -> bool,
        ) -> Option<BString> {
            map.iter()
                .find(|(_, v)| is_default(v))
                .map(|(k, _)| k.0.as_slice())
                .or_else(|| {
                    map.contains_key(b"default".as_slice())
                        .then_some("default".as_bytes())
                })
                .map(Into::into)
        }

        let ConfigDefault {
            host,
            repo,
            tree,
            open,
        } = mem::take(&mut self.default);

        self.default = ConfigDefault {
            host: host.or_else(|| find_default(&self.host, |e| e.common.default)),
            repo: repo.or_else(|| find_default(&self.repo, |e| e.common.default)),
            tree: tree.or_else(|| find_default(&self.tree, |e| e.common.default)),
            open: open.or_else(|| find_default(&self.open, |e| e.common.default)),
        };
    }

    pub fn merge(&mut self, other: Self) {
        let Self {
            schema: _,
            host,
            repo,
            tree,
            open,
            default,
        } = other;
        merge_entries(&mut self.host, host);
        merge_entries(&mut self.repo, repo);
        merge_entries(&mut self.tree, tree);
        merge_entries(&mut self.open, open);

        pick_default(&mut self.default.host, default.host, &self.host);
        pick_default(&mut self.default.repo, default.repo, &self.repo);
        pick_default(&mut self.default.tree, default.tree, &self.tree);
        pick_default(&mut self.default.open, default.open, &self.open);
    }
}

fn merge_entries<T: std::fmt::Debug + MergeEntry>(
    s: &mut BTreeMap<BString, T>,
    o: BTreeMap<BString, T>,
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

fn pick_default<T>(
    default: &mut Option<BString>,
    other: Option<BString>,
    map: &BTreeMap<BString, T>,
) {
    if let Some(other) = other
        && map.contains_key(&other)
    {
        *default = Some(other)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
#[schemars(rename = "config/default")]
pub struct ConfigDefault {
    pub host: Option<BString>,
    pub repo: Option<BString>,
    pub tree: Option<BString>,
    pub open: Option<BString>,
}

#[derive(
    Debug, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
#[schemars(rename = "source")]
pub enum ConfigSource {
    #[default]
    None,
    Builtin,
    File(PathBuf),
    Resolved,
}

impl Borrow<Path> for ConfigSource {
    fn borrow(&self) -> &Path {
        match self {
            ConfigSource::File(path) => path,
            _ => Path::new("<invalid>"),
        }
    }
}

impl PartialEq<Path> for ConfigSource {
    fn eq(&self, other: &Path) -> bool {
        match self {
            ConfigSource::None => false,
            ConfigSource::Builtin => false,
            ConfigSource::File(path) => path.eq(other),
            ConfigSource::Resolved => false,
        }
    }
}
