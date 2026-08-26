use std::{collections::BTreeMap, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

use crate::m9_auth_verification::M9RuntimeExecutionSeam;
use crate::semantic_runtime_kernel::{
    ExecutionProfile, FailureKind, InputFrontier, KernelDiagnosticKind, KernelReceipt, KernelSeed,
    KernelStateKey, LocusRef, PrincipalRef, RemoteInputConsumeRequest, RemoteInputReleaseTuple,
    RemoteInputResult, RequestIdentity, SealedM9RuntimeAdmission, SemanticRuntimeKernel,
    SemanticValue, VisibilityClass,
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const DESIGNATED_FIXTURE: &str = "canonical_attack_bundle.mir";

const PRINCIPAL: &str = "self";
const OWNER_LOCUS: &str = "S";
const SECOND_OWNER_LOCUS: &str = "T";
const EVALUATOR_LOCUS: &str = "E";
const TARGET_ID: &str = "target";
const OWNER_OPERATION: &str = "attack";
const DESIGNATED_RESULT: &str = "result";
const DESIGNATED_INPUT_FRONTIER: &str = "F";
const DESIGNATED_INPUT_RELEASE_LABEL: &str = "input:player[self].atk:S:F";

const TWO_OWNER_SOURCE: &str = r#"
module Combat.Sys2.TwoOwners

locus S
locus T
principal self
principal target
type Player

state player[id: Player] at S {
  hp: Int
  atk: Int
}

state enemy[id: Player] at T {
  hp: Int
  atk: Int
}

Role[self] at S {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {
    at S {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}

Role[self] at T {
  when strike(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {
    at T {
      enemy[target].hp = enemy[target].hp - enemy[self].atk
    }
  }
}

with auth MembershipAuth

verify finite_refinement
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
struct OwnerRunSummary {
    hp: Option<i64>,
    lifecycles: Vec<Vec<&'static str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RemoteInputRunSummary {
    value: Option<SemanticValue>,
    lifecycle: Vec<&'static str>,
}

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_checked_fixture(name: &str) -> CheckedSurfaceV0 {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    check_and_elaborate_surface_v0(FixtureSource::new(relative, source))
        .expect("SYS-2 backend tests start from checked source")
}

fn load_checked_inline(path: &str, source: &str) -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(path.to_owned(), source.to_owned()))
        .expect("inline SYS-2 source checks before backend profile admission")
}

fn owner_locus() -> LocusRef {
    LocusRef::new(OWNER_LOCUS)
}

fn evaluator_locus() -> LocusRef {
    LocusRef::new(EVALUATOR_LOCUS)
}

fn principal() -> PrincipalRef {
    PrincipalRef::new(PRINCIPAL)
}

fn hp_key() -> KernelStateKey {
    KernelStateKey::indexed_field("player", TARGET_ID, "hp")
}

fn atk_key() -> KernelStateKey {
    KernelStateKey::indexed_field("player", PRINCIPAL, "atk")
}

fn owner_seed() -> KernelSeed {
    KernelSeed::new()
        .with_int(hp_key(), 100)
        .with_int(atk_key(), 10)
}

fn owner_arguments() -> BTreeMap<String, String> {
    BTreeMap::from([("target".to_string(), TARGET_ID.to_string())])
}

fn designated_release_tuple() -> RemoteInputReleaseTuple {
    RemoteInputReleaseTuple::new(
        principal(),
        owner_locus(),
        evaluator_locus(),
        DESIGNATED_INPUT_RELEASE_LABEL,
    )
    .with_visibility(VisibilityClass::RestrictedRedacted)
}

fn owner_kernel(profile: ExecutionProfile) -> (CheckedSurfaceV0, SemanticRuntimeKernel) {
    let checked = load_checked_fixture(DESIGNATED_FIXTURE);
    let seam = M9RuntimeExecutionSeam::test_real_admitted_owner_seam_for_kernel(
        &checked,
        OWNER_OPERATION,
        PRINCIPAL,
        OWNER_LOCUS,
    )
    .expect("SYS-2 uses a production-style M9 owner seam, not fabricated authority");
    let kernel = SemanticRuntimeKernel::from_m9_execution_seam_with_profile(
        checked.clone(),
        seam,
        owner_seed(),
        profile,
    )
    .expect("selected execution profile admits the source-derived owner kernel");
    (checked, kernel)
}

fn owner_and_remote_kernel(profile: ExecutionProfile) -> (CheckedSurfaceV0, SemanticRuntimeKernel) {
    let checked = load_checked_fixture(DESIGNATED_FIXTURE);
    let seam =
        M9RuntimeExecutionSeam::test_real_admitted_owner_and_designated_remote_input_seam_for_kernel(
            &checked,
            OWNER_OPERATION,
            PRINCIPAL,
            OWNER_LOCUS,
            EVALUATOR_LOCUS,
            DESIGNATED_RESULT,
            0,
        )
        .expect("SYS-2 uses one real M9 seam carrying owner and remote-input lineage");

    let admission = SealedM9RuntimeAdmission::from_m9_execution_seam(&checked, &seam)
        .expect("real M9 seam converts into sealed kernel admission");
    let sealed = admission
        .m9_sealed_remote_input_lineage_for_test(EVALUATOR_LOCUS, DESIGNATED_RESULT, 0)
        .expect("remote-input release lineage is sealed by M9");
    assert_eq!(sealed.source_owner(), &owner_locus());
    assert_eq!(sealed.target_evaluator(), &evaluator_locus());
    assert_eq!(
        sealed.input_frontier(),
        &InputFrontier::new(DESIGNATED_INPUT_FRONTIER)
    );
    assert_eq!(sealed.release_tuple(), &designated_release_tuple());
    assert_eq!(
        sealed.visibility_class(),
        VisibilityClass::RestrictedRedacted
    );

    let kernel = SemanticRuntimeKernel::from_m9_execution_seam_with_profile(
        checked.clone(),
        seam,
        owner_seed(),
        profile,
    )
    .expect("selected execution profile admits the mixed owner/remote-input kernel");
    (checked, kernel)
}

fn enqueue_owner(kernel: &mut SemanticRuntimeKernel) -> RequestIdentity {
    let queued = kernel
        .enqueue_owner_request(
            kernel
                .owner_request_from_admitted_lineage(
                    OWNER_OPERATION,
                    owner_locus(),
                    owner_arguments(),
                )
                .expect("M9-sealed owner lineage materializes the carrier"),
        )
        .expect("source-derived owner request queues");
    queued.request_identity().clone()
}

fn complete_owner_request(kernel: &mut SemanticRuntimeKernel) -> KernelReceipt {
    let served = kernel
        .serve_next_owner(owner_locus())
        .expect("owner worker serves the next admitted request");
    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("served owner request emits one reply");
    kernel
        .receive_reply(reply)
        .expect("reply installs one typed receipt")
}

fn run_two_owner_attacks(profile: ExecutionProfile) -> OwnerRunSummary {
    let (_checked, mut kernel) = owner_kernel(profile);
    let mut lifecycles = Vec::new();
    for _ in 0..2 {
        let identity = enqueue_owner(&mut kernel);
        let receipt = complete_owner_request(&mut kernel);
        assert_eq!(receipt.request_identity(), &identity);
        assert_eq!(
            kernel.trace().lifecycle_for(&identity),
            ["request", "serve", "reply", "receive_receipt"]
        );
        lifecycles.push(kernel.trace().lifecycle_for(&identity));
    }
    if profile == ExecutionProfile::Ow1 {
        let ordering = kernel.ordering_evidence();
        let worker = ordering
            .dedicated_owner_worker(&owner_locus())
            .expect("OW1 reports a dedicated owner worker for S");
        assert_eq!(worker.target_owner(), &owner_locus());
        assert_eq!(worker.mailbox().target_owner(), &owner_locus());
        assert!(worker.mailbox().is_fifo());
    }
    OwnerRunSummary {
        hp: kernel.semantic_snapshot().int(&hp_key()),
        lifecycles,
    }
}

#[test]
fn st_and_ow1_execute_same_owner_rmw_with_identical_result_and_lifecycle() {
    let st = run_two_owner_attacks(ExecutionProfile::St);
    let ow1 = run_two_owner_attacks(ExecutionProfile::Ow1);

    assert_eq!(st.hp, Some(80));
    assert_eq!(ow1.hp, st.hp);
    assert_eq!(ow1.lifecycles, st.lifecycles);
}

fn run_remote_input_after_owner(profile: ExecutionProfile) -> RemoteInputRunSummary {
    let (checked, mut kernel) = owner_and_remote_kernel(profile);
    enqueue_owner(&mut kernel);
    complete_owner_request(&mut kernel);

    let requested = kernel
        .enqueue_remote_input_request(
            kernel
                .remote_input_request_from_admitted_lineage(EVALUATOR_LOCUS, DESIGNATED_RESULT, 0)
                .expect("M9-sealed designated remote input materializes the carrier"),
        )
        .expect("source-owner remote input queues");
    let served = kernel
        .serve_next_remote_input(owner_locus())
        .expect("source-owner worker serves the remote input read");
    let reply = kernel
        .reply_to_remote_input(
            served.serve_occurrence(),
            RemoteInputResult::success(SemanticValue::Int(10)),
        )
        .expect("remote input result matches the source-owner value");
    let receipt = kernel
        .receive_remote_input_reply(reply)
        .expect("remote input reply installs typed receipt");
    let consumed = kernel
        .consume_remote_input_receipt(
            RemoteInputConsumeRequest::from_checked_designated_dependency(
                &checked,
                EVALUATOR_LOCUS,
                DESIGNATED_RESULT,
                0,
            )
            .with_receipt(receipt.receipt_id())
            .with_evaluator(evaluator_locus()),
        )
        .expect("designated evaluator consumes the exact receipt");

    let ordering = kernel.ordering_evidence();
    let read = ordering
        .reads_from(&requested.request_identity().clone(), &atk_key())
        .expect("ordering evidence records the source-owner read");
    assert_eq!(read.source_owner(), &owner_locus());
    assert_eq!(
        read.observed_version(),
        ordering.latest_owner_version(&owner_locus(), &atk_key())
    );
    assert_eq!(
        kernel.trace().lifecycle_for(requested.request_identity()),
        ["request", "serve", "reply", "receive_receipt", "consume"]
    );

    RemoteInputRunSummary {
        value: consumed.value().cloned(),
        lifecycle: kernel.trace().lifecycle_for(requested.request_identity()),
    }
}

#[test]
fn ow1_remote_input_reads_latest_worker_owned_owner_version_and_matches_st() {
    let st = run_remote_input_after_owner(ExecutionProfile::St);
    let ow1 = run_remote_input_after_owner(ExecutionProfile::Ow1);

    assert_eq!(st.value, Some(SemanticValue::Int(10)));
    assert_eq!(ow1.value, st.value);
    assert_eq!(ow1.lifecycle, st.lifecycle);
}

fn revoke_owner_capability_after_enqueue_then_serve(profile: ExecutionProfile) {
    let (checked, mut kernel) = owner_kernel(profile);
    let identity = enqueue_owner(&mut kernel);
    let before = kernel.semantic_snapshot().clone();
    let g0 = kernel.current_authority_generation().clone();
    let g1 = M9RuntimeExecutionSeam::test_real_successor_generation_revoking_owner_cap_for_kernel(
        &checked,
        OWNER_OPERATION,
        PRINCIPAL,
        OWNER_LOCUS,
        &g0,
    )
    .expect("successor generation revokes the real M9 owner capability");
    kernel
        .install_and_ack_m9_successor_generation(g1.clone())
        .expect("kernel installs and acknowledges the opaque M9 successor generation");

    let served = kernel
        .serve_next_owner(owner_locus())
        .expect("post-revocation serve yields a typed failure occurrence, not a panic");
    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("post-revocation served request emits a typed failure reply");
    assert_eq!(reply.failure(), Some(FailureKind::MissingCapability));
    let receipt = kernel
        .receive_reply(reply)
        .expect("post-revocation failure reply still installs a typed receipt");
    assert_eq!(receipt.request_identity(), &identity);
    assert_eq!(receipt.failure(), Some(FailureKind::MissingCapability));
    assert_eq!(kernel.semantic_snapshot(), &before);
    assert!(
        kernel
            .trace()
            .contains_typed_failure_receipt(&identity, FailureKind::MissingCapability)
    );
    assert!(
        kernel
            .ordering_evidence()
            .authority_generation_publish_before_serve(
                g1.generation_ref(),
                served.serve_occurrence()
            )
    );
}

fn serve_before_revoke_then_later_use_rejects(profile: ExecutionProfile) {
    let (checked, mut kernel) = owner_kernel(profile);
    let identity = enqueue_owner(&mut kernel);
    let receipt = complete_owner_request(&mut kernel);
    assert_eq!(receipt.request_identity(), &identity);
    assert_eq!(receipt.failure(), None);
    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), Some(90));

    let g0 = kernel.current_authority_generation().clone();
    let g1 = M9RuntimeExecutionSeam::test_real_successor_generation_revoking_owner_cap_for_kernel(
        &checked,
        OWNER_OPERATION,
        PRINCIPAL,
        OWNER_LOCUS,
        &g0,
    )
    .expect("successor generation revokes the real M9 owner capability");
    kernel
        .install_and_ack_m9_successor_generation(g1)
        .expect("kernel installs and acknowledges the opaque M9 successor generation");

    let later = kernel
        .enqueue_owner_request(
            kernel
                .owner_request_from_admitted_lineage(
                    OWNER_OPERATION,
                    owner_locus(),
                    owner_arguments(),
                )
                .expect("source-derived lineage is requested after revocation"),
        )
        .expect_err("later owner-capability use rejects after revocation");
    assert_eq!(
        later.primary().kind(),
        KernelDiagnosticKind::MissingCapability
    );
    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), Some(90));
}

