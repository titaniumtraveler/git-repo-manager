use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "merge")]
pub struct Merge {
    #[serde(default)]
    pub only_if: OnlyIf,
    #[serde(default)]
    pub behavior: Behavior,
}

// `bool` would be kind of ambiguos about what `false` means and would have no good way to specify
// an explicit non-value. That's why this enum
#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "merge-only-if")]
pub enum OnlyIf {
    #[default]
    #[serde(rename = "")]
    None,
    Present,
    NotPresent,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "merge-behavior")]
pub enum Behavior {
    #[default]
    Merge,
    Replace,
}
