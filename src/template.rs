use crate::template::Token::*;
use bstr::ByteSlice;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

pub struct Template<'a> {
    pub bytes: &'a [u8],
}

impl<'a> Template<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl Display for Template<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let template = Template::new(self.bytes);
        for token in template {
            Debug::fmt(&token, f)?;
        }
        Ok(())
    }
}

pub enum Token<'a> {
    Str(&'a [u8]),
    Var(&'a [u8]),
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Str(str) => f.debug_tuple("Str").field(&str.as_bstr()).finish(),
            Var(var) => f.debug_tuple("Var").field(&var.as_bstr()).finish(),
        }
    }
}

impl PartialOrd<Token<'_>> for Token<'_> {
    fn partial_cmp(&self, other: &Token<'_>) -> Option<Ordering> {
        match (self, other) {
            (Str(_), Var(_)) => Some(Ordering::Less),
            (Var(_), Str(_)) => Some(Ordering::Greater),

            (Str(s), Str(o)) => s.partial_cmp(o),
            (Var(s), Var(o)) => s.partial_cmp(o),
        }
    }
}

impl Ord for Token<'_> {
    fn cmp(&self, other: &Token<'_>) -> Ordering {
        match (self, other) {
            (Str(_), Var(_)) => Ordering::Less,
            (Var(_), Str(_)) => Ordering::Greater,

            (Str(s), Str(o)) => s.cmp(o),
            (Var(s), Var(o)) => s.cmp(o),
        }
    }
}

impl Eq for Token<'_> {}
impl PartialEq<Token<'_>> for Token<'_> {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Str(s), Str(o)) => s.eq(o),
            (Var(s), Var(o)) => s.eq(o),
            (Str(_), Var(_)) => false,
            (Var(_), Str(_)) => false,
        }
    }
}

impl<'a> Iterator for Template<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if self.bytes.is_empty() {
            return None;
        }

        let mut str = self.bytes;
        match str.find("$") {
            None => Some(Str(str)),
            Some(0) => {
                str = &str[1..];
                match str.first() {
                    None => None,
                    Some(b'{') => {
                        str = &str[1..];
                        match str.find("}") {
                            // Broken variable
                            None => Some(Str(self.bytes)),
                            Some(idx) => {
                                self.bytes = &str[(idx + 1)..];
                                Some(Var(&str[..idx]))
                            }
                        }
                    }
                    Some(_) => {
                        const SPECIAL: &[u8] = b" \t\n()<>[]|&;/";
                        let idx = str.find_byteset(SPECIAL).unwrap_or(str.len());

                        self.bytes = &str[idx..];
                        Some(Var(&str[..idx]))
                    }
                }
            }
            Some(idx) => {
                let (str, trailing) = str.split_at(idx);
                self.bytes = trailing;

                Some(Str(str))
            }
        }
    }
}

#[test]
fn test_1() {
    use crate::template::{Template, Token::*};

    let template = Template::new(b"$HOME/projects/${host/name}/${repo/path}/${repo/url}");
    assert_eq!(
        template.collect::<Vec<Token>>(),
        [
            Var(b"HOME"),
            Str(b"/projects/"),
            Var(b"host/name"),
            Str(b"/"),
            Var(b"repo/path"),
            Str(b"/"),
            Var(b"repo/url"),
        ]
    );
}
