use std::{collections::BTreeSet, ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::m8_runtime_admission::{M8Runtime, M8RuntimeAdmission, M8RuntimeInstance};
use mir_runtime::m8_runtime_authority::{
    M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
};
use mir_runtime::m8_runtime_owner_queue::{
    M8AuthorityUse, M8DeclaredFailure, M8EnqueueDiagnosticKind, M8ExecutionSeed, M8OwnerRequest,
    M8QueueTrace, M8QueueTraceEntry, M8QueueTraceKind, M8RuntimeExecution, M8ServeDiagnosticKind,
    M8ServeOutcome, M8StateKey,
};
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        M7DiagnosticKind, SurfaceV0PipelineDiagnostics, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:epoch1";
const ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:epoch1";
const ATTACK_WITNESS_REF: &str = "witness:attack:S:self:epoch1";
const REVOKED_ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:revoked";
const STALE_ATTACK_WITNESS_REF: &str = "witness:attack:S:self:stale";
const UNSEEDED_ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:epoch2";
const UNSEEDED_ATTACK_WITNESS_REF: &str = "witness:attack:S:self:epoch2";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn checked_runtime_instance(name: &str) -> (String, String, M8RuntimeInstance) {
    let (path, source) = load_surface_fixture(name);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("surface-v0 fixture checks through M7 before M8 runtime admission");
    let admission = M8RuntimeAdmission::new(checked.program_identity().clone());
    let runtime_instance = M8Runtime::default()
        .admit(checked, admission)
        .expect("residual-free owner fixture admits through M8 Phase 1");
    (path, source, runtime_instance)
}

fn checked_error(name: &str) -> SurfaceV0PipelineDiagnostics {
    let (path, source) = load_surface_fixture(name);
    check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect_err("fixture is rejected before M8 admission")
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1_u32;
    let mut column = 1_u32;
    for byte in source[..byte_offset].bytes() {
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
    let range = byte_range(source, lexeme);
    let (start_line, start_column) = line_column(source, range.start);
    let (end_line, end_column) = line_column(source, range.end);
    SourceRef::new(
        path.to_owned(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}

fn attack_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "player[target].hp = player[target].hp - player[self].atk",
    )
}

fn hp_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "target", "hp")
}

fn atk_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "self", "atk")
}

fn owner_authority_state() -> M8AuthorityState {
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted(OWNER_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus("S")
                .with_epoch("epoch1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(ATTACK_CAPABILITY_REF)
                .for_owner_evaluation("attack")
                .with_owner_locus("S")
                .with_principal("self")
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("epoch1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(ATTACK_WITNESS_REF)
                .for_capability(ATTACK_CAPABILITY_REF)
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("epoch1"),
        )
}

fn valid_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ATTACK_CAPABILITY_REF)
        .with_witness_ref(ATTACK_WITNESS_REF)
}

fn missing_capability_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(REVOKED_ATTACK_CAPABILITY_REF)
        .with_witness_ref(ATTACK_WITNESS_REF)
}

fn stale_witness_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ATTACK_CAPABILITY_REF)
        .with_witness_ref(STALE_ATTACK_WITNESS_REF)
}

fn matching_unseeded_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(UNSEEDED_ATTACK_CAPABILITY_REF)
        .with_witness_ref(UNSEEDED_ATTACK_WITNESS_REF)
}

fn execution(instance: M8RuntimeInstance) -> M8RuntimeExecution {
    instance.into_execution(
        M8ExecutionSeed::new()
            .with_int(hp_key(), 100)
            .with_int(atk_key(), 10)
            .with_authority_state(owner_authority_state()),
    )
}

fn attack_request(authority_use: M8AuthorityUse) -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(authority_use)
}

fn assert_owner_success(outcome: &M8ServeOutcome, before: i64, after: i64, source_ref: &SourceRef) {
    assert_eq!(outcome.evaluation(), "attack");
    assert_eq!(outcome.owner_locus(), "S");
    assert_eq!(outcome.failure(), None);
    assert_eq!(outcome.read_int(&hp_key()), Some(before));
    assert_eq!(outcome.read_int(&atk_key()), Some(10));
    assert_eq!(outcome.written_int(&hp_key()), Some(after));
    assert_eq!(outcome.source_ref(), source_ref);
}

