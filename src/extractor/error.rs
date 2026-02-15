use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::extractor::{agent::AgentError, git::GitError, package::PackageError};

#[derive(Debug, Clone, Serialize, Deserialize, Error, valuable::Valuable)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "$type",
    content = "context"
)]
pub enum ExtractError {
    #[serde(rename = "dev.thmsn.build_info.extract.error.package")]
    #[error("Failed to extract package information")]
    Package(#[from] PackageError),
    #[serde(rename = "dev.thmsn.build_info.extract.error.agent")]
    #[error("Failed to extract agent information")]
    Agent(#[from] AgentError),
    #[serde(rename = "dev.thmsn.build_info.extract.error.git")]
    #[error("Failed to extract git information")]
    Git(#[from] GitError),
}
