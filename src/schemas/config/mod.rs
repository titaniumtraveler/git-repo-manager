use crate::{
    schemas::{
        config::merge::{Behavior, MergeEntry, OnlyIf},
        utils::verbose::{self, VerboseEntry},
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
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
};

pub use self::{bstring::BString, host::Host, merge::Merge, open::Open, repo::Repo, tree::Tree};

pub mod bstring;
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
    pub host: BTreeMap<String, Host>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub repo: BTreeMap<String, Repo>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
    pub tree: BTreeMap<String, Tree>,
    #[serde(deserialize_with = "verbose::map::deserialize", default)]
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
            repo: {
                let mut storage = default.repo;
                storage.insert(
                    "default".to_owned(),
                    Repo {
                        name: Some(BString("repo/${host.name}/${repo.name}".into())),
                        path: Some(BString(
                            PathBuf::from_iter([env.data_dir(), "${entry.name}".as_ref()])
                                .into_os_string()
                                .into_vec(),
                        )),
                        default: repo::RepoDefault::from_short(true),
                        merge: MERGE_DEFAULT,
                        url: None,
                        manifest: Some(BString("".into())),
                        alias: Vec::new(),
                    },
                );
                storage
            },
            tree: {
                let mut tree = default.tree;

                tree.insert(
                    "default".to_owned(),
                    Tree {
                        name: Some(BString(
                            "tree/${host.name}/${repo.name}/${tree.name}".into(),
                        )),
                        path: Some(BString(
                            PathBuf::from_iter([env.data_dir(), "${entry.name}".as_ref()])
                                .into_os_string()
                                .into_vec(),
                        )),
                        default: <_>::from_short(true),
                        merge: MERGE_DEFAULT,
                        repo: None,
                        manifest: Some(BString("".into())),
                        alias: Vec::new(),
                    },
                );

                tree
            },
            ..default
        }
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        utils::read_toml_from_path(path).with_context(|| {
            anyhow!(
                "failed to read config file from `{path}`",
                path = path.display()
            )
        })
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
            repo,
            tree,
            open,
        } = mem::take(&mut self.default);

        self.default = ConfigDefault {
            host: host.or_else(|| find_default(&self.host, |e| e.default)),
            repo: repo.or_else(|| find_default(&self.repo, |e| e.default.default)),
            tree: tree.or_else(|| find_default(&self.tree, |e| e.default.default)),
            open: open.or_else(|| find_default(&self.open, |e| e.default)),
        };
    }

    pub fn merge(&mut self, other: Self) {
        let Self {
            schema: _,
            host,
            repo: storage,
            tree,
            open,
            default,
        } = other;
        merge_entries(&mut self.host, host);
        merge_entries(&mut self.repo, storage);
        merge_entries(&mut self.tree, tree);
        merge_entries(&mut self.open, open);

        pick_default(&mut self.default.host, default.host, &self.host);
        pick_default(&mut self.default.repo, default.repo, &self.repo);
        pick_default(&mut self.default.tree, default.tree, &self.tree);
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
#[schemars(rename = "config/default")]
pub struct ConfigDefault {
    pub host: Option<String>,
    pub repo: Option<String>,
    pub tree: Option<String>,
    pub open: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, PartialEq, Eq, PartialOrd, Ord)]
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
