use std::{
    fs,
    path::{Path, PathBuf},
};

use mir_runtime::m10_reference_system::{M10CliFacadeCommand, M10ReferenceSystem};
use serde_json::{Value, json};

const CORPUS_RELATIVE_ROOT: &str = "samples/clean-near-end/i1plus-reference";
const PROFILE_INPUT_RELATIVE_ROOT: &str = "samples/clean-near-end/i1plus-reference-profile";
const SCN02_RELATIVE_SOURCE: &str = "samples/clean-near-end/i1plus-reference/scn-02/positive.mir";
const SCN09_RELATIVE_BASE: &str = "samples/clean-near-end/i1plus-reference/scn-09/base.mir";
const SCN09_RELATIVE_CANDIDATE: &str =
    "samples/clean-near-end/i1plus-reference/scn-09/candidate-accepted.mir";
const SCN10_RELATIVE_SOURCE: &str = "samples/clean-near-end/i1plus-reference/scn-10/positive.mir";
const SCN12_RELATIVE_SOURCE: &str =
    "samples/clean-near-end/i1plus-reference/scn-12/bird-relation.mir";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn abs_path(relative: &str) -> String {
    workspace_root()
        .join(relative)
        .to_string_lossy()
        .into_owned()
}

fn deterministic_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn source_hash(path: &str) -> String {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("can read CLI source {path}: {error}"));
    deterministic_hash(&format!("{path}\0{source}"))
}

fn corpus_root() -> String {
    abs_path(CORPUS_RELATIVE_ROOT)
}

fn profile_input_json(relative: &str) -> Value {
    let path = workspace_root()
        .join(PROFILE_INPUT_RELATIVE_ROOT)
        .join(relative);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("can read profile input {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("profile input {} is valid JSON: {error}", path.display()))
}

fn collect_files(root: &Path, files: &mut Vec<String>) {
    for entry in fs::read_dir(root)
        .unwrap_or_else(|error| panic!("can read corpus directory {}: {error}", root.display()))
    {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path.to_string_lossy().into_owned());
        }
    }
}

fn corpus_source_units() -> Vec<String> {
    let mut files = Vec::new();
    collect_files(&workspace_root().join(CORPUS_RELATIVE_ROOT), &mut files);
    files.sort();
    files
}

fn schedule() -> Value {
    json!({
        "schema_version": "m10-i1plus-typed-cli-schedule-v0",
        "kind": "typed_conformance_input",
        "direct_mutation_api": false,
        "requests": [
            {"event": "attack", "principal": "self", "target": "target", "attacks": 2},
            {"event": "save_s1"},
            {"event": "load_fresh"},
            {"event": "project_relation", "relation": "bird_follow", "consumer": "Viewer"}
        ]
    })
}

fn valid_patch_intent(base: &str, candidate: &str) -> Value {
    json!({
        "id": "scn09-candidate-a",
        "kind": "source_patch_intent",
        "base_source": base,
        "candidate_source": candidate,
        "base_source_hash": source_hash(base),
        "candidate_source_hash": source_hash(candidate),
        "state_additions": [{"state": "lamp", "fields": ["enabled"]}],
        "required_capabilities": ["patch.activate"],
        "required_effects": ["observer_publish"],
        "required_failures": ["VisibilityDenied"],
        "authority_intent": {"kind": "none"}
    })
}

fn run_command(command: M10CliFacadeCommand) -> Value {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report = system
        .run_cli(command)
        .expect("M10 CLI facade command returns a typed report");
    serde_json::to_value(report).expect("M10 CLI report serializes")
}

fn run_command_result(command: M10CliFacadeCommand) -> Result<Value, String> {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    system
        .run_cli(command)
        .map(|report| serde_json::to_value(report).expect("M10 CLI report serializes"))
}

fn assert_pointer(value: &Value, pointer: &str, expected: Value) {
    assert_eq!(
        value.pointer(pointer),
        Some(&expected),
        "unexpected value at {pointer}: {value:#}"
    );
}

