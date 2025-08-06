use crate::{
    schemas::config::Repo,
    state::{State, query::QueryResult},
};

pub struct QueryRepo<'state> {
    pub(crate) state: &'state mut State,
}

impl QueryRepo<'_> {
    pub fn search_by_key<'a>(
        &'a self,
        query: &'a [u8],
    ) -> impl Iterator<Item = QueryResult<'a, &'a Repo>> {
        self.state
            .config
            .resolved_config
            .repo
            .iter()
            .filter(|_| true)
            .map(|(key, repo)| QueryResult {
                key,
                query,
                val: repo,
                partial: false,
            })
    }

    // pub fn search_by_name(&self, name: &[u8]) -> impl Iterator<Item = &config::Repo> {
    //     self.state
    //         .config
    //         .resolved_config
    //         .repo
    //         .iter()
    //         .find(|_|true).map(|)
    // }
}
