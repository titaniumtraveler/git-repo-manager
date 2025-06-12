use crate::{cli::GlobalArgs, schemas::config::Config, utils::write_json_to_stdout};
use clap::Subcommand;

pub use self::schema::Schema;

mod schema;

#[derive(Debug, Subcommand, Clone)]
pub enum Info {
    Config,
    Paths,
    Schema {
        #[clap(subcommand)]
        schema: Schema,
    },
}

impl Info {
    pub fn run(self, global_args: GlobalArgs) -> anyhow::Result<()> {
        match self {
            Info::Config => {
                let mut config = Config::from_file(global_args.config_file())?;
                config.resolve_defaults();
                write_json_to_stdout(&config)?;
                Ok(())
            }
            Info::Schema { schema } => schema.run(),
            Info::Paths => {
                let paths = global_args.paths();
                write_json_to_stdout(&paths)?;
                Ok(())
            }
        }
    }
}
