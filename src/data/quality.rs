use std::collections::{HashMap, HashSet};

use crate::product::{
    manifest::{DataType, ProductManifest, RuleType},
    report::ValidationReport,
};

pub(crate) fn validate_row(
    manifest: &ProductManifest,
    row_number: usize,
    row: &csv::StringRecord,
    indexes: &HashMap<&str, usize>,
    unique_values: &mut HashMap<String, HashSet<String>>,
    report: &mut ValidationReport,
) {
    for column in &manifest.data.schema {
        let Some(index) = indexes.get(column.name.as_str()).copied() else {
            continue;
        };
        let value = row.get(index).unwrap_or_default().trim();
        let location = format!("row[{row_number}].{}", column.name);

        if value.is_empty() {
            if column.required {
                report.error("data.value.required", location, "required value is empty");
            }
            continue;
        }

        if !value_matches_type(value, column.data_type) {
            report.error(
                "data.value.type",
                &location,
                format!("value '{value}' is not a valid {:?}", column.data_type),
            );
            continue;
        }

        for rule in manifest
            .quality
            .iter()
            .filter(|rule| rule.column == column.name)
        {
            match rule.rule {
                RuleType::Unique => {
                    let values = unique_values.entry(column.name.clone()).or_default();
                    if !values.insert(value.to_owned()) {
                        report.error(
                            "quality.unique.duplicate",
                            &location,
                            format!("value '{value}' is duplicated"),
                        );
                    }
                }
                RuleType::Range => {
                    validate_range(value, rule.minimum, rule.maximum, &location, report);
                }
            }
        }
    }
}

fn value_matches_type(value: &str, data_type: DataType) -> bool {
    match data_type {
        DataType::String => true,
        DataType::Integer => value.parse::<i64>().is_ok(),
        DataType::Decimal => value.parse::<f64>().is_ok(),
        DataType::Boolean => matches!(value.to_ascii_lowercase().as_str(), "true" | "false"),
        DataType::Date => valid_iso_date(value),
    }
}

fn valid_iso_date(value: &str) -> bool {
    let mut parts = value.split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    year.len() == 4
        && month.len() == 2
        && day.len() == 2
        && year.parse::<u16>().is_ok()
        && month
            .parse::<u8>()
            .is_ok_and(|value| (1..=12).contains(&value))
        && day
            .parse::<u8>()
            .is_ok_and(|value| (1..=31).contains(&value))
}

fn validate_range(
    value: &str,
    minimum: Option<f64>,
    maximum: Option<f64>,
    location: &str,
    report: &mut ValidationReport,
) {
    let Ok(number) = value.parse::<f64>() else {
        return;
    };
    if minimum.is_some_and(|minimum| number < minimum)
        || maximum.is_some_and(|maximum| number > maximum)
    {
        report.error(
            "quality.range.outside",
            location,
            format!("value {number} is outside the declared range"),
        );
    }
}
