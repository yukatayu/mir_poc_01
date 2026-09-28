//! W4-C current-authority counterexample search; no real-network claim.
use crate::{
    m8_runtime_local_cut::M8LocalTraceKind,
    m8_runtime_owner_queue::{M8ServeOutcome, M8StateKey},
    m9_auth_verification::M9RuntimeExecutionSeam,
    sys3_projection::{BackendProfile, DeclaredLogicalTopology, project_checked_core},
    sys4_dispatch::{
        FabricProgram, LocalFabric, SealedFabricAdmission, SourceAction, Sys4DiagnosticKind,
        Sys4InitialStateSeed,
    },
};
use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::check_and_elaborate_surface_v0;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex, TryLockError, mpsc},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PauseAt {
    BeforeOwner,
    AfterOwner,
    AfterWrite,
}

#[derive(Debug)]
struct ProbeChannels {
    at: PauseAt,
    entered: mpsc::SyncSender<()>,
    resume: Mutex<mpsc::Receiver<()>>,
    panic_on_resume: bool,
    observed_write: Mutex<Option<M8ServeOutcome>>,
}

/// Test-only rendezvous follows the actual context into the OW1 thread.
/// It neither changes the request nor creates an execution/observer receipt.
#[derive(Clone, Debug)]
pub(crate) struct OwnerUseProbe(Arc<ProbeChannels>);

impl PartialEq for OwnerUseProbe {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for OwnerUseProbe {}

thread_local! {
    static OWNER_USE_PROBE: RefCell<Option<OwnerUseProbe>> = const { RefCell::new(None) };
}

pub(crate) fn current_owner_use_probe() -> Option<OwnerUseProbe> {
    OWNER_USE_PROBE.with(|slot| slot.borrow().clone())
}

impl OwnerUseProbe {
    fn pause(&self, at: PauseAt) {
        if self.0.at == at {
            self.0.entered.send(()).expect("test controller alive");
            self.0
                .resume
                .lock()
                .unwrap()
                .recv()
                .expect("test releases owner");
            assert!(
                !self.0.panic_on_resume,
                "intentional test-only owner worker panic"
            );
        }
    }

    pub(crate) fn before_owner(&self) {
        self.pause(PauseAt::BeforeOwner);
    }

    pub(crate) fn after_write(&self, outcome: &M8ServeOutcome) {
        if self.0.at == PauseAt::AfterWrite {
            *self.0.observed_write.lock().unwrap() = Some(outcome.clone());
            self.pause(PauseAt::AfterWrite);
        }
    }

    pub(crate) fn after_owner(&self) {
        self.pause(PauseAt::AfterOwner);
    }
}

fn admitted_siblings(profile: BackendProfile) -> (LocalFabric, LocalFabric) {
    let source =
        include_str!("../../mir-ast/tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir",
        source,
    ))
    .expect("ordinary owner read/assignment source checks");
    let topology = DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"])
        .expect("finite source-derived loci");
    let projection = project_checked_core(&checked, &topology).expect("checked Core projects");
    let program = FabricProgram::from_projection(projection).expect("static fabric");
    let seam = M9RuntimeExecutionSeam::test_real_admitted_sys4_fabric_seam(&checked)
        .expect("normal M9 admission route, no fabricated grant");
    let seed = Sys4InitialStateSeed::for_checked_program(checked.program_identity().clone())
        .with_int("S", "player", "self", "hp", 100)
        .with_int("S", "player", "self", "atk", 10);
    let admission = SealedFabricAdmission::from_m9_execution_seam(&program, seam, seed)
        .expect("sealed admission");
    let authority_owner = LocalFabric::bootstrap(program.clone(), admission.clone(), profile)
        .expect("first admitted fabric");
    let sibling = LocalFabric::bootstrap(program, admission, profile)
        .expect("second admitted fabric from the same sealed admission");
    assert_eq!(
        authority_owner.m9_authority_live_floor_identity_snapshot(),
        sibling.m9_authority_live_floor_identity_snapshot(),
        "the two fabrics share the same authority floor"
    );
    (authority_owner, sibling)
}

