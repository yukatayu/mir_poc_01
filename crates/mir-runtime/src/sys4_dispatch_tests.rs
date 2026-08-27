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

fn stage_designated_publish_until_delivery_outbox(fabric: &mut LocalFabric) {
    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("publish request is staged as E outbox input-request envelope");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());
    fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("transport moves the exact input request to S");
    fabric
        .step_locus("S")
        .expect("S validates source-release and emits input receipt");
    let input_receipt = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_receipt.typed_value(), RuntimeValue::int(10));
    fabric
        .step_transport("S", "E", input_receipt.envelope_id())
        .expect("transport moves the exact input receipt to E");
    fabric
        .step_locus("E")
        .expect("E installs the exact input receipt and emits result delivery");
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

    let current_m9 = fabric.current_m9_authority_inspection();
    let revocation = fabric
        .m9_authority_lifecycle_mut()
        .revoke_designated_consumer_capability("E.result", "C")
        .expect("revocation is produced through the admitted M9 authority lifecycle");
    let revocation_view = revocation.sealed_m9_inspection();
    assert_eq!(
        revocation_view.transition_kind(),
        crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerCapabilityRevoked
    );
    assert_eq!(revocation_view.prior_generation(), current_m9.generation());
    assert!(revocation_view.successor_generation().is_m9_produced());
    assert_eq!(
        revocation_view.consumer_lineage(),
        current_m9
            .designated_consumer_lineage("E.result", "C")
            .expect("current M9 output contains the C consumer lineage")
    );
    assert_eq!(
        revocation_view.consumer_lineage().consumer_locus(),
        "C",
        "successor evidence must identify the revoked consumer lineage without SYS4 minting it"
    );
    let expected_consumer_lineage = revocation_view.consumer_lineage().clone();
    let successor_generation = revocation_view.successor_generation().clone();
    let authority_before_apply = fabric.m8_authority_state_digest("C");
    fabric
        .apply_admitted_authority_lifecycle(revocation)
        .expect("fabric installs the M9 successor authority generation");
    assert_eq!(
        fabric.current_m9_authority_inspection().generation(),
        successor_generation
    );
    assert_ne!(
        fabric.m8_authority_state_digest("C"),
        authority_before_apply,
        "installing an M9 successor must refresh the target-owned M8 authority state"
    );
    let before_revoked_retry = fabric.semantic_snapshot();
    let cache_before_revoked_retry = fabric.designated_cache_snapshot();
    let consumed_before_revoked_retry = fabric
        .m8_actual_trace()
        .value_consumed_count(semantic_identity, "C");
    let m8_non_consuming_before_revoked_retry = fabric
        .m8_actual_trace()
        .non_consuming_designated_cache_validation_count(semantic_identity, "C");
    let submitted = fabric
        .submit_source_action(consume_designated_action())
        .expect("retry request is admitted before C validates live M9 authority");
    let revoked = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::MissingConsumerCapability,
    );
    assert_eq!(revoked.rejected_request_id(), Some(submitted.request_id()));
    let failure: &crate::m9_auth_verification::M9SealedFailureInspection = revoked
        .m9_failure_inspection()
        .expect("capability retry rejection must expose sealed M9 failure evidence");
    assert_eq!(
        failure.admission_error_kind(),
        crate::m9_auth_verification::M9AdmissionErrorKind::InvalidCapabilityLineage
    );
    assert_eq!(failure.installed_generation(), successor_generation);
    assert_eq!(failure.consumer_lineage(), &expected_consumer_lineage);
    assert_eq!(failure.request_id(), submitted.request_id());
    assert_eq!(failure.semantic_identity(), semantic_identity);
    assert_eq!(failure.consumer_locus(), "C");
    assert!(
        failure.rejected_before_m8_non_consuming_validation(),
        "M9 failure must close the retry before SYS4 asks M8 for a non-consuming cache validation"
    );
    assert!(revoked.m8_non_consuming_validation_node_id().is_none());
    assert_eq!(
        fabric
            .m8_actual_trace()
            .non_consuming_designated_cache_validation_count(semantic_identity, "C"),
        m8_non_consuming_before_revoked_retry
    );
    assert!(revoked.primary().typed_success().is_none());
    assert!(!revoked.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_revoked_retry));
    assert_eq!(
        fabric.designated_cache_snapshot(),
        cache_before_revoked_retry
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(semantic_identity, "C"),
        consumed_before_revoked_retry
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

    let live_generation = fabric.current_m9_authority_inspection();
    let live_retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("live retry revalidates authority and returns cache");
    assert!(live_retry.returned_from_designated_cache_after_authority_revalidation());
    let retry_validation = live_retry
        .m9_cache_validation()
        .expect("live cache retry exposes an observer-safe M9 validation occurrence");
    assert_eq!(retry_validation.generation(), live_generation.generation());
    assert_eq!(
        retry_validation.consumer_lineage(),
        live_generation
            .designated_consumer_lineage("E.result", "C")
            .expect("current M9 output has C consumer lineage")
    );
    assert_eq!(retry_validation.semantic_identity(), &semantic_identity);
    assert_eq!(retry_validation.consumer_locus(), "C");
    let m8_validation = fabric
        .m8_actual_trace()
        .non_consuming_designated_cache_validation(
            live_retry
                .m8_non_consuming_validation_node_id()
                .expect("retry uses actual admitted M8 non-consuming validation"),
        )
        .expect("M8 validation trace is queryable by semantic identity and consumer");
    assert_eq!(m8_validation.semantic_identity(), &semantic_identity);
    assert_eq!(m8_validation.consumer_locus(), "C");
    assert!(
        fabric
            .causality()
            .predecessor_ids(m8_validation.node_id())
            .contains(&retry_validation.occurrence_id().to_string()),
        "M8 non-consuming validation must causally depend on the current M9 validation occurrence"
    );
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
    let membership_first = membership_retired
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let membership_identity = membership_first.semantic_consumption_identity().to_string();
    let current_m9 = membership_retired.current_m9_authority_inspection();
    let transition = membership_retired
        .m9_authority_lifecycle_mut()
        .retire_designated_consumer_membership("E.result", "C")
        .expect("membership retirement is produced by M9 lifecycle");
    let transition_view = transition.sealed_m9_inspection();
    assert_eq!(
        transition_view.transition_kind(),
        crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerMembershipRetired
    );
    assert_eq!(transition_view.prior_generation(), current_m9.generation());
    assert!(transition_view.successor_generation().is_m9_produced());
    assert_eq!(
        transition_view.consumer_lineage(),
        current_m9
            .designated_consumer_lineage("E.result", "C")
            .expect("current M9 output has C consumer lineage")
    );
    assert_eq!(transition_view.consumer_lineage().consumer_locus(), "C");
    let expected_consumer_lineage = transition_view.consumer_lineage().clone();
    let successor_generation = transition_view.successor_generation().clone();
    let authority_before_apply = membership_retired.m8_authority_state_digest("C");
    membership_retired
        .apply_admitted_authority_lifecycle(transition)
        .expect("membership retirement transition installs");
    assert_eq!(
        membership_retired
            .current_m9_authority_inspection()
            .generation(),
        successor_generation
    );
    assert_ne!(
        membership_retired.m8_authority_state_digest("C"),
        authority_before_apply
    );
    let before_membership_retry = membership_retired.semantic_snapshot();
    let cache_before_membership = membership_retired.designated_cache_snapshot();
    let consumed_before_membership = membership_retired
        .m8_actual_trace()
        .value_consumed_count(&membership_identity, "C");
    let m8_non_consuming_before_membership = membership_retired
        .m8_actual_trace()
        .non_consuming_designated_cache_validation_count(&membership_identity, "C");
    let submitted = membership_retired
        .submit_source_action(consume_designated_action())
        .expect("retry request is admitted before C validates live M9 authority");
    let membership = assert_sys4_diag(
        membership_retired.step_locus("C"),
        Sys4DiagnosticKind::MissingConsumerMembership,
    );
    assert_eq!(
        membership.rejected_request_id(),
        Some(submitted.request_id())
    );
    let failure: &crate::m9_auth_verification::M9SealedFailureInspection = membership
        .m9_failure_inspection()
        .expect("membership retry rejection must expose sealed M9 failure evidence");
    assert_eq!(
        failure.admission_error_kind(),
        crate::m9_auth_verification::M9AdmissionErrorKind::InvalidMembershipLineage
    );
    assert_eq!(failure.installed_generation(), successor_generation);
    assert_eq!(failure.consumer_lineage(), &expected_consumer_lineage);
    assert_eq!(failure.request_id(), submitted.request_id());
    assert_eq!(failure.semantic_identity(), &membership_identity);
    assert_eq!(failure.consumer_locus(), "C");
    assert!(
        failure.rejected_before_m8_non_consuming_validation(),
        "M9 membership failure must close the retry before M8 non-consuming validation"
    );
    assert!(membership.m8_non_consuming_validation_node_id().is_none());
    assert_eq!(
        membership_retired
            .m8_actual_trace()
            .non_consuming_designated_cache_validation_count(&membership_identity, "C"),
        m8_non_consuming_before_membership
    );
    assert!(membership.primary().typed_success().is_none());
    assert!(!membership.exposes_raw_payload());
    assert!(
        membership_retired
            .semantic_snapshot()
            .same_state(&before_membership_retry)
    );
    assert_eq!(
        membership_retired.designated_cache_snapshot(),
        cache_before_membership
    );
    assert_eq!(
        membership_retired
            .m8_actual_trace()
            .value_consumed_count(&membership_identity, "C"),
        consumed_before_membership
    );

    let mut witness_retired = boot(
        &checked,
        fabric_program(designated_projection(&checked)),
        BackendProfile::St,
    );
    witness_retired
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let witness_first = witness_retired
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let witness_identity = witness_first.semantic_consumption_identity().to_string();
    let current_m9 = witness_retired.current_m9_authority_inspection();
    let transition = witness_retired
        .m9_authority_lifecycle_mut()
        .retire_designated_consumer_witness("E.result", "C")
        .expect("witness retirement is produced by M9 lifecycle");
    let transition_view = transition.sealed_m9_inspection();
    assert_eq!(
        transition_view.transition_kind(),
        crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerWitnessRetired
    );
    assert_eq!(transition_view.prior_generation(), current_m9.generation());
    assert!(transition_view.successor_generation().is_m9_produced());
    assert_eq!(
        transition_view.consumer_lineage(),
        current_m9
            .designated_consumer_lineage("E.result", "C")
            .expect("current M9 output has C consumer lineage")
    );
    assert_eq!(transition_view.consumer_lineage().consumer_locus(), "C");
    let expected_consumer_lineage = transition_view.consumer_lineage().clone();
    let successor_generation = transition_view.successor_generation().clone();
    let authority_before_apply = witness_retired.m8_authority_state_digest("C");
    witness_retired
        .apply_admitted_authority_lifecycle(transition)
        .expect("witness retirement transition installs");
    assert_eq!(
        witness_retired
            .current_m9_authority_inspection()
            .generation(),
        successor_generation
    );
    assert_ne!(
        witness_retired.m8_authority_state_digest("C"),
        authority_before_apply
    );
    let before_witness_retry = witness_retired.semantic_snapshot();
    let cache_before_witness = witness_retired.designated_cache_snapshot();
    let consumed_before_witness = witness_retired
        .m8_actual_trace()
        .value_consumed_count(&witness_identity, "C");
    let m8_non_consuming_before_witness = witness_retired
        .m8_actual_trace()
        .non_consuming_designated_cache_validation_count(&witness_identity, "C");
    let submitted = witness_retired
        .submit_source_action(consume_designated_action())
        .expect("retry request is admitted before C validates live M9 authority");
    let witness = assert_sys4_diag(
        witness_retired.step_locus("C"),
        Sys4DiagnosticKind::MissingConsumerWitness,
    );
    assert_eq!(witness.rejected_request_id(), Some(submitted.request_id()));
    let failure: &crate::m9_auth_verification::M9SealedFailureInspection = witness
        .m9_failure_inspection()
        .expect("witness retry rejection must expose sealed M9 failure evidence");
    assert_eq!(
        failure.admission_error_kind(),
        crate::m9_auth_verification::M9AdmissionErrorKind::InvalidCapabilityLineage
    );
    assert_eq!(failure.installed_generation(), successor_generation);
    assert_eq!(failure.consumer_lineage(), &expected_consumer_lineage);
    assert_eq!(failure.request_id(), submitted.request_id());
    assert_eq!(failure.semantic_identity(), &witness_identity);
    assert_eq!(failure.consumer_locus(), "C");
    assert!(
        failure.rejected_before_m8_non_consuming_validation(),
        "M9 witness failure must close the retry before M8 non-consuming validation"
    );
    assert!(witness.m8_non_consuming_validation_node_id().is_none());
    assert_eq!(
        witness_retired
            .m8_actual_trace()
            .non_consuming_designated_cache_validation_count(&witness_identity, "C"),
        m8_non_consuming_before_witness
    );
    assert!(witness.primary().typed_success().is_none());
    assert!(!witness.exposes_raw_payload());
    assert!(
        witness_retired
            .semantic_snapshot()
            .same_state(&before_witness_retry)
    );
    assert_eq!(
        witness_retired.designated_cache_snapshot(),
        cache_before_witness
    );
    assert_eq!(
        witness_retired
            .m8_actual_trace()
            .value_consumed_count(&witness_identity, "C"),
        consumed_before_witness
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
fn external_fault_dispatch_is_source_derived_observer_safe_and_cannot_target_or_mint() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let fault = ExternalAction::fault_event(FaultInjection::route_unavailable_for_edge(
        owner_request_edge.edge_ref(),
    ));
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
    assert!(receipt.source_derived_from_edge(owner_request_edge.edge_ref()));
    assert!(!receipt.exposes_raw_payload());

    let fault_trace = fabric.trace().for_fault(receipt.fault_id());
    assert!(fault_trace.all_entries_observer_safe());
    assert_eq!(fault_trace.target_locus_override(), None);
}

#[test]
fn staged_owner_mailbox_dispatch_requires_transport_and_locus_steps_before_m8() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let owner_reply_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerReplyReceipt, "S", "A")
        .expect("projection has owner reply edge");
    let program = fabric_program(projection.clone());
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let before_submit = fabric.semantic_snapshot();
    assert_eq!(
        fabric
            .locus_runtime("A")
            .expect("A exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .len(),
        0
    );
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .pending_envelopes()
            .len(),
        0
    );
    assert_eq!(fabric.m8_owner_queue_depth("S"), 0);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("source action admission only submits a generated carrier");
    assert_eq!(submitted.operation_id(), "attack");
    assert_eq!(submitted.origin_locus(), "A");
    assert_eq!(submitted.target_locus(), "S");
    assert!(fabric.semantic_snapshot().same_state(&before_submit));
    assert_eq!(fabric.m8_owner_queue_depth("S"), 0);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack", "S"),
        0,
        "submit must not synchronously enqueue M8 owner work"
    );

    let a_outbox = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes();
    assert_eq!(a_outbox.len(), 1);
    let request_envelope = a_outbox.single();
    assert_eq!(request_envelope.envelope_id(), submitted.envelope_id());
    assert_eq!(request_envelope.carrier_id(), submitted.carrier_id());
    assert_eq!(
        request_envelope.edge_kind(),
        CommunicationEdgeKind::OwnerRequest
    );
    assert_eq!(request_envelope.edge_ref(), owner_request_edge.edge_ref());
    assert_eq!(
        request_envelope.carrier_contract(),
        owner_request_edge.carrier_contract()
    );
    assert_eq!(
        request_envelope.source_ref(),
        owner_request_edge.source_ref()
    );
    assert_eq!(request_envelope.core_ref(), owner_request_edge.core_ref());
    assert_eq!(
        request_envelope.source_fragment_ref(),
        owner_request_edge.source_fragment_ref()
    );
    assert_eq!(
        request_envelope.target_fragment_ref(),
        owner_request_edge.target_fragment_ref()
    );

    let transport_to_s = fabric
        .step_transport("A", "S", request_envelope.envelope_id())
        .expect("transport moves the exact request envelope to S");
    assert_eq!(transport_to_s.envelope_id(), request_envelope.envelope_id());
    assert_eq!(transport_to_s.carrier_id(), request_envelope.carrier_id());
    assert_eq!(
        transport_to_s.source_outbox_dequeue_record_id(),
        request_envelope.mailbox_record_id()
    );
    let request_outbox_dequeue_id = transport_to_s
        .source_outbox_dequeue_occurrence_id()
        .to_string();
    let request_inbox_enqueue_id = transport_to_s
        .target_inbox_enqueue_occurrence_id()
        .to_string();
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(&request_inbox_enqueue_id),
        vec![request_outbox_dequeue_id.clone()],
        "transport must record outbox dequeue -> inbox enqueue before the target locus can observe the envelope"
    );
    assert_eq!(
        fabric
            .locus_runtime("A")
            .expect("A exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .len(),
        0
    );
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        request_envelope.envelope_id()
    );
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .pending_envelopes()
            .single()
            .mailbox_record_id(),
        transport_to_s.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack", "S"),
        0,
        "transport must not synchronously invoke M8"
    );
    assert!(fabric.semantic_snapshot().same_state(&before_submit));

    let s_step = fabric
        .step_locus("S")
        .expect("S validates and serves the exact owner request envelope");
    assert_eq!(
        s_step.consumed_envelope_id(),
        request_envelope.envelope_id()
    );
    assert_eq!(
        s_step.locus_dequeue_record_id(),
        transport_to_s.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(s_step.locus_dequeue_occurrence_id()),
        vec![request_inbox_enqueue_id.clone()],
        "S locus dequeue must causally depend on the transport inbox enqueue"
    );
    assert_eq!(
        s_step.m9_validation().owner_lineage_ref(),
        request_envelope.m9_owner_lineage_ref()
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(s_step.m8_request_node_id())
            .contains(&s_step.locus_dequeue_occurrence_id().to_string()),
        "M8 owner request must not begin before S dequeues and validates the exact envelope"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(s_step.m8_serve_node_id())
            .contains(&s_step.m8_request_node_id().to_string())
    );
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
    let reply = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(reply.envelope_id(), s_step.reply_envelope_id());
    assert_eq!(reply.edge_kind(), CommunicationEdgeKind::OwnerReplyReceipt);
    assert_eq!(reply.edge_ref(), owner_reply_edge.edge_ref());
    assert_eq!(
        reply.carrier_contract(),
        owner_reply_edge.carrier_contract()
    );
    assert_eq!(reply.request_carrier_id(), request_envelope.carrier_id());

    let reply_move = fabric
        .step_transport("S", "A", reply.envelope_id())
        .expect("transport moves the exact reply envelope to A");
    assert_eq!(reply_move.envelope_id(), reply.envelope_id());
    assert_eq!(
        reply_move.source_outbox_dequeue_record_id(),
        reply.mailbox_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(reply_move.target_inbox_enqueue_occurrence_id()),
        vec![reply_move.source_outbox_dequeue_occurrence_id().to_string()],
        "reply transport must record S outbox dequeue -> A inbox enqueue"
    );
    assert_eq!(
        fabric
            .locus_runtime("A")
            .expect("A exists")
            .incoming_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        reply.envelope_id()
    );

    let a_step = fabric
        .step_locus("A")
        .expect("A consumes the exact reply into a receipt");
    assert_eq!(a_step.consumed_envelope_id(), reply.envelope_id());
    assert_eq!(
        a_step.locus_dequeue_record_id(),
        reply_move.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(a_step.locus_dequeue_occurrence_id()),
        vec![reply_move.target_inbox_enqueue_occurrence_id().to_string()]
    );
    let receipt = a_step.receipt().expect("reply step materializes receipt");
    assert_eq!(receipt.operation_id(), "attack");
    assert_eq!(receipt.typed_value(), RuntimeValue::unit());
}

