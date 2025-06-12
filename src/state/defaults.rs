use crate::state::{AppState, AppStateType, Opts, Result, impl_default_opts};
use std::path::PathBuf;

#[derive(Default)]
pub struct Defaults;

#[derive(Default)]
pub struct NoDefaults;

#[derive(Default)]
pub struct DefaultValues {
    pub config_path: Option<PathBuf>,
}

impl Defaults {
    pub fn no_defaults() -> NoDefaults {
        NoDefaults
    }
}

impl_default_opts!(Defaults as NoDefaults);

impl<InputStateType: AppStateType> Opts<Defaults, InputStateType> for NoDefaults {
    type Ok = InputStateType::SetDefaults<DefaultValues>;
    type Err = InputStateType;

    fn run(self, input: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
        Ok(input.set_defaults(DefaultValues::default()))
    }
}

impl<InputStateType: AppStateType> Opts<Defaults, InputStateType> for DefaultValues {
    type Ok = InputStateType::SetDefaults<DefaultValues>;
    type Err = InputStateType;

    fn run(self, input: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
        Ok(input.set_defaults(self))
    }
}
