use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::full_system_v1_projection::project_full_system_v1_path;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_file(root: &Path, relative_path: &str, content: &str) -> PathBuf {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("path should have parent"))
        .expect("parent should be created");
    fs::write(&path, content).expect("file should be written");
    path
}

#[test]
fn projection_runtime_realizes_server_client_adapter_manifests() {
    let root = unique_temp_dir("mir-full-system-v1-projection-accept");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");

    write_file(
        &root,
        "shared/src/add-one.mir",
        r#"module Shared.AddOne

fn add_one(x: Int64) -> Int64 {
  return x + 1
}
"#,
    );
    let source = write_file(
        &root,
        "main/src/effectful-sugoroku-positive.mir",
        r#"module FullSystemV1.EffectfulSugorokuPositive

import Shared.AddOne

capability HostRead
capability HostWrite
capability Publisher
capability Observer
capability WitnessAuthority
capability HandoffAuthority
capability CutAuthority

effect read_int {
  requires HostRead
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

effect publish_roll(value: Int64) {
  requires Publisher
  failure PublishRejected
}

effect observe_roll {
  requires Observer
  output seen: Int64
  failure ObserveRejected
}

effect issue_turn_witness(turn: Int64) {
  requires WitnessAuthority
  output ticket: Text
  failure WitnessRejected
}

effect handoff_turn(ticket: Text) {
  requires HandoffAuthority
  failure HandoffRejected
}

effect seal_places {
  requires CutAuthority
  failure CutRejected
}

effect quiesce_messages {
  requires CutAuthority
  failure CutRejected
}

effect atomic_cut(label: Text) {
  requires CutAuthority
  failure CutRejected
}

transition main at SugorokuPlace requires HostRead, HostWrite, Publisher, Observer, WitnessAuthority, HandoffAuthority, CutAuthority {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
  perform publish_roll(y) via publish_bus
  seen <- perform observe_roll via observe_bus
    ensure seen = y
  ticket <- perform issue_turn_witness(seen) via witness_store
  perform handoff_turn(ticket) via handoff_port
  perform seal_places via session_admin
  perform quiesce_messages via session_admin
  perform atomic_cut("turn-finished") via session_cut
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "effectful-sugoroku-projection",
  "targets": [
    {
      "target_id": "world-server",
      "role": "server",
      "place_refs": ["SugorokuPlace"],
      "entry_transitions": ["main"],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    },
    {
      "target_id": "world-client",
      "role": "client",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": false,
      "prediction_allowed": true
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "host_input",
      "boundary_kind": "ffi",
      "effect_names": ["read_int"],
      "from_target": "host-adapter",
      "to_target": "world-server",
      "authority": "adapter_read_only",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.host_input.read_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    },
    {
      "boundary_ref": "host_output",
      "boundary_kind": "ffi",
      "effect_names": ["write_int"],
      "from_target": "world-server",
      "to_target": "host-adapter",
      "authority": "adapter_write_side_effect",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.host_output.write_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    },
    {
      "boundary_ref": "publish_bus",
      "boundary_kind": "packet",
      "effect_names": ["publish_roll"],
      "from_target": "world-server",
      "to_target": "world-client",
      "authority": "server_publish",
      "required_witnesses": [],
      "packet_schema_ref": "packet.roll.publish",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "publish_replay_visible"
    },
    {
      "boundary_ref": "observe_bus",
      "boundary_kind": "packet",
      "effect_names": ["observe_roll"],
      "from_target": "world-server",
      "to_target": "world-client",
      "authority": "observer_safe_view",
      "required_witnesses": [],
      "packet_schema_ref": "packet.roll.observe",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "observer_projection_only"
    },
    {
      "boundary_ref": "witness_store",
      "boundary_kind": "packet",
      "effect_names": ["issue_turn_witness"],
      "from_target": "world-server",
      "to_target": "world-client",
      "authority": "witness_issue",
      "required_witnesses": [],
      "packet_schema_ref": "packet.turn.witness",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "savepoint_anchor_required"
    },
    {
      "boundary_ref": "handoff_port",
      "boundary_kind": "packet",
      "effect_names": ["handoff_turn"],
      "from_target": "world-client",
      "to_target": "world-server",
      "authority": "witness_handoff",
      "required_witnesses": ["turn_ticket"],
      "packet_schema_ref": "packet.turn.handoff",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "witness_reacquire_on_load"
    },
    {
      "boundary_ref": "session_admin",
      "boundary_kind": "packet",
      "effect_names": ["seal_places", "quiesce_messages"],
      "from_target": "world-server",
      "to_target": "world-server",
      "authority": "save_load_admin",
      "required_witnesses": [],
      "packet_schema_ref": "packet.session.admin",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "quiescent_required"
    },
    {
      "boundary_ref": "session_cut",
      "boundary_kind": "packet",
      "effect_names": ["atomic_cut"],
      "from_target": "world-server",
      "to_target": "world-server",
      "authority": "save_load_admin",
      "required_witnesses": [],
      "packet_schema_ref": "packet.session.cut",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "cut_frontier_required"
    }
  ]
}"#,
    );

    let report = project_full_system_v1_path(&source, &request);

    assert!(report.accepted, "{report:?}");
    assert_eq!(report.projection_ir.targets.len(), 3);
    assert_eq!(
        report
            .target_manifests
            .iter()
            .find(|row| row.target_id == "world-server")
            .expect("server manifest should exist")
            .role,
        "server"
    );
    let server_manifest = report
        .target_manifests
        .iter()
        .find(|row| row.target_id == "world-server")
        .expect("server manifest should exist");
    let client_manifest = report
        .target_manifests
        .iter()
        .find(|row| row.target_id == "world-client")
        .expect("client manifest should exist");
    let adapter_manifest = report
        .target_manifests
        .iter()
        .find(|row| row.target_id == "host-adapter")
        .expect("adapter manifest should exist");

    assert!(
        server_manifest
            .capability_row
            .contains(&"HostWrite".to_string())
    );
    assert!(
        server_manifest
            .capability_row
            .contains(&"Publisher".to_string())
    );
    assert!(
        server_manifest
            .capability_row
            .contains(&"WitnessAuthority".to_string())
    );
    assert!(
        !client_manifest
            .capability_row
            .contains(&"Publisher".to_string())
    );
    assert!(
        !client_manifest
            .capability_row
            .contains(&"WitnessAuthority".to_string())
    );
    assert!(client_manifest.capability_row.is_empty());
    assert!(adapter_manifest.capability_row.is_empty());
    assert!(
        report
            .preservation_report
            .checked_effect_rows
            .contains(&"publish_roll".to_string())
    );
    assert!(
        report
            .preservation_report
            .checked_provider_policy_rows
            .contains(&"host-adapter:native_disabled".to_string())
    );
    assert!(
        report
            .preservation_report
            .residual_obligations
            .iter()
            .any(|row| row.code == "packet_ffi_schema_semantics_deferred")
    );
}

#[test]
fn projection_runtime_rejects_client_write_authority_escalation() {
    let root = unique_temp_dir("mir-full-system-v1-projection-reject");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_file(
        &root,
        "main/src/client-write-authority-negative.mir",
        r#"module FullSystemV1.ClientWriteAuthorityNegative

capability HostWrite

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition update at ClientView requires HostWrite {
  perform write_int(1) via client_output
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "client-write-authority-negative",
  "targets": [
    {
      "target_id": "world-server",
      "role": "server",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    },
    {
      "target_id": "world-client",
      "role": "client",
      "place_refs": ["ClientView"],
      "entry_transitions": ["update"],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": false,
      "prediction_allowed": true
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "client_output",
      "boundary_kind": "ffi",
      "effect_names": ["write_int"],
      "from_target": "world-client",
      "to_target": "host-adapter",
      "authority": "client_world_write",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.client.write_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    }
  ]
}"#,
    );

    let report = project_full_system_v1_path(&source, &request);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report
            .diagnostics
            .iter()
            .map(|row| row.code.as_str())
            .collect::<Vec<_>>(),
        vec!["client_write_authority_escalation"]
    );
    assert!(report.target_manifests.is_empty());
    assert!(
        report
            .preservation_report
            .rejected_rows
            .contains(&"client_output:client_write_authority_escalation".to_string())
    );
}

