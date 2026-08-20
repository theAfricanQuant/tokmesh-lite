use std::{collections::HashMap, path::Path};

use crate::{
    AppError,
    product::{manifest::ProductManifest, report::ValidationReport},
};

use super::quality::validate_row;

/// Validates a CSV file against a parsed product manifest.
///
/// # Errors
///
/// Returns [`AppError::Csv`] when the CSV cannot be opened or read.
pub fn validate_data(
    manifest: &ProductManifest,
    path: &Path,
) -> Result<ValidationReport, AppError> {
    let path_display = path.display().to_string();
    let mut reader = csv::Reader::from_path(path).map_err(|source| AppError::Csv {
        path: path_display.clone(),
        source,
    })?;
    let headers = reader
        .headers()
        .map_err(|source| AppError::Csv {
            path: path_display.clone(),
            source,
        })?
        .clone();

    let indexes: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(index, name)| (name, index))
        .collect();
    let mut report = ValidationReport::new();

    for column in &manifest.data.schema {
        if !indexes.contains_key(column.name.as_str()) {
            report.error(
                "data.column.missing",
                format!("data.header.{}", column.name),
                format!("declared column '{}' is missing from the CSV", column.name),
            );
        }
    }

    for header in &headers {
        if !manifest
            .data
            .schema
            .iter()
            .any(|column| column.name == header)
        {
            report.error(
                "data.column.undeclared",
                format!("data.header.{header}"),
                format!("CSV column '{header}' is not declared in the manifest"),
            );
        }
    }

    let mut unique_values = HashMap::new();
    for (index, result) in reader.records().enumerate() {
        let row = result.map_err(|source| AppError::Csv {
            path: path_display.clone(),
            source,
        })?;
        validate_row(
            manifest,
            index + 2,
            &row,
            &indexes,
            &mut unique_values,
            &mut report,
        );
    }

    Ok(report)
}
