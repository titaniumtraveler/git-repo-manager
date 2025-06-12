use crate::state::{AppState, AppStateType};
use std::fmt::{self, Debug, Display, Formatter};

pub type Result<T, U> = std::result::Result<AppState<T>, Error<U>>;

pub struct Error<StateType: AppStateType> {
    pub state: AppState<StateType>,
    pub err: anyhow::Error,
}

impl<Data> std::error::Error for Error<Data>
where
    Data: AppStateType,
    AppState<Data>: Debug,
{
}

impl<Data> Debug for Error<Data>
where
    Data: AppStateType,
    AppState<Data>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("data", &self.state)
            .field("err", &self.err)
            .finish()
    }
}

impl<Data> Display for Error<Data>
where
    Data: AppStateType,
    AppState<Data>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
