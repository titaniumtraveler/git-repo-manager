use anyhow::Context;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    io::{self, BufWriter, Write},
    path::Path,
};

pub(crate) fn read_json_from_path<T: DeserializeOwned>(path: &Path) -> anyhow::Result<Option<T>> {
    let str = match std::fs::read_to_string(path) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        o => o,
    }
    .with_context(|| format!("{}", path.display()))?;

    let config: T = serde_json::from_str(&str)?;
    Ok(Some(config))
}

pub(crate) fn read_toml_from_path<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let str = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let config = toml::from_str(&str)?;
    Ok(config)
}

pub(crate) fn write_json_to_stdout<T: Serialize>(val: &T, pretty: bool) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(io::stdout().lock());
    if !pretty {
        serde_json::to_writer(&mut writer, val)?;
    } else {
        serde_json::to_writer_pretty(&mut writer, val)?;
    }
    writeln!(writer)?;
    Ok(())
}

pub(crate) fn write_schema_to_stdout<T: JsonSchema>(pretty: bool) -> anyhow::Result<()> {
    write_json_to_stdout(
        &SchemaGenerator::default().into_root_schema_for::<T>(),
        pretty,
    )
}
