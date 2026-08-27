use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::ResultVersion,
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSurfaceV0, M7DiagnosticKind, check_and_elaborate_surface_v0,
    },
};

use crate::{
    m8_runtime_local_cut::{M8LocalTrace, M8LocalTraceKind, M8LocalTraceObservation},
    m9_auth_verification::M9RuntimeExecutionSeam,
    sys3_projection::{
        BackendEligibility, BackendIneligibilityReason, BackendProfile, CommunicationEdge,
        CommunicationEdgeKind, DeclaredLogicalTopology, GlobalProjectionResult,
        ProjectedOperationFragmentKind, RuntimeAdmissionStatus, project_checked_core,
    },
    sys4_dispatch::{
        CachedDelivery, CausalityGraph, EndpointCarrierRecord, ExternalAction, FabricProgram,
        FabricReceipt, FabricRouteKey, FabricSemanticSnapshot, FabricTrace, FaultInjection,
        LocalFabric, MailboxEnvelope, RuntimeStoreRead, RuntimeStoreWrite, RuntimeValue,
        SealedDeliveryBinding, SealedFabricAdmission, SourceAction, Sys4DiagnosticKind,
        Sys4DispatchDiagnostics, Sys4InitialStateSeed, Sys4LocalCut, Sys4TraceEntry, Sys4TraceKind,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER_ENDPOINT_FIXTURE: &str = "sys4_ow1_endpoint_crossing.mir";
const DESIGNATED_CONSUME_FIXTURE: &str = "sys4_designated_consume_with_auth.mir";
const RELATION_ONLY_FIXTURE: &str = "maintained_bird_relation.mir";
const FOUR_LOCUS_FIXTURE: &str = "sys4_two_owner_four_locus_with_auth.mir";

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

fn relation_only_checked() -> CheckedSurfaceV0 {
    load_checked_fixture(RELATION_ONLY_FIXTURE)
}

fn four_locus_checked() -> CheckedSurfaceV0 {
    load_checked_fixture(FOUR_LOCUS_FIXTURE)
}

fn checked_inline(relative: &str, source: &str) -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(relative, source.to_string()))
        .expect("inline SYS-4 source is parsed, checked, and elaborated by the real M7 pipeline")
}

fn combined_owner_designated_checked() -> CheckedSurfaceV0 {
    checked_inline(
        "tests/inline/sys4_combined_owner_designated_same_source_owner.mir",
        r#"
module Combat.Sys4.CombinedOwnerDesignatedSameSourceOwner

locus A
locus S
locus E
locus C
principal self
principal target
type Player

state player[id: Player] at S {
  hp: Int
  atk: Int
}

Role[self] at A {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {
    at S {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}

designated evaluate E on tick F publish result = player[self].atk + 1
designated consume E.result at C

with auth MembershipAuth

verify finite_refinement
"#,
    )
}

fn owner_endpoint_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["A", "S"])
}

fn designated_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["C", "E", "S"])
}

fn relation_only_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["C", "S"])
}

fn four_locus_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["A", "S", "T", "V"])
}

fn combined_owner_designated_projection(checked: &CheckedSurfaceV0) -> GlobalProjectionResult {
    project_fixture(checked, ["A", "S", "E", "C"])
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

fn m9_four_locus_multi_owner_seam(checked: &CheckedSurfaceV0) -> M9RuntimeExecutionSeam {
    M9RuntimeExecutionSeam::test_real_admitted_multi_owner_seam_for_kernel(
        checked,
        [("attack_s", "self", "S"), ("attack_t", "self", "T")],
    )
    .expect("four-locus SYS-4 test uses real M9 multi-owner admission")
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

fn four_locus_initial_state_seed(identity: &CheckedProgramIdentity) -> Sys4InitialStateSeed {
    Sys4InitialStateSeed::for_checked_program(identity.clone())
        .with_int("S", "player", "self", "hp", 100)
        .with_int("S", "player", "self", "atk", 10)
        .with_int("T", "shield", "self", "hp", 200)
        .with_int("T", "shield", "self", "atk", 7)
}

fn sealed_four_locus_admission(
    checked: &CheckedSurfaceV0,
    program: &FabricProgram,
) -> SealedFabricAdmission {
    SealedFabricAdmission::from_m9_execution_seam(
        program,
        m9_four_locus_multi_owner_seam(checked),
        four_locus_initial_state_seed(checked.program_identity()),
    )
    .expect("complete real M9 multi-owner seam plus explicit seed admits four-locus fabric")
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

fn save_sys4_local_cut(
    fabric: &mut LocalFabric,
    cut_id: &str,
) -> Result<Sys4LocalCut, Sys4DispatchDiagnostics> {
    fabric.save_local_cut(cut_id)
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

fn missing_designated_tick_diag() -> Sys4DiagnosticKind {
    Sys4DiagnosticKind::MissingDesignatedTick
}

fn delivery_publication_identity_mismatch_diag() -> Sys4DiagnosticKind {
    Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch
}

fn cache_binding_digest_mismatch_diag() -> Sys4DiagnosticKind {
    Sys4DiagnosticKind::CacheBindingDigestMismatch
}

fn assert_envelope_tick(envelope: &MailboxEnvelope, tick: &str, frontier: &str) {
    assert_eq!(envelope.designated_tick_id(), tick);
    assert_eq!(envelope.designated_tick_frontier(), frontier);
}

fn envelope_m8_publication_id(envelope: &MailboxEnvelope) -> &str {
    envelope.m8_publication_id()
}

fn envelope_logical_tick_id(envelope: &MailboxEnvelope) -> &str {
    envelope.logical_tick_id()
}

fn envelope_logical_tick_frontier(envelope: &MailboxEnvelope) -> &str {
    envelope.logical_tick_frontier()
}

fn binding_m8_publication_id(binding: &SealedDeliveryBinding) -> &str {
    binding.m8_publication_id()
}

fn binding_logical_tick_id(binding: &SealedDeliveryBinding) -> &str {
    binding.logical_tick_id()
}

fn binding_logical_tick_frontier(binding: &SealedDeliveryBinding) -> &str {
    binding.logical_tick_frontier()
}

fn is_local_cache_retry(envelope: &MailboxEnvelope) -> bool {
    envelope.is_local_cache_retry()
}

fn receipt_m8_publication_id(receipt: &FabricReceipt) -> &str {
    receipt.m8_publication_id()
}

fn receipt_logical_tick_id(receipt: &FabricReceipt) -> &str {
    receipt.logical_tick_id()
}

fn m8_local_runtime_trace(fabric: &LocalFabric) -> &M8LocalTrace {
    let trace: &M8LocalTrace = fabric.m8_local_runtime_trace();
    trace
}

fn assert_backend_publication_observation(
    fabric: &LocalFabric,
    node_id: &str,
    tick: &str,
    frontier: &str,
) {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    let observation: M8LocalTraceObservation = trace
        .observation(node_id)
        .expect("node id must query an actual M8-owned backend publication observation");
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedValuePublished
    );
    assert_eq!(observation.logical_tick_id(), tick);
    assert_eq!(observation.logical_tick_frontier(), frontier);
}

fn assert_backend_consume_observation(
    fabric: &LocalFabric,
    node_id: &str,
    publication_id: &str,
    tick: &str,
) {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    let observation: M8LocalTraceObservation = trace
        .observation(node_id)
        .expect("node id must query an actual M8-owned backend consume observation");
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedValueConsumed
    );
    assert_eq!(observation.m8_publication_id(), publication_id);
    assert_eq!(observation.logical_tick_id(), tick);
}

fn assert_backend_cache_validation_observation(
    fabric: &LocalFabric,
    node_id: &str,
    semantic_identity: &str,
    consumer: &str,
    publication_id: &str,
    tick: &str,
    previous_sequence: u64,
) {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    let observation: M8LocalTraceObservation = trace
        .observation(node_id)
        .expect("node id must query an actual M8-owned non-consuming cache validation occurrence");
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedCacheValidated
    );
    assert!(
        observation.sequence() > previous_sequence,
        "cache retry must emit a fresh M8 trace occurrence after prior M8 events"
    );
    assert_eq!(observation.semantic_identity(), semantic_identity);
    assert_eq!(observation.consumer_locus(), consumer);
    assert_eq!(observation.m8_publication_id(), publication_id);
    assert_eq!(observation.logical_tick_id(), tick);
}

fn assert_backend_consumption_rejection_observation(
    fabric: &LocalFabric,
    node_id: &str,
    envelope_id: &str,
    semantic_identity: &str,
    consumer: &str,
    publication_id: &str,
    tick: &str,
    previous_sequence: u64,
) {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    let observation: M8LocalTraceObservation = trace
        .observation(node_id)
        .expect("node id must query an actual M8-owned designated consumption rejection");
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedConsumptionRejected
    );
    assert!(
        observation.sequence() > previous_sequence,
        "backend failure must be a fresh M8 attempt after C dequeues the delivery"
    );
    assert_eq!(observation.envelope_id(), envelope_id);
    assert_eq!(observation.semantic_identity(), semantic_identity);
    assert_eq!(observation.consumer_locus(), consumer);
    assert_eq!(observation.m8_publication_id(), publication_id);
    assert_eq!(observation.logical_tick_id(), tick);
}

fn m8_backend_trace_count(
    fabric: &LocalFabric,
    kind: M8LocalTraceKind,
    semantic_identity: &str,
    consumer: &str,
) -> usize {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    trace.count_designated(kind, semantic_identity, consumer)
}

fn m8_backend_latest_sequence(fabric: &LocalFabric) -> u64 {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    trace.latest_sequence().unwrap_or(0)
}

fn m8_trace_kind_count(fabric: &LocalFabric, kind: M8LocalTraceKind) -> usize {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    trace
        .kinds()
        .into_iter()
        .filter(|candidate| *candidate == kind)
        .count()
}

fn causality_reaches(causality: &CausalityGraph, later: &str, earlier: &str) -> bool {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([later.to_string()]);
    while let Some(current) = queue.pop_front() {
        if !seen.insert(current.clone()) {
            continue;
        }
        for predecessor in causality.predecessor_ids(&current) {
            if predecessor == earlier {
                return true;
            }
            queue.push_back(predecessor);
        }
    }
    false
}

fn m8_backend_node_ids(
    fabric: &LocalFabric,
    kind: M8LocalTraceKind,
    semantic_identity: &str,
    consumer: &str,
) -> Vec<String> {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    trace.node_ids_for_designated(kind, semantic_identity, consumer)
}

fn m8_owned_observation(fabric: &LocalFabric, node_id: &str) -> M8LocalTraceObservation {
    let trace: &M8LocalTrace = m8_local_runtime_trace(fabric);
    trace
        .observation(node_id)
        .expect("node id must resolve in M8-owned local runtime trace")
}

fn assert_backend_owner_operation_rejection_observation(
    fabric: &LocalFabric,
    node_id: &str,
    envelope_id: &str,
    operation: &str,
    owner_locus: &str,
    previous_sequence: u64,
) {
    let observation = m8_owned_observation(fabric, node_id);
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(observation.kind(), M8LocalTraceKind::OwnerOperationRejected);
    assert!(
        observation.sequence() > previous_sequence,
        "owner backend failure must be a fresh M8 trace occurrence after dequeue"
    );
    assert_eq!(observation.envelope_id(), envelope_id);
    assert_eq!(observation.operation_id(), operation);
    assert_eq!(observation.owner_locus(), owner_locus);
}

fn assert_owner_m8_context_observation(
    fabric: &LocalFabric,
    node_id: &str,
    envelope: &MailboxEnvelope,
    operation: &str,
    owner_locus: &str,
    kind: M8LocalTraceKind,
) {
    let observation = m8_owned_observation(fabric, node_id);
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(observation.kind(), kind);
    assert_eq!(observation.envelope_id(), envelope.envelope_id());
    assert_eq!(observation.operation_id(), operation);
    assert_eq!(observation.owner_locus(), owner_locus);
    assert_eq!(observation.edge_ref(), envelope.edge_ref());
}

fn assert_backend_designated_evaluation_rejection_observation(
    fabric: &LocalFabric,
    node_id: &str,
    envelope_id: &str,
    operation: &str,
    evaluator_locus: &str,
    tick: &str,
    previous_sequence: u64,
) {
    let observation = m8_owned_observation(fabric, node_id);
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedEvaluationRejected
    );
    assert!(
        observation.sequence() > previous_sequence,
        "designated evaluation backend failure must be a fresh M8 trace occurrence after input receipt"
    );
    assert_eq!(observation.envelope_id(), envelope_id);
    assert_eq!(observation.operation_id(), operation);
    assert_eq!(observation.evaluator_locus(), evaluator_locus);
    assert_eq!(observation.logical_tick_id(), tick);
}

fn assert_backend_designated_idempotent_observation(
    fabric: &LocalFabric,
    node_id: &str,
    envelope_id: &str,
    operation: &str,
    evaluator_locus: &str,
    tick: &str,
    previous_sequence: u64,
) {
    let observation = m8_owned_observation(fabric, node_id);
    assert_eq!(observation.node_id(), node_id);
    assert_eq!(
        observation.kind(),
        M8LocalTraceKind::DesignatedEvaluationIdempotent
    );
    assert!(
        observation.sequence() > previous_sequence,
        "idempotent fixed-version evaluation must still emit its own M8 occurrence"
    );
    assert_eq!(observation.envelope_id(), envelope_id);
    assert_eq!(observation.operation_id(), operation);
    assert_eq!(observation.evaluator_locus(), evaluator_locus);
    assert_eq!(observation.logical_tick_id(), tick);
}

fn assert_endpoint_record_provenance(
    record: &EndpointCarrierRecord,
    edge: &CommunicationEdge,
    carrier_id: &str,
) {
    assert_eq!(record.carrier_id(), carrier_id);
    assert_eq!(record.edge_ref(), edge.edge_ref());
    assert_eq!(record.source_ref(), edge.source_ref());
    assert_eq!(record.core_ref(), edge.core_ref());
    assert_eq!(record.source_fragment_ref(), edge.source_fragment_ref());
    assert_eq!(record.target_fragment_ref(), edge.target_fragment_ref());
}

fn assert_trace_row_provenance(row: &Sys4TraceEntry, edge: &CommunicationEdge) {
    assert_eq!(row.edge_ref(), edge.edge_ref());
    assert_eq!(row.source_ref(), edge.source_ref());
    assert_eq!(row.core_ref(), edge.core_ref());
    assert_eq!(row.source_fragment_ref(), edge.source_fragment_ref());
    assert_eq!(row.target_fragment_ref(), edge.target_fragment_ref());
}

