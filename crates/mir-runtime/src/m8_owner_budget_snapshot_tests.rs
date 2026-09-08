use std::collections::BTreeSet;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{
    CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
};
use serde_json::Value;

use super::{
    M8AdmissionDiagnosticKind, M8I3PrivateSnapshot, M8I3PrivateSnapshotError, M8Runtime,
    M8RuntimeAdmission, prepare_deferred_m9_base,
};
use crate::m8_runtime_local_cut::{
    M8LocalDesignatedTraceContext, M8LocalRuntime, M8LocalRuntimeSeed,
};
use crate::m8_runtime_owner_queue::{M8EnqueueDiagnosticKind, M8ExecutionSeed, M8OwnerRequest};
use crate::semantic_runtime_kernel::LocusRef;
use crate::sys2_execution_backend::{Ow1ContextualM8Execution, Ow1WorkerBackend};

const BUDGETED_SOURCE_PATH: &str =
    "tests/inline/i3_owner_admission_budget_m8_private_snapshot_budgeted.mir";
const UNANNOTATED_SOURCE_PATH: &str =
    "tests/inline/i3_owner_admission_budget_m8_private_snapshot_unannotated.mir";
const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");

fn owner_budget_source(include_clause: bool) -> String {
    let clause = if include_clause {
        " within owner_ticks 1"
    } else {
        ""
    };
    format!(
        "module Combat.I3.OwnerAdmissionBudgetPrivateSnapshot

locus S
principal self
principal target
type Player

state player[id: Player] at S {{
  hp: Int
  atk: Int
}}

Role[self] at S {{
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired){clause} {{
    at S {{
      player[target].hp = player[target].hp - player[self].atk
    }}
  }}
}}
"
    )
}

fn checked_owner_source(path: &str, include_clause: bool) -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        path,
        owner_budget_source(include_clause),
    ))
    .expect("ordinary budgeted owner source checks before genuine M8 admission")
}

fn checked_provider_effect_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the ordinary provider-effect source checks before the private deferred-M9 guard")
}

fn admitted_owner_instance(path: &str, include_clause: bool) -> super::M8RuntimeInstance {
    let checked = checked_owner_source(path, include_clause);
    M8Runtime::default()
        .admit(
            checked.clone(),
            M8RuntimeAdmission::new(checked.program_identity().clone()),
        )
        .expect("residual-free ordinary owner source admits through genuine M8 admission")
}

fn owner_plan_budget(
    instance: &super::M8RuntimeInstance,
) -> Option<mir_semantics::surface_v0_classification::OwnerAdmissionBudgetCondition> {
    instance
        .owner_execution_plans()
        .iter()
        .find(|plan| plan.evaluation() == "attack")
        .and_then(|plan| plan.owner_admission_budget())
        .cloned()
}

fn private_snapshot_with_budget_field_replaced(
    pointer: &str,
    replacement: &str,
) -> M8I3PrivateSnapshot {
    let snapshot = admitted_owner_instance(BUDGETED_SOURCE_PATH, true)
        .i3_private_snapshot()
        .expect("an ordinary admitted owner instance exports a private M8 snapshot");
    let mut serialized = serde_json::to_value(snapshot)
        .expect("private M8 snapshot serializes for a bounded DTO falsifier");
    let field = serialized
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("private M8 snapshot must retain budget component {pointer}"));
    assert!(
        field.is_string(),
        "private M8 budget component {pointer} must remain a typed string in the DTO"
    );
    *field = Value::String(replacement.to_string());
    serde_json::from_value(serialized)
        .expect("the malformed semantic component remains syntactically decodable as a private DTO")
}

