use std::{fs, path::PathBuf, process::Command};

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use tempfile::tempdir;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn check_bundle(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("check-bundles")
        .join(name)
        .join("clilint.toml")
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
        .find(|finding| finding["check"] == "clilint/help/useful-example")
        .unwrap();
    let directory = tempdir().unwrap();
    let assessment = directory.path().join("assessment.toml");
    fs::write(
        &assessment,
        format!(
            r#"format_version = 1
check = "clilint/help/useful-example"
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
check = "clilint/help/useful-example"
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
fn local_extension_adds_checks_without_replacing_core_checks() {
    let directory = tempdir().unwrap();
    let bundle = directory.path().join("team.toml");
    fs::write(
        &bundle,
        r#"format_version = 1
extends = "clilint"

[check_bundle]
name = "team"
version = "1.0.0"

[[checks]]
id = "team/help/address-book"
title = "Help describes the address book"
severity = "warn"
evaluation_method = "deterministic"
[checks.checker]
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
            "--check-bundle",
            bundle.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["check_bundles"].as_array().unwrap().len(), 2);
    assert_eq!(report["findings"].as_array().unwrap().len(), 18);
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|finding| finding["check"] == "team/help/address-book")
    );
}

#[test]
fn report_uses_check_bundle_check_and_checker_vocabulary() {
    let report = json_report("useful-help-cli");
    assert!(report.get("check_bundles").is_some());
    assert!(report.get("packages").is_none());
    assert!(report["findings"][0].get("check").is_some());
    assert!(report["findings"][0].get("rule").is_none());
}

#[test]
fn installs_codekiln_help_for_one_project_and_runs_it_automatically() {
    // Worked-example walkthrough: docs/codekiln-help.md
    let project = tempdir().unwrap();
    let bundle = check_bundle("codekiln-help");
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .current_dir(project.path())
        .args(["bundle", "install", bundle.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("codekiln-help: installed"));

    let config = fs::read_to_string(project.path().join(".clilint/config.toml")).unwrap();
    assert!(config.contains("[check_bundles.codekiln-help]"));
    assert!(config.contains("source = \"local\""));

    let output = clilint()
        .current_dir(project.path())
        .args([
            "check",
            fixture("hierarchical-help-cli").to_str().unwrap(),
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
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let help_findings = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| {
            finding["check"]
                .as_str()
                .is_some_and(|check| check.starts_with("codekiln-help/"))
        })
        .collect::<Vec<_>>();
    assert_eq!(help_findings.len(), 10);
    assert!(
        help_findings
            .iter()
            .all(|finding| finding["result"] == "pass")
    );
    let web = help_findings
        .iter()
        .find(|finding| finding["check"] == "codekiln-help/help/web-view")
        .unwrap();
    assert_eq!(
        web["required_for_ratings"],
        serde_json::json!(["good", "excellent"])
    );
}

#[test]
fn a_local_bundle_extends_installed_codekiln_help_in_order() {
    let project = tempdir().unwrap();
    let extension = project.path().join("team.toml");
    fs::write(
        &extension,
        r#"format_version = 1
extends = "codekiln-help"
[check_bundle]
name = "team"
version = "1.0.0"
[[checks]]
id = "team/help/name"
title = "Help names the fixture"
severity = "warn"
evaluation_method = "deterministic"
[checks.checker]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["fixture"] }]
"#,
    )
    .unwrap();
    for path in [check_bundle("codekiln-help"), extension] {
        assert_cmd::Command::new(cargo_bin!("clilint"))
            .current_dir(project.path())
            .args(["bundle", "install", path.to_str().unwrap()])
            .assert()
            .success();
    }
    let output = clilint()
        .current_dir(project.path())
        .args([
            "check",
            fixture("hierarchical-help-cli").to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let names = report["check_bundles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|bundle| bundle["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, ["clilint", "codekiln-help", "team"]);
    assert_eq!(report["findings"].as_array().unwrap().len(), 28);
}

#[test]
fn hierarchical_failures_name_root_group_and_deep_paths() {
    for (fixture_name, expected_path) in [
        ("hierarchical-help-fails-root", "[]"),
        ("hierarchical-help-fails-group", "[\"repo\"]"),
        ("hierarchical-help-fails-deep", "[\"repo\", \"clone\"]"),
    ] {
        let output = clilint()
            .args([
                "check",
                fixture(fixture_name).to_str().unwrap(),
                "--check-bundle",
                check_bundle("codekiln-help").to_str().unwrap(),
                "--format",
                "json",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let finding = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|finding| finding["check"] == "codekiln-help/help/outline")
            .unwrap();
        assert_eq!(finding["result"], "fail");
        assert!(finding["detail"].as_str().unwrap().contains(expected_path));
    }
}

#[test]
fn hierarchical_checker_reports_exceeded_limits_and_timeouts() {
    let directory = tempdir().unwrap();
    let bundle = directory.path().join("limited.toml");
    fs::write(
        &bundle,
        r#"format_version = 1
extends = "clilint"
[check_bundle]
name = "limited"
version = "1.0.0"
[[checks]]
id = "limited/help/discovery"
title = "Limited discovery"
severity = "error"
evaluation_method = "deterministic"
[checks.checker]
type = "hierarchical-help"
behavior = "command-discovery"
command_count = 1
timeout_ms = 1
"#,
    )
    .unwrap();
    let output = clilint()
        .args([
            "check",
            fixture("hierarchical-help-cli").to_str().unwrap(),
            "--check-bundle",
            bundle.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["check"] == "limited/help/discovery")
        .unwrap();
    assert_eq!(finding["result"], "fail");
    let detail = finding["detail"].as_str().unwrap();
    assert!(detail.contains("timed out") || detail.contains("count limit"));
}

#[test]
fn hierarchical_checker_rejects_malformed_json_and_invalid_paths() {
    for fixture_name in [
        "hierarchical-help-malformed",
        "hierarchical-help-wrong-path",
        "hierarchical-help-invalid-heading",
    ] {
        let output = clilint()
            .args([
                "check",
                fixture(fixture_name).to_str().unwrap(),
                "--check-bundle",
                check_bundle("codekiln-help").to_str().unwrap(),
                "--format",
                "json",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            report["findings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|finding| {
                    finding["check"]
                        .as_str()
                        .is_some_and(|check| check.starts_with("codekiln-help/"))
                        && finding["result"] == "fail"
                })
        );
    }
}

#[test]
fn git_bundle_can_be_locked_installed_offline_listed_and_removed() {
    let project = tempdir().unwrap();
    let repository = tempdir().unwrap();
    let data = project.path().join("data");
    let cache = project.path().join("cache");
    fs::write(
        repository.path().join("clilint.toml"),
        r#"format_version = 1
extends = "clilint"
[check_bundle]
name = "git-example"
version = "1.0.0"
[[checks]]
id = "git-example/help/name"
title = "Help names the fixture"
severity = "warn"
evaluation_method = "deterministic"
[checks.checker]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["fixture"] }]
"#,
    )
    .unwrap();
    for args in [
        vec!["init", "--quiet"],
        vec!["add", "clilint.toml"],
        vec![
            "-c",
            "user.name=Clilint Test",
            "-c",
            "user.email=clilint@example.test",
            "commit",
            "--quiet",
            "-m",
            "bundle",
        ],
    ] {
        assert!(
            Command::new("git")
                .current_dir(repository.path())
                .args(args)
                .status()
                .unwrap()
                .success()
        );
    }
    let source = format!("file://{}#HEAD", repository.path().display());
    let command = || {
        let mut command = assert_cmd::Command::new(cargo_bin!("clilint"));
        command
            .current_dir(project.path())
            .env("CLILINT_DATA_DIR", &data)
            .env("CLILINT_CACHE_DIR", &cache);
        command
    };
    command()
        .args(["bundle", "install", &source])
        .assert()
        .success();
    command().args(["bundle", "lock"]).assert().success();
    command()
        .args(["bundle", "install", "--locked"])
        .assert()
        .success();
    command()
        .args(["bundle", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"resolved_commit\""))
        .stdout(predicate::str::contains("\"installed\": true"));
    command()
        .args([
            "check",
            fixture("hierarchical-help-cli").to_str().unwrap(),
            "--offline",
            "--locked",
        ])
        .assert()
        .success();
    command()
        .args(["bundle", "remove", "git-example"])
        .assert()
        .success();
    assert!(
        !fs::read_to_string(project.path().join(".clilint/config.toml"))
            .unwrap()
            .contains("git-example")
    );
}

#[test]
fn programmatic_help_keeps_the_default_document_and_adds_guidance() {
    let target = fixture("hierarchical-help-cli");
    let default = Command::new(&target)
        .env("CLILINT_OFFLINE", "1")
        .args(["repo", "clone", "help"])
        .output()
        .unwrap();
    let programmatic = Command::new(&target)
        .env("CLILINT_OFFLINE", "1")
        .args(["repo", "clone", "help", "--programmatic"])
        .output()
        .unwrap();
    assert!(default.status.success());
    assert!(programmatic.status.success());
    let default = String::from_utf8(default.stdout).unwrap();
    let programmatic = String::from_utf8(programmatic.stdout).unwrap();
    assert!(programmatic.starts_with(&default));
    assert!(programmatic.contains("without a pager"));
    assert!(programmatic.contains("help section"));
}

#[test]
fn offline_check_fails_before_running_when_a_declared_bundle_is_missing() {
    let project = tempdir().unwrap();
    fs::create_dir(project.path().join(".clilint")).unwrap();
    fs::write(
        project.path().join(".clilint/config.toml"),
        r#"[check_bundles.missing]
source = "git"
url = "https://example.invalid/missing.git"
ref = "main"
"#,
    )
    .unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .current_dir(project.path())
        .env("CLILINT_DATA_DIR", project.path().join("data"))
        .args(["check", "./target-that-must-not-run", "--offline"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "check bundle missing is not installed",
        ))
        .stderr(predicate::str::contains("clilint bundle install"));
}
