use super::*;
use crate::utils::read_json_from_path;
use anyhow::{Context, anyhow, ensure};
use schemars::SchemaGenerator;

fn default_config() -> PathBuf {
    PathBuf::from(format!(
        "{dir}/resources/default_config.toml",
        dir = env!("CARGO_MANIFEST_DIR")
    ))
}

#[test]
fn parse_default() -> anyhow::Result<()> {
    let path = default_config();
    let _config = Config::read_from_path(&path)
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
        let actual = read_json_from_path(&path)
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
            "run `cargo run -- info schema config > '{path}'`",
            path = path.display()
        )
    })
}
