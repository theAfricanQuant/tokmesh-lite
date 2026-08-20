pub mod cli;
pub mod data;
pub mod product;

use std::{fs, path::Path};

use product::manifest::ProductManifest;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        source: std::io::Error,
    },
    #[error("could not parse YAML manifest {path}: {source}")]
    Manifest {
        path: String,
        source: serde_yaml::Error,
    },
    #[error("could not serialize output: {0}")]
    Output(#[from] serde_json::Error),
    #[error("could not read CSV data {path}: {source}")]
    Csv { path: String, source: csv::Error },
}

/// Loads and deserializes a product manifest from a YAML file.
///
/// # Errors
///
/// Returns [`AppError::Read`] when the file cannot be read and
/// [`AppError::Manifest`] when its YAML cannot be deserialized.
pub fn load_manifest(path: &Path) -> Result<ProductManifest, AppError> {
    let text = fs::read_to_string(path).map_err(|source| AppError::Read {
        path: path.display().to_string(),
        source,
    })?;

    serde_yaml::from_str(&text).map_err(|source| AppError::Manifest {
        path: path.display().to_string(),
        source,
    })
}
