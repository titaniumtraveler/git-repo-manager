use crate::{
    schemas::{
        config::{BString, Config, Merge, common::Common, merge::MergeEntry},
        utils::verbose::VerboseEntry,
    },
    state::resolve::{Resolve, ResolveArgs, ResolvedTask},
    task::Task,
};
use anyhow::anyhow;
use bstr::{BStr, ByteSlice};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, mem, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "repo")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Repo {
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

    #[serde(flatten)]
    pub common: Common,
}

impl VerboseEntry<'_> for Repo {
    type Short = BString;

    fn from_short(name: Self::Short) -> Self {
        Self {
            url: Some(name),
            ..Default::default()
        }
    }
}

impl MergeEntry for Repo {
    fn merge_config(&self) -> &Merge {
        &self.common.merge
    }
    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.common.merge
    }

    fn merge_entries(
        &mut self,
        Repo {
            path,
            url,
            mut common,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            path: path.or(s.path),
            url: url.or(s.url),
            common: {
                common.merge(s.common);
                common
            },
        }
    }
}

impl Resolve for Repo {
    fn common(&self) -> &Common {
        &self.common
    }

    fn task_ref(task: Task<'_>) -> Option<&'_ [u8]> {
        task.repo
    }

    fn task_ref_mut<'o, 'i>(task: &'o mut Task<'i>) -> &'o mut Option<&'i [u8]> {
        &mut task.repo
    }

    fn resolved_task_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<Self> {
        &mut resolved_task.repo
    }

    fn resolved_task_new_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<PathBuf> {
        &mut resolved_task.new_repo
    }

    fn config_ref(config: &Config) -> &BTreeMap<BString, Self> {
        &config.repo
    }

    fn expand_template_var(
        &self,
        name: &BStr,
        _args: &ResolveArgs,
        buf: &mut Vec<u8>,
    ) -> anyhow::Result<bool> {
        let mut f = |var: &Option<BString>, name| {
            buf.extend_from_slice(
                var.as_ref()
                    .ok_or_else(|| anyhow!("{name} has to be set at this point"))?,
            );
            anyhow::Ok(true)
        };

        Ok(match name.as_bytes() {
            b"repo.name" => f(&self.common.name, name)?,
            b"repo.path" => f(&self.path, name)?,
            b"repo.url" => f(&self.url, name)?,
            _ => false,
        })
    }

    fn expand(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<Self> {
        let mut f = |val: &Option<BString>| {
            anyhow::Ok(match val.as_deref() {
                Some(str) => Some(self.template(str.as_bstr(), args)?),
                None => None,
            })
        };

        Ok(Self {
            path: f(&self.path)?,
            url: f(&self.url)?,
            common: Common {
                name: f(&self.common.name)?,
                manifest: None,
                alias: self.common.alias.clone(),
                default: self.common.default,
                merge: self.common.merge.clone(),
            },
        })
    }
}
