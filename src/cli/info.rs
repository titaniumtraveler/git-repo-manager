use crate::{cli::GlobalArgs, schemas::config::Config, utils::write_json_to_stdout};
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
    pub fn run(self, global_args: GlobalArgs) -> anyhow::Result<()> {
        let Self { pretty, info } = self;
        match info {
            InfoKind::Config => {
                let mut config = Config::from_file(global_args.config_file())?;
                config.resolve_defaults();
                write_json_to_stdout(&config, pretty)?;
                Ok(())
            }

            InfoKind::Schema { schema } => schema.run(pretty),
            InfoKind::Paths => {
                let paths = global_args.paths();
                write_json_to_stdout(&paths, pretty)?;
                Ok(())
            }
        }
    }
}
