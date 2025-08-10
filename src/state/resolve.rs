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

    pub fn resolve_task(
        &mut self,
        task: &Task,
        resolved_task: &mut ResolvedTask,
        config: &mut Config,
    ) -> anyhow::Result<()> {
        if let host @ None = &mut resolved_task.host {
            *host = match task.host {
                None => None,
                Some(b"") => {
                    println!("selected default");
                    config
                        .host
                        .iter()
                        .find_map(|(_, v)| match v {
                            Host { default: true, .. } => Some(v),
                            _ => None,
                        })
                        .cloned()
                }
                Some(host) => {
                    println!("selected `{host}`", host = <&BStr>::from(host));
                    config
                        .host
                        .iter()
                        .find_map(|(k, v)| {
                            if k.as_bytes() == host || v.alias.iter().any(|str| str.0 == host) {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .cloned()
                }
            };
        }

        if let repo @ None = &mut resolved_task.repo {
            *repo = match task.repo {
                None | Some(b"") => {
                    println!("selected default repo");
                    config
                        .repo
                        .iter()
                        .find_map(|(_, v)| if v.default.default { Some(v) } else { None })
                        .cloned()
                }
                Some(repo) => {
                    println!("selected `{repo}`", repo = <&BStr>::from(repo));
                    config
                        .repo
                        .iter()
                        .find_map(|(_, v)| {
                            if v.name
                                .as_ref()
                                .is_some_and(|name| name.0.as_slice() == repo)
                                || v.alias.iter().any(|str| str.as_bytes() == repo)
                            {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .cloned()
                }
            };
        }

        if let tree @ None = &mut resolved_task.tree {
            *tree = match task.tree {
                None => None,
                Some(b"") => {
                    println!("selected default");
                    config
                        .tree
                        .iter()
                        .find_map(|(_, v)| if v.default.default { Some(v) } else { None })
                        .cloned()
                }
                Some(tree) => {
                    println!("selected `{tree}`", tree = <&BStr>::from(tree));
                    config
                        .tree
                        .iter()
                        .find_map(|(_, v)| {
                            if v.name
                                .as_ref()
                                .is_some_and(|name| name.0.as_slice() == tree)
                                || v.alias.iter().any(|str| str.0 == tree)
                            {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .cloned()
                }
            };
        }

        if let open @ None = &mut resolved_task.open {
            *open = match task.open {
                None => None,
                Some(b"") => {
                    println!("selected default");
                    config
                        .open
                        .iter()
                        .find_map(|(_, v)| if v.default { Some(v) } else { None })
                        .cloned()
                }
                Some(open) => {
                    println!("selected `{open}`", open = <&BStr>::from(open));
                    config
                        .open
                        .iter()
                        .find_map(|(k, v)| if k.as_bytes() == open { Some(v) } else { None })
                        .cloned()
                }
            };
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
