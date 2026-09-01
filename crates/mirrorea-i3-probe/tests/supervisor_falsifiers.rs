// This integration target exercises private child-supervisor falsifiers; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

use mirrorea_i3_probe::{
    SupervisorFaultDisposition, SupervisorTestFault, TransportCandidate, run_supervisor_fault_probe,
};

#[test]
fn common_supervisor_rejects_non_loopback_ready_after_a_real_child_spawn_and_reap() {
    let outcome = run_supervisor_fault_probe(
        TransportCandidate::TlsOverTcpFramedReliableStream,
        SupervisorTestFault::EmitNonLoopbackReady,
    )
    .expect("the private supervisor falsifier probe must complete deterministically");

    assert_eq!(outcome.fault(), SupervisorTestFault::EmitNonLoopbackReady);
    assert_eq!(
        outcome.disposition(),
        SupervisorFaultDisposition::NonLoopbackReadyRejected
    );
    assert!(outcome.actual_child_spawned());
    assert!(outcome.kill_attempted());
    assert!(outcome.wait_completed());
    assert!(outcome.no_orphan_remains());
}

#[test]
fn forced_post_spawn_setup_failure_reaps_and_waits_every_actual_child() {
    let outcome = run_supervisor_fault_probe(
        TransportCandidate::TlsOverTcpFramedReliableStream,
        SupervisorTestFault::FailPostSpawnSetup,
    )
    .expect("the private supervisor falsifier probe must complete deterministically");

    assert_eq!(outcome.fault(), SupervisorTestFault::FailPostSpawnSetup);
    assert_eq!(
        outcome.disposition(),
        SupervisorFaultDisposition::PostSpawnSetupFailure
    );
    assert!(outcome.actual_child_spawned());
    assert!(outcome.kill_attempted());
    assert!(outcome.wait_completed());
    assert!(outcome.no_orphan_remains());
}

#[test]
fn forced_deadline_kills_reaps_and_waits_every_actual_child() {
    let outcome = run_supervisor_fault_probe(
        TransportCandidate::TlsOverTcpFramedReliableStream,
        SupervisorTestFault::ExpireDeadline,
    )
    .expect("the private supervisor falsifier probe must complete deterministically");

    assert_eq!(outcome.fault(), SupervisorTestFault::ExpireDeadline);
    assert_eq!(
        outcome.disposition(),
        SupervisorFaultDisposition::DeadlineExpired
    );
    assert!(outcome.actual_child_spawned());
    assert!(
        outcome.deadline_elapsed_before_cleanup(),
        "forced-timeout evidence must establish deadline expiry before kill/wait/reap"
    );
    assert!(outcome.kill_attempted());
    assert!(outcome.wait_completed());
    assert!(outcome.no_orphan_remains());
}