#[test]
fn authority_generation_revoke_after_enqueue_blocks_serve_and_reverse_order_preserves_result() {
    for profile in [ExecutionProfile::St, ExecutionProfile::Ow1] {
        revoke_owner_capability_after_enqueue_then_serve(profile);
        serve_before_revoke_then_later_use_rejects(profile);
    }
}

#[test]
fn ow1_rejects_more_than_one_owner_locus_without_state_duplication() {
    let checked = load_checked_inline("tests/inline/sys2_two_owner_ow1.mir", TWO_OWNER_SOURCE);
    let seam = M9RuntimeExecutionSeam::test_real_admitted_multi_owner_seam_for_kernel(
        &checked,
        [
            (OWNER_OPERATION, PRINCIPAL, OWNER_LOCUS),
            ("strike", PRINCIPAL, SECOND_OWNER_LOCUS),
        ],
    )
    .expect("multi-owner source is admitted by M9 before OW1 profile selection");

    let diagnostics = SemanticRuntimeKernel::from_m9_execution_seam_with_profile(
        checked,
        seam,
        owner_seed(),
        ExecutionProfile::Ow1,
    )
    .expect_err("OW1 is one-owner-worker only and must not duplicate owner state");
    assert_eq!(
        diagnostics.primary().kind(),
        KernelDiagnosticKind::ExecutionProfileUnsupported
    );
}