fn m9_validation_occurrence_count(
    fabric: &LocalFabric,
    operation: &str,
    consumer: &str,
    semantic_identity: &str,
) -> usize {
    fabric
        .current_m9_authority_inspection()
        .validation_occurrence_count(operation, consumer, semantic_identity)
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

fn staged_four_locus_owner_attack(
    fabric: &mut LocalFabric,
    projection: &GlobalProjectionResult,
    operation: &str,
    owner: &str,
    state: &str,
    starting_hp: i64,
    atk: i64,
) -> FabricReceipt {
    let owner_request_edge = projection
        .communication_plan()
        .single_edge(operation, CommunicationEdgeKind::OwnerRequest, "A", owner)
        .unwrap_or_else(|| panic!("{operation} has generated A→{owner} owner request edge"));
    let owner_reply_edge = projection
        .communication_plan()
        .single_edge(
            operation,
            CommunicationEdgeKind::OwnerReplyReceipt,
            owner,
            "A",
        )
        .unwrap_or_else(|| panic!("{operation} has generated {owner}→A owner reply edge"));

    let submitted = fabric
        .submit_source_action(owner_attack_action(operation))
        .unwrap_or_else(|_| panic!("{operation} source action submits a generated owner request"));
    let request_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(request_envelope.envelope_id(), submitted.envelope_id());
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
        fabric
            .m8_actual_trace()
            .owner_request_node_count(operation, owner),
        0,
        "submit must not synchronously invoke M8"
    );

    let request_transport = fabric
        .step_transport("A", owner, request_envelope.envelope_id())
        .unwrap_or_else(|_| panic!("{operation} request crosses A→{owner} endpoint"));
    assert_eq!(
        request_transport.source_outbox_dequeue_record_id(),
        request_envelope.mailbox_record_id()
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
        "transport must record outbox dequeue -> target inbox enqueue"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count(operation, owner),
        0,
        "transport alone must not synchronously invoke M8"
    );

    let owner_step = fabric
        .step_locus(owner)
        .unwrap_or_else(|_| panic!("{owner} validates and serves {operation}"));
    assert_eq!(
        owner_step.consumed_envelope_id(),
        request_envelope.envelope_id()
    );
    assert_eq!(
        owner_step.locus_dequeue_record_id(),
        request_transport.target_inbox_enqueue_record_id()
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(owner_step.m8_request_node_id())
            .contains(&owner_step.locus_dequeue_occurrence_id().to_string()),
        "M8 owner request must be causally after exact target-locus dequeue"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(owner_step.m8_serve_node_id())
            .contains(&owner_step.m8_request_node_id().to_string()),
        "M8 owner serve/write must be causally after M8 request/enqueue"
    );
    assert_owner_m8_context_observation(
        fabric,
        owner_step.m8_request_node_id(),
        &request_envelope,
        operation,
        owner,
        M8LocalTraceKind::OwnerEnqueued,
    );
    assert_owner_m8_context_observation(
        fabric,
        owner_step.m8_serve_node_id(),
        &request_envelope,
        operation,
        owner,
        M8LocalTraceKind::OwnerWrite,
    );

    let reply_envelope = fabric
        .locus_runtime(owner)
        .expect("owner locus exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(reply_envelope.envelope_id(), owner_step.reply_envelope_id());
    assert_eq!(reply_envelope.edge_ref(), owner_reply_edge.edge_ref());
    assert_eq!(
        reply_envelope.carrier_contract(),
        owner_reply_edge.carrier_contract()
    );
    assert_eq!(
        reply_envelope.request_carrier_id(),
        request_envelope.carrier_id()
    );
    let reply_transport = fabric
        .step_transport(owner, "A", reply_envelope.envelope_id())
        .unwrap_or_else(|_| panic!("{operation} reply crosses {owner}→A endpoint"));
    assert!(
        causality_reaches(
            fabric.causality(),
            reply_transport.source_outbox_dequeue_occurrence_id(),
            owner_step.m8_serve_node_id(),
        ),
        "reply dispatch must be causally downstream of exact M8 owner write"
    );
    let receipt_step = fabric
        .step_locus("A")
        .unwrap_or_else(|_| panic!("A receives {operation} owner reply"));
    assert_eq!(
        receipt_step.locus_dequeue_record_id(),
        reply_transport.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        fabric
            .causality()
            .predecessor_ids(receipt_step.locus_dequeue_occurrence_id()),
        vec![
            reply_transport
                .target_inbox_enqueue_occurrence_id()
                .to_string()
        ],
        "A receipt must be causally after exact reply inbox enqueue"
    );
    let receipt = receipt_step
        .receipt()
        .cloned()
        .expect("A locus step receives an owner reply receipt");
    assert_eq!(receipt.operation_id(), operation);
    assert_eq!(receipt.origin_locus(), "A");
    assert_eq!(receipt.target_locus(), owner);
    let rmw = receipt
        .owner_rmw_report()
        .expect("owner RMW report remains attached to reply receipt");
    assert_eq!(
        rmw.m8_reads(),
        vec![
            RuntimeStoreRead::int(owner, state, "self", "hp", starting_hp),
            RuntimeStoreRead::int(owner, state, "self", "atk", atk),
        ]
    );
    assert_eq!(
        rmw.m8_writes(),
        vec![RuntimeStoreWrite::int(
            owner,
            state,
            "self",
            "hp",
            starting_hp - atk
        )]
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count(operation, owner),
        1,
        "{operation} must emit exactly one actual M8 owner request node at {owner}"
    );
    receipt
}

fn publish_designated_action() -> SourceAction {
    SourceAction::designated_tick("E.result").with_tick("F", "tick:F:1")
}

fn publish_designated_action_with_tick(tick: &str) -> SourceAction {
    SourceAction::designated_tick("E.result").with_tick("F", tick)
}

fn consume_designated_action() -> SourceAction {
    SourceAction::consume_designated_result("E.result")
}

fn owner_attack_action_with_target(operation: &str, target: &str) -> SourceAction {
    SourceAction::owner_operation(operation).with_argument("target", target)
}

fn designated_replay_log() -> Vec<SourceAction> {
    let log = vec![
        publish_designated_action(),
        consume_designated_action(),
        consume_designated_action(),
    ];
    assert_eq!(
        log.iter()
            .map(SourceAction::operation_id)
            .collect::<Vec<_>>(),
        vec!["E.result", "E.result", "E.result"]
    );
    for action in &log {
        assert!(action.origin_locus_override().is_none());
        assert!(action.authority_principal_override().is_none());
        assert!(action.target_locus_override().is_none());
        assert!(!action.can_carry_checked_core_identity());
        assert!(!action.can_carry_authority_grant());
        assert!(!action.can_carry_state_delta());
        assert!(!action.can_carry_expected_result());
    }
    log
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DesignatedReplayResult {
    receipts: Vec<FabricReceipt>,
    semantic_snapshot: FabricSemanticSnapshot,
    trace: FabricTrace,
    m8_actual_digest: String,
    m8_backend_trace: M8LocalTrace,
    m8_backend_latest_sequence: u64,
    cache: BTreeMap<String, CachedDelivery>,
    publication: Option<String>,
    artifact_identity: CheckedProgramIdentity,
}

fn run_designated_replay(
    profile: BackendProfile,
    program: &FabricProgram,
    admission: &SealedFabricAdmission,
    log: &[SourceAction],
) -> DesignatedReplayResult {
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), profile);
    let receipts = log
        .iter()
        .cloned()
        .map(|action| {
            fabric
                .dispatch_source_action(action)
                .expect("replay action dispatches through generated SYS-4 fabric")
        })
        .collect::<Vec<_>>();
    assert_eq!(receipts.len(), 3);
    assert!(receipts.iter().all(FabricReceipt::is_observer_safe));
    assert_eq!(receipts[0].operation_id(), "E.result");
    assert_eq!(receipts[0].typed_value(), RuntimeValue::int(11));
    assert_eq!(receipts[0].m8_publication_id(), receipts[0].delivery_id());
    assert_eq!(receipts[1].operation_id(), "E.result");
    assert_eq!(receipts[1].typed_value(), RuntimeValue::int(11));
    assert_eq!(receipts[1].result_version(), Some(ResultVersion::new(1)));
    assert!(receipts[1].performed_m8_semantic_consumption());
    assert!(!receipts[1].returned_from_designated_cache_after_authority_revalidation());
    assert_eq!(receipts[1].m8_publication_id(), receipts[0].delivery_id());
    assert_eq!(receipts[2].operation_id(), "E.result");
    assert_eq!(receipts[2].typed_value(), receipts[1].typed_value());
    assert_eq!(receipts[2].result_version(), receipts[1].result_version());
    assert_eq!(receipts[2].delivery_id(), receipts[1].delivery_id());
    assert!(receipts[2].returned_from_designated_cache_after_authority_revalidation());
    assert!(!receipts[2].performed_m8_semantic_consumption());
    assert_eq!(
        receipts[2].m8_publication_id(),
        receipts[1].m8_publication_id()
    );
    assert_eq!(receipts[2].logical_tick_id(), receipts[1].logical_tick_id());

    let semantic_identity = receipts[1].semantic_consumption_identity().to_string();
    assert_eq!(
        receipts[2].semantic_consumption_identity(),
        semantic_identity
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        1,
        "replay performs exactly one semantic M8 consume"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .designated_evaluation_count("E.result"),
        1,
        "replay performs exactly one designated evaluation"
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedValueConsumed,
            &semantic_identity,
            "C",
        ),
        1,
        "M8 backend records exactly one semantic consume"
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            &semantic_identity,
            "C",
        ),
        1,
        "exact retry records exactly one non-consuming M8 cache validation"
    );
    assert!(
        fabric
            .m8_designated_publication_snapshot("E.result")
            .is_some(),
        "replay leaves a concrete M8 publication binding state"
    );
    let cache = fabric.designated_cache_snapshot();
    let cache_entry = cache
        .get(&semantic_identity)
        .expect("replay installs one designated cache entry");
    assert_eq!(
        cache_entry.sealed_delivery_binding().m8_publication_id(),
        receipts[1].m8_publication_id()
    );
    assert_eq!(
        cache_entry.sealed_delivery_binding().logical_tick_id(),
        receipts[1].logical_tick_id()
    );
    assert_eq!(
        cache_entry.sealed_delivery_binding_digest(),
        format!("{:?}", cache_entry.sealed_delivery_binding())
    );
    assert!(
        fabric
            .trace()
            .for_designated_delivery("E.result", receipts[1].delivery_id())
            .all_entries_observer_safe()
    );
    assert!(
        fabric
            .trace()
            .for_designated_delivery("E.result", receipts[2].delivery_id())
            .all_entries_observer_safe()
    );

    DesignatedReplayResult {
        receipts,
        semantic_snapshot: fabric.semantic_snapshot(),
        trace: fabric.trace().clone(),
        m8_actual_digest: fabric.m8_actual_trace().stable_digest(),
        m8_backend_trace: fabric.m8_local_runtime_trace().clone(),
        m8_backend_latest_sequence: m8_backend_latest_sequence(&fabric),
        cache,
        publication: fabric.m8_designated_publication_snapshot("E.result"),
        artifact_identity: fabric.projected_artifact_identity().clone(),
    }
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
fn seed_rejects_valid_player_state_at_known_non_owner_a() {
    let checked = four_locus_checked();
    let program = fabric_program(four_locus_projection(&checked));
    let seed = four_locus_initial_state_seed(checked.program_identity())
        .with_int("A", "player", "self", "hp", 100);

    assert_sys4_diag(
        SealedFabricAdmission::from_m9_execution_seam(
            &program,
            m9_four_locus_multi_owner_seam(&checked),
            seed,
        ),
        Sys4DiagnosticKind::ForeignSeedState,
    );
}

#[test]
fn duplicate_state_player_at_distinct_s_t_owners_rejects_m7_duplicate_declaration() {
    let source = r#"
module Combat.Sys4.DuplicateStatePlayerST

locus A
locus S
locus T
principal self
type Player

state player[id: Player] at S {
  hp: Int
}

state player[id: Player] at T {
  hp: Int
}
"#;

    let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/sys4_duplicate_state_player_s_t.mir",
        source,
    ))
    .expect_err("duplicate state player declarations across owners must fail in M7");
    assert_eq!(diagnostics.entries().len(), 1);
    assert_eq!(
        diagnostics.primary().kind(),
        M7DiagnosticKind::DuplicateDeclaration
    );
    assert!(
        !diagnostics.has_executable_core(),
        "M7 duplicate declarations must not leave executable Core"
    );
}

#[test]
fn four_locus_two_owner_projection_and_bootstrap_reject_ow1_with_exact_reason() {
    let checked = four_locus_checked();
    let projection = four_locus_projection(&checked);
    let expected_reason =
        BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { count: 2 };
    assert_eq!(
        projection
            .backend_requirements()
            .eligibility(BackendProfile::Ow1),
        BackendEligibility::Ineligible {
            reason: expected_reason.clone(),
        },
        "four-locus two-owner/source-owner projection must not advertise OW1 eligibility"
    );

    let program = fabric_program(projection);
    assert_eq!(
        program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Ineligible {
            reason: expected_reason.clone(),
        },
        "FabricProgram must preserve SYS-3's exact OW1 ineligibility reason"
    );
    let admission = sealed_four_locus_admission(&checked, &program);
    let diagnostics = assert_sys4_diag(
        LocalFabric::bootstrap(program, admission, BackendProfile::Ow1),
        Sys4DiagnosticKind::BackendIneligible,
    );
    assert_eq!(
        diagnostics.backend_ineligibility_reason(),
        Some(&expected_reason),
        "bootstrap rejection must expose the same typed SYS-3 reason instead of a generic BackendIneligible"
    );
}

#[test]
fn four_locus_st_attack_s_and_attack_t_mutate_only_their_authoritative_owner_state() {
    let checked = four_locus_checked();
    let projection = four_locus_projection(&checked);
    let program = fabric_program(projection.clone());
    let admission = sealed_four_locus_admission(&checked, &program);
    let mut fabric = boot_with_admission(program, admission, BackendProfile::St);

    assert_eq!(fabric.locus_names(), vec!["A", "S", "T", "V"]);
    let before = fabric.semantic_snapshot();
    assert_eq!(before.int("S", "player", "self", "hp"), Some(100));
    assert_eq!(before.int("S", "player", "self", "atk"), Some(10));
    assert_eq!(before.int("T", "shield", "self", "hp"), Some(200));
    assert_eq!(before.int("T", "shield", "self", "atk"), Some(7));

    let receipt_s = staged_four_locus_owner_attack(
        &mut fabric,
        &projection,
        "attack_s",
        "S",
        "player",
        100,
        10,
    );
    assert_eq!(receipt_s.typed_value(), RuntimeValue::unit());
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "atk"),
        Some(10)
    );
    assert_eq!(
        fabric.semantic_snapshot().int("T", "shield", "self", "hp"),
        Some(200),
        "attack_s must not move S values into T or mutate T"
    );
    assert_eq!(
        fabric.semantic_snapshot().int("T", "shield", "self", "atk"),
        Some(7)
    );
    assert_eq!(
        fabric.semantic_snapshot().changed_loci_since(&before),
        vec!["S"],
        "after attack_s only S authoritative local store may change"
    );

    let before_t = fabric.semantic_snapshot();
    let receipt_t =
        staged_four_locus_owner_attack(&mut fabric, &projection, "attack_t", "T", "shield", 200, 7);
    assert_eq!(receipt_t.typed_value(), RuntimeValue::unit());
    assert_eq!(
        fabric.semantic_snapshot().int("T", "shield", "self", "hp"),
        Some(193)
    );
    assert_eq!(
        fabric.semantic_snapshot().int("T", "shield", "self", "atk"),
        Some(7)
    );
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90),
        "attack_t must not move T values into S or mutate S after attack_s"
    );
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "atk"),
        Some(10)
    );
    assert_eq!(
        fabric.semantic_snapshot().changed_loci_since(&before_t),
        vec!["T"],
        "after attack_t only T authoritative local store may change"
    );
    assert!(
        fabric
            .semantic_snapshot()
            .locus_unchanged_since("A", &before)
    );
    assert!(
        fabric
            .semantic_snapshot()
            .locus_unchanged_since("V", &before)
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack_s", "S"),
        1
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack_s", "T"),
        0,
        "attack_s M8 occurrence must not be attributed to T"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack_t", "T"),
        1
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack_t", "S"),
        0,
        "attack_t M8 occurrence must not be attributed to S"
    );
}

#[test]
fn four_locus_st_admission_requires_test_visible_m8_partition_evidence_per_locus() {
    let checked = four_locus_checked();
    let projection = four_locus_projection(&checked);
    let program = fabric_program(projection.clone());
    assert_eq!(program.locus_names(), vec!["A", "S", "T", "V"]);
    let admission = sealed_four_locus_admission(&checked, &program);
    let mut fabric = boot_with_admission(program, admission, BackendProfile::St);

    let boot_evidence = fabric.m8_partition_evidence();
    assert!(
        boot_evidence.is_observer_safe(),
        "partition evidence is a devtools/test view and must not expose raw authority material"
    );
    assert_eq!(boot_evidence.locus_names(), vec!["A", "S", "T", "V"]);

    let session_ids: BTreeSet<_> = ["A", "S", "T", "V"]
        .into_iter()
        .map(|locus| {
            boot_evidence
                .partition(locus)
                .unwrap_or_else(|| panic!("{locus} has boot partition evidence"))
                .session_id()
                .to_string()
        })
        .collect();
    assert_eq!(
        session_ids.len(),
        4,
        "each admitted locus must have a distinct M8 semantic runtime/session identity"
    );
    let partition_ids: BTreeSet<_> = ["A", "S", "T", "V"]
        .into_iter()
        .map(|locus| {
            boot_evidence
                .partition(locus)
                .unwrap_or_else(|| panic!("{locus} has boot partition evidence"))
                .partition_id()
                .to_string()
        })
        .collect();
    assert_eq!(
        partition_ids.len(),
        4,
        "each admitted locus must have a distinct M8 state partition identity"
    );

    assert_eq!(
        boot_evidence
            .partition("S")
            .expect("S partition exists")
            .authoritative_state_key_refs(),
        vec!["player[self].atk", "player[self].hp"],
        "S partition must contain only S-owned player seed keys"
    );
    assert_eq!(
        boot_evidence
            .partition("T")
            .expect("T partition exists")
            .authoritative_state_key_refs(),
        vec!["shield[self].atk", "shield[self].hp"],
        "T partition must contain only T-owned shield seed keys"
    );
    assert!(
        boot_evidence
            .partition("A")
            .expect("A partition exists")
            .authoritative_state_key_refs()
            .is_empty(),
        "A is admitted as an invocation locus but owns no authoritative state in this fixture"
    );
    assert!(
        boot_evidence
            .partition("V")
            .expect("V partition exists")
            .authoritative_state_key_refs()
            .is_empty(),
        "V is admitted as a viewer locus but owns no authoritative state in this fixture"
    );
    for locus in ["A", "S", "T", "V"] {
        let partition = boot_evidence
            .partition(locus)
            .unwrap_or_else(|| panic!("{locus} partition exists"));
        assert!(
            partition.state_inventory().is_derived_from_m8_session(),
            "{locus} state inventory must be derived from the real M8 local session, not copied from LocusLocalStore"
        );
        assert!(
            partition
                .publication_inventory()
                .is_derived_from_m8_session(),
            "{locus} publication inventory must be derived from the real M8 local session"
        );
        assert_eq!(
            partition.state_inventory().key_refs(),
            partition.authoritative_state_key_refs(),
            "{locus} partition state-key evidence must match the observer-safe M8 session inventory"
        );
        assert!(
            partition
                .publication_inventory()
                .published_value_refs()
                .is_empty(),
            "four-locus owner-only fixture has no designated publications at boot"
        );
    }

    staged_four_locus_owner_attack(&mut fabric, &projection, "attack_s", "S", "player", 100, 10);
    let after_s = fabric.m8_partition_evidence();
    assert_eq!(
        after_s.changed_partitions_since(&boot_evidence),
        vec!["S"],
        "attack_s may change only the S M8 partition evidence"
    );
    assert_ne!(
        after_s
            .partition("S")
            .expect("S partition exists")
            .state_digest(),
        boot_evidence
            .partition("S")
            .expect("S partition exists")
            .state_digest(),
        "S partition digest must change after S-owned mutation"
    );
    assert_eq!(
        after_s
            .partition("T")
            .expect("T partition exists")
            .state_digest(),
        boot_evidence
            .partition("T")
            .expect("T partition exists")
            .state_digest(),
        "T partition digest must not change after S-owned mutation"
    );

    staged_four_locus_owner_attack(&mut fabric, &projection, "attack_t", "T", "shield", 200, 7);
    let after_t = fabric.m8_partition_evidence();
    assert_eq!(
        after_t.changed_partitions_since(&after_s),
        vec!["T"],
        "attack_t may change only the T M8 partition evidence"
    );
    assert_ne!(
        after_t
            .partition("T")
            .expect("T partition exists")
            .state_digest(),
        after_s
            .partition("T")
            .expect("T partition exists")
            .state_digest(),
        "T partition digest must change after T-owned mutation"
    );
    assert_eq!(
        after_t
            .partition("S")
            .expect("S partition exists")
            .state_digest(),
        after_s
            .partition("S")
            .expect("S partition exists")
            .state_digest(),
        "S partition digest must not change after T-owned mutation"
    );

    let s_partition = after_t.partition("S").expect("S partition exists");
    let t_partition = after_t.partition("T").expect("T partition exists");
    let a_partition = after_t.partition("A").expect("A partition exists");
    let v_partition = after_t.partition("V").expect("V partition exists");
    assert!(
        a_partition
            .m8_trace_occurrences_for_operation("attack_s")
            .is_empty()
            && a_partition
                .m8_trace_occurrences_for_operation("attack_t")
                .is_empty(),
        "A invocation locus must not receive owner M8 operation occurrences"
    );
    assert!(
        v_partition
            .m8_trace_occurrences_for_operation("attack_s")
            .is_empty()
            && v_partition
                .m8_trace_occurrences_for_operation("attack_t")
                .is_empty(),
        "V viewer locus must not receive owner M8 operation occurrences"
    );

    assert_eq!(
        s_partition
            .m8_trace_occurrence_count_for_operation("attack_s", M8LocalTraceKind::OwnerEnqueued),
        1,
        "attack_s request must be enqueued only in S's M8 partition"
    );
    assert_eq!(
        s_partition
            .m8_trace_occurrence_count_for_operation("attack_s", M8LocalTraceKind::OwnerRead),
        1,
        "attack_s RMW records one actual OwnerRead occurrence with a two-key read-set"
    );
    let s_read = s_partition
        .single_m8_trace_occurrence_for_operation("attack_s", M8LocalTraceKind::OwnerRead);
    assert_eq!(
        s_read.observer_safe_read_key_refs(),
        vec!["player[self].hp", "player[self].atk"],
        "attack_s OwnerRead evidence must expose the exact observer-safe two-key read-set without raw payloads"
    );
    assert_eq!(
        s_partition
            .m8_trace_occurrence_count_for_operation("attack_s", M8LocalTraceKind::OwnerWrite),
        1,
        "attack_s RMW must write only S's M8 partition"
    );
    assert!(
        !t_partition.has_m8_trace_occurrences_for_operation("attack_s"),
        "attack_s must not appear in T's M8 partition"
    );

    assert_eq!(
        t_partition
            .m8_trace_occurrence_count_for_operation("attack_t", M8LocalTraceKind::OwnerEnqueued),
        1,
        "attack_t request must be enqueued only in T's M8 partition"
    );
    assert_eq!(
        t_partition
            .m8_trace_occurrence_count_for_operation("attack_t", M8LocalTraceKind::OwnerRead),
        1,
        "attack_t RMW records one actual OwnerRead occurrence with a two-key read-set"
    );
    let t_read = t_partition
        .single_m8_trace_occurrence_for_operation("attack_t", M8LocalTraceKind::OwnerRead);
    assert_eq!(
        t_read.observer_safe_read_key_refs(),
        vec!["shield[self].hp", "shield[self].atk"],
        "attack_t OwnerRead evidence must expose the exact observer-safe two-key read-set without raw payloads"
    );
    assert_eq!(
        t_partition
            .m8_trace_occurrence_count_for_operation("attack_t", M8LocalTraceKind::OwnerWrite),
        1,
        "attack_t RMW must write only T's M8 partition"
    );
    assert!(
        !s_partition.has_m8_trace_occurrences_for_operation("attack_t"),
        "attack_t must not appear in S's M8 partition"
    );
    assert!(
        after_t.all_m8_trace_occurrence_ids_are_unique(),
        "fabric-qualified M8 occurrence IDs from all partitions must be globally unique"
    );
    assert!(
        after_t.all_m8_trace_occurrences_resolve_in(fabric.m8_actual_trace(), fabric.causality()),
        "every partition occurrence must resolve by exact fabric-qualified ID/kind in ActualM8Trace and CausalityGraph"
    );

    let sys4 = read_runtime_src("sys4_dispatch.rs");
    let compact_sys4 = normalize_source_for_boundary_scan(&sys4);
    assert!(
        compact_sys4.contains("M8RuntimePartitionEvidence")
            || compact_sys4.contains("m8_runtime_partition_evidence")
            || compact_sys4.contains("m8_partition_evidence"),
        "SYS-4 two-owner ST admission must expose observer-safe test-visible evidence that every admitted locus has its own M8 semantic runtime/session partition"
    );
    assert!(
        !compact_sys4.contains("St(Box<M8LocalRuntime>)"),
        "a single unpartitioned ST M8LocalRuntime cannot be the SYS-4 runtime shape for a four-locus two-owner fabric"
    );
}

