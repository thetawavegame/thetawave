#[test]
#[cfg(all(not(target_arch = "wasm32"), feature = "cli"))]
fn test_cli_help() {
    // Run this as an integration test because we need the whole game binary to be built.
    assert_cmd::Command::cargo_bin("thetawave")
        .unwrap()
        .args(&["--help"])
        .unwrap();
}
