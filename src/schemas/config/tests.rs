use super::*;
use anyhow::{Context, anyhow};
use schemars::{Schema, SchemaGenerator};
use serde::de::DeserializeOwned;
use std::io;

pub(crate) fn read_json_from_path<T: DeserializeOwned>(path: &Path) -> anyhow::Result<Option<T>> {
    let str = match std::fs::read_to_string(path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        o => o,
    }
    .with_context(|| format!("{}", path.display()))?;

    let config: T = serde_json::from_str(&str)?;
    Ok(Some(config))
}

fn default_config() -> PathBuf {
    PathBuf::from(format!(
        "{dir}/resources/default_config.toml",
        dir = env!("CARGO_MANIFEST_DIR")
    ))
}

#[test]
fn parse_default() -> anyhow::Result<()> {
    let path = default_config();
    let _config = Config::from_file(&path)
        .with_context(|| anyhow!("failed to parse schema at `{path}`", path = path.display()))?;
    Ok(())
}

#[test]
fn schema_present_and_correct() -> anyhow::Result<()> {
    let path = PathBuf::from(format!(
        "{dir}/resources/config_schema.json",
        dir = env!("CARGO_MANIFEST_DIR"),
    ));

    (|| {
        let expected = SchemaGenerator::default().into_root_schema_for::<Config>();
        let actual: Schema = read_json_from_path(&path)
            .with_context(|| path.to_string_lossy().into_owned())?
            .ok_or_else(|| anyhow!("missing schema at {path}", path = path.display()))?;

        if expected == actual {
            Ok(())
        } else {
            Err(anyhow!(
                "\
assertion `left == right` failed
  left: {expected:?}
 right: {actual:?}"
            ))
        }
    })()
    .with_context(|| {
        format!(
            "run `cargo run -- info --pretty schema config > '{path}'`",
            path = path.display()
        )
    })
}
