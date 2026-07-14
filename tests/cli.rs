use std::{fs, path::PathBuf, process::Command};

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use tempfile::tempdir;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn clilint() -> Command {
    Command::new(cargo_bin!("clilint"))
}

fn json_report(target: &str) -> serde_json::Value {
    let output = clilint()
        .args([
            "check",
            fixture(target).to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn version_is_printed_to_stdout() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .arg("--version")
        .assert()
        .success()
        .stdout("clilint 0.0.2\n")
        .stderr("");
}

#[test]
fn json_output_is_one_report_with_separate_summaries() {
    let report = json_report("useful-help-cli");
    assert_eq!(report["tool_version"], "0.0.2");
    assert_eq!(report["deterministic"]["fail"], 0);
    assert_eq!(report["ai_agent"]["unassessed"], 1);
    assert_eq!(report["findings"].as_array().unwrap().len(), 17);
}

#[test]
fn human_output_names_both_evaluation_methods() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", fixture("useful-help-cli").to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deterministic score: 100/100"))
        .stdout(predicate::str::contains("AI agent:"))
        .stderr("");
}

#[test]
fn deterministic_failure_exits_one() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", fixture("bad-cli").to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("fail"))
        .stderr("");
}

#[test]
fn missing_target_is_a_diagnostic_error() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", "./definitely-missing-clilint-target"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::starts_with("error: could not run target"));
}

#[test]
fn valid_assessment_is_attached() {
    let first = json_report("useful-help-cli");
    let finding = first["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["rule"] == "clilint/help/useful-example")
        .unwrap();
    let directory = tempdir().unwrap();
    let assessment = directory.path().join("assessment.toml");
    fs::write(
        &assessment,
        format!(
            r#"format_version = 1
rule = "clilint/help/useful-example"
result = "pass"
explanation = "The contacts add example teaches a likely task."
evidence_digest = "{}"
assessor = "integration-test"

[skill]
name = "assess-cli-help"
version = "1.0.0"
"#,
            finding["evidence_digest"].as_str().unwrap()
        ),
    )
    .unwrap();

    let output = clilint()
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--format",
            "json",
            "--assessment",
            assessment.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["ai_agent"]["pass"], 1);
    assert_eq!(report["ai_agent"]["unassessed"], 0);
}

#[test]
fn stale_and_malformed_assessments_are_rejected() {
    let directory = tempdir().unwrap();
    let stale = directory.path().join("stale.toml");
    fs::write(
        &stale,
        r#"format_version = 1
rule = "clilint/help/useful-example"
result = "pass"
explanation = "stale"
evidence_digest = "sha256:stale"
[skill]
name = "assess-cli-help"
version = "1.0.0"
"#,
    )
    .unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            stale.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::contains("stale evidence digest"));

    let malformed = directory.path().join("malformed.toml");
    fs::write(&malformed, "this is not = toml =").unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            malformed.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::contains("invalid assessment"));
}

#[test]
fn local_extension_adds_rules_without_replacing_global_rules() {
    let directory = tempdir().unwrap();
    let package = directory.path().join("team.toml");
    fs::write(
        &package,
        r#"format_version = 1
extends = "clilint"

[package]
name = "team"
version = "1.0.0"

[[rules]]
id = "team/help/address-book"
title = "Help describes the address book"
severity = "warn"
evaluation_method = "deterministic"
[rules.check]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["address book"] }]
"#,
    )
    .unwrap();

    let output = clilint()
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--package",
            package.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["packages"].as_array().unwrap().len(), 2);
    assert_eq!(report["findings"].as_array().unwrap().len(), 18);
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|finding| finding["rule"] == "team/help/address-book")
    );
}