#[test]
fn i3_private_snapshot_rejects_tampered_provider_effect_lowering_before_restore() {
    let admitted = admitted_owner_instance(UNANNOTATED_SOURCE_PATH, false);
    let serialized =
        serde_json::to_value(admitted.i3_private_snapshot().expect(
            "an ordinary admitted unannotated owner instance exports a private M8 snapshot",
        ))
        .expect("a genuine admitted unannotated owner instance serializes as a private M8 DTO");

    let untampered: M8I3PrivateSnapshot = serde_json::from_value(serialized.clone())
        .expect("the genuine private M8 DTO remains syntactically decodable");
    let restored = super::M8RuntimeInstance::from_i3_private_snapshot(untampered)
        .expect("the untampered unannotated owner snapshot restores positively");
    assert!(restored.is_runtime_admitted());
    assert_eq!(
        owner_plan_budget(&restored),
        None,
        "the positive control remains the existing unannotated owner instance"
    );

    for provider_effect_kind in [
        "read_only_provider_effect_request",
        "read_only_provider_effect_invocation",
        "read_only_provider_effect_result",
        "read_only_provider_effect_result_consume",
    ] {
        let mut tampered = serialized.clone();
        let kind = tampered
            .pointer_mut("/ordered_lowering/0/kind")
            .expect("the genuine private M8 DTO retains its first ordered lowering kind");
        assert!(
            kind.is_string(),
            "the ordered lowering kind remains a typed string in the private DTO"
        );
        *kind = Value::String(provider_effect_kind.to_string());

        let tampered_snapshot: M8I3PrivateSnapshot = serde_json::from_value(tampered).expect(
            "a newly recognized provider-effect lowering kind remains syntactically decodable so the execution-image guard, rather than DTO parsing, decides it",
        );
        super::M8RuntimeInstance::from_i3_private_snapshot(tampered_snapshot).expect_err(
            "a private M8 snapshot cannot restore a provider-effect lowering without a provider Core, handler, or authority grant",
        );
    }
}

