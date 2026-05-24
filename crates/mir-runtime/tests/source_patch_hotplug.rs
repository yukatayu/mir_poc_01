use mir_runtime::surface_source_patch_hotplug::check_surface_source_patch_source;

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

fn undeclared_failure_patch() -> &'static str {
    r#"
module Patch.UndeclaredFailure

place Client
place World

record Player {
  hp: Int64,
}

World {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

Client {
  when tick {
    World {
      player[self].hp = 1
    }
  }
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
fn accepted_patch_runs_checked_pipeline_and_emits_activation_cut() {
    let report = check_surface_source_patch_source(visible_state_patch(), "session#world");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.direct_eval_performed);
    assert!(report.runtime_mutation_applied);
    assert_eq!(report.stage_status("parse"), Some(true));
    assert_eq!(report.stage_status("typecheck"), Some(true));
    assert_eq!(report.stage_status("elaborate"), Some(true));
    assert_eq!(report.stage_status("compatibility"), Some(true));
    assert_eq!(report.stage_status("admission"), Some(true));
    assert_eq!(
        report
            .hotplug_verdict
            .as_ref()
            .map(|row| row.verdict_kind.as_str()),
        Some("accepted")
    );
    assert!(report.hotplug_request.is_some());
    let capability_refs = &report
        .hotplug_request
        .as_ref()
        .expect("request should be present")
        .capability_refs;
    assert!(capability_refs.contains(&"capability#PatchSource".to_string()));
    assert!(capability_refs.contains(&"capability#AddState(World)".to_string()));
    assert!(capability_refs.contains(&"capability#PublishVisible(World)".to_string()));
    assert!(report.activation_cut.is_some());
    assert!(
        report
            .compatibility
            .state_additions
            .iter()
            .any(|row| row.owner_locus == "World"
                && row.state_name == "lamp"
                && row.visible_fields == ["enabled"])
    );
}

#[test]
fn undeclared_generated_failure_rejects_without_runtime_mutation() {
    let report = check_surface_source_patch_source(undeclared_failure_patch(), "session#world");

    assert!(!report.accepted);
    assert!(!report.direct_eval_performed);
    assert!(!report.runtime_mutation_applied);
    assert_eq!(
        report
            .hotplug_verdict
            .as_ref()
            .map(|row| row.verdict_kind.as_str()),
        Some("rejected")
    );
    assert!(report.activation_cut.is_none());
    assert!(
        report
            .diagnostic_codes()
            .contains(&"generated_failure_not_declared".to_string())
    );
}

#[test]
fn self_grant_server_authority_is_rejected_before_activation_cut() {
    let report = check_surface_source_patch_source(self_grant_patch(), "session#world");

    assert!(!report.accepted);
    assert!(!report.runtime_mutation_applied);
    assert!(report.activation_cut.is_none());
    assert!(
        report
            .diagnostic_codes()
            .contains(&"patch_self_grant_server_authority_rejected".to_string())
    );
    assert_eq!(
        report
            .hotplug_verdict
            .as_ref()
            .map(|row| row.verdict_kind.as_str()),
        Some("rejected")
    );
}
