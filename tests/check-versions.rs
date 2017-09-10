#[macro_use]
extern crate check_versions;

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}
