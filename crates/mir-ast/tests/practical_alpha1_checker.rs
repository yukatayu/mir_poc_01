#[path = "support/practical_alpha1_checker_support.rs"]
mod support;

use mir_ast::practical_alpha1_checker::{
    PracticalAlpha1CheckErrorKind, check_practical_alpha1_package_path,
};
use support::{load_expected_check_report, practical_package_dir};

#[test]
fn practical_alpha1_checker_matches_positive_expected_reports() {
    let cases = [
        (
            "chk-lif-02-fallback-access-valid",
            "chk-lif-02-fallback-access-valid.expected.json",
        ),
        (
            "chk-lif-03-inherited-chain-valid",
            "chk-lif-03-inherited-chain-valid.expected.json",
        ),
        (
            "chk-lif-04-snapshot-selected-distinction",
            "chk-lif-04-snapshot-selected-distinction.expected.json",
        ),
        (
            "chk-var-01-logging-layer-valid",
            "chk-var-01-logging-layer-valid.expected.json",
        ),
    ];

    for (package_name, expected_name) in cases {
        let observed = check_practical_alpha1_package_path(practical_package_dir(package_name))
            .expect("checker should accept positive practical sample");
        let expected =
            load_expected_check_report(expected_name).expect("expected report should load");
        assert_eq!(
            observed, expected,
            "positive checker drift for {package_name}"
        );
        assert!(!observed.accepted_obligations.is_empty());
        assert!(observed.rejected_rows.is_empty());
    }
}

#[test]
fn practical_alpha1_checker_matches_negative_expected_reports() {
    let cases = [
        (
            "chk-lif-01-raw-dangling",
            "chk-lif-01-raw-dangling.expected.json",
        ),
        (
            "chk-var-02-precondition-strengthening-rejected",
            "chk-var-02-precondition-strengthening-rejected.expected.json",
        ),
        (
            "chk-var-03-mutable-covariance-rejected",
            "chk-var-03-mutable-covariance-rejected.expected.json",
        ),
        (
            "chk-cut-01-invalid-distributed-cut-rejected",
            "chk-cut-01-invalid-distributed-cut-rejected.expected.json",
        ),
        (
            "chk-pkg-01-unsigned-native-rejected",
            "chk-pkg-01-unsigned-native-rejected.expected.json",
        ),
        (
            "chk-pkg-02-over-capability-rejected",
            "chk-pkg-02-over-capability-rejected.expected.json",
        ),
    ];

    for (package_name, expected_name) in cases {
        let observed = check_practical_alpha1_package_path(practical_package_dir(package_name))
            .expect("checker should return explicit negative report");
        let expected =
            load_expected_check_report(expected_name).expect("expected report should load");
        assert_eq!(
            observed, expected,
            "negative checker drift for {package_name}"
        );
        assert!(observed.accepted_obligations.is_empty());
        assert!(!observed.rejected_rows.is_empty());
    }
}

#[test]
fn practical_alpha1_checker_rejects_package_without_checker_section() {
    let error = check_practical_alpha1_package_path(practical_package_dir("src-01-minimal-world"))
        .expect_err("front-door-only package should not auto-promote into checker floor");
    assert_eq!(
        error.kind,
        PracticalAlpha1CheckErrorKind::MissingCheckerSection
    );
}
