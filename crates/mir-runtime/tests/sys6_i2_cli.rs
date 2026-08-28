use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

const TOY_SOURCE: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const TOY_SOURCE_TEXT: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const PLUS_TWO_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir";
const OWNER_RMW_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir";
const SELECTED_OW1_SOURCE: &str =
    "samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir";
const SELECTED_OW1_SOURCE_TEXT: &str = include_str!(
    "../../../samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir"
);
const SECRET_MARKER: &str = "sk-test-secret-conform-i2-redaction";
const ALLOWED_EVIDENCE_CLASSES: &[&str] = &[
    "lean-proved",
    "lean-stated",
    "model-checked-bounded",
    "runtime-monitored",
    "intentionally-deferred",
];
const REQUIRED_I2_ROWS: &[&str] = &[
    "i2.ordinary_source_authority",
    "i2.checked_global_core_identity",
    "i2.core_to_locus_artifacts",
    "i2.generated_communication_complete",
    "i2.actual_dispatch_over_generated_edges",
    "i2.st_ow_selected_correspondence",
    "i2.owner_data_race_freedom_selected_backend",
    "i2.no_hidden_communication",
    "i2.no_direct_remote_store",
    "i2.no_source_free_authority_mint",
    "i2.no_source_free_state_mint",
    "i2.failure_containment",
    "i2.visibility_redaction_preserved",
    "i2.relation_projection_coherence",
    "i2.semantic_presentation_fallback_separation",
    "i2.designated_evaluator_non_reexecution",
    "i2.source_core_artifact_trace_correspondence",
    "i2.save_restore_consistent_local_cut",
    "i2.patch_lifecycle_checked",
    "i2.observer_safe_devtools",
    "i2.projection_determinism",
    "i2.non_claims_and_lifecycle_boundaries",
];

#[test]
fn mir_conform_i2_cli_emits_finite_source_first_report() {
    let output = run_cli(&canonical_args());
    let value = json_stdout(&output);

    assert!(
        output.status.success(),
        "conform-i2 should accept the bounded canonical I2 profile: {value:#}"
    );
    assert_eq!(value["schema_version"], "mirrorea-i2-conformance-report-v0");
    assert_eq!(value["command"], "conform-i2");
    assert_eq!(value["status"], "accepted");
    assert_eq!(value["source_authority"], "ordinary_mir_source");
    assert_eq!(value["profile_scope"], "bounded-finite-i2");
    assert_eq!(value["public_api_or_wire_contract"], false);
    assert_eq!(value["final_public_api_frozen"], false);
    assert_eq!(value["public_wire_frozen"], false);
    assert_absent(&value, "cli_falsifier_surface_available");
    assert_source_first_flags_are_actual_provenance_not_self_attestation(&value);
    assert_nonempty_string(&value["i2_manifest_identity_ref"], "I2 manifest identity");
    assert_bounded_implementation_source_fingerprint(&value);
    assert_exact_pass_row_inventory(&value);
    assert_evidence_class_universe(&value);
    assert_rows_have_executed_evidence_refs(&value);
    assert_controls_cross_join_executed_evidence(&value);
    assert_accepted_negative_evidence_is_actual_control_not_unexecuted_detected(&value);
    assert_owner_preservation_subclaim(&value);
    assert_selected_backend_actions_are_typed_successes(&value);
    assert_owner_data_race_runtime_row_and_model_ordering_evidence_are_separate(&value);
    assert_selected_ow1_row_uses_selected_source_actual_anchor(&value, "S");
    assert_observer_sensitive_evidence_binds_actual_marker_candidate_and_redacted_output(&value);
    assert_lifecycle_boundary_evidence_is_actual_typed_candidate(&value);
    assert_row_specific_anchors(&value);
    assert_json_array_contains_all(
        &value["evidence_classes"],
        &["runtime-monitored", "model-checked-bounded"],
    );
    assert_json_array_contains_all(
        &value["non_claims"],
        &[
            "real transport",
            "public ABI or wire freeze",
            "durable distributed save/load",
            "general metatheory",
        ],
    );
    assert_observer_safe_cli_payload(&value, &output);
}

#[test]
fn mir_conform_i2_cli_output_is_deterministic_and_observer_safe() {
    let first = run_cli(&canonical_args());
    let first_value = json_stdout(&first);
    assert!(
        first.status.success(),
        "first conform-i2 run should succeed: {first_value:#}"
    );
    let second = run_cli(&canonical_args());
    let second_value = json_stdout(&second);
    assert!(
        second.status.success(),
        "second conform-i2 run should succeed: {second_value:#}"
    );

    assert_eq!(
        first.stdout, second.stdout,
        "conform-i2 output must be byte-deterministic for identical finite-profile inputs"
    );
    assert_observer_safe_cli_payload(&first_value, &first);
    assert_observer_safe_cli_payload(&second_value, &second);
}

#[test]
fn mir_conform_i2_cli_observer_safe_serialization_hides_arbitrary_source_path_marker() {
    let marker = "private_review_marker_20260828_sensitive_path_token";
    let primary = TempMirSource::new(marker, TOY_SOURCE_TEXT);
    let selected = TempMirSource::new(
        "selected-private_review_marker_20260828_sensitive_path_token",
        SELECTED_OW1_SOURCE_TEXT,
    );
    let output = run_cli_owned(&canonical_owned_args_with_selected(
        primary.path.as_path(),
        selected.path.as_path(),
    ));
    let value = json_stdout(&output);

    assert!(
        output.status.success(),
        "valid source at sensitive host path should still conform after observer-safe serialization: {value:#}"
    );
    assert_dynamic_markers_absent_from_cli_payload(
        &value,
        &output,
        &[
            marker.to_string(),
            primary.path.display().to_string(),
            selected.path.display().to_string(),
        ],
    );
    assert_observer_safe_cli_payload(&value, &output);
}

#[test]
fn mir_conform_i2_cli_rejects_invalid_inputs_as_typed_json() {
    for args in [
        vec!["conform-i2", "--format", "json"],
        vec![
            "conform-i2",
            "samples/clean-near-end/mirrorea-i2-local-toy/missing.mir",
            "--selected-ow1-source",
            SELECTED_OW1_SOURCE,
            "--format",
            "json",
        ],
        vec![
            "conform-i2",
            TOY_SOURCE,
            "--selected-ow1-source",
            "samples/clean-near-end/mirrorea-i2-conformance/token=sk-test-secret-conform-i2-redaction.mir",
            "--unexpected",
            "value",
            "--format",
            "json",
        ],
        vec![
            "conform-i2",
            TOY_SOURCE,
            "--selected-ow1-source",
            SELECTED_OW1_SOURCE,
        ],
    ] {
        let output = run_cli(&args);
        let value = json_stdout(&output);

        assert!(
            !output.status.success(),
            "invalid conform-i2 args must reject nonzero: {value:#}"
        );
        assert_eq!(value["command"], "conform-i2");
        assert_eq!(value["status"], "error");
        assert_eq!(value["source_authority"], "ordinary_mir_source");
        assert_eq!(value["public_api_or_wire_contract"], false);
        assert_eq!(value["final_public_api_frozen"], false);
        assert_eq!(value["public_wire_frozen"], false);
        assert_absent(&value, "cli_falsifier_surface_available");
        assert_nonempty_string(&value["diagnostic_code"], "diagnostic code");
        assert!(
            !String::from_utf8_lossy(&output.stdout).contains(SECRET_MARKER)
                && !String::from_utf8_lossy(&output.stderr).contains(SECRET_MARKER),
            "secret-like invalid input must be redacted from stdout/stderr"
        );
        assert_observer_safe_cli_payload(&value, &output);
    }
}

