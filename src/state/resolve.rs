use crate::{
    schemas::{
        config::{BString, Config, ConfigSource, Host, Open, Repo, Tree, common::Common},
        utils::verbose::bool,
    },
    state::{State, config_stack::ConfigStack},
    task::Task,
    template::{Template, Token},
};
use anyhow::anyhow;
use bstr::{BStr, ByteSlice, ByteVec};
use directories_next::ProjectDirs;
use std::{
    borrow::Borrow,
    collections::BTreeMap,
    env,
    ops::ControlFlow,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

pub use self::resolved_task::ResolvedTask;

mod resolved_task;

trait ToControlFlow<B, C> {
    fn to_control_flow(self) -> ControlFlow<B, C>;
}

impl<B, C> ToControlFlow<B, C> for Result<C, B> {
    fn to_control_flow(self) -> ControlFlow<B, C> {
        match self {
            Ok(ok) => ControlFlow::Continue(ok),
            Err(err) => ControlFlow::Break(err),
        }
    }
}

impl State {
    pub fn resolve(&mut self, task: Task, resolved_task: &mut ResolvedTask) -> anyhow::Result<()> {
        self.resolve_task(task, resolved_task)
    }

    pub fn resolve_task(
        &mut self,
        task: Task,
        resolved_task: &mut ResolvedTask,
    ) -> anyhow::Result<()> {
        let mut store_queue = Vec::new();
        let mut stack_queue = Vec::new();
        let mut args = {
            ResolveArgs {
                key: b"".as_bstr(),
                path: Path::new(""),
                dirs: &self.dirs,
                task,
                resolved_task,
                config_stack: const {
                    &ConfigStack {
                        store: BTreeMap::new(),
                        stack: Vec::new(),
                    }
                },
                store_queue: &mut store_queue,
                stack_queue: &mut stack_queue,
            }
        };

        match task.host.is_some_and(<[u8]>::is_empty) {
            true => args.resolve_in_stack(Host::by_default, &mut self.configs)?,
            false => args.resolve_in_stack(Host::by_task_name, &mut self.configs)?,
        }
        match task.repo.is_some_and(<[u8]>::is_empty) {
            true => args.resolve_in_stack(Repo::by_default, &mut self.configs)?,
            false => args.resolve_in_stack(Repo::by_task_name, &mut self.configs)?,
        }
        match task.tree.is_some_and(<[u8]>::is_empty) {
            true => args.resolve_in_stack(Tree::by_default, &mut self.configs)?,
            false => args.resolve_in_stack(Tree::by_task_name, &mut self.configs)?,
        }
        match task.open.is_some_and(<[u8]>::is_empty) {
            true => args.resolve_in_stack(Open::by_default, &mut self.configs)?,
            false => args.resolve_in_stack(Open::by_task_name, &mut self.configs)?,
        }

        args.resolve_in_stack(Host::by_manifest, &mut self.configs)?;
        args.resolve_in_stack(Repo::by_manifest, &mut self.configs)?;
        args.resolve_in_stack(Tree::by_manifest, &mut self.configs)?;
        args.resolve_in_stack(Open::by_manifest, &mut self.configs)?;

        Ok(())
    }
}

pub trait Resolve: Sized {
    fn common(&self) -> &Common;
    fn task_ref(task: Task<'_>) -> Option<&'_ [u8]>;
    fn task_ref_mut<'o, 'i>(task: &'o mut Task<'i>) -> &'o mut Option<&'i [u8]>;
    fn resolved_task_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<Self>;
    fn resolved_task_new_mut_ref(resolved_task: &mut ResolvedTask) -> &mut Option<PathBuf>;
    fn config_ref(config: &Config) -> &BTreeMap<BString, Self>;

    fn by_task_name(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<()> {
        let Some(task_ref) = Self::task_ref(args.task) else {
            return Ok(());
        };

        if args.key == task_ref
            || self
                .common()
                .name
                .as_ref()
                .is_some_and(|name| name.as_bstr() == task_ref)
            || self
                .common()
                .alias
                .iter()
                .any(|name| name.as_bstr() == task_ref)
        {
            *Self::resolved_task_mut_ref(args.resolved_task) = Some(self.expand_manifest(args)?);
        }
        Ok(())
    }

    fn by_manifest(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<()> {
        let Some(task_ref) = Self::task_ref(args.task) else {
            return Ok(());
        };

        if self.common().manifest.is_some()
            && (match task_ref.is_empty() {
                false => {
                    args.key == task_ref
                        || self.common().name.as_ref().is_some_and(|name| {
                            name.as_bstr() == task_ref
                                || Template::new(name).any(|token| matches!(token, Token::Var(_)))
                        })
                        || self
                            .common()
                            .alias
                            .iter()
                            .any(|name| name.as_bstr() == task_ref)
                }
                true => self.common().default,
            })
        {
            *Self::resolved_task_mut_ref(args.resolved_task) = Some(self.expand_manifest(args)?);
        }
        Ok(())
    }

    fn by_default(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<()> {
        if self.common().default {
            *Self::resolved_task_mut_ref(args.resolved_task) = Some(self.expand_manifest(args)?);
        }
        Ok(())
    }

    fn expand_template_var(
        &self,
        name: &BStr,
        args: &ResolveArgs,
        buf: &mut Vec<u8>,
    ) -> anyhow::Result<bool>;

    fn template(&self, bstr: &BStr, args: &mut ResolveArgs) -> anyhow::Result<BString> {
        let mut buf = Vec::with_capacity(bstr.len());

        for token in Template::new(bstr.as_ref()) {
            match token {
                Token::Str(str) => {
                    buf.push_str(str);
                }
                Token::Var(var) => match () {
                    _ if args.expand_template_var(var.as_bstr(), &mut buf)? => {}
                    _ => {
                        return Err(anyhow!(
                            "could not resolve variable `{var}`",
                            var = var.as_bstr()
                        ));
                    }
                },
            }
        }

        Ok(BString(buf))
    }

    fn expand(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<Self>;
    fn expand_manifest(&self, args: &mut ResolveArgs<'_>) -> anyhow::Result<Self> {
        if let Some(path) = &self.common().manifest {
            let path = self.template(path.as_bstr(), args)?.0.into_path_buf()?;
            let (config, src) = { (Config::from_file(&path)?, ConfigSource::File(path)) };

            let name = self.template(
                self.common()
                    .name
                    .as_deref()
                    .ok_or_else(|| anyhow!("name needs to be set here"))?
                    .as_bstr(),
                args,
            )?;

            let mut task = args.task;
            *Self::task_ref_mut(&mut task) = Some(&name);
            ResolveArgs {
                key: args.key,
                path: args.path,
                dirs: args.dirs,
                task,
                resolved_task: args.resolved_task,
                config_stack: args.config_stack,
                store_queue: args.store_queue,
                stack_queue: args.stack_queue,
            }
            .resolve_in_config(Self::by_task_name, &src, &config, args.config_stack)
            .break_value()
            .unwrap_or(Ok(()))?;

            let ConfigSource::File(src) = src else {
                unreachable!()
            };

            args.stack_queue.push(src.clone());
            args.store_queue.push((src, config));

            match Self::resolved_task_mut_ref(args.resolved_task) {
                val @ Some(_) => {
                    let val = val.take().expect("we just checked that `val` is `Some(_)`");
                    val.expand_manifest(args)
                }
                None => {
                    let (path, _) = args.store_queue.last().expect("we just pushed to this");
                    *Self::resolved_task_new_mut_ref(args.resolved_task) = Some(path.clone());

                    self.expand(args)
                }
            }
        } else {
            self.expand(args)
        }
    }
}

pub struct ResolveArgs<'a> {
    pub key: &'a BStr,
    pub path: &'a Path,
    pub dirs: &'a ProjectDirs,

    pub task: Task<'a>,
    pub resolved_task: &'a mut ResolvedTask,

    pub config_stack: &'a ConfigStack,
    pub store_queue: &'a mut Vec<(PathBuf, Config)>,
    pub stack_queue: &'a mut Vec<PathBuf>,
}

impl<'a> ResolveArgs<'a> {
    fn resolve_in_stack<T, F>(
        &mut self,
        mut f: F,
        config_stack: &mut ConfigStack,
    ) -> anyhow::Result<()>
    where
        F: FnMut(&T, &mut ResolveArgs) -> anyhow::Result<()>,
        T: Resolve,
    {
        config_stack
            .cursors()
            .try_for_each(|cur| {
                if T::resolved_task_mut_ref(&mut *self.resolved_task).is_some() {
                    return ControlFlow::Break(Ok(()));
                }

                let (src, config) = config_stack
                    .with_cursor(cur)
                    .map_err(Err)
                    .to_control_flow()?;

                self.resolve_in_config(&mut f, src, config, config_stack)
            })
            .break_value()
            .unwrap_or(Ok(()))?;

        config_stack.store.extend(
            self.store_queue
                .drain(..)
                .map(|(path, config)| (ConfigSource::File(path), config)),
        );
        config_stack
            .stack
            .extend(self.stack_queue.drain(..).map(ConfigSource::File));
        Ok(())
    }

    fn resolve_in_config<T, F>(
        &mut self,
        mut f: F,
        src: &ConfigSource,
        config: &Config,
        config_stack: &ConfigStack,
    ) -> ControlFlow<anyhow::Result<()>>
    where
        F: FnMut(&T, &mut ResolveArgs) -> anyhow::Result<()>,
        T: Resolve,
    {
        T::config_ref(config).iter().try_for_each(|(key, val)| {
            if T::resolved_task_mut_ref(&mut *self.resolved_task).is_none() {
                f(
                    val,
                    &mut ResolveArgs {
                        key: key.as_bstr(),
                        path: src.borrow(),
                        dirs: self.dirs,
                        task: self.task,
                        resolved_task: self.resolved_task,
                        config_stack,
                        store_queue: self.store_queue,
                        stack_queue: self.stack_queue,
                    },
                )
                .map_err(Err)
                .to_control_flow()
            } else {
                ControlFlow::Break(Ok(()))
            }
        })
    }

    fn expand_template_var(&self, name: &BStr, buf: &mut Vec<u8>) -> anyhow::Result<bool> {
        Ok(match &self {
            Self {
                resolved_task:
                    ResolvedTask {
                        host: Some(host), ..
                    },
                ..
            } if host.expand_template_var(name, self, buf)? => true,
            Self {
                resolved_task:
                    ResolvedTask {
                        repo: Some(repo), ..
                    },
                ..
            } if repo.expand_template_var(name, self, buf)? => true,
            Self {
                resolved_task:
                    ResolvedTask {
                        tree: Some(tree), ..
                    },
                ..
            } if tree.expand_template_var(name, self, buf)? => true,
            Self {
                resolved_task:
                    ResolvedTask {
                        open: Some(open), ..
                    },
                ..
            } if open.expand_template_var(name, self, buf)? => true,

            Self {
                task: Task {
                    host: Some(host), ..
                },
                ..
            } if name == "host.name" => {
                buf.extend_from_slice(host);
                true
            }
            Self {
                task: Task {
                    repo: Some(repo), ..
                },
                ..
            } if name == "repo.name" => {
                buf.extend_from_slice(repo);
                true
            }
            Self {
                task: Task {
                    tree: Some(tree), ..
                },
                ..
            } if name == "tree.name" => {
                buf.extend_from_slice(tree);
                true
            }
            Self {
                task: Task {
                    open: Some(open), ..
                },
                ..
            } if name == "open.name" => {
                buf.extend_from_slice(open);
                true
            }

            _ if name.starts_with_str("env.") => {
                buf.extend_from_slice(
                    env::var_os(name["env.".len()..].to_os_str()?)
                        .unwrap()
                        .as_bytes(),
                );
                true
            }

            _ if name.starts_with_str("dir.") => match &*name["dir.".len()..] {
                b"config" => {
                    buf.extend_from_slice(self.dirs.config_dir().as_os_str().as_bytes());
                    true
                }
                b"data" => {
                    buf.extend_from_slice(self.dirs.data_dir().as_os_str().as_bytes());
                    true
                }
                _ => false,
            },

            _ => false,
        })
    }
}
