use crate::{
    cli::GlobalArgs,
    schemas::config::{Config, ConfigSource},
};
use anyhow::anyhow;
use directories_next::ProjectDirs;
use std::{
    collections::{BTreeMap, btree_map::Entry},
    path::PathBuf,
};

// pub mod query;
pub mod resolve;

#[derive(Debug, Default)]
pub struct State {
    pub config: Configs,
}

#[derive(Debug, Default)]
pub struct Configs {
    pub resolved_config: Config,
    pub sources: BTreeMap<ConfigSource, Config>,
}

impl State {
    pub fn new() -> Self {
        Self {
            config: Configs {
                resolved_config: Config::default(),
                sources: BTreeMap::new(),
            },
        }
    }

    pub fn init_defaults(&mut self, args: &GlobalArgs) -> anyhow::Result<()> {
        let dirs = ProjectDirs::from_path(PathBuf::from("git-repo-manager"))
            .ok_or_else(|| anyhow!("failed to create project dirs"))?;

        self.config.load_default(&dirs);

        self.config.load_from_file(
            args.config
                .as_ref()
                .cloned()
                .unwrap_or_else(|| [dirs.config_dir(), "config.toml".as_ref()].iter().collect()),
            true,
        )?;

        Ok(())
    }
}

impl Configs {
    fn insert(&mut self, source: ConfigSource, config: Config) -> bool {
        match self.sources.entry(source) {
            Entry::Vacant(vacant) => {
                vacant.insert(config.clone());
                true
            }
            // Already loaded
            Entry::Occupied(_) => false,
        }
    }

    pub fn load(&mut self, source: ConfigSource, mut config: Config, merge: bool) -> bool {
        if self.insert(source, config.clone()) {
            config.resolve_defaults();
            if merge {
                self.resolved_config.merge(config);
            }
            true
        } else {
            false
        }
    }

    pub fn load_raw(&mut self, source: ConfigSource, config: Config, merge: bool) -> bool {
        if self.insert(source, config.clone()) {
            if merge {
                self.resolved_config.merge(config);
            }
            true
        } else {
            false
        }
    }

    pub fn load_default(&mut self, dirs: &ProjectDirs) {
        self.load(ConfigSource::Builtin, Config::default_config(dirs), true);
    }

    pub fn load_from_file(
        &mut self,
        path: impl Into<PathBuf>,
        merge: bool,
    ) -> anyhow::Result<bool> {
        let path = path.into();
        if !self.sources.contains_key(&*path) {
            let config = Config::from_file(&path)?;
            let was_loaded = self.load_raw(ConfigSource::File(path), config, merge);
            debug_assert!(
                was_loaded,
                "config was not in `configs.sources`, but wasn't loaded"
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
