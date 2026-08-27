use std::{
    collections::{BTreeSet, VecDeque},
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::ResultVersion,
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSurfaceV0, check_and_elaborate_surface_v0,
    },
};

use crate::{
    m9_auth_verification::M9RuntimeExecutionSeam,
    sys3_projection::{
        BackendEligibility, BackendProfile, CommunicationEdgeKind, DeclaredLogicalTopology,
        GlobalProjectionResult, RuntimeAdmissionStatus, project_checked_core,
    },
    sys4_dispatch::{
        ExternalAction, FabricProgram, FabricRouteKey, FaultInjection, LocalFabric,
        RuntimeStoreRead, RuntimeStoreWrite, RuntimeValue, SealedFabricAdmission, SourceAction,
        Sys4DiagnosticKind, Sys4DispatchDiagnostics, Sys4InitialStateSeed, Sys4TraceKind,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER_ENDPOINT_FIXTURE: &str = "sys4_ow1_endpoint_crossing.mir";
const DESIGNATED_CONSUME_FIXTURE: &str = "sys4_designated_consume_with_auth.mir";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn load_checked_fixture(name: &str) -> CheckedSurfaceV0 {
    let (relative, source) = load_surface_fixture(name);
    check_and_elaborate_surface_v0(FixtureSource::new(relative, source))
        .expect("SYS-4 dispatch tests start from real parsed and checked M7 source")
}

fn topology<const N: usize>(
    identity: &CheckedProgramIdentity,
    loci: [&'static str; N],
) -> DeclaredLogicalTopology {
    DeclaredLogicalTopology::try_new(identity.clone(), loci)
        .expect("test topology has unique declared loci")
}

fn project_fixture<const N: usize>(
    checked: &CheckedSurfaceV0,
    loci: [&'static str; N],
) -> GlobalProjectionResult {
    let projection = project_checked_core(checked, &topology(checked.program_identity(), loci))
        .expect("fixture projects from checked Core");
    assert_eq!(
        projection.runtime_admission_status(),
        RuntimeAdmissionStatus::BlockedByResidual,
        "SYS-4 auth/verify fixtures remain static programs blocked by residuals until a complete M9 final seam is supplied"
    );
    projection
}

fn owner_endpoint_checked() -> CheckedSurfaceV0 {
    load_checked_fixture(OWNER_ENDPOINT_FIXTURE)
}

fn designated_checked() -> CheckedSurfaceV0 {
    load_checked_fixture(DESIGNATED_CONSUME_FIXTURE)
}

fn owner_endpoint_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["A", "S"])
}

fn designated_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["C", "E", "S"])
}

fn fabric_program(projection: GlobalProjectionResult) -> FabricProgram {
    let program =
        FabricProgram::from_projection(projection).expect("projection becomes a static program");
    assert_eq!(
        program.runtime_admission_status(),
        RuntimeAdmissionStatus::BlockedByResidual,
        "FabricProgram is static; it is not admitted merely because it was projected"
    );
    program
}

fn initial_state_seed(identity: &CheckedProgramIdentity) -> Sys4InitialStateSeed {
    Sys4InitialStateSeed::for_checked_program(identity.clone())
        .with_int("S", "player", "self", "hp", 100)
        .with_int("S", "player", "self", "atk", 10)
}

fn m9_fabric_seam(checked: &CheckedSurfaceV0) -> M9RuntimeExecutionSeam {
    M9RuntimeExecutionSeam::test_real_admitted_sys4_fabric_seam(checked)
        .expect("SYS-4 test seam is produced by the normal M9 pipeline")
}

fn incomplete_m9_fabric_seam(checked: &CheckedSurfaceV0) -> M9RuntimeExecutionSeam {
    M9RuntimeExecutionSeam::test_incomplete_sys4_fabric_seam_missing_residual_discharge(checked)
        .expect("test helper returns an opaque incomplete M9 seam for negative SYS-4 admission")
}

fn sealed_admission(checked: &CheckedSurfaceV0, program: &FabricProgram) -> SealedFabricAdmission {
    SealedFabricAdmission::from_m9_execution_seam(
        program,
        m9_fabric_seam(checked),
        initial_state_seed(checked.program_identity()),
    )
    .expect("complete M9 final seam plus explicit seed admits the static fabric")
}

fn boot(
    checked: &CheckedSurfaceV0,
    program: FabricProgram,
    backend: BackendProfile,
) -> LocalFabric {
    let admission = sealed_admission(checked, &program);
    LocalFabric::bootstrap(program, admission, backend).expect("fabric bootstrap succeeds")
}

fn boot_with_admission(
    program: FabricProgram,
    admission: SealedFabricAdmission,
    backend: BackendProfile,
) -> LocalFabric {
    LocalFabric::bootstrap(program, admission, backend).expect("fabric bootstrap succeeds")
}

