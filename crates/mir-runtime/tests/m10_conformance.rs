use std::{
    fs,
    path::{Path, PathBuf},
};

use mir_ast::surface_v0::{FixtureSource, parse_surface_v0};
use mir_runtime::m10_reference_system::{
    M10ConformanceReport, M10ReferenceSystem, M10SourceRunRequest,
};
use mir_semantics::surface_v0_pipeline::{M7DiagnosticKind, check_and_elaborate_surface_v0};
use serde_json::{Map, Value, json};

const CORPUS_RELATIVE_ROOT: &str = "samples/clean-near-end/i1plus-reference";
const PROFILE_INPUT_RELATIVE_ROOT: &str = "samples/clean-near-end/i1plus-reference-profile";
const FIVE_DOMAIN_HASH_KEYS: [&str; 5] = [
    "store_hash",
    "membership_hash",
    "grant_hash",
    "relation_hash",
    "config_hash",
];
const DOMAIN_PROJECTION_REQUIREMENTS: [(&str, &str); 5] = [
    ("store", "store_hash"),
    ("membership", "membership_hash"),
    ("grant", "grant_hash"),
    ("relation", "relation_hash"),
    ("config", "config_hash"),
];
const M10_SCN02_SCENARIO_SOURCE_PATH: &str = "mirrorea_canon/scenarios/SCN-02-attack.md";
const M10_SCN02_STALE_CANON_LINE_START: i64 = 39;
const M10_SCN02_STALE_CANON_LINE_END: i64 = 47;
const M10_SCN10_CANON_SOURCE_PATH: &str = "mirrorea_canon/spec/11-m10-i1plus-conformance.md";
const M10_SCN10_CANON_LINE_START: i64 = 19;
const M10_SCN10_CANON_LINE_END: i64 = 24;
const M10_SCN10_SCENARIO_SOURCE_PATH: &str =
    "mirrorea_canon/scenarios/SCN-10-saveload-stale-reject.md";
const M10_SCN10_SCENARIO_LINE_START: i64 = 15;
const M10_SCN10_SCENARIO_LINE_END: i64 = 27;
const M10_NO_STALE_RESURRECTION_CANON_SOURCE_PATH: &str =
    "mirrorea_canon/theory/04-ordering-and-cuts.md";
