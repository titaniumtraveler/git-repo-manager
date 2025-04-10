use crate::{schemas::config::Config, utils};
use clap::Subcommand;

#[derive(Debug, Subcommand, Clone)]
pub enum Schema {
    Config,
}

impl Schema {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Schema::Config => utils::write_schema_to_stdout::<Config>(),
        }
    }
}
