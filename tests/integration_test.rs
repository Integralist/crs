/// help_output validates the crs binary produces expected help output.
#[test]
fn help_output() {
    let bin = "crs";
    let output = test_bin::get_test_bin(bin)
        .arg("--help")
        .output()
        .unwrap_or_else(|_| panic!("Failed to start {bin}"));

    let output = String::from_utf8_lossy(&output.stdout);
    assert!(output.contains("crs 1.0.0"));
}