fn stale_sibling_must_refuse(profile: BackendProfile) {
    let (mut authority_owner, mut sibling) = admitted_siblings(profile);
    let transition = authority_owner
        .m9_authority_lifecycle_mut()
        .revoke_owner_capability("attack", "S")
        .expect("genuine M9 revocation");
    authority_owner
        .apply_admitted_authority_lifecycle(transition)
        .expect("revocation published through the shared floor");
    let before = sibling.semantic_snapshot();
    let before_m8 = sibling
        .try_m8_partition_evidence()
        .expect("fresh actual backend observation before the rejected use");
    let result = sibling.dispatch_source_action(
        SourceAction::owner_operation("attack").with_argument("target", "self"),
    );
    let after_m8 = sibling
        .try_m8_partition_evidence()
        .expect("fresh actual backend observation after the rejected use");
    assert!(
        after_m8.changed_partitions_since(&before_m8).is_empty(),
        "actual M8 sessions changed under stale authority; a coordinator mirror is insufficient"
    );
    let failure = result.expect_err("stale shared-floor authority must refuse owner write");
    assert_eq!(
        sibling
            .current_m9_authority_inspection()
            .owner_operation_validation_count(
                "attack",
                "S",
                failure
                    .rejected_request_id()
                    .expect("typed refused request")
            ),
        0,
        "floor rejection precedes an admitted M9 use observation"
    );
    assert_eq!(
        failure.primary().kind(),
        Sys4DiagnosticKind::M8ExecutionRejected
    );
    assert!(
        sibling.semantic_snapshot().same_state(&before),
        "refused stale authority must not change owner state"
    );
}

#[test]
fn proof_first_shared_authority_floor_rejects_stale_sibling_owner_write() {
    stale_sibling_must_refuse(BackendProfile::St);
}

#[test]
fn proof_first_shared_authority_floor_rejects_stale_sibling_worker_write() {
    stale_sibling_must_refuse(BackendProfile::Ow1);
}

#[test]
fn proof_first_shared_authority_floor_rechecks_previously_generated_carrier() {
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        for revoke_before_delivery in [false, true] {
            let (mut authority_owner, mut sibling) = admitted_siblings(profile);
            let submission = sibling
                .submit_source_action(
                    SourceAction::owner_operation("attack").with_argument("target", "self"),
                )
                .expect("ordinary source generates a carrier before revocation");
            let carrier = sibling
                .take_outbound_process_carrier(submission.origin_locus(), submission.envelope_id())
                .expect("retain the actual generated carrier without executing it");
            if revoke_before_delivery {
                let transition = authority_owner
                    .m9_authority_lifecycle_mut()
                    .revoke_owner_capability("attack", "S")
                    .expect("genuine revocation after carrier generation");
                authority_owner
                    .apply_admitted_authority_lifecycle(transition)
                    .expect("publish the revocation");
            }
            let before = sibling.try_m8_partition_evidence().expect("actual backend");
            let result = sibling.accept_inbound_process_carrier(carrier);
            let after = sibling.try_m8_partition_evidence().expect("actual backend");
            if revoke_before_delivery {
                assert!(
                    after.changed_partitions_since(&before).is_empty(),
                    "a previously generated carrier must not write after shared revocation"
                );
                assert_eq!(
                    result
                        .expect_err("stale carrier must be refused")
                        .primary()
                        .kind(),
                    Sys4DiagnosticKind::M8ExecutionRejected
                );
                assert_eq!(
                    sibling
                        .current_m9_authority_inspection()
                        .owner_operation_validation_count("attack", "S", submission.request_id()),
                    0
                );
            } else {
                result.expect("unchanged authority admits the same carrier entry");
                assert_eq!(
                    after
                        .partition("S")
                        .expect("owner session")
                        .m8_trace_occurrence_count_for_operation(
                            "attack",
                            M8LocalTraceKind::OwnerWrite
                        ),
                    1
                );
            }
        }
    }
}

#[test]
fn proof_first_shared_authority_floor_preserves_current_repeated_owner_use() {
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        let (mut first, mut sibling) = admitted_siblings(profile);
        // Validation counters may differ between siblings; they are not authority.
        // Each real backend starts at hp100 and independently executes the source.
        for (count, expected) in [(1, 90), (2, 80)] {
            for fabric in [&mut first, &mut sibling] {
                let receipt = fabric
                    .dispatch_source_action(
                        SourceAction::owner_operation("attack").with_argument("target", "self"),
                    )
                    .expect("current authority can execute");
                let report = receipt.owner_rmw_report().expect("actual M8 owner report");
                assert!(report.has_checked_source_core_provenance());
                assert!(report.has_exact_int_write("S", "player", "self", "hp", expected));
                let actual = fabric
                    .try_m8_partition_evidence()
                    .expect("actual backend observation");
                assert_eq!(
                    actual
                        .partition("S")
                        .expect("real owner session")
                        .m8_trace_occurrence_count_for_operation(
                            "attack",
                            M8LocalTraceKind::OwnerWrite
                        ),
                    count
                );
            }
        }
    }
}

