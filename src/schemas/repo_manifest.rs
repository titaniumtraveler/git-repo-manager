use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[schemars(rename = "repo-manifest.json")]
pub struct RepoManifest {
    pub repos: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "repo")]
pub struct Repo {
    #[serde(rename = "repo/name")]
    pub repo_name: PathBuf,
    #[serde(rename = "repo/url")]
    pub repo_url: String,
    #[serde(rename = "repo/url/hash")]
    pub repo_url_hash: HexStrHash,
    #[serde(rename = "repo/file-path")]
    pub repo_file_path: PathBuf,
}

/// u64 Hash of the repo URL as little endian hexadecimal string
#[derive(Debug, JsonSchema)]
#[schemars(with = String)]
#[schemars(rename = "hex-hash")]
pub struct HexStrHash(u64);

impl Serialize for HexStrHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            hex::serde::serialize(self.0.to_le_bytes(), serializer)
        } else {
            serializer.serialize_u64(self.0)
        }
    }
}

impl<'de> Deserialize<'de> for HexStrHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        hex::serde::deserialize(deserializer).map(|bytes| HexStrHash(u64::from_le_bytes(bytes)))
    }
}
