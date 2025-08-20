use bstr::{BStr, ByteSlice};
use std::fmt::{self, Debug, Display, Write};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task<'a> {
    pub host: Option<&'a [u8]>,
    pub repo: Option<&'a [u8]>,
    pub tree: Option<&'a [u8]>,
    pub open: Option<&'a [u8]>,
}

impl Debug for Task<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const NONE: &Option<&BStr> = &None;

        let mut s = f.debug_struct("Task");

        let mut field = |name, value| {
            match value {
                None => s.field(name, NONE),
                Some(str) => s.field(name, &BStr::new(str)),
            };
        };

        field("host", self.host);
        field("repo", self.repo);
        field("tree", self.tree);
        field("open", self.open);

        s.finish()
    }
}

impl Display for Task<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = |sep, field| {
            if let Some(field) = field {
                f.write_char(sep)?;
                Display::fmt(BStr::new(field), f)?;
            }
            Ok(())
        };

        f('+', self.host)?;
        f(':', self.repo)?;
        f('@', self.tree)?;
        f('!', self.open)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    None,
    Host,
    Repo,
    Tree,
    Open,
}

impl TokenKind {
    const fn kind(byte: u8) -> Self {
        match byte {
            b'+' => TokenKind::Host,
            b':' => TokenKind::Repo,
            b'@' => TokenKind::Tree,
            b'!' => TokenKind::Open,
            _ => unreachable!(),
        }
    }

    const fn byteset(&self) -> &[u8] {
        match self {
            TokenKind::None => b"+:@!",
            TokenKind::Host => b":@!",
            TokenKind::Repo => b"@!",
            TokenKind::Tree => b"!",
            TokenKind::Open => b"",
        }
    }
}

impl<'a> From<&'a [u8]> for Task<'a> {
    fn from(mut str: &'a [u8]) -> Self {
        use TokenKind::*;

        let mut s = Self::default();
        let mut current_token = None;

        if let Some(first) = str.first()
            && TokenKind::None.byteset().contains(first)
        {
            str = &str[1..];
            current_token = TokenKind::kind(*first);
        }

        loop {
            let Some(idx) = str.find_byteset(current_token.byteset()) else {
                match current_token {
                    None if str.is_empty() => {}
                    None => s.repo = Some(str), //  <repo>...

                    Host => s.host = Some(str), // +<host>...
                    Repo => s.repo = Some(str), // :<repo>...
                    Tree => s.tree = Some(str), // @<tree>...
                    Open => s.open = Some(str), // !<open>...
                }
                break;
            };

            let token_pair = (current_token, TokenKind::kind(str[idx]));

            let mut step = |s: &mut Option<&'a [u8]>| {
                *s = Some(&str[..idx]);
                str = &str[(idx + 1)..];
                current_token = token_pair.1;
            };

            match token_pair {
                (None, Repo) => step(&mut s.host), //  <host>:<repo>...
                (None, Tree) => step(&mut s.repo), //  <repo>@<tree>...
                (None, Open) => step(&mut s.repo), //  <repo>!<open>...

                (Host, Repo) => step(&mut s.host), // +<host>:<repo>...
                (Host, Tree) => step(&mut s.host), // +<host>@<tree>...
                (Host, Open) => step(&mut s.host), // +<host>!<open>...

                (Repo, Tree) => step(&mut s.repo), // :<repo>@<tree>...
                (Repo, Open) => step(&mut s.repo), // :<repo>!<tree>...

                (Tree, Open) => step(&mut s.tree), // @<tree>!<open>...

                _ => unreachable!(),
            }
        }
        s
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

    const CHARS: &str = {
        match str::from_utf8(TokenKind::None.byteset()) {
            Ok(str) => str,
            Err(_) => panic!(),
        }
    };

    macro_rules! assert_task {
        ($name:ident,$str:expr,$task:expr) => {
            #[test]
            fn $name() {
                let task = $task;
                let string = $str;

                let display = $str;

                assert_expected! {
                    &task,
                    &Task::from(string.as_bytes())
                }

                assert_expected! {
                    string.trim_start_matches(CHARS),
                    display.trim_start_matches(CHARS)
                }
            }
        };
    }

    assert_task!(
        empty,
        "",
        Task {
            host: None,
            repo: None,
            tree: None,
            open: None
        }
    );

    assert_task!(
        symbols_1,
        "+:@!",
        Task {
            host: Some(b""),
            repo: Some(b""),
            tree: Some(b""),
            open: Some(b""),
        }
    );

    assert_task!(
        symbols_2,
        ":@!",
        Task {
            host: None,
            repo: Some(b""),
            tree: Some(b""),
            open: Some(b""),
        }
    );

    assert_task!(
        symbols_3,
        ":@",
        Task {
            host: None,
            repo: Some(b""),
            tree: Some(b""),
            open: None,
        }
    );

    assert_task!(
        symbols_4,
        ":",
        Task {
            host: None,
            repo: Some(b""),
            tree: None,
            open: None,
        }
    );

    assert_task!(
        empty_open,
        "!",
        Task {
            host: None,
            repo: None,
            tree: None,
            open: Some(b""),
        }
    );

    assert_task!(
        host_repo,
        "host:repo",
        Task {
            host: Some(b"host"),
            repo: Some(b"repo"),
            tree: None,
            open: None,
        }
    );

    assert_task!(
        repo_tree,
        "repo@tree",
        Task {
            host: None,
            repo: Some(b"repo"),
            tree: Some(b"tree"),
            open: None,
        }
    );

    assert_task!(
        repo_open,
        "repo!open",
        Task {
            host: None,
            repo: Some(b"repo"),
            tree: None,
            open: Some(b"open"),
        }
    );

    assert_task!(
        full,
        "+host:repo@tree!open",
        Task {
            host: Some(b"host"),
            repo: Some(b"repo"),
            tree: Some(b"tree"),
            open: Some(b"open"),
        }
    );
}
