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