#[test]
fn sequential_st_owner_ops_retain_prior_write_dependency_but_ow1_physical_cross_locus_does_not() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let mut st = boot(&checked, program, BackendProfile::St);

    let first = st
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("first same-owner operation dispatches");
    let after_first = st.m8_partition_evidence();
    let first_write = after_first
        .partition("S")
        .expect("S partition exists")
        .single_m8_trace_occurrence_for_request(first.request_id(), M8LocalTraceKind::OwnerWrite);
    assert_eq!(first_write.operation_id(), "attack");

    let second = st
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("second same-owner operation dispatches");
    let after_second = st.m8_partition_evidence();
    let second_write = after_second
        .partition("S")
        .expect("S partition exists")
        .single_m8_trace_occurrence_for_request(second.request_id(), M8LocalTraceKind::OwnerWrite);
    assert_eq!(second_write.operation_id(), "attack");
    assert!(
        second_write
            .qualified_dependency_graph()
            .reaches(first_write.fabric_qualified_id()),
        "the second same-owner/same-locus owner write must retain the first write as a semantic M8 predecessor"
    );
    assert!(
        causality_reaches(
            st.causality(),
            second_write.fabric_qualified_id(),
            first_write.fabric_qualified_id()
        ),
        "SYS-4 causality must mirror the qualified M8 dependency edge from second write back to first write"
    );

    let combined = combined_owner_designated_checked();
    let combined_program = fabric_program(combined_owner_designated_projection(&combined));
    assert_eq!(
        combined_program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible,
        "combined fixture has one semantic owner/source-owner worker locus S"
    );
    let mut ow1 = boot(&combined, combined_program, BackendProfile::Ow1);
    let owner_submitted = ow1
        .submit_source_action(owner_attack_action("attack"))
        .expect("OW1 owner attack submits before the designated source read");
    let owner_request = ow1
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(owner_request.envelope_id(), owner_submitted.envelope_id());
    ow1.step_transport("A", "S", owner_request.envelope_id())
        .expect("OW1 owner request crosses A→S endpoint");
    let owner_step = ow1
        .step_locus("S")
        .expect("OW1 worker serves the owner request");
    assert_owner_m8_context_observation(
        &ow1,
        owner_step.m8_request_node_id(),
        &owner_request,
        "attack",
        "S",
        M8LocalTraceKind::OwnerEnqueued,
    );
    assert_owner_m8_context_observation(
        &ow1,
        owner_step.m8_serve_node_id(),
        &owner_request,
        "attack",
        "S",
        M8LocalTraceKind::OwnerWrite,
    );
    let owner_write = m8_owned_observation(&ow1, owner_step.m8_serve_node_id());

    let submitted = ow1
        .submit_source_action(publish_designated_action_with_tick(
            "tick:F:ow1-physical-red",
        ))
        .expect("designated publish submits E→S input request");
    let input = ow1
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input.envelope_id(), submitted.envelope_id());
    ow1.step_transport("E", "S", input.envelope_id())
        .expect("input request transports to S");
    let source_step = ow1.step_locus("S").expect("S worker reads source input");
    let source_read_audit = source_step
        .local_store_read_audit()
        .expect("source read audit exists");
    let source_read = m8_local_runtime_trace(&ow1)
        .latest_observation_for_occurrence(
            M8LocalTraceKind::OwnerRead,
            source_read_audit.occurrence_id(),
        )
        .expect("source read audit occurrence resolves to M8-owned OwnerRead row");
    assert_eq!(source_read.operation_id(), "E.result");
    assert_eq!(source_read_audit.occurrence_id(), source_read.node_id());
    assert!(
        !source_read
            .predecessor_ids()
            .contains(&owner_write.node_id().to_string()),
        "OW1 worker FIFO order alone must not create a cross-semantic-locus M8 predecessor from owner mutation to remote input read"
    );
    assert!(
        !causality_reaches(
            ow1.causality(),
            source_read.node_id(),
            owner_write.node_id()
        ),
        "SYS-4 causality must not retain a physical-only OW1 predecessor across independent semantic operations"
    );
}

#[test]
fn same_operation_owner_reads_expose_per_occurrence_read_sets_not_union() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let seed = initial_state_seed(checked.program_identity())
        .with_int("S", "player", "target", "hp", 2222);
    let admission =
        SealedFabricAdmission::from_m9_execution_seam(&program, m9_fabric_seam(&checked), seed)
            .expect("explicit target seed admits the source-first owner fixture");
    let mut fabric = boot_with_admission(program, admission, BackendProfile::St);

    let self_receipt = fabric
        .dispatch_source_action(owner_attack_action_with_target("attack", "self"))
        .expect("self-target owner attack dispatches");
    let target_receipt = fabric
        .dispatch_source_action(owner_attack_action_with_target("attack", "target"))
        .expect("target owner attack dispatches with same operation identity");
    let evidence = fabric.m8_partition_evidence();
    let partition = evidence.partition("S").expect("S partition exists");
    let self_read = partition.single_m8_trace_occurrence_for_request(
        self_receipt.request_id(),
        M8LocalTraceKind::OwnerRead,
    );
    let target_read = partition.single_m8_trace_occurrence_for_request(
        target_receipt.request_id(),
        M8LocalTraceKind::OwnerRead,
    );
    assert_eq!(self_read.operation_id(), "attack");
    assert_eq!(target_read.operation_id(), "attack");
    assert_eq!(
        self_read.observer_safe_read_key_refs(),
        vec!["player[self].hp", "player[self].atk"],
        "first OwnerRead occurrence must expose only its concrete read-set"
    );
    assert_eq!(
        target_read.observer_safe_read_key_refs(),
        vec!["player[target].hp", "player[self].atk"],
        "second OwnerRead occurrence must expose only its concrete read-set, not the union for operation 'attack'"
    );
}

#[test]
fn observer_safe_partition_evidence_redacts_seeded_literal_payload_values() {
    let checked = four_locus_checked();
    let projection = four_locus_projection(&checked);
    let program = fabric_program(projection);
    let seed = Sys4InitialStateSeed::for_checked_program(checked.program_identity().clone())
        .with_int("S", "player", "self", "hp", 8_675_309)
        .with_int("S", "player", "self", "atk", 3_141_592)
        .with_int("T", "shield", "self", "hp", 4_242_424)
        .with_int("T", "shield", "self", "atk", 2_718_281);
    let admission = SealedFabricAdmission::from_m9_execution_seam(
        &program,
        m9_four_locus_multi_owner_seam(&checked),
        seed,
    )
    .expect("distinctive literal seed admits the four-locus fixture");
    let fabric = boot_with_admission(program, admission, BackendProfile::St);
    let evidence = fabric.m8_partition_evidence();
    assert!(evidence.is_observer_safe());

    let debug_export = format!("{evidence:?}");
    for literal in ["8675309", "3141592", "4242424", "2718281"] {
        assert!(
            !debug_export.contains(literal),
            "observer-safe partition evidence Debug/export must not leak seeded payload literal {literal}"
        );
        for locus in ["S", "T"] {
            assert!(
                !evidence
                    .partition(locus)
                    .expect("owner partition exists")
                    .state_digest()
                    .contains(literal),
                "{locus} observer-safe partition digest must not include seeded payload literal {literal}"
            );
        }
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
    assert!(rmw.has_checked_source_core_provenance());

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
    let m8_non_consuming_before_revoked_retry = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedCacheValidated,
        semantic_identity,
        "C",
    );
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
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            semantic_identity,
            "C",
        ),
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
    let m8_non_consuming_before_membership = m8_backend_trace_count(
        &membership_retired,
        M8LocalTraceKind::DesignatedCacheValidated,
        &membership_identity,
        "C",
    );
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
        m8_backend_trace_count(
            &membership_retired,
            M8LocalTraceKind::DesignatedCacheValidated,
            &membership_identity,
            "C",
        ),
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
    let m8_non_consuming_before_witness = m8_backend_trace_count(
        &witness_retired,
        M8LocalTraceKind::DesignatedCacheValidated,
        &witness_identity,
        "C",
    );
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
        m8_backend_trace_count(
            &witness_retired,
            M8LocalTraceKind::DesignatedCacheValidated,
            &witness_identity,
            "C",
        ),
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
    assert_owner_m8_context_observation(
        &fabric,
        s_step.m8_request_node_id(),
        &request_envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerEnqueued,
    );
    assert_owner_m8_context_observation(
        &fabric,
        s_step.m8_serve_node_id(),
        &request_envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerWrite,
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
fn designated_st_delivery_imports_e_publication_into_c_partition_before_single_consume() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
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
    assert_eq!(
        binding_m8_publication_id(delivery.immutable_delivery_binding()),
        delivery.m8_publication_id(),
        "generated delivery carrier must seal the exact M8 publication identity produced at E"
    );

    let before_c = fabric.m8_partition_evidence();
    let e_publication = before_c
        .partition("E")
        .expect("E partition exists")
        .publication_inventory()
        .publication(delivery.m8_publication_id())
        .expect("E partition exposes the actual M8 publication carried by the delivery");
    assert_eq!(
        e_publication.m8_publication_id(),
        delivery.m8_publication_id()
    );
    assert_eq!(e_publication.node_id(), delivery.m8_evaluation_node_id());
    assert_eq!(
        e_publication.kind(),
        M8LocalTraceKind::DesignatedValuePublished
    );
    assert!(
        !before_c
            .partition("C")
            .expect("C partition exists")
            .publication_inventory()
            .contains_publication_id(delivery.m8_publication_id()),
        "C must not have the evaluator publication before C dequeues the E→C delivery"
    );
    assert!(
        before_c
            .partition("C")
            .expect("C partition exists")
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValuePublished)
            .is_empty(),
        "consumer C must not evaluate the designated expression locally"
    );

    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("delivery crosses E→C endpoint");
    let consumer_step = fabric
        .step_locus("C")
        .expect("C imports the E publication, then consumes exactly once");
    let c_dequeue = consumer_step.locus_dequeue_occurrence_id().to_string();
    let consume_node = consumer_step.m8_consume_node_id().to_string();
    let after_c = fabric.m8_partition_evidence();
    let c_partition = after_c.partition("C").expect("C partition exists");
    assert!(
        c_partition
            .publication_inventory()
            .contains_publication_id(delivery.m8_publication_id()),
        "C publication inventory must show import of the exact E publication before consume"
    );
    let imports =
        c_partition.m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported);
    assert_eq!(
        imports.len(),
        1,
        "C must record one explicit M8-owned import before semantic consumption"
    );
    let imported = imports.single();
    assert_eq!(imported.fabric_qualified_id(), imported.node_id());
    assert_eq!(imported.m8_publication_id(), delivery.m8_publication_id());
    assert_eq!(imported.envelope_id(), delivery.envelope_id());
    assert_eq!(imported.operation_id(), "E.result");
    assert_eq!(imported.edge_ref(), delivery.edge_ref());
    assert_eq!(imported.evaluator_locus(), "E");
    assert_eq!(imported.consumer_locus(), "C");
    assert_eq!(
        fabric
            .m8_actual_trace()
            .node(imported.fabric_qualified_id())
            .kind(),
        M8LocalTraceKind::DesignatedPublicationImported,
        "ActualM8Trace must resolve C's import node as an M8-owned import occurrence"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(imported.fabric_qualified_id())
            .contains(&delivery.m8_evaluation_node_id().to_string()),
        "C import must be causally after the exact E publication carried by the delivery"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(imported.fabric_qualified_id())
            .contains(&c_dequeue),
        "C import must be causally after C dequeues the delivery carrier"
    );
    let consumes =
        c_partition.m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed);
    assert_eq!(consumes.len(), 1);
    let consume = consumes.single();
    assert_eq!(consume.fabric_qualified_id(), consume_node);
    assert_eq!(consume.m8_publication_id(), delivery.m8_publication_id());
    assert!(
        fabric
            .causality()
            .predecessor_ids(consume.fabric_qualified_id())
            .contains(&imported.fabric_qualified_id().to_string()),
        "C consume must be causally after the explicit C import"
    );
    assert!(
        c_partition
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValuePublished)
            .is_empty(),
        "C must not re-evaluate/publish the evaluator expression"
    );
    assert!(
        c_partition
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedEvaluationIdempotent)
            .is_empty(),
        "C must not synthesize an evaluator idempotence occurrence"
    );

    let retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("exact duplicate/cache retry succeeds by revalidation");
    assert!(retry.returned_from_designated_cache_after_authority_revalidation());
    let after_retry = fabric.m8_partition_evidence();
    let c_after_retry = after_retry.partition("C").expect("C partition exists");
    assert_eq!(
        c_after_retry
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
            .len(),
        1,
        "exact retry must not import the same publication again"
    );
    assert_eq!(
        c_after_retry
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
            .len(),
        1,
        "exact retry must not perform a second M8 semantic consume"
    );
    assert_eq!(
        c_after_retry
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedCacheValidated)
            .len(),
        1,
        "exact retry records one non-consuming M8 cache validation"
    );
}

#[test]
fn imported_designated_publication_and_receipt_survive_sys4_local_cut_restore() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("delivery crosses to C");
    let consumed = fabric
        .step_locus("C")
        .expect("C imports and consumes before the cut");
    let receipt = consumed
        .receipt()
        .expect("C consume returns receipt")
        .clone();
    assert_eq!(
        receipt_m8_publication_id(&receipt),
        delivery.m8_publication_id()
    );
    let before_cut_evidence = fabric.m8_partition_evidence();
    let c_before_cut = before_cut_evidence
        .partition("C")
        .expect("C partition exists");
    assert!(
        c_before_cut
            .publication_inventory()
            .contains_publication_id(delivery.m8_publication_id()),
        "pre-cut C partition contains the imported E publication"
    );
    assert_eq!(
        c_before_cut
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
            .len(),
        1
    );
    assert_eq!(
        c_before_cut
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
            .len(),
        1
    );

    let cut = save_sys4_local_cut(&mut fabric, "sys4-imported-designated-publication-cut")
        .expect("ST whole-fabric cut saves imported designated publication state");
    assert!(
        cut.imported_designated_publication_state().contains_exact(
            delivery.semantic_identity(),
            delivery.m8_publication_id(),
            delivery.immutable_delivery_digest()
        ),
        "SYS-4 local cut must persist exact imported publication identity/digest, not only a coordinator cache flag"
    );
    assert!(
        cut.designated_receipt_state().contains_exact_receipt(
            receipt.semantic_consumption_identity(),
            receipt_m8_publication_id(&receipt),
            receipt_logical_tick_id(&receipt)
        ),
        "SYS-4 local cut must persist the exact imported publication receipt needed for retry semantics"
    );

    let mut restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("local cut restores through the same source-first program/admission path");
    let restored_evidence = restored.m8_partition_evidence();
    let c_restored = restored_evidence
        .partition("C")
        .expect("C partition exists after restore");
    assert_eq!(
        c_restored.publication_inventory(),
        c_before_cut.publication_inventory(),
        "restored C partition must retain exact imported publication state"
    );
    assert_eq!(
        c_restored
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
            .len(),
        1,
        "restore must not require a second import trace for an already persisted exact publication"
    );
    let retry = restored
        .dispatch_source_action(consume_designated_action())
        .expect("post-restore retry uses persisted publication/receipt state");
    assert_eq!(retry.typed_value(), receipt.typed_value());
    assert_eq!(
        receipt_m8_publication_id(&retry),
        receipt_m8_publication_id(&receipt)
    );
    let c_after_retry = restored
        .m8_partition_evidence()
        .partition("C")
        .expect("C partition exists after retry")
        .clone();
    assert_eq!(
        c_after_retry
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
            .len(),
        1,
        "post-restore exact retry must not re-import the persisted publication"
    );
    assert_eq!(
        c_after_retry
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
            .len(),
        1,
        "post-restore exact retry must not re-consume the persisted delivery"
    );
}

#[test]
fn sys4_local_cut_restores_pending_owner_request_endpoint_mailbox_and_continues_once() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("source-first owner request submits a generated A outbox carrier");
    let request = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(request.envelope_id(), submitted.envelope_id());

    let outbox_cut = save_sys4_local_cut(&mut fabric, "sys4-cut-owner-request-a-outbox")
        .expect("ST cut with pending owner request in A outbox succeeds");
    let mut restored_outbox = LocalFabric::restore_local_cut(
        program.clone(),
        admission.clone(),
        BackendProfile::St,
        &outbox_cut,
    )
    .expect("pending A outbox owner request restores");
    assert_eq!(
        restored_outbox
            .locus_runtime("A")
            .expect("A exists after restore")
            .outgoing_mailbox()
            .pending_envelopes()
            .single(),
        request,
        "restore must preserve the exact A outbox owner-request envelope"
    );

    let request_transport = restored_outbox
        .step_transport("A", "S", request.envelope_id())
        .expect("restored request transports exactly once");
    let inbox_request = restored_outbox
        .locus_runtime("S")
        .expect("S exists after request transport")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(inbox_request.envelope_id(), request.envelope_id());
    assert_eq!(inbox_request.carrier_id(), request.carrier_id());
    assert_eq!(
        restored_outbox
            .locus_runtime("A")
            .expect("A exists")
            .outgoing_endpoint()
            .single(CommunicationEdgeKind::OwnerRequest, "A", "S")
            .carrier_id(),
        request.carrier_id()
    );
    assert_eq!(
        restored_outbox
            .locus_runtime("S")
            .expect("S exists")
            .incoming_endpoint()
            .single(CommunicationEdgeKind::OwnerRequest, "A", "S")
            .carrier_id(),
        request.carrier_id()
    );

    let inbox_cut = save_sys4_local_cut(&mut restored_outbox, "sys4-cut-owner-request-s-inbox")
        .expect("ST cut with pending owner request in S inbox succeeds");
    let mut restored_inbox =
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &inbox_cut)
            .expect("pending S inbox owner request restores");
    assert_eq!(
        restored_inbox
            .locus_runtime("S")
            .expect("S exists after second restore")
            .incoming_mailbox()
            .pending_envelopes()
            .single(),
        inbox_request,
        "restore must preserve the exact S inbox owner-request envelope"
    );
    assert_eq!(
        restored_inbox
            .locus_runtime("A")
            .expect("A exists after second restore")
            .outgoing_endpoint()
            .single(CommunicationEdgeKind::OwnerRequest, "A", "S")
            .carrier_id(),
        request.carrier_id(),
        "restore must preserve the source endpoint dequeue history matching S inbox"
    );
    assert_eq!(
        restored_inbox
            .locus_runtime("S")
            .expect("S exists after second restore")
            .incoming_endpoint()
            .single(CommunicationEdgeKind::OwnerRequest, "A", "S")
            .carrier_id(),
        request.carrier_id(),
        "restore must preserve the target endpoint enqueue history matching S inbox"
    );

    let owner_step = restored_inbox
        .step_locus("S")
        .expect("restored S inbox request serves once");
    assert_eq!(
        owner_step.locus_dequeue_record_id(),
        request_transport.target_inbox_enqueue_record_id()
    );
    assert_owner_m8_context_observation(
        &restored_inbox,
        owner_step.m8_request_node_id(),
        &inbox_request,
        "attack",
        "S",
        M8LocalTraceKind::OwnerEnqueued,
    );
    assert_owner_m8_context_observation(
        &restored_inbox,
        owner_step.m8_serve_node_id(),
        &inbox_request,
        "attack",
        "S",
        M8LocalTraceKind::OwnerWrite,
    );
    let reply = restored_inbox
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let reply_transport = restored_inbox
        .step_transport("S", "A", reply.envelope_id())
        .expect("restored owner reply transports once");
    let receipt_step = restored_inbox
        .step_locus("A")
        .expect("A consumes restored owner reply once");
    assert_eq!(
        receipt_step.locus_dequeue_record_id(),
        reply_transport.target_inbox_enqueue_record_id()
    );
    assert_eq!(
        restored_inbox
            .m8_actual_trace()
            .owner_request_node_count("attack", "S"),
        1,
        "restored owner request must not be served twice"
    );
}