#[test]
fn ordering_evidence_records_lifecycle_linearization_reads_from_and_generation_edges() {
    let (checked, mut kernel) = owner_kernel(ExecutionProfile::Ow1);
    let identity = enqueue_owner(&mut kernel);
    let receipt = complete_owner_request(&mut kernel);
    assert_eq!(receipt.request_identity(), &identity);

    let ordering = kernel.ordering_evidence();
    let lifecycle = ordering
        .lifecycle_occurrences(&identity)
        .expect("ordering evidence retains concrete occurrence ids");
    assert!(lifecycle.request_before_serve());
    assert!(lifecycle.serve_before_reply());
    assert!(lifecycle.reply_before_receive_receipt());
    let commit = ordering
        .owner_commit_linearization_point(&identity)
        .expect("owner write has an explicit linearization point");
    assert_eq!(commit.owner(), &owner_locus());
    assert_eq!(commit.key(), &hp_key());
    assert_eq!(commit.written_version(), 1);
    let read = ordering
        .reads_from(&identity, &atk_key())
        .expect("owner RMW records its source-state read");
    assert_eq!(read.source_owner(), &owner_locus());
    assert_eq!(
        read.observed_version(),
        ordering.latest_owner_version(&owner_locus(), &atk_key())
    );

    let queued_after_g0 = enqueue_owner(&mut kernel);
    let g0 = kernel.current_authority_generation().clone();
    let g1 = M9RuntimeExecutionSeam::test_real_successor_generation_revoking_owner_cap_for_kernel(
        &checked,
        OWNER_OPERATION,
        PRINCIPAL,
        OWNER_LOCUS,
        &g0,
    )
    .expect("successor generation revokes the real M9 owner capability");
    kernel
        .install_and_ack_m9_successor_generation(g1.clone())
        .expect("successor generation publishes before the queued serve");
    let served = kernel
        .serve_next_owner(owner_locus())
        .expect("queued request after generation publish is served as typed failure");
    assert!(
        kernel
            .ordering_evidence()
            .authority_generation_publish_before_request_serve(
                g1.generation_ref(),
                &queued_after_g0,
                served.serve_occurrence(),
            )
    );
}

#[test]
fn ow1_worker_owns_m8_and_exposes_no_public_shared_store_surface() {
    let (_checked, kernel) = owner_kernel(ExecutionProfile::Ow1);
    let ordering = kernel.ordering_evidence();
    let worker = ordering
        .dedicated_owner_worker(&owner_locus())
        .expect("OW1 exposes typed worker evidence for the owner locus");

    assert!(worker.owns_m8_runtime());
    assert!(worker.public_shared_store_surface().is_none());
    assert!(
        !worker
            .debug_type_surface()
            .contains("Arc<Mutex<M8LocalRuntime>>"),
        "OW1 must not expose a public Arc/Mutex/shared-store surface"
    );
}
