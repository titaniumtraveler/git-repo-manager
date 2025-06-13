use crate::{cli::GlobalArgs, state::State, utils::write_json_to_stdout};
use clap::{Args, Subcommand};

pub use self::schema::Schema;

mod schema;

#[derive(Debug, Args, Clone)]
pub struct Info {
    /// Pretty print output
    #[arg(short, long)]
    pretty: bool,

    #[clap(subcommand)]
    info: InfoKind,
}

#[derive(Debug, Subcommand, Clone)]
pub enum InfoKind {
    Config,
    Paths,
    Schema {
        #[clap(subcommand)]
        schema: Schema,
    },
}

impl Info {
    pub fn run(self, args: GlobalArgs) -> anyhow::Result<()> {
        let Self { pretty, info } = self;
        match info {
            InfoKind::Config => {
                let mut state = State::new();
                state.init_defaults(&args)?;
                write_json_to_stdout(&state.config, pretty)?;
                Ok(())
            }

            InfoKind::Schema { schema } => schema.run(pretty),
            InfoKind::Paths => {
                let paths = args.paths();
                write_json_to_stdout(&paths, pretty)?;
                Ok(())
            }
        }
    }
}
