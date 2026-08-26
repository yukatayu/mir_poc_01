use std::{collections::BTreeMap, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

use crate::m9_auth_verification::M9RuntimeExecutionSeam;
use crate::semantic_runtime_kernel::{
    FailureKind, InputFrontier, KernelDiagnosticKind, KernelReceipt, KernelSeed, KernelStateKey,
    LocusRef, PrincipalRef, QueuedOwnerRequest, RemoteInputConsumeRequest, RemoteInputReleaseTuple,
    RemoteInputResult, RequestIdentity, SealedM9RuntimeAdmission, SemanticRuntimeKernel,
    SemanticValue, VisibilityClass,
};
use crate::sys2_execution_backend::ExecutionProfile;

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
    request_ids: Vec<RequestIdentity>,
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

fn enqueue_owner_queued(kernel: &mut SemanticRuntimeKernel) -> QueuedOwnerRequest {
    kernel
        .enqueue_owner_request(
            kernel
                .owner_request_from_admitted_lineage(
                    OWNER_OPERATION,
                    owner_locus(),
                    owner_arguments(),
                )
                .expect("M9-sealed owner lineage materializes the carrier"),
        )
        .expect("source-derived owner request queues")
}

fn enqueue_owner(kernel: &mut SemanticRuntimeKernel) -> RequestIdentity {
    enqueue_owner_queued(kernel).request_identity().clone()
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

fn run_two_same_owner_attacks(profile: ExecutionProfile) -> OwnerRunSummary {
    let (_checked, mut kernel) = owner_kernel(profile);
    let mut request_ids = Vec::new();
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
        request_ids.push(identity);
    }

    let first_id = &request_ids[0];
    let second_id = &request_ids[1];
    let ordering = kernel.ordering_evidence();
    let first_commit = ordering
        .owner_commit_linearization_point(first_id)
        .expect("first same-owner RMW records an owner commit LP");
    let second_commit = ordering
        .owner_commit_linearization_point(second_id)
        .expect("second same-owner RMW records an owner commit LP");
    assert_eq!(first_commit.owner(), &owner_locus());
    assert_eq!(second_commit.owner(), &owner_locus());
    assert_eq!(first_commit.key(), &hp_key());
    assert_eq!(second_commit.key(), &hp_key());
    assert_eq!(first_commit.written_version(), 1);
    assert_eq!(second_commit.written_version(), 2);
    assert!(
        first_commit.written_version() < second_commit.written_version(),
        "per-key modification order for hp must place first same-owner RMW before second"
    );

    let second_hp_read = ordering
        .reads_from(second_id, &hp_key())
        .expect("second same-owner RMW reads the first hp commit");
    assert_eq!(second_hp_read.source_owner(), &owner_locus());
    assert_eq!(
        second_hp_read.observed_version(),
        first_commit.written_version()
    );
    assert_eq!(
        second_hp_read.producer_request(),
        Some(first_id),
        "second hp read must point to the first same-owner commit, not an unversioned store"
    );
    for identity in [first_id, second_id] {
        let atk_read = ordering
            .reads_from(identity, &atk_key())
            .expect("same-owner RMW records the atk read");
        assert_eq!(atk_read.source_owner(), &owner_locus());
        assert_eq!(atk_read.observed_version(), 0);
        assert!(
            atk_read.is_initial_seed(),
            "atk stays at the Init/v0 source for both same-owner RMWs"
        );
    }

    if profile == ExecutionProfile::Ow1 {
        let worker = ordering
            .dedicated_owner_worker(&owner_locus())
            .expect("OW1 reports a dedicated owner worker for S");
        assert_eq!(worker.target_owner(), &owner_locus());
        assert_eq!(worker.mailbox().target_owner(), &owner_locus());
        assert!(worker.mailbox().is_fifo());
        assert_eq!(
            worker.mailbox().observed_request_order(),
            request_ids.as_slice(),
            "OW1 owner mailbox evidence must retain exact FIFO request order"
        );
    }
    OwnerRunSummary {
        hp: kernel.semantic_snapshot().int(&hp_key()),
        request_ids,
        lifecycles,
    }
}

#[test]
fn st_and_ow1_execute_same_owner_rmw_with_identical_result_and_lifecycle() {
    let st = run_two_same_owner_attacks(ExecutionProfile::St);
    let ow1 = run_two_same_owner_attacks(ExecutionProfile::Ow1);

    assert_eq!(st.hp, Some(80));
    assert_eq!(ow1.hp, st.hp);
    assert_eq!(ow1.lifecycles, st.lifecycles);
    assert_eq!(
        st.request_ids.len(),
        2,
        "same-owner comparison keeps both request identities"
    );
    assert_eq!(
        ow1.request_ids.len(),
        2,
        "OW1 same-owner comparison keeps both request identities"
    );
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
        .reply_to_served_remote_input(served.serve_occurrence())
        .expect("remote input reply is derived from the source-owner worker value");
    let source_read = reply
        .source_owner_read()
        .expect("derived remote reply exposes typed source-owner read evidence");
    assert_eq!(source_read.owner(), &owner_locus());
    assert_eq!(source_read.key(), &atk_key());
    assert_eq!(source_read.value(), &SemanticValue::Int(10));
    let source_read_version = source_read.observed_version();
    let receipt = kernel
        .receive_remote_input_reply(reply)
        .expect("remote input reply installs typed receipt");
    assert_eq!(receipt.value(), Some(&SemanticValue::Int(10)));
    assert_eq!(receipt.source_owner(), &owner_locus());
    assert_eq!(receipt.target_evaluator(), &evaluator_locus());
    assert_eq!(receipt.release_tuple(), &designated_release_tuple());
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
    assert_eq!(read.observed_version(), source_read_version);
    assert_eq!(
        kernel.trace().lifecycle_for(requested.request_identity()),
        ["request", "serve", "reply", "receive_receipt", "consume"]
    );
    let remote_hb = ordering
        .remote_input_hb(requested.request_identity())
        .expect("remote input ordering evidence retains typed HB occurrences");
    assert!(remote_hb.request_before_source_owner_serve());
    assert!(remote_hb.source_owner_serve_before_reply());
    assert!(remote_hb.reply_before_receive_receipt());
    assert!(remote_hb.receive_receipt_before_consume());
    assert_eq!(remote_hb.producer(), &owner_locus());
    assert_eq!(remote_hb.evaluator(), &evaluator_locus());
    assert_eq!(remote_hb.release_tuple(), &designated_release_tuple());

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

#[test]
fn legacy_explicit_remote_input_result_mismatch_rejects_without_reply_receipt_or_mutation() {
    let (_checked, mut kernel) = owner_and_remote_kernel(ExecutionProfile::Ow1);
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
    let before = kernel.semantic_snapshot().clone();
    let before_receipts = kernel.receipt_store().clone();
    let before_lifecycle = kernel.trace().lifecycle_for(requested.request_identity());

    let diagnostics = kernel
        .reply_to_remote_input(
            served.serve_occurrence(),
            RemoteInputResult::success(SemanticValue::Int(999)),
        )
        .expect_err("legacy explicit remote-input result cannot fabricate source-owner value");
    assert_eq!(
        diagnostics.primary().kind(),
        KernelDiagnosticKind::RemoteInputValueMismatch
    );
    assert_eq!(kernel.semantic_snapshot(), &before);
    assert_eq!(kernel.receipt_store(), &before_receipts);
    assert_eq!(
        kernel.trace().lifecycle_for(requested.request_identity()),
        before_lifecycle,
        "mismatched explicit result records no reply or receipt lifecycle event"
    );
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

fn serve_write_then_revoke_before_reply_preserves_completed_mutation_and_later_use_rejects(
    profile: ExecutionProfile,
) {
    let (checked, mut kernel) = owner_kernel(profile);
    let identity = enqueue_owner(&mut kernel);
    let served = kernel
        .serve_next_owner(owner_locus())
        .expect("owner worker serves and commits the admitted request before revocation");
    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), Some(90));
    let after_serve = kernel.semantic_snapshot().clone();

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
    assert_eq!(
        kernel.semantic_snapshot(),
        &after_serve,
        "revocation publish/ack cannot roll back a completed owner mutation"
    );

    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("reply after revocation remains a report for the already-served request");
    assert_eq!(reply.failure(), None);
    assert!(
        !reply.transfers_authority(),
        "reply/receipt is not an authority-transfer mechanism"
    );
    let receipt = kernel
        .receive_reply(reply)
        .expect("receipt after revocation records completed result without granting authority");
    assert_eq!(receipt.request_identity(), &identity);
    assert_eq!(receipt.failure(), None);
    assert!(
        !receipt.transfers_authority(),
        "receiving the reply cannot revive or transfer revoked authority"
    );
    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), Some(90));

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
    assert!(
        kernel
            .ordering_evidence()
            .owner_commit_before_authority_generation_publish(&identity, g1.generation_ref())
    );
    assert!(
        kernel
            .ordering_evidence()
            .authority_generation_publish_before_reply_receive(
                g1.generation_ref(),
                served.serve_occurrence(),
                receipt.request_identity(),
            )
    );
}