const M10_NO_STALE_RESURRECTION_CANON_LINE_START: i64 = 86;
const M10_NO_STALE_RESURRECTION_CANON_LINE_END: i64 = 96;

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root)
        .unwrap_or_else(|error| panic!("can read corpus directory {}: {error}", root.display()))
    {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn corpus_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(&corpus_root(), &mut files);
    files.sort();
    files
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus_root() -> PathBuf {
    workspace_root().join(CORPUS_RELATIVE_ROOT)
}

fn corpus_root_string() -> String {
    corpus_root().to_string_lossy().into_owned()
}

fn corpus_path(relative: &str) -> PathBuf {
    corpus_root().join(relative)
}

fn profile_input_path(relative: &str) -> PathBuf {
    workspace_root()
        .join(PROFILE_INPUT_RELATIVE_ROOT)
        .join(relative)
}

fn m10_production_source_paths() -> Vec<PathBuf> {
    let runtime_src = workspace_root().join("crates/mir-runtime/src");
    let mut paths = fs::read_dir(&runtime_src)
        .unwrap_or_else(|error| {
            panic!(
                "can read mir-runtime source directory {}: {error}",
                runtime_src.display()
            )
        })
        .filter_map(|entry| {
            let path = entry.expect("mir-runtime source entry").path();
            let file_name = path.file_name()?.to_string_lossy();
            (path.extension().and_then(|extension| extension.to_str()) == Some("rs")
                && file_name.starts_with("m10_"))
            .then_some(path)
        })
        .collect::<Vec<_>>();
    let cli_entry = runtime_src.join("bin/mir.rs");
    if cli_entry.exists() {
        paths.push(cli_entry);
    }
    paths.sort();
    paths
}

fn profile_input_json(relative: &str) -> Value {
    let path = profile_input_path(relative);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("can read profile input {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("profile input {} is valid JSON: {error}", path.display()))
}

fn source_text(relative: &str) -> String {
    fs::read_to_string(corpus_path(relative))
        .unwrap_or_else(|error| panic!("can read {relative}: {error}"))
}

fn deterministic_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn canonical_value_hash(value: &Value) -> String {
    deterministic_hash(&value.to_string())
}

fn source_hash(relative: &str) -> String {
    deterministic_hash(&format!("{relative}\0{}", source_text(relative)))
}

fn source_artifact_identity(relative: &str) -> String {
    format!("source:{relative}:{}", source_hash(relative))
}

fn schedule_case(id: &str, scn: &str, source: Option<&str>, operation: Value) -> Value {
    let mut case = Map::new();
    assert!(
        id.starts_with(&scn.replace('-', "")),
        "schedule action {id} must preserve its scenario prefix {scn}"
    );
    case.insert("id".to_string(), json!(id));
    case.insert("scn".to_string(), json!(scn));
    if let Some(source) = source {
        case.insert("source".to_string(), json!(source));
    }
    case.insert("operation".to_string(), operation);
    Value::Object(case)
}

fn schedule_cases() -> Vec<Value> {
    vec![
        schedule_case(
            "SCN01.roll.self.draw3",
            "SCN-01",
            Some("scn-01/positive.mir"),
            json!({"kind": "owner_event", "event": "roll", "principal": "self", "arguments": {"draw": 3}}),
        ),
        schedule_case(
            "SCN02.attack.repeat2.step1",
            "SCN-02",
            Some("scn-02/positive.mir"),
            json!({"kind": "owner_event", "event": "attack", "principal": "self", "target": "target", "step": 1, "seed": {"player[target].hp": 100, "player[self].atk": 10}}),
        ),
        schedule_case(
            "SCN02.attack.repeat2.step2",
            "SCN-02",
            Some("scn-02/positive.mir"),
            json!({"kind": "owner_event", "event": "attack", "principal": "self", "target": "target", "step": 2, "seed": {"player[target].hp": 90, "player[self].atk": 10}}),
        ),
        schedule_case(
            "SCN02.attack.without_capability",
            "SCN-02",
            Some("scn-02/positive.mir"),
            json!({"kind": "corrupted_request", "event": "attack", "principal": "self", "target": "target", "missing": "capability"}),
        ),
        schedule_case(
            "SCN02.target.leave",
            "SCN-02",
            Some("scn-02/positive.mir"),
            json!({"kind": "target_leave", "target": "target"}),
        ),
        schedule_case(
            "SCN02.attack.stale_membership",
            "SCN-02",
            Some("scn-02/positive.mir"),
            json!({"kind": "corrupted_request", "event": "attack", "principal": "self", "target": "target", "membership": "stale"}),
        ),
        schedule_case(
            "SCN03.admission_then_start",
            "SCN-03",
            Some("scn-03/positive.mir"),
            json!({"kind": "admission_then_owner_event", "event": "start", "principal": "self"}),
        ),
        schedule_case(
            "SCN03.start_before_admission",
            "SCN-03",
            Some("scn-03/negative-write-before-verdict.mir"),
            json!({"kind": "owner_event_before_admission", "event": "start", "principal": "self"}),
        ),
        schedule_case(
            "SCN03.role_spoof",
            "SCN-03",
            Some("scn-03/positive.mir"),
            json!({"kind": "corrupted_request", "event": "start", "principal": "attacker", "spoofed_role": "self"}),
        ),
        schedule_case(
            "SCN03.capability_replay",
            "SCN-03",
            Some("scn-03/positive.mir"),
            json!({"kind": "corrupted_request", "event": "start", "principal": "self", "capability": "replayed_from_prior_epoch"}),
        ),
        schedule_case(
            "SCN04.leave.attack_stale",
            "SCN-04",
            Some("scn-04/positive.mir"),
            json!({"kind": "membership_lifecycle", "events": ["leave", "attack_stale"]}),
        ),
        schedule_case(
            "SCN04.leave",
            "SCN-04",
            Some("scn-04/positive.mir"),
            json!({"kind": "membership_lifecycle", "events": ["leave"]}),
        ),
        schedule_case(
            "SCN04.compact_before_audit_cut",
            "SCN-04",
            Some("scn-04/positive.mir"),
            json!({"kind": "compaction_request", "membership_frontier": "before_audit_cut"}),
        ),
        schedule_case(
            "SCN04.compact_after_audit_cut",
            "SCN-04",
            Some("scn-04/positive.mir"),
            json!({"kind": "compaction_request", "membership_frontier": "after_audit_cut"}),
        ),
        schedule_case(
            "SCN04.rejoin",
            "SCN-04",
            Some("scn-04/positive.mir"),
            json!({"kind": "membership_lifecycle", "events": ["rejoin"], "fresh_incarnation": true}),
        ),
        schedule_case(
            "SCN04.leave_then_rejoin_without_fresh_incarnation",
            "SCN-04",
            Some("scn-04/negative-hidden-repair.mir"),
            json!({"kind": "membership_lifecycle", "events": ["leave", "rejoin"], "fresh_incarnation": false}),
        ),
        schedule_case(
            "SCN05.leave_a.join_b.spawn_b",
            "SCN-05",
            Some("scn-05/positive.mir"),
            json!({"kind": "portal_handoff", "events": ["leave_a", "join_b", "spawn_b"]}),
        ),
        schedule_case(
            "SCN05.cross_locus_observation_request",
            "SCN-05",
            Some("scn-05/negative-secret-cross-locus.mir"),
            json!({"kind": "observation_request", "request_class": "cross_locus_observation", "validated_policy_carrier_ref": "portal-secret-redaction-policy"}),
        ),
        schedule_case(
            "SCN05.cross_locus_secret_request",
            "SCN-05",
            Some("scn-05/negative-secret-cross-locus.mir"),
            json!({"kind": "observation_request", "request_class": "cross_locus_secret_read", "validated_policy_carrier_ref": "portal-secret-redaction-policy"}),
        ),
        schedule_case(
            "SCN05.observation_wrong_capability",
            "SCN-05",
            Some("scn-05/negative-secret-cross-locus.mir"),
            json!({"kind": "corrupted_request", "request_class": "cross_locus_secret_read", "capability": "wrong_observation_capability", "validated_policy_carrier_ref": "portal-secret-redaction-policy"}),
        ),
        schedule_case(
            "SCN06.route_absent",
            "SCN-06",
            Some("scn-06/positive.mir"),
            json!({"kind": "route_context", "events": ["invoke_before_patch"]}),
        ),
        schedule_case(
            "SCN06.invoke_before_patch.submit_checked_route_patch_artifact.invoke_after_patch",
            "SCN-06",
            Some("scn-06/positive.mir"),
            json!({"kind": "route_context", "events": ["invoke_before_patch", "submit_checked_route_patch_artifact", "invoke_after_patch"], "route_patch_carrier_ref": "scn06-route-patch-east-west"}),
        ),
        schedule_case(
            "SCN06.route_absent_with_finite_turn_budget",
            "SCN-06",
            Some("scn-06/positive.mir"),
            json!({"kind": "route_context", "events": ["invoke_before_patch"], "turn_budget": "finite"}),
        ),
        schedule_case(
            "SCN07.observer_projection",
            "SCN-07",
            Some("scn-07/positive.mir"),
            json!({"kind": "observer_projection", "policy_carrier_ref": "observer-safe-position-only", "channel": "observer_safe"}),
        ),
        schedule_case(
            "SCN07.admin_projection",
            "SCN-07",
            Some("scn-07/positive.mir"),
            json!({"kind": "observer_projection", "policy_carrier_ref": "observer-safe-position-only", "channel": "admin_debug"}),
        ),
        schedule_case(
            "SCN07.history_origin_violation",
            "SCN-07",
            Some("scn-07/positive.mir"),
            json!({"kind": "corrupted_request", "projection": "observer_history", "origin": "forged_without_redaction"}),
        ),
        schedule_case(
            "SCN08.live",
            "SCN-08",
            Some("scn-08/positive.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["live"]}),
        ),
        schedule_case(
            "SCN08.lease_expiry",
            "SCN-08",
            Some("scn-08/positive.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["lease_expiry"]}),
        ),
        schedule_case(
            "SCN08.write",
            "SCN-08",
            Some("scn-08/positive.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["write"]}),
        ),
        schedule_case(
            "SCN08.rollback",
            "SCN-08",
            Some("scn-08/positive.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["rollback"]}),
        ),
        schedule_case(
            "SCN08.fresh_reacquire",
            "SCN-08",
            Some("scn-08/positive.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["fresh_reacquire"]}),
        ),
        schedule_case(
            "SCN08.write_after_read_lineage",
            "SCN-08",
            Some("scn-08/negative-write-after-read-lineage.mir"),
            json!({"kind": "lease_option_lifecycle", "events": ["write_after_read_lineage"]}),
        ),
        schedule_case(
            "SCN09.submit_checked_patch_a",
            "SCN-09",
            None,
            json!({"kind": "submit_checked_patch_artifact", "patch_carrier_ref": "scn09-candidate-a"}),
        ),
        schedule_case(
            "SCN09.membership_frontier_drift",
            "SCN-09",
            None,
            json!({"kind": "membership_frontier_drift", "events": ["admit_patch", "membership_changes", "activate_checked_patch"], "patch_carrier_ref": "scn09-candidate-a"}),
        ),
        schedule_case(
            "SCN09.membership_frontier_drift_between_admit_activation",
            "SCN-09",
            None,
            json!({"kind": "membership_frontier_drift", "events": ["admit_patch", "membership_changes", "activate_checked_patch"], "patch_carrier_ref": "scn09-candidate-a"}),
        ),
        schedule_case(
            "SCN10.save_s1",
            "SCN-10",
            Some("scn-10/positive.mir"),
            json!({"kind": "save_load_timeline", "events": ["save_s1"]}),
        ),
        schedule_case(
            "SCN10.leave_a.lease_expiry.save_s2",
            "SCN-10",
            Some("scn-10/positive.mir"),
            json!({"kind": "save_load_timeline", "events": ["leave_a", "lease_expiry", "save_s2"]}),
        ),
        schedule_case(
            "SCN10.load_s1_fresh",
            "SCN-10",
            Some("scn-10/positive.mir"),
            json!({"kind": "save_load_timeline", "events": ["load_s1_fresh"]}),
        ),
        schedule_case(
            "SCN10.merge_stale_s1_into_current",
            "SCN-10",
            Some("scn-10/negative-stale-restore.mir"),
            json!({"kind": "save_load_timeline", "events": ["merge_stale_s1_into_current"]}),
        ),
        schedule_case(
            "SCN10.doctor_expired_lease_live",
            "SCN-10",
            Some("scn-10/negative-stale-restore.mir"),
            json!({"kind": "corrupted_request", "events": ["doctor_expired_lease_live"]}),
        ),
        schedule_case(
            "SCN10.doctor_cut_receive_without_send",
            "SCN-10",
            Some("scn-10/negative-stale-restore.mir"),
            json!({"kind": "corrupted_request", "events": ["doctor_cut_receive_without_send"]}),
        ),
        schedule_case(
            "SCN10.timeline_panel",
            "SCN-10",
            Some("scn-10/positive.mir"),
            json!({"kind": "save_load_timeline", "events": ["timeline_panel"]}),
        ),
        schedule_case(
            "SCN10.reacquire_after_load",
            "SCN-10",
            Some("scn-10/positive.mir"),
            json!({"kind": "save_load_timeline", "events": ["reacquire_after_load"]}),
        ),
        schedule_case(
            "SCN11.designated_version",
            "SCN-11",
            Some("scn-11/designated-version.mir"),
            json!({"kind": "designated_consumption", "designated_value_ref": "Evaluator.result", "consumer": "self", "version": 1}),
        ),
        schedule_case(
            "SCN11.duplicate_consumption",
            "SCN-11",
            Some("scn-11/duplicate-consumption.mir"),
            json!({"kind": "designated_consumption", "designated_value_ref": "Evaluator.result", "consumer": "self", "version": 1, "repeat": 2}),
        ),
        schedule_case(
            "SCN12.bird_relation",
            "SCN-12",
            Some("scn-12/bird-relation.mir"),
            json!({"kind": "relation_projection", "relation": "bird_follow", "consumer": "Viewer"}),
        ),
        schedule_case(
            "SCN12.split_frame",
            "SCN-12",
            Some("scn-12/split-frame.mir"),
            json!({"kind": "relation_projection", "relation": "bird_follow", "consumer": "Viewer", "presentation_context": "split_frame"}),
        ),
        schedule_case(
            "SCN12.fallback",
            "SCN-12",
            Some("scn-12/fallback.mir"),
            json!({"kind": "relation_projection", "relation": "bird_follow", "consumer": "Viewer", "presentation_context": "fallback"}),
        ),
        schedule_case(
            "SCN12.reacquire",
            "SCN-12",
            Some("scn-12/reacquire.mir"),
            json!({"kind": "relation_projection", "relation": "bird_follow", "consumer": "Viewer", "presentation_context": "fresh_reacquire"}),
        ),
    ]
}

fn action_schedule() -> Value {
    json!({
        "schema_version": "m10-i1plus-action-context-schedule-v0",
        "kind": "typed_conformance_input",
        "direct_mutation_api": false,
        "cases": schedule_cases()
    })
}

fn carrier_value_in<'a>(carriers: &'a Value, carrier: &str) -> (&'static str, &'a Value) {
    for (kind, key) in [
        ("patch", "patch_carriers"),
        ("observation_policy", "policy_carriers"),
        ("fallback", "fallback_carriers"),
        ("route_patch", "route_patch_carriers"),
    ] {
        if let Some(value) = carriers
            .pointer(&format!("/{key}"))
            .and_then(Value::as_array)
            .and_then(|rows| {
                rows.iter()
                    .find(|row| row.get("id").and_then(Value::as_str) == Some(carrier))
            })
        {
            return (kind, value);
        }
    }
    panic!("typed carrier {carrier} is present in committed typed carriers: {carriers:#}");
}

fn typed_carrier_identity_from(carriers: &Value, carrier: &str) -> String {
    let (_kind, value) = carrier_value_in(carriers, carrier);
    format!("typed_carrier:{carrier}:{}", canonical_value_hash(value))
}

fn typed_carrier_identity(carrier: &str) -> String {
    typed_carrier_identity_from(&typed_carriers(), carrier)
}

fn patch_pair_identity_from(
    carriers: &Value,
    carrier: &str,
    base: &str,
    candidate: &str,
) -> String {
    let (kind, value) = carrier_value_in(carriers, carrier);
    assert_eq!(
        kind, "patch",
        "patch pair {carrier} must bind a patch carrier object"
    );
    format!(
        "patch_pair:{carrier}:base={}:candidate={}:carrier={}",
        source_hash(base),
        source_hash(candidate),
        canonical_value_hash(value)
    )
}

fn patch_pair_identity(carrier: &str, base: &str, candidate: &str) -> String {
    patch_pair_identity_from(&typed_carriers(), carrier, base, candidate)
}

fn schedule_action_id(reference: &str) -> &str {
    reference.strip_prefix("schedule:").unwrap_or(reference)
}

fn schedule_case_in<'a>(schedule: &'a Value, action: &str) -> &'a Value {
    let action = schedule_action_id(action);
    schedule
        .pointer("/cases")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("id").and_then(Value::as_str) == Some(action))
        })
        .unwrap_or_else(|| panic!("schedule action {action} is committed in {schedule:#}"))
}

fn schedule_action_identity_from(schedule: &Value, action: &str) -> String {
    let action = schedule_action_id(action);
    let value = schedule_case_in(schedule, action);
    format!("schedule_action:{action}:{}", canonical_value_hash(value))
}

fn schedule_action_identity(action: &str) -> String {
    schedule_action_identity_from(&action_schedule(), action)
}

fn action_id_from_schedule_identity(identity: &str) -> &str {
    identity
        .strip_prefix("schedule_action:")
        .and_then(|rest| rest.rsplit_once(":fnv1a64:").map(|(action, _hash)| action))
        .unwrap_or_else(|| panic!("schedule identity is hash-bound: {identity}"))
}

fn typed_carriers() -> Value {
    json!({
        "schema_version": "m10-i1plus-typed-carriers-v0",
        "patch_carriers": [
            {
                "id": "scn09-candidate-a",
                "kind": "source_patch_intent",
                "base_source": "scn-09/base.mir",
                "candidate_source": "scn-09/candidate-accepted.mir",
                "base_source_hash": source_hash("scn-09/base.mir"),
                "candidate_source_hash": source_hash("scn-09/candidate-accepted.mir"),
                "state_additions": [{"state": "lamp", "fields": ["enabled"]}],
                "required_capabilities": ["patch.activate"],
                "required_effects": ["observer_publish"],
                "required_failures": ["VisibilityDenied"],
                "authority_intent": {"kind": "none"}
            },
            {
                "id": "scn09-candidate-b",
                "kind": "source_patch_intent",
                "base_source": "scn-09/base.mir",
                "candidate_source": "scn-09/candidate-rejected.mir",
                "base_source_hash": source_hash("scn-09/base.mir"),
                "candidate_source_hash": source_hash("scn-09/candidate-rejected.mir"),
                "state_additions": [{"state": "lamp", "fields": ["enabled"]}],
                "required_capabilities": ["patch.activate"],
                "required_effects": ["observer_publish"],
                "required_failures": ["VisibilityDenied"],
                "authority_intent": {"kind": "self_grant", "authority": "ServerAuthority", "grantee": "self"}
            },
            {
                "id": "scn09-candidate-c",
                "kind": "source_patch_intent",
                "base_source": "scn-09/base.mir",
                "candidate_source": "scn-09/candidate-missing-capability.mir",
                "base_source_hash": source_hash("scn-09/base.mir"),
                "candidate_source_hash": source_hash("scn-09/candidate-missing-capability.mir"),
                "state_additions": [{"state": "lamp", "fields": ["enabled"]}],
                "required_capabilities": [],
                "required_effects": ["observer_publish"],
                "required_failures": ["VisibilityDenied"],
                "authority_intent": {"kind": "none"}
            }
        ],
        "policy_carriers": [
            {"id": "portal-secret-redaction-policy", "subject_source": "scn-05/negative-secret-cross-locus.mir", "private_state": "player_a", "private_field": "secret_key", "source_owner_locus": "WorldA", "destination_state": "player_b", "destination_field": "position", "destination_owner_locus": "WorldB", "request_class": "cross_locus_observation", "required_failures": ["VisibilityDenied"]},
            {"id": "portal-secret-missing-required-failure", "subject_source": "scn-05/negative-secret-cross-locus.mir", "private_state": "player_a", "private_field": "secret_key", "source_owner_locus": "WorldA", "destination_state": "player_b", "destination_field": "position", "destination_owner_locus": "WorldB", "request_class": "cross_locus_observation", "required_failures": []},
            {"id": "observer-safe-position-only", "field_policy_source": "scn-07/positive.mir", "observer_fields": ["position"], "debug_fields": ["position", "hp", "inventory_note"]},
            {"id": "inventory-note-private-policy", "field_policy_source": "scn-07/negative-inventory-visible.mir", "private_like_fields": ["inventory_note"], "observer_channel": "observer_safe"}
        ],
        "fallback_carriers": [
            {"id": "view-pose-normal-fallback", "relation": "view_pose", "source": "scn-08/positive.mir", "negative_capability_floor": "write_after_read_without_fresh_reacquire", "options": [
                {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": [{"from": "live", "to": "anchor"}]},
                {"kind": "frozen", "target": "default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "static", "lineage_edges": [{"from": "anchor", "to": "frozen"}]}
            ]},
            {"id": "view-pose-missing-lineage", "relation": "view_pose", "source": "scn-08/negative-missing-fallback-anchor.mir", "negative_capability_floor": "missing_live_to_anchor_lineage_edge", "options": [
                {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": []},
                {"kind": "frozen", "target": "default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "static", "lineage_edges": [{"from": "anchor", "to": "frozen"}]}
            ]},
            {"id": "view-pose-write-after-read", "relation": "view_pose", "source": "scn-08/negative-write-after-read-lineage.mir", "negative_capability_floor": "write_after_read_without_fresh_reacquire", "options": [
                {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": [{"from": "live", "to": "anchor"}]},
                {"kind": "frozen", "target": "default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "static", "lineage_edges": [{"from": "anchor", "to": "frozen"}]}
            ]}
        ],
        "route_patch_carriers": [
            {"id": "scn06-route-patch-east-west", "kind": "route_patch_intent", "candidate_source": "scn-06/route-candidate-accepted.mir", "candidate_source_hash": source_hash("scn-06/route-candidate-accepted.mir"), "from_locus": "ShardA", "to_locus": "ShardB", "route_state": "available", "required_capabilities": ["route.patch"], "authority_intent": {"kind": "none"}}
        ]
    })
}

#[derive(Clone, Copy)]
enum ArtifactBinding {
    Source(&'static str),
    TypedCarrier(&'static str),
    PatchPair {
        carrier: &'static str,
        base: &'static str,
        candidate: &'static str,
    },
    Schedule(&'static str),
}

impl ArtifactBinding {
    fn carrier_kind(self) -> &'static str {
        match self {
            Self::Source(_) => "ordinary_source",
            Self::TypedCarrier(_) => "typed_carrier",
            Self::PatchPair { .. } => "patch_source",
            Self::Schedule(_) => "schedule_action",
        }
    }

    fn identity(self) -> String {
        match self {
            Self::Source(source) => source_artifact_identity(source),
            Self::TypedCarrier(carrier) => typed_carrier_identity(carrier),
            Self::PatchPair {
                carrier,
                base,
                candidate,
            } => patch_pair_identity(carrier, base, candidate),
            Self::Schedule(action) => schedule_action_identity(action),
        }
    }
}

#[derive(Clone, Copy)]
struct ExpectedRow {
    scn_id: &'static str,
    expectation_id: &'static str,
    phase: &'static str,
    artifact: ArtifactBinding,
    diagnostic_location: &'static str,
    source_derived_reference: Option<&'static str>,
    schedule_action_reference: Option<&'static str>,
    evidence_predicate: &'static str,
}

impl ExpectedRow {
    fn value(self) -> Value {
        json!({
            "scn_id": self.scn_id,
            "expectation_id": self.expectation_id,
            "phase": self.phase,
            "carrier_kind": self.artifact.carrier_kind(),
            "artifact_identity": self.artifact.identity(),
            "diagnostic_location": self.diagnostic_location,
            "source_derived_reference": self.source_derived_reference,
            "schedule_action_reference": self.schedule_action_reference,
            "evidence_predicate": self.evidence_predicate,
        })
    }
}

const FROZEN_EXPECTATION_ROWS: &[ExpectedRow] = &[
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-P-REQ",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:4804cdea39e7fa6f",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: None,
        evidence_predicate: "static.request_edge.exactly_one.BrowserClient_self_to_World.write.player_self_position",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-P-DEP",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:9a71b7bab138a5d1",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: None,
        evidence_predicate: "static.dependency.same_field.player_self_position.read_for_position_write",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-P-PUB",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:d6b78c4ba09f2837",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: None,
        evidence_predicate: "static.observer_publish_effect.position.is_source_declared",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-P-SPANS",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:171d373812650294",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: None,
        evidence_predicate: "static.source_spans.position_visibility_and_write.are_exact",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-P-CAP",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:045628de0a092809",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: None,
        evidence_predicate: "static.obligation.cap_write.player.required_for_position_write",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-S-N-VISROW",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-01/negative-missing-visibility-denied.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:f2f080abca21e939",
        source_derived_reference: Some("m10-source-ref:fnv1a64:c0f646b34b508ce0"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-ROW-002.missing_failure.VisibilityDenied.no_checked_core",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-R-P-STATE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:1f06a50a2a0077c3",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: Some(
            "schedule_action:SCN01.roll.self.draw3:fnv1a64:a5564c4072d4c629",
        ),
        evidence_predicate: "runtime.final_state.player_self_position.equals.3",
    },
    ExpectedRow {
        scn_id: "SCN-01",
        expectation_id: "SCN01-R-P-ORDER",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-01/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:6bc4b68b9317d22c",
        source_derived_reference: Some("m10-source-ref:fnv1a64:ac5b8ec1fed2fd56"),
        schedule_action_reference: Some(
            "schedule_action:SCN01.roll.self.draw3:fnv1a64:a5564c4072d4c629",
        ),
        evidence_predicate: "runtime.history.request_before_serve_before_publish",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-P-REQ-RMW",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:362fb35bff9ed199",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "static.owner_rmw.requires.MissingCapability.MissingWitness.RouteUnavailable.StaleMembership",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-P-DEPS",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:7a38160b689091c7",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "static.dependencies.include.target_hp_read_write.and.self_atk_read",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-P-FAIL-SPAN",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:5242a44191cf3314",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "static.failure_rows.retain_exact_source_span",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-P-LOCUS",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:f8f0a68ad98853f5",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "static.cross_locus_actor_origin_does_not_mint_owner_authority",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-N-CAPROW",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/negative-missing-capability-row.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:2370ed24c47f5b2b",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d6b54593ec26ce5"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-ROW-001.missing_failure.MissingCapability.no_checked_core",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-N-REQUESTER-READ",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:4cd86c1c202fe0f7",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "structural_rejection.no_mutation.requester_read_does_not_bypass_owner_locus",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-N-BLIND-WRITE",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:598c3a9f4cf75e8d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "structural_rejection.no_mutation.blind_cross_owner_write_rejected",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-S-N-NO-XOWNER-TXN",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:fcc14dd19dbef979",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: None,
        evidence_predicate: "structural_rejection.no_mutation.cross_owner_transaction_not_fabricated",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-R-P-ONE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:b70eb9cf308ff337",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: Some(
            "schedule_action:SCN02.attack.repeat2.step1:fnv1a64:ee49a6a228ee90bb",
        ),
        evidence_predicate: "runtime.hp_history.after_first_attack.equals.90",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-R-P-TWO",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-02/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:719d19745442e95b",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: Some(
            "schedule_action:SCN02.attack.repeat2.step2:fnv1a64:375e8cc51d3b9cd3",
        ),
        evidence_predicate: "runtime.hp_history.after_second_attack.equals.80",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-R-N-NOCAP",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN02.attack.without_capability"),
        diagnostic_location: "m10-evidence-location:fnv1a64:d604fb83ec6e20b1",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: Some(
            "schedule_action:SCN02.attack.without_capability:fnv1a64:ecfc4901153a9a11",
        ),
        evidence_predicate: "structural_rejection.no_mutation.owner_request_without_capability",
    },
    ExpectedRow {
        scn_id: "SCN-02",
        expectation_id: "SCN02-R-N-STALE",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN02.attack.stale_membership"),
        diagnostic_location: "m10-evidence-location:fnv1a64:6df288c46b17db66",
        source_derived_reference: Some("m10-source-ref:fnv1a64:416d963ab81664ab"),
        schedule_action_reference: Some(
            "schedule_action:SCN02.attack.stale_membership:fnv1a64:5514f92ce02687fb",
        ),
        evidence_predicate: "structural_rejection.no_mutation.owner_request_with_stale_membership",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-S-N-PREVERDICT",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-03/negative-write-before-verdict.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:0fe3a2da22b028ff",
        source_derived_reference: Some("m10-source-ref:fnv1a64:18b3238134546b4e"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.start_before_admission:fnv1a64:8ac95da5599256fb",
        ),
        evidence_predicate: "structural_rejection.no_mutation.pre_verdict_write_has_no_runtime_artifact",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-P-ADMIT",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-03/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:1c270976f92922ef",
        source_derived_reference: Some("m10-source-ref:fnv1a64:af79975a81b5ceca"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.admission_then_start:fnv1a64:e264cdc629cfac95",
        ),
        evidence_predicate: "runtime.admission_verdict_precedes_owner_write",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-P-LINEAGE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-03/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:c53c5957f567f558",
        source_derived_reference: Some("m10-source-ref:fnv1a64:af79975a81b5ceca"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.admission_then_start:fnv1a64:e264cdc629cfac95",
        ),
        evidence_predicate: "runtime.lineage_binds_checked_source_identity_to_authority_inventory",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-P-PAST",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-03/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:5ae0b549e704b0db",
        source_derived_reference: Some("m10-source-ref:fnv1a64:af79975a81b5ceca"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.admission_then_start:fnv1a64:e264cdc629cfac95",
        ),
        evidence_predicate: "runtime.past_world_cut_remains_audit_visible",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-N-PREVERDICT",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-03/negative-write-before-verdict.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:fa672e59ce6c73e0",
        source_derived_reference: Some("m10-source-ref:fnv1a64:18b3238134546b4e"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.start_before_admission:fnv1a64:8ac95da5599256fb",
        ),
        evidence_predicate: "structural_rejection.no_mutation.pre_verdict_owner_write",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-N-ROLE-SPOOF",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN03.role_spoof"),
        diagnostic_location: "m10-evidence-location:fnv1a64:68931f9e33e5929d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:af79975a81b5ceca"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.role_spoof:fnv1a64:292dbb80fcd8738f",
        ),
        evidence_predicate: "structural_rejection.no_mutation.spoofed_role_origin",
    },
    ExpectedRow {
        scn_id: "SCN-03",
        expectation_id: "SCN03-R-N-CAPREPLAY",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN03.capability_replay"),
        diagnostic_location: "m10-evidence-location:fnv1a64:ca62c790d813ac55",
        source_derived_reference: Some("m10-source-ref:fnv1a64:af79975a81b5ceca"),
        schedule_action_reference: Some(
            "schedule_action:SCN03.capability_replay:fnv1a64:7bf94bdddde50665",
        ),
        evidence_predicate: "structural_rejection.no_mutation.replayed_capability",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-P-STALE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-04/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:09dfc454cdacb54e",
        source_derived_reference: Some("m10-source-ref:fnv1a64:12d992c2a2361bbd"),
        schedule_action_reference: Some(
            "schedule_action:SCN04.leave.attack_stale:fnv1a64:01274353549f9fae",
        ),
        evidence_predicate: "runtime.stale_incarnation_request_rejected_without_state_mutation",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-P-AUDIT",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-04/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:252fb12505c4657f",
        source_derived_reference: Some("m10-source-ref:fnv1a64:12d992c2a2361bbd"),
        schedule_action_reference: Some("schedule_action:SCN04.leave:fnv1a64:ecba3301f9ae7eae"),
        evidence_predicate: "runtime.membership_audit_retains_leave_and_rejoin_history",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-P-BLOCK-COMPACT",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN04.compact_before_audit_cut"),
        diagnostic_location: "m10-evidence-location:fnv1a64:680c4c00105f746a",
        source_derived_reference: Some("m10-source-ref:fnv1a64:12d992c2a2361bbd"),
        schedule_action_reference: Some(
            "schedule_action:SCN04.compact_before_audit_cut:fnv1a64:8c6305ab61a76e23",
        ),
        evidence_predicate: "runtime.compaction_before_audit_cut_is_blocked",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-P-ALLOW-COMPACT",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN04.compact_after_audit_cut"),
        diagnostic_location: "m10-evidence-location:fnv1a64:33cc2b7cc60afd44",
        source_derived_reference: Some("m10-source-ref:fnv1a64:12d992c2a2361bbd"),
        schedule_action_reference: Some(
            "schedule_action:SCN04.compact_after_audit_cut:fnv1a64:589a3b9cb2fded4f",
        ),
        evidence_predicate: "runtime.compaction_after_audit_cut_is_allowed",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-P-REJOIN",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-04/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:9fe77cf19cfb656d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:12d992c2a2361bbd"),
        schedule_action_reference: Some("schedule_action:SCN04.rejoin:fnv1a64:6bbe08df60cabcad"),
        evidence_predicate: "runtime.rejoin_requires_fresh_incarnation",
    },
    ExpectedRow {
        scn_id: "SCN-04",
        expectation_id: "SCN04-R-N-HIDDEN-REPAIR",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-04/negative-hidden-repair.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:6e2353e9af5b9f3a",
        source_derived_reference: Some("m10-source-ref:fnv1a64:09ac893b9d21f684"),
        schedule_action_reference: Some(
            "schedule_action:SCN04.leave_then_rejoin_without_fresh_incarnation:fnv1a64:9e84b143d501b987",
        ),
        evidence_predicate: "structural_rejection.no_mutation.hidden_membership_repair",
    },
    ExpectedRow {
        scn_id: "SCN-05",
        expectation_id: "SCN05-S-N-MISSING-VISROW",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("portal-secret-missing-required-failure"),
        diagnostic_location: "m10-evidence-location:fnv1a64:0e50bdd683a78820",
        source_derived_reference: Some("m10-source-ref:fnv1a64:3f364e45f94e3245"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-ROW-002.missing_required_failure.VisibilityDenied.retains_source_span.player_a.secret_key",
    },
    ExpectedRow {
        scn_id: "SCN-05",
        expectation_id: "SCN05-R-P-HANDOFF",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-05/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:a169df72b4361aad",
        source_derived_reference: Some("m10-source-ref:fnv1a64:3cbb80ee49e34e78"),
        schedule_action_reference: Some(
            "schedule_action:SCN05.leave_a.join_b.spawn_b:fnv1a64:04beea09a1196ec5",
        ),
        evidence_predicate: "runtime.portal_handoff.orders.leave_verdict_before.join_verdict.before.spawn_write",
    },
    ExpectedRow {
        scn_id: "SCN-05",
        expectation_id: "SCN05-R-P-OBS",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("portal-secret-redaction-policy"),
        diagnostic_location: "m10-evidence-location:fnv1a64:8ca4d467adc63fb4",
        source_derived_reference: Some("m10-source-ref:fnv1a64:3f364e45f94e3245"),
        schedule_action_reference: Some(
            "schedule_action:SCN05.cross_locus_observation_request:fnv1a64:24d3cfa3db13ae24",
        ),
        evidence_predicate: "runtime.observer_projection_exports_no_secret_key",
    },
    ExpectedRow {
        scn_id: "SCN-05",
        expectation_id: "SCN05-R-N-SECRET",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("portal-secret-redaction-policy"),
        diagnostic_location: "m10-evidence-location:fnv1a64:c64052b16763afc4",
        source_derived_reference: Some("m10-source-ref:fnv1a64:3f364e45f94e3245"),
        schedule_action_reference: Some(
            "schedule_action:SCN05.cross_locus_secret_request:fnv1a64:f384a40ea2b2be89",
        ),
        evidence_predicate: "diagnostic.VisibilityDenied.no_publication.no_state_mutation",
    },
    ExpectedRow {
        scn_id: "SCN-05",
        expectation_id: "SCN05-R-N-WRONGCAP",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN05.observation_wrong_capability"),
        diagnostic_location: "m10-evidence-location:fnv1a64:2a9ed7ceb5ca7f44",
        source_derived_reference: Some("m10-source-ref:fnv1a64:3f364e45f94e3245"),
        schedule_action_reference: Some(
            "schedule_action:SCN05.observation_wrong_capability:fnv1a64:877b9f68d04ca1fd",
        ),
        evidence_predicate: "structural_rejection.no_mutation.wrong_observation_capability",
    },
    ExpectedRow {
        scn_id: "SCN-06",
        expectation_id: "SCN06-S-P-REQFAIL",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-06/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:5329655d801da495",
        source_derived_reference: Some("m10-source-ref:fnv1a64:c44c35ca46c812ce"),
        schedule_action_reference: None,
        evidence_predicate: "static.route_unavailable_failure_row.present_for_owner_route",
    },
    ExpectedRow {
        scn_id: "SCN-06",
        expectation_id: "SCN06-S-N-ROW",
        phase: "static",
        artifact: ArtifactBinding::Source("scn-06/negative-route-unavailable-row.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:e79b68836e9dfca2",
        source_derived_reference: Some("m10-source-ref:fnv1a64:cf1ab06c7e9fe433"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-ROW-001.missing_failure.RouteUnavailable.no_checked_core",
    },
    ExpectedRow {
        scn_id: "SCN-06",
        expectation_id: "SCN06-R-P-ABSENT",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN06.route_absent"),
        diagnostic_location: "m10-evidence-location:fnv1a64:fb14f608edd2f63a",
        source_derived_reference: Some("m10-source-ref:fnv1a64:c44c35ca46c812ce"),
        schedule_action_reference: Some(
            "schedule_action:SCN06.route_absent:fnv1a64:9fad6527bc9764c8",
        ),
        evidence_predicate: "runtime.route_absent_yields_explicit_RouteUnavailable_store_unchanged_route_trace",
    },
    ExpectedRow {
        scn_id: "SCN-06",
        expectation_id: "SCN06-R-P-PATCHED",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("scn06-route-patch-east-west"),
        diagnostic_location: "m10-evidence-location:fnv1a64:53a64d84d7fd0a3a",
        source_derived_reference: Some("m10-source-ref:fnv1a64:c44c35ca46c812ce"),
        schedule_action_reference: Some(
            "schedule_action:SCN06.invoke_before_patch.submit_checked_route_patch_artifact.invoke_after_patch:fnv1a64:2b3982ed8551be08",
        ),
        evidence_predicate: "runtime.checked_route_patch_artifact_makes_same_source_succeed_without_source_edit",
    },
    ExpectedRow {
        scn_id: "SCN-06",
        expectation_id: "SCN06-R-N-NOHANG",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN06.route_absent_with_finite_turn_budget"),
        diagnostic_location: "m10-evidence-location:fnv1a64:e92c271c86f6bbb3",
        source_derived_reference: Some("m10-source-ref:fnv1a64:c44c35ca46c812ce"),
        schedule_action_reference: Some(
            "schedule_action:SCN06.route_absent_with_finite_turn_budget:fnv1a64:811fd0497b59ded9",
        ),
        evidence_predicate: "structural_rejection.no_mutation.route_absence_returns_terminal_failure_within_turn_budget",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-S-N-PRIVATEPOL",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("inventory-note-private-policy"),
        diagnostic_location: "m10-evidence-location:fnv1a64:fb8a7914e96cd7ff",
        source_derived_reference: Some("m10-source-ref:fnv1a64:991565aba52d3458"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-VIS-002.private_like_field_cannot_be_observer_safe",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-S-N-WIDEN",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("inventory-note-private-policy"),
        diagnostic_location: "m10-evidence-location:fnv1a64:d532c4d4f7fcd11d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:991565aba52d3458"),
        schedule_action_reference: None,
        evidence_predicate: "structural_rejection.no_publication.no_mutation.policy_cannot_widen_private_like_field",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-R-P-FIELDS",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("observer-safe-position-only"),
        diagnostic_location: "m10-evidence-location:fnv1a64:5351801889852ab1",
        source_derived_reference: Some("m10-source-ref:fnv1a64:10f65c9c1f283c37"),
        schedule_action_reference: Some(
            "schedule_action:SCN07.observer_projection:fnv1a64:ac2db5bd759cc804",
        ),
        evidence_predicate: "runtime.observer_projection_contains_only.position",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-R-P-ADMIN",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("observer-safe-position-only"),
        diagnostic_location: "m10-evidence-location:fnv1a64:1c6f177a4c76a4c1",
        source_derived_reference: Some("m10-source-ref:fnv1a64:10f65c9c1f283c37"),
        schedule_action_reference: Some(
            "schedule_action:SCN07.admin_projection:fnv1a64:6cf94473f6ab7c70",
        ),
        evidence_predicate: "runtime.admin_debug_view_does_not_leak_authority_payloads",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-R-P-POLICY",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("observer-safe-position-only"),
        diagnostic_location: "m10-evidence-location:fnv1a64:3e3457b41907fe00",
        source_derived_reference: Some("m10-source-ref:fnv1a64:10f65c9c1f283c37"),
        schedule_action_reference: Some(
            "schedule_action:SCN07.observer_projection:fnv1a64:ac2db5bd759cc804",
        ),
        evidence_predicate: "runtime.redaction_order_preserves_policy_before_projection",
    },
    ExpectedRow {
        scn_id: "SCN-07",
        expectation_id: "SCN07-R-N-HORIGIN",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN07.history_origin_violation"),
        diagnostic_location: "m10-evidence-location:fnv1a64:df5e9a9c06ea6106",
        source_derived_reference: Some("m10-source-ref:fnv1a64:10f65c9c1f283c37"),
        schedule_action_reference: Some(
            "schedule_action:SCN07.history_origin_violation:fnv1a64:42acdcfeb84fac32",
        ),
        evidence_predicate: "structural_rejection.no_publication.history_origin_redaction_violation",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-S-P-CARRIER",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:ae069294078bb62a",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: None,
        evidence_predicate: "static.three_option_carrier.live_anchor_frozen.with_monotone_lineage",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-S-N-LINEAGE",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("view-pose-missing-lineage"),
        diagnostic_location: "m10-evidence-location:fnv1a64:427cc13abd648710",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9bc5b0efb6087ac8"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-DECL-001.missing_typed_lineage_edge.live_to_anchor",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-S-N-CAPFLOOR",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("view-pose-write-after-read"),
        diagnostic_location: "m10-evidence-location:fnv1a64:220bf888f18b1afd",
        source_derived_reference: Some("m10-source-ref:fnv1a64:fcc52a1738154cd4"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-LIN-003.read_to_write_strengthening_without_reacquire",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-P-LIVE",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:0eb33f592d12b3fb",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: Some("schedule_action:SCN08.live:fnv1a64:c2c97d19751ec95e"),
        evidence_predicate: "runtime.view_pose.selects_live_option_before_expiry",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-P-EXPIRE",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:cbf19acefce33e30",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: Some(
            "schedule_action:SCN08.lease_expiry:fnv1a64:cccf8f3732042022",
        ),
        evidence_predicate: "runtime.lease_expiry_monotonically_selects_anchor_then_frozen",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-P-WRITE",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:2c7318c3945f8c00",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: Some("schedule_action:SCN08.write:fnv1a64:4cb3c37b778c9d18"),
        evidence_predicate: "runtime.write_after_option_selection_requires_current_write_capability",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-P-REACQUIRE",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:dcd7e840e693fc29",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: Some(
            "schedule_action:SCN08.fresh_reacquire:fnv1a64:75a3f3bfb458c1d2",
        ),
        evidence_predicate: "runtime.fresh_reacquire_creates_new_lineage",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-P-ROLLBACK",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-normal-fallback"),
        diagnostic_location: "m10-evidence-location:fnv1a64:641d61c9079d0ea9",
        source_derived_reference: Some("m10-source-ref:fnv1a64:9d7158e3eb2112b8"),
        schedule_action_reference: Some("schedule_action:SCN08.rollback:fnv1a64:06b5bc31c3f231da"),
        evidence_predicate: "runtime.rollback_does_not_rewind_selected_option",
    },
    ExpectedRow {
        scn_id: "SCN-08",
        expectation_id: "SCN08-R-N-REPROMOTE",
        phase: "runtime",
        artifact: ArtifactBinding::TypedCarrier("view-pose-write-after-read"),
        diagnostic_location: "m10-evidence-location:fnv1a64:3de6339c1466e68b",
        source_derived_reference: Some("m10-source-ref:fnv1a64:fcc52a1738154cd4"),
        schedule_action_reference: Some(
            "schedule_action:SCN08.write_after_read_lineage:fnv1a64:7896b43ae75fa7d0",
        ),
        evidence_predicate: "structural_rejection.no_mutation.same_lineage_repromotion_without_reacquire",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-S-P-CHECKEDPAIR",
        phase: "static",
        artifact: ArtifactBinding::PatchPair {
            carrier: "scn09-candidate-a",
            base: "scn-09/base.mir",
            candidate: "scn-09/candidate-accepted.mir",
        },
        diagnostic_location: "m10-evidence-location:fnv1a64:db297936010912bf",
        source_derived_reference: Some("m10-source-ref:fnv1a64:2041a1aba42273d9"),
        schedule_action_reference: None,
        evidence_predicate: "static.patch_candidate_pair_checked_and_compatible",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-S-N-SELFGRANT",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("scn09-candidate-b"),
        diagnostic_location: "m10-evidence-location:fnv1a64:db55b65e42271d0b",
        source_derived_reference: Some("m10-source-ref:fnv1a64:8d5f6986e77bc378"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-PATCH-003.self_grant_candidate_check.no_activation",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-S-N-MISSINGCAP",
        phase: "static",
        artifact: ArtifactBinding::TypedCarrier("scn09-candidate-c"),
        diagnostic_location: "m10-evidence-location:fnv1a64:7b272a173d6f5a2b",
        source_derived_reference: Some("m10-source-ref:fnv1a64:1812af8b8cb286d1"),
        schedule_action_reference: None,
        evidence_predicate: "diagnostic.E-PATCH-002.missing_patch_capability.no_activation",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-R-P-PIPELINE",
        phase: "runtime",
        artifact: ArtifactBinding::PatchPair {
            carrier: "scn09-candidate-a",
            base: "scn-09/base.mir",
            candidate: "scn-09/candidate-accepted.mir",
        },
        diagnostic_location: "m10-evidence-location:fnv1a64:10ed7f1652feb8f6",
        source_derived_reference: Some("m10-source-ref:fnv1a64:2041a1aba42273d9"),
        schedule_action_reference: Some(
            "schedule_action:SCN09.submit_checked_patch_a:fnv1a64:241772740143de23",
        ),
        evidence_predicate: "runtime.patch_pipeline_uses_checked_pair_not_schedule_verdict",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-R-P-INIT",
        phase: "runtime",
        artifact: ArtifactBinding::PatchPair {
            carrier: "scn09-candidate-a",
            base: "scn-09/base.mir",
            candidate: "scn-09/candidate-accepted.mir",
        },
        diagnostic_location: "m10-evidence-location:fnv1a64:c205b37ca671f3c3",
        source_derived_reference: Some("m10-source-ref:fnv1a64:2041a1aba42273d9"),
        schedule_action_reference: Some(
            "schedule_action:SCN09.submit_checked_patch_a:fnv1a64:241772740143de23",
        ),
        evidence_predicate: "runtime.patch_initializes_declared_state_addition",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-R-P-OBS",
        phase: "runtime",
        artifact: ArtifactBinding::PatchPair {
            carrier: "scn09-candidate-a",
            base: "scn-09/base.mir",
            candidate: "scn-09/candidate-accepted.mir",
        },
        diagnostic_location: "m10-evidence-location:fnv1a64:157eea8fcedbc338",
        source_derived_reference: Some("m10-source-ref:fnv1a64:2041a1aba42273d9"),
        schedule_action_reference: Some(
            "schedule_action:SCN09.submit_checked_patch_a:fnv1a64:241772740143de23",
        ),
        evidence_predicate: "runtime.patch_observer_projection_uses_new_checked_effect",
    },
    ExpectedRow {
        scn_id: "SCN-09",
        expectation_id: "SCN09-R-N-DRIFT",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN09.membership_frontier_drift"),
        diagnostic_location: "m10-evidence-location:fnv1a64:6d426b97161e15dc",
        source_derived_reference: Some("m10-source-ref:fnv1a64:2041a1aba42273d9"),
        schedule_action_reference: Some(
            "schedule_action:SCN09.membership_frontier_drift:fnv1a64:aacc80fddd4d4959",
        ),
        evidence_predicate: "structural_deferred.no_activation.membership_frontier_drift_between_admit_and_activation",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-P-S1",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:2d958c25cc28ff84",
        source_derived_reference: Some("m10-source-ref:fnv1a64:30686b005f02c665"),
        schedule_action_reference: Some("schedule_action:SCN10.save_s1:fnv1a64:e198430d8f18b26a"),
        evidence_predicate: "runtime.save_creates_world_cut_S1",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-P-S2",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:f3fc1b350d7f3a0f",
        source_derived_reference: Some("m10-source-ref:fnv1a64:30686b005f02c665"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.leave_a.lease_expiry.save_s2:fnv1a64:b4848f35500fc83a",
        ),
        evidence_predicate: "runtime.save_S2_after_leave_and_lease_expiry_creates_current_world_cut",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-P-LOADFRESH",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:ecd68d0f3cf4d5c7",
        source_derived_reference: Some("m10-source-ref:fnv1a64:30686b005f02c665"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.load_s1_fresh:fnv1a64:40113a8445010bae",
        ),
        evidence_predicate: "runtime.load_S1_into_fresh_session_preserves_past_world_cut",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-N-MERGE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/negative-stale-restore.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:426072730ad8832d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:00dcce2b6bcdd96f"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.merge_stale_s1_into_current:fnv1a64:856917b9f4806141",
        ),
        evidence_predicate: "structural_rejection.no_mutation.E-CUT-002.stale_membership_epoch_resurrection",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-N-LEASEDOCTOR",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN10.doctor_expired_lease_live"),
        diagnostic_location: "m10-evidence-location:fnv1a64:af8eb5b2ed83395d",
        source_derived_reference: Some("m10-source-ref:fnv1a64:00dcce2b6bcdd96f"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.doctor_expired_lease_live:fnv1a64:3534d9a24aa2d4c7",
        ),
        evidence_predicate: "structural_rejection.no_mutation.E-CUT-001_or_E-CUT-002.expired_lease_resurrection",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-N-CUTDOCTOR",
        phase: "runtime",
        artifact: ArtifactBinding::Schedule("SCN10.doctor_cut_receive_without_send"),
        diagnostic_location: "m10-evidence-location:fnv1a64:b9988c5f71590990",
        source_derived_reference: Some("m10-source-ref:fnv1a64:00dcce2b6bcdd96f"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.doctor_cut_receive_without_send:fnv1a64:de000de2641fade5",
        ),
        evidence_predicate: "structural_rejection.no_mutation.E-CUT-001_or_E-CUT-002.consistent_cut_violation",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-P-TIMELINE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:2af889c376d46b15",
        source_derived_reference: Some("m10-source-ref:fnv1a64:30686b005f02c665"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.timeline_panel:fnv1a64:38e17653071486ba",
        ),
        evidence_predicate: "runtime.timeline_panel_lists_S1_S2_refusals_with_reasons",
    },
    ExpectedRow {
        scn_id: "SCN-10",
        expectation_id: "SCN10-R-P-REACQUIRE",
        phase: "runtime",
        artifact: ArtifactBinding::Source("scn-10/positive.mir"),
        diagnostic_location: "m10-evidence-location:fnv1a64:e78f22255fd9da88",
        source_derived_reference: Some("m10-source-ref:fnv1a64:30686b005f02c665"),
        schedule_action_reference: Some(
            "schedule_action:SCN10.reacquire_after_load:fnv1a64:fa7e936e27e36772",
        ),
        evidence_predicate: "runtime.reacquire_after_load_is_new_occurrence_new_epoch_witness",
    },
];

fn predicate_profile() -> Value {
    json!({
        "schema_version": "m10-i1plus-correspondence-predicates-v0",
        "correspondence_rows": FROZEN_EXPECTATION_ROWS
            .iter()
            .copied()
            .map(ExpectedRow::value)
            .collect::<Vec<_>>(),
    })
}

fn flipped_predicate_profile() -> Value {
    let mut profile = predicate_profile();
    let row = profile
        .pointer_mut("/correspondence_rows")
        .and_then(Value::as_array_mut)
        .and_then(|rows| {
            rows.iter_mut().find(|row| {
                row.get("expectation_id").and_then(Value::as_str) == Some("SCN09-S-N-SELFGRANT")
            })
        })
        .expect("profile keeps SCN09 self-grant predicate");
    row.pointer_mut("/evidence_predicate")
        .expect("SCN09 self-grant row has an exact predicate")
        .clone_from(&json!("Accepted"));
    profile
}

fn predicate_profile_missing_row() -> Value {
    let mut profile = predicate_profile();
    profile
        .pointer_mut("/correspondence_rows")
        .and_then(Value::as_array_mut)
        .expect("profile keeps correspondence rows")
        .retain(|row| row.get("expectation_id").and_then(Value::as_str) != Some("SCN06-S-N-ROW"));
    profile
}

fn predicate_profile_with_phase(expectation_id: &str, phase: &str) -> Value {
    let mut profile = predicate_profile();
    let row = profile
        .pointer_mut("/correspondence_rows")
        .and_then(Value::as_array_mut)
        .and_then(|rows| {
            rows.iter_mut().find(|row| {
                row.get("expectation_id").and_then(Value::as_str) == Some(expectation_id)
            })
        })
        .unwrap_or_else(|| panic!("profile keeps row {expectation_id}"));
    row.pointer_mut("/phase")
        .expect("correspondence row has a phase")
        .clone_from(&json!(phase));
    profile
}

fn predicate_profile_with_row_field(expectation_id: &str, field: &str, value: Value) -> Value {
    let mut profile = predicate_profile();
    let rows = profile
        .pointer_mut("/correspondence_rows")
        .and_then(Value::as_array_mut)
        .expect("predicate profile keeps correspondence rows");
    let row = rows
        .iter_mut()
        .find(|row| row.get("expectation_id").and_then(Value::as_str) == Some(expectation_id))
        .and_then(Value::as_object_mut)
        .unwrap_or_else(|| panic!("profile keeps mutable row {expectation_id}"));
    assert!(
        row.contains_key(field),
        "correspondence row {expectation_id} has field {field}"
    );
    row.insert(field.to_string(), value);
    profile
}

fn correspondence_rows(profile: &Value) -> &[Value] {
    profile
        .pointer("/correspondence_rows")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .expect("predicate profile is row-shaped")
}

fn frozen_row_ids_for_phase(profile: &Value, phase: &str) -> Vec<String> {
    correspondence_rows(profile)
        .iter()
        .filter(|row| row.get("phase").and_then(Value::as_str) == Some(phase))
        .map(|row| {
            row.get("expectation_id")
                .and_then(Value::as_str)
                .expect("correspondence row has id")
                .to_string()
        })
        .collect()
}

fn frozen_row_ids(profile: &Value) -> Vec<String> {
    correspondence_rows(profile)
        .iter()
        .map(|row| {
            row.get("expectation_id")
                .and_then(Value::as_str)
                .expect("correspondence row has id")
                .to_string()
        })
        .collect()
}

fn correspondence_row<'a>(profile: &'a Value, id: &str) -> &'a Value {
    correspondence_rows(profile)
        .iter()
        .find(|row| row.get("expectation_id").and_then(Value::as_str) == Some(id))
        .unwrap_or_else(|| panic!("missing correspondence row {id}: {profile:#}"))
}

fn typed_falsifier(fault: &str) -> Value {
    let mutation = match fault {
        "source_sensitivity_changed_text_same_name" => {
            json!({"kind": "rewrite_source_text_same_path", "source": "scn-02/positive.mir", "edit": "rename_module_only"})
        }
        "construct_deletion_visible_field" => {
            json!({"kind": "delete_construct", "source": "scn-01/positive.mir", "construct": "visible observer_safe fields (position)"})
        }
        "m7_bypass_artifact_mismatch" => {
            json!({"kind": "attach_checked_artifact_from_other_source", "source": "scn-02/positive.mir", "artifact_source": "scn-02/negative-missing-capability-row.mir"})
        }
        "negative_source_positive_core_attachment" => {
            json!({"kind": "attach_core_to_rejected_source", "source": "scn-01/negative-missing-visibility-denied.mir", "core_source": "scn-01/positive.mir"})
        }
        "scn09_patch_provenance_mismatch" => {
            json!({"kind": "rewrite_patch_carrier_candidate_identity", "carrier": "scn09-candidate-b", "candidate_source": "scn-09/candidate-accepted.mir"})
        }
        "typed_carrier_same_id_content_hash_mismatch" => {
            json!({"kind": "rewrite_typed_carrier_content_same_id", "carrier": "portal-secret-redaction-policy", "preserve_id": true, "edit": {"required_failures": []}})
        }
        "schedule_action_same_id_content_hash_mismatch" => {
            json!({"kind": "rewrite_schedule_action_content_same_id", "action_id": "SCN02.attack.without_capability", "preserve_id": true, "edit": {"principal": "target"}})
        }
        "source_same_path_semantic_change_parse_checkable" => {
            json!({"kind": "rewrite_source_text_same_path", "source": "scn-02/positive.mir", "edit": "change_attack_damage_parse_checkable"})
        }
        "schedule_same_id_meaningful_content_change" => {
            json!({"kind": "rewrite_schedule_case_content_same_id", "case_id": "SCN02.attack.without_capability", "preserve_id": true, "edit": {"target": "other_target"}})
        }
        "failure_no_mutation" => {
            json!({"kind": "force_mutation_after_rejected_step", "source": "scn-03/negative-write-before-verdict.mir"})
        }
        "fallback_repromotion_without_reacquire" => {
            json!({"kind": "fallback_lineage_repromote_without_reacquire", "carrier": "view-pose-write-after-read"})
        }
        "scn08_normal_carrier_missing_anchor_to_frozen" => {
            json!({
                "kind": "rewrite_typed_carrier_content_same_id",
                "carrier": "view-pose-normal-fallback",
                "preserve_id": true,
                "edit": {
                    "options": [
                        {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                        {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": [{"from": "live", "to": "anchor"}]},
                        {"kind": "frozen", "target": "default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "static", "lineage_edges": []}
                    ]
                }
            })
        }
        "scn08_normal_carrier_mutated_default_pose" => {
            json!({
                "kind": "rewrite_typed_carrier_content_same_id",
                "carrier": "view-pose-normal-fallback",
                "preserve_id": true,
                "edit": {
                    "options": [
                        {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                        {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": [{"from": "live", "to": "anchor"}]},
                        {"kind": "frozen", "target": "mutated_default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "static", "lineage_edges": [{"from": "anchor", "to": "frozen"}]}
                    ]
                }
            })
        }
        "scn08_normal_carrier_mutated_static_epoch" => {
            json!({
                "kind": "rewrite_typed_carrier_content_same_id",
                "carrier": "view-pose-normal-fallback",
                "preserve_id": true,
                "edit": {
                    "options": [
                        {"kind": "live", "target": "live_pose", "lease": "lease:view_pose:live", "capability": "cap:relation:view_pose:live", "epoch": "avatar_session", "lineage_edges": []},
                        {"kind": "anchor", "target": "room_anchor", "lease": "lease:view_pose:anchor", "capability": "cap:relation:view_pose:anchor", "epoch": "room_epoch", "lineage_edges": [{"from": "live", "to": "anchor"}]},
                        {"kind": "frozen", "target": "default_pose", "lease": "lease:view_pose:frozen", "capability": "cap:relation:view_pose:frozen", "epoch": "mutated_static", "lineage_edges": [{"from": "anchor", "to": "frozen"}]}
                    ]
                }
            })
        }
        "scn08_source_primary_target_carrier_mismatch" => {
            json!({
                "kind": "rewrite_source_text_same_path",
                "source": "scn-08/positive.mir",
                "edit": "scn08_primary_target_mutated_live_pose_to_live_anchor"
            })
        }
        "projection_history_origin_redaction_violation" => {
            json!({"kind": "emit_projection_history_without_origin_redaction", "source": "scn-12/bird-relation.mir"})
        }
        "restore_stale_membership_resurrection" => {
            json!({"kind": "merge_stale_save_over_new_membership", "source": "scn-10/negative-stale-restore.mir"})
        }
        "deterministic_replay_drift" => {
            json!({"kind": "alter_replay_order_same_profile", "source": "scn-02/positive.mir"})
        }
        other => panic!("unknown typed falsifier {other}"),
    };
    json!({
        "schema_version": "m10-i1plus-source-run-mutation-v0",
        "id": fault,
        "mutation": mutation
    })
}

fn request_with_predicates(predicates: Value) -> M10SourceRunRequest {
    request_with_inputs(action_schedule(), typed_carriers(), predicates)
}

fn request_with_inputs(schedule: Value, carriers: Value, predicates: Value) -> M10SourceRunRequest {
    M10SourceRunRequest::corpus_path(corpus_root_string())
        .typed_schedule_json(schedule)
        .typed_carriers_json(carriers)
        .predicate_profile_json(predicates)
        .forbid_fixture_name_result_lookup()
        .forbid_expected_output_sidecars()
}

fn run_conformance_with(predicates: Value) -> Value {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report: M10ConformanceReport = system
        .run_conformance(request_with_predicates(predicates))
        .expect("M10 conformance report is produced from source corpus");
    serde_json::to_value(report).expect("M10 conformance report serializes")
}

fn run_conformance() -> Value {
    run_conformance_with(predicate_profile())
}

fn run_conformance_result() -> Result<Value, String> {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    system
        .run_conformance(request_with_predicates(predicate_profile()))
        .map(|report| serde_json::to_value(report).expect("M10 conformance report serializes"))
}

fn run_conformance_with_schedule_and_predicates(schedule: Value, predicates: Value) -> Value {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report: M10ConformanceReport = system
        .run_conformance(request_with_inputs(schedule, typed_carriers(), predicates))
        .expect("M10 conformance report is produced from typed source corpus inputs");
    serde_json::to_value(report).expect("M10 conformance report serializes")
}

fn schedule_with_case_id_replaced(original: &str, replacement: &str) -> Value {
    let mut schedule = action_schedule();
    let cases = schedule
        .pointer_mut("/cases")
        .and_then(Value::as_array_mut)
        .expect("typed schedule keeps cases");
    let case = cases
        .iter_mut()
        .find(|case| case.get("id").and_then(Value::as_str) == Some(original))
        .unwrap_or_else(|| panic!("typed schedule keeps case {original}"));
    case.pointer_mut("/id")
        .expect("schedule case has an id")
        .clone_from(&json!(replacement));
    schedule
}

fn schedule_with_case_moved_before(case_id: &str, before_id: &str) -> Value {
    let mut schedule = action_schedule();
    let cases = schedule
        .pointer_mut("/cases")
        .and_then(Value::as_array_mut)
        .expect("typed schedule keeps cases");
    let from = cases
        .iter()
        .position(|case| case.get("id").and_then(Value::as_str) == Some(case_id))
        .unwrap_or_else(|| panic!("typed schedule keeps case {case_id}"));
    let case = cases.remove(from);
    let to = cases
        .iter()
        .position(|case| case.get("id").and_then(Value::as_str) == Some(before_id))
        .unwrap_or_else(|| panic!("typed schedule keeps case {before_id}"));
    cases.insert(to, case);
    schedule
}

fn profile_rebound_to_actual_evidence(report: &Value) -> Value {
    let rows = report
        .pointer("/verification/inventory/correspondence_rows")
        .and_then(Value::as_array)
        .expect("verifier exposes row-level correspondence evidence");
    let correspondence_rows = rows
        .iter()
        .map(|row| {
            let evidence = match row.get("actual_evidence") {
                Some(Value::Object(_)) => row.get("actual_evidence").expect("checked above"),
                Some(Value::Array(candidates)) if candidates.len() == 1 => &candidates[0],
                Some(other) => panic!(
                    "{} has non-unique actual evidence candidate: {other:#}",
                    row.get("expectation_id")
                        .and_then(Value::as_str)
                        .unwrap_or("?")
                ),
                None => panic!(
                    "{} has no actual evidence candidate",
                    row.get("expectation_id")
                        .and_then(Value::as_str)
                        .unwrap_or("?")
                ),
            };
            json!({
                "scn_id": evidence
                    .get("scn_id")
                    .expect("actual evidence has scn id"),
                "expectation_id": row
                    .get("expectation_id")
                    .expect("inventory row keeps expectation id"),
                "phase": evidence
                    .get("phase")
                    .expect("actual evidence has phase"),
                "carrier_kind": evidence
                    .get("carrier_kind")
                    .expect("actual evidence has carrier kind"),
                "artifact_identity": evidence
                    .get("artifact_identity")
                    .expect("actual evidence has artifact identity"),
                "diagnostic_location": evidence
                    .get("diagnostic_location")
                    .expect("actual evidence has diagnostic location"),
                "source_derived_reference": evidence
                    .get("source_derived_reference")
                    .expect("actual evidence has source ref"),
                "schedule_action_reference": evidence
                    .get("schedule_action_reference")
                    .cloned()
                    .unwrap_or(Value::Null),
                "evidence_predicate": evidence
                    .get("evidence_predicate")
                    .expect("actual evidence has predicate"),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schema_version": "m10-i1plus-correspondence-predicates-v0",
        "correspondence_rows": correspondence_rows,
    })
}

fn runtime_domain_delta_probe(domain: &str, source: &str) -> Result<Value, String> {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let mutation = json!({
        "schema_version": "m10-i1plus-source-run-mutation-v0",
        "id": format!("projection_delta_probe_{domain}_only"),
        "mutation": {
            "kind": "projection_delta_probe",
            "domain": domain,
            "source": source,
            "mutation": format!("{domain}_only_semantic_delta"),
        },
    });
    system
        .run_conformance(
            request_with_predicates(predicate_profile()).typed_input_mutation(mutation),
        )
        .map(|report| serde_json::to_value(report).expect("domain delta probe serializes"))
}

fn assert_pointer(value: &Value, pointer: &str, expected: Value) {
    assert_eq!(
        value.pointer(pointer),
        Some(&expected),
        "unexpected value at {pointer}: {value:#}"
    );
}

fn assert_absent_or_null(value: &Value, pointer: &str) {
    assert!(
        value.pointer(pointer).is_none_or(Value::is_null),
        "expected {pointer} to be absent/null: {value:#}"
    );
}

fn assert_status(value: &Value, pointer: &str, expected: &str) {
    let status = value
        .pointer(pointer)
        .and_then(|entry| {
            entry
                .as_str()
                .or_else(|| entry.pointer("/status").and_then(Value::as_str))
        })
        .unwrap_or_else(|| panic!("missing status at {pointer}: {value:#}"));
    assert_eq!(
        status, expected,
        "unexpected status at {pointer}: {value:#}"
    );
}

fn assert_has_any_pointer(value: &Value, pointers: &[&str], context: &str) {
    assert!(
        pointers
            .iter()
            .any(|pointer| value.pointer(pointer).is_some()),
        "{context} must expose at least one of {pointers:?}: {value:#}"
    );
}

fn missing_pointers<'a>(value: &Value, pointers: &'a [&'a str]) -> Vec<&'a str> {
    pointers
        .iter()
        .copied()
        .filter(|pointer| value.pointer(pointer).is_none())
        .collect()
}

fn first_existing_pointer<'a, 'b>(
    value: &'a Value,
    pointers: &'b [&'b str],
) -> Option<(&'b str, &'a Value)> {
    pointers
        .iter()
        .find_map(|pointer| value.pointer(pointer).map(|value| (*pointer, value)))
}

fn has_any_pointer(value: &Value, pointers: &[&str]) -> bool {
    pointers
        .iter()
        .any(|pointer| value.pointer(pointer).is_some())
}

fn is_structured_transition_or_validator_trace(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            map.keys().any(|key| {
                matches!(
                    key.as_str(),
                    "validator"
                        | "validator_id"
                        | "validator_name"
                        | "validator_trace"
                        | "transition"
                        | "transition_kind"
                        | "transition_trace"
                        | "state_before"
                        | "state_after"
                        | "before_identity"
                        | "after_identity"
                        | "diagnostic"
                        | "diagnostic_code"
                        | "component"
                        | "input_identity"
                        | "actual_identity"
                        | "expected_identity"
                        | "consumed_version"
                        | "result_version"
                )
            }) || map
                .values()
                .any(is_structured_transition_or_validator_trace)
        }
        Value::Array(values) => values
            .iter()
            .any(is_structured_transition_or_validator_trace),
        _ => false,
    }
}

fn has_structured_trace(value: &Value, pointers: &[&str]) -> bool {
    pointers
        .iter()
        .filter_map(|pointer| value.pointer(pointer))
        .any(is_structured_transition_or_validator_trace)
}

fn transition_entries<'a>(row: &'a Value, expectation_id: &str) -> &'a [Value] {
    row.pointer("/runtime_transition_trace/transition_trace")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_else(|| {
            panic!(
                "{expectation_id} must expose runtime_transition_trace.transition_trace: {row:#}"
            )
        })
}

fn find_transition_entry<'a>(row: &'a Value, expectation_id: &str, transition: &str) -> &'a Value {
    transition_entries(row, expectation_id)
        .iter()
        .find(|entry| entry.get("transition").and_then(Value::as_str) == Some(transition))
        .unwrap_or_else(|| panic!("{expectation_id} must include transition {transition}: {row:#}"))
}

fn require_transition_entry<'a>(
    row: &'a Value,
    expectation_id: &str,
    transition: &str,
    failures: &mut Vec<String>,
) -> Option<&'a Value> {
    let Some(entries) = row
        .pointer("/runtime_transition_trace/transition_trace")
        .and_then(Value::as_array)
    else {
        failures.push(format!(
            "{expectation_id} missing runtime_transition_trace.transition_trace"
        ));
        return None;
    };
    let entry = entries
        .iter()
        .find(|entry| entry.get("transition").and_then(Value::as_str) == Some(transition));
    if entry.is_none() {
        failures.push(format!(
            "{expectation_id} missing transition {transition} in runtime_transition_trace"
        ));
    }
    entry
}

fn missing_hash_bundle_keys(entry: &Value, side: &str, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter_map(|key| {
            let pointer = format!("/{side}/{key}");
            entry
                .pointer(&pointer)
                .and_then(Value::as_str)
                .filter(|value| value.starts_with("fnv1a64:"))
                .map(|_| ())
                .is_none()
                .then_some(pointer)
        })
        .collect()
}

fn assert_transition_has_hash_bundle(
    entry: &Value,
    expectation_id: &str,
    transition: &str,
    keys: &[&str],
) {
    let mut missing = Vec::new();
    missing.extend(missing_hash_bundle_keys(entry, "before", keys));
    missing.extend(missing_hash_bundle_keys(entry, "after", keys));
    assert!(
        missing.is_empty(),
        "{expectation_id} {transition} must expose an actual semantic before/after hash bundle for {keys:?}; missing {missing:?}; entry={entry:#}"
    );
}

fn assert_transition_uses_non_root_source_ref(
    entry: &Value,
    expectation_id: &str,
    transition: &str,
) {
    let source_ref = entry.pointer("/source_ref").unwrap_or_else(|| {
        panic!("{expectation_id} {transition} must expose a source_ref: {entry:#}")
    });
    let start_line = source_ref
        .get("start_line")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let start_column = source_ref
        .get("start_column")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    assert!(
        start_line > 1 || start_column > 1,
        "{expectation_id} {transition} must cite the actual semantic source range, not the program root/full-file range: {source_ref:#}"
    );
}

fn assert_transition_preserves_hash_bundle(
    entry: &Value,
    expectation_id: &str,
    transition: &str,
    keys: &[&str],
) {
    assert_transition_has_hash_bundle(entry, expectation_id, transition, keys);
    let mut changed = Vec::new();
    for key in keys {
        let before = entry.pointer(&format!("/before/{key}"));
        let after = entry.pointer(&format!("/after/{key}"));
        if before != after {
            changed.push(format!("{key}: before={before:?} after={after:?}"));
        }
    }
    assert!(
        changed.is_empty(),
        "{expectation_id} {transition} rejection must preserve the full semantic hash bundle; changed {changed:?}; entry={entry:#}"
    );
}

fn require_domain_projection_provenance(
    entry: &Value,
    expectation_id: &str,
    transition: &str,
    failures: &mut Vec<String>,
) {
    let mut accessor_components = Vec::new();
    for (domain, hash_key) in DOMAIN_PROJECTION_REQUIREMENTS {
        let base = format!("/domain_projection_provenance/{domain}");
        let mut missing = Vec::new();
        for suffix in [
            "actual_accessor",
            "component",
            "before_projection_identity",
            "after_projection_identity",
            "hash_key",
        ] {
            let pointer = format!("{base}/{suffix}");
            if entry.pointer(&pointer).is_none() {
                missing.push(pointer);
            }
        }
        if !missing.is_empty() {
            failures.push(format!(
                "{expectation_id} {transition} missing {domain} projection provenance {missing:?}"
            ));
            continue;
        }
        if entry.pointer(&format!("{base}/hash_key")) != Some(&json!(hash_key)) {
            failures.push(format!(
                "{expectation_id} {transition} {domain} provenance must bind hash_key={hash_key}; got {:?}",
                entry.pointer(&format!("{base}/hash_key"))
            ));
        }
        let accessor = entry
            .pointer(&format!("{base}/actual_accessor"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        let component = entry
            .pointer(&format!("{base}/component"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if accessor.is_empty() || component.is_empty() {
            failures.push(format!(
                "{expectation_id} {transition} {domain} provenance must name non-empty actual accessor and component"
            ));
        }
        let pair = format!("{accessor}::{component}");
        if let Some((prior_domain, _)) = accessor_components
            .iter()
            .find(|(_, prior_pair)| prior_pair == &pair)
        {
            failures.push(format!(
                "{expectation_id} {transition} {domain} shares accessor/component with {prior_domain}: {pair}"
            ));
        }
        accessor_components.push((domain, pair));

        let before_hash = entry.pointer(&format!("/before/{hash_key}"));
        let after_hash = entry.pointer(&format!("/after/{hash_key}"));
        let before_projection = entry.pointer(&format!("{base}/before_projection_identity"));
        let after_projection = entry.pointer(&format!("{base}/after_projection_identity"));
        if before_hash != after_hash && before_projection == after_projection {
            failures.push(format!(
                "{expectation_id} {transition} changed {hash_key} but kept the same {domain} projection identity"
            ));
        }
    }
}

fn require_persistent_semantic_session(
    row: &Value,
    expectation_id: &str,
    expected_layer: &str,
    failures: &mut Vec<String>,
) {
    let trace = match row.pointer("/runtime_transition_trace") {
        Some(trace) => trace,
        None => {
            failures.push(format!(
                "{expectation_id} missing runtime_transition_trace for semantic session check"
            ));
            return;
        }
    };
    let text = serde_json::to_string(trace).expect("runtime trace serializes");
    if text.contains("M10ScenarioState") {
        failures.push(format!(
            "{expectation_id} must not cite M10ScenarioState as semantic state owner"
        ));
    }

    let owner_layer = trace
        .pointer("/semantic_state_owner/layer")
        .and_then(Value::as_str);
    let owner_session = trace
        .pointer("/semantic_state_owner/session_id")
        .and_then(Value::as_str);
    let row_session = trace.pointer("/session_id").and_then(Value::as_str);

    if owner_layer != Some(expected_layer) {
        failures.push(format!(
            "{expectation_id} semantic_state_owner.layer must be {expected_layer}, got {owner_layer:?}"
        ));
    }
    let Some(owner_session) = owner_session else {
        failures.push(format!(
            "{expectation_id} missing semantic_state_owner.session_id"
        ));
        return;
    };
    if row_session != Some(owner_session) {
        failures.push(format!(
            "{expectation_id} runtime session_id must be the same persistent semantic owner session; row={row_session:?}, owner={owner_session:?}"
        ));
    }
    let expected_prefix = format!("{}-", expected_layer.to_ascii_lowercase());
    if !owner_session.starts_with(&expected_prefix) {
        failures.push(format!(
            "{expectation_id} semantic session must be an {expected_layer} persistent session id starting with {expected_prefix:?}, got {owner_session:?}"
        ));
    }
}

fn require_structured_runtime_provenance(
    row: &Value,
    expectation_id: &str,
    failures: &mut Vec<String>,
) {
    let missing = missing_pointers(
        row,
        &[
            "/runtime_transition_trace/session_id",
            "/runtime_transition_trace/monotone_trace_range",
            "/runtime_transition_trace/program_artifact/checked_effect_ref",
            "/runtime_transition_trace/program_artifact/source_ref",
            "/runtime_transition_trace/schedule_action/action_id",
            "/runtime_transition_trace/schedule_action/reference",
            "/runtime_transition_trace/m8_m9_receipt/receipt_id",
            "/runtime_transition_trace/m8_m9_receipt/m9_resolution_ref",
            "/runtime_transition_trace/m8_m9_receipt/m8_runtime_ref",
        ],
    );
    if !missing.is_empty() {
        failures.push(format!(
            "{expectation_id} missing structured runtime provenance {missing:?}"
        ));
    }
    if row
        .pointer("/runtime_transition_trace/receipt_origin")
        .is_some_and(Value::is_string)
    {
        failures.push(format!(
            "{expectation_id} must expose a structured persistent M8/M9 receipt, not only a receipt_origin string"
        ));
    }
}

fn require_transition_sequence(
    row: &Value,
    expectation_id: &str,
    expected: &[&str],
    failures: &mut Vec<String>,
) {
    let Some(entries) = row
        .pointer("/runtime_transition_trace/transition_trace")
        .and_then(Value::as_array)
    else {
        failures.push(format!(
            "{expectation_id} missing runtime_transition_trace.transition_trace"
        ));
        return;
    };
    let actual = entries
        .iter()
        .filter_map(|entry| entry.get("transition").and_then(Value::as_str))
        .collect::<Vec<_>>();
    for transition in expected {
        if !actual.contains(transition) {
            failures.push(format!(
                "{expectation_id} missing transition {transition}; actual transitions={actual:?}"
            ));
        }
    }
}

fn require_no_mutation_reject_snapshot(
    row: &Value,
    expectation_id: &str,
    transition: &str,
    diagnostic: &str,
    failures: &mut Vec<String>,
) {
    let Some(entries) = row
        .pointer("/runtime_transition_trace/transition_trace")
        .and_then(Value::as_array)
    else {
        failures.push(format!(
            "{expectation_id} missing reject transition trace {transition}"
        ));
        return;
    };
    let Some(entry) = entries
        .iter()
        .find(|entry| entry.get("transition").and_then(Value::as_str) == Some(transition))
    else {
        failures.push(format!(
            "{expectation_id} missing reject transition {transition}"
        ));
        return;
    };

    let mut missing = Vec::new();
    missing.extend(missing_hash_bundle_keys(
        entry,
        "before",
        &FIVE_DOMAIN_HASH_KEYS,
    ));
    missing.extend(missing_hash_bundle_keys(
        entry,
        "after",
        &FIVE_DOMAIN_HASH_KEYS,
    ));
    if !missing.is_empty() {
        failures.push(format!(
            "{expectation_id} {transition} missing five-domain before/after hash bundle {missing:?}"
        ));
    }
    let changed = changed_hash_keys(entry, &FIVE_DOMAIN_HASH_KEYS);
    if !changed.is_empty() {
        failures.push(format!(
            "{expectation_id} {transition} must preserve five-domain snapshots on reject; changed={changed:?}"
        ));
    }
    if entry.pointer("/diagnostic/code") != Some(&json!(diagnostic)) {
        failures.push(format!(
            "{expectation_id} {transition} must carry diagnostic {diagnostic}; entry={entry:#}"
        ));
    }
    for pointer in [
        "/diagnostic/source_ref",
        "/program_artifact/source_ref",
        "/schedule_action/reference",
    ] {
        if entry.pointer(pointer).is_none() {
            failures.push(format!(
                "{expectation_id} {transition} missing per-transition provenance {pointer}"
            ));
        }
    }
}

fn require_observer_publication_origin_and_redaction(
    row: &Value,
    expectation_id: &str,
    forbidden_fields: &[&str],
    failures: &mut Vec<String>,
) {
    let missing = missing_pointers(
        row,
        &[
            "/observer_publication/subject_history_occurrence",
            "/observer_publication/publication_origin",
            "/observer_publication/source_ref",
            "/observer_publication/policy_carrier_ref",
            "/observer_publication/redaction/input_label",
            "/observer_publication/redaction/output_label",
            "/observer_publication/redaction/order_proof",
            "/observer_publication/exported_fields",
            "/observer_publication/raw_authority_payload_present",
            "/observer_publication/raw_witness_payload_present",
            "/observer_publication/raw_verification_payload_present",
        ],
    );
    if !missing.is_empty() {
        failures.push(format!(
            "{expectation_id} missing observer publication origin/redaction evidence {missing:?}"
        ));
    }
    for pointer in [
        "/observer_publication/raw_authority_payload_present",
        "/observer_publication/raw_witness_payload_present",
        "/observer_publication/raw_verification_payload_present",
    ] {
        if let Some(value) = row.pointer(pointer)
            && value != &json!(false)
        {
            failures.push(format!(
                "{expectation_id} must not expose raw authority/witness/verification payloads at {pointer}: {value:#}"
            ));
        }
    }
    if let Some(fields) = row
        .pointer("/observer_publication/exported_fields")
        .and_then(Value::as_array)
    {
        for forbidden in forbidden_fields {
            if fields
                .iter()
                .any(|field| field.as_str() == Some(*forbidden))
            {
                failures.push(format!(
                    "{expectation_id} must not publish forbidden field {forbidden}: {fields:#?}"
                ));
            }
        }
    }
}

fn require_pointer<'a>(
    value: &'a Value,
    pointer: &str,
    context: &str,
    failures: &mut Vec<String>,
) -> Option<&'a Value> {
    match value.pointer(pointer) {
        Some(value) => Some(value),
        None => {
            failures.push(format!("{context} missing {pointer}"));
            None
        }
    }
}

fn require_object_pointer<'a>(
    value: &'a Value,
    pointer: &str,
    context: &str,
    failures: &mut Vec<String>,
) -> Option<&'a Map<String, Value>> {
    match require_pointer(value, pointer, context, failures) {
        Some(actual) => match actual.as_object() {
            Some(object) => Some(object),
            None => {
                failures.push(format!(
                    "{context} expected {pointer} to be a structured object, got {actual:#}"
                ));
                None
            }
        },
        None => None,
    }
}

fn require_non_empty_string_pointer<'a>(
    value: &'a Value,
    pointer: &str,
    context: &str,
    failures: &mut Vec<String>,
) -> Option<&'a str> {
    match require_pointer(value, pointer, context, failures) {
        Some(actual) => match actual.as_str() {
            Some(value) if !value.trim().is_empty() => Some(value),
            Some(_) => {
                failures.push(format!("{context} expected {pointer} to be non-empty"));
                None
            }
            None => {
                failures.push(format!(
                    "{context} expected {pointer} to be a string, got {actual:#}"
                ));
                None
            }
        },
        None => None,
    }
}

