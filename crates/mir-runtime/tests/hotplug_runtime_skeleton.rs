use mir_runtime::hotplug_runtime::{
    assemble_hotplug_runtime_skeleton_report, build_hotplug_runtime_skeleton_report,
};
use mirrorea_core::{
    hotplug_request_lanes, hotplug_verdict_lanes, HotPlugRequest, HotPlugVerdict,
    LogicalPlaceRuntimeShell,
};

fn example_admitted_inputs() -> (LogicalPlaceRuntimeShell, HotPlugRequest, HotPlugVerdict) {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place("AttachPoint[ExampleRoom#1]", "AttachPoint")
        .unwrap();
    shell.add_initial_participant("ExampleAdmin").unwrap();

    let request = HotPlugRequest {
        request_id: "hotplug_request#runtime_skeleton".to_string(),
        attachpoint_ref: "AttachPoint[ExampleRoom#1]".to_string(),
        patch_ref: "Patch[ExamplePackage@runtime]".to_string(),
        operation_kind: "attach".to_string(),
        requesting_principal: "ExampleAdmin".to_string(),
        requesting_participant_place: "ParticipantPlace[ExampleAdmin]".to_string(),
        message_envelope_ref: "hotplug_request_envelope#runtime_skeleton".to_string(),
        auth_evidence_ref: None,
        capability_refs: vec![
            "ManageAttachPoint(ExampleRoom#1)".to_string(),
            "InstallPatch(ExamplePackage@runtime)".to_string(),
        ],
        witness_refs: vec!["membership_frontier_snapshot#runtime".to_string()],
        notes: vec![
            "test admitted carrier".to_string(),
            "no helper-local lifecycle ids imported as canonical runtime state".to_string(),
        ],
    };
    let verdict = HotPlugVerdict {
        request_ref: request.request_id.clone(),
        verdict_kind: "accepted".to_string(),
        compatibility_reason_refs: vec!["attachpoint_registered".to_string()],
        authorization_reason_refs: vec!["attach_capability_present".to_string()],
        membership_freshness_reason_refs: vec!["membership_frontier_verified".to_string()],
        notes: vec![
            "test admitted verdict".to_string(),
            "completed engine semantics remain deferred".to_string(),
        ],
    };

    (shell, request, verdict)
}

#[test]
fn hotplug_runtime_skeleton_report_tracks_admitted_carriers_over_runtime_substrate() {
    let (shell, request, verdict) = example_admitted_inputs();
    let report = assemble_hotplug_runtime_skeleton_report(&shell, request, verdict).unwrap();
    assert_eq!(
        report.skeleton_scope,
        "report_local_orchestration_skeleton".to_string()
    );
    assert_eq!(report.request_lanes, hotplug_request_lanes());
    assert_eq!(report.verdict_lanes, hotplug_verdict_lanes());
    assert_eq!(report.verdict.request_ref, report.request.request_id);
    assert_eq!(
        report.request.requesting_participant_place,
        report.runtime_snapshot.membership.members["ExampleAdmin"].place
    );
    assert_eq!(
        report
            .runtime_snapshot
            .place_catalog
            .places
            .get(&report.request.attachpoint_ref),
        Some(&"AttachPoint".to_string())
    );
    assert!(report
        .consumed_substrates
        .contains(&"LogicalPlaceRuntimeShell".to_string()));
    assert!(report.notes.contains(
        &"consumer-side runtime/report assembly over admitted carriers and existing substrate"
            .to_string()
    ));
}

#[test]
fn hotplug_runtime_skeleton_report_keeps_helper_preview_ids_and_engine_claims_out() {
    let report = build_hotplug_runtime_skeleton_report().unwrap();
    assert_ne!(report.request.request_id, "attach_request#1");
    assert_ne!(report.request.message_envelope_ref, "attach_request#1");
    assert_ne!(report.request.request_id, "detach_request#1");
    assert!(report
        .retained_later_refs
        .contains(&"helper_local_lifecycle_ids".to_string()));
    assert!(report
        .retained_later_refs
        .contains(&"completed_engine".to_string()));
    assert!(report.notes.contains(
        &"no helper-local lifecycle ids imported as canonical runtime state".to_string()
    ));
}

#[test]
fn hotplug_runtime_skeleton_report_requires_membership_for_requesting_principal() {
    let (shell, mut request, verdict) = example_admitted_inputs();
    request.requesting_principal = "Mallory".to_string();
    request.requesting_participant_place = "ParticipantPlace[Mallory]".to_string();

    let err = assemble_hotplug_runtime_skeleton_report(&shell, request, verdict).unwrap_err();
    assert_eq!(
        err.to_string(),
        "hot-plug runtime skeleton requires active membership for requesting principal Mallory"
            .to_string()
    );
}