#[test]
fn staged_owner_wrong_target_rejects_before_target_dequeue_or_m8_mutation() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                "not-a-projected-edge",
                "not-a-live-envelope",
                "A",
            ),
        )),
        Sys4DiagnosticKind::UnknownProjectedEdge,
    );

    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                owner_request_edge.edge_ref(),
                "not-a-live-envelope",
                "A",
            ),
        )),
        Sys4DiagnosticKind::UnavailableEnvelope,
    );

    let first = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("first request carrier is submitted to A outbox");
    let second = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("second same-edge request carrier is submitted to A outbox");
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                owner_request_edge.edge_ref(),
                first.envelope_id(),
                "A",
            ),
        ))
        .expect("fault is bound to the first live same-edge envelope only");

    fabric
        .step_transport("A", "S", second.envelope_id())
        .expect("same-edge sibling envelope remains unaffected by exact-envelope retarget fault");
    fabric
        .step_locus("S")
        .expect("S serves the unaffected sibling request");
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
    let before_reject = fabric.semantic_snapshot();
    let rejected = assert_sys4_diag(
        fabric.step_transport("A", "S", first.envelope_id()),
        Sys4DiagnosticKind::WrongTargetLocus,
    );
    assert_eq!(rejected.rejected_envelope_id(), Some(first.envelope_id()));
    assert!(rejected.endpoint_dequeue_occurrence_id().is_none());
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .pending_envelopes()
            .is_empty()
    );
    assert_eq!(fabric.m8_owner_queue_depth("S"), 0);
    assert!(fabric.semantic_snapshot().same_state(&before_reject));
}

