use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::check_bundle::{self, CheckBundleManifest};

const CONFIG_PATH: &str = ".clilint/config.toml";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectConfig {
    pub check_bundles: BTreeMap<String, BundleSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "source", rename_all = "lowercase", deny_unknown_fields)]
pub enum BundleSource {
    Local { path: PathBuf },
}

#[derive(Clone, Debug, Serialize)]
pub struct BundleStatus {
    pub name: String,
    pub source: BundleSource,
    pub installed: bool,
}

pub fn load_config(root: &Path) -> Result<ProjectConfig, String> {
    let path = root.join(CONFIG_PATH);
    if !path.exists() {
        return Ok(ProjectConfig::default());
    }
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    toml::from_str(&text).map_err(|error| format!("invalid {}: {error}", path.display()))
}

pub fn save_config(root: &Path, config: &ProjectConfig) -> Result<(), String> {
    let path = root.join(CONFIG_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let text = toml::to_string_pretty(config).map_err(|error| error.to_string())?;
    fs::write(&path, text).map_err(|error| format!("could not write {}: {error}", path.display()))
}

pub fn load_for_check(
    root: &Path,
    direct_path: Option<&Path>,
) -> Result<CheckBundleManifest, String> {
    let config = load_config(root)?;
    load_from_config(root, &config, direct_path)
}

fn load_from_config(
    root: &Path,
    config: &ProjectConfig,
    direct_path: Option<&Path>,
) -> Result<CheckBundleManifest, String> {
    let mut manifests = Vec::new();
    for (name, source) in &config.check_bundles {
        let BundleSource::Local { path } = source;
        let manifest = load_local(root, path)?;
        if manifest.check_bundle.name != *name {
            return Err(format!(
                "declared check bundle {name} loaded bundle {}",
                manifest.check_bundle.name
            ));
        }
        manifests.push(manifest);
    }
    if let Some(path) = direct_path {
        manifests.push(load_local(root, path)?);
    }
    resolve_manifests(manifests)
}

fn resolve_manifests(
    mut manifests: Vec<CheckBundleManifest>,
) -> Result<CheckBundleManifest, String> {
    let mut resolved = check_bundle::core()?;
    let mut names = HashSet::new();
    for manifest in &manifests {
        if !names.insert(manifest.check_bundle.name.clone()) {
            return Err(format!(
                "duplicate check bundle declaration {}",
                manifest.check_bundle.name
            ));
        }
    }
    let mut installed = HashSet::from([resolved.check_bundle.name.clone()]);
    while !manifests.is_empty() {
        let next = manifests
            .iter()
            .position(|bundle| {
                bundle
                    .extends
                    .as_ref()
                    .is_some_and(|parent| installed.contains(parent))
            })
            .ok_or_else(|| {
                let names = manifests
                    .iter()
                    .map(|bundle| bundle.check_bundle.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("check bundle dependencies contain a missing name or cycle: {names}")
            })?;
        let bundle_to_add = manifests.remove(next);
        let name = bundle_to_add.check_bundle.name.clone();
        resolved = check_bundle::resolve(resolved, bundle_to_add)?;
        installed.insert(name);
    }
    Ok(resolved)
}

fn load_local(root: &Path, path: &Path) -> Result<CheckBundleManifest, String> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    load_bundle_at(&path)
}

fn load_bundle_at(path: &Path) -> Result<CheckBundleManifest, String> {
    let mut manifest_path = path.to_path_buf();
    if manifest_path.is_dir() {
        manifest_path.push("clilint.toml");
    }
    let text = fs::read_to_string(&manifest_path).map_err(|error| {
        format!(
            "could not read check bundle {}: {error}",
            manifest_path.display()
        )
    })?;
    let mut manifest = check_bundle::parse(&text, &manifest_path.display().to_string())?;
    let bundle_root = manifest_path
        .parent()
        .ok_or_else(|| {
            format!(
                "check bundle manifest {} has no parent directory",
                manifest_path.display()
            )
        })?
        .to_path_buf();
    for check in &mut manifest.checks {
        check.bundle_root = Some(bundle_root.clone());
    }
    check_bundle::validate(&manifest)?;
    Ok(manifest)
}

pub fn install(root: &Path, source: &Path) -> Result<Vec<BundleStatus>, String> {
    let manifest = load_local(root, source)?;
    let mut config = load_config(root)?;
    let path = if source.is_absolute() {
        relative_path(root, source)?
    } else {
        source.to_path_buf()
    };
    config
        .check_bundles
        .insert(manifest.check_bundle.name, BundleSource::Local { path });
    let _ = load_from_config(root, &config, None)?;
    save_config(root, &config)?;
    list(root)
}

pub fn list(root: &Path) -> Result<Vec<BundleStatus>, String> {
    load_config(root)?
        .check_bundles
        .into_iter()
        .map(|(name, source)| {
            let BundleSource::Local { path } = &source;
            let installed = root.join(path).exists();
            Ok(BundleStatus {
                name,
                source,
                installed,
            })
        })
        .collect()
}

pub fn remove(root: &Path, name: &str) -> Result<(), String> {
    let mut config = load_config(root)?;
    if config.check_bundles.remove(name).is_none() {
        return Err(format!("unknown check bundle {name}"));
    }
    let _ = load_from_config(root, &config, None)?;
    save_config(root, &config)
}

fn relative_path(from: &Path, to: &Path) -> Result<PathBuf, String> {
    let from = normalize_absolute(from)?;
    let to = normalize_absolute(to)?;
    let from_components = from.components().collect::<Vec<_>>();
    let to_components = to.components().collect::<Vec<_>>();
    let common = from_components
        .iter()
        .zip(&to_components)
        .take_while(|(left, right)| left == right)
        .count();
    if common == 0 {
        return Err(format!(
            "cannot make {} relative to {}",
            to.display(),
            from.display()
        ));
    }
    let mut result = PathBuf::new();
    for _ in common..from_components.len() {
        result.push("..");
    }
    for component in &to_components[common..] {
        result.push(component.as_os_str());
    }
    Ok(if result.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        result
    })
}

