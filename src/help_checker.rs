use std::collections::{HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{
    check_bundle::{HelpLimits, HierarchicalHelpBehavior},
    model::{InvocationSpec, Observation},
    runner::Runner,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelpOverview {
    format_version: u32,
    command_path: Vec<String>,
    programmatic: bool,
    child_commands: Vec<ChildCommand>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChildCommand {
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Outline {
    format_version: u32,
    command_path: Vec<String>,
    programmatic: bool,
    headings: Vec<Heading>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Heading {
    level: u8,
    title: String,
    section: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SectionResponse {
    format_version: u32,
    command_path: Vec<String>,
    programmatic: bool,
    sections: Vec<Section>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Section {
    level: u8,
    title: String,
    section: String,
    content: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchResponse {
    format_version: u32,
    command_path: Vec<String>,
    programmatic: bool,
    results: Vec<SearchResult>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchResult {
    command_path: Vec<String>,
    section: String,
    title: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct HelpObservation {
    pub command_path: Vec<String>,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_section: Option<String>,
    pub observation: Observation,
}

#[derive(Clone, Debug, Serialize)]
pub struct HelpValidationFailure {
    pub behavior: HierarchicalHelpBehavior,
    pub command_path: Vec<String>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct HelpEvidence {
    pub command_paths: Vec<Vec<String>>,
    pub commands_run: Vec<HelpObservation>,
    pub validation_failures: Vec<HelpValidationFailure>,
}

pub struct HelpContext {
    evidence: HelpEvidence,
}

impl HelpContext {
    pub fn collect(runner: &mut Runner, limits: &HelpLimits) -> Self {
        let mut collector = Collector {
            runner,
            limits,
            observations: Vec::new(),
            failures: Vec::new(),
            command_paths: Vec::new(),
            total_commands: 0,
            tool_version: None,
        };
        collector.collect();
        Self {
            evidence: HelpEvidence {
                command_paths: collector.command_paths,
                commands_run: collector.observations,
                validation_failures: collector.failures,
            },
        }
    }

    pub fn result(&self, behavior: HierarchicalHelpBehavior) -> (bool, String, serde_json::Value) {
        let failures = self
            .evidence
            .validation_failures
            .iter()
            .filter(|failure| failure.behavior == behavior)
            .collect::<Vec<_>>();
        let passed = failures.is_empty();
        let detail = if passed {
            "hierarchical help check passed".to_owned()
        } else {
            failures
                .iter()
                .map(|failure| format!("{} at {:?}", failure.message, failure.command_path))
                .collect::<Vec<_>>()
                .join("; ")
        };
        let relevant_commands = self
            .evidence
            .commands_run
            .iter()
            .filter(|observation| {
                behavior == HierarchicalHelpBehavior::NonInteractiveOutput
                    || operation_behavior(&observation.operation) == behavior
            })
            .collect::<Vec<_>>();
        let evidence = serde_json::json!({
            "command_paths": self.evidence.command_paths,
            "commands_run": relevant_commands,
            "validation_failures": failures,
        });
        (passed, detail, evidence)
    }
}

struct Collector<'a> {
    runner: &'a mut Runner,
    limits: &'a HelpLimits,
    observations: Vec<HelpObservation>,
    failures: Vec<HelpValidationFailure>,
    command_paths: Vec<Vec<String>>,
    total_commands: usize,
    tool_version: Option<String>,
}

impl Collector<'_> {
    fn collect(&mut self) {
        if let Some(observation) = self.run(
            &[],
            &["--version"],
            "web-version",
            HierarchicalHelpBehavior::WebView,
            None,
        ) {
            self.tool_version = observation
                .stdout
                .split_whitespace()
                .find(|value| {
                    value.as_bytes().windows(3).any(|window| {
                        window[0].is_ascii_digit()
                            && window[1] == b'.'
                            && window[2].is_ascii_digit()
                    })
                })
                .map(|value| value.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '.'))
                .map(ToOwned::to_owned);
            if self.tool_version.is_none() {
                self.fail(
                    HierarchicalHelpBehavior::WebView,
                    &[],
                    "--version did not contain a version number",
                );
            }
        }
        let mut pending = VecDeque::from([Vec::new()]);
        let mut seen = HashSet::new();
        while let Some(path) = pending.pop_front() {
            if !seen.insert(path.clone()) {
                self.fail(
                    HierarchicalHelpBehavior::CommandDiscovery,
                    &path,
                    "command hierarchy contains a duplicate path",
                );
                continue;
            }
            if path.len() > self.limits.command_depth {
                self.fail(
                    HierarchicalHelpBehavior::CommandDiscovery,
                    &path,
                    "command depth limit exceeded",
                );
                continue;
            }
            if seen.len() > self.limits.command_count {
                self.fail(
                    HierarchicalHelpBehavior::CommandDiscovery,
                    &path,
                    "command count limit exceeded",
                );
                break;
            }
            self.command_paths.push(path.clone());

            if let Some(overview) = self.json::<HelpOverview>(
                &path,
                &["help", "--format", "json"],
                "overview",
                HierarchicalHelpBehavior::CommandDiscovery,
                None,
            ) {
                self.validate_header(
                    overview.format_version,
                    &overview.command_path,
                    overview.programmatic,
                    &path,
                    false,
                    HierarchicalHelpBehavior::CommandDiscovery,
                );
                let mut child_names = HashSet::new();
                for child in overview.child_commands {
                    if child.name.is_empty()
                        || child.name.chars().any(char::is_whitespace)
                        || !child_names.insert(child.name.clone())
                    {
                        self.fail(
                            HierarchicalHelpBehavior::CommandDiscovery,
                            &path,
                            "child command names must be non-empty, single tokens, and unique",
                        );
                        continue;
                    }
                    let mut child_path = path.clone();
                    child_path.push(child.name);
                    pending.push_back(child_path);
                }
            }
            self.collect_path(&path);
        }
    }

    fn collect_path(&mut self, path: &[String]) {
        let help = self.run(
            path,
            &["help"],
            "shared-help",
            HierarchicalHelpBehavior::SharedHelp,
            None,
        );
        let option_help = self.run(
            path,
            &["--help"],
            "help-option",
            HierarchicalHelpBehavior::Availability,
            None,
        );
        if let (Some(help), Some(option_help)) = (&help, &option_help) {
            if help.exit_status != Some(0) || option_help.exit_status != Some(0) {
                self.fail(
                    HierarchicalHelpBehavior::Availability,
                    path,
                    "help and --help must exit successfully",
                );
            }
            if help.stdout.trim().is_empty() || help.stdout != option_help.stdout {
                self.fail(
                    HierarchicalHelpBehavior::SharedHelp,
                    path,
                    "help must contain the same non-empty documentation as --help",
                );
            }
        }

        let programmatic = self.run(
            path,
            &["help", "--programmatic"],
            "programmatic-help",
            HierarchicalHelpBehavior::ProgrammaticGuidance,
            None,
        );
        if let (Some(help), Some(programmatic)) = (&help, &programmatic)
            && (!programmatic.stdout.contains(&help.stdout)
                || !programmatic.stdout.to_lowercase().contains("json")
                || !programmatic.stdout.to_lowercase().contains("section"))
        {
            self.fail(
                HierarchicalHelpBehavior::ProgrammaticGuidance,
                path,
                "--programmatic must retain default help and add JSON and section guidance",
            );
        }
        if let Some(overview) = self.json::<HelpOverview>(
            path,
            &["help", "--programmatic", "--format", "json"],
            "programmatic-overview",
            HierarchicalHelpBehavior::ProgrammaticGuidance,
            None,
        ) {
            self.validate_header(
                overview.format_version,
                &overview.command_path,
                overview.programmatic,
                path,
                true,
                HierarchicalHelpBehavior::ProgrammaticGuidance,
            );
        }

        self.collect_outline(path, false);
        self.collect_outline(path, true);
        self.collect_search(path, false);
        self.collect_search(path, true);

        let local_view = self.run(
            path,
            &["help", "view"],
            "local-view",
            HierarchicalHelpBehavior::LocalView,
            None,
        );
        if local_view
            .as_ref()
            .is_none_or(|value| value.exit_status != Some(0) || value.stdout.trim().is_empty())
        {
            self.fail(
                HierarchicalHelpBehavior::LocalView,
                path,
                "non-interactive local view must write the document and exit successfully",
            );
        }
        let programmatic_local_view = self.run(
            path,
            &["help", "view", "--programmatic"],
            "local-view",
            HierarchicalHelpBehavior::LocalView,
            None,
        );
        if let (Some(default), Some(programmatic)) = (&local_view, &programmatic_local_view)
            && !programmatic.stdout.contains(&default.stdout)
        {
            self.fail(
                HierarchicalHelpBehavior::LocalView,
                path,
                "programmatic local view must retain the default document",
            );
        }

        let web_view = self.run(
            path,
            &["help", "view", "--web"],
            "web-view",
            HierarchicalHelpBehavior::WebView,
            None,
        );
        if web_view.as_ref().is_none_or(|value| {
            value.exit_status != Some(0)
                || !(value.stdout.trim().starts_with("https://")
                    || value.stdout.trim().starts_with("http://"))
                || self
                    .tool_version
                    .as_ref()
                    .is_none_or(|version| !value.stdout.contains(version))
        }) {
            self.fail(
                HierarchicalHelpBehavior::WebView,
                path,
                "non-interactive web view must write an HTTP URL and exit successfully",
            );
        }
        let programmatic_web_view = self.run(
            path,
            &["help", "view", "--web", "--programmatic"],
            "web-view",
            HierarchicalHelpBehavior::WebView,
            None,
        );
        if programmatic_web_view.as_ref().is_none_or(|value| {
            value.exit_status != Some(0)
                || !(value.stdout.trim().starts_with("https://")
                    || value.stdout.trim().starts_with("http://"))
                || self
                    .tool_version
                    .as_ref()
                    .is_none_or(|version| !value.stdout.contains(version))
        }) {
            self.fail(
                HierarchicalHelpBehavior::WebView,
                path,
                "programmatic web view must write the versioned HTTP URL",
            );
        }
    }

    fn collect_outline(&mut self, path: &[String], programmatic: bool) {
        let mut args = vec!["help", "outline"];
        if programmatic {
            args.push("--programmatic");
        }
        args.extend(["--format", "json"]);
        let operation = if programmatic {
            "programmatic-outline"
        } else {
            "outline"
        };
        let Some(outline) = self.json::<Outline>(
            path,
            &args,
            operation,
            HierarchicalHelpBehavior::Outline,
            None,
        ) else {
            return;
        };
        self.validate_header(
            outline.format_version,
            &outline.command_path,
            outline.programmatic,
            path,
            programmatic,
            HierarchicalHelpBehavior::Outline,
        );
        let mut headings = Vec::new();
        for heading in outline.headings {
            if !(1..=6).contains(&heading.level)
                || heading.title.trim().is_empty()
                || heading.section.trim().is_empty()
            {
                self.fail(
                    HierarchicalHelpBehavior::Outline,
                    path,
                    "outline heading has an invalid level, title, or section",
                );
                continue;
            }
            headings.push(heading);
        }
        for (index, heading) in headings.iter().enumerate() {
            let expected_matches = headings
                .iter()
                .filter(|candidate| candidate.section == heading.section)
                .count();
            self.collect_section(
                path,
                &heading.section,
                programmatic,
                false,
                std::slice::from_ref(&heading.section),
                expected_matches,
            );
            let mut recursive_sections = vec![heading.section.clone()];
            for descendant in headings.iter().skip(index + 1) {
                if descendant.level <= heading.level {
                    break;
                }
                recursive_sections.push(descendant.section.clone());
            }
            self.collect_section(
                path,
                &heading.section,
                programmatic,
                true,
                &recursive_sections,
                expected_matches,
            );
        }
        self.collect_unknown_section(path, programmatic);
        self.collect_filtered_outline(path, programmatic, false);
        self.collect_filtered_outline(path, programmatic, true);
    }

    fn collect_filtered_outline(
        &mut self,
        path: &[String],
        programmatic: bool,
        through_level: bool,
    ) {
        let mut args = vec!["help", "outline"];
        if programmatic {
            args.push("--programmatic");
        }
        if through_level {
            args.extend(["--max-level", "2"]);
        } else {
            args.extend(["--level", "2"]);
        }
        args.extend(["--format", "json"]);
        let Some(outline) = self.json::<Outline>(
            path,
            &args,
            "outline",
            HierarchicalHelpBehavior::Outline,
            None,
        ) else {
            return;
        };
        self.validate_header(
            outline.format_version,
            &outline.command_path,
            outline.programmatic,
            path,
            programmatic,
            HierarchicalHelpBehavior::Outline,
        );
        let invalid = outline.headings.iter().any(|heading| {
            if through_level {
                !(1..=2).contains(&heading.level)
            } else {
                heading.level != 2
            }
        });
        if invalid {
            self.fail(
                HierarchicalHelpBehavior::Outline,
                path,
                if through_level {
                    "--max-level returned a heading below the requested depth"
                } else {
                    "--level returned a different heading level"
                },
            );
        }
    }

    fn collect_section(
        &mut self,
        path: &[String],
        requested: &str,
        programmatic: bool,
        recursive: bool,
        allowed_sections: &[String],
        expected_matches: usize,
    ) {
        let mut owned = vec![
            "help".to_owned(),
            "section".to_owned(),
            requested.to_owned(),
        ];
        if recursive {
            owned.push("--recursive".to_owned());
        }
        if programmatic {
            owned.push("--programmatic".to_owned());
        }
        owned.extend(["--format".to_owned(), "json".to_owned()]);
        let args = owned.iter().map(String::as_str).collect::<Vec<_>>();
        let Some(response) = self.json::<SectionResponse>(
            path,
            &args,
            if recursive {
                "recursive-section"
            } else {
                "section"
            },
            HierarchicalHelpBehavior::SectionRetrieval,
            Some(requested.to_owned()),
        ) else {
            return;
        };
        self.validate_header(
            response.format_version,
            &response.command_path,
            response.programmatic,
            path,
            programmatic,
            HierarchicalHelpBehavior::SectionRetrieval,
        );
        if response.sections.is_empty()
            || response.sections.len() < expected_matches
            || !response
                .sections
                .iter()
                .any(|section| section.section == requested)
            || allowed_sections.iter().any(|allowed| {
                !response
                    .sections
                    .iter()
                    .any(|section| section.section == *allowed)
            })
            || response.sections.iter().any(|section| {
                !allowed_sections.contains(&section.section)
                    || !(1..=6).contains(&section.level)
                    || section.title.trim().is_empty()
                    || section.content.len() > self.limits.document_bytes
            })
        {
            self.fail(
                HierarchicalHelpBehavior::SectionRetrieval,
                path,
                "section result must return the requested valid section",
            );
        }
    }

    fn collect_unknown_section(&mut self, path: &[String], programmatic: bool) {
        let unknown = "clilint-unknown-section-7f3d";
        let mut args = vec!["help", "section", unknown];
        if programmatic {
            args.push("--programmatic");
        }
        args.extend(["--format", "json"]);
        let response = self.run(
            path,
            &args,
            "section",
            HierarchicalHelpBehavior::SectionRetrieval,
            Some(unknown.to_owned()),
        );
        if response.as_ref().is_none_or(|observation| {
            observation.exit_status.is_none_or(|status| status == 0)
                || !observation.stderr.to_lowercase().contains("section")
        }) {
            self.fail(
                HierarchicalHelpBehavior::SectionRetrieval,
                path,
                "an unknown section must exit non-zero and identify the section argument",
            );
        }
    }

    fn collect_search(&mut self, path: &[String], programmatic: bool) {
        let query = if programmatic {
            "programmatic"
        } else {
            "permissions"
        };
        let mut args = vec!["help", "search", query];
        if programmatic {
            args.push("--programmatic");
        }
        args.extend(["--format", "json"]);
        let operation = if programmatic {
            "programmatic-search"
        } else {
            "search"
        };
        let Some(response) = self.json::<SearchResponse>(
            path,
            &args,
            operation,
            HierarchicalHelpBehavior::Search,
            None,
        ) else {
            return;
        };
        self.validate_header(
            response.format_version,
            &response.command_path,
            response.programmatic,
            path,
            programmatic,
            HierarchicalHelpBehavior::Search,
        );
        let invalid = response.results.len() > self.limits.search_results
            || response.results.iter().any(|result| {
                !result.command_path.starts_with(path)
                    || result.section.trim().is_empty()
                    || result.title.trim().is_empty()
            });
        if invalid {
            self.fail(
                HierarchicalHelpBehavior::Search,
                path,
                "search returned an invalid or excessive result",
            );
            return;
        }
        for result in response.results {
            let mut owned = result.command_path.clone();
            owned.extend([
                "help".to_owned(),
                "section".to_owned(),
                result.section.clone(),
            ]);
            if programmatic {
                owned.push("--programmatic".to_owned());
            }
            owned.extend(["--format".to_owned(), "json".to_owned()]);
            let args = owned
                .iter()
                .skip(result.command_path.len())
                .map(String::as_str)
                .collect::<Vec<_>>();
            let Some(section_response) = self.json::<SectionResponse>(
                &result.command_path,
                &args,
                "search-section",
                HierarchicalHelpBehavior::Search,
                Some(result.section.clone()),
            ) else {
                continue;
            };
            self.validate_header(
                section_response.format_version,
                &section_response.command_path,
                section_response.programmatic,
                &result.command_path,
                programmatic,
                HierarchicalHelpBehavior::Search,
            );
            if !section_response
                .sections
                .iter()
                .any(|section| section.section == result.section)
            {
                self.fail(
                    HierarchicalHelpBehavior::Search,
                    &result.command_path,
                    "search section could not be retrieved",
                );
            }
        }
    }

    fn json<T: for<'de> Deserialize<'de>>(
        &mut self,
        path: &[String],
        args: &[&str],
        operation: &str,
        behavior: HierarchicalHelpBehavior,
        requested_section: Option<String>,
    ) -> Option<T> {
        let observation = self.run(path, args, operation, behavior, requested_section)?;
        if observation.exit_status != Some(0) {
            self.fail(behavior, path, "help JSON command exited unsuccessfully");
            return None;
        }
        if observation.stdout.len() > self.limits.document_bytes {
            self.fail(behavior, path, "captured document byte limit exceeded");
            return None;
        }
        match serde_json::from_str(&observation.stdout) {
            Ok(value) => Some(value),
            Err(error) => {
                self.fail(
                    behavior,
                    path,
                    &format!("invalid JSON help response: {error}"),
                );
                None
            }
        }
    }

    fn run(
        &mut self,
        path: &[String],
        suffix: &[&str],
        operation: &str,
        behavior: HierarchicalHelpBehavior,
        requested_section: Option<String>,
    ) -> Option<Observation> {
        if self.total_commands >= self.limits.total_commands {
            self.fail(behavior, path, "total help command limit exceeded");
            return None;
        }
        self.total_commands += 1;
        let mut args = path.to_vec();
        args.extend(suffix.iter().map(|value| (*value).to_owned()));
        let spec = InvocationSpec {
            args,
            env: [
                ("CLILINT_OFFLINE".to_owned(), "1".to_owned()),
                ("NO_COLOR".to_owned(), "1".to_owned()),
            ]
            .into_iter()
            .collect(),
            stdin: None,
            timeout_ms: Some(self.limits.timeout_ms),
        };
        match self.runner.run_bounded(&spec, self.limits.document_bytes) {
            Ok(observation) => {
                if observation.timed_out {
                    self.fail(behavior, path, "help command timed out");
                }
                if observation
                    .stdout
                    .len()
                    .saturating_add(observation.stderr.len())
                    > self.limits.document_bytes
                {
                    self.fail(behavior, path, "captured document byte limit exceeded");
                }
                if observation.has_ansi {
                    self.fail(
                        HierarchicalHelpBehavior::NonInteractiveOutput,
                        path,
                        "non-interactive help output contains terminal control sequences",
                    );
                }
                self.observations.push(HelpObservation {
                    command_path: path.to_vec(),
                    operation: operation.to_owned(),
                    requested_section,
                    observation: observation.clone(),
                });
                Some(observation)
            }
            Err(error) => {
                self.fail(behavior, path, &error);
                None
            }
        }
    }

    fn validate_header(
        &mut self,
        format_version: u32,
        actual_path: &[String],
        actual_programmatic: bool,
        expected_path: &[String],
        expected_programmatic: bool,
        behavior: HierarchicalHelpBehavior,
    ) {
        if format_version != 1
            || actual_path != expected_path
            || actual_programmatic != expected_programmatic
        {
            self.fail(
                behavior,
                expected_path,
                "JSON format version, command path, or programmatic marker is invalid",
            );
        }
    }

    fn fail(&mut self, behavior: HierarchicalHelpBehavior, path: &[String], message: &str) {
        self.failures.push(HelpValidationFailure {
            behavior,
            command_path: path.to_vec(),
            message: message.to_owned(),
        });
    }
}

fn operation_behavior(operation: &str) -> HierarchicalHelpBehavior {
    match operation {
        "overview" => HierarchicalHelpBehavior::CommandDiscovery,
        "outline" | "programmatic-outline" => HierarchicalHelpBehavior::Outline,
        "section" | "recursive-section" => HierarchicalHelpBehavior::SectionRetrieval,
        "shared-help" => HierarchicalHelpBehavior::SharedHelp,
        "programmatic-help" | "programmatic-overview" => {
            HierarchicalHelpBehavior::ProgrammaticGuidance
        }
        "search" | "programmatic-search" | "search-section" => HierarchicalHelpBehavior::Search,
        "local-view" => HierarchicalHelpBehavior::LocalView,
        "web-view" | "web-version" => HierarchicalHelpBehavior::WebView,
        _ => HierarchicalHelpBehavior::Availability,
    }
}