#[test]
fn authority_generation_revoke_after_enqueue_blocks_serve_and_reverse_order_preserves_result() {
    for profile in [ExecutionProfile::St, ExecutionProfile::Ow1] {
        revoke_owner_capability_after_enqueue_then_serve(profile);
        serve_write_then_revoke_before_reply_preserves_completed_mutation_and_later_use_rejects(
            profile,
        );
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
    assert!(read.is_initial_seed());

    let queued_after_g0 = enqueue_owner_queued(&mut kernel);
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
    let (checked, mut kernel) = owner_and_remote_kernel(ExecutionProfile::Ow1);
    let owner_identity = enqueue_owner(&mut kernel);
    complete_owner_request(&mut kernel);
    let remote_requested = kernel
        .enqueue_remote_input_request(
            kernel
                .remote_input_request_from_admitted_lineage(EVALUATOR_LOCUS, DESIGNATED_RESULT, 0)
                .expect("M9-sealed designated remote input materializes the carrier"),
        )
        .expect("source-owner remote input queues");
    let remote_served = kernel
        .serve_next_remote_input(owner_locus())
        .expect("source owner serves remote input through OW1 worker");
    let remote_reply = kernel
        .reply_to_served_remote_input(remote_served.serve_occurrence())
        .expect("source-owner worker derives remote input value");
    let remote_receipt = kernel
        .receive_remote_input_reply(remote_reply)
        .expect("remote input reply installs typed receipt");
    kernel
        .consume_remote_input_receipt(
            RemoteInputConsumeRequest::from_checked_designated_dependency(
                &checked,
                EVALUATOR_LOCUS,
                DESIGNATED_RESULT,
                0,
            )
            .with_receipt(remote_receipt.receipt_id())
            .with_evaluator(evaluator_locus()),
        )
        .expect("designated evaluator consumes exact receipt");

    let ordering = kernel.ordering_evidence();
    let worker = ordering
        .dedicated_owner_worker(&owner_locus())
        .expect("OW1 exposes typed worker evidence for the owner locus");
    let owner_observed = ordering
        .worker_execution_observations(&owner_identity)
        .expect("OW1 records actual M8 command execution for owner request");
    assert_eq!(owner_observed.owner_worker_token(), worker.worker_token());
    assert_eq!(owner_observed.enqueue_worker_token(), worker.worker_token());
    assert_eq!(owner_observed.serve_worker_token(), worker.worker_token());
    assert_eq!(
        owner_observed.read_worker_token(&atk_key()),
        Some(worker.worker_token())
    );
    assert_ne!(
        owner_observed.coordinator_token(),
        worker.worker_token(),
        "coordinator and owner worker tokens must be distinct"
    );

    let remote_observed = ordering
        .remote_worker_execution_observations(remote_requested.request_identity())
        .expect("OW1 records actual M8 command execution for remote source-owner read");
    assert_eq!(
        remote_observed.source_owner_serve_worker_token(),
        worker.worker_token()
    );
    assert_eq!(
        remote_observed.read_worker_token(&atk_key()),
        Some(worker.worker_token())
    );
    assert_ne!(remote_observed.coordinator_token(), worker.worker_token());
    let owner_worker_tokens = ordering.owner_worker_tokens();
    assert_eq!(
        owner_worker_tokens.len(),
        1,
        "OW1 must not create a second owner worker for this selected profile"
    );
    assert_eq!(owner_worker_tokens[0], worker.worker_token());

    assert!(worker.owns_m8_runtime());
    assert!(worker.public_shared_store_surface().is_none());
    assert!(
        !worker
            .debug_type_surface()
            .contains("Arc<Mutex<M8LocalRuntime>>"),
        "OW1 must not expose a public Arc/Mutex/shared-store surface"
    );
}
