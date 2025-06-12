use crate::state::{AppState, AppStateType, Opts, Result, impl_default_opts};

#[derive(Default)]
pub struct AppName;

#[derive(Default)]
pub struct ProjectName;

pub struct Custom<T>(T);

impl AppName {
    pub const PROJECT_NAME: &str = "git-repo-manager";

    pub fn project_name() -> ProjectName {
        ProjectName
    }

    pub fn custom<T: AsRef<str>>(app_name: T) -> Custom<T> {
        Custom(app_name)
    }
}

impl_default_opts!(AppName as ProjectName);

impl<InputStateType: AppStateType> Opts<AppName, InputStateType> for ProjectName {
    type Ok = InputStateType::SetAppName<&'static str>;
    type Err = InputStateType;

    fn run(self, input: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
        Ok(input.set_app_name(AppName::PROJECT_NAME))
    }
}

impl<InputStateType: AppStateType, T: AsRef<str>> Opts<AppName, InputStateType> for Custom<T> {
    type Ok = InputStateType::SetAppName<T>;
    type Err = InputStateType;

    fn run(self, input: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
        Ok(input.set_app_name(self.0))
    }
}