#[test]
fn staged_designated_path_requires_source_release_receipt_before_evaluation_and_consume() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let input_receipt_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputReceipt,
            "S",
            "E",
        )
        .expect("projection has input receipt edge");
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has result delivery edge");
    let program = fabric_program(projection.clone());
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let before_publish = fabric.semantic_snapshot();
    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("publish submission only emits a designated input request");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());
    assert_eq!(
        input_request.edge_kind(),
        CommunicationEdgeKind::DesignatedInputRequest
    );
    assert_eq!(
        input_request.carrier_contract(),
        input_request_edge.carrier_contract()
    );
    assert_eq!(input_request.source_ref(), input_request_edge.source_ref());
    assert_eq!(input_request.core_ref(), input_request_edge.core_ref());
    assert!(fabric.semantic_snapshot().same_state(&before_publish));
    assert_eq!(
        fabric
            .m8_actual_trace()
            .designated_evaluation_count("E.result"),
        0,
        "submit must not directly read S or evaluate at E"
    );

    let request_transport = fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("transport moves exact input request to S");
    assert_eq!(
        request_transport.source_outbox_dequeue_record_id(),
        input_request.mailbox_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(request_transport.target_inbox_enqueue_occurrence_id()),
        vec![
            request_transport
                .source_outbox_dequeue_occurrence_id()
                .to_string()
        ],
        "input-request transport must record E outbox dequeue -> S inbox enqueue"
    );
    let m9_before_source_read = fabric.current_m9_authority_inspection();
    let source_step = fabric
        .step_locus("S")
        .expect("S validates source-release lineage before reading local state");
    assert_eq!(
        source_step.consumed_envelope_id(),
        input_request.envelope_id()
    );
    assert_eq!(
        source_step.locus_dequeue_record_id(),
        request_transport.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(source_step.locus_dequeue_occurrence_id()),
        vec![
            request_transport
                .target_inbox_enqueue_occurrence_id()
                .to_string()
        ]
    );
    let source_release_validation = source_step.m9_validation().source_release_inspection();
    assert_eq!(
        source_release_validation.generation(),
        m9_before_source_read.generation()
    );
    assert_eq!(
        source_release_validation.lineage(),
        input_request.m9_source_release_lineage()
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(
                source_step
                    .local_store_read_audit()
                    .expect("S read audit exists only after source-release validation")
                    .occurrence_id()
            )
            .contains(&source_release_validation.occurrence_id().to_string()),
        "S local-store read audit must causally depend on M9 source-release validation"
    );
    assert_eq!(
        source_step.local_store_reads(),
        vec![RuntimeStoreRead::int("S", "player", "self", "atk", 10)]
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .designated_evaluation_count("E.result"),
        0,
        "source owner read must not evaluate E before receipt transport"
    );
    let input_receipt = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(
        input_receipt.edge_kind(),
        CommunicationEdgeKind::DesignatedInputReceipt
    );
    assert_eq!(
        input_receipt.request_carrier_id(),
        input_request.carrier_id()
    );
    assert_eq!(
        input_receipt.carrier_contract(),
        input_receipt_edge.carrier_contract()
    );
    assert_eq!(input_receipt.typed_value(), RuntimeValue::int(10));

    let receipt_transport = fabric
        .step_transport("S", "E", input_receipt.envelope_id())
        .expect("transport moves exact input receipt to E");
    assert_eq!(
        receipt_transport.source_outbox_dequeue_record_id(),
        input_receipt.mailbox_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(receipt_transport.target_inbox_enqueue_occurrence_id()),
        vec![
            receipt_transport
                .source_outbox_dequeue_occurrence_id()
                .to_string()
        ],
        "input-receipt transport must record S outbox dequeue -> E inbox enqueue"
    );
    let evaluator_step = fabric
        .step_locus("E")
        .expect("E installs input receipt in M8 and then evaluates");
    assert_eq!(
        evaluator_step.consumed_envelope_id(),
        input_receipt.envelope_id()
    );
    assert_eq!(
        evaluator_step.locus_dequeue_record_id(),
        receipt_transport.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(evaluator_step.locus_dequeue_occurrence_id()),
        vec![
            receipt_transport
                .target_inbox_enqueue_occurrence_id()
                .to_string()
        ]
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(evaluator_step.m8_input_receipt_node_id())
            .contains(&evaluator_step.locus_dequeue_occurrence_id().to_string()),
        "E must install the exact S→E input receipt before M8 evaluation"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(evaluator_step.m8_evaluation_node_id())
            .contains(&evaluator_step.m8_input_receipt_node_id().to_string())
    );
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(
        delivery.edge_kind(),
        CommunicationEdgeKind::DesignatedResultDelivery
    );
    assert_eq!(
        delivery.input_receipt_carrier_id(),
        input_receipt.carrier_id()
    );
    assert_eq!(
        delivery.carrier_contract(),
        delivery_edge.carrier_contract()
    );
    assert_eq!(
        delivery.input_frontier(),
        delivery_edge.carrier_contract().input_frontier()
    );
    assert_eq!(
        delivery.result_frontier(),
        delivery_edge.carrier_contract().result_frontier()
    );
    assert_eq!(
        delivery.observation_policy(),
        delivery_edge.carrier_contract().observation_policy()
    );
    assert_eq!(
        delivery.policy_stamp(),
        delivery_edge.carrier_contract().policy_stamp()
    );
    assert_eq!(
        delivery.visibility_policy(),
        delivery_edge.carrier_contract().visibility_policy()
    );
    assert_eq!(delivery.typed_value(), RuntimeValue::int(11));

    let delivery_transport = fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("transport moves immutable delivery to C");
    assert_eq!(
        delivery_transport.source_outbox_dequeue_record_id(),
        delivery.mailbox_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(delivery_transport.target_inbox_enqueue_occurrence_id()),
        vec![
            delivery_transport
                .source_outbox_dequeue_occurrence_id()
                .to_string()
        ],
        "delivery transport must record E outbox dequeue -> C inbox enqueue"
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        delivery.envelope_id()
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(delivery.semantic_identity(), "C"),
        0,
        "delivery transport alone must not consume M8 designated value"
    );

    let consumer_step = fabric
        .step_locus("C")
        .expect("C dequeues exact delivery and first-consumes/caches");
    assert_eq!(consumer_step.consumed_envelope_id(), delivery.envelope_id());
    assert_eq!(
        consumer_step.locus_dequeue_record_id(),
        delivery_transport.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(consumer_step.locus_dequeue_occurrence_id()),
        vec![
            delivery_transport
                .target_inbox_enqueue_occurrence_id()
                .to_string()
        ]
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(consumer_step.m8_consume_node_id())
            .contains(&consumer_step.locus_dequeue_occurrence_id().to_string()),
        "M8 consume must not precede C locus dequeue of the generated delivery"
    );
    assert_eq!(
        consumer_step
            .receipt()
            .expect("C step returns receipt")
            .typed_value(),
        RuntimeValue::int(11)
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(delivery.semantic_identity(), "C"),
        1
    );
    let cache_entry = fabric
        .designated_cache_entry(delivery.semantic_identity())
        .expect("first consume installs a cache entry keyed by the delivery semantic identity");
    assert_eq!(
        cache_entry.sealed_delivery_binding_digest(),
        delivery.immutable_delivery_digest(),
        "cache digest must be derived from the exact immutable delivery envelope, not recomputed from local literals"
    );
    let cache_binding = cache_entry.sealed_delivery_binding();
    assert_eq!(cache_binding, delivery.immutable_delivery_binding());
    assert_eq!(cache_binding.source_ref(), delivery.source_ref());
    assert_eq!(cache_binding.core_ref(), delivery.core_ref());
    assert_eq!(
        cache_binding.source_fragment_ref(),
        delivery.source_fragment_ref()
    );
    assert_eq!(
        cache_binding.target_fragment_ref(),
        delivery.target_fragment_ref()
    );
    assert_eq!(cache_binding.input_frontier(), delivery.input_frontier());
    assert_eq!(cache_binding.result_frontier(), delivery.result_frontier());
    assert_eq!(cache_binding.result_version(), ResultVersion::new(1));
    assert_eq!(cache_binding.consumer_locus(), "C");
    assert_eq!(cache_binding.policy_stamp(), delivery.policy_stamp());
    assert_eq!(
        cache_binding.visibility_policy(),
        delivery.visibility_policy()
    );
    assert_eq!(
        cache_binding.redaction_policy(),
        delivery.redaction_policy()
    );

    let retry = fabric
        .submit_source_action(consume_designated_action())
        .expect("retry submits local C cache validation");
    let retry_m9_generation = fabric.current_m9_authority_inspection();
    let retry_step = fabric
        .step_locus("C")
        .expect("retry validates live M9/M8 authority without semantic consumption");
    assert_eq!(retry_step.request_id(), retry.request_id());
    assert_eq!(retry_step.semantic_identity(), delivery.semantic_identity());
    let retry_m9_validation = retry_step
        .m9_cache_validation()
        .expect("retry exposes a typed M9 cache validation evidence object");
    assert_eq!(
        retry_m9_validation.generation(),
        retry_m9_generation.generation()
    );
    assert_eq!(
        retry_m9_validation.consumer_lineage(),
        retry_m9_generation
            .designated_consumer_lineage("E.result", "C")
            .expect("current M9 output has C consumer lineage")
    );
    assert_eq!(
        retry_m9_validation.semantic_identity(),
        delivery.semantic_identity()
    );
    assert_eq!(retry_m9_validation.consumer_locus(), "C");
    let retry_m8_validation = fabric
        .m8_actual_trace()
        .non_consuming_designated_cache_validation(
            retry_step
                .m8_non_consuming_validation_node_id()
                .expect("retry uses actual M8 non-consuming validation"),
        )
        .expect("M8 non-consuming validation is queryable as semantic evidence");
    assert_eq!(
        retry_m8_validation.semantic_identity(),
        delivery.semantic_identity()
    );
    assert_eq!(retry_m8_validation.consumer_locus(), "C");
    assert!(
        fabric
            .causality()
            .predecessor_ids(retry_m8_validation.node_id())
            .contains(&retry_m9_validation.occurrence_id().to_string()),
        "M8 non-consuming validation must depend on the live M9 validation occurrence"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(delivery.semantic_identity(), "C"),
        1
    );
}

