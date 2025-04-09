use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Completions { shell: Shell },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.subcommands {
            Commands::Completions { shell } => {
                clap_complete::generate(
                    shell,
                    &mut Cli::command(),
                    env!("CARGO_PKG_NAME"),
                    &mut io::stdout(),
                );
                Ok(())
            }
        }
    }
}
