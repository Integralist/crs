/// help_output validates the crs binary produces expected help output.
#[test]
fn help_output() {
    let bin = "crs";
    let output = test_bin::get_test_bin(bin)
        .arg("--help")
        .output()
        .unwrap_or_else(|_| panic!("Failed to start {bin}"));

    let output = String::from_utf8_lossy(&output.stdout);
    println!("{:?}", output);
    assert!(output.contains("A CLI that can make a HTTP request, then sort, filter and display the HTTP response headers."));
}

/// version_output validates the crs binary produces expected version output.
#[test]
fn version_output() {
    let bin = "crs";
    let output = test_bin::get_test_bin(bin)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Failed to start {bin}"));

    let output = String::from_utf8_lossy(&output.stdout);
    println!("{:?}", output);
    assert!(output.contains("crs 1.1.0"));
}