fn assert_sys4_diag<T: Debug>(
    actual: Result<T, Sys4DispatchDiagnostics>,
    expected: Sys4DiagnosticKind,
) -> Sys4DispatchDiagnostics {
    let diagnostics = actual.expect_err("operation should fail closed with SYS-4 diagnostics");
    assert_eq!(diagnostics.primary().kind(), expected);
    assert!(
        diagnostics.partial_fabric().is_none(),
        "SYS-4 failures must not expose a partial fabric"
    );
    diagnostics
}

fn communication_edge_refs(projection: &GlobalProjectionResult) -> BTreeSet<String> {
    projection
        .communication_plan()
        .edges()
        .iter()
        .map(|edge| edge.edge_ref().to_string())
        .collect()
}

fn owner_attack_action(operation: &str) -> SourceAction {
    SourceAction::owner_operation(operation).with_argument("target", "self")
}

fn publish_designated_action() -> SourceAction {
    SourceAction::designated_tick("E.result").with_tick("F", "tick:F:1")
}

fn consume_designated_action() -> SourceAction {
    SourceAction::consume_designated_result("E.result")
}

#[test]
fn bootstrap_consumes_complete_m9_final_seam_seed_and_plan_only_routes() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let expected_edge_refs = communication_edge_refs(&projection);
    let program = fabric_program(projection.clone());
    let admission = sealed_admission(&checked, &program);

    assert_eq!(program.locus_names(), vec!["A", "S"]);
    assert_eq!(program.locus_count(), projection.locus_order().len());
    assert_eq!(program.route_index().edge_refs(), expected_edge_refs);
    assert!(
        program
            .route_index()
            .all_routes_derive_from_plan(projection.communication_plan()),
        "route index is derived only from the SYS-3 CommunicationPlan"
    );
    assert!(
        program.projected_authority_grants().is_empty(),
        "projection must not mint membership/capability/witness grants"
    );

    let summary = admission.observer_safe_m9_summary();
    assert_eq!(
        summary.checked_program_identity(),
        program.checked_program_identity()
    );
    assert!(summary.is_complete_final_m9_runtime_seam());
    assert!(summary.residuals_discharged_for_static_program());
    assert!(summary.contains_owner_lineage("attack", "self", "A", "S"));
    assert!(summary.generated_by_projection().is_empty());
    assert_eq!(
        admission
            .initial_state_seed()
            .int("S", "player", "self", "hp"),
        Some(100)
    );
    assert_eq!(
        admission
            .initial_state_seed()
            .int("S", "player", "self", "atk"),
        Some(10)
    );

    let fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);
    assert_eq!(fabric.locus_names(), vec!["A", "S"]);
    for locus in ["A", "S"] {
        let runtime = fabric
            .locus_runtime(locus)
            .unwrap_or_else(|| panic!("{locus} locus runtime exists"));
        assert_eq!(runtime.locus(), locus);
        assert_eq!(
            runtime.program_identity(),
            program.checked_program_identity()
        );
        assert!(runtime.local_store().is_owned_by_locus(locus));
        assert!(
            !runtime.local_store().contains_remote_locus_state(),
            "locus runtime must not use a global remote-store shortcut"
        );
    }

    assert_sys4_diag(
        SealedFabricAdmission::from_m9_execution_seam(
            &program,
            incomplete_m9_fabric_seam(&checked),
            initial_state_seed(checked.program_identity()),
        ),
        Sys4DiagnosticKind::IncompleteM9ResidualDischarge,
    );

    let other_checked = designated_checked();
    assert_sys4_diag(
        SealedFabricAdmission::from_m9_execution_seam(
            &program,
            m9_fabric_seam(&other_checked),
            initial_state_seed(checked.program_identity()),
        ),
        Sys4DiagnosticKind::ProgramAdmissionMismatch,
    );

    let mut missing_route = program.clone();
    missing_route.for_test_remove_route(FabricRouteKey::owner_request("attack", "A", "S"));
    assert_sys4_diag(
        LocalFabric::bootstrap(
            missing_route,
            sealed_admission(&checked, &program),
            BackendProfile::St,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn bootstrap_rejects_foreign_seed_schema_owner_index_and_field() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));

    for (seed, expected) in [
        (
            initial_state_seed(checked.program_identity())
                .with_int("Foreign", "player", "self", "hp", 100),
            Sys4DiagnosticKind::ForeignSeedLocus,
        ),
        (
            initial_state_seed(checked.program_identity())
                .with_int("S", "enemy", "self", "hp", 100),
            Sys4DiagnosticKind::ForeignSeedState,
        ),
        (
            initial_state_seed(checked.program_identity())
                .with_int("S", "player", "intruder", "hp", 100),
            Sys4DiagnosticKind::ForeignSeedIndex,
        ),
        (
            initial_state_seed(checked.program_identity())
                .with_int("S", "player", "self", "mana", 100),
            Sys4DiagnosticKind::ForeignSeedField,
        ),
    ] {
        assert_sys4_diag(
            SealedFabricAdmission::from_m9_execution_seam(&program, m9_fabric_seam(&checked), seed),
            expected,
        );
    }
}

