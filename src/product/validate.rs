use std::collections::HashSet;

use semver::Version;

use super::{
    manifest::{DataType, ProductManifest, RuleType},
    report::ValidationReport,
};

#[must_use]
pub fn validate_product(manifest: &ProductManifest) -> ValidationReport {
    let mut report = ValidationReport::new();

    required(&mut report, "product.id.required", "id", &manifest.id);
    required(&mut report, "product.name.required", "name", &manifest.name);
    required(
        &mut report,
        "owner.country.required",
        "owner.country",
        &manifest.owner.country,
    );
    required(
        &mut report,
        "owner.organization.required",
        "owner.organization",
        &manifest.owner.organization,
    );
    required(
        &mut report,
        "sovereignty.classification.required",
        "sovereignty.classification",
        &manifest.sovereignty.classification,
    );

    if Version::parse(&manifest.version).is_err() {
        report.error(
            "product.version.invalid",
            "version",
            "version must follow semantic versioning, for example 1.0.0",
        );
    }

    if manifest.data.schema.is_empty() {
        report.error(
            "data.schema.empty",
            "data.schema",
            "at least one column is required",
        );
    }

    let mut names = HashSet::new();
    for (index, column) in manifest.data.schema.iter().enumerate() {
        let location = format!("data.schema[{index}].name");
        required(
            &mut report,
            "data.column.name.required",
            &location,
            &column.name,
        );
        if !column.name.trim().is_empty() && !names.insert(column.name.as_str()) {
            report.error(
                "data.column.name.duplicate",
                location,
                format!("column name '{}' appears more than once", column.name),
            );
        }
    }

    for (index, rule) in manifest.quality.iter().enumerate() {
        let Some(column) = manifest
            .data
            .schema
            .iter()
            .find(|column| column.name == rule.column)
        else {
            report.error(
                "quality.column.unknown",
                format!("quality[{index}].column"),
                format!("quality rule references unknown column '{}'", rule.column),
            );
            continue;
        };

        if rule.rule == RuleType::Range {
            if !matches!(column.data_type, DataType::Integer | DataType::Decimal) {
                report.error(
                    "quality.range.non_numeric",
                    format!("quality[{index}]"),
                    "range rules require an integer or decimal column",
                );
            }
            match (rule.minimum, rule.maximum) {
                (Some(minimum), Some(maximum)) if minimum > maximum => report.error(
                    "quality.range.reversed",
                    format!("quality[{index}]"),
                    "minimum cannot be greater than maximum",
                ),
                (None, None) => report.error(
                    "quality.range.unbounded",
                    format!("quality[{index}]"),
                    "a range rule needs a minimum, maximum, or both",
                ),
                _ => {}
            }
        }
    }

    report
}

fn required(report: &mut ValidationReport, code: &str, location: &str, value: &str) {
    if value.trim().is_empty() {
        report.error(code, location, "value is required");
    }
}