fn require_any_pointer<'a, 'b>(
    value: &'a Value,
    pointers: &'b [&'b str],
    context: &str,
    failures: &mut Vec<String>,
) -> Option<(&'b str, &'a Value)> {
    match first_existing_pointer(value, pointers) {
        Some(found) => Some(found),
        None => {
            failures.push(format!(
                "{context} missing every allowed pointer {pointers:?}"
            ));
            None
        }
    }
}

fn require_json_pointer_not_equal(
    value: &Value,
    left: &str,
    right: &str,
    context: &str,
    failures: &mut Vec<String>,
) {
    let left_value = require_pointer(value, left, context, failures).cloned();
    let right_value = require_pointer(value, right, context, failures).cloned();
    if left_value.is_some() && right_value.is_some() && left_value == right_value {
        failures.push(format!(
            "{context} expected {left} and {right} to differ; both={left_value:?}"
        ));
    }
}

fn require_json_pointer_equal(
    value: &Value,
    left: &str,
    right: &str,
    context: &str,
    failures: &mut Vec<String>,
) {
    let left_value = require_pointer(value, left, context, failures).cloned();
    let right_value = require_pointer(value, right, context, failures).cloned();
    if left_value.is_some() && right_value.is_some() && left_value != right_value {
        failures.push(format!(
            "{context} expected {left} and {right} to match; left={left_value:?}, right={right_value:?}"
        ));
    }
}

fn require_json_pointer_equal_across(
    left_value: &Value,
    left: &str,
    right_value: &Value,
    right: &str,
    context: &str,
    failures: &mut Vec<String>,
) {
    let left_actual = require_pointer(left_value, left, context, failures).cloned();
    let right_actual = require_pointer(right_value, right, context, failures).cloned();
    if left_actual.is_some() && right_actual.is_some() && left_actual != right_actual {
        failures.push(format!(
            "{context} expected {left} and {right} to match across rows; left={left_actual:?}, right={right_actual:?}"
        ));
    }
}

fn fail_if_true_pointer(value: &Value, pointer: &str, context: &str, failures: &mut Vec<String>) {
    if value.pointer(pointer) == Some(&json!(true)) {
        failures.push(format!("{context} must not report {pointer}=true"));
    }
}

fn require_scn10_canon_line_binding(row: &Value, expectation_id: &str, failures: &mut Vec<String>) {
    let context = format!("{expectation_id} Canon SCN10 bundle binding");
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/canon_refs/0/source_path",
        json!(M10_SCN10_CANON_SOURCE_PATH),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/canon_refs/0/line_start",
        json!(M10_SCN10_CANON_LINE_START),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/canon_refs/0/line_end",
        json!(M10_SCN10_CANON_LINE_END),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/canon_refs/0/scn_id",
        json!("SCN-10"),
        &context,
        failures,
    );
}

fn require_scn10_no_stale_resurrection_canon_binding(
    row: &Value,
    expectation_id: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} Canon THM-003 no-stale-resurrection binding");
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/no_stale_resurrection/canon_refs/0/source_path",
        json!(M10_NO_STALE_RESURRECTION_CANON_SOURCE_PATH),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/no_stale_resurrection/canon_refs/0/line_start",
        json!(M10_NO_STALE_RESURRECTION_CANON_LINE_START),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/no_stale_resurrection/canon_refs/0/line_end",
        json!(M10_NO_STALE_RESURRECTION_CANON_LINE_END),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/no_stale_resurrection/canon_refs/0/theorem",
        json!("THM-003"),
        &context,
        failures,
    );
}

fn require_scn10_scenario_thm003_guard_binding(
    row: &Value,
    expectation_id: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} Canon SCN10 stale-reject scenario/THM-003 guard");
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/canon_refs/0/source_path",
        json!(M10_SCN10_SCENARIO_SOURCE_PATH),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/canon_refs/0/line_start",
        json!(M10_SCN10_SCENARIO_LINE_START),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/canon_refs/0/line_end",
        json!(M10_SCN10_SCENARIO_LINE_END),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/canon_refs/0/scn_id",
        json!("SCN-10"),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/thm_refs/0/theorem",
        json!("THM-003"),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/thm_refs/0/source_path",
        json!(M10_NO_STALE_RESURRECTION_CANON_SOURCE_PATH),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/thm_refs/0/line_start",
        json!(M10_NO_STALE_RESURRECTION_CANON_LINE_START),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_stale_reject_guard/thm_refs/0/line_end",
        json!(M10_NO_STALE_RESURRECTION_CANON_LINE_END),
        &context,
        failures,
    );
}

fn require_scn10_current_s2_positive_lineage(
    row: &Value,
    s2_row: &Value,
    expectation_id: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} SCN10 current S2 positive lineage guard");
    require_scn10_scenario_thm003_guard_binding(row, expectation_id, failures);
    for (index, event) in [
        (0, "save_s1"),
        (1, "a_leave"),
        (2, "maintainer_actual_lease_expiry"),
        (3, "save_s2"),
    ] {
        require_json_value_pointer(
            row,
            &format!("/runtime_transition_trace/scn10_current_s2_lineage/events/{index}"),
            json!(event),
            &context,
            failures,
        );
    }
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/fresh_initial_negative_session_used",
        false,
        &context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/sentinel_only_live_floor_used",
        false,
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/sentinel_only_control/result",
        json!("insufficient_without_persistent_s2_lineage"),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/sentinel_only_control/sentinel",
        json!("M8LiveFloor::with_stale_membership(\"m10-stale\")"),
        &context,
        failures,
    );
    require_json_pointer_equal_across(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/session_id",
        s2_row,
        "/runtime_transition_trace/session_id",
        &context,
        failures,
    );
}

fn require_scn10_current_s2_no_domain_mutation(
    row: &Value,
    expectation_id: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} current S2 five-domain no-mutation guard");
    require_bool_pointer(
        row,
        "/runtime_transition_trace/current_s2_no_mutation/no_current_s2_mutation",
        true,
        &context,
        failures,
    );
    for hash_key in [
        "store_hash",
        "membership_hash",
        "grant_hash",
        "relation_hash",
        "config_hash",
    ] {
        let before =
            format!("/runtime_transition_trace/current_s2_no_mutation/original_before/{hash_key}");
        let after =
            format!("/runtime_transition_trace/current_s2_no_mutation/final_after/{hash_key}");
        require_json_pointer_equal(row, &before, &after, &context, failures);
    }
}

fn require_scn10_s2_cut_clone_doctor_guard(
    row: &Value,
    expectation_id: &str,
    mutation_kind: &str,
    diagnostic_code: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} actual S2 cut clone doctor guard");
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/mutation_kind",
        json!(mutation_kind),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/base_cut/save_id",
        json!("S2"),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/result",
        json!("rejected"),
        &context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/diagnostic/code",
        json!(diagnostic_code),
        &context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/rejected_before_current_s2_restore",
        true,
        &context,
        failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/clone_identity_before",
        "/runtime_transition_trace/s2_cut_clone_mutation/clone_identity_after",
        &context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/scn10_current_s2_lineage/session_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/base_cut/session_id",
        &context,
        failures,
    );
    require_scn10_current_s2_no_domain_mutation(row, expectation_id, failures);
}

fn require_scn10_cutdoctor_send_receive_edge_guard(row: &Value, failures: &mut Vec<String>) {
    let context = "SCN10-R-N-CUTDOCTOR actual send/receive cut-prefix doctor guard";
    require_object_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor",
        context,
        failures,
    );
    require_object_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation",
        context,
        failures,
    );
    for pointer in [
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/send_occurrence/provenance/source_accessor",
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/receive_occurrence/provenance/source_accessor",
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/dependency_edge/provenance/source_accessor",
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/dependency_edge/provenance/source_cut_ref",
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation/source_cut_ref",
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation/doctored_cut_ref",
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/accessor",
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/consistency_check",
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/diagnostic/code",
    ] {
        require_pointer(row, pointer, context, failures);
    }
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/send_occurrence/occurrence_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/dependency_edge/from_occurrence_id",
        context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/receive_occurrence/occurrence_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/dependency_edge/to_occurrence_id",
        context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/dependency_edge/edge_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation/retained_dependency_edge_id",
        context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/send_occurrence/occurrence_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation/excluded_send_occurrence_id",
        context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/actual_m8_cut_before_doctor/receive_occurrence/occurrence_id",
        "/runtime_transition_trace/s2_cut_clone_mutation/doctored_m8_cut_prefix_mutation/retained_receive_occurrence_id",
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/accessor",
        json!("M8LocalRuntime::try_restore_local_cut"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/consistency_check",
        json!("send_receive_dependency"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/diagnostic/code",
        json!("E-CUT-001"),
        context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/restore_detector/private_integrity_marker_used",
        false,
        context,
        failures,
    );
    fail_if_true_pointer(
        row,
        "/runtime_transition_trace/s2_cut_clone_mutation/private_integrity_marker_only",
        context,
        failures,
    );
}

fn require_scn02_stale_membership_canon_binding(row: &Value, failures: &mut Vec<String>) {
    let context = "SCN02-R-N-STALE Canon stale-membership guard binding";
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/canon_refs/0/source_path",
        json!(M10_SCN02_SCENARIO_SOURCE_PATH),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/canon_refs/0/line_start",
        json!(M10_SCN02_STALE_CANON_LINE_START),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/canon_refs/0/line_end",
        json!(M10_SCN02_STALE_CANON_LINE_END),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/canon_refs/0/scn_id",
        json!("SCN-02"),
        context,
        failures,
    );
}

