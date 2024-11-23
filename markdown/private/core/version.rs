use clap::Parser;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::{MetadataMap, Version};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    raw_version_file: String,
    deps_metadata_file: String,
    metadata_out_file: String,
    #[arg(long = "version_override")]
    version_override: Option<String>,
    #[arg(long = "repo_override")]
    repo_override: Option<String>,
}

fn sorted_targets(dep_versions: &HashMap<String, Version>) -> Vec<&String> {
    let mut targets = Vec::new();
    for target in dep_versions.keys() {
        targets.push(target);
    }
    targets.sort();
    targets
}

fn version_string(
    raw_version: &str,
    version_override: Option<String>,
    dirty_deps: bool,
    unversioned_deps: bool,
) -> String {
    if let Some(version) = version_override {
        return version;
    }

    let mut version = String::from(raw_version);
    if dirty_deps {
        version.push_str(", dirty deps");
    }
    if unversioned_deps {
        version.push_str(", unversioned deps");
    }
    version
}

fn get_version(
    raw_version: &Version,
    dep_versions: &HashMap<String, Version>,
    version_override: Option<String>,
    repo_override: Option<String>,
) -> Result<Version, Box<dyn Error>> {
    let mut dirty_deps = Vec::new();
    let mut unversioned_deps = Vec::new();

    for target in sorted_targets(dep_versions) {
        let version = &dep_versions[target];
        if version.version.contains("dirty") {
            dirty_deps.push((target, version));
        }
        if version.version.contains("unversioned") {
            unversioned_deps.push((target, version));
        }
    }

    let version = Version::build(
        &version_string(
            &raw_version.version,
            version_override,
            !dirty_deps.is_empty(),
            !unversioned_deps.is_empty(),
        ),
        &raw_version.repo,
    )?;

    // Dirty or unversioned deps in the same repo are OK
    let bad_dirty_deps: Vec<&String> = dirty_deps
        .iter()
        .filter(|(_, v)| v.repo != version.repo)
        .map(|(t, _)| *t)
        .collect();
    let bad_unversioned_deps: Vec<&String> = unversioned_deps
        .iter()
        .filter(|(_, v)| v.repo != version.repo)
        .map(|(t, _)| *t)
        .collect();

    if !bad_dirty_deps.is_empty() || !bad_unversioned_deps.is_empty() {
        let mut msg = vec![String::from("Target has dirty or unversioned deps")];

        if !bad_dirty_deps.is_empty() {
            msg.push(String::from("Dirty deps:"));
            msg.append(
                &mut bad_dirty_deps
                    .iter()
                    .map(|dep| String::from("  ") + dep.as_str())
                    .collect(),
            );
        }
        if !bad_unversioned_deps.is_empty() {
            msg.push(String::from("Unversioned deps:"));
            msg.append(
                &mut bad_unversioned_deps
                    .iter()
                    .map(|dep| String::from("  ") + dep.as_str())
                    .collect(),
            );
        }

        return Err(msg.join("\n").into());
    }

    if let Some(repo) = repo_override {
        return Ok(Version::build(&version.version, &repo)?);
    }

    Ok(version)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let raw_version: Version = from_json(&read_to_string(args.raw_version_file)?)?;
    let dep_metadata: MetadataMap = from_json(&read_to_string(args.deps_metadata_file)?)?;

    let mut dep_versions: HashMap<String, Version> = HashMap::new();
    for (target, metadata) in dep_metadata.data().iter() {
        dep_versions.insert(
            String::from(target),
            Version::build(metadata.version(), metadata.repo())?,
        );
    }

    let version = get_version(
        &raw_version,
        &dep_versions,
        args.version_override,
        args.repo_override,
    )?;

    version.write_json(args.metadata_out_file)
}

#[cfg(test)]
mod version_test {
    use super::{get_version, HashMap, Version};

    #[test]
    fn test_version() {
        let base = Version::build("1", "foo").unwrap();
        let clean = Version::build("2", "bar").unwrap();
        let dirty = Version::build("3-dirty", "baz").unwrap();
        let unversioned = Version::build("unversioned", "quux").unwrap();
        let dirty_same_repo = Version::build("4-dirty", "foo").unwrap();
        let unversioned_same_repo = Version::build("unversioned", "foo").unwrap();

        let v = get_version(&base, &HashMap::new(), None, None).unwrap();
        assert_eq!(v.version, "1");
        assert_eq!(v.repo, "foo");

        let v = get_version(
            &base,
            &HashMap::from([(String::from("a"), clean.clone())]),
            None,
            None,
        )
        .unwrap();
        assert_eq!(v.version, "1");
        assert_eq!(v.repo, "foo");

        let v = get_version(
            &base,
            &HashMap::from([
                (String::from("a"), clean.clone()),
                (String::from("b"), dirty_same_repo.clone()),
                (String::from("c"), unversioned_same_repo.clone()),
            ]),
            None,
            None,
        )
        .unwrap();
        assert_eq!(v.version, "1, dirty deps, unversioned deps");
        assert_eq!(v.repo, "foo");

        assert!(get_version(
            &base,
            &HashMap::from([(String::from("a"), dirty.clone())]),
            None,
            None
        )
        .is_err());
        assert!(get_version(
            &base,
            &HashMap::from([(String::from("a"), unversioned.clone())]),
            None,
            None
        )
        .is_err());

        let v = get_version(&base, &HashMap::new(), Some(String::from("OVERRIDE")), None).unwrap();
        assert_eq!(v.version, "OVERRIDE");
        assert_eq!(v.repo, "foo");

        let v = get_version(
            &base,
            &HashMap::from([(String::from("a"), clean.clone())]),
            Some(String::from("OVERRIDE")),
            None,
        )
        .unwrap();
        assert_eq!(v.version, "OVERRIDE");
        assert_eq!(v.repo, "foo");

        let v = get_version(
            &base,
            &HashMap::from([
                (String::from("a"), clean.clone()),
                (String::from("b"), dirty_same_repo.clone()),
                (String::from("c"), unversioned_same_repo.clone()),
            ]),
            Some(String::from("OVERRIDE")),
            None,
        )
        .unwrap();
        assert_eq!(v.version, "OVERRIDE");
        assert_eq!(v.repo, "foo");

        let v = get_version(&base, &HashMap::new(), None, Some(String::from("OVERRIDE"))).unwrap();
        assert_eq!(v.version, "1");
        assert_eq!(v.repo, "OVERRIDE");

        assert!(get_version(
            &base,
            &HashMap::from([(String::from("a"), dirty.clone())]),
            Some(String::from("OVERRIDE")),
            Some(String::from("OVERRIDE"))
        )
        .is_err());
        assert!(get_version(
            &base,
            &HashMap::from([(String::from("a"), unversioned.clone())]),
            Some(String::from("OVERRIDE")),
            Some(String::from("OVERRIDE"))
        )
        .is_err());
    }
}
