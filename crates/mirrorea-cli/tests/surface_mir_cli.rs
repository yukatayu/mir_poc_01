use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

fn cli_bin() -> &'static str {
    env!("CARGO_BIN_EXE_mirrorea-alpha")
}

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

fn run_cli(args: &[&str]) -> Output {
    Command::new(cli_bin())
        .args(args)
        .output()
        .expect("mirrorea-alpha should run")
}

fn parse_json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout should be JSON")
}

fn visible_state_patch() -> &'static str {
    r#"
module Patch.DebugLamp

import Surface.WorldCore

place World

record DebugLamp {
  enabled: Bool,
}

World {
  state lamp[p: Participant]: DebugLamp
    init DebugLamp { enabled: true }
    visible observer_safe fields { enabled }
}
"#
}

fn self_grant_patch() -> &'static str {
    r#"
module Patch.SelfGrantServerAuthority

place World

World {
  when start {
    grant ServerAuthority to self
  }
}
"#
}

#[test]
fn parse_source_command_accepts_surface_patch_source() {
    let root = unique_temp_dir("mirrorea-surface-parse-source");
    let source = write_file(&root, "patch.mir", visible_state_patch());

    let output = run_cli(&["parse-source", source.to_str().unwrap(), "--format", "json"]);
    let payload = parse_json(&output);

    assert!(output.status.success(), "{payload:#}");
    assert_eq!(payload["command"], "parse-source");
    assert_eq!(payload["accepted"], true);
    assert_eq!(payload["canonical_place_scope_syntax"], "S { ... }");
}

#[test]
fn patch_source_command_emits_hotplug_verdict_and_activation_cut() {
    let root = unique_temp_dir("mirrorea-surface-patch-source");
    let source = write_file(&root, "patch.mir", visible_state_patch());

    let output = run_cli(&[
        "patch-source",
        "session#world",
        source.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let payload = parse_json(&output);

    assert!(output.status.success(), "{payload:#}");
    assert_eq!(payload["command"], "patch-source");
    assert_eq!(payload["accepted"], true);
    assert_eq!(payload["hotplug_verdict"]["verdict_kind"], "accepted");
    assert_eq!(payload["activation_cut"]["cut_kind"], "activation_cut");
    assert_eq!(payload["direct_eval_performed"], false);
}

#[test]
fn check_source_command_is_inspection_only_without_activation_cut() {
    let root = unique_temp_dir("mirrorea-surface-check-source");
    let source = write_file(&root, "patch.mir", visible_state_patch());

    let output = run_cli(&["check-source", source.to_str().unwrap(), "--format", "json"]);
    let payload = parse_json(&output);

    assert!(output.status.success(), "{payload:#}");
    assert_eq!(payload["command"], "check-source");
    assert_eq!(payload["accepted"], true);
    assert_eq!(payload["hotplug_verdict"]["verdict_kind"], "accepted");
    assert!(payload["activation_cut"].is_null());
    assert_eq!(payload["runtime_mutation_applied"], false);
    assert_eq!(payload["direct_eval_performed"], false);
}

#[test]
fn elaborate_source_command_is_inspection_only_without_activation_cut() {
    let root = unique_temp_dir("mirrorea-surface-elaborate-source");
    let source = write_file(&root, "patch.mir", visible_state_patch());

    let output = run_cli(&[
        "elaborate-source",
        source.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let payload = parse_json(&output);

    assert!(output.status.success(), "{payload:#}");
    assert_eq!(payload["command"], "elaborate-source");
    assert_eq!(payload["accepted"], true);
    assert!(payload["activation_cut"].is_null());
    assert_eq!(payload["runtime_mutation_applied"], false);
    assert_eq!(payload["direct_eval_performed"], false);
}

#[test]
fn source_commands_report_missing_file_as_io_error() {
    let root = unique_temp_dir("mirrorea-surface-missing-source");
    let missing = root.join("missing.mir");

    let output = run_cli(&[
        "patch-source",
        "session#world",
        missing.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let payload = parse_json(&output);

    assert!(!output.status.success(), "{payload:#}");
    assert_eq!(payload["command"], "patch-source");
    assert_eq!(payload["diagnostic_code"], "source_path_io_error");
}

#[test]
fn patch_source_rejects_self_grant_without_activation_cut() {
    let root = unique_temp_dir("mirrorea-surface-patch-source-reject");
    let source = write_file(&root, "patch.mir", self_grant_patch());

    let output = run_cli(&[
        "patch-source",
        "session#world",
        source.to_str().unwrap(),
        "--format",
        "json",
    ]);
    let payload = parse_json(&output);

    assert!(!output.status.success(), "{payload:#}");
    assert_eq!(payload["accepted"], false);
    assert_eq!(payload["hotplug_verdict"]["verdict_kind"], "rejected");
    assert!(payload["activation_cut"].is_null());
    assert!(
        payload["diagnostic_codes"]
            .as_array()
            .expect("diagnostic codes should be array")
            .iter()
            .any(|code| code == "patch_self_grant_server_authority_rejected")
    );
}
