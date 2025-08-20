use anyhow::anyhow;
use git_repo_manager::schemas::config::{Config, ConfigSource};
use std::collections::{BTreeMap, btree_map::Entry};

pub struct ConfigStack {
    cache: BTreeMap<ConfigSource, Config>,
    stack: Vec<ConfigSource>,
}

impl ConfigStack {
    pub fn with_cursor(&mut self, cur: Cursor) -> anyhow::Result<(&ConfigSource, &mut Config)> {
        let Some(src) = self.stack.get(cur.pos) else {
            return Err(anyhow!(
                "cursor location `{pos}` isn't available in `{stack:?}`",
                pos = cur.pos,
                stack = self.stack,
            ));
        };

        if !self.cache.contains_key(src) {
            let src_ref = src;
            let src = src.clone();
            let config = Config::from_file(match &src {
                ConfigSource::File(path) => Ok(path),
                _ => Err(anyhow!("config stack only allows reading from `File`")),
            }?)?;

            match self.cache.entry(src) {
                Entry::Vacant(entry) => Ok((src_ref, entry.insert(config))),
                _ => unreachable!(),
            }
        } else {
            self.cache
                .get_mut(src)
                .map(|config| (src, config))
                .ok_or_else(|| anyhow!("we just checked that the key exists"))
        }
    }
}

pub struct IntoIter {
    pos: usize,
}

pub struct Cursor {
    pub pos: usize,
}

impl IntoIterator for &ConfigStack {
    type IntoIter = IntoIter;
    type Item = Cursor;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            pos: self.stack.len().checked_sub(1).unwrap_or(usize::MAX),
        }
    }
}

impl Iterator for IntoIter {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < usize::MAX {
            let cur = Cursor { pos: self.pos };
            self.pos = self.pos.checked_sub(1).unwrap_or(usize::MAX);

            Some(cur)
        } else {
            None
        }
    }
}

fn main() {}