#[test]
fn owner_rmw_dispatch_uses_explicit_seed_and_mutates_only_owner_local_store() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let mut fabric = boot(&checked, program.clone(), BackendProfile::St);

    let before = fabric.semantic_snapshot();
    assert_eq!(before.int("S", "player", "self", "hp"), Some(100));
    assert_eq!(before.int("S", "player", "self", "atk"), Some(10));

    let receipt = fabric
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("A invokes source-derived attack request to S");

    assert_eq!(receipt.operation_id(), "attack");
    assert_eq!(receipt.origin_locus(), "A");
    assert_eq!(receipt.target_locus(), "S");
    assert_eq!(receipt.typed_value(), RuntimeValue::unit());
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
    assert_eq!(
        fabric.semantic_snapshot().changed_loci_since(&before),
        vec!["S"]
    );
    assert!(
        fabric
            .semantic_snapshot()
            .locus_unchanged_since("A", &before)
    );

    let rmw = receipt
        .owner_rmw_report()
        .expect("owner RMW report is present");
    assert_eq!(
        rmw.m8_reads(),
        vec![
            RuntimeStoreRead::int("S", "player", "self", "hp", 100),
            RuntimeStoreRead::int("S", "player", "self", "atk", 10),
        ]
    );
    assert_eq!(
        rmw.m8_writes(),
        vec![RuntimeStoreWrite::int("S", "player", "self", "hp", 90)]
    );
    assert!(rmw.all_reads_and_writes_have_source_core_provenance());

    let trace = fabric.trace().for_request(receipt.request_id());
    assert_eq!(
        trace.kinds(),
        vec![
            Sys4TraceKind::RequestAdmitted,
            Sys4TraceKind::Dispatched,
            Sys4TraceKind::Received,
            Sys4TraceKind::Served,
            Sys4TraceKind::M8OwnerRead,
            Sys4TraceKind::M8OwnerWrite,
            Sys4TraceKind::ReplyDispatched,
            Sys4TraceKind::ReplyReceived,
        ]
    );
    assert!(trace.crossed_endpoint_boundary("A", "S"));
    assert!(trace.all_entries_have_source_core_fragment_and_edge_provenance());
    assert!(trace.contains_edge_kind(CommunicationEdgeKind::OwnerRequest));
    assert!(trace.contains_edge_kind(CommunicationEdgeKind::OwnerReplyReceipt));

    let mut missing_route_program = program.clone();
    missing_route_program.for_test_remove_route(FabricRouteKey::owner_request("attack", "A", "S"));
    let mut missing_route_fabric = boot(&checked, missing_route_program, BackendProfile::St);
    let before_missing = missing_route_fabric.semantic_snapshot();
    assert_sys4_diag(
        missing_route_fabric.dispatch_source_action(owner_attack_action("attack")),
        Sys4DiagnosticKind::RouteUnavailable,
    );
    assert!(
        missing_route_fabric
            .semantic_snapshot()
            .same_state(&before_missing)
    );

    let mut wrong_route_program = program;
    wrong_route_program
        .for_test_retarget_route(FabricRouteKey::owner_request("attack", "A", "S"), "A");
    let mut wrong_route_fabric = boot(&checked, wrong_route_program, BackendProfile::St);
    let before_wrong = wrong_route_fabric.semantic_snapshot();
    assert_sys4_diag(
        wrong_route_fabric.dispatch_source_action(owner_attack_action("attack")),
        Sys4DiagnosticKind::WrongTargetLocus,
    );
    assert!(
        wrong_route_fabric
            .semantic_snapshot()
            .same_state(&before_wrong)
    );
}