#[test]
fn mir_conform_i2_cli_does_not_expose_test_only_falsifiers() {
    let output = run_cli(&[
        "conform-i2",
        TOY_SOURCE,
        "--selected-ow1-source",
        SELECTED_OW1_SOURCE,
        "--test-falsifier",
        "RemoveGeneratedCommunicationEdge",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(
        !output.status.success(),
        "CLI must reject test-only conformance falsifiers: {value:#}"
    );
    assert_eq!(value["command"], "conform-i2");
    assert_eq!(value["status"], "error");
    assert!(
        matches!(
            value["diagnostic_code"].as_str(),
            Some("test_falsifier_not_available_via_cli" | "i2_unexpected_arguments")
        ),
        "CLI must reject test-only falsifier naming with a typed diagnostic: {value:#}"
    );
    assert_absent(&value, "test_only_falsifier");
    assert_absent(&value, "cli_falsifier_surface_available");
    assert_observer_safe_cli_payload(&value, &output);
    assert!(
        !String::from_utf8_lossy(&output.stdout).contains("RemoveGeneratedCommunicationEdge")
            && !String::from_utf8_lossy(&output.stderr)
                .contains("RemoveGeneratedCommunicationEdge"),
        "CLI rejection must not echo internal falsifier names"
    );
}

#[test]
fn mir_conform_i2_cli_exits_nonzero_while_emitting_typed_rejected_report() {
    let sensitive_source = observer_sensitive_source_text();
    let source = TempMirSource::new("credential-secret-rejected-report", &sensitive_source);
    let output = run_cli_owned(&canonical_owned_args(source.path.as_path()));
    let value = json_stdout(&output);

    assert!(
        !output.status.success(),
        "a typed rejected I2 report must make conform-i2 exit nonzero: {value:#}"
    );
    assert_eq!(value["command"], "conform-i2");
    assert_eq!(value["status"], "rejected");
    assert_eq!(value["source_authority"], "ordinary_mir_source");
    assert_eq!(
        value["typed_rejection"]["diagnostic_code"],
        "ObserverSensitiveIdentifier"
    );
    assert_eq!(
        value["typed_rejection"]["observer_policy"]["action"],
        "redact"
    );
    assert_nonempty_array(&value["rows"], "typed rejected report rows");
    assert_observer_safe_cli_payload(&value, &output);
}

#[test]
fn mir_conform_i2_cli_redacts_observer_sensitive_identifiers_inside_valid_source() {
    let sensitive_source = observer_sensitive_source_text();
    assert!(
        sensitive_source.contains("credential_secret_input"),
        "test source must contain a syntactically valid observer-sensitive identifier"
    );
    let source = TempMirSource::new("credential-secret-redaction", &sensitive_source);
    let output = run_cli_owned(&canonical_owned_args(source.path.as_path()));
    let value = json_stdout(&output);

    assert!(
        !String::from_utf8_lossy(&output.stdout).contains("credential_secret_input")
            && !String::from_utf8_lossy(&output.stderr).contains("credential_secret_input"),
        "raw stdout/stderr must not expose observer-sensitive source identifiers"
    );
    assert_eq!(value["source_authority"], "ordinary_mir_source");
    assert!(
        value["typed_rejection"]["observer_policy"]["redacted_identifiers"]
            .as_array()
            .is_some_and(|entries| entries
                .iter()
                .any(|entry| entry["redacted_as"] == "[redacted-observer-sensitive-identifier]")),
        "typed rejection/redaction must record observer policy without raw identifier leakage: {value:#}"
    );
    assert_observer_safe_cli_payload(&value, &output);
}

#[test]
fn mir_conform_i2_cli_redacts_sensitive_selected_ow1_source_locus_and_name() {
    let selected_source_text = selected_ow1_sensitive_worker_source_text();
    assert!(
        selected_source_text.contains("credential_secret_worker"),
        "test selected OW1 source must contain an observer-sensitive locus/name"
    );
    let selected_source =
        TempMirSource::new("selected-ow1-credential-secret", &selected_source_text);
    let output = run_cli_owned(&canonical_owned_args_with_selected(
        Path::new(TOY_SOURCE),
        selected_source.path.as_path(),
    ));
    let value = json_stdout(&output);
    let raw_stdout = String::from_utf8_lossy(&output.stdout);
    let raw_stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "observer-sensitive selected OW1 source must produce a typed rejection: {value:#}"
    );
    assert_eq!(value["command"], "conform-i2");
    assert_eq!(value["status"], "rejected");
    assert_eq!(
        value["typed_rejection"]["diagnostic_code"],
        "ObserverSensitiveIdentifier"
    );
    assert_eq!(
        value["typed_rejection"]["observer_policy"]["action"],
        "redact"
    );
    assert!(
        !raw_stdout.contains("credential_secret_worker")
            && !raw_stderr.contains("credential_secret_worker")
            && !serde_json::to_string(&value)
                .expect("JSON payload should serialize")
                .contains("credential_secret_worker"),
        "selected OW1 sensitive locus/name must not leak through telemetry, residuals, loci, inventories, stdout, or stderr"
    );
    assert_observer_safe_cli_payload(&value, &output);
}

fn canonical_args() -> Vec<&'static str> {
    vec![
        "conform-i2",
        TOY_SOURCE,
        "--selected-ow1-source",
        SELECTED_OW1_SOURCE,
        "--patch",
        PLUS_TWO_PATCH,
        "--patch",
        OWNER_RMW_PATCH,
        "--format",
        "json",
    ]
}

fn canonical_owned_args(source_path: &Path) -> Vec<String> {
    canonical_owned_args_with_selected(source_path, Path::new(SELECTED_OW1_SOURCE))
}

fn canonical_owned_args_with_selected(source_path: &Path, selected_ow1_path: &Path) -> Vec<String> {
    vec![
        "conform-i2".to_string(),
        source_path.display().to_string(),
        "--selected-ow1-source".to_string(),
        selected_ow1_path.display().to_string(),
        "--patch".to_string(),
        PLUS_TWO_PATCH.to_string(),
        "--patch".to_string(),
        OWNER_RMW_PATCH.to_string(),
        "--format".to_string(),
        "json".to_string(),
    ]
}

fn run_cli(args: &[&str]) -> Output {
    let mut command = match cli_binary_path() {
        Some(binary) => Command::new(binary),
        None => {
            let mut cargo = Command::new("cargo");
            cargo.args(["run", "-q", "-p", "mir-runtime", "--bin", "mir", "--"]);
            cargo
        }
    };
    command
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("mir CLI should run")
}

