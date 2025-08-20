use crate::{
    schemas::config::{Config, Host, Open, Repo, Tree, common::Common},
    state::resolve::Resolve,
};
use anyhow::{Context, anyhow};
use bstr::{BStr, ByteSlice};
use git2::{Repository, Worktree, build::RepoBuilder};
use serde::Serialize;
use std::{fmt::Display, fs, path::PathBuf, process::Command};

#[derive(Debug, Default, Serialize)]
pub struct ResolvedTask {
    pub host: Option<Host>,
    pub repo: Option<Repo>,
    pub tree: Option<Tree>,
    pub open: Option<Open>,

    pub new_host: Option<PathBuf>,
    pub new_repo: Option<PathBuf>,
    pub new_tree: Option<PathBuf>,
    pub new_open: Option<PathBuf>,
}

impl ResolvedTask {
    pub fn run(&self) -> anyhow::Result<()> {
        let repo = if self.repo.is_some() {
            self.repo()?
        } else {
            return Ok(());
        };

        if self.tree.is_some() {
            self.tree(&repo)?;
        }

        if self.open.is_some() {
            self.open()?;
        }

        Ok(())
    }

    pub fn repo(&self) -> anyhow::Result<git2::Repository> {
        let Some(
            repo @ Repo {
                path: Some(path), ..
            },
        ) = &self.repo
        else {
            return Err(anyhow!("repo needs path to be set"));
        };
        let path = path.as_bstr().to_path()?;

        // TODO: allow force `--bare`. Maybe just warn when not as expected.
        let repository = match Repository::open_ext(
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

                let url = match &self.host {
                    Some(Host {
                        host: Some(url), ..
                    }) => url.as_bstr().to_str()?,
                    _ => return Err(anyhow!("host is missing required fields")),
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
        }?;

        if let Some(new_repo) = &self.new_repo
            && let Some(name) = &repo.common().name
        {
            let name = name.as_bstr().to_str()?;

            let mut doc = toml_edit::ser::to_document(&Config::from_file(new_repo)?)?;
            let repo = toml_edit::ser::to_document(repo)?;
            doc["repo"][name] = repo.into_item();

            std::fs::write(new_repo, doc.to_string())?;
        }

        Ok(repository)
    }

    pub fn tree(&self, repository: &git2::Repository) -> anyhow::Result<Worktree> {
        let Some(
            tree @ Tree {
                path: Some(path),
                common: Common {
                    name: Some(name), ..
                },
                ..
            },
        ) = &self.tree
        else {
            return Err(anyhow!("name and path of the tree must be defined"));
        };

        let name = name.as_bstr().to_str()?;
        let path = path.as_bstr().to_path()?;

        let worktree = if path.exists() {
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
        }?;

        if let Some(new_tree) = &self.new_tree {
            let mut doc = toml_edit::ser::to_document(&Config::from_file(new_tree)?)?;
            let tree = toml_edit::ser::to_document(tree)?;
            doc["tree"][name] = tree.into_item();

            std::fs::write(new_tree, doc.to_string())?;
        }

        Ok(worktree)
    }

    pub fn open(&self) -> anyhow::Result<()> {
        let Some(open) = &self.open else {
            return Err(anyhow!("missing open"));
        };

        let path = {
            match (&open.directory, &self.tree, &self.repo) {
                (Some(path), _, _)
                | (
                    _,
                    Some(Tree {
                        path: Some(path), ..
                    }),
                    _,
                )
                | (
                    _,
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

        if let Some(new_open) = &self.new_open
            && let Some(name) = &open.common().name
        {
            let name = name.as_bstr().to_str()?;

            let mut doc = toml_edit::ser::to_document(&Config::from_file(new_open)?)?;
            let open = toml_edit::ser::to_document(open)?;
            doc["open"][name] = open.into_item();

            std::fs::write(new_open, doc.to_string())?;
        }

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
