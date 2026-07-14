use std::{collections::HashSet, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{EvaluationMethod, InvocationSpec, PackageIdentity, Severity, SkillRef};

const GLOBAL_PACKAGE: &str = include_str!("../packages/clilint/clilint.toml");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageManifest {
    pub format_version: u32,
    pub package: PackageIdentity,
    #[serde(default)]
    pub extends: Option<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub strengthen: Vec<Strengthening>,
    pub rules: Vec<RuleDefinition>,
    #[serde(skip)]
    pub resolved_packages: Vec<PackageIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Strengthening {
    pub rule: String,
    pub severity: Severity,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleDefinition {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub evaluation_method: EvaluationMethod,
    #[serde(default)]
    pub check: Option<CheckDefinition>,
    #[serde(default)]
    pub skill: Option<SkillRef>,
    #[serde(default)]
    pub evidence: Option<InvocationSpec>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CheckDefinition {
    Invocation {
        #[serde(flatten)]
        invocation: InvocationCheck,
    },
    AnyInvocation {
        invocations: Vec<InvocationCheck>,
    },
    AllInvocations {
        invocations: Vec<InvocationCheck>,
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
#[serde(tag = "type", rename_all = "kebab-case")]
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

pub fn load_resolved(extension_path: Option<&Path>) -> Result<PackageManifest, String> {
    let mut global = parse(GLOBAL_PACKAGE, "bundled global package")?;
    validate(&global)?;
    if let Some(path) = extension_path {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("could not read package {}: {error}", path.display()))?;
        let extension = parse(&text, &path.display().to_string())?;
        resolve(global, extension)
    } else {
        global.resolved_packages = vec![global.package.clone()];
        Ok(global)
    }
}

pub fn parse(text: &str, source: &str) -> Result<PackageManifest, String> {
    toml::from_str(text).map_err(|error| format!("invalid package {source}: {error}"))
}

pub fn validate(package: &PackageManifest) -> Result<(), String> {
    if package.format_version != 1 {
        return Err(format!(
            "package {} uses unsupported format version {}",
            package.package.name, package.format_version
        ));
    }
    validate_name("package", &package.package.name)?;
    validate_version("package", &package.package.version)?;
    let prefix = format!("{}/", package.package.name);
    let mut ids = HashSet::new();
    for rule in &package.rules {
        if !rule.id.starts_with(&prefix) || rule.id.len() == prefix.len() {
            return Err(format!(
                "rule {} is not scoped to package {}",
                rule.id, package.package.name
            ));
        }
        if !ids.insert(rule.id.clone()) {
            return Err(format!("duplicate rule identifier {}", rule.id));
        }
        match rule.evaluation_method {
            EvaluationMethod::Deterministic => {
                if rule.check.is_none() || rule.skill.is_some() || rule.evidence.is_some() {
                    return Err(format!(
                        "deterministic rule {} must have one check and no agent fields",
                        rule.id
                    ));
                }
            }
            EvaluationMethod::AiAgent => {
                if rule.check.is_some() || rule.skill.is_none() || rule.evidence.is_none() {
                    return Err(format!(
                        "AI-agent rule {} must have a skill and evidence invocation",
                        rule.id
                    ));
                }
                let skill = rule.skill.as_ref().expect("checked above");
                validate_name("skill", &skill.name)?;
                validate_version("skill", &skill.version)?;
            }
        }
    }
    Ok(())
}

fn resolve(
    mut global: PackageManifest,
    extension: PackageManifest,
) -> Result<PackageManifest, String> {
    validate(&extension)?;
    if extension.package.name == global.package.name {
        return Err(format!(
            "extension package {} conflicts with the bundled package identity",
            extension.package.name
        ));
    }
    if extension.extends.as_deref() != Some(global.package.name.as_str()) {
        return Err(format!(
            "extension package {} must declare extends = {:?}",
            extension.package.name, global.package.name
        ));
    }
    if let Some(rule) = extension.exclude.first() {
        return Err(format!(
            "extension package {} cannot exclude inherited rule {rule}",
            extension.package.name
        ));
    }

    let mut inherited: HashSet<String> = global.rules.iter().map(|rule| rule.id.clone()).collect();
    for rule in &extension.rules {
        if !inherited.insert(rule.id.clone()) {
            return Err(format!(
                "extension rule {} conflicts with an inherited rule",
                rule.id
            ));
        }
    }
    for strengthening in &extension.strengthen {
        let rule = global
            .rules
            .iter_mut()
            .find(|rule| rule.id == strengthening.rule)
            .ok_or_else(|| {
                format!(
                    "extension tries to strengthen unknown inherited rule {}",
                    strengthening.rule
                )
            })?;
        if strengthening.severity.rank() < rule.severity.rank() {
            return Err(format!(
                "extension weakens inherited rule {} from {:?} to {:?}",
                rule.id, rule.severity, strengthening.severity
            ));
        }
        rule.severity = strengthening.severity;
    }

    global.resolved_packages = vec![global.package.clone(), extension.package.clone()];
    global.rules.extend(extension.rules);
    Ok(global)
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
        let parsed = parse(GLOBAL_PACKAGE, "test").unwrap();
        validate(&parsed).unwrap();
        let encoded = toml::to_string(&parsed).unwrap();
        let reparsed = parse(&encoded, "round trip").unwrap();
        assert_eq!(parsed.rules.len(), reparsed.rules.len());
        assert_eq!(parsed.package.name, "clilint");
    }

    #[test]
    fn rejects_unknown_check_type() {
        let invalid =
            GLOBAL_PACKAGE.replacen("type = \"any-invocation\"", "type = \"shell-program\"", 1);
        let error = parse(&invalid, "test").unwrap_err();
        assert!(error.contains("unknown variant") || error.contains("shell-program"));
    }

    #[test]
    fn rejects_duplicate_rule_identifier() {
        let mut package = parse(GLOBAL_PACKAGE, "test").unwrap();
        package.rules.push(package.rules[0].clone());
        assert!(validate(&package).unwrap_err().contains("duplicate rule"));
    }

    #[test]
    fn rejects_invalid_skill_reference() {
        let mut package = parse(GLOBAL_PACKAGE, "test").unwrap();
        let rule = package
            .rules
            .iter_mut()
            .find(|rule| rule.evaluation_method == EvaluationMethod::AiAgent)
            .unwrap();
        rule.skill.as_mut().unwrap().name = "bad skill".into();
        assert!(validate(&package).unwrap_err().contains("invalid skill"));
    }

    fn extension() -> PackageManifest {
        parse(
            r#"
format_version = 1
extends = "clilint"

[package]
name = "team"
version = "1.0.0"

[[rules]]
id = "team/help/team-flag"
title = "Help mentions the team flag"
severity = "warn"
evaluation_method = "deterministic"
[rules.check]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["--team"] }]
"#,
            "extension",
        )
        .unwrap()
    }

    #[test]
    fn extension_is_additive() {
        let global = parse(GLOBAL_PACKAGE, "test").unwrap();
        let global_count = global.rules.len();
        let resolved = resolve(global, extension()).unwrap();
        assert_eq!(resolved.rules.len(), global_count + 1);
        assert!(
            resolved
                .rules
                .iter()
                .any(|rule| rule.id == "team/help/team-flag")
        );
    }

    #[test]
    fn rejects_weakening() {
        let global = parse(GLOBAL_PACKAGE, "test").unwrap();
        let mut extension = extension();
        extension.strengthen.push(Strengthening {
            rule: "clilint/basics/success-exit".into(),
            severity: Severity::Warn,
        });
        assert!(resolve(global, extension).unwrap_err().contains("weakens"));
    }

    #[test]
    fn rejects_exclusion() {
        let global = parse(GLOBAL_PACKAGE, "test").unwrap();
        let mut extension = extension();
        extension.exclude.push("clilint/basics/success-exit".into());
        assert!(
            resolve(global, extension)
                .unwrap_err()
                .contains("cannot exclude")
        );
    }

    #[test]
    fn rejects_package_identity_conflict() {
        let global = parse(GLOBAL_PACKAGE, "test").unwrap();
        let mut extension = extension();
        extension.package.name = "clilint".into();
        extension.rules[0].id = "clilint/help/team-flag".into();
        assert!(
            resolve(global, extension)
                .unwrap_err()
                .contains("conflicts")
        );
    }
}
