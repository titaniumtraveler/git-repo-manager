use crate::{cli::GlobalArgs, schemas::config::Config};
use clap::Subcommand;
use std::io::{self, BufWriter};

#[derive(Debug, Subcommand, Clone)]
pub enum Info {
    Config,
    Paths,
}

impl Info {
    pub fn run(self, global_args: GlobalArgs) -> anyhow::Result<()> {
        match self {
            Info::Config => {
                let mut config = Config::read_from_path(global_args.config_file())?;
                config.resolve_defaults();
                serde_json::to_writer(BufWriter::new(io::stdout()), &config)?;
                println!();
                Ok(())
            }
            Info::Paths => {
                let paths = global_args.paths();
                serde_json::to_writer(BufWriter::new(io::stdout()), &paths)?;
                println!();
                Ok(())
            }
        }
    }
}
