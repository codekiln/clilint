use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::check_bundle::{self, CheckBundleManifest};

const CONFIG_PATH: &str = ".clilint/config.toml";
const LOCK_PATH: &str = ".clilint/lock.toml";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectConfig {
    pub check_bundles: BTreeMap<String, BundleSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "source", rename_all = "lowercase", deny_unknown_fields)]
pub enum BundleSource {
    Local {
        path: PathBuf,
    },
    Git {
        url: String,
        #[serde(default, rename = "ref")]
        ref_name: Option<String>,
        #[serde(default)]
        path: Option<PathBuf>,
    },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectLock {
    pub check_bundles: BTreeMap<String, LockedBundle>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LockedBundle {
    pub url: String,
    pub requested_ref: String,
    pub commit: String,
    #[serde(default)]
    pub path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LoadOptions {
    pub offline: bool,
    pub locked: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct BundleStatus {
    pub name: String,
    pub source: BundleSource,
    pub requested_ref: Option<String>,
    pub resolved_commit: Option<String>,
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

pub fn load_lock(root: &Path) -> Result<Option<ProjectLock>, String> {
    let path = root.join(LOCK_PATH);
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    toml::from_str(&text)
        .map(Some)
        .map_err(|error| format!("invalid {}: {error}", path.display()))
}

pub fn save_lock(root: &Path, lock: &ProjectLock) -> Result<(), String> {
    let path = root.join(LOCK_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let text = toml::to_string_pretty(lock).map_err(|error| error.to_string())?;
    fs::write(&path, text).map_err(|error| format!("could not write {}: {error}", path.display()))
}

pub fn load_for_check(
    root: &Path,
    direct_path: Option<&Path>,
    options: LoadOptions,
) -> Result<CheckBundleManifest, String> {
    let config = load_config(root)?;
    let lock = load_lock(root)?;
    if options.locked && lock.is_none() && !config.check_bundles.is_empty() {
        return Err(format!(
            "{} is required by --locked; run `clilint bundle lock`",
            root.join(LOCK_PATH).display()
        ));
    }
    validate_lock(&config, lock.as_ref(), options.locked)?;

    let mut manifests = Vec::new();
    for (name, source) in &config.check_bundles {
        let manifest = load_declared(root, name, source, lock.as_ref(), options)?;
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
            let name = &manifest.check_bundle.name;
            return Err(format!("duplicate check bundle declaration {name}"));
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
                format!("check bundle inheritance is missing a parent or contains a cycle: {names}")
            })?;
        let extension = manifests.remove(next);
        let name = extension.check_bundle.name.clone();
        resolved = check_bundle::resolve(resolved, extension)?;
        installed.insert(name);
    }
    Ok(resolved)
}

fn load_declared(
    root: &Path,
    name: &str,
    source: &BundleSource,
    lock: Option<&ProjectLock>,
    options: LoadOptions,
) -> Result<CheckBundleManifest, String> {
    match source {
        BundleSource::Local { path } => load_local(root, path),
        BundleSource::Git {
            url,
            ref_name,
            path,
        } => {
            validate_git_subpath(path.as_deref())?;
            let locked = lock
                .and_then(|lock| lock.check_bundles.get(name))
                .filter(|entry| {
                    entry.url == *url
                        && entry.requested_ref == ref_name.as_deref().unwrap_or("HEAD")
                        && entry.path == *path
                });
            let requested = ref_name.as_deref().unwrap_or("HEAD");
            let commit = locked.map(|entry| entry.commit.as_str());
            let installed = installed_git_path(name, url, commit.unwrap_or(requested))?;
            if !installed.exists() {
                if options.offline {
                    return Err(format!(
                        "check bundle {name} is not installed; run `clilint bundle install` without --offline"
                    ));
                }
                install_git(url, commit.unwrap_or(requested), &installed)?;
            }
            load_bundle_at(&installed, path.as_deref())
        }
    }
}

fn load_local(root: &Path, path: &Path) -> Result<CheckBundleManifest, String> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    load_bundle_at(&path, None)
}

fn load_bundle_at(root: &Path, subpath: Option<&Path>) -> Result<CheckBundleManifest, String> {
    let mut path = root.to_path_buf();
    if let Some(subpath) = subpath {
        path.push(subpath);
    }
    if path.is_dir() {
        path.push("clilint.toml");
    }
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("could not read check bundle {}: {error}", path.display()))?;
    let manifest = check_bundle::parse(&text, &path.display().to_string())?;
    check_bundle::validate(&manifest)?;
    Ok(manifest)
}

fn validate_git_subpath(path: Option<&Path>) -> Result<(), String> {
    if path.is_some_and(|path| {
        path.is_absolute()
            || path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
    }) {
        return Err("a Git check-bundle path must stay within its repository".to_owned());
    }
    Ok(())
}

pub fn install(
    root: &Path,
    source: Option<&str>,
    options: LoadOptions,
) -> Result<Vec<BundleStatus>, String> {
    let mut config = load_config(root)?;
    if let Some(source) = source {
        let path = PathBuf::from(source);
        if path.exists() || root.join(&path).exists() {
            let manifest = load_local(root, &path)?;
            let absolute = if path.is_absolute() {
                path
            } else {
                root.join(path)
            };
            config.check_bundles.insert(
                manifest.check_bundle.name,
                BundleSource::Local { path: absolute },
            );
            save_config(root, &config)?;
        } else {
            let (url, requested_ref, subpath) = parse_git_source(source);
            let temporary_name = format!("source-{:x}", Sha256::digest(source.as_bytes()));
            let temporary = cache_dir()?.join(&temporary_name);
            if temporary.exists() {
                fs::remove_dir_all(&temporary).map_err(|error| {
                    format!("could not refresh {}: {error}", temporary.display())
                })?;
            }
            install_git(&url, requested_ref.as_deref().unwrap_or("HEAD"), &temporary)?;
            let manifest = load_bundle_at(&temporary, subpath.as_deref())?;
            let name = manifest.check_bundle.name;
            config.check_bundles.insert(
                name,
                BundleSource::Git {
                    url,
                    ref_name: requested_ref,
                    path: subpath,
                },
            );
            save_config(root, &config)?;
        }
    }
    let _ = load_for_check(root, None, options)?;
    list(root)
}

pub fn list(root: &Path) -> Result<Vec<BundleStatus>, String> {
    let config = load_config(root)?;
    let lock = load_lock(root)?;
    config
        .check_bundles
        .into_iter()
        .map(|(name, source)| {
            let locked = lock.as_ref().and_then(|lock| lock.check_bundles.get(&name));
            let (requested_ref, resolved_commit, installed) = match &source {
                BundleSource::Local { path } => {
                    (None, None, root.join(path).exists() || path.exists())
                }
                BundleSource::Git { url, ref_name, .. } => {
                    let requested = ref_name.clone().unwrap_or_else(|| "HEAD".into());
                    let resolved = locked.map(|entry| entry.commit.clone());
                    let key = resolved.as_deref().unwrap_or(&requested);
                    let installed = installed_git_path(&name, url, key)?.exists();
                    (Some(requested), resolved, installed)
                }
            };
            Ok(BundleStatus {
                name,
                source,
                requested_ref,
                resolved_commit,
                installed,
            })
        })
        .collect()
}

pub fn lock(root: &Path, only: Option<&str>) -> Result<ProjectLock, String> {
    let config = load_config(root)?;
    let mut lock = load_lock(root)?.unwrap_or_default();
    for (name, source) in &config.check_bundles {
        if only.is_some_and(|only| only != name) || lock.check_bundles.contains_key(name) {
            continue;
        }
        if let BundleSource::Git {
            url,
            ref_name,
            path,
        } = source
        {
            let requested_ref = ref_name.clone().unwrap_or_else(|| "HEAD".into());
            let commit = resolve_remote_ref(url, &requested_ref)?;
            lock.check_bundles.insert(
                name.clone(),
                LockedBundle {
                    url: url.clone(),
                    requested_ref,
                    commit,
                    path: path.clone(),
                },
            );
        }
    }
    if only.is_some_and(|name| !config.check_bundles.contains_key(name)) {
        return Err(format!("unknown check bundle {}", only.unwrap()));
    }
    save_lock(root, &lock)?;
    Ok(lock)
}

pub fn update(root: &Path, only: Option<&str>) -> Result<Vec<BundleStatus>, String> {
    let config = load_config(root)?;
    let mut lock = load_lock(root)?.unwrap_or_default();
    for (name, source) in &config.check_bundles {
        if only.is_some_and(|only| only != name) {
            continue;
        }
        if let BundleSource::Git {
            url,
            ref_name,
            path,
        } = source
        {
            let requested_ref = ref_name.clone().unwrap_or_else(|| "HEAD".into());
            let commit = resolve_remote_ref(url, &requested_ref)?;
            lock.check_bundles.insert(
                name.clone(),
                LockedBundle {
                    url: url.clone(),
                    requested_ref,
                    commit,
                    path: path.clone(),
                },
            );
        }
    }
    if only.is_some_and(|name| !config.check_bundles.contains_key(name)) {
        return Err(format!("unknown check bundle {}", only.unwrap()));
    }
    save_lock(root, &lock)?;
    let _ = load_for_check(root, None, LoadOptions::default())?;
    list(root)
}

pub fn remove(root: &Path, name: &str) -> Result<(), String> {
    let mut config = load_config(root)?;
    if config.check_bundles.remove(name).is_none() {
        return Err(format!("unknown check bundle {name}"));
    }
    save_config(root, &config)?;
    if let Some(mut lock) = load_lock(root)? {
        lock.check_bundles.remove(name);
        save_lock(root, &lock)?;
    }
    Ok(())
}

fn validate_lock(
    config: &ProjectConfig,
    lock: Option<&ProjectLock>,
    strict: bool,
) -> Result<(), String> {
    let Some(lock) = lock else {
        return Ok(());
    };
    for (name, source) in &config.check_bundles {
        if let BundleSource::Git {
            url,
            ref_name,
            path,
        } = source
        {
            match lock.check_bundles.get(name) {
                Some(entry)
                    if entry.url == *url
                        && entry.requested_ref == ref_name.as_deref().unwrap_or("HEAD")
                        && entry.path == *path => {}
                _ if strict => {
                    return Err(format!(
                        "lock entry for {name} does not match .clilint/config.toml; run `clilint bundle lock`"
                    ));
                }
                _ => {}
            }
        }
    }
    if strict {
        for name in lock.check_bundles.keys() {
            if !config.check_bundles.contains_key(name) {
                return Err(format!(
                    "lock entry for {name} has no declaration in .clilint/config.toml"
                ));
            }
        }
    }
    Ok(())
}

fn install_git(url: &str, revision: &str, destination: &Path) -> Result<(), String> {
    if revision.starts_with('-') {
        return Err("a Git ref cannot start with '-'".to_owned());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    let temporary = destination.with_extension(format!("tmp-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)
            .map_err(|error| format!("could not clear {}: {error}", temporary.display()))?;
    }
    run_git(&[
        "clone",
        "--quiet",
        "--no-checkout",
        "--",
        url,
        temporary.to_str().unwrap(),
    ])?;
    run_git_in(&temporary, &["checkout", "--quiet", revision])?;
    if destination.exists() {
        fs::remove_dir_all(destination)
            .map_err(|error| format!("could not replace {}: {error}", destination.display()))?;
    }
    fs::rename(&temporary, destination)
        .map_err(|error| format!("could not install {}: {error}", destination.display()))
}

fn resolve_remote_ref(url: &str, requested_ref: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["ls-remote", url, requested_ref])
        .output()
        .map_err(|error| format!("could not run git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "could not resolve Git ref {requested_ref} from {url}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .filter(|commit| commit.len() == 40 && commit.chars().all(|c| c.is_ascii_hexdigit()))
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("Git ref {requested_ref} was not found at {url}"))
}

fn run_git(args: &[&str]) -> Result<(), String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| format!("could not run git: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run_git_in(directory: &Path, args: &[&str]) -> Result<(), String> {
    let output = Command::new("git")
        .current_dir(directory)
        .args(args)
        .output()
        .map_err(|error| format!("could not run git: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn parse_git_source(source: &str) -> (String, Option<String>, Option<PathBuf>) {
    let (repository_and_ref, path) = source
        .split_once("::")
        .map_or((source, None), |(repository, path)| {
            (repository, Some(PathBuf::from(path)))
        });
    let (url, requested_ref) = repository_and_ref
        .rsplit_once('#')
        .map_or((repository_and_ref.to_owned(), None), |(url, reference)| {
            (url.to_owned(), Some(reference.to_owned()))
        });
    (url, requested_ref, path)
}

fn installed_git_path(name: &str, url: &str, revision: &str) -> Result<PathBuf, String> {
    let digest = Sha256::digest(format!("{url}\0{revision}").as_bytes());
    Ok(data_dir()?
        .join("check-bundles")
        .join(name)
        .join(format!("{digest:x}")))
}

fn data_dir() -> Result<PathBuf, String> {
    storage_dir("CLILINT_DATA_DIR", "XDG_DATA_HOME", ".local/share")
}

fn cache_dir() -> Result<PathBuf, String> {
    storage_dir("CLILINT_CACHE_DIR", "XDG_CACHE_HOME", ".cache")
}

fn storage_dir(override_name: &str, xdg_name: &str, fallback: &str) -> Result<PathBuf, String> {
    if let Some(path) = env::var_os(override_name) {
        return Ok(PathBuf::from(path));
    }
    if let Some(path) = env::var_os(xdg_name) {
        return Ok(PathBuf::from(path).join("clilint"));
    }
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(fallback).join("clilint"))
        .ok_or_else(|| format!("{override_name}, {xdg_name}, and HOME are unset"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_compact_git_source() {
        let (url, reference, path) =
            parse_git_source("https://example.com/bundles.git#v1::bundles/help");
        assert_eq!(url, "https://example.com/bundles.git");
        assert_eq!(reference.as_deref(), Some("v1"));
        assert_eq!(path.as_deref(), Some(Path::new("bundles/help")));
    }

    #[test]
    fn rejects_git_paths_outside_the_repository() {
        assert!(validate_git_subpath(Some(Path::new("../bundle"))).is_err());
        assert!(validate_git_subpath(Some(Path::new("bundles/help"))).is_ok());
    }
}