#[test]
fn staged_designated_source_release_invalidation_rejects_before_s_read_or_receipt() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("publish request is staged as E outbox input-request envelope");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.edge_ref(), input_request_edge.edge_ref());
    fabric
        .step_transport("E", "S", submitted.envelope_id())
        .expect("transport moves the exact input request to S");

    let current_m9 = fabric.current_m9_authority_inspection();
    let transition = fabric
        .m9_authority_lifecycle_mut()
        .revoke_designated_source_release(input_request.m9_source_release_lineage())
        .expect("source-release revocation is produced by the admitted M9 lifecycle");
    let transition_view = transition.sealed_m9_inspection();
    assert_eq!(
        transition_view.transition_kind(),
        crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedSourceReleaseRevoked
    );
    assert_eq!(transition_view.prior_generation(), current_m9.generation());
    assert_eq!(
        transition_view.source_release_lineage(),
        current_m9
            .designated_source_release_lineage(
                "E",
                "result",
                "S",
                0,
                input_request.input_frontier(),
            )
            .expect("current M9 output has the source-release lineage used by this carrier")
    );
    let successor_generation = transition_view.successor_generation().clone();
    let authority_before_apply = fabric.m8_authority_state_digest("S");
    fabric
        .apply_admitted_authority_lifecycle(transition)
        .expect("fabric installs the M9 source-release successor");
    assert_eq!(
        fabric.current_m9_authority_inspection().generation(),
        successor_generation
    );
    assert_ne!(
        fabric.m8_authority_state_digest("S"),
        authority_before_apply
    );

    let before_reject = fabric.semantic_snapshot();
    let read_audit_before = fabric.local_store_read_audit("S").stable_digest();
    let s_outbox_before = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .len();
    let m8_before = fabric.m8_actual_trace().stable_digest();
    let cache_before = fabric.designated_cache_snapshot();
    let rejected = assert_sys4_diag(
        fabric.step_locus("S"),
        Sys4DiagnosticKind::MissingSourceReleaseAuthority,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(submitted.envelope_id())
    );
    assert!(rejected.local_store_read_audit_id().is_none());
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(fabric.semantic_snapshot().same_state(&before_reject));
    assert_eq!(
        fabric.local_store_read_audit("S").stable_digest(),
        read_audit_before
    );
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .len(),
        s_outbox_before,
        "S must not enqueue a DesignatedInputReceipt after source-release invalidation"
    );
    assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
}

