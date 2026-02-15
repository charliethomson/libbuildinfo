use std::path::PathBuf;

use liberror::AnyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::extractor::ExtractError;

#[derive(Debug, Clone, Serialize, Deserialize, Error, valuable::Valuable)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "$type",
    content = "context"
)]
pub enum BuildInfoError {
    #[serde(rename = "dev.thmsn.build_info.error.extract")]
    #[error("Failed to extract build information: {inner_error}")]
    Extract { inner_error: ExtractError },
    #[serde(rename = "dev.thmsn.build_info.error.serialize")]
    #[error("Failed to serialize build information: {inner_error}")]
    Serialize { inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.error.env_var")]
    #[error("Failed to retrieve environment variable '{var}': {inner_error}")]
    EnvVar { var: String, inner_error: AnyError },
    #[serde(rename = "dev.thmsn.build_info.error.open")]
    #[error("Failed to open file: {inner_error}")]
    Open {
        path: PathBuf,
        inner_error: AnyError,
    },
    #[serde(rename = "dev.thmsn.build_info.error.write")]
    #[error("Failed to write to file: {inner_error}")]
    Write {
        path: PathBuf,
        inner_error: AnyError,
    },
    #[serde(rename = "dev.thmsn.build_info.error.load")]
    #[error("Failed to load build information: {inner_error}")]
    Load { inner_error: AnyError },
}
pub type BuildInfoResult<T> = Result<T, BuildInfoError>;
