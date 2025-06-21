use crate::{cli::GlobalArgs, schemas::config::Config};
use anyhow::Context;
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct CloneCommand {
    repo: PathBuf,
}

impl CloneCommand {
    pub fn run(self, global_args: GlobalArgs) -> anyhow::Result<()> {
        let mut config = Config::from_file(global_args.config_file())?;
        config.resolve_defaults();

        let (_, tree) = config
            .repo
            .iter()
            .find(|(_, tree)| tree.default.default && tree.path.is_some())
            .context("failed to retrieve repo storage")?;

        let mut _storage_path = tree
            .path
            .as_deref()
            .expect("a storage path that is `Some(_)`");

        // storage_path.push("manifest.json");
        // let _manifest: Option<RepoManifest> = read_json_from_path(&storage_path)?;
        // storage_path.pop();

        todo!()
    }
}
