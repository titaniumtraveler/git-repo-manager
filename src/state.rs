use crate::schemas::{config::Config, repo_manifest::RepoManifest};
use std::{collections::BTreeMap, marker::PhantomData, path::PathBuf};

pub use self::{
    app_name::AppName,
    defaults::Defaults,
    error::{Error as AppStateError, Error, Result as AppStateResult, Result},
    project_dirs::ProjectDirs,
};

pub mod app_name;
pub mod defaults;
pub mod project_dirs;

mod error;

pub struct AppState<StateType: AppStateType = Type> {
    app_name: StateType::AppName,
    project_dirs: StateType::ProjectDirs,
    defaults: StateType::Defaults,

    storages: StateType::Storages,
    checkouts: StateType::Checkouts,
}

pub trait AppStateType {
    type AppName;
    type ProjectDirs;
    type Defaults;

    type Storages;
    type Checkouts;

    type SetAppName<T>: AppStateType<
        AppName = T,
        ProjectDirs = Self::ProjectDirs,
        Defaults = Self::Defaults,
        Storages = Self::Storages,
        Checkouts = Self::Checkouts,
    >;
    type SetProjectDirs<T>: AppStateType<
        AppName = Self::AppName,
        ProjectDirs = T,
        Defaults = Self::Defaults,
        Storages = Self::Storages,
        Checkouts = Self::Checkouts,
    >;
    type SetDefaults<T>: AppStateType<
        AppName = Self::AppName,
        ProjectDirs = Self::ProjectDirs,
        Defaults = T,
        Storages = Self::Storages,
        Checkouts = Self::Checkouts,
    >;
    type SetStorages<T>: AppStateType<
        AppName = Self::AppName,
        ProjectDirs = Self::ProjectDirs,
        Defaults = Self::Defaults,
        Storages = T,
        Checkouts = Self::Checkouts,
    >;
    type SetCheckouts<T>: AppStateType<
        AppName = Self::AppName,
        ProjectDirs = Self::ProjectDirs,
        Defaults = Self::Defaults,
        Storages = Self::Storages,
        Checkouts = T,
    >;
}

pub struct Type<
    AppName = Empty,
    ProjectDirs = Empty,
    Defaults = Empty,
    Storages = Empty,
    Checkouts = Empty,
> {
    app_name: PhantomData<AppName>,
    project_dirs: PhantomData<ProjectDirs>,
    defaults: PhantomData<Defaults>,

    storages: PhantomData<Storages>,
    checkouts: PhantomData<Checkouts>,
}
pub struct Empty;

impl<AppName, ProjectDirs, Defaults, Storages, Checkouts> AppStateType
    for Type<AppName, ProjectDirs, Defaults, Storages, Checkouts>
{
    type AppName = AppName;
    type ProjectDirs = ProjectDirs;
    type Defaults = Defaults;

    type Storages = Storages;
    type Checkouts = Checkouts;

    type SetAppName<T> = Type<T, ProjectDirs, Defaults, Storages, Checkouts>;
    type SetProjectDirs<T> = Type<AppName, T, Defaults, Storages, Checkouts>;
    type SetDefaults<T> = Type<AppName, ProjectDirs, T, Storages, Checkouts>;
    type SetStorages<T> = Type<AppName, ProjectDirs, Defaults, T, Checkouts>;
    type SetCheckouts<T> = Type<AppName, ProjectDirs, Defaults, Storages, T>;
}

pub trait Opts<Module, In: AppStateType> {
    type Ok: AppStateType;
    type Err: AppStateType;

    fn run(self, input: AppState<In>) -> Result<Self::Ok, Self::Err>;
}

macro_rules! impl_default_opts {
    (
      $module:ty as $default:ty
      $( where $($bounds:tt)* )?
    ) => {
        impl<InputStateType> Opts<$module, InputStateType> for $module
        where
            InputStateType: AppStateType,
            $default: Default + Opts<$module, InputStateType>,
            $( $( $bounds)* )?
        {
            type Ok = <$default as Opts<$module, InputStateType>>::Ok;
            type Err = <$default as Opts<$module, InputStateType>>::Err;

            fn run(self, input: AppState<InputStateType>) -> Result<Self::Ok, Self::Err> {
                <$default as Default>::default().run(input)
            }
        }
    };
}
use impl_default_opts;

impl<StateType: AppStateType> AppState<StateType> {
    pub fn add_module<M, T: Opts<M, StateType>>(self, module_opts: T) -> Result<T::Ok, T::Err> {
        module_opts.run(self)
    }

    pub fn with_error<E>(self) -> impl FnOnce(E) -> self::Error<StateType>
    where
        anyhow::Error: From<E>,
    {
        move |err| Error {
            state: self,
            err: err.into(),
        }
    }

    fn set_app_name<AppName>(self, app_name: AppName) -> AppState<StateType::SetAppName<AppName>> {
        let AppState {
            app_name: _,
            project_dirs,
            defaults,
            storages,
            checkouts,
        } = self;

        AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts,
        }
    }
    fn set_project_dirs<ProjectDirs>(
        self,
        project_dirs: ProjectDirs,
    ) -> AppState<StateType::SetProjectDirs<ProjectDirs>> {
        let AppState {
            app_name,
            project_dirs: _,
            defaults,
            storages,
            checkouts,
        } = self;

        AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts,
        }
    }
    fn set_defaults<Defaults>(
        self,
        defaults: Defaults,
    ) -> AppState<StateType::SetDefaults<Defaults>> {
        let AppState {
            app_name,
            project_dirs,
            defaults: _,
            storages,
            checkouts,
        } = self;

        AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts,
        }
    }
    fn set_storages<Storages>(
        self,
        storages: Storages,
    ) -> AppState<StateType::SetStorages<Storages>> {
        let AppState {
            app_name,
            project_dirs,
            defaults,
            storages: _,
            checkouts,
        } = self;

        AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts,
        }
    }
    fn set_checkouts<Checkouts>(
        self,
        checkouts: Checkouts,
    ) -> AppState<StateType::SetCheckouts<Checkouts>> {
        let AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts: _,
        } = self;

        AppState {
            app_name,
            project_dirs,
            defaults,
            storages,
            checkouts,
        }
    }
}

pub struct Paths {
    name: AppName,

    project_dirs: ProjectDirs,
    defaults: Defaults,
}

impl Defaults {
    pub fn get_config_path(&self, dirs: &ProjectDirs) -> PathBuf {
        todo!()
    }
}

impl Paths {
    pub fn load_config_from_path(self, config_path: PathBuf) -> anyhow::Result<ConfigData> {
        let config = Config::read_from_path(&config_path)?;

        Ok(ConfigData {
            paths: self,

            config,
            config_path: Some(config_path),
        })
    }

    pub fn load_config(self) -> anyhow::Result<ConfigData> {
        let config_path = self.defaults.get_config_path(&self.project_dirs);

        self.load_config_from_path(config_path)
    }

    pub fn set_config(self, config: Config) -> ConfigData {
        ConfigData {
            paths: self,
            config,
            config_path: None,
        }
    }

    pub fn set_config_with_path(self, config: Config, path: PathBuf) -> ConfigData {
        ConfigData {
            paths: self,
            config,
            config_path: Some(path),
        }
    }
}

pub struct ConfigData {
    paths: Paths,

    config: Config,
    config_path: Option<PathBuf>,
}

impl ConfigData {}

pub struct RepoStorageManifests {
    config: ConfigData,

    storages: BTreeMap<String, RepoManifest>,
}
