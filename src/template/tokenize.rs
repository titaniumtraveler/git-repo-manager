use crate::template::tokenize::Token::*;
use bstr::{BStr, ByteSlice};
use std::cmp::Ordering;

pub struct Tokenize<'a>(pub(super) &'a BStr);

#[derive(Debug)]
pub enum Token<'a, T: AsRef<[u8]> + ?Sized> {
    Str(&'a T),
    Var(&'a T),
}

impl<T: AsRef<[u8]> + ?Sized, Rhs: AsRef<[u8]> + ?Sized> PartialOrd<Token<'_, Rhs>>
    for Token<'_, T>
{
    fn partial_cmp(&self, other: &Token<'_, Rhs>) -> Option<Ordering> {
        match (self, other) {
            (Str(_), Var(_)) => Some(Ordering::Less),
            (Var(_), Str(_)) => Some(Ordering::Greater),

            (Str(s), Str(o)) => s.as_ref().partial_cmp(o.as_ref()),
            (Var(s), Var(o)) => s.as_ref().partial_cmp(o.as_ref()),
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> Ord for Token<'_, T> {
    fn cmp(&self, other: &Token<'_, T>) -> Ordering {
        match (self, other) {
            (Str(_), Var(_)) => Ordering::Less,
            (Var(_), Str(_)) => Ordering::Greater,

            (Str(s), Str(o)) => s.as_ref().cmp(o.as_ref()),
            (Var(s), Var(o)) => s.as_ref().cmp(o.as_ref()),
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> Eq for Token<'_, T> {}
impl<T: AsRef<[u8]> + ?Sized, Rhs: AsRef<[u8]> + ?Sized> PartialEq<Token<'_, Rhs>>
    for Token<'_, T>
{
    fn eq(&self, other: &Token<Rhs>) -> bool {
        match (self, other) {
            (Str(s), Str(o)) => s.as_ref().eq(o.as_ref()),
            (Var(s), Var(o)) => s.as_ref().eq(o.as_ref()),
            (Str(_), Var(_)) => false,
            (Var(_), Str(_)) => false,
        }
    }
}

impl<'a> Iterator for Tokenize<'a> {
    type Item = Token<'a, BStr>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if self.0.is_empty() {
            return None;
        }

        let mut str = self.0;
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
                            None => Some(Str(self.0)),
                            Some(idx) => {
                                self.0 = &str[(idx + 1)..];
                                Some(Var(&str[..idx]))
                            }
                        }
                    }
                    Some(_) => {
                        const SPECIAL: &[u8] = b" \t\n()<>[]|&;/";
                        let idx = str.find_byteset(SPECIAL).unwrap_or(str.len());

                        self.0 = &str[idx..];
                        Some(Var(&str[..idx]))
                    }
                }
            }
            Some(idx) => {
                let (str, trailing) = str.split_at(idx);
                self.0 = trailing.as_ref();

                Some(Str(str.as_ref()))
            }
        }
    }
}

#[test]
fn test_1() {
    use crate::template::{Template, tokenize::Token::*};

    let template = Template::new("$HOME/projects/${host/name}/${repo/path}/${repo/url}");
    assert_eq!(
        template.tokenize().collect::<Vec<Token<BStr>>>(),
        [
            Var("HOME"),
            Str("/projects/"),
            Var("host/name"),
            Str("/"),
            Var("repo/path"),
            Str("/"),
            Var("repo/url"),
        ]
    );
}