#[test]
fn sys4_local_cut_restores_inflight_designated_delivery_and_consumes_once() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let cut = save_sys4_local_cut(&mut fabric, "sys4-cut-designated-delivery-e-outbox")
        .expect("ST cut with E→C delivery in flight succeeds");
    let mut restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("in-flight E→C designated delivery restores");
    assert_eq!(
        restored
            .locus_runtime("E")
            .expect("E exists after restore")
            .outgoing_mailbox()
            .pending_envelopes()
            .single(),
        delivery,
        "restore must preserve the exact E outbox designated delivery carrier"
    );

    restored
        .step_transport("E", "C", delivery.envelope_id())
        .expect("restored delivery transports to C once");
    let consumed = restored
        .step_locus("C")
        .expect("C imports and consumes restored delivery once");
    let receipt = consumed
        .receipt()
        .expect("restored delivery consume returns receipt");
    assert_eq!(
        receipt_m8_publication_id(receipt),
        delivery.m8_publication_id()
    );
    assert_eq!(
        restored
            .m8_actual_trace()
            .value_consumed_count(delivery.semantic_identity(), "C"),
        1,
        "restored designated delivery must not consume twice"
    );
    assert_eq!(
        restored
            .designated_consumption_state()
            .semantic_consumption_count(delivery.semantic_identity(), "C"),
        1
    );
}

#[test]
fn sys4_local_cut_preserves_m9_successor_tombstone_for_revoked_cache_retry() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("delivery crosses E→C");
    fabric
        .step_locus("C")
        .expect("C imports and consumes before revocation");

    let current_m9 = fabric.current_m9_authority_inspection();
    let transition = fabric
        .m9_authority_lifecycle_mut()
        .revoke_designated_consumer_capability("E.result", "C")
        .expect("M9 lifecycle produces designated consumer revocation");
    let transition_view = transition.sealed_m9_inspection();
    assert_eq!(transition_view.prior_generation(), current_m9.generation());
    let successor_generation = transition_view.successor_generation().clone();
    let expected_consumer_lineage = transition_view.consumer_lineage().clone();
    fabric
        .apply_admitted_authority_lifecycle(transition)
        .expect("fabric installs M9 successor before cut");
    assert_eq!(
        fabric.current_m9_authority_inspection().generation(),
        successor_generation
    );

    let cut = save_sys4_local_cut(&mut fabric, "sys4-cut-after-consumer-revocation")
        .expect("ST cut after M9 revocation preserves authority lifecycle state");
    let mut restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("post-revocation cut restores");
    assert_eq!(
        restored.current_m9_authority_inspection().generation(),
        successor_generation,
        "restore must keep the M9 successor generation and tombstone, not revert to initial admission"
    );

    let semantic_identity = delivery.semantic_identity().to_string();
    let before_retry_state = restored.semantic_snapshot();
    let before_retry_cache = restored.designated_cache_snapshot();
    let before_retry_consumed = restored
        .m8_actual_trace()
        .value_consumed_count(&semantic_identity, "C");
    let before_retry_m8_cache_validations = m8_backend_trace_count(
        &restored,
        M8LocalTraceKind::DesignatedCacheValidated,
        &semantic_identity,
        "C",
    );
    let submitted = restored
        .submit_source_action(consume_designated_action())
        .expect("cache retry submits after restore");
    let rejected = assert_sys4_diag(
        restored.step_locus("C"),
        Sys4DiagnosticKind::MissingConsumerCapability,
    );
    assert_eq!(rejected.rejected_request_id(), Some(submitted.request_id()));
    let failure = rejected
        .m9_failure_inspection()
        .expect("restored revocation rejection exposes sealed M9 failure");
    assert_eq!(failure.installed_generation(), successor_generation);
    assert_eq!(failure.consumer_lineage(), &expected_consumer_lineage);
    assert_eq!(failure.semantic_identity(), semantic_identity);
    assert_eq!(failure.consumer_locus(), "C");
    assert!(rejected.m8_non_consuming_validation_node_id().is_none());
    assert_eq!(
        m8_backend_trace_count(
            &restored,
            M8LocalTraceKind::DesignatedCacheValidated,
            &semantic_identity,
            "C"
        ),
        before_retry_m8_cache_validations,
        "revoked restored retry must fail before M8 non-consuming cache validation"
    );
    assert_eq!(
        restored
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        before_retry_consumed
    );
    assert!(restored.semantic_snapshot().same_state(&before_retry_state));
    assert_eq!(restored.designated_cache_snapshot(), before_retry_cache);
}

#[test]
fn sys4_local_cut_restores_next_request_and_endpoint_counters_without_collision() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let first = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("first owner request submits before cut");
    let first_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let cut = save_sys4_local_cut(&mut fabric, "sys4-cut-next-id-counters")
        .expect("ST cut captures request and endpoint counters");
    let mut restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("counter cut restores");
    let second = restored
        .submit_source_action(owner_attack_action("attack"))
        .expect("second owner request submits after restore");
    let second_envelope = restored
        .locus_runtime("A")
        .expect("A exists after restore")
        .outgoing_mailbox()
        .pending_envelopes()
        .for_request(second.request_id());

    assert_ne!(first.request_id(), second.request_id());
    assert_ne!(first.envelope_id(), second.envelope_id());
    assert_ne!(first.carrier_id(), second.carrier_id());
    assert_ne!(first_envelope.envelope_id(), second_envelope.envelope_id());
    assert_ne!(first_envelope.carrier_id(), second_envelope.carrier_id());
    assert_ne!(
        first_envelope.mailbox_record_id(),
        second_envelope.mailbox_record_id(),
        "post-restore endpoint/mailbox record IDs must not collide with pre-cut pending carrier state"
    );
}

#[test]
fn sys4_local_cut_restores_owner_read_observer_safe_read_key_evidence_exactly() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("designated publish submits E→S input request");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());
    fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("input request transports to S");
    let source_step = fabric
        .step_locus("S")
        .expect("S validates source release and reads local source value");
    let read_audit_digest = source_step
        .local_store_read_audit()
        .expect("source read exposes read audit")
        .stable_digest();
    let before_cut_evidence = fabric.m8_partition_evidence();
    let before_read = before_cut_evidence
        .partition("S")
        .expect("S partition exists before cut")
        .m8_trace_occurrences_by_kind(M8LocalTraceKind::OwnerRead)
        .single()
        .clone();
    assert_eq!(
        before_read.observer_safe_read_key_refs(),
        vec!["player[self].atk"],
        "read-key evidence must name only the source-derived key and no payload"
    );

    let cut = save_sys4_local_cut(&mut fabric, "sys4-cut-owner-read-key-evidence")
        .expect("ST cut with source-owner read evidence succeeds");
    let restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("source-owner read evidence restores");
    assert_eq!(
        restored.local_store_read_audit("S").stable_digest(),
        read_audit_digest,
        "restore must preserve the exact per-occurrence local read audit"
    );
    let after_cut_evidence = restored.m8_partition_evidence();
    let after_read = after_cut_evidence
        .partition("S")
        .expect("S partition exists after restore")
        .m8_trace_occurrences_by_kind(M8LocalTraceKind::OwnerRead)
        .single()
        .clone();
    assert_eq!(
        after_read.fabric_qualified_id(),
        before_read.fabric_qualified_id()
    );
    assert_eq!(
        after_read.observer_safe_read_key_refs(),
        before_read.observer_safe_read_key_refs(),
        "restore must preserve exact observer-safe read-key evidence for the same M8 OwnerRead occurrence"
    );
}

#[test]
fn sys4_local_cut_ow1_returns_typed_backend_ineligible_without_panic_or_mutation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::Ow1);
    let before = fabric.semantic_snapshot();
    let before_trace = fabric.m8_actual_trace().stable_digest();

    let _diagnostics = assert_sys4_diag(
        save_sys4_local_cut(&mut fabric, "sys4-cut-ow1-typed-reject"),
        Sys4DiagnosticKind::BackendIneligible,
    );
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.m8_actual_trace().stable_digest(), before_trace);
}

#[test]
fn sys4_local_cut_restore_rejects_receive_without_matching_send_endpoint_record() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("owner request submits before cut tamper");
    let request = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("A", "S", request.envelope_id())
        .expect("request reaches S inbox before cut tamper");
    let mut cut = save_sys4_local_cut(&mut fabric, "sys4-cut-tampered-receive-without-send")
        .expect("ST cut captures matched A send and S receive before tamper");
    cut.for_test_drop_outgoing_endpoint_record("A", submitted.request_id(), request.carrier_id());

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_receive_record_drop_with_pending_inbox_and_source_send() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("owner request submits before cut tamper");
    let request = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let transport = fabric
        .step_transport("A", "S", request.envelope_id())
        .expect("request reaches S inbox before cut tamper");
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        request.envelope_id()
    );
    assert_eq!(
        fabric
            .locus_runtime("A")
            .expect("A exists")
            .outgoing_endpoint()
            .carrier_history_for_request(submitted.request_id())
            .carrier_history_len(),
        1,
        "source send record is present before target receive tamper"
    );
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_endpoint()
            .carrier_history_for_request(submitted.request_id())
            .carrier_history_len(),
        1,
        "target receive record is present before tamper"
    );

    let mut cut = save_sys4_local_cut(&mut fabric, "sys4-cut-tampered-receive-record-drop")
        .expect("ST cut captures matched source send, target receive, and S inbox before tamper");
    cut.for_test_drop_incoming_endpoint_record(
        "S",
        submitted.request_id(),
        request.carrier_id(),
        transport.target_inbox_enqueue_record_id(),
    );

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_older_cut_against_later_m9_generation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("delivery crosses E→C before old cut");
    fabric.step_locus("C").expect("C consumes before old cut");
    let old_cut = save_sys4_local_cut(&mut fabric, "sys4-cut-before-m9-revocation")
        .expect("old cut saves before later M9 successor");

    let transition = fabric
        .m9_authority_lifecycle_mut()
        .revoke_designated_consumer_capability("E.result", "C")
        .expect("M9 lifecycle produces later successor after old cut");
    fabric
        .apply_admitted_authority_lifecycle(transition)
        .expect("live fabric advances past old cut generation");

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &old_cut),
        Sys4DiagnosticKind::ProgramAdmissionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_authority_floor_advance_between_preflight_and_install() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut live = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut live);
    let delivery = live
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    live.step_transport("E", "C", delivery.envelope_id())
        .expect("delivery crosses E→C before cut");
    live.step_locus("C").expect("C consumes before cut");
    let old_cut = save_sys4_local_cut(&mut live, "sys4-cut-authority-floor-race-g0")
        .expect("old cut saves at G0 before interleaved floor advance");

    let live_generation_before_race = live.current_m9_authority_inspection().generation();
    let live_semantic_before_race = live.semantic_snapshot();
    let live_m8_before_race = live.m8_actual_trace().stable_digest();
    let transition = live
        .m9_authority_lifecycle_mut()
        .revoke_designated_consumer_capability("E.result", "C")
        .expect("M9 lifecycle produces competing G1 transition for restore race");
    let successor_generation = transition
        .sealed_m9_inspection()
        .successor_generation()
        .clone();
    assert_ne!(successor_generation, live_generation_before_race);

    assert_sys4_diag(
        LocalFabric::for_test_restore_local_cut_with_authority_floor_interleaving(
            program,
            admission,
            BackendProfile::St,
            &old_cut,
            transition,
        ),
        Sys4DiagnosticKind::ProgramAdmissionMismatch,
    );
    assert_eq!(
        live.current_m9_authority_inspection().generation(),
        live_generation_before_race,
        "a restore losing the authority-floor race must not install stale or competing authority into the live fabric"
    );
    assert!(
        live.semantic_snapshot()
            .same_state(&live_semantic_before_race),
        "a restore losing the authority-floor race must not mutate live semantic state"
    );
    assert_eq!(
        live.m8_actual_trace().stable_digest(),
        live_m8_before_race,
        "a restore losing the authority-floor race must not mutate the live backend/M8 trace"
    );
}

#[test]
fn sys4_local_cut_restore_rejects_duplicate_pending_inbox_receive_mapping() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("owner request submits before duplicate cut tamper");
    let request = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let transport = fabric
        .step_transport("A", "S", request.envelope_id())
        .expect("request reaches S inbox before duplicate tamper");
    let inbox = fabric
        .locus_runtime("S")
        .expect("S exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(inbox.envelope_id(), request.envelope_id());
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack", "S"),
        0,
        "pre-restore duplicate cut is taken before S can serve the request"
    );

    let mut cut = save_sys4_local_cut(&mut fabric, "sys4-cut-duplicate-pending-inbox")
        .expect("ST cut captures one valid pending S inbox owner request");
    cut.for_test_duplicate_pending_inbox_envelope_and_receive_record(
        "S",
        submitted.request_id(),
        request.carrier_id(),
        transport.target_inbox_enqueue_record_id(),
    );

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_counter_rollback_below_retained_ids() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let first = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("first request submits before counter rollback tamper");
    let first_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let mut cut = save_sys4_local_cut(&mut fabric, "sys4-cut-counter-rollback")
        .expect("ST cut captures retained request and endpoint IDs");
    cut.for_test_set_next_request_below_retained_max(first.request_id());
    cut.for_test_set_next_endpoint_occurrence_below_retained_max(
        first_envelope.mailbox_record_id(),
    );

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_fault_only_request_counter_rollback_without_endpoint_carrier() {
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
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let first_fault = fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::route_unavailable_for_edge(owner_request_edge.edge_ref()),
        ))
        .expect("route fault is source-derived and allocates a retained fabric request id");
    assert!(first_fault.is_fault());
    assert_eq!(
        fabric.trace().for_fault(first_fault.fault_id()).kinds(),
        vec![Sys4TraceKind::RequestAdmitted],
        "fault-only external actions retain a FabricTrace request row even without any endpoint carrier"
    );
    for locus in ["A", "S"] {
        let runtime = fabric.locus_runtime(locus).expect("locus exists");
        assert_eq!(
            runtime
                .outgoing_endpoint()
                .carrier_history_for_request(first_fault.fault_id())
                .carrier_history_len(),
            0,
            "fault-only action must not synthesize an outgoing endpoint carrier at {locus}"
        );
        assert_eq!(
            runtime
                .incoming_endpoint()
                .carrier_history_for_request(first_fault.fault_id())
                .carrier_history_len(),
            0,
            "fault-only action must not synthesize an incoming endpoint carrier at {locus}"
        );
    }

    let valid_cut = save_sys4_local_cut(&mut fabric, "sys4-cut-fault-only-request-retained")
        .expect("ST cut captures retained fault trace request id");
    let mut tampered = valid_cut.clone();
    tampered.for_test_set_next_request_below_retained_max(first_fault.fault_id());
    assert_sys4_diag(
        LocalFabric::restore_local_cut(
            program.clone(),
            admission.clone(),
            BackendProfile::St,
            &tampered,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );

    let mut restored =
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &valid_cut)
            .expect("untampered cut restores with the retained fault request floor");
    let second_fault = restored
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::route_unavailable_for_edge(owner_reply_edge.edge_ref()),
        ))
        .expect("a later fault allocates a fresh request id after restore");
    assert_ne!(
        second_fault.fault_id(),
        first_fault.fault_id(),
        "restored next_request must not reuse a retained fault-only FabricTrace request id"
    );
}

#[test]
fn sys4_local_cut_restore_rejects_missing_derived_m8_trace_or_causality_row() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let receipt = fabric
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("owner request serves before trace tamper cut");
    let evidence = fabric.m8_partition_evidence();
    let owner_write = evidence
        .partition("S")
        .expect("S partition exists")
        .single_m8_trace_occurrence_for_request(receipt.request_id(), M8LocalTraceKind::OwnerWrite)
        .clone();
    assert_eq!(
        fabric
            .m8_actual_trace()
            .node(owner_write.fabric_qualified_id())
            .kind(),
        M8LocalTraceKind::OwnerWrite
    );
    assert!(
        fabric
            .causality()
            .contains_occurrence(owner_write.fabric_qualified_id())
    );

    let mut missing_actual = save_sys4_local_cut(&mut fabric, "sys4-cut-missing-actual-m8-row")
        .expect("ST cut captures M8 cut plus derived trace rows");
    missing_actual.for_test_drop_actual_m8_trace_row(owner_write.fabric_qualified_id());
    assert_sys4_diag(
        LocalFabric::restore_local_cut(
            program.clone(),
            admission.clone(),
            BackendProfile::St,
            &missing_actual,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );

    let mut missing_causality = save_sys4_local_cut(&mut fabric, "sys4-cut-missing-causality-row")
        .expect("ST cut captures M8 cut plus causality rows");
    missing_causality.for_test_drop_causality_row(owner_write.fabric_qualified_id());
    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &missing_causality),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn sys4_local_cut_restore_rejects_actual_m8_predecessor_not_in_causality_or_raw_dependencies() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let program = fabric_program(projection);
    let admission = sealed_admission(&checked, &program);
    let mut fabric = boot_with_admission(program.clone(), admission.clone(), BackendProfile::St);

    let first = fabric
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("first owner request serves before trace predecessor tamper");
    let second = fabric
        .dispatch_source_action(owner_attack_action("attack"))
        .expect("second owner request provides an unrelated existing M8 occurrence");
    let evidence = fabric.m8_partition_evidence();
    let s_partition = evidence.partition("S").expect("S partition exists");
    let first_write = s_partition
        .single_m8_trace_occurrence_for_request(first.request_id(), M8LocalTraceKind::OwnerWrite)
        .clone();
    let second_write = s_partition
        .single_m8_trace_occurrence_for_request(second.request_id(), M8LocalTraceKind::OwnerWrite)
        .clone();
    assert_ne!(
        first_write.fabric_qualified_id(),
        second_write.fabric_qualified_id()
    );
    assert!(
        !fabric
            .causality()
            .predecessor_ids(first_write.fabric_qualified_id())
            .contains(&second_write.fabric_qualified_id().to_string()),
        "the later owner write is not a semantic or raw-M8 predecessor of the first write"
    );

    let mut cut = save_sys4_local_cut(&mut fabric, "sys4-cut-actual-m8-predecessor-forgery")
        .expect("ST cut captures exact actual M8 trace and causality rows");
    cut.for_test_add_actual_m8_trace_predecessor(
        first_write.fabric_qualified_id(),
        second_write.fabric_qualified_id(),
    );

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn ow1_designated_source_owner_read_is_m8_worker_owned_and_causally_acks_reply() {
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
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::Ow1);

    let submitted = fabric
        .submit_source_action(publish_designated_action())
        .expect("publish submission emits only the generated E→S input request");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());
    assert_eq!(input_request.edge_ref(), input_request_edge.edge_ref());
    assert_eq!(
        input_request.carrier_contract(),
        input_request_edge.carrier_contract()
    );

    fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("transport moves exact generated input request to source owner S");
    let semantic_before_s_step = fabric.semantic_snapshot();
    let local_store_before_s_step = fabric
        .locus_runtime("S")
        .expect("S exists")
        .local_store()
        .clone();
    let sequence_before_s_step = m8_backend_latest_sequence(&fabric);
    let owner_read_count_before_s_step = m8_trace_kind_count(&fabric, M8LocalTraceKind::OwnerRead);
    let owner_write_count_before_s_step =
        m8_trace_kind_count(&fabric, M8LocalTraceKind::OwnerWrite);
    let source_step = fabric
        .step_locus("S")
        .expect("S services source input after endpoint dequeue and M9 release validation");
    let source_release = source_step.m9_validation().source_release_inspection();
    let read_audit = source_step
        .local_store_read_audit()
        .expect("S reports the source-value read audit");
    let trace: &M8LocalTrace = m8_local_runtime_trace(&fabric);
    assert!(
        fabric
            .semantic_snapshot()
            .same_state(&semantic_before_s_step),
        "source-owner input service is a read-only path and must not mutate semantic state"
    );
    assert_eq!(
        fabric.locus_runtime("S").expect("S exists").local_store(),
        &local_store_before_s_step,
        "source-owner input service must not mutate S local store while reading"
    );
    assert_eq!(
        m8_trace_kind_count(&fabric, M8LocalTraceKind::OwnerWrite),
        owner_write_count_before_s_step,
        "source-owner input service must not report a write while servicing a read"
    );
    assert!(
        trace
            .latest_observation_for_occurrence(
                M8LocalTraceKind::OwnerWrite,
                read_audit.occurrence_id()
            )
            .is_none(),
        "read audit occurrence must not resolve to an M8 OwnerWrite"
    );
    assert_eq!(
        m8_trace_kind_count(&fabric, M8LocalTraceKind::OwnerRead),
        owner_read_count_before_s_step + 1,
        "source-owner input service must emit exactly one fresh M8 OwnerRead occurrence"
    );
    let owner_read = trace
        .latest_observation_for_occurrence(M8LocalTraceKind::OwnerRead, read_audit.occurrence_id())
        .expect("source-owner read audit occurrence must resolve to an M8-owned OwnerRead");
    assert!(
        owner_read.sequence() > sequence_before_s_step,
        "OW1 source-owner service must advance the worker-owned M8 trace when reading S state"
    );
    assert_eq!(owner_read.kind(), M8LocalTraceKind::OwnerRead);
    assert_eq!(owner_read.envelope_id(), input_request.envelope_id());
    assert_eq!(owner_read.operation_id(), "E.result");
    assert_eq!(owner_read.owner_locus(), "S");
    assert_eq!(owner_read.edge_ref(), input_request.edge_ref());
    assert_eq!(
        owner_read.logical_tick_id(),
        input_request.logical_tick_id()
    );
    assert_eq!(
        owner_read.logical_tick_frontier(),
        input_request.logical_tick_frontier()
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(owner_read.node_id())
            .contains(&source_release.occurrence_id().to_string()),
        "M8 OwnerRead must be causally after M9 source-release validation"
    );

    let input_receipt = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_receipt.edge_ref(), input_receipt_edge.edge_ref());
    assert_eq!(
        input_receipt.request_carrier_id(),
        input_request.carrier_id()
    );
    assert_eq!(input_receipt.typed_value(), RuntimeValue::int(10));
    let receipt_transport = fabric
        .step_transport("S", "E", input_receipt.envelope_id())
        .expect("transport moves the acknowledged S→E input receipt");
    assert!(
        causality_reaches(
            fabric.causality(),
            receipt_transport.source_outbox_dequeue_occurrence_id(),
            owner_read.node_id(),
        ),
        "input receipt must be causally downstream of the M8-owned source-owner read"
    );
}