fn forced_publication_history(profile: BackendProfile, at: PauseAt) {
    let (mut publisher, mut sibling) = admitted_siblings(profile);
    // Preparing a transition is not publication. The temporary lifecycle
    // accessor is dropped at this statement, before either thread starts.
    let transition = publisher
        .m9_authority_lifecycle_mut()
        .revoke_owner_capability("attack", "S")
        .expect("prepare the genuine successor before the owner interval");
    let floor = sibling.for_test_owner_authority_floor();
    let (entered_tx, entered_rx) = mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = mpsc::sync_channel(0);
    let probe = OwnerUseProbe(Arc::new(ProbeChannels {
        at,
        entered: entered_tx,
        resume: Mutex::new(resume_rx),
        panic_on_resume: false,
        observed_write: Mutex::new(None),
    }));
    let owner = thread::spawn(move || {
        OWNER_USE_PROBE.with(|slot| *slot.borrow_mut() = Some(probe));
        let result = sibling.dispatch_source_action(
            SourceAction::owner_operation("attack").with_argument("target", "self"),
        );
        OWNER_USE_PROBE.with(|slot| *slot.borrow_mut() = None);
        (sibling, result)
    });
    entered_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("actual backend/continuation reached");
    // A direct try_lock observes exclusion, rather than inferring it from
    // lack of a response during a scheduler-dependent sleep.
    let held = matches!(floor.try_lock(), Err(TryLockError::WouldBlock));
    let (published_tx, published_rx) = mpsc::sync_channel(0);
    let publication = thread::spawn(move || {
        let result = publisher.apply_admitted_authority_lifecycle(transition);
        published_tx.send(result).expect("test controller alive");
        publisher
    });
    // If a weakened implementation released early, force the BAD ordering:
    // publication completes before allowing the real old backend effect.
    // With a retained guard, release the owner first; never wait for the
    // publisher while withholding the lock holder's continuation.
    if !held {
        published_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("unlocked publication completes")
            .expect("genuine successor installs");
    }
    resume_tx.send(()).expect("resume actual owner path");
    if held {
        published_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("publication follows owner release")
            .expect("genuine successor installs");
    }
    let _publisher = publication.join().expect("publisher thread");
    let (mut sibling, result) = owner.join().expect("owner thread");
    let receipt = result.expect("earlier protected owner effect completes");
    assert!(
        receipt
            .owner_rmw_report()
            .unwrap()
            .has_exact_int_write("S", "player", "self", "hp", 90)
    );
    let before = sibling
        .try_m8_partition_evidence()
        .expect("actual owner state");
    assert_eq!(
        before
            .partition("S")
            .unwrap()
            .m8_trace_occurrence_count_for_operation("attack", M8LocalTraceKind::OwnerWrite),
        1
    );
    let next = sibling.dispatch_source_action(
        SourceAction::owner_operation("attack").with_argument("target", "self"),
    );
    let after = sibling
        .try_m8_partition_evidence()
        .expect("actual owner after refusal");
    // Join/release all test participants before asserting against a mutant.
    assert_eq!(
        held,
        at == PauseAt::BeforeOwner,
        "the protected interval must include backend use and end before historical continuation"
    );
    assert!(
        next.is_err(),
        "later use must observe completed publication"
    );
    assert!(after.changed_partitions_since(&before).is_empty());
}

#[test]
fn proof_first_shared_authority_floor_serializes_actual_owner_interval() {
    forced_publication_history(BackendProfile::St, PauseAt::BeforeOwner);
}

#[test]
fn proof_first_shared_authority_floor_serializes_actual_worker_interval() {
    forced_publication_history(BackendProfile::Ow1, PauseAt::BeforeOwner);
}

#[test]
fn proof_first_shared_authority_floor_preserves_historical_continuation() {
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        forced_publication_history(profile, PauseAt::AfterOwner);
    }
}

#[test]
fn proof_first_shared_authority_floor_current_facts_do_not_grant_revoked_authority() {
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        let (mut publisher, _sibling) = admitted_siblings(profile);
        // Create observation-count skew before genuine publication. Counters
        // must not prevent subsequent source use or publisher synchronization.
        for _ in 0..3 {
            publisher
                .dispatch_source_action(
                    SourceAction::owner_operation("attack").with_argument("target", "self"),
                )
                .expect("current authority executes despite changed audit counts");
        }
        let transition = publisher
            .m9_authority_lifecycle_mut()
            .revoke_owner_capability("attack", "S")
            .unwrap();
        publisher
            .apply_admitted_authority_lifecycle(transition)
            .expect("publish after observation skew");
        let before = publisher.try_m8_partition_evidence().unwrap();
        let failure = publisher
            .dispatch_source_action(
                SourceAction::owner_operation("attack").with_argument("target", "self"),
            )
            .expect_err("current-but-revoked authority remains rejected by M8");
        assert_eq!(
            failure.primary().kind(),
            Sys4DiagnosticKind::M8ExecutionRejected
        );
        let after = publisher.try_m8_partition_evidence().unwrap();
        assert_eq!(
            before
                .partition("S")
                .unwrap()
                .m8_trace_occurrence_count_for_operation("attack", M8LocalTraceKind::OwnerWrite),
            3
        );
        assert_eq!(
            after
                .partition("S")
                .unwrap()
                .m8_trace_occurrence_count_for_operation("attack", M8LocalTraceKind::OwnerWrite),
            3
        );
    }
}