fn run_cli_owned(args: &[String]) -> Output {
    let mut command = match cli_binary_path() {
        Some(binary) => Command::new(binary),
        None => {
            let mut cargo = Command::new("cargo");
            cargo.args(["run", "-q", "-p", "mir-runtime", "--bin", "mir", "--"]);
            cargo
        }
    };
    command
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("mir CLI should run")
}

fn observer_sensitive_source_text() -> String {
    TOY_SOURCE_TEXT.replace("participant_input", "credential_secret_input")
}

fn selected_ow1_sensitive_worker_source_text() -> String {
    SELECTED_OW1_SOURCE_TEXT
        .replace("locus S\n", "locus credential_secret_worker\n")
        .replace(" at S", " at credential_secret_worker")
        .replace("at S {", "at credential_secret_worker {")
}

fn cli_binary_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_mir") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    let current = std::env::current_exe().ok()?;
    let target_dir = current.parent()?.parent()?;
    let candidate = target_dir.join(format!("mir{}", std::env::consts::EXE_SUFFIX));
    candidate.exists().then_some(candidate)
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-runtime")
        .to_path_buf()
}

struct TempMirSource {
    dir: PathBuf,
    path: PathBuf,
}

impl TempMirSource {
    fn new(label: &str, source_text: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("mir-i2-cli-{}-{nanos}-{label}", std::process::id()));
        fs::create_dir_all(&dir).expect("temporary Mir CLI source directory should be creatable");
        let path = dir.join("cli-source.mir");
        fs::write(&path, source_text).expect("temporary Mir CLI source should be writable");
        Self { dir, path }
    }
}

impl Drop for TempMirSource {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn json_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout should be JSON: {error}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn assert_exact_pass_row_inventory(value: &Value) {
    let rows = value["rows"]
        .as_array()
        .expect("rows should be a JSON array");
    assert_eq!(
        rows.len(),
        REQUIRED_I2_ROWS.len(),
        "conform-i2 must emit exactly the bounded I2 row inventory"
    );
    let ids = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id should be a string"))
        .collect::<Vec<_>>();
    let actual = ids.iter().copied().collect::<BTreeSet<_>>();
    let expected = REQUIRED_I2_ROWS.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        actual, expected,
        "conform-i2 must emit exactly the bounded I2 row inventory"
    );
    for row in rows {
        let id = row["id"].as_str().expect("row id should be a string");
        assert_eq!(
            row["status"], "pass",
            "accepted CLI row should pass: {row:#}"
        );
        assert_eq!(
            row["scope"].as_str(),
            Some(expected_scope(id)),
            "accepted CLI row scope must be exact, bounded, and nonempty: {row:#}"
        );
        assert!(
            ALLOWED_EVIDENCE_CLASSES.contains(&row["evidence_class"].as_str().unwrap_or("")),
            "accepted CLI row evidence_class must be one of the exact assurance classes: {row:#}"
        );
    }
}

fn expected_scope(id: &str) -> &'static str {
    match id {
        "i2.ordinary_source_authority" => "primary-source-first",
        "i2.checked_global_core_identity" => "checked-finite-core",
        "i2.core_to_locus_artifacts" => "four-locus-projection",
        "i2.generated_communication_complete" => "generated-edge-inventory",
        "i2.actual_dispatch_over_generated_edges" => "st-local-dispatch",
        "i2.st_ow_selected_correspondence" => "selected-one-owner-worker-fragment",
        "i2.owner_data_race_freedom_selected_backend" => "selected-one-owner-worker-fragment",
        "i2.no_hidden_communication" => "generated-edge-only",
        "i2.no_direct_remote_store" => "locus-endpoint-boundary",
        "i2.no_source_free_authority_mint" => "m9-admitted-source-authority",
        "i2.no_source_free_state_mint" => "source-admitted-state-only",
        "i2.failure_containment" => "typed-failure-before-mutation",
        "i2.visibility_redaction_preserved" => "observer-safe-redaction",
        "i2.relation_projection_coherence" => "finite-relation-fallback",
        "i2.semantic_presentation_fallback_separation" => "presentation-gap-nonmutation",
        "i2.designated_evaluator_non_reexecution" => "designated-result-delivery",
        "i2.source_core_artifact_trace_correspondence" => "source-core-artifact-occurrence",
        "i2.save_restore_consistent_local_cut" => "st-local-cut-restore",
        "i2.patch_lifecycle_checked" => "checked-patch-lifecycle",
        "i2.observer_safe_devtools" => "reference-only-observer-view",
        "i2.projection_determinism" => "same-source-repeat",
        "i2.non_claims_and_lifecycle_boundaries" => "provisional-internal-boundary",
        other => panic!("unexpected CLI I2 row id while checking scope: {other}"),
    }
}

fn assert_evidence_class_universe(value: &Value) {
    for class in value["evidence_classes"]
        .as_array()
        .expect("top-level evidence_classes should be an array")
    {
        let class = class
            .as_str()
            .unwrap_or_else(|| panic!("evidence class should be a string: {value:#}"));
        assert!(
            ALLOWED_EVIDENCE_CLASSES.contains(&class),
            "top-level evidence class must be one of the exact assurance classes: {class}"
        );
    }
}

