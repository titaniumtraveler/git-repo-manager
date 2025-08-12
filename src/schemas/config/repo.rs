use crate::schemas::{
    config::{BString, Merge, merge::MergeEntry},
    utils::verbose::{self, VerboseEntry},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "repo")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Repo {
    /// unique identifier of the repo
    /// - if none, derive from path
    /// - if template, might point to multiple
    #[serde(default)]
    pub name: Option<BString>,

    /// path pointing to the repo
    /// - if template, potentially pointing to multiple
    /// - allows for `${entry/name}`
    ///   ```toml
    ///   [tree.default]
    ///   path = "~/${entry.name}"
    ///   name = "projects/${host.name}/${repo.name}.git"
    ///   manifest = true
    ///   ```
    #[serde(default)]
    pub path: Option<BString>,

    #[serde(default)]
    pub url: Option<BString>,

    /// file describing the sub trees of the `tree` that contains multiple trees
    /// - defaults to true if `path` references `${repo/*}`
    /// - if true, derive from path
    ///   - would be `~/projects/config.toml`
    /// - if path template, that path template
    ///   - if not absolute, relative to
    #[serde(default)]
    pub manifest: Option<BString>,

    #[serde(default)]
    pub alias: Vec<BString>,

    #[serde(deserialize_with = "verbose::bool::deserialize", default)]
    pub default: RepoDefault,

    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Repo {
    type Short = BString;

    fn from_short(name: Self::Short) -> Self {
        Self {
            name: Some(name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "repo::default")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct RepoDefault {
    #[serde(default)]
    pub default: bool,

    /// The default host of this repo
    #[serde(default)]
    pub host: Option<BString>,

    /// The default tree of this repo
    #[serde(default)]
    pub tree: Option<BString>,

    /// The default open of this repo
    #[serde(default)]
    pub open: Option<BString>,
}

impl VerboseEntry<'_> for RepoDefault {
    type Short = bool;

    fn from_short(default: Self::Short) -> Self {
        Self {
            default,
            host: None,
            tree: None,
            open: None,
        }
    }
}

impl RepoDefault {
    pub fn merge(
        &mut self,
        Self {
            default,
            host,
            tree,
            open,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            default: default | s.default,
            host: host.or(s.host),
            tree: tree.or(s.tree),
            open: open.or(s.open),
        }
    }
}

impl MergeEntry for Repo {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Repo {
            merge: _,
            name,
            path,
            url,
            manifest,
            mut alias,
            mut default,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            name: name.or(s.name),
            path: path.or(s.path),
            url: url.or(s.url),
            manifest: manifest.or(s.manifest),
            alias: {
                alias.extend(s.alias);
                alias
            },
            default: {
                default.merge(s.default);
                default
            },
            merge: s.merge,
        }
    }
}
