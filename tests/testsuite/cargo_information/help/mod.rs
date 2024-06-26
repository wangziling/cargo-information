use cargo_test_macro::cargo_test;
use cargo_test_support::file;

use super::cargo_info_with_color;

#[cargo_test]
fn case() {
    cargo_info_with_color()
        .arg("--help")
        .arg("--registry=dummy-registry")
        .assert()
        .success()
        .stdout_eq_(file!["stdout.log"])
        .stderr_eq_(file!["stderr.log"]);
}
