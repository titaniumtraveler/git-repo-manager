use crate::cli::GlobalArgs;
use directories_next::ProjectDirs;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static PROJECT_PATHS: LazyLock<ProjectPaths> = LazyLock::new(ProjectPaths::new);

pub struct ProjectPaths {
    #[allow(dead_code)]
    dirs: ProjectDirs,
    config_file: PathBuf,
}

impl ProjectPaths {
    pub const PROJECT_NAME: &str = "git-repo-manager";
    pub fn new() -> Self {
        let dirs = ProjectDirs::from_path(Path::new(Self::PROJECT_NAME).to_path_buf())
            .expect("failed to initialize project paths");

        let config_file = PathBuf::from_iter([dirs.config_dir(), Path::new("config.toml")]);
        Self { dirs, config_file }
    }

    pub fn config_file(&self) -> &Path {
        &self.config_file
    }
}

impl Default for ProjectPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalArgs {
    pub fn config_file(&self) -> &Path {
        self.config.as_deref().unwrap_or(&PROJECT_PATHS.config_file)
    }

    pub fn paths(&self) -> Paths {
        Paths {
            config_file: self.config_file(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Paths<'a> {
    pub config_file: &'a Path,
}