fn dependency_ids(entry: &M8QueueTraceEntry) -> Vec<&str> {
    entry
        .dependencies()
        .iter()
        .map(|dependency| dependency.as_str())
        .collect()
}

fn trace_entry_for<'a>(
    trace: &'a M8QueueTrace,
    kind: M8QueueTraceKind,
    request_occurrence_id: &str,
) -> &'a M8QueueTraceEntry {
    trace
        .entries()
        .iter()
        .find(|entry| {
            entry.kind() == kind && entry.request_occurrence_id() == Some(request_occurrence_id)
        })
        .unwrap_or_else(|| panic!("missing {kind:?} entry for {request_occurrence_id}"))
}

fn assert_monotone_trace_dag(trace: &M8QueueTrace) {
    let mut seen_node_ids = BTreeSet::new();
    let mut previous_index = None;

    for entry in trace.entries() {
        if let Some(previous_index) = previous_index {
            assert!(
                previous_index < entry.trace_node_index(),
                "trace node indexes must advance monotonically"
            );
        }
        previous_index = Some(entry.trace_node_index());

        assert_ne!(
            Some(entry.trace_node_id()),
            entry.request_occurrence_id(),
            "trace node id must not alias request occurrence id"
        );
        assert!(
            !dependency_ids(entry).contains(&entry.trace_node_id()),
            "trace node must not depend on itself"
        );
        for dependency in dependency_ids(entry) {
            assert!(
                seen_node_ids.contains(dependency),
                "dependency {dependency} must name an earlier trace node"
            );
        }
        assert!(
            seen_node_ids.insert(entry.trace_node_id().to_owned()),
            "trace node ids must be unique"
        );
    }
}