fn require_scn02_actor_self_target_stale_guard(row: &Value, failures: &mut Vec<String>) {
    let context = "SCN02-R-N-STALE actor-self authority and retired-target guard";
    for pointer in [
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_membership_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_capability_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_witness_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/target_existence_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retired_membership_ref",
    ] {
        require_pointer(row, pointer, context, failures);
    }
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/actor_principal",
        json!("self"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/authority_principal",
        json!("self"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/target_identity",
        json!("target"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/origin",
        json!("BrowserClient[self]"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/principal",
        json!("self"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/membership_status",
        json!("live"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/target_identity",
        json!("target"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retire_action_target",
        json!("target"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/stale_membership_trace/principal",
        json!("target"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/diagnostic/stale_subject",
        json!("target_membership"),
        context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_presence_check/derived_from_checked_owner_plan",
        true,
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_presence_check/binding_parameter",
        json!("target"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_presence_check/resolved_identity",
        json!("target"),
        context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_presence_check/optional_request_target_ref_used",
        false,
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_presence_check/registry_status",
        json!("retired"),
        context,
        failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_membership_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retired_membership_ref",
        context,
        failures,
    );
}

fn require_scn02_five_domain_no_mutation_guard(row: &Value, failures: &mut Vec<String>) {
    let context = "SCN02-R-N-STALE five semantic-domain no-mutation guard";
    let base = "/runtime_transition_trace/scn02_stale_membership_guard/five_domain_no_mutation";
    if row
        .pointer("/runtime_transition_trace/scn02_stale_membership_guard/store_no_mutation")
        .is_some()
        && row.pointer(base).is_none()
    {
        failures.push(
            "SCN02-R-N-STALE store_no_mutation alone is insufficient; stale target rejection must expose store/membership/grant/relation/config before/after hashes"
                .to_string(),
        );
    }
    require_bool_pointer(
        row,
        &format!("{base}/no_semantic_domain_mutation"),
        true,
        context,
        failures,
    );
    for hash_key in FIVE_DOMAIN_HASH_KEYS {
        let before = format!("{base}/original_before/{hash_key}");
        let after = format!("{base}/final_after/{hash_key}");
        require_json_pointer_equal(row, &before, &after, context, failures);
    }
}

fn require_bool_pointer(
    value: &Value,
    pointer: &str,
    expected: bool,
    context: &str,
    failures: &mut Vec<String>,
) {
    match require_pointer(value, pointer, context, failures) {
        Some(actual) if actual == &json!(expected) => {}
        Some(actual) => failures.push(format!(
            "{context} expected {pointer}={expected}, got {actual:#}"
        )),
        None => {}
    }
}

fn require_json_value_pointer(
    value: &Value,
    pointer: &str,
    expected: Value,
    context: &str,
    failures: &mut Vec<String>,
) {
    match require_pointer(value, pointer, context, failures) {
        Some(actual) if actual == &expected => {}
        Some(actual) => failures.push(format!(
            "{context} expected {pointer}={expected:#}, got {actual:#}"
        )),
        None => {}
    }
}

fn require_absent_or_null_pointer(
    value: &Value,
    pointer: &str,
    context: &str,
    failures: &mut Vec<String>,
) {
    if value
        .pointer(pointer)
        .is_some_and(|actual| !actual.is_null())
    {
        failures.push(format!(
            "{context} expected {pointer} to be absent/null, got {:?}",
            value.pointer(pointer)
        ));
    }
}

fn require_fail_closed_outcome_evidence(report: &Value, check: &str, failures: &mut Vec<String>) {
    let pointer = format!("/release_manifest/fail_closed_checks/{check}");
    let context = format!("release fail-closed check {check}");
    let Some(check_value) = require_pointer(report, &pointer, &context, failures) else {
        return;
    };
    if check_value.is_boolean() {
        failures.push(format!(
            "{context} must be an actual check outcome object, not a literal boolean {check_value:#}"
        ));
        return;
    }
    require_any_pointer(
        check_value,
        &[
            "/input_mutation/kind",
            "/mutation/kind",
            "/mutated_input/kind",
            "/request_mutation/kind",
        ],
        &context,
        failures,
    );
    require_any_pointer(
        check_value,
        &[
            "/observed_failure/code",
            "/observed_failure/outcome",
            "/diagnostic/code",
            "/failure/code",
        ],
        &context,
        failures,
    );
    require_any_pointer(
        check_value,
        &[
            "/terminal_outcome",
            "/verification_outcome",
            "/outcome/terminal_outcome",
            "/outcome/verification_outcome",
        ],
        &context,
        failures,
    );
    require_any_pointer(
        check_value,
        &[
            "/release_anchor_before/expected_manifest_hash",
            "/anchor_before/expected_manifest_hash",
            "/bound_release_anchor/expected_manifest_hash",
        ],
        &context,
        failures,
    );
    require_any_pointer(
        check_value,
        &[
            "/release_anchor_after/expected_manifest_hash",
            "/anchor_after/expected_manifest_hash",
            "/bound_release_anchor_after/expected_manifest_hash",
        ],
        &context,
        failures,
    );
    require_bool_pointer(check_value, "/fail_closed", true, &context, failures);
}

fn require_transition_lineage_ref(
    entry: &Value,
    expectation_id: &str,
    transition: &str,
    expected_lineage: &Value,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} {transition} M9->M8 authority lineage");
    let lineage = require_any_pointer(
        entry,
        &[
            "/m9_to_m8_authority_lineage/session_id",
            "/authority_lineage_ref",
            "/m9_to_m8_lineage/session_id",
        ],
        &context,
        failures,
    )
    .map(|(_, value)| value.clone());
    if lineage.is_some() && lineage.as_ref() != Some(expected_lineage) {
        failures.push(format!(
            "{context} must reuse row lineage {expected_lineage:#}, got {lineage:?}"
        ));
    }
}

fn require_m9_m8_lineage_decision_after_semantic_mutation(
    row: &Value,
    expectation_id: &str,
    semantic_transition: &str,
    m8_decision_transition: &str,
    failures: &mut Vec<String>,
) {
    let context = format!("{expectation_id} M9 semantic mutation -> M8 runtime decision");
    let trace = match require_pointer(row, "/runtime_transition_trace", &context, failures) {
        Some(trace) => trace,
        None => return,
    };
    for pointer in [
        "/m9_to_m8_authority_lineage/session_id",
        "/m9_to_m8_authority_lineage/m9_authority_session_id",
        "/m9_to_m8_authority_lineage/m8_runtime_session_id",
        "/m9_to_m8_authority_lineage/m9_snapshot_ref",
        "/m9_to_m8_authority_lineage/m8_authority_use_ref",
        "/m8_decisions_after_m9/0/transition",
        "/m8_decisions_after_m9/0/decision",
        "/m8_decisions_after_m9/0/authority_lineage_ref",
        "/m8_decisions_after_m9/0/runtime_session_id",
    ] {
        require_pointer(trace, pointer, &context, failures);
    }
    require_json_pointer_equal(
        trace,
        "/m9_to_m8_authority_lineage/session_id",
        "/m8_decisions_after_m9/0/authority_lineage_ref",
        &context,
        failures,
    );
    require_json_pointer_equal(
        trace,
        "/m9_to_m8_authority_lineage/m8_runtime_session_id",
        "/m8_decisions_after_m9/0/runtime_session_id",
        &context,
        failures,
    );
    if trace.pointer("/m8_decisions_after_m9/0/transition") != Some(&json!(m8_decision_transition))
    {
        failures.push(format!(
            "{context} must expose the actual post-mutation M8 transition {m8_decision_transition}, got {:?}",
            trace.pointer("/m8_decisions_after_m9/0/transition")
        ));
    }
    let lineage = trace
        .pointer("/m9_to_m8_authority_lineage/session_id")
        .cloned();
    let semantic_entry =
        require_transition_entry(row, expectation_id, semantic_transition, failures);
    if let Some(semantic_entry) = semantic_entry
        && semantic_entry.pointer("/accepted") != Some(&json!(true))
    {
        failures.push(format!(
            "{expectation_id} {semantic_transition} must be the accepted M9 semantic mutation: {semantic_entry:#}"
        ));
    }
    let decision_entry =
        require_transition_entry(row, expectation_id, m8_decision_transition, failures);
    if let Some(decision_entry) = decision_entry
        && decision_entry.pointer("/accepted") != Some(&json!(false))
    {
        failures.push(format!(
            "{expectation_id} {m8_decision_transition} must be the actual rejected M8 decision after {semantic_transition}: {decision_entry:#}"
        ));
    }
    if let Some(expected_lineage) = lineage.as_ref() {
        if let Some(semantic_entry) = semantic_entry {
            require_transition_lineage_ref(
                semantic_entry,
                expectation_id,
                semantic_transition,
                expected_lineage,
                failures,
            );
        }
        if let Some(decision_entry) = decision_entry {
            require_transition_lineage_ref(
                decision_entry,
                expectation_id,
                m8_decision_transition,
                expected_lineage,
                failures,
            );
        }
    }
}

fn require_scn08_m8_relation_lifecycle(
    report: &Value,
    expectation_id: &str,
    expected_floor: &str,
    expected_option_index: i64,
    expected_transitions: &[&str],
    failures: &mut Vec<String>,
) {
    let row = inventory_row(report, expectation_id);
    let context = format!("{expectation_id} actual M8 relation lifecycle");
    let trace = match require_pointer(row, "/runtime_transition_trace", &context, failures) {
        Some(trace) => trace,
        None => return,
    };
    for forbidden in ["/m10_fallback_trace", "/typed_fallback_cursor"] {
        require_absent_or_null_pointer(trace, forbidden, &context, failures);
    }
    for pointer in [
        "/m8_relation_state/selected_floor",
        "/m8_relation_state/selected_option_index",
        "/m8_relation_state/selected_target",
        "/m8_relation_state/relation_projection_before",
        "/m8_relation_state/relation_projection_after",
        "/m8_relation_state/derived_from_actual_m8_relation_state",
        "/m8_relation_state/derived_from_actual_m8_relation_projection",
        "/m8_option_chain",
        "/m8_relation_trace",
    ] {
        require_pointer(trace, pointer, &context, failures);
    }
    require_json_value_pointer(
        trace,
        "/m8_relation_state/selected_floor",
        json!(expected_floor),
        &context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_relation_state/selected_option_index",
        json!(expected_option_index),
        &context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_relation_state/selected_target",
        json!(scn08_expected_target(expected_floor)),
        &context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_state/derived_from_actual_m8_relation_state",
        true,
        &context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_state/derived_from_actual_m8_relation_projection",
        true,
        &context,
        failures,
    );
    require_scn08_validated_three_option_chain(trace, &context, failures);
    let Some(entries) = trace
        .pointer("/m8_relation_trace")
        .and_then(Value::as_array)
    else {
        return;
    };
    let actual_transitions = entries
        .iter()
        .filter_map(|entry| entry.get("transition").and_then(Value::as_str))
        .collect::<Vec<_>>();
    if actual_transitions != expected_transitions {
        failures.push(format!(
            "{context} must expose exact M8 relation transitions {expected_transitions:?}, got {actual_transitions:?}"
        ));
    }
    for (index, entry) in entries.iter().enumerate() {
        let entry_context = format!("{context} m8_relation_trace[{index}]");
        for pointer in [
            "/transition",
            "/selected_floor",
            "/selected_option_index",
            "/selected_target",
            "/relation_projection_before",
            "/relation_projection_after",
            "/derived_from_actual_m8_relation_state",
            "/derived_from_actual_m8_relation_projection",
        ] {
            require_pointer(entry, pointer, &entry_context, failures);
        }
        require_bool_pointer(
            entry,
            "/derived_from_actual_m8_relation_state",
            true,
            &entry_context,
            failures,
        );
        require_bool_pointer(
            entry,
            "/derived_from_actual_m8_relation_projection",
            true,
            &entry_context,
            failures,
        );
    }
}

fn scn08_expected_target(floor: &str) -> &'static str {
    match floor {
        "live" => "live_pose",
        "anchor" => "room_anchor",
        "frozen" => "default_pose",
        other => panic!("unknown SCN08 floor {other}"),
    }
}

fn require_scn08_validated_three_option_chain(
    trace: &Value,
    context: &str,
    failures: &mut Vec<String>,
) {
    for (index, floor, target, lease, capability, epoch) in [
        (
            0,
            "live",
            "live_pose",
            "lease:view_pose:live",
            "cap:relation:view_pose:live",
            "avatar_session",
        ),
        (
            1,
            "anchor",
            "room_anchor",
            "lease:view_pose:anchor",
            "cap:relation:view_pose:anchor",
            "room_epoch",
        ),
        (
            2,
            "frozen",
            "default_pose",
            "lease:view_pose:frozen",
            "cap:relation:view_pose:frozen",
            "static",
        ),
    ] {
        let option_context = format!("{context} m8_option_chain.options[{index}]");
        for (field, expected) in [
            ("floor", floor),
            ("kind", floor),
            ("target", target),
            ("target_identity", target),
            ("lease", lease),
            ("lease_identity", lease),
            ("capability", capability),
            ("capability_identity", capability),
            ("epoch", epoch),
            ("epoch_identity", epoch),
        ] {
            let pointer = format!("/m8_option_chain/options/{index}/{field}");
            require_json_value_pointer(trace, &pointer, json!(expected), &option_context, failures);
        }
        require_json_value_pointer(
            trace,
            &format!("/m8_option_chain/options/{index}/index"),
            json!(index),
            &option_context,
            failures,
        );
    }
    require_json_value_pointer(
        trace,
        "/m8_option_chain/owner",
        json!("M8"),
        context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_option_chain/validated_by_m8",
        true,
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/1/lineage_edges/0/from",
        json!("live"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/1/lineage_edges/0/to",
        json!("anchor"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/2/lineage_edges/0/from",
        json!("anchor"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/2/lineage_edges/0/to",
        json!("frozen"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/2/projection_kind",
        json!("opaque_default_pose"),
        context,
        failures,
    );
}

#[allow(
    clippy::too_many_arguments,
    reason = "test helper mirrors each SCN08 trace edge field explicitly at call sites"
)]
fn require_scn08_trace_edge(
    trace: &Value,
    index: usize,
    from_option_index: i64,
    to_option_index: i64,
    expected_floor: &str,
    expected_target: &str,
    context: &str,
    failures: &mut Vec<String>,
) {
    let entry_context = format!("{context} m8_relation_trace[{index}]");
    require_json_value_pointer(
        trace,
        &format!("/m8_relation_trace/{index}/from_option_index"),
        json!(from_option_index),
        &entry_context,
        failures,
    );
    require_json_value_pointer(
        trace,
        &format!("/m8_relation_trace/{index}/to_option_index"),
        json!(to_option_index),
        &entry_context,
        failures,
    );
    require_json_value_pointer(
        trace,
        &format!("/m8_relation_trace/{index}/selected_option_index"),
        json!(to_option_index),
        &entry_context,
        failures,
    );
    require_json_value_pointer(
        trace,
        &format!("/m8_relation_trace/{index}/selected_floor"),
        json!(expected_floor),
        &entry_context,
        failures,
    );
    require_json_value_pointer(
        trace,
        &format!("/m8_relation_trace/{index}/selected_target"),
        json!(expected_target),
        &entry_context,
        failures,
    );
}

fn require_scn08_expiry_semantics(trace: &Value, context: &str, failures: &mut Vec<String>) {
    for pointer in [
        "/m8_relation_trace/0/invalidation_reason",
        "/m8_relation_trace/0/audit_subreason",
    ] {
        require_json_value_pointer(trace, pointer, json!("lease-expired"), context, failures);
    }
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/0/lease_state_after_expiry",
        json!("expired"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/1/lease_state_after_expiry",
        json!("expired"),
        context,
        failures,
    );
    require_json_value_pointer(
        trace,
        "/m8_option_chain/options/2/lease_state_after_expiry",
        json!("current"),
        context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_state/no_dedicated_semantic_domain_occurrence_created",
        true,
        context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_state/old_live_lease_usable",
        false,
        context,
        failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_state/old_live_lease_restorable",
        false,
        context,
        failures,
    );
}

fn require_scn08_rollback_local_cut_guard(row: &Value, context: &str, failures: &mut Vec<String>) {
    for pointer in [
        "/runtime_transition_trace/m8_local_cut_restore/attempted",
        "/runtime_transition_trace/m8_local_cut_restore/result",
        "/runtime_transition_trace/m8_local_cut_restore/diagnostic",
        "/runtime_transition_trace/m8_local_cut_restore/diagnostic/source",
        "/runtime_transition_trace/m8_local_cut_restore/no_five_domain_mutation",
        "/runtime_transition_trace/m8_local_cut_restore/schedule_action_reference",
        "/runtime_transition_trace/m8_local_cut_restore/trace_range",
        "/runtime_transition_trace/m8_local_cut_restore/trace_range/start",
        "/runtime_transition_trace/m8_local_cut_restore/trace_range/end",
        "/runtime_transition_trace/m8_local_cut_restore/trace_range/covers_restore_attempt",
        "/runtime_transition_trace/m8_relation_state/selected_floor",
        "/runtime_transition_trace/m8_relation_state/selected_option_index",
        "/runtime_transition_trace/m8_relation_state/selected_target",
    ] {
        require_pointer(row, pointer, context, failures);
    }
    require_bool_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/attempted",
        true,
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/result",
        json!("rejected"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/diagnostic/code",
        json!("ExpiredLease"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/diagnostic/source",
        json!("M8LocalRuntime::try_restore_local_cut"),
        context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/no_five_domain_mutation",
        true,
        context,
        failures,
    );
    require_json_pointer_equal(
        row,
        "/schedule_action_reference",
        "/runtime_transition_trace/m8_local_cut_restore/schedule_action_reference",
        context,
        failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/m8_local_cut_restore/trace_range/covers_restore_attempt",
        true,
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_relation_state/selected_floor",
        json!("frozen"),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_relation_state/selected_option_index",
        json!(2),
        context,
        failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/m8_relation_state/selected_target",
        json!("default_pose"),
        context,
        failures,
    );
    require_absent_or_null_pointer(
        row,
        "/runtime_transition_trace/m8_relation_trace/0/transition_report_only",
        context,
        failures,
    );
}

fn require_scn08_rollback_restore_source_guard(failures: &mut Vec<String>) {
    let source_path = workspace_root().join("crates/mir-runtime/src/m10_reference_system.rs");
    let source_text = fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("can read {}: {error}", source_path.display()));
    let match_start_marker = "let mut m8_relation_trace = match events.as_slice()";
    let match_start = source_text.find(match_start_marker).unwrap_or_else(|| {
        panic!(
            "SCN08 relation lifecycle match exists in {}",
            source_path.display()
        )
    });
    let match_end = source_text[match_start..]
        .find("let relation_projection_after =")
        .map(|offset| match_start + offset)
        .unwrap_or_else(|| {
            panic!(
                "SCN08 relation lifecycle match end exists in {}",
                source_path.display()
            )
        });
    let match_source = &source_text[match_start..match_end];
    let rollback_start = match_source
        .find("[event] if event == \"rollback\"")
        .expect("SCN08 relation lifecycle has a rollback event branch");
    let rollback_source = &match_source[rollback_start..];
    if !rollback_source.contains("try_restore_local_cut") {
        failures.push(format!(
            "SCN08 rollback event branch in {} must execute try_restore_local_cut in the rollback path, not reuse cached lease_expiry evidence",
            source_path.display()
        ));
    }
    if rollback_source.contains("rollback_evidence")
        && !rollback_source.contains("try_restore_local_cut")
    {
        failures.push(format!(
            "SCN08 rollback event branch in {} must not satisfy rollback from cached rollback_evidence without a restore call in the rollback branch",
            source_path.display()
        ));
    }
}

fn require_scn08_write_current_option_source_guard(failures: &mut Vec<String>) {
    let source_path = workspace_root().join("crates/mir-runtime/src/m10_reference_system.rs");
    let source_text = fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("can read {}: {error}", source_path.display()));
    let match_start_marker = "let mut m8_relation_trace = match events.as_slice()";
    let match_start = source_text.find(match_start_marker).unwrap_or_else(|| {
        panic!(
            "SCN08 relation lifecycle match exists in {}",
            source_path.display()
        )
    });
    let match_end = source_text[match_start..]
        .find("let relation_projection_after =")
        .map(|offset| match_start + offset)
        .unwrap_or_else(|| {
            panic!(
                "SCN08 relation lifecycle match end exists in {}",
                source_path.display()
            )
        });
    let match_source = &source_text[match_start..match_end];
    let Some(write_start) = match_source.find("[event] if event == \"write\"") else {
        failures.push(format!(
            "SCN08 write event in {} must have an explicit branch that validates the current selected option capability after expiry",
            source_path.display()
        ));
        return;
    };
    let write_tail = &match_source[write_start..];
    let branch_end = write_tail
        .find("\n                            [event] if event ==")
        .or_else(|| write_tail.find("\n                            _ =>"))
        .unwrap_or(write_tail.len());
    let write_source = &write_tail[..branch_end];
    if !write_source.contains("request_selected_option_write") {
        failures.push(format!(
            "SCN08 write event branch in {} must call M8 request_selected_option_write for actual current-option capability validation, not fall through to report-only selection evidence",
            source_path.display()
        ));
    }
    if write_source.contains("\"select_primary\"") {
        failures.push(format!(
            "SCN08 write event branch in {} must not report generic select_primary evidence for a write-after-expiry reject",
            source_path.display()
        ));
    }
}

fn require_scn08_schedule_order(failures: &mut Vec<String>) {
    let schedule = action_schedule();
    let cases = schedule
        .pointer("/cases")
        .and_then(Value::as_array)
        .expect("typed action schedule has cases");
    let position = |id: &str| {
        cases
            .iter()
            .position(|case| case.get("id").and_then(Value::as_str) == Some(id))
            .unwrap_or_else(|| panic!("typed action schedule has case {id}"))
    };
    let live = position("SCN08.live");
    let expiry = position("SCN08.lease_expiry");
    let write = position("SCN08.write");
    let rollback = position("SCN08.rollback");
    let reacquire = position("SCN08.fresh_reacquire");
    if !(live < expiry && expiry < write && write < rollback && rollback < reacquire) {
        failures.push(format!(
            "SCN08 schedule order must be live -> lease_expiry -> write -> rollback -> fresh_reacquire so rollback observes Frozen/default_pose before fresh M9 reacquire; positions live={live}, expiry={expiry}, write={write}, rollback={rollback}, reacquire={reacquire}"
        ));
    }
}

fn require_stage_specific_mutation_evidence(
    report: &Value,
    fault: &str,
    validator: &str,
    specific_state_pointers: &[&str],
    failures: &mut Vec<String>,
) {
    for pointer in [
        "/mutation_application/actual_inputs/parsed/before_identity",
        "/mutation_application/actual_inputs/parsed/after_identity",
        "/mutation_application/actual_inputs/checked/before_identity",
        "/mutation_application/actual_inputs/checked/after_identity",
        "/mutation_application/actual_inputs/runtime/before_identity",
        "/mutation_application/actual_inputs/runtime/after_identity",
        "/validation/validator_results",
        "/runtime/no_mutation_boundary/stage",
        "/runtime/no_mutation_boundary/transition_attempted",
        "/runtime/no_mutation_boundary/changed_hash_keys",
        "/runtime/no_mutation_boundary/mutation_count_delta",
    ] {
        require_pointer(report, pointer, fault, failures);
    }

    let result_base = format!("/validation/validator_results/{validator}");
    for suffix in [
        "result",
        "diagnostic_code",
        "input_stage",
        "source_span",
        "state_before",
        "state_after",
    ] {
        let pointer = format!("{result_base}/{suffix}");
        require_pointer(report, &pointer, fault, failures);
    }
    if report.pointer(&format!("{result_base}/result")) != Some(&json!("rejected")) {
        failures.push(format!(
            "{fault}: {validator} must expose an explicit rejected validator result, not only a mapped validator label"
        ));
    }

    for pointer in specific_state_pointers {
        require_pointer(report, pointer, fault, failures);
    }

    if report.pointer("/runtime/no_mutation_boundary/changed_hash_keys") != Some(&json!([])) {
        failures.push(format!(
            "{fault}: exact no-mutation boundary must report no changed semantic hashes; got {:?}",
            report.pointer("/runtime/no_mutation_boundary/changed_hash_keys")
        ));
    }
    if report.pointer("/runtime/no_mutation_boundary/mutation_count_delta") != Some(&json!(0)) {
        failures.push(format!(
            "{fault}: exact no-mutation boundary must report mutation_count_delta=0"
        ));
    }
    for key in FIVE_DOMAIN_HASH_KEYS {
        let before_pointer = format!("/runtime/no_mutation_boundary/before_snapshot/{key}");
        let after_pointer = format!("/runtime/no_mutation_boundary/after_snapshot/{key}");
        let before = require_pointer(report, &before_pointer, fault, failures).cloned();
        let after = require_pointer(report, &after_pointer, fault, failures).cloned();
        if before.is_some() && after.is_some() && before != after {
            failures.push(format!(
                "{fault}: rejected validator must preserve {key} across the no-mutation boundary; before={before:?}, after={after:?}"
            ));
        }
    }

    if report
        .pointer("/mutation_application/mutated_clone/payload/text")
        .and_then(Value::as_str)
        .is_some_and(|text| text.contains("M10 attempted"))
    {
        failures.push(format!(
            "{fault}: mutated payload is still a comment-appended synthetic clone, not parsed/checked/runtime input evidence"
        ));
    }
    let trace_components = report
        .pointer("/validation/actual_validator_trace")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("component").and_then(Value::as_str))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if trace_components == ["clone_validator_input", validator] {
        failures.push(format!(
            "{fault}: validator trace is only clone_validator_input plus mapped {validator} label"
        ));
    }
}

fn changed_hash_keys(entry: &Value, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter_map(|key| {
            let before = entry.pointer(&format!("/before/{key}"));
            let after = entry.pointer(&format!("/after/{key}"));
            (before != after).then_some((*key).to_string())
        })
        .collect()
}

fn assert_changed_hash_keys(entry: &Value, expectation_id: &str, expected: &[&str]) {
    let domain_keys = [
        "store_hash",
        "membership_hash",
        "grant_hash",
        "relation_hash",
        "config_hash",
    ];
    let actual = changed_hash_keys(entry, &domain_keys);
    let expected = expected
        .iter()
        .map(|key| (*key).to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        actual, expected,
        "{expectation_id} must change only domain-native projection hashes {expected:?}; entry={entry:#}"
    );
}

fn falsifier_report(fault: &str) -> Value {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report: M10ConformanceReport = system
        .run_conformance(
            request_with_predicates(predicate_profile())
                .typed_input_mutation(typed_falsifier(fault)),
        )
        .expect("falsifier returns a typed conformance failure report");
    serde_json::to_value(report).expect("falsifier report serializes")
}

fn source_row<'a>(value: &'a Value, path: &str) -> &'a Value {
    value
        .pointer("/sources")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("path").and_then(Value::as_str) == Some(path))
        })
        .unwrap_or_else(|| panic!("report contains source row {path}: {value:#}"))
}

fn typed_carrier_row<'a>(value: &'a Value, group: &str, id: &str) -> &'a Value {
    value
        .pointer(&format!("/carriers/{group}"))
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.get("id").and_then(Value::as_str) == Some(id))
        })
        .unwrap_or_else(|| panic!("report contains {group} carrier row {id}: {value:#}"))
}

fn inventory_row<'a>(value: &'a Value, expectation_id: &str) -> &'a Value {
    value
        .pointer("/verification/inventory/correspondence_rows")
        .or_else(|| value.pointer("/verification/inventory/rows"))
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter().find(|row| {
                row.get("expectation_id").and_then(Value::as_str) == Some(expectation_id)
            })
        })
        .unwrap_or_else(|| panic!("report contains inventory row {expectation_id}: {value:#}"))
}

fn walk_json_objects(value: &Value, visit: &mut impl FnMut(&Map<String, Value>)) {
    match value {
        Value::Object(map) => {
            visit(map);
            for value in map.values() {
                walk_json_objects(value, visit);
            }
        }
        Value::Array(values) => {
            for value in values {
                walk_json_objects(value, visit);
            }
        }
        _ => {}
    }
}

fn assert_source_rejects(
    relative: &str,
    expected_kind: M7DiagnosticKind,
    expected_code: &str,
    expected_missing_failure: &str,
    lexeme: &str,
) {
    let source = source_text(relative);
    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(relative.to_string(), source.clone()))
            .unwrap_err();
    let primary = diagnostics.primary();
    assert_eq!(primary.kind(), expected_kind, "{relative}");
    assert_eq!(primary.canonical_code(), expected_code, "{relative}");
    let missing_failure = primary
        .generated_failure_reason()
        .expect("generated-failure diagnostics retain the missing failure reason");
    assert_eq!(
        missing_failure.missing_failure(),
        expected_missing_failure,
        "{relative}"
    );
    assert!(!diagnostics.has_executable_core(), "{relative}");
    assert_eq!(primary.span().lexeme(&source), lexeme, "{relative}");
}

#[test]
fn release_corpus_is_finite_m6_mir_only_and_has_no_expected_outputs() {
    let files = corpus_files();
    let relative_files = files
        .iter()
        .map(|path| {
            path.strip_prefix(corpus_root())
                .expect("corpus file stays under corpus root")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        relative_files,
        vec![
            "scn-01/negative-missing-visibility-denied.mir",
            "scn-01/positive.mir",
            "scn-02/negative-missing-capability-row.mir",
            "scn-02/positive.mir",
            "scn-03/negative-write-before-verdict.mir",
            "scn-03/positive.mir",
            "scn-04/negative-hidden-repair.mir",
            "scn-04/positive.mir",
            "scn-05/negative-secret-cross-locus.mir",
            "scn-05/positive.mir",
            "scn-06/negative-route-unavailable-row.mir",
            "scn-06/positive.mir",
            "scn-06/route-candidate-accepted.mir",
            "scn-07/negative-inventory-visible.mir",
            "scn-07/positive.mir",
            "scn-08/negative-missing-fallback-anchor.mir",
            "scn-08/negative-write-after-read-lineage.mir",
            "scn-08/positive.mir",
            "scn-09/base.mir",
            "scn-09/candidate-accepted.mir",
            "scn-09/candidate-missing-capability.mir",
            "scn-09/candidate-rejected.mir",
            "scn-10/negative-stale-restore.mir",
            "scn-10/positive.mir",
            "scn-11/designated-version.mir",
            "scn-11/duplicate-consumption.mir",
            "scn-12/bird-relation.mir",
            "scn-12/fallback.mir",
            "scn-12/reacquire.mir",
            "scn-12/split-frame.mir",
        ],
        "unexpected corpus file set"
    );
    assert!(
        files
            .iter()
            .all(|path| path.extension().and_then(|ext| ext.to_str()) == Some("mir")),
        "M10 corpus must contain ordinary .mir only: {files:?}"
    );
    assert!(
        files
            .iter()
            .all(|path| !path.to_string_lossy().contains("expected")),
        "expected-output sidecars are not conformance input: {files:?}"
    );

    for path in files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("can read {}: {error}", path.display()));
        parse_surface_v0(FixtureSource::new(path.to_string_lossy(), source)).unwrap_or_else(
            |diagnostics| panic!("{} parses as finite M6: {diagnostics:?}", path.display()),
        );
    }
}

#[test]
fn committed_typed_profile_inputs_match_runtime_builders() {
    assert_eq!(
        profile_input_json("action-context.schedule.json"),
        action_schedule(),
        "checked-in action schedule must match the Rust conformance builder"
    );
    assert_eq!(
        profile_input_json("typed-carriers.json"),
        typed_carriers(),
        "checked-in typed carriers must match the Rust conformance builder and source hashes"
    );
    assert_eq!(
        profile_input_json("correspondence-predicates.json"),
        predicate_profile(),
        "checked-in predicate profile must match the exact 73-row Rust builder"
    );
}

#[test]
fn action_schedule_has_no_expectation_predicate_result_or_patch_intent_fields() {
    let schedule = action_schedule();
    walk_json_objects(&schedule, &mut |map| {
        for key in map.keys() {
            let normalized = key.to_ascii_lowercase();
            assert!(
                !normalized.contains("expect")
                    && !normalized.contains("predicate")
                    && !normalized.contains("verdict")
                    && !normalized.contains("outcome")
                    && normalized != "result"
                    && !normalized.ends_with("_result")
                    && normalized != "base_source"
                    && normalized != "candidate_source",
                "action schedule must be request/context only; found key {key:?} in {schedule:#}"
            );
        }
    });

    let cases = schedule
        .pointer("/cases")
        .and_then(Value::as_array)
        .expect("schedule has case rows");
    for case in cases.iter().filter(|case| {
        case.get("scn")
            .and_then(Value::as_str)
            .is_some_and(|scn| scn.starts_with("SCN0") || scn.starts_with("SCN10"))
    }) {
        let operation_text =
            serde_json::to_string(case.get("operation").expect("case has typed operation"))
                .expect("operation serializes");
        for forbidden in ["hidden_repair", "fallback"] {
            assert!(
                !operation_text.contains(forbidden),
                "SCN-01..10 schedule operations may describe requests/context only, not hidden repair or fallback result terms: {operation_text}"
            );
        }
    }

    let schedule_text = serde_json::to_string(&schedule).expect("schedule serializes");
    for forbidden in [
        "explicit_RouteUnavailable",
        "same_source_succeeds",
        "fallback_carrier_ref",
    ] {
        assert!(
            !schedule_text.contains(forbidden),
            "typed schedule must not embed generator result term {forbidden}: {schedule_text}"
        );
    }
    assert!(!schedule_text.contains("candidate-accepted.mir"));
    assert!(!schedule_text.contains("candidate-rejected.mir"));
    assert!(!schedule_text.contains("candidate-missing-capability.mir"));
}

#[test]
fn predicate_profile_has_row_shaped_correspondence_not_input_pass_lists() {
    let profile = predicate_profile();
    assert_absent_or_null(&profile, "/c_static");
    assert_absent_or_null(&profile, "/c_runtime");

    let rows = correspondence_rows(&profile);
    assert_eq!(
        rows.len(),
        73,
        "M10 frozen SCN-01..10 inventory must be the exact closed 73-row set"
    );
    assert_eq!(
        frozen_row_ids(&profile),
        FROZEN_EXPECTATION_ROWS
            .iter()
            .map(|row| row.expectation_id.to_string())
            .collect::<Vec<_>>()
    );
    for expected in FROZEN_EXPECTATION_ROWS {
        let row = correspondence_row(&profile, expected.expectation_id);
        assert_eq!(
            row.get("scn_id").and_then(Value::as_str),
            Some(expected.scn_id),
            "{} must bind to the exact frozen SCN",
            expected.expectation_id
        );
        let phase = row
            .get("phase")
            .and_then(Value::as_str)
            .expect("correspondence row names its phase");
        assert!(
            matches!(phase, "static" | "runtime"),
            "{} must use the normative row phase vocabulary: {row:#}",
            expected.expectation_id
        );
        assert_eq!(phase, expected.phase, "{}", expected.expectation_id);
        let expected_artifact_identity = expected.artifact.identity();
        assert_eq!(
            row.get("carrier_kind").and_then(Value::as_str),
            Some(expected.artifact.carrier_kind()),
            "{} must bind to the exact carrier kind",
            expected.expectation_id
        );
        assert_eq!(
            row.get("artifact_identity").and_then(Value::as_str),
            Some(expected_artifact_identity.as_str()),
            "{} must bind to the exact source/carrier/schedule identity",
            expected.expectation_id
        );
        assert_eq!(
            row.get("diagnostic_location").and_then(Value::as_str),
            Some(expected.diagnostic_location),
            "{} must retain the exact diagnostic/provenance location",
            expected.expectation_id
        );
        assert_eq!(
            row.get("source_derived_reference").and_then(Value::as_str),
            expected.source_derived_reference,
            "{} must retain the exact source-derived reference",
            expected.expectation_id
        );
        assert_eq!(
            row.get("schedule_action_reference").and_then(Value::as_str),
            expected.schedule_action_reference,
            "{} must retain the exact schedule-action reference identity",
            expected.expectation_id
        );
        assert!(
            row.get("evidence_predicate")
                .and_then(Value::as_str)
                .is_some_and(|predicate| predicate == expected.evidence_predicate),
            "{} must carry the exact evidence predicate: {row:#}",
            expected.expectation_id
        );
    }
    assert_eq!(
        correspondence_row(&profile, "SCN05-S-N-MISSING-VISROW").pointer("/evidence_predicate"),
        Some(&json!(
            "diagnostic.E-ROW-002.missing_required_failure.VisibilityDenied.retains_source_span.player_a.secret_key"
        ))
    );
    assert_eq!(
        correspondence_row(&profile, "SCN08-S-N-LINEAGE").pointer("/evidence_predicate"),
        Some(&json!(
            "diagnostic.E-DECL-001.missing_typed_lineage_edge.live_to_anchor"
        ))
    );
    assert_eq!(
        correspondence_row(&profile, "SCN08-S-N-CAPFLOOR").pointer("/evidence_predicate"),
        Some(&json!(
            "diagnostic.E-LIN-003.read_to_write_strengthening_without_reacquire"
        ))
    );
    assert_eq!(
        correspondence_row(&profile, "SCN09-S-N-SELFGRANT").pointer("/evidence_predicate"),
        Some(&json!(
            "diagnostic.E-PATCH-003.self_grant_candidate_check.no_activation"
        ))
    );
    assert_eq!(
        correspondence_row(&profile, "SCN09-S-N-MISSINGCAP").pointer("/evidence_predicate"),
        Some(&json!(
            "diagnostic.E-PATCH-002.missing_patch_capability.no_activation"
        ))
    );
    let profile_text = serde_json::to_string(&profile).expect("profile serializes");
    assert!(!profile_text.contains("N/A"));
    assert!(!profile_text.to_ascii_lowercase().contains("waiver"));
    assert!(!profile_text.contains("profile_context:"));
    assert!(!profile_text.contains("schedule:"));
}

#[test]
fn committed_profile_artifact_identities_recompute_from_committed_carriers_and_schedule_cases() {
    let carriers = profile_input_json("typed-carriers.json");
    let schedule = profile_input_json("action-context.schedule.json");
    let profile = profile_input_json("correspondence-predicates.json");

    for row in correspondence_rows(&profile) {
        if let Some(reference) = row.get("schedule_action_reference").and_then(Value::as_str) {
            assert!(
                !reference.starts_with("schedule:"),
                "{} must not keep a free schedule label reference",
                row.get("expectation_id")
                    .and_then(Value::as_str)
                    .unwrap_or("?")
            );
            let action = action_id_from_schedule_identity(reference);
            assert_eq!(
                reference,
                schedule_action_identity_from(&schedule, action),
                "{} schedule reference must resolve to the exact committed case hash",
                row.get("expectation_id")
                    .and_then(Value::as_str)
                    .unwrap_or("?")
            );
        }
        if row.get("carrier_kind").and_then(Value::as_str) == Some("schedule_action") {
            let artifact = row
                .get("artifact_identity")
                .and_then(Value::as_str)
                .expect("schedule row has an artifact identity");
            let action = action_id_from_schedule_identity(artifact);
            assert_eq!(
                artifact,
                schedule_action_identity_from(&schedule, action),
                "{} schedule artifact must resolve to the exact committed case hash",
                row.get("expectation_id")
                    .and_then(Value::as_str)
                    .unwrap_or("?")
            );
        }
    }

    for expected in FROZEN_EXPECTATION_ROWS {
        let row = correspondence_row(&profile, expected.expectation_id);
        let recomputed_artifact_identity = match expected.artifact {
            ArtifactBinding::Source(source) => source_artifact_identity(source),
            ArtifactBinding::TypedCarrier(carrier) => {
                typed_carrier_identity_from(&carriers, carrier)
            }
            ArtifactBinding::PatchPair {
                carrier,
                base,
                candidate,
            } => patch_pair_identity_from(&carriers, carrier, base, candidate),
            ArtifactBinding::Schedule(action) => schedule_action_identity_from(&schedule, action),
        };
        assert_eq!(
            row.get("artifact_identity").and_then(Value::as_str),
            Some(recomputed_artifact_identity.as_str()),
            "{} artifact identity must be recomputed from committed source/carrier/schedule inputs",
            expected.expectation_id
        );

        let recomputed_schedule_reference = expected.schedule_action_reference.map(|reference| {
            let action = action_id_from_schedule_identity(reference);
            schedule_action_identity_from(&schedule, action)
        });
        assert_eq!(
            row.get("schedule_action_reference").and_then(Value::as_str),
            recomputed_schedule_reference.as_deref(),
            "{} schedule reference must resolve to a committed schedule case hash",
            expected.expectation_id
        );
    }
}

