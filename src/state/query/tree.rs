use crate::state::State;

pub struct QueryTree<'state> {
    pub(crate) state: &'state mut State,
}