#[test]
fn i3_provider_effect_private_deferred_m9_base_rejects_before_ordered_lowering() {
    let checked = checked_provider_effect_source();
    let diagnostics = prepare_deferred_m9_base(
        &checked,
        &M8RuntimeAdmission::new(checked.program_identity().clone()),
    )
    .expect_err(
        "the retained provider Core must reject before a deferred M9 base can retain ordered lowering",
    );

    assert_eq!(
        diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile
    );
    assert_eq!(
        diagnostics.primary().residual_kind(),
        Some(ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported)
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
}

#[test]
fn i3_owner_admission_budget_private_m8_snapshot_retains_the_exact_restricted_plan_and_still_fails_closed_at_enqueue()
 {
    let checked = checked_owner_source(BUDGETED_SOURCE_PATH, true);
    let expected_budget = checked
        .evaluation("attack")
        .and_then(|evaluation| evaluation.owner_admission_budget())
        .cloned()
        .expect("the checked source retains its declared owner-admission condition");
    let restricted = admitted_owner_instance(BUDGETED_SOURCE_PATH, true)
        .restricted_to_loci(&BTreeSet::from(["S".to_string()]));

    let restored = super::M8RuntimeInstance::from_i3_private_snapshot(
        restricted
            .i3_private_snapshot()
            .expect("an ordinary restricted owner instance exports a private M8 snapshot"),
    )
    .expect("the exact restricted private M8 snapshot restores structurally");
    assert_eq!(
        owner_plan_budget(&restored),
        Some(expected_budget),
        "private M8 snapshot restore must retain the exact checked condition, not erase it"
    );

    let mut execution = restored.into_execution(M8ExecutionSeed::new());
    let rejected = execution
        .try_enqueue(M8OwnerRequest::new("attack"))
        .expect_err(
            "the restored annotated plan must still reject ordinary enqueue without a sealed admission authorization",
        );
    assert_eq!(
        rejected.primary().kind(),
        M8EnqueueDiagnosticKind::OwnerAdmissionAuthorizationRequired
    );
    assert!(
        execution.owner_queue("S").occurrence_ids().is_empty(),
        "the restored guard must allocate no owner queue occurrence"
    );
    assert!(
        execution.trace().entries().is_empty(),
        "the restored guard must not emit an enqueue or owner-service trace row"
    );
    assert_eq!(
        execution.next_occurrence, 0,
        "the restored guard must preserve the first occurrence index after rejection"
    );

    let unannotated = admitted_owner_instance(UNANNOTATED_SOURCE_PATH, false)
        .restricted_to_loci(&BTreeSet::from(["S".to_string()]));
    let restored_unannotated = super::M8RuntimeInstance::from_i3_private_snapshot(
        unannotated
            .i3_private_snapshot()
            .expect("an ordinary restricted unannotated instance exports a private M8 snapshot"),
    )
    .expect("legacy unannotated private M8 snapshot restores structurally");
    assert_eq!(
        owner_plan_budget(&restored_unannotated),
        None,
        "legacy unannotated owner plans retain no synthetic budget condition"
    );
}

#[test]
fn i3_owner_admission_budget_private_m8_snapshot_rejects_malformed_owner_domain_or_span_components()
{
    for (pointer, replacement) in [
        (
            "/owner_execution_plans/0/owner_admission_budget/owner_locus",
            "OtherOwner",
        ),
        (
            "/owner_execution_plans/0/owner_admission_budget/clock_domain",
            "CallerSelectedClock",
        ),
        (
            "/owner_execution_plans/0/owner_admission_budget/source_span/file",
            "tests/inline/forged_owner_budget_span.mir",
        ),
    ] {
        let rejected = super::M8RuntimeInstance::from_i3_private_snapshot(
            private_snapshot_with_budget_field_replaced(pointer, replacement),
        )
        .expect_err(
            "a syntactically decodable private M8 budget component must still match its owner, canonical clock domain, and source-span-derived reference",
        );
        assert_eq!(
            rejected,
            M8I3PrivateSnapshotError::SemanticSnapshot,
            "private M8 snapshot field {pointer} must reject before it restores an executable instance"
        );
    }
}

#[test]
fn i3_owner_admission_budget_ow1_contextual_worker_returns_the_unobserved_enqueue_guard_without_mutation()
 {
    let instance = admitted_owner_instance(BUDGETED_SOURCE_PATH, true)
        .restricted_to_loci(&BTreeSet::from(["S".to_string()]));
    let runtime = M8LocalRuntime::from_admitted(instance, M8LocalRuntimeSeed::new());
    let worker = Ow1WorkerBackend::spawn(LocusRef::new("S"), runtime);
    let state_before = worker
        .snapshot()
        .expect("the fresh OW1 worker returns its genuine admitted M8 snapshot");
    let trace_before = worker
        .local_trace_snapshot()
        .expect("the fresh OW1 worker exposes its empty M8 trace");

    let result = worker.execute_owner_with_context(
        "S",
        M8OwnerRequest::new("attack"),
        M8LocalDesignatedTraceContext::new(
            "ow1-budget-envelope",
            "ow1-budget-semantic",
            "",
            "",
            "",
            "",
        )
        .with_operation_id("attack")
        .with_owner_locus("S")
        .with_edge_ref("edge:S"),
    );

    match result {
        Ok(Ow1ContextualM8Execution::AdmissionRejected { diagnostics }) => assert_eq!(
            diagnostics.primary().kind(),
            M8EnqueueDiagnosticKind::OwnerAdmissionAuthorizationRequired,
            "the actual OW1 response must preserve the typed pre-enqueue owner-admission guard"
        ),
        other => panic!(
            "the actual OW1 contextual worker must return its unobserved admission rejection, not disconnect or fabricate a trace result: {other:?}"
        ),
    }

    let state_after = worker
        .snapshot()
        .expect("the OW1 worker remains connected after its typed admission rejection");
    let trace_after = worker
        .local_trace_snapshot()
        .expect("the OW1 worker trace remains observable after its typed admission rejection");
    assert_eq!(
        state_after, state_before,
        "the worker guard must allocate no occurrence, queue no owner request, and mutate no owner state"
    );
    assert_eq!(
        trace_after, trace_before,
        "the worker guard must not fabricate an enqueue, serve, write, or rejection trace row"
    );
    assert!(
        state_after.pending_owner_fifo("S").is_empty(),
        "the rejected contextual request must leave the actual worker FIFO empty"
    );
    assert_eq!(
        worker
            .shutdown_extract()
            .expect("the actual OW1 worker cleanly joins after the typed guard response"),
        state_before,
        "clean worker extraction must retain the unchanged genuine admitted M8 state"
    );
}