#[test]
fn staged_fault_selector_rejects_mismatched_envelope_association_without_corrupting_live_carrier() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has result delivery edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("input-request envelope is live in E outbox");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.edge_ref(), input_request_edge.edge_ref());
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());

    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::drop_in_transit_envelope_payload_for_edge(
                delivery_edge.edge_ref(),
                input_request.envelope_id(),
            ),
        )),
        Sys4DiagnosticKind::FaultEnvelopeRouteMismatch,
    );
    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                delivery_edge.edge_ref(),
                input_request.envelope_id(),
                "C",
            ),
        )),
        Sys4DiagnosticKind::FaultEnvelopeRouteMismatch,
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_exact_envelope(delivery_edge.edge_ref(), input_request.envelope_id())
    );
    assert_eq!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        input_request.envelope_id(),
        "mismatched fault selectors must not drop or corrupt the live input-request envelope"
    );
    fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("unaffected input request still transports after rejected selector attempts");
    let source_step = fabric
        .step_locus("S")
        .expect("S can still validate and read the unaffected input request");
    assert_eq!(
        source_step.local_store_reads(),
        vec![RuntimeStoreRead::int("S", "player", "self", "atk", 10)]
    );
}

#[test]
fn staged_designated_delivery_payload_fault_rejects_at_c_before_consume_cache_mutation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let input_receipt_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputReceipt,
            "S",
            "E",
        )
        .expect("projection has input receipt edge");
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has result delivery edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(delivery.edge_ref(), delivery_edge.edge_ref());
    assert_eq!(
        delivery.carrier_contract(),
        delivery_edge.carrier_contract()
    );
    let semantic_identity = delivery.semantic_identity().to_string();
    let before_fault = fabric.semantic_snapshot();
    let m8_before = fabric.m8_actual_trace().stable_digest();
    let cache_before = fabric.designated_cache_snapshot();
    let publication_before = fabric
        .m8_designated_publication_snapshot("E.result")
        .expect("M8 publication exists before carrier fault");

    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::drop_in_transit_envelope_payload_for_edge(
                "not-a-projected-edge",
                delivery.envelope_id(),
            ),
        )),
        Sys4DiagnosticKind::UnknownProjectedEdge,
    );
    assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::drop_in_transit_envelope_payload_for_edge(
                delivery_edge.edge_ref(),
                "not-a-live-envelope",
            ),
        )),
        Sys4DiagnosticKind::UnavailableEnvelope,
    );
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::drop_in_transit_envelope_payload_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
            ),
        ))
        .expect("payload fault targets one checked delivery edge/envelope");
    assert!(
        fabric
            .in_transit_faults()
            .affects_exact_envelope(delivery_edge.edge_ref(), delivery.envelope_id())
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_request_edge.edge_ref()),
        "faulting E→C delivery must not corrupt the E→S input request route"
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_receipt_edge.edge_ref()),
        "faulting E→C delivery must not corrupt the S→E input receipt route"
    );

    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("transport moves the corrupted delivery carrier to C");
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::MissingTypedDesignatedValue,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .pending_envelopes()
            .is_empty(),
        "malformed carrier must leave the active inbox after terminal rejection"
    );
    let quarantine = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .terminal_rejected_envelope(delivery.envelope_id())
        .expect("malformed delivery is recorded in terminal rejected/quarantine state");
    assert_eq!(
        quarantine.terminal_state(),
        crate::sys4_dispatch::MailboxEnvelopeTerminalState::RejectedQuarantined
    );
    assert_eq!(
        quarantine.diagnostic_kind(),
        Sys4DiagnosticKind::MissingTypedDesignatedValue
    );
    assert!(quarantine.observer_safe_audit().is_observer_safe());
    assert!(fabric.semantic_snapshot().same_state(&before_fault));
    assert_eq!(
        fabric
            .m8_designated_publication_snapshot("E.result")
            .expect("M8 publication remains intact after carrier fault"),
        publication_before
    );
    assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        0
    );

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let clean_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("E", "C", clean_delivery.envelope_id())
        .expect("clean delivery transports after prior malformed envelope quarantine");
    let clean_step = fabric
        .step_locus("C")
        .expect("prior quarantined envelope does not head-block a later clean delivery");
    assert_eq!(
        clean_step
            .receipt()
            .expect("clean C step has receipt")
            .typed_value(),
        RuntimeValue::int(11)
    );
}