#[test]
fn designated_result_delivery_endpoint_revalidates_cache_and_revocation_fails_closed() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let admission = sealed_admission(&checked, &program);
    let summary = admission.observer_safe_m9_summary();
    assert!(summary.contains_designated_evaluator("E.result", "E"));
    assert!(summary.contains_designated_consumer("E.result", "C"));
    assert!(summary.contains_designated_consumer_lineage("E.result", "C"));

    let mut fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("complete M9 seam admits designated fabric");

    let consumer = fabric
        .locus_runtime("C")
        .expect("consumer locus exists")
        .artifact();
    assert!(consumer.has_designated_result_consumer("E.result"));
    assert!(
        !consumer.has_designated_evaluation_expression("E.result"),
        "consumer artifact must not receive or re-place the evaluator expression"
    );

    let publish = fabric
        .dispatch_source_action(publish_designated_action())
        .expect("E publishes the projected designated result");
    assert_eq!(publish.typed_value(), RuntimeValue::int(11));

    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("C consumes the first accepted source/Core-bound delivery");
    assert_eq!(first.operation_id(), "E.result");
    assert_eq!(first.typed_value(), RuntimeValue::int(11));
    assert_eq!(first.result_version(), Some(ResultVersion::new(1)));
    assert!(first.performed_m8_semantic_consumption());

    let semantic_identity = first.semantic_consumption_identity();
    assert_eq!(
        fabric
            .m8_local_trace()
            .value_consumed_count(semantic_identity, "C"),
        1
    );
    assert_eq!(
        fabric
            .designated_consumption_state()
            .semantic_consumption_count(semantic_identity, "C"),
        1
    );

    let delivery_trace = fabric
        .trace()
        .for_designated_delivery("E.result", first.delivery_id());
    assert_eq!(
        delivery_trace.kinds(),
        vec![
            Sys4TraceKind::DesignatedResultPublished,
            Sys4TraceKind::DesignatedResultDispatched,
            Sys4TraceKind::DesignatedResultReceived,
            Sys4TraceKind::DesignatedResultConsumed,
        ]
    );
    assert!(delivery_trace.contains_edge_kind(CommunicationEdgeKind::DesignatedResultDelivery));
    assert!(delivery_trace.all_entries_have_source_core_fragment_and_edge_provenance());
    assert_eq!(
        delivery_trace.m8_value_consumed_count_for(semantic_identity, "C"),
        1
    );

    let m8_before_retry = fabric.m8_local_trace().clone();
    let live_retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("same consumer/result/frontier/version/policy retry revalidates and returns cache");
    assert_eq!(live_retry.typed_value(), first.typed_value());
    assert_eq!(live_retry.result_version(), first.result_version());
    assert!(live_retry.returned_from_designated_cache_after_authority_revalidation());
    assert!(!live_retry.performed_m8_semantic_consumption());
    assert_eq!(
        fabric
            .m8_local_trace()
            .value_consumed_count(semantic_identity, "C"),
        1
    );
    assert_eq!(
        fabric
            .m8_local_trace()
            .new_entries_since(&m8_before_retry)
            .value_consumed_count(semantic_identity, "C"),
        0
    );

    let revocation = fabric
        .m9_authority_lifecycle_mut()
        .revoke_designated_consumer_capability("E.result", "C")
        .expect("revocation is produced through the admitted M9 authority lifecycle");
    assert!(revocation.is_opaque_m9_successor_generation());
    assert!(!revocation.generation_ref().is_empty());
    assert!(!revocation.consumer_lineage_ref().is_empty());
    assert!(revocation.was_produced_by_m9_lifecycle());
    fabric
        .apply_admitted_authority_lifecycle(revocation)
        .expect("fabric installs the M9 successor authority generation");
    let before_revoked_retry = fabric.semantic_snapshot();
    let revoked = assert_sys4_diag(
        fabric.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::MissingConsumerCapability,
    );
    assert!(revoked.primary().typed_success().is_none());
    assert!(!revoked.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_revoked_retry));
    assert_eq!(
        fabric
            .m8_local_trace()
            .value_consumed_count(semantic_identity, "C"),
        1
    );
}

