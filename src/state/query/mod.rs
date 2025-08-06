use crate::{state::query::tokens::QueryTokens, template::Template};
use bstr::ByteSlice;
use std::fmt::Debug;

pub mod repo;
pub mod tokens;
pub mod tree;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueryResult<'a, T> {
    pub key: &'a str,
    pub query: &'a [u8],
    pub val: T,
    pub partial: bool,
}

impl<T: Debug> Debug for QueryResult<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryResult")
            .field("key", &self.key)
            .field("query", &self.query.as_bstr())
            .field("val", &self.val)
            .field("partial", &self.partial)
            .finish()
    }
}

pub fn match_query<'a>(query: &'a [u8], val: &'a [u8]) -> Option<QueryResult<'a, ()>> {
    let mut tokens = QueryTokens::new(query);
    let mut template = Template::new(val);

    loop {
        match tokens.next() {
            Some(token) => 'inner: loop {
                use crate::template::Token::*;
                match template.next() {
                    Some(Str(str)) if token.is_last() => {
                        if str.starts_with(token.token) {
                            return Some(QueryResult {
                                query: token.query,
                                partial: token.token.len() < str.len(),
                                ..Default::default()
                            });
                        } else {
                            return None;
                        }
                    }
                    Some(Str(str)) => {
                        if token.eq(str) {
                            tokens.is_absolute = true;
                            break 'inner;
                        } else if tokens.is_absolute {
                            return None;
                        } else {
                            continue 'inner;
                        }
                    }
                    Some(Var(_)) => {
                        return Some(QueryResult {
                            query: token.query,
                            partial: false,
                            ..Default::default()
                        });
                    }
                    None => return None,
                }
            },
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_expected {
        ($expected:expr,$actual:expr) => {{
            let expected = $expected;
            let actual = $actual;
            if expected != actual {
                panic!(
                    "\
assertion `expected == actual` failed
expected: {expected:?}
  actual: {actual:?  }
"
                );
            }
        }};
    }

    macro_rules! assert_match {
        ($name:ident,$query:expr,$val:expr, $res:expr) => {
            #[test]
            fn $name() {
                assert_expected!($res, match_query($query, $val));
            }
        };
    }

    assert_match!(
        basic,
        b"proj",
        b"proj",
        Some(QueryResult {
            query: b"proj",
            partial: false,
            ..Default::default()
        })
    );

    assert_match!(
        partial,
        b"p",
        b"proj",
        Some(QueryResult {
            query: b"p",
            partial: true,
            ..Default::default()
        })
    );

    // TODO
    // assert_match!(
    //     var,
    //     b"repo/author/project",
    //     b"/repo/${repo.url}",
    //     Some(QueryResult {
    //         query: b"proj",
    //         partial: false,
    //         ..Default::default()
    //     })
    // );

    assert_match!(
        basic2,
        b"proj",
        b"proj",
        Some(QueryResult {
            query: b"proj",
            partial: false,
            ..Default::default()
        })
    );
}