fn assert_rows_have_executed_evidence_refs(value: &Value) {
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let id = row["id"].as_str().expect("row id");
        assert_nonempty_array(&row["positive_evidence_refs"], "row positive evidence refs");
        assert_nonempty_array(
            &row["falsifier_evidence_refs"],
            "row falsifier evidence refs",
        );
        for field in ["positive_evidence_refs", "falsifier_evidence_refs"] {
            for evidence_ref in row[field]
                .as_array()
                .expect("evidence refs should be an array")
            {
                let text = evidence_ref
                    .as_str()
                    .unwrap_or_else(|| panic!("{id}/{field} entry must be string: {row:#}"));
                let evidence = executed.get(text).unwrap_or_else(|| {
                    panic!("{id}/{field} ref must cross-join executed_evidence inventory: {text}")
                });
                assert_executed_evidence_binds_row(evidence, id);
                assert!(
                    ALLOWED_EVIDENCE_CLASSES
                        .contains(&evidence["evidence_class"].as_str().unwrap_or("")),
                    "{id}/{field} executed evidence class must be exact: {evidence:#}"
                );
                assert_eq!(
                    evidence["executed"], true,
                    "{id}/{field} evidence must be tied to an executed producer/control invocation: {evidence:#}"
                );
                match field {
                    "positive_evidence_refs" => assert_eq!(
                        evidence["outcome"], "observed",
                        "{id}/{field} evidence must carry the exact positive outcome observed: {evidence:#}"
                    ),
                    "falsifier_evidence_refs" => assert_eq!(
                        evidence["outcome"], "detected",
                        "{id}/{field} evidence must carry the exact falsifier outcome detected: {evidence:#}"
                    ),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn assert_controls_cross_join_executed_evidence(value: &Value) {
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let row_id = row["id"].as_str().unwrap_or("<unknown>");
        let controls = row["controls"]
            .as_array()
            .unwrap_or_else(|| panic!("row {row_id} should carry explicit controls: {row:#}"));
        for control in controls {
            let control_id = control["id"]
                .as_str()
                .unwrap_or_else(|| panic!("row {row_id} control must have id: {control:#}"));
            let evidence_ref = control["evidence_ref"].as_str().unwrap_or_else(|| {
                panic!("row {row_id} control {control_id} must link executed evidence: {control:#}")
            });
            let evidence = executed.get(evidence_ref).unwrap_or_else(|| {
                panic!(
                    "row {row_id} control {control_id} evidence_ref must cross-join executed_evidence inventory: {evidence_ref}"
                )
            });
            assert_executed_evidence_binds_row(evidence, row_id);
            assert!(
                control["outcome"]
                    .as_str()
                    .is_some_and(|outcome| { evidence["outcome"].as_str() == Some(outcome) }),
                "row {row_id} control {control_id} outcome must match executed evidence: control={control:#} evidence={evidence:#}"
            );
        }
    }
}

fn assert_accepted_negative_evidence_is_actual_control_not_unexecuted_detected(value: &Value) {
    assert_eq!(value["status"], "accepted");
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let id = row["id"].as_str().expect("row id");
        for evidence_ref in row["falsifier_evidence_refs"]
            .as_array()
            .expect("falsifier evidence refs should be an array")
        {
            let evidence_ref = evidence_ref
                .as_str()
                .unwrap_or_else(|| panic!("{id}/falsifier evidence ref should be string"));
            let evidence = executed.get(evidence_ref).unwrap_or_else(|| {
                panic!(
                    "{id}/falsifier evidence ref must cross-join executed evidence: {evidence_ref}"
                )
            });
            assert_executed_evidence_binds_row(evidence, id);
            assert_nonempty_string(&evidence["control_ref"], "negative evidence control ref");
            assert_nonempty_string(
                &evidence["producer_invocation_ref"],
                "negative evidence producer invocation ref",
            );
            let outcome = evidence["outcome"].as_str();
            let kind = evidence["kind"].as_str().unwrap_or("");
            let claims_execution =
                matches!(outcome, Some("detected" | "executed")) || kind.contains("executed");
            if claims_execution {
                assert_eq!(
                    evidence["executed"], true,
                    "accepted report must not mark unexecuted falsifier controls as executed/detected: {evidence:#}"
                );
                assert_nonempty_string(
                    &evidence["candidate_identity_before"],
                    "executed negative evidence candidate before",
                );
                assert_nonempty_string(
                    &evidence["candidate_identity_after"],
                    "executed negative evidence candidate after",
                );
                assert_nonempty_string(
                    &evidence["diagnostic_code"],
                    "executed negative evidence diagnostic",
                );
            } else {
                assert_eq!(
                    evidence["executed"], false,
                    "registered but unrun negative controls must explicitly say executed=false: {evidence:#}"
                );
                assert!(
                    matches!(
                        outcome,
                        Some("control-registered" | "available-control" | "not-run")
                    ),
                    "unexecuted negative controls must not claim detected/executed outcome: {evidence:#}"
                );
            }
        }
    }
}

fn assert_source_first_flags_are_actual_provenance_not_self_attestation(value: &Value) {
    for legacy_flag in [
        "runtime_reparsed_source",
        "name_dispatch_used",
        "expected_json_lookup_used",
        "accepted_runtime_core_or_authority_injection",
        "accepted_runtime_state_injection",
    ] {
        assert!(
            value.get(legacy_flag).is_none(),
            "SYS-6 CLI JSON must not expose literal legacy boolean `{legacy_flag}`; use source-first causal provenance instead: {value:#}"
        );
    }
    for nested_literal_flag in ["fixture_name_dispatch", "expected_json_lookup"] {
        assert_json_tree_lacks_key(value, nested_literal_flag);
    }

    let provenance = source_first_causal_provenance_by_kind(value);
    assert_source_bound_causal_entry(
        provenance.get("ordinary-source-bound").unwrap_or_else(|| {
            panic!("source-first evidence must include ordinary-source-bound causal entry")
        }),
        "sys5-workflow",
    );
    assert_checked_project_causal_entry(
        value,
        provenance
            .get("checked-project")
            .unwrap_or_else(|| panic!("source-first evidence must include checked-project entry")),
    );
    assert_sealed_admission_causal_entry(
        provenance
            .get("sealed-admission")
            .unwrap_or_else(|| panic!("source-first evidence must include sealed-admission entry")),
    );
    assert_generated_dispatch_causal_entry(
        value,
        provenance.get("generated-dispatch").unwrap_or_else(|| {
            panic!("source-first evidence must include generated-dispatch entry")
        }),
    );
    assert_unknown_action_admission_rejection(
        provenance
            .get("unknown-action-admission-rejection")
            .unwrap_or_else(|| {
                panic!(
                    "source-first/no-expected-result evidence must include a real unknown-action admission rejection"
                )
            }),
    );
}

fn source_first_causal_provenance_by_kind(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["source_first_causal_provenance"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.source_first_causal_provenance: {value:#}"));
    let mut by_kind = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("source-first causal entry must have id: {entry:#}"));
        let kind = entry["kind"]
            .as_str()
            .unwrap_or_else(|| panic!("source-first causal entry must name kind: {entry:#}"));
        assert!(!id.is_empty(), "source-first causal id must be nonempty");
        assert!(
            !id.starts_with("i2-property-") && !id.starts_with("literal-false"),
            "source-first causal id must not be invented/self-attested: {entry:#}"
        );
        assert_json_tree_lacks_invented_property_ref(entry, id);
        assert_executed_evidence_binds_row(entry, "i2.ordinary_source_authority");
        assert_nonempty_string(
            &entry["producer_invocation_ref"],
            &format!("{kind} producer invocation ref"),
        );
        assert_nonempty_string(
            &entry["typed_producer_ref"],
            &format!("{kind} typed producer ref"),
        );
        let previous = by_kind.insert(kind.to_string(), entry);
        assert!(
            previous.is_none(),
            "source-first causal entries must be unique by kind, duplicate: {kind}"
        );
    }
    by_kind
}

fn assert_source_bound_causal_entry(entry: &Value, expected_producer: &str) {
    assert_eq!(
        entry["source"], "actual-source-bound-causal-inventory",
        "ordinary source authority must be proven by actual source-bound causal inventory: {entry:#}"
    );
    assert_eq!(entry["produced_by"], expected_producer);
    assert_nonempty_string(
        &entry["source_content_identity_ref"],
        "source-bound causal source content identity",
    );
    assert_nonempty_string(
        &entry["logical_source_ref"],
        "source-bound causal logical source ref",
    );
    assert_nonempty_string(
        &entry["source_span_ref"],
        "source-bound causal source span ref",
    );
}

fn assert_checked_project_causal_entry(value: &Value, entry: &Value) {
    assert_eq!(
        entry["source"], "actual-checked-project",
        "checked Core/project identity must be proven by actual checked project provenance: {entry:#}"
    );
    assert!(matches!(
        entry["produced_by"].as_str(),
        Some("sys3-projection" | "sys5-workflow")
    ));
    let checked_ref = entry["checked_program_identity_ref"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("checked-project entry must carry checked_program_identity_ref: {entry:#}")
        });
    assert!(
        inventory_set(value, "checked_program_identity_refs").contains(checked_ref),
        "checked-project checked_program_identity_ref must cross-join checked inventory: {checked_ref}"
    );
}

fn assert_sealed_admission_causal_entry(entry: &Value) {
    assert_eq!(
        entry["source"], "actual-sealed-admission",
        "source-first evidence must bind the checked project to a sealed admission, not a fixture name: {entry:#}"
    );
    assert_eq!(entry["produced_by"], "sys5-workflow");
    assert_nonempty_string(&entry["sealed_admission_ref"], "sealed admission ref");
    assert_eq!(entry["manual_route_or_interface_admitted"], false);
    assert_eq!(entry["runtime_core_or_authority_injection_admitted"], false);
    assert_eq!(entry["runtime_state_injection_admitted"], false);
}

fn assert_generated_dispatch_causal_entry(value: &Value, entry: &Value) {
    assert_eq!(
        entry["source"], "actual-generated-dispatch",
        "source-first/no-name-dispatch claim must bind actual dispatch provenance: {entry:#}"
    );
    assert_eq!(entry["produced_by"], "sys4-dispatch");
    assert_eq!(entry["routing_source"], "generated_communication_plan");
    assert_nonempty_string(
        &entry["communication_plan_ref"],
        "generated dispatch communication plan ref",
    );
    let edge = entry["edge_ref"]
        .as_str()
        .unwrap_or_else(|| panic!("generated-dispatch edge_ref must be string: {entry:#}"));
    let occurrence = entry["dispatch_occurrence_ref"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("generated-dispatch dispatch_occurrence_ref must be string: {entry:#}")
        });
    assert!(
        inventory_set(value, "communication_edge_refs").contains(edge),
        "generated dispatch edge_ref must cross-join actual communication edge inventory: {edge}"
    );
    assert!(
        inventory_set(value, "runtime_occurrence_refs").contains(occurrence),
        "generated dispatch occurrence must cross-join actual runtime occurrence inventory: {occurrence}"
    );
}