#[test]
fn projection_runtime_rejects_client_mutation_even_when_effect_name_is_renamed() {
    let root = unique_temp_dir("mir-full-system-v1-projection-client-mutate");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_file(
        &root,
        "main/src/client-mutate-negative.mir",
        r#"module FullSystemV1.ClientMutateNegative

capability HostWrite

effect mutate_turn(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition update at ClientView requires HostWrite {
  perform mutate_turn(1) via client_output
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "client-mutate-negative",
  "targets": [
    {
      "target_id": "world-client",
      "role": "client",
      "place_refs": ["ClientView"],
      "entry_transitions": ["update"],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": false,
      "prediction_allowed": true
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "client_output",
      "boundary_kind": "ffi",
      "effect_names": ["mutate_turn"],
      "from_target": "world-client",
      "to_target": "host-adapter",
      "authority": "client_projection_mutation",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.client.mutate_turn",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    }
  ]
}"#,
    );

    let report = project_full_system_v1_path(&source, &request);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report
            .diagnostics
            .iter()
            .map(|row| row.code.as_str())
            .collect::<Vec<_>>(),
        vec!["client_write_authority_escalation"]
    );
}

#[test]
fn projection_runtime_rejects_unassigned_source_places() {
    let root = unique_temp_dir("mir-full-system-v1-projection-unassigned-place");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_file(
        &root,
        "main/src/unassigned-place-negative.mir",
        r#"module FullSystemV1.UnassignedPlaceNegative

capability HostWrite

effect mutate_turn(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition update at SugorokuPlace requires HostWrite {
  perform mutate_turn(1) via server_output
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "unassigned-place-negative",
  "targets": [
    {
      "target_id": "world-server",
      "role": "server",
      "place_refs": [],
      "entry_transitions": ["update"],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "server_output",
      "boundary_kind": "ffi",
      "effect_names": ["mutate_turn"],
      "from_target": "world-server",
      "to_target": "host-adapter",
      "authority": "server_state_write",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.server.mutate_turn",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    }
  ]
}"#,
    );

    let report = project_full_system_v1_path(&source, &request);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report
            .diagnostics
            .iter()
            .map(|row| row.code.as_str())
            .collect::<Vec<_>>(),
        vec!["unassigned_place_ref"]
    );
}

#[test]
fn projection_runtime_rejects_save_load_authority_on_client_target() {
    let root = unique_temp_dir("mir-full-system-v1-projection-save-load-client");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_file(
        &root,
        "main/src/client-save-load-negative.mir",
        r#"module FullSystemV1.ClientSaveLoadNegative

capability Observer

effect observe_roll {
  requires Observer
  output seen: Int64
  failure ObserveRejected
}

transition observe at ClientView requires Observer {
  seen <- perform observe_roll via observe_bus
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "client-save-load-negative",
  "targets": [
    {
      "target_id": "world-client",
      "role": "client",
      "place_refs": ["ClientView"],
      "entry_transitions": ["observe"],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": true
    },
    {
      "target_id": "world-server",
      "role": "server",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "observe_bus",
      "boundary_kind": "packet",
      "effect_names": ["observe_roll"],
      "from_target": "world-server",
      "to_target": "world-client",
      "authority": "observer_safe_view",
      "required_witnesses": [],
      "packet_schema_ref": "packet.roll.observe",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "observer_projection_only"
    }
  ]
}"#,
    );

    let report = project_full_system_v1_path(&source, &request);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report
            .diagnostics
            .iter()
            .map(|row| row.code.as_str())
            .collect::<Vec<_>>(),
        vec!["save_load_authority_requires_server_target"]
    );
}