#[test]
fn owner_fifo_uses_service_time_state_not_enqueue_time_snapshot() {
    let (path, source, instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let source_ref = attack_source_ref(&path, &source);
    let mut execution = execution(instance);

    let first_occurrence = execution.enqueue(attack_request(valid_authority_use()));
    let second_occurrence = execution.enqueue(attack_request(valid_authority_use()));
    assert_ne!(first_occurrence.id(), second_occurrence.id());
    assert_eq!(
        execution.owner_queue("S").occurrence_ids(),
        vec![first_occurrence.id(), second_occurrence.id()]
    );
    assert_eq!(execution.snapshot().int(&hp_key()), Some(100));

    let first = execution
        .serve_next_owner("S")
        .expect("first queued owner request serves");
    assert_owner_success(&first, 100, 90, &source_ref);
    assert_eq!(execution.snapshot().int(&hp_key()), Some(90));

    let second = execution
        .serve_next_owner("S")
        .expect("second queued owner request serves after first write");
    assert_owner_success(&second, 90, 80, &source_ref);
    assert_eq!(execution.snapshot().int(&hp_key()), Some(80));
    assert_ne!(
        execution
            .trace()
            .entries()
            .iter()
            .map(|entry| entry.written_int(&hp_key()))
            .collect::<Vec<_>>(),
        vec![Some(90), Some(90)]
    );
}

#[test]
fn owner_request_authority_is_validated_at_serve_and_failure_does_not_mutate_snapshot() {
    let (path, source, instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let source_ref = attack_source_ref(&path, &source);
    let mut execution = execution(instance);
    let before = execution.snapshot();

    for (authority_use, expected_failure) in [
        (
            missing_capability_authority_use(),
            M8DeclaredFailure::MissingCapability,
        ),
        (
            stale_witness_authority_use(),
            M8DeclaredFailure::MissingWitness,
        ),
        (
            matching_unseeded_authority_use(),
            M8DeclaredFailure::MissingCapability,
        ),
    ] {
        execution.enqueue(attack_request(authority_use));
        let diagnostics = execution
            .serve_next_owner("S")
            .expect_err("authority references not present in the admitted state reject");
        assert_eq!(
            diagnostics.primary().kind(),
            M8ServeDiagnosticKind::DeclaredFailure(expected_failure)
        );
        assert_eq!(diagnostics.primary().source_ref(), &source_ref);
        assert_eq!(execution.snapshot(), before);
    }

    for expected_failure in [
        M8DeclaredFailure::MissingCapability,
        M8DeclaredFailure::MissingWitness,
    ] {
        assert!(
            execution
                .trace()
                .entries()
                .iter()
                .any(|entry| entry.failure() == Some(expected_failure)),
            "{expected_failure:?} failure trace is present"
        );
    }
    assert!(
        execution
            .trace()
            .entries()
            .iter()
            .any(|entry| entry.authority().capability_ref()
                == Some(UNSEEDED_ATTACK_CAPABILITY_REF)
                && entry.failure() == Some(M8DeclaredFailure::MissingCapability))
    );
}

#[test]
fn cross_owner_without_receipt_stops_before_checked_artifact_and_hidden_runtime_request() {
    let diagnostics = checked_error("cross_owner_without_receipt.mir");
    assert_eq!(
        diagnostics.primary().kind(),
        M7DiagnosticKind::CrossOwnerOperandRequiresReceipt
    );
    assert!(!diagnostics.has_executable_core());

    assert!(
        M8RuntimeExecution::from_rejected_m7(diagnostics).is_err(),
        "M8 owner queue must not fabricate a checked artifact, request, or receipt path from an M7 cross-owner diagnostic"
    );
}

#[test]
fn mixed_valid_and_invalid_requests_emit_source_bound_occurrence_dependency_authority_failure_trace()
 {
    let (path, source, instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let source_ref = attack_source_ref(&path, &source);
    let mut execution = execution(instance);

    let valid = execution.enqueue(attack_request(valid_authority_use()));
    let invalid = execution.enqueue(attack_request(missing_capability_authority_use()));
    execution
        .serve_next_owner("S")
        .expect("valid request is served first");
    execution
        .serve_next_owner("S")
        .expect_err("invalid request records typed failure");

    let trace = execution.trace();
    assert_eq!(
        trace.kinds(),
        vec![
            M8QueueTraceKind::Enqueued,
            M8QueueTraceKind::Enqueued,
            M8QueueTraceKind::AuthorityValidated,
            M8QueueTraceKind::OwnerRead,
            M8QueueTraceKind::OwnerWrite,
            M8QueueTraceKind::DeclaredFailure,
        ]
    );
    let served = trace
        .entries()
        .iter()
        .find(|entry| entry.kind() == M8QueueTraceKind::OwnerWrite)
        .expect("served trace entry is present");
    let valid_request = trace_entry_for(trace, M8QueueTraceKind::Enqueued, valid.id());
    let invalid_request = trace_entry_for(trace, M8QueueTraceKind::Enqueued, invalid.id());
    let validation = trace_entry_for(trace, M8QueueTraceKind::AuthorityValidated, valid.id());
    let read = trace_entry_for(trace, M8QueueTraceKind::OwnerRead, valid.id());
    let write = trace_entry_for(trace, M8QueueTraceKind::OwnerWrite, valid.id());
    let failure = trace_entry_for(trace, M8QueueTraceKind::DeclaredFailure, invalid.id());

    assert_eq!(valid_request.request_occurrence_id(), Some(valid.id()));
    assert_eq!(invalid_request.request_occurrence_id(), Some(invalid.id()));
    assert!(dependency_ids(valid_request).is_empty());
    assert!(dependency_ids(invalid_request).is_empty());
    assert_eq!(
        dependency_ids(validation),
        vec![valid_request.trace_node_id()]
    );
    assert_eq!(dependency_ids(read), vec![validation.trace_node_id()]);
    assert_eq!(dependency_ids(write), vec![read.trace_node_id()]);
    assert_eq!(
        dependency_ids(failure),
        vec![invalid_request.trace_node_id()]
    );
    assert_monotone_trace_dag(trace);

    assert_eq!(served.request_occurrence_id(), Some(valid.id()));
    assert_eq!(served.source_ref(), &source_ref);
    assert_eq!(served.authority().principal(), "self");
    assert_eq!(
        served.authority().capability_ref(),
        Some(ATTACK_CAPABILITY_REF)
    );
    assert_eq!(served.authority().witness_ref(), Some(ATTACK_WITNESS_REF));

    assert_eq!(failure.source_ref(), &source_ref);
    assert_eq!(
        failure.failure(),
        Some(M8DeclaredFailure::MissingCapability)
    );
    assert_eq!(failure.authority().principal(), "self");
}

#[test]
fn unknown_evaluation_enqueue_is_typed_rejection_without_panic_or_semantic_mutation() {
    let (_, _, instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let mut execution = execution(instance);
    let before_snapshot = execution.snapshot();
    let before_queue = execution.owner_queue("S").occurrence_ids();
    let before_trace_len = execution.trace().entries().len();

    let diagnostics = execution
        .try_enqueue(M8OwnerRequest::new("not_admitted").with_authority_use(valid_authority_use()))
        .expect_err("unknown evaluation is rejected as typed enqueue diagnostics");

    assert_eq!(
        diagnostics.primary().kind(),
        M8EnqueueDiagnosticKind::UnknownEvaluation
    );
    assert_eq!(diagnostics.primary().evaluation(), "not_admitted");
    assert_eq!(execution.snapshot(), before_snapshot);
    assert_eq!(execution.owner_queue("S").occurrence_ids(), before_queue);

    let new_trace = &execution.trace().entries()[before_trace_len..];
    assert_eq!(new_trace.len(), 1);
    let rejection = &new_trace[0];
    assert_eq!(rejection.kind(), M8QueueTraceKind::TypedEnqueueRejected);
    assert_eq!(rejection.request_occurrence_id(), None);
    assert_eq!(
        rejection.enqueue_diagnostic_kind(),
        Some(M8EnqueueDiagnosticKind::UnknownEvaluation)
    );
    assert!(dependency_ids(rejection).is_empty());
    assert_monotone_trace_dag(execution.trace());
}

#[test]
fn target_presence_retirement_rejects_attack_enqueue_without_runtime_mutation() {
    let (_, _, instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let mut execution = execution(instance);

    execution.retire_entity_presence("player", "target");
    let before_rejected_enqueue = execution.clone();

    let diagnostics = execution
        .try_enqueue(attack_request(valid_authority_use()))
        .expect_err("attack(target) must reject before enqueue when target presence is retired");

    assert_eq!(
        diagnostics.primary().kind(),
        M8EnqueueDiagnosticKind::StaleMembership
    );
    assert_eq!(
        execution, before_rejected_enqueue,
        "stale target rejection must not allocate an occurrence, enqueue a request, append trace rows, or advance owner counters"
    );
}

#[test]
fn replay_of_same_checked_artifact_seed_and_request_log_is_exactly_deterministic() {
    let (_, _, first_instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let (_, _, second_instance) = checked_runtime_instance("m7_owner_only_no_residuals.mir");
    let seed = M8ExecutionSeed::new()
        .with_int(hp_key(), 100)
        .with_int(atk_key(), 10)
        .with_authority_state(owner_authority_state());
    let request_log = vec![
        attack_request(valid_authority_use()),
        attack_request(valid_authority_use()),
        attack_request(missing_capability_authority_use()),
    ];

    let first = first_instance
        .into_execution(seed.clone())
        .run_replay(request_log.clone());
    let second = second_instance.into_execution(seed).run_replay(request_log);

    assert_eq!(first.outcomes(), second.outcomes());
    assert_eq!(first.snapshot(), second.snapshot());
    assert_eq!(first.trace(), second.trace());
    assert_eq!(first.snapshot().int(&hp_key()), Some(80));
    assert_eq!(
        first
            .outcomes()
            .iter()
            .map(|outcome| outcome.failure())
            .collect::<Vec<_>>(),
        vec![None, None, Some(M8DeclaredFailure::MissingCapability)]
    );
}