#[test]
fn correspondence_profile_rejects_c_level_phase_names() {
    for phase in ["C-static", "C-runtime", "STATIC", "runtime "] {
        let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
        let error = system
            .run_conformance(request_with_predicates(predicate_profile_with_phase(
                "SCN01-S-P-REQ",
                phase,
            )))
            .expect_err("M10 conformance must reject non-canonical row phases");
        assert!(
            error.contains("unsupported correspondence phase"),
            "unexpected phase rejection error for {phase:?}: {error}"
        );
    }
}

#[test]
fn selected_corpus_negatives_have_real_source_diagnostics_or_real_carrier_shapes() {
    assert_source_rejects(
        "scn-01/negative-missing-visibility-denied.mir",
        M7DiagnosticKind::GeneratedFailureNotDeclared,
        "E-ROW-002",
        "VisibilityDenied",
        "player[self].position = player[self].position + draw",
    );
    assert_source_rejects(
        "scn-02/negative-missing-capability-row.mir",
        M7DiagnosticKind::GeneratedFailureNotDeclared,
        "E-ROW-001",
        "MissingCapability",
        "fails (StaleMembership, MissingWitness, RouteUnavailable)",
    );
    assert_source_rejects(
        "scn-06/negative-route-unavailable-row.mir",
        M7DiagnosticKind::GeneratedFailureNotDeclared,
        "E-ROW-001",
        "RouteUnavailable",
        "fails (MissingCapability, MissingWitness, StaleMembership)",
    );
    check_and_elaborate_surface_v0(FixtureSource::new(
        "scn-08/negative-missing-fallback-anchor.mir",
        source_text("scn-08/negative-missing-fallback-anchor.mir"),
    ))
    .expect("SCN08 missing-lineage negative must reach the typed carrier validator");

    let scn05_source = source_text("scn-05/negative-secret-cross-locus.mir");
    let scn05 = parse_surface_v0(FixtureSource::new(
        "scn-05/negative-secret-cross-locus.mir",
        scn05_source.clone(),
    ))
    .expect("SCN05 negative remains ordinary finite M6 source");
    let secret = scn05
        .state("player_a")
        .and_then(|state| state.field("secret_key"))
        .expect("SCN05 negative source declares the private source field");
    assert_eq!(
        secret.visibility().map(|visibility| visibility.channel()),
        None
    );
    check_and_elaborate_surface_v0(FixtureSource::new(
        "scn-05/negative-secret-cross-locus.mir",
        scn05_source,
    ))
    .expect("SCN05 privacy row must reach the typed carrier validator, not M6 precedence");

    for relative in [
        "scn-06/route-candidate-accepted.mir",
        "scn-09/candidate-rejected.mir",
        "scn-09/candidate-missing-capability.mir",
        "scn-07/negative-inventory-visible.mir",
        "scn-08/negative-write-after-read-lineage.mir",
    ] {
        check_and_elaborate_surface_v0(FixtureSource::new(relative, source_text(relative)))
            .expect("carrier-negative source checks; typed carrier must cause terminal rejection");
    }
}

#[test]
fn conformance_derives_static_and_runtime_pass_outputs_from_frozen_correspondence_rows() {
    let first = run_conformance();
    let second = run_conformance();
    let profile = predicate_profile();
    let frozen_static_rows = frozen_row_ids_for_phase(&profile, "static");
    let frozen_runtime_rows = frozen_row_ids_for_phase(&profile, "runtime");
    let all_frozen_rows = frozen_row_ids(&profile);

    assert_pointer(&first, "/level", json!("C-runtime"));
    assert_pointer(
        &first,
        "/inputs/setup_kind",
        json!("typed_conformance_input"),
    );
    assert_pointer(&first, "/inputs/setup_source_path", Value::Null);
    assert_pointer(
        &first,
        "/inputs/expected_output_sidecars_loaded",
        json!(false),
    );
    assert_pointer(&first, "/generator/expected_outputs_read", json!(false));
    assert_pointer(
        &first,
        "/generator/expected_outputs_generated",
        json!(false),
    );
    assert_pointer(
        &first,
        "/generator/fixture_name_result_lookup_used",
        json!(false),
    );
    assert_pointer(
        &first,
        "/generator/evidence_generated_before_predicate_profile",
        json!(true),
    );
    assert_pointer(
        &first,
        "/verification/compared_against_predicates",
        json!(true),
    );
    assert_pointer(&first, "/verification/inventory/complete", json!(true));
    assert_pointer(&first, "/verification/inventory/missing_rows", json!([]));
    assert_pointer(&first, "/verification/inventory/waiver_rows", json!([]));
    assert_pointer(
        &first,
        "/verification/inventory/frozen_row_ids",
        json!(all_frozen_rows),
    );
    for expected_row in correspondence_rows(&profile) {
        let expectation_id = expected_row
            .get("expectation_id")
            .and_then(Value::as_str)
            .expect("profile row has an expectation id");
        let actual_row = inventory_row(&first, expectation_id);
        for key in [
            "scn_id",
            "expectation_id",
            "phase",
            "carrier_kind",
            "artifact_identity",
            "diagnostic_location",
            "source_derived_reference",
            "schedule_action_reference",
            "evidence_predicate",
        ] {
            assert_eq!(
                actual_row.get(key),
                expected_row.get(key),
                "{expectation_id} verifier inventory must preserve profile field {key}"
            );
        }
        assert_eq!(
            actual_row.get("result").and_then(Value::as_str),
            Some("pass"),
            "{expectation_id} canonical profile row must pass"
        );
        assert!(
            actual_row
                .get("evidence_refs")
                .is_some_and(Value::is_object),
            "{expectation_id} canonical profile row must expose concrete evidence refs: {actual_row:#}"
        );
        assert!(
            actual_row
                .get("actual_evidence")
                .is_some_and(Value::is_object),
            "{expectation_id} canonical profile row must expose actual evidence: {actual_row:#}"
        );
    }
    assert!(
        first
            .pointer("/verification/inventory/source_digest")
            .and_then(Value::as_str)
            .is_some_and(|digest| !digest.is_empty()),
        "verifier must cite the frozen correspondence inventory digest before deriving pass output: {first:#}"
    );
    assert_pointer(
        &first,
        "/c_static/correspondence_row_pass",
        json!(frozen_static_rows),
    );
    assert_pointer(
        &first,
        "/c_runtime/correspondence_row_pass",
        json!(frozen_runtime_rows),
    );
    assert_pointer(
        &first,
        "/c_static/pass_count",
        json!(frozen_row_ids_for_phase(&profile, "static").len()),
    );
    assert_pointer(
        &first,
        "/c_runtime/pass_count",
        json!(frozen_row_ids_for_phase(&profile, "runtime").len()),
    );
    assert_pointer(&first, "/scn_fail", json!([]));
    assert_pointer(&first, "/waiver_carrier", Value::Null);
    assert_eq!(
        first.pointer("/profile_hash"),
        second.pointer("/profile_hash"),
        "fresh runs must produce a deterministic profile hash"
    );

    assert_absent_or_null(&first, "/c_static/scn_pass_input_shortcut_used");
    assert_absent_or_null(&first, "/c_runtime/scn_pass_input_shortcut_used");
}

#[test]
fn changing_predicates_does_not_change_generated_evidence_hash_but_flips_verifier() {
    let valid = run_conformance_with(predicate_profile());
    let invalid = run_conformance_with(flipped_predicate_profile());

    assert_eq!(
        valid.pointer("/generator/evidence_hash"),
        invalid.pointer("/generator/evidence_hash"),
        "predicate profile must not feed evidence generation"
    );
    assert_pointer(&valid, "/verification/terminal_outcome", json!("Accepted"));
    assert_pointer(
        &invalid,
        "/verification/terminal_outcome",
        json!("PredicateMismatch"),
    );
    assert_pointer(
        &invalid,
        "/verification/mismatches/0/predicate_id",
        json!("SCN09-S-N-SELFGRANT"),
    );
    assert_pointer(
        &invalid,
        "/generator/predicate_profile_read_before_evidence_generation",
        json!(false),
    );
}

#[test]
fn missing_predicate_row_does_not_change_evidence_but_prevents_pass_output() {
    let valid = run_conformance_with(predicate_profile());
    let missing = run_conformance_with(predicate_profile_missing_row());

    assert_eq!(
        valid.pointer("/generator/evidence_hash"),
        missing.pointer("/generator/evidence_hash"),
        "removing a predicate row must not alter generated evidence"
    );
    assert_pointer(
        &missing,
        "/verification/terminal_outcome",
        json!("MissingCorrespondenceRow"),
    );
    assert_pointer(
        &missing,
        "/verification/missing_rows/0",
        json!("SCN06-S-N-ROW"),
    );
    assert_pointer(&missing, "/waiver_carrier", Value::Null);
    assert_absent_or_null(&missing, "/c_static/pass_count");
    assert_absent_or_null(&missing, "/c_runtime/pass_count");
}

#[test]
fn source_negative_terminal_rows_have_no_checked_or_core_success_identity() {
    let report = run_conformance();
    for (path, code) in [
        ("scn-01/negative-missing-visibility-denied.mir", "E-ROW-002"),
        ("scn-02/negative-missing-capability-row.mir", "E-ROW-001"),
        ("scn-06/negative-route-unavailable-row.mir", "E-ROW-001"),
    ] {
        let row = source_row(&report, path);
        assert_eq!(
            row.pointer("/terminal/source_identity"),
            row.pointer("/source_identity"),
            "terminal identity must still be source-bound for rejected source {path}"
        );
        assert_eq!(row.pointer("/terminal/diagnostic/code"), Some(&json!(code)));
        assert_eq!(
            row.pointer("/terminal/diagnostic/source_path"),
            Some(&json!(path))
        );
        assert_absent_or_null(row, "/checked/source_identity");
        assert_absent_or_null(row, "/checked/core_identity");
        assert_absent_or_null(row, "/core_identity");
        assert_eq!(row.pointer("/attached_positive_core"), Some(&json!(false)));
    }
}

#[test]
fn carrier_negative_rows_keep_source_check_identity_but_reject_on_separate_carrier_identity() {
    let report = run_conformance();
    for (group, path, carrier, diagnostic, terminal_outcome) in [
        (
            "observation_policy",
            "scn-05/negative-secret-cross-locus.mir",
            "portal-secret-missing-required-failure",
            "E-ROW-002",
            "CarrierRejectedBeforeRuntime",
        ),
        (
            "observation_policy",
            "scn-05/negative-secret-cross-locus.mir",
            "portal-secret-redaction-policy",
            "VisibilityDenied",
            "RuntimeRejectedBeforeMutation",
        ),
        (
            "observation_policy",
            "scn-07/negative-inventory-visible.mir",
            "inventory-note-private-policy",
            "E-VIS-002",
            "CarrierRejectedBeforeRuntime",
        ),
        (
            "fallback",
            "scn-08/negative-missing-fallback-anchor.mir",
            "view-pose-missing-lineage",
            "E-DECL-001",
            "CarrierRejectedBeforeRuntime",
        ),
        (
            "fallback",
            "scn-08/negative-write-after-read-lineage.mir",
            "view-pose-write-after-read",
            "E-LIN-003",
            "CarrierRejectedBeforeRuntime",
        ),
    ] {
        let source = source_row(&report, path);
        assert_eq!(
            source.pointer("/checked/source_identity"),
            source.pointer("/source_identity")
        );
        assert!(source.pointer("/checked/core_identity").is_some());
        let row = typed_carrier_row(&report, group, carrier);
        assert_eq!(
            row.pointer("/source_identity"),
            Some(&json!(source_artifact_identity(path))),
            "{carrier} must retain the ordinary source identity for {path}"
        );
        assert_eq!(
            row.pointer("/terminal/outcome"),
            Some(&json!(terminal_outcome))
        );
        assert_eq!(
            row.pointer("/terminal/carrier_identity"),
            row.pointer("/carrier_identity")
        );
        assert_ne!(
            row.pointer("/terminal/carrier_identity"),
            Some(&json!(source_artifact_identity(path))),
            "carrier-negative terminal identity must not collapse into source identity for {path}"
        );
        assert_eq!(
            row.pointer("/terminal/diagnostic/code"),
            Some(&json!(diagnostic))
        );
        if terminal_outcome == "CarrierRejectedBeforeRuntime" {
            assert_eq!(row.pointer("/runtime/admitted"), Some(&json!(false)));
        } else {
            assert_eq!(row.pointer("/runtime/mutation_count"), Some(&json!(0)));
        }
        assert_absent_or_null(row, "/runtime/composite_success");
    }
}

