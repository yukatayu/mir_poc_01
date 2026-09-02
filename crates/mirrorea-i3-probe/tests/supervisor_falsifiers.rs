// This integration target executes supervisor falsifiers outside the test
// process so a broken cleanup path cannot hang the I3-1 test runner itself.
#![allow(unused_crate_dependencies)]

#[cfg(target_os = "linux")]
use std::{
    io::{self, Read},
    process::{Child, Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;

#[cfg(target_os = "linux")]
use mirrorea_i3_probe::{
    SupervisorCleanupBreachDimension, SupervisorCleanupFailureKind, SupervisorFaultDisposition,
    SupervisorFaultProbeOutcome, SupervisorTestFault,
};

#[cfg(target_os = "linux")]
const HELPER_OUTER_DEADLINE: Duration = Duration::from_secs(5);
#[cfg(target_os = "linux")]
const HELPER_GROUP_DISAPPEARANCE_DEADLINE: Duration = Duration::from_millis(250);
// Test-owned private/provisional contract.  A typed I/O breach cannot satisfy
// this with a one-nanosecond self-report: the helper wall clock must cross the
// same bounded reaper deadline.
#[cfg(target_os = "linux")]
const EXPECTED_PRIVATE_REAPER_DEADLINE: Duration = Duration::from_millis(100);

#[cfg(target_os = "linux")]
fn helper_selector(fault: SupervisorTestFault) -> &'static str {
    match fault {
        SupervisorTestFault::EmitNonLoopbackReady => {
            "--i3-private-supervisor-fault=emit-non-loopback-ready"
        }
        SupervisorTestFault::FailPostSpawnSetup => {
            "--i3-private-supervisor-fault=fail-post-spawn-setup"
        }
        SupervisorTestFault::ExpireDeadline => "--i3-private-supervisor-fault=expire-main-deadline",
        SupervisorTestFault::DelayIoCompletionPastReaperDeadline => {
            "--i3-private-supervisor-fault=delay-io-past-reaper-deadline"
        }
    }
}

#[cfg(target_os = "linux")]
struct HelperProcessGroup {
    child: Child,
    pgid: u32,
    group_confirmed_absent: bool,
}

#[cfg(target_os = "linux")]
impl HelperProcessGroup {
    fn spawn(fault: SupervisorTestFault) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_mirrorea-i3-probe"));
        command
            .env_clear()
            .arg(helper_selector(fault))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .process_group(0);
        let child = command
            .spawn()
            .expect("the private supervisor helper binary must spawn in its own process group");
        let pgid = child.id();
        Self {
            child,
            pgid,
            group_confirmed_absent: false,
        }
    }

    fn negative_process_group(&self) -> String {
        format!("-{}", self.pgid)
    }

    fn signal_group(&self, signal: &str) -> io::Result<ExitStatus> {
        let negative_group = self.negative_process_group();
        let mut command = Command::new("kill");
        command
            .args([format!("-{signal}"), "--".to_string(), negative_group])
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        command.status()
    }

    fn group_exists(&self) -> io::Result<bool> {
        let status = self.signal_group("0")?;
        match status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            Some(code) => Err(io::Error::other(format!(
                "kill -0 -- {} returned unexpected status {code}",
                self.negative_process_group()
            ))),
            None => Err(io::Error::other("kill -0 terminated by a signal")),
        }
    }

    // The wait is intentionally unconditional after SIGKILL is attempted:
    // a kill race/failure may not skip reaping the direct helper child.
    fn kill_group_then_wait(&mut self) -> (io::Result<ExitStatus>, io::Result<ExitStatus>) {
        let kill_status = self.signal_group("KILL");
        let wait_status = self.child.wait();
        (kill_status, wait_status)
    }

    fn confirm_group_absent_before_read(&mut self) -> Result<(), String> {
        let started = Instant::now();
        loop {
            match self.group_exists() {
                Ok(false) => {
                    self.group_confirmed_absent = true;
                    return Ok(());
                }
                Ok(true) if started.elapsed() < HELPER_GROUP_DISAPPEARANCE_DEADLINE => {
                    thread::sleep(Duration::from_millis(10));
                }
                Ok(true) => {
                    let (kill_status, wait_status) = self.kill_group_then_wait();
                    return Err(format!(
                        "helper descendants survived the bounded process-group disappearance deadline; kill={kill_status:?}; wait={wait_status:?}"
                    ));
                }
                Err(error) => {
                    let (kill_status, wait_status) = self.kill_group_then_wait();
                    return Err(format!(
                        "could not verify helper process-group absence; probe={error}; kill={kill_status:?}; wait={wait_status:?}"
                    ));
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for HelperProcessGroup {
    fn drop(&mut self) {
        if !self.group_confirmed_absent {
            // Best effort for assertion panics and unexpected I/O errors. The
            // normal path establishes absence explicitly before reading stdout.
            let _ = self.kill_group_then_wait();
        }
    }
}

#[cfg(target_os = "linux")]
fn spawn_helper_in_own_process_group(fault: SupervisorTestFault) -> HelperProcessGroup {
    HelperProcessGroup::spawn(fault)
}

#[cfg(target_os = "linux")]
fn run_fault_in_hard_bounded_helper(
    fault: SupervisorTestFault,
) -> (SupervisorFaultProbeOutcome, Duration) {
    let mut helper = spawn_helper_in_own_process_group(fault);
    let started = Instant::now();
    let status = loop {
        match helper.child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < HELPER_OUTER_DEADLINE => {
                thread::sleep(Duration::from_millis(10));
            }
            Ok(None) => {
                let (kill_status, wait_status) = helper.kill_group_then_wait();
                let absence = helper.confirm_group_absent_before_read();
                panic!(
                    "private supervisor helper exceeded the hard outer deadline; kill={kill_status:?}; wait={wait_status:?}; group_absence={absence:?}"
                )
            }
            Err(error) => {
                let (kill_status, wait_status) = helper.kill_group_then_wait();
                panic!(
                    "private supervisor helper status became unobservable; error={error}; kill={kill_status:?}; wait={wait_status:?}"
                )
            }
        }
    };
    let waited_status = helper.child.wait().unwrap_or_else(|error| {
        let (kill_status, retry_wait_status) = helper.kill_group_then_wait();
        panic!(
            "the exited helper could not be waited; error={error}; kill={kill_status:?}; retry_wait={retry_wait_status:?}"
        )
    });
    assert_eq!(
        status, waited_status,
        "the post-exit helper wait must complete with the same status observed by the poll"
    );
    helper
        .confirm_group_absent_before_read()
        .unwrap_or_else(|failure| panic!("{failure}"));
    assert!(
        status.success(),
        "a typed supervisor outcome must be emitted before helper exit"
    );
    let mut output = Vec::new();
    helper
        .child
        .stdout
        .take()
        .expect("the helper retains its private typed-outcome stdout")
        .read_to_end(&mut output)
        .expect("the exited helper stdout remains readable");
    (
        serde_json::from_slice::<SupervisorFaultProbeOutcome>(&output)
            .expect("the helper stdout must be exactly one typed supervisor outcome"),
        started.elapsed(),
    )
}

#[cfg(target_os = "linux")]
fn assert_no_orphan_claim_is_derived(outcome: &SupervisorFaultProbeOutcome) {
    assert_eq!(
        outcome.no_orphan_claim(),
        outcome.children_reaped_before_reaper_deadline()
            && outcome.io_threads_completed_before_reaper_deadline()
            && outcome.cleanup_failure().is_none(),
        "no-orphan claim must be derived from factual child reap, I/O completion, and no cleanup failure"
    );
}

#[cfg(target_os = "linux")]
fn assert_normal_cleanup_complete(outcome: &SupervisorFaultProbeOutcome) {
    // `no_orphan_remains` is only the factual child-reap observation. Normal
    // cleanup completion is the stronger derived claim and must also retain
    // all reader/stderr/writer completion facts before it can be reported.
    assert!(outcome.children_reaped_before_reaper_deadline());
    assert!(outcome.io_threads_completed_before_reaper_deadline());
    assert_eq!(outcome.breached_cleanup_dimension(), None);
    assert_eq!(outcome.cleanup_failure(), None);
    assert_no_orphan_claim_is_derived(outcome);
    assert!(outcome.no_orphan_claim());
}

#[cfg(target_os = "linux")]
#[test]
fn normal_forced_faults_prove_main_deadline_then_distinct_reaper_deadline_completion() {
    for (fault, disposition, main_deadline_elapsed) in [
        (
            SupervisorTestFault::EmitNonLoopbackReady,
            SupervisorFaultDisposition::NonLoopbackReadyRejected,
            false,
        ),
        (
            SupervisorTestFault::FailPostSpawnSetup,
            SupervisorFaultDisposition::PostSpawnSetupFailure,
            false,
        ),
        (
            SupervisorTestFault::ExpireDeadline,
            SupervisorFaultDisposition::DeadlineExpired,
            true,
        ),
    ] {
        let (outcome, wall_elapsed) = run_fault_in_hard_bounded_helper(fault);

        assert_eq!(outcome.fault(), fault);
        assert_eq!(outcome.disposition(), disposition);
        assert_eq!(
            outcome.deadline_elapsed_before_cleanup(),
            main_deadline_elapsed,
            "the main deadline evidence must remain distinct from reaper completion"
        );
        assert!(outcome.actual_child_spawned());
        assert!(outcome.kill_attempted());
        assert!(outcome.wait_completed());
        assert!(outcome.reaper_deadline_enforced());
        assert!(outcome.no_orphan_remains());
        assert!(!outcome.reaper_deadline_ref().is_empty());
        assert_eq!(
            outcome.reaper_deadline_duration(),
            EXPECTED_PRIVATE_REAPER_DEADLINE,
            "the private reaper deadline is an exact bounded contract, not a self-reported arbitrary duration"
        );
        assert!(outcome.cleanup_elapsed() <= wall_elapsed);
        assert!(
            outcome.main_deadline_duration() + outcome.reaper_deadline_duration()
                < HELPER_OUTER_DEADLINE - Duration::from_secs(1),
            "the hard helper deadline must retain a clear outer margin beyond inner main and reaper deadlines"
        );
        assert_normal_cleanup_complete(&outcome);
    }
}

#[cfg(target_os = "linux")]
#[test]
fn synthetic_io_completion_breach_is_typed_and_never_becomes_a_no_orphan_claim() {
    let (outcome, wall_elapsed) =
        run_fault_in_hard_bounded_helper(SupervisorTestFault::DelayIoCompletionPastReaperDeadline);

    assert_eq!(
        outcome.fault(),
        SupervisorTestFault::DelayIoCompletionPastReaperDeadline
    );
    assert_eq!(
        outcome.disposition(),
        SupervisorFaultDisposition::ReaperDeadlineExceeded
    );
    assert!(outcome.actual_child_spawned());
    assert!(outcome.kill_attempted());
    assert!(outcome.wait_completed());
    assert!(outcome.reaper_deadline_enforced());
    assert_eq!(
        outcome.breached_cleanup_dimension(),
        Some(SupervisorCleanupBreachDimension::IoThreadCompletion),
        "the synthetic fault is an actual pipe-reader/writer completion delay, not an immediate self-reported failure"
    );
    assert!(!outcome.reaper_deadline_ref().is_empty());
    assert_eq!(
        outcome.reaper_deadline_duration(),
        EXPECTED_PRIVATE_REAPER_DEADLINE,
        "the actual I/O latch delay crosses the fixed private reaper deadline, not an arbitrary reported value"
    );
    assert!(
        outcome.cleanup_elapsed() >= EXPECTED_PRIVATE_REAPER_DEADLINE,
        "the measured cleanup time must actually cross the inner reaper deadline"
    );
    assert!(
        wall_elapsed >= EXPECTED_PRIVATE_REAPER_DEADLINE && wall_elapsed < HELPER_OUTER_DEADLINE,
        "the helper wall clock crosses the inner reaper deadline but remains bounded by the hard outer deadline"
    );
    assert!(
        outcome.main_deadline_duration() + outcome.reaper_deadline_duration()
            < HELPER_OUTER_DEADLINE - Duration::from_secs(1),
        "the hard outer deadline must retain clear margin over inner main plus reaper deadlines"
    );
    assert!(
        outcome.children_reaped_before_reaper_deadline(),
        "this synthetic residual names an I/O completion breach, not an invented child-reap failure"
    );
    assert!(
        !outcome.io_threads_completed_before_reaper_deadline(),
        "the synthetic delay must cross the distinct reaper deadline without hanging the outer test process"
    );
    assert!(
        outcome.no_orphan_remains(),
        "factual child reap may remain true even though the stronger no-orphan claim is unavailable"
    );
    assert_eq!(
        outcome.cleanup_failure(),
        Some(SupervisorCleanupFailureKind::ReaperDeadlineExceeded),
        "a reaper-deadline breach is a typed cleanup residual"
    );
    assert_no_orphan_claim_is_derived(&outcome);
    assert!(!outcome.no_orphan_claim());
}
