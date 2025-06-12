use crate::state::{AppState, AppStateType, Opts, Result, impl_default_opts};
use anyhow::Context;
use std::path::PathBuf;

#[derive(Default)]
pub struct ProjectDirs;

#[derive(Default)]
pub struct FromAppName;

impl ProjectDirs {
    pub fn from_app_name() -> FromAppName {
        FromAppName
    }
}

impl_default_opts!(ProjectDirs as FromAppName);

impl<InputStateType> Opts<ProjectDirs, InputStateType> for FromAppName
where
    InputStateType: AppStateType,
    InputStateType::AppName: AsRef<str>,
{
    type Ok = InputStateType::SetProjectDirs<directories_next::ProjectDirs>;
    type Err = InputStateType;

    fn run(self, state: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
        let Some(project_dirs) =
            directories_next::ProjectDirs::from_path(PathBuf::from(state.app_name.as_ref()))
        else {
            return None
                .context("failed to get project_dirs")
                .map_err(state.with_error());
        };

        Ok(state.set_project_dirs(project_dirs))
    }
}