#[test]
fn relation_only_projection_derives_ow1_worker_locus_without_owner_rmw_or_designated_source() {
    let checked = relation_only_checked();
    let projection = relation_only_projection(&checked);
    assert_eq!(
        projection
            .backend_requirements()
            .eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible,
        "a relation-only owner locus is a valid OW1 worker derivation candidate"
    );
    let fragments = projection.sys4_artifact_fragments();
    assert!(
        fragments
            .single(
                "bird_follow",
                ProjectedOperationFragmentKind::RelationPublication
            )
            .is_some(),
        "relation-only program has an S-owned relation publication fragment"
    );
    assert!(
        fragments
            .single(
                "bird_follow",
                ProjectedOperationFragmentKind::ConsumerLocalRelationProjection
            )
            .is_some(),
        "relation-only program has a C-local relation projection consumer fragment"
    );
    assert!(
        fragments
            .single(
                "bird_follow",
                ProjectedOperationFragmentKind::OwnerRmwExecution
            )
            .is_none()
    );
    assert!(
        fragments
            .single(
                "bird_follow",
                ProjectedOperationFragmentKind::DesignatedRemoteInputService
            )
            .is_none()
    );
    let relation_edge = projection
        .communication_plan()
        .single_edge(
            "bird_follow",
            CommunicationEdgeKind::RelationProjectionPublication,
            "S",
            "C",
        )
        .expect("relation-only projection derives S→C relation publication edge");
    assert_eq!(relation_edge.source_locus(), "S");
    assert_eq!(relation_edge.target_locus(), "C");

    let program = fabric_program(projection);
    assert_eq!(
        program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible
    );
    let sys4 = read_runtime_src("sys4_dispatch.rs");
    let bootstrap_body = extract_balanced_fn_body(&sys4, "pub(crate) fn bootstrap(")
        .expect("LocalFabric::bootstrap body is available for narrow derivation scan");
    let ow1_bootstrap_branch =
        extract_balanced_block_after_marker(&bootstrap_body, "BackendProfile::Ow1 =>")
            .expect("LocalFabric::bootstrap has an OW1 derivation branch");
    let compact_ow1 = normalize_source_for_boundary_scan(&ow1_bootstrap_branch);
    let compact_sys4 = normalize_source_for_boundary_scan(&sys4);
    assert!(
        !compact_sys4.contains("pubfnow1_worker_locus_candidates("),
        "OW1 worker-locus derivation helper must be crate-private, not public API"
    );
    assert!(
        compact_sys4.contains("pub(crate)fnow1_worker_locus_candidates("),
        "M9 relation-only SYS-4 admission is not available yet, so this structural RED pins a crate-private pure helper for OW1 worker-locus derivation"
    );
    assert!(
        compact_ow1.contains("ow1_worker_locus_candidates("),
        "LocalFabric::bootstrap OW1 branch must use the factored worker-locus derivation helper"
    );
    let helper_body = extract_balanced_fn_body(&sys4, "pub(crate) fn ow1_worker_locus_candidates(")
        .expect("OW1 relation-only derivation helper body is structurally inspectable");
    let compact_helper = normalize_source_for_boundary_scan(&helper_body);
    assert!(
        compact_helper.contains(
            "ifletSome(core)=fragment.relation_checked_core(){worker_loci.insert(core.owner_locus().to_string());}"
        ),
        "OW1 worker_loci derivation must exactly include relation_checked_core -> core.owner_locus().to_string() -> worker_loci.insert so relation-only projection derives S without owner-RMW/designated-source fragments"
    );
}

#[test]
fn staged_designated_publish_preserves_nondefault_tick_frontier_through_receipt_and_m8() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);
    let requested_tick = "tick:F:41";
    let requested_frontier = "F";

    let submitted = fabric
        .submit_source_action(publish_designated_action_with_tick(requested_tick))
        .expect("publish submission creates a generated E→S input request");
    let input_request = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_request.envelope_id(), submitted.envelope_id());
    assert_envelope_tick(&input_request, requested_tick, requested_frontier);

    fabric
        .step_transport("E", "S", input_request.envelope_id())
        .expect("input request transports to S");
    let source_step = fabric
        .step_locus("S")
        .expect("S validates source release and emits receipt");
    let input_receipt = fabric
        .locus_runtime("S")
        .expect("S exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(input_receipt.envelope_id(), source_step.reply_envelope_id());
    assert_eq!(input_receipt.typed_value(), RuntimeValue::int(10));
    assert_envelope_tick(&input_receipt, requested_tick, requested_frontier);

    fabric
        .step_transport("S", "E", input_receipt.envelope_id())
        .expect("input receipt transports to E");
    let evaluator_step = fabric
        .step_locus("E")
        .expect("E installs the receipt and evaluates through M8");
    assert_backend_publication_observation(
        &fabric,
        evaluator_step.m8_evaluation_node_id(),
        requested_tick,
        requested_frontier,
    );
}

#[test]
fn designated_publish_without_explicit_tick_fails_before_outbox_m8_or_state() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);
    let before = fabric.semantic_snapshot();
    let m8_before = fabric.m8_actual_trace().stable_digest();

    let diagnostics = assert_sys4_diag(
        fabric.submit_source_action(SourceAction::designated_tick("E.result")),
        missing_designated_tick_diag(),
    );

    assert!(diagnostics.rejected_envelope_id().is_none());
    assert!(diagnostics.endpoint_dequeue_occurrence_id().is_none());
    assert!(diagnostics.m8_trace_node_id().is_none());
    assert!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .is_empty(),
        "missing tick must fail before an E outbox carrier is materialized"
    );
    assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
    assert!(fabric.semantic_snapshot().same_state(&before));
}

#[test]
fn designated_delivery_envelope_seals_m8_publication_identity_tick_and_not_latest_map() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let first_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:41"))
        .expect("first publish creates E→S request");
    fabric
        .step_transport("E", "S", first_submit.envelope_id())
        .expect("first input request transports to S");
    let first_source_step = fabric.step_locus("S").expect("S emits first input receipt");
    fabric
        .step_transport("S", "E", first_source_step.reply_envelope_id())
        .expect("first input receipt transports to E");
    let first_evaluator_step = fabric
        .step_locus("E")
        .expect("E publishes first result delivery");
    let first_m8_delivery_id = first_evaluator_step
        .receipt()
        .expect("first evaluator step returns M8 publication receipt")
        .delivery_id()
        .to_string();
    let first_m8_version = first_evaluator_step
        .receipt()
        .expect("first evaluator step returns M8 publication receipt")
        .result_version()
        .expect("designated publication has a result version");
    let first_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(
        envelope_m8_publication_id(&first_delivery),
        first_m8_delivery_id
    );
    assert_eq!(
        binding_m8_publication_id(first_delivery.immutable_delivery_binding()),
        first_m8_delivery_id
    );
    assert_eq!(envelope_logical_tick_id(&first_delivery), "tick:F:41");
    assert_eq!(
        binding_logical_tick_id(first_delivery.immutable_delivery_binding()),
        "tick:F:41"
    );
    assert_eq!(envelope_logical_tick_frontier(&first_delivery), "F");
    assert_eq!(
        binding_logical_tick_frontier(first_delivery.immutable_delivery_binding()),
        "F"
    );

    let second_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:42"))
        .expect("second publish creates another E→S request while first delivery stays pending");
    fabric
        .step_transport("E", "S", second_submit.envelope_id())
        .expect("second input request transports to S without consuming first delivery");
    let second_source_step = fabric
        .step_locus("S")
        .expect("S emits second input receipt");
    fabric
        .step_transport("S", "E", second_source_step.reply_envelope_id())
        .expect("second input receipt transports to E");
    let second_evaluator_step = fabric
        .step_locus("E")
        .expect("E publishes second result delivery");
    let second_m8_delivery_id = second_evaluator_step
        .receipt()
        .expect("second evaluator step returns M8 publication receipt")
        .delivery_id()
        .to_string();
    let second_m8_version = second_evaluator_step
        .receipt()
        .expect("second evaluator step returns M8 publication receipt")
        .result_version()
        .expect("designated publication has a result version");

    assert_eq!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .len(),
        2,
        "two generated delivery envelopes must remain independently live at E before C consumes either"
    );
    assert_eq!(
        first_m8_version, second_m8_version,
        "current finite Core profile fixes the designated result version; SYS-4 must therefore seal the M8 publication identity/tick on each envelope instead of using an operation-latest map"
    );
    assert!(
        !second_m8_delivery_id.is_empty(),
        "second publication identity is recorded even when the fixed finite profile reuses the same result version"
    );

    fabric
        .step_transport("E", "C", first_delivery.envelope_id())
        .expect("A delivery transports to C out of order after B publication exists");
    let first_consume = fabric
        .step_locus("C")
        .expect("C consumes the exact first delivery envelope, not the operation-latest map");
    let first_receipt = first_consume
        .receipt()
        .expect("first delivery consume returns receipt");
    assert_eq!(
        first_receipt.delivery_id(),
        envelope_m8_publication_id(&first_delivery)
    );
    assert_eq!(
        receipt_m8_publication_id(first_receipt),
        envelope_m8_publication_id(&first_delivery)
    );
    assert_eq!(
        receipt_logical_tick_id(first_receipt),
        envelope_logical_tick_id(&first_delivery)
    );
    assert_backend_consume_observation(
        &fabric,
        first_consume.m8_consume_node_id(),
        envelope_m8_publication_id(&first_delivery),
        envelope_logical_tick_id(&first_delivery),
    );

    let cache_validation_count_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedCacheValidated,
        first_receipt.semantic_consumption_identity(),
        "C",
    );
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);
    let initial_consumer_authority_nodes = m8_backend_node_ids(
        &fabric,
        M8LocalTraceKind::DesignatedConsumerAuthorityValidated,
        first_receipt.semantic_consumption_identity(),
        "C",
    );
    let retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("retry validates cached first delivery even after B is latest");
    assert_eq!(
        receipt_m8_publication_id(&retry),
        envelope_m8_publication_id(&first_delivery)
    );
    assert_eq!(
        receipt_logical_tick_id(&retry),
        envelope_logical_tick_id(&first_delivery)
    );
    let validation_node_id = retry
        .m8_non_consuming_validation_node_id()
        .expect("retry returns actual M8 non-consuming cache validation id");
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            first_receipt.semantic_consumption_identity(),
            "C",
        ),
        cache_validation_count_before + 1,
        "retry must add exactly one fresh M8 cache-validation trace occurrence"
    );
    assert!(
        !initial_consumer_authority_nodes
            .iter()
            .any(|node| node == validation_node_id),
        "cache retry validation must not reuse the initial consumer-authority validation node"
    );
    assert_backend_cache_validation_observation(
        &fabric,
        validation_node_id,
        first_receipt.semantic_consumption_identity(),
        "C",
        envelope_m8_publication_id(&first_delivery),
        envelope_logical_tick_id(&first_delivery),
        prior_m8_sequence,
    );
}

#[test]
fn exact_duplicate_cache_retry_is_idempotent_but_corrupted_duplicate_binding_fails_closed() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("first consume installs the designated cache");
    let semantic_identity = first.semantic_consumption_identity().to_string();
    let consumed_after_first = fabric
        .m8_actual_trace()
        .value_consumed_count(&semantic_identity, "C");

    let exact_retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("exact same semantic delivery retry is idempotent where policy permits");
    assert_eq!(exact_retry.delivery_id(), first.delivery_id());
    assert_eq!(exact_retry.typed_value(), first.typed_value());
    assert_eq!(exact_retry.result_version(), first.result_version());
    assert!(exact_retry.returned_from_designated_cache_after_authority_revalidation());
    assert!(!exact_retry.performed_m8_semantic_consumption());
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        consumed_after_first,
        "exact duplicate retry must not perform a second M8 semantic consume"
    );

    let conflict = fabric
        .submit_source_action(consume_designated_action())
        .expect("conflicting duplicate starts as a real local cache-retry envelope");
    let conflict_envelope = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(conflict_envelope.envelope_id(), conflict.envelope_id());
    assert!(is_local_cache_retry(&conflict_envelope));

    let before_conflict = fabric.semantic_snapshot();
    let cache_before_conflict = fabric.designated_cache_snapshot();
    let consumed_before_conflict = fabric
        .m8_actual_trace()
        .value_consumed_count(&semantic_identity, "C");
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_local_cache_retry_binding_digest(
                conflict_envelope.envelope_id(),
            ),
        ))
        .expect("conflicting duplicate fault targets the exact local retry envelope");
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::CacheBindingDigestMismatch,
    );

    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(conflict_envelope.envelope_id())
    );
    assert!(rejected.m8_non_consuming_validation_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_conflict));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before_conflict);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        consumed_before_conflict
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(conflict_envelope.envelope_id())
            .expect("corrupted duplicate retry is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::CacheBindingDigestMismatch
    );
}

#[test]
fn fixed_version_second_tick_delivery_split_frame_rejects_before_second_consume() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let first_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:41"))
        .expect("first publish creates E→S request");
    fabric
        .step_transport("E", "S", first_submit.envelope_id())
        .expect("first input request transports to S");
    let first_source_step = fabric.step_locus("S").expect("S emits first input receipt");
    fabric
        .step_transport("S", "E", first_source_step.reply_envelope_id())
        .expect("first input receipt transports to E");
    let first_evaluator_step = fabric.step_locus("E").expect("E emits first delivery");
    let first_publication =
        m8_owned_observation(&fabric, first_evaluator_step.m8_evaluation_node_id());
    assert_eq!(
        first_publication.kind(),
        M8LocalTraceKind::DesignatedValuePublished
    );
    assert_eq!(first_publication.logical_tick_id(), "tick:F:41");
    let first_publication_tick = first_publication.logical_tick_id().to_string();
    let first_request_id = first_evaluator_step.request_id().to_string();
    let first_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .for_request(first_evaluator_step.request_id());
    let first_delivery_publication_id = envelope_m8_publication_id(&first_delivery).to_string();
    let first_delivery_tick = envelope_logical_tick_id(&first_delivery).to_string();
    let first_delivery_binding_tick =
        binding_logical_tick_id(first_delivery.immutable_delivery_binding()).to_string();
    let first_delivery_binding_publication_id =
        binding_m8_publication_id(first_delivery.immutable_delivery_binding()).to_string();
    let first_delivery_digest = first_delivery.immutable_delivery_digest().to_string();
    assert_eq!(
        first_delivery_binding_publication_id,
        first_delivery_publication_id
    );
    assert_eq!(first_delivery_tick, first_publication_tick);
    assert_eq!(first_delivery_binding_tick, first_publication_tick);

    let second_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:42"))
        .expect("second publish creates E→S request under fixed result_version");
    fabric
        .step_transport("E", "S", second_submit.envelope_id())
        .expect("second input request transports to S");
    let second_source_step = fabric
        .step_locus("S")
        .expect("S emits second input receipt");
    fabric
        .step_transport("S", "E", second_source_step.reply_envelope_id())
        .expect("second input receipt transports to E");
    let before_second_evaluation_sequence = m8_backend_latest_sequence(&fabric);
    let second_evaluator_step = fabric
        .step_locus("E")
        .expect("E emits second delivery candidate");
    assert_backend_designated_idempotent_observation(
        &fabric,
        second_evaluator_step.m8_evaluation_node_id(),
        second_evaluator_step.consumed_envelope_id(),
        "E.result",
        "E",
        "tick:F:42",
        before_second_evaluation_sequence,
    );
    let first_publication_after_second =
        m8_owned_observation(&fabric, first_evaluator_step.m8_evaluation_node_id());
    assert_eq!(
        first_publication_after_second.logical_tick_id(),
        first_publication_tick,
        "idempotent second evaluation must not rewrite the first M8 publication tick"
    );
    let e_outbox_after_second = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes();
    assert_eq!(
        e_outbox_after_second.len(),
        2,
        "A and B deliveries must remain independently queued until transport"
    );
    let first_delivery_after_second = e_outbox_after_second.for_request(&first_request_id);
    assert_eq!(
        envelope_m8_publication_id(&first_delivery_after_second),
        first_delivery_publication_id,
        "queued A delivery publication id must not be rewritten while B is generated"
    );
    assert_eq!(
        binding_m8_publication_id(first_delivery_after_second.immutable_delivery_binding()),
        first_delivery_binding_publication_id,
        "queued A delivery binding id must not be rewritten while B is generated"
    );
    assert_eq!(
        envelope_logical_tick_id(&first_delivery_after_second),
        first_delivery_tick,
        "queued A delivery tick must remain immutable while B is generated"
    );
    assert_eq!(
        binding_logical_tick_id(first_delivery_after_second.immutable_delivery_binding()),
        first_delivery_binding_tick,
        "queued A delivery binding tick must remain immutable while B is generated"
    );
    assert_eq!(
        first_delivery_after_second.immutable_delivery_digest(),
        first_delivery_digest,
        "queued A delivery digest must remain immutable while B is generated"
    );

    let second_delivery = e_outbox_after_second.for_request(second_evaluator_step.request_id());
    assert_eq!(
        envelope_m8_publication_id(&second_delivery),
        first_delivery_publication_id,
        "fixed-version idempotent evaluation reuses the accepted M8 publication identity"
    );
    assert_eq!(
        binding_m8_publication_id(second_delivery.immutable_delivery_binding()),
        first_delivery_publication_id,
        "B sealed binding must refer to the same fixed-version M8 publication id as A"
    );
    assert_eq!(
        second_delivery
            .immutable_delivery_binding()
            .result_version(),
        first_delivery.immutable_delivery_binding().result_version(),
        "finite profile keeps a fixed designated result_version"
    );
    assert_eq!(
        binding_logical_tick_id(second_delivery.immutable_delivery_binding()),
        "tick:F:42",
        "B sealed binding records the second source tick, making the frame split against the fixed M8 publication"
    );
    assert_eq!(
        second_delivery.immutable_delivery_digest(),
        format!("{:?}", second_delivery.immutable_delivery_binding()),
        "B immutable delivery digest must be derived from its exact sealed binding"
    );
    assert_ne!(
        second_delivery.immutable_delivery_digest(),
        first_delivery_digest,
        "B digest must differ from captured A digest because the sealed binding tick differs"
    );

    fabric
        .step_transport("E", "C", first_delivery_after_second.envelope_id())
        .expect("A delivery crosses E→C after B has already been generated");
    let first_consume = fabric
        .step_locus("C")
        .expect("A delivery consumes after B generation did not mutate it");
    let first_receipt = first_consume
        .receipt()
        .expect("first consume returns receipt");
    assert_eq!(
        receipt_logical_tick_id(first_receipt),
        envelope_logical_tick_id(&first_delivery_after_second)
    );
    assert_eq!(
        second_delivery
            .immutable_delivery_binding()
            .result_version(),
        first_receipt
            .result_version()
            .expect("first designated consume records result version"),
        "B carrier remains tied to the same fixed result_version after A consume"
    );
    assert_eq!(
        envelope_logical_tick_id(&second_delivery),
        "tick:F:42",
        "SYS-4 currently seals the second source tick into the generated delivery"
    );
    assert_ne!(
        envelope_logical_tick_id(&second_delivery),
        first_delivery_tick,
        "the second generated carrier tick differs from the first delivery tick"
    );
    assert_ne!(
        envelope_logical_tick_id(&second_delivery),
        first_publication_tick,
        "the second generated carrier tick differs from the M8 publication tick"
    );
    assert_eq!(
        envelope_logical_tick_id(&first_delivery_after_second),
        first_delivery_tick,
        "A delivery remains immutable through B generation and A consume"
    );
    assert_eq!(
        binding_logical_tick_id(first_delivery_after_second.immutable_delivery_binding()),
        first_delivery_binding_tick,
        "A delivery binding tick remains immutable through B generation and A consume"
    );

    let before_second = fabric.semantic_snapshot();
    let cache_before_second = fabric.designated_cache_snapshot();
    let consumed_before_second = fabric
        .m8_actual_trace()
        .value_consumed_count(first_delivery.semantic_identity(), "C");
    let m8_trace_digest_before_second = fabric.m8_actual_trace().stable_digest();
    fabric
        .step_transport("E", "C", second_delivery.envelope_id())
        .expect("second split-frame candidate crosses the generated E→C endpoint");
    let b_step = fabric.step_locus("C");
    assert!(
        b_step.is_err(),
        "B split-frame was accepted by C before A; expected fail-closed DeliveryPublicationIdentityMismatch"
    );
    let rejected = assert_sys4_diag(b_step, delivery_publication_identity_mismatch_diag());

    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(second_delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_second));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before_second);
    assert_eq!(
        fabric.m8_actual_trace().stable_digest(),
        m8_trace_digest_before_second,
        "B split-frame rejection must not add rejected, non-consuming, or consuming M8 trace rows"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(first_delivery.semantic_identity(), "C"),
        consumed_before_second,
        "split-frame candidate must not perform a second M8 consume"
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(second_delivery.envelope_id())
            .expect("split-frame delivery is terminally quarantined")
            .diagnostic_kind(),
        delivery_publication_identity_mismatch_diag()
    );
}

