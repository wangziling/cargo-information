use cargo_test_macro::cargo_test;
use cargo_test_support::file;

use super::{cargo_info, init_registry_without_token};

#[cargo_test]
fn case() {
    init_registry_without_token();
    cargo_test_support::registry::Package::new("my-package", "0.1.1")
        .feature("default", &["feature1", "feature2"])
        .feature("feature1", &[])
        .feature("feature2", &[])
        .publish();

    cargo_info()
        .arg("my-package")
        .arg("--registry=dummy-registry")
        .assert()
        .success()
        .stdout_eq_(file!["stdout.log"])
        .stderr_eq_(file!["stderr.log"]);
}
