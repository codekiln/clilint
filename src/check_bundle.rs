use std::{collections::HashSet, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{
    CheckBundleIdentity, EvaluationMethod, InvocationSpec, RatingLevel, Severity, SkillRef,
};

const CORE_CHECK_BUNDLE: &str = include_str!("../check-bundles/clilint/clilint.toml");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckBundleManifest {
    pub format_version: u32,
    pub check_bundle: CheckBundleIdentity,
    #[serde(default)]
    pub extends: Option<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub strengthen: Vec<Strengthening>,
    pub checks: Vec<CheckDefinition>,
    #[serde(skip)]
    pub resolved_check_bundles: Vec<CheckBundleIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Strengthening {
    pub check: String,
    pub severity: Severity,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckDefinition {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub evaluation_method: EvaluationMethod,
    #[serde(default)]
    pub required_for_ratings: Vec<RatingLevel>,
    #[serde(default)]
    pub checker: Option<CheckerDefinition>,
    #[serde(default)]
    pub skill: Option<SkillRef>,
    #[serde(default)]
    pub evidence: Option<InvocationSpec>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CheckerDefinition {
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
    HierarchicalHelp {
        behavior: HierarchicalHelpBehavior,
        #[serde(default, flatten)]
        limits: HelpLimits,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchicalHelpBehavior {
    Availability,
    CommandDiscovery,
    Outline,
    SectionRetrieval,
    SharedHelp,
    ProgrammaticGuidance,
    Search,
    NonInteractiveOutput,
    LocalView,
    WebView,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct HelpLimits {
    pub command_count: usize,
    pub command_depth: usize,
    pub document_bytes: usize,
    pub search_results: usize,
    pub total_commands: usize,
    pub timeout_ms: u64,
}

impl Default for HelpLimits {
    fn default() -> Self {
        Self {
            command_count: 64,
            command_depth: 8,
            document_bytes: 1_048_576,
            search_results: 256,
            total_commands: 1_024,
            timeout_ms: 2_000,
        }
    }
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

pub fn load_resolved(extension_path: Option<&Path>) -> Result<CheckBundleManifest, String> {
    let mut core = parse(CORE_CHECK_BUNDLE, "built-in core check bundle")?;
    validate(&core)?;
    if let Some(path) = extension_path {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("could not read check bundle {}: {error}", path.display()))?;
        let extension = parse(&text, &path.display().to_string())?;
        resolve(core, extension)
    } else {
        core.resolved_check_bundles = vec![core.check_bundle.clone()];
        Ok(core)
    }
}

pub fn parse(text: &str, source: &str) -> Result<CheckBundleManifest, String> {
    toml::from_str(text).map_err(|error| format!("invalid check bundle {source}: {error}"))
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
    let mut hierarchical_help_limits: Option<&HelpLimits> = None;
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
            EvaluationMethod::Deterministic => {
                if check.checker.is_none() || check.skill.is_some() || check.evidence.is_some() {
                    return Err(format!(
                        "deterministic check {} must have one checker and no agent fields",
                        check.id
                    ));
                }
                if let Some(CheckerDefinition::HierarchicalHelp { limits, .. }) = &check.checker
                    && (limits.command_count == 0
                        || limits.command_depth == 0
                        || limits.document_bytes == 0
                        || limits.search_results == 0
                        || limits.total_commands == 0
                        || limits.timeout_ms == 0)
                {
                    return Err(format!(
                        "hierarchical-help checker {} has a zero limit",
                        check.id
                    ));
                }
                if let Some(CheckerDefinition::HierarchicalHelp { limits, .. }) = &check.checker {
                    if hierarchical_help_limits.is_some_and(|expected| expected != limits) {
                        return Err(format!(
                            "hierarchical-help checkers in {} must use the same limits",
                            bundle.check_bundle.name
                        ));
                    }
                    hierarchical_help_limits = Some(limits);
                }
            }
            EvaluationMethod::AiAgent => {
                if check.checker.is_some() || check.skill.is_none() || check.evidence.is_none() {
                    return Err(format!(
                        "AI-agent check {} must have a skill and evidence invocation",
                        check.id
                    ));
                }
                let skill = check.skill.as_ref().expect("checked above");
                validate_name("skill", &skill.name)?;
                validate_version("skill", &skill.version)?;
            }
        }
    }
    Ok(())
}

pub fn resolve(
    mut inherited_bundle: CheckBundleManifest,
    extension: CheckBundleManifest,
) -> Result<CheckBundleManifest, String> {
    validate(&extension)?;
    let parent_name = extension
        .extends
        .as_deref()
        .ok_or_else(|| {
            format!(
                "extension check bundle {} must declare extends",
                extension.check_bundle.name
            )
        })?
        .to_owned();
    if extension.check_bundle.name == inherited_bundle.check_bundle.name {
        return Err(format!(
            "extension check bundle {} conflicts with the inherited check bundle identity",
            extension.check_bundle.name
        ));
    }
    let parent_installed = parent_name == inherited_bundle.check_bundle.name
        || inherited_bundle
            .resolved_check_bundles
            .iter()
            .any(|identity| identity.name == parent_name);
    if !parent_installed {
        return Err(format!(
            "extension check bundle {} extends unavailable check bundle {:?}",
            extension.check_bundle.name, parent_name
        ));
    }
    if let Some(check) = extension.exclude.first() {
        return Err(format!(
            "extension check bundle {} cannot exclude inherited check {check}",
            extension.check_bundle.name
        ));
    }

    let mut inherited: HashSet<String> = inherited_bundle
        .checks
        .iter()
        .map(|check| check.id.clone())
        .collect();
    for check in &extension.checks {
        if !inherited.insert(check.id.clone()) {
            return Err(format!(
                "extension check {} conflicts with an inherited check",
                check.id
            ));
        }
    }
    for strengthening in &extension.strengthen {
        let check = inherited_bundle
            .checks
            .iter_mut()
            .find(|check| check.id == strengthening.check)
            .ok_or_else(|| {
                format!(
                    "extension tries to strengthen unknown inherited check {}",
                    strengthening.check
                )
            })?;
        if strengthening.severity.rank() < check.severity.rank() {
            return Err(format!(
                "extension weakens inherited check {} from {:?} to {:?}",
                check.id, check.severity, strengthening.severity
            ));
        }
        check.severity = strengthening.severity;
    }

    if inherited_bundle.resolved_check_bundles.is_empty() {
        inherited_bundle
            .resolved_check_bundles
            .push(inherited_bundle.check_bundle.clone());
    }
    inherited_bundle
        .resolved_check_bundles
        .push(extension.check_bundle.clone());
    inherited_bundle.checks.extend(extension.checks);
    Ok(inherited_bundle)
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
        assert_eq!(bundle.checks.len(), 10);
    }

    #[test]
    fn rejects_unknown_check_type() {
        let invalid =
            CORE_CHECK_BUNDLE.replacen("type = \"any-invocation\"", "type = \"shell-program\"", 1);
        let error = parse(&invalid, "test").unwrap_err();
        assert!(error.contains("unknown variant") || error.contains("shell-program"));
    }

    #[test]
    fn rejects_unknown_hierarchical_help_behavior() {
        let invalid = r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "invalid"
version = "1.0.0"
[[checks]]
id = "invalid/help/example"
title = "Invalid"
severity = "error"
evaluation_method = "deterministic"
[checks.checker]
type = "hierarchical-help"
behavior = "invented"
"#;
        assert!(parse(invalid, "test").unwrap_err().contains("invented"));
    }

    #[test]
    fn rejects_zero_hierarchical_help_limit() {
        let invalid = r#"
format_version = 1
extends = "clilint"
[check_bundle]
name = "invalid"
version = "1.0.0"
[[checks]]
id = "invalid/help/example"
title = "Invalid"
severity = "error"
evaluation_method = "deterministic"
[checks.checker]
type = "hierarchical-help"
behavior = "outline"
command_count = 0
"#;
        let bundle = parse(invalid, "test").unwrap();
        assert!(validate(&bundle).unwrap_err().contains("zero limit"));
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
            .find(|check| check.evaluation_method == EvaluationMethod::AiAgent)
            .unwrap();
        check.skill.as_mut().unwrap().name = "bad skill".into();
        assert!(validate(&bundle).unwrap_err().contains("invalid skill"));
    }

    fn extension() -> CheckBundleManifest {
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
severity = "warn"
evaluation_method = "deterministic"
[checks.checker]
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
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let core_count = core.checks.len();
        let resolved = resolve(core, extension()).unwrap();
        assert_eq!(resolved.checks.len(), core_count + 1);
        assert!(
            resolved
                .checks
                .iter()
                .any(|check| check.id == "team/help/team-flag")
        );
    }

    #[test]
    fn rejects_weakening() {
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let mut extension = extension();
        extension.strengthen.push(Strengthening {
            check: "clilint/basics/success-exit".into(),
            severity: Severity::Warn,
        });
        assert!(resolve(core, extension).unwrap_err().contains("weakens"));
    }

    #[test]
    fn rejects_exclusion() {
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let mut extension = extension();
        extension.exclude.push("clilint/basics/success-exit".into());
        assert!(
            resolve(core, extension)
                .unwrap_err()
                .contains("cannot exclude")
        );
    }

    #[test]
    fn rejects_check_bundle_identity_conflict() {
        let core = parse(CORE_CHECK_BUNDLE, "test").unwrap();
        let mut extension = extension();
        extension.check_bundle.name = "clilint".into();
        extension.checks[0].id = "clilint/help/team-flag".into();
        assert!(resolve(core, extension).unwrap_err().contains("conflicts"));
    }
}
