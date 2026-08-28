use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

const TOY_SOURCE: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const PLUS_TWO_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir";
const OWNER_RMW_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir";

#[test]
fn sys5_project_loci_cli_projects_canonical_source_into_generated_artifacts() {
    let output = run_cli(&["project-loci", TOY_SOURCE, "--format", "json"]);
    let value = json_stdout(&output);

    assert!(
        output.status.success(),
        "project-loci should accept the canonical SYS-5 ordinary .mir source: {value:#}"
    );
    assert_eq!(value["status"], "ok");
    assert_eq!(value["command"], "project-loci");
    assert_eq!(value["source_authority"], "ordinary_mir_source");
    assert_eq!(value["profile_name"], "sys5-local-slice");
    assert_eq!(
        value["profile_status"],
        "provisional-no-compatibility-promise"
    );
    assert_eq!(value["public_api_or_wire_contract"], false);
    assert_eq!(value["final_public_api_frozen"], false);
    assert_eq!(value["public_wire_frozen"], false);
    assert_eq!(value["requires_runtime_execution"], false);
    assert_json_array_contains_all(
        &value["loci"],
        &["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    );
    assert_json_array_len(&value["loci"], 4);
    assert_json_array_has_object(&value["locus_programs"], "locus", "WorldAuthority");
    assert_json_array_has_object(&value["locus_programs"], "locus", "ParticipantA");
    assert_json_array_has_object(&value["locus_programs"], "locus", "ParticipantB");
    assert_json_array_has_object(&value["locus_programs"], "locus", "ViewerC");
    assert_json_array_has_object(&value["generated_communication"], "kind", "owner-request");
    assert_json_array_has_object(
        &value["generated_communication"],
        "kind",
        "relation-projection-publication",
    );
    assert_json_array_has_object(
        &value["generated_communication"],
        "kind",
        "designated-result-delivery",
    );
    assert_json_array_has_object(
        &value["source_core_artifact_mappings"],
        "operation_id",
        "attack",
    );
    assert_observer_safe_cli_payload(&value, &output.stdout);
}

#[test]
fn sys5_run_local_and_inspect_cli_emit_joined_deterministic_observer_safe_workflow() {
    let first = run_cli(&[
        "run-local",
        TOY_SOURCE,
        "--patch",
        PLUS_TWO_PATCH,
        "--patch",
        OWNER_RMW_PATCH,
        "--format",
        "json",
    ]);
    let first_value = json_stdout(&first);
    assert!(
        first.status.success(),
        "run-local should execute the canonical SYS-5 workflow from source and source-first patches: {first_value:#}"
    );
    assert_eq!(first_value["status"], "ok");
    assert_eq!(first_value["command"], "run-local");
    assert_eq!(first_value["source_authority"], "ordinary_mir_source");
    assert_eq!(first_value["runtime_profile"], "ST");
    assert_eq!(first_value["local_fabric_instance_count"], 1);
    assert_eq!(first_value["public_api_or_wire_contract"], false);
    assert_eq!(first_value["final_public_api_frozen"], false);
    assert_eq!(first_value["public_wire_frozen"], false);
    assert_nonempty_string(
        &first_value["checked_program_identity_ref"],
        "checked program identity",
    );
    assert_json_array_contains_all(
        &first_value["loci"],
        &["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    );
    assert_json_array_has_object(&first_value["locus_programs"], "locus", "WorldAuthority");
    assert_json_array_has_object(&first_value["locus_programs"], "locus", "ParticipantA");
    assert_json_array_has_object(&first_value["locus_programs"], "locus", "ParticipantB");
    assert_json_array_has_object(&first_value["locus_programs"], "locus", "ViewerC");
    assert_json_array_has_object(
        &first_value["generated_communication"],
        "kind",
        "owner-request",
    );
    assert_json_array_has_object(
        &first_value["generated_communication"],
        "kind",
        "relation-projection-publication",
    );
    assert_json_array_has_object(
        &first_value["generated_communication"],
        "kind",
        "designated-result-delivery",
    );
    assert_base_mappings_use_cli_source(&first_value["source_core_artifact_mappings"]);
    assert_attack_mapping_has_source_span(&first_value["source_core_artifact_mappings"]);
    assert_patch_provenance_links_verdicts(&first_value);
    assert_runtime_summary_exposes_observer_safe_state_relation_designated_cache(
        &first_value["runtime_summary"],
    );
    assert_joined_rows_include_typed_causal_segment(&first_value["joined_rows"]);
    assert_json_array_contains_all(
        &first_value["actual_steps"],
        &[
            "startup",
            "attack",
            "designated_publish",
            "viewer_consume",
            "relation_primary",
            "presentation_gap",
            "participant_a_leave",
            "fresh_reacquire",
            "save",
            "restore",
            "patch_accepted",
            "patch_rejected",
            "consumer_capability_revoke",
            "failed_consume",
            "verification",
        ],
    );
    assert_json_array_lacks_string(&first_value["actual_steps"], "relation_invalidate_fallback");
    assert_json_array_has_object(&first_value["patch_verdicts"], "verdict", "accepted");
    assert_json_array_has_object(
        &first_value["patch_verdicts"],
        "diagnostic",
        "OwnerRmwExpressionChanged",
    );
    assert_json_array_has_object(
        &first_value["typed_failures"],
        "diagnostic",
        "MissingConsumerCapability",
    );
    assert_json_array_has_object(&first_value["joined_rows"], "kind", "source_span");
    assert_json_array_has_object(&first_value["joined_rows"], "kind", "runtime_occurrence");
    assert_json_array_has_object(&first_value["joined_rows"], "kind", "presentation_gap");
    assert_observer_safe_cli_payload(&first_value, &first.stdout);

    let second = run_cli(&[
        "run-local",
        TOY_SOURCE,
        "--patch",
        PLUS_TWO_PATCH,
        "--patch",
        OWNER_RMW_PATCH,
        "--format",
        "json",
    ]);
    assert!(
        second.status.success(),
        "second run-local should also succeed: {:#}",
        json_stdout(&second)
    );
    assert_eq!(
        first.stdout, second.stdout,
        "run-local should be deterministic for the same source, patch contents, and bounded workflow schedule"
    );

    let inspect = run_cli(&[
        "inspect",
        TOY_SOURCE,
        "--patch",
        PLUS_TWO_PATCH,
        "--patch",
        OWNER_RMW_PATCH,
        "--format",
        "json",
    ]);
    let inspect_value = json_stdout(&inspect);
    assert!(
        inspect.status.success(),
        "inspect should expose the joined source->Core->artifact->communication->runtime view: {inspect_value:#}"
    );
    assert_eq!(inspect_value["status"], "ok");
    assert_eq!(inspect_value["command"], "inspect");
    assert_eq!(inspect_value["source_authority"], "ordinary_mir_source");
    assert_eq!(inspect_value["public_api_or_wire_contract"], false);
    assert_nonempty_string(
        &inspect_value["checked_program_identity_ref"],
        "checked program identity",
    );
    assert_json_array_has_object(&inspect_value["locus_programs"], "locus", "WorldAuthority");
    assert_json_array_has_object(
        &inspect_value["generated_communication"],
        "kind",
        "owner-request",
    );
    assert_base_mappings_use_cli_source(&inspect_value["source_core_artifact_mappings"]);
    assert_attack_mapping_has_source_span(&inspect_value["source_core_artifact_mappings"]);
    assert_patch_provenance_links_verdicts(&inspect_value);
    assert_runtime_summary_exposes_observer_safe_state_relation_designated_cache(
        &inspect_value["runtime_summary"],
    );
    assert_json_array_has_object(&inspect_value["joined_rows"], "kind", "source_span");
    assert_json_array_has_object(
        &inspect_value["joined_rows"],
        "kind",
        "generated_communication_edge",
    );
    assert_json_array_has_object(&inspect_value["joined_rows"], "kind", "runtime_occurrence");
    assert_joined_rows_include_typed_causal_segment(&inspect_value["joined_rows"]);
    assert_observer_safe_cli_payload(&inspect_value, &inspect.stdout);
}

#[test]
fn sys5_cli_rejects_missing_malformed_and_filename_based_patch_verdicts_typed_nonzero() {
    let missing_source = run_cli(&[
        "project-loci",
        "samples/clean-near-end/mirrorea-i2-local-toy/missing.mir",
        "--format",
        "json",
    ]);
    let missing_value = json_stdout(&missing_source);
    assert!(!missing_source.status.success());
    assert_eq!(missing_value["status"], "error");
    assert_eq!(missing_value["command"], "project-loci");
    assert_eq!(missing_value["diagnostic_code"], "source_path_io_error");
    assert_eq!(missing_value["source_authority"], "ordinary_mir_source");

    let extra_arg = run_cli(&["inspect", TOY_SOURCE, "unexpected", "--format", "json"]);
    let extra_value = json_stdout(&extra_arg);
    assert!(!extra_arg.status.success());
    assert_eq!(extra_value["status"], "error");
    assert_eq!(extra_value["command"], "inspect");
    assert_eq!(extra_value["diagnostic_code"], "unexpected_arguments");

    let temp_dir = unique_temp_dir("sys5-cli-filename-verdict");
    let misleading_patch = temp_dir.join("designated-plus-two.mir");
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let owner_rmw_patch_text = fs::read_to_string(repo_root().join(OWNER_RMW_PATCH))
        .expect("canonical owner-RMW patch should exist once SYS-5 CLI samples are added");
    fs::write(&misleading_patch, owner_rmw_patch_text)
        .expect("misleading patch fixture should be written under temp dir");
    let misleading = run_cli(&[
        "run-local",
        TOY_SOURCE,
        "--patch",
        misleading_patch
            .to_str()
            .expect("temp patch path should be utf-8"),
        "--format",
        "json",
    ]);
    let misleading_value = json_stdout(&misleading);
    assert!(
        misleading.status.success(),
        "run-local should execute and classify patch verdicts by checked content, not by filename: {misleading_value:#}"
    );
    assert_json_array_has_object(
        &misleading_value["patch_verdicts"],
        "diagnostic",
        "OwnerRmwExpressionChanged",
    );
    assert_json_array_lacks_object(&misleading_value["patch_verdicts"], "verdict", "accepted");
    assert_observer_safe_cli_payload(&misleading_value, &misleading.stdout);
    let _ = fs::remove_dir_all(&temp_dir);
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
        .expect("mirrorea-alpha CLI should run")
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

fn json_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout should be JSON: {error}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn assert_json_array_len(value: &Value, expected: usize) {
    let array = value.as_array().expect("value should be a JSON array");
    assert_eq!(array.len(), expected, "unexpected array length: {value:#}");
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

fn assert_json_array_has_object(value: &Value, key: &str, expected: &str) {
    let array = value.as_array().expect("value should be a JSON array");
    assert!(
        array
            .iter()
            .filter_map(Value::as_object)
            .any(|object| object.get(key).and_then(Value::as_str) == Some(expected)),
        "array missing object with {key}={expected}: {value:#}"
    );
}

fn assert_json_array_lacks_object(value: &Value, key: &str, denied: &str) {
    let array = value.as_array().expect("value should be a JSON array");
    assert!(
        !array
            .iter()
            .filter_map(Value::as_object)
            .any(|object| object.get(key).and_then(Value::as_str) == Some(denied)),
        "array must not contain object with {key}={denied}: {value:#}"
    );
}

fn assert_json_array_lacks_string(value: &Value, denied: &str) {
    let array = value.as_array().expect("value should be a JSON array");
    assert!(
        !array.iter().any(|entry| entry.as_str() == Some(denied)),
        "array must not contain shortcut label `{denied}`: {value:#}"
    );
}

fn assert_nonempty_string(value: &Value, label: &str) {
    assert!(
        value.as_str().is_some_and(|text| !text.is_empty()),
        "{label} should be a nonempty string: {value:#}"
    );
}

fn assert_attack_mapping_has_source_span(value: &Value) {
    let array = value.as_array().expect("mappings should be a JSON array");
    let mapping = array
        .iter()
        .filter_map(Value::as_object)
        .find(|object| object.get("operation_id").and_then(Value::as_str) == Some("attack"))
        .unwrap_or_else(|| panic!("missing attack source/Core/artifact mapping: {value:#}"));
    assert_nonempty_string(
        mapping.get("core_ref").unwrap_or(&Value::Null),
        "attack core ref",
    );
    assert_nonempty_string(
        mapping.get("locus_program_ref").unwrap_or(&Value::Null),
        "attack locus program ref",
    );
    assert_source_span_is_nonzero(
        mapping.get("source_span").unwrap_or(&Value::Null),
        "attack mapping source span",
    );
}

fn assert_base_mappings_use_cli_source(value: &Value) {
    let array = value.as_array().expect("mappings should be a JSON array");
    assert!(
        !array.is_empty(),
        "base source/Core/artifact mappings should be present"
    );
    for mapping in array.iter().filter_map(Value::as_object) {
        assert_eq!(
            mapping.get("logical_path").and_then(Value::as_str),
            Some("cli-source.mir"),
            "base mappings must use deterministic redacted logical source path: {mapping:#?}"
        );
    }
}

fn assert_patch_provenance_links_verdicts(value: &Value) {
    let provenance = value["patch_provenance"]
        .as_array()
        .expect("patch provenance should be a JSON array");
    assert_eq!(
        provenance.len(),
        2,
        "run-local/inspect should expose both patch candidates: {provenance:#?}"
    );
    let first = patch_provenance_row(provenance, "cli-patch-001.mir");
    let second = patch_provenance_row(provenance, "cli-patch-002.mir");
    assert_ne!(
        first.get("checked_program_identity_ref"),
        second.get("checked_program_identity_ref"),
        "distinct patch contents must have distinct checked program identities"
    );
    assert_ne!(
        first.get("patch_ref"),
        second.get("patch_ref"),
        "distinct patch candidates must have distinct patch refs"
    );
    for row in [first, second] {
        assert_nonempty_string(
            row.get("checked_program_identity_ref")
                .unwrap_or(&Value::Null),
            "patch checked program identity ref",
        );
        assert_nonempty_string(row.get("patch_ref").unwrap_or(&Value::Null), "patch ref");
    }
    assert_eq!(
        first.get("verdict").and_then(Value::as_str),
        Some("accepted"),
        "first patch should link to accepted verdict: {first:#?}"
    );
    assert_eq!(
        second.get("verdict").and_then(Value::as_str),
        Some("rejected"),
        "second patch should link to rejected verdict: {second:#?}"
    );
    assert_eq!(
        second.get("diagnostic").and_then(Value::as_str),
        Some("OwnerRmwExpressionChanged"),
        "rejected patch should link to actual typed diagnostic: {second:#?}"
    );
}

fn patch_provenance_row<'a>(
    provenance: &'a [Value],
    logical_path: &str,
) -> &'a serde_json::Map<String, Value> {
    provenance
        .iter()
        .filter_map(Value::as_object)
        .find(|row| row.get("logical_path").and_then(Value::as_str) == Some(logical_path))
        .unwrap_or_else(|| {
            panic!(
                "missing patch provenance row for deterministic logical path {logical_path}: {provenance:#?}"
            )
        })
}

fn assert_source_span_is_nonzero(value: &Value, label: &str) {
    let start = value
        .get("start")
        .and_then(Value::as_u64)
        .unwrap_or_else(|| panic!("{label} missing numeric start: {value:#}"));
    let end = value
        .get("end")
        .and_then(Value::as_u64)
        .unwrap_or_else(|| panic!("{label} missing numeric end: {value:#}"));
    assert!(end > start, "{label} must be nonzero: {value:#}");
}

fn assert_runtime_summary_exposes_observer_safe_state_relation_designated_cache(value: &Value) {
    assert_nonempty_string(
        &value["observer_safe_state_digest"],
        "observer-safe state digest",
    );
    assert_nonempty_string(
        &value["relation"]["selected_anchor_ref"],
        "relation selected anchor ref",
    );
    assert_nonempty_string(&value["relation"]["floor_ref"], "relation floor ref");
    assert_nonempty_string(&value["relation"]["semantic_ref"], "relation semantic ref");
    assert_nonempty_string(&value["relation"]["lineage_ref"], "relation lineage ref");
    assert_nonempty_string(&value["designated"]["result_ref"], "designated result ref");
    assert!(
        value["designated"]["version"]
            .as_u64()
            .is_some_and(|version| version > 0),
        "designated version should be positive: {value:#}"
    );
    assert_nonempty_string(&value["cache"]["version_ref"], "cache version ref");
}

fn assert_joined_rows_include_typed_causal_segment(value: &Value) {
    let rows = value
        .as_array()
        .expect("joined rows should be a JSON array");
    assert!(
        rows.iter()
            .filter_map(Value::as_object)
            .filter(|row| row.get("kind").and_then(Value::as_str) == Some("typed_causal_segment"))
            .any(|row| {
                let detail = row.get("detail").unwrap_or(&Value::Null);
                detail
                    .get("logical_path")
                    .and_then(Value::as_str)
                    .is_some_and(|text| !text.is_empty())
                    && source_span_present(detail.get("source_span").unwrap_or(&Value::Null))
                    && nonempty_field(detail, "core_ref")
                    && nonempty_field(detail, "request_fragment_ref")
                    && nonempty_field(detail, "serve_fragment_ref")
                    && nonempty_field(detail, "edge_ref")
                    && nonempty_field(detail, "request_identity")
                    && nonempty_field(detail, "request_enqueue_occurrence_ref")
                    && nonempty_field(detail, "dispatch_occurrence_ref")
                    && nonempty_field(detail, "receive_occurrence_ref")
                    && nonempty_field(detail, "serve_occurrence_ref")
            }),
        "joined rows must include one actual typed causal segment with distinct request identity and enqueue/dispatch/receive/serve occurrence detail: {value:#}"
    );
}

fn source_span_present(value: &Value) -> bool {
    value
        .get("start")
        .and_then(Value::as_u64)
        .zip(value.get("end").and_then(Value::as_u64))
        .is_some_and(|(start, end)| end > start)
}

fn nonempty_field(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(Value::as_str)
        .is_some_and(|text| !text.is_empty())
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn assert_observer_safe_cli_payload(value: &Value, stdout: &[u8]) {
    let rendered = serde_json::to_string(value).expect("JSON payload should serialize");
    let raw_stdout = String::from_utf8_lossy(stdout);
    for denied in [
        "/home/",
        "/root/",
        "avatar[self].atk",
        "avatar[target].hp =",
        "participant_input[self].focus +",
        "private avatar atk",
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
    ] {
        assert!(
            !rendered.contains(denied) && !raw_stdout.contains(denied),
            "observer-safe CLI output leaked denied fragment `{denied}`:\n{raw_stdout}"
        );
    }
}
