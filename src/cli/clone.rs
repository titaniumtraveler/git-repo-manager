use crate::{
    cli::GlobalArgs,
    schemas::{config::Config, repo_manifest::RepoManifest},
    utils::read_json_from_path,
};
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

        let (_, storage) = config
            .storage
            .iter()
            .find(|(_, storage)| storage.default.unwrap_or(false))
            .context("failed to retrieve repo storage")?;

        let mut storage_path = PathBuf::from(&storage.path);

        storage_path.push("manifest.json");
        let mut manifest: Option<RepoManifest> = read_json_from_path(&storage_path)?;
        storage_path.pop();

        todo!()
    }
}
