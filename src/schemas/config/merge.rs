use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
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
#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "merge-behavior")]
pub enum Behavior {
    #[default]
    Merge,
    Replace,
}

pub trait MergeEntry: Sized {
    fn merge_config(&self) -> &Merge;
    fn merge_config_mut(&mut self) -> &mut Merge;
    fn merge_entries(&mut self, other: Self);

    // TODO: rework this into something based on the rough idea
    // ```
    // struct Merge {
    //     merge_with_prev: _,
    //     merge_with_next: _,
    // }
    // ```
    fn merge(&mut self, other: Self) {
        match (&self.merge_config().only_if, &other.merge_config().only_if) {
            // always use freshest `OnlyIf::NotPresent`
            (OnlyIf::NotPresent, OnlyIf::NotPresent) => {
                *self = other;
                return;
            }
            // `other` should be ignored if there already is an entry
            (_, OnlyIf::NotPresent) => return,
            // `other` should be ignored if there isn't an entry and `OnlyIf::NotPresent` doesn't
            // count as one in this context and therefore gets ignored
            (OnlyIf::NotPresent, OnlyIf::Present) => {
                return;
            }

            (OnlyIf::NotPresent, OnlyIf::None) => {
                self.merge_config_mut().only_if = OnlyIf::None;
            }

            _ => {}
        }

        if let (Behavior::Merge, Behavior::Merge) = (
            &self.merge_config().behavior,
            &other.merge_config().behavior,
        ) {
            self.merge_entries(other);
        } else {
            *self = other
        }
    }
}
