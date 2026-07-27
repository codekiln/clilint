use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::model::{
    CheckBundleIdentity, CheckMessageLevel, EvaluationMethod, InvocationSpec, SkillRef,
};

const CORE_CHECK_BUNDLE: &str = include_str!("../check-bundles/clilint/clilint.toml");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckBundleManifest {
    pub format_version: u32,
    pub check_bundle: CheckBundleIdentity,
    #[serde(default)]
    pub extends: Option<String>,
    pub checks: Vec<CheckDefinition>,
    #[serde(skip)]
    pub resolved_check_bundles: Vec<CheckBundleIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckDefinition {
    pub id: String,
    pub title: String,
    pub evaluation_method: EvaluationMethod,
    #[serde(default)]
    pub checker: Option<CheckerDefinition>,
    #[serde(default)]
    pub skill: Option<SkillRef>,
    #[serde(default)]
    pub evidence: Option<InvocationSpec>,
    #[serde(skip)]
    pub bundle_root: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CheckerDefinition {
    SingleInvocation {
        failure_message_level: CheckMessageLevel,
        #[serde(flatten)]
        invocation: InvocationCheck,
    },
    AnyInvocation {
        failure_message_level: CheckMessageLevel,
        invocations: Vec<InvocationCheck>,
    },
    AllInvocations {
        failure_message_level: CheckMessageLevel,
        invocations: Vec<InvocationCheck>,
    },
    Cli {
        command: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InvocationCheck {
    #[serde(flatten)]
    pub invocation: InvocationSpec,
    pub assertions: Vec<Assertion>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Assertion {
    ExitCode { value: i32 },
    ExitNonZero,
    NotTimedOut,
    StdoutNotEmpty,
    StderrNotEmpty,
    StdoutAtLeastStderr,
    OutputContainsAny { values: Vec<String> },
    StdoutContainsAny { values: Vec<String> },
    NoAnsi,
    DurationAtMost { milliseconds: f64 },
    VersionNumber,
}

pub fn parse(text: &str, source: &str) -> Result<CheckBundleManifest, String> {
    let manifest: CheckBundleManifest =
        toml::from_str(text).map_err(|error| format!("invalid check bundle {source}: {error}"))?;
    let document: toml::Value =
        toml::from_str(text).map_err(|error| format!("invalid check bundle {source}: {error}"))?;
    reject_unknown_checker_fields(&document, source)?;
    Ok(manifest)
}

fn reject_unknown_checker_fields(document: &toml::Value, source: &str) -> Result<(), String> {
    let Some(checks) = document.get("checks").and_then(toml::Value::as_array) else {
        return Ok(());
    };
    for check in checks {
        let Some(checker) = check.get("checker").and_then(toml::Value::as_table) else {
            continue;
        };
        let Some(checker_type) = checker.get("type").and_then(toml::Value::as_str) else {
            continue;
        };
        let allowed: &[&str] = match checker_type {
            "single-invocation" => &[
                "type",
                "failure_message_level",
                "args",
                "env",
                "stdin",
                "timeout_ms",
                "assertions",
            ],
            "any-invocation" | "all-invocations" => {
                &["type", "failure_message_level", "invocations"]
            }
            "cli" => &["type", "command"],
            _ => continue,
        };
        if let Some(field) = checker
            .keys()
            .find(|field| !allowed.contains(&field.as_str()))
        {
            return Err(format!(
                "invalid check bundle {source}: unknown field `{field}` in Checker `{checker_type}`"
            ));
        }
    }
    Ok(())
}

pub fn validate(bundle: &CheckBundleManifest) -> Result<(), String> {
    if bundle.format_version != 1 {
        return Err(format!(
            "check bundle {} uses unsupported format version {}",
            bundle.check_bundle.name, bundle.format_version
        ));
    }
    validate_name("check bundle", &bundle.check_bundle.name)?;
    validate_version("check bundle", &bundle.check_bundle.version)?;
    let prefix = format!("{}/", bundle.check_bundle.name);
    let mut ids = HashSet::new();
    for check in &bundle.checks {
        if !check.id.starts_with(&prefix) || check.id.len() == prefix.len() {
            return Err(format!(
                "check {} is not scoped to check bundle {}",
                check.id, bundle.check_bundle.name
            ));
        }
        if !ids.insert(check.id.clone()) {
            return Err(format!("duplicate check identifier {}", check.id));
        }
        match check.evaluation_method {
            EvaluationMethod::Mechanistic => {
                if check.checker.is_none() || check.skill.is_some() || check.evidence.is_some() {
                    return Err(format!(
                        "mechanistic check {} must have one checker and no judgment fields",
                        check.id
                    ));
                }
            }
            EvaluationMethod::JudgmentBased => {
                let built_in = bundle.check_bundle.name == "clilint";
                if built_in
                    && (check.checker.is_some()
                        || check.skill.is_none()
                        || check.evidence.is_none())
                {
                    return Err(format!(
                        "built-in judgment-based check {} must have a skill and evidence invocation",
                        check.id
                    ));
                }
                if !built_in
                    && (check.checker.is_none()
                        || check.skill.is_some()
                        || check.evidence.is_some())
                {
                    return Err(format!(
                        "judgment-based check {} in a local bundle must declare one Checker CLI and omit [checks.skill] and [checks.evidence]",
                        check.id
                    ));
                }
                if let Some(skill) = &check.skill {
                    validate_name("skill", &skill.name)?;
                    validate_version("skill", &skill.version)?;
                }
            }
        }
        if bundle.check_bundle.name != "clilint"
            && !matches!(
                check.checker,
                Some(CheckerDefinition::Cli { ref command }) if !command.is_empty()
            )
        {
            return Err(format!(
                "check {} in a local bundle must declare a nonempty Checker CLI command",
                check.id
            ));
        }
        if matches!(
            check.checker,
            Some(CheckerDefinition::Cli { ref command }) if command.is_empty()
        ) {
            return Err(format!("Checker CLI {} has an empty command", check.id));
        }
    }
    Ok(())
}

pub fn resolve(
    mut resolved_bundle: CheckBundleManifest,
    bundle_to_add: CheckBundleManifest,
) -> Result<CheckBundleManifest, String> {
    validate(&bundle_to_add)?;
    let included_bundle_name = bundle_to_add
        .extends
        .as_deref()
        .ok_or_else(|| {
            format!(
                "check bundle {} must declare extends",
                bundle_to_add.check_bundle.name
            )
        })?
        .to_owned();
    if bundle_to_add.check_bundle.name == resolved_bundle.check_bundle.name {
        return Err(format!(
            "check bundle {} conflicts with an included check bundle identity",
            bundle_to_add.check_bundle.name
        ));
    }
    let included_bundle_is_available = included_bundle_name == resolved_bundle.check_bundle.name
        || resolved_bundle
            .resolved_check_bundles
            .iter()
            .any(|identity| identity.name == included_bundle_name);
    if !included_bundle_is_available {
        return Err(format!(
            "check bundle {} extends unavailable check bundle {:?}",
            bundle_to_add.check_bundle.name, included_bundle_name
        ));
    }

    let mut check_ids: HashSet<String> = resolved_bundle
        .checks
        .iter()
        .map(|check| check.id.clone())
        .collect();
    for check in &bundle_to_add.checks {
        if !check_ids.insert(check.id.clone()) {
            return Err(format!(
                "check {} conflicts with a Check from an included bundle",
                check.id
            ));
        }
    }

    if resolved_bundle.resolved_check_bundles.is_empty() {
        resolved_bundle
            .resolved_check_bundles
            .push(resolved_bundle.check_bundle.clone());
    }
    resolved_bundle
        .resolved_check_bundles
        .push(bundle_to_add.check_bundle.clone());
    resolved_bundle.checks.extend(bundle_to_add.checks);
    Ok(resolved_bundle)
}

pub fn core() -> Result<CheckBundleManifest, String> {
    let mut core = parse(CORE_CHECK_BUNDLE, "built-in core check bundle")?;
    validate(&core)?;
    core.resolved_check_bundles = vec![core.check_bundle.clone()];
    Ok(core)
}

fn validate_name(kind: &str, name: &str) -> Result<(), String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Err(format!("invalid {kind} name {name:?}"));
    }
    Ok(())
}

fn validate_version(kind: &str, version: &str) -> Result<(), String> {
    let mut parts = version.split('.');
    let valid = (0..3).all(|_| {
        parts
            .next()
            .is_some_and(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    }) && parts.next().is_none();
    if !valid {
        return Err(format!("invalid {kind} version {version:?}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_source_parses_consistently() {
        let parsed = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        validate(&parsed).unwrap();
        let encoded = toml::to_string(&parsed).unwrap();
        let reparsed = parse(&encoded, "round trip").unwrap();
        assert_eq!(parsed.checks.len(), reparsed.checks.len());
        assert_eq!(parsed.check_bundle.name, "clilint");
    }

    #[test]
    fn codekiln_help_uses_the_user_bundle_format() {
        let text = include_str!("../check-bundles/codekiln-help/clilint.toml");
        let bundle = parse(text, "codekiln-help").unwrap();
        validate(&bundle).unwrap();
        assert_eq!(bundle.check_bundle.name, "codekiln-help");
        assert_eq!(bundle.checks.len(), 1);
    }

    #[test]
    fn rejects_unknown_check_type() {
        let invalid =
            CORE_CHECK_BUNDLE.replacen("type = \"any-invocation\"", "type = \"shell-program\"", 1);
        let error = parse(&invalid, "test").unwrap_err();
        assert!(error.contains("unknown variant") || error.contains("shell-program"));
    }

    #[test]
    fn rejects_unknown_checker_type() {
        let invalid = r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "invalid"
version = "1.0.0"
[[checks]]
id = "invalid/help/example"
title = "Invalid"
evaluation_method = "mechanistic"
[checks.checker]
type = "invented"
"#;
        assert!(parse(invalid, "test").unwrap_err().contains("invented"));
    }

    #[test]
    fn rejects_empty_checker_cli_command() {
        let invalid = r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "invalid"
version = "1.0.0"
[[checks]]
id = "invalid/help/example"
title = "Invalid"
evaluation_method = "mechanistic"
[checks.checker]
type = "cli"
command = []
"#;
        let bundle = parse(invalid, "test").unwrap();
        assert!(validate(&bundle).unwrap_err().contains("nonempty"));
    }

    #[test]
    fn rejects_duplicate_check_identifier() {
        let mut bundle = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        bundle.checks.push(bundle.checks[0].clone());
        assert!(validate(&bundle).unwrap_err().contains("duplicate check"));
    }

    #[test]
    fn rejects_invalid_skill_reference() {
        let mut bundle = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let check = bundle
            .checks
            .iter_mut()
            .find(|check| check.evaluation_method == EvaluationMethod::JudgmentBased)
            .unwrap();
        check.skill.as_mut().unwrap().name = "bad skill".into();
        assert!(validate(&bundle).unwrap_err().contains("invalid skill"));
    }

    fn additional_bundle() -> CheckBundleManifest {
        parse(
            r#"
format_version = 1
extends = "clilint"

[check_bundle]
name = "team"
version = "1.0.0"

[[checks]]
id = "team/help/team-flag"
title = "Help mentions the team flag"
evaluation_method = "mechanistic"
[checks.checker]
type = "cli"
command = ["team-checker"]
"#,
            "additional bundle",
        )
        .unwrap()
    }

    #[test]
    fn combining_bundles_is_additive() {
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let core_count = core.checks.len();
        let resolved = resolve(core, additional_bundle()).unwrap();
        assert_eq!(resolved.checks.len(), core_count + 1);
        assert!(
            resolved
                .checks
                .iter()
                .any(|check| check.id == "team/help/team-flag")
        );
    }

    #[test]
    fn rejects_deferred_composition_fields() {
        for field in ["exclude", "strengthen", "required_for_ratings"] {
            let invalid = format!(
                r#"
format_version = 1
extends = "clilint"
{field} = []

[check_bundle]
name = "team"
version = "1.0.0"

[[checks]]
id = "team/help/team-flag"
title = "Help mentions the team flag"
evaluation_method = "mechanistic"
[checks.checker]
type = "cli"
command = ["team-checker"]
"#
            );
            let error = parse(&invalid, "additional bundle").unwrap_err();
            assert!(error.contains(field), "{error}");
        }
    }

    #[test]
    fn rejects_check_bundle_identity_conflict() {
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let mut bundle = additional_bundle();
        bundle.check_bundle.name = "clilint".into();
        bundle.checks[0].id = "clilint/help/team-flag".into();
        assert!(resolve(core, bundle).unwrap_err().contains("conflicts"));
    }
}