fn assert_unknown_action_admission_rejection(entry: &Value) {
    assert_eq!(
        entry["source"], "actual-unknown-action-admission-candidate",
        "source-first/no-expected-result evidence must use an actual rejected unknown action candidate: {entry:#}"
    );
    assert!(matches!(
        entry["produced_by"].as_str(),
        Some("sys4-dispatch" | "sys5-workflow")
    ));
    assert_eq!(entry["candidate_kind"], "unknown-source-action");
    assert_eq!(entry["accepted"], false);
    assert_eq!(entry["state_unchanged"], true);
    assert_eq!(entry["mutation_applied"], false);
    assert!(matches!(
        entry["diagnostic_code"].as_str(),
        Some("UnknownSourceAction" | "SourceActionNotAdmitted")
    ));
    assert_nonempty_string(
        &entry["candidate_ref"],
        "unknown action rejected candidate ref",
    );
    assert_nonempty_string(
        &entry["state_before_ref"],
        "unknown action state before ref",
    );
    assert_nonempty_string(&entry["state_after_ref"], "unknown action state after ref");
    assert_eq!(entry["state_before_ref"], entry["state_after_ref"]);
}

fn assert_owner_preservation_subclaim(value: &Value) {
    let artifact_row = value["rows"]
        .as_array()
        .expect("rows should be array")
        .iter()
        .find(|row| row["id"] == "i2.core_to_locus_artifacts")
        .expect("core_to_locus_artifacts row should exist");
    let subclaims = artifact_row["subclaims"].as_array().unwrap_or_else(|| {
        panic!(
            "core_to_locus_artifacts row must carry owner-preservation subclaims: {artifact_row:#}"
        )
    });
    let subclaim = subclaims
        .iter()
        .find(|subclaim| subclaim["id"] == "owner-preservation-worldauthority-attack")
        .unwrap_or_else(|| panic!("missing owner-preservation subclaim: {artifact_row:#}"));
    assert_eq!(subclaim["status"], "pass");
    assert_eq!(subclaim["operation_id"], "attack");
    assert_eq!(subclaim["expected_owner_locus"], "WorldAuthority");
    assert_eq!(subclaim["observed_owner_locus"], "WorldAuthority");
}

fn assert_row_specific_anchors(value: &Value) {
    assert_actual_provenance_anchor(
        value,
        "i2.relation_projection_coherence",
        "relation-fallback-lineage",
        &["workflow", "project"],
        &["sys5-workflow", "sys3-projection"],
        "causal_segment_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.designated_evaluator_non_reexecution",
        "designated-result-delivery",
        &["workflow", "project"],
        &["sys5-workflow", "sys3-projection"],
        "causal_segment_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.save_restore_consistent_local_cut",
        "save-cut-lifecycle",
        &["workflow"],
        &["sys5-workflow"],
        "lifecycle_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.patch_lifecycle_checked",
        "patch-lifecycle",
        &["workflow"],
        &["sys5-workflow"],
        "lifecycle_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.owner_data_race_freedom_selected_backend",
        "st-ow-refinement-model",
        &["model"],
        &["sys2-model"],
        "model_ref",
    );

    for row_id in [
        "i2.save_restore_consistent_local_cut",
        "i2.patch_lifecycle_checked",
        "i2.owner_data_race_freedom_selected_backend",
    ] {
        let row = row(value, row_id);
        assert_optional_actual_or_typed_non_applicable_ref(value, row, "core_ref", "core_refs");
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "artifact_ref",
            "artifact_refs",
        );
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "edge_ref",
            "communication_edge_refs",
        );
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "request_identity",
            "request_identity_refs",
        );
    }
}

fn assert_actual_provenance_anchor(
    value: &Value,
    row_id: &str,
    expected_kind: &str,
    allowed_domains: &[&str],
    allowed_producers: &[&str],
    required_anchor_field: &str,
) {
    let row = row(value, row_id);
    assert_json_tree_lacks_invented_property_ref(row, row_id);
    let anchor_ref = row["provenance_anchor_ref"]
        .as_str()
        .unwrap_or_else(|| panic!("row {row_id} must expose provenance_anchor_ref: {row:#}"));
    assert!(
        !anchor_ref.is_empty() && !anchor_ref.starts_with("i2-property-"),
        "row {row_id} provenance_anchor_ref must be actual, not invented: {anchor_ref}"
    );
    let anchors = provenance_anchor_inventory(value);
    let anchor = anchors.get(anchor_ref).unwrap_or_else(|| {
        panic!("row {row_id} provenance_anchor_ref must cross-join inventories.provenance_anchors: {anchor_ref}")
    });
    assert_eq!(anchor["id"], anchor_ref);
    assert_eq!(
        anchor["kind"], expected_kind,
        "row {row_id} must bind a property-specific actual provenance kind"
    );
    assert!(
        allowed_domains
            .iter()
            .any(|domain| anchor["domain"].as_str() == Some(domain)),
        "row {row_id} provenance anchor must come from an actual accepted domain {allowed_domains:?}: {anchor:#}"
    );
    assert!(
        allowed_producers
            .iter()
            .any(|producer| anchor["produced_by"].as_str() == Some(producer)),
        "row {row_id} provenance anchor must name the actual producing subsystem {allowed_producers:?}: {anchor:#}"
    );
    assert_eq!(
        anchor["source"], "actual",
        "row {row_id} provenance anchor must be actual, not synthetic"
    );
    assert_executed_evidence_binds_row(anchor, row_id);
    assert_nonempty_string(
        &anchor[required_anchor_field],
        &format!("{row_id} {required_anchor_field}"),
    );
    assert_json_tree_lacks_invented_property_ref(anchor, anchor_ref);
}

