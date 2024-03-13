use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("yolors")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}



#[test]
fn success_with_args() -> TestResult {
    Command::cargo_bin("yolors")?
        .args(&["demo_images/running2.jpg"])
        .assert()
        .success();
    Ok(())
}

