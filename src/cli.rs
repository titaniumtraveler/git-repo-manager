use crate::{
    schemas::config::BString,
    state::{State, resolve::ResolvedTask},
    task::Task,
};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::{env, io};

pub use self::{clone::CloneCommand, global_args::GlobalArgs, info::Info};

mod clone;
mod global_args;
mod info;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub global_args: GlobalArgs,
    #[clap(subcommand)]
    pub subcommands: Option<Commands>,
    pub task: Option<BString>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clone(CloneCommand),
    Info(Info),
    Completions { shell: Shell },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        if let Some(subcommand) = self.subcommands {
            return match subcommand {
                Commands::Clone(clone) => clone.run(self.global_args),
                Commands::Info(info) => info.run(self.global_args),
                Commands::Completions { shell } => {
                    clap_complete::generate(
                        shell,
                        &mut Cli::command(),
                        std::env::args()
                            .nth(0)
                            .as_deref()
                            .unwrap_or(env!("CARGO_PKG_NAME")),
                        &mut io::stdout(),
                    );
                    Ok(())
                }
            };
        }

        let Some(task) = self.task else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::DisplayHelp,
                    "needs either `subcommand` or task to be set",
                )
                .exit()
        };
        let task = Task::from(&*task.0);
        eprintln!("task: {task:?}");

        let mut state = State::new()?;
        state.init_defaults(&self.global_args)?;

        let mut resolved_task = ResolvedTask::default();
        state.resolve(task, &mut resolved_task)?;
        println!("{}", serde_json::to_string_pretty(&resolved_task)?);
        resolved_task.run()
    }
}