fn provenance_anchor_inventory(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["provenance_anchors"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.provenance_anchors: {value:#}"));
    let mut by_id = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("provenance anchor entry should have id: {entry:#}"));
        assert!(
            !id.starts_with("i2-property-"),
            "provenance anchor IDs must not use invented i2-property-* refs: {entry:#}"
        );
        assert_nonempty_string(&entry["domain"], "provenance anchor domain");
        assert_nonempty_string(&entry["produced_by"], "provenance anchor producer");
        assert_nonempty_string(&entry["kind"], "provenance anchor kind");
        assert_has_binding_field(entry);
        assert_json_tree_lacks_invented_property_ref(entry, id);
        let previous = by_id.insert(id.to_string(), entry);
        assert!(
            previous.is_none(),
            "provenance anchor IDs must be unique, duplicate id: {id}"
        );
    }
    by_id
}

fn assert_optional_actual_or_typed_non_applicable_ref(
    value: &Value,
    row: &Value,
    field: &str,
    inventory: &str,
) {
    let row_id = row["id"].as_str().unwrap_or("<unknown>");
    let Some(candidate) = row.get(field) else {
        return;
    };
    if candidate.is_null() {
        return;
    }
    if let Some(text) = candidate.as_str() {
        if text.is_empty() {
            return;
        }
        assert!(
            !text.starts_with("i2-property-"),
            "row {row_id} field {field} must not use invented i2-property-* refs: {text}"
        );
        let inventory = inventory_set(value, inventory);
        assert!(
            inventory.contains(text),
            "row {row_id} field {field} must be absent, typed non-applicable, or cross-join an actual inventory entry: {text}"
        );
        return;
    }
    if let Some(object) = candidate.as_object() {
        assert_eq!(
            object.get("applicability").and_then(Value::as_str),
            Some("not-applicable"),
            "row {row_id} field {field} object must be typed non-applicable when no actual ref exists: {candidate:#}"
        );
        assert_nonempty_string(
            object.get("reason").unwrap_or(&Value::Null),
            &format!("{row_id} {field} non-applicable reason"),
        );
        return;
    }
    panic!(
        "row {row_id} field {field} must be absent, typed non-applicable, or an actual inventory ref: {candidate:#}"
    );
}

fn assert_selected_backend_actions_are_typed_successes(value: &Value) {
    let row = row(value, "i2.st_ow_selected_correspondence");
    for field in ["st_backend_telemetry", "ow1_backend_telemetry"] {
        let telemetry = &row[field];
        assert_eq!(
            telemetry["all_actions_succeeded"], true,
            "{field} must report all selected generated actions as typed successes"
        );
        let outcomes = telemetry["action_outcomes"]
            .as_array()
            .unwrap_or_else(|| panic!("{field}.action_outcomes must be an array: {telemetry:#}"));
        assert!(
            outcomes.len() >= 3,
            "{field}.action_outcomes must include the selected owner RMW, designated publish, and consume actions: {telemetry:#}"
        );
        for outcome in outcomes {
            assert!(
                outcome.as_object().is_some(),
                "{field}.action_outcomes entries must be typed objects, not string classifications: {outcome:#}"
            );
            assert_nonempty_string(&outcome["action_ref"], &format!("{field} action_ref"));
            assert_eq!(outcome["attempted"], true);
            assert_eq!(outcome["completed"], true);
            assert_eq!(
                outcome["status"], "typed_success",
                "{field} action must be a typed success, not a typed rejection or missing result: {outcome:#}"
            );
            assert!(
                !matches!(
                    outcome["result_kind"].as_str(),
                    Some("missing" | "rejected" | "typed_rejection")
                ),
                "{field} action result_kind must not be missing or rejected: {outcome:#}"
            );
            assert_nonempty_string(
                &outcome["typed_result_ref"],
                &format!("{field} typed_result_ref"),
            );
            assert_nonempty_string(
                &outcome["receipt_occurrence_ref"],
                &format!("{field} receipt_occurrence_ref"),
            );
            let attempted_provenance = outcome["attempted_provenance_ref"]
                .as_str()
                .unwrap_or_else(|| {
                    panic!(
                        "{field} action must expose attempted_provenance_ref from actual dispatch telemetry: {outcome:#}"
                    )
                });
            assert!(
                !attempted_provenance.is_empty()
                    && !attempted_provenance.starts_with("const:")
                    && !attempted_provenance.starts_with("i2-property-"),
                "{field} attempted_provenance_ref must not be const/invented: {outcome:#}"
            );
            assert!(
                outcome
                    .get("diagnostic_code")
                    .is_none_or(|diagnostic| diagnostic.is_null() || diagnostic == ""),
                "{field} typed success action must not carry a rejection diagnostic: {outcome:#}"
            );
        }
    }
    assert_eq!(
        row["ow1_backend_telemetry"]["same_mailbox_fifo_control"]["second_enqueued_before_first_serve"],
        true,
        "OW1 FIFO control must exercise a real two-message same-mailbox ordering pressure case"
    );
}

fn assert_owner_data_race_runtime_row_and_model_ordering_evidence_are_separate(value: &Value) {
    assert_eq!(
        row(value, "i2.owner_data_race_freedom_selected_backend")["evidence_class"],
        "runtime-monitored",
        "owner data-race row is a runtime backend observation; bounded ordering evidence is separate"
    );

    let executed = executed_evidence_inventory(value);
    let runtime_evidence = executed
        .get("i2-evidence:selected-backend-positive")
        .expect("selected backend runtime evidence should exist");
    assert_eq!(runtime_evidence["evidence_class"], "runtime-monitored");
    assert_executed_evidence_binds_row(
        runtime_evidence,
        "i2.owner_data_race_freedom_selected_backend",
    );

    let model_evidence = executed
        .get("i2-evidence:model-required-edge-detected")
        .expect("bounded model ordering evidence should exist");
    assert_eq!(model_evidence["evidence_class"], "model-checked-bounded");
    assert_eq!(
        model_evidence["diagnostic_code"],
        "MissingOwnerRequestServeEdge"
    );
    assert_executed_evidence_binds_row(
        model_evidence,
        "i2.owner_data_race_freedom_selected_backend",
    );
}

