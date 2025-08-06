use bstr::ByteSlice;
use core::cmp::Ordering;
use std::{fmt::Debug, ops::Deref};

pub struct QueryTokens<'a> {
    pub is_absolute: bool,
    pub query: &'a [u8],
}

impl<'a> QueryTokens<'a> {
    pub fn new(mut query: &'a [u8]) -> Self {
        let is_absolute = if query.starts_with_str("/") {
            query = &query[1..];
            true
        } else {
            false
        };

        Self { is_absolute, query }
    }
}

impl<'a> Iterator for QueryTokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.query.is_empty() {
            None
        } else if let Some((token, tail)) = self.query.split_once_str("/") {
            let token = Token {
                token,
                query: self.query,
            };
            self.query = tail;
            Some(token)
        } else {
            let token = Token {
                token: self.query,
                query: self.query,
            };
            self.query = b"";
            Some(token)
        }
    }
}

pub struct Token<'a> {
    pub token: &'a [u8],
    pub query: &'a [u8],
}

impl Token<'_> {
    pub fn is_last(&self) -> bool {
        self.token.eq(self.query)
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("token", &self.token.as_bstr())
            .field("query", &self.query.as_bstr())
            .finish()
    }
}

impl Eq for Token<'_> {}
impl PartialEq for Token<'_> {
    fn eq(&self, other: &Token) -> bool {
        self.token.eq(other.token) && self.query.eq(other.query)
    }
}

impl Ord for Token<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token
            .cmp(other.token)
            .then_with(|| self.query.cmp(other.query))
    }
}

impl PartialOrd for Token<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<[u8]> for Token<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.token.eq(other)
    }
}

impl PartialOrd<[u8]> for Token<'_> {
    fn partial_cmp(&self, other: &[u8]) -> Option<Ordering> {
        self.token.partial_cmp(other)
    }
}

impl PartialEq<&[u8]> for Token<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.token.eq(*other)
    }
}

impl PartialOrd<&[u8]> for Token<'_> {
    fn partial_cmp(&self, other: &&[u8]) -> Option<Ordering> {
        self.token.partial_cmp(*other)
    }
}

impl Deref for Token<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute() {
        let str = b"/projects/author/proj";
        let tokens = QueryTokens::new(str);

        assert!(tokens.is_absolute);
        assert_eq!(
            tokens.collect::<Vec<_>>(),
            [
                Token {
                    token: b"projects",
                    query: b"projects/author/proj",
                },
                Token {
                    token: b"author",
                    query: b"author/proj",
                },
                Token {
                    token: b"proj",
                    query: b"proj",
                },
            ]
        );
    }
}
