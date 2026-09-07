use mir_ast::surface_v0::FixtureSource;
use mir_runtime::{
    m8_runtime_admission::{M8Runtime, M8RuntimeAdmission},
    m8_runtime_authority::{
        M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
    },
    m8_runtime_local_cut::{M8LocalRuntime, M8LocalRuntimeSeed, M8LocalTraceKind},
    m8_runtime_owner_queue::{M8AuthorityUse, M8EnqueueDiagnosticKind, M8OwnerRequest, M8StateKey},
};
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

const BUDGETED_OWNER_SOURCE_PATH: &str =
    "tests/inline/i3_owner_admission_budget_runtime_budgeted.mir";
const UNANNOTATED_OWNER_SOURCE_PATH: &str =
    "tests/inline/i3_owner_admission_budget_runtime_unannotated.mir";
const OWNER: &str = "S";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:owner_epoch1";
const ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:owner_epoch1";
const ATTACK_WITNESS_REF: &str = "witness:attack:S:self:owner_epoch1";

fn owner_admission_budget_source(include_clause: bool) -> String {
    let clause = if include_clause {
        " within owner_ticks 1"
    } else {
        ""
    };
    format!(
        "module Combat.I3.OwnerAdmissionBudgetRuntime

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
        owner_admission_budget_source(include_clause),
    ))
    .expect("ordinary source-generated owner request checks before genuine M8 admission")
}

fn hp_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "target", "hp")
}

fn atk_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "self", "atk")
}

fn admitted_owner_authority() -> M8AuthorityState {
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted(OWNER_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("owner_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(ATTACK_CAPABILITY_REF)
                .for_owner_evaluation("attack")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("owner_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(ATTACK_WITNESS_REF)
                .for_capability(ATTACK_CAPABILITY_REF)
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("owner_epoch:1"),
        )
}

fn admitted_owner_request() -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(
            M8AuthorityUse::for_principal("self")
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_capability_ref(ATTACK_CAPABILITY_REF)
                .with_witness_ref(ATTACK_WITNESS_REF),
        )
}

fn local_runtime(path: &str, include_clause: bool) -> M8LocalRuntime {
    let checked = checked_owner_source(path, include_clause);
    let instance = M8Runtime::default()
        .admit(
            checked.clone(),
            M8RuntimeAdmission::new(checked.program_identity().clone()),
        )
        .expect("residual-free ordinary owner source admits through genuine M8 admission");
    M8LocalRuntime::from_admitted(
        instance,
        M8LocalRuntimeSeed::new()
            .with_owner_int(hp_key(), 100)
            .with_owner_int(atk_key(), 10)
            .with_authority_state(admitted_owner_authority()),
    )
}

#[test]
fn i3_owner_admission_budget_rejects_at_central_m8_enqueue_before_queue_or_mutation() {
    let mut runtime = local_runtime(BUDGETED_OWNER_SOURCE_PATH, true);
    let before_state = runtime.owner_state().clone();
    let before_trace_len = runtime.trace().len();

    let rejected = runtime
        .enqueue_owner(admitted_owner_request())
        .expect_err(
        "a source-declared owner admission budget must reject at M8 enqueue until a sealed admission authorization exists"
    );
    assert_eq!(
        rejected.primary().kind(),
        M8EnqueueDiagnosticKind::OwnerAdmissionAuthorizationRequired,
        "central M8 enqueue must distinguish a missing sealed admission authorization from an owner serve outcome"
    );
    assert!(
        runtime.pending_owner_fifo(OWNER).is_empty(),
        "budget rejection must allocate no owner occurrence or queue entry"
    );
    assert_eq!(
        runtime.owner_state(),
        &before_state,
        "budget rejection must not mutate owner state"
    );
    let post_rejection_kinds = runtime.trace().suffix_from(before_trace_len).kinds();
    assert!(
        post_rejection_kinds.iter().all(|kind| {
            !matches!(
                kind,
                M8LocalTraceKind::OwnerEnqueued
                    | M8LocalTraceKind::OwnerAuthorityValidated
                    | M8LocalTraceKind::OwnerRead
                    | M8LocalTraceKind::OwnerWrite
            )
        }),
        "budget rejection must not create enqueue, authority-validation, read, or write occurrences"
    );
}

#[test]
fn i3_owner_admission_budget_keeps_unannotated_owner_enqueue_and_serve_behavior_unchanged() {
    let mut runtime = local_runtime(UNANNOTATED_OWNER_SOURCE_PATH, false);

    let queued = runtime
        .enqueue_owner(admitted_owner_request())
        .expect("unannotated source remains on the ordinary admitted M8 owner path");
    assert_eq!(
        runtime.pending_owner_fifo(OWNER),
        vec![queued.id().to_string()]
    );
    let served = runtime
        .serve_next_owner(OWNER)
        .expect("unannotated admitted owner request still serves normally");
    assert_eq!(served.failure(), None);
    assert_eq!(served.read_int(&hp_key()), Some(100));
    assert_eq!(served.written_int(&hp_key()), Some(90));
    assert_eq!(runtime.owner_state().int(&hp_key()), Some(90));
}