fn normalize_absolute(path: &Path) -> Result<PathBuf, String> {
    if !path.is_absolute() {
        return Err(format!("expected an absolute path, got {}", path.display()));
    }
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !result.pop() {
                    return Err(format!("path escapes its root: {}", path.display()));
                }
            }
            other => result.push(other.as_os_str()),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn makes_sibling_path_relative() {
        let path = relative_path(
            Path::new("/work/project"),
            Path::new("/work/check-bundles/example"),
        )
        .unwrap();
        assert_eq!(path, Path::new("../check-bundles/example"));
    }

    #[test]
    fn keeps_descendant_path_relative() {
        let path = relative_path(
            Path::new("/work/project"),
            Path::new("/work/project/checks"),
        )
        .unwrap();
        assert_eq!(path, Path::new("checks"));
    }

    #[test]
    fn failed_install_leaves_project_configuration_unchanged() {
        let directory = tempdir().unwrap();
        let bundle = directory.path().join("bundle");
        write_test_bundle(&bundle, "broken", "missing");

        let error = install(directory.path(), &bundle).unwrap_err();

        assert!(error.contains("missing name or cycle"));
        assert!(
            load_config(directory.path())
                .unwrap()
                .check_bundles
                .is_empty()
        );
        assert!(!directory.path().join(CONFIG_PATH).exists());
    }

    #[test]
    fn removing_a_bundle_used_by_another_bundle_leaves_both_installed() {
        let directory = tempdir().unwrap();
        let parent = directory.path().join("parent");
        let child = directory.path().join("child");
        write_test_bundle(&parent, "parent", "clilint");
        write_test_bundle(&child, "child", "parent");
        install(directory.path(), &parent).unwrap();
        install(directory.path(), &child).unwrap();

        let error = remove(directory.path(), "parent").unwrap_err();

        assert!(error.contains("missing name or cycle"));
        let config = load_config(directory.path()).unwrap();
        assert!(config.check_bundles.contains_key("parent"));
        assert!(config.check_bundles.contains_key("child"));
    }

    fn write_test_bundle(path: &Path, name: &str, extends: &str) {
        fs::create_dir_all(path).unwrap();
        fs::write(
            path.join("clilint.toml"),
            format!(
                r#"
format_version = 1
extends = "{extends}"

[check_bundle]
name = "{name}"
version = "1.0.0"

[[checks]]
id = "{name}/example"
title = "Example"
evaluation_method = "mechanistic"

[checks.checker]
type = "cli"
command = ["checker"]
"#
            ),
        )
        .unwrap();
    }
}
