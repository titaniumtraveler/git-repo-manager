use crate::{
    cli::GlobalArgs,
    schemas::config::{Config, ConfigSource},
    state::config_stack::ConfigStack,
};
use anyhow::anyhow;
use directories_next::ProjectDirs;
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf};

// pub mod query;
pub mod config_stack;
pub mod resolve;

#[derive(Debug, Serialize)]
pub struct State {
    pub configs: ConfigStack,
    #[serde(skip)]
    pub dirs: ProjectDirs,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            configs: ConfigStack {
                store: BTreeMap::default(),
                stack: Vec::new(),
            },
            dirs: ProjectDirs::from_path(PathBuf::from("git-repo-manager"))
                .ok_or_else(|| anyhow!("failed to create project dirs"))?,
        })
    }

    pub fn init_defaults(&mut self, args: &GlobalArgs) -> anyhow::Result<()> {
        let path = args.config.as_ref().cloned().unwrap_or_else(|| {
            [self.dirs.config_dir(), "config.toml".as_ref()]
                .iter()
                .collect()
        });
        let config = Config::from_file(&path)?;

        self.configs.push(ConfigSource::File(path), config);

        Ok(())
    }
}

impl ConfigStack {
    fn push(&mut self, source: ConfigSource, config: Config) {
        self.stack.push(source.clone());
        self.store.insert(source, config);
    }
}
