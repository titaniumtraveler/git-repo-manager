use clap::{Args, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct GlobalArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,
}
