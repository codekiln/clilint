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
}

fn clilint() -> Command {
    Command::new(cargo_bin!("clilint"))
}

fn json_report(target: &str) -> serde_json::Value {
    json_report_with(target, None, &[])
}

fn json_report_with(
    target: &str,
    bundle: Option<PathBuf>,
    environment: &[(&str, &str)],
) -> serde_json::Value {
    let has_checker_cli = bundle.is_some();
    let mut command = clilint();
    command.args([
        "check",
        fixture(target).to_str().unwrap(),
        "--format",
        "json",
    ]);
    if let Some(bundle) = bundle {
        command.args(["--check-bundle", bundle.to_str().unwrap()]);
    }
    command.envs(environment.iter().copied());
    let output = command.output().unwrap();
    assert!(
        output.status.code().is_some_and(|code| code <= 1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    if !has_checker_cli {
        assert!(output.stderr.is_empty());
    }
    serde_json::from_slice(&output.stdout).unwrap()
}

fn check<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["check"] == id)
        .unwrap()
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
fn json_output_uses_check_outcomes_scores_and_messages() {
    let report = json_report("useful-help-cli");
    assert_eq!(report["format_version"], 3);
    assert_eq!(report["tool_version"], "0.0.2");
    assert_eq!(report["summary"]["check_errors"], 0);
    assert_eq!(report["summary"]["awaiting_assessment"], 1);
    assert_eq!(report["checks"].as_array().unwrap().len(), 17);
    assert_eq!(
        check(&report, "clilint/help/useful-example")["outcome"],
        "awaiting-assessment"
    );
}

#[test]
fn human_output_names_both_check_methods() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", fixture("useful-help-cli").to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("mechanistic"))
        .stdout(predicate::str::contains("judgment-based"))
        .stdout(predicate::str::contains("Awaiting Assessment"))
        .stderr("");
}

#[test]
fn error_message_exits_one_but_warning_does_not() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", fixture("bad-cli").to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error"))
        .stderr("");

    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["check", fixture("useful-help-cli").to_str().unwrap()])
        .assert()
        .success();
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
fn valid_assessment_produces_a_scored_result() {
    let assessment = fixture("useful-help-assessment.json");
    let output = clilint()
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            assessment.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let result = check(&report, "clilint/help/useful-example");
    assert_eq!(result["outcome"], "result");
    assert_eq!(result["result"]["score"], 4.0);
    assert_eq!(
        result["result"]["assessment"]["explanation"],
        "The examples teach both a likely write task and a machine-readable read task."
    );
}

#[test]
fn stale_and_malformed_assessments_are_rejected() {
    let directory = tempdir().unwrap();
    let stale = directory.path().join("stale.json");
    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(fixture("useful-help-assessment.json")).unwrap())
            .unwrap();
    value["evidence_digest"] = "sha256:stale".into();
    fs::write(&stale, serde_json::to_vec(&value).unwrap()).unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            stale.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("stale evidence digest"));

    let malformed = directory.path().join("malformed.json");
    fs::write(&malformed, "{not-json").unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            malformed.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("invalid Assessment"));

    let assessment = fixture("useful-help-assessment.json");
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--assessment",
            assessment.to_str().unwrap(),
            "--assessment",
            assessment.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "more than one Assessment was supplied",
        ));
}

#[test]
fn local_bundle_install_records_a_relative_path_and_can_be_removed() {
    let directory = tempdir().unwrap();
    let project = directory.path().join("project");
    let bundle = directory.path().join("bundle");
    fs::create_dir_all(&project).unwrap();
    copy_fixture_bundle("checker-bundle", &bundle);

    assert_cmd::Command::new(cargo_bin!("clilint"))
        .current_dir(&project)
        .args(["bundle", "install", "../bundle"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fixture-checker: installed"));
    let config = fs::read_to_string(project.join(".clilint/config.toml")).unwrap();
    assert!(config.contains("path = \"../bundle\""));
    assert!(!config.contains(directory.path().to_str().unwrap()));

    assert_cmd::Command::new(cargo_bin!("clilint"))
        .current_dir(&project)
        .args(["bundle", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"installed\": true"));
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .current_dir(&project)
        .args(["bundle", "remove", "fixture-checker"])
        .assert()
        .success();
}

#[test]
fn installed_codekiln_help_and_its_extension_run_in_inheritance_order() {
    let directory = tempdir().unwrap();
    let extension = directory.path().join("extension");
    copy_fixture_bundle("checker-bundle", &extension);
    let manifest_path = extension.join("clilint.toml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace("extends = \"clilint\"", "extends = \"codekiln-help\"");
    fs::write(manifest_path, manifest).unwrap();

    for bundle in [check_bundle("codekiln-help"), extension] {
        assert_cmd::Command::new(cargo_bin!("clilint"))
            .current_dir(directory.path())
            .arg("bundle")
            .arg("install")
            .arg(bundle)
            .assert()
            .success();
    }
    let output = clilint()
        .current_dir(directory.path())
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
    assert_eq!(
        report["check_bundles"],
        serde_json::json!([
            {"name": "clilint", "version": "0.0.2"},
            {"name": "codekiln-help", "version": "0.1.0"},
            {"name": "fixture-checker", "version": "1.0.0"}
        ])
    );
    assert_eq!(
        check(&report, "codekiln-help/help/hierarchical")["result"]["score"],
        4.0
    );
    assert_eq!(
        check(&report, "fixture-checker/help/default-output")["outcome"],
        "result"
    );
}

#[test]
fn removed_git_lock_update_and_offline_options_are_rejected() {
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["bundle", "lock"])
        .assert()
        .code(2);
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args(["bundle", "update"])
        .assert()
        .code(2);
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--offline",
        ])
        .assert()
        .code(2);
}