#[test]
fn proof_first_shared_authority_floor_poison_refuses_before_m9_and_m8_use() {
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        let (_publisher, mut sibling) = admitted_siblings(profile);
        let floor = sibling.for_test_owner_authority_floor();
        let poisoned = thread::spawn(move || {
            let _guard = floor.lock().unwrap();
            panic!("intentional test-only floor poison");
        })
        .join();
        assert!(poisoned.is_err());
        let before = sibling.try_m8_partition_evidence().unwrap();
        let failure = sibling
            .dispatch_source_action(
                SourceAction::owner_operation("attack").with_argument("target", "self"),
            )
            .expect_err("poisoned authoritative floor fails closed");
        assert_eq!(
            failure.primary().kind(),
            Sys4DiagnosticKind::M8ExecutionRejected
        );
        assert_eq!(
            sibling
                .current_m9_authority_inspection()
                .owner_operation_validation_count(
                    "attack",
                    "S",
                    failure.rejected_request_id().unwrap()
                ),
            0
        );
        let after = sibling.try_m8_partition_evidence().unwrap();
        assert!(after.changed_partitions_since(&before).is_empty());
    }
}

fn worker_panic_releases_owner_interval(at: PauseAt) {
    let (mut publisher, mut sibling) = admitted_siblings(BackendProfile::Ow1);
    let transition = publisher
        .m9_authority_lifecycle_mut()
        .revoke_owner_capability("attack", "S")
        .unwrap();
    let floor = sibling.for_test_owner_authority_floor();
    let (entered_tx, entered_rx) = mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = mpsc::sync_channel(0);
    let probe = OwnerUseProbe(Arc::new(ProbeChannels {
        at,
        entered: entered_tx,
        resume: Mutex::new(resume_rx),
        panic_on_resume: true,
        observed_write: Mutex::new(None),
    }));
    let inspection = probe.clone();
    let owner = thread::spawn(move || {
        OWNER_USE_PROBE.with(|slot| *slot.borrow_mut() = Some(probe));
        let result = sibling.dispatch_source_action(
            SourceAction::owner_operation("attack").with_argument("target", "self"),
        );
        OWNER_USE_PROBE.with(|slot| *slot.borrow_mut() = None);
        (sibling, result)
    });
    entered_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("real worker reached fault point");
    let held = matches!(floor.try_lock(), Err(TryLockError::WouldBlock));
    // This is the actual synchronous M8 serve outcome retained before the
    // worker panics, not an observer event manufactured after a lost reply.
    let written = inspection.0.observed_write.lock().unwrap().clone();
    resume_tx.send(()).unwrap();
    let (mut sibling, result) = owner
        .join()
        .expect("coordinator survives worker disconnect");
    assert!(
        held,
        "parent retains the floor while actual worker can write"
    );
    assert!(
        result.is_err(),
        "a lost worker reply is not a successful source receipt"
    );
    assert_eq!(written.is_some(), at == PauseAt::AfterWrite);
    if let Some(outcome) = written {
        assert_eq!(
            outcome.written_int(&M8StateKey::indexed_field("player", "self", "hp")),
            Some(90)
        );
    }
    // Worker unwind does not poison a guard held by the coordinator. The
    // dead worker cannot make a later write; observation must report failure.
    drop(
        floor
            .try_lock()
            .expect("coordinator released an unpoisoned floor"),
    );
    assert!(sibling.try_m8_partition_evidence().is_err());
    publisher
        .apply_admitted_authority_lifecycle(transition)
        .expect("genuine revocation can publish after worker disconnect");
    assert!(
        sibling
            .dispatch_source_action(
                SourceAction::owner_operation("attack").with_argument("target", "self")
            )
            .is_err()
    );
}

#[test]
fn proof_first_shared_authority_floor_worker_panic_before_write() {
    worker_panic_releases_owner_interval(PauseAt::BeforeOwner);
}

#[test]
fn proof_first_shared_authority_floor_worker_panic_after_write_before_reply() {
    worker_panic_releases_owner_interval(PauseAt::AfterWrite);
}
