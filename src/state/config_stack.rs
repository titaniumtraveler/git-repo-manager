use crate::schemas::config::{Config, ConfigSource};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, btree_map::Entry};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigStack {
    pub store: BTreeMap<ConfigSource, Config>,
    pub stack: Vec<ConfigSource>,
}

impl ConfigStack {
    pub fn with_cursor(&self, cur: Cursor) -> anyhow::Result<(&ConfigSource, &Config)> {
        let Some(src) = self.stack.get(cur.pos) else {
            return Err(anyhow!(
                "cursor location `{pos}` isn't available in `{stack:?}`",
                pos = cur.pos,
                stack = self.stack,
            ));
        };

        if !self.store.contains_key(src) {
            Err(anyhow!(
                "config stack needs to be mutably accessed to read files"
            ))
        } else {
            self.store
                .get(src)
                .map(|config| (src, config))
                .ok_or_else(|| anyhow!("we just checked that the key exists"))
        }
    }

    pub fn with_cursor_mut(&mut self, cur: Cursor) -> anyhow::Result<(&ConfigSource, &mut Config)> {
        let Some(src) = self.stack.get(cur.pos) else {
            return Err(anyhow!(
                "cursor location `{pos}` isn't available in `{stack:?}`",
                pos = cur.pos,
                stack = self.stack,
            ));
        };

        if !self.store.contains_key(src) {
            let src_ref = src;
            let src = src.clone();
            let config = Config::from_file(match &src {
                ConfigSource::File(path) => Ok(path),
                _ => Err(anyhow!("config stack only allows reading from `File`")),
            }?)?;

            match self.store.entry(src) {
                Entry::Vacant(entry) => Ok((src_ref, entry.insert(config))),
                _ => unreachable!(),
            }
        } else {
            self.store
                .get_mut(src)
                .map(|config| (src, config))
                .ok_or_else(|| anyhow!("we just checked that the key exists"))
        }
    }

    pub fn cursors(&self) -> Cursors {
        Cursors {
            pos: self.stack.len().checked_sub(1).unwrap_or(usize::MAX),
        }
    }
}

pub struct Cursors {
    pos: usize,
}

pub struct Cursor {
    pub pos: usize,
}

impl Iterator for Cursors {
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