#[test]
fn p0_scn08_normal_fallback_carrier_must_bind_exact_checked_m7_relation_core() {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "scn-08/positive.mir",
        source_text("scn-08/positive.mir"),
    ))
    .expect("SCN08 positive source checks through M7 Core");
    let relation = checked
        .relation("view_pose")
        .and_then(|evaluation| evaluation.relation_core())
        .expect("SCN08 positive source exposes checked M7 relation core");
    let relation_name = "view_pose";
    let primary_target = relation.primary().anchor();
    let primary_epoch = relation.primary().epoch();
    let fallback_target = relation.fallback().anchor();
    let fallback_epoch = relation.fallback().epoch();

    assert_eq!(primary_target, "live_pose");
    assert_eq!(primary_epoch, "avatar_session");
    assert_eq!(fallback_target, "room_anchor");
    assert_eq!(fallback_epoch, "room_epoch");

    let carriers = typed_carriers();
    let (_, carrier) = carrier_value_in(&carriers, "view-pose-normal-fallback");
    assert_pointer(carrier, "/relation", json!(relation_name));
    assert_pointer(carrier, "/source", json!("scn-08/positive.mir"));
    assert_pointer(carrier, "/options/0/target", json!(primary_target));
    assert_pointer(carrier, "/options/0/epoch", json!(primary_epoch));
    assert_pointer(carrier, "/options/1/target", json!(fallback_target));
    assert_pointer(carrier, "/options/1/epoch", json!(fallback_epoch));
    assert_pointer(carrier, "/options/2/target", json!("default_pose"));
    assert_pointer(carrier, "/options/2/epoch", json!("static"));

    let mut failures = Vec::new();
    match run_conformance_result() {
        Ok(report) => {
            let context = "SCN08 normal fallback source-to-carrier binding";
            for (pointer, expected) in [
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_relation_name",
                    relation_name,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_primary_target",
                    primary_target,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_primary_epoch",
                    primary_epoch,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_fallback_target",
                    fallback_target,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_fallback_epoch",
                    fallback_epoch,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/relation_name",
                    relation_name,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/primary_target",
                    primary_target,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/primary_epoch",
                    primary_epoch,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/fallback_target",
                    fallback_target,
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/fallback_epoch",
                    fallback_epoch,
                ),
            ] {
                require_json_value_pointer(&report, pointer, json!(expected), context, &mut failures);
            }
            for (left, right) in [
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_relation_name",
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/relation_name",
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_primary_target",
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/primary_target",
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_primary_epoch",
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/primary_epoch",
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_fallback_target",
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/fallback_target",
                ),
                (
                    "/derivation/SCN-08/normal_fallback_carrier/source_binding/m7_fallback_epoch",
                    "/derivation/SCN-08/normal_fallback_carrier/carrier_binding/fallback_epoch",
                ),
            ] {
                require_json_pointer_equal(&report, left, right, context, &mut failures);
            }
            require_bool_pointer(
                &report,
                "/derivation/SCN-08/normal_fallback_carrier/source_binding/checked_by_m7_core",
                true,
                context,
                &mut failures,
            );
            require_bool_pointer(
                &report,
                "/derivation/SCN-08/normal_fallback_carrier/source_binding/parallel_carrier_created",
                false,
                context,
                &mut failures,
            );
        }
        Err(error) => failures.push(format!(
            "M10 conformance report must be generated before SCN08 source-carrier binding can be checked: {error}"
        )),
    }

    assert!(
        failures.is_empty(),
        "SCN08 normal fallback carrier must be source-bound to the checked M7 relation core and must not admit a parallel carrier with mismatched targets/epochs:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn08_normal_carrier_exactness_mutations_reject_before_m8_chain_admission() {
    let mut failures = Vec::new();
    let validator = "scn08_finite_fallback_carrier_validator";
    for (fault, mutated_pointer, mutated_value, violation) in [
        (
            "scn08_normal_carrier_missing_anchor_to_frozen",
            "/mutation_application/mutated_clone/payload/input/fallback_carriers/0/options/2/lineage_edges",
            json!([]),
            "missing_anchor_to_frozen",
        ),
        (
            "scn08_normal_carrier_mutated_default_pose",
            "/mutation_application/mutated_clone/payload/input/fallback_carriers/0/options/2/target",
            json!("mutated_default_pose"),
            "frozen_target_not_default_pose",
        ),
        (
            "scn08_normal_carrier_mutated_static_epoch",
            "/mutation_application/mutated_clone/payload/input/fallback_carriers/0/options/2/epoch",
            json!("mutated_static"),
            "frozen_epoch_not_static",
        ),
    ] {
        let report = falsifier_report(fault);
        let context = format!("{fault} SCN08 finite carrier exactness");
        let mut specific_state_pointers = vec![
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_id",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core/relation_name",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core/primary_target",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core/primary_epoch",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core/fallback_target",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core/fallback_epoch",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/target",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/epoch",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admission_attempted",
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admitted",
        ];
        if fault == "scn08_normal_carrier_missing_anchor_to_frozen" {
            specific_state_pointers.push(
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/lineage_edges",
            );
        } else {
            specific_state_pointers.extend([
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/lineage_edges/0/from",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/lineage_edges/0/to",
            ]);
        }
        require_json_value_pointer(
            &report,
            "/terminal_outcome",
            json!("ConformanceFailure"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/diagnostics/0/code",
            json!("SCN08FallbackCarrierExactnessViolation"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            mutated_pointer,
            mutated_value,
            &context,
            &mut failures,
        );
        require_stage_specific_mutation_evidence(
            &report,
            fault,
            validator,
            &specific_state_pointers,
            &mut failures,
        );
        if fault == "scn08_normal_carrier_missing_anchor_to_frozen" {
            require_json_value_pointer(
                &report,
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/2/lineage_edges",
                json!([]),
                &context,
                &mut failures,
            );
        }
        require_json_value_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/violation",
            json!(violation),
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/before_m8_chain_admission",
            true,
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admission_attempted",
            false,
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admitted",
            false,
            &context,
            &mut failures,
        );
    }
    let fault = "scn08_source_primary_target_carrier_mismatch";
    let violation = "m7_primary_target_disagrees_with_canonical_carrier";
    {
        let report = falsifier_report(fault);
        let context = format!("{fault} SCN08 source/Core to carrier exactness");
        require_json_value_pointer(
            &report,
            "/terminal_outcome",
            json!("ConformanceFailure"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/diagnostics/0/code",
            json!("SCN08FallbackCarrierExactnessViolation"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/mutation_application/mutated_clone/payload/path",
            json!("scn-08/positive.mir"),
            &context,
            &mut failures,
        );
        match report
            .pointer("/mutation_application/mutated_clone/payload/text")
            .and_then(Value::as_str)
        {
            Some(text)
                if text.contains("primary live_anchor epoch avatar_session")
                    && !text.contains("primary live_pose epoch avatar_session") => {}
            Some(text) => failures.push(format!(
                "{context} mutated source text must change the checked primary target from live_pose to live_anchor while carrier remains canonical; text={text:?}"
            )),
            None => failures.push(format!("{context} missing mutated source text payload")),
        }
        require_stage_specific_mutation_evidence(
            &report,
            fault,
            validator,
            &[
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_id",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_before/relation_name",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_before/primary_target",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_before/primary_epoch",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_after/relation_name",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_after/primary_target",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_after/primary_epoch",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/0/target",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/0/epoch",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admission_attempted",
                "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admitted",
            ],
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_before/primary_target",
            json!("live_pose"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/source_core_after/primary_target",
            json!("live_anchor"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/carrier_chain/options/0/target",
            json!("live_pose"),
            &context,
            &mut failures,
        );
        require_json_value_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/violation",
            json!(violation),
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/before_m8_chain_admission",
            true,
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admission_attempted",
            false,
            &context,
            &mut failures,
        );
        require_bool_pointer(
            &report,
            "/validation/validator_state/scn08_finite_fallback_carrier_validator/m8_chain_admitted",
            false,
            &context,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "SCN08 malformed normal carriers must be rejected by finite carrier exactness before M8 chain admission and preserve all five semantic domains:\n{}",
        failures.join("\n")
    );
}

#[test]
fn patch_intent_carriers_are_hash_bound_to_base_and_candidate_source_artifacts() {
    let report = run_conformance();
    for (carrier, candidate, terminal, diagnostic) in [
        (
            "scn09-candidate-a",
            "scn-09/candidate-accepted.mir",
            "PatchAccepted",
            Value::Null,
        ),
        (
            "scn09-candidate-b",
            "scn-09/candidate-rejected.mir",
            "PatchRejectedAtCarrierCheck",
            json!("E-PATCH-003"),
        ),
        (
            "scn09-candidate-c",
            "scn-09/candidate-missing-capability.mir",
            "PatchRejectedAtCarrierCheck",
            json!("E-PATCH-002"),
        ),
    ] {
        let carrier_row = report
            .pointer("/carriers/patch")
            .and_then(Value::as_array)
            .and_then(|rows| {
                rows.iter()
                    .find(|row| row.get("id").and_then(Value::as_str) == Some(carrier))
            })
            .unwrap_or_else(|| panic!("report contains patch carrier {carrier}: {report:#}"));
        assert_eq!(
            carrier_row.pointer("/base_source_identity"),
            Some(&json!(source_artifact_identity("scn-09/base.mir"))),
            "{carrier} base identity must be derived from checked base source"
        );
        assert_eq!(
            carrier_row.pointer("/candidate_source_identity"),
            Some(&json!(source_artifact_identity(candidate))),
            "{carrier} candidate identity must be derived from checked candidate source"
        );
        assert_eq!(
            carrier_row.pointer("/terminal/outcome"),
            Some(&json!(terminal))
        );
        assert_eq!(
            carrier_row.pointer("/terminal/diagnostic/code"),
            Some(&diagnostic)
        );
        match carrier {
            "scn09-candidate-a" => {
                assert_eq!(
                    carrier_row.pointer("/authority_intent/kind"),
                    Some(&json!("none"))
                );
                assert_eq!(
                    carrier_row.pointer("/state_additions/0/state"),
                    Some(&json!("lamp"))
                );
                assert_eq!(
                    carrier_row.pointer("/state_additions/0/fields/0"),
                    Some(&json!("enabled"))
                );
            }
            "scn09-candidate-b" => {
                assert_eq!(
                    carrier_row.pointer("/authority_intent/kind"),
                    Some(&json!("self_grant"))
                );
                assert_eq!(
                    carrier_row.pointer("/authority_intent/authority"),
                    Some(&json!("ServerAuthority"))
                );
                assert_eq!(
                    carrier_row.pointer("/authority_intent/grantee"),
                    Some(&json!("self"))
                );
            }
            "scn09-candidate-c" => {
                assert_eq!(
                    carrier_row.pointer("/required_capabilities"),
                    Some(&json!([]))
                );
                assert_eq!(
                    carrier_row.pointer("/terminal/diagnostic/location"),
                    Some(&json!("required_capabilities"))
                );
            }
            _ => unreachable!("covered patch carriers only"),
        }
        assert_eq!(
            carrier_row.pointer("/hash_binding/includes_base_source_identity"),
            Some(&json!(true))
        );
        assert_eq!(
            carrier_row.pointer("/hash_binding/includes_candidate_source_identity"),
            Some(&json!(true))
        );
        assert_eq!(
            carrier_row.pointer("/verdict_from_schedule_or_name"),
            Some(&json!(false))
        );
    }
}

#[test]
fn scn06_route_patch_uses_checked_candidate_artifact_not_schedule_label() {
    let report = run_conformance();
    let carrier_row = typed_carrier_row(&report, "route_patch", "scn06-route-patch-east-west");
    assert_eq!(
        carrier_row.pointer("/candidate_source_identity"),
        Some(&json!(source_artifact_identity(
            "scn-06/route-candidate-accepted.mir"
        )))
    );
    assert_eq!(
        carrier_row.pointer("/terminal/outcome"),
        Some(&json!("RoutePatchChecked"))
    );
    assert_eq!(
        carrier_row.pointer("/route/from_locus"),
        Some(&json!("ShardA"))
    );
    assert_eq!(
        carrier_row.pointer("/route/to_locus"),
        Some(&json!("ShardB"))
    );
    assert_eq!(
        carrier_row.pointer("/verdict_from_schedule_or_name"),
        Some(&json!(false))
    );
    assert_pointer(
        &report,
        "/derivation/SCN-06/route_patch/submitted_checked_artifact",
        json!(true),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-06/route_patch/same_source_succeeds_after_activation",
        json!(true),
    );
}

#[test]
fn scn05_scn07_scn08_and_scn09_derivations_do_not_use_filenames_or_expected_enums() {
    let report = run_conformance();
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/id",
        json!("portal-secret-redaction-policy"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/missing_required_failure_carrier/id",
        json!("portal-secret-missing-required-failure"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/missing_required_failure_carrier/diagnostics/0/code",
        json!("E-ROW-002"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/missing_required_failure_carrier/source_field_span",
        json!("player_a.secret_key"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/source_state",
        json!("player_a"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/source_field",
        json!("secret_key"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/cross_locus_observation_request/from_locus",
        json!("WorldA"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/cross_locus_observation_request/to_locus",
        json!("WorldB"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/policy_carrier/destination_locus",
        json!("WorldB"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/filename_result_lookup_used",
        json!(false),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/runtime_diagnostic/code",
        json!("VisibilityDenied"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/runtime_diagnostic/mutation_count",
        json!(0),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-05/validators/visibility_policy/invocations",
        json!(2),
    );

    assert_pointer(
        &report,
        "/derivation/SCN-07/policy_carrier/id",
        json!("inventory-note-private-policy"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-07/policy_carrier/hash_bound_to_source",
        json!(true),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-07/filename_result_lookup_used",
        json!(false),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-07/validators/observer_policy/invocations",
        json!(1),
    );

    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/id",
        json!("view-pose-write-after-read"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/relation",
        json!("view_pose"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/hash_bound_to_source",
        json!(true),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/options/0/kind",
        json!("live"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/options/1/kind",
        json!("anchor"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/options/2/kind",
        json!("frozen"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/fallback_carrier/negative_capability_floor",
        json!("write_after_read_without_fresh_reacquire"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/missing_lineage_carrier/id",
        json!("view-pose-missing-lineage"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/missing_lineage_carrier/diagnostics/0/code",
        json!("E-DECL-001"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/write_after_read_carrier/id",
        json!("view-pose-write-after-read"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/write_after_read_carrier/diagnostics/0/code",
        json!("E-LIN-003"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/filename_result_lookup_used",
        json!(false),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-08/validators/fallback_lineage/invocations",
        json!(1),
    );

    assert_pointer(
        &report,
        "/derivation/SCN-09/self_grant_candidate/source_path",
        json!("scn-09/candidate-rejected.mir"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/self_grant_candidate/m7_checked",
        json!(true),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/self_grant_candidate/diagnostics/0/code",
        json!("E-PATCH-003"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/self_grant_candidate/verdict_from_schedule_or_name",
        json!(false),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/missing_capability_candidate/source_path",
        json!("scn-09/candidate-missing-capability.mir"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/missing_capability_candidate/m7_checked",
        json!(true),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/missing_capability_candidate/diagnostics/0/code",
        json!("E-PATCH-002"),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/validators/candidate_checker/invocations",
        json!(3),
    );
    assert_pointer(
        &report,
        "/derivation/SCN-09/validators/patch_intent_compat/invocations",
        json!(2),
    );
}

#[test]
fn m8_direct_path_remains_deferred_to_m9_before_source_bound_admission() {
    let report = run_conformance();
    for path in [
        "scn-01/positive.mir",
        "scn-02/positive.mir",
        "scn-08/positive.mir",
    ] {
        let row = source_row(&report, path);
        assert_eq!(
            row.pointer("/m8/direct_residuals/0/outcome"),
            Some(&json!("DeferredToM9")),
            "M8 must not turn M9 residuals into direct runtime success for {path}"
        );
        assert_eq!(
            row.pointer("/m9/source_bound_admission/outcome"),
            Some(&json!("accepted"))
        );
    }
}

#[test]
fn pressure_rows_scn11_and_scn12_are_reported_separately_from_frozen_ten() {
    let report = run_conformance();
    let frozen_rows = frozen_row_ids(&predicate_profile());
    assert_status(&report, "/pressure/SCN-11/designated_version", "accepted");
    assert_status(
        &report,
        "/pressure/SCN-11/duplicate_consumption",
        "rejected",
    );
    assert_status(&report, "/pressure/SCN-12/bird_relation", "accepted");
    assert_status(&report, "/pressure/SCN-12/split_frame", "rejected");
    assert_status(&report, "/pressure/SCN-12/fallback", "accepted");
    assert_status(&report, "/pressure/SCN-12/reacquire", "accepted");
    assert!(
        frozen_rows
            .iter()
            .all(|row| !row.starts_with("SCN11-") && !row.starts_with("SCN12-")),
        "pressure rows are not frozen correspondence rows: {frozen_rows:?}"
    );
    assert_pointer(
        &report,
        "/verification/inventory/pressure_rows_are_frozen",
        json!(false),
    );
}

#[test]
fn pressure_rows_expose_runtime_provenance_not_literal_status_only() {
    let report = run_conformance();
    let scn11 = report
        .pointer("/pressure/SCN-11")
        .expect("SCN-11 pressure section exists");
    let designated = scn11
        .pointer("/designated_version")
        .expect("SCN-11 designated-version pressure row exists");
    assert!(
        designated.is_object(),
        "SCN-11 designated_version must expose provenance, not only a literal status: {designated:#}"
    );
    assert_has_any_pointer(
        designated,
        &[
            "/trace",
            "/runtime_trace",
            "/provenance",
            "/version",
            "/designated_value_ref",
        ],
        "SCN-11 designated consumption",
    );
    let duplicate = scn11
        .pointer("/duplicate_consumption")
        .expect("SCN-11 duplicate-consumption pressure row exists");
    assert!(
        duplicate.is_object(),
        "SCN-11 duplicate_consumption must expose rejection provenance, not only a literal status: {duplicate:#}"
    );
    assert_has_any_pointer(
        duplicate,
        &["/trace", "/runtime_trace", "/provenance", "/diagnostic"],
        "SCN-11 duplicate consumption",
    );

    let scn12 = report
        .pointer("/pressure/SCN-12")
        .expect("SCN-12 pressure section exists");
    let relation = scn12
        .pointer("/bird_relation")
        .expect("SCN-12 bird-relation pressure row exists");
    assert!(
        relation.is_object(),
        "SCN-12 bird_relation must expose relation projection evidence, not only a literal status: {relation:#}"
    );
    assert_has_any_pointer(
        relation,
        &[
            "/projection",
            "/relation",
            "/derived_pose",
            "/trace",
            "/runtime_trace",
        ],
        "SCN-12 bird relation projection",
    );
}

#[test]
fn forged_schedule_action_reference_in_correspondence_row_rejects_profile() {
    let report = run_conformance_with(predicate_profile_with_row_field(
        "SCN01-R-P-STATE",
        "schedule_action_reference",
        json!("schedule_action:SCN99.forged_nonexistent:fnv1a64:0000000000000000"),
    ));

    let outcome = report.pointer("/terminal_outcome");
    assert_eq!(
        outcome,
        Some(&json!("ConformanceFailure")),
        "forged nonexistent schedule_action_reference must reject profile; got terminal_outcome={outcome:?}"
    );
}

#[test]
fn forged_source_derived_reference_in_correspondence_row_rejects_profile() {
    let report = run_conformance_with(predicate_profile_with_row_field(
        "SCN01-S-P-REQ",
        "source_derived_reference",
        json!("source_ref:SCN99.forged_nonexistent"),
    ));

    let outcome = report.pointer("/terminal_outcome");
    assert_eq!(
        outcome,
        Some(&json!("ConformanceFailure")),
        "forged nonexistent source_derived_reference must reject profile; got terminal_outcome={outcome:?}"
    );
}

#[test]
fn scn11_pressure_exposes_actual_m8_designated_consumption_and_duplicate_delivery_trace() {
    let report = run_conformance();
    let designated = report
        .pointer("/pressure/SCN-11/designated_version")
        .expect("SCN-11 designated version pressure row exists");
    let duplicate = report
        .pointer("/pressure/SCN-11/duplicate_consumption")
        .expect("SCN-11 duplicate consumption pressure row exists");

    assert_status(&report, "/pressure/SCN-11/designated_version", "accepted");
    assert_status(
        &report,
        "/pressure/SCN-11/duplicate_consumption",
        "rejected",
    );

    let designated_missing = missing_pointers(
        designated,
        &[
            "/m8_designated_evaluation_trace",
            "/m8_consumption_trace",
            "/result_version",
        ],
    );
    let duplicate_missing = missing_pointers(
        duplicate,
        &[
            "/m8_duplicate_delivery_trace",
            "/duplicate_delivery_rejected",
            "/double_consumption_prevented",
        ],
    );
    assert!(
        designated_missing.is_empty() && duplicate_missing.is_empty(),
        "SCN-11 pressure must expose actual M8 designated evaluation, consumption, result-version, duplicate-delivery rejection, and no-double-consumption evidence; missing designated={designated_missing:?}, duplicate={duplicate_missing:?}; designated={designated:#}; duplicate={duplicate:#}"
    );
}

#[test]
fn runtime_rows_expose_transition_or_validator_trace_not_schedule_event_echo() {
    let report = run_conformance();
    let trace_pointers = [
        "/transition_trace",
        "/runtime_transition_trace",
        "/validator_trace",
        "/runtime_trace",
        "/evidence_trace",
    ];
    let mut failures = Vec::new();

    for (expectation_id, context) in [
        ("SCN04-R-P-STALE", "membership stale-request rejection"),
        ("SCN04-R-P-BLOCK-COMPACT", "membership compaction guard"),
        ("SCN10-R-P-S1", "save S1 cut creation"),
        ("SCN10-R-P-LOADFRESH", "fresh load of S1"),
        ("SCN08-R-P-LIVE", "fallback live option selection"),
        (
            "SCN08-R-N-REPROMOTE",
            "fallback same-lineage repromotion rejection",
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        if !has_structured_trace(row, &trace_pointers) {
            failures.push(format!(
                "{expectation_id} ({context}) lacks a structured transition/validator trace; row={row:#}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "runtime inventory rows must expose actual transition/validator trace evidence, not only schedule event enum echo:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn05_06_07_runtime_rows_require_receipt_backed_transitions_and_observer_publication_origin() {
    let report = run_conformance();
    let mut failures = Vec::new();

    let handoff = inventory_row(&report, "SCN05-R-P-HANDOFF");
    require_structured_runtime_provenance(handoff, "SCN05-R-P-HANDOFF", &mut failures);
    require_transition_sequence(
        handoff,
        "SCN05-R-P-HANDOFF",
        &[
            "portal.leave_verdict",
            "portal.join_verdict",
            "portal.spawn_write",
        ],
        &mut failures,
    );

    for expectation_id in [
        "SCN05-R-P-OBS",
        "SCN07-R-P-FIELDS",
        "SCN07-R-P-ADMIN",
        "SCN07-R-P-POLICY",
    ] {
        let row = inventory_row(&report, expectation_id);
        require_structured_runtime_provenance(row, expectation_id, &mut failures);
        require_observer_publication_origin_and_redaction(
            row,
            expectation_id,
            &["secret_key", "inventory_note"],
            &mut failures,
        );
    }

    let patched = inventory_row(&report, "SCN06-R-P-PATCHED");
    require_structured_runtime_provenance(patched, "SCN06-R-P-PATCHED", &mut failures);
    require_transition_sequence(
        patched,
        "SCN06-R-P-PATCHED",
        &[
            "route.reject_before_patch",
            "route.patch.activate",
            "route.owner_write_after_patch",
        ],
        &mut failures,
    );

    for (expectation_id, transition, diagnostic) in [
        (
            "SCN05-R-N-SECRET",
            "observation.visibility_denied",
            "VisibilityDenied",
        ),
        (
            "SCN05-R-N-WRONGCAP",
            "observation.wrong_capability",
            "MissingCapability",
        ),
        ("SCN06-R-P-ABSENT", "route.unavailable", "RouteUnavailable"),
        ("SCN06-R-N-NOHANG", "route.unavailable", "RouteUnavailable"),
        (
            "SCN07-R-N-HORIGIN",
            "observer.history_origin_reject",
            "E-VIS-003",
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        require_structured_runtime_provenance(row, expectation_id, &mut failures);
        require_no_mutation_reject_snapshot(
            row,
            expectation_id,
            transition,
            diagnostic,
            &mut failures,
        );
        if row.pointer("/runtime_transition_trace/no_publication") != Some(&json!(true)) {
            failures.push(format!(
                "{expectation_id} rejection must record no_publication=true"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "SCN05/06/07 runtime evidence must come from actual persistent M8/M9 receipts, transition traces, five-domain reject snapshots, and observer publication origin/redaction, not schedule or carrier names:\n{}",
        failures.join("\n")
    );
}

#[test]
fn typed_mutations_report_actual_clone_application_before_validator_evidence() {
    let before_identity_pointers = [
        "/mutation/application/before_identity",
        "/mutation_application/before_identity",
        "/falsifier/input/before_identity",
        "/validation/mutated_input/before_identity",
    ];
    let after_identity_pointers = [
        "/mutation/application/after_identity",
        "/mutation_application/after_identity",
        "/falsifier/input/after_identity",
        "/validation/mutated_input/after_identity",
    ];
    let diagnostic_detail_pointers = [
        "/span",
        "/source_ref",
        "/validator",
        "/validator_trace",
        "/input_identity",
        "/actual_identity",
        "/expected_identity",
    ];
    let mut failures = Vec::new();

    for (fault, clone_kind, validator) in [
        (
            "source_sensitivity_changed_text_same_name",
            "source",
            "source_identity_validator",
        ),
        (
            "scn09_patch_provenance_mismatch",
            "carrier",
            "patch_carrier_validator",
        ),
        (
            "schedule_action_same_id_content_hash_mismatch",
            "schedule",
            "artifact_identity_validator",
        ),
    ] {
        let value = falsifier_report(fault);

        if value.pointer("/terminal_outcome") != Some(&json!("ConformanceFailure")) {
            failures.push(format!(
                "{fault}: terminal outcome must be ConformanceFailure: {value:#}"
            ));
        }
        if value.pointer(&format!("/validation/invocations/{validator}")) != Some(&json!(1)) {
            failures.push(format!(
                "{fault}: corresponding {clone_kind} validator {validator} must run exactly once: {value:#}"
            ));
        }
        if value.pointer("/validation/real_validator_invoked") != Some(&json!(true)) {
            failures.push(format!(
                "{fault}: report must mark a real validator invocation: {value:#}"
            ));
        }

        match (
            first_existing_pointer(&value, &before_identity_pointers),
            first_existing_pointer(&value, &after_identity_pointers),
        ) {
            (Some((before_pointer, before)), Some((after_pointer, after))) => {
                if before == after {
                    failures.push(format!(
                        "{fault}: {clone_kind} mutation must show different before/after identities at {before_pointer} and {after_pointer}; value={value:#}"
                    ));
                }
            }
            (before, after) => failures.push(format!(
                "{fault}: {clone_kind} mutation must expose before/after clone identities before validation; before_present={}, after_present={}, value={value:#}",
                before.is_some(),
                after.is_some()
            )),
        }

        let store_before = value.pointer("/runtime/store_hash_before_failure");
        let store_after = value.pointer("/runtime/store_hash_after_failure");
        if store_before.is_none() || store_after.is_none() || store_before != store_after {
            failures.push(format!(
                "{fault}: no-mutation snapshot must expose equal store hashes before/after failure; before={store_before:?}, after={store_after:?}"
            ));
        }
        if value.pointer("/runtime/mutation_count_after_failure") != Some(&json!(0)) {
            failures.push(format!(
                "{fault}: no-mutation snapshot must show zero runtime mutations after failure: {value:#}"
            ));
        }

        let diagnostic = value
            .pointer("/diagnostics/0")
            .unwrap_or_else(|| panic!("{fault}: falsifier report has a primary diagnostic"));
        if !diagnostic.is_object() || !has_any_pointer(diagnostic, &diagnostic_detail_pointers) {
            failures.push(format!(
                "{fault}: primary diagnostic must be a real validator diagnostic object with source/identity/validator detail, not only a hard-coded code/source_path pair; diagnostic={diagnostic:#}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "typed mutation evidence gaps:\n{}",
        failures.join("\n")
    );
}

#[test]
fn verification_inventory_rows_expose_per_row_result_evidence_refs_and_fail_diagnostics() {
    let report = run_conformance();
    let rows = report
        .pointer("/verification/inventory/correspondence_rows")
        .or_else(|| report.pointer("/verification/inventory/rows"))
        .and_then(Value::as_array)
        .expect("verification inventory exposes correspondence rows");
    let mut failures = Vec::new();

    for row in rows {
        let expectation_id = row
            .pointer("/expectation_id")
            .and_then(Value::as_str)
            .unwrap_or("<missing expectation_id>");
        let result = row.pointer("/result").and_then(Value::as_str);
        if !matches!(result, Some("pass" | "fail")) {
            failures.push(format!(
                "{expectation_id}: inventory row result must be pass|fail, got {result:?}"
            ));
        }
        if !has_any_pointer(
            row,
            &[
                "/evidence_ref",
                "/evidence_refs",
                "/source_evidence_ref",
                "/runtime_evidence_ref",
                "/diagnostic_evidence_ref",
            ],
        ) {
            failures.push(format!(
                "{expectation_id}: inventory row must expose evidence refs"
            ));
        }
        if result == Some("fail")
            && !has_any_pointer(row, &["/diagnostic", "/fail_diagnostic", "/diagnostics"])
        {
            failures.push(format!(
                "{expectation_id}: failing inventory row must expose a fail diagnostic"
            ));
        }
    }

    let preview = failures.iter().take(12).cloned().collect::<Vec<_>>();
    assert!(
        failures.is_empty(),
        "verification.inventory.correspondence_rows must expose per-row result, evidence refs, and fail diagnostics; showing first {} of {} gaps:\n{}",
        preview.len(),
        failures.len(),
        preview.join("\n")
    );
}

#[test]
fn runtime_bridge_receipts_require_actual_source_ranges_and_five_semantic_hash_bundle() {
    let report = run_conformance();
    let semantic_hash_keys = [
        "store_hash",
        "membership_hash",
        "grant_hash",
        "relation_hash",
        "config_hash",
    ];

    for (expectation_id, transition) in [
        ("SCN04-R-P-STALE", "membership.request"),
        ("SCN08-R-P-LIVE", "fallback.advance"),
        ("SCN09-R-N-DRIFT", "patch.activate"),
        ("SCN10-R-P-S1", "cut.save"),
        ("SCN10-R-N-MERGE", "cut.restore"),
    ] {
        let row = inventory_row(&report, expectation_id);
        let entry = find_transition_entry(row, expectation_id, transition);
        assert_transition_uses_non_root_source_ref(entry, expectation_id, transition);
        assert_transition_has_hash_bundle(entry, expectation_id, transition, &semantic_hash_keys);
    }
}

#[test]
fn scn04_leave_requires_real_m9_authority_failure_and_revoked_m8_authority_snapshot() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN04-R-P-STALE");
    let entry = find_transition_entry(row, "SCN04-R-P-STALE", "membership.request");
    let mut failures = Vec::new();

    for pointer in [
        "/m9_authority_use/attempted_after_leave",
        "/m9_authority_use/result",
        "/m9_authority_use/diagnostic",
        "/after/authority_snapshot/membership/status",
        "/after/authority_snapshot/capability/status",
        "/after/authority_snapshot/witness/status",
        "/after/authority_snapshot/capability/ref",
        "/after/authority_snapshot/witness/ref",
    ] {
        require_pointer(entry, pointer, "SCN04 stale request", &mut failures);
    }

    if let Some(result) = entry.pointer("/m9_authority_use/result")
        && result != &json!("fail")
    {
        failures.push(format!(
            "SCN04 stale request M9 authority use must fail after leave, got {result}"
        ));
    }
    if let Some(status) = entry.pointer("/after/authority_snapshot/capability/status")
        && status != &json!("revoked")
    {
        failures.push(format!(
            "SCN04 active capability must be revoked after leave, got {status}"
        ));
    }
    if let Some(status) = entry.pointer("/after/authority_snapshot/witness/status")
        && status != &json!("invalidated")
    {
        failures.push(format!(
            "SCN04 active witness must be invalidated after leave, got {status}"
        ));
    }

    assert!(
        failures.is_empty(),
        "SCN04 membership fact must come from real M9 authority failure and active M8 membership/capability/witness snapshot, not the M10ScenarioState membership map:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn08_fallback_requires_actual_m8_relation_lifecycle_and_fresh_reacquire() {
    let report = run_conformance();
    let mut failures = Vec::new();

    for (expectation_id, pointer, expected) in [
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_relation_trace/0/transition",
            "invalidate_primary",
        ),
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_relation_trace/1/selected_floor",
            "anchor",
        ),
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_relation_trace/2/transition",
            "freeze_fallback",
        ),
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_relation_trace/2/selected_floor",
            "frozen",
        ),
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_relation_trace/2/selected_target",
            "default_pose",
        ),
        (
            "SCN08-R-P-EXPIRE",
            "/runtime_transition_trace/m8_option_chain/options/2/projection_kind",
            "opaque_default_pose",
        ),
        (
            "SCN08-R-N-REPROMOTE",
            "/runtime_transition_trace/m8_relation_trace/0/transition",
            "reject_same_lineage_live_repromotion",
        ),
        (
            "SCN08-R-P-REACQUIRE",
            "/runtime_transition_trace/m8_relation_trace/0/fresh_epoch",
            "true",
        ),
        (
            "SCN08-R-P-REACQUIRE",
            "/runtime_transition_trace/m8_relation_trace/0/fresh_witness",
            "true",
        ),
        (
            "SCN08-R-P-REACQUIRE",
            "/runtime_transition_trace/m8_relation_trace/0/selected_floor",
            "live",
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        match row.pointer(pointer) {
            Some(Value::Bool(value)) if expected == "true" && *value => {}
            Some(Value::String(value)) if value == expected => {}
            Some(other) => failures.push(format!(
                "{expectation_id} {pointer} expected {expected}, got {other}"
            )),
            None => failures.push(format!("{expectation_id} missing {pointer}")),
        }
    }

    let expire = inventory_row(&report, "SCN08-R-P-EXPIRE");
    let trace = expire
        .pointer("/runtime_transition_trace")
        .expect("SCN08-R-P-EXPIRE exposes runtime_transition_trace");
    require_absent_or_null_pointer(
        trace,
        "/m10_fallback_trace",
        "SCN08-R-P-EXPIRE actual M8 fallback lifecycle",
        &mut failures,
    );
    require_scn08_trace_edge(
        trace,
        2,
        1,
        2,
        "frozen",
        "default_pose",
        "SCN08-R-P-EXPIRE actual M8 anchor-to-frozen lifecycle",
        &mut failures,
    );

    assert!(
        failures.is_empty(),
        "SCN08 fallback facts must be backed by actual M8 relation invalidation/reacquire behavior and anchor-to-frozen default_pose state, not m10_fallback_trace:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn08_positive_fallback_must_not_use_m10_manual_cursor_or_selected_floor_shortcut() {
    let mut failures = Vec::new();
    require_scn08_schedule_order(&mut failures);
    require_scn08_rollback_restore_source_guard(&mut failures);
    require_scn08_write_current_option_source_guard(&mut failures);
    for path in m10_production_source_paths() {
        let text = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("can read M10 production source {}: {error}", path.display())
        });
        for forbidden in ["M10TypedFallbackFloor", "fallback_cursor"] {
            for (line_index, line) in text.lines().enumerate() {
                if line.contains(forbidden) {
                    failures.push(format!(
                        "M10 production source {}:{} must not contain {forbidden}; SCN08 fallback state must be derived from M8 relation state/projection",
                        path.display(),
                        line_index + 1
                    ));
                }
            }
        }
    }

    match run_conformance_result() {
        Ok(report) => {
            require_scn08_m8_relation_lifecycle(
                &report,
                "SCN08-R-P-LIVE",
                "live",
                0,
                &["select_primary"],
                &mut failures,
            );
            require_scn08_m8_relation_lifecycle(
                &report,
                "SCN08-R-P-EXPIRE",
                "frozen",
                2,
                &[
                    "invalidate_primary",
                    "select_fallback",
                    "freeze_fallback",
                ],
                &mut failures,
            );
            let expire = inventory_row(&report, "SCN08-R-P-EXPIRE");
            let expire_trace = expire
                .pointer("/runtime_transition_trace")
                .expect("SCN08-R-P-EXPIRE exposes runtime_transition_trace");
            require_scn08_trace_edge(
                expire_trace,
                0,
                0,
                1,
                "anchor",
                "room_anchor",
                "SCN08-R-P-EXPIRE actual M8 live-to-anchor lifecycle",
                &mut failures,
            );
            require_scn08_trace_edge(
                expire_trace,
                2,
                1,
                2,
                "frozen",
                "default_pose",
                "SCN08-R-P-EXPIRE actual M8 anchor-to-frozen lifecycle",
                &mut failures,
            );
            require_scn08_expiry_semantics(
                expire_trace,
                "SCN08-R-P-EXPIRE actual lease expiry semantics",
                &mut failures,
            );
            require_scn08_m8_relation_lifecycle(
                &report,
                "SCN08-R-P-ROLLBACK",
                "frozen",
                2,
                &["reject_same_lineage_live_repromotion"],
                &mut failures,
            );
            let rollback = inventory_row(&report, "SCN08-R-P-ROLLBACK");
            if rollback
                .pointer("/runtime_transition_trace/m8_relation_state/rollback_repromoted")
                != Some(&json!(false))
            {
                failures.push(format!(
                    "SCN08-R-P-ROLLBACK must prove rollback cannot repromote the prior live relation; got {:?}",
                    rollback.pointer(
                        "/runtime_transition_trace/m8_relation_state/rollback_repromoted"
                    )
                ));
            }
            require_scn08_rollback_local_cut_guard(
                rollback,
                "SCN08-R-P-ROLLBACK actual M8 local-cut restore guard",
                &mut failures,
            );
            require_scn08_m8_relation_lifecycle(
                &report,
                "SCN08-R-P-REACQUIRE",
                "live",
                0,
                &["reacquire_primary"],
                &mut failures,
            );
            let reacquire = inventory_row(&report, "SCN08-R-P-REACQUIRE");
            for pointer in [
                "/runtime_transition_trace/m8_relation_state/fresh_m9_reacquire",
                "/runtime_transition_trace/m8_relation_state/fresh_lineage",
                "/runtime_transition_trace/m8_relation_state/fresh_epoch",
                "/runtime_transition_trace/m8_relation_state/fresh_witness",
                "/runtime_transition_trace/m8_relation_state/index0_created_by_fresh_m9_reacquire",
            ] {
                require_bool_pointer(
                    reacquire,
                    pointer,
                    true,
                    "SCN08-R-P-REACQUIRE actual M8 fresh lineage",
                    &mut failures,
                );
            }
            require_bool_pointer(
                reacquire,
                "/runtime_transition_trace/m8_relation_state/manual_index0_reset",
                false,
                "SCN08-R-P-REACQUIRE actual M8 fresh lineage",
                &mut failures,
            );
        }
        Err(error) => failures.push(format!(
            "M10 conformance report must be generated before SCN08 relation lifecycle evidence can be checked: {error}"
        )),
    }

    assert!(
        failures.is_empty(),
        "SCN08 positive fallback rows must be derived from actual M8 relation state/projection, not M10TypedFallbackFloor/fallback_cursor, m10_fallback_trace, or manual selected_floor shortcuts:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn08_write_after_expiry_requires_actual_m8_current_option_capability_reject() {
    let mut failures = Vec::new();
    require_scn08_schedule_order(&mut failures);
    require_scn08_write_current_option_source_guard(&mut failures);

    match run_conformance_result() {
        Ok(report) => {
            let row = inventory_row(&report, "SCN08-R-P-WRITE");
            let context = "SCN08-R-P-WRITE actual M8 current-option capability reject";
            let trace = match require_pointer(row, "/runtime_transition_trace", context, &mut failures)
            {
                Some(trace) => trace,
                None => {
                    assert!(
                        failures.is_empty(),
                        "SCN08 write guard cannot inspect missing runtime trace:\n{}",
                        failures.join("\n")
                    );
                    return;
                }
            };
            for pointer in [
                "/m8_current_option_capability_validation/attempted",
                "/m8_current_option_capability_validation/validator",
                "/m8_current_option_capability_validation/selected_floor",
                "/m8_current_option_capability_validation/selected_option_index",
                "/m8_current_option_capability_validation/selected_target",
                "/m8_current_option_capability_validation/current_option_capability",
                "/m8_current_option_capability_validation/write_capable",
                "/m8_current_option_capability_validation/later_write_capable_option_exists",
                "/m8_current_option_capability_validation/request_level_reject",
                "/m8_current_option_capability_validation/outcome",
                "/m8_current_option_capability_validation/diagnostic/code",
                "/m8_current_option_capability_validation/no_five_domain_mutation",
                "/m8_current_option_capability_validation/schedule_action_reference",
                "/m8_current_option_capability_validation/trace_range",
            ] {
                require_pointer(trace, pointer, context, &mut failures);
            }
            require_json_pointer_equal(
                row,
                "/schedule_action_reference",
                "/runtime_transition_trace/m8_current_option_capability_validation/schedule_action_reference",
                context,
                &mut failures,
            );
            require_bool_pointer(
                trace,
                "/m8_current_option_capability_validation/attempted",
                true,
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/validator",
                json!("m8_current_option_capability_validator"),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/selected_floor",
                json!("frozen"),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/selected_option_index",
                json!(2),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/selected_target",
                json!("default_pose"),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/current_option_capability",
                json!("cap:relation:view_pose:frozen"),
                context,
                &mut failures,
            );
            require_bool_pointer(
                trace,
                "/m8_current_option_capability_validation/write_capable",
                false,
                context,
                &mut failures,
            );
            require_bool_pointer(
                trace,
                "/m8_current_option_capability_validation/later_write_capable_option_exists",
                false,
                context,
                &mut failures,
            );
            require_bool_pointer(
                trace,
                "/m8_current_option_capability_validation/request_level_reject",
                true,
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/outcome",
                json!("rejected"),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                trace,
                "/m8_current_option_capability_validation/diagnostic/code",
                json!("WriteCapabilityUnavailable"),
                context,
                &mut failures,
            );
            require_bool_pointer(
                trace,
                "/m8_current_option_capability_validation/no_five_domain_mutation",
                true,
                context,
                &mut failures,
            );
            require_no_mutation_reject_snapshot(
                row,
                "SCN08-R-P-WRITE",
                "fallback.write",
                "WriteCapabilityUnavailable",
                &mut failures,
            );
            if let Some(entries) = trace
                .pointer("/transition_trace")
                .and_then(Value::as_array)
            {
                let actual = entries
                    .iter()
                    .filter_map(|entry| entry.get("transition").and_then(Value::as_str))
                    .collect::<Vec<_>>();
                if actual.contains(&"select_primary") {
                    failures.push(format!(
                        "SCN08-R-P-WRITE must not accept generic select_primary/report-only evidence after expiry; actual transitions={actual:?}"
                    ));
                }
            }
        }
        Err(error) => failures.push(format!(
            "M10 conformance report must be generated before SCN08 write capability evidence can be checked: {error}"
        )),
    }

    assert!(
        failures.is_empty(),
        "SCN08 write after expiry must invoke actual M8 current-option capability validation, reject with WriteCapabilityUnavailable, and preserve all five semantic domains:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn09_patch_facts_require_actual_m8_activation_delta_and_drift_without_activation_cut() {
    let report = run_conformance();
    let mut failures = Vec::new();

    for (expectation_id, delta_pointer) in [
        (
            "SCN09-R-P-PIPELINE",
            "/runtime_transition_trace/m8_patch_activation/activation_cut/hash",
        ),
        (
            "SCN09-R-P-INIT",
            "/runtime_transition_trace/m8_patch_activation/store_delta_hash",
        ),
        (
            "SCN09-R-P-OBS",
            "/runtime_transition_trace/m8_patch_activation/projection_delta_hash",
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        if row.pointer("/runtime_transition_trace/m8_patch_activation/activate_patch_called")
            != Some(&json!(true))
        {
            failures.push(format!(
                "{expectation_id} must prove actual M8 activate_patch was called"
            ));
        }
        if row.pointer(delta_pointer).is_none() {
            failures.push(format!("{expectation_id} missing {delta_pointer}"));
        }
    }

    let drift = inventory_row(&report, "SCN09-R-N-DRIFT");
    if drift.pointer("/runtime_transition_trace/m8_patch_activation/activate_patch_called")
        != Some(&json!(false))
    {
        failures.push(
            "SCN09-R-N-DRIFT must prove M8 activate_patch was not called after membership drift"
                .to_string(),
        );
    }
    if !drift
        .pointer("/runtime_transition_trace/m8_patch_activation/activation_cut")
        .is_none_or(Value::is_null)
    {
        failures.push("SCN09-R-N-DRIFT must not expose an activation cut".to_string());
    }

    assert!(
        failures.is_empty(),
        "SCN09 patch facts must be generated from actual M8 activation/projection deltas, and drift must stop before activation:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn10_composite_cut_requires_m8_m9_ledger_fallback_config_bundle_and_restore_invariance() {
    let report = run_conformance();
    let cut_bundle_keys = [
        "m8_cut_hash",
        "m9_authority_hash",
        "ledger_hash",
        "fallback_hash",
        "config_hash",
    ];
    let s1 = inventory_row(&report, "SCN10-R-P-S1");
    let save = find_transition_entry(s1, "SCN10-R-P-S1", "cut.save");
    assert_transition_has_hash_bundle(save, "SCN10-R-P-S1", "cut.save", &cut_bundle_keys);

    for (expectation_id, transition) in [
        ("SCN10-R-N-MERGE", "cut.restore"),
        ("SCN10-R-N-LEASEDOCTOR", "cut.restore"),
        ("SCN10-R-N-CUTDOCTOR", "cut.restore"),
    ] {
        let row = inventory_row(&report, expectation_id);
        let entry = find_transition_entry(row, expectation_id, transition);
        assert_transition_preserves_hash_bundle(
            entry,
            expectation_id,
            transition,
            &cut_bundle_keys,
        );
    }
}

#[test]
fn p0_scn10_load_s1_fresh_must_restore_into_new_m9_m8_bridged_composite_session() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN10-R-P-LOADFRESH");
    let context = "SCN10-R-P-LOADFRESH fresh M9->M8 composite restore";
    let mut failures = Vec::new();

    require_scn10_canon_line_binding(row, "SCN10-R-P-LOADFRESH", &mut failures);
    for pointer in [
        "/runtime_transition_trace/fresh_load/composite_restore",
        "/runtime_transition_trace/fresh_load/composite_restore/attempted",
        "/runtime_transition_trace/fresh_load/composite_restore/result",
        "/runtime_transition_trace/fresh_load/composite_restore/new_composite_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/new_m9_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/new_m8_runtime_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/m9_to_m8_bridge/session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/m9_to_m8_bridge/accessor",
        "/runtime_transition_trace/fresh_load/composite_restore/m8_runtime_construction/accessor",
        "/runtime_transition_trace/fresh_load/composite_restore/m8_runtime_construction/new_runtime_constructed",
        "/runtime_transition_trace/fresh_load/composite_restore/m8_restore/accessor",
        "/runtime_transition_trace/fresh_load/composite_restore/m8_restore/cut_id",
        "/runtime_transition_trace/fresh_load/composite_restore/m8_restore/result",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/no_mutation",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/session_id_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/session_id_after",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/m9_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/m8_runtime_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/relation_hash_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/relation_hash_after",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/cut_hash_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/cut_hash_after",
    ] {
        require_pointer(row, pointer, context, &mut failures);
    }
    require_object_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore",
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/attempted",
        true,
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/result",
        json!("accepted"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m8_runtime_construction/accessor",
        json!("M8LocalRuntime::from_admitted"),
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m8_runtime_construction/new_runtime_constructed",
        true,
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m9_to_m8_bridge/accessor",
        json!("M9M10AuthorityBridge::authority_state"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m8_restore/accessor",
        json!("M8LocalRuntime::try_restore_local_cut"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m8_restore/result",
        json!("accepted"),
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/no_mutation",
        true,
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/session_id_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/session_id_after",
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/relation_hash_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/relation_hash_after",
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/cut_hash_before",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/cut_hash_after",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/new_composite_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session_id",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/new_m9_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/m9_session_id",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/new_m8_runtime_session_id",
        "/runtime_transition_trace/fresh_load/composite_restore/old_current_session/m8_runtime_session_id",
        context,
        &mut failures,
    );
    fail_if_true_pointer(
        row,
        "/runtime_transition_trace/fresh_load/m9_projection_comparison_only",
        context,
        &mut failures,
    );
    fail_if_true_pointer(
        row,
        "/runtime_transition_trace/fresh_load/composite_restore/m9_projection_only",
        context,
        &mut failures,
    );

    assert!(
        failures.is_empty(),
        "SCN10 load_s1_fresh must restore S1 into a genuinely new bridged M9->M8 composite session while proving the old current session is unchanged; projection comparison alone is insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_s2_lease_expiry_must_expire_seeded_relation_lease_in_persistent_m8_runtime() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN10-R-P-S2");
    let context = "SCN10-R-P-S2 persistent M8 seeded relation lease expiry";
    let mut failures = Vec::new();

    require_scn10_canon_line_binding(row, "SCN10-R-P-S2", &mut failures);
    for pointer in [
        "/runtime_transition_trace/s2_lease_expiry",
        "/runtime_transition_trace/s2_lease_expiry/m8_runtime_session_id",
        "/runtime_transition_trace/s2_lease_expiry/relation_domain",
        "/runtime_transition_trace/s2_lease_expiry/seeded_relation_lease_ref",
        "/runtime_transition_trace/s2_lease_expiry/lease_expiry/accessor",
        "/runtime_transition_trace/s2_lease_expiry/lease_expiry/result",
        "/runtime_transition_trace/s2_lease_expiry/lease_state_before/status",
        "/runtime_transition_trace/s2_lease_expiry/lease_state_after/status",
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta",
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/accessor",
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/before/hash",
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/after/hash",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/accessor",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/before/hash",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/after/hash",
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/transition",
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/invalidation_reason",
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/derived_from_actual_m8_relation_state",
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
    ] {
        require_pointer(row, pointer, context, &mut failures);
    }
    for pointer in [
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta",
    ] {
        require_object_pointer(row, pointer, context, &mut failures);
    }
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/session_id",
        "/runtime_transition_trace/s2_lease_expiry/m8_runtime_session_id",
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/relation_domain",
        json!("semantic_relation_lease"),
        context,
        &mut failures,
    );
    require_non_empty_string_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/seeded_relation_lease_ref",
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/lease_expiry/accessor",
        json!("M8LocalRuntime::invalidate_primary"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/lease_expiry/result",
        json!("expired"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/lease_state_before/status",
        json!("live"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/lease_state_after/status",
        json!("expired"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/accessor",
        json!("M8LocalRuntime::canonical_relation_projection"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/accessor",
        json!("M8LocalCut::canonical_semantic_projection"),
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/before/hash",
        "/runtime_transition_trace/s2_lease_expiry/native_relation_delta/after/hash",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/before/hash",
        "/runtime_transition_trace/s2_lease_expiry/native_cut_delta/after/hash",
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/transition",
        json!("invalidate_primary"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/invalidation_reason",
        json!("lease-expired"),
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/derived_from_actual_m8_relation_state",
        true,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/s2_lease_expiry/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
        true,
        context,
        &mut failures,
    );
    if row.pointer("/runtime_transition_trace/deltas/lease_expiry") == Some(&json!(true))
        && row
            .pointer("/runtime_transition_trace/s2_lease_expiry/native_relation_delta")
            .is_none()
    {
        failures.push(
            "SCN10-R-P-S2 JSON deltas.lease_expiry=true is insufficient without native M8 relation/cut delta evidence"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "SCN10 S2 lease_expiry must expire a seeded relation lease in the persistent M8 runtime and expose native relation/cut state deltas, not only JSON deltas.lease_expiry=true:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_reacquire_after_load_must_execute_fresh_m9_admission_bridge_and_m8_occurrence() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN10-R-P-REACQUIRE");
    let context = "SCN10-R-P-REACQUIRE fresh M9 admission and M8 occurrence";
    let mut failures = Vec::new();

    require_scn10_canon_line_binding(row, "SCN10-R-P-REACQUIRE", &mut failures);
    for pointer in [
        "/runtime_transition_trace/reacquire_after_load",
        "/runtime_transition_trace/reacquire_after_load/persistent_session_id",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/accessor",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/before_membership_ref",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_membership_ref",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/before_epoch",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_epoch",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_witness_ref",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/accessor",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/before_generation",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/after_generation",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/before_authority_snapshot_ref",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/after_authority_snapshot_ref",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/runtime_session_id",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/source_accessor",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/before_occurrence_id",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/after_occurrence_id",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/new_lease_ref",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/lease_source_accessor",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/native_relation_delta/before/hash",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/native_relation_delta/after/hash",
    ] {
        require_pointer(row, pointer, context, &mut failures);
    }
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/session_id",
        "/runtime_transition_trace/reacquire_after_load/persistent_session_id",
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/session_id",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/runtime_session_id",
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/m9_admission/accessor",
        json!("M10MembershipLifecycleSession::admit_fresh"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/accessor",
        json!("M10CompositeCutSession::refresh_bridge"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/source_accessor",
        json!("M8LocalRuntime::enqueue_owner"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/lease_source_accessor",
        json!("M8LocalRuntime::reacquire_primary"),
        context,
        &mut failures,
    );
    require_non_empty_string_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_witness_ref",
        context,
        &mut failures,
    );
    require_non_empty_string_pointer(
        row,
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/new_lease_ref",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/m9_admission/before_membership_ref",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_membership_ref",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/m9_admission/before_epoch",
        "/runtime_transition_trace/reacquire_after_load/m9_admission/after_epoch",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/before_generation",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/after_generation",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/before_authority_snapshot_ref",
        "/runtime_transition_trace/reacquire_after_load/bridge_refresh/after_authority_snapshot_ref",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/before_occurrence_id",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/after_occurrence_id",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/native_relation_delta/before/hash",
        "/runtime_transition_trace/reacquire_after_load/m8_occurrence/native_relation_delta/after/hash",
        context,
        &mut failures,
    );
    if row.pointer("/runtime_transition_trace/receipt_origin")
        == Some(&json!("m8-local-cut-reacquire"))
        && row
            .pointer("/runtime_transition_trace/reacquire_after_load/m9_admission")
            .is_none()
    {
        failures.push(
            "SCN10-R-P-REACQUIRE receipt_origin=m8-local-cut-reacquire is insufficient without fresh M9 admission, bridge refresh, and native M8 occurrence/lease evidence"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "SCN10 reacquire_after_load must execute fresh M9 admission, produce a fresh epoch/witness, refresh the M9->M8 bridge, and create a distinct native M8 occurrence/lease; receipt-only evidence is insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_negative_branches_must_start_from_same_persistent_positive_s2_lineage() {
    let report = run_conformance();
    let s2_row = inventory_row(&report, "SCN10-R-P-S2");
    let mut failures = Vec::new();

    for expectation_id in [
        "SCN10-R-N-MERGE",
        "SCN10-R-N-LEASEDOCTOR",
        "SCN10-R-N-CUTDOCTOR",
    ] {
        let row = inventory_row(&report, expectation_id);
        require_scn10_canon_line_binding(row, expectation_id, &mut failures);
        require_scn10_no_stale_resurrection_canon_binding(row, expectation_id, &mut failures);
        require_scn10_current_s2_positive_lineage(row, s2_row, expectation_id, &mut failures);
    }

    assert!(
        failures.is_empty(),
        "SCN10 negative stale merge / lease doctor / cut doctor branches must all start from the same persistent positive M9+M8 current session that executed S1 -> A leave -> maintainer actual lease expiry -> S2; sentinel-only stale floors or fresh initial negative sessions are insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_stale_merge_must_preflight_reject_without_current_m9_or_m8_mutation() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN10-R-N-MERGE");
    let s2_row = inventory_row(&report, "SCN10-R-P-S2");
    let context = "SCN10-R-N-MERGE stale merge current-session no-mutation";
    let mut failures = Vec::new();

    require_scn10_canon_line_binding(row, "SCN10-R-N-MERGE", &mut failures);
    require_scn10_no_stale_resurrection_canon_binding(row, "SCN10-R-N-MERGE", &mut failures);
    require_scn10_current_s2_positive_lineage(row, s2_row, "SCN10-R-N-MERGE", &mut failures);
    for pointer in [
        "/runtime_transition_trace/stale_merge_preflight",
        "/runtime_transition_trace/stale_merge_preflight/source",
        "/runtime_transition_trace/stale_merge_preflight/preflight_accessor",
        "/runtime_transition_trace/stale_merge_preflight/preflight_target",
        "/runtime_transition_trace/stale_merge_preflight/result",
        "/runtime_transition_trace/stale_merge_preflight/diagnostic/code",
        "/runtime_transition_trace/stale_merge_preflight/current_session_id",
        "/runtime_transition_trace/stale_merge_preflight/candidate_session_id",
        "/runtime_transition_trace/stale_merge_preflight/clone_source_session_id",
        "/runtime_transition_trace/stale_merge_preflight/clone_runtime_constructed",
        "/runtime_transition_trace/stale_merge_preflight/rejected_before_current_restore",
        "/runtime_transition_trace/stale_merge_preflight/no_current_m9_restore_attempted",
        "/runtime_transition_trace/stale_merge_preflight/no_current_m8_restore_attempted",
        "/runtime_transition_trace/stale_merge_preflight/candidate_m9_restore/accessor",
        "/runtime_transition_trace/stale_merge_preflight/candidate_m9_restore/result",
        "/runtime_transition_trace/stale_merge_preflight/candidate_m8_restore/accessor",
        "/runtime_transition_trace/stale_merge_preflight/candidate_m8_restore/result",
        "/runtime_transition_trace/current_session_no_mutation/original_before/session_id",
        "/runtime_transition_trace/current_session_no_mutation/final_after/session_id",
        "/runtime_transition_trace/current_session_no_mutation/original_before/store_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/store_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/membership_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/membership_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/grant_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/grant_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/relation_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/relation_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/config_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/config_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/cut_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/cut_hash",
        "/runtime_transition_trace/current_session_no_mutation/original_before/ledger_hash",
        "/runtime_transition_trace/current_session_no_mutation/final_after/ledger_hash",
    ] {
        require_pointer(row, pointer, context, &mut failures);
    }
    require_object_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight",
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/source",
        json!("candidate_preflight_clone"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/preflight_accessor",
        json!("M10CompositeCutSession::preflight_stale_merge_on_candidate"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/preflight_target",
        json!("S2_current"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/s1_candidate_ref",
        json!("S1"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/current_s2_ref",
        json!("S2"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/result",
        json!("rejected"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/diagnostic/code",
        json!("E-CUT-002"),
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/clone_runtime_constructed",
        true,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/rejected_before_current_restore",
        true,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/no_current_m9_restore_attempted",
        true,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/no_current_m8_restore_attempted",
        true,
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m9_restore/accessor",
        json!("M10MembershipLifecycleSession::restore_authority_cut"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m9_restore/stage",
        json!("candidate_m9_restore"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m9_restore/result",
        json!("accepted"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/s2_current_preflight/stage",
        json!("s2_current_preflight"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/s2_current_preflight/result",
        json!("rejected"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/s2_current_preflight/diagnostic/code",
        json!("E-CUT-002"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m8_restore/accessor",
        json!("M8LocalRuntime::try_restore_local_cut"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m8_restore/stage",
        json!("candidate_m8_restore"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/stale_merge_preflight/candidate_m8_restore/result",
        json!("rejected"),
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/stale_merge_preflight/current_session_id",
        "/runtime_transition_trace/current_session_no_mutation/original_before/session_id",
        context,
        &mut failures,
    );
    require_json_pointer_equal(
        row,
        "/runtime_transition_trace/current_session_no_mutation/original_before/session_id",
        "/runtime_transition_trace/current_session_no_mutation/final_after/session_id",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/stale_merge_preflight/current_session_id",
        "/runtime_transition_trace/stale_merge_preflight/candidate_session_id",
        context,
        &mut failures,
    );
    require_json_pointer_not_equal(
        row,
        "/runtime_transition_trace/stale_merge_preflight/current_session_id",
        "/runtime_transition_trace/stale_merge_preflight/clone_source_session_id",
        context,
        &mut failures,
    );
    for hash_key in [
        "store_hash",
        "membership_hash",
        "grant_hash",
        "relation_hash",
        "config_hash",
        "cut_hash",
        "ledger_hash",
    ] {
        let before = format!(
            "/runtime_transition_trace/current_session_no_mutation/original_before/{hash_key}"
        );
        let after =
            format!("/runtime_transition_trace/current_session_no_mutation/final_after/{hash_key}");
        require_json_pointer_equal(row, &before, &after, context, &mut failures);
    }
    require_scn10_current_s2_no_domain_mutation(row, "SCN10-R-N-MERGE", &mut failures);
    if let Some(entries) = row
        .pointer("/runtime_transition_trace/transition_trace")
        .and_then(Value::as_array)
    {
        for (index, entry) in entries.iter().enumerate() {
            let is_current_restore = entry.pointer("/transition") == Some(&json!("m9.cut.restore"))
                && entry.pointer("/accepted") == Some(&json!(true))
                && entry.pointer("/session_role") != Some(&json!("candidate_preflight"))
                && entry.pointer("/preflight_candidate") != Some(&json!(true));
            if is_current_restore {
                failures.push(format!(
                    "{context} must not contain accepted current-session m9.cut.restore before stale merge rejection; transition_trace[{index}]={entry:#}"
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "SCN10 stale merge must preflight an S1 candidate against current S2, keep candidate M9 restore acceptance separate from rejected S2/M8 preflight, and preserve current S2 store/membership/grant/relation/config hashes:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_doctor_branches_must_mutate_actual_s2_cut_clone_and_preserve_current_s2() {
    let report = run_conformance();
    let s2_row = inventory_row(&report, "SCN10-R-P-S2");
    let mut failures = Vec::new();

    for (expectation_id, mutation_kind, diagnostic_code) in [
        (
            "SCN10-R-N-LEASEDOCTOR",
            "expired_lease_flipped_live",
            "E-CUT-001",
        ),
        (
            "SCN10-R-N-CUTDOCTOR",
            "receive_without_send_injected",
            "E-CUT-001",
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        require_scn10_canon_line_binding(row, expectation_id, &mut failures);
        require_scn10_no_stale_resurrection_canon_binding(row, expectation_id, &mut failures);
        require_scn10_current_s2_positive_lineage(row, s2_row, expectation_id, &mut failures);
        require_scn10_s2_cut_clone_doctor_guard(
            row,
            expectation_id,
            mutation_kind,
            diagnostic_code,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "SCN10 lease doctor and cut doctor must mutate an actual S2 cut clone, reject that clone, and prove the current S2 store/membership/grant/relation/config domains remain unchanged:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn10_cutdoctor_must_remove_actual_send_from_real_receive_dependency_edge() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN10-R-N-CUTDOCTOR");
    let s2_row = inventory_row(&report, "SCN10-R-P-S2");
    let mut failures = Vec::new();

    require_scn10_canon_line_binding(row, "SCN10-R-N-CUTDOCTOR", &mut failures);
    require_scn10_no_stale_resurrection_canon_binding(row, "SCN10-R-N-CUTDOCTOR", &mut failures);
    require_scn10_current_s2_positive_lineage(row, s2_row, "SCN10-R-N-CUTDOCTOR", &mut failures);
    require_scn10_s2_cut_clone_doctor_guard(
        row,
        "SCN10-R-N-CUTDOCTOR",
        "receive_without_send_injected",
        "E-CUT-001",
        &mut failures,
    );
    require_scn10_cutdoctor_send_receive_edge_guard(row, &mut failures);

    assert!(
        failures.is_empty(),
        "SCN10 cut doctor must start from an actual S2 M8 cut with concrete send/receive occurrence IDs and a dependency edge, doctor a real cut prefix by excluding the send while retaining the receive, and have ordinary M8LocalRuntime::try_restore_local_cut reject the missing-send consistency violation; private marker booleans are insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p1_fallback_repromotion_falsifier_must_expose_actual_m8_negative_stage_or_narrow_claim() {
    let value = falsifier_report("fallback_repromotion_without_reacquire");
    let context = "fallback_repromotion_without_reacquire actual M8 negative stage";
    let mut failures = Vec::new();

    assert_pointer(&value, "/terminal_outcome", json!("ConformanceFailure"));
    let claim_narrowed = value.pointer("/validation/fallback_lineage_claim_scope")
        == Some(&json!("typed_carrier_only"))
        && value.pointer("/runtime/no_m8_negative_stage_claimed") == Some(&json!(true));
    let has_m8_stage_evidence = has_any_pointer(
        &value,
        &[
            "/validation/validator_results/fallback_lineage_validator/input_stage",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/runtime_session_id",
            "/runtime/no_mutation_boundary/stage",
        ],
    );
    if claim_narrowed
        && !has_m8_stage_evidence
        && value.pointer("/validation/real_validator_invoked") == Some(&json!(true))
    {
        failures.push(
            "fallback_repromotion_without_reacquire typed-carrier-only scope has no M8 stage evidence, so validation.real_validator_invoked must be false or omitted".to_string(),
        );
    }
    if !claim_narrowed {
        for pointer in [
            "/validation/validator_results/fallback_lineage_validator/input_stage",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/runtime_session_id",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/source_accessor",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/diagnostic_code",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/transition",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/selected_floor_before",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/attempted_floor",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/selected_floor_after",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/derived_from_actual_m8_relation_state",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/no_mutation_snapshot_before/relation_hash",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/no_mutation_snapshot_after/relation_hash",
            "/runtime/no_mutation_boundary/stage",
            "/runtime/no_mutation_boundary/before_snapshot/relation_hash",
            "/runtime/no_mutation_boundary/after_snapshot/relation_hash",
        ] {
            require_pointer(&value, pointer, context, &mut failures);
        }
        require_json_value_pointer(
            &value,
            "/validation/validator_results/fallback_lineage_validator/input_stage",
            json!("runtime"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/source_accessor",
            json!("M8LocalRuntime::note_primary_available_same_lineage"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/diagnostic_code",
            json!("E-LIN-003"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/transition",
            json!("reject_same_lineage_live_repromotion"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/selected_floor_before",
            json!("anchor"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/attempted_floor",
            json!("live"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/selected_floor_after",
            json!("anchor"),
            context,
            &mut failures,
        );
        require_bool_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/derived_from_actual_m8_relation_state",
            true,
            context,
            &mut failures,
        );
        require_bool_pointer(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
            true,
            context,
            &mut failures,
        );
        require_json_pointer_equal(
            &value,
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/no_mutation_snapshot_before/relation_hash",
            "/validation/validator_state/fallback_lineage_validator/m8_negative_stage/no_mutation_snapshot_after/relation_hash",
            context,
            &mut failures,
        );
        require_json_pointer_equal(
            &value,
            "/runtime/no_mutation_boundary/before_snapshot/relation_hash",
            "/runtime/no_mutation_boundary/after_snapshot/relation_hash",
            context,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "fallback_repromotion_without_reacquire must either expose actual M8 validator negative-stage evidence for the same-lineage repromotion rejection or explicitly narrow the claim to typed-carrier validation:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p1_scn02_stale_membership_must_use_target_leave_live_self_authority_and_target_stale_reject() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN02-R-N-STALE");
    let context = "SCN02-R-N-STALE target leave/live self authority lifecycle guard";
    let mut failures = Vec::new();

    require_scn02_stale_membership_canon_binding(row, &mut failures);
    for pointer in [
        "/runtime_transition_trace/scn02_stale_membership_guard/persistent_session/m9_authority_session_id",
        "/runtime_transition_trace/scn02_stale_membership_guard/persistent_session/m8_runtime_session_id",
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/request_identity",
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/actor_authority_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_membership_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_capability_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/live_witness_ref",
        "/runtime_transition_trace/scn02_stale_membership_guard/exogenous_leave_action/input_action_id",
    ] {
        require_pointer(row, pointer, context, &mut failures);
    }
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/actor_principal",
        json!("self"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/authority_principal",
        json!("self"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/target_identity",
        json!("target"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/leave_action_source",
        json!("exogenous_schedule_input"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retire_transition",
        json!("membership.retire"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/principal",
        json!("self"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/actor_authority/membership_status",
        json!("live"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/bridge_refresh/accessor",
        json!("M9M10AuthorityBridge::refresh"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/bridge_refresh/result",
        json!("refreshed_after_target_leave"),
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/reuses_same_attack_request",
        true,
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/result",
        json!("rejected"),
        context,
        &mut failures,
    );
    require_json_value_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/attack_request/diagnostic/code",
        json!("StaleMembership"),
        context,
        &mut failures,
    );
    for (left, right) in [
        (
            "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retired_membership_ref",
            "/runtime_transition_trace/scn02_stale_membership_guard/stale_membership_trace/membership_ref",
        ),
        (
            "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retired_epoch",
            "/runtime_transition_trace/scn02_stale_membership_guard/stale_membership_trace/epoch",
        ),
        (
            "/runtime_transition_trace/scn02_stale_membership_guard/target_membership_lifecycle/retired_incarnation",
            "/runtime_transition_trace/scn02_stale_membership_guard/stale_membership_trace/incarnation",
        ),
        (
            "/runtime_transition_trace/scn02_stale_membership_guard/five_domain_no_mutation/original_before/store_hash",
            "/runtime_transition_trace/scn02_stale_membership_guard/five_domain_no_mutation/final_after/store_hash",
        ),
    ] {
        require_json_pointer_equal(row, left, right, context, &mut failures);
    }
    require_scn02_five_domain_no_mutation_guard(row, &mut failures);
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/exogenous_leave_action/result_supplied",
        false,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/exogenous_leave_action/authority_supplied",
        false,
        context,
        &mut failures,
    );
    require_bool_pointer(
        row,
        "/runtime_transition_trace/scn02_stale_membership_guard/generic_corrupted_object_used",
        false,
        context,
        &mut failures,
    );

    assert!(
        failures.is_empty(),
        "SCN02 stale-membership lifecycle evidence must come from target_leave retiring target while attack(target) is still issued by live BrowserClient[self] authority, then reject with StaleMembership and preserve the five semantic domains; old target-authority or store-only evidence is insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn02_stale_attack_must_keep_self_actor_authority_and_target_stale_subject() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN02-R-N-STALE");
    let mut failures = Vec::new();

    require_scn02_stale_membership_canon_binding(row, &mut failures);
    require_scn02_actor_self_target_stale_guard(row, &mut failures);
    require_scn02_five_domain_no_mutation_guard(row, &mut failures);

    assert!(
        failures.is_empty(),
        "SCN02 post-target-leave stale attack must keep the request actor and authority principal as BrowserClient[self] with live self authority, retire the separate target membership/existence ref via target_leave, reject attack(target) because the target membership is stale, and preserve store/membership/grant/relation/config semantic domains; substituting target as the authority principal or store-only no-mutation evidence is insufficient:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p1_authority_snapshot_name_must_match_m8_inventory_or_m9_retired_provenance() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN04-R-P-STALE");
    let entry = find_transition_entry(row, "SCN04-R-P-STALE", "membership.request");
    let context = "SCN04-R-P-STALE authority snapshot naming/provenance";
    let mut failures = Vec::new();

    if row
        .pointer("/runtime_transition_trace/m9_retired_authority_snapshot")
        .is_some()
    {
        require_json_value_pointer(
            row,
            "/runtime_transition_trace/m9_retired_authority_snapshot/provenance/source_layer",
            json!("M9"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            row,
            "/runtime_transition_trace/m9_retired_authority_snapshot/provenance/source_accessor",
            json!("M10MembershipLifecycleSession::retired_authority_details"),
            context,
            &mut failures,
        );
        require_absent_or_null_pointer(
            row,
            "/runtime_transition_trace/m8_authority_snapshot",
            context,
            &mut failures,
        );
        require_absent_or_null_pointer(
            entry,
            "/after/m8_authority_snapshot",
            context,
            &mut failures,
        );
    } else {
        for pointer in [
            "/after/authority_snapshot/membership/status",
            "/after/authority_snapshot/capability/status",
            "/after/authority_snapshot/witness/status",
            "/after/authority_snapshot/provenance/source_layer",
            "/after/authority_snapshot/provenance/source_accessor",
            "/after/authority_snapshot/provenance/derived_from_actual_m8_inventory",
        ] {
            require_pointer(entry, pointer, context, &mut failures);
        }
        require_json_value_pointer(
            entry,
            "/after/authority_snapshot/provenance/source_layer",
            json!("M8"),
            context,
            &mut failures,
        );
        require_json_value_pointer(
            entry,
            "/after/authority_snapshot/provenance/source_accessor",
            json!("M8AuthorityInventory::snapshot"),
            context,
            &mut failures,
        );
        require_bool_pointer(
            entry,
            "/after/authority_snapshot/provenance/derived_from_actual_m8_inventory",
            true,
            context,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "authority snapshot evidence must be named/provenanced as actual M8 inventory, or renamed to m9_retired_authority_snapshot when it is the retired M9 authority view:\n{}",
        failures.join("\n")
    );
}

#[test]
fn receipt_action_name_substitution_cannot_pass_by_rebinding_profile_to_generated_evidence() {
    let substituted_schedule = schedule_with_case_id_replaced(
        "SCN04.leave.attack_stale",
        "SCN04.leave.attack_stale.substituted_receipt_name",
    );
    let first = run_conformance_with_schedule_and_predicates(
        substituted_schedule.clone(),
        predicate_profile(),
    );
    assert_pointer(
        &first,
        "/verification/terminal_outcome",
        json!("PredicateMismatch"),
    );
    assert_pointer(
        &first,
        "/verification/mismatches/0/predicate_id",
        json!("SCN04-R-P-STALE"),
    );

    let rebound_profile = profile_rebound_to_actual_evidence(&first);
    let rebound =
        run_conformance_with_schedule_and_predicates(substituted_schedule, rebound_profile);
    let outcome = rebound.pointer("/terminal_outcome");
    assert_eq!(
        outcome,
        Some(&json!("ConformanceFailure")),
        "renaming only an action/receipt id and rebinding the profile to generated evidence must still fail; current verifier accepted name-substituted evidence with outcome={outcome:?}"
    );
}

#[test]
fn release_manifest_fail_closed_against_rebound_identity_and_content_mutations() {
    let mut accepted_rebound_mutations = Vec::new();
    for (original, replacement) in [
        ("SCN08.live", "SCN08.live.manifest_renamed"),
        ("SCN10.save_s1", "SCN10.save_s1.manifest_renamed"),
    ] {
        let schedule = schedule_with_case_id_replaced(original, replacement);
        let mismatch =
            run_conformance_with_schedule_and_predicates(schedule.clone(), predicate_profile());
        let rebound = run_conformance_with_schedule_and_predicates(
            schedule,
            profile_rebound_to_actual_evidence(&mismatch),
        );
        if rebound.pointer("/terminal_outcome") == Some(&json!("ConformanceAccepted")) {
            accepted_rebound_mutations.push(format!("{original} -> {replacement}"));
        }
    }

    let baseline = run_conformance();
    let mut failures = Vec::new();
    if !accepted_rebound_mutations.is_empty() {
        failures.push(format!(
            "release manifest must reject action identity/content mutations even after generated evidence/profile rebound; accepted {accepted_rebound_mutations:?}"
        ));
    }
    for pointer in [
        "/release_manifest/profile_schema_version",
        "/release_manifest/source_revision/base_identity",
        "/release_manifest/source_content_identities",
        "/release_manifest/typed_carriers_identity",
        "/release_manifest/schedule_identity",
        "/release_manifest/correspondence_profile_identity",
        "/release_manifest/action_inventory",
        "/release_manifest/fail_closed_checks/missing_manifest",
        "/release_manifest/fail_closed_checks/unknown_extra_input",
        "/release_manifest/fail_closed_checks/revision_mismatch",
    ] {
        require_pointer(&baseline, pointer, "release manifest", &mut failures);
    }
    let schedule = action_schedule();
    let expected_action_count = schedule
        .pointer("/cases")
        .and_then(Value::as_array)
        .map(Vec::len)
        .expect("typed schedule keeps action cases");
    let manifest_action_count = baseline
        .pointer("/release_manifest/action_inventory")
        .and_then(Value::as_array)
        .map(Vec::len);
    if manifest_action_count != Some(expected_action_count) {
        failures.push(format!(
            "release manifest action inventory must bind all {expected_action_count} schedule actions, got {manifest_action_count:?}"
        ));
    }

    assert!(
        failures.is_empty(),
        "M10 release manifest must fail closed across source/carrier/schedule/profile identity mutations:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_second_release_anchor_checks_are_fixed_outcomes_not_literal_bools() {
    let baseline = run_conformance();
    let substituted_schedule = schedule_with_case_id_replaced(
        "SCN08.live",
        "SCN08.live.anchor_profile_schedule_co_mutated",
    );
    let mismatch = run_conformance_with_schedule_and_predicates(
        substituted_schedule.clone(),
        predicate_profile(),
    );
    let rebound = run_conformance_with_schedule_and_predicates(
        substituted_schedule,
        profile_rebound_to_actual_evidence(&mismatch),
    );
    let mut failures = Vec::new();

    for pointer in [
        "/release_manifest/anchor/expected_source_revision",
        "/release_manifest/anchor/expected_execution_identity",
        "/release_manifest/anchor/expected_manifest_hash",
        "/release_manifest/anchor/expected_verifier_profile_hash",
    ] {
        require_pointer(&baseline, pointer, "release anchor", &mut failures);
    }
    if baseline.pointer("/release_manifest/anchor") != rebound.pointer("/release_manifest/anchor") {
        failures.push(format!(
            "release anchor must remain fixed under live schedule/profile co-mutation; baseline={:?}, rebound={:?}",
            baseline.pointer("/release_manifest/anchor"),
            rebound.pointer("/release_manifest/anchor")
        ));
    }
    if rebound.pointer("/terminal_outcome") == Some(&json!("ConformanceAccepted")) {
        failures.push(
            "schedule/profile co-mutation must fail closed even after rebinding the profile to generated evidence"
                .to_string(),
        );
    }
    if rebound.pointer("/verification/terminal_outcome")
        != Some(&json!("FrozenReleaseManifestMismatch"))
    {
        failures.push(format!(
            "schedule/profile co-mutation must report FrozenReleaseManifestMismatch, got {:?}",
            rebound.pointer("/verification/terminal_outcome")
        ));
    }
    for check in [
        "missing_manifest",
        "unknown_extra_input",
        "revision_mismatch",
    ] {
        require_fail_closed_outcome_evidence(&baseline, check, &mut failures);
    }

    assert!(
        failures.is_empty(),
        "M10 release anchor must be fixed independently of live source/profile/schedule mutations, and fail_closed_checks must be observed outcomes rather than literal true values:\n{}",
        failures.join("\n")
    );
}

#[test]
fn m8_authority_refs_must_losslessly_match_m9_canonical_snapshot_without_direct_mint() {
    let report = run_conformance();
    let mut failures = Vec::new();

    for (context, value) in [
        (
            "SCN11 designated version",
            report
                .pointer("/pressure/SCN-11/designated_version")
                .expect("SCN11 designated row exists"),
        ),
        (
            "SCN08 live fallback",
            inventory_row(&report, "SCN08-R-P-LIVE"),
        ),
    ] {
        for pointer in [
            "/m9_to_m8_authority_translation/m9_snapshot/source_ref",
            "/m9_to_m8_authority_translation/m9_snapshot/trace_range",
            "/m9_to_m8_authority_translation/m9_snapshot/active_membership_ref",
            "/m9_to_m8_authority_translation/m9_snapshot/active_capability_ref",
            "/m9_to_m8_authority_translation/m9_snapshot/active_witness_ref",
            "/m9_to_m8_authority_translation/m8_authority_use/membership_ref",
            "/m9_to_m8_authority_translation/m8_authority_use/capability_ref",
            "/m9_to_m8_authority_translation/m8_authority_use/witness_ref",
            "/m9_to_m8_authority_translation/lossless_exact_match",
            "/direct_m10_already_admitted_authority_ref_rejected",
            "/direct_m10_lease_ref_rejected",
        ] {
            require_pointer(value, pointer, context, &mut failures);
        }
        if value.pointer("/m9_to_m8_authority_translation/lossless_exact_match")
            != Some(&json!(true))
        {
            failures.push(format!(
                "{context} must prove exact M9 snapshot -> M8 authority ref match"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "SCN08/SCN11 must prove no direct M10 authority mint and lossless M9->M8 authority translation:\n{}",
        failures.join("\n")
    );
}

#[test]
fn persistent_session_families_reject_reordered_or_name_only_predecessor_actions() {
    let reordered =
        schedule_with_case_moved_before("SCN10.leave_a.lease_expiry.save_s2", "SCN10.save_s1");
    let reordered_report =
        run_conformance_with_schedule_and_predicates(reordered, predicate_profile());
    assert_ne!(
        reordered_report.pointer("/terminal_outcome"),
        Some(&json!("ConformanceAccepted")),
        "SCN10 S2 moved before S1 must not pass; facts must depend on same-session predecessor execution"
    );
    let reordered_row = inventory_row(&reordered_report, "SCN10-R-N-MERGE");
    assert_eq!(
        reordered_row.pointer("/result"),
        Some(&json!("fail")),
        "SCN10-R-N-MERGE must fail when reordered S2 has no honest S1 predecessor evidence: {reordered_row:#}"
    );
    assert_eq!(
        reordered_row.pointer("/actual_evidence"),
        Some(&json!([])),
        "SCN10-R-N-MERGE reordered path must expose no actual evidence candidates rather than a fabricated same-predicate candidate: {reordered_row:#}"
    );
    assert_eq!(
        reordered_row.pointer("/fail_diagnostic/code"),
        Some(&json!("CorrespondenceEvidenceMismatch")),
        "SCN10-R-N-MERGE reordered path must expose a correspondence mismatch/fail diagnostic: {reordered_row:#}"
    );

    let report = run_conformance();
    let mut failures = Vec::new();
    for (expectation_id, pointers) in [
        (
            "SCN04-R-P-REJOIN",
            &[
                "/runtime_transition_trace/session_id",
                "/runtime_transition_trace/monotone_trace_range",
                "/runtime_transition_trace/old_authority_unusable",
            ][..],
        ),
        (
            "SCN05-R-P-HANDOFF",
            &[
                "/runtime_transition_trace/session_id",
                "/runtime_transition_trace/dependencies/leave_a_before_join_b",
                "/runtime_transition_trace/dependencies/join_b_before_spawn_b",
            ][..],
        ),
        (
            "SCN10-R-P-S2",
            &[
                "/runtime_transition_trace/session_id",
                "/runtime_transition_trace/predecessors/S1",
                "/runtime_transition_trace/deltas/leave",
                "/runtime_transition_trace/deltas/lease_expiry",
            ][..],
        ),
        (
            "SCN10-R-P-TIMELINE",
            &[
                "/runtime_transition_trace/receipt_origin",
                "/runtime_transition_trace/occurrence_range",
            ][..],
        ),
        (
            "SCN10-R-P-REACQUIRE",
            &[
                "/runtime_transition_trace/receipt_origin",
                "/runtime_transition_trace/occurrence_range",
            ][..],
        ),
    ] {
        let row = inventory_row(&report, expectation_id);
        for pointer in pointers {
            require_pointer(row, pointer, expectation_id, &mut failures);
        }
    }

    assert!(
        failures.is_empty(),
        "M10 runtime families must share persistent sessions with monotone trace ranges and predecessor-derived facts:\n{}",
        failures.join("\n")
    );
}

#[test]
fn domain_native_hashes_change_only_their_actual_projection_domain() {
    let report = run_conformance();

    assert_changed_hash_keys(
        find_transition_entry(
            inventory_row(&report, "SCN01-R-P-STATE"),
            "SCN01-R-P-STATE",
            "owner.write",
        ),
        "SCN01-R-P-STATE",
        &["store_hash"],
    );
    let scn04_stale = inventory_row(&report, "SCN04-R-P-STALE");
    let scn04_entries = transition_entries(scn04_stale, "SCN04-R-P-STALE");
    let retire_index = scn04_entries
        .iter()
        .position(|entry| entry.get("transition").and_then(Value::as_str) == Some("membership.retire"))
        .unwrap_or_else(|| {
            panic!("SCN04-R-P-STALE must include accepted membership.retire before stale request: {scn04_stale:#}")
        });
    let request_index = scn04_entries
        .iter()
        .position(|entry| entry.get("transition").and_then(Value::as_str) == Some("membership.request"))
        .unwrap_or_else(|| {
            panic!("SCN04-R-P-STALE must include rejected membership.request after retire: {scn04_stale:#}")
        });
    assert!(
        retire_index < request_index,
        "SCN04-R-P-STALE must retire membership before the stale request: {scn04_stale:#}"
    );
    let retire = &scn04_entries[retire_index];
    let request = &scn04_entries[request_index];
    assert_eq!(
        retire.pointer("/accepted"),
        Some(&json!(true)),
        "SCN04 membership.retire must be the accepted semantic mutation: {retire:#}"
    );
    assert_eq!(
        request.pointer("/accepted"),
        Some(&json!(false)),
        "SCN04 membership.request must be the rejected stale access: {request:#}"
    );
    assert_changed_hash_keys(
        retire,
        "SCN04-R-P-STALE membership.retire",
        &["membership_hash", "grant_hash"],
    );
    assert_changed_hash_keys(request, "SCN04-R-P-STALE membership.request", &[]);
    let mut scn04_session_failures = Vec::new();
    require_persistent_semantic_session(
        scn04_stale,
        "SCN04-R-P-STALE",
        "M9",
        &mut scn04_session_failures,
    );
    let row_session = scn04_stale
        .pointer("/runtime_transition_trace/session_id")
        .and_then(Value::as_str)
        .expect("SCN04-R-P-STALE must expose a persistent M9 session id");
    for (transition, entry) in [
        ("membership.retire", retire),
        ("membership.request", request),
    ] {
        let entry_session = entry
            .pointer("/session_id")
            .and_then(Value::as_str)
            .unwrap_or(row_session);
        if entry_session != row_session {
            scn04_session_failures.push(format!(
                "SCN04 {transition} must use the same persistent M9 session as the row; entry_session={entry_session:?}, row_session={row_session:?}"
            ));
        }
    }
    assert!(
        scn04_session_failures.is_empty(),
        "SCN04 retire/request transitions must be ordered in the same persistent M9 session, not M10ScenarioState:\n{}",
        scn04_session_failures.join("\n")
    );
    assert_changed_hash_keys(
        find_transition_entry(
            inventory_row(&report, "SCN08-R-P-EXPIRE"),
            "SCN08-R-P-EXPIRE",
            "fallback.advance",
        ),
        "SCN08-R-P-EXPIRE",
        &["relation_hash"],
    );
    assert_changed_hash_keys(
        find_transition_entry(
            inventory_row(&report, "SCN09-R-P-INIT"),
            "SCN09-R-P-INIT",
            "patch.activate",
        ),
        "SCN09-R-P-INIT",
        &["store_hash", "config_hash"],
    );

    let scn09_drift = inventory_row(&report, "SCN09-R-N-DRIFT");
    let scn09_entries = transition_entries(scn09_drift, "SCN09-R-N-DRIFT");
    let retire_index = scn09_entries
        .iter()
        .position(|entry| {
            entry.get("transition").and_then(Value::as_str) == Some("membership.retire")
        })
        .unwrap_or_else(|| {
            panic!("SCN09-R-N-DRIFT must include accepted M9 membership.retire/frontier drift before patch activation: {scn09_drift:#}")
        });
    let activate_index = scn09_entries
        .iter()
        .position(|entry| entry.get("transition").and_then(Value::as_str) == Some("patch.activate"))
        .unwrap_or_else(|| {
            panic!("SCN09-R-N-DRIFT must include rejected patch.activate after M9 frontier drift: {scn09_drift:#}")
        });
    assert!(
        retire_index < activate_index,
        "SCN09-R-N-DRIFT must retire M9 membership/frontier before rejected patch.activate: {scn09_drift:#}"
    );
    let retire = &scn09_entries[retire_index];
    let activate = &scn09_entries[activate_index];
    assert_eq!(
        retire.pointer("/accepted"),
        Some(&json!(true)),
        "SCN09 membership.retire/frontier drift must be the accepted semantic mutation: {retire:#}"
    );
    assert_eq!(
        activate.pointer("/accepted"),
        Some(&json!(false)),
        "SCN09 patch.activate must be rejected after membership frontier drift: {activate:#}"
    );
    assert_changed_hash_keys(
        retire,
        "SCN09-R-N-DRIFT membership.retire",
        &["membership_hash", "grant_hash"],
    );
    assert_changed_hash_keys(activate, "SCN09-R-N-DRIFT patch.activate", &[]);
    let mut scn09_session_failures = Vec::new();
    require_persistent_semantic_session(
        scn09_drift,
        "SCN09-R-N-DRIFT",
        "M9",
        &mut scn09_session_failures,
    );
    let row_session = scn09_drift
        .pointer("/runtime_transition_trace/session_id")
        .and_then(Value::as_str)
        .expect("SCN09-R-N-DRIFT must expose a persistent M9 patch session id");
    if !row_session.contains("patch") {
        scn09_session_failures.push(format!(
            "SCN09-R-N-DRIFT session must be a persistent patch session, got {row_session:?}"
        ));
    }
    for (transition, entry) in [("membership.retire", retire), ("patch.activate", activate)] {
        let entry_session = entry
            .pointer("/session_id")
            .and_then(Value::as_str)
            .unwrap_or(row_session);
        if entry_session != row_session {
            scn09_session_failures.push(format!(
                "SCN09 {transition} must use the same persistent patch session as the row; entry_session={entry_session:?}, row_session={row_session:?}"
            ));
        }
    }
    assert!(
        scn09_session_failures.is_empty(),
        "SCN09 drift/activate transitions must be ordered in the same persistent M9 patch session:\n{}",
        scn09_session_failures.join("\n")
    );
    assert!(
        scn09_drift
            .pointer("/runtime_transition_trace/m8_patch_activation/activation_cut")
            .is_none_or(Value::is_null),
        "SCN09-R-N-DRIFT must not expose an activation cut after membership frontier drift: {scn09_drift:#}"
    );

    for (expectation_id, transition) in [
        ("SCN08-R-N-REPROMOTE", "fallback.advance"),
        ("SCN10-R-N-MERGE", "cut.restore"),
    ] {
        assert_changed_hash_keys(
            find_transition_entry(
                inventory_row(&report, expectation_id),
                expectation_id,
                transition,
            ),
            expectation_id,
            &[],
        );
    }
}

#[test]
fn runtime_transition_receipts_expose_distinct_domain_projection_provenance() {
    let report = run_conformance();
    let rows = report
        .pointer("/verification/inventory/rows")
        .and_then(Value::as_array)
        .expect("verification inventory exposes rows");
    let mut failures = Vec::new();

    for row in rows
        .iter()
        .filter(|row| row.get("phase").and_then(Value::as_str) == Some("runtime"))
    {
        let expectation_id = row
            .get("expectation_id")
            .and_then(Value::as_str)
            .unwrap_or("<missing expectation_id>");
        let Some(entries) = row
            .pointer("/runtime_transition_trace/transition_trace")
            .and_then(Value::as_array)
        else {
            continue;
        };
        for entry in entries {
            let transition = entry
                .get("transition")
                .and_then(Value::as_str)
                .unwrap_or("<missing transition>");
            require_domain_projection_provenance(entry, expectation_id, transition, &mut failures);
        }
    }

    assert!(
        failures.is_empty(),
        "runtime receipts must expose domain_projection_provenance with distinct actual accessors/components and before/after projection identities for store, membership, grant, relation, and config:\n{}",
        failures.join("\n")
    );
}

#[test]
fn runtime_domain_delta_probes_are_independent_per_projection_domain() {
    let mut failures = Vec::new();
    for (domain, hash_key, source) in [
        ("store", "store_hash", "scn-01/positive.mir"),
        ("membership", "membership_hash", "scn-04/positive.mir"),
        ("grant", "grant_hash", "scn-03/positive.mir"),
        ("relation", "relation_hash", "scn-08/positive.mir"),
        ("config", "config_hash", "scn-09/candidate-accepted.mir"),
    ] {
        let report = match runtime_domain_delta_probe(domain, source) {
            Ok(report) => report,
            Err(error) => {
                failures.push(format!(
                    "{domain}-only runtime projection delta probe did not produce an actual probe report: {error}"
                ));
                continue;
            }
        };
        if report.pointer("/projection_delta_probe/domain") != Some(&json!(domain)) {
            failures.push(format!(
                "{domain}-only probe must report its target domain; report={report:#}"
            ));
        }
        if report.pointer("/projection_delta_probe/mutation_applied_to_actual_runtime")
            != Some(&json!(true))
        {
            failures.push(format!(
                "{domain}-only probe must execute against the actual runtime projection, not a source-text scan or name shortcut"
            ));
        }
        if report.pointer("/projection_delta_probe/changed_hash_keys") != Some(&json!([hash_key])) {
            failures.push(format!(
                "{domain}-only probe must change exactly {hash_key}; got {:?}",
                report.pointer("/projection_delta_probe/changed_hash_keys")
            ));
        }
        let provenance = report
            .pointer(&format!(
                "/projection_delta_probe/domain_projection_provenance/{domain}"
            ))
            .unwrap_or(&Value::Null);
        for suffix in [
            "actual_accessor",
            "component",
            "before_projection_identity",
            "after_projection_identity",
        ] {
            if provenance.get(suffix).is_none() {
                failures.push(format!(
                    "{domain}-only probe missing {suffix} in domain projection provenance: {provenance:#}"
                ));
            }
        }
        if provenance.get("before_projection_identity")
            == provenance.get("after_projection_identity")
        {
            failures.push(format!(
                "{domain}-only probe must mutate the {domain} projection identity: {provenance:#}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "independent runtime delta probes must prove store/membership/grant/relation/config hashes are produced from separate actual projections:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn04_08_10_semantic_state_owner_is_persistent_m9_or_m8_session() {
    let report = run_conformance();
    let mut failures = Vec::new();

    for expectation_id in [
        "SCN04-R-P-STALE",
        "SCN04-R-P-REJOIN",
        "SCN04-R-N-HIDDEN-REPAIR",
    ] {
        require_persistent_semantic_session(
            inventory_row(&report, expectation_id),
            expectation_id,
            "M9",
            &mut failures,
        );
    }
    for expectation_id in [
        "SCN08-R-P-LIVE",
        "SCN08-R-P-EXPIRE",
        "SCN08-R-P-REACQUIRE",
        "SCN08-R-N-REPROMOTE",
        "SCN10-R-P-S1",
        "SCN10-R-P-S2",
        "SCN10-R-P-TIMELINE",
        "SCN10-R-P-REACQUIRE",
        "SCN10-R-N-MERGE",
        "SCN10-R-N-LEASEDOCTOR",
        "SCN10-R-N-CUTDOCTOR",
    ] {
        require_persistent_semantic_session(
            inventory_row(&report, expectation_id),
            expectation_id,
            "M8",
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "SCN04/08/10 semantic state must be owned by persistent M9/M8 sessions, never by M10ScenarioState or m10-* session labels:\n{}",
        failures.join("\n")
    );
}

#[test]
fn all_typed_mutations_require_actual_clone_payload_validator_trace_span_and_snapshot() {
    let mut failures = Vec::new();
    for fault in [
        "construct_deletion_visible_field",
        "m7_bypass_artifact_mismatch",
        "negative_source_positive_core_attachment",
        "failure_no_mutation",
        "projection_history_origin_redaction_violation",
        "restore_stale_membership_resurrection",
        "deterministic_replay_drift",
    ] {
        let report = falsifier_report(fault);
        for pointer in [
            "/mutation_application/mutated_clone/payload",
            "/mutation_application/mutated_clone/before_identity",
            "/mutation_application/mutated_clone/after_identity",
            "/validation/actual_validator_trace/0/component",
            "/validation/actual_validator_trace/0/source_ref",
            "/diagnostics/0/source_span",
            "/runtime/actual_snapshot_before_failure",
            "/runtime/actual_snapshot_after_failure",
        ] {
            require_pointer(&report, pointer, fault, &mut failures);
        }
        if report.pointer("/runtime/actual_snapshot_before_failure")
            != report.pointer("/runtime/actual_snapshot_after_failure")
        {
            failures.push(format!(
                "{fault}: rejected mutation must keep actual runtime snapshot unchanged"
            ));
        }
        if report.pointer("/mutation_application/mutated_clone/before_identity")
            == report.pointer("/mutation_application/mutated_clone/after_identity")
        {
            failures.push(format!(
                "{fault}: mutated clone identity must change, stable_hash label alone is insufficient"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "all typed falsifiers must report actual mutated clone payload, validator trace, diagnostic span, and zero-mutation snapshot:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_falsifiers_require_stage_specific_validator_results_not_synthetic_comment_clones() {
    let mut failures = Vec::new();
    for (fault, validator, specific_state_pointers) in [
        (
            "construct_deletion_visible_field",
            "m6_m7_recheck",
            &[
                "/validation/validator_state/m6_m7_recheck/deleted_construct_ref",
                "/validation/validator_state/m6_m7_recheck/parse_before/result",
                "/validation/validator_state/m6_m7_recheck/parse_after/result",
                "/validation/validator_state/m6_m7_recheck/check_before/result",
                "/validation/validator_state/m6_m7_recheck/check_after/result",
            ][..],
        ),
        (
            "m7_bypass_artifact_mismatch",
            "checked_artifact_validator",
            &[
                "/validation/validator_state/checked_artifact_validator/source_identity",
                "/validation/validator_state/checked_artifact_validator/attached_artifact_source_identity",
                "/validation/validator_state/checked_artifact_validator/expected_checked_identity",
                "/validation/validator_state/checked_artifact_validator/actual_checked_identity",
            ][..],
        ),
        (
            "negative_source_positive_core_attachment",
            "terminal_identity_validator",
            &[
                "/validation/validator_state/terminal_identity_validator/negative_terminal_identity",
                "/validation/validator_state/terminal_identity_validator/attempted_core_identity",
                "/validation/validator_state/terminal_identity_validator/core_attached",
                "/validation/validator_state/terminal_identity_validator/source_terminal",
            ][..],
        ),
        (
            "failure_no_mutation",
            "mutation_guard",
            &[
                "/validation/validator_state/mutation_guard/rejected_transition",
                "/validation/validator_state/mutation_guard/runtime_input_before_identity",
                "/validation/validator_state/mutation_guard/runtime_input_after_identity",
                "/validation/validator_state/mutation_guard/mutation_prevented_at_boundary",
            ][..],
        ),
        (
            "projection_history_origin_redaction_violation",
            "projection_history_validator",
            &[
                "/validation/validator_state/projection_history_validator/projection_row_identity",
                "/validation/validator_state/projection_history_validator/origin_ref",
                "/validation/validator_state/projection_history_validator/redaction_ref",
                "/validation/validator_state/projection_history_validator/publication_emitted",
            ][..],
        ),
        (
            "restore_stale_membership_resurrection",
            "restore_cut_validator",
            &[
                "/validation/validator_state/restore_cut_validator/save_identity",
                "/validation/validator_state/restore_cut_validator/current_membership_identity",
                "/validation/validator_state/restore_cut_validator/restored_membership_identity",
                "/validation/validator_state/restore_cut_validator/merge_rejected",
            ][..],
        ),
        (
            "deterministic_replay_drift",
            "deterministic_replay_validator",
            &[
                "/validation/validator_state/deterministic_replay_validator/baseline_replay_identity",
                "/validation/validator_state/deterministic_replay_validator/mutated_replay_identity",
                "/validation/validator_state/deterministic_replay_validator/replay_equal",
                "/validation/validator_state/deterministic_replay_validator/divergence_trace",
            ][..],
        ),
    ] {
        let report = falsifier_report(fault);
        require_stage_specific_mutation_evidence(
            &report,
            fault,
            validator,
            specific_state_pointers,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "P0 typed falsifiers must expose actual parsed/checked/runtime input identities, validator-specific state/results, and exact no-mutation boundaries, not synthetic comment clones or mapped validator labels:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_second_scn04_09_10_share_exact_m9_to_m8_lineage_for_post_mutation_decisions() {
    let report = run_conformance();
    let mut failures = Vec::new();

    for (expectation_id, semantic_transition, m8_decision_transition) in [
        ("SCN04-R-P-STALE", "membership.retire", "membership.request"),
        ("SCN09-R-N-DRIFT", "membership.retire", "patch.activate"),
        ("SCN10-R-N-MERGE", "m9.cut.restore", "cut.restore"),
        ("SCN10-R-N-LEASEDOCTOR", "m9.cut.restore", "cut.restore"),
        ("SCN10-R-N-CUTDOCTOR", "m9.cut.restore", "cut.restore"),
    ] {
        require_m9_m8_lineage_decision_after_semantic_mutation(
            inventory_row(&report, expectation_id),
            expectation_id,
            semantic_transition,
            m8_decision_transition,
            &mut failures,
        );
    }

    assert!(
        failures.is_empty(),
        "SCN04/09/10 must bind each M9 semantic mutation and the following M8 runtime decision to the same explicit M9->M8 authority lineage/session:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_second_scn06_route_patch_reuses_persistent_runtime_contract_for_postpatch_owner_request() {
    let report = run_conformance();
    let row = inventory_row(&report, "SCN06-R-P-PATCHED");
    let trace = row
        .pointer("/runtime_transition_trace")
        .expect("SCN06-R-P-PATCHED exposes runtime_transition_trace");
    let mut failures = Vec::new();

    for pointer in [
        "/persistent_execution_runtime/session_id",
        "/persistent_execution_runtime/contract_identity",
        "/route_patch/runtime_session_id",
        "/route_patch/contract_before",
        "/route_patch/contract_after",
        "/route_patch/accepted_patch_identity",
        "/postpatch_owner_request/runtime_session_id",
        "/postpatch_owner_request/contract_ref",
        "/postpatch_owner_request/decision",
        "/postpatch_owner_request/source_transition",
    ] {
        require_pointer(
            trace,
            pointer,
            "SCN06 persistent route patch runtime",
            &mut failures,
        );
    }
    require_json_pointer_equal(
        trace,
        "/persistent_execution_runtime/session_id",
        "/route_patch/runtime_session_id",
        "SCN06 route patch runtime session",
        &mut failures,
    );
    require_json_pointer_equal(
        trace,
        "/persistent_execution_runtime/session_id",
        "/postpatch_owner_request/runtime_session_id",
        "SCN06 postpatch owner request runtime session",
        &mut failures,
    );
    require_json_pointer_equal(
        trace,
        "/route_patch/contract_after",
        "/postpatch_owner_request/contract_ref",
        "SCN06 postpatch owner request contract",
        &mut failures,
    );
    if trace.pointer("/route_patch/contract_before") == trace.pointer("/route_patch/contract_after")
    {
        failures.push("SCN06 route patch must change the persistent runtime contract".to_string());
    }
    if trace.pointer("/postpatch_owner_request/decision") != Some(&json!("accepted")) {
        failures.push(format!(
            "SCN06 postpatch owner request must be an actual accepted M8 decision, got {:?}",
            trace.pointer("/postpatch_owner_request/decision")
        ));
    }
    require_bool_pointer(
        trace,
        "/fresh_runtime_created",
        false,
        "SCN06 route patch runtime reuse",
        &mut failures,
    );
    require_bool_pointer(
        trace,
        "/bool_only_facade_used",
        false,
        "SCN06 route patch runtime reuse",
        &mut failures,
    );

    assert!(
        failures.is_empty(),
        "SCN06 route patch must mutate the same persistent execution runtime contract consumed by the postpatch owner request, not expose a bool-only facade or fresh runtime:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_m10_authority_lineage_uses_m9_module_bridge_not_direct_m8_record_minting() {
    let mut failures = Vec::new();
    let forbidden_calls = [
        "M8MembershipRecord::already_admitted",
        "M8CapabilityGrant::already_admitted",
        "M8WitnessRecord::live",
    ];
    let source_paths = m10_production_source_paths();
    assert!(
        !source_paths.is_empty(),
        "M10 production source scan must include at least one bounded source path"
    );
    for path in source_paths {
        let text = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("can read M10 production source {}: {error}", path.display())
        });
        for forbidden in forbidden_calls {
            for (line_index, line) in text.lines().enumerate() {
                if line.contains(forbidden) {
                    failures.push(format!(
                        "M10 production source {}:{} directly calls {forbidden}; M8 authority records must be issued by the M9 module bridge",
                        path.display(),
                        line_index + 1
                    ));
                }
            }
        }
    }

    let report = run_conformance();
    let mut lineage_count = 0_usize;
    walk_json_objects(&report, &mut |object| {
        for key in ["m9_to_m8_authority_lineage"] {
            let Some(lineage) = object.get(key) else {
                continue;
            };
            lineage_count += 1;
            let context = format!("reported {key} #{lineage_count}");
            let issuer = require_any_pointer(
                lineage,
                &[
                    "/authority_issuer",
                    "/issuer",
                    "/bridge_issuer",
                    "/provenance/issuer",
                ],
                &context,
                &mut failures,
            )
            .map(|(_, value)| value.clone());
            let provenance = require_any_pointer(
                lineage,
                &[
                    "/authority_bridge_provenance",
                    "/provenance",
                    "/bridge_provenance",
                    "/m9_bridge_provenance",
                ],
                &context,
                &mut failures,
            )
            .map(|(_, value)| value.clone());
            if let Some(value) = issuer {
                let text = serde_json::to_string(&value).expect("lineage issuer serializes");
                if !text.contains("M9") {
                    failures.push(format!("{context} issuer must name M9, got {value:#}"));
                }
            }
            if let Some(value) = provenance {
                let text = serde_json::to_string(&value).expect("lineage provenance serializes");
                if !text.contains("crate::m9_auth_verification::M9M10AuthorityBridge") {
                    failures.push(format!(
                        "{context} provenance must name crate::m9_auth_verification::M9M10AuthorityBridge, got {value:#}"
                    ));
                }
            }
        }
    });
    if lineage_count == 0 {
        failures.push(
            "M10 report must expose at least one M9->M8 authority lineage or translation object"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "M10 must not directly mint M8 authority records; every reported M9->M8 lineage must name the M9 module bridge as issuer/provenance:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p0_scn12_relation_projection_schedule_must_not_execute_discarded_per_action_m8_runtimes() {
    let mut failures = Vec::new();
    let source_path = workspace_root().join("crates/mir-runtime/src/m10_reference_system.rs");
    let source_text = fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("can read {}: {error}", source_path.display()));
    let branch_start = source_text
        .find("M10ScheduleOperation::RelationProjection")
        .expect("M10 schedule relation projection branch exists");
    let branch_end_marker =
        "if let Some(checked) = checked_sources.get(\"scn-12/bird-relation.mir\")";
    let branch_end = source_text[branch_start..]
        .find(branch_end_marker)
        .map(|offset| branch_start + offset)
        .expect("SCN12 persistent session projection follows schedule branch");
    let relation_projection_branch = &source_text[branch_start..branch_end];
    for forbidden in [
        "M8LocalRuntime::from_admitted",
        ".project_relation(",
        ".entry(\"SCN-12\"",
        "pressure.entry(\"SCN-12\"",
    ] {
        if relation_projection_branch.contains(forbidden) {
            failures.push(format!(
                "SCN12 schedule RelationProjection branch in {} must not instantiate/execute discarded per-action M8 runtime path before the persistent session; found {forbidden}",
                source_path.display()
            ));
        }
    }
    if relation_projection_branch.contains("SCN-12")
        && source_text[branch_end..].contains("m10_run_scn12_relation_session")
    {
        failures.push(
            "SCN12 schedule RelationProjection branch must not write a per-action SCN-12 pressure row and then overwrite it with the persistent session"
                .to_string(),
        );
    }

    match run_conformance_result() {
        Ok(report) => {
            let scn12 = report
                .pointer("/pressure/SCN-12")
                .expect("SCN-12 pressure section exists");
            let context = "SCN12 persistent relation projection session";
            for pointer in [
                "/execution_session/session_id",
                "/execution_session/action_receipts",
                "/execution_session/reused_across_actions",
                "/execution_session/discarded_isolated_per_action_execution",
                "/execution_session/relation_projection_schedule_branch_executed",
                "/execution_session/per_action_m8_runtime_count",
                "/execution_session/discarded_m8_runtime_count",
                "/execution_session/persistent_session_started_before_schedule_actions",
            ] {
                require_pointer(scn12, pointer, context, &mut failures);
            }
            require_bool_pointer(
                scn12,
                "/execution_session/reused_across_actions",
                true,
                context,
                &mut failures,
            );
            require_bool_pointer(
                scn12,
                "/execution_session/discarded_isolated_per_action_execution",
                false,
                context,
                &mut failures,
            );
            require_bool_pointer(
                scn12,
                "/execution_session/relation_projection_schedule_branch_executed",
                false,
                context,
                &mut failures,
            );
            require_json_value_pointer(
                scn12,
                "/execution_session/per_action_m8_runtime_count",
                json!(0),
                context,
                &mut failures,
            );
            require_json_value_pointer(
                scn12,
                "/execution_session/discarded_m8_runtime_count",
                json!(0),
                context,
                &mut failures,
            );
            require_bool_pointer(
                scn12,
                "/execution_session/persistent_session_started_before_schedule_actions",
                true,
                context,
                &mut failures,
            );
            for pointer in [
                "/bird_relation/execution_session_id",
                "/fallback/execution_session_id",
                "/reacquire/execution_session_id",
            ] {
                require_json_pointer_equal(
                    scn12,
                    "/execution_session/session_id",
                    pointer,
                    context,
                    &mut failures,
                );
            }
            if let Some(session_id) = scn12.pointer("/execution_session/session_id").cloned()
                && let Some(receipts) = scn12
                    .pointer("/execution_session/action_receipts")
                    .and_then(Value::as_array)
            {
                for (index, receipt) in receipts.iter().enumerate() {
                    let receipt_context =
                        format!("{context} action_receipts[{index}] must reuse session");
                    require_json_value_pointer(
                        receipt,
                        "/session_id",
                        session_id.clone(),
                        &receipt_context,
                        &mut failures,
                    );
                    require_absent_or_null_pointer(
                        receipt,
                        "/discarded_runtime_id",
                        &receipt_context,
                        &mut failures,
                    );
                    require_absent_or_null_pointer(
                        receipt,
                        "/per_action_runtime_id",
                        &receipt_context,
                        &mut failures,
                    );
                }
            }
        }
        Err(error) => failures.push(format!(
            "M10 conformance report must be generated before SCN12 relation session evidence can be checked: {error}"
        )),
    }

    assert!(
        failures.is_empty(),
        "SCN12 RelationProjection schedule path must not execute discarded per-action M8 runtimes before the one persistent relation session:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p1_second_scn12_and_scn08_negative_use_actual_m8_execution_not_isolated_manual_cursors() {
    let report = run_conformance();
    let mut failures = Vec::new();

    let scn12 = report
        .pointer("/pressure/SCN-12")
        .expect("SCN-12 pressure section exists");
    for pointer in [
        "/execution_session/session_id",
        "/execution_session/action_receipts",
        "/execution_session/reused_across_actions",
        "/execution_session/discarded_isolated_per_action_execution",
    ] {
        require_pointer(
            scn12,
            pointer,
            "SCN12 persistent execution session",
            &mut failures,
        );
    }
    require_bool_pointer(
        scn12,
        "/execution_session/reused_across_actions",
        true,
        "SCN12 persistent execution session",
        &mut failures,
    );
    require_bool_pointer(
        scn12,
        "/execution_session/discarded_isolated_per_action_execution",
        false,
        "SCN12 persistent execution session",
        &mut failures,
    );
    for pointer in [
        "/bird_relation/execution_session_id",
        "/fallback/execution_session_id",
        "/reacquire/execution_session_id",
    ] {
        require_json_pointer_equal(
            scn12,
            "/execution_session/session_id",
            pointer,
            "SCN12 pressure action session reuse",
            &mut failures,
        );
    }

    let scn08 = inventory_row(&report, "SCN08-R-N-REPROMOTE");
    let trace = scn08
        .pointer("/runtime_transition_trace")
        .expect("SCN08-R-N-REPROMOTE exposes runtime_transition_trace");
    for pointer in [
        "/m8_relation_trace/0/runtime_projection_before",
        "/m8_relation_trace/0/runtime_projection_after",
        "/m8_relation_trace/0/relation_projection_trace_ref",
        "/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
        "/m8_relation_trace/0/manual_cursor_used",
    ] {
        require_pointer(
            trace,
            pointer,
            "SCN08 negative M8 relation projection trace",
            &mut failures,
        );
    }
    require_bool_pointer(
        trace,
        "/m8_relation_trace/0/derived_from_actual_m8_relation_projection",
        true,
        "SCN08 negative M8 relation projection trace",
        &mut failures,
    );
    require_bool_pointer(
        trace,
        "/m8_relation_trace/0/manual_cursor_used",
        false,
        "SCN08 negative M8 relation projection trace",
        &mut failures,
    );
    if trace.pointer("/m8_relation_trace/0/runtime_projection_before")
        == trace.pointer("/m8_relation_trace/0/runtime_projection_after")
    {
        failures.push(
            "SCN08 negative must derive the selected-floor rejection from an actual M8 relation projection delta, not a manual cursor"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "SCN12 must not discard isolated per-action execution, and SCN08 negative must be derived from actual M8 relation projection traces rather than a manual cursor:\n{}",
        failures.join("\n")
    );
}

#[test]
fn scn12_relation_session_runs_semantic_fallback_reject_reacquire_without_absolute_stream_split() {
    let report = run_conformance();
    let scn12 = report
        .pointer("/pressure/SCN-12")
        .expect("SCN-12 pressure section exists");
    let mut failures = Vec::new();

    for pointer in [
        "/relation_session_id",
        "/monotone_trace_range",
        "/bird_projection/semantic_relation_delta",
        "/semantic_fallback/selected_floor",
        "/same_lineage_reject/diagnostic",
        "/fresh_reacquire/fresh_epoch",
        "/fresh_reacquire/fresh_witness",
        "/presentation_shortage/semantic_lineage_unchanged",
        "/privacy_join/no_absolute_stream",
        "/privacy_join/no_split_frame",
    ] {
        require_pointer(scn12, pointer, "SCN12 relation session", &mut failures);
    }
    if scn12.pointer("/privacy_join/no_absolute_stream") != Some(&json!(true)) {
        failures.push("SCN12 must not create an independent absolute stream".to_string());
    }
    if scn12.pointer("/privacy_join/no_split_frame") != Some(&json!(true)) {
        failures.push("SCN12 privacy join must reject split-frame leakage".to_string());
    }

    assert!(
        failures.is_empty(),
        "SCN12 must run bird projection -> semantic fallback -> same-lineage reject -> fresh reacquire in one relation session without privacy stream split:\n{}",
        failures.join("\n")
    );
}

#[test]
fn falsifiers_use_typed_input_mutations_real_validators_and_exact_diagnostics() {
    for (fault, diagnostic, validator, source_path) in [
        (
            "source_sensitivity_changed_text_same_name",
            "SourceIdentityMismatch",
            "source_identity_validator",
            "scn-02/positive.mir",
        ),
        (
            "construct_deletion_visible_field",
            "SourceConstructDeleted",
            "m6_m7_recheck",
            "scn-01/positive.mir",
        ),
        (
            "m7_bypass_artifact_mismatch",
            "CheckedArtifactSourceMismatch",
            "checked_artifact_validator",
            "scn-02/positive.mir",
        ),
        (
            "negative_source_positive_core_attachment",
            "RejectedSourceHasCoreArtifact",
            "terminal_identity_validator",
            "scn-01/negative-missing-visibility-denied.mir",
        ),
        (
            "scn09_patch_provenance_mismatch",
            "PatchCarrierCandidateSourceMismatch",
            "patch_carrier_validator",
            "scn-09/candidate-accepted.mir",
        ),
        (
            "failure_no_mutation",
            "RejectedStepAttemptedMutation",
            "mutation_guard",
            "scn-03/negative-write-before-verdict.mir",
        ),
        (
            "fallback_repromotion_without_reacquire",
            "E-LIN-003",
            "fallback_lineage_validator",
            "scn-08/negative-write-after-read-lineage.mir",
        ),
        (
            "projection_history_origin_redaction_violation",
            "ProjectionHistoryOriginRedactionViolation",
            "projection_history_validator",
            "scn-12/bird-relation.mir",
        ),
        (
            "restore_stale_membership_resurrection",
            "RestoreStaleMembershipResurrection",
            "restore_cut_validator",
            "scn-10/negative-stale-restore.mir",
        ),
        (
            "deterministic_replay_drift",
            "DeterministicReplayMismatch",
            "deterministic_replay_validator",
            "scn-02/positive.mir",
        ),
    ] {
        let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
        let report: M10ConformanceReport = system
            .run_conformance(
                request_with_predicates(predicate_profile())
                    .typed_input_mutation(typed_falsifier(fault)),
            )
            .expect("falsifier returns a typed conformance failure report");
        let value = serde_json::to_value(report).expect("falsifier report serializes");

        assert_pointer(&value, "/terminal_outcome", json!("ConformanceFailure"));
        assert_pointer(&value, "/falsifier/name", json!(fault));
        assert_pointer(
            &value,
            "/falsifier/input/schema_version",
            json!("m10-i1plus-source-run-mutation-v0"),
        );
        assert_pointer(&value, "/falsifier/name_driven_terminal_used", json!(false));
        if fault == "fallback_repromotion_without_reacquire" {
            assert_pointer(
                &value,
                "/validation/fallback_lineage_claim_scope",
                json!("typed_carrier_only"),
            );
            assert_pointer(&value, "/validation/real_validator_invoked", json!(false));
        } else {
            assert_pointer(&value, "/validation/real_validator_invoked", json!(true));
        }
        assert_pointer(
            &value,
            &format!("/validation/invocations/{validator}"),
            json!(1),
        );
        assert_pointer(&value, "/diagnostics/0/code", json!(diagnostic));
        assert_pointer(&value, "/diagnostics/0/source_path", json!(source_path));
        assert_pointer(&value, "/waiver_carrier", Value::Null);
        assert_pointer(&value, "/runtime/mutation_count_after_failure", json!(0));
        assert_eq!(
            value.pointer("/runtime/store_hash_before_failure"),
            value.pointer("/runtime/store_hash_after_failure"),
            "falsifier {fault} must fail before runtime mutation"
        );
    }
}

#[test]
fn same_id_carrier_or_schedule_content_mutation_rejects_artifact_identity_mismatch() {
    for (fault, diagnostic) in [
        (
            "typed_carrier_same_id_content_hash_mismatch",
            "TypedCarrierIdentityMismatch",
        ),
        (
            "schedule_action_same_id_content_hash_mismatch",
            "ScheduleActionIdentityMismatch",
        ),
    ] {
        let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
        let report: M10ConformanceReport = system
            .run_conformance(
                request_with_predicates(predicate_profile())
                    .typed_input_mutation(typed_falsifier(fault)),
            )
            .expect("same-id hash mutation returns a typed conformance failure report");
        let value = serde_json::to_value(report).expect("falsifier report serializes");

        assert_pointer(&value, "/terminal_outcome", json!("ConformanceFailure"));
        assert_pointer(&value, "/falsifier/name", json!(fault));
        assert_pointer(&value, "/falsifier/name_driven_terminal_used", json!(false));
        assert_pointer(&value, "/validation/real_validator_invoked", json!(true));
        assert_pointer(
            &value,
            "/validation/invocations/artifact_identity_validator",
            json!(1),
        );
        assert_pointer(&value, "/diagnostics/0/code", json!(diagnostic));
        assert_pointer(&value, "/waiver_carrier", Value::Null);
        assert_pointer(&value, "/runtime/mutation_count_after_failure", json!(0));
        assert_eq!(
            value.pointer("/runtime/store_hash_before_failure"),
            value.pointer("/runtime/store_hash_after_failure"),
            "same-id content mutation {fault} must fail before runtime mutation"
        );
    }
}

#[test]
fn semantic_source_or_schedule_content_mutation_cannot_reuse_old_generated_evidence_or_pass_rows() {
    let baseline = run_conformance();
    let baseline_evidence_hash = baseline.pointer("/generator/evidence_hash").cloned();

    for fault in [
        "source_same_path_semantic_change_parse_checkable",
        "schedule_same_id_meaningful_content_change",
    ] {
        let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
        let report: M10ConformanceReport = system
            .run_conformance(
                request_with_predicates(predicate_profile())
                    .typed_input_mutation(typed_falsifier(fault)),
            )
            .expect("semantic content mutation returns a typed conformance failure report");
        let value = serde_json::to_value(report).expect("falsifier report serializes");

        assert_pointer(&value, "/terminal_outcome", json!("ConformanceFailure"));
        assert_pointer(&value, "/falsifier/name", json!(fault));
        assert_pointer(&value, "/falsifier/name_driven_terminal_used", json!(false));
        assert_absent_or_null(&value, "/c_static/pass_count");
        assert_absent_or_null(&value, "/c_runtime/pass_count");
        assert_ne!(
            value.pointer("/generator/evidence_hash"),
            baseline_evidence_hash.as_ref(),
            "semantic mutation {fault} must not retain the original generated evidence hash and pass rows"
        );
        assert_pointer(&value, "/runtime/mutation_count_after_failure", json!(0));
    }
}

#[test]
fn typed_schedule_has_no_direct_mutation_surface_for_store_grant_verdict_history_projection_or_fallback()
 {
    let schedule = action_schedule();
    let schedule_text = serde_json::to_string(&schedule).expect("schedule serializes");
    for forbidden in [
        "store_after",
        "grant_table",
        "direct_grant",
        "verdict_override",
        "fallback_state",
        "history_rows",
        "projection_rows",
        "direct_projection_mutation",
        "expected",
        "expect_",
    ] {
        assert!(
            !schedule_text.contains(forbidden),
            "typed schedule must provide request/context only, not direct mutation or expectation key {forbidden}"
        );
    }

    let report = run_conformance();
    assert_pointer(
        &report,
        "/inputs/schedule/direct_store_mutation_api_available",
        json!(false),
    );
    assert_pointer(
        &report,
        "/inputs/schedule/direct_grant_mutation_api_available",
        json!(false),
    );
    assert_pointer(
        &report,
        "/inputs/schedule/direct_verdict_mutation_api_available",
        json!(false),
    );
    assert_pointer(
        &report,
        "/inputs/schedule/direct_fallback_mutation_api_available",
        json!(false),
    );
    assert_pointer(
        &report,
        "/inputs/schedule/direct_history_mutation_api_available",
        json!(false),
    );
    assert_pointer(
        &report,
        "/inputs/schedule/direct_projection_mutation_api_available",
        json!(false),
    );
}