#[test]
fn b_before_a_fixed_version_split_frame_rejects_with_empty_consumer_cache() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let first_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:41"))
        .expect("A publish creates E→S request");
    fabric
        .step_transport("E", "S", first_submit.envelope_id())
        .expect("A input request transports to S");
    let first_source_step = fabric.step_locus("S").expect("S emits A input receipt");
    fabric
        .step_transport("S", "E", first_source_step.reply_envelope_id())
        .expect("A input receipt transports to E");
    let first_evaluator_step = fabric.step_locus("E").expect("E emits A delivery");
    let first_publication =
        m8_owned_observation(&fabric, first_evaluator_step.m8_evaluation_node_id());
    assert_eq!(
        first_publication.kind(),
        M8LocalTraceKind::DesignatedValuePublished
    );
    assert_eq!(first_publication.logical_tick_id(), "tick:F:41");
    let first_request_id = first_evaluator_step.request_id().to_string();
    let first_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .for_request(&first_request_id);
    let first_publication_id = envelope_m8_publication_id(&first_delivery).to_string();
    let first_tick = envelope_logical_tick_id(&first_delivery).to_string();
    let first_binding_tick =
        binding_logical_tick_id(first_delivery.immutable_delivery_binding()).to_string();
    let first_digest = first_delivery.immutable_delivery_digest().to_string();
    assert_eq!(first_tick, "tick:F:41");
    assert_eq!(first_binding_tick, "tick:F:41");

    let second_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:42"))
        .expect("B publish creates E→S request while A stays queued");
    fabric
        .step_transport("E", "S", second_submit.envelope_id())
        .expect("B input request transports to S while A stays queued");
    let second_source_step = fabric.step_locus("S").expect("S emits B input receipt");
    fabric
        .step_transport("S", "E", second_source_step.reply_envelope_id())
        .expect("B input receipt transports to E");
    let before_b_evaluation_sequence = m8_backend_latest_sequence(&fabric);
    let second_evaluator_step = fabric
        .step_locus("E")
        .expect("E emits B split-frame candidate");
    assert_backend_designated_idempotent_observation(
        &fabric,
        second_evaluator_step.m8_evaluation_node_id(),
        second_evaluator_step.consumed_envelope_id(),
        "E.result",
        "E",
        "tick:F:42",
        before_b_evaluation_sequence,
    );

    let e_outbox = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes();
    assert_eq!(e_outbox.len(), 2);
    let first_still_queued = e_outbox.for_request(&first_request_id);
    assert_eq!(
        envelope_m8_publication_id(&first_still_queued),
        first_publication_id
    );
    assert_eq!(
        binding_m8_publication_id(first_still_queued.immutable_delivery_binding()),
        first_publication_id
    );
    assert_eq!(envelope_logical_tick_id(&first_still_queued), first_tick);
    assert_eq!(
        binding_logical_tick_id(first_still_queued.immutable_delivery_binding()),
        first_binding_tick
    );
    assert_eq!(first_still_queued.immutable_delivery_digest(), first_digest);

    let second_delivery = e_outbox.for_request(second_evaluator_step.request_id());
    assert_eq!(
        envelope_m8_publication_id(&second_delivery),
        first_publication_id,
        "B reuses the fixed-version M8 publication id"
    );
    assert_eq!(
        binding_m8_publication_id(second_delivery.immutable_delivery_binding()),
        first_publication_id,
        "B sealed binding reuses the fixed-version M8 publication id"
    );
    assert_eq!(
        envelope_logical_tick_id(&second_delivery),
        "tick:F:42",
        "B generated carrier records the new source tick"
    );
    assert_eq!(
        binding_logical_tick_id(second_delivery.immutable_delivery_binding()),
        "tick:F:42",
        "B sealed binding records the new source tick"
    );
    assert_eq!(
        second_delivery.immutable_delivery_digest(),
        format!("{:?}", second_delivery.immutable_delivery_binding()),
        "B digest is coherent with the exact sealed binding"
    );
    assert_ne!(
        second_delivery.immutable_delivery_digest(),
        first_digest,
        "B digest differs from A because B seals a different tick"
    );
    assert!(
        fabric
            .designated_cache_entry(second_delivery.semantic_identity())
            .is_none(),
        "C cache is intentionally empty because B is processed before A"
    );

    fabric
        .step_transport("E", "C", second_delivery.envelope_id())
        .expect("B crosses E→C before A");
    assert_eq!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .single()
            .envelope_id(),
        first_still_queued.envelope_id(),
        "A remains queued at E while B is processed first"
    );
    let before_b = fabric.semantic_snapshot();
    let cache_before_b = fabric.designated_cache_snapshot();
    let m9_before_b = m9_validation_occurrence_count(
        &fabric,
        "E.result",
        "C",
        second_delivery.semantic_identity(),
    );
    let m8_digest_before_b = fabric.m8_actual_trace().stable_digest();
    let m8_backend_sequence_before_b = m8_backend_latest_sequence(&fabric);
    let consumed_before_b = fabric
        .m8_actual_trace()
        .value_consumed_count(second_delivery.semantic_identity(), "C");

    let b_step = fabric.step_locus("C");
    assert!(
        b_step.is_err(),
        "B split-frame was accepted by C before A; expected fail-closed DeliveryPublicationIdentityMismatch"
    );
    let rejected = assert_sys4_diag(b_step, delivery_publication_identity_mismatch_diag());
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(second_delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(rejected.m8_non_consuming_validation_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_b));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before_b);
    assert_eq!(
        m9_validation_occurrence_count(
            &fabric,
            "E.result",
            "C",
            second_delivery.semantic_identity()
        ),
        m9_before_b,
        "B split-frame rejection must happen before M9 consumer validation"
    );
    assert_eq!(
        fabric.m8_actual_trace().stable_digest(),
        m8_digest_before_b,
        "B split-frame rejection must happen before any M8 rejected/non-consuming/consume trace"
    );
    assert_eq!(
        m8_backend_latest_sequence(&fabric),
        m8_backend_sequence_before_b,
        "B split-frame rejection must not append to the M8LocalTrace backend"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(second_delivery.semantic_identity(), "C"),
        consumed_before_b
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(second_delivery.envelope_id())
            .expect("B split-frame delivery is terminally quarantined")
            .diagnostic_kind(),
        delivery_publication_identity_mismatch_diag()
    );
}

#[test]
fn designated_delivery_mismatched_m8_publication_identity_rejects_before_consume() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
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
    let original_identity = delivery.semantic_identity().to_string();
    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let partition_before = fabric.m8_partition_evidence();
    let c_imports_before = partition_before
        .partition("C")
        .expect("C partition exists")
        .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
        .len();
    let c_consumes_before = partition_before
        .partition("C")
        .expect("C partition exists")
        .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
        .len();
    let consumed_before = fabric
        .m8_actual_trace()
        .value_consumed_count(&original_identity, "C");

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_in_transit_envelope_m8_publication_id_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
                "m8-forged-publication-id",
            ),
        ))
        .expect("fault selector targets one checked delivery edge/envelope");
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("corrupted delivery transports to C");
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        delivery_publication_identity_mismatch_diag(),
    );

    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    let partition_after = fabric.m8_partition_evidence();
    assert_eq!(
        partition_after
            .partition("C")
            .expect("C partition exists")
            .publication_inventory(),
        partition_before
            .partition("C")
            .expect("C partition exists")
            .publication_inventory(),
        "publication-identity corruption must not import or mutate C's M8 publication inventory"
    );
    assert_eq!(
        partition_after
            .partition("C")
            .expect("C partition exists")
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
            .len(),
        c_imports_before,
        "rejected corrupt delivery must not create a C import occurrence"
    );
    assert_eq!(
        partition_after
            .partition("C")
            .expect("C partition exists")
            .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
            .len(),
        c_consumes_before,
        "rejected corrupt delivery must not create a C consume occurrence"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&original_identity, "C"),
        consumed_before
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(delivery.envelope_id())
            .expect("mismatched identity carrier is quarantined")
            .diagnostic_kind(),
        delivery_publication_identity_mismatch_diag()
    );
}

#[test]
fn forged_designated_carrier_provenance_visibility_or_redaction_quarantines_before_c_import() {
    enum ForgeryCase {
        SourceRef,
        Visibility,
        Redaction,
    }

    for case in [
        ForgeryCase::SourceRef,
        ForgeryCase::Visibility,
        ForgeryCase::Redaction,
    ] {
        let checked = designated_checked();
        let projection = designated_projection(&checked);
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
        let semantic_identity = delivery.semantic_identity().to_string();
        let before = fabric.semantic_snapshot();
        let cache_before = fabric.designated_cache_snapshot();
        let c_partition_before = fabric
            .m8_partition_evidence()
            .partition("C")
            .expect("C partition exists")
            .clone();
        let m8_digest_before = fabric.m8_actual_trace().stable_digest();
        let expected = match case {
            ForgeryCase::SourceRef => {
                fabric
                    .dispatch_external_action(ExternalAction::fault_event(
                        FaultInjection::corrupt_in_transit_envelope_source_ref_for_edge(
                            delivery_edge.edge_ref(),
                            delivery.envelope_id(),
                            "tests/forged/sys4_source_ref.mir",
                        ),
                    ))
                    .expect("source-ref forgery fault targets the exact delivery edge/envelope");
                Sys4DiagnosticKind::CarrierProvenanceMismatch
            }
            ForgeryCase::Visibility => {
                fabric
                    .dispatch_external_action(ExternalAction::fault_event(
                        FaultInjection::corrupt_in_transit_envelope_visibility_for_edge(
                            delivery_edge.edge_ref(),
                            delivery.envelope_id(),
                            "sys4-forged-visibility",
                        ),
                    ))
                    .expect("visibility forgery fault targets the exact delivery edge/envelope");
                Sys4DiagnosticKind::CarrierVisibilityMismatch
            }
            ForgeryCase::Redaction => {
                fabric
                    .dispatch_external_action(ExternalAction::fault_event(
                        FaultInjection::corrupt_in_transit_envelope_redaction_for_edge(
                            delivery_edge.edge_ref(),
                            delivery.envelope_id(),
                        ),
                    ))
                    .expect("redaction forgery fault targets the exact delivery edge/envelope");
                Sys4DiagnosticKind::CarrierRedactionMismatch
            }
        };

        fabric
            .step_transport("E", "C", delivery.envelope_id())
            .expect("forged delivery still crosses endpoint before C rejects it");
        let rejected = assert_sys4_diag(fabric.step_locus("C"), expected);
        assert_eq!(
            rejected.rejected_envelope_id(),
            Some(delivery.envelope_id())
        );
        assert!(rejected.m8_trace_node_id().is_none());
        assert!(!rejected.exposes_raw_payload());
        assert!(fabric.semantic_snapshot().same_state(&before));
        assert_eq!(fabric.designated_cache_snapshot(), cache_before);
        assert_eq!(
            fabric.m8_actual_trace().stable_digest(),
            m8_digest_before,
            "carrier forgery must be rejected before C import or consume mutates M8 trace"
        );
        let c_partition_after = fabric
            .m8_partition_evidence()
            .partition("C")
            .expect("C partition exists")
            .clone();
        assert_eq!(
            c_partition_after.publication_inventory(),
            c_partition_before.publication_inventory(),
            "forged carrier must not import or mutate C's publication inventory"
        );
        assert_eq!(
            c_partition_after
                .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
                .len(),
            c_partition_before
                .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedPublicationImported)
                .len(),
            "forged carrier must not create a C import occurrence"
        );
        assert_eq!(
            c_partition_after
                .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
                .len(),
            c_partition_before
                .m8_trace_occurrences_by_kind(M8LocalTraceKind::DesignatedValueConsumed)
                .len(),
            "forged carrier must not create a C consume occurrence"
        );
        assert_eq!(
            fabric
                .m8_actual_trace()
                .value_consumed_count(&semantic_identity, "C"),
            0
        );
        assert_eq!(
            fabric
                .locus_runtime("C")
                .expect("C exists")
                .incoming_mailbox()
                .terminal_rejected_envelope(delivery.envelope_id())
                .expect("forged delivery is terminally quarantined")
                .diagnostic_kind(),
            expected
        );
    }
}

#[test]
fn stale_publication_identity_fault_preserves_exact_carrier_before_reject() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
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
    let semantic_identity = delivery.semantic_identity().to_string();
    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let consumed_before = fabric
        .m8_actual_trace()
        .value_consumed_count(&semantic_identity, "C");

    let stale_publication_id = "m8-stale-publication-id";
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_in_transit_envelope_m8_publication_id_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
                stale_publication_id,
            ),
        ))
        .expect("stale-publication fault selector targets one checked delivery edge/envelope");
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("stale delivery transports to C for fail-closed target validation");
    let stale_delivery = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(
        envelope_m8_publication_id(&stale_delivery),
        stale_publication_id,
        "stale publication fault must mutate the actual generated carrier to the exact supplied stale identity, not a hardcoded sentinel"
    );
    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        delivery_publication_identity_mismatch_diag(),
    );

    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(delivery.envelope_id())
    );
    assert!(rejected.m8_trace_node_id().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        consumed_before
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(delivery.envelope_id())
            .expect("stale delivery carrier is terminally quarantined")
            .diagnostic_kind(),
        delivery_publication_identity_mismatch_diag()
    );
}

#[test]
fn split_frame_policy_digest_fault_rejects_and_terminally_quarantines() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
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
    let semantic_identity = delivery.semantic_identity().to_string();
    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let consumed_before = fabric
        .m8_actual_trace()
        .value_consumed_count(&semantic_identity, "C");

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_in_transit_envelope_policy_for_edge(
                delivery_edge.edge_ref(),
                delivery.envelope_id(),
            ),
        ))
        .expect("split-frame policy fault selector targets one checked delivery edge/envelope");
    fabric
        .step_transport("E", "C", delivery.envelope_id())
        .expect("split-frame delivery transports to C for fail-closed target validation");
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
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .m8_actual_trace()
            .value_consumed_count(&semantic_identity, "C"),
        consumed_before
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(delivery.envelope_id())
            .expect("split-frame carrier is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::CarrierPolicyMismatch
    );
}

#[test]
fn m8_cache_retry_validation_node_id_is_actual_m8_trace_occurrence_not_sys4_token() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let semantic_identity = first.semantic_consumption_identity().to_string();
    let cache_validation_count_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedCacheValidated,
        &semantic_identity,
        "C",
    );
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);
    let initial_consumer_authority_nodes = m8_backend_node_ids(
        &fabric,
        M8LocalTraceKind::DesignatedConsumerAuthorityValidated,
        &semantic_identity,
        "C",
    );
    let retry = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("retry returns cache after live authority revalidation");
    let validation_node_id = retry
        .m8_non_consuming_validation_node_id()
        .expect("retry reports the M8 non-consuming cache validation occurrence");
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            &semantic_identity,
            "C",
        ),
        cache_validation_count_before + 1,
        "cache retry must append exactly one M8-owned cache-validation occurrence"
    );
    assert!(
        !initial_consumer_authority_nodes
            .iter()
            .any(|node| node == validation_node_id),
        "cache retry validation must not reuse the initial consumer-authority validation node"
    );
    assert_backend_cache_validation_observation(
        &fabric,
        validation_node_id,
        &semantic_identity,
        "C",
        receipt_m8_publication_id(&retry),
        receipt_logical_tick_id(&retry),
        prior_m8_sequence,
    );
}

#[test]
fn cache_retry_faulted_binding_digest_rejects_before_m9_m8_or_payload() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("first consume succeeds");
    let semantic_identity = first.semantic_consumption_identity().to_string();
    let retry_submission = fabric
        .submit_source_action(consume_designated_action())
        .expect("retry creates a local CacheRetry envelope in C inbox");
    let retry_envelope = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(retry_envelope.envelope_id(), retry_submission.envelope_id());
    assert!(is_local_cache_retry(&retry_envelope));

    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let m9_validation_before =
        m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity);
    let m8_nonconsume_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedCacheValidated,
        &semantic_identity,
        "C",
    );
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::corrupt_local_cache_retry_binding_digest(retry_envelope.envelope_id()),
        ))
        .expect("local cache-retry fault selector is bound to the exact envelope id");

    let rejected = assert_sys4_diag(fabric.step_locus("C"), cache_binding_digest_mismatch_diag());
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(retry_envelope.envelope_id())
    );
    assert!(rejected.m9_failure_inspection().is_none());
    assert!(rejected.m8_non_consuming_validation_node_id().is_none());
    assert!(rejected.primary().typed_success().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity),
        m9_validation_before,
        "cache binding corruption must fail before live M9 validation"
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            &semantic_identity,
            "C",
        ),
        m8_nonconsume_before,
        "cache binding corruption must fail before M8 non-consuming validation"
    );
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(retry_envelope.envelope_id())
            .expect("faulted cache retry is quarantined")
            .diagnostic_kind(),
        cache_binding_digest_mismatch_diag()
    );
}

