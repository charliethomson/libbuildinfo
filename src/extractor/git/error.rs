use liberror::AnyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, valuable::Valuable)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "$type",
    content = "context"
)]
pub enum GitError {
    #[serde(rename = "dev.thmsn.build_info.extract.git.error.discover")]
    #[error("Failed to discover git repository: {inner_error}")]
    Discover { inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.extract.git.error.head")]
    #[error("Failed to resolve HEAD: {inner_error}")]
    Head { inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.extract.git.error.peel")]
    #[error("Failed to peel reference to commit: {inner_error}")]
    Peel { inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.extract.git.error.decode")]
    #[error("Failed to decode object: {inner_error}")]
    Decode { inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.extract.git.error.status")]
    #[error("Failed to check repository status: {inner_error}")]
    Status { inner_error: AnyError },
}
