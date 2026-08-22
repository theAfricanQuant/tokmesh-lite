use std::process::Command;

#[test]
fn valid_product_command_exits_successfully() {
    let root = env!("CARGO_MANIFEST_DIR");
    let status = Command::new(env!("CARGO_BIN_EXE_tokmesh-lite"))
        .args([
            "product",
            "validate",
            &format!("{root}/examples/copper-mines/product.yaml"),
        ])
        .status()
        .expect("CLI should run");

    assert!(status.success());
}

#[test]
fn invalid_product_command_uses_validation_exit_code() {
    let root = env!("CARGO_MANIFEST_DIR");
    let status = Command::new(env!("CARGO_BIN_EXE_tokmesh-lite"))
        .args([
            "product",
            "validate",
            &format!("{root}/examples/invalid-products/duplicate-columns.yaml"),
        ])
        .status()
        .expect("CLI should run");

    assert_eq!(status.code(), Some(1));
}

#[test]
fn nigeria_lithium_valid_example_exits_successfully() {
    let root = env!("CARGO_MANIFEST_DIR");

    let status = Command::new(env!("CARGO_BIN_EXE_tokmesh-lite"))
        .args([
            "data",
            "validate",
            &format!("{root}/examples/nigeria-lithium/product.yaml"),
            &format!("{root}/examples/nigeria-lithium/valid.csv"),
        ])
        .status()
        .expect("CLI should run");

    assert!(status.success());
}

#[test]
fn nigeria_lithium_invalid_example_uses_validation_exit_code() {
    let root = env!("CARGO_MANIFEST_DIR");

    let status = Command::new(env!("CARGO_BIN_EXE_tokmesh-lite"))
        .args([
            "data",
            "validate",
            &format!("{root}/examples/nigeria-lithium/product.yaml"),
            &format!("{root}/examples/nigeria-lithium/invalid.csv"),
        ])
        .status()
        .expect("CLI should run");

    assert_eq!(status.code(), Some(1));
}
