use crate::schemas::config::Config;
use clap::Subcommand;
use schemars::SchemaGenerator;
use std::io::{self, BufWriter};

#[derive(Debug, Subcommand, Clone)]
pub enum Schema {
    Config,
}

impl Schema {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Schema::Config => {
                let schema = SchemaGenerator::default().into_root_schema_for::<Config>();
                serde_json::to_writer(BufWriter::new(io::stdout()), &schema)?;
                println!();
                Ok(())
            }
        }
    }
}