#[test]
fn post_dequeue_m8_consume_failure_quarantines_and_does_not_head_block_later_delivery() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    let first_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let semantic_identity = first_delivery.semantic_identity().to_string();
    let publication_id = envelope_m8_publication_id(&first_delivery).to_string();
    let logical_tick = envelope_logical_tick_id(&first_delivery).to_string();
    fabric
        .step_transport("E", "C", first_delivery.envelope_id())
        .expect("first delivery transports to C");
    fabric
        .m8_backend_test_support_mut()
        .reject_next_designated_consume_after_validation(
            first_delivery.envelope_id(),
            &publication_id,
            "C",
        )
        .expect("fault is armed inside actual M8 backend test support, not as a SYS-4 precheck");
    let before_failure = fabric.semantic_snapshot();
    let cache_before_failure = fabric.designated_cache_snapshot();
    let m9_validation_before =
        m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity);
    let m8_success_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedValueConsumed,
        &semantic_identity,
        "C",
    );
    let m8_reject_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedConsumptionRejected,
        &semantic_identity,
        "C",
    );
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);

    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::M8ExecutionRejected,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(first_delivery.envelope_id())
    );
    assert!(
        rejected.endpoint_dequeue_occurrence_id().is_some(),
        "failure is post-dequeue: C has already consumed the endpoint envelope before backend M8 rejects"
    );
    assert_eq!(
        m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity),
        m9_validation_before + 1,
        "backend failure is armed after live M9 validation, so M9 validation evidence must advance"
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedValueConsumed,
            &semantic_identity,
            "C",
        ),
        m8_success_before,
        "rejected backend consume attempt must not be counted as a successful value consumption"
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedConsumptionRejected,
            &semantic_identity,
            "C",
        ),
        m8_reject_before + 1,
        "backend rejection must append exactly one M8-owned failure occurrence"
    );
    let backend_failure = rejected
        .backend_m8_failure_inspection()
        .expect("diagnostic must expose typed M8 backend failure evidence");
    let backend_node_id = backend_failure.node_id();
    assert_eq!(
        rejected.m8_trace_node_id(),
        Some(backend_node_id),
        "diagnostic trace node must be the actual M8 backend rejection occurrence"
    );
    assert_eq!(
        backend_failure.kind(),
        M8LocalTraceKind::DesignatedConsumptionRejected
    );
    assert_eq!(backend_failure.envelope_id(), first_delivery.envelope_id());
    assert_eq!(backend_failure.m8_publication_id(), publication_id);
    assert_eq!(backend_failure.consumer_locus(), "C");
    assert_backend_consumption_rejection_observation(
        &fabric,
        backend_node_id,
        first_delivery.envelope_id(),
        &semantic_identity,
        "C",
        &publication_id,
        &logical_tick,
        prior_m8_sequence,
    );
    assert!(rejected.primary().typed_success().is_none());
    assert!(!rejected.exposes_raw_payload());
    assert!(fabric.semantic_snapshot().same_state(&before_failure));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before_failure);
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(first_delivery.envelope_id())
            .expect("post-dequeue M8 failure is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::M8ExecutionRejected
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
        .expect("later clean delivery transports after prior quarantine");
    let clean = fabric
        .step_locus("C")
        .expect("terminal quarantine must not head-block a later clean delivery");
    assert_eq!(
        clean
            .receipt()
            .expect("clean delivery returns receipt")
            .typed_value(),
        RuntimeValue::int(11)
    );
}

#[test]
fn initial_designated_delivery_auth_failures_remain_distinct_sealed_m9_causes() {
    #[derive(Clone, Copy)]
    enum InitialAuthCase {
        Membership,
        Capability,
        Witness,
    }

    for auth_case in [
        InitialAuthCase::Membership,
        InitialAuthCase::Capability,
        InitialAuthCase::Witness,
    ] {
        let checked = designated_checked();
        let program = fabric_program(designated_projection(&checked));
        let mut fabric = boot(&checked, program, BackendProfile::St);
        stage_designated_publish_until_delivery_outbox(&mut fabric);
        let delivery = fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .single();
        let semantic_identity = delivery.semantic_identity().to_string();
        let current_m9 = fabric.current_m9_authority_inspection();
        let transition = match auth_case {
            InitialAuthCase::Membership => fabric
                .m9_authority_lifecycle_mut()
                .retire_designated_consumer_membership("E.result", "C")
                .expect("membership retirement is produced by M9 lifecycle"),
            InitialAuthCase::Capability => fabric
                .m9_authority_lifecycle_mut()
                .revoke_designated_consumer_capability("E.result", "C")
                .expect("capability revocation is produced by M9 lifecycle"),
            InitialAuthCase::Witness => fabric
                .m9_authority_lifecycle_mut()
                .retire_designated_consumer_witness("E.result", "C")
                .expect("witness retirement is produced by M9 lifecycle"),
        };
        let transition_view = transition.sealed_m9_inspection();
        let expected_diag = match auth_case {
            InitialAuthCase::Membership => Sys4DiagnosticKind::MissingConsumerMembership,
            InitialAuthCase::Capability => Sys4DiagnosticKind::MissingConsumerCapability,
            InitialAuthCase::Witness => Sys4DiagnosticKind::MissingConsumerWitness,
        };
        let expected_error = match auth_case {
            InitialAuthCase::Membership => {
                crate::m9_auth_verification::M9AdmissionErrorKind::InvalidMembershipLineage
            }
            InitialAuthCase::Capability | InitialAuthCase::Witness => {
                crate::m9_auth_verification::M9AdmissionErrorKind::InvalidCapabilityLineage
            }
        };
        let expected_transition = match auth_case {
            InitialAuthCase::Membership => {
                crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerMembershipRetired
            }
            InitialAuthCase::Capability => {
                crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerCapabilityRevoked
            }
            InitialAuthCase::Witness => {
                crate::m9_auth_verification::M9AuthorityTransitionKind::DesignatedConsumerWitnessRetired
            }
        };
        assert_eq!(transition_view.transition_kind(), expected_transition);
        assert_eq!(transition_view.prior_generation(), current_m9.generation());
        let successor_generation = transition_view.successor_generation().clone();
        let expected_lineage = transition_view.consumer_lineage().clone();
        fabric
            .apply_admitted_authority_lifecycle(transition)
            .expect("fabric installs the M9 successor before first delivery consume");

        fabric
            .step_transport("E", "C", delivery.envelope_id())
            .expect("delivery transports to C before C validates live consumer authority");
        let before_reject = fabric.semantic_snapshot();
        let m8_before = fabric.m8_actual_trace().stable_digest();
        let rejected = assert_sys4_diag(fabric.step_locus("C"), expected_diag);
        assert_eq!(
            rejected.rejected_envelope_id(),
            Some(delivery.envelope_id())
        );
        let failure = rejected
            .m9_failure_inspection()
            .expect("initial delivery rejection must expose sealed M9 failure evidence");
        assert_eq!(failure.admission_error_kind(), expected_error);
        assert_eq!(failure.installed_generation(), successor_generation);
        assert_eq!(failure.consumer_lineage(), &expected_lineage);
        assert_eq!(failure.semantic_identity(), &semantic_identity);
        assert_eq!(failure.consumer_locus(), "C");
        assert!(rejected.m8_trace_node_id().is_none());
        assert!(rejected.primary().typed_success().is_none());
        assert!(!rejected.exposes_raw_payload());
        assert!(fabric.semantic_snapshot().same_state(&before_reject));
        assert_eq!(fabric.m8_actual_trace().stable_digest(), m8_before);
        assert_eq!(
            fabric
                .m8_actual_trace()
                .value_consumed_count(&semantic_identity, "C"),
            0
        );
    }
}

#[test]
fn endpoint_records_and_trace_rows_expose_exact_envelope_provenance_not_booleans() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("owner request submits a generated A outbox envelope");
    let request_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    let transport = fabric
        .step_transport("A", "S", request_envelope.envelope_id())
        .expect("transport moves owner request through endpoints");

    let source_history = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_endpoint()
        .carrier_history_for_request(submitted.request_id());
    let target_history = fabric
        .locus_runtime("S")
        .expect("S exists")
        .incoming_endpoint()
        .carrier_history_for_request(submitted.request_id());
    assert_eq!(
        source_history.carrier_history_len(),
        1,
        "source LocusRuntime outgoing endpoint must own the actual outbox dequeue carrier record, not leave endpoint evidence only in a global side list"
    );
    assert_eq!(
        target_history.carrier_history_len(),
        1,
        "target LocusRuntime incoming endpoint must own the actual inbox enqueue carrier record"
    );

    let source_record = source_history.single(CommunicationEdgeKind::OwnerRequest, "A", "S");
    let target_record = target_history.single(CommunicationEdgeKind::OwnerRequest, "A", "S");
    assert_endpoint_record_provenance(
        &source_record,
        &owner_request_edge,
        request_envelope.carrier_id(),
    );
    assert_endpoint_record_provenance(
        &target_record,
        &owner_request_edge,
        request_envelope.carrier_id(),
    );

    let dispatch_row = fabric.trace().endpoint_row_for_carrier(
        request_envelope.carrier_id(),
        Sys4TraceKind::Dispatched,
        transport.source_outbox_dequeue_record_id(),
        "A",
        "S",
    );
    let receive_row = fabric.trace().endpoint_row_for_carrier(
        request_envelope.carrier_id(),
        Sys4TraceKind::Received,
        transport.target_inbox_enqueue_record_id(),
        "A",
        "S",
    );
    assert_trace_row_provenance(dispatch_row, &owner_request_edge);
    assert_trace_row_provenance(receive_row, &owner_request_edge);
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
fn designated_generated_dispatch_replay_is_deterministic_and_st_ow1_correspondent() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let admission = sealed_admission(&checked, &program);
    let log = designated_replay_log();

    let st_first = run_designated_replay(BackendProfile::St, &program, &admission, &log);
    let st_second = run_designated_replay(BackendProfile::St, &program, &admission, &log);
    assert_eq!(
        st_first, st_second,
        "ST replay of the same source-action log must be deterministic across independent boots"
    );

    let ow1_first = run_designated_replay(BackendProfile::Ow1, &program, &admission, &log);
    let ow1_second = run_designated_replay(BackendProfile::Ow1, &program, &admission, &log);
    assert_eq!(
        ow1_first, ow1_second,
        "OW1 replay of the same source-action log must be deterministic across independent boots"
    );

    assert_eq!(
        st_first.semantic_snapshot, ow1_first.semantic_snapshot,
        "ST and OW1 designated replay must agree on semantic state"
    );
    assert_eq!(
        st_first.trace, ow1_first.trace,
        "ST and OW1 designated replay must agree on full observer-safe source→Core→artifact→trace correspondence"
    );
    assert_eq!(
        st_first.receipts, ow1_first.receipts,
        "ST and OW1 designated replay must return exact same receipts"
    );
    assert_eq!(
        st_first.m8_actual_digest, ow1_first.m8_actual_digest,
        "ST and OW1 designated replay must expose the same M8 semantic evidence"
    );
    assert_eq!(
        st_first.m8_backend_trace, ow1_first.m8_backend_trace,
        "ST and OW1 designated replay must expose the same M8-owned backend trace"
    );
    assert_eq!(
        st_first.m8_backend_latest_sequence, ow1_first.m8_backend_latest_sequence,
        "ST and OW1 designated replay must expose the same M8 backend sequence"
    );
    assert_eq!(
        st_first.cache, ow1_first.cache,
        "ST and OW1 designated replay must agree on cache binding state"
    );
    assert_eq!(
        st_first.publication, ow1_first.publication,
        "ST and OW1 designated replay must agree on publication binding state"
    );
    assert_eq!(
        st_first.artifact_identity, ow1_first.artifact_identity,
        "ST and OW1 designated replay must execute the same projected artifacts"
    );
}

#[test]
fn ow1_owner_dispatch_refreshes_exact_m8_context_observations_after_dequeue() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
    assert_eq!(
        program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible
    );
    let mut fabric = boot(&checked, program, BackendProfile::Ow1);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("OW1 owner request is staged");
    let request_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(request_envelope.envelope_id(), submitted.envelope_id());
    assert_eq!(request_envelope.edge_ref(), owner_request_edge.edge_ref());
    let transport = fabric
        .step_transport("A", "S", request_envelope.envelope_id())
        .expect("request transports to owner locus");
    let s_step = fabric
        .step_locus("S")
        .expect("OW1 worker serves after S dequeues the exact owner request");
    assert_eq!(
        s_step.consumed_envelope_id(),
        request_envelope.envelope_id()
    );
    assert_eq!(
        s_step.locus_dequeue_record_id(),
        transport.target_inbox_enqueue_record_id()
    );

    assert_owner_m8_context_observation(
        &fabric,
        s_step.m8_request_node_id(),
        &request_envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerEnqueued,
    );
    assert_owner_m8_context_observation(
        &fabric,
        s_step.m8_serve_node_id(),
        &request_envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerWrite,
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(s_step.m8_request_node_id())
            .contains(&s_step.locus_dequeue_occurrence_id().to_string()),
        "OW1 M8 request observation must causally depend on the S dequeue occurrence"
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(s_step.m8_serve_node_id())
            .contains(&s_step.m8_request_node_id().to_string()),
        "OW1 M8 serve observation must causally depend on the worker-owned request observation"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .owner_request_node_count("attack", "S"),
        1,
        "fabric devtools surface must contain the OW1 owner request node"
    );
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
}

#[test]
fn ow1_owner_declared_failure_after_dequeue_returns_m8_rejection_and_quarantines() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    assert_eq!(
        program.backend_eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible
    );
    let mut fabric = boot(&checked, program, BackendProfile::Ow1);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("OW1 owner request is staged");
    let request_envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(request_envelope.envelope_id(), submitted.envelope_id());
    fabric
        .step_transport("A", "S", request_envelope.envelope_id())
        .expect("request reaches S before backend rejection is armed");
    fabric
        .m8_backend_test_support_mut()
        .reject_next_owner_operation_after_dequeue(request_envelope.envelope_id(), "attack", "S")
        .expect(
            "OW1 must support a declared owner failure after dequeue inside the worker backend",
        );

    let before = fabric.semantic_snapshot();
    let rejected = assert_sys4_diag(
        fabric.step_locus("S"),
        Sys4DiagnosticKind::M8ExecutionRejected,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(request_envelope.envelope_id())
    );
    let dequeue_occurrence = rejected
        .endpoint_dequeue_occurrence_id()
        .expect("OW1 owner failure is post-dequeue at S");
    let backend_failure = rejected
        .backend_m8_failure_inspection()
        .expect("OW1 failure diagnostic must expose exact M8-owned rejection evidence");
    assert_eq!(rejected.m8_trace_node_id(), Some(backend_failure.node_id()));
    assert_owner_m8_context_observation(
        &fabric,
        backend_failure.node_id(),
        &request_envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerOperationRejected,
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(backend_failure.node_id())
            .contains(&dequeue_occurrence.to_string()),
        "OW1 owner rejection observation must causally depend on the S dequeue occurrence"
    );
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(request_envelope.envelope_id())
            .expect("post-dequeue OW1 owner failure is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::M8ExecutionRejected
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
fn retarget_fault_attempted_target_is_checked_and_audited_not_inert() {
    let checked = owner_endpoint_checked();
    let projection = owner_endpoint_projection(&checked);
    let owner_request_edge = projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("projection has owner request edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let first = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("first request carrier is submitted to A outbox");
    let second = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("second same-edge request carrier is submitted to A outbox");

    let invalid = assert_sys4_diag(
        fabric.dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                owner_request_edge.edge_ref(),
                first.envelope_id(),
                "MissingLocus",
            ),
        )),
        Sys4DiagnosticKind::UnknownRetargetLocus,
    );
    let invalid_retarget = invalid
        .retarget_fault_inspection()
        .expect("invalid retarget must expose typed attempted-target evidence");
    assert_eq!(invalid_retarget.edge_ref(), owner_request_edge.edge_ref());
    assert_eq!(invalid_retarget.envelope_id(), first.envelope_id());
    assert_eq!(invalid_retarget.attempted_target_locus(), "MissingLocus");
    assert!(invalid_retarget.rejected_at_fault_admission());
    assert!(invalid_retarget.target_enqueue_occurrence_id().is_none());

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::retarget_in_transit_envelope_for_edge(
                owner_request_edge.edge_ref(),
                first.envelope_id(),
                "A",
            ),
        ))
        .expect("valid but semantically wrong target is recorded against exact envelope");

    fabric
        .step_transport("A", "S", second.envelope_id())
        .expect("same-edge sibling remains unaffected by exact-envelope retarget");
    fabric
        .step_locus("S")
        .expect("unaffected sibling owner request still serves");
    let before_reject = fabric.semantic_snapshot();
    let valid_wrong_target = assert_sys4_diag(
        fabric.step_transport("A", "S", first.envelope_id()),
        Sys4DiagnosticKind::WrongTargetLocus,
    );
    let valid_retarget = valid_wrong_target
        .retarget_fault_inspection()
        .expect("wrong-target step failure must retain attempted-target evidence");
    assert_eq!(valid_retarget.edge_ref(), owner_request_edge.edge_ref());
    assert_eq!(valid_retarget.envelope_id(), first.envelope_id());
    assert_eq!(valid_retarget.attempted_target_locus(), "A");
    assert!(!valid_retarget.rejected_at_fault_admission());
    assert_ne!(
        invalid_retarget.evidence_id(),
        valid_retarget.evidence_id(),
        "distinct attempted targets must produce distinct typed evidence"
    );
    assert!(
        valid_wrong_target
            .endpoint_dequeue_occurrence_id()
            .is_none()
    );
    assert!(valid_retarget.target_enqueue_occurrence_id().is_none());
    assert!(fabric.semantic_snapshot().same_state(&before_reject));
}

#[test]
fn post_dequeue_owner_backend_rejection_quarantines_without_state_mutation_or_head_block() {
    let checked = owner_endpoint_checked();
    let program = fabric_program(owner_endpoint_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let submitted = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("owner request is staged");
    let envelope = fabric
        .locus_runtime("A")
        .expect("A exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .step_transport("A", "S", envelope.envelope_id())
        .expect("owner request reaches S inbox");
    fabric
        .m8_backend_test_support_mut()
        .reject_next_owner_operation_after_dequeue(envelope.envelope_id(), "attack", "S")
        .expect("owner failure is armed inside actual M8 backend test support");

    let before = fabric.semantic_snapshot();
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);
    let owner_validation_before = fabric
        .current_m9_authority_inspection()
        .owner_operation_validation_count("attack", "S", submitted.request_id());
    let rejected = assert_sys4_diag(
        fabric.step_locus("S"),
        Sys4DiagnosticKind::M8ExecutionRejected,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(envelope.envelope_id())
    );
    let owner_dequeue_occurrence = rejected
        .endpoint_dequeue_occurrence_id()
        .expect("owner backend rejection is post-dequeue at S");
    assert_eq!(
        fabric
            .current_m9_authority_inspection()
            .owner_operation_validation_count("attack", "S", submitted.request_id()),
        owner_validation_before + 1,
        "owner backend rejection must occur after live authority validation"
    );
    let backend_failure = rejected
        .backend_m8_failure_inspection()
        .expect("diagnostic must expose typed M8 owner failure evidence");
    assert_eq!(rejected.m8_trace_node_id(), Some(backend_failure.node_id()));
    assert_backend_owner_operation_rejection_observation(
        &fabric,
        backend_failure.node_id(),
        envelope.envelope_id(),
        "attack",
        "S",
        prior_m8_sequence,
    );
    assert_owner_m8_context_observation(
        &fabric,
        backend_failure.node_id(),
        &envelope,
        "attack",
        "S",
        M8LocalTraceKind::OwnerOperationRejected,
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(backend_failure.node_id())
            .contains(&owner_dequeue_occurrence.to_string()),
        "M8 owner rejection node must causally depend on the S locus dequeue occurrence"
    );
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(
        fabric
            .locus_runtime("S")
            .expect("S exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(envelope.envelope_id())
            .expect("owner rejection is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::M8ExecutionRejected
    );

    let clean = fabric
        .submit_source_action(owner_attack_action("attack"))
        .expect("later clean owner request is admitted");
    fabric
        .step_transport("A", "S", clean.envelope_id())
        .expect("later clean owner request transports");
    fabric
        .step_locus("S")
        .expect("later clean owner request is not head-blocked");
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
}

#[test]
fn post_dequeue_designated_evaluator_backend_rejection_quarantines_without_publication() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let submitted = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:evaluator-red"))
        .expect("designated publish request is staged");
    fabric
        .step_transport("E", "S", submitted.envelope_id())
        .expect("input request reaches S");
    let source_release_validation_before_s = fabric
        .current_m9_authority_inspection()
        .source_release_validation_count("E.result", "S", submitted.envelope_id());
    let source_step = fabric
        .step_locus("S")
        .expect("S emits input receipt before evaluator failure");
    let source_release_validation_after_s = fabric
        .current_m9_authority_inspection()
        .source_release_validation_count("E.result", "S", submitted.envelope_id());
    assert_eq!(
        source_release_validation_after_s,
        source_release_validation_before_s + 1,
        "the original E→S input request is source-release validated exactly once at S"
    );
    fabric
        .step_transport("S", "E", source_step.reply_envelope_id())
        .expect("input receipt reaches E");
    let input_receipt = fabric
        .locus_runtime("E")
        .expect("E exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    fabric
        .m8_backend_test_support_mut()
        .reject_next_designated_evaluation_after_input_receipt(
            input_receipt.envelope_id(),
            "E.result",
            "E",
            "tick:F:evaluator-red",
        )
        .expect("evaluation failure is armed inside actual M8 backend test support");

    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let publication_before = fabric.m8_designated_publication_snapshot("E.result");
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);
    let source_release_validation_before_e_failure = fabric
        .current_m9_authority_inspection()
        .source_release_validation_count("E.result", "S", submitted.envelope_id());
    let rejected = assert_sys4_diag(
        fabric.step_locus("E"),
        Sys4DiagnosticKind::M8ExecutionRejected,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(input_receipt.envelope_id())
    );
    assert_eq!(
        fabric
            .current_m9_authority_inspection()
            .source_release_validation_count("E.result", "S", submitted.envelope_id()),
        source_release_validation_before_e_failure,
        "E-side evaluator backend rejection must not perform a second source-release validation"
    );
    let evaluator_dequeue_occurrence = rejected
        .endpoint_dequeue_occurrence_id()
        .expect("designated evaluator backend rejection is post-dequeue at E");
    let backend_failure = rejected
        .backend_m8_failure_inspection()
        .expect("diagnostic must expose typed M8 evaluation failure evidence");
    assert_eq!(rejected.m8_trace_node_id(), Some(backend_failure.node_id()));
    assert_backend_designated_evaluation_rejection_observation(
        &fabric,
        backend_failure.node_id(),
        input_receipt.envelope_id(),
        "E.result",
        "E",
        "tick:F:evaluator-red",
        prior_m8_sequence,
    );
    assert!(
        fabric
            .causality()
            .predecessor_ids(backend_failure.node_id())
            .contains(&evaluator_dequeue_occurrence.to_string()),
        "M8 evaluation rejection node must causally depend on the E receipt locus-dequeue occurrence"
    );
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric.m8_designated_publication_snapshot("E.result"),
        publication_before,
        "failed evaluator backend attempt must not publish a value"
    );
    assert!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .is_empty(),
        "failed evaluator backend attempt must not enqueue a result delivery"
    );
    assert_eq!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(input_receipt.envelope_id())
            .expect("evaluator rejection is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::M8ExecutionRejected
    );

    stage_designated_publish_until_delivery_outbox(&mut fabric);
    assert_eq!(
        fabric
            .locus_runtime("E")
            .expect("E exists")
            .outgoing_mailbox()
            .pending_envelopes()
            .len(),
        1,
        "later clean evaluator request is not head-blocked by quarantined input receipt"
    );
}

