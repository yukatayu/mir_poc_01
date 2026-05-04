use std::fs;
use std::path::PathBuf;

use mir_runtime::practical_alpha1_avatar::render_practical_alpha1_avatar_preview_path;
use serde_json::Value;

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

fn read_expected_report(name: &str) -> Value {
    let path = practical_root().join("expected").join(name);
    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("failed to read expected report {}: {error}", path.display())
    });
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!(
            "failed to parse expected report {}: {error}",
            path.display()
        )
    })
}

#[test]
fn practical_avatar_matches_placeholder_preview_row() {
    let report = render_practical_alpha1_avatar_preview_path(practical_package_dir(
        "av-a1-01-placeholder-avatar-runtime",
    ))
    .expect("AV-A1-01 should render a placeholder avatar preview report");
    let expected = read_expected_report("av-a1-01-placeholder-avatar-runtime.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize avatar preview"),
        expected
    );
}

#[test]
fn practical_avatar_matches_custom_runtime_preview_row() {
    let report = render_practical_alpha1_avatar_preview_path(practical_package_dir(
        "av-a1-02-custom-mir-avatar-runtime",
    ))
    .expect("AV-A1-02 should render a custom avatar preview report");
    let expected = read_expected_report("av-a1-02-custom-mir-avatar-runtime.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize avatar preview"),
        expected
    );
}

#[test]
fn practical_avatar_matches_visible_fallback_preview_row() {
    let report = render_practical_alpha1_avatar_preview_path(practical_package_dir(
        "av-a1-03-unsupported-runtime-fallback",
    ))
    .expect("AV-A1-03 should render a visible unsupported-runtime fallback preview");
    let expected = read_expected_report("av-a1-03-unsupported-runtime-fallback.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize avatar preview"),
        expected
    );
}
