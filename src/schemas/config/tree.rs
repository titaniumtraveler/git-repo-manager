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
#[schemars(rename = "tree")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Tree {
    /// unique identifier of the tree
    /// - if none, derive from path
    /// - if template, might point to multiple
    #[serde(default)]
    pub name: Option<BString>,

    /// path pointing to the tree
    /// - if template, potentially pointing to multiple
    /// - allows for `${entry/name}`
    ///   ```toml
    ///   [tree.default]
    ///   path = "~/${entry.name}"
    ///   name = "projects/${host.name}/${repo.name}/${tree.name}"
    ///   manifest = true
    ///   ```
    #[serde(default)]
    pub path: Option<BString>,

    /// Repo this tree is associated with
    #[serde(default)]
    pub repo: Option<BString>,

    /// file describing the sub repos of the `repo` that contains multiple repos
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
    pub default: TreeDefault,

    #[serde(default)]
    pub merge: Merge,
}

impl VerboseEntry<'_> for Tree {
    type Short = BString;

    fn from_short(name: Self::Short) -> Self {
        Self {
            name: Some(name),
            ..Default::default()
        }
    }
}

impl MergeEntry for Tree {
    fn merge_config(&self) -> &Merge {
        &self.merge
    }
    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.merge
    }

    fn merge_entries(
        &mut self,
        Self {
            merge: _,
            name,
            path,
            repo,
            manifest,
            mut alias,
            mut default,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            name: name.or(s.name),
            path: path.or(s.path),
            repo: repo.or(s.repo),
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
        };
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "tree::default")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct TreeDefault {
    #[serde(default)]
    pub default: bool,

    /// The default host of this repo
    #[serde(default)]
    pub host: Option<BString>,

    /// The default open of this repo
    #[serde(default)]
    pub open: Option<BString>,
}

impl VerboseEntry<'_> for TreeDefault {
    type Short = bool;

    fn from_short(default: Self::Short) -> Self {
        Self {
            default,
            host: None,
            open: None,
        }
    }
}

impl TreeDefault {
    pub fn merge(
        &mut self,
        Self {
            default,
            host,
            open,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            default: default | s.default,
            host: host.or(s.host),
            open: open.or(s.open),
        }
    }
}