#[test]
fn owner_dispatch_requires_endpoint_owned_carrier_records_and_runtime_fault_injection() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let before_success = fabric.semantic_snapshot();
    let target_inbox_before = fabric
        .locus_runtime("S")
        .expect("target locus exists")
        .incoming_endpoint()
        .carrier_history_len();
    let target_m8_queue_before = fabric.m8_owner_queue_depth("S");

    let receipt = fabric
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("source-derived owner request dispatches");
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90),
        "positive owner dispatch must first mutate only the owner-local S store"
    );
    assert!(
        !fabric.semantic_snapshot().same_state(&before_success),
        "positive dispatch must not be a no-op"
    );
    let after_success = fabric.semantic_snapshot();
    let target_inbox_after_success = fabric
        .locus_runtime("S")
        .expect("target locus exists")
        .incoming_endpoint()
        .carrier_history_len();
    let target_m8_queue_after_success = fabric.m8_owner_queue_depth("S");

    let outgoing = fabric
        .locus_runtime("A")
        .expect("origin locus exists")
        .outgoing_endpoint()
        .carrier_history_for_request(receipt.request_id())
        .single(CommunicationEdgeKind::OwnerRequest, "A", "S");
    let incoming = fabric
        .locus_runtime("S")
        .expect("owner locus exists")
        .incoming_endpoint()
        .carrier_history_for_request(receipt.request_id())
        .single(CommunicationEdgeKind::OwnerRequest, "A", "S");

    assert_eq!(outgoing.carrier_id(), incoming.carrier_id());
    assert_eq!(outgoing.edge_ref(), incoming.edge_ref());
    assert_eq!(outgoing.source_locus(), "A");
    assert_eq!(outgoing.target_locus(), "S");
    assert!(outgoing.has_source_core_fragment_edge_provenance());
    assert!(incoming.has_source_core_fragment_edge_provenance());
    let enqueue_id = outgoing.enqueue_occurrence_id().to_string();
    let dequeue_id = incoming.dequeue_occurrence_id().to_string();
    let m8_request_node = fabric
        .m8_actual_trace()
        .owner_request_node_id(receipt.request_id())
        .expect("actual M8 owner request node exists")
        .to_string();
    let m8_serve_node = fabric
        .m8_actual_trace()
        .owner_serve_node_id(receipt.request_id())
        .expect("actual M8 owner serve node exists")
        .to_string();
    assert_eq!(
        fabric.causality().predecessor_ids(&dequeue_id),
        vec![enqueue_id.clone()],
        "target dequeue occurrence must causally depend on origin enqueue occurrence"
    );
    assert_eq!(
        fabric.causality().predecessor_ids(&m8_request_node),
        vec![dequeue_id.clone()],
        "M8 request node must be caused by endpoint dequeue, not by direct fabric call"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(&m8_serve_node)
            .contains(&m8_request_node),
        "M8 serve node must depend on the actual M8 request node"
    );

    assert!(
        fabric
            .trace()
            .endpoint_row_for_carrier(
                outgoing.carrier_id(),
                Sys4TraceKind::Dispatched,
                outgoing.record_id(),
                "A",
                "S"
            )
            .endpoint_occurrence_id()
            == outgoing.enqueue_occurrence_id(),
        "dispatch trace must reference the origin endpoint enqueue record"
    );
    assert!(
        fabric
            .trace()
            .endpoint_row_for_carrier(
                incoming.carrier_id(),
                Sys4TraceKind::Received,
                incoming.record_id(),
                "A",
                "S"
            )
            .endpoint_occurrence_id()
            == incoming.dequeue_occurrence_id(),
        "receive trace must reference the target endpoint dequeue record"
    );

    let fault = ExternalAction::fault_event(FaultInjection::route_unavailable("attack"));
    assert!(!fault.can_carry_checked_core_identity());
    assert!(!fault.can_carry_authority_grant());
    assert!(!fault.can_carry_state_delta());
    assert!(!fault.can_carry_expected_result());
    assert_eq!(fault.target_locus_override(), None);

    let fault_receipt = fabric
        .dispatch_external_action(fault)
        .expect("bounded source-derived fault is admitted through the fabric");
    assert!(fault_receipt.is_observer_safe());
    assert!(fault_receipt.source_derived_from_operation("attack"));
    assert!(!fault_receipt.exposes_raw_payload());

    let failed = assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::source_operation(owner_attack_action(
            "attack",
        ))),
        Sys4DiagnosticKind::RouteUnavailable,
    );
    assert!(failed.endpoint_dequeue_occurrence_id().is_none());
    assert!(failed.m8_trace_node_id().is_none());
    assert!(fabric.semantic_snapshot().same_state(&after_success));
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("target locus exists")
            .incoming_endpoint()
            .carrier_history_len(),
        target_inbox_after_success,
        "faulted follow-up request must not enqueue into the target inbox"
    );
    assert_eq!(
        target_inbox_after_success,
        target_inbox_before + 1,
        "only the successful request should have reached the target inbox"
    );
    assert_eq!(
        fabric.m8_owner_queue_depth("S"),
        target_m8_queue_after_success
    );
    assert_eq!(
        target_m8_queue_after_success, target_m8_queue_before,
        "served owner requests must not leave a pending M8 queue item"
    );
}

#[test]
fn designated_publish_requires_remote_input_endpoint_roundtrip_before_m8_evaluation() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let publish = fabric
        .dispatch_source_action(publish_designated_action())
        .expect("E publishes through generated remote-input endpoints before evaluation");

    let audit = fabric
        .endpoint_audit()
        .for_designated_publish("E.result", publish.request_id());
    let input_request = audit.single(CommunicationEdgeKind::DesignatedInputRequest, "E", "S");
    let input_receipt = audit.single(CommunicationEdgeKind::DesignatedInputReceipt, "S", "E");
    let result_delivery = audit.single(CommunicationEdgeKind::DesignatedResultDelivery, "E", "C");

    assert_eq!(
        input_receipt.request_carrier_id(),
        input_request.carrier_id()
    );
    assert_eq!(
        result_delivery.input_receipt_carrier_id(),
        input_receipt.carrier_id()
    );
    assert!(input_request.has_source_core_fragment_edge_provenance());
    assert!(input_receipt.has_source_core_fragment_edge_provenance());
    assert!(result_delivery.has_source_core_fragment_edge_provenance());
    let request_enqueue_id = input_request.enqueue_occurrence_id().to_string();
    let request_dequeue_id = input_request.dequeue_occurrence_id().to_string();
    let receipt_enqueue_id = input_receipt.enqueue_occurrence_id().to_string();
    let receipt_dequeue_id = input_receipt.dequeue_occurrence_id().to_string();
    let evaluation_node = audit
        .actual_m8_trace_node_id("DesignatedValueEvaluated")
        .expect("actual M8 designated evaluation node exists")
        .to_string();
    assert_eq!(
        audit.causality().predecessor_ids(&request_dequeue_id),
        vec![request_enqueue_id],
        "S dequeue of DesignatedInputRequest must depend on E enqueue"
    );
    assert!(
        audit
            .causality()
            .predecessor_ids(&receipt_enqueue_id)
            .contains(&request_dequeue_id),
        "S enqueue of DesignatedInputReceipt must depend on handling the input request"
    );
    assert!(
        audit
            .causality()
            .predecessor_ids(&receipt_dequeue_id)
            .contains(&receipt_enqueue_id),
        "E dequeue of DesignatedInputReceipt must depend on S enqueue"
    );
    assert!(
        audit
            .actual_m8_trace_node(&evaluation_node)
            .predecessor_ids()
            .contains(&receipt_dequeue_id),
        "M8 designated evaluation must not begin until E receives the source-owner input receipt"
    );
}