fn assert_selected_ow1_row_uses_selected_source_actual_anchor(value: &Value, worker_locus: &str) {
    let row = row(value, "i2.st_ow_selected_correspondence");
    let anchor_ref = row["provenance_anchor_ref"].as_str().unwrap_or_else(|| {
        panic!("selected ST/OW row must expose selected OW1 provenance_anchor_ref: {row:#}")
    });
    let anchors = provenance_anchor_inventory(value);
    let anchor = anchors.get(anchor_ref).unwrap_or_else(|| {
        panic!(
            "selected ST/OW row provenance anchor must cross-join provenance inventory: {anchor_ref}"
        )
    });
    assert_eq!(
        anchor["kind"], "selected-st-ow-correspondence",
        "ST/OW row must not reuse the primary toy attack anchor"
    );
    assert_eq!(
        anchor["source"], "actual-selected-ow1-source",
        "ST/OW row anchor must come from the selected OW1 source, not the primary WorldAuthority toy"
    );
    assert_eq!(anchor["owner_worker_locus"], worker_locus);
    assert_eq!(anchor["requester_locus"], "A");
    assert_eq!(anchor["designated_evaluator_locus"], "E");
    assert_eq!(anchor["consumer_locus"], "C");
    assert_json_array_contains_all(&anchor["source_loci"], &["A", worker_locus, "E", "C"]);
    let rendered_anchor = serde_json::to_string(anchor).expect("anchor JSON should serialize");
    for primary_locus in ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"] {
        assert!(
            !rendered_anchor.contains(primary_locus),
            "selected ST/OW provenance anchor must not point at primary toy locus {primary_locus}: {anchor:#}"
        );
    }
    for field in [
        "core_ref",
        "artifact_ref",
        "edge_ref",
        "request_identity",
        "locus_program_ref",
    ] {
        let value = row[field]
            .as_str()
            .unwrap_or_else(|| panic!("selected ST/OW row {field} must be a selected-source ref"));
        for primary_locus in ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"] {
            assert!(
                !value.contains(primary_locus),
                "selected ST/OW row {field} must use selected OW1 refs/loci, not primary toy {primary_locus}: {row:#}"
            );
        }
    }
}

fn assert_observer_sensitive_evidence_binds_actual_marker_candidate_and_redacted_output(
    value: &Value,
) {
    let executed = executed_evidence_inventory(value);
    let evidence = executed
        .get("i2-evidence:observer-sensitive-scan-detected")
        .unwrap_or_else(|| {
            panic!(
                "accepted I2 report must include observer-sensitive marker/redaction control evidence: {value:#}"
            )
        });
    assert_executed_evidence_binds_row(evidence, "i2.visibility_redaction_preserved");
    assert_eq!(evidence["executed"], true);
    assert_eq!(evidence["outcome"], "detected");
    assert_eq!(evidence["diagnostic_code"], "ObserverSensitiveIdentifier");
    assert_eq!(
        evidence["candidate_source"], "actual-marker-bearing-report-candidate",
        "observer evidence must hash the actual marker-bearing report candidate, not a static safe constant: {evidence:#}"
    );
    assert_eq!(
        evidence["redacted_output_source"], "actual-redacted-serialized-output",
        "observer evidence must hash the actual redacted serialized output, not a static redacted constant: {evidence:#}"
    );
    assert_eq!(evidence["marker_present_in_candidate"], true);
    assert_eq!(evidence["marker_absent_after_redaction"], true);
    assert_nonempty_string(
        &evidence["marker_bearing_report_candidate_ref"],
        "observer marker-bearing report candidate ref",
    );
    assert_nonempty_string(
        &evidence["redacted_serialized_output_ref"],
        "observer redacted serialized output ref",
    );
    assert_eq!(
        evidence["candidate_identity_before"], evidence["marker_bearing_report_candidate_ref"],
        "observer candidate_identity_before must equal the hash returned from the actual marker-bearing candidate"
    );
    assert_eq!(
        evidence["candidate_identity_after"], evidence["redacted_serialized_output_ref"],
        "observer candidate_identity_after must equal the hash returned from the actual redacted serialized output"
    );
    assert_ne!(
        evidence["candidate_identity_before"], evidence["candidate_identity_after"],
        "observer evidence must bind a real before/after redaction delta"
    );
}

fn assert_lifecycle_boundary_evidence_is_actual_typed_candidate(value: &Value) {
    let lifecycle = &value["lifecycle_state"];
    let executed = executed_evidence_inventory(value);
    let evidence = executed
        .get("i2-evidence:lifecycle-boundary-detected")
        .unwrap_or_else(|| {
            panic!(
                "accepted I2 CLI report must include actual lifecycle boundary control evidence: {value:#}"
            )
        });
    assert_executed_evidence_binds_row(evidence, "i2.non_claims_and_lifecycle_boundaries");
    assert_eq!(evidence["executed"], true);
    assert_eq!(evidence["outcome"], "detected");
    assert_eq!(evidence["diagnostic_code"], "LifecycleBoundaryOverclaim");
    assert_eq!(
        evidence["candidate_source"],
        "actual-lifecycle-boundary-candidate"
    );
    assert_nonempty_string(
        &evidence["producer_invocation_ref"],
        "lifecycle boundary producer invocation ref",
    );
    assert_nonempty_string(
        &evidence["candidate_identity_before"],
        "lifecycle boundary candidate before",
    );
    assert_nonempty_string(
        &evidence["candidate_identity_after"],
        "lifecycle boundary candidate after",
    );
    assert_ne!(
        evidence["candidate_identity_before"], evidence["candidate_identity_after"],
        "lifecycle boundary evidence must bind an actual overclaim candidate delta"
    );

    let observed_candidate = &evidence["lifecycle_boundary_candidate"];
    assert_eq!(
        observed_candidate["source"], "actual-lifecycle-boundary-candidate",
        "normal lifecycle row must derive from a typed lifecycle candidate, not static non_claims text: {observed_candidate:#}"
    );
    for field in [
        "broad_i1_exit_accepted",
        "i2_entry_accepted",
        "i2_exit_accepted",
        "sys7_goal_active",
        "i3_program_active",
        "public_transport_claim",
        "real_transport_selected",
        "production_deployment_claim",
    ] {
        assert_eq!(
            observed_candidate[field], lifecycle[field],
            "normal lifecycle row must derive {field} from the typed lifecycle candidate"
        );
    }

    let overclaim_candidate = &evidence["overclaim_candidate"];
    assert_eq!(
        overclaim_candidate["source"], "actual-lifecycle-boundary-candidate",
        "lifecycle falsifier control must mutate a typed lifecycle candidate"
    );
    assert_eq!(overclaim_candidate["i2_exit_accepted"], true);
    assert_eq!(overclaim_candidate["public_transport_claim"], true);
    assert_eq!(overclaim_candidate["accepted"], false);
    assert_eq!(overclaim_candidate["mutation_applied"], false);
    assert_nonempty_string(
        &overclaim_candidate["candidate_ref"],
        "lifecycle overclaim candidate ref",
    );
}

