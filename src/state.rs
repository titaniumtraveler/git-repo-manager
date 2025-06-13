use crate::{cli::GlobalArgs, schemas::config::Config};
use anyhow::anyhow;
use directories_next::ProjectDirs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct State {
    pub config: Config,
}

impl State {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn init_defaults(&mut self, args: &GlobalArgs) -> anyhow::Result<()> {
        let dirs = ProjectDirs::from_path(PathBuf::from("git-repo-manager"))
            .ok_or_else(|| anyhow!("failed to create project dirs"))?;

        self.load_config(Config::default_config(&dirs));

        if let Some(config) = &args.config {
            self.load_config(Config::from_file(config)?);
        } else {
            let path = PathBuf::from_iter(&[dirs.config_dir(), "config.toml".as_ref()]);

            self.load_config(Config::from_file(&path)?);
        }

        Ok(())
    }

    pub fn load_config(&mut self, mut config: Config) {
        config.resolve_defaults();
        self.config.merge(config);
    }
}