#[test]
fn fixed_version_second_evaluation_has_distinct_m8_idempotent_occurrence_without_rewriting_a() {
    let checked = designated_checked();
    let program = fabric_program(designated_projection(&checked));
    let mut fabric = boot(&checked, program, BackendProfile::St);

    let first_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:41"))
        .expect("A publish creates E->S request");
    fabric
        .step_transport("E", "S", first_submit.envelope_id())
        .expect("A request transports");
    let first_source = fabric.step_locus("S").expect("S emits A input receipt");
    fabric
        .step_transport("S", "E", first_source.reply_envelope_id())
        .expect("A input receipt transports");
    let first_eval = fabric.step_locus("E").expect("E publishes A");
    let first_node_id = first_eval.m8_evaluation_node_id();
    let first_observation = m8_owned_observation(&fabric, first_node_id);
    assert_eq!(
        first_observation.kind(),
        M8LocalTraceKind::DesignatedValuePublished
    );
    let first_context = first_observation.designated_context_digest().to_string();
    let first_predecessors = first_observation.predecessor_ids().to_vec();
    let prior_m8_sequence = m8_backend_latest_sequence(&fabric);

    let second_submit = fabric
        .submit_source_action(publish_designated_action_with_tick("tick:F:42"))
        .expect("B publish creates E->S request while A delivery remains pending");
    fabric
        .step_transport("E", "S", second_submit.envelope_id())
        .expect("B request transports");
    let second_source = fabric.step_locus("S").expect("S emits B input receipt");
    fabric
        .step_transport("S", "E", second_source.reply_envelope_id())
        .expect("B input receipt transports");
    let second_eval = fabric
        .step_locus("E")
        .expect("E observes fixed-version idempotent B evaluation and emits B delivery");
    let idempotent_node_id = second_eval.m8_evaluation_node_id();
    assert_ne!(
        idempotent_node_id, first_node_id,
        "fixed-version idempotent B evaluation must not reuse A's M8 publication node"
    );
    assert_backend_designated_idempotent_observation(
        &fabric,
        idempotent_node_id,
        second_eval.consumed_envelope_id(),
        "E.result",
        "E",
        "tick:F:42",
        prior_m8_sequence,
    );

    let first_after = m8_owned_observation(&fabric, first_node_id);
    assert_eq!(first_after.designated_context_digest(), first_context);
    assert_eq!(first_after.predecessor_ids(), first_predecessors);
    let second_delivery = fabric
        .locus_runtime("E")
        .expect("E exists")
        .outgoing_mailbox()
        .pending_envelopes()
        .for_request(second_eval.request_id());
    assert_eq!(
        second_delivery.m8_evaluation_node_id(),
        idempotent_node_id,
        "B delivery must depend on B's idempotent M8 occurrence, not A's publication node"
    );
    assert_eq!(second_delivery.logical_tick_id(), "tick:F:42");
    assert_eq!(second_delivery.edge_ref(), first_after.edge_ref());
}

#[test]
fn cache_retry_projection_mismatch_rejects_before_m9_m8_or_payload_without_cache_mutation() {
    let checked = designated_checked();
    let projection = designated_projection(&checked);
    let delivery_edge = projection
        .communication_plan()
        .single_edge(
            "E.result",
            CommunicationEdgeKind::DesignatedResultDelivery,
            "E",
            "C",
        )
        .expect("projection has delivery edge");
    let program = fabric_program(projection);
    let mut fabric = boot(&checked, program, BackendProfile::St);

    fabric
        .dispatch_source_action(publish_designated_action())
        .expect("publish succeeds");
    let first = fabric
        .dispatch_source_action(consume_designated_action())
        .expect("first consume installs cache");
    let semantic_identity = first.semantic_consumption_identity().to_string();
    let retry_submission = fabric
        .submit_source_action(consume_designated_action())
        .expect("retry creates a local cache-retry envelope");
    let retry_envelope = fabric
        .locus_runtime("C")
        .expect("C exists")
        .incoming_mailbox()
        .pending_envelopes()
        .single();
    assert_eq!(retry_envelope.envelope_id(), retry_submission.envelope_id());
    assert!(is_local_cache_retry(&retry_envelope));

    let before = fabric.semantic_snapshot();
    let cache_before = fabric.designated_cache_snapshot();
    let m9_before = m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity);
    let m8_before = m8_backend_trace_count(
        &fabric,
        M8LocalTraceKind::DesignatedCacheValidated,
        &semantic_identity,
        "C",
    );
    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::rewrite_local_cache_retry_projection_binding_for_edge(
                retry_envelope.envelope_id(),
                delivery_edge.edge_ref(),
                "forged-core-ref",
                "forged-policy-stamp",
                "forged-redaction",
            ),
        ))
        .expect("cache retry projection-mismatch fault is bound to exact local envelope");

    let rejected = assert_sys4_diag(
        fabric.step_locus("C"),
        Sys4DiagnosticKind::CacheProjectionMismatch,
    );
    assert_eq!(
        rejected.rejected_envelope_id(),
        Some(retry_envelope.envelope_id())
    );
    let mismatch = rejected
        .cache_projection_mismatch_inspection()
        .expect("diagnostic exposes projection-derived mismatch evidence");
    assert_eq!(mismatch.envelope_id(), retry_envelope.envelope_id());
    assert_eq!(mismatch.expected_edge_ref(), delivery_edge.edge_ref());
    assert_eq!(mismatch.expected_source_ref(), delivery_edge.source_ref());
    assert_eq!(mismatch.expected_core_ref(), delivery_edge.core_ref());
    assert_ne!(mismatch.carrier_core_ref(), mismatch.expected_core_ref());
    assert!(mismatch.rejected_before_m9_validation());
    assert!(mismatch.rejected_before_m8_validation());
    assert!(!rejected.exposes_raw_payload());
    assert_eq!(
        m9_validation_occurrence_count(&fabric, "E.result", "C", &semantic_identity),
        m9_before
    );
    assert_eq!(
        m8_backend_trace_count(
            &fabric,
            M8LocalTraceKind::DesignatedCacheValidated,
            &semantic_identity,
            "C",
        ),
        m8_before
    );
    assert!(fabric.semantic_snapshot().same_state(&before));
    assert_eq!(fabric.designated_cache_snapshot(), cache_before);
    assert_eq!(
        fabric
            .locus_runtime("C")
            .expect("C exists")
            .incoming_mailbox()
            .terminal_rejected_envelope(retry_envelope.envelope_id())
            .expect("projection-mismatched retry is terminally quarantined")
            .diagnostic_kind(),
        Sys4DiagnosticKind::CacheProjectionMismatch
    );
}

#[test]
fn release_m8_context_path_must_not_drop_or_cfg_gate_carrier_provenance() {
    let sys4 = read_runtime_src("sys4_dispatch.rs");
    let m8 = read_runtime_src("m8_runtime_local_cut.rs");
    let compact_sys4 = normalize_source_for_boundary_scan(&sys4);
    let compact_m8 = normalize_source_for_boundary_scan(&m8);

    let mut violations: Vec<String> = Vec::new();
    if compact_sys4.contains("let_=&context;") {
        violations.push(
            "release SYS-4→M8 paths discard carrier/source/Core/edge context with `let _ = &context;`"
                .to_string(),
        );
    }
    if compact_sys4.contains("drop(context);") {
        violations.push(
            "release SYS-4→M8 paths must not silence carrier/source/Core/edge context with `drop(context)`"
                .to_string(),
        );
    }

    for (signature_marker, label) in [
        (
            "fn enqueue_and_serve(",
            "M8ExecutionBackend::enqueue_and_serve",
        ),
        (
            "fn evaluate_designated(",
            "M8ExecutionBackend::evaluate_designated",
        ),
        (
            "fn consume_designated(",
            "M8ExecutionBackend::consume_designated",
        ),
    ] {
        let Some(body) = extract_balanced_fn_body(&sys4, signature_marker) else {
            violations.push(format!(
                "{label} body must exist for release-path boundary scan"
            ));
            continue;
        };
        let compact_body = normalize_source_for_boundary_scan(&body);
        if body.contains("#[cfg(test)]") || body.contains("#[cfg(not(test))]") {
            violations.push(format!(
                "{label} must not route context-bearing M8 execution through cfg(test)/cfg(not(test)) branches"
            ));
        }
        if compact_body.contains("let_=&context;") {
            violations.push(format!(
                "{label} must not discard carrier/source/Core/edge context with `let _ = &context;`"
            ));
        }
        if compact_body.contains("drop(context);") {
            violations.push(format!(
                "{label} must not silence carrier/source/Core/edge context with `drop(context)`"
            ));
        }
        if compact_body.contains(".latest_observation(")
            || compact_body.contains(".latest_observation_any(")
            || compact_body.contains("latest_trace_node_id(")
        {
            violations.push(format!(
                "{label} must propagate the observation returned by its M8 context entrypoint instead of recovering a latest trace row"
            ));
        }
    }

    for (request_type, backend_method, label) in [
        ("M8OwnerRequest", "enqueue_and_serve", "owner execution"),
        (
            "M8DesignatedEvaluationRequest",
            "evaluate_designated",
            "designated evaluation",
        ),
        (
            "M8ConsumeRequest",
            "consume_designated",
            "designated consumption",
        ),
    ] {
        let Some(entrypoint) = find_unconditional_m8_observed_context_entrypoint(&m8, request_type)
        else {
            violations.push(format!(
                "M8 must expose an unconditional {label} function accepting {request_type} plus M8LocalDesignatedTraceContext and returning M8LocalTraceObservation"
            ));
            continue;
        };
        let backend_body = extract_balanced_fn_body(&sys4, &format!("fn {backend_method}("))
            .expect("scoped backend method exists");
        if !body_calls_context_entrypoint(&backend_body, &entrypoint) {
            violations.push(format!(
                "M8ExecutionBackend::{backend_method} must pass context to the selected M8-owned observation entrypoint {entrypoint}"
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "release M8 carrier provenance context must be carried by non-test M8-owned owner/evaluator APIs:\n{}",
        violations.join("\n")
    );
    assert!(
        compact_m8.contains("M8LocalDesignatedTraceContext"),
        "M8 remains the owner of the carrier provenance context type used for runtime trace rows"
    );
}

#[test]
fn post_dequeue_m8_outcome_ids_must_be_exact_not_latest_trace_lookup() {
    let sys4 = read_runtime_src("sys4_dispatch.rs");
    let compact_sys4 = normalize_source_for_boundary_scan(&sys4);

    let mut violations = Vec::new();
    if compact_sys4.contains("latest_trace_node_id(") {
        violations.push(
            "post-dequeue SYS-4 handling must not recover M8 node IDs through latest_trace_node_id after an envelope has been dequeued".to_string(),
        );
    }
    let banned_latest_recovery = [
        ".latest_observation(M8LocalTraceKind::OwnerOperationRejected)",
        ".latest_observation(M8LocalTraceKind::DesignatedConsumptionRejected)",
        ".latest_observation(M8LocalTraceKind::DesignatedEvaluationRejected)",
    ];
    for needle in banned_latest_recovery {
        if compact_sys4.contains(needle) {
            violations.push(format!(
                "post-dequeue SYS-4 handling must use exact M8-owned typed outcome node IDs, not recover '{needle}' by latest-trace lookup after the envelope has been dequeued"
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "post-dequeue SYS-4 must use exact M8-owned outcome observations instead of lookup recovery:\n{}",
        violations.join("\n")
    );
}

#[test]
fn ow1_owner_backend_branch_must_preserve_context_and_return_serve_observation() {
    let sys4 = read_runtime_src("sys4_dispatch.rs");
    let enqueue_body = extract_balanced_fn_body(&sys4, "fn enqueue_and_serve(")
        .expect("M8ExecutionBackend::enqueue_and_serve body exists");
    let ow1_arm = extract_balanced_block_after_marker(&enqueue_body, "Self::Ow1(")
        .expect("M8ExecutionBackend::enqueue_and_serve has an OW1 branch");
    let compact_ow1 = normalize_source_for_boundary_scan(&ow1_arm);

    assert!(
        compact_ow1.contains("context"),
        "OW1 owner execution branch must pass the exact dequeued M8LocalDesignatedTraceContext into worker-owned command/execution"
    );
    assert!(
        !compact_ow1.contains("serve_observation:None"),
        "OW1 owner execution branch must return an exact M8-owned serve observation, not synthesize serve_observation: None"
    );
}

#[test]
fn m8_runtime_local_cut_must_not_depend_on_sys4_dispatch_module() {
    let m8 = read_runtime_src("m8_runtime_local_cut.rs");
    let banned = [
        "crate::sys4_dispatch",
        "super::sys4_dispatch",
        "sys4_dispatch::",
    ];

    for needle in banned {
        assert!(
            !m8.contains(needle),
            "M8 runtime module must not depend on SYS-4 dispatch types or compatibility shims: found {needle}"
        );
    }
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
        "#![allow(",
        "#[allow(dead_code)]",
        "cfg(any())",
        "retired_direct",
        "retired_history",
        "retired_seed",
        "source_core_fragment_edge_provenance: bool",
        "source_core_artifact_bound: bool",
        "actual_m8_trace_observed: bool",
        "has_source_core_fragment_edge_provenance",
        "all_reads_and_writes_have_source_core_provenance",
        "all_entries_have_source_core_fragment_and_edge_provenance",
        "crossed_endpoint_boundary(&self, _source: &str, _target: &str)",
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

fn read_runtime_src(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "runtime source file must be readable for structural SYS-4 RED {}: {error}",
            path.display()
        )
    })
}

fn normalize_source_for_boundary_scan(source: &str) -> String {
    source.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn extract_balanced_fn_body(source: &str, signature_marker: &str) -> Option<String> {
    let signature_start = source.find(signature_marker)?;
    let open_brace = signature_start + source[signature_start..].find('{')?;
    let mut depth = 0usize;
    for (relative_index, ch) in source[open_brace..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let close_brace = open_brace + relative_index;
                    return Some(source[open_brace..=close_brace].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_balanced_block_after_marker(source: &str, marker: &str) -> Option<String> {
    let marker_start = source.find(marker)?;
    let open_brace = marker_start + source[marker_start..].find('{')?;
    extract_balanced_body_from_open_brace(source, open_brace)
}

fn find_unconditional_m8_observed_context_entrypoint(
    source: &str,
    request_type: &str,
) -> Option<String> {
    let mut search_from = 0usize;
    while let Some(relative_fn) = source[search_from..].find("fn ") {
        let fn_start = search_from + relative_fn;
        let open_relative = source[fn_start..].find('{')?;
        let signature = &source[fn_start..fn_start + open_relative];
        if signature.contains(request_type)
            && signature.contains("M8LocalDesignatedTraceContext")
            && signature.contains("M8LocalTraceObservation")
            && !has_attached_test_cfg_gate(source, fn_start)
            && function_body_uses_context(source, fn_start)
        {
            let name_start = fn_start + "fn ".len();
            let name_end = source[name_start..]
                .find('(')
                .map(|offset| name_start + offset)?;
            return Some(source[name_start..name_end].trim().to_string());
        }
        search_from = fn_start + "fn ".len();
    }
    None
}

fn body_calls_context_entrypoint(body: &str, entrypoint: &str) -> bool {
    let compact = normalize_source_for_boundary_scan(body);
    let marker = format!(".{entrypoint}(");
    let Some(call_start) = compact.find(&marker) else {
        return false;
    };
    let open = call_start + marker.len() - 1;
    let mut depth = 0usize;
    for (relative, character) in compact[open..].char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                let Some(next_depth) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next_depth;
                if depth == 0 {
                    return contains_identifier(&compact[open..=open + relative], "context");
                }
            }
            _ => {}
        }
    }
    false
}

fn contains_identifier(source: &str, identifier: &str) -> bool {
    source.match_indices(identifier).any(|(start, _)| {
        let before = source[..start].chars().next_back();
        let after = source[start + identifier.len()..].chars().next();
        !before.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
            && !after.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
    })
}

fn has_attached_test_cfg_gate(source: &str, item_start: usize) -> bool {
    for line in source[..item_start].lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("#[") {
            if trimmed.contains("cfg(test)") || trimmed.contains("cfg(not(test))") {
                return true;
            }
            continue;
        }
        break;
    }
    false
}

fn function_body_uses_context(source: &str, fn_start: usize) -> bool {
    let Some(open_relative) = source[fn_start..].find('{') else {
        return false;
    };
    let open_brace = fn_start + open_relative;
    let Some(body) = extract_balanced_body_from_open_brace(source, open_brace) else {
        return false;
    };
    let compact_body = normalize_source_for_boundary_scan(&body);
    compact_body.contains("context")
        && !compact_body.contains("#[cfg(test)]")
        && !compact_body.contains("#[cfg(not(test))]")
        && !compact_body.contains("let_=&context;")
        && !compact_body.contains("drop(context);")
}

fn extract_balanced_body_from_open_brace(source: &str, open_brace: usize) -> Option<String> {
    let mut depth = 0usize;
    for (relative_index, ch) in source[open_brace..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let close_brace = open_brace + relative_index;
                    return Some(source[open_brace..=close_brace].to_string());
                }
            }
            _ => {}
        }
    }
    None
}
