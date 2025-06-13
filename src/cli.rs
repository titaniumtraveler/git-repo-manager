use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io;

pub use self::{clone::CloneCommand, global_args::GlobalArgs, info::Info};

mod clone;
mod global_args;
mod info;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    #[clap(subcommand)]
    pub subcommands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clone(CloneCommand),
    Info(Info),
    Completions { shell: Shell },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.subcommands {
            Commands::Clone(clone) => clone.run(self.global_args),
            Commands::Info(info) => info.run(self.global_args),
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