#[test]
fn admission_rejects_same_program_seam_missing_projected_authority_family() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let evaluator_only =
        M9RuntimeExecutionSeam::test_real_admitted_designated_evaluator_only_seam_for_kernel(
            &checked, "E", "result", "F",
        )
        .expect(
            "normal M9 pipeline can admit evaluator authority without every projected SYS-4 family",
        );

    assert_sys4_diag(
        SealedFabricAdmission::from_m9_execution_seam(
            &program,
            evaluator_only,
            initial_state_seed(checked.program_identity()),
        ),
        Sys4DiagnosticKind::IncompleteM9AuthorityInventory,
    );
}

#[test]
fn designated_cache_retry_revalidates_membership_witness_and_carrier_integrity() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("designated publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let semantic_identity = first.semantic_consumption_identity().to_string();
    let cache = fabric
        .designated_cache_entry(&semantic_identity)
        .expect("first consume installs a typed cache entry");
    assert!(
        cache.matches_semantic_identity_source_core_frontiers_version_policy_visibility_redaction(
            &semantic_identity,
            "E.result",
            "C",
            ResultVersion::new(1),
        )
    );

    let live_retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("live retry revalidates authority and returns cache");
    assert!(live_retry.returned_from_designated_cache_after_authority_revalidation());
    assert_eq!(
        fabric
            .m8_local_trace()
            .value_consumed_count(&semantic_identity, "C"),
        1
    );

    let mut membership_retired = boot(
        &checked,
        fabric_program(designated_projection(&checked)),
        BackendProfile::St,
    );
    membership_retired
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    membership_retired
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let transition = membership_retired
        .m9_authority_lifecycle_mut()
        .retire_designated_consumer_membership("E.result", "C")
        .expect("membership retirement is produced by M9 lifecycle");
    assert!(transition.is_opaque_m9_successor_generation());
    assert!(!transition.generation_ref().is_empty());
    assert!(!transition.consumer_lineage_ref().is_empty());
    assert!(transition.was_produced_by_m9_lifecycle());
    membership_retired
        .apply_admitted_authority_lifecycle(transition)
        .expect("membership retirement transition installs");
    let before_membership_retry = membership_retired.semantic_snapshot();
    let membership = assert_sys4_diag(
        membership_retired.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::MissingConsumerMembership,
    );
    assert!(membership.primary().typed_success().is_none());
    assert!(!membership.exposes_raw_payload());
    assert!(
        membership_retired
            .semantic_snapshot()
            .same_state(&before_membership_retry)
    );

    let mut witness_retired = boot(
        &checked,
        fabric_program(designated_projection(&checked)),
        BackendProfile::St,
    );
    witness_retired
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    witness_retired
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let transition = witness_retired
        .m9_authority_lifecycle_mut()
        .retire_designated_consumer_witness("E.result", "C")
        .expect("witness retirement is produced by M9 lifecycle");
    assert!(transition.is_opaque_m9_successor_generation());
    assert!(!transition.generation_ref().is_empty());
    assert!(!transition.consumer_lineage_ref().is_empty());
    assert!(transition.was_produced_by_m9_lifecycle());
    witness_retired
        .apply_admitted_authority_lifecycle(transition)
        .expect("witness retirement transition installs");
    let before_witness_retry = witness_retired.semantic_snapshot();
    let witness = assert_sys4_diag(
        witness_retired.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::MissingConsumerWitness,
    );
    assert!(witness.primary().typed_success().is_none());
    assert!(!witness.exposes_raw_payload());
    assert!(
        witness_retired
            .semantic_snapshot()
            .same_state(&before_witness_retry)
    );

    let mut visibility_fault = boot(
        &checked,
        fabric_program(designated_projection(&checked)),
        BackendProfile::St,
    );
    visibility_fault
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = visibility_fault
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let identity = first.semantic_consumption_identity().to_string();
    let publication_before = visibility_fault
        .m8_designated_publication_snapshot("E.result")
        .expect("M8 publication exists");
    let cache_before = visibility_fault
        .designated_cache_entry(&identity)
        .expect("cache exists")
        .stable_observer_safe_digest();
    visibility_fault
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_designated_carrier_visibility_redaction("E.result"),
        ))
        .expect("carrier-integrity fault is admitted without changing M9 authority");
    let before_visibility_retry = visibility_fault.semantic_snapshot();
    let visibility = assert_sys4_diag(
        visibility_fault.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::CarrierVisibilityRedactionMismatch,
    );
    assert!(visibility.primary().typed_success().is_none());
    assert!(!visibility.exposes_raw_payload());
    assert!(
        visibility_fault
            .semantic_snapshot()
            .same_state(&before_visibility_retry)
    );
    assert_eq!(
        visibility_fault
            .m8_designated_publication_snapshot("E.result")
            .expect("M8 publication still exists"),
        publication_before
    );
    assert_eq!(
        visibility_fault
            .designated_cache_entry(&identity)
            .expect("cache still exists")
            .stable_observer_safe_digest(),
        cache_before
    );
    assert_eq!(
        visibility_fault
            .m8_local_trace()
            .value_consumed_count(&identity, "C"),
        1
    );

    let mut policy_fault = boot(
        &checked,
        fabric_program(designated_projection(&checked)),
        BackendProfile::St,
    );
    policy_fault
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = policy_fault
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let identity = first.semantic_consumption_identity().to_string();
    let publication_before = policy_fault
        .m8_designated_publication_snapshot("E.result")
        .expect("M8 publication exists");
    let cache_before = policy_fault
        .designated_cache_entry(&identity)
        .expect("cache exists")
        .stable_observer_safe_digest();
    policy_fault
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_designated_carrier_policy_stamp("E.result"),
        ))
        .expect("carrier-integrity fault is admitted without changing M9 authority");
    let before_policy_retry = policy_fault.semantic_snapshot();
    let policy = assert_sys4_diag(
        policy_fault.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::CarrierPolicyMismatch,
    );
    assert!(policy.primary().typed_success().is_none());
    assert!(!policy.exposes_raw_payload());
    assert!(
        policy_fault
            .semantic_snapshot()
            .same_state(&before_policy_retry)
    );
    assert_eq!(
        policy_fault
            .m8_designated_publication_snapshot("E.result")
            .expect("M8 publication still exists"),
        publication_before
    );
    assert_eq!(
        policy_fault
            .designated_cache_entry(&identity)
            .expect("cache still exists")
            .stable_observer_safe_digest(),
        cache_before
    );
    assert_eq!(
        policy_fault
            .m8_local_trace()
            .value_consumed_count(&identity, "C"),
        1
    );
}

