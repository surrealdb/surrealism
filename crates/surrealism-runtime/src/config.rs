use anyhow::{Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurrealismConfig {
    #[serde(rename = "package")]
    pub meta: SurrealismMeta,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurrealismMeta {
    pub organisation: String,
    pub name: String,
    pub version: Version,
}

impl SurrealismConfig {
    pub fn parse(s: &str) -> Result<Self> {
        toml::from_str(s).with_context(|| "Failed to parse Surrealism config")
    }

    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).with_context(|| "Failed to serialize Surrealism config")
    }

    pub fn file_name(&self) -> String {
        format!(
            "{}-{}-{}.surli",
            self.meta.organisation, self.meta.name, self.meta.version
        )
    }
}
