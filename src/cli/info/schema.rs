use crate::{
    schemas::{config::Config, repo_manifest::RepoManifest},
    utils,
};
use clap::Subcommand;

#[derive(Debug, Subcommand, Clone)]
pub enum Schema {
    Config,
    RepoManifest,
}

impl Schema {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Schema::Config => utils::write_schema_to_stdout::<Config>(),
            Schema::RepoManifest => utils::write_schema_to_stdout::<RepoManifest>(),
        }
    }
}