fn assert_pointer_absent_or_false(value: &Value, pointer: &str) {
    assert_ne!(
        value.pointer(pointer),
        Some(&json!(true)),
        "unexpected true value at {pointer}: {value:#}"
    );
}

fn assert_status(value: &Value, pointer: &str, expected: &str) {
    let actual = value
        .pointer(pointer)
        .unwrap_or_else(|| panic!("missing status at {pointer}: {value:#}"));
    let status = actual
        .as_str()
        .or_else(|| actual.pointer("/status").and_then(Value::as_str))
        .unwrap_or_else(|| {
            panic!("status at {pointer} is neither a string nor object status: {actual:#}")
        });
    assert_eq!(status, expected, "{pointer}");
}

fn source_parse_count(value: &Value, source_path: &str) -> u64 {
    value
        .pointer("/cli_pipeline/source_parse_counts")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("path").and_then(Value::as_str) == Some(source_path))
        })
        .and_then(|row| row.get("m6_parse_count"))
        .and_then(Value::as_u64)
        .unwrap_or_else(|| panic!("missing per-source parse row for {source_path}: {value:#}"))
}

fn assert_source_identity_unit(value: &Value, source_path: &str) {
    let unit = value
        .pointer("/identity/source_units")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("path").and_then(Value::as_str) == Some(source_path))
        })
        .unwrap_or_else(|| panic!("missing source identity unit for {source_path}: {value:#}"));
    assert_eq!(
        unit.pointer("/source_identity"),
        unit.pointer("/terminal_source_identity"),
        "terminal identity must be per-source for {source_path}"
    );
    assert_eq!(
        unit.pointer("/fixture_name_result_lookup_used"),
        Some(&json!(false))
    );
}