#[test]
fn m8_designated_consumption_trace_is_semantic_identity_and_consumer_specific() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("consume succeeds");
    let semantic_identity = first.semantic_consumption_identity();
    let delivery_trace = fabric
        .trace()
        .for_designated_delivery("E.result", first.delivery_id());

    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(semantic_identity, "C"),
        1
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count("wrong-semantic-identity", "C"),
        0
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(semantic_identity, "WrongConsumer"),
        0
    );
    let consume_node = fabric
        .m8_actual_trace()
        .designated_consume_node_id(semantic_identity, "C")
        .expect("actual M8 consume node is keyed by semantic identity and consumer");
    assert_eq!(
        fabric
            .m8_actual_trace()
            .node(&consume_node)
            .semantic_identity(),
        semantic_identity
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .node(&consume_node)
            .consumer_locus(),
        "C"
    );
    assert_eq!(
        delivery_trace.m8_value_consumed_count_for("wrong-semantic-identity", "C"),
        0,
        "trace-backed M8 count must be keyed by semantic identity, not by any consumed row"
    );
    assert_eq!(
        delivery_trace.m8_value_consumed_count_for(semantic_identity, "WrongConsumer"),
        0,
        "trace-backed M8 count must be keyed by consumer locus"
    );
}

#[test]
fn designated_missing_typed_payload_fails_closed_not_success_zero() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds before payload fault");
    let publication_before = fabric
        .m8_designated_publication_snapshot("E.result")
        .expect("M8 publication exists before carrier fault");
    let cache_before = fabric.designated_cache_snapshot();
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::strip_designated_carrier_int_payload("E.result"),
        ))
        .expect("bounded payload fault is admitted as observer-safe external action");

    let diagnostics = assert_sys4_diag(
        fabric.dispatch_source_action(consume_designated_action()),
        Sys4DiagnosticKind::MissingTypedDesignatedValue,
    );
    assert!(diagnostics.primary().typed_success().is_none());
    assert!(!diagnostics.exposes_raw_payload());
    assert_eq!(
        fabric
            .m8_designated_publication_snapshot("E.result")
            .expect("M8 publication remains intact after carrier fault"),
        publication_before
    );
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_ne!(
        fabric.designated_cache_contains_typed_value("E.result", "C", RuntimeValue::int(0)),
        true,
        "missing typed value must not be converted into success zero"
    );
}

#[test]
fn external_fault_dispatch_is_source_derived_observer_safe_and_cannot_target_or_mint() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let fault = ExternalAction::fault_event(FaultInjection::route_unavailable("attack"));
    assert!(fault.is_fault_event());
    assert_eq!(fault.target_locus_override(), None);
    assert_eq!(fault.authority_principal_override(), None);
    assert!(!fault.can_carry_checked_core_identity());
    assert!(!fault.can_carry_authority_grant());
    assert!(!fault.can_carry_state_delta());
    assert!(!fault.can_carry_expected_result());

    let receipt = fabric
        .dispatch_external_action(fault)
        .expect("fault event enters through the same external action API as source actions");
    assert!(receipt.is_fault());
    assert!(receipt.is_observer_safe());
    assert!(receipt.source_derived_from_operation("attack"));
    assert!(!receipt.exposes_raw_payload());

    let fault_trace = fabric.trace().for_fault(receipt.fault_id());
    assert!(fault_trace.all_entries_have_source_core_fragment_and_edge_provenance());
    assert!(fault_trace.all_entries_observer_safe());
    assert_eq!(fault_trace.target_locus_override(), None);
}