#[test]
fn staged_designated_delivery_policy_fault_rejects_at_c_before_consume_cache_mutation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let input_receipt_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputReceipt,
            "S",
            "E",
        )
        .expect("projection has input receipt edge");
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has result delivery edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(delivery.edge_ref(), delivery_edge.edge_ref());
    let semantic_identity = delivery.semantic_identity().to_string();
    let before_fault = fabric.semantic_snapshot();
    let m8_before = fabric.m8_actual_trace().stable_digest();
    let cache_before = fabric.designated_cache_snapshot();

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_in_transit_envelope_policy_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
            ),
        ))
        .expect("policy fault targets one checked delivery edge/envelope");
    assert!(
        fabric
            .in_transit_faults()
            .affects_exact_envelope(delivery_edge.edge_ref(), delivery.envelope_id())
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_request_edge.edge_ref())
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_receipt_edge.edge_ref())
    );

    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("transport moves the policy-corrupted delivery carrier to C");
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::CarrierPolicyMismatch,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    let quarantine = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .terminal_rejected_envelope(delivery.envelope_id())
        .expect("policy-corrupted delivery is terminally quarantined");
    assert_eq!(
        quarantine.diagnostic_kind(),
        Sys4DiagnosticKind::CarrierPolicyMismatch
    );
    assert!(quarantine.observer_safe_audit().is_observer_safe());
    assert!(fabric.semantic_snapshot().same_state(&before_fault));
    assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        0
    );
}

