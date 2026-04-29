use mir_runtime::hotplug_runtime::{
    assemble_hotplug_runtime_engine_report, assemble_hotplug_runtime_skeleton_report,
    build_hotplug_runtime_engine_report, build_hotplug_runtime_skeleton_report,
};
use mirrorea_core::{
    HotPlugRequest, HotPlugVerdict, LogicalPlaceRuntimeShell, hotplug_request_lanes,
    hotplug_verdict_lanes,
};

fn example_admitted_inputs() -> (LogicalPlaceRuntimeShell, HotPlugRequest, HotPlugVerdict) {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell
        .register_place("AttachPoint[ExampleRoom#1]", "AttachPoint")
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
    assert!(
        report
            .consumed_substrates
            .contains(&"LogicalPlaceRuntimeShell".to_string())
    );
    assert!(
        report.notes.contains(
            &"consumer-side runtime/report assembly over admitted carriers and existing substrate"
                .to_string()
        )
    );
}

#[test]
fn hotplug_runtime_skeleton_report_keeps_helper_preview_ids_and_engine_claims_out() {
    let report = build_hotplug_runtime_skeleton_report().unwrap();
    assert_ne!(report.request.request_id, "attach_request#1");
    assert_ne!(report.request.message_envelope_ref, "attach_request#1");
    assert_ne!(report.request.request_id, "detach_request#1");
    assert!(
        report
            .retained_later_refs
            .contains(&"helper_local_lifecycle_ids".to_string())
    );
    assert!(
        report
            .retained_later_refs
            .contains(&"completed_engine".to_string())
    );
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

#[test]
fn hotplug_runtime_engine_report_maps_attach_accept_to_runtime_state() {
    let report = build_hotplug_runtime_engine_report().unwrap();

    assert_eq!(
        report.engine_scope,
        "runtime_side_engine_state_progression_narrow".to_string()
    );
    assert_eq!(
        report.engine_state.state_kind,
        "attach_ready_for_activation_cut".to_string()
    );
    assert_eq!(
        report.engine_state.request_ref,
        report.skeleton.request.request_id
    );
    assert_eq!(report.engine_state.operation_kind, "attach".to_string());
    assert_eq!(report.engine_state.verdict_kind, "accepted".to_string());
    assert!(
        report
            .retained_later_refs
            .contains(&"rollback_protocol".to_string())
    );
    assert!(report.notes.contains(
        &"runtime-side engine state progression remains narrow and non-public".to_string()
    ));
}

#[test]
fn hotplug_runtime_engine_report_maps_detach_deferred_to_boundary_state() {
    let (shell, mut request, mut verdict) = example_admitted_inputs();
    request.operation_kind = "detach".to_string();
    verdict.verdict_kind = "deferred".to_string();
    verdict.notes = vec![
        "detach stays at boundary cut".to_string(),
        "rollback remains deferred".to_string(),
    ];

    let report = assemble_hotplug_runtime_engine_report(&shell, request, verdict).unwrap();

    assert_eq!(
        report.engine_state.state_kind,
        "detach_deferred_before_boundary".to_string()
    );
    assert!(
        report
            .retained_later_refs
            .contains(&"durable_migration".to_string())
    );
    assert!(report.engine_state.notes.contains(
        &"rollback / durable migration / distributed activation ordering remain later".to_string()
    ));
}

#[test]
fn hotplug_runtime_engine_report_covers_remaining_admitted_state_pairs() {
    let cases = [
        ("attach", "rejected", "attach_rejected_before_activation"),
        ("attach", "deferred", "attach_deferred_before_activation"),
        ("detach", "accepted", "detach_ready_for_boundary_cut"),
        ("detach", "rejected", "detach_rejected_before_boundary"),
    ];

    for (operation_kind, verdict_kind, expected_state_kind) in cases {
        let (shell, mut request, mut verdict) = example_admitted_inputs();
        request.operation_kind = operation_kind.to_string();
        verdict.verdict_kind = verdict_kind.to_string();

        let report = assemble_hotplug_runtime_engine_report(&shell, request, verdict).unwrap();

        assert_eq!(
            report.engine_state.state_kind,
            expected_state_kind.to_string(),
            "unexpected state kind for {operation_kind}/{verdict_kind}"
        );
    }
}

#[test]
fn hotplug_runtime_engine_report_flattens_reason_refs_and_tracks_membership_epoch() {
    let (mut shell, request, mut verdict) = example_admitted_inputs();
    shell.add_participant("ExampleObserver").unwrap();
    verdict.compatibility_reason_refs = vec![
        "attachpoint_registered".to_string(),
        "attachpoint_kind_ok".to_string(),
    ];
    verdict.authorization_reason_refs = vec![
        "attach_capability_present".to_string(),
        "admin_role_confirmed".to_string(),
    ];
    verdict.membership_freshness_reason_refs = vec![
        "membership_frontier_verified".to_string(),
        "membership_epoch_current".to_string(),
    ];

    let report = assemble_hotplug_runtime_engine_report(&shell, request, verdict).unwrap();

    assert_eq!(report.engine_state.active_membership_epoch, 1);
    assert_eq!(
        report.engine_state.reason_refs,
        vec![
            "attachpoint_registered".to_string(),
            "attachpoint_kind_ok".to_string(),
            "attach_capability_present".to_string(),
            "admin_role_confirmed".to_string(),
            "membership_frontier_verified".to_string(),
            "membership_epoch_current".to_string(),
        ]
    );
    assert_eq!(report.engine_state.requesting_principal, "ExampleAdmin");
    assert_eq!(
        report.engine_state.requesting_participant_place,
        "ParticipantPlace[ExampleAdmin]".to_string()
    );
}

#[test]
fn hotplug_runtime_engine_report_rejects_unknown_operation_kind() {
    let (shell, mut request, verdict) = example_admitted_inputs();
    request.operation_kind = "migrate".to_string();

    let err = assemble_hotplug_runtime_engine_report(&shell, request, verdict).unwrap_err();

    assert_eq!(
        err.to_string(),
        "hot-plug runtime engine report does not admit operation_kind `migrate` with verdict_kind `accepted` in the current narrow cut"
            .to_string()
    );
}
