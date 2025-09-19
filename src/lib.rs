mod build_info;
mod error;
mod extractor;

use std::{io::Write, path::PathBuf};

pub use build_info::BuildInfo;
pub use error::{BuildInfoError, BuildInfoResult};

pub fn emit() -> BuildInfoResult<()> {
    let bi = BuildInfo::extract().map_err(|e| BuildInfoError::Extract { inner_error: e })?;
    let content = rmp_serde::to_vec(&bi).map_err(|e| BuildInfoError::Serialize {
        inner_error: e.into(),
    })?;

    let mut path = PathBuf::from(
        std::env::var("OUT_DIR").map_err(|e| BuildInfoError::EnvVar {
            var: "OUT_DIR".into(),
            inner_error: e.into(),
        })?,
    );
    path.push("buildinfo.bin");

    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .map_err(|e| BuildInfoError::Open {
            path: path.clone(),
            inner_error: e.into(),
        })?;

    f.write_all(&content).map_err(|e| BuildInfoError::Write {
        path: path.clone(),
        inner_error: e.into(),
    })?;

    Ok(())
}

pub fn __internal_load(s: &[u8]) -> BuildInfoResult<BuildInfo> {
    rmp_serde::from_slice(s).map_err(|e| BuildInfoError::Load {
        inner_error: e.into(),
    })
}

#[macro_export]
macro_rules! load_build_info {
    () => {{ $crate::__internal_load(include_bytes!(concat!(env!("OUT_DIR"), "/buildinfo.bin"))) }};
}
