use crate::{
    schemas::config::{Config, Host, Open, Repo, Tree},
    state::State,
    task::Task,
};
use anyhow::{Context, anyhow};
use bstr::{BStr, ByteSlice};
use git2::{Repository, Worktree, build::RepoBuilder};
use std::{fmt::Display, fs, process::Command};

impl State {
    pub fn resolve(
        &mut self,
        task: &Task,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        self.resolve_task(task, resolved_task, config)
    }

    pub fn resolve_host_by_name(
        &mut self,
        name: &BStr,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.host = config
            .host
            .iter()
            .find_map(|(k, v)| {
                (k.as_bytes() == name || v.alias.iter().any(|str| str.0 == name)).then_some(v)
            })
            .cloned();
        Ok(())
    }

    pub fn resolve_host_by_manifest(
        &mut self,
        _resolved_config: &mut Config,
        _config: &mut Config,
    ) -> anyhow::Result<()> {
        // TODO: actually implement this
        Ok(())
    }

    pub fn resolve_host_by_default(
        &mut self,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.host = config
            .host
            .iter()
            .find_map(|(_, v)| v.default.then_some(v))
            .cloned();
        Ok(())
    }

    pub fn resolve_repo_by_name(
        &mut self,
        name: &BStr,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.repo = config
            .repo
            .iter()
            .find_map(|(k, v)| {
                (k.as_bytes() == name
                    || v.name.as_ref().is_some_and(|str| str.0 == name)
                    || v.alias.iter().any(|str| str == name))
                .then_some(v)
            })
            .cloned();
        Ok(())
    }

    pub fn resolve_repo_by_manifest(
        &mut self,
        _resolved_config: &mut Config,
        _config: &mut Config,
    ) -> anyhow::Result<()> {
        // TODO: actually implement this
        Ok(())
    }

    pub fn resolve_repo_by_default(
        &mut self,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.repo = config
            .repo
            .iter()
            .find_map(|(_, v)| v.default.default.then_some(v))
            .cloned();
        Ok(())
    }

    pub fn resolve_tree_by_name(
        &mut self,
        name: &BStr,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.tree = config
            .tree
            .iter()
            .find_map(|(k, v)| {
                (k.as_bytes() == name
                    || v.name.as_ref().is_some_and(|str| str.0 == name)
                    || v.alias.iter().any(|str| str.0 == name))
                .then_some(v)
            })
            .cloned();
        Ok(())
    }

    pub fn resolve_tree_by_manifest(
        &mut self,
        _resolved_config: &mut Config,
        _config: &mut Config,
    ) -> anyhow::Result<()> {
        // TODO: actually implement this
        Ok(())
    }

    pub fn resolve_tree_by_default(
        &mut self,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.tree = config
            .tree
            .iter()
            .find_map(|(_, v)| v.default.default.then_some(v))
            .cloned();
        Ok(())
    }

    pub fn resolve_open_by_name(
        &mut self,
        name: &BStr,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.open = config
            .open
            .iter()
            .find_map(|(k, v)| (k.as_bytes() == name).then_some(v))
            .cloned();
        Ok(())
    }

    pub fn resolve_open_by_manifest(
        &mut self,
        _resolved_config: &mut Config,
        _config: &mut Config,
    ) -> anyhow::Result<()> {
        // TODO: actually implement this
        Ok(())
    }

    pub fn resolve_open_by_default(
        &mut self,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        resolved_task.open = config
            .open
            .iter()
            .find_map(|(_, v)| v.default.then_some(v))
            .cloned();
        Ok(())
    }

