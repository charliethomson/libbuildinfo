use serde::{Deserialize, Serialize};

use crate::extractor::{ExtractError, Extractor, agent::Agent, package::Package};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub package: Package,
    pub agent: Agent,
}

impl BuildInfo {
    pub fn extract() -> Result<Self, ExtractError> {
        let package = Package::extract()?;
        let agent = Agent::extract()?;
        Ok(BuildInfo { package, agent })
    }
}