#[test]
fn staged_designated_delivery_redaction_fault_rejects_at_c_before_consume_cache_mutation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let input_request_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S",
        )
        .expect("projection has input request edge");
    let input_receipt_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedInputReceipt,
            "S",
            "E",
        )
        .expect("projection has input receipt edge");
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has result delivery edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(delivery.edge_ref(), delivery_edge.edge_ref());
    let semantic_identity = delivery.semantic_identity().to_string();
    let before_fault = fabric.semantic_snapshot();
    let m8_before = fabric.m8_actual_trace().stable_digest();
    let cache_before = fabric.designated_cache_snapshot();

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_in_transit_envelope_redaction_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
            ),
        ))
        .expect("redaction fault targets one checked delivery edge/envelope");
    assert!(
        fabric
            .in_transit_faults()
            .affects_exact_envelope(delivery_edge.edge_ref(), delivery.envelope_id())
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_request_edge.edge_ref())
    );
    assert!(
        !fabric
            .in_transit_faults()
            .affects_edge(input_receipt_edge.edge_ref())
    );

    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("transport moves the redaction-corrupted delivery carrier to C");
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::CarrierRedactionMismatch,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    let quarantine = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .terminal_rejected_envelope(delivery.envelope_id())
        .expect("redaction-corrupted delivery is terminally quarantined");
    assert_eq!(
        quarantine.diagnostic_kind(),
        Sys4DiagnosticKind::CarrierRedactionMismatch
    );
    assert!(quarantine.observer_safe_audit().is_observer_safe());
    assert!(fabric.semantic_snapshot().same_state(&before_fault));
    assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        0
    );
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
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
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

    let fault = ExternalAction::fault_event(FaultInjection::route_unavailable_for_edge(
        owner_request_edge.edge_ref(),
    ));
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
