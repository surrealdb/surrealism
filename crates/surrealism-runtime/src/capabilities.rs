use surrealdb::dbs::capabilities::{Targets, FuncTarget, NetTarget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurrealismCapabilities {
    #[serde(default)]
    pub allow_scripting: bool,
    #[serde(default)]
    pub allow_arbitrary_queries: bool,
    #[serde(with = "super::targets_serde", default = "default_targets_func")]
    pub allow_functions: Targets<FuncTarget>,
    #[serde(with = "super::targets_serde", default = "default_targets_net")]
    pub allow_net: Targets<NetTarget>,
}

impl Default for SurrealismCapabilities {
    fn default() -> Self {
        Self {
            allow_scripting: false,
            allow_arbitrary_queries: false,
            allow_functions: Targets::None,
            allow_net: Targets::None,
        }
    }
}

fn default_targets_func() -> Targets<FuncTarget> {
    Targets::None
}

fn default_targets_net() -> Targets<NetTarget> {
    Targets::None
}