#[test]
fn checker_cli_runs_from_the_project_and_finds_bundle_resources() {
    let report = json_report_with(
        "hierarchical-help-cli",
        Some(fixture("checker-bundle")),
        &[],
    );
    let result = check(&report, "fixture-checker/help/default-output");
    assert_eq!(result["outcome"], "result");
    assert_eq!(result["result"]["score"], 2.5);
    let evidence = &result["result"]["messages"][0]["evidence"];
    assert_eq!(
        evidence["cwd"],
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .to_string_lossy()
            .as_ref()
    );
    assert!(
        evidence["resource"]
            .as_str()
            .unwrap()
            .ends_with("checker-bundle/expectation.txt")
    );
}

#[test]
fn checker_cli_logs_continue_to_clilint_standard_error() {
    let output = clilint()
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--check-bundle",
            fixture("checker-bundle").to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("fixture Checker ran"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn malformed_checker_output_becomes_a_check_error() {
    let directory = tempdir().unwrap();
    let bundle = directory.path().join("bundle");
    fs::create_dir_all(&bundle).unwrap();
    fs::write(
        bundle.join("clilint.toml"),
        r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "malformed"
version = "1.0.0"
[[checks]]
id = "malformed/protocol/output"
title = "Malformed output"
severity = "error"
evaluation_method = "mechanistic"
[checks.checker]
type = "cli"
command = ["python3", "{bundle}/checker.py"]
"#,
    )
    .unwrap();
    fs::write(bundle.join("checker.py"), "print('{not-json')\n").unwrap();
    let report = json_report_with("useful-help-cli", Some(bundle), &[]);
    let result = check(&report, "malformed/protocol/output");
    assert_eq!(result["outcome"], "error");
    assert!(
        result["error"]["message"]
            .as_str()
            .unwrap()
            .contains("invalid JSON")
    );
}

#[test]
fn bundle_validation_rejects_unknown_fields_and_an_empty_command() {
    let directory = tempdir().unwrap();
    let typo = directory.path().join("typo.toml");
    fs::write(
        &typo,
        r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "invalid"
version = "1.0.0"
[[checks]]
id = "invalid/protocol/example"
title = "Invalid"
severity = "error"
evaluation_method = "mechanistic"
[checks.checker]
type = "cli"
command = ["checker"]
timeout_mss = 1
"#,
    )
    .unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--check-bundle",
            typo.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("timeout_mss"));

    let empty = directory.path().join("empty.toml");
    fs::write(
        &empty,
        fs::read_to_string(&typo)
            .unwrap()
            .replace("command = [\"checker\"]\ntimeout_mss = 1", "command = []"),
    )
    .unwrap();
    assert_cmd::Command::new(cargo_bin!("clilint"))
        .args([
            "check",
            fixture("useful-help-cli").to_str().unwrap(),
            "--check-bundle",
            empty.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("nonempty Checker CLI"));
}

#[test]
fn codekiln_help_returns_one_perfect_result_for_the_passing_fixture() {
    let report = json_report_with(
        "hierarchical-help-cli",
        Some(check_bundle("codekiln-help")),
        &[],
    );
    let result = check(&report, "codekiln-help/help/hierarchical");
    assert_eq!(result["outcome"], "result");
    assert_eq!(result["result"]["score"], 4.0);
    assert_eq!(result["result"]["messages"].as_array().unwrap().len(), 0);
}

#[test]
fn codekiln_help_returns_focused_messages_for_nested_failures() {
    for failure in ["root:outline", "repo:outline", "repo-clone:outline"] {
        let report = json_report_with(
            "hierarchical-help-cli",
            Some(check_bundle("codekiln-help")),
            &[("HIERARCHICAL_HELP_FAIL", failure)],
        );
        let result = check(&report, "codekiln-help/help/hierarchical");
        assert_eq!(result["outcome"], "result");
        assert!(result["result"]["score"].as_f64().unwrap() < 4.0);
        assert!(
            result["result"]["messages"]
                .as_array()
                .unwrap()
                .iter()
                .all(|message| message["level"] == "error")
        );
        let expected_path = match failure {
            "root:outline" => Vec::<String>::new(),
            "repo:outline" => vec!["repo".into()],
            _ => vec!["repo".into(), "clone".into()],
        };
        assert!(
            result["result"]["messages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|message| message["evidence"]["command_path"]
                    == serde_json::json!(expected_path))
        );
    }
}

#[test]
fn codekiln_help_rejects_malformed_relationships_and_empty_search() {
    for (name, value) in [
        ("HIERARCHICAL_HELP_MALFORMED", "1"),
        ("HIERARCHICAL_HELP_WRONG_PATH", "1"),
        ("HIERARCHICAL_HELP_INVALID_HEADING", "1"),
        ("HIERARCHICAL_HELP_EMPTY_SEARCH", "1"),
        ("HIERARCHICAL_HELP_INCOMPLETE_PROGRAMMATIC", "1"),
    ] {
        let report = json_report_with(
            "hierarchical-help-cli",
            Some(check_bundle("codekiln-help")),
            &[(name, value)],
        );
        let result = check(&report, "codekiln-help/help/hierarchical");
        assert_eq!(result["outcome"], "result");
        assert!(result["result"]["score"].as_f64().unwrap() < 4.0);
        assert!(!result["result"]["messages"].as_array().unwrap().is_empty());
    }
}

#[test]
fn codekiln_help_reports_an_exhausted_command_count_budget() {
    let report = json_report_with(
        "hierarchical-help-cli",
        Some(check_bundle("codekiln-help")),
        &[("HIERARCHICAL_HELP_WIDE", "1")],
    );
    let result = check(&report, "codekiln-help/help/hierarchical");
    assert!(
        result["result"]["messages"]
            .as_array()
            .unwrap()
            .iter()
            .any(|message| message["message"]
                .as_str()
                .unwrap()
                .contains("command-count limit"))
    );
}

#[test]
fn judgment_checker_cli_uses_the_file_based_two_pass_handoff() {
    let directory = tempdir().unwrap();
    fs::write(
        directory.path().join("README.md"),
        "# Example\n\nRun `example start` for the first task.\n",
    )
    .unwrap();
    let bundle = fixture("judgment-checker-bundle");
    let target = fixture("useful-help-cli");
    let first = clilint()
        .current_dir(directory.path())
        .args([
            "check",
            target.to_str().unwrap(),
            "--check-bundle",
            bundle.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(first.status.success());
    let first: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    let pending = check(&first, "fixture-judgment/docs/first-task");
    assert_eq!(pending["outcome"], "awaiting-assessment");

    let request = &pending["assessment_request"];
    let assessment = serde_json::json!({
        "format_version": 1,
        "request_id": request["request_id"],
        "check": "fixture-judgment/docs/first-task",
        "evidence_digest": request["evidence_digest"],
        "skill": request["skill"],
        "score": 4.0,
        "messages": [],
        "explanation": "The README gives a concrete command for a first task.",
        "assessor": "integration-test"
    });
    let assessment_path = directory.path().join("assessment.json");
    fs::write(
        &assessment_path,
        serde_json::to_vec_pretty(&assessment).unwrap(),
    )
    .unwrap();
    let second = clilint()
        .current_dir(directory.path())
        .args([
            "check",
            target.to_str().unwrap(),
            "--check-bundle",
            bundle.to_str().unwrap(),
            "--assessment",
            assessment_path.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(second.status.success());
    let second: serde_json::Value = serde_json::from_slice(&second.stdout).unwrap();
    let completed = check(&second, "fixture-judgment/docs/first-task");
    assert_eq!(completed["outcome"], "result");
    assert_eq!(completed["result"]["score"], 4.0);
}

#[test]
fn judgment_checker_reports_a_missing_bundled_skill() {
    let directory = tempdir().unwrap();
    fs::write(directory.path().join("README.md"), "# Example\n").unwrap();
    let bundle = directory.path().join("bundle");
    copy_fixture_bundle("judgment-checker-bundle", &bundle);
    fs::remove_file(bundle.join("SKILL.md")).unwrap();
    let output = clilint()
        .current_dir(directory.path())
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
    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let result = check(&report, "fixture-judgment/docs/first-task");
    assert_eq!(result["outcome"], "error");
    assert!(result.get("score").is_none());
}

fn copy_fixture_bundle(name: &str, destination: &std::path::Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(fixture(name)).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
        }
    }
}
