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
use std::{collections::BTreeMap, mem, ops::Deref, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "open")]
#[schemars(inline)]
#[schemars(transform = Self::transform_schema)]
pub struct Open {
    #[serde(default)]
    pub command: Vec<BString>,
    #[serde(default)]
    pub directory: Option<BString>,
    #[serde(default)]
    pub environment: BTreeMap<BString, BString>,
    #[serde(flatten)]
    pub common: Common,
}

impl VerboseEntry<'_> for Open {
    type Short = Vec<BString>;

    fn from_short(command: Self::Short) -> Self {
        Self {
            command,
            ..Default::default()
        }
    }
}

impl MergeEntry for Open {
    fn merge_config(&self) -> &Merge {
        &self.common.merge
    }

    fn merge_config_mut(&mut self) -> &mut Merge {
        &mut self.common.merge
    }

    fn merge_entries(
        &mut self,
        Self {
            command,
            directory: working_directory,
            mut environment,
            mut common,
        }: Self,
    ) {
        let s = mem::take(self);
        *self = Self {
            command: {
                if !s.command.is_empty() {
                    s.command
                } else {
                    command
                }
            },
            directory: working_directory.or(s.directory),
            environment: {
                environment.extend(s.environment);
                environment
            },
            common: {
                common.merge(s.common);
                common
            },
        }
    }
}

impl Resolve for Open {
    fn common(&self) -> &Common {
        &self.common
    }

    fn task_ref(task: Task<'_>) -> Option<&'_ [u8]> {
        task.open
    }

    fn task_ref_mut<'o, 'i>(task: &'o mut Task<'i>) -> &'o mut Option<&'i [u8]> {
        &mut task.open
    }

    fn resolved_task_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<Self> {
        &mut resolved_task.open
    }

    fn resolved_task_new_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<PathBuf> {
        &mut resolved_task.new_open
    }

    fn config_ref(config: &Config) -> &BTreeMap<BString, Self> {
        &config.open
    }

    fn expand_template_var(
        &self,
        name: &BStr,
        _args: &ResolveArgs,
        buf: &mut Vec<u8>,
    ) -> anyhow::Result<bool> {
        let mut f = |var: Option<&[u8]>, name| {
            buf.extend_from_slice(
                var.as_ref()
                    .ok_or_else(|| anyhow!("{name} has to be set at this point"))?,
            );
            anyhow::Ok(true)
        };

        Ok(match name.as_bytes() {
            b"open.name" => f(self.common.name.as_deref(), name)?,
            b"open.command" => f(self.command.first().map(Deref::deref), name)?,
            b"open.directory" => f(self.directory.as_deref(), name)?,
            _ => false,
        })
    }
    fn expand(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<Self> {
        let command = self
            .command
            .iter()
            .map(|str| self.template(str.as_bstr(), args))
            .collect::<Result<_, _>>()?;
        let environment = self
            .environment
            .iter()
            .map(|(key, val)| {
                anyhow::Ok((
                    self.template(key.as_bstr(), args)?,
                    self.template(val.as_bstr(), args)?,
                ))
            })
            .collect::<Result<_, _>>()?;

        let key = args.key;

        let mut f = |val: &Option<BString>| {
            anyhow::Ok(match val.as_deref() {
                Some(str) => Some(self.template(str.as_bstr(), args)?),
                None => None,
            })
        };

        Ok(Self {
            command,
            environment,
            directory: f(&self.directory)?,
            common: Common {
                name: f(&self.common.name)?.or_else(|| Some(key.into())),
                manifest: None,
                alias: self.common.alias.clone(),
                default: self.common.default,
                merge: self.common.merge.clone(),
            },
        })
    }
}