#[test]
fn cli_parse_check_elaborate_run_trace_project_save_load_and_patch_are_source_first() {
    let scn02 = abs_path(SCN02_RELATIVE_SOURCE);
    let scn09_base = abs_path(SCN09_RELATIVE_BASE);
    let scn09_candidate = abs_path(SCN09_RELATIVE_CANDIDATE);
    let scn10 = abs_path(SCN10_RELATIVE_SOURCE);
    let scn12 = abs_path(SCN12_RELATIVE_SOURCE);
    let cases: Vec<(&str, &str, M10CliFacadeCommand, Vec<String>, &str)> = vec![
        (
            "parse",
            "ordinary_mir_source",
            M10CliFacadeCommand::parse().source_path(scn02.clone()),
            vec![scn02.clone()],
            "Inspected",
        ),
        (
            "check",
            "ordinary_mir_source",
            M10CliFacadeCommand::check().source_path(scn02.clone()),
            vec![scn02.clone()],
            "Inspected",
        ),
        (
            "elaborate",
            "ordinary_mir_source",
            M10CliFacadeCommand::elaborate().source_path(scn02.clone()),
            vec![scn02.clone()],
            "Inspected",
        ),
        (
            "run",
            "ordinary_mir_source",
            M10CliFacadeCommand::run()
                .source_path(scn02.clone())
                .typed_schedule_json(schedule()),
            vec![scn02.clone()],
            "Accepted",
        ),
        (
            "trace",
            "ordinary_mir_source",
            M10CliFacadeCommand::trace()
                .source_path(scn02.clone())
                .typed_schedule_json(schedule()),
            vec![scn02.clone()],
            "Accepted",
        ),
        (
            "project",
            "ordinary_mir_source",
            M10CliFacadeCommand::project()
                .source_path(scn12.clone())
                .typed_schedule_json(schedule()),
            vec![scn12.clone()],
            "Projected",
        ),
        (
            "save",
            "ordinary_mir_source",
            M10CliFacadeCommand::save()
                .source_path(scn10.clone())
                .typed_schedule_json(schedule()),
            vec![scn10.clone()],
            "Saved",
        ),
        (
            "load",
            "ordinary_mir_source",
            M10CliFacadeCommand::load()
                .source_path(scn10.clone())
                .typed_schedule_json(schedule()),
            vec![scn10.clone()],
            "LoadedFresh",
        ),
        (
            "patch",
            "ordinary_mir_patch_source_pair",
            M10CliFacadeCommand::patch()
                .source_path(scn09_base.clone())
                .candidate_source_path(scn09_candidate.clone())
                .typed_schedule_json(schedule())
                .patch_intent_json(valid_patch_intent(&scn09_base, &scn09_candidate)),
            vec![scn09_base.clone(), scn09_candidate.clone()],
            "PatchActivated",
        ),
    ];

    for (command_name, input_kind, command, expected_sources, terminal_outcome) in cases {
        let value = run_command(command);
        assert_pointer(&value, "/command", json!(command_name));
        assert_pointer(&value, "/terminal_outcome", json!(terminal_outcome));
        assert_pointer(&value, "/facade/source_first", json!(true));
        assert_pointer(&value, "/facade/final_public_abi_claimed", json!(false));
        assert_pointer(
            &value,
            "/facade/fixture_name_result_lookup_used",
            json!(false),
        );
        assert_pointer(
            &value,
            "/facade/expected_output_sidecars_loaded",
            json!(false),
        );
        assert_pointer(&value, "/cli_pipeline/input_kind", json!(input_kind));
        assert_pointer(
            &value,
            "/cli_pipeline/m6_parse_count",
            json!(expected_sources.len()),
        );
        assert_eq!(
            value
                .pointer("/cli_pipeline/source_parse_counts")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(expected_sources.len()),
            "parse count must be represented as an exact per-source map for {command_name}: {value:#}"
        );
        for source in expected_sources {
            assert_eq!(
                source_parse_count(&value, &source),
                1,
                "{command_name} must parse {source} exactly once"
            );
            assert_source_identity_unit(&value, &source);
        }

        match command_name {
            "project" => {
                assert_pointer(&value, "/projection/relation/name", json!("bird_follow"));
                assert!(
                    value.pointer("/projection/relation/derived_pose").is_some(),
                    "project must report a real derived pose: {value:#}"
                );
                assert_pointer(
                    &value,
                    "/projection/relation/provenance/schedule_action",
                    json!("consumer_local_projection"),
                );
            }
            "save" => {
                assert_pointer(&value, "/save/consistent_cut", json!(true));
                assert!(
                    value
                        .pointer("/save/cut_identity")
                        .and_then(Value::as_str)
                        .is_some(),
                    "save must report a concrete cut identity: {value:#}"
                );
            }
            "load" => {
                assert_pointer(&value, "/load/fresh_session", json!(true));
                assert_pointer(
                    &value,
                    "/stale_restore/terminal_outcome",
                    json!("RejectedBeforeMutation"),
                );
                assert_pointer(&value, "/stale_restore/runtime/mutation_count", json!(0));
            }
            "patch" => {
                assert_pointer(&value, "/activation/performed", json!(true));
                assert_pointer(&value, "/activation/activation_cut", json!(true));
            }
            _ => {}
        }
    }
}

#[test]
fn cli_patch_without_hash_bound_carrier_fails_closed_before_activation() {
    let scn09_base = abs_path(SCN09_RELATIVE_BASE);
    let scn09_candidate = abs_path(SCN09_RELATIVE_CANDIDATE);
    let error = run_command_result(
        M10CliFacadeCommand::patch()
            .source_path(scn09_base)
            .candidate_source_path(scn09_candidate)
            .typed_schedule_json(schedule()),
    )
    .expect_err("patch without a hash-bound PatchIntentCarrier must fail closed");
    assert!(
        error.contains("PatchIntentCarrier") || error.contains("patch carrier"),
        "unexpected missing-carrier failure: {error}"
    );
}

