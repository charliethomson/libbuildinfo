use serde::{Deserialize, Serialize};

use crate::extractor::{ExtractError, Extractor, agent::Agent, git::GitInfo, package::Package};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub package: Package,
    pub agent: Agent,
    pub git: GitInfo,
}

impl BuildInfo {
    pub fn extract() -> Result<Self, ExtractError> {
        let package = Package::extract()?;
        let agent = Agent::extract()?;
        let git = GitInfo::extract()?;
        Ok(BuildInfo {
            package,
            agent,
            git,
        })
    }
}