fn assert_bounded_implementation_source_fingerprint(value: &Value) {
    assert_json_tree_lacks_key(value, "implementation_cut");
    let fingerprint = &value["bounded_implementation_source_fingerprint"];
    assert!(
        fingerprint.as_object().is_some(),
        "provisional conform-i2 report must expose bounded_implementation_source_fingerprint as typed metadata, not an implementation_cut release claim: {fingerprint:#}"
    );
    assert_nonempty_string(
        &fingerprint["id"],
        "bounded implementation source fingerprint id",
    );
    assert!(
        fingerprint["id"]
            .as_str()
            .is_some_and(|id| id.starts_with("i2-bounded-implementation-source-sha256-v1:")),
        "bounded implementation source fingerprint must be I2/provisional-source namespaced, not a Git/release cut: {fingerprint:#}"
    );
    assert_eq!(
        fingerprint["scope"], "i2-provisional-implementation-source",
        "bounded implementation fingerprint scope must not claim public/release acceptance"
    );
    assert_eq!(
        fingerprint["runtime_identity_claim"], false,
        "bounded implementation source fingerprint is evidence metadata, not runtime identity"
    );
    assert_eq!(
        fingerprint["public_release_cut"], false,
        "bounded implementation source fingerprint must not claim a public release cut"
    );
    assert_json_array_contains_all(
        &fingerprint["source_components"],
        &[
            "crates/mir-runtime/src/sys6_i2_conformance.rs",
            "crates/mir-runtime/src/sys2_execution_backend.rs",
        ],
    );
    assert!(
        fingerprint
            .get("acceptance_metadata")
            .is_none_or(|metadata| metadata
                .get("exact_git_cut")
                .is_none_or(|cut| cut.is_string())),
        "exact Git cut, if present, belongs only in later acceptance_metadata.exact_git_cut: {fingerprint:#}"
    );
}

fn inventory_set(value: &Value, field: &str) -> BTreeSet<String> {
    value["inventories"][field]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventory {field}: {value:#}"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("inventory {field} entry should be string: {entry:#}"))
                .to_string()
        })
        .collect()
}

fn assert_json_tree_lacks_invented_property_ref(value: &Value, label: &str) {
    let rendered = serde_json::to_string(value).expect("JSON tree should serialize");
    assert!(
        !rendered.contains("i2-property-"),
        "{label} must not contain invented i2-property-* refs: {rendered}"
    );
}

fn assert_json_tree_lacks_key(value: &Value, denied_key: &str) {
    match value {
        Value::Object(object) => {
            assert!(
                !object.contains_key(denied_key),
                "provisional conform-i2 report must not expose `{denied_key}`; use bounded_implementation_source_fingerprint for source fingerprint metadata: {value:#}"
            );
            for child in object.values() {
                assert_json_tree_lacks_key(child, denied_key);
            }
        }
        Value::Array(entries) => {
            for child in entries {
                assert_json_tree_lacks_key(child, denied_key);
            }
        }
        _ => {}
    }
}

fn row<'a>(value: &'a Value, id: &str) -> &'a Value {
    value["rows"]
        .as_array()
        .expect("rows should be an array")
        .iter()
        .find(|row| row["id"] == id)
        .unwrap_or_else(|| panic!("missing row {id}: {value:#}"))
}

fn executed_evidence_inventory(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["executed_evidence"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.executed_evidence: {value:#}"));
    let mut by_id = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("executed evidence entry should have id: {entry:#}"));
        assert_nonempty_string(&entry["kind"], "executed evidence kind");
        assert_nonempty_string(&entry["outcome"], "executed evidence outcome");
        assert_has_binding_field(entry);
        let previous = by_id.insert(id.to_string(), entry);
        assert!(
            previous.is_none(),
            "executed evidence IDs must be unique, duplicate id: {id}"
        );
    }
    by_id
}

fn assert_has_binding_field(evidence: &Value) {
    let has_row_ids = evidence["row_ids"]
        .as_array()
        .is_some_and(|row_ids| !row_ids.is_empty());
    let has_property_ids = evidence["property_ids"]
        .as_array()
        .is_some_and(|property_ids| !property_ids.is_empty());
    assert!(
        has_row_ids || has_property_ids,
        "executed evidence must bind at least one row_ids or property_ids entry: {evidence:#}"
    );
}

fn assert_executed_evidence_binds_row(evidence: &Value, row_id: &str) {
    assert_has_binding_field(evidence);
    let row_bound = evidence["row_ids"]
        .as_array()
        .is_some_and(|row_ids| row_ids.iter().any(|id| id.as_str() == Some(row_id)));
    let property_bound = evidence["property_ids"]
        .as_array()
        .is_some_and(|property_ids| property_ids.iter().any(|id| id.as_str() == Some(row_id)));
    assert!(
        row_bound || property_bound,
        "executed evidence referenced by row {row_id} must bind that row through row_ids or property_ids: {evidence:#}"
    );
}

fn assert_json_array_contains_all(value: &Value, expected: &[&str]) {
    let array = value.as_array().expect("value should be a JSON array");
    for fragment in expected {
        assert!(
            array.iter().any(|entry| entry.as_str() == Some(fragment)),
            "array missing `{fragment}`: {value:#}"
        );
    }
}

fn assert_nonempty_string(value: &Value, label: &str) {
    assert!(
        value.as_str().is_some_and(|text| !text.is_empty()),
        "{label} should be a nonempty string: {value:#}"
    );
}

fn assert_nonempty_array(value: &Value, label: &str) {
    assert!(
        value.as_array().is_some_and(|entries| !entries.is_empty()),
        "{label} should be a nonempty array: {value:#}"
    );
}

fn assert_absent(value: &Value, field: &str) {
    assert!(
        value.get(field).is_none(),
        "{field} must be absent; literal false is not source-first evidence: {value:#}"
    );
}

fn assert_observer_safe_cli_payload(value: &Value, output: &Output) {
    let rendered = serde_json::to_string(value).expect("JSON payload should serialize");
    let raw_stdout = String::from_utf8_lossy(&output.stdout);
    let raw_stderr = String::from_utf8_lossy(&output.stderr);
    for denied in [
        "/home/",
        "/root/",
        "/tmp/",
        "avatar[self].atk",
        "avatar[target].hp =",
        "participant_input[self].focus +",
        "raw_source",
        "source_text",
        "raw_authority",
        "raw_capability",
        "raw_witness",
        "raw_credential",
        "capability_secret",
        "witness_secret",
        "credential_secret",
        "host_absolute_path",
        "expected_result",
        SECRET_MARKER,
    ] {
        assert!(
            !rendered.contains(denied)
                && !raw_stdout.contains(denied)
                && !raw_stderr.contains(denied),
            "observer-safe CLI output leaked denied fragment `{denied}`:\nstdout:\n{raw_stdout}\nstderr:\n{raw_stderr}"
        );
    }
}

fn assert_dynamic_markers_absent_from_cli_payload(
    value: &Value,
    output: &Output,
    markers: &[String],
) {
    let rendered = serde_json::to_string(value).expect("JSON payload should serialize");
    let raw_stdout = String::from_utf8_lossy(&output.stdout);
    let raw_stderr = String::from_utf8_lossy(&output.stderr);
    for marker in markers {
        assert!(
            !marker.is_empty(),
            "dynamic observer marker under test must be nonempty"
        );
        assert!(
            !rendered.contains(marker)
                && !raw_stdout.contains(marker)
                && !raw_stderr.contains(marker),
            "CLI serialization must route through observer-safe report rendering; dynamic source-controlled marker/path leaked: {marker}"
        );
    }
}
