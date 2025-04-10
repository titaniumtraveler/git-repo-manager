use crate::cli::GlobalArgs;
use clap::Subcommand;
use std::io::{self, BufWriter};

#[derive(Debug, Subcommand, Clone)]
pub enum Info {
    Paths,
}

impl Info {
    pub fn run(self, global_args: GlobalArgs) -> anyhow::Result<()> {
        match self {
            Info::Paths => {
                let paths = global_args.paths();
                serde_json::to_writer(BufWriter::new(io::stdout()), &paths)?;
                println!();
                Ok(())
            }
        }
    }
}
