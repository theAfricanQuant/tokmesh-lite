use tokmesh_lite::{data::csv::validate_data, load_manifest, product::validate::validate_product};

#[test]
fn valid_example_manifest_and_data_pass() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = root.join("examples/copper-mines/product.yaml");
    let data_path = root.join("examples/copper-mines/data.csv");
    let manifest = load_manifest(&manifest_path).expect("example manifest should parse");

    assert!(validate_product(&manifest).valid);
    assert!(
        validate_data(&manifest, &data_path)
            .expect("example data should be readable")
            .valid
    );
}

#[test]
fn invalid_example_reports_stable_issue_codes() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/invalid-products/duplicate-columns.yaml");
    let manifest = load_manifest(&path).expect("invalid contract should still parse");
    let report = validate_product(&manifest);
    let codes: Vec<&str> = report
        .issues
        .iter()
        .map(|issue| issue.code.as_str())
        .collect();

    assert!(!report.valid);
    assert!(codes.contains(&"product.version.invalid"));
    assert!(codes.contains(&"data.column.name.duplicate"));
    assert!(codes.contains(&"quality.column.unknown"));
}

#[test]
fn data_validation_rejects_value_outside_accepted_values() {
    let directory = tempfile::tempdir().expect("temporary directory should be created");
    let manifest_path = directory.path().join("product.yaml");
    let data_path = directory.path().join("data.csv");

    std::fs::write(
        &manifest_path,
        r"
id: tokmesh.ng-lithium-sites
name: Nigeria Lithium Sites
version: 1.0.0
owner:
  country: Nigeria
  organization: TokMesh Learning Laboratory
data:
  format: csv
  schema:
    - name: site_id
      type: string
      required: true
    - name: status
      type: string
      required: true
quality:
  - column: status
    rule: accepted_values
    values:
      - operating
      - suspended
      - closed
sovereignty:
  classification: national
  allowed_countries:
    - Nigeria
",
    )
    .expect("manifest fixture should be written");
    std::fs::write(
        &data_path,
        "site_id,status\nNG-LI-000001,operating\nNG-LI-000002,unknown\n",
    )
    .expect("CSV fixture should be written");

    let manifest = load_manifest(&manifest_path).expect("manifest should parse");
    let report = validate_data(&manifest, &data_path).expect("CSV should be readable");

    assert_eq!(report.issues.len(), 1);
    assert_eq!(report.issues[0].code, "quality.accepted_values.rejected");
    assert_eq!(report.issues[0].location, "row[3].status");
}

#[test]
fn product_validation_rejects_empty_accepted_values() {
    let directory = tempfile::tempdir().expect("temporary directory should be created");

    let manifest_path = directory.path().join("product.yaml");

    std::fs::write(
        &manifest_path,
        r"
  id: tokmesh.ng-lithium-sites
  name: Nigeria Lithium Sites
  version: 1.0.0
  owner:
    country: Nigeria
    organization: TokMesh Learning Laboratory
  data:
    format: csv
    schema:
      - name: site_id
        type: string
        required: true
      - name: status
        type: string
        required: true
  quality:
    - column: status
      rule: accepted_values
      values: []
  sovereignty:
    classification: national
    allowed_countries:
      - Nigeria
  ",
    )
    .expect("manifest fixture should be written");

    let manifest = load_manifest(&manifest_path).expect("manifest should parse");

    let report = validate_product(&manifest);

    assert_eq!(report.issues.len(), 1);
    assert_eq!(report.issues[0].code, "quality.accepted_values.empty");
    assert_eq!(report.issues[0].location, "quality[0].values");
}

#[test]
fn nigeria_lithium_invalid_example_rejects_unknown_jurisdiction() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = root.join("examples/nigeria-lithium/product.yaml");
    let data_path = root.join("examples/nigeria-lithium/invalid.csv");

    let manifest = load_manifest(&manifest_path).expect("manifest should parse");

    let report = validate_data(&manifest, &data_path).expect("CSV should be readable");

    assert!(report.issues.iter().any(|issue| {
        issue.code == "quality.accepted_values.rejected" && issue.location == "row[4].jurisdiction"
    }));
}

#[test]
fn nigeria_lithium_invalid_example_rejects_grade_outside_range() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = root.join("examples/nigeria-lithium/product.yaml");
    let data_path = root.join("examples/nigeria-lithium/invalid.csv");

    let manifest = load_manifest(&manifest_path).expect("manifest should parse");

    let report = validate_data(&manifest, &data_path).expect("CSV should be readable");

    assert!(report.issues.iter().any(|issue| {
        issue.code == "quality.range.outside" && issue.location == "row[5].li2o_grade_percent"
    }));
}

#[test]
fn nigeria_lithium_invalid_example_rejects_missing_site_name() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = root.join("examples/nigeria-lithium/product.yaml");
    let data_path = root.join("examples/nigeria-lithium/invalid.csv");

    let manifest = load_manifest(&manifest_path).expect("manifest should parse");

    let report = validate_data(&manifest, &data_path).expect("CSV should be readable");

    assert!(report.issues.iter().any(|issue| {
        issue.code == "data.value.required" && issue.location == "row[6].site_name"
    }));
}
