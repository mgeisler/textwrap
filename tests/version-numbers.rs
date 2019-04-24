#[macro_use]
extern crate version_sync;
extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_readme_changelog() {
    assert_contains_regex!(
        "README.md",
        r"^### Version {version} — .* \d\d?.., 20\d\d$"
    );
}

#[test]
fn test_readme_rustc_min_version() {
    let mut file = File::open(".travis.yml").expect("Could not open");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Could not read");

    let yaml = YamlLoader::load_from_str(&buf).expect("Could not parse");
    assert_eq!(yaml.len(), 1, "Expected a single YAML document");

    let rust_versions = &yaml[0]["rust"]
        .as_vec()
        .expect("Could not find vector with Rust versions");

    let min_rust_version = rust_versions
        .iter()
        .filter_map(|version| match version.as_str() {
            Some("stable") | Some("beta") | Some("nightly") => None,
            version => version,
        })
        .last()
        .expect("No minimum Rust version found");

    let pattern = "https://img.shields.io/badge/rustc-{version}-4d76ae.svg";
    version_sync::check_contains_regex("README.md", pattern, "", min_rust_version)
        .expect("Minimum Rust version not found");
}

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}