#[test]
fn cli_conform_requires_typed_carriers_and_predicate_profile_then_reaches_verifier() {
    let missing_inputs = run_command_result(
        M10CliFacadeCommand::conform()
            .corpus_path(corpus_root())
            .typed_schedule_json(profile_input_json("action-context.schedule.json")),
    )
    .expect("missing typed carriers/profile still returns a typed fail-closed report");
    assert_pointer(
        &missing_inputs,
        "/terminal_outcome",
        json!("RejectedBeforeExecution"),
    );
    assert_pointer(
        &missing_inputs,
        "/facade/missing_typed_carriers",
        json!(true),
    );
    assert_pointer(
        &missing_inputs,
        "/facade/missing_predicate_profile",
        json!(true),
    );

    let corpus_sources = corpus_source_units();
    let value = run_command_result(
        M10CliFacadeCommand::conform()
            .corpus_path(corpus_root())
            .typed_schedule_json(profile_input_json("action-context.schedule.json"))
            .typed_carriers_json(profile_input_json("typed-carriers.json"))
            .predicate_profile_json(profile_input_json("correspondence-predicates.json")),
    )
    .unwrap_or_else(|error| {
        panic!("valid typed conform inputs must return a typed verifier report, not a raw error: {error}")
    });
    assert_pointer(&value, "/command", json!("conform"));
    assert!(
        matches!(
            value.pointer("/terminal_outcome").and_then(Value::as_str),
            Some("ConformanceAccepted") | Some("ConformanceFailure")
        ),
        "typed conform inputs must reach the conformance verifier rather than facade rejection: {value:#}"
    );
    assert_pointer(&value, "/facade/source_first", json!(true));
    assert_pointer_absent_or_false(&value, "/facade/missing_typed_carriers");
    assert_pointer_absent_or_false(&value, "/facade/missing_predicate_profile");
    assert_pointer(
        &value,
        "/cli_pipeline/input_kind",
        json!("ordinary_mir_corpus"),
    );
    assert_pointer(
        &value,
        "/cli_pipeline/m6_parse_count",
        json!(corpus_sources.len()),
    );
    assert_pointer(
        &value,
        "/verification/inventory/frozen_row_count",
        json!(73),
    );
    assert_pointer(
        &value,
        "/verification/inventory/pressure_rows_are_frozen",
        json!(false),
    );
    assert_eq!(
        value.pointer("/verification/inventory/missing_rows"),
        Some(&json!([])),
        "valid CLI conform input must not lose frozen rows before verification: {value:#}"
    );
    assert_eq!(
        value.pointer("/verification/inventory/unexpected_rows"),
        Some(&json!([])),
        "valid CLI conform input must not add rows before verification: {value:#}"
    );
    assert!(
        value
            .pointer("/verification/inventory/source_digest")
            .and_then(Value::as_str)
            .is_some(),
        "valid CLI conform input must report a concrete predicate-profile digest: {value:#}"
    );
    assert_status(&value, "/pressure/SCN-11/designated_version", "accepted");
    assert_status(&value, "/pressure/SCN-12/bird_relation", "accepted");
    for source in corpus_sources {
        assert_eq!(source_parse_count(&value, &source), 1);
        assert_source_identity_unit(&value, &source);
    }
}

#[test]
fn cli_rejects_expected_output_mode_and_source_absent_artifact_execution() {
    let scn02 = abs_path(SCN02_RELATIVE_SOURCE);
    for command in [
        M10CliFacadeCommand::run()
            .source_path(scn02.clone())
            .expected_output_json(abs_path(
                "samples/clean-near-end/i1plus-reference/scn-02/positive.expected.json",
            ))
            .typed_schedule_json(schedule()),
        M10CliFacadeCommand::run()
            .checked_artifact_without_source("artifact#scn02-positive")
            .typed_schedule_json(schedule()),
    ] {
        let value = run_command(command);
        assert_pointer(
            &value,
            "/terminal_outcome",
            json!("RejectedBeforeExecution"),
        );
        assert_pointer(&value, "/runtime/mutation_count", json!(0));
        assert_pointer(&value, "/facade/source_first", json!(false));
    }
}
