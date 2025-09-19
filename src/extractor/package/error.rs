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
pub enum PackageError {
    #[serde(rename = "ltd.pog.ttr.build_info.extract.package.error.metadata")]
    #[error("Failed to load package metadata: {inner_error}")]
    Metadata { inner_error: AnyError },
    #[serde(rename = "ltd.pog.ttr.build_info.extract.package.error.resolve")]
    #[error("Failed to resolve package")]
    Resolve,
    #[serde(rename = "ltd.pog.ttr.build_info.extract.package.error.root_package")]
    #[error("Failed to resolve root package with id {id}")]
    RootPackage { id: String },
    #[serde(rename = "ltd.pog.ttr.build_info.extract.package.error.root_package_id")]
    #[error("Failed to resolve root package ID")]
    RootPackageId,
    #[serde(rename = "ltd.pog.ttr.build_info.extract.package.error.serialize")]
    #[error("Serialization failed: {inner_error}")]
    Serialize { inner_error: AnyError },
}
