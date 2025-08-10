use crate::{
    schemas::config::{Config, Host, Open, Repo, Tree},
    state::State,
    task::Task,
};
use anyhow::{Context, anyhow};
use bstr::{BStr, ByteSlice};
use git2::{Repository, build::RepoBuilder};
use std::{fs, process::Command};

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
        let repo = self
            .repo
            .as_ref()
            .ok_or_else(|| anyhow!("repo needs to be present"))?; // TODO: infer from working directory
        let repo_path = repo
            .path
            .as_ref()
            .ok_or_else(|| anyhow!("repo needs path to be set"))?
            .to_path()?;

        // TODO: allow force `--bare`. Maybe just warn when not as expected.
        let repository = {
            use git2::RepositoryOpenFlags as RepoOF;
            match Repository::open_ext(
                repo_path,
                RepoOF::NO_SEARCH | RepoOF::NO_DOTGIT,
                &[] as &[&std::ffi::OsStr],
            ) {
                Ok(repo) => repo,
                Err(err) if err.code() == git2::ErrorCode::NotFound => {
                    let mut builder = RepoBuilder::new();

                    let host = self.host.as_ref().ok_or_else(|| {
                        anyhow!("host needs to be set for the repo to be cloneable")
                    })?;

                    let host_url = host
                        .host
                        .as_ref()
                        .ok_or_else(|| {
                            anyhow!("host needs to be set for the repo to be cloneable")
                        })?
                        .0
                        .as_bstr()
                        .to_str()?;

                    builder
                        .fetch_options({
                            use git2::{Cred, RemoteCallbacks};

                            // Prepare callbacks.
                            let mut callbacks = RemoteCallbacks::new();
                            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                                Cred::ssh_key(
                                    username_from_url.unwrap(),
                                    None,
                                    host.key
                                        .as_ref()
                                        .ok_or_else(|| {
                                            git2::Error::from_str("key needs to be set to clone")
                                        })?
                                        .to_path()
                                        .map_err(|err| git2::Error::from_str(&err.to_string()))?,
                                    Some(
                                        build_cmd(
                                            host.pass_program.iter().map(|str| str.0.as_bstr()),
                                        )
                                        .map_err(|err| git2::Error::from_str(&err.to_string()))?
                                        .output()
                                        .map_err(|err| git2::Error::from_str(&err.to_string()))?
                                        .stdout
                                        .to_str()
                                        .map_err(|err| git2::Error::from_str(&err.to_string()))?
                                        .trim_end_matches('\n'),
                                    ),
                                )
                            });

                            // Prepare fetch options.
                            let mut fo = git2::FetchOptions::new();
                            fo.remote_callbacks(callbacks);

                            fo
                        })
                        .bare(true)
                        .clone(host_url, repo_path)
                        .with_context(|| anyhow!("failed to clone repo from {host_url}"))?
                }
                Err(err) => return Err(err.into()),
            }
        };

        let Some(tree) = self.tree.as_ref() else {
            if self.open.as_ref().is_some() {
                return Err(anyhow!("open was set, but no tree was given"));
            }
            return Ok(());
        };

        let tree_name = tree
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("tree needs name to be set"))?
            .to_str()?;

        let tree_path = tree
            .path
            .as_ref()
            .ok_or_else(|| anyhow!("tree needs path to be set"))?
            .to_path()?;

        if !tree_path.exists() {
            use git2::{
                BranchType::{Local, Remote},
                WorktreeAddOptions,
            };

            fs::create_dir_all(
                tree_path
                    .parent()
                    .ok_or_else(|| anyhow!("tree.path should have parent"))?,
            )?;

            // Borrow checker makes me sad here. If you need to, just keep temporaries
            // around as long as they are referenced...
            let reference;
            repository
                .worktree(
                    tree_name,
                    tree_path,
                    Some(&'opts: {
                        let mut opts = WorktreeAddOptions::new();

                        if let Ok(branch) = repository.find_branch(tree_name, Local) {
                            reference = branch.into_reference();
                            opts.reference(Some(&reference));
                            break 'opts opts;
                        }

                        if let Ok(branch) = repository.find_branch(tree_name, Remote) {
                            reference = branch.into_reference();
                            opts.reference(Some(&reference));
                            break 'opts opts;
                        }

                        reference = repository.head()?;
                        opts.reference(Some(&reference));

                        opts
                    }),
                )
                .with_context(|| anyhow!("failed to create worktree"))?;
        }

        let Some(open) = self.open.as_ref() else {
            return Ok(());
        };

        let mut cmd = build_cmd(open.command.iter().map(|str| str.0.as_bstr()))?;
        cmd.current_dir(tree_path);
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