#[test]
fn st_and_ow1_dispatch_same_projected_program_with_same_semantic_correspondence() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    assert_eq!(
        program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible,
        "SYS-4 OW1 fixture has separate requester A but exactly one combined owner/source-owner locus S"
    );

    let admission = sealed_admission(&checked, &program);
    let source_action = owner_attack_action("attack");

    let mut st = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);
    let mut ow1 = boot_with_admission(program, admission, BackendProfile::Ow1);

    let st_receipt = st
        .dispatch_source_action(source_action.clone())
        .expect("ST dispatches the selected source action");
    let ow1_receipt = ow1
        .dispatch_source_action(source_action)
        .expect("OW1 dispatches the same selected source action");

    assert_eq!(st_receipt.typed_value(), ow1_receipt.typed_value());
    assert_eq!(st.semantic_snapshot(), ow1.semantic_snapshot());
    assert_eq!(
        st.trace()
            .canonical_correspondence_excluding_debug_worker_tokens(),
        ow1.trace()
            .canonical_correspondence_excluding_debug_worker_tokens()
    );
    assert_eq!(
        st.projected_artifact_identity(),
        ow1.projected_artifact_identity()
    );
}

#[test]
fn external_action_carries_only_source_operation_args_tick_or_fault_event() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let source_action = owner_attack_action("attack");

    assert_eq!(source_action.operation_id(), "attack");
    assert_eq!(source_action.origin_locus_override(), None);
    assert_eq!(source_action.authority_principal_override(), None);
    assert_eq!(source_action.target_locus_override(), None);
    assert!(!source_action.can_carry_checked_core_identity());
    assert!(!source_action.can_carry_authority_grant());
    assert!(!source_action.can_carry_state_delta());
    assert!(!source_action.can_carry_expected_result());

    let external = ExternalAction::source_operation(source_action);
    assert!(external.is_source_operation());
    assert_eq!(external.target_locus_override(), None);
    assert_eq!(
        program
            .derive_route_for_external_action(&external)
            .unwrap()
            .key(),
        &FabricRouteKey::owner_request("attack", "A", "S")
    );

    let forged_target =
        ExternalAction::for_test_attempt_target_override(owner_attack_action("attack"), "A");
    assert_sys4_diag(
        program.derive_route_for_external_action(&forged_target),
        Sys4DiagnosticKind::ExternalTargetOverrideRejected,
    );

    let forged_authority =
        ExternalAction::for_test_attempt_authority_override(owner_attack_action("attack"), "self");
    assert_sys4_diag(
        program.derive_route_for_external_action(&forged_authority),
        Sys4DiagnosticKind::ExternalAuthorityOverrideRejected,
    );

    let fault = ExternalAction::fault_event(FaultInjection::route_unavailable("attack"));
    assert!(fault.is_fault_event());
    assert!(!fault.can_carry_checked_core_identity());
    assert!(!fault.can_carry_authority_grant());
    assert!(!fault.can_carry_state_delta());
    assert!(!fault.can_carry_expected_result());
}

#[test]
fn sys4_dispatch_module_has_no_shortcut_dependencies_when_present() {
    let sources = collect_sys4_dispatch_sources();
    if sources.is_empty() {
        return;
    }

    let banned_needles = [
        "check_and_elaborate_surface_v0",
        "parse_surface_v0",
        "FixtureSource::new",
        "m10_reference_system",
        "expected JSON",
        "expected_json",
        "fixture name",
        "fixture_name",
        "global_remote_store",
        "remote_store_shortcut",
        "HashMap<LocusTag, RemoteStore>",
    ];

    for source in sources {
        let body = fs::read_to_string(&source).expect("sys4 dispatch source is readable");
        for needle in banned_needles {
            assert!(
                !body.contains(needle),
                "SYS-4 dispatch source must not depend on parser/checker/M10/fixture lookup/global store shortcut: {} contains {needle}",
                source.display()
            );
        }
    }
}

fn collect_sys4_dispatch_sources() -> Vec<PathBuf> {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    let root = crate_root.join("src/sys4_dispatch.rs");
    if root.exists() {
        sources.push(root);
    }

    let dir = crate_root.join("src/sys4_dispatch");
    if !dir.exists() {
        return sources;
    }

    let mut queue = VecDeque::from([dir]);
    while let Some(next) = queue.pop_front() {
        for entry in fs::read_dir(&next).expect("sys4 dispatch dir is readable") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                queue.push_back(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}