    pub fn resolve_task(
        &mut self,
        task: &Task,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        match task.host {
            _ if resolved_task.host.is_some() => {}
            None => {}
            Some(b"") => self.resolve_host_by_default(resolved_task, config)?,
            Some(host) => self.resolve_host_by_name(host.as_bstr(), resolved_task, config)?,
        }

        match task.repo {
            _ if resolved_task.repo.is_some() => {}
            None | Some(b"") => self.resolve_repo_by_default(resolved_task, config)?,
            Some(repo) => self.resolve_repo_by_name(repo.as_bstr(), resolved_task, config)?,
        }

        match task.tree {
            _ if resolved_task.tree.is_some() => {}
            None => {}
            Some(b"") => self.resolve_tree_by_default(resolved_task, config)?,
            Some(tree) => self.resolve_tree_by_name(tree.as_bstr(), resolved_task, config)?,
        }

        match task.open {
            _ if resolved_task.open.is_some() => {}
            None => {}
            Some(b"") => self.resolve_open_by_default(resolved_task, config)?,
            Some(open) => self.resolve_open_by_name(open.as_bstr(), resolved_task, config)?,
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ResolvedTask {
    pub host: Option<Host>,
    pub repo: Option<Repo>,
    pub tree: Option<Tree>,
    pub open: Option<Open>,
}

impl ResolvedTask {
    pub fn run(&self) -> anyhow::Result<()> {
        let repo = self.repo()?;
        self.tree(&repo)?;
        self.open()?;

        Ok(())
    }

    pub fn repo(&self) -> anyhow::Result<git2::Repository> {
        let path = match &self.repo {
            Some(Repo {
                path: Some(path), ..
            }) => path.0.to_path()?,
            _ => return Err(anyhow!("")),
        };

        // TODO: allow force `--bare`. Maybe just warn when not as expected.
        match Repository::open_ext(
            path,
            {
                use git2::RepositoryOpenFlags as RepoFlags;

                RepoFlags::NO_SEARCH | RepoFlags::NO_DOTGIT
            },
            &[] as &[&std::ffi::OsStr],
        ) {
            Ok(repo) => Ok(repo),
            Err(err) if err.code() == git2::ErrorCode::NotFound => {
                let mut builder = RepoBuilder::new();

                let (_key, url, _pass_program) = match &self.host {
                    Some(Host {
                        host: Some(url),
                        key: Some(key),
                        pass_program,
                        ..
                    }) => (
                        key.as_bstr().to_path()?,
                        url.as_bstr().to_str()?,
                        pass_program,
                    ),
                    _ => return Err(anyhow!("host is missing fields required fields")),
                };

                builder
                    .fetch_options({
                        use git2::{Cred, RemoteCallbacks};

                        let mut callbacks = RemoteCallbacks::new();
                        callbacks.credentials(|_url, username_from_url, _allowed_types| {
                            Cred::ssh_key_from_agent(username_from_url.ok_or_else(|| {
                                into_git2_err("missing username to use for ssh agent")
                            })?)
                        });

                        let mut fo = git2::FetchOptions::new();
                        fo.remote_callbacks(callbacks);

                        fo
                    })
                    .bare(true)
                    .clone(url, path)
                    .with_context(|| anyhow!("failed to clone repo from {url}"))
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn tree(&self, repository: &git2::Repository) -> anyhow::Result<Worktree> {
        let (name, path) = match &self.tree {
            Some(Tree {
                name: Some(name),
                path: Some(path),
                ..
            }) => (name.0.as_bstr().to_str()?, path.0.as_bstr().to_path()?),
            _ => return Err(anyhow!("name and path of the tree must be defined")),
        };

        if path.exists() {
            repository.find_worktree(name).map_err(Into::into)
        } else {
            use git2::{
                BranchType::{Local, Remote},
                WorktreeAddOptions,
            };

            fs::create_dir_all(
                path.parent()
                    .context("tree.path should have parent to place the worktree into")?,
            )?;

            // Borrow checker makes me sad here. If you need to, just keep temporaries
            // around as long as they are referenced...
            let reference;
            repository
                .worktree(
                    name,
                    path,
                    Some(&'opts: {
                        let mut opts = WorktreeAddOptions::new();

                        if let Ok(branch) = repository.find_branch(name, Local) {
                            reference = branch.into_reference();
                            opts.reference(Some(&reference));
                            break 'opts opts;
                        }

                        if let Ok(branch) = repository.find_branch(name, Remote) {
                            reference = branch.into_reference();
                            opts.reference(Some(&reference));
                            break 'opts opts;
                        }

                        reference = repository.head()?;
                        opts.reference(Some(&reference));

                        opts
                    }),
                )
                .context("failed to create worktree")
        }
    }

    pub fn open(&self) -> anyhow::Result<()> {
        let Some(open) = &self.open else {
            return Err(anyhow!("missing open"));
        };

        let path = {
            match (&self.tree, &self.repo) {
                (
                    Some(Tree {
                        path: Some(path), ..
                    }),
                    _,
                )
                | (
                    _,
                    Some(Repo {
                        path: Some(path), ..
                    }),
                ) => path,
                _ => return Err(anyhow!("open needs a path")),
            }
        }
        .as_bstr()
        .to_path()?;

        let mut cmd = build_cmd(open.command.iter().map(|str| str.0.as_bstr()))?;
        cmd.current_dir(path);
        cmd.status()?;

        Ok(())
    }
}

fn build_cmd<'a>(command: impl IntoIterator<Item = &'a BStr>) -> anyhow::Result<Command> {
    let mut iter = command.into_iter();
    let cmd = iter.next().context("command needs to be non-empty")?;

    let mut err = Ok(());
    let mut cmd = Command::new(cmd.to_os_str()?);
    cmd.args(iter.scan((), |_, str| match str.to_os_str() {
        Ok(str) => Some(str),
        Err(e) => {
            err = Err(e);
            None
        }
    }));
    err?;
    Ok(cmd)
}

fn into_git2_err<E: Display>(err: E) -> git2::Error {
    git2::Error::from_str(&err.to_string())
}
