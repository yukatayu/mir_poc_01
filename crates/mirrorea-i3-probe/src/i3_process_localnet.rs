//! Private I3-2 two-process QUIC execution slice.
//!
//! This module is intentionally isolated from the I3-0 candidate harness and
//! the static-adapter spike.  It composes the checked I2 project/image path
//! with two actual `exec` children, a tainted image descriptor, a distinct
//! inherited trusted Unix control descriptor, mutually authenticated QUIC,
//! and the runtime's authenticated ingress gate.  None of the names below is
//! a public process, wire, certificate, deployment, or compatibility ABI.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, UdpSocket},
    os::{
        fd::{AsRawFd, FromRawFd, RawFd},
        unix::{net::UnixStream, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    sync::Arc,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use mir_runtime::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3InstalledProviderChildRuntime,
        Sys5I3PreparedProviderLocalnetLaunch, Sys5I3PrivateProcessCodec, Sys5I3ProcessCohort,
        Sys5I3ProviderChildRole, Sys5I3ProviderLaunchError, Sys5I3ProviderLaunchErrorKind,
        Sys5I3ProviderTerminalOutcomeClass, Sys5I3TrustedProviderLocalnetControl,
        Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
    },
    sys5_local_slice::{Sys5I3AdapterCarrierContract, Sys5SourceInput, build_project},
};
use quinn::{
    Endpoint,
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
};
use rcgen::{
    BasicConstraints, CertificateParams, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose, PublicKeyData,
};
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    server::WebPkiClientVerifier,
};
use serde::{
    Deserialize, Serialize,
    de::{Error as _, MapAccess, Visitor},
};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use super::i3_process_faults::{
    I3LocalnetAdapterDeliveryFailure, I3LocalnetAdapterDeliveryProfile, I3LocalnetFaultAudit,
    I3LocalnetFaultAuditFalsifier, I3LocalnetFaultProfile, I3LocalnetLateIngressAckReaderOutcome,
    I3LocalnetLateIngressEvidenceRejection, I3LocalnetLateIngressFalsifier,
    I3LocalnetLateIngressLifecycleProvenance,
    I3LocalnetLateIngressNonregisteredAckInputDisposition, I3LocalnetLateIngressOwnerOutcome,
    I3LocalnetLateIngressParentPublication, I3LocalnetLateIngressProfile,
    I3LocalnetLateIngressRequesterOutcome, I3LocalnetOwnerReplyReplayAudit,
    I3LocalnetOwnerReplyReplayChildAudit, I3LocalnetOwnerReplyReplayFalsifier,
    I3LocalnetOwnerReplyReplayFirstOutcome, I3LocalnetOwnerReplyReplayOutcome,
    I3LocalnetOwnerReplyReplayOwnerState, I3LocalnetOwnerReplyReplayProfile,
    I3LocalnetOwnerReplyReplayReceiverRejection, I3LocalnetOwnerReplyReplayRequesterState,
    I3LocalnetReconnectOwnerOutcome, I3LocalnetRemoteAdmissionEvidence,
    I3LocalnetRemoteEvidenceRejection, I3LocalnetRequesterFaultObservation,
    I3LocalnetRequesterLocalWaitAudit, I3LocalnetRequesterLocalWaitFalsifier,
    I3LocalnetRequesterLocalWaitProfile, I3LocalnetRetryAttemptAudit, I3LocalnetRetryAttemptReason,
    I3LocalnetRetryAudit, I3LocalnetRetryAuditFalsifier, I3LocalnetRetryChildAudit,
    I3LocalnetRetryChildAuditEvidence, I3LocalnetRetryEvidenceRejection, I3LocalnetRetryFalsifier,
    I3LocalnetRetryProfile, I3LocalnetRetryRequesterOutcome,
};

const PROCESS_A_SLOT: &str = "process-a";
const PROCESS_B_SLOT: &str = "process-b";
const PROCESS_A_LOCI: [&str; 2] = ["ParticipantA", "ViewerC"];
const PROCESS_B_LOCI: [&str; 2] = ["WorldAuthority", "ParticipantB"];
const ACTIVE_I2_LOGICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const OWNER_ADMISSION_LOGICAL_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-owner-admission/main.mir";
const REQUESTER_LOCAL_WAIT_MINIMUM: Duration = Duration::from_millis(20);
const LOCALNET_CONTROL_FD: i32 = 3;
// Bounded, private B-to-parent lifecycle completion route. It is never
// multiplexed with child stdout observer events or the trusted bootstrap FD.
const LOCALNET_OWNER_LIFECYCLE_ACK_FD: i32 = 4;
const MAX_TRUSTED_CONTROL_BYTES: usize = 512 * 1024;
// The runtime independently rejects a larger provider transport blob before
// it writes the one inherited FD3 frame. Keeping the same private bound here
// prevents the probe from serializing an oversized TLS/slot record first.
const MAX_PROVIDER_TRANSPORT_BOOTSTRAP_BYTES: usize = 64 * 1024;
const MAX_CHILD_EVENT_BYTES: usize = 64 * 1024;
const PRIVATE_LOCALNET_ALPN: &[u8] = b"mirrorea-i3-process-localnet-v1";
const FIXED_PROVIDER_TERMINAL_AUDIT_PROFILE_V2: &str = "fixed-provider-terminal-audit-v2";
const FIXED_PROVIDER_NETWORK_PROVENANCE_SCHEMA_V2: &str =
    "fixed-provider-terminal-audit-network-provenance-v2";
// The reaper reserve is part of the finite lifecycle allowance: it leaves
// room to observe a post-kill exit without exceeding the caller's total main
// deadline plus reaper allowance under suite load.
const LIFECYCLE_REAP_RESERVE: Duration = Duration::from_millis(100);
// This bounds only the selected unavailable-endpoint connect attempt. It is
// deliberately below the enclosing child deadline and is not a semantic
// lease, retry, or reaper timeout.
const ENDPOINT_UNAVAILABLE_CONNECT_BUDGET: Duration = Duration::from_millis(250);

/// The fixed, provisional deployment grouping for this finite profile.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetChildSlot {
    ProcessA,
    ProcessB,
}

impl I3LocalnetChildSlot {
    fn slot_name(self) -> &'static str {
        match self {
            Self::ProcessA => PROCESS_A_SLOT,
            Self::ProcessB => PROCESS_B_SLOT,
        }
    }

    fn assigned_loci(self) -> [&'static str; 2] {
        match self {
            Self::ProcessA => PROCESS_A_LOCI,
            Self::ProcessB => PROCESS_B_LOCI,
        }
    }

    fn child_arg(self) -> &'static str {
        match self {
            Self::ProcessA => "--i3-2-private-child-slot=process-a",
            Self::ProcessB => "--i3-2-private-child-slot=process-b",
        }
    }

    fn provider_child_arg(self) -> &'static str {
        match self {
            Self::ProcessA => "--i3-3-private-provider-child-slot=process-a",
            Self::ProcessB => "--i3-3-private-provider-child-slot=process-b",
        }
    }

    const fn provider_role(self) -> Sys5I3ProviderChildRole {
        match self {
            Self::ProcessA => Sys5I3ProviderChildRole::RequesterConsumer,
            Self::ProcessB => Sys5I3ProviderChildRole::Executor,
        }
    }
}

/// Physical whole-run limits for the private source-derived provider launch.
///
/// This carries neither source, fixture/path, effect authority, image/control
/// bytes, nor a semantic result. The opaque runtime-prepared launch supplies
/// the only admitted provider material. These limits are operational
/// supervisor inputs, not semantic leases or a public process contract.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct I3ReadOnlyProviderLocalnetRequest {
    whole_run_deadline: Duration,
    reaper_allowance: Duration,
    // A finite physical lifecycle falsifier. It is transferred only in A's
    // private provider transport bootstrap after source-derived preparation;
    // it cannot select a fixture, semantic result, authority, or carrier.
    requester_exit_nonzero_after_completed: bool,
}

impl I3ReadOnlyProviderLocalnetRequest {
    /// The selected finite profile's largest permitted whole-run bounds.
    pub const fn bounded_defaults() -> Self {
        Self {
            whole_run_deadline: Duration::from_secs(15),
            reaper_allowance: Duration::from_secs(1),
            requester_exit_nonzero_after_completed: false,
        }
    }

    /// Changes only the supervisor's physical whole-run deadline.
    pub const fn with_whole_run_deadline(mut self, whole_run_deadline: Duration) -> Self {
        self.whole_run_deadline = whole_run_deadline;
        self
    }

    /// Changes only the finite allowance reserved for forced reaping.
    pub const fn with_reaper_allowance(mut self, reaper_allowance: Duration) -> Self {
        self.reaper_allowance = reaper_allowance;
        self
    }

    /// Selects the finite physical falsifier where A exits nonzero only after
    /// its real consume, private fixture assertion, terminal audit emission,
    /// and QUIC close/drain have all completed. This is not a source,
    /// authority, result, or fault-outcome input.
    #[doc(hidden)]
    pub const fn with_requester_exit_nonzero_after_completed(mut self) -> Self {
        self.requester_exit_nonzero_after_completed = true;
        self
    }
}

/// Observer-safe bounded supervision failures for the private provider
/// runner. A rejected run does not imply that no provider invocation occurred;
/// its stage is operational evidence only.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3ReadOnlyProviderLocalnetRunErrorKind {
    InvalidWholeRunDeadline,
    InvalidReaperAllowance,
    ProviderLaunchRejected,
    /// The runtime rejected the source/composite activation boundary before a
    /// provider child could enter its checked local path. The supervisor has
    /// reaped any already registered child before returning.
    ProviderRuntimeActivationPending,
}

/// Bounded, observer-safe progress categories for the private provider
/// runner. A category identifies only the supervisor/child boundary which
/// rejected; it does not expose an OS error, endpoint, source value, control,
/// authority material, witness, or provider result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3ReadOnlyProviderLocalnetFailureStage {
    Preflight,
    CredentialProvision,
    ExecutorBootstrap,
    ExecutorReady,
    RequesterBootstrap,
    ExecutorTerminal,
    RequesterTerminal,
    TerminalObservationCorrelation,
    NaturalReap,
    EndpointRebind,
}

/// Fixed physical bootstrap boundary for an observer-safe provider-launch
/// rejection. This is diagnostic evidence only: it neither authenticates a
/// child nor grants authority, and it retains no operating-system error or
/// private bootstrap material.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3ReadOnlyProviderLocalnetBootstrapSubstage {
    RouteConsistency,
    ExecutableResolve,
    ControlPipeSetup,
    SpawnRegistration,
    ImageHandoff,
    ControlHandoff,
}

/// A typed private provider-run failure with no fixture, authority, carrier,
/// control, or result material.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct I3ReadOnlyProviderLocalnetRunError {
    kind: I3ReadOnlyProviderLocalnetRunErrorKind,
    stage: I3ReadOnlyProviderLocalnetFailureStage,
    bootstrap_substage: Option<I3ReadOnlyProviderLocalnetBootstrapSubstage>,
    provider_launch_error_kind: Option<Sys5I3ProviderLaunchErrorKind>,
}

impl I3ReadOnlyProviderLocalnetRunError {
    const fn new(kind: I3ReadOnlyProviderLocalnetRunErrorKind) -> Self {
        Self::at(kind, I3ReadOnlyProviderLocalnetFailureStage::Preflight)
    }

    const fn at(
        kind: I3ReadOnlyProviderLocalnetRunErrorKind,
        stage: I3ReadOnlyProviderLocalnetFailureStage,
    ) -> Self {
        Self {
            kind,
            stage,
            bootstrap_substage: None,
            provider_launch_error_kind: None,
        }
    }

    const fn bootstrap_rejection(
        kind: I3ReadOnlyProviderLocalnetRunErrorKind,
        stage: I3ReadOnlyProviderLocalnetFailureStage,
        bootstrap_substage: Option<I3ReadOnlyProviderLocalnetBootstrapSubstage>,
        provider_launch_error_kind: Option<Sys5I3ProviderLaunchErrorKind>,
    ) -> Self {
        Self {
            kind,
            stage,
            bootstrap_substage,
            provider_launch_error_kind,
        }
    }

    pub const fn kind(&self) -> I3ReadOnlyProviderLocalnetRunErrorKind {
        self.kind
    }

    pub const fn stage(&self) -> I3ReadOnlyProviderLocalnetFailureStage {
        self.stage
    }

    /// The fixed physical bootstrap boundary, when the rejection occurred
    /// during parent-side child setup. It exposes no OS error or child data.
    pub const fn bootstrap_substage(&self) -> Option<I3ReadOnlyProviderLocalnetBootstrapSubstage> {
        self.bootstrap_substage
    }

    /// The existing typed runtime launch category, when the runtime rejected
    /// a privileged image/control operation. This is not a carrier, result,
    /// source, or authority diagnostic.
    pub const fn provider_launch_error_kind(&self) -> Option<Sys5I3ProviderLaunchErrorKind> {
        self.provider_launch_error_kind
    }
}

/// One source-real two-process provider run. The terminal records are only
/// strict decoded observer-view candidates which the supervisor correlated
/// with its own children and physical run; they are neither M9 permits nor
/// reconstructible issued audits.
#[doc(hidden)]
pub struct I3ReadOnlyProviderLocalnetRun {
    requester_terminal_observer_view: Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
    executor_terminal_observer_view: Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
}

impl I3ReadOnlyProviderLocalnetRun {
    /// A's bounded, reference-only terminal observation candidate.
    pub fn requester_terminal_observer_view(
        &self,
    ) -> &Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
        &self.requester_terminal_observer_view
    }

    /// B's bounded, reference-only terminal observation candidate.
    pub fn executor_terminal_observer_view(
        &self,
    ) -> &Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
        &self.executor_terminal_observer_view
    }
}

/// Opaque completion of one sealed fixed terminal-observation conformance
/// route. It carries no profile, M9 diagnostic, audit, provider result,
/// reference, count, or non-execution claim.
#[doc(hidden)]
pub struct I3ReadOnlyProviderTerminalObservationConformanceRun {
    _private: (),
}

/// Opaque completion of one sealed fixed provider-network conformance route.
/// It carries no selected fault profile, terminal audit, result, provenance
/// reference, count, or reason for any denied transition.
#[doc(hidden)]
pub struct I3ReadOnlyProviderNetworkConformanceRun {
    _private: (),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrivateProviderTerminalExpectation {
    NormalAudit,
    FixedTerminalObservationConformance,
    FixedProviderNetworkConformance,
}

enum PrivateProviderTerminalCompletion {
    NormalAudit(Box<I3ReadOnlyProviderLocalnetRun>),
    FixedTerminalObservationConformance(I3ReadOnlyProviderTerminalObservationConformanceRun),
    FixedProviderNetworkConformance(I3ReadOnlyProviderNetworkConformanceRun),
}

/// Deliberately limited test-facing fault switches.  They model one finite
/// binding swap and one delivery-origin injection; neither adds retry,
/// reconnect, routing, authority, or an expected semantic result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetFalsifier {
    SwapImageAndBindingPairs,
    InjectUnauthenticatedReply,
    DeliverReplyFromCaSignedWrongSpkiPeer,
    StallImageOrControlBootstrap,
    StallCleanup,
    CompletedThenNonzero,
    CompletedThenHang,
    /// Test-only supervisor-observation falsifier.  It leaves both child
    /// reports and their natural exits intact, then consumes the already
    /// configured lifecycle interval before the supervisor records reaps.
    DelaySupervisorExitObservationPastDeadline,
    SetupFailureDuringStallMode,
    AsymmetricCompletedAndRejected,
}

/// Exact private adapter rejection category retained by the process supervisor.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetAdapterRejectionKind {
    PeerBindingRejected,
    LocalAttemptRejected,
}

/// Concrete lifecycle cause; a selected test mode cannot rewrite this cause.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetLifecycleRejectionCause {
    /// The finite force-reap reserve is part of the private launcher input,
    /// not a timeout observed after any child exists.
    InvalidReaperAllowance,
    SetupOrControlFailure,
    CompletedChildExitedNonzero,
    /// A real B child retained one source-budgeted request Awaiting a trusted
    /// owner-runtime driver. This is not an owner result or a fabricated
    /// failure reply.
    OwnerAdmissionDriveRequired,
}

/// The finite child report outcome retained even when the aggregate run fails.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetChildTerminalOutcome {
    Completed,
    Rejected,
    HandledDeliveryFault,
    /// A verified B session admitted a source-budgeted request, whose owner
    /// runtime retained it Awaiting a later trusted clock/driver decision.
    OwnerAdmissionAwaiting,
}

/// Observer-safe counters from one terminal child report.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetChildTerminalEvent {
    slot: Option<I3LocalnetChildSlot>,
    outcome: I3LocalnetChildTerminalOutcome,
    semantic_admission_count: usize,
    owner_mutation_count: usize,
    owner_serve_count: Option<usize>,
    trusted_control_consumed: Option<bool>,
    unauthenticated_semantic_admission_count: Option<usize>,
    tls_peer_verified: Option<bool>,
    reciprocal_preface_verified: Option<bool>,
    owner_admission_awaiting_count: Option<usize>,
    observed_exit_status_code: Option<i32>,
    was_force_killed: bool,
}

impl I3LocalnetChildTerminalEvent {
    /// The actual child slot when the terminal format records one. Generic
    /// lifecycle rejections make no slot claim.
    pub const fn slot(&self) -> Option<I3LocalnetChildSlot> {
        self.slot
    }

    pub const fn outcome(&self) -> I3LocalnetChildTerminalOutcome {
        self.outcome
    }

    pub const fn semantic_admission_count(&self) -> usize {
        self.semantic_admission_count
    }

    pub const fn owner_mutation_count(&self) -> usize {
        self.owner_mutation_count
    }

    /// Present only when the terminal carries an actual owner runtime
    /// observation. In particular, requester and generic rejection records
    /// cannot manufacture a zero owner-serve count.
    pub const fn owner_serve_count(&self) -> Option<usize> {
        self.owner_serve_count
    }

    /// The child-reported one-shot control consumption when that terminal
    /// format carries the observation. This is never synthesized for a
    /// generic rejection or fault record.
    pub const fn trusted_control_consumed(&self) -> Option<bool> {
        self.trusted_control_consumed
    }

    /// The child-reported unauthenticated semantic admission count when the
    /// terminal format carries it. It is not a global aggregate.
    pub const fn unauthenticated_semantic_admission_count(&self) -> Option<usize> {
        self.unauthenticated_semantic_admission_count
    }

    /// Actual child-local TLS peer verification where the terminal format
    /// records it. A generic rejection remains unknown.
    pub const fn tls_peer_verified(&self) -> Option<bool> {
        self.tls_peer_verified
    }

    /// Actual child-local reciprocal preface verification where the terminal
    /// format records it. A generic rejection remains unknown.
    pub const fn reciprocal_preface_verified(&self) -> Option<bool> {
        self.reciprocal_preface_verified
    }

    /// Present only when B itself reported its bounded owner-admission
    /// disposition. Generic lifecycle failures deliberately carry no zero
    /// substitute for this observation.
    pub const fn owner_admission_awaiting_count(&self) -> Option<usize> {
        self.owner_admission_awaiting_count
    }

    /// The supervisor-observed OS exit code, not a semantic outcome.
    pub const fn observed_exit_status_code(&self) -> Option<i32> {
        self.observed_exit_status_code
    }

    /// Whether this PID required the bounded external kill/reap path.
    pub const fn was_force_killed(&self) -> bool {
        self.was_force_killed
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetControlDelivery {
    DedicatedOneShotTrustedFd,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetImageDelivery {
    DedicatedTaintedImageFd,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetFailureStage {
    BeforeOwnerStart,
    BeforeSemanticAdmission,
    /// The requester observed a missing reply or receipt. This observation
    /// alone makes no claim about remote admission.
    RequesterReplyOrReceiptNotObserved,
    /// Set only after a validated owner admission record is joined.
    AfterRemoteAdmission,
    /// The supervisor joined B's actual retained declared-expiry evidence,
    /// while A still retained the original request after a lost reply.
    AfterRemoteDeclaredOwnerExpiry,
    /// A received and consumed a gate-produced declared owner failure. This
    /// is a semantic terminal command result, not a lifecycle or transport
    /// failure stage.
    RequesterTerminalFailureConsumed,
    /// A first generated reply was already consumed by A. The later T0
    /// replay action then reached its selected receiver rejection or local
    /// pre-write refusal; it is neither an ambiguous delivery nor lifecycle
    /// setup failure.
    AfterKnownRequesterDecision,
    LifecycleEvidenceRejected,
    BootstrapDeadline,
    CleanupDeadline,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetRunErrorKind {
    StartBindingRejected,
    PeerBindingRejected,
    SourceBuildRejected,
    LifecycleRejected,
    LifecycleDeadlineExceeded,
    DeliveryUnavailable,
    AmbiguousDelivery,
    /// The generated declared owner failure reached and was consumed by the
    /// requester. The enclosing `Err` preserves the existing result shape;
    /// it is not a lifecycle or transport failure.
    TerminalFailureConsumed,
    /// One known generated reply was consumed first, then the bounded T0
    /// replay action was rejected by the existing adapter/runtime path.
    OwnerReplyReplayRejected,
}

/// One neutral, fixed owner-runtime schedule used only after B has actually
/// retained a budgeted request Awaiting. It supplies neither a raw tick nor
/// an expected semantic outcome: the runtime's checked gate still determines
/// whether the returned message serves or declares expiry.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetOwnerAdmissionDriveProfile {
    AdvanceOneTickAfterAwaiting,
}

/// Ordinary source input for one bounded localnet run.  The supervisor never
/// accepts images, precomputed carrier bytes, deployment-selected operation,
/// or a semantic expected result from this public test seam.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3ProcessLocalnetRequest {
    ordinary_source_path: PathBuf,
    deadline: Duration,
    reaper_allowance: Duration,
    falsifier: Option<I3LocalnetFalsifier>,
    fault_profile: Option<I3LocalnetFaultProfile>,
    fault_audit_falsifier: Option<I3LocalnetFaultAuditFalsifier>,
    requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
    requester_local_wait_falsifier: Option<I3LocalnetRequesterLocalWaitFalsifier>,
    retry_profile: Option<I3LocalnetRetryProfile>,
    retry_falsifier: Option<I3LocalnetRetryFalsifier>,
    retry_audit_falsifier: Option<I3LocalnetRetryAuditFalsifier>,
    owner_admission_drive_profile: Option<I3LocalnetOwnerAdmissionDriveProfile>,
    owner_reply_replay_profile: Option<I3LocalnetOwnerReplyReplayProfile>,
    owner_reply_replay_falsifier: Option<I3LocalnetOwnerReplyReplayFalsifier>,
    late_ingress_profile: Option<I3LocalnetLateIngressProfile>,
    late_ingress_falsifier: Option<I3LocalnetLateIngressFalsifier>,
    // Selects only a concrete bounded adapter action after source generation.
    // No endpoint, carrier, authority, or expected result comes from this
    // control value.
    adapter_delivery_profile: Option<I3LocalnetAdapterDeliveryProfile>,
}

impl I3ProcessLocalnetRequest {
    pub fn from_ordinary_source_path(path: impl Into<PathBuf>) -> Self {
        Self {
            ordinary_source_path: path.into(),
            deadline: Duration::from_secs(15),
            reaper_allowance: Duration::from_secs(1),
            falsifier: None,
            fault_profile: None,
            fault_audit_falsifier: None,
            requester_local_wait_profile: None,
            requester_local_wait_falsifier: None,
            retry_profile: None,
            retry_falsifier: None,
            retry_audit_falsifier: None,
            owner_admission_drive_profile: None,
            owner_reply_replay_profile: None,
            owner_reply_replay_falsifier: None,
            late_ingress_profile: None,
            late_ingress_falsifier: None,
            adapter_delivery_profile: None,
        }
    }

    pub fn with_deadline(mut self, deadline: Duration) -> Self {
        self.deadline = deadline;
        self
    }

    /// Finite post-report allowance for observing natural child exit before a
    /// failure cleanup may force-kill an unreaped child.
    pub fn with_reaper_allowance(mut self, reaper_allowance: Duration) -> Self {
        self.reaper_allowance = reaper_allowance;
        self
    }

    /// Minimum private reserve retained from the caller's reaper allowance
    /// for observing an exit after a forced kill.  It is deliberately finite
    /// and provisional; it is neither a public process contract nor a
    /// promise about arbitrary host scheduling.
    pub const fn minimum_force_reap_reserve() -> Duration {
        LIFECYCLE_REAP_RESERVE
    }

    pub fn with_falsifier(mut self, falsifier: I3LocalnetFalsifier) -> Self {
        self.falsifier = Some(falsifier);
        self
    }

    /// Select one finite actual-delivery schedule after ordinary source has
    /// been built and admitted.  This profile has no carrier, expected result,
    /// authority, retry, or session identity input.
    pub fn with_fault_profile(mut self, profile: I3LocalnetFaultProfile) -> Self {
        self.fault_profile = Some(profile);
        self
    }

    /// Select one private post-admission observer-record falsifier.  It is
    /// accepted only with the matching delivery-fault profile and never
    /// reaches source, carrier, owner-dispatch, or mutation handling.
    pub fn with_fault_audit_falsifier(mut self, falsifier: I3LocalnetFaultAuditFalsifier) -> Self {
        self.fault_audit_falsifier = Some(falsifier);
        self
    }

    /// Select the one fixed requester-local monotonic wait after A observed
    /// an actual lost reply. It carries no duration, retry, session, carrier,
    /// or semantic outcome input.
    pub fn with_requester_local_wait_profile(
        mut self,
        profile: I3LocalnetRequesterLocalWaitProfile,
    ) -> Self {
        self.requester_local_wait_profile = Some(profile);
        self
    }

    /// Select one observer-only corruption after the real requester-local
    /// wait has completed. It cannot alter the request, transport, owner, or
    /// requester runtime state.
    pub fn with_requester_local_wait_falsifier(
        mut self,
        falsifier: I3LocalnetRequesterLocalWaitFalsifier,
    ) -> Self {
        self.requester_local_wait_falsifier = Some(falsifier);
        self
    }

    /// Select one bounded two-session use of the runtime-retained original
    /// request.  This is probe scheduling only, never a caller retry policy.
    pub fn with_retry_profile(mut self, profile: I3LocalnetRetryProfile) -> Self {
        self.retry_profile = Some(profile);
        self
    }

    /// Select a negative-only attempt made on the fully verified first
    /// session.  It cannot add a reconnect session or carrier write.
    pub fn with_retry_falsifier(mut self, falsifier: I3LocalnetRetryFalsifier) -> Self {
        self.retry_falsifier = Some(falsifier);
        self
    }

    /// Select one post-send observer-only reconnect-evidence falsifier.
    pub fn with_retry_audit_falsifier(mut self, falsifier: I3LocalnetRetryAuditFalsifier) -> Self {
        self.retry_audit_falsifier = Some(falsifier);
        self
    }

    /// Select one fixed owner-runtime scheduling action after B has observed
    /// an actual Awaiting admission. The profile cannot provide a tick,
    /// carrier, authority, source operation, or expected result.
    pub fn with_owner_admission_drive_profile(
        mut self,
        profile: I3LocalnetOwnerAdmissionDriveProfile,
    ) -> Self {
        self.owner_admission_drive_profile = Some(profile);
        self
    }

    /// Select the one T0 replay of B's actual source-generated reply after A
    /// has consumed its first known result. This is probe scheduling only;
    /// it accepts neither a reply body nor a semantic retry/result input.
    pub fn with_owner_reply_replay_profile(
        mut self,
        profile: I3LocalnetOwnerReplyReplayProfile,
    ) -> Self {
        self.owner_reply_replay_profile = Some(profile);
        self
    }

    /// Select the negative-only use of the opaque B replay token on its
    /// original session. The adapter rejects before another write or replay
    /// occurrence can exist.
    pub fn with_owner_reply_replay_falsifier(
        mut self,
        falsifier: I3LocalnetOwnerReplyReplayFalsifier,
    ) -> Self {
        self.owner_reply_replay_falsifier = Some(falsifier);
        self
    }

    /// Select one bounded route that retains a complete checked session-one
    /// frame opaquely until session two.  It is neither a resend nor a source
    /// operation and cannot supply authority or an expected result.
    pub fn with_late_ingress_profile(mut self, profile: I3LocalnetLateIngressProfile) -> Self {
        self.late_ingress_profile = Some(profile);
        self
    }

    /// Select one probe-owned negative for the genuine pre-staged lifecycle
    /// ACK route.  The control contains no candidate, receipt, ACK, owner
    /// label, publisher, or raw record.
    pub fn with_late_ingress_falsifier(
        mut self,
        falsifier: I3LocalnetLateIngressFalsifier,
    ) -> Self {
        self.late_ingress_falsifier = Some(falsifier);
        self
    }

    /// Select one bounded actual adapter-delivery action. The profile does
    /// not accept source, carrier, authority, retry, or expected-result data.
    pub fn with_adapter_delivery_profile(
        mut self,
        profile: I3LocalnetAdapterDeliveryProfile,
    ) -> Self {
        self.adapter_delivery_profile = Some(profile);
        self
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetChildAudit {
    slot: I3LocalnetChildSlot,
    pid: u32,
    reaped: bool,
    exec_confirmed: bool,
    assigned_loci: Vec<String>,
    trusted_control_consumed: bool,
    tainted_image_consumed: bool,
    observed_exit_status: Option<ExitStatus>,
    was_force_killed: bool,
}

impl I3LocalnetChildAudit {
    pub const fn slot(&self) -> I3LocalnetChildSlot {
        self.slot
    }

    pub const fn exec_confirmed(&self) -> bool {
        self.exec_confirmed
    }

    pub const fn pid(&self) -> u32 {
        self.pid
    }

    pub fn assigned_loci(&self) -> [&str; 2] {
        // The child reports its decoded image manifest.  The supervisor
        // rejects any completion report that is not the exact two-locus
        // deployment, so indexing here cannot expose a partial report.
        [
            self.assigned_loci[0].as_str(),
            self.assigned_loci[1].as_str(),
        ]
    }

    pub const fn trusted_control_delivery(&self) -> I3LocalnetControlDelivery {
        I3LocalnetControlDelivery::DedicatedOneShotTrustedFd
    }

    pub const fn tainted_image_delivery(&self) -> I3LocalnetImageDelivery {
        I3LocalnetImageDelivery::DedicatedTaintedImageFd
    }

    pub const fn reaped(&self) -> bool {
        self.reaped
    }

    pub fn observed_exit_status(&self) -> Option<ExitStatus> {
        self.observed_exit_status
    }

    pub const fn was_force_killed(&self) -> bool {
        self.was_force_killed
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct I3LocalnetStartupAudit {
    supervisor_ordinary_source_build_count: usize,
    supervisor_admission_count: usize,
    supervisor_m9_generation_count: usize,
    child_bootstrap_is_image_only_no_source_or_global_authority: bool,
    stores_are_process_local_and_distinct: bool,
    exact_one_shot_bindings_consumed: bool,
}

impl I3LocalnetStartupAudit {
    pub const fn supervisor_ordinary_source_build_count(&self) -> usize {
        self.supervisor_ordinary_source_build_count
    }
    pub const fn supervisor_admission_count(&self) -> usize {
        self.supervisor_admission_count
    }
    pub const fn supervisor_m9_generation_count(&self) -> usize {
        self.supervisor_m9_generation_count
    }
    /// Structural child-bootstrap fact, not an inferred zero counter.  The
    /// two exec children received only their tainted image pipe and the
    /// separately framed trusted control FD; source/Core/global-M9 inputs do
    /// not cross either child boundary.
    pub const fn child_bootstrap_is_image_only_no_source_or_global_authority(&self) -> bool {
        self.child_bootstrap_is_image_only_no_source_or_global_authority
    }
    /// The completed children reported distinct local-store references.  No
    /// store handle is carried in image/control/bootstrap records.
    pub const fn stores_are_process_local_and_distinct(&self) -> bool {
        self.stores_are_process_local_and_distinct
    }
    pub const fn exact_one_shot_bindings_consumed(&self) -> bool {
        self.exact_one_shot_bindings_consumed
    }
}

/// The finite one-stream delivery phases retained from actual child reports.
/// They are observer-safe occurrence labels, not a public transport protocol.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum I3LocalnetDeliveryPhase {
    RequestSend,
    RequestReceive,
    ReplySend,
    ReplyReceive,
}

/// Runtime-produced successful stream-write observation for the selected
/// complete-frame control. It has no bytes, frame length, packet, or read
/// boundary information. Normal generated sends intentionally carry none.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct I3LocalnetGeneratedFrameWriteObservation {
    application_write_count: u8,
    frame_prefix_split_across_writes: bool,
    complete_frame_written: bool,
}

impl I3LocalnetGeneratedFrameWriteObservation {
    pub const fn application_write_count(&self) -> u8 {
        self.application_write_count
    }

    pub const fn frame_prefix_split_across_writes(&self) -> bool {
        self.frame_prefix_split_across_writes
    }

    pub const fn complete_frame_written(&self) -> bool {
        self.complete_frame_written
    }
}

/// One child-reported delivery record.  Its checked provenance is extracted
/// by the runtime adapter from the exact encoded/decoded carrier bytes; the
/// supervisor merely equality-joins it to the generated contract.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct I3LocalnetObserverSafeDeliveryRecord {
    phase: I3LocalnetDeliveryPhase,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    carrier_ref: String,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    network_occurrence_ref: String,
    candidate_commitment_ref: String,
    generated_frame_write_observation: Option<I3LocalnetGeneratedFrameWriteObservation>,
}

impl I3LocalnetObserverSafeDeliveryRecord {
    pub const fn phase(&self) -> I3LocalnetDeliveryPhase {
        self.phase
    }
    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }
    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }
    pub fn source_artifact_ref(&self) -> &str {
        &self.source_artifact_ref
    }
    pub fn target_artifact_ref(&self) -> &str {
        &self.target_artifact_ref
    }
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
    pub fn carrier_ref(&self) -> &str {
        &self.carrier_ref
    }
    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }
    pub fn linked_request_identity_ref(&self) -> Option<&str> {
        self.linked_request_identity_ref.as_deref()
    }
    pub fn network_occurrence_ref(&self) -> &str {
        &self.network_occurrence_ref
    }
    pub fn candidate_commitment_ref(&self) -> &str {
        &self.candidate_commitment_ref
    }

    /// Present only when the adapter itself completed the selected two-write
    /// generated frame send. It is not a probe profile echo.
    pub fn generated_frame_write_observation(
        &self,
    ) -> Option<&I3LocalnetGeneratedFrameWriteObservation> {
        self.generated_frame_write_observation.as_ref()
    }
}

/// Owner-local evidence copied only from the runtime's retained,
/// producer-derived declared-expiry decision and its actual counters. It
/// carries no clock value, driver, permit, carrier bytes, source, authority,
/// witness, or expected semantic result.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetOwnerAdmissionExpiryEvidence {
    decision_commitment_ref: String,
    decision_occurrence_ref: String,
    generated_reply_count: usize,
    expired_count: usize,
    owner_serve_count: usize,
    owner_mutation_count: usize,
}

impl I3LocalnetOwnerAdmissionExpiryEvidence {
    pub fn decision_commitment_ref(&self) -> &str {
        &self.decision_commitment_ref
    }

    pub fn decision_occurrence_ref(&self) -> &str {
        &self.decision_occurrence_ref
    }

    pub const fn generated_reply_count(&self) -> usize {
        self.generated_reply_count
    }

    pub const fn expired_count(&self) -> usize {
        self.expired_count
    }

    pub const fn owner_serve_count(&self) -> usize {
        self.owner_serve_count
    }

    pub const fn owner_mutation_count(&self) -> usize {
        self.owner_mutation_count
    }
}

/// Joined actual delivery and local-terminal evidence for one generated
/// declared owner deadline expiry that reached requester consumption. The
/// four delivery records remain distinct actual adapter observations; the
/// producer references are copied only from B's runtime record.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetDeclaredOwnerDeadlineExpiredAudit {
    request_identity_ref: String,
    request_send: I3LocalnetObserverSafeDeliveryRecord,
    request_receive: I3LocalnetObserverSafeDeliveryRecord,
    reply_send: I3LocalnetObserverSafeDeliveryRecord,
    reply_receive: I3LocalnetObserverSafeDeliveryRecord,
    owner_expiry: I3LocalnetOwnerAdmissionExpiryEvidence,
    requester_terminal_failure_occurrence_ref: String,
    requester_terminal_failure_consumed_count: usize,
    requester_local_receipt_count: usize,
}

impl I3LocalnetDeclaredOwnerDeadlineExpiredAudit {
    pub fn request_identity_ref(&self) -> &str {
        &self.request_identity_ref
    }

    pub fn request_send(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.request_send
    }

    pub fn request_receive(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.request_receive
    }

    pub fn reply_send(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.reply_send
    }

    pub fn reply_receive(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.reply_receive
    }

    pub fn owner_expiry(&self) -> &I3LocalnetOwnerAdmissionExpiryEvidence {
        &self.owner_expiry
    }

    pub fn requester_terminal_failure_occurrence_ref(&self) -> &str {
        &self.requester_terminal_failure_occurrence_ref
    }

    pub const fn requester_terminal_failure_consumed_count(&self) -> usize {
        self.requester_terminal_failure_consumed_count
    }

    pub const fn requester_local_receipt_count(&self) -> usize {
        self.requester_local_receipt_count
    }
}

/// Observer-safe transport evidence for one complete frame held before any
/// semantic admission.  It intentionally omits decoded source/Core/artifact,
/// carrier, request identity, and owner facts: those remain unavailable until
/// the runtime actually admits the frame.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRetainedIngressEvidence {
    session_generation: u8,
    candidate_commitment_ref: String,
    network_occurrence_ref: String,
}

impl I3LocalnetRetainedIngressEvidence {
    pub const fn session_generation(&self) -> u8 {
        self.session_generation
    }

    pub fn candidate_commitment_ref(&self) -> &str {
        &self.candidate_commitment_ref
    }

    pub fn network_occurrence_ref(&self) -> &str {
        &self.network_occurrence_ref
    }
}

/// Two-session transport/lifecycle evidence emitted by one actual child for
/// the late-ingress schedule.  It carries no frame bytes, source, candidate,
/// receipt, capability, or authority material.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetLateIngressChildAudit {
    slot: I3LocalnetChildSlot,
    run_ref: String,
    first_session_generation: u8,
    reconnect_session_generation: u8,
    first_session_peer_spki_verified: bool,
    first_session_reciprocal_preface_verified: bool,
    reconnect_session_peer_spki_verified: bool,
    reconnect_session_reciprocal_preface_verified: bool,
    terminal_outcome: I3LocalnetChildTerminalOutcome,
}

impl I3LocalnetLateIngressChildAudit {
    pub const fn slot(&self) -> I3LocalnetChildSlot {
        self.slot
    }

    pub fn run_ref(&self) -> &str {
        &self.run_ref
    }

    pub const fn first_session_generation(&self) -> u8 {
        self.first_session_generation
    }

    pub const fn reconnect_session_generation(&self) -> u8 {
        self.reconnect_session_generation
    }

    pub const fn first_session_peer_spki_verified(&self) -> bool {
        self.first_session_peer_spki_verified
    }

    pub const fn first_session_reciprocal_preface_verified(&self) -> bool {
        self.first_session_reciprocal_preface_verified
    }

    pub const fn reconnect_session_peer_spki_verified(&self) -> bool {
        self.reconnect_session_peer_spki_verified
    }

    pub const fn reconnect_session_reciprocal_preface_verified(&self) -> bool {
        self.reconnect_session_reciprocal_preface_verified
    }

    pub const fn terminal_outcome(&self) -> I3LocalnetChildTerminalOutcome {
        self.terminal_outcome
    }
}

/// Joined observer-safe evidence for one actual retained session-one ingress
/// route.  This audit never exposes a held decoded candidate or lifecycle ACK
/// data; it reports only outcomes already produced by the adapter, runtime,
/// registered child route, and coordinator.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetLateIngressAudit {
    profile: I3LocalnetLateIngressProfile,
    request_identity_ref: String,
    requester_child: I3LocalnetLateIngressChildAudit,
    owner_child: I3LocalnetLateIngressChildAudit,
    initial_sender_delivery: I3LocalnetObserverSafeDeliveryRecord,
    retained_session_one_ingress: I3LocalnetRetainedIngressEvidence,
    retained_ingress_repeat_acquisition_rejection: Option<I3LocalnetAdapterRejectionKind>,
    late_admission_delivery: Option<I3LocalnetObserverSafeDeliveryRecord>,
    admission_session_generation: u8,
    owner_outcome: Option<I3LocalnetLateIngressOwnerOutcome>,
    requester_outcome: I3LocalnetLateIngressRequesterOutcome,
    outbound_request_frame_write_count: usize,
    lifecycle_provenance: I3LocalnetLateIngressLifecycleProvenance,
    m9_lifecycle_source_derived: Option<bool>,
    registered_owner_ack_reader_outcome: I3LocalnetLateIngressAckReaderOutcome,
    nonregistered_ack_input_disposition: I3LocalnetLateIngressNonregisteredAckInputDisposition,
    nonregistered_ack_input_count: usize,
    parent_publication: I3LocalnetLateIngressParentPublication,
    parent_publication_commit_count: usize,
}

impl I3LocalnetLateIngressAudit {
    pub const fn profile(&self) -> I3LocalnetLateIngressProfile {
        self.profile
    }

    pub fn request_identity_ref(&self) -> &str {
        &self.request_identity_ref
    }

    pub fn requester_child(&self) -> &I3LocalnetLateIngressChildAudit {
        &self.requester_child
    }

    pub fn owner_child(&self) -> &I3LocalnetLateIngressChildAudit {
        &self.owner_child
    }

    pub fn initial_sender_delivery(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.initial_sender_delivery
    }

    pub fn retained_session_one_ingress(&self) -> &I3LocalnetRetainedIngressEvidence {
        &self.retained_session_one_ingress
    }

    /// Present only when B actually called its production retained-ingress
    /// acquisition a second time after retaining the single session-one
    /// frame. The adapter's local rejection occurs before a second I/O read.
    pub const fn retained_ingress_repeat_acquisition_rejection(
        &self,
    ) -> Option<I3LocalnetAdapterRejectionKind> {
        self.retained_ingress_repeat_acquisition_rejection
    }

    pub fn late_admission_delivery(&self) -> Option<&I3LocalnetObserverSafeDeliveryRecord> {
        self.late_admission_delivery.as_ref()
    }

    pub const fn admission_session_generation(&self) -> u8 {
        self.admission_session_generation
    }

    pub fn owner_outcome(&self) -> Option<&I3LocalnetLateIngressOwnerOutcome> {
        self.owner_outcome.as_ref()
    }

    pub const fn requester_outcome(&self) -> I3LocalnetLateIngressRequesterOutcome {
        self.requester_outcome
    }

    pub const fn outbound_request_frame_write_count(&self) -> usize {
        self.outbound_request_frame_write_count
    }

    pub const fn lifecycle_provenance(&self) -> I3LocalnetLateIngressLifecycleProvenance {
        self.lifecycle_provenance
    }

    pub const fn m9_lifecycle_source_derived(&self) -> Option<bool> {
        self.m9_lifecycle_source_derived
    }

    pub const fn registered_owner_ack_reader_outcome(
        &self,
    ) -> I3LocalnetLateIngressAckReaderOutcome {
        self.registered_owner_ack_reader_outcome
    }

    /// Observation of bounded, decoded-as-tainted candidate input from A's
    /// actual stdout route. It is not a registered-B completion or authority
    /// publication route.
    pub const fn nonregistered_ack_input_disposition(
        &self,
    ) -> I3LocalnetLateIngressNonregisteredAckInputDisposition {
        self.nonregistered_ack_input_disposition
    }

    /// Number of bounded A stdout candidate inputs actually observed by the
    /// parent. The finite route permits at most one.
    pub const fn nonregistered_ack_input_count(&self) -> usize {
        self.nonregistered_ack_input_count
    }

    pub const fn parent_publication(&self) -> I3LocalnetLateIngressParentPublication {
        self.parent_publication
    }

    pub const fn parent_publication_commit_count(&self) -> usize {
        self.parent_publication_commit_count
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetExecutionAudit {
    requester_child: I3LocalnetChildSlot,
    owner_child: I3LocalnetChildSlot,
    generated_request_count: usize,
    remote_owner_serve_count: usize,
    remote_owner_write_count: usize,
    generated_reply_count: usize,
    requester_local_receipt_count: usize,
    network_receipt_frame_count: usize,
    source_derived_only: bool,
}

impl I3LocalnetExecutionAudit {
    pub const fn requester_child(&self) -> I3LocalnetChildSlot {
        self.requester_child
    }
    pub const fn owner_child(&self) -> I3LocalnetChildSlot {
        self.owner_child
    }
    pub const fn generated_request_count(&self) -> usize {
        self.generated_request_count
    }
    pub const fn remote_owner_serve_count(&self) -> usize {
        self.remote_owner_serve_count
    }
    pub const fn remote_owner_write_count(&self) -> usize {
        self.remote_owner_write_count
    }
    pub const fn generated_reply_count(&self) -> usize {
        self.generated_reply_count
    }
    pub const fn requester_local_receipt_count(&self) -> usize {
        self.requester_local_receipt_count
    }
    pub const fn network_receipt_frame_count(&self) -> usize {
        self.network_receipt_frame_count
    }
    pub const fn source_derived_only(&self) -> bool {
        self.source_derived_only
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct I3LocalnetTransportAudit {
    mutually_authenticated_quic_peer_binding: bool,
    reliable_bidirectional_streams_only: bool,
    quic_datagrams_enabled: bool,
    unauthenticated_semantic_admission_count: usize,
    ephemeral_endpoint_reuse_verified: bool,
}

impl I3LocalnetTransportAudit {
    pub const fn mutually_authenticated_quic_peer_binding(&self) -> bool {
        self.mutually_authenticated_quic_peer_binding
    }
    pub const fn reliable_bidirectional_streams_only(&self) -> bool {
        self.reliable_bidirectional_streams_only
    }
    pub const fn quic_datagrams_enabled(&self) -> bool {
        self.quic_datagrams_enabled
    }
    pub const fn unauthenticated_semantic_admission_count(&self) -> usize {
        self.unauthenticated_semantic_admission_count
    }
    pub const fn ephemeral_endpoint_reuse_verified(&self) -> bool {
        self.ephemeral_endpoint_reuse_verified
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetObserverSafeTrace {
    observer_safe: bool,
    exact_chain: bool,
    source_ref_count: usize,
    core_ref_count: usize,
    artifact_ref_count: usize,
    semantic_request_identity_count: usize,
    network_occurrence_count: usize,
    runtime_occurrence_count: usize,
    actual_delivery_records: Vec<I3LocalnetObserverSafeDeliveryRecord>,
    actual_source_ref_inventory: Vec<String>,
    actual_core_ref_inventory: Vec<String>,
    actual_artifact_ref_inventory: Vec<String>,
    actual_edge_ref_inventory: Vec<String>,
    references: I3LocalnetObserverSafeReferences,
}

impl I3LocalnetObserverSafeTrace {
    pub const fn is_observer_safe(&self) -> bool {
        self.observer_safe
    }
    pub const fn has_exact_source_core_artifact_carrier_network_runtime_chain(&self) -> bool {
        self.exact_chain
    }
    pub const fn source_ref_count(&self) -> usize {
        self.source_ref_count
    }
    pub const fn core_ref_count(&self) -> usize {
        self.core_ref_count
    }
    pub const fn artifact_ref_count(&self) -> usize {
        self.artifact_ref_count
    }
    pub const fn semantic_request_identity_count(&self) -> usize {
        self.semantic_request_identity_count
    }
    pub const fn network_occurrence_count(&self) -> usize {
        self.network_occurrence_count
    }
    pub const fn runtime_occurrence_count(&self) -> usize {
        self.runtime_occurrence_count
    }
    pub fn actual_delivery_records(&self) -> &[I3LocalnetObserverSafeDeliveryRecord] {
        &self.actual_delivery_records
    }
    pub fn actual_source_ref_inventory(&self) -> &[String] {
        &self.actual_source_ref_inventory
    }
    pub fn actual_core_ref_inventory(&self) -> &[String] {
        &self.actual_core_ref_inventory
    }
    pub fn actual_artifact_ref_inventory(&self) -> &[String] {
        &self.actual_artifact_ref_inventory
    }
    pub fn actual_edge_ref_inventory(&self) -> &[String] {
        &self.actual_edge_ref_inventory
    }
    pub fn actual_source_ref_count(&self) -> usize {
        self.actual_source_ref_inventory.len()
    }
    pub fn actual_core_ref_count(&self) -> usize {
        self.actual_core_ref_inventory.len()
    }
    pub fn actual_artifact_ref_count(&self) -> usize {
        self.actual_artifact_ref_inventory.len()
    }
    pub fn actual_edge_ref_count(&self) -> usize {
        self.actual_edge_ref_inventory.len()
    }
    pub fn references(&self) -> &I3LocalnetObserverSafeReferences {
        &self.references
    }
}

/// Actual observer-safe lineage and occurrence references reported by the
/// two child runtimes.  These values never contain raw source, payload,
/// credential, capability, witness, socket address, or session data.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct I3LocalnetObserverSafeReferences {
    request_source_ref: String,
    request_core_ref: String,
    request_source_artifact_ref: String,
    request_target_artifact_ref: String,
    request_edge_ref: String,
    reply_source_ref: String,
    reply_core_ref: String,
    reply_source_artifact_ref: String,
    reply_target_artifact_ref: String,
    reply_edge_ref: String,
    request_carrier_ref: String,
    reply_carrier_ref: String,
    semantic_request_identity_ref: String,
    network_request_identity_ref: String,
    network_reply_linked_request_identity_ref: String,
    network_request_occurrence_ref: String,
    network_reply_occurrence_ref: String,
    runtime_serve_request_identity_ref: String,
    runtime_write_request_identity_ref: String,
    runtime_receipt_linked_request_identity_ref: String,
    runtime_serve_occurrence_ref: String,
    runtime_write_occurrence_ref: String,
    runtime_receipt_occurrence_ref: String,
    requester_local_store_ref: String,
    owner_local_store_ref: String,
}

macro_rules! reference_getters {
    ($($name:ident),+ $(,)?) => {$(
        pub fn $name(&self) -> &str { &self.$name }
    )+};
}

impl I3LocalnetObserverSafeReferences {
    reference_getters!(
        request_source_ref,
        request_core_ref,
        request_source_artifact_ref,
        request_target_artifact_ref,
        request_edge_ref,
        reply_source_ref,
        reply_core_ref,
        reply_source_artifact_ref,
        reply_target_artifact_ref,
        reply_edge_ref,
        request_carrier_ref,
        reply_carrier_ref,
        semantic_request_identity_ref,
        network_request_identity_ref,
        network_reply_linked_request_identity_ref,
        network_request_occurrence_ref,
        network_reply_occurrence_ref,
        runtime_serve_request_identity_ref,
        runtime_write_request_identity_ref,
        runtime_receipt_linked_request_identity_ref,
        runtime_serve_occurrence_ref,
        runtime_write_occurrence_ref,
        runtime_receipt_occurrence_ref,
        requester_local_store_ref,
        owner_local_store_ref,
    );
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct I3LocalnetLifecycleAudit {
    all_children_reaped: bool,
    clean_shutdown_backed_by_zero_exit_reaps_without_force_kill: bool,
    observed_supervised_process_lifecycle_elapsed: Duration,
    observed_supervised_process_lifecycle_bound: Duration,
    zero_exit_reap_observed_within_deadline: bool,
    captured_zero_exit_reap_observation_elapsed: Duration,
}

impl I3LocalnetLifecycleAudit {
    pub const fn clean_shutdown(&self) -> bool {
        self.clean_shutdown_backed_by_zero_exit_reaps_without_force_kill
    }
    pub const fn all_children_reaped(&self) -> bool {
        self.all_children_reaped
    }

    pub const fn clean_shutdown_is_backed_by_zero_exit_reaps_without_force_kill(&self) -> bool {
        self.clean_shutdown_backed_by_zero_exit_reaps_without_force_kill
    }

    /// Observed duration of the supervisor's process-execution/reaping phase.
    /// Synchronous source, checked-project, cohort, and credential preflight
    /// intentionally occur before this bounded I3-2 phase.
    pub const fn observed_supervised_process_lifecycle_elapsed(&self) -> Duration {
        self.observed_supervised_process_lifecycle_elapsed
    }

    /// Configured bound of the same process-execution/reaping phase: the
    /// selected main deadline plus the explicit reaper allowance.
    pub const fn observed_supervised_process_lifecycle_bound(&self) -> Duration {
        self.observed_supervised_process_lifecycle_bound
    }

    /// Whether the supervisor itself observed both natural zero exits before
    /// the lifecycle deadline.  This does not claim a host scheduler or
    /// kernel exit timestamp beyond that observation point.
    pub const fn zero_exit_reap_observed_within_deadline(&self) -> bool {
        self.zero_exit_reap_observed_within_deadline
    }

    /// Captured at terminal natural-reap observation, before later joined
    /// trace/evidence assembly can affect the reported lifecycle duration.
    pub const fn captured_zero_exit_reap_observation_elapsed(&self) -> Duration {
        self.captured_zero_exit_reap_observation_elapsed
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRejectionAudit {
    stage: I3LocalnetFailureStage,
    all_children_reaped: bool,
    fixed_child_control_descriptors_preserved: Option<bool>,
    child_owner_starts: usize,
    quic_certificate_initializations: Option<usize>,
    quic_handshake_count: Option<usize>,
    semantic_admission_count: usize,
    owner_mutation_count: usize,
    observer_safe: bool,
    real_wrong_peer_delivery_attempted: bool,
    wrong_peer_certificate_chains_to_run_ca: bool,
    wrong_peer_spki_differs_from_expected: bool,
    requester_observer_state_before: String,
    requester_observer_state_after: String,
    requester_pending_request_is_retained: bool,
    deadline_enforced: bool,
    reaper_deadline_enforced: bool,
    lifecycle_rejection_cause: Option<I3LocalnetLifecycleRejectionCause>,
    adapter_rejection_kind: Option<I3LocalnetAdapterRejectionKind>,
    adapter_delivery_failure: Option<I3LocalnetAdapterDeliveryFailure>,
    owner_request_delivery_record: Option<I3LocalnetObserverSafeDeliveryRecord>,
    wrong_peer_ca_validated_leaf_ref: Option<String>,
    expected_peer_spki_ref: Option<String>,
    actual_peer_spki_ref: Option<String>,
    requester_first_session_peer_spki_verified: bool,
    requester_first_session_reciprocal_preface_verified: bool,
    child_terminal_events: Vec<I3LocalnetChildTerminalEvent>,
    unknown_child_failure_count: usize,
    structurally_valid_completed_child_report_observed: bool,
    no_orphan_child_pids: bool,
    completed_child_exited_nonzero: bool,
    completed_child_ignored_graceful_completion: bool,
    completed_child_was_force_killed_after_reaper_allowance: bool,
    spawned_child_count: usize,
    observed_supervised_process_lifecycle_elapsed: Duration,
    observed_supervised_process_lifecycle_bound: Duration,
    zero_exit_reap_observed_within_deadline: bool,
    captured_zero_exit_reap_observation_elapsed: Duration,
}

impl I3LocalnetRejectionAudit {
    pub const fn stage(&self) -> I3LocalnetFailureStage {
        self.stage
    }
    /// Fixed-control preservation is reported only by a typed child terminal
    /// that carries it; generic failures make no such observation.
    pub const fn fixed_child_control_descriptors_preserved(&self) -> Option<bool> {
        self.fixed_child_control_descriptors_preserved
    }
    /// Sum from typed child observations only. If
    /// `unknown_child_failure_count` is nonzero, this is not a claim that an
    /// unobserved child contributed zero starts.
    pub const fn child_owner_starts(&self) -> usize {
        self.child_owner_starts
    }
    /// Certificate setup count reported only by a typed child terminal that
    /// carries it, never synthesized from a global child count.
    pub const fn quic_certificate_initializations(&self) -> Option<usize> {
        self.quic_certificate_initializations
    }
    /// Handshake count reported only by a typed child terminal that carries
    /// it, never synthesized from a global child count.
    pub const fn quic_handshake_count(&self) -> Option<usize> {
        self.quic_handshake_count
    }
    /// Sum from typed child observations only. See
    /// `unknown_child_failure_count` before treating a zero as exhaustive.
    pub const fn semantic_admission_count(&self) -> usize {
        self.semantic_admission_count
    }
    /// Sum from typed child observations only. See
    /// `unknown_child_failure_count` before treating a zero as exhaustive.
    pub const fn owner_mutation_count(&self) -> usize {
        self.owner_mutation_count
    }
    pub const fn all_children_reaped(&self) -> bool {
        self.all_children_reaped
    }
    pub const fn observer_safe(&self) -> bool {
        self.observer_safe
    }
    pub const fn real_wrong_peer_delivery_attempted(&self) -> bool {
        self.real_wrong_peer_delivery_attempted
    }
    pub const fn wrong_peer_certificate_chains_to_run_ca(&self) -> bool {
        self.wrong_peer_certificate_chains_to_run_ca
    }
    pub const fn wrong_peer_spki_differs_from_expected(&self) -> bool {
        self.wrong_peer_spki_differs_from_expected
    }
    pub fn requester_observer_state_before(&self) -> &str {
        &self.requester_observer_state_before
    }
    pub fn requester_observer_state_after(&self) -> &str {
        &self.requester_observer_state_after
    }
    pub const fn requester_pending_request_is_retained(&self) -> bool {
        self.requester_pending_request_is_retained
    }
    pub const fn deadline_enforced(&self) -> bool {
        self.deadline_enforced
    }
    pub const fn reaper_deadline_enforced(&self) -> bool {
        self.reaper_deadline_enforced
    }
    pub const fn lifecycle_rejection_cause(&self) -> Option<I3LocalnetLifecycleRejectionCause> {
        self.lifecycle_rejection_cause
    }
    pub const fn adapter_rejection_kind(&self) -> Option<I3LocalnetAdapterRejectionKind> {
        self.adapter_rejection_kind
    }
    /// Concrete local adapter failure observed by an actual selected delivery
    /// terminal. It carries no conclusion about remote admission.
    pub const fn adapter_delivery_failure(&self) -> Option<I3LocalnetAdapterDeliveryFailure> {
        self.adapter_delivery_failure
    }
    /// An admitted owner request-receive record only when a fault or retry
    /// audit has already validated that B evidence against the retained
    /// requester contract. Other terminals, including malformed or
    /// no-admission observations, deliberately expose none.
    pub fn owner_request_delivery_record(&self) -> Option<&I3LocalnetObserverSafeDeliveryRecord> {
        self.owner_request_delivery_record.as_ref()
    }
    pub fn wrong_peer_ca_validated_leaf_ref(&self) -> Option<&str> {
        self.wrong_peer_ca_validated_leaf_ref.as_deref()
    }
    pub fn expected_peer_spki_ref(&self) -> Option<&str> {
        self.expected_peer_spki_ref.as_deref()
    }
    pub fn actual_peer_spki_ref(&self) -> Option<&str> {
        self.actual_peer_spki_ref.as_deref()
    }
    /// Actual requester-side observation immediately before a local
    /// generation misuse attempt; it is never inferred from a profile.
    pub const fn requester_first_session_peer_spki_verified(&self) -> bool {
        self.requester_first_session_peer_spki_verified
    }
    /// Actual requester-side reciprocal-preface observation immediately
    /// before a local generation misuse attempt.
    pub const fn requester_first_session_reciprocal_preface_verified(&self) -> bool {
        self.requester_first_session_reciprocal_preface_verified
    }
    pub fn child_terminal_events(&self) -> &[I3LocalnetChildTerminalEvent] {
        &self.child_terminal_events
    }
    pub fn child_terminal_event_count(&self) -> usize {
        self.child_terminal_events.len()
    }
    /// Generic child failure preserves no semantic or owner-runtime counters.
    /// Its count is kept separately so callers cannot mistake absent evidence
    /// for an observed zero terminal record.
    pub const fn unknown_child_failure_count(&self) -> usize {
        self.unknown_child_failure_count
    }

    /// The one narrow B-local observation for a source-budgeted request held
    /// Awaiting a trusted owner driver. It contains no driver, clock handle,
    /// permit, carrier bytes, key, or source text.
    pub fn owner_admission_awaiting_observation(&self) -> Option<&I3LocalnetChildTerminalEvent> {
        self.child_terminal_events
            .iter()
            .find(|event| event.outcome == I3LocalnetChildTerminalOutcome::OwnerAdmissionAwaiting)
    }

    /// Sum of counters carried by actual typed terminal observations. A
    /// generic child failure is excluded and is exposed by
    /// `unknown_child_failure_count` instead of contributing a synthetic zero.
    pub fn aggregate_semantic_admission_count(&self) -> usize {
        self.child_terminal_events
            .iter()
            .map(I3LocalnetChildTerminalEvent::semantic_admission_count)
            .sum()
    }
    pub fn aggregate_owner_mutation_count(&self) -> usize {
        self.child_terminal_events
            .iter()
            .map(I3LocalnetChildTerminalEvent::owner_mutation_count)
            .sum()
    }
    pub const fn structurally_valid_completed_child_report_observed(&self) -> bool {
        self.structurally_valid_completed_child_report_observed
    }
    pub const fn no_orphan_child_pids(&self) -> bool {
        self.no_orphan_child_pids
    }
    pub const fn completed_child_exited_nonzero(&self) -> bool {
        self.completed_child_exited_nonzero
    }
    pub const fn completed_child_ignored_graceful_completion(&self) -> bool {
        self.completed_child_ignored_graceful_completion
    }
    pub const fn completed_child_was_force_killed_after_reaper_allowance(&self) -> bool {
        self.completed_child_was_force_killed_after_reaper_allowance
    }
    pub const fn spawned_child_count(&self) -> usize {
        self.spawned_child_count
    }
    pub const fn observed_supervised_process_lifecycle_elapsed(&self) -> Duration {
        self.observed_supervised_process_lifecycle_elapsed
    }
    pub const fn observed_supervised_process_lifecycle_bound(&self) -> Duration {
        self.observed_supervised_process_lifecycle_bound
    }
    pub const fn zero_exit_reap_observed_within_deadline(&self) -> bool {
        self.zero_exit_reap_observed_within_deadline
    }
    pub const fn captured_zero_exit_reap_observation_elapsed(&self) -> Duration {
        self.captured_zero_exit_reap_observation_elapsed
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRunError {
    kind: I3LocalnetRunErrorKind,
    rejection_audit: I3LocalnetRejectionAudit,
    fault_audit: Option<I3LocalnetFaultAudit>,
    retry_audit: Option<I3LocalnetRetryAudit>,
    owner_reply_replay_audit: Option<I3LocalnetOwnerReplyReplayAudit>,
    declared_owner_deadline_expired_audit: Option<I3LocalnetDeclaredOwnerDeadlineExpiredAudit>,
    late_ingress_audit: Option<I3LocalnetLateIngressAudit>,
    late_ingress_evidence_rejection: Option<I3LocalnetLateIngressEvidenceRejection>,
}

impl I3LocalnetRunError {
    #[expect(
        clippy::too_many_arguments,
        reason = "the private constructor retains independently typed named audit channels rather than an untyped error bundle"
    )]
    fn new(
        kind: I3LocalnetRunErrorKind,
        rejection_audit: I3LocalnetRejectionAudit,
        fault_audit: Option<I3LocalnetFaultAudit>,
        retry_audit: Option<I3LocalnetRetryAudit>,
        owner_reply_replay_audit: Option<I3LocalnetOwnerReplyReplayAudit>,
        declared_owner_deadline_expired_audit: Option<I3LocalnetDeclaredOwnerDeadlineExpiredAudit>,
        late_ingress_audit: Option<I3LocalnetLateIngressAudit>,
        late_ingress_evidence_rejection: Option<I3LocalnetLateIngressEvidenceRejection>,
    ) -> Self {
        Self {
            kind,
            rejection_audit,
            fault_audit,
            retry_audit,
            owner_reply_replay_audit,
            declared_owner_deadline_expired_audit,
            late_ingress_audit,
            late_ingress_evidence_rejection,
        }
    }

    pub const fn kind(&self) -> I3LocalnetRunErrorKind {
        self.kind
    }
    pub fn rejection_audit(&self) -> I3LocalnetRejectionAudit {
        self.rejection_audit.clone()
    }

    /// Present only when an actual child formed the source-derived request
    /// and reached one selected delivery-fault schedule.  Earlier setup or
    /// provenance failures intentionally retain no nominal fault outcome.
    pub fn fault_audit(&self) -> Option<&I3LocalnetFaultAudit> {
        self.fault_audit.as_ref()
    }

    /// Present only after a real two-session retry profile reached its
    /// mandatory reconnect send. A first-session local-attempt rejection has
    /// no retry audit because it made no reconnect session or frame effect.
    pub fn retry_audit(&self) -> Option<&I3LocalnetRetryAudit> {
        self.retry_audit.as_ref()
    }

    /// Present only after both children reported the actual first known
    /// reply and the selected bounded replay disposition.
    pub fn owner_reply_replay_audit(&self) -> Option<&I3LocalnetOwnerReplyReplayAudit> {
        self.owner_reply_replay_audit.as_ref()
    }

    /// Present only for a complete actual route whose generated declared
    /// owner failure reached requester terminal consumption.
    pub fn declared_owner_deadline_expired_audit(
        &self,
    ) -> Option<&I3LocalnetDeclaredOwnerDeadlineExpiredAudit> {
        self.declared_owner_deadline_expired_audit.as_ref()
    }

    /// Present only when actual child records reached the retained-ingress
    /// join.  A rejected bootstrap may intentionally retain no owner outcome.
    pub fn late_ingress_audit(&self) -> Option<&I3LocalnetLateIngressAudit> {
        self.late_ingress_audit.as_ref()
    }

    /// Present only when the retained session-one adapter evidence failed the
    /// strict observer join. It makes no owner semantic conclusion.
    pub const fn late_ingress_evidence_rejection(
        &self,
    ) -> Option<I3LocalnetLateIngressEvidenceRejection> {
        self.late_ingress_evidence_rejection
    }
}

struct LocalnetFailure {
    kind: I3LocalnetRunErrorKind,
    stage: I3LocalnetFailureStage,
    child_rejection: Option<Box<PrivateChildEvent>>,
    evidence: LocalnetRejectionEvidence,
    lifecycle_rejection_cause: Option<I3LocalnetLifecycleRejectionCause>,
    fault_profile: Option<I3LocalnetFaultProfile>,
    requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
    retry_audit: Option<I3LocalnetRetryAudit>,
    owner_reply_replay_audit: Option<I3LocalnetOwnerReplyReplayAudit>,
    declared_owner_deadline_expired_audit: Option<I3LocalnetDeclaredOwnerDeadlineExpiredAudit>,
    late_ingress_audit: Option<I3LocalnetLateIngressAudit>,
    late_ingress_evidence_rejection: Option<I3LocalnetLateIngressEvidenceRejection>,
}

#[derive(Default)]
struct LocalnetRejectionEvidence {
    observed_owner_runtime_start_count: usize,
    real_wrong_peer_delivery_attempted: bool,
    wrong_peer_certificate_chains_to_run_ca: bool,
    wrong_peer_spki_differs_from_expected: bool,
    requester_observer_state_before: String,
    requester_observer_state_after: String,
    requester_pending_request_is_retained: bool,
    deadline_enforced: bool,
    reaper_deadline_enforced: bool,
    adapter_rejection_kind: Option<I3LocalnetAdapterRejectionKind>,
    adapter_delivery_failure: Option<I3LocalnetAdapterDeliveryFailure>,
    wrong_peer_ca_validated_leaf_ref: Option<String>,
    expected_peer_spki_ref: Option<String>,
    actual_peer_spki_ref: Option<String>,
}

impl LocalnetFailure {
    fn lifecycle() -> Self {
        Self {
            kind: I3LocalnetRunErrorKind::LifecycleRejected,
            stage: I3LocalnetFailureStage::BeforeOwnerStart,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence::default(),
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
    }

    fn from_child(
        kind: I3LocalnetRunErrorKind,
        stage: I3LocalnetFailureStage,
        child_rejection: PrivateChildEvent,
    ) -> Self {
        Self {
            kind,
            stage,
            child_rejection: Some(Box::new(child_rejection)),
            evidence: LocalnetRejectionEvidence::default(),
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
    }

    fn lifecycle_with_cause(cause: I3LocalnetLifecycleRejectionCause) -> Self {
        let mut failure = Self::lifecycle();
        failure.lifecycle_rejection_cause = Some(cause);
        failure
    }

    /// Available lifecycle evidence cannot justify one of the accepted
    /// terminal shapes. This does not claim a remote admission or an owner
    /// start; a caller that actually observed `Ready` retains it explicitly.
    fn lifecycle_evidence_rejected() -> Self {
        let mut failure = Self::lifecycle();
        failure.stage = I3LocalnetFailureStage::LifecycleEvidenceRejected;
        failure
    }

    /// The owner child itself observed the one source-budgeted request held
    /// Awaiting a later trusted driver decision. This is diagnostic evidence
    /// of a missing local continuation, never a generated semantic outcome.
    fn owner_admission_drive_required(child_event: PrivateChildEvent) -> Self {
        let mut failure = Self::from_child(
            I3LocalnetRunErrorKind::LifecycleRejected,
            I3LocalnetFailureStage::LifecycleEvidenceRejected,
            child_event,
        );
        failure.lifecycle_rejection_cause =
            Some(I3LocalnetLifecycleRejectionCause::OwnerAdmissionDriveRequired);
        failure
    }

    fn delivery_fault(
        profile: I3LocalnetFaultProfile,
        requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
    ) -> Self {
        // A profile selects a real scheduling point, not a proven remote
        // outcome. Post-loss begins with the requester observation and may be
        // refined only by the later supervisor-side evidence join.
        let (kind, stage) = match profile {
            I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite => (
                I3LocalnetRunErrorKind::DeliveryUnavailable,
                I3LocalnetFailureStage::BeforeSemanticAdmission,
            ),
            I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission
            | I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit => (
                I3LocalnetRunErrorKind::AmbiguousDelivery,
                I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
            ),
        };
        Self {
            kind,
            stage,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence::default(),
            lifecycle_rejection_cause: None,
            fault_profile: Some(profile),
            requester_local_wait_profile,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
    }

    fn adapter_delivery_failure(failure: I3LocalnetAdapterDeliveryFailure) -> Self {
        Self {
            kind: I3LocalnetRunErrorKind::DeliveryUnavailable,
            stage: I3LocalnetFailureStage::BeforeSemanticAdmission,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence {
                adapter_delivery_failure: Some(failure),
                ..LocalnetRejectionEvidence::default()
            },
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
    }

    fn retry_delivery(retry_audit: I3LocalnetRetryAudit) -> Self {
        let requester_pending_request_is_retained = matches!(
            retry_audit.requester_outcome(),
            I3LocalnetRetryRequesterOutcome::PendingReplyOrReceiptNotObserved
        );
        Self {
            kind: I3LocalnetRunErrorKind::AmbiguousDelivery,
            // The requester observed no reply/receipt.  The supervisor may
            // refine this only after it validates the independently emitted
            // initial owner admission record.
            stage: I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence {
                // `retry_audit` is created only from the retained requester
                // pending observation after its source-contract validation.
                requester_pending_request_is_retained,
                ..LocalnetRejectionEvidence::default()
            },
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: Some(retry_audit),
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
    }

    fn declared_owner_deadline_expired_consumed(
        audit: I3LocalnetDeclaredOwnerDeadlineExpiredAudit,
    ) -> Self {
        let mut failure = Self::lifecycle();
        failure.kind = I3LocalnetRunErrorKind::TerminalFailureConsumed;
        failure.stage = I3LocalnetFailureStage::RequesterTerminalFailureConsumed;
        failure.declared_owner_deadline_expired_audit = Some(audit);
        failure
    }

    fn owner_reply_replay_rejected(audit: I3LocalnetOwnerReplyReplayAudit) -> Self {
        let mut failure = Self::lifecycle();
        failure.kind = I3LocalnetRunErrorKind::OwnerReplyReplayRejected;
        failure.stage = I3LocalnetFailureStage::AfterKnownRequesterDecision;
        failure.owner_reply_replay_audit = Some(audit);
        failure
    }

    fn late_ingress_delivery(late_ingress_audit: I3LocalnetLateIngressAudit) -> Self {
        let requester_pending_request_is_retained = matches!(
            late_ingress_audit.requester_outcome(),
            I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved
        );
        Self {
            kind: I3LocalnetRunErrorKind::AmbiguousDelivery,
            stage: I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence {
                requester_pending_request_is_retained,
                ..LocalnetRejectionEvidence::default()
            },
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: Some(late_ingress_audit),
            late_ingress_evidence_rejection: None,
        }
    }

    fn late_ingress_evidence_rejected(rejection: I3LocalnetLateIngressEvidenceRejection) -> Self {
        let mut failure = Self::lifecycle_evidence_rejected();
        failure.late_ingress_evidence_rejection = Some(rejection);
        failure
    }

    /// A server `Ready` event is emitted only after the owner image/control
    /// pair has started its local runtime.  Later peer rejection must retain
    /// that observed fact instead of reporting a fabricated zero start count
    /// from the rejecting requester child.
    fn after_observed_owner_runtime_start(mut self) -> Self {
        self.evidence.observed_owner_runtime_start_count = 1;
        self
    }

    fn into_error(self, all_children_reaped: bool) -> I3LocalnetRunError {
        self.into_error_with_terminal_events(
            all_children_reaped,
            Vec::new(),
            0,
            false,
            false,
            false,
            0,
            Duration::ZERO,
            Duration::ZERO,
            false,
            Duration::ZERO,
            None,
            None,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)] // private audit preserves independent lifecycle observations.
    fn into_error_with_terminal_events(
        self,
        all_children_reaped: bool,
        child_terminal_events: Vec<I3LocalnetChildTerminalEvent>,
        unknown_child_failure_count: usize,
        completed_child_exited_nonzero: bool,
        completed_child_ignored_graceful_completion: bool,
        completed_child_was_force_killed_after_reaper_allowance: bool,
        spawned_child_count: usize,
        observed_supervised_process_lifecycle_elapsed: Duration,
        observed_supervised_process_lifecycle_bound: Duration,
        zero_exit_reap_observed_within_deadline: bool,
        captured_zero_exit_reap_observation_elapsed: Duration,
        fault_audit: Option<I3LocalnetFaultAudit>,
        retry_audit: Option<I3LocalnetRetryAudit>,
        owner_request_delivery_record: Option<I3LocalnetObserverSafeDeliveryRecord>,
    ) -> I3LocalnetRunError {
        let aggregate_semantic_admission_count = child_terminal_events
            .iter()
            .map(I3LocalnetChildTerminalEvent::semantic_admission_count)
            .sum();
        let aggregate_owner_mutation_count = child_terminal_events
            .iter()
            .map(I3LocalnetChildTerminalEvent::owner_mutation_count)
            .sum();
        let (
            fixed_child_control_descriptors_preserved,
            child_owner_starts,
            quic_certificate_initializations,
            quic_handshake_count,
            _child_semantic_admission_count,
            _child_owner_mutation_count,
            observer_safe,
            child_evidence,
        ) = match self.child_rejection.map(|event| *event) {
            Some(PrivateChildEvent::Rejected {
                fixed_control_descriptor_preserved,
                owner_start_count,
                certificate_initialization_count,
                handshake_count,
                semantic_admission_count: _child_semantic_admission_count,
                owner_mutation_count: _child_owner_mutation_count,
                evidence: child_evidence,
                ..
            }) => (
                Some(fixed_control_descriptor_preserved),
                owner_start_count,
                Some(certificate_initialization_count),
                Some(handshake_count),
                _child_semantic_admission_count,
                _child_owner_mutation_count,
                true,
                child_evidence,
            ),
            Some(PrivateChildEvent::OwnerAdmissionAwaiting {
                fixed_control_descriptor_preserved,
                owner_start_count,
                certificate_initialization_count,
                handshake_count,
                ..
            }) => (
                Some(fixed_control_descriptor_preserved),
                owner_start_count,
                Some(certificate_initialization_count),
                Some(handshake_count),
                0,
                0,
                true,
                PrivateChildRejectionEvidence::default(),
            ),
            // Only an actual rejected terminal supplies these three local
            // observations. Other terminal shapes, and no terminal at all,
            // must remain unknown rather than appearing as global zeroes.
            None | Some(_) => (
                None,
                0,
                None,
                None,
                0,
                0,
                false,
                PrivateChildRejectionEvidence::default(),
            ),
        };
        I3LocalnetRunError::new(
            self.kind,
            I3LocalnetRejectionAudit {
                stage: self.stage,
                all_children_reaped,
                fixed_child_control_descriptors_preserved,
                child_owner_starts: child_owner_starts
                    .max(self.evidence.observed_owner_runtime_start_count),
                quic_certificate_initializations,
                quic_handshake_count,
                semantic_admission_count: aggregate_semantic_admission_count,
                owner_mutation_count: aggregate_owner_mutation_count,
                // Rejection audit emits only fixed enums, counts, and
                // reference strings. Reaping confirms no child remains whose
                // stdout/control could later extend this finite observation.
                observer_safe: observer_safe || all_children_reaped,
                real_wrong_peer_delivery_attempted: self
                    .evidence
                    .real_wrong_peer_delivery_attempted
                    || child_evidence.real_wrong_peer_delivery_attempted,
                wrong_peer_certificate_chains_to_run_ca: self
                    .evidence
                    .wrong_peer_certificate_chains_to_run_ca
                    || child_evidence.wrong_peer_certificate_chains_to_run_ca,
                wrong_peer_spki_differs_from_expected: self
                    .evidence
                    .wrong_peer_spki_differs_from_expected
                    || child_evidence.wrong_peer_spki_differs_from_expected,
                requester_observer_state_before: if self
                    .evidence
                    .requester_observer_state_before
                    .is_empty()
                {
                    child_evidence.requester_observer_state_before
                } else {
                    self.evidence.requester_observer_state_before
                },
                requester_observer_state_after: if self
                    .evidence
                    .requester_observer_state_after
                    .is_empty()
                {
                    child_evidence.requester_observer_state_after
                } else {
                    self.evidence.requester_observer_state_after
                },
                requester_pending_request_is_retained: self
                    .evidence
                    .requester_pending_request_is_retained
                    || child_evidence.requester_pending_request_is_retained,
                deadline_enforced: self.evidence.deadline_enforced,
                reaper_deadline_enforced: self.evidence.reaper_deadline_enforced,
                lifecycle_rejection_cause: self.lifecycle_rejection_cause,
                adapter_rejection_kind: self
                    .evidence
                    .adapter_rejection_kind
                    .or(child_evidence.adapter_rejection_kind),
                adapter_delivery_failure: self.evidence.adapter_delivery_failure,
                owner_request_delivery_record,
                wrong_peer_ca_validated_leaf_ref: self
                    .evidence
                    .wrong_peer_ca_validated_leaf_ref
                    .or(child_evidence.wrong_peer_ca_validated_leaf_ref.clone()),
                expected_peer_spki_ref: self
                    .evidence
                    .expected_peer_spki_ref
                    .or(child_evidence.expected_peer_spki_ref.clone()),
                actual_peer_spki_ref: self
                    .evidence
                    .actual_peer_spki_ref
                    .or(child_evidence.actual_peer_spki_ref.clone()),
                requester_first_session_peer_spki_verified: child_evidence
                    .requester_first_session_peer_spki_verified,
                requester_first_session_reciprocal_preface_verified: child_evidence
                    .requester_first_session_reciprocal_preface_verified,
                structurally_valid_completed_child_report_observed: child_terminal_events
                    .iter()
                    .any(|event| event.outcome == I3LocalnetChildTerminalOutcome::Completed),
                no_orphan_child_pids: all_children_reaped,
                child_terminal_events,
                unknown_child_failure_count,
                completed_child_exited_nonzero,
                completed_child_ignored_graceful_completion,
                completed_child_was_force_killed_after_reaper_allowance,
                spawned_child_count,
                observed_supervised_process_lifecycle_elapsed,
                observed_supervised_process_lifecycle_bound,
                zero_exit_reap_observed_within_deadline,
                captured_zero_exit_reap_observation_elapsed,
            },
            fault_audit,
            retry_audit,
            self.owner_reply_replay_audit,
            self.declared_owner_deadline_expired_audit,
            self.late_ingress_audit,
            self.late_ingress_evidence_rejection,
        )
    }

    fn into_supervisor_error(
        mut self,
        supervisor: &LocalnetSupervisor,
        all_children_reaped: bool,
        lineage: &SupervisorLineageEvidence,
    ) -> I3LocalnetRunError {
        let completed_nonzero = supervisor.completed_child_exited_nonzero();
        let completed_hung = supervisor.completed_child_hung_and_was_force_killed()
            || supervisor.completed_child_exhausted_natural_reaper();
        let late_zero_exit_observation = supervisor.zero_exit_reap_observed_late();
        if completed_hung || late_zero_exit_observation {
            self.kind = I3LocalnetRunErrorKind::LifecycleDeadlineExceeded;
            self.stage = I3LocalnetFailureStage::CleanupDeadline;
            self.evidence.deadline_enforced = true;
            self.evidence.reaper_deadline_enforced = true;
        } else if completed_nonzero {
            self.kind = I3LocalnetRunErrorKind::LifecycleRejected;
            self.lifecycle_rejection_cause =
                Some(I3LocalnetLifecycleRejectionCause::CompletedChildExitedNonzero);
        }
        let fault_join = self.fault_profile.and_then(|profile| {
            supervisor.fault_audit(
                profile,
                self.requester_local_wait_profile,
                &lineage.request_contract,
            )
        });
        if self.requester_local_wait_profile.is_some()
            && fault_join.is_none()
            && self.kind == I3LocalnetRunErrorKind::AmbiguousDelivery
            && self.stage == I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved
        {
            // A selected wait is accepted only from A's actual pre/post
            // runtime observations and the existing independent B admission
            // join. A malformed emitted observation is evidence rejection,
            // never remote unavailability or retained-pending proof.
            self.kind = I3LocalnetRunErrorKind::LifecycleRejected;
            self.stage = I3LocalnetFailureStage::LifecycleEvidenceRejected;
            self.evidence.requester_pending_request_is_retained = false;
        }
        if fault_join
            .as_ref()
            .is_some_and(|(_, requester_pending_request_is_retained)| {
                *requester_pending_request_is_retained
            })
        {
            // This flag comes only from A's typed post-loss terminal after
            // its own runtime retained the original request, and only after
            // the parent joined that terminal with the selected fault route.
            self.evidence.requester_pending_request_is_retained = true;
        }
        let fault_audit = fault_join.map(|(audit, _)| audit);
        if self.stage == I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved
            && fault_audit.as_ref().is_some_and(|audit| {
                matches!(
                    audit.remote_admission(),
                    Some(I3LocalnetRemoteAdmissionEvidence::Admitted { .. })
                )
            })
        {
            // A selected fault schedule is not evidence of remote admission.
            // Only the supervisor's validated owner event refines the
            // requester's missing-reply observation to this later phase.
            self.stage = I3LocalnetFailureStage::AfterRemoteAdmission;
        }
        if self.stage == I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved
            && self.retry_audit.as_ref().is_some_and(|audit| {
                matches!(
                    audit.initial_owner_admission(),
                    Some(I3LocalnetRemoteAdmissionEvidence::Admitted { .. })
                )
            })
        {
            self.stage = I3LocalnetFailureStage::AfterRemoteAdmission;
        }
        if self.stage == I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved
            && self
                .retry_audit
                .as_ref()
                .is_some_and(|audit| audit.initial_owner_expiry().is_some())
        {
            // The retry join validates the retained B producer record and
            // the exact session-one receive. It is stronger and distinct
            // from a served remote admission.
            self.stage = I3LocalnetFailureStage::AfterRemoteDeclaredOwnerExpiry;
        }
        let retry_audit = self.retry_audit.take();
        // This record is an optional projection of an already validated
        // audit join. It must never be re-read directly from a raw B terminal:
        // malformed owner evidence remains rejected rather than becoming an
        // apparently observer-safe delivery record.
        let owner_request_delivery_record = fault_audit
            .as_ref()
            .and_then(|audit| match audit.remote_admission() {
                Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
                    request_receive, ..
                }) => Some(request_receive.as_ref()),
                Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission { .. }) | None => None,
            })
            .or_else(|| {
                retry_audit
                    .as_ref()
                    .and_then(|audit| match audit.initial_owner_admission() {
                        Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
                            request_receive,
                            ..
                        }) => Some(request_receive.as_ref()),
                        Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission { .. }) | None => None,
                    })
            })
            .cloned();
        self.into_error_with_terminal_events(
            all_children_reaped,
            supervisor.terminal_events(),
            supervisor.unknown_child_failure_count(),
            completed_nonzero,
            completed_hung,
            completed_hung,
            supervisor.children.len(),
            supervisor.observed_lifecycle_elapsed(),
            supervisor.lifecycle_bound(),
            supervisor.zero_exit_reap_observed_within_deadline(),
            supervisor.captured_zero_exit_reap_observation_elapsed(),
            fault_audit,
            retry_audit,
            owner_request_delivery_record,
        )
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3ProcessLocalnetRun {
    children: BTreeMap<I3LocalnetChildSlot, I3LocalnetChildAudit>,
    execution: I3LocalnetExecutionAudit,
    startup: I3LocalnetStartupAudit,
    transport: I3LocalnetTransportAudit,
    trace: I3LocalnetObserverSafeTrace,
    lifecycle: I3LocalnetLifecycleAudit,
    retry_audit: Option<I3LocalnetRetryAudit>,
    late_ingress_audit: Option<I3LocalnetLateIngressAudit>,
}

impl I3ProcessLocalnetRun {
    pub fn child(&self, slot: I3LocalnetChildSlot) -> Option<&I3LocalnetChildAudit> {
        self.children.get(&slot)
    }
    pub const fn startup_audit(&self) -> I3LocalnetStartupAudit {
        self.startup
    }
    pub fn execution_audit(&self) -> &I3LocalnetExecutionAudit {
        &self.execution
    }
    pub const fn transport_audit(&self) -> I3LocalnetTransportAudit {
        self.transport
    }
    pub fn observer_safe_trace(&self) -> &I3LocalnetObserverSafeTrace {
        &self.trace
    }
    pub const fn lifecycle(&self) -> I3LocalnetLifecycleAudit {
        self.lifecycle
    }
    pub fn retry_audit(&self) -> Option<&I3LocalnetRetryAudit> {
        self.retry_audit.as_ref()
    }

    pub fn late_ingress_audit(&self) -> Option<&I3LocalnetLateIngressAudit> {
        self.late_ingress_audit.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildControl {
    slot: I3LocalnetChildSlot,
    endpoint: Option<String>,
    trusted_runtime_control: Vec<u8>,
    ca_der: Vec<u8>,
    leaf_cert_der: Vec<u8>,
    // This is a probe-owned short-lived control copy. It never reaches argv,
    // env, a file, or an observer event. Zeroization makes no claim about
    // copies held internally by rcgen/rustls.
    leaf_key_der: Zeroizing<Vec<u8>>,
    inject_bad_preface: bool,
    // Test-only finite lifecycle falsifiers.  They affect descriptor delivery
    // or process liveness only; neither fabricates source/carrier/authority.
    stall_bootstrap: bool,
    stall_cleanup: bool,
    setup_failure_during_stall: bool,
    terminal_lifecycle_falsifier: PrivateChildTerminalLifecycleFalsifier,
    emit_request_before_wrong_peer_rejection: bool,
    // This finite schedule control is delivery-only.  It cannot alter the
    // checked route, carrier, owner, semantic request identity, or outcome.
    fault_profile: Option<I3LocalnetFaultProfile>,
    // A private, post-admission observer-event falsifier. It is not carried
    // in the generated request/reply traffic or runtime dispatch.
    fault_audit_falsifier: Option<I3LocalnetFaultAuditFalsifier>,
    // A requester-local observer-only wait, selected only after this child
    // has actually observed the selected reply-loss schedule.
    requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
    requester_local_wait_falsifier: Option<I3LocalnetRequesterLocalWaitFalsifier>,
    // I3-3's bounded two-session schedules.  These controls select only
    // actual adapter timing and post-send observer evidence; they never
    // transport a request, grant, expected result, or source operation.
    retry_profile: Option<I3LocalnetRetryProfile>,
    retry_falsifier: Option<I3LocalnetRetryFalsifier>,
    retry_audit_falsifier: Option<I3LocalnetRetryAuditFalsifier>,
    // The only owner-runtime scheduling hint carried to B. It selects no
    // semantic result; B acts only after an actual Awaiting disposition.
    owner_admission_drive_profile: Option<I3LocalnetOwnerAdmissionDriveProfile>,
    owner_reply_replay_profile: Option<I3LocalnetOwnerReplyReplayProfile>,
    owner_reply_replay_falsifier: Option<I3LocalnetOwnerReplyReplayFalsifier>,
    late_ingress_profile: Option<I3LocalnetLateIngressProfile>,
    late_ingress_falsifier: Option<I3LocalnetLateIngressFalsifier>,
    adapter_delivery_profile: Option<I3LocalnetAdapterDeliveryProfile>,
    // A negative-only candidate-shaped ACK frame for A's actual stdout route.
    // It cannot be decoded to an installed receipt or offered to the
    // registered-B reader; the parent may decode it only as tainted input.
    tainted_owner_lifecycle_ack_candidate: Option<Vec<u8>>,
    timeout_millis: u64,
}

/// Probe-owned, transport-only input carried beside the opaque runtime
/// authority in its one provider FD3 frame. This must never grow source,
/// path/resource, grant, request, result, or M9 fields: those remain in the
/// runtime-owned opaque half of the frame.
#[derive(Serialize, Deserialize)]
struct PrivateProviderChildTransportBootstrap {
    slot: I3LocalnetChildSlot,
    endpoint: Option<String>,
    // These three strings bind only this ephemeral QUIC run and its two TLS
    // leaves. They are neither Mir source/Core provenance nor authority.
    run_ref: String,
    local_spki_ref: String,
    peer_spki_ref: String,
    ca_der: Vec<u8>,
    leaf_cert_der: Vec<u8>,
    leaf_key_der: Zeroizing<Vec<u8>>,
    timeout_millis: u64,
    // The parent sets this only for A. It is a finite process-liveness
    // selector, never provider semantic input or runtime authority.
    requester_exit_nonzero_after_completed: bool,
}

/// Provider children receive only this physical half of the runtime-owned
/// FD3 frame. Reject duplicate members before ordinary serde decoding so a
/// compromised inherited descriptor cannot select a last-key-wins slot,
/// endpoint, or certificate record.
struct StrictPrivateProviderChildTransportBootstrap(PrivateProviderChildTransportBootstrap);

impl<'de> Deserialize<'de> for StrictPrivateProviderChildTransportBootstrap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrictProviderTransportVisitor;

        impl<'de> Visitor<'de> for StrictProviderTransportVisitor {
            type Value = StrictPrivateProviderChildTransportBootstrap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("provider transport bootstrap without duplicate members")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut fields = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if fields.contains_key(&key) {
                        return Err(A::Error::custom(
                            "duplicate provider transport bootstrap member",
                        ));
                    }
                    fields.insert(key, map.next_value::<serde_json::Value>()?);
                }
                serde_json::from_value(serde_json::Value::Object(fields))
                    .map(StrictPrivateProviderChildTransportBootstrap)
                    .map_err(A::Error::custom)
            }
        }

        deserializer.deserialize_map(StrictProviderTransportVisitor)
    }
}

fn decode_private_provider_child_transport_bootstrap(
    bytes: &[u8],
) -> Result<PrivateProviderChildTransportBootstrap, ()> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let bootstrap = StrictPrivateProviderChildTransportBootstrap::deserialize(&mut deserializer)
        .map_err(|_| ())?;
    deserializer.end().map_err(|_| ())?;
    Ok(bootstrap.0)
}

fn provider_transport_bootstrap_matches_slot(
    bootstrap: &PrivateProviderChildTransportBootstrap,
    slot: I3LocalnetChildSlot,
) -> bool {
    bootstrap.slot == slot
        && bootstrap
            .endpoint
            .as_deref()
            .is_none_or(is_loopback_endpoint)
        && !bootstrap.run_ref.is_empty()
        && !bootstrap.local_spki_ref.is_empty()
        && !bootstrap.peer_spki_ref.is_empty()
        && bootstrap.local_spki_ref != bootstrap.peer_spki_ref
        && !bootstrap.ca_der.is_empty()
        && !bootstrap.leaf_cert_der.is_empty()
        && !bootstrap.leaf_key_der.is_empty()
        && bootstrap.timeout_millis > 0
        && bootstrap.timeout_millis <= 15_000
        && (slot != I3LocalnetChildSlot::ProcessB
            || !bootstrap.requester_exit_nonzero_after_completed)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateChildTerminalLifecycleFalsifier {
    #[default]
    None,
    ExitNonzeroAfterCompleted,
    HangAfterCompleted,
    RejectAfterCompleted,
}

/// The trusted descriptor is still untrusted input from the child process's
/// perspective.  Reject duplicate top-level members before deserializing the
/// finite control DTO; `serde_json::Value` alone would silently apply
/// last-key-wins.  This DTO has no nested objects (only scalars/byte arrays),
/// so this covers every object member in this private control format.
struct StrictPrivateChildControl(PrivateChildControl);

impl<'de> Deserialize<'de> for StrictPrivateChildControl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrictControlVisitor;
        impl<'de> Visitor<'de> for StrictControlVisitor {
            type Value = StrictPrivateChildControl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("private child control without duplicate members")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut fields = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if fields.contains_key(&key) {
                        return Err(A::Error::custom("duplicate private child control member"));
                    }
                    fields.insert(key, map.next_value::<serde_json::Value>()?);
                }
                serde_json::from_value(serde_json::Value::Object(fields))
                    .map(StrictPrivateChildControl)
                    .map_err(A::Error::custom)
            }
        }
        deserializer.deserialize_map(StrictControlVisitor)
    }
}

fn decode_private_child_control(bytes: &[u8]) -> Result<PrivateChildControl, ()> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let control = StrictPrivateChildControl::deserialize(&mut deserializer).map_err(|_| ())?;
    deserializer.end().map_err(|_| ())?;
    Ok(control.0)
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum PrivateChildEvent {
    Ready {
        endpoint: String,
    },
    Completed {
        slot: I3LocalnetChildSlot,
        exec_confirmed: bool,
        assigned_loci: Vec<String>,
        trusted_control_consumed: bool,
        tainted_image_consumed: bool,
        tls_peer_verified: bool,
        reciprocal_preface_verified: bool,
        reliable_bidi_stream_count: usize,
        quic_datagrams_enabled: bool,
        semantic_admission_count: usize,
        unauthenticated_semantic_admission_count: usize,
        network_receipt_frame_count: usize,
        generated_request_count: usize,
        served_count: usize,
        write_count: usize,
        reply_count: usize,
        receipt_count: usize,
        runtime_occurrence_count: usize,
        observer_evidence: PrivateChildObserverEvidence,
    },
    /// Provider-specific terminal that deliberately reports only actual
    /// physical bootstrap and QUIC session facts plus one M9-authorized,
    /// strict reference-only observer-view frame. The frame is not an issued
    /// audit or permit: the parent must decode and correlate it with the
    /// owned provider run before it can become evidence. It contains no raw
    /// value, path, source text, grant, witness, or host-outcome payload.
    ProviderCompleted {
        slot: I3LocalnetChildSlot,
        assigned_loci: Vec<String>,
        provider_control_consumed: bool,
        tainted_image_consumed: bool,
        // This fixed fact is emitted only after A's runtime-owned finite T0
        // assertion has checked its actual local consume receipt. It carries
        // neither the selected profile nor a raw provider value or outcome.
        requester_fixture_assertion_completed: bool,
        tls_peer_verified: bool,
        reciprocal_preface_verified: bool,
        reliable_bidi_stream_count: usize,
        quic_datagrams_enabled: bool,
        terminal_observer_view_frame: Vec<u8>,
    },
    /// Generic terminal marker for the sealed fixed observer-conformance
    /// routes. It carries only the physical child slot; it is neither an
    /// audit export nor evidence of a particular observer verdict.
    ProviderTerminalObservationConformanceCompleted {
        slot: I3LocalnetChildSlot,
    },
    /// Generic terminal marker for the sealed fixed provider-network
    /// conformance routes. It carries only the physical child slot; the
    /// selected schedule, transport facts, and internal duplicate/rejection
    /// checks remain within the installed runtime.
    ProviderNetworkConformanceCompleted {
        slot: I3LocalnetChildSlot,
    },
    Rejected {
        rejection: PrivateChildRejection,
        fixed_control_descriptor_preserved: bool,
        owner_start_count: usize,
        certificate_initialization_count: usize,
        handshake_count: usize,
        semantic_admission_count: usize,
        owner_mutation_count: usize,
        evidence: PrivateChildRejectionEvidence,
    },
    HandledDeliveryFault {
        slot: I3LocalnetChildSlot,
        request_identity_ref: Option<String>,
        requester_observation: Option<I3LocalnetRequesterFaultObservation>,
        // Set only by A after its own runtime still retains the exact
        // source-generated request following an observed missing reply.
        // Other fault terminals make no requester-state claim.
        requester_pending_request_is_retained: Option<bool>,
        // Present only when A completed the selected actual local wait after
        // observing the reply loss. It is never a transport or semantic fact.
        requester_local_wait: Option<PrivateRequesterLocalWaitEvidence>,
        remote_admission: Option<PrivateRemoteAdmissionEvidence>,
        semantic_admission_count: usize,
        owner_mutation_count: usize,
        retry_evidence: Option<PrivateChildRetryEvidence>,
        owner_reply_replay_evidence: Option<PrivateChildOwnerReplyReplayEvidence>,
    },
    /// Exact child terminal for the selected adapter-delivery controls. It
    /// is still consumed by the existing supervisor/reap path, but retains
    /// the actual transport shape instead of borrowing the older fault
    /// profile's looser terminal contract.
    AdapterDeliveryFault {
        slot: I3LocalnetChildSlot,
        exec_confirmed: bool,
        assigned_loci: Vec<String>,
        trusted_control_consumed: bool,
        tainted_image_consumed: bool,
        tls_peer_verified: bool,
        reciprocal_preface_verified: bool,
        reliable_bidi_stream_count: usize,
        quic_datagrams_enabled: bool,
        request_identity_ref: Option<String>,
        requester_pending_request_is_retained: Option<bool>,
        adapter_delivery_failure: Option<I3LocalnetAdapterDeliveryFailure>,
        remote_admission: Option<PrivateRemoteAdmissionEvidence>,
        semantic_admission_count: usize,
        unauthenticated_semantic_admission_count: usize,
        owner_mutation_count: usize,
    },
    /// Dedicated terminal record for the bounded late-ingress schedules. It
    /// keeps the held-frame evidence distinct from ordinary completion and
    /// fault records, so a pre-admission frame can never borrow their decoded
    /// provenance fields.
    LateIngress {
        slot: I3LocalnetChildSlot,
        terminal_outcome: PrivateLateIngressTerminalOutcome,
        exec_confirmed: bool,
        assigned_loci: Vec<String>,
        trusted_control_consumed: bool,
        tainted_image_consumed: bool,
        tls_peer_verified: bool,
        reciprocal_preface_verified: bool,
        reliable_bidi_stream_count: usize,
        quic_datagrams_enabled: bool,
        semantic_admission_count: usize,
        unauthenticated_semantic_admission_count: usize,
        network_receipt_frame_count: usize,
        generated_request_count: usize,
        served_count: usize,
        write_count: usize,
        reply_count: usize,
        receipt_count: usize,
        runtime_occurrence_count: usize,
        observer_evidence: PrivateChildObserverEvidence,
        late_ingress_evidence: PrivateChildLateIngressEvidence,
    },
    /// A generic child error carries no trustworthy lifecycle, transport, or
    /// semantic counters. Keeping it separate from `Rejected` prevents an
    /// unobserved state from being rendered as an observed all-zero state.
    UnknownLifecycleFailure {
        slot: I3LocalnetChildSlot,
    },
    /// B alone records this exact branch after the ordinary private receive
    /// path has admitted a source-budgeted request but before a trusted owner
    /// runtime driver has resolved it. It exposes only fixed counters and
    /// verified transport booleans, never control bytes, keys, a clock handle,
    /// a permit, raw carrier bytes, or a semantic expected result.
    OwnerAdmissionAwaiting {
        slot: I3LocalnetChildSlot,
        fixed_control_descriptor_preserved: bool,
        owner_start_count: usize,
        certificate_initialization_count: usize,
        handshake_count: usize,
        trusted_control_consumed: bool,
        tainted_image_consumed: bool,
        tls_peer_verified: bool,
        reciprocal_preface_verified: bool,
        reliable_bidi_stream_count: usize,
        quic_datagrams_enabled: bool,
        semantic_admission_count: usize,
        owner_admission_awaiting_count: usize,
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateLateIngressTerminalOutcome {
    Completed,
    HandledDeliveryFault,
}

impl PrivateLateIngressTerminalOutcome {
    const fn public(self) -> I3LocalnetChildTerminalOutcome {
        match self {
            Self::Completed => I3LocalnetChildTerminalOutcome::Completed,
            Self::HandledDeliveryFault => I3LocalnetChildTerminalOutcome::HandledDeliveryFault,
        }
    }
}

/// Child-local evidence for the one late session-one ingress.  The retained
/// receiver evidence intentionally has no decoded carrier lineage until the
/// runtime's later successful admission returns an ordinary delivery record.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildLateIngressEvidence {
    run_ref: String,
    first_session_generation: u8,
    reconnect_session_generation: u8,
    first_session_peer_spki_verified: bool,
    first_session_reciprocal_preface_verified: bool,
    reconnect_session_peer_spki_verified: bool,
    reconnect_session_reciprocal_preface_verified: bool,
    initial_sender_delivery: Option<PrivateDeliveryEvidence>,
    retained_session_one_ingress: Option<PrivateRetainedIngressEvidence>,
    retained_ingress_repeat_acquisition_rejection: Option<I3LocalnetAdapterRejectionKind>,
    late_admission_delivery: Option<PrivateDeliveryEvidence>,
    owner_outcome: Option<PrivateLateIngressOwnerOutcome>,
    requester_outcome: Option<I3LocalnetLateIngressRequesterOutcome>,
    outbound_request_frame_write_count: Option<usize>,
    tainted_owner_lifecycle_ack_candidate: Option<Vec<u8>>,
    lifecycle_provenance: Option<I3LocalnetLateIngressLifecycleProvenance>,
    m9_lifecycle_source_derived: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateRetainedIngressEvidence {
    session_generation: u8,
    candidate_commitment_ref: String,
    network_occurrence_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
enum PrivateLateIngressOwnerOutcome {
    Admitted {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
    CarrierAdmissionRejected {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
}

/// Parent-side facts from the one actual registered-owner ACK reader. This
/// stays separate from child stdout events: no child can nominate a reader or
/// publication disposition by serializing an observer record.
#[derive(Clone, Copy)]
struct PrivateLateIngressParentObservation {
    ack_reader_outcome: I3LocalnetLateIngressAckReaderOutcome,
    nonregistered_ack_input_disposition: I3LocalnetLateIngressNonregisteredAckInputDisposition,
    nonregistered_ack_input_count: usize,
    publication: I3LocalnetLateIngressParentPublication,
    publication_commit_count: usize,
}

/// Serialized child evidence for the bounded retry schedules.  Its delivery
/// fields are copied only from adapter returns; its runtime-attempt summary is
/// copied only from the requester runtime before a successful receipt can
/// consume the pending record.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildRetryEvidence {
    run_ref: String,
    first_session_generation: u8,
    reconnect_session_generation: u8,
    first_session_peer_spki_verified: bool,
    first_session_reciprocal_preface_verified: bool,
    reconnect_session_peer_spki_verified: bool,
    reconnect_session_reciprocal_preface_verified: bool,
    // Set only after the requester receives an actual pending outcome and
    // confirms the runtime still retains that exact original request.
    requester_pending_request_is_retained: bool,
    first_session_runtime_attempt: Option<PrivateRetryAttemptEvidence>,
    reconnect_session_runtime_attempt: Option<PrivateRetryAttemptEvidence>,
    initial_request_delivery: Option<PrivateDeliveryEvidence>,
    reconnect_request_delivery: Option<PrivateDeliveryEvidence>,
    initial_owner_admission: Option<PrivateRemoteAdmissionEvidence>,
    initial_owner_expiry: Option<PrivateOwnerAdmissionExpiryEvidence>,
    reconnect_owner_outcome: Option<PrivateReconnectOwnerOutcome>,
}

/// A-only observer evidence captured around one actual, fixed local wait.
/// It deliberately contains no tick, session, carrier, source, or semantic
/// result. The parent accepts it only alongside the independently emitted B
/// remote-admission evidence for the existing loss route.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateRequesterLocalWaitEvidence {
    observed_elapsed: Duration,
    required_minimum_elapsed: Duration,
    pending_before: usize,
    pending_after: usize,
    local_receipt_before: usize,
    local_receipt_after: usize,
    terminal_failure_before: usize,
    terminal_failure_after: usize,
}

impl PrivateRequesterLocalWaitEvidence {
    fn public(&self) -> I3LocalnetRequesterLocalWaitAudit {
        I3LocalnetRequesterLocalWaitAudit::from_actual_observation(
            self.observed_elapsed,
            self.required_minimum_elapsed,
            self.pending_before,
            self.pending_after,
            self.local_receipt_before,
            self.local_receipt_after,
            self.terminal_failure_before,
            self.terminal_failure_after,
        )
    }

    fn is_exact(&self) -> bool {
        self.required_minimum_elapsed == REQUESTER_LOCAL_WAIT_MINIMUM
            && self.observed_elapsed >= self.required_minimum_elapsed
            && self.pending_before == 1
            && self.pending_after == 1
            && self.local_receipt_before == 0
            && self.local_receipt_after == 0
            && self.terminal_failure_before == 0
            && self.terminal_failure_after == 0
    }
}

/// B-local declared-expiry observation, formed only after the runtime returns
/// the generated reply and exposes its producer-derived decision record. It
/// deliberately contains no host tick, driver, carrier bytes, authority,
/// witness, or source text.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateOwnerAdmissionExpiryEvidence {
    // Retained privately only to join B's actual initial receive to the
    // requester-side first-session delivery in the reconnect audit. The
    // public expiry projection deliberately does not expose a carrier record.
    request_receive: Option<Box<PrivateDeliveryEvidence>>,
    decision_commitment_ref: String,
    decision_occurrence_ref: String,
    generated_reply_count: usize,
    expired_count: usize,
    owner_serve_count: usize,
    owner_mutation_count: usize,
}

impl PrivateOwnerAdmissionExpiryEvidence {
    fn public(&self) -> I3LocalnetOwnerAdmissionExpiryEvidence {
        I3LocalnetOwnerAdmissionExpiryEvidence {
            decision_commitment_ref: self.decision_commitment_ref.clone(),
            decision_occurrence_ref: self.decision_occurrence_ref.clone(),
            generated_reply_count: self.generated_reply_count,
            expired_count: self.expired_count,
            owner_serve_count: self.owner_serve_count,
            owner_mutation_count: self.owner_mutation_count,
        }
    }

    fn is_exact_declared_expiry(&self) -> bool {
        !self.decision_commitment_ref.is_empty()
            && !self.decision_occurrence_ref.is_empty()
            && self.generated_reply_count == 1
            && self.expired_count == 1
            && self.owner_serve_count == 0
            && self.owner_mutation_count == 0
    }
}

enum PrivateResolvedOwnerReply {
    Served(mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessMessage),
    DeclaredDeadlineExpired {
        reply: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessMessage,
        expiry: PrivateOwnerAdmissionExpiryEvidence,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateRetryAttemptEvidence {
    semantic_request_identity_ref: String,
    source_requester_locus: String,
    requester_binding_ref: String,
    attempt_generation: u8,
    reason: I3LocalnetRetryAttemptReason,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
enum PrivateReconnectOwnerOutcome {
    Replied {
        owner_serve_count: usize,
        owner_mutation_count: usize,
        owner_serve_occurrence_ref: String,
        owner_write_occurrence_ref: Option<String>,
    },
    DuplicateRequestRejected {
        candidate_commitment_ref: String,
        network_occurrence_ref: String,
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
    DuplicateRequestRejectedAfterDeclaredDeadlineExpired {
        candidate_commitment_ref: String,
        network_occurrence_ref: String,
        owner_expired_count: usize,
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
}

/// Private event payload from the owner child.  `NoAdmission` intentionally
/// carries no semantic request identity because no request frame arrived.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
enum PrivateRemoteAdmissionEvidence {
    NoAdmission {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
    Admitted {
        request_receive: Box<PrivateDeliveryEvidence>,
        owner_serve_count: usize,
        owner_mutation_count: usize,
        owner_serve_occurrence_ref: String,
        owner_write_occurrence_ref: Option<String>,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildRejectionEvidence {
    real_wrong_peer_delivery_attempted: bool,
    wrong_peer_certificate_chains_to_run_ca: bool,
    wrong_peer_spki_differs_from_expected: bool,
    requester_observer_state_before: String,
    requester_observer_state_after: String,
    requester_pending_request_is_retained: bool,
    adapter_rejection_kind: Option<I3LocalnetAdapterRejectionKind>,
    wrong_peer_ca_validated_leaf_ref: Option<String>,
    expected_peer_spki_ref: Option<String>,
    actual_peer_spki_ref: Option<String>,
    requester_first_session_peer_spki_verified: bool,
    requester_first_session_reciprocal_preface_verified: bool,
}

struct PrivateChildCompleted {
    slot: I3LocalnetChildSlot,
    exec_confirmed: bool,
    assigned_loci: Vec<String>,
    trusted_control_consumed: bool,
    tainted_image_consumed: bool,
    tls_peer_verified: bool,
    reciprocal_preface_verified: bool,
    reliable_bidi_stream_count: usize,
    quic_datagrams_enabled: bool,
    semantic_admission_count: usize,
    unauthenticated_semantic_admission_count: usize,
    network_receipt_frame_count: usize,
    generated_request_count: usize,
    served_count: usize,
    write_count: usize,
    reply_count: usize,
    receipt_count: usize,
    runtime_occurrence_count: usize,
    observer_evidence: PrivateChildObserverEvidence,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildObserverEvidence {
    request_sent: Option<PrivateDeliveryEvidence>,
    request_received: Option<PrivateDeliveryEvidence>,
    reply_sent: Option<PrivateDeliveryEvidence>,
    reply_received: Option<PrivateDeliveryEvidence>,
    local_store_ref: String,
    owner_serve_occurrence_ref: Option<String>,
    owner_write_occurrence_ref: Option<String>,
    requester_receipt_occurrence_ref: Option<String>,
    requester_terminal_failure_occurrence_ref: Option<String>,
    owner_expiry: Option<PrivateOwnerAdmissionExpiryEvidence>,
    retry_evidence: Option<PrivateChildRetryEvidence>,
}

/// Child-local facts from the bounded T0 generated-reply replay.  Every
/// transport and runtime field is copied after its actual event; no profile
/// value creates a semantic outcome, source identity, or session fact.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateChildOwnerReplyReplayEvidence {
    run_ref: String,
    cohort_provenance_ref: String,
    first_session_generation: u8,
    first_session_peer_spki_verified: bool,
    first_session_reciprocal_preface_verified: bool,
    replay_session_generation: Option<u8>,
    replay_session_peer_spki_verified: Option<bool>,
    replay_session_reciprocal_preface_verified: Option<bool>,
    first_request_send: Option<PrivateDeliveryEvidence>,
    first_request_receive: Option<PrivateDeliveryEvidence>,
    first_reply_send: Option<PrivateDeliveryEvidence>,
    first_reply_receive: Option<PrivateDeliveryEvidence>,
    token_original_first_reply_send_occurrence_ref: Option<String>,
    token_original_first_reply_carrier_ref: Option<String>,
    requester_first_outcome: Option<PrivateOwnerReplyReplayFirstOutcome>,
    requester_state_after_first: Option<PrivateOwnerReplyReplayRequesterState>,
    requester_final_state: Option<PrivateOwnerReplyReplayRequesterState>,
    owner_state_after_first: Option<PrivateOwnerReplyReplayOwnerState>,
    owner_state_after_replay_rejection: Option<PrivateOwnerReplyReplayOwnerState>,
    owner_expiry: Option<PrivateOwnerAdmissionExpiryEvidence>,
    replay_reply_send: Option<PrivateDeliveryEvidence>,
    receiver_rejection: Option<PrivateOwnerReplyReplayReceiverRejection>,
    replay_prewrite_rejection: Option<I3LocalnetAdapterRejectionKind>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateOwnerReplyReplayFirstOutcome {
    ReceiptConsumed,
    TerminalFailureConsumed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateOwnerReplyReplayRequesterState {
    pending_request_count: usize,
    receipt_count: usize,
    terminal_failure_count: usize,
    receipt_occurrence_ref: Option<String>,
    terminal_failure_occurrence_ref: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateOwnerReplyReplayOwnerState {
    tombstone_count: usize,
    served_owner_request_count: usize,
    owner_mutation_count: usize,
    expired_owner_admission_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
enum PrivateOwnerReplyReplayReceiverRejection {
    CarrierAdmissionRejected {
        rejected_candidate_commitment_ref: String,
        network_occurrence_ref: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateDeliveryEvidence {
    carrier_ref: String,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    network_occurrence_ref: String,
    candidate_commitment_ref: String,
    generated_frame_write_observation: Option<PrivateGeneratedFrameWriteObservation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateGeneratedFrameWriteObservation {
    application_write_count: u8,
    frame_prefix_split_across_writes: bool,
    complete_frame_written: bool,
}

impl From<mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicDeliveryEvidence>
    for PrivateDeliveryEvidence
{
    fn from(value: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicDeliveryEvidence) -> Self {
        Self {
            carrier_ref: value.carrier_ref().to_string(),
            semantic_request_identity_ref: value.semantic_request_identity_ref().to_string(),
            linked_request_identity_ref: value.linked_request_identity_ref().map(str::to_string),
            source_ref: value.source_ref().to_string(),
            core_ref: value.core_ref().to_string(),
            source_artifact_ref: value.source_artifact_ref().to_string(),
            target_artifact_ref: value.target_artifact_ref().to_string(),
            edge_ref: value.edge_ref().to_string(),
            network_occurrence_ref: value.network_occurrence_ref().to_string(),
            candidate_commitment_ref: value.candidate_commitment_ref().to_string(),
            generated_frame_write_observation: value.generated_frame_write_observation().map(
                |observation| PrivateGeneratedFrameWriteObservation {
                    application_write_count: observation.application_write_count(),
                    frame_prefix_split_across_writes: observation
                        .frame_prefix_split_across_writes(),
                    complete_frame_written: observation.complete_frame_written(),
                },
            ),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct PrivateQuicTransportEvidence {
    datagram_receive_enabled: bool,
    datagram_send_enabled: bool,
}

impl PrivateQuicTransportEvidence {
    #[allow(dead_code)]
    fn datagrams_enabled(self) -> bool {
        self.datagram_receive_enabled || self.datagram_send_enabled
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateChildRejection {
    StartBinding,
    PeerBinding,
    Lifecycle,
}

impl PrivateChildEvent {
    fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed { .. }
                | Self::ProviderCompleted { .. }
                | Self::ProviderTerminalObservationConformanceCompleted { .. }
                | Self::ProviderNetworkConformanceCompleted { .. }
                | Self::Rejected { .. }
                | Self::HandledDeliveryFault { .. }
                | Self::AdapterDeliveryFault { .. }
                | Self::LateIngress { .. }
                | Self::UnknownLifecycleFailure { .. }
                | Self::OwnerAdmissionAwaiting { .. }
        )
    }

    fn is_completed(&self) -> bool {
        matches!(
            self,
            Self::Completed { .. }
                | Self::ProviderCompleted { .. }
                | Self::LateIngress {
                    terminal_outcome: PrivateLateIngressTerminalOutcome::Completed,
                    ..
                }
        )
    }

    fn retry_evidence(&self) -> Option<&PrivateChildRetryEvidence> {
        match self {
            Self::Completed {
                observer_evidence, ..
            } => observer_evidence.retry_evidence.as_ref(),
            Self::HandledDeliveryFault { retry_evidence, .. } => retry_evidence.as_ref(),
            Self::Ready { .. }
            | Self::ProviderCompleted { .. }
            | Self::ProviderTerminalObservationConformanceCompleted { .. }
            | Self::ProviderNetworkConformanceCompleted { .. }
            | Self::Rejected { .. }
            | Self::AdapterDeliveryFault { .. }
            | Self::LateIngress { .. }
            | Self::UnknownLifecycleFailure { .. }
            | Self::OwnerAdmissionAwaiting { .. } => None,
        }
    }

    fn owner_reply_replay_evidence(&self) -> Option<&PrivateChildOwnerReplyReplayEvidence> {
        match self {
            Self::HandledDeliveryFault {
                owner_reply_replay_evidence,
                ..
            } => owner_reply_replay_evidence.as_ref(),
            Self::Ready { .. }
            | Self::Completed { .. }
            | Self::ProviderCompleted { .. }
            | Self::ProviderTerminalObservationConformanceCompleted { .. }
            | Self::ProviderNetworkConformanceCompleted { .. }
            | Self::Rejected { .. }
            | Self::AdapterDeliveryFault { .. }
            | Self::LateIngress { .. }
            | Self::UnknownLifecycleFailure { .. }
            | Self::OwnerAdmissionAwaiting { .. } => None,
        }
    }

    fn reported_semantic_admission_count(&self) -> Option<usize> {
        match self {
            Self::Completed {
                semantic_admission_count,
                ..
            }
            | Self::Rejected {
                semantic_admission_count,
                ..
            }
            | Self::HandledDeliveryFault {
                semantic_admission_count,
                ..
            }
            | Self::AdapterDeliveryFault {
                semantic_admission_count,
                ..
            }
            | Self::LateIngress {
                semantic_admission_count,
                ..
            }
            | Self::OwnerAdmissionAwaiting {
                semantic_admission_count,
                ..
            } => Some(*semantic_admission_count),
            Self::Ready { .. }
            | Self::ProviderCompleted { .. }
            | Self::ProviderTerminalObservationConformanceCompleted { .. }
            | Self::ProviderNetworkConformanceCompleted { .. }
            | Self::UnknownLifecycleFailure { .. } => None,
        }
    }

    fn terminal_event(&self) -> Option<I3LocalnetChildTerminalEvent> {
        match self {
            Self::Completed {
                slot,
                trusted_control_consumed,
                unauthenticated_semantic_admission_count,
                semantic_admission_count,
                served_count,
                write_count,
                tls_peer_verified,
                reciprocal_preface_verified,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: Some(*slot),
                outcome: I3LocalnetChildTerminalOutcome::Completed,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *write_count,
                // `served_count` is meaningful as an owner-runtime count
                // only for B. A completed requester report must not turn its
                // own zero into a claim about B's owner work.
                owner_serve_count: (*slot == I3LocalnetChildSlot::ProcessB)
                    .then_some(*served_count),
                trusted_control_consumed: Some(*trusted_control_consumed),
                unauthenticated_semantic_admission_count: Some(
                    *unauthenticated_semantic_admission_count,
                ),
                tls_peer_verified: Some(*tls_peer_verified),
                reciprocal_preface_verified: Some(*reciprocal_preface_verified),
                owner_admission_awaiting_count: None,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::Rejected {
                semantic_admission_count,
                owner_mutation_count,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: None,
                outcome: I3LocalnetChildTerminalOutcome::Rejected,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *owner_mutation_count,
                owner_serve_count: None,
                trusted_control_consumed: None,
                unauthenticated_semantic_admission_count: None,
                tls_peer_verified: None,
                reciprocal_preface_verified: None,
                owner_admission_awaiting_count: None,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::HandledDeliveryFault {
                slot,
                semantic_admission_count,
                owner_mutation_count,
                remote_admission,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: Some(*slot),
                outcome: I3LocalnetChildTerminalOutcome::HandledDeliveryFault,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *owner_mutation_count,
                owner_serve_count: match remote_admission {
                    Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                        owner_serve_count, ..
                    })
                    | Some(PrivateRemoteAdmissionEvidence::Admitted {
                        owner_serve_count, ..
                    }) => Some(*owner_serve_count),
                    None => None,
                },
                trusted_control_consumed: None,
                unauthenticated_semantic_admission_count: None,
                tls_peer_verified: None,
                reciprocal_preface_verified: None,
                owner_admission_awaiting_count: None,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::AdapterDeliveryFault {
                slot,
                trusted_control_consumed,
                tls_peer_verified,
                reciprocal_preface_verified,
                remote_admission,
                semantic_admission_count,
                unauthenticated_semantic_admission_count,
                owner_mutation_count,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: Some(*slot),
                outcome: I3LocalnetChildTerminalOutcome::HandledDeliveryFault,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *owner_mutation_count,
                owner_serve_count: match remote_admission {
                    Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                        owner_serve_count, ..
                    })
                    | Some(PrivateRemoteAdmissionEvidence::Admitted {
                        owner_serve_count, ..
                    }) => Some(*owner_serve_count),
                    None => None,
                },
                trusted_control_consumed: Some(*trusted_control_consumed),
                unauthenticated_semantic_admission_count: Some(
                    *unauthenticated_semantic_admission_count,
                ),
                tls_peer_verified: Some(*tls_peer_verified),
                reciprocal_preface_verified: Some(*reciprocal_preface_verified),
                owner_admission_awaiting_count: None,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::LateIngress {
                slot,
                terminal_outcome,
                trusted_control_consumed,
                unauthenticated_semantic_admission_count,
                semantic_admission_count,
                served_count,
                write_count,
                tls_peer_verified,
                reciprocal_preface_verified,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: Some(*slot),
                outcome: terminal_outcome.public(),
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *write_count,
                // As above, preserve an actual owner-runtime observation
                // only from the owner slot; requester late-ingress records
                // cannot manufacture a zero owner-serve conclusion.
                owner_serve_count: (*slot == I3LocalnetChildSlot::ProcessB)
                    .then_some(*served_count),
                trusted_control_consumed: Some(*trusted_control_consumed),
                unauthenticated_semantic_admission_count: Some(
                    *unauthenticated_semantic_admission_count,
                ),
                tls_peer_verified: Some(*tls_peer_verified),
                reciprocal_preface_verified: Some(*reciprocal_preface_verified),
                owner_admission_awaiting_count: None,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::OwnerAdmissionAwaiting {
                slot,
                trusted_control_consumed,
                tls_peer_verified,
                reciprocal_preface_verified,
                semantic_admission_count,
                owner_admission_awaiting_count,
                owner_serve_count,
                owner_mutation_count,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                slot: Some(*slot),
                outcome: I3LocalnetChildTerminalOutcome::OwnerAdmissionAwaiting,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *owner_mutation_count,
                owner_serve_count: Some(*owner_serve_count),
                trusted_control_consumed: Some(*trusted_control_consumed),
                unauthenticated_semantic_admission_count: Some(0),
                tls_peer_verified: Some(*tls_peer_verified),
                reciprocal_preface_verified: Some(*reciprocal_preface_verified),
                owner_admission_awaiting_count: Some(*owner_admission_awaiting_count),
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::Ready { .. }
            | Self::ProviderCompleted { .. }
            | Self::ProviderTerminalObservationConformanceCompleted { .. }
            | Self::ProviderNetworkConformanceCompleted { .. }
            | Self::UnknownLifecycleFailure { .. } => None,
        }
    }

    fn rejected(
        rejection: PrivateChildRejection,
        fixed_control_descriptor_preserved: bool,
        owner_start_count: usize,
        certificate_initialization_count: usize,
        handshake_count: usize,
        semantic_admission_count: usize,
        owner_mutation_count: usize,
    ) -> Self {
        Self::Rejected {
            rejection,
            fixed_control_descriptor_preserved,
            owner_start_count,
            certificate_initialization_count,
            handshake_count,
            semantic_admission_count,
            owner_mutation_count,
            evidence: PrivateChildRejectionEvidence::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rejected_with_evidence(
        rejection: PrivateChildRejection,
        fixed_control_descriptor_preserved: bool,
        owner_start_count: usize,
        certificate_initialization_count: usize,
        handshake_count: usize,
        semantic_admission_count: usize,
        owner_mutation_count: usize,
        evidence: PrivateChildRejectionEvidence,
    ) -> Self {
        Self::Rejected {
            rejection,
            fixed_control_descriptor_preserved,
            owner_start_count,
            certificate_initialization_count,
            handshake_count,
            semantic_admission_count,
            owner_mutation_count,
            evidence,
        }
    }

    fn into_completed(self) -> Option<PrivateChildCompleted> {
        let (
            slot,
            exec_confirmed,
            assigned_loci,
            trusted_control_consumed,
            tainted_image_consumed,
            tls_peer_verified,
            reciprocal_preface_verified,
            reliable_bidi_stream_count,
            quic_datagrams_enabled,
            semantic_admission_count,
            unauthenticated_semantic_admission_count,
            network_receipt_frame_count,
            generated_request_count,
            served_count,
            write_count,
            reply_count,
            receipt_count,
            runtime_occurrence_count,
            observer_evidence,
        ) = match self {
            Self::Completed {
                slot,
                exec_confirmed,
                assigned_loci,
                trusted_control_consumed,
                tainted_image_consumed,
                tls_peer_verified,
                reciprocal_preface_verified,
                reliable_bidi_stream_count,
                quic_datagrams_enabled,
                semantic_admission_count,
                unauthenticated_semantic_admission_count,
                network_receipt_frame_count,
                generated_request_count,
                served_count,
                write_count,
                reply_count,
                receipt_count,
                runtime_occurrence_count,
                observer_evidence,
            }
            | Self::LateIngress {
                terminal_outcome: PrivateLateIngressTerminalOutcome::Completed,
                slot,
                exec_confirmed,
                assigned_loci,
                trusted_control_consumed,
                tainted_image_consumed,
                tls_peer_verified,
                reciprocal_preface_verified,
                reliable_bidi_stream_count,
                quic_datagrams_enabled,
                semantic_admission_count,
                unauthenticated_semantic_admission_count,
                network_receipt_frame_count,
                generated_request_count,
                served_count,
                write_count,
                reply_count,
                receipt_count,
                runtime_occurrence_count,
                observer_evidence,
                ..
            } => (
                slot,
                exec_confirmed,
                assigned_loci,
                trusted_control_consumed,
                tainted_image_consumed,
                tls_peer_verified,
                reciprocal_preface_verified,
                reliable_bidi_stream_count,
                quic_datagrams_enabled,
                semantic_admission_count,
                unauthenticated_semantic_admission_count,
                network_receipt_frame_count,
                generated_request_count,
                served_count,
                write_count,
                reply_count,
                receipt_count,
                runtime_occurrence_count,
                observer_evidence,
            ),
            _ => return None,
        };
        Some(PrivateChildCompleted {
            slot,
            exec_confirmed,
            assigned_loci,
            trusted_control_consumed,
            tainted_image_consumed,
            tls_peer_verified,
            reciprocal_preface_verified,
            reliable_bidi_stream_count,
            quic_datagrams_enabled,
            semantic_admission_count,
            unauthenticated_semantic_admission_count,
            network_receipt_frame_count,
            generated_request_count,
            served_count,
            write_count,
            reply_count,
            receipt_count,
            runtime_occurrence_count,
            observer_evidence,
        })
    }
}

struct LeafMaterial {
    certificate_der: Vec<u8>,
    private_key_der: Zeroizing<Vec<u8>>,
    spki_ref: String,
}

struct RunCredentials {
    ca_der: Vec<u8>,
    process_a: LeafMaterial,
    process_b: LeafMaterial,
    wrong_peer: LeafMaterial,
}

struct SpawnedChild {
    slot: I3LocalnetChildSlot,
    child: Child,
    events: Receiver<Result<PrivateChildEvent, ()>>,
    reader: Option<thread::JoinHandle<()>>,
    bootstrap_done: Receiver<io::Result<()>>,
    bootstrap: Option<thread::JoinHandle<()>>,
    bootstrap_complete: bool,
    reaped: bool,
    observed_exit_status: Option<ExitStatus>,
    was_force_killed: bool,
    terminal_event: Option<PrivateChildEvent>,
    // Present only for the actual ProcessB G2 launch. This parent endpoint is
    // moved exactly once into the runtime-owned registered reader; callers
    // cannot write an ACK or substitute a child identity for it.
    owner_lifecycle_ack_stream: Option<UnixStream>,
}

struct LocalnetSupervisor {
    lifecycle_started: Instant,
    deadline: Instant,
    total_lifecycle_deadline: Instant,
    natural_reaper_exhausted: bool,
    zero_exit_reap_observed_at: Option<Instant>,
    zero_exit_reap_observation_elapsed: Option<Duration>,
    zero_exit_reap_observed_within_deadline: bool,
    children: Vec<SpawnedChild>,
    // The opaque runtime launch retains the trusted setup guard until this
    // supervisor has naturally or forcibly reaped every provider child.
    provider_launch: Option<Sys5I3PreparedProviderLocalnetLaunch>,
}

enum ProviderChildSpawnError {
    Rejected {
        bootstrap_substage: Option<I3ReadOnlyProviderLocalnetBootstrapSubstage>,
        provider_launch_error_kind: Option<Sys5I3ProviderLaunchErrorKind>,
    },
}

impl ProviderChildSpawnError {
    const fn physical(bootstrap_substage: I3ReadOnlyProviderLocalnetBootstrapSubstage) -> Self {
        Self::Rejected {
            bootstrap_substage: Some(bootstrap_substage),
            provider_launch_error_kind: None,
        }
    }

    const fn runtime(
        bootstrap_substage: I3ReadOnlyProviderLocalnetBootstrapSubstage,
        error: &Sys5I3ProviderLaunchError,
    ) -> Self {
        Self::Rejected {
            bootstrap_substage: Some(bootstrap_substage),
            provider_launch_error_kind: Some(error.kind()),
        }
    }

    const fn deadline() -> Self {
        Self::Rejected {
            bootstrap_substage: None,
            provider_launch_error_kind: None,
        }
    }
}

struct ProviderRunFailure {
    stage: I3ReadOnlyProviderLocalnetFailureStage,
    bootstrap_substage: Option<I3ReadOnlyProviderLocalnetBootstrapSubstage>,
    provider_launch_error_kind: Option<Sys5I3ProviderLaunchErrorKind>,
}

impl ProviderRunFailure {
    const fn io(stage: I3ReadOnlyProviderLocalnetFailureStage) -> Self {
        Self {
            stage,
            bootstrap_substage: None,
            provider_launch_error_kind: None,
        }
    }

    const fn bootstrap(
        stage: I3ReadOnlyProviderLocalnetFailureStage,
        error: ProviderChildSpawnError,
    ) -> Self {
        let ProviderChildSpawnError::Rejected {
            bootstrap_substage,
            provider_launch_error_kind,
        } = error;
        Self {
            stage,
            bootstrap_substage,
            provider_launch_error_kind,
        }
    }
}

#[derive(Clone)]
struct SupervisorLineageEvidence {
    ordinary_source_build_count: usize,
    admission_count: usize,
    m9_generation_count: usize,
    request_contract: Sys5I3AdapterCarrierContract,
    reply_contract: Sys5I3AdapterCarrierContract,
}

/// This is the deliberately fail-closed Stage3 provider-runner seam. The
/// opaque launch is accepted only by move, while this probe boundary accepts
/// only physical whole-run supervision limits. It has no path to recover
/// control bytes, an image, a capability, a fixture value, or a semantic
/// result from the launch. It registers an executor child before the one-shot
/// opaque image/control handoff and reaps it on every partial-launch failure;
/// it cannot construct a successful audit without the child runtime's checked
/// install, carrier, and separately permitted observer paths.
#[doc(hidden)]
pub fn run_i3_read_only_provider_localnet(
    launch: Sys5I3PreparedProviderLocalnetLaunch,
    request: I3ReadOnlyProviderLocalnetRequest,
) -> Result<I3ReadOnlyProviderLocalnetRun, I3ReadOnlyProviderLocalnetRunError> {
    match run_i3_read_only_provider_localnet_with_terminal_expectation(
        launch,
        request,
        PrivateProviderTerminalExpectation::NormalAudit,
    )? {
        PrivateProviderTerminalCompletion::NormalAudit(run) => Ok(*run),
        PrivateProviderTerminalCompletion::FixedTerminalObservationConformance(_)
        | PrivateProviderTerminalCompletion::FixedProviderNetworkConformance(_) => {
            Err(I3ReadOnlyProviderLocalnetRunError::at(
                I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
                I3ReadOnlyProviderLocalnetFailureStage::Preflight,
            ))
        }
    }
}

/// Runs one source-real provider launch selected by a sealed fixed
/// terminal-observation conformance factory. The opaque completion provides
/// no audit, profile, failure reason, result, or authority.
#[doc(hidden)]
pub fn run_i3_read_only_provider_terminal_observer_conformance(
    launch: Sys5I3PreparedProviderLocalnetLaunch,
    request: I3ReadOnlyProviderLocalnetRequest,
) -> Result<I3ReadOnlyProviderTerminalObservationConformanceRun, I3ReadOnlyProviderLocalnetRunError>
{
    match run_i3_read_only_provider_localnet_with_terminal_expectation(
        launch,
        request,
        PrivateProviderTerminalExpectation::FixedTerminalObservationConformance,
    )? {
        PrivateProviderTerminalCompletion::FixedTerminalObservationConformance(completion) => {
            Ok(completion)
        }
        PrivateProviderTerminalCompletion::NormalAudit(_)
        | PrivateProviderTerminalCompletion::FixedProviderNetworkConformance(_) => {
            Err(I3ReadOnlyProviderLocalnetRunError::at(
                I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
                I3ReadOnlyProviderLocalnetFailureStage::Preflight,
            ))
        }
    }
}

/// Runs one source-real provider launch selected by a sealed fixed
/// provider-network conformance factory. The opaque completion is only the
/// child-private profile's pass/fail result; it carries no fault record,
/// result, occurrence, or observer audit.
#[doc(hidden)]
pub fn run_i3_read_only_provider_network_conformance(
    launch: Sys5I3PreparedProviderLocalnetLaunch,
    request: I3ReadOnlyProviderLocalnetRequest,
) -> Result<I3ReadOnlyProviderNetworkConformanceRun, I3ReadOnlyProviderLocalnetRunError> {
    match run_i3_read_only_provider_localnet_with_terminal_expectation(
        launch,
        request,
        PrivateProviderTerminalExpectation::FixedProviderNetworkConformance,
    )? {
        PrivateProviderTerminalCompletion::FixedProviderNetworkConformance(completion) => {
            Ok(completion)
        }
        PrivateProviderTerminalCompletion::NormalAudit(_)
        | PrivateProviderTerminalCompletion::FixedTerminalObservationConformance(_) => {
            Err(I3ReadOnlyProviderLocalnetRunError::at(
                I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
                I3ReadOnlyProviderLocalnetFailureStage::Preflight,
            ))
        }
    }
}

fn run_i3_read_only_provider_localnet_with_terminal_expectation(
    launch: Sys5I3PreparedProviderLocalnetLaunch,
    request: I3ReadOnlyProviderLocalnetRequest,
    terminal_expectation: PrivateProviderTerminalExpectation,
) -> Result<PrivateProviderTerminalCompletion, I3ReadOnlyProviderLocalnetRunError> {
    let sealed_terminal_observation = launch.has_fixed_terminal_observation_conformance_profile();
    let sealed_provider_network = launch.has_fixed_provider_network_conformance_profile();
    let expectation_matches_sealed_profile = match terminal_expectation {
        PrivateProviderTerminalExpectation::NormalAudit => {
            !sealed_terminal_observation && !sealed_provider_network
        }
        PrivateProviderTerminalExpectation::FixedTerminalObservationConformance => {
            sealed_terminal_observation && !sealed_provider_network
        }
        PrivateProviderTerminalExpectation::FixedProviderNetworkConformance => {
            !sealed_terminal_observation && sealed_provider_network
        }
    };
    if !expectation_matches_sealed_profile {
        return Err(I3ReadOnlyProviderLocalnetRunError::at(
            I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
            I3ReadOnlyProviderLocalnetFailureStage::Preflight,
        ));
    }
    if request.whole_run_deadline.is_zero() || request.whole_run_deadline > Duration::from_secs(15)
    {
        return Err(I3ReadOnlyProviderLocalnetRunError::new(
            I3ReadOnlyProviderLocalnetRunErrorKind::InvalidWholeRunDeadline,
        ));
    }
    if request.reaper_allowance < LIFECYCLE_REAP_RESERVE
        || request.reaper_allowance > Duration::from_secs(1)
    {
        return Err(I3ReadOnlyProviderLocalnetRunError::new(
            I3ReadOnlyProviderLocalnetRunErrorKind::InvalidReaperAllowance,
        ));
    }

    let credentials = generate_run_credentials().map_err(|_| {
        I3ReadOnlyProviderLocalnetRunError::at(
            I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
            I3ReadOnlyProviderLocalnetFailureStage::CredentialProvision,
        )
    })?;
    let RunCredentials {
        ca_der,
        process_a,
        process_b,
        wrong_peer: _,
    } = credentials;
    let provider_run_ref =
        fresh_provider_transport_run_ref(&process_a.spki_ref, &process_b.spki_ref);
    let requester_spki_ref = process_a.spki_ref.clone();
    let executor_spki_ref = process_b.spki_ref.clone();
    let lifecycle_started = Instant::now();
    let main_deadline = lifecycle_started + request.whole_run_deadline;
    let mut supervisor = LocalnetSupervisor {
        lifecycle_started,
        deadline: main_deadline,
        total_lifecycle_deadline: main_deadline + request.reaper_allowance,
        natural_reaper_exhausted: false,
        zero_exit_reap_observed_at: None,
        zero_exit_reap_observation_elapsed: None,
        zero_exit_reap_observed_within_deadline: false,
        children: Vec::new(),
        provider_launch: Some(launch),
    };
    let transport = PrivateProviderChildTransportBootstrap {
        slot: I3LocalnetChildSlot::ProcessB,
        endpoint: None,
        run_ref: provider_run_ref.clone(),
        local_spki_ref: executor_spki_ref.clone(),
        peer_spki_ref: requester_spki_ref.clone(),
        ca_der: ca_der.clone(),
        leaf_cert_der: process_b.certificate_der,
        leaf_key_der: process_b.private_key_der,
        timeout_millis: request
            .whole_run_deadline
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX),
        requester_exit_nonzero_after_completed: false,
    };
    let provider_execution =
        (|| -> Result<PrivateProviderTerminalCompletion, ProviderRunFailure> {
            supervisor
                .spawn_provider_child(transport)
                .map_err(|error| {
                    ProviderRunFailure::bootstrap(
                        I3ReadOnlyProviderLocalnetFailureStage::ExecutorBootstrap,
                        error,
                    )
                })?;
            let endpoint = match supervisor
                .next_event(I3LocalnetChildSlot::ProcessB)
                .map_err(|_| {
                    ProviderRunFailure::io(I3ReadOnlyProviderLocalnetFailureStage::ExecutorReady)
                })? {
                PrivateChildEvent::Ready { endpoint } if is_loopback_endpoint(&endpoint) => {
                    endpoint
                }
                _ => {
                    return Err(ProviderRunFailure::io(
                        I3ReadOnlyProviderLocalnetFailureStage::ExecutorReady,
                    ));
                }
            };
            let requester_transport = PrivateProviderChildTransportBootstrap {
                slot: I3LocalnetChildSlot::ProcessA,
                endpoint: Some(endpoint.clone()),
                run_ref: provider_run_ref,
                local_spki_ref: requester_spki_ref,
                peer_spki_ref: executor_spki_ref,
                ca_der,
                leaf_cert_der: process_a.certificate_der,
                leaf_key_der: process_a.private_key_der,
                timeout_millis: request
                    .whole_run_deadline
                    .as_millis()
                    .try_into()
                    .unwrap_or(u64::MAX),
                requester_exit_nonzero_after_completed: request
                    .requester_exit_nonzero_after_completed,
            };
            supervisor
                .spawn_provider_child(requester_transport)
                .map_err(|error| {
                    ProviderRunFailure::bootstrap(
                        I3ReadOnlyProviderLocalnetFailureStage::RequesterBootstrap,
                        error,
                    )
                })?;
            let executor_terminal = supervisor
                .next_event(I3LocalnetChildSlot::ProcessB)
                .map_err(|_| {
                    ProviderRunFailure::io(I3ReadOnlyProviderLocalnetFailureStage::ExecutorTerminal)
                })?;
            let requester_terminal = supervisor
                .next_event(I3LocalnetChildSlot::ProcessA)
                .map_err(|_| {
                    ProviderRunFailure::io(
                        I3ReadOnlyProviderLocalnetFailureStage::RequesterTerminal,
                    )
                })?;
            let terminal_completion = match terminal_expectation {
                PrivateProviderTerminalExpectation::NormalAudit => {
                    let executor_terminal_observer_view = provider_completion_observer_view(
                        executor_terminal,
                        I3LocalnetChildSlot::ProcessB,
                    )
                    .map_err(|_| {
                        ProviderRunFailure::io(
                            I3ReadOnlyProviderLocalnetFailureStage::TerminalObservationCorrelation,
                        )
                    })?;
                    let requester_terminal_observer_view = provider_completion_observer_view(
                        requester_terminal,
                        I3LocalnetChildSlot::ProcessA,
                    )
                    .map_err(|_| {
                        ProviderRunFailure::io(
                            I3ReadOnlyProviderLocalnetFailureStage::TerminalObservationCorrelation,
                        )
                    })?;
                    if !provider_terminal_observer_views_match(
                        &requester_terminal_observer_view,
                        &executor_terminal_observer_view,
                    ) {
                        return Err(ProviderRunFailure::io(
                            I3ReadOnlyProviderLocalnetFailureStage::TerminalObservationCorrelation,
                        ));
                    }
                    PrivateProviderTerminalCompletion::NormalAudit(Box::new(
                        I3ReadOnlyProviderLocalnetRun {
                            requester_terminal_observer_view,
                            executor_terminal_observer_view,
                        },
                    ))
                }
                PrivateProviderTerminalExpectation::FixedTerminalObservationConformance => {
                    provider_terminal_observation_conformance_completed(
                        executor_terminal,
                        I3LocalnetChildSlot::ProcessB,
                    )
                    .and_then(|_| {
                        provider_terminal_observation_conformance_completed(
                            requester_terminal,
                            I3LocalnetChildSlot::ProcessA,
                        )
                    })
                    .map_err(|_| {
                        ProviderRunFailure::io(
                            I3ReadOnlyProviderLocalnetFailureStage::TerminalObservationCorrelation,
                        )
                    })?;
                    PrivateProviderTerminalCompletion::FixedTerminalObservationConformance(
                        I3ReadOnlyProviderTerminalObservationConformanceRun { _private: () },
                    )
                }
                PrivateProviderTerminalExpectation::FixedProviderNetworkConformance => {
                    provider_network_conformance_completed(
                        executor_terminal,
                        I3LocalnetChildSlot::ProcessB,
                    )
                    .and_then(|_| {
                        provider_network_conformance_completed(
                            requester_terminal,
                            I3LocalnetChildSlot::ProcessA,
                        )
                    })
                    .map_err(|_| {
                        ProviderRunFailure::io(
                            I3ReadOnlyProviderLocalnetFailureStage::TerminalObservationCorrelation,
                        )
                    })?;
                    PrivateProviderTerminalCompletion::FixedProviderNetworkConformance(
                        I3ReadOnlyProviderNetworkConformanceRun { _private: () },
                    )
                }
            };
            // `wait_for_natural_exits` is also the cleanup primitive, where a
            // reaped nonzero child is a successful reaping result. A provider
            // positive additionally requires the existing captured proof that
            // both children exited zero naturally inside this same deadline.
            if !supervisor.wait_for_natural_exits()
                || !supervisor.zero_exit_reap_observed_within_deadline()
            {
                return Err(ProviderRunFailure::io(
                    I3ReadOnlyProviderLocalnetFailureStage::NaturalReap,
                ));
            }
            if !verify_actual_ready_endpoint_rebind(&endpoint) {
                return Err(ProviderRunFailure::io(
                    I3ReadOnlyProviderLocalnetFailureStage::EndpointRebind,
                ));
            }
            Ok(terminal_completion)
        })();
    let cleanup_succeeded = supervisor.cleanup_after_failure();
    let failure = match provider_execution {
        Ok(completion) if cleanup_succeeded => return Ok(completion),
        Ok(_) => {
            return Err(I3ReadOnlyProviderLocalnetRunError::at(
                I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
                I3ReadOnlyProviderLocalnetFailureStage::NaturalReap,
            ));
        }
        Err(failure) => failure,
    };
    let kind = match failure.provider_launch_error_kind {
        Some(Sys5I3ProviderLaunchErrorKind::ProviderRuntimeActivationPending) => {
            I3ReadOnlyProviderLocalnetRunErrorKind::ProviderRuntimeActivationPending
        }
        _ => I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
    };
    Err(I3ReadOnlyProviderLocalnetRunError::bootstrap_rejection(
        kind,
        failure.stage,
        failure.bootstrap_substage,
        failure.provider_launch_error_kind,
    ))
}

/// Builds/checks ordinary source once, creates one checked/admitted cohort,
/// then uses only the cohort's images and generated carrier bytes in actual
/// process children.  The coordinator never decodes a semantic message or
/// calculates an owner result.
#[doc(hidden)]
#[allow(clippy::result_large_err)] // private test seam returns its full observer-safe audit.
pub fn run_i3_process_localnet(
    request: I3ProcessLocalnetRequest,
) -> Result<I3ProcessLocalnetRun, I3LocalnetRunError> {
    let deadline = request.deadline;
    let late_ingress_profile = request.late_ingress_profile;
    let late_ingress_falsifier = request.late_ingress_falsifier;
    let adapter_delivery_profile = request.adapter_delivery_profile;
    let owner_admission_drive_profile = request.owner_admission_drive_profile;
    let owner_reply_replay_profile = request.owner_reply_replay_profile;
    let owner_reply_replay_falsifier = request.owner_reply_replay_falsifier;
    let requester_local_wait_profile = request.requester_local_wait_profile;
    let requester_local_wait_falsifier = request.requester_local_wait_falsifier;
    if deadline.is_zero() {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    // A force-reap observation reserve is part of the finite launcher input.
    // Reject it before source work or child spawn: this is malformed launch
    // configuration, not a deadline observed by a running child.
    if request.reaper_allowance < LIFECYCLE_REAP_RESERVE {
        return Err(LocalnetFailure::lifecycle_with_cause(
            I3LocalnetLifecycleRejectionCause::InvalidReaperAllowance,
        )
        .into_error(true));
    }
    let source = fs::read_to_string(&request.ordinary_source_path).map_err(|_| {
        LocalnetFailure {
            kind: I3LocalnetRunErrorKind::SourceBuildRejected,
            stage: I3LocalnetFailureStage::BeforeOwnerStart,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence::default(),
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
        .into_error(true)
    })?;
    let mut ordinary_source_build_count = 0;
    let project = build_project(Sys5SourceInput::inline(
        logical_source_path(&request.ordinary_source_path),
        source,
    ))
    .map_err(|_| {
        LocalnetFailure {
            kind: I3LocalnetRunErrorKind::SourceBuildRejected,
            stage: I3LocalnetFailureStage::BeforeOwnerStart,
            child_rejection: None,
            evidence: LocalnetRejectionEvidence::default(),
            lifecycle_rejection_cause: None,
            fault_profile: None,
            requester_local_wait_profile: None,
            retry_audit: None,
            owner_reply_replay_audit: None,
            declared_owner_deadline_expired_audit: None,
            late_ingress_audit: None,
            late_ingress_evidence_rejection: None,
        }
        .into_error(true)
    })?;
    ordinary_source_build_count += 1;
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(PROCESS_A_SLOT, "127.0.0.1:0", PROCESS_A_LOCI),
            Sys5I3DeploymentSlot::new(PROCESS_B_SLOT, "127.0.0.1:0", PROCESS_B_LOCI),
        ],
    )
    .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let summary = cohort.observer_safe_summary();
    if summary.full_admission_count() != 1 || summary.authority_generation_count() != 1 {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let request_contract = project_adapter_contract(&project, "owner-request")
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let reply_contract = project_adapter_contract(&project, "owner-reply-receipt")
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let credentials =
        generate_run_credentials().map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let run_ref = fresh_run_ref(
        summary.cohort_occurrence_ref(),
        &credentials.process_a.spki_ref,
        &credentials.process_b.spki_ref,
    );
    if late_ingress_profile
        == Some(
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
        )
    {
        // This consumes only parent-owned checked contract facts and must
        // complete before either image or expected start binding is taken.
        // The bootstrap negative retains this same genuine stage and changes
        // only B's later tainted wrapper.
        cohort
            .prestage_owner_capability_revocation(&run_ref, &request_contract)
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    }
    let (first_control, second_control) = if late_ingress_profile
        == Some(
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
        )
    {
        cohort
            .split_trusted_localnet_controls_with_prestaged_lifecycle(
                &codec,
                &run_ref,
                PROCESS_A_SLOT,
                &credentials.process_a.spki_ref,
                PROCESS_B_SLOT,
                &credentials.process_b.spki_ref,
            )
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?
    } else {
        let first_binding = cohort
            .parent_held_expected_start_binding(PROCESS_A_SLOT)
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
        let second_binding = cohort
            .parent_held_expected_start_binding(PROCESS_B_SLOT)
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
        codec
            .split_trusted_localnet_controls(
                &run_ref,
                first_binding,
                credentials.process_a.spki_ref.clone(),
                second_binding,
                credentials.process_b.spki_ref.clone(),
            )
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?
    };
    // This is candidate-shaped test input only. It is created from the
    // genuine parent-held prestage before either image is taken, then sent
    // only to A's bounded stdout event path. It never receives B's installed
    // receipt, registered descriptor, or publication authority.
    let requester_stdout_tainted_ack_candidate = if late_ingress_falsifier
        == Some(I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout)
    {
        let candidate = cohort
            .test_only_encode_prestaged_owner_lifecycle_ack_candidate(&codec)
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
        if candidate.len() > MAX_CHILD_EVENT_BYTES {
            return Err(LocalnetFailure::lifecycle().into_error(true));
        }
        Some(candidate)
    } else {
        None
    };
    let first_image = codec
        .encode_image(
            cohort
                .take_process_image(PROCESS_A_SLOT)
                .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?,
        )
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let second_process_image = cohort
        .take_process_image(PROCESS_B_SLOT)
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let second_process_image = if late_ingress_falsifier
        == Some(I3LocalnetLateIngressFalsifier::GenuineCandidateBindingTamper)
    {
        // The stage and trusted B bootstrap expectation above remain genuine.
        // This consumes the sole tainted B image and alters only its staged
        // wrapper target, so B rejects before Ready without a second image or
        // any source/A child activity.
        second_process_image.into_test_only_tamper(
            mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessImageTamper::mismatch_prestaged_owner_capability_lifecycle_target_slot(),
        )
    } else {
        second_process_image
    };
    let second_image = codec
        .encode_image(second_process_image)
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let lineage = SupervisorLineageEvidence {
        ordinary_source_build_count,
        admission_count: summary.full_admission_count(),
        m9_generation_count: summary.authority_generation_count(),
        request_contract,
        reply_contract,
    };

    // This finite I3-2 deadline begins only after synchronous source/build,
    // admission/cohort, and credential preflight.  It bounds actual child
    // process execution and reaping, not those supervisor-only preparations.
    let lifecycle_started = Instant::now();
    let main_deadline = lifecycle_started + deadline;
    let mut supervisor = LocalnetSupervisor {
        lifecycle_started,
        deadline: main_deadline,
        total_lifecycle_deadline: main_deadline + request.reaper_allowance,
        natural_reaper_exhausted: false,
        zero_exit_reap_observed_at: None,
        zero_exit_reap_observation_elapsed: None,
        zero_exit_reap_observed_within_deadline: false,
        children: Vec::new(),
        provider_launch: None,
    };
    if request.falsifier.is_some()
        && (request.fault_profile.is_some()
            || request.fault_audit_falsifier.is_some()
            || requester_local_wait_profile.is_some()
            || requester_local_wait_falsifier.is_some()
            || request.retry_profile.is_some()
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || owner_admission_drive_profile.is_some()
            || owner_reply_replay_profile.is_some()
            || owner_reply_replay_falsifier.is_some()
            || late_ingress_profile.is_some()
            || late_ingress_falsifier.is_some()
            || adapter_delivery_profile.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if request.fault_audit_falsifier.is_some()
        && request.fault_profile != Some(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission)
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if requester_local_wait_profile.is_some()
        && (request.fault_profile != Some(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission)
            || request.fault_audit_falsifier.is_some()
            || request.retry_profile.is_some()
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || owner_admission_drive_profile.is_some()
            || owner_reply_replay_profile.is_some()
            || owner_reply_replay_falsifier.is_some()
            || late_ingress_profile.is_some()
            || late_ingress_falsifier.is_some()
            || adapter_delivery_profile.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if requester_local_wait_falsifier.is_some() && requester_local_wait_profile.is_none() {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if request.retry_profile.is_some()
        && (request.fault_profile.is_some() || request.fault_audit_falsifier.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if request.retry_falsifier.is_some()
        && request.retry_profile != Some(I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite)
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if request.retry_audit_falsifier.is_some()
        && request.retry_profile
            != Some(I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply)
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if owner_admission_drive_profile.is_some()
        && (request.fault_profile.is_some()
            || request.fault_audit_falsifier.is_some()
            || request.retry_profile.is_some_and(|profile| {
                profile != I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply
            })
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || late_ingress_profile.is_some()
            || late_ingress_falsifier.is_some()
            || adapter_delivery_profile.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if late_ingress_profile.is_some()
        && (request.fault_profile.is_some()
            || request.fault_audit_falsifier.is_some()
            || request.retry_profile.is_some()
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || owner_reply_replay_profile.is_some()
            || owner_reply_replay_falsifier.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if adapter_delivery_profile.is_some()
        && (request.fault_profile.is_some()
            || request.fault_audit_falsifier.is_some()
            || request.retry_profile.is_some()
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || owner_reply_replay_profile.is_some()
            || owner_reply_replay_falsifier.is_some()
            || late_ingress_profile.is_some()
            || late_ingress_falsifier.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if let Some(falsifier) = late_ingress_falsifier {
        let required_profile = match falsifier {
            I3LocalnetLateIngressFalsifier::RepeatRetainedIngressAcquisition => {
                I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect
            }
            I3LocalnetLateIngressFalsifier::GenuineCandidateBindingTamper
            | I3LocalnetLateIngressFalsifier::ClearRetainedIngressCandidateCommitment
            | I3LocalnetLateIngressFalsifier::MutateRetainedIngressCandidateCommitment
            | I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout
            | I3LocalnetLateIngressFalsifier::DropBRegisteredAck
            | I3LocalnetLateIngressFalsifier::ReplayBRegisteredAck
            | I3LocalnetLateIngressFalsifier::SetOwnerLateIngressTerminalUnauthenticatedAdmissionAndClearTrustedControl
            | I3LocalnetLateIngressFalsifier::ClearRequesterLateIngressTerminalTrustedControl => {
                I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission
            }
        };
        if late_ingress_profile != Some(required_profile) {
            return Err(LocalnetFailure::lifecycle().into_error(true));
        }
    }
    if owner_reply_replay_falsifier.is_some() && owner_reply_replay_profile.is_none() {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    if owner_reply_replay_profile.is_some()
        && (request.fault_profile.is_some()
            || request.fault_audit_falsifier.is_some()
            || requester_local_wait_profile.is_some()
            || requester_local_wait_falsifier.is_some()
            || request.retry_profile.is_some()
            || request.retry_falsifier.is_some()
            || request.retry_audit_falsifier.is_some()
            || late_ingress_profile.is_some()
            || late_ingress_falsifier.is_some()
            || adapter_delivery_profile.is_some())
    {
        return Err(LocalnetFailure::lifecycle().into_error(true));
    }
    let fault_profile = request.fault_profile;
    let fault_audit_falsifier = request.fault_audit_falsifier;
    let retry_profile = request.retry_profile;
    let retry_falsifier = request.retry_falsifier;
    let retry_audit_falsifier = request.retry_audit_falsifier;
    let outcome = match request.falsifier {
        Some(I3LocalnetFalsifier::SwapImageAndBindingPairs) => run_swapped_pair_falsifier(
            &mut supervisor,
            &codec,
            first_image,
            second_image,
            first_control,
            second_control,
            credentials,
            deadline,
        ),
        falsifier => {
            let stall_bootstrap =
                falsifier == Some(I3LocalnetFalsifier::StallImageOrControlBootstrap);
            let stall_cleanup = falsifier == Some(I3LocalnetFalsifier::StallCleanup);
            let wrong_peer =
                falsifier == Some(I3LocalnetFalsifier::DeliverReplyFromCaSignedWrongSpkiPeer);
            let terminal_lifecycle_falsifier = match falsifier {
                Some(I3LocalnetFalsifier::CompletedThenNonzero) => {
                    PrivateChildTerminalLifecycleFalsifier::ExitNonzeroAfterCompleted
                }
                Some(I3LocalnetFalsifier::CompletedThenHang) => {
                    PrivateChildTerminalLifecycleFalsifier::HangAfterCompleted
                }
                Some(I3LocalnetFalsifier::AsymmetricCompletedAndRejected) => {
                    PrivateChildTerminalLifecycleFalsifier::RejectAfterCompleted
                }
                _ => PrivateChildTerminalLifecycleFalsifier::None,
            };
            let setup_failure_during_stall =
                falsifier == Some(I3LocalnetFalsifier::SetupFailureDuringStallMode);
            let delay_supervisor_exit_observation =
                falsifier == Some(I3LocalnetFalsifier::DelaySupervisorExitObservationPastDeadline);
            run_positive_or_peer_falsifier(
                &mut supervisor,
                &codec,
                &mut cohort,
                first_image,
                second_image,
                first_control,
                second_control,
                credentials,
                deadline,
                falsifier == Some(I3LocalnetFalsifier::InjectUnauthenticatedReply),
                stall_bootstrap,
                stall_cleanup,
                wrong_peer,
                terminal_lifecycle_falsifier,
                setup_failure_during_stall,
                delay_supervisor_exit_observation,
                fault_profile,
                fault_audit_falsifier,
                requester_local_wait_profile,
                requester_local_wait_falsifier,
                retry_profile,
                retry_falsifier,
                retry_audit_falsifier,
                owner_admission_drive_profile,
                owner_reply_replay_profile,
                owner_reply_replay_falsifier,
                late_ingress_profile,
                late_ingress_falsifier,
                adapter_delivery_profile,
                requester_stdout_tainted_ack_candidate,
                &lineage,
            )
        }
    };
    match outcome {
        Ok(run) => Ok(run),
        Err(failure) => {
            let cleanup = supervisor.cleanup_after_failure();
            Err(failure.into_supervisor_error(&supervisor, cleanup, &lineage))
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::result_large_err)] // preserves the private audit error across setup helpers.
fn run_swapped_pair_falsifier(
    supervisor: &mut LocalnetSupervisor,
    codec: &Sys5I3PrivateProcessCodec,
    first_image: Vec<u8>,
    second_image: Vec<u8>,
    first_control: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    second_control: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    credentials: RunCredentials,
    deadline: Duration,
) -> Result<I3ProcessLocalnetRun, LocalnetFailure> {
    let RunCredentials {
        ca_der,
        process_a,
        process_b,
        wrong_peer: _,
    } = credentials;
    // Both fixed child descriptors and their private leaf-key assignments
    // stay in the original exec slots.  Only the complete image/control
    // payload pairs are crossed, so neither child may reach certificate,
    // endpoint, semantic-admission, or mutation work.
    let control_a = PrivateChildControl {
        slot: I3LocalnetChildSlot::ProcessB,
        endpoint: None,
        trusted_runtime_control: codec
            .encode_trusted_localnet_control(second_control)
            .map_err(|_| LocalnetFailure::lifecycle())?,
        ca_der: ca_der.clone(),
        leaf_cert_der: process_a.certificate_der,
        leaf_key_der: process_a.private_key_der,
        inject_bad_preface: false,
        stall_bootstrap: false,
        stall_cleanup: false,
        setup_failure_during_stall: false,
        terminal_lifecycle_falsifier: PrivateChildTerminalLifecycleFalsifier::None,
        emit_request_before_wrong_peer_rejection: false,
        fault_profile: None,
        fault_audit_falsifier: None,
        requester_local_wait_profile: None,
        requester_local_wait_falsifier: None,
        retry_profile: None,
        retry_falsifier: None,
        retry_audit_falsifier: None,
        owner_admission_drive_profile: None,
        owner_reply_replay_profile: None,
        owner_reply_replay_falsifier: None,
        late_ingress_profile: None,
        late_ingress_falsifier: None,
        adapter_delivery_profile: None,
        tainted_owner_lifecycle_ack_candidate: None,
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    let control_b = PrivateChildControl {
        slot: I3LocalnetChildSlot::ProcessA,
        endpoint: None,
        trusted_runtime_control: codec
            .encode_trusted_localnet_control(first_control)
            .map_err(|_| LocalnetFailure::lifecycle())?,
        ca_der,
        leaf_cert_der: process_b.certificate_der,
        leaf_key_der: process_b.private_key_der,
        inject_bad_preface: false,
        stall_bootstrap: false,
        stall_cleanup: false,
        setup_failure_during_stall: false,
        terminal_lifecycle_falsifier: PrivateChildTerminalLifecycleFalsifier::None,
        emit_request_before_wrong_peer_rejection: false,
        fault_profile: None,
        fault_audit_falsifier: None,
        requester_local_wait_profile: None,
        requester_local_wait_falsifier: None,
        retry_profile: None,
        retry_falsifier: None,
        retry_audit_falsifier: None,
        owner_admission_drive_profile: None,
        owner_reply_replay_profile: None,
        owner_reply_replay_falsifier: None,
        late_ingress_profile: None,
        late_ingress_falsifier: None,
        adapter_delivery_profile: None,
        tainted_owner_lifecycle_ack_candidate: None,
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    supervisor
        .spawn(I3LocalnetChildSlot::ProcessA, second_image, control_a)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    supervisor
        .spawn(I3LocalnetChildSlot::ProcessB, first_image, control_b)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let a_event = supervisor
        .next_event(I3LocalnetChildSlot::ProcessA)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let b_event = supervisor
        .next_event(I3LocalnetChildSlot::ProcessB)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    match (a_event, b_event) {
        (
            rejection @ PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::StartBinding,
                ..
            },
            PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::StartBinding,
                ..
            },
        ) => Err(LocalnetFailure::from_child(
            I3LocalnetRunErrorKind::StartBindingRejected,
            I3LocalnetFailureStage::BeforeOwnerStart,
            rejection,
        )),
        _ => Err(LocalnetFailure::lifecycle()),
    }
}

fn is_expected_suppressed_owner_fault_requester_event(event: &PrivateChildEvent) -> bool {
    matches!(
        event,
        PrivateChildEvent::HandledDeliveryFault {
            slot: I3LocalnetChildSlot::ProcessA,
            request_identity_ref: Some(request_identity_ref),
            requester_observation: Some(
                I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved,
            ),
            requester_pending_request_is_retained: Some(true),
            remote_admission: None,
            semantic_admission_count: 0,
            owner_mutation_count: 0,
            ..
        } if !request_identity_ref.is_empty()
    )
}

fn is_profile_shaped_normal_delivery_fault_pair(
    profile: I3LocalnetFaultProfile,
    requester: &PrivateChildEvent,
    owner: &PrivateChildEvent,
) -> bool {
    let PrivateChildEvent::HandledDeliveryFault {
        slot: I3LocalnetChildSlot::ProcessA,
        request_identity_ref: Some(request_identity_ref),
        requester_observation: Some(requester_observation),
        requester_pending_request_is_retained,
        remote_admission: None,
        ..
    } = requester
    else {
        return false;
    };
    if request_identity_ref.is_empty() {
        return false;
    }
    match profile {
        I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite => {
            *requester_observation
                == I3LocalnetRequesterFaultObservation::RequestCarrierWriteNotAttempted
                && matches!(
                    owner,
                    PrivateChildEvent::HandledDeliveryFault {
                        slot: I3LocalnetChildSlot::ProcessB,
                        request_identity_ref: None,
                        requester_observation: None,
                        remote_admission: Some(PrivateRemoteAdmissionEvidence::NoAdmission { .. }),
                        ..
                    }
                )
        }
        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission => {
            *requester_observation == I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved
                && *requester_pending_request_is_retained == Some(true)
                && matches!(
                    owner,
                    PrivateChildEvent::HandledDeliveryFault {
                        slot: I3LocalnetChildSlot::ProcessB,
                        request_identity_ref: Some(_),
                        requester_observation: None,
                        remote_admission: Some(PrivateRemoteAdmissionEvidence::Admitted { .. }),
                        ..
                    }
                )
        }
        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit => false,
    }
}

fn adapter_delivery_failure_from_actual_terminals(
    profile: I3LocalnetAdapterDeliveryProfile,
    requester: &PrivateChildEvent,
    owner: &PrivateChildEvent,
) -> Option<I3LocalnetAdapterDeliveryFailure> {
    match profile {
        I3LocalnetAdapterDeliveryProfile::EndpointClosedBeforeConnect
            if exact_adapter_delivery_terminal(
                requester,
                I3LocalnetChildSlot::ProcessA,
                false,
                0,
                true,
                Some(I3LocalnetAdapterDeliveryFailure::EndpointUnavailable),
                false,
            ) && exact_adapter_delivery_terminal(
                owner,
                I3LocalnetChildSlot::ProcessB,
                false,
                0,
                false,
                None,
                true,
            ) =>
        {
            Some(I3LocalnetAdapterDeliveryFailure::EndpointUnavailable)
        }
        I3LocalnetAdapterDeliveryProfile::TruncateGeneratedRequestAfterBodyPrefix
            if exact_adapter_delivery_terminal(
                requester,
                I3LocalnetChildSlot::ProcessA,
                true,
                1,
                true,
                Some(I3LocalnetAdapterDeliveryFailure::FrameRejected),
                false,
            ) && exact_adapter_delivery_terminal(
                owner,
                I3LocalnetChildSlot::ProcessB,
                true,
                1,
                false,
                Some(I3LocalnetAdapterDeliveryFailure::FrameRejected),
                true,
            ) =>
        {
            Some(I3LocalnetAdapterDeliveryFailure::FrameRejected)
        }
        I3LocalnetAdapterDeliveryProfile::CompleteGeneratedRequestInTwoWrites => None,
        _ => None,
    }
}

fn exact_adapter_delivery_terminal(
    event: &PrivateChildEvent,
    expected_slot: I3LocalnetChildSlot,
    expected_transport_verified: bool,
    expected_stream_count: usize,
    requires_requester_pending: bool,
    expected_failure: Option<I3LocalnetAdapterDeliveryFailure>,
    requires_owner_no_admission: bool,
) -> bool {
    let PrivateChildEvent::AdapterDeliveryFault {
        slot,
        exec_confirmed,
        assigned_loci,
        trusted_control_consumed,
        tainted_image_consumed,
        tls_peer_verified,
        reciprocal_preface_verified,
        reliable_bidi_stream_count,
        quic_datagrams_enabled,
        request_identity_ref,
        requester_pending_request_is_retained,
        adapter_delivery_failure,
        remote_admission,
        semantic_admission_count,
        unauthenticated_semantic_admission_count,
        owner_mutation_count,
    } = event
    else {
        return false;
    };
    let request_shape = if requires_requester_pending {
        request_identity_ref
            .as_deref()
            .is_some_and(|reference| !reference.is_empty())
            && *requester_pending_request_is_retained == Some(true)
            && remote_admission.is_none()
    } else {
        request_identity_ref.is_none() && requester_pending_request_is_retained.is_none()
    };
    let owner_no_admission = if requires_owner_no_admission {
        matches!(
            remote_admission,
            Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                owner_serve_count: 0,
                owner_mutation_count: 0,
            })
        )
    } else {
        remote_admission.is_none()
    };
    *slot == expected_slot
        && *exec_confirmed
        && *assigned_loci == expected_slot.assigned_loci().map(str::to_owned)
        && *trusted_control_consumed
        && *tainted_image_consumed
        && *tls_peer_verified == expected_transport_verified
        && *reciprocal_preface_verified == expected_transport_verified
        && *reliable_bidi_stream_count == expected_stream_count
        && !*quic_datagrams_enabled
        && request_shape
        && *adapter_delivery_failure == expected_failure
        && owner_no_admission
        && *semantic_admission_count == 0
        && *unauthenticated_semantic_admission_count == 0
        && *owner_mutation_count == 0
}

fn unexpected_owner_terminal_failure(owner_terminal: PrivateChildEvent) -> LocalnetFailure {
    // This phase is sourced only from the retained B terminal.  In
    // particular, a selected post-admission profile cannot establish remote
    // admission by itself, and a zero mutation count does not undo a reported
    // semantic admission.
    let stage = if owner_terminal
        .reported_semantic_admission_count()
        .is_some_and(|count| count > 0)
    {
        I3LocalnetFailureStage::AfterRemoteAdmission
    } else {
        I3LocalnetFailureStage::LifecycleEvidenceRejected
    };
    let failure = match owner_terminal {
        rejection @ PrivateChildEvent::Rejected { .. } => {
            LocalnetFailure::from_child(I3LocalnetRunErrorKind::LifecycleRejected, stage, rejection)
        }
        _ => {
            let mut failure = LocalnetFailure::lifecycle_evidence_rejected();
            failure.stage = stage;
            failure
        }
    };
    // The server Ready event was already observed before this later terminal
    // is requested, independent of the terminal payload's claimed outcome.
    failure.after_observed_owner_runtime_start()
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::result_large_err)] // preserves the private audit error across setup helpers.
fn run_positive_or_peer_falsifier(
    supervisor: &mut LocalnetSupervisor,
    codec: &Sys5I3PrivateProcessCodec,
    cohort: &mut Sys5I3ProcessCohort,
    first_image: Vec<u8>,
    second_image: Vec<u8>,
    first_control: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    second_control: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    credentials: RunCredentials,
    deadline: Duration,
    inject_bad_preface: bool,
    stall_bootstrap: bool,
    stall_cleanup: bool,
    wrong_peer: bool,
    terminal_lifecycle_falsifier: PrivateChildTerminalLifecycleFalsifier,
    setup_failure_during_stall: bool,
    delay_supervisor_exit_observation: bool,
    fault_profile: Option<I3LocalnetFaultProfile>,
    fault_audit_falsifier: Option<I3LocalnetFaultAuditFalsifier>,
    requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
    requester_local_wait_falsifier: Option<I3LocalnetRequesterLocalWaitFalsifier>,
    retry_profile: Option<I3LocalnetRetryProfile>,
    retry_falsifier: Option<I3LocalnetRetryFalsifier>,
    retry_audit_falsifier: Option<I3LocalnetRetryAuditFalsifier>,
    owner_admission_drive_profile: Option<I3LocalnetOwnerAdmissionDriveProfile>,
    owner_reply_replay_profile: Option<I3LocalnetOwnerReplyReplayProfile>,
    owner_reply_replay_falsifier: Option<I3LocalnetOwnerReplyReplayFalsifier>,
    late_ingress_profile: Option<I3LocalnetLateIngressProfile>,
    late_ingress_falsifier: Option<I3LocalnetLateIngressFalsifier>,
    adapter_delivery_profile: Option<I3LocalnetAdapterDeliveryProfile>,
    requester_stdout_tainted_ack_candidate: Option<Vec<u8>>,
    lineage: &SupervisorLineageEvidence,
) -> Result<I3ProcessLocalnetRun, LocalnetFailure> {
    let RunCredentials {
        ca_der,
        process_a,
        process_b,
        wrong_peer: third_peer,
    } = credentials;
    let (server_certificate_der, server_private_key_der) = if wrong_peer {
        (third_peer.certificate_der, third_peer.private_key_der)
    } else {
        (process_b.certificate_der, process_b.private_key_der)
    };
    let server_control = PrivateChildControl {
        slot: I3LocalnetChildSlot::ProcessB,
        endpoint: None,
        trusted_runtime_control: codec
            .encode_trusted_localnet_control(second_control)
            .map_err(|_| LocalnetFailure::lifecycle())?,
        ca_der: ca_der.clone(),
        leaf_cert_der: server_certificate_der,
        leaf_key_der: server_private_key_der,
        inject_bad_preface: false,
        stall_bootstrap: stall_bootstrap || setup_failure_during_stall,
        stall_cleanup,
        setup_failure_during_stall,
        // Lifecycle faults are injected after the requester has received the
        // actual reply. The remote owner must remain available long enough to
        // complete that real semantic round trip first.
        terminal_lifecycle_falsifier: PrivateChildTerminalLifecycleFalsifier::None,
        emit_request_before_wrong_peer_rejection: false,
        fault_profile,
        fault_audit_falsifier,
        requester_local_wait_profile: None,
        requester_local_wait_falsifier: None,
        retry_profile,
        retry_falsifier,
        retry_audit_falsifier,
        owner_admission_drive_profile,
        owner_reply_replay_profile,
        owner_reply_replay_falsifier,
        late_ingress_profile,
        late_ingress_falsifier,
        adapter_delivery_profile,
        tainted_owner_lifecycle_ack_candidate: None,
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    let process_b = supervisor
        .spawn(I3LocalnetChildSlot::ProcessB, second_image, server_control)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let mut registered_owner_lifecycle_ack_reader = if late_ingress_profile
        == Some(
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
        )
    {
        let remaining = supervisor
            .deadline
            .checked_duration_since(Instant::now())
            .filter(|duration| !duration.is_zero())
            .ok_or_else(LocalnetFailure::lifecycle)?;
        let stream = supervisor
            .take_registered_owner_lifecycle_ack_stream()
            .ok_or_else(LocalnetFailure::lifecycle)?;
        Some(
            cohort
                .take_registered_owner_lifecycle_ack_reader(PROCESS_B_SLOT, stream, remaining)
                .map_err(|_| LocalnetFailure::lifecycle())?,
        )
    } else {
        None
    };
    let server_initial_event = supervisor.next_event(process_b).map_err(|_| {
        if setup_failure_during_stall {
            LocalnetFailure::lifecycle_with_cause(
                I3LocalnetLifecycleRejectionCause::SetupOrControlFailure,
            )
        } else if stall_bootstrap {
            let mut failure = LocalnetFailure::lifecycle();
            failure.kind = I3LocalnetRunErrorKind::LifecycleDeadlineExceeded;
            failure.stage = I3LocalnetFailureStage::BootstrapDeadline;
            failure.evidence.deadline_enforced = true;
            failure.evidence.reaper_deadline_enforced = true;
            failure
        } else if stall_cleanup {
            let mut failure = LocalnetFailure::lifecycle();
            failure.kind = I3LocalnetRunErrorKind::LifecycleDeadlineExceeded;
            failure.stage = I3LocalnetFailureStage::CleanupDeadline;
            failure.evidence.deadline_enforced = true;
            failure.evidence.reaper_deadline_enforced = true;
            failure
        } else {
            LocalnetFailure::lifecycle()
        }
    })?;
    let endpoint = match server_initial_event {
        PrivateChildEvent::Ready { endpoint } if is_loopback_endpoint(&endpoint) => endpoint,
        rejection @ PrivateChildEvent::Rejected {
            rejection: PrivateChildRejection::StartBinding,
            ..
        } => {
            return Err(LocalnetFailure::from_child(
                I3LocalnetRunErrorKind::StartBindingRejected,
                I3LocalnetFailureStage::BeforeOwnerStart,
                rejection,
            ));
        }
        rejection @ PrivateChildEvent::Rejected {
            rejection: PrivateChildRejection::PeerBinding,
            ..
        } => {
            return Err(LocalnetFailure::from_child(
                I3LocalnetRunErrorKind::PeerBindingRejected,
                I3LocalnetFailureStage::BeforeSemanticAdmission,
                rejection,
            ));
        }
        // The generic child wrapper has no authenticated or typed lifecycle
        // observation to turn into an all-zero pre-start assertion.
        PrivateChildEvent::UnknownLifecycleFailure { .. } => {
            return Err(LocalnetFailure::lifecycle_evidence_rejected());
        }
        _ => return Err(LocalnetFailure::lifecycle()),
    };
    let client_control = PrivateChildControl {
        slot: I3LocalnetChildSlot::ProcessA,
        endpoint: Some(endpoint.clone()),
        trusted_runtime_control: codec
            .encode_trusted_localnet_control(first_control)
            .map_err(|_| LocalnetFailure::lifecycle())?,
        ca_der,
        leaf_cert_der: process_a.certificate_der,
        leaf_key_der: process_a.private_key_der,
        inject_bad_preface,
        stall_bootstrap: false,
        stall_cleanup: false,
        setup_failure_during_stall: false,
        terminal_lifecycle_falsifier,
        emit_request_before_wrong_peer_rejection: wrong_peer,
        fault_profile,
        fault_audit_falsifier: None,
        requester_local_wait_profile,
        requester_local_wait_falsifier,
        retry_profile,
        retry_falsifier,
        retry_audit_falsifier,
        owner_admission_drive_profile,
        owner_reply_replay_profile,
        owner_reply_replay_falsifier,
        late_ingress_profile,
        late_ingress_falsifier,
        adapter_delivery_profile,
        tainted_owner_lifecycle_ack_candidate: requester_stdout_tainted_ack_candidate,
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    let process_a = supervisor
        .spawn(I3LocalnetChildSlot::ProcessA, first_image, client_control)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let a_event = supervisor
        .next_event(process_a)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let b_event = match supervisor.next_event(process_b) {
        Ok(event) => Some(event),
        Err(())
            if fault_profile
                == Some(
                    I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit,
                )
                && is_expected_suppressed_owner_fault_requester_event(&a_event) =>
        {
            // Only this selected schedule permits an absent owner report: A
            // has already reported its real missing-reply observation and B
            // has physically closed after its real serve.  Bootstrap, peer,
            // and all other child failures still take the lifecycle path.
            None
        }
        Err(()) => {
            // B had already emitted Ready, so a missing later terminal is
            // evidence rejection after observed owner start, not a fabricated
            // pre-start outcome.  It establishes no remote admission.
            return Err(
                LocalnetFailure::lifecycle_evidence_rejected().after_observed_owner_runtime_start()
            );
        }
    };
    if let Some(
        profile @ (I3LocalnetAdapterDeliveryProfile::EndpointClosedBeforeConnect
        | I3LocalnetAdapterDeliveryProfile::TruncateGeneratedRequestAfterBodyPrefix),
    ) = adapter_delivery_profile
    {
        let owner_event = b_event.as_ref().ok_or_else(|| {
            LocalnetFailure::lifecycle_evidence_rejected().after_observed_owner_runtime_start()
        })?;
        let failure_kind =
            adapter_delivery_failure_from_actual_terminals(profile, &a_event, owner_event)
                .ok_or_else(|| {
                    LocalnetFailure::lifecycle_evidence_rejected()
                        .after_observed_owner_runtime_start()
                })?;
        let mut failure = LocalnetFailure::adapter_delivery_failure(failure_kind)
            .after_observed_owner_runtime_start();
        // The exact requester terminal checks the runtime's pending count;
        // retaining this fact here does not infer it from the selected profile.
        failure.evidence.requester_pending_request_is_retained = true;
        return Err(failure);
    }
    if let Some(profile) = owner_reply_replay_profile {
        let audit = supervisor
            .owner_reply_replay_audit(profile, lineage)
            .ok_or_else(|| {
                LocalnetFailure::lifecycle_evidence_rejected().after_observed_owner_runtime_start()
            })?;
        return Err(LocalnetFailure::owner_reply_replay_rejected(audit)
            .after_observed_owner_runtime_start());
    }
    if retry_falsifier == Some(I3LocalnetRetryFalsifier::RetryOnInitialVerifiedSession) {
        if let PrivateChildEvent::Rejected {
            rejection: PrivateChildRejection::Lifecycle,
            evidence,
            ..
        } = &a_event
        {
            let kind = match evidence.adapter_rejection_kind {
                Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected) => {
                    I3LocalnetRunErrorKind::LifecycleRejected
                }
                Some(I3LocalnetAdapterRejectionKind::PeerBindingRejected) => {
                    I3LocalnetRunErrorKind::PeerBindingRejected
                }
                None => return Err(LocalnetFailure::lifecycle_evidence_rejected()),
            };
            return Err(LocalnetFailure::from_child(
                kind,
                I3LocalnetFailureStage::BeforeSemanticAdmission,
                a_event,
            )
            .after_observed_owner_runtime_start());
        }
        return Err(
            LocalnetFailure::lifecycle_evidence_rejected().after_observed_owner_runtime_start()
        );
    }
    let retry_audit = if let Some(profile) = retry_profile {
        let audit = match supervisor.retry_audit(profile, lineage) {
            Some(audit) => audit,
            None => {
                let mut failure = LocalnetFailure::lifecycle_evidence_rejected()
                    .after_observed_owner_runtime_start();
                // This remains requester-local evidence: it comes only from
                // the A terminal's actual opaque pending outcome and its
                // source-contract-validated retained request lineage. It
                // does not accept the failed retry join or publish B's
                // duplicate conclusion.
                if supervisor
                    .validated_requester_retry_pending_observation(&lineage.request_contract)
                {
                    failure.evidence.requester_pending_request_is_retained = true;
                }
                return Err(failure);
            }
        };
        if profile == I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply {
            return Err(LocalnetFailure::retry_delivery(audit).after_observed_owner_runtime_start());
        }
        Some(audit)
    } else {
        None
    };
    let late_ingress_audit = if let Some(profile) = late_ingress_profile {
        // G1 has no registered lifecycle ACK route by construction. This is
        // an actual parent launch disposition, retained separately from B's
        // child event rather than inferred from delivery results.
        let parent_observation = match profile {
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
                PrivateLateIngressParentObservation {
                    ack_reader_outcome: I3LocalnetLateIngressAckReaderOutcome::NotSelected,
                    nonregistered_ack_input_disposition:
                        I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved,
                    nonregistered_ack_input_count: 0,
                    publication: I3LocalnetLateIngressParentPublication::NoPrestageSelected,
                    publication_commit_count: 0,
                }
            }
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
                let selected_requester_stdout_route = late_ingress_falsifier
                    == Some(
                        I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout,
                    );
                let (nonregistered_ack_input_disposition, nonregistered_ack_input_count) =
                    match (
                        selected_requester_stdout_route,
                        supervisor.requester_stdout_tainted_owner_lifecycle_ack_candidate(),
                    ) {
                        (true, Some(candidate)) => {
                            // This is an actual bounded A stdout input. The
                            // codec can establish only a tainted shape; the
                            // result is deliberately dropped and never
                            // offered to B's registered reader or cohort.
                            let _tainted = codec.decode_owner_lifecycle_ack(candidate).map_err(|_| {
                                LocalnetFailure::lifecycle_evidence_rejected()
                                    .after_observed_owner_runtime_start()
                            })?;
                            (
                                I3LocalnetLateIngressNonregisteredAckInputDisposition::RequesterStdoutTaintedCandidateIgnored,
                                1,
                            )
                        }
                        (true, None) | (false, Some(_)) => {
                            return Err(
                                LocalnetFailure::lifecycle_evidence_rejected()
                                    .after_observed_owner_runtime_start(),
                            );
                        }
                        (false, None) => (
                            I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved,
                            0,
                        ),
                    };
                // The sole accepting route owns the actual parent endpoint
                // of B's dedicated inherited FD.  It reads and decodes the
                // frame internally, then yields an opaque completion for the
                // cohort.  No probe caller can substitute bytes, a decoded
                // DTO, an FD label, or an observer record here.
                let reader = registered_owner_lifecycle_ack_reader
                    .as_mut()
                    .ok_or_else(LocalnetFailure::lifecycle)?;
                match reader.read_next_completion(codec) {
                    Ok(completion) => {
                        if selected_requester_stdout_route {
                            // The selected A route must not be masked by a
                            // registered-B completion. Do not publish before
                            // rejecting this contradictory actual evidence.
                            return Err(
                                LocalnetFailure::lifecycle_evidence_rejected()
                                    .after_observed_owner_runtime_start(),
                            );
                        }
                        cohort
                            .publish_registered_owner_lifecycle_completion(completion)
                            .map_err(|_| LocalnetFailure::lifecycle())?;
                        let ack_reader_outcome = if late_ingress_falsifier
                            == Some(I3LocalnetLateIngressFalsifier::ReplayBRegisteredAck)
                        {
                            // The reader consumes its internal completion
                            // state before returning the first completion. A
                            // second call therefore exercises the registered
                            // reader's replay rejection without accepting a
                            // second publication.
                            if reader.read_next_completion(codec).is_ok() {
                                return Err(
                                    LocalnetFailure::lifecycle_evidence_rejected()
                                        .after_observed_owner_runtime_start(),
                                );
                            }
                            I3LocalnetLateIngressAckReaderOutcome::ReplayRejected
                        } else {
                            I3LocalnetLateIngressAckReaderOutcome::AcceptedRegisteredOwnerChildFd
                        };
                        let publication = cohort.observer_safe_lifecycle_publication_summary();
                        if publication.publication_outcome()
                            != Some(
                                mir_runtime::sys5_i3_process_runtime::Sys5I3LifecyclePublicationOutcome::G2Published,
                            )
                            || publication.published_authority_generation_ref().is_empty()
                        {
                            return Err(
                                LocalnetFailure::lifecycle_evidence_rejected()
                                    .after_observed_owner_runtime_start(),
                            );
                        }
                        PrivateLateIngressParentObservation {
                            ack_reader_outcome,
                            nonregistered_ack_input_disposition,
                            nonregistered_ack_input_count,
                            publication: I3LocalnetLateIngressParentPublication::G2Published,
                            publication_commit_count: 1,
                        }
                    }
                    Err(_) => {
                        // An EOF, timeout, malformed registered-B frame, or
                        // a deliberately withheld route cannot publish G2.
                        // The cohort retains that terminal observation as
                        // incomplete without treating any observer/A route
                        // as a completion path.
                        cohort
                            .mark_registered_owner_lifecycle_publication_incomplete()
                            .map_err(|_| LocalnetFailure::lifecycle())?;
                        let publication = cohort.observer_safe_lifecycle_publication_summary();
                        if publication.publication_outcome()
                            != Some(
                                mir_runtime::sys5_i3_process_runtime::Sys5I3LifecyclePublicationOutcome::PublicationIncomplete,
                            )
                        {
                            return Err(
                                LocalnetFailure::lifecycle_evidence_rejected()
                                    .after_observed_owner_runtime_start(),
                            );
                        }
                        let ack_reader_outcome = match late_ingress_falsifier {
                            Some(
                                I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout,
                            ) => I3LocalnetLateIngressAckReaderOutcome::UntrustedRouteIgnored,
                            Some(I3LocalnetLateIngressFalsifier::DropBRegisteredAck) => {
                                I3LocalnetLateIngressAckReaderOutcome::LostBeforeParentAcceptance
                            }
                            _ => {
                                return Err(
                                    LocalnetFailure::lifecycle_evidence_rejected()
                                        .after_observed_owner_runtime_start(),
                                );
                            }
                        };
                        PrivateLateIngressParentObservation {
                            ack_reader_outcome,
                            nonregistered_ack_input_disposition,
                            nonregistered_ack_input_count,
                            publication: I3LocalnetLateIngressParentPublication::PublicationIncomplete,
                            publication_commit_count: 0,
                        }
                    }
                }
            }
        };
        match supervisor.late_ingress_join(profile, lineage, parent_observation) {
            Some(PrivateLateIngressJoin::Accepted(audit)) => {
                let audit = *audit;
                if profile
                    == I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission
                {
                    return Err(
                        LocalnetFailure::late_ingress_delivery(audit)
                            .after_observed_owner_runtime_start(),
                    );
                }
                Some(audit)
            }
            Some(PrivateLateIngressJoin::EvidenceRejected(rejection)) => {
                let mut failure = LocalnetFailure::late_ingress_evidence_rejected(rejection)
                    .after_observed_owner_runtime_start();
                if supervisor
                    .validated_requester_late_ingress_pending_observation(&lineage.request_contract)
                {
                    failure.evidence.requester_pending_request_is_retained = true;
                }
                return Err(failure);
            }
            None => {
                let mut failure = LocalnetFailure::lifecycle_evidence_rejected()
                    .after_observed_owner_runtime_start();
                if supervisor
                    .validated_requester_late_ingress_pending_observation(&lineage.request_contract)
                {
                    failure.evidence.requester_pending_request_is_retained = true;
                }
                return Err(failure);
            }
        }
    } else {
        None
    };
    let (a, b) = match (a_event, b_event) {
        (
            rejection @ PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::PeerBinding,
                ..
            },
            _,
        )
        | (
            _,
            Some(
                rejection @ PrivateChildEvent::Rejected {
                    rejection: PrivateChildRejection::PeerBinding,
                    ..
                },
            ),
        ) => {
            return Err(LocalnetFailure::from_child(
                I3LocalnetRunErrorKind::PeerBindingRejected,
                I3LocalnetFailureStage::BeforeSemanticAdmission,
                rejection,
            )
            .after_observed_owner_runtime_start());
        }
        (
            rejection @ PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::StartBinding,
                ..
            },
            _,
        )
        | (
            _,
            Some(
                rejection @ PrivateChildEvent::Rejected {
                    rejection: PrivateChildRejection::StartBinding,
                    ..
                },
            ),
        ) => {
            return Err(LocalnetFailure::from_child(
                I3LocalnetRunErrorKind::StartBindingRejected,
                I3LocalnetFailureStage::BeforeOwnerStart,
                rejection,
            ));
        }
        (
            PrivateChildEvent::UnknownLifecycleFailure {
                slot: I3LocalnetChildSlot::ProcessA,
            },
            Some(
                owner @ PrivateChildEvent::OwnerAdmissionAwaiting {
                    slot: I3LocalnetChildSlot::ProcessB,
                    ..
                },
            ),
        ) => {
            // B emitted this exact record only after the verified session
            // admitted the source-budgeted request and retained it Awaiting.
            // A's later generic EOF/error cannot erase that owner observation
            // or relabel it as a pre-start failure.
            return Err(LocalnetFailure::owner_admission_drive_required(owner)
                .after_observed_owner_runtime_start());
        }
        (
            requester @ PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                ..
            },
            Some(
                owner @ PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessB,
                    ..
                },
            ),
        ) if fault_profile.is_some_and(|profile| {
            is_profile_shaped_normal_delivery_fault_pair(profile, &requester, &owner)
        }) =>
        {
            return Err(LocalnetFailure::delivery_fault(
                fault_profile.expect("guarded delivery-fault profile"),
                requester_local_wait_profile,
            ));
        }
        (
            PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                ..
            },
            Some(owner_terminal),
        ) if fault_profile.is_some() => {
            return Err(unexpected_owner_terminal_failure(owner_terminal));
        }
        (
            PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                ..
            },
            None,
        ) if fault_profile
            == Some(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit) =>
        {
            return Err(LocalnetFailure::delivery_fault(
                I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit,
                None,
            ));
        }
        (
            PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                ..
            },
            _,
        ) if fault_profile.is_some() => {
            // A normal profile accepts only its paired HandledDeliveryFault
            // event shape. A retained unexpected owner terminal is classified
            // above from that terminal's own counters; a missing terminal is
            // lifecycle-evidence rejection, not the requester's ambiguity
            // conclusion. The explicit suppress-audit profile above is the
            // sole remote-unknown exception.
            return Err(
                LocalnetFailure::lifecycle_evidence_rejected().after_observed_owner_runtime_start()
            );
        }
        (a, b) => match (
            a.into_completed(),
            b.and_then(PrivateChildEvent::into_completed),
        ) {
            (Some(a), Some(b))
                if a.slot == I3LocalnetChildSlot::ProcessA
                    && b.slot == I3LocalnetChildSlot::ProcessB =>
            {
                (a, b)
            }
            // B Ready was observed before A launched. A later unknown child
            // failure therefore cannot truthfully become `BeforeOwnerStart`.
            _ => {
                return Err(LocalnetFailure::lifecycle_evidence_rejected()
                    .after_observed_owner_runtime_start());
            }
        },
    };
    let request_write_observation = a
        .observer_evidence
        .request_sent
        .as_ref()
        .and_then(|delivery| delivery.generated_frame_write_observation.as_ref());
    match adapter_delivery_profile {
        Some(I3LocalnetAdapterDeliveryProfile::CompleteGeneratedRequestInTwoWrites)
            if request_write_observation.is_some_and(|observation| {
                observation.application_write_count == 2
                    && observation.frame_prefix_split_across_writes
                    && observation.complete_frame_written
            }) => {}
        None if request_write_observation.is_none() => {}
        _ => return Err(LocalnetFailure::lifecycle()),
    }
    if let Some(expiry_audit) = joined_declared_owner_deadline_expired_evidence(lineage, &a, &b) {
        // A semantic terminal failure uses the same verified QUIC route as a
        // receipt, but it has no owner serve/write and no requester receipt.
        // Validate that exact completed shape before retaining the natural
        // child exits in the error audit.
        if a.generated_request_count != 1
            || a.receipt_count != 0
            || a.served_count != 0
            || a.write_count != 0
            || a.reply_count != 0
            || a.runtime_occurrence_count != 1
            || b.generated_request_count != 0
            || b.served_count != 0
            || b.write_count != 0
            || b.reply_count != 1
            || b.receipt_count != 0
            || b.runtime_occurrence_count != 0
            || !a.tls_peer_verified
            || !b.tls_peer_verified
            || !a.reciprocal_preface_verified
            || !b.reciprocal_preface_verified
            || a.reliable_bidi_stream_count != 1
            || b.reliable_bidi_stream_count != 1
            || a.quic_datagrams_enabled
            || b.quic_datagrams_enabled
            || a.semantic_admission_count != 1
            || b.semantic_admission_count != 1
            || a.unauthenticated_semantic_admission_count != 0
            || b.unauthenticated_semantic_admission_count != 0
            || a.assigned_loci
                != I3LocalnetChildSlot::ProcessA
                    .assigned_loci()
                    .map(str::to_owned)
            || b.assigned_loci
                != I3LocalnetChildSlot::ProcessB
                    .assigned_loci()
                    .map(str::to_owned)
            || !a.exec_confirmed
            || !b.exec_confirmed
            || !a.trusted_control_consumed
            || !b.trusted_control_consumed
            || !a.tainted_image_consumed
            || !b.tainted_image_consumed
        {
            return Err(LocalnetFailure::lifecycle());
        }
        let mut terminal_children = BTreeMap::new();
        for child in &supervisor.children {
            let report = match child.slot {
                I3LocalnetChildSlot::ProcessA => &a,
                I3LocalnetChildSlot::ProcessB => &b,
            };
            terminal_children.insert(
                child.slot,
                I3LocalnetChildAudit {
                    slot: child.slot,
                    pid: child.child.id(),
                    reaped: false,
                    exec_confirmed: report.exec_confirmed
                        && report.slot == child.slot
                        && child.child.id() != std::process::id(),
                    assigned_loci: report.assigned_loci.clone(),
                    trusted_control_consumed: report.trusted_control_consumed,
                    tainted_image_consumed: report.tainted_image_consumed,
                    observed_exit_status: None,
                    was_force_killed: false,
                },
            );
        }
        if !terminal_children.values().all(|child| child.exec_confirmed)
            || !supervisor.mark_reaped_in(&mut terminal_children)
        {
            return Err(LocalnetFailure::lifecycle());
        }
        return Err(
            LocalnetFailure::declared_owner_deadline_expired_consumed(expiry_audit)
                .after_observed_owner_runtime_start(),
        );
    }
    if a.generated_request_count != 1
        || a.receipt_count != 1
        || b.served_count != 1
        || b.write_count != 1
        || b.reply_count != 1
        || !a.tls_peer_verified
        || !b.tls_peer_verified
        || !a.reciprocal_preface_verified
        || !b.reciprocal_preface_verified
        || a.reliable_bidi_stream_count != 1
        || b.reliable_bidi_stream_count != 1
        || a.quic_datagrams_enabled
        || b.quic_datagrams_enabled
        || a.semantic_admission_count != 1
        || b.semantic_admission_count != 1
    {
        return Err(LocalnetFailure::lifecycle());
    }
    if a.assigned_loci
        != I3LocalnetChildSlot::ProcessA
            .assigned_loci()
            .map(str::to_owned)
        || b.assigned_loci
            != I3LocalnetChildSlot::ProcessB
                .assigned_loci()
                .map(str::to_owned)
        || !a.exec_confirmed
        || !b.exec_confirmed
        || !a.trusted_control_consumed
        || !b.trusted_control_consumed
        || !a.tainted_image_consumed
        || !b.tainted_image_consumed
    {
        return Err(LocalnetFailure::lifecycle());
    }
    let mut children = BTreeMap::new();
    for child in &supervisor.children {
        let report = match child.slot {
            I3LocalnetChildSlot::ProcessA => &a,
            I3LocalnetChildSlot::ProcessB => &b,
        };
        let exec_confirmed = report.exec_confirmed
            && report.slot == child.slot
            && child.child.id() != std::process::id();
        children.insert(
            child.slot,
            I3LocalnetChildAudit {
                slot: child.slot,
                pid: child.child.id(),
                reaped: false,
                exec_confirmed,
                assigned_loci: report.assigned_loci.clone(),
                trusted_control_consumed: report.trusted_control_consumed,
                tainted_image_consumed: report.tainted_image_consumed,
                observed_exit_status: None,
                was_force_killed: false,
            },
        );
    }
    if !children.values().all(|child| child.exec_confirmed) {
        return Err(LocalnetFailure::lifecycle());
    }
    // This test-only hook consumes the already-selected lifecycle interval
    // after both children have reported Completed and naturally returned, but
    // before the supervisor records their exit statuses.  It introduces no
    // extra reaper allowance or later cleanup deadline.
    if delay_supervisor_exit_observation {
        supervisor.delay_terminal_exit_observation_past_deadline();
    }
    // The actual reaper runs on return; the result deliberately contains only
    // its observer-safe final state, set below by `mark_reaped`.
    let all_children_reaped = supervisor.mark_reaped_in(&mut children);
    if !all_children_reaped {
        return Err(LocalnetFailure::lifecycle());
    }
    let exact_one_shot_bindings_consumed = children
        .values()
        .all(|child| child.trusted_control_consumed && child.tainted_image_consumed);
    let joined =
        joined_observer_evidence(lineage, &a, &b).ok_or_else(LocalnetFailure::lifecycle)?;
    let source_ref_count = joined.source_ref_inventory.len();
    let core_ref_count = joined.core_ref_inventory.len();
    let artifact_ref_count = joined.artifact_ref_inventory.len();
    let edge_ref_count = joined.edge_ref_inventory.len();
    let semantic_request_identity_count = joined
        .delivery_records
        .iter()
        .map(I3LocalnetObserverSafeDeliveryRecord::semantic_request_identity_ref)
        .collect::<BTreeSet<_>>()
        .len();
    let network_occurrence_count = joined
        .delivery_records
        .iter()
        .map(I3LocalnetObserverSafeDeliveryRecord::network_occurrence_ref)
        .collect::<BTreeSet<_>>()
        .len();
    let references = joined.references;
    let source_derived_only = lineage.ordinary_source_build_count == 1
        && lineage.admission_count == 1
        && lineage.m9_generation_count == 1
        && a.semantic_admission_count == 1
        && b.semantic_admission_count == 1
        && joined.delivery_records.len() == 4
        && source_ref_count > 0
        && core_ref_count > 0
        && artifact_ref_count >= 2
        && edge_ref_count > 0;
    // Runtime summaries count local semantic transitions; generated request
    // and reply occurrences are retained by the two child reports as distinct
    // generated-carrier transitions.
    let runtime_occurrence_count = a
        .runtime_occurrence_count
        .saturating_add(b.runtime_occurrence_count)
        .saturating_add(a.generated_request_count)
        .saturating_add(b.reply_count);
    let observer_safe = source_derived_only
        && exact_one_shot_bindings_consumed
        && children.values().all(|child| child.reaped)
        && runtime_occurrence_count >= 5;
    let clean_shutdown_backed_by_zero_exit_reaps_without_force_kill = all_children_reaped
        && children.values().all(|child| {
            child
                .observed_exit_status
                .as_ref()
                .is_some_and(ExitStatus::success)
                && !child.was_force_killed
        });
    Ok(I3ProcessLocalnetRun {
        children,
        execution: I3LocalnetExecutionAudit {
            requester_child: I3LocalnetChildSlot::ProcessA,
            owner_child: I3LocalnetChildSlot::ProcessB,
            generated_request_count: a.generated_request_count,
            remote_owner_serve_count: b.served_count,
            remote_owner_write_count: b.write_count,
            generated_reply_count: b.reply_count,
            requester_local_receipt_count: a.receipt_count,
            network_receipt_frame_count: a
                .network_receipt_frame_count
                .saturating_add(b.network_receipt_frame_count),
            source_derived_only,
        },
        startup: I3LocalnetStartupAudit {
            supervisor_ordinary_source_build_count: lineage.ordinary_source_build_count,
            supervisor_admission_count: lineage.admission_count,
            supervisor_m9_generation_count: lineage.m9_generation_count,
            child_bootstrap_is_image_only_no_source_or_global_authority:
                exact_one_shot_bindings_consumed
                    && a.tainted_image_consumed
                    && b.tainted_image_consumed
                    && a.trusted_control_consumed
                    && b.trusted_control_consumed,
            stores_are_process_local_and_distinct: references.requester_local_store_ref()
                != references.owner_local_store_ref(),
            exact_one_shot_bindings_consumed,
        },
        transport: I3LocalnetTransportAudit {
            mutually_authenticated_quic_peer_binding: a.tls_peer_verified
                && b.tls_peer_verified
                && a.reciprocal_preface_verified
                && b.reciprocal_preface_verified,
            reliable_bidirectional_streams_only: a.reliable_bidi_stream_count == 1
                && b.reliable_bidi_stream_count == 1,
            quic_datagrams_enabled: a.quic_datagrams_enabled || b.quic_datagrams_enabled,
            unauthenticated_semantic_admission_count: a
                .unauthenticated_semantic_admission_count
                .saturating_add(b.unauthenticated_semantic_admission_count),
            ephemeral_endpoint_reuse_verified: all_children_reaped
                && verify_actual_ready_endpoint_rebind(&endpoint),
        },
        trace: I3LocalnetObserverSafeTrace {
            observer_safe,
            exact_chain: source_derived_only
                && a.tls_peer_verified
                && b.tls_peer_verified
                && a.reciprocal_preface_verified
                && b.reciprocal_preface_verified
                && exact_observer_chain(&references),
            source_ref_count,
            core_ref_count,
            artifact_ref_count,
            semantic_request_identity_count,
            network_occurrence_count,
            runtime_occurrence_count,
            actual_delivery_records: joined.delivery_records,
            actual_source_ref_inventory: joined.source_ref_inventory,
            actual_core_ref_inventory: joined.core_ref_inventory,
            actual_artifact_ref_inventory: joined.artifact_ref_inventory,
            actual_edge_ref_inventory: joined.edge_ref_inventory,
            references,
        },
        lifecycle: I3LocalnetLifecycleAudit {
            all_children_reaped,
            clean_shutdown_backed_by_zero_exit_reaps_without_force_kill,
            observed_supervised_process_lifecycle_elapsed: supervisor.observed_lifecycle_elapsed(),
            observed_supervised_process_lifecycle_bound: supervisor.lifecycle_bound(),
            zero_exit_reap_observed_within_deadline: supervisor
                .zero_exit_reap_observed_within_deadline(),
            captured_zero_exit_reap_observation_elapsed: supervisor
                .captured_zero_exit_reap_observation_elapsed(),
        },
        retry_audit,
        late_ingress_audit,
    })
}

impl LocalnetSupervisor {
    fn lifecycle_bound(&self) -> Duration {
        self.total_lifecycle_deadline
            .saturating_duration_since(self.lifecycle_started)
    }

    fn observed_lifecycle_elapsed(&self) -> Duration {
        self.zero_exit_reap_observation_elapsed
            .unwrap_or_else(|| self.lifecycle_started.elapsed())
    }

    fn captured_zero_exit_reap_observation_elapsed(&self) -> Duration {
        self.zero_exit_reap_observation_elapsed
            .unwrap_or(Duration::ZERO)
    }

    fn zero_exit_reap_observed_within_deadline(&self) -> bool {
        self.zero_exit_reap_observed_within_deadline
    }

    fn zero_exit_reap_observed_late(&self) -> bool {
        self.zero_exit_reap_observed_at.is_some() && !self.zero_exit_reap_observed_within_deadline
    }

    fn natural_reap_deadline(&self) -> Instant {
        // Validation retains at least this reserve, so the subtraction cannot
        // underflow.  Keep the fallback conservative for defensive callers.
        self.total_lifecycle_deadline
            .checked_sub(LIFECYCLE_REAP_RESERVE)
            .unwrap_or(self.lifecycle_started)
    }

    fn sleep_until(deadline: Instant) {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if !remaining.is_zero() {
            thread::sleep(remaining.min(Duration::from_millis(2)));
        }
    }

    fn delay_terminal_exit_observation_past_deadline(&self) {
        debug_assert!(self.children.iter().all(|child| {
            child
                .terminal_event
                .as_ref()
                .is_some_and(PrivateChildEvent::is_completed)
        }));
        // Consume only the existing lifecycle interval.  A spin/yield after
        // reaching its absolute end makes this falsifier deterministic without
        // adding a second wait or extending the lifecycle budget.
        Self::sleep_until(self.total_lifecycle_deadline);
        while Instant::now() <= self.total_lifecycle_deadline {
            std::hint::spin_loop();
        }
    }

    fn capture_zero_exit_reap_observation(&mut self) -> bool {
        if self.zero_exit_reap_observed_at.is_none() {
            // This timestamp is the supervisor's observation of reaped PIDs,
            // not an assertion about OS scheduling or the instant a child
            // actually terminated.
            let observed_at = Instant::now();
            self.zero_exit_reap_observation_elapsed =
                Some(observed_at.saturating_duration_since(self.lifecycle_started));
            self.zero_exit_reap_observed_within_deadline =
                observed_at <= self.total_lifecycle_deadline;
            self.zero_exit_reap_observed_at = Some(observed_at);
        }
        self.zero_exit_reap_observed_within_deadline
    }

    fn spawn(
        &mut self,
        slot: I3LocalnetChildSlot,
        image: Vec<u8>,
        control: PrivateChildControl,
    ) -> io::Result<I3LocalnetChildSlot> {
        if Instant::now() >= self.deadline {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "deadline"));
        }
        let executable = probe_binary_path()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "probe binary"))?;
        let (mut parent_control, child_control) = UnixStream::pair()?;
        set_close_on_exec(parent_control.as_raw_fd())?;
        set_close_on_exec(child_control.as_raw_fd())?;
        let (parent_owner_lifecycle_ack, child_owner_lifecycle_ack) = if slot
            == I3LocalnetChildSlot::ProcessB
            && control.late_ingress_profile
                == Some(
                    I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
                )
        {
            let (parent, child) = UnixStream::pair()?;
            // The A child is spawned after B. Mark the parent end close-on-
            // exec before either spawn so no later child inherits a usable
            // B-completion route.
            set_close_on_exec(parent.as_raw_fd())?;
            set_close_on_exec(child.as_raw_fd())?;
            (Some(parent), Some(child))
        } else {
            (None, None)
        };
        let child_fd = child_control.as_raw_fd();
        let child_owner_lifecycle_ack_fd =
            child_owner_lifecycle_ack.as_ref().map(AsRawFd::as_raw_fd);
        let mut command = Command::new(executable);
        command
            .env_clear()
            .arg(slot.child_arg())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        // SAFETY: the pre-exec closure only duplicates the already-open Unix
        // control descriptor to a fixed, documented child FD and checks every
        // syscall. The closure does not allocate, lock, or inspect Rust state.
        unsafe {
            command.pre_exec(move || {
                // Duplicate both sources above the reserved targets before
                // either `dup2`: a UnixStream pair can otherwise allocate a
                // source descriptor equal to the other target (3/4), making
                // the second replacement silently lose an ACK endpoint.
                let inherited_control = duplicate_child_fd(child_fd)?;
                let inherited_ack = match child_owner_lifecycle_ack_fd {
                    Some(fd) => Some(duplicate_child_fd(fd)?),
                    None => None,
                };
                let install = (|| {
                    install_child_fd(inherited_control, LOCALNET_CONTROL_FD)?;
                    if let Some(inherited_ack) = inherited_ack {
                        install_child_fd(inherited_ack, LOCALNET_OWNER_LIFECYCLE_ACK_FD)?;
                    }
                    Ok(())
                })();
                let _ = libc::close(inherited_control);
                if let Some(inherited_ack) = inherited_ack {
                    let _ = libc::close(inherited_ack);
                }
                install
            });
        }
        let child = command.spawn()?;
        drop(child_control);
        drop(child_owner_lifecycle_ack);
        let (sender, receiver) = mpsc::channel();
        let (bootstrap_sender, bootstrap_done) = mpsc::channel();
        // Register the PID before any fallible descriptor extraction or
        // worker setup.  Every later `?` remains inside this supervisor's
        // Drop/outer cleanup backstop, so it cannot orphan an exec child.
        self.children.push(SpawnedChild {
            slot,
            child,
            events: receiver,
            reader: None,
            bootstrap_done,
            bootstrap: None,
            bootstrap_complete: false,
            reaped: false,
            observed_exit_status: None,
            was_force_killed: false,
            terminal_event: None,
            owner_lifecycle_ack_stream: parent_owner_lifecycle_ack,
        });
        let tracked = self.children.last_mut().expect("registered child");
        let stdout = tracked
            .child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("stdout"))?;
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                // Do not allow a compromised child stdout stream to allocate
                // beyond the private event bound before parse rejection.
                let mut bounded = reader.by_ref().take((MAX_CHILD_EVENT_BYTES + 1) as u64);
                match bounded.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(size) if size <= MAX_CHILD_EVENT_BYTES => {
                        let _ = sender.send(serde_json::from_str(&line).map_err(|_| ()));
                    }
                    _ => {
                        let _ = sender.send(Err(()));
                        break;
                    }
                }
            }
        });
        tracked.reader = Some(reader);
        let stdin = tracked
            .child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("stdin"))?;
        let bootstrap = thread::spawn(move || {
            // The stalled falsifier deliberately withholds both records.  It
            // still drops the FDs immediately, allowing the child to be
            // reaped while the supervisor's bootstrap deadline is exercised.
            if control.setup_failure_during_stall {
                drop(stdin);
                drop(parent_control);
                let _ = bootstrap_sender.send(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "private setup/control failure",
                )));
                return;
            }
            if control.stall_bootstrap {
                drop(stdin);
                drop(parent_control);
                thread::sleep(Duration::from_millis(control.timeout_millis.max(1)));
                let _ = bootstrap_sender.send(Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "private bootstrap withheld",
                )));
                return;
            }
            let result = (|| {
                write_tainted_image(stdin, image)?;
                let encoded = Zeroizing::new(
                    serde_json::to_vec(&control).map_err(|_| io::Error::other("control encode"))?,
                );
                write_trusted_control(&mut parent_control, encoded.as_ref())
            })();
            let _ = bootstrap_sender.send(result);
        });
        tracked.bootstrap = Some(bootstrap);
        Ok(slot)
    }

    /// Register one provider child before moving either opaque image or FD3
    /// authority out of the runtime launch. Unlike the ordinary path above,
    /// the probe never serializes an image or a runtime control: both writes
    /// stay at the runtime-owned privileged boundary.
    fn spawn_provider_child(
        &mut self,
        transport: PrivateProviderChildTransportBootstrap,
    ) -> Result<(), ProviderChildSpawnError> {
        if Instant::now() >= self.deadline {
            return Err(ProviderChildSpawnError::deadline());
        }
        let slot = transport.slot;
        let role = slot.provider_role();
        let route = self
            .provider_launch
            .as_ref()
            .ok_or_else(|| {
                ProviderChildSpawnError::physical(
                    I3ReadOnlyProviderLocalnetBootstrapSubstage::RouteConsistency,
                )
            })?
            .child_route(role)
            .map_err(|error| {
                ProviderChildSpawnError::runtime(
                    I3ReadOnlyProviderLocalnetBootstrapSubstage::RouteConsistency,
                    &error,
                )
            })?;
        if route.role() != role
            || route.slot_name() != slot.slot_name()
            || !is_loopback_endpoint(route.endpoint())
        {
            return Err(ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::RouteConsistency,
            ));
        }
        let executable = probe_binary_path().ok_or_else(|| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ExecutableResolve,
            )
        })?;
        let (mut parent_control, child_control) = UnixStream::pair().map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlPipeSetup,
            )
        })?;
        set_close_on_exec(parent_control.as_raw_fd()).map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlPipeSetup,
            )
        })?;
        set_close_on_exec(child_control.as_raw_fd()).map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlPipeSetup,
            )
        })?;
        let child_fd = child_control.as_raw_fd();
        let mut command = Command::new(executable);
        command
            .env_clear()
            .arg(slot.provider_child_arg())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        // SAFETY: as on the ordinary path, this closure only installs the
        // already-open provider FD3 descriptor and checks every syscall.
        unsafe {
            command.pre_exec(move || {
                let inherited_control = duplicate_child_fd(child_fd)?;
                let install = install_child_fd(inherited_control, LOCALNET_CONTROL_FD);
                let _ = libc::close(inherited_control);
                install
            });
        }
        let child = command.spawn().map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::SpawnRegistration,
            )
        })?;
        drop(child_control);
        let (sender, receiver) = mpsc::channel();
        let (bootstrap_sender, bootstrap_done) = mpsc::channel();
        // This registration precedes the image and FD3 handoffs. Thus every
        // subsequent partial bootstrap, including the current typed-Pending
        // runtime implementation, remains inside the supervisor's reaping
        // ownership rather than orphaning an exec child.
        self.children.push(SpawnedChild {
            slot,
            child,
            events: receiver,
            reader: None,
            bootstrap_done,
            bootstrap: None,
            bootstrap_complete: false,
            reaped: false,
            observed_exit_status: None,
            was_force_killed: false,
            terminal_event: None,
            owner_lifecycle_ack_stream: None,
        });
        let tracked = self.children.last_mut().expect("registered provider child");
        let stdout = tracked.child.stdout.take().ok_or_else(|| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::SpawnRegistration,
            )
        })?;
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                let mut bounded = reader.by_ref().take((MAX_CHILD_EVENT_BYTES + 1) as u64);
                match bounded.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(size) if size <= MAX_CHILD_EVENT_BYTES => {
                        let _ = sender.send(serde_json::from_str(&line).map_err(|_| ()));
                    }
                    _ => {
                        let _ = sender.send(Err(()));
                        break;
                    }
                }
            }
        });
        tracked.reader = Some(reader);
        let stdin = tracked.child.stdin.take().ok_or_else(|| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::SpawnRegistration,
            )
        })?;
        let mut stdin = DeadlineChildStdin::new(stdin, self.deadline).map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::SpawnRegistration,
            )
        })?;
        let encoded = Zeroizing::new(serde_json::to_vec(&transport).map_err(|_| {
            ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlHandoff,
            )
        })?);
        if encoded.len() > MAX_PROVIDER_TRANSPORT_BOOTSTRAP_BYTES {
            return Err(ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlHandoff,
            ));
        }
        let launch = self
            .provider_launch
            .as_mut()
            .expect("provider launch was retained through child registration");
        launch
            .write_provider_child_image_once(role, &mut stdin)
            .map_err(|error| {
                ProviderChildSpawnError::runtime(
                    I3ReadOnlyProviderLocalnetBootstrapSubstage::ImageHandoff,
                    &error,
                )
            })?;
        // The child must observe EOF for its tainted image before the sole
        // runtime-owned FD3 frame is written and shut down.
        drop(stdin);
        if Instant::now() >= self.deadline {
            return Err(ProviderChildSpawnError::physical(
                I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlHandoff,
            ));
        }
        // The runtime keeps both opaque frame construction and bounded
        // nonblocking progress. Passing this unchanged supervisor deadline
        // permits continuation of only the one already-issued frame; it does
        // not let the probe retry or append a second control frame.
        launch
            .write_provider_child_control_once(
                role,
                &mut parent_control,
                encoded.as_ref(),
                self.deadline,
            )
            .map_err(|error| {
                ProviderChildSpawnError::runtime(
                    I3ReadOnlyProviderLocalnetBootstrapSubstage::ControlHandoff,
                    &error,
                )
            })?;
        let _ = bootstrap_sender.send(Ok(()));
        Ok(())
    }

    fn next_event(&mut self, slot: I3LocalnetChildSlot) -> Result<PrivateChildEvent, ()> {
        let remaining = self
            .deadline
            .checked_duration_since(Instant::now())
            .filter(|d| !d.is_zero())
            .ok_or(())?;
        let child = self
            .children
            .iter_mut()
            .find(|child| child.slot == slot)
            .ok_or(())?;
        if !child.bootstrap_complete {
            let bootstrap = child
                .bootstrap_done
                .recv_timeout(remaining)
                .map_err(|_| ())?;
            Self::join_finished_worker(&mut child.bootstrap);
            bootstrap.map_err(|_| ())?;
            child.bootstrap_complete = true;
        }
        let remaining = self
            .deadline
            .checked_duration_since(Instant::now())
            .filter(|d| !d.is_zero())
            .ok_or(())?;
        let event = child
            .events
            .recv_timeout(remaining)
            .map_err(|_| ())?
            .map_err(|_| ())?;
        if event.is_terminal() {
            child.terminal_event = Some(event.clone());
        }
        Ok(event)
    }

    fn take_registered_owner_lifecycle_ack_stream(&mut self) -> Option<UnixStream> {
        self.children
            .iter_mut()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessB)?
            .owner_lifecycle_ack_stream
            .take()
    }

    fn record_natural_exits(&mut self) {
        for child in &mut self.children {
            if child.reaped {
                continue;
            }
            if let Ok(Some(status)) = child.child.try_wait() {
                child.observed_exit_status = Some(status);
                child.reaped = true;
            }
        }
    }

    fn wait_for_natural_exits(&mut self) -> bool {
        // Both the natural-exit wait and forced-kill observation are slices of
        // one absolute lifecycle deadline.  The fixed reserve is carved from
        // the caller's allowance; cleanup never creates another timeout.
        let reaper_deadline = self.natural_reap_deadline();
        loop {
            self.record_natural_exits();
            if self.children.iter().all(|child| child.reaped) {
                let all_natural_zero = self.children.iter().all(|child| {
                    child
                        .observed_exit_status
                        .as_ref()
                        .is_some_and(ExitStatus::success)
                        && !child.was_force_killed
                });
                // Record all exit statuses first, then atomically classify a
                // natural zero-exit observation before any success return or
                // later evidence assembly.  A nonzero status is still reaped
                // here, but cannot acquire the positive-path observation.
                let observed_within_deadline =
                    !all_natural_zero || self.capture_zero_exit_reap_observation();
                self.join_finished_workers();
                return observed_within_deadline;
            }
            if Instant::now() >= reaper_deadline {
                self.natural_reaper_exhausted = true;
                return false;
            }
            Self::sleep_until(reaper_deadline);
        }
    }

    fn force_reap_after_allowance(&mut self) -> bool {
        for child in &mut self.children {
            if !child.reaped && child.child.kill().is_ok() {
                child.was_force_killed = true;
            }
        }
        // The remaining reserve ends at the same absolute deadline used by
        // `wait_for_natural_exits`; do not append a second 100ms timeout.
        let reap_deadline = self.total_lifecycle_deadline;
        loop {
            self.record_natural_exits();
            if self.children.iter().all(|child| child.reaped) {
                self.join_finished_workers();
                return true;
            }
            if Instant::now() >= reap_deadline {
                self.join_finished_workers();
                return false;
            }
            Self::sleep_until(reap_deadline);
        }
    }

    fn cleanup_after_failure(&mut self) -> bool {
        if self.natural_reaper_exhausted {
            return self.force_reap_after_allowance();
        }
        if self.wait_for_natural_exits() {
            return true;
        }
        self.force_reap_after_allowance()
    }

    fn join_finished_workers(&mut self) {
        for child in &mut self.children {
            // A worker is joined only after its completion channel confirms it
            // has returned; dropping an unfinished handle is non-blocking.
            Self::join_finished_worker(&mut child.reader);
            Self::join_finished_worker(&mut child.bootstrap);
        }
    }

    fn join_finished_worker(worker: &mut Option<thread::JoinHandle<()>>) {
        if worker.as_ref().is_some_and(thread::JoinHandle::is_finished)
            && let Some(worker) = worker.take()
        {
            let _ = worker.join();
        }
    }

    fn cleanup(&mut self) -> bool {
        self.cleanup_after_failure()
    }

    fn terminal_events(&self) -> Vec<I3LocalnetChildTerminalEvent> {
        self.children
            .iter()
            .filter_map(|child| {
                let mut event = child.terminal_event.as_ref()?.terminal_event()?;
                event.observed_exit_status_code = child
                    .observed_exit_status
                    .as_ref()
                    .and_then(ExitStatus::code);
                event.was_force_killed = child.was_force_killed;
                Some(event)
            })
            .collect()
    }

    /// A child may report only that it failed; this is intentionally not
    /// upgraded into an all-zero semantic or owner-runtime terminal record.
    fn unknown_child_failure_count(&self) -> usize {
        self.children
            .iter()
            .filter(|child| {
                child.terminal_event.as_ref().is_some_and(|event| {
                    matches!(event, PrivateChildEvent::UnknownLifecycleFailure { .. })
                })
            })
            .count()
    }

    /// Join only the two actual child terminal events.  The selected profile
    /// chooses which event shape is admissible, but never fills a reference,
    /// counter, or remote conclusion itself.
    fn fault_audit(
        &self,
        profile: I3LocalnetFaultProfile,
        requester_local_wait_profile: Option<I3LocalnetRequesterLocalWaitProfile>,
        request_contract: &Sys5I3AdapterCarrierContract,
    ) -> Option<(I3LocalnetFaultAudit, bool)> {
        let requester = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)?
            .terminal_event
            .as_ref()?;
        let owner = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessB)
            .and_then(|child| child.terminal_event.as_ref());
        let PrivateChildEvent::HandledDeliveryFault {
            slot: requester_slot,
            request_identity_ref: Some(request_identity_ref),
            requester_observation: Some(requester_observation),
            requester_pending_request_is_retained,
            requester_local_wait,
            remote_admission: None,
            ..
        } = requester
        else {
            return None;
        };
        if *requester_slot != I3LocalnetChildSlot::ProcessA || request_identity_ref.is_empty() {
            return None;
        }
        let validated_requester_local_wait = match requester_local_wait_profile {
            None if requester_local_wait.is_none() => None,
            Some(I3LocalnetRequesterLocalWaitProfile::WaitLocallyAfterObservedLoss)
                if profile == I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission =>
            {
                let evidence = requester_local_wait.as_ref()?;
                if !evidence.is_exact() {
                    return None;
                }
                Some(evidence.public())
            }
            _ => return None,
        };
        let (remote_admission, remote_evidence_rejection, requester_pending_request_is_retained) =
            match profile {
                I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite
                    if *requester_observation
                        == I3LocalnetRequesterFaultObservation::RequestCarrierWriteNotAttempted =>
                {
                    match owner {
                        Some(PrivateChildEvent::HandledDeliveryFault {
                            slot: I3LocalnetChildSlot::ProcessB,
                            request_identity_ref: None,
                            requester_observation: None,
                            remote_admission:
                                Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                                    owner_serve_count,
                                    owner_mutation_count: remote_owner_mutation_count,
                                }),
                            semantic_admission_count: 0,
                            owner_mutation_count: 0,
                            ..
                        }) if *owner_serve_count == 0 && *remote_owner_mutation_count == 0 => (
                            Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission {
                                owner_serve_count: *owner_serve_count,
                                owner_mutation_count: *remote_owner_mutation_count,
                            }),
                            None,
                            false,
                        ),
                        Some(PrivateChildEvent::HandledDeliveryFault {
                            slot: I3LocalnetChildSlot::ProcessB,
                            ..
                        }) => (
                            None,
                            Some(I3LocalnetRemoteEvidenceRejection::ProvenanceMismatch),
                            false,
                        ),
                        _ => (
                            None,
                            Some(I3LocalnetRemoteEvidenceRejection::ProvenanceMismatch),
                            false,
                        ),
                    }
                }
                I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission
                    if *requester_observation
                        == I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved
                        && *requester_pending_request_is_retained == Some(true) =>
                {
                    match owner {
                        Some(PrivateChildEvent::HandledDeliveryFault {
                            slot: I3LocalnetChildSlot::ProcessB,
                            request_identity_ref: Some(owner_request_identity_ref),
                            requester_observation: None,
                            remote_admission:
                                Some(PrivateRemoteAdmissionEvidence::Admitted {
                                    request_receive,
                                    owner_serve_count,
                                    owner_mutation_count: remote_owner_mutation_count,
                                    owner_serve_occurrence_ref,
                                    owner_write_occurrence_ref,
                                }),
                            semantic_admission_count: 1,
                            owner_mutation_count: 1,
                            ..
                        }) if owner_request_identity_ref == request_identity_ref
                            && request_receive.semantic_request_identity_ref
                                == *request_identity_ref
                            && *owner_serve_count == 1
                            && *remote_owner_mutation_count == 1
                            && !owner_serve_occurrence_ref.is_empty()
                            && owner_write_occurrence_ref
                                .as_deref()
                                .is_some_and(|reference| !reference.is_empty())
                            && request_receive.linked_request_identity_ref.is_none()
                            && !request_receive.carrier_ref.is_empty()
                            && !request_receive.network_occurrence_ref.is_empty()
                            && delivery_matches_contract(request_receive, request_contract) =>
                        {
                            (
                                Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
                                    request_receive: Box::new(delivery_record(
                                        I3LocalnetDeliveryPhase::RequestReceive,
                                        request_receive,
                                    )),
                                    owner_serve_count: *owner_serve_count,
                                    owner_mutation_count: *remote_owner_mutation_count,
                                    owner_serve_occurrence_ref: owner_serve_occurrence_ref.clone(),
                                    owner_write_occurrence_ref: owner_write_occurrence_ref.clone(),
                                }),
                                None,
                                true,
                            )
                        }
                        Some(PrivateChildEvent::HandledDeliveryFault {
                            slot: I3LocalnetChildSlot::ProcessB,
                            ..
                        }) => (
                            None,
                            Some(I3LocalnetRemoteEvidenceRejection::ProvenanceMismatch),
                            false,
                        ),
                        _ => (
                            None,
                            Some(I3LocalnetRemoteEvidenceRejection::ProvenanceMismatch),
                            false,
                        ),
                    }
                }
                I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit
                    if *requester_observation
                        == I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved
                        && *requester_pending_request_is_retained == Some(true)
                        && owner.is_none() =>
                {
                    (None, None, true)
                }
                _ => return None,
            };
        Some((
            I3LocalnetFaultAudit::from_actual_child_events(
                profile,
                request_identity_ref.clone(),
                *requester_observation,
                remote_admission,
                remote_evidence_rejection,
                validated_requester_local_wait,
            ),
            requester_pending_request_is_retained,
        ))
    }

    fn late_ingress_join(
        &self,
        profile: I3LocalnetLateIngressProfile,
        lineage: &SupervisorLineageEvidence,
        parent_observation: PrivateLateIngressParentObservation,
    ) -> Option<PrivateLateIngressJoin> {
        let requester = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)?
            .terminal_event
            .as_ref()?;
        let owner = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessB)?
            .terminal_event
            .as_ref()?;
        joined_late_ingress_audit(profile, lineage, requester, owner, parent_observation)
    }

    /// Returns only a bounded candidate-shaped ACK frame that A actually
    /// serialized in its captured terminal stdout event. The caller may decode
    /// it only as tainted input; this accessor cannot nominate B's registered
    /// reader or create a completion.
    fn requester_stdout_tainted_owner_lifecycle_ack_candidate(&self) -> Option<&[u8]> {
        let PrivateChildEvent::LateIngress {
            slot: I3LocalnetChildSlot::ProcessA,
            late_ingress_evidence,
            ..
        } = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)?
            .terminal_event
            .as_ref()?
        else {
            return None;
        };
        late_ingress_evidence
            .tainted_owner_lifecycle_ack_candidate
            .as_deref()
    }

    /// Join the two terminal retry reports.  This comparison never derives
    /// facts from a selected profile: contracts, identities, sessions,
    /// runtime summaries, counts, and the sender/receiver commitment all
    /// originate in the retained child events.
    fn retry_audit(
        &self,
        profile: I3LocalnetRetryProfile,
        lineage: &SupervisorLineageEvidence,
    ) -> Option<I3LocalnetRetryAudit> {
        let requester_event = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)?
            .terminal_event
            .as_ref()?;
        let owner_event = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessB)?
            .terminal_event
            .as_ref()?;
        let requester = requester_event.retry_evidence()?;
        let owner = owner_event.retry_evidence()?;
        if !retry_child_sessions_valid(requester)
            || !retry_child_sessions_valid(owner)
            || requester.run_ref != owner.run_ref
            || requester.run_ref.is_empty()
        {
            return None;
        }
        let request_delivery = requester.reconnect_request_delivery.as_ref()?;
        let request_identity_ref = request_delivery.semantic_request_identity_ref.clone();
        if !retry_request_delivery_matches_contract(
            request_delivery,
            &lineage.request_contract,
            &request_identity_ref,
        ) {
            return None;
        }
        let requester_child =
            retry_child_audit(I3LocalnetChildSlot::ProcessA, requester_event, requester)?;
        let owner_child = retry_child_audit(I3LocalnetChildSlot::ProcessB, owner_event, owner)?;
        let reconnect_attempt = requester
            .reconnect_session_runtime_attempt
            .as_ref()
            .and_then(retry_attempt_audit)?;
        if !retry_attempt_matches(
            &reconnect_attempt,
            &request_identity_ref,
            match profile {
                I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => 1,
                I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => 2,
            },
            I3LocalnetRetryAttemptReason::ReconnectRetry,
        ) {
            return None;
        }

        match profile {
            I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => {
                if !matches!(requester_event, PrivateChildEvent::Completed { .. })
                    || !matches!(owner_event, PrivateChildEvent::Completed { .. })
                    || requester.initial_request_delivery.is_some()
                    || requester.first_session_runtime_attempt.is_some()
                    || owner.initial_owner_admission.is_some()
                {
                    return None;
                }
                let PrivateReconnectOwnerOutcome::Replied {
                    owner_serve_count,
                    owner_mutation_count,
                    owner_serve_occurrence_ref,
                    owner_write_occurrence_ref,
                } = owner.reconnect_owner_outcome.as_ref()?
                else {
                    return None;
                };
                if *owner_serve_count != 1
                    || *owner_mutation_count != 1
                    || owner_serve_occurrence_ref.is_empty()
                    || owner_write_occurrence_ref
                        .as_deref()
                        .is_none_or(str::is_empty)
                {
                    return None;
                }
                Some(I3LocalnetRetryAudit::from_actual_child_events(
                    profile,
                    request_identity_ref,
                    I3LocalnetRetryRequesterOutcome::ReceiptConsumed,
                    false,
                    None,
                    delivery_record(I3LocalnetDeliveryPhase::RequestSend, request_delivery),
                    None,
                    None,
                    Some(I3LocalnetReconnectOwnerOutcome::Replied {
                        owner_serve_count: *owner_serve_count,
                        owner_mutation_count: *owner_mutation_count,
                        owner_serve_occurrence_ref: owner_serve_occurrence_ref.clone(),
                        owner_write_occurrence_ref: owner_write_occurrence_ref.clone(),
                    }),
                    None,
                    requester_child,
                    owner_child,
                ))
            }
            I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
                let PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessA,
                    request_identity_ref: Some(requester_identity_ref),
                    requester_observation:
                        Some(I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved),
                    remote_admission: None,
                    semantic_admission_count: 0,
                    owner_mutation_count: 0,
                    ..
                } = requester_event
                else {
                    return None;
                };
                let PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessB,
                    request_identity_ref: Some(owner_identity_ref),
                    requester_observation: None,
                    remote_admission: owner_admission_event,
                    semantic_admission_count: 1,
                    owner_mutation_count: event_owner_mutation_count,
                    ..
                } = owner_event
                else {
                    return None;
                };
                if requester_identity_ref != &request_identity_ref
                    || owner_identity_ref != &request_identity_ref
                    || !requester.requester_pending_request_is_retained
                {
                    return None;
                }
                let initial_request_delivery = requester.initial_request_delivery.as_ref()?;
                if !retry_request_delivery_matches_contract(
                    initial_request_delivery,
                    &lineage.request_contract,
                    &request_identity_ref,
                ) || initial_request_delivery.candidate_commitment_ref.is_empty()
                    || !delivery_semantics_match(initial_request_delivery, request_delivery)
                    || initial_request_delivery.network_occurrence_ref
                        == request_delivery.network_occurrence_ref
                {
                    return None;
                }
                let first_attempt = requester
                    .first_session_runtime_attempt
                    .as_ref()
                    .and_then(retry_attempt_audit)?;
                if !retry_attempt_matches(
                    &first_attempt,
                    &request_identity_ref,
                    1,
                    I3LocalnetRetryAttemptReason::InitialDelivery,
                ) || first_attempt.requester_binding_ref()
                    != reconnect_attempt.requester_binding_ref()
                {
                    return None;
                }
                match (
                    owner.initial_owner_admission.as_ref(),
                    owner.initial_owner_expiry.as_ref(),
                    owner_admission_event.as_ref(),
                ) {
                    (Some(initial_owner_admission), None, Some(owner_admission_event)) => {
                        if *event_owner_mutation_count != 1 {
                            return None;
                        }
                        let initial_owner_admission = retry_owner_admission(
                            initial_owner_admission,
                            &lineage.request_contract,
                            &request_identity_ref,
                        )?;
                        let event_owner_admission = retry_owner_admission(
                            owner_admission_event,
                            &lineage.request_contract,
                            &request_identity_ref,
                        )?;
                        if !same_remote_admission(&initial_owner_admission, &event_owner_admission)
                        {
                            return None;
                        }
                        let I3LocalnetRemoteAdmissionEvidence::Admitted {
                            request_receive: initial_owner_receive,
                            ..
                        } = &initial_owner_admission
                        else {
                            return None;
                        };
                        // The first sender and the owner admission must describe the
                        // same actual session-one frame, not merely a contract-shaped
                        // carrier with the same semantic request identity.
                        if initial_owner_receive.candidate_commitment_ref().is_empty()
                            || initial_request_delivery.candidate_commitment_ref.is_empty()
                            || initial_owner_receive.candidate_commitment_ref()
                                != initial_request_delivery.candidate_commitment_ref
                        {
                            return None;
                        }
                        let PrivateReconnectOwnerOutcome::DuplicateRequestRejected {
                            candidate_commitment_ref,
                            network_occurrence_ref,
                            owner_serve_count,
                            owner_mutation_count,
                        } = owner.reconnect_owner_outcome.as_ref()?
                        else {
                            return None;
                        };
                        if *owner_serve_count != 1
                            || *owner_mutation_count != 1
                            || network_occurrence_ref.is_empty()
                        {
                            return None;
                        }
                        let (reconnect_owner_outcome, evidence_rejection) = if request_delivery
                            .candidate_commitment_ref
                            .is_empty()
                            || candidate_commitment_ref.is_empty()
                        {
                            (
                                None,
                                Some(I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMissing),
                            )
                        } else if request_delivery.candidate_commitment_ref
                            != *candidate_commitment_ref
                        {
                            (
                                None,
                                Some(I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMismatch),
                            )
                        } else {
                            (
                                Some(I3LocalnetReconnectOwnerOutcome::DuplicateRequestRejected {
                                    candidate_commitment_ref: candidate_commitment_ref.clone(),
                                    network_occurrence_ref: network_occurrence_ref.clone(),
                                    owner_serve_count: *owner_serve_count,
                                    owner_mutation_count: *owner_mutation_count,
                                }),
                                None,
                            )
                        };
                        Some(I3LocalnetRetryAudit::from_actual_child_events(
                            profile,
                            request_identity_ref,
                            I3LocalnetRetryRequesterOutcome::PendingReplyOrReceiptNotObserved,
                            true,
                            Some(delivery_record(
                                I3LocalnetDeliveryPhase::RequestSend,
                                initial_request_delivery,
                            )),
                            delivery_record(I3LocalnetDeliveryPhase::RequestSend, request_delivery),
                            Some(initial_owner_admission),
                            None,
                            reconnect_owner_outcome,
                            evidence_rejection,
                            requester_child,
                            owner_child,
                        ))
                    }
                    (None, Some(expiry), None) => {
                        let request_receive = expiry.request_receive.as_deref()?;
                        if *event_owner_mutation_count != 0
                            || !expiry.is_exact_declared_expiry()
                            || !retry_request_delivery_matches_contract(
                                request_receive,
                                &lineage.request_contract,
                                &request_identity_ref,
                            )
                            || request_receive.candidate_commitment_ref.is_empty()
                            || request_receive.candidate_commitment_ref
                                != initial_request_delivery.candidate_commitment_ref
                        {
                            return None;
                        }
                        let PrivateReconnectOwnerOutcome::DuplicateRequestRejectedAfterDeclaredDeadlineExpired {
                            candidate_commitment_ref,
                            network_occurrence_ref,
                            owner_expired_count,
                            owner_serve_count,
                            owner_mutation_count,
                        } = owner.reconnect_owner_outcome.as_ref()?
                        else {
                            return None;
                        };
                        if *owner_expired_count != 1
                            || *owner_serve_count != 0
                            || *owner_mutation_count != 0
                            || network_occurrence_ref.is_empty()
                        {
                            return None;
                        }
                        let (reconnect_owner_outcome, evidence_rejection) = if request_delivery
                            .candidate_commitment_ref
                            .is_empty()
                            || candidate_commitment_ref.is_empty()
                        {
                            (
                                None,
                                Some(I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMissing),
                            )
                        } else if request_delivery.candidate_commitment_ref
                            != *candidate_commitment_ref
                        {
                            (
                                None,
                                Some(I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMismatch),
                            )
                        } else {
                            (
                                Some(I3LocalnetReconnectOwnerOutcome::DuplicateRequestRejectedAfterDeclaredDeadlineExpired {
                                    candidate_commitment_ref: candidate_commitment_ref.clone(),
                                    network_occurrence_ref: network_occurrence_ref.clone(),
                                    owner_expired_count: *owner_expired_count,
                                    owner_serve_count: *owner_serve_count,
                                    owner_mutation_count: *owner_mutation_count,
                                }),
                                None,
                            )
                        };
                        Some(I3LocalnetRetryAudit::from_actual_child_events(
                            profile,
                            request_identity_ref,
                            I3LocalnetRetryRequesterOutcome::PendingReplyOrReceiptNotObserved,
                            true,
                            Some(delivery_record(
                                I3LocalnetDeliveryPhase::RequestSend,
                                initial_request_delivery,
                            )),
                            delivery_record(I3LocalnetDeliveryPhase::RequestSend, request_delivery),
                            None,
                            Some(expiry.public()),
                            reconnect_owner_outcome,
                            evidence_rejection,
                            requester_child,
                            owner_child,
                        ))
                    }
                    _ => None,
                }
            }
        }
    }

    /// Joins only the two actual child reports for the bounded generated
    /// reply replay. The selected profile does not supply a result, delivery,
    /// session, or final state: every accepted fact below originated in the
    /// child event that observed it.
    fn owner_reply_replay_audit(
        &self,
        profile: I3LocalnetOwnerReplyReplayProfile,
        lineage: &SupervisorLineageEvidence,
    ) -> Option<I3LocalnetOwnerReplyReplayAudit> {
        let requester_event = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)?
            .terminal_event
            .as_ref()?;
        let owner_event = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessB)?
            .terminal_event
            .as_ref()?;
        let requester = requester_event.owner_reply_replay_evidence()?;
        let owner = owner_event.owner_reply_replay_evidence()?;
        if requester.run_ref.is_empty()
            || requester.run_ref != owner.run_ref
            || requester.cohort_provenance_ref.is_empty()
            || requester.cohort_provenance_ref != owner.cohort_provenance_ref
        {
            return None;
        }
        let requester_child = owner_reply_replay_child_audit(
            I3LocalnetChildSlot::ProcessA,
            requester_event,
            requester,
        )?;
        let owner_child =
            owner_reply_replay_child_audit(I3LocalnetChildSlot::ProcessB, owner_event, owner)?;
        let first_request_send = requester.first_request_send.as_ref()?;
        let first_request_receive = owner.first_request_receive.as_ref()?;
        let first_reply_send = owner.first_reply_send.as_ref()?;
        let first_reply_receive = requester.first_reply_receive.as_ref()?;
        let request_identity_ref = first_request_send.semantic_request_identity_ref.clone();
        if request_identity_ref.is_empty()
            || !retry_request_delivery_matches_contract(
                first_request_send,
                &lineage.request_contract,
                &request_identity_ref,
            )
            || !retry_request_delivery_matches_contract(
                first_request_receive,
                &lineage.request_contract,
                &request_identity_ref,
            )
            || !owner_reply_replay_reply_matches_contract(
                first_reply_send,
                &lineage.reply_contract,
                &request_identity_ref,
            )
            || !owner_reply_replay_reply_matches_contract(
                first_reply_receive,
                &lineage.reply_contract,
                &request_identity_ref,
            )
            || !delivery_semantics_match(first_request_send, first_request_receive)
            || !delivery_semantics_match(first_reply_send, first_reply_receive)
            || first_request_send.candidate_commitment_ref.is_empty()
            || first_request_receive.candidate_commitment_ref.is_empty()
            || first_request_send.candidate_commitment_ref
                != first_request_receive.candidate_commitment_ref
            || first_reply_send.candidate_commitment_ref
                != first_reply_receive.candidate_commitment_ref
            || owner
                .token_original_first_reply_send_occurrence_ref
                .as_deref()
                != Some(first_reply_send.network_occurrence_ref.as_str())
            || owner.token_original_first_reply_carrier_ref.as_deref()
                != Some(first_reply_send.carrier_ref.as_str())
        {
            return None;
        }
        let first_records = [
            first_request_send,
            first_request_receive,
            first_reply_send,
            first_reply_receive,
        ];
        if first_records
            .iter()
            .map(|record| record.network_occurrence_ref.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != first_records.len()
        {
            return None;
        }
        let requester_state_after_first = requester.requester_state_after_first.as_ref()?;
        let requester_final_state = requester.requester_final_state.as_ref()?;
        let owner_state_after_first = owner.owner_state_after_first.as_ref()?;
        let owner_state_after_replay_rejection =
            owner.owner_state_after_replay_rejection.as_ref()?;
        if !owner_reply_replay_terminal_matches(
            requester_event,
            I3LocalnetChildSlot::ProcessA,
            &request_identity_ref,
            0,
        ) || !owner_reply_replay_terminal_matches(
            owner_event,
            I3LocalnetChildSlot::ProcessB,
            &request_identity_ref,
            owner_state_after_first.owner_mutation_count,
        ) {
            return None;
        }
        let first_outcome = match requester.requester_first_outcome? {
            PrivateOwnerReplyReplayFirstOutcome::ReceiptConsumed => {
                if owner.owner_expiry.is_some()
                    || !owner_reply_replay_requester_state_is_exact(
                        requester_state_after_first,
                        false,
                    )
                    || !owner_reply_replay_owner_state_is_exact(owner_state_after_first, false)
                {
                    return None;
                }
                I3LocalnetOwnerReplyReplayFirstOutcome::ReceiptConsumed
            }
            PrivateOwnerReplyReplayFirstOutcome::TerminalFailureConsumed => {
                let expiry = owner.owner_expiry.as_ref()?;
                let expiry_request_receive = expiry.request_receive.as_deref()?;
                if !expiry.is_exact_declared_expiry()
                    || !delivery_semantics_match(expiry_request_receive, first_request_receive)
                    || expiry_request_receive.candidate_commitment_ref
                        != first_request_receive.candidate_commitment_ref
                    || expiry_request_receive.network_occurrence_ref
                        != first_request_receive.network_occurrence_ref
                    || !owner_reply_replay_requester_state_is_exact(
                        requester_state_after_first,
                        true,
                    )
                    || !owner_reply_replay_owner_state_is_exact(owner_state_after_first, true)
                {
                    return None;
                }
                I3LocalnetOwnerReplyReplayFirstOutcome::TerminalFailureConsumed {
                    owner_expiry: expiry.public(),
                }
            }
        };
        if requester_final_state != requester_state_after_first
            || owner_state_after_replay_rejection != owner_state_after_first
        {
            return None;
        }
        let replay_outcome = match (
            owner.replay_reply_send.as_ref(),
            requester.receiver_rejection.as_ref(),
            owner.replay_prewrite_rejection,
        ) {
            (Some(replay_reply_send), Some(receiver_rejection), None) => {
                if !owner_reply_replay_successor_session_valid(requester)
                    || !owner_reply_replay_successor_session_valid(owner)
                    || !owner_reply_replay_reply_matches_contract(
                        replay_reply_send,
                        &lineage.reply_contract,
                        &request_identity_ref,
                    )
                    || !delivery_semantics_match(first_reply_send, replay_reply_send)
                    || replay_reply_send.carrier_ref != first_reply_send.carrier_ref
                    || replay_reply_send.candidate_commitment_ref
                        == first_reply_send.candidate_commitment_ref
                    || replay_reply_send.network_occurrence_ref
                        == first_reply_send.network_occurrence_ref
                {
                    return None;
                }
                let PrivateOwnerReplyReplayReceiverRejection::CarrierAdmissionRejected {
                    rejected_candidate_commitment_ref,
                    network_occurrence_ref,
                } = receiver_rejection;
                if rejected_candidate_commitment_ref.is_empty()
                    || network_occurrence_ref.is_empty()
                    || rejected_candidate_commitment_ref
                        != &replay_reply_send.candidate_commitment_ref
                {
                    return None;
                }
                let records = [
                    first_request_send,
                    first_request_receive,
                    first_reply_send,
                    first_reply_receive,
                    replay_reply_send,
                ];
                if records
                    .iter()
                    .map(|record| record.network_occurrence_ref.as_str())
                    .chain(std::iter::once(network_occurrence_ref.as_str()))
                    .collect::<BTreeSet<_>>()
                    .len()
                    != records.len() + 1
                {
                    return None;
                }
                I3LocalnetOwnerReplyReplayOutcome::ReceiverRejected {
                    replay_reply_send: Box::new(delivery_record(
                        I3LocalnetDeliveryPhase::ReplySend,
                        replay_reply_send,
                    )),
                    receiver_rejection:
                        I3LocalnetOwnerReplyReplayReceiverRejection::CarrierAdmissionRejected {
                            rejected_candidate_commitment_ref: rejected_candidate_commitment_ref
                                .clone(),
                            network_occurrence_ref: network_occurrence_ref.clone(),
                        },
                }
            }
            (None, None, Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected)) => {
                if !owner_reply_replay_no_successor_session(requester)
                    || !owner_reply_replay_no_successor_session(owner)
                {
                    return None;
                }
                I3LocalnetOwnerReplyReplayOutcome::RejectedBeforeReplayWrite {
                    rejection: I3LocalnetAdapterRejectionKind::LocalAttemptRejected,
                }
            }
            _ => return None,
        };
        Some(I3LocalnetOwnerReplyReplayAudit::from_actual_observation(
            profile,
            request_identity_ref,
            requester.cohort_provenance_ref.clone(),
            delivery_record(I3LocalnetDeliveryPhase::RequestSend, first_request_send),
            delivery_record(
                I3LocalnetDeliveryPhase::RequestReceive,
                first_request_receive,
            ),
            delivery_record(I3LocalnetDeliveryPhase::ReplySend, first_reply_send),
            delivery_record(I3LocalnetDeliveryPhase::ReplyReceive, first_reply_receive),
            owner
                .token_original_first_reply_send_occurrence_ref
                .clone()?,
            owner.token_original_first_reply_carrier_ref.clone()?,
            first_outcome,
            replay_outcome,
            owner_reply_replay_public_requester_state(requester_state_after_first),
            owner_reply_replay_public_requester_state(requester_final_state),
            owner_reply_replay_public_owner_state(owner_state_after_first),
            owner_reply_replay_public_owner_state(owner_state_after_replay_rejection),
            requester_child,
            owner_child,
        ))
    }

    /// Retains only A's independently checkable pending observation when a
    /// larger retry join is rejected.  This does not validate B's outcome or
    /// turn the retry into an accepted audit.
    fn validated_requester_retry_pending_observation(
        &self,
        request_contract: &Sys5I3AdapterCarrierContract,
    ) -> bool {
        let Some(requester_event) = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)
            .and_then(|child| child.terminal_event.as_ref())
        else {
            return false;
        };
        let PrivateChildEvent::HandledDeliveryFault {
            slot: I3LocalnetChildSlot::ProcessA,
            request_identity_ref: Some(request_identity_ref),
            requester_observation:
                Some(I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved),
            // Retry owns its retained-pending proof in `retry_evidence`; it
            // must not borrow the ordinary post-loss fault observation.
            requester_pending_request_is_retained: None,
            requester_local_wait: None,
            remote_admission: None,
            semantic_admission_count: 0,
            owner_mutation_count: 0,
            retry_evidence: Some(retry_evidence),
            ..
        } = requester_event
        else {
            return false;
        };
        if request_identity_ref.is_empty() || !retry_evidence.requester_pending_request_is_retained
        {
            return false;
        }
        let (Some(initial_delivery), Some(reconnect_delivery)) = (
            retry_evidence.initial_request_delivery.as_ref(),
            retry_evidence.reconnect_request_delivery.as_ref(),
        ) else {
            return false;
        };
        let (Some(first_attempt), Some(reconnect_attempt)) = (
            retry_evidence
                .first_session_runtime_attempt
                .as_ref()
                .and_then(retry_attempt_audit),
            retry_evidence
                .reconnect_session_runtime_attempt
                .as_ref()
                .and_then(retry_attempt_audit),
        ) else {
            return false;
        };
        retry_request_delivery_matches_contract(
            initial_delivery,
            request_contract,
            request_identity_ref,
        ) && retry_request_delivery_matches_contract(
            reconnect_delivery,
            request_contract,
            request_identity_ref,
        ) && delivery_semantics_match(initial_delivery, reconnect_delivery)
            && initial_delivery.network_occurrence_ref != reconnect_delivery.network_occurrence_ref
            && retry_attempt_matches(
                &first_attempt,
                request_identity_ref,
                1,
                I3LocalnetRetryAttemptReason::InitialDelivery,
            )
            && retry_attempt_matches(
                &reconnect_attempt,
                request_identity_ref,
                2,
                I3LocalnetRetryAttemptReason::ReconnectRetry,
            )
    }

    /// Preserve only A's independently source-contract-checked pending fact
    /// when the later retained-ingress observer join is rejected. B's
    /// received frame, lifecycle, and semantic outcome stay unaccepted.
    fn validated_requester_late_ingress_pending_observation(
        &self,
        request_contract: &Sys5I3AdapterCarrierContract,
    ) -> bool {
        let Some(requester_event) = self
            .children
            .iter()
            .find(|child| child.slot == I3LocalnetChildSlot::ProcessA)
            .and_then(|child| child.terminal_event.as_ref())
        else {
            return false;
        };
        let PrivateChildEvent::LateIngress {
            slot: I3LocalnetChildSlot::ProcessA,
            terminal_outcome: PrivateLateIngressTerminalOutcome::HandledDeliveryFault,
            generated_request_count: 1,
            semantic_admission_count: 0,
            receipt_count: 0,
            late_ingress_evidence,
            ..
        } = requester_event
        else {
            return false;
        };
        if validated_late_ingress_child_audit(
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
            I3LocalnetChildSlot::ProcessA,
            requester_event,
            late_ingress_evidence,
        )
        .is_none()
        {
            return false;
        }
        let Some(initial_sender) = late_ingress_evidence.initial_sender_delivery.as_ref() else {
            return false;
        };
        let request_identity_ref = &initial_sender.semantic_request_identity_ref;
        late_ingress_child_sessions_valid(late_ingress_evidence)
            && !late_ingress_evidence.run_ref.is_empty()
            && late_ingress_evidence.requester_outcome
                == Some(I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved)
            && late_ingress_evidence.outbound_request_frame_write_count == Some(1)
            && !request_identity_ref.is_empty()
            && retry_request_delivery_matches_contract(
                initial_sender,
                request_contract,
                request_identity_ref,
            )
            && !initial_sender.candidate_commitment_ref.is_empty()
    }

    fn completed_child_exited_nonzero(&self) -> bool {
        self.children.iter().any(|child| {
            child
                .terminal_event
                .as_ref()
                .is_some_and(PrivateChildEvent::is_completed)
                && child
                    .observed_exit_status
                    .as_ref()
                    .is_some_and(|status| !status.success())
        })
    }

    fn completed_child_hung_and_was_force_killed(&self) -> bool {
        self.children.iter().any(|child| {
            child
                .terminal_event
                .as_ref()
                .is_some_and(PrivateChildEvent::is_completed)
                && child.was_force_killed
        })
    }

    fn completed_child_exhausted_natural_reaper(&self) -> bool {
        self.natural_reaper_exhausted
            && self.children.iter().any(|child| {
                child
                    .terminal_event
                    .as_ref()
                    .is_some_and(PrivateChildEvent::is_completed)
            })
    }

    fn mark_reaped_in(
        &mut self,
        audits: &mut BTreeMap<I3LocalnetChildSlot, I3LocalnetChildAudit>,
    ) -> bool {
        // Success is valid only after every child naturally exits zero; a
        // force kill is evidence of failure even if the PID is reaped.
        let all_reaped = self.wait_for_natural_exits();
        for audit in audits.values_mut() {
            if let Some(child) = self.children.iter().find(|child| child.slot == audit.slot) {
                audit.reaped = child.reaped;
                audit.observed_exit_status = child.observed_exit_status;
                audit.was_force_killed = child.was_force_killed;
            }
        }
        all_reaped
            && audits.values().all(|audit| {
                audit
                    .observed_exit_status
                    .as_ref()
                    .is_some_and(ExitStatus::success)
                    && !audit.was_force_killed
            })
    }
}

impl Drop for LocalnetSupervisor {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn set_close_on_exec(fd: RawFd) -> io::Result<()> {
    // Parent-held descriptor ends must not survive a later child exec. The
    // inherited child end is installed separately below with FD_CLOEXEC
    // cleared only at its fixed private target.
    let result = unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) };
    if result == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn duplicate_child_fd(fd: RawFd) -> io::Result<RawFd> {
    let duplicate = unsafe {
        libc::fcntl(
            fd,
            libc::F_DUPFD_CLOEXEC,
            LOCALNET_OWNER_LIFECYCLE_ACK_FD + 1,
        )
    };
    if duplicate == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(duplicate)
}

fn install_child_fd(source: RawFd, target: RawFd) -> io::Result<()> {
    if unsafe { libc::dup2(source, target) } == -1 {
        return Err(io::Error::last_os_error());
    }
    if unsafe { libc::fcntl(target, libc::F_SETFD, 0) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// A one-shot image writer bounded by the supervisor's existing absolute
/// main deadline. It owns no semantic/control data and makes no retry: a
/// stalled child stdin fails closed so the caller can enter the existing
/// natural/forced reap path.
struct DeadlineChildStdin {
    inner: std::process::ChildStdin,
    deadline: Instant,
}

impl DeadlineChildStdin {
    fn new(inner: std::process::ChildStdin, deadline: Instant) -> io::Result<Self> {
        let fd = inner.as_raw_fd();
        // SAFETY: fcntl only changes the parent-owned pipe descriptor that is
        // consumed by this wrapper and never inherited by another child.
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags == -1 {
            return Err(io::Error::last_os_error());
        }
        if unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self { inner, deadline })
    }

    fn deadline_exceeded() -> io::Error {
        io::Error::new(io::ErrorKind::TimedOut, "provider image handoff deadline")
    }

    fn wait_writable(&self) -> io::Result<()> {
        let mut pollfd = libc::pollfd {
            fd: self.inner.as_raw_fd(),
            events: libc::POLLOUT,
            revents: 0,
        };
        loop {
            let remaining = self.deadline.saturating_duration_since(Instant::now());
            // Failing closed during a sub-millisecond remainder is preferable
            // to rounding a poll timeout upward beyond the finite main
            // deadline. Recompute after EINTR rather than retaining a stale
            // timeout that could outlive that deadline.
            let timeout_millis = i32::try_from(remaining.as_millis()).unwrap_or(i32::MAX);
            if timeout_millis == 0 {
                return Err(Self::deadline_exceeded());
            }
            // SAFETY: pollfd points to one valid parent-owned pipe descriptor
            // for the duration of this call.
            let result = unsafe { libc::poll(&mut pollfd, 1, timeout_millis) };
            if result == 0 {
                return Err(Self::deadline_exceeded());
            }
            if result < 0 {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::Interrupted {
                    if Instant::now() >= self.deadline {
                        return Err(Self::deadline_exceeded());
                    }
                    continue;
                }
                return Err(error);
            }
            if pollfd.revents & libc::POLLOUT != 0 {
                return Ok(());
            }
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "provider image pipe is no longer writable",
            ));
        }
    }
}

impl Write for DeadlineChildStdin {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        loop {
            if Instant::now() >= self.deadline {
                return Err(Self::deadline_exceeded());
            }
            match self.inner.write(buffer) {
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => self.wait_writable()?,
                result => return result,
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        // This wrapper has no user-space buffer; successful writes already
        // reached the kernel pipe. EOF on drop remains the image boundary.
        Ok(())
    }
}

fn write_tainted_image(mut stdin: std::process::ChildStdin, image: Vec<u8>) -> io::Result<()> {
    stdin.write_all(&image)?;
    stdin.flush()
}

fn write_trusted_control(stream: &mut UnixStream, encoded: &[u8]) -> io::Result<()> {
    if encoded.len() > MAX_TRUSTED_CONTROL_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "control too large",
        ));
    }
    stream.write_all(
        &(u32::try_from(encoded.len()).map_err(|_| io::Error::other("control length"))?)
            .to_be_bytes(),
    )?;
    stream.write_all(encoded)?;
    stream.flush()?;
    stream.shutdown(Shutdown::Write)
}

fn logical_source_path(path: &Path) -> &'static str {
    let path = path.to_string_lossy();
    if path.ends_with(ACTIVE_I2_LOGICAL_SOURCE_PATH) {
        ACTIVE_I2_LOGICAL_SOURCE_PATH
    } else if path.ends_with(OWNER_ADMISSION_LOGICAL_SOURCE_PATH) {
        OWNER_ADMISSION_LOGICAL_SOURCE_PATH
    } else {
        "i3-2-private-input.mir"
    }
}

fn project_adapter_contract(
    project: &mir_runtime::sys5_local_slice::Sys5LocalProject,
    kind: &str,
) -> Result<Sys5I3AdapterCarrierContract, ()> {
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == "init_avatar_hp" && edge.kind == kind)
        .ok_or(())?;
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .map_err(|_| ())
}

struct JoinedObserverEvidence {
    references: I3LocalnetObserverSafeReferences,
    delivery_records: Vec<I3LocalnetObserverSafeDeliveryRecord>,
    source_ref_inventory: Vec<String>,
    core_ref_inventory: Vec<String>,
    artifact_ref_inventory: Vec<String>,
    edge_ref_inventory: Vec<String>,
}

fn joined_observer_evidence(
    lineage: &SupervisorLineageEvidence,
    requester: &PrivateChildCompleted,
    owner: &PrivateChildCompleted,
) -> Option<JoinedObserverEvidence> {
    let request_sent = requester.observer_evidence.request_sent.as_ref()?;
    let request_received = owner.observer_evidence.request_received.as_ref()?;
    let reply_sent = owner.observer_evidence.reply_sent.as_ref()?;
    let reply_received = requester.observer_evidence.reply_received.as_ref()?;
    let request_identity = &request_sent.semantic_request_identity_ref;
    if request_identity.is_empty()
        || request_identity != &request_received.semantic_request_identity_ref
        || request_identity != &reply_sent.semantic_request_identity_ref
        || request_identity != &reply_received.semantic_request_identity_ref
        || reply_sent.linked_request_identity_ref.as_deref() != Some(request_identity)
        || reply_received.linked_request_identity_ref.as_deref() != Some(request_identity)
        || !delivery_semantics_match(request_sent, request_received)
        || !delivery_semantics_match(reply_sent, reply_received)
        || !delivery_matches_contract(request_sent, &lineage.request_contract)
        || !delivery_matches_contract(request_received, &lineage.request_contract)
        || !delivery_matches_contract(reply_sent, &lineage.reply_contract)
        || !delivery_matches_contract(reply_received, &lineage.reply_contract)
        || requester.observer_evidence.local_store_ref.is_empty()
        || owner.observer_evidence.local_store_ref.is_empty()
        || requester.observer_evidence.local_store_ref == owner.observer_evidence.local_store_ref
        || !valid_generated_frame_write_observation(
            request_sent.generated_frame_write_observation.as_ref(),
        )
        || request_received.generated_frame_write_observation.is_some()
        || reply_sent.generated_frame_write_observation.is_some()
        || reply_received.generated_frame_write_observation.is_some()
    {
        return None;
    }
    let serve = owner
        .observer_evidence
        .owner_serve_occurrence_ref
        .as_ref()?;
    let write = owner
        .observer_evidence
        .owner_write_occurrence_ref
        .as_ref()?;
    let receipt = requester
        .observer_evidence
        .requester_receipt_occurrence_ref
        .as_ref()?;
    if serve.is_empty() || write.is_empty() || receipt.is_empty() {
        return None;
    }
    let delivery_records = vec![
        delivery_record(I3LocalnetDeliveryPhase::RequestSend, request_sent),
        delivery_record(I3LocalnetDeliveryPhase::RequestReceive, request_received),
        delivery_record(I3LocalnetDeliveryPhase::ReplySend, reply_sent),
        delivery_record(I3LocalnetDeliveryPhase::ReplyReceive, reply_received),
    ];
    if delivery_records
        .iter()
        .map(I3LocalnetObserverSafeDeliveryRecord::network_occurrence_ref)
        .collect::<BTreeSet<_>>()
        .len()
        != delivery_records.len()
    {
        return None;
    }
    let source_ref_inventory = delivery_records
        .iter()
        .map(|record| record.source_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let core_ref_inventory = delivery_records
        .iter()
        .map(|record| record.core_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let artifact_ref_inventory = delivery_records
        .iter()
        .flat_map(|record| {
            [
                record.source_artifact_ref.clone(),
                record.target_artifact_ref.clone(),
            ]
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let edge_ref_inventory = delivery_records
        .iter()
        .map(|record| record.edge_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let references = I3LocalnetObserverSafeReferences {
        request_source_ref: request_sent.source_ref.clone(),
        request_core_ref: request_sent.core_ref.clone(),
        request_source_artifact_ref: request_sent.source_artifact_ref.clone(),
        request_target_artifact_ref: request_sent.target_artifact_ref.clone(),
        request_edge_ref: request_sent.edge_ref.clone(),
        reply_source_ref: reply_sent.source_ref.clone(),
        reply_core_ref: reply_sent.core_ref.clone(),
        reply_source_artifact_ref: reply_sent.source_artifact_ref.clone(),
        reply_target_artifact_ref: reply_sent.target_artifact_ref.clone(),
        reply_edge_ref: reply_sent.edge_ref.clone(),
        request_carrier_ref: request_sent.carrier_ref.clone(),
        reply_carrier_ref: reply_sent.carrier_ref.clone(),
        semantic_request_identity_ref: request_identity.clone(),
        network_request_identity_ref: request_received.semantic_request_identity_ref.clone(),
        network_reply_linked_request_identity_ref: reply_received
            .linked_request_identity_ref
            .clone()
            .unwrap_or_default(),
        network_request_occurrence_ref: request_received.network_occurrence_ref.clone(),
        network_reply_occurrence_ref: reply_received.network_occurrence_ref.clone(),
        runtime_serve_request_identity_ref: request_identity.clone(),
        runtime_write_request_identity_ref: request_identity.clone(),
        runtime_receipt_linked_request_identity_ref: request_identity.clone(),
        runtime_serve_occurrence_ref: serve.clone(),
        runtime_write_occurrence_ref: write.clone(),
        runtime_receipt_occurrence_ref: receipt.clone(),
        requester_local_store_ref: requester.observer_evidence.local_store_ref.clone(),
        owner_local_store_ref: owner.observer_evidence.local_store_ref.clone(),
    };
    Some(JoinedObserverEvidence {
        references,
        delivery_records,
        source_ref_inventory,
        core_ref_inventory,
        artifact_ref_inventory,
        edge_ref_inventory,
    })
}

/// Join the completed children only for a generated declared owner failure
/// that was actually delivered and consumed. This is deliberately separate
/// from the successful receipt join: the carrier route is identical, while
/// the runtime terminal and owner counters are not.
fn joined_declared_owner_deadline_expired_evidence(
    lineage: &SupervisorLineageEvidence,
    requester: &PrivateChildCompleted,
    owner: &PrivateChildCompleted,
) -> Option<I3LocalnetDeclaredOwnerDeadlineExpiredAudit> {
    let request_sent = requester.observer_evidence.request_sent.as_ref()?;
    let request_received = owner.observer_evidence.request_received.as_ref()?;
    let reply_sent = owner.observer_evidence.reply_sent.as_ref()?;
    let reply_received = requester.observer_evidence.reply_received.as_ref()?;
    let request_identity_ref = &request_sent.semantic_request_identity_ref;
    let terminal_occurrence_ref = requester
        .observer_evidence
        .requester_terminal_failure_occurrence_ref
        .as_ref()?;
    let expiry = owner.observer_evidence.owner_expiry.as_ref()?;
    if lineage.ordinary_source_build_count != 1
        || lineage.admission_count != 1
        || lineage.m9_generation_count != 1
        || request_identity_ref.is_empty()
        || request_identity_ref != &request_received.semantic_request_identity_ref
        || request_identity_ref != &reply_sent.semantic_request_identity_ref
        || request_identity_ref != &reply_received.semantic_request_identity_ref
        || reply_sent.linked_request_identity_ref.as_deref() != Some(request_identity_ref)
        || reply_received.linked_request_identity_ref.as_deref() != Some(request_identity_ref)
        || !delivery_semantics_match(request_sent, request_received)
        || !delivery_semantics_match(reply_sent, reply_received)
        || !delivery_matches_contract(request_sent, &lineage.request_contract)
        || !delivery_matches_contract(request_received, &lineage.request_contract)
        || !delivery_matches_contract(reply_sent, &lineage.reply_contract)
        || !delivery_matches_contract(reply_received, &lineage.reply_contract)
        || requester.observer_evidence.local_store_ref.is_empty()
        || owner.observer_evidence.local_store_ref.is_empty()
        || requester.observer_evidence.local_store_ref == owner.observer_evidence.local_store_ref
        || !valid_generated_frame_write_observation(
            request_sent.generated_frame_write_observation.as_ref(),
        )
        || request_sent.carrier_ref.is_empty()
        || reply_sent.carrier_ref.is_empty()
        || request_received.generated_frame_write_observation.is_some()
        || reply_sent.generated_frame_write_observation.is_some()
        || reply_received.generated_frame_write_observation.is_some()
        || requester
            .observer_evidence
            .requester_receipt_occurrence_ref
            .is_some()
        || owner.observer_evidence.owner_serve_occurrence_ref.is_some()
        || owner.observer_evidence.owner_write_occurrence_ref.is_some()
        || terminal_occurrence_ref.is_empty()
        || !expiry.is_exact_declared_expiry()
    {
        return None;
    }
    let records = [
        delivery_record(I3LocalnetDeliveryPhase::RequestSend, request_sent),
        delivery_record(I3LocalnetDeliveryPhase::RequestReceive, request_received),
        delivery_record(I3LocalnetDeliveryPhase::ReplySend, reply_sent),
        delivery_record(I3LocalnetDeliveryPhase::ReplyReceive, reply_received),
    ];
    if records
        .iter()
        .map(I3LocalnetObserverSafeDeliveryRecord::network_occurrence_ref)
        .collect::<BTreeSet<_>>()
        .len()
        != records.len()
    {
        return None;
    }
    Some(I3LocalnetDeclaredOwnerDeadlineExpiredAudit {
        request_identity_ref: request_identity_ref.clone(),
        request_send: records[0].clone(),
        request_receive: records[1].clone(),
        reply_send: records[2].clone(),
        reply_receive: records[3].clone(),
        owner_expiry: expiry.public(),
        requester_terminal_failure_occurrence_ref: terminal_occurrence_ref.clone(),
        requester_terminal_failure_consumed_count: requester.runtime_occurrence_count,
        requester_local_receipt_count: requester.receipt_count,
    })
}

fn valid_generated_frame_write_observation(
    observation: Option<&PrivateGeneratedFrameWriteObservation>,
) -> bool {
    observation.is_none_or(|observation| {
        observation.application_write_count == 2
            && observation.frame_prefix_split_across_writes
            && observation.complete_frame_written
    })
}

fn delivery_record(
    phase: I3LocalnetDeliveryPhase,
    evidence: &PrivateDeliveryEvidence,
) -> I3LocalnetObserverSafeDeliveryRecord {
    I3LocalnetObserverSafeDeliveryRecord {
        phase,
        source_ref: evidence.source_ref.clone(),
        core_ref: evidence.core_ref.clone(),
        source_artifact_ref: evidence.source_artifact_ref.clone(),
        target_artifact_ref: evidence.target_artifact_ref.clone(),
        edge_ref: evidence.edge_ref.clone(),
        carrier_ref: evidence.carrier_ref.clone(),
        semantic_request_identity_ref: evidence.semantic_request_identity_ref.clone(),
        linked_request_identity_ref: evidence.linked_request_identity_ref.clone(),
        network_occurrence_ref: evidence.network_occurrence_ref.clone(),
        candidate_commitment_ref: evidence.candidate_commitment_ref.clone(),
        generated_frame_write_observation: evidence.generated_frame_write_observation.map(
            |observation| I3LocalnetGeneratedFrameWriteObservation {
                application_write_count: observation.application_write_count,
                frame_prefix_split_across_writes: observation.frame_prefix_split_across_writes,
                complete_frame_written: observation.complete_frame_written,
            },
        ),
    }
}

enum PrivateLateIngressJoin {
    Accepted(Box<I3LocalnetLateIngressAudit>),
    EvidenceRejected(I3LocalnetLateIngressEvidenceRejection),
}

fn late_ingress_child_sessions_valid(evidence: &PrivateChildLateIngressEvidence) -> bool {
    evidence.first_session_generation == 1
        && evidence.reconnect_session_generation == 2
        && evidence.first_session_peer_spki_verified
        && evidence.first_session_reciprocal_preface_verified
        && evidence.reconnect_session_peer_spki_verified
        && evidence.reconnect_session_reciprocal_preface_verified
}

/// Validate the complete observer-safe terminal contract before accepting any
/// late-ingress child evidence. This is deliberately shared by the full join
/// and A's independently retained-pending fallback so a malformed A terminal
/// cannot keep pending merely because B's later join failed.
fn validated_late_ingress_child_audit(
    profile: I3LocalnetLateIngressProfile,
    slot: I3LocalnetChildSlot,
    event: &PrivateChildEvent,
    evidence: &PrivateChildLateIngressEvidence,
) -> Option<I3LocalnetLateIngressChildAudit> {
    let PrivateChildEvent::LateIngress {
        slot: reported_slot,
        terminal_outcome,
        exec_confirmed,
        assigned_loci,
        trusted_control_consumed,
        tainted_image_consumed,
        tls_peer_verified,
        reciprocal_preface_verified,
        reliable_bidi_stream_count,
        quic_datagrams_enabled,
        semantic_admission_count,
        unauthenticated_semantic_admission_count,
        network_receipt_frame_count,
        generated_request_count,
        served_count,
        write_count,
        reply_count,
        receipt_count,
        runtime_occurrence_count,
        ..
    } = event
    else {
        return None;
    };
    if *reported_slot != slot
        || evidence.run_ref.is_empty()
        || !late_ingress_child_sessions_valid(evidence)
        || !*exec_confirmed
        || !matches_slot_loci(assigned_loci, slot)
        || !*trusted_control_consumed
        || !*tainted_image_consumed
        || !*tls_peer_verified
        || !*reciprocal_preface_verified
        || *reliable_bidi_stream_count != 1
        || *quic_datagrams_enabled
        || *unauthenticated_semantic_admission_count != 0
        || *network_receipt_frame_count != 0
    {
        return None;
    }
    let counts_valid = match (profile, slot) {
        (
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect,
            I3LocalnetChildSlot::ProcessA,
        ) => {
            *terminal_outcome == PrivateLateIngressTerminalOutcome::Completed
                && *generated_request_count == 1
                && *served_count == 0
                && *write_count == 0
                && *reply_count == 0
                && *receipt_count == 1
                && *semantic_admission_count == 1
                && *runtime_occurrence_count == 1
        }
        (
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect,
            I3LocalnetChildSlot::ProcessB,
        ) => {
            *terminal_outcome == PrivateLateIngressTerminalOutcome::Completed
                && *generated_request_count == 0
                && *served_count == 1
                && *write_count == 1
                && *reply_count == 1
                && *receipt_count == 0
                && *semantic_admission_count == 1
                && *runtime_occurrence_count == 2
        }
        (
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
            I3LocalnetChildSlot::ProcessA,
        ) => {
            *terminal_outcome == PrivateLateIngressTerminalOutcome::HandledDeliveryFault
                && *generated_request_count == 1
                && *served_count == 0
                && *write_count == 0
                && *reply_count == 0
                && *receipt_count == 0
                && *semantic_admission_count == 0
                && *runtime_occurrence_count == 0
        }
        (
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
            I3LocalnetChildSlot::ProcessB,
        ) => {
            *terminal_outcome == PrivateLateIngressTerminalOutcome::HandledDeliveryFault
                && *generated_request_count == 0
                && *served_count == 0
                && *write_count == 0
                && *reply_count == 0
                && *receipt_count == 0
                && *semantic_admission_count == 0
                && *runtime_occurrence_count == 0
        }
    };
    if !counts_valid {
        return None;
    }
    Some(I3LocalnetLateIngressChildAudit {
        slot,
        run_ref: evidence.run_ref.clone(),
        first_session_generation: evidence.first_session_generation,
        reconnect_session_generation: evidence.reconnect_session_generation,
        first_session_peer_spki_verified: evidence.first_session_peer_spki_verified,
        first_session_reciprocal_preface_verified: evidence
            .first_session_reciprocal_preface_verified,
        reconnect_session_peer_spki_verified: evidence.reconnect_session_peer_spki_verified,
        reconnect_session_reciprocal_preface_verified: evidence
            .reconnect_session_reciprocal_preface_verified,
        terminal_outcome: terminal_outcome.public(),
    })
}

fn late_ingress_owner_outcome(
    outcome: &PrivateLateIngressOwnerOutcome,
) -> I3LocalnetLateIngressOwnerOutcome {
    match outcome {
        PrivateLateIngressOwnerOutcome::Admitted {
            owner_serve_count,
            owner_mutation_count,
        } => I3LocalnetLateIngressOwnerOutcome::Admitted {
            owner_serve_count: *owner_serve_count,
            owner_mutation_count: *owner_mutation_count,
        },
        PrivateLateIngressOwnerOutcome::CarrierAdmissionRejected {
            owner_serve_count,
            owner_mutation_count,
        } => I3LocalnetLateIngressOwnerOutcome::CarrierAdmissionRejected {
            owner_serve_count: *owner_serve_count,
            owner_mutation_count: *owner_mutation_count,
        },
    }
}

fn late_ingress_parent_observation_valid(
    profile: I3LocalnetLateIngressProfile,
    observation: PrivateLateIngressParentObservation,
) -> bool {
    match profile {
        I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
            observation.ack_reader_outcome == I3LocalnetLateIngressAckReaderOutcome::NotSelected
                && observation.nonregistered_ack_input_disposition
                    == I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved
                && observation.nonregistered_ack_input_count == 0
                && observation.publication
                    == I3LocalnetLateIngressParentPublication::NoPrestageSelected
                && observation.publication_commit_count == 0
        }
        I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
            match observation.ack_reader_outcome {
                I3LocalnetLateIngressAckReaderOutcome::AcceptedRegisteredOwnerChildFd
                | I3LocalnetLateIngressAckReaderOutcome::ReplayRejected => {
                    observation.nonregistered_ack_input_disposition
                        == I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved
                        && observation.nonregistered_ack_input_count == 0
                        &&
                    observation.publication == I3LocalnetLateIngressParentPublication::G2Published
                        && observation.publication_commit_count == 1
                }
                I3LocalnetLateIngressAckReaderOutcome::UntrustedRouteIgnored => {
                    observation.nonregistered_ack_input_disposition
                        == I3LocalnetLateIngressNonregisteredAckInputDisposition::RequesterStdoutTaintedCandidateIgnored
                        && observation.nonregistered_ack_input_count == 1
                        && observation.publication
                            == I3LocalnetLateIngressParentPublication::PublicationIncomplete
                        && observation.publication_commit_count == 0
                }
                I3LocalnetLateIngressAckReaderOutcome::LostBeforeParentAcceptance => {
                    observation.nonregistered_ack_input_disposition
                        == I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved
                        && observation.nonregistered_ack_input_count == 0
                        && observation.publication
                        == I3LocalnetLateIngressParentPublication::PublicationIncomplete
                        && observation.publication_commit_count == 0
                }
                I3LocalnetLateIngressAckReaderOutcome::NotSelected => false,
            }
        }
    }
}

fn joined_late_ingress_audit(
    profile: I3LocalnetLateIngressProfile,
    lineage: &SupervisorLineageEvidence,
    requester_event: &PrivateChildEvent,
    owner_event: &PrivateChildEvent,
    parent_observation: PrivateLateIngressParentObservation,
) -> Option<PrivateLateIngressJoin> {
    let PrivateChildEvent::LateIngress {
        slot: I3LocalnetChildSlot::ProcessA,
        terminal_outcome: requester_terminal,
        generated_request_count: requester_generated_count,
        receipt_count: requester_receipt_count,
        semantic_admission_count: requester_semantic_admission_count,
        late_ingress_evidence: requester_evidence,
        ..
    } = requester_event
    else {
        return None;
    };
    let PrivateChildEvent::LateIngress {
        slot: I3LocalnetChildSlot::ProcessB,
        terminal_outcome: owner_terminal,
        served_count: owner_served_count,
        write_count: owner_write_count,
        reply_count: owner_reply_count,
        semantic_admission_count: owner_semantic_admission_count,
        late_ingress_evidence: owner_evidence,
        ..
    } = owner_event
    else {
        return None;
    };
    if requester_evidence.run_ref.is_empty()
        || requester_evidence.run_ref != owner_evidence.run_ref
        || !late_ingress_parent_observation_valid(profile, parent_observation)
    {
        return None;
    }
    let requester_child = validated_late_ingress_child_audit(
        profile,
        I3LocalnetChildSlot::ProcessA,
        requester_event,
        requester_evidence,
    )?;
    let owner_child = validated_late_ingress_child_audit(
        profile,
        I3LocalnetChildSlot::ProcessB,
        owner_event,
        owner_evidence,
    )?;
    let initial_sender = requester_evidence.initial_sender_delivery.as_ref()?;
    let request_identity_ref = initial_sender.semantic_request_identity_ref.clone();
    if request_identity_ref.is_empty()
        || !retry_request_delivery_matches_contract(
            initial_sender,
            &lineage.request_contract,
            &request_identity_ref,
        )
        || initial_sender.candidate_commitment_ref.is_empty()
    {
        return None;
    }
    let retained = owner_evidence.retained_session_one_ingress.as_ref()?;
    if retained.session_generation != requester_evidence.first_session_generation
        || retained.session_generation != owner_evidence.first_session_generation
        || retained.network_occurrence_ref.is_empty()
    {
        return None;
    }
    if retained.candidate_commitment_ref.is_empty() {
        return Some(PrivateLateIngressJoin::EvidenceRejected(
            I3LocalnetLateIngressEvidenceRejection::RetainedIngressCommitmentMissing,
        ));
    }
    if retained.candidate_commitment_ref != initial_sender.candidate_commitment_ref {
        return Some(PrivateLateIngressJoin::EvidenceRejected(
            I3LocalnetLateIngressEvidenceRejection::RetainedIngressCommitmentMismatch,
        ));
    }
    if retained.network_occurrence_ref == initial_sender.network_occurrence_ref {
        return None;
    }
    if !matches!(
        owner_evidence.retained_ingress_repeat_acquisition_rejection,
        None | Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected)
    ) {
        return None;
    }
    let (late_admission_delivery, owner_outcome, requester_outcome, lifecycle_provenance) =
        match profile {
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
                if *requester_terminal != PrivateLateIngressTerminalOutcome::Completed
                    || *owner_terminal != PrivateLateIngressTerminalOutcome::Completed
                    || *requester_generated_count != 1
                    || *requester_receipt_count != 1
                    || *requester_semantic_admission_count != 1
                    || *owner_semantic_admission_count != 1
                    || *owner_served_count != 1
                    || *owner_write_count != 1
                    || *owner_reply_count != 1
                    || requester_evidence.requester_outcome
                        != Some(I3LocalnetLateIngressRequesterOutcome::ReceiptConsumed)
                    || owner_evidence.lifecycle_provenance
                        != Some(I3LocalnetLateIngressLifecycleProvenance::NotSelected)
                    || owner_evidence.m9_lifecycle_source_derived.is_some()
                {
                    return None;
                }
                let delivery = owner_evidence.late_admission_delivery.as_ref()?;
                if !retry_request_delivery_matches_contract(
                    delivery,
                    &lineage.request_contract,
                    &request_identity_ref,
                ) || !delivery_semantics_match(initial_sender, delivery)
                    || delivery.candidate_commitment_ref != retained.candidate_commitment_ref
                    || delivery.network_occurrence_ref != retained.network_occurrence_ref
                {
                    return None;
                }
                let PrivateLateIngressOwnerOutcome::Admitted {
                    owner_serve_count,
                    owner_mutation_count,
                } = owner_evidence.owner_outcome.as_ref()?
                else {
                    return None;
                };
                if *owner_serve_count != 1 || *owner_mutation_count != 1 {
                    return None;
                }
                (
                    Some(delivery_record(I3LocalnetDeliveryPhase::RequestReceive, delivery)),
                    late_ingress_owner_outcome(owner_evidence.owner_outcome.as_ref()?),
                    I3LocalnetLateIngressRequesterOutcome::ReceiptConsumed,
                    I3LocalnetLateIngressLifecycleProvenance::NotSelected,
                )
            }
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
                if *requester_terminal != PrivateLateIngressTerminalOutcome::HandledDeliveryFault
                    || *owner_terminal != PrivateLateIngressTerminalOutcome::HandledDeliveryFault
                    || *requester_generated_count != 1
                    || *requester_receipt_count != 0
                    || *requester_semantic_admission_count != 0
                    || *owner_semantic_admission_count != 0
                    || *owner_served_count != 0
                    || *owner_write_count != 0
                    || *owner_reply_count != 0
                    || requester_evidence.requester_outcome
                        != Some(I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved)
                    || owner_evidence.lifecycle_provenance
                        != Some(I3LocalnetLateIngressLifecycleProvenance::M9AdmittedLifecycle)
                    || owner_evidence.m9_lifecycle_source_derived != Some(false)
                    || owner_evidence.late_admission_delivery.is_some()
                {
                    return None;
                }
                let PrivateLateIngressOwnerOutcome::CarrierAdmissionRejected {
                    owner_serve_count,
                    owner_mutation_count,
                } = owner_evidence.owner_outcome.as_ref()?
                else {
                    return None;
                };
                if *owner_serve_count != 0 || *owner_mutation_count != 0 {
                    return None;
                }
                (
                    None,
                    late_ingress_owner_outcome(owner_evidence.owner_outcome.as_ref()?),
                    I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved,
                    I3LocalnetLateIngressLifecycleProvenance::M9AdmittedLifecycle,
                )
            }
        };
    let outbound_request_frame_write_count =
        requester_evidence.outbound_request_frame_write_count?;
    if outbound_request_frame_write_count != 1 {
        return None;
    }
    Some(PrivateLateIngressJoin::Accepted(Box::new(
        I3LocalnetLateIngressAudit {
            profile,
            request_identity_ref,
            requester_child,
            owner_child,
            initial_sender_delivery: delivery_record(
                I3LocalnetDeliveryPhase::RequestSend,
                initial_sender,
            ),
            retained_session_one_ingress: I3LocalnetRetainedIngressEvidence {
                session_generation: retained.session_generation,
                candidate_commitment_ref: retained.candidate_commitment_ref.clone(),
                network_occurrence_ref: retained.network_occurrence_ref.clone(),
            },
            retained_ingress_repeat_acquisition_rejection: owner_evidence
                .retained_ingress_repeat_acquisition_rejection,
            late_admission_delivery,
            admission_session_generation: owner_evidence.reconnect_session_generation,
            owner_outcome: Some(owner_outcome),
            requester_outcome,
            outbound_request_frame_write_count,
            lifecycle_provenance,
            m9_lifecycle_source_derived: owner_evidence.m9_lifecycle_source_derived,
            registered_owner_ack_reader_outcome: parent_observation.ack_reader_outcome,
            nonregistered_ack_input_disposition: parent_observation
                .nonregistered_ack_input_disposition,
            nonregistered_ack_input_count: parent_observation.nonregistered_ack_input_count,
            parent_publication: parent_observation.publication,
            parent_publication_commit_count: parent_observation.publication_commit_count,
        },
    )))
}

fn retry_child_sessions_valid(evidence: &PrivateChildRetryEvidence) -> bool {
    evidence.first_session_generation == 1
        && evidence.reconnect_session_generation == 2
        && evidence.first_session_peer_spki_verified
        && evidence.first_session_reciprocal_preface_verified
        && evidence.reconnect_session_peer_spki_verified
        && evidence.reconnect_session_reciprocal_preface_verified
}

fn retry_attempt_audit(
    evidence: &PrivateRetryAttemptEvidence,
) -> Option<I3LocalnetRetryAttemptAudit> {
    if evidence.semantic_request_identity_ref.is_empty()
        || evidence.source_requester_locus.is_empty()
        || evidence.requester_binding_ref.is_empty()
        || evidence.attempt_generation == 0
    {
        return None;
    }
    Some(I3LocalnetRetryAttemptAudit::from_runtime_summary(
        evidence.semantic_request_identity_ref.clone(),
        evidence.source_requester_locus.clone(),
        evidence.requester_binding_ref.clone(),
        evidence.attempt_generation,
        evidence.reason,
    ))
}

fn retry_attempt_matches(
    attempt: &I3LocalnetRetryAttemptAudit,
    request_identity_ref: &str,
    generation: u8,
    reason: I3LocalnetRetryAttemptReason,
) -> bool {
    attempt.semantic_request_identity_ref() == request_identity_ref
        && attempt.source_requester_locus() == "ParticipantA"
        && !attempt.requester_binding_ref().is_empty()
        && attempt.attempt_generation() == generation
        && attempt.reason() == reason
}

fn retry_child_audit(
    slot: I3LocalnetChildSlot,
    event: &PrivateChildEvent,
    evidence: &PrivateChildRetryEvidence,
) -> Option<I3LocalnetRetryChildAudit> {
    let terminal_outcome = event.terminal_event()?.outcome();
    Some(I3LocalnetRetryChildAudit::from_actual_child_event(
        slot,
        terminal_outcome,
        I3LocalnetRetryChildAuditEvidence {
            run_ref: evidence.run_ref.clone(),
            first_session_generation: evidence.first_session_generation,
            reconnect_session_generation: evidence.reconnect_session_generation,
            first_session_peer_spki_verified: evidence.first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified: evidence
                .first_session_reciprocal_preface_verified,
            reconnect_session_peer_spki_verified: evidence.reconnect_session_peer_spki_verified,
            reconnect_session_reciprocal_preface_verified: evidence
                .reconnect_session_reciprocal_preface_verified,
            first_session_runtime_attempt: evidence
                .first_session_runtime_attempt
                .as_ref()
                .and_then(retry_attempt_audit),
            reconnect_session_runtime_attempt: evidence
                .reconnect_session_runtime_attempt
                .as_ref()
                .and_then(retry_attempt_audit),
        },
    ))
}

fn retry_request_delivery_matches_contract(
    delivery: &PrivateDeliveryEvidence,
    contract: &Sys5I3AdapterCarrierContract,
    request_identity_ref: &str,
) -> bool {
    delivery_matches_contract(delivery, contract)
        && delivery.semantic_request_identity_ref == request_identity_ref
        && delivery.linked_request_identity_ref.is_none()
        && !delivery.carrier_ref.is_empty()
        && !delivery.network_occurrence_ref.is_empty()
}

fn retry_owner_admission(
    admission: &PrivateRemoteAdmissionEvidence,
    request_contract: &Sys5I3AdapterCarrierContract,
    request_identity_ref: &str,
) -> Option<I3LocalnetRemoteAdmissionEvidence> {
    let PrivateRemoteAdmissionEvidence::Admitted {
        request_receive,
        owner_serve_count,
        owner_mutation_count,
        owner_serve_occurrence_ref,
        owner_write_occurrence_ref,
    } = admission
    else {
        return None;
    };
    if *owner_serve_count != 1
        || *owner_mutation_count != 1
        || owner_serve_occurrence_ref.is_empty()
        || owner_write_occurrence_ref
            .as_deref()
            .is_none_or(str::is_empty)
        || !retry_request_delivery_matches_contract(
            request_receive,
            request_contract,
            request_identity_ref,
        )
    {
        return None;
    }
    Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
        request_receive: Box::new(delivery_record(
            I3LocalnetDeliveryPhase::RequestReceive,
            request_receive,
        )),
        owner_serve_count: *owner_serve_count,
        owner_mutation_count: *owner_mutation_count,
        owner_serve_occurrence_ref: owner_serve_occurrence_ref.clone(),
        owner_write_occurrence_ref: owner_write_occurrence_ref.clone(),
    })
}

fn same_remote_admission(
    left: &I3LocalnetRemoteAdmissionEvidence,
    right: &I3LocalnetRemoteAdmissionEvidence,
) -> bool {
    match (left, right) {
        (
            I3LocalnetRemoteAdmissionEvidence::Admitted {
                request_receive: left_request,
                owner_serve_count: left_serve,
                owner_mutation_count: left_write,
                owner_serve_occurrence_ref: left_serve_ref,
                owner_write_occurrence_ref: left_write_ref,
            },
            I3LocalnetRemoteAdmissionEvidence::Admitted {
                request_receive: right_request,
                owner_serve_count: right_serve,
                owner_mutation_count: right_write,
                owner_serve_occurrence_ref: right_serve_ref,
                owner_write_occurrence_ref: right_write_ref,
            },
        ) => {
            left_request == right_request
                && left_serve == right_serve
                && left_write == right_write
                && left_serve_ref == right_serve_ref
                && left_write_ref == right_write_ref
        }
        _ => false,
    }
}

fn delivery_semantics_match(
    sent: &PrivateDeliveryEvidence,
    received: &PrivateDeliveryEvidence,
) -> bool {
    sent.source_ref == received.source_ref
        && sent.core_ref == received.core_ref
        && sent.source_artifact_ref == received.source_artifact_ref
        && sent.target_artifact_ref == received.target_artifact_ref
        && sent.edge_ref == received.edge_ref
        && sent.carrier_ref == received.carrier_ref
        && sent.semantic_request_identity_ref == received.semantic_request_identity_ref
        && sent.linked_request_identity_ref == received.linked_request_identity_ref
        && !sent.network_occurrence_ref.is_empty()
        && !received.network_occurrence_ref.is_empty()
}

fn delivery_matches_contract(
    delivery: &PrivateDeliveryEvidence,
    contract: &Sys5I3AdapterCarrierContract,
) -> bool {
    delivery.source_ref == contract.source_ref()
        && delivery.core_ref == contract.core_ref()
        && delivery.source_artifact_ref == contract.source_artifact_ref()
        && delivery.target_artifact_ref == contract.target_artifact_ref()
        && delivery.edge_ref == contract.edge_ref()
}

fn owner_reply_replay_reply_matches_contract(
    delivery: &PrivateDeliveryEvidence,
    contract: &Sys5I3AdapterCarrierContract,
    request_identity_ref: &str,
) -> bool {
    delivery_matches_contract(delivery, contract)
        && delivery.semantic_request_identity_ref == request_identity_ref
        && delivery.linked_request_identity_ref.as_deref() == Some(request_identity_ref)
        && !delivery.carrier_ref.is_empty()
        && !delivery.network_occurrence_ref.is_empty()
        && !delivery.candidate_commitment_ref.is_empty()
}

fn owner_reply_replay_child_audit(
    slot: I3LocalnetChildSlot,
    event: &PrivateChildEvent,
    evidence: &PrivateChildOwnerReplyReplayEvidence,
) -> Option<I3LocalnetOwnerReplyReplayChildAudit> {
    let PrivateChildEvent::HandledDeliveryFault {
        slot: event_slot,
        semantic_admission_count: 1,
        owner_reply_replay_evidence: Some(_),
        ..
    } = event
    else {
        return None;
    };
    let terminal = event.terminal_event()?;
    if *event_slot != slot
        || terminal.slot() != Some(slot)
        || terminal.outcome() != I3LocalnetChildTerminalOutcome::HandledDeliveryFault
        || evidence.run_ref.is_empty()
        || evidence.first_session_generation != 1
        || !evidence.first_session_peer_spki_verified
        || !evidence.first_session_reciprocal_preface_verified
    {
        return None;
    }
    Some(
        I3LocalnetOwnerReplyReplayChildAudit::from_actual_observation(
            slot,
            evidence.run_ref.clone(),
            evidence.first_session_generation,
            evidence.first_session_peer_spki_verified,
            evidence.first_session_reciprocal_preface_verified,
            evidence.replay_session_generation,
            evidence.replay_session_peer_spki_verified,
            evidence.replay_session_reciprocal_preface_verified,
            terminal.outcome(),
        ),
    )
}

fn owner_reply_replay_terminal_matches(
    event: &PrivateChildEvent,
    slot: I3LocalnetChildSlot,
    request_identity_ref: &str,
    owner_mutation_count: usize,
) -> bool {
    matches!(
        event,
        PrivateChildEvent::HandledDeliveryFault {
            slot: event_slot,
            request_identity_ref: Some(event_identity_ref),
            requester_observation: None,
            requester_pending_request_is_retained: None,
            requester_local_wait: None,
            remote_admission: None,
            semantic_admission_count: 1,
            owner_mutation_count: event_owner_mutation_count,
            retry_evidence: None,
            owner_reply_replay_evidence: Some(_),
        } if *event_slot == slot
            && event_identity_ref == request_identity_ref
            && *event_owner_mutation_count == owner_mutation_count
    )
}

fn owner_reply_replay_successor_session_valid(
    evidence: &PrivateChildOwnerReplyReplayEvidence,
) -> bool {
    evidence.replay_session_generation == Some(2)
        && evidence.replay_session_peer_spki_verified == Some(true)
        && evidence.replay_session_reciprocal_preface_verified == Some(true)
}

fn owner_reply_replay_no_successor_session(
    evidence: &PrivateChildOwnerReplyReplayEvidence,
) -> bool {
    evidence.replay_session_generation.is_none()
        && evidence.replay_session_peer_spki_verified.is_none()
        && evidence
            .replay_session_reciprocal_preface_verified
            .is_none()
}

fn owner_reply_replay_public_requester_state(
    state: &PrivateOwnerReplyReplayRequesterState,
) -> I3LocalnetOwnerReplyReplayRequesterState {
    I3LocalnetOwnerReplyReplayRequesterState::from_actual_observation(
        state.pending_request_count,
        state.receipt_count,
        state.terminal_failure_count,
        state.receipt_occurrence_ref.clone(),
        state.terminal_failure_occurrence_ref.clone(),
    )
}

fn owner_reply_replay_public_owner_state(
    state: &PrivateOwnerReplyReplayOwnerState,
) -> I3LocalnetOwnerReplyReplayOwnerState {
    I3LocalnetOwnerReplyReplayOwnerState::from_actual_observation(
        state.tombstone_count,
        state.served_owner_request_count,
        state.owner_mutation_count,
        state.expired_owner_admission_count,
    )
}

fn exact_observer_chain(references: &I3LocalnetObserverSafeReferences) -> bool {
    !references.request_source_ref.is_empty()
        && !references.request_core_ref.is_empty()
        && !references.request_source_artifact_ref.is_empty()
        && !references.request_target_artifact_ref.is_empty()
        && !references.request_edge_ref.is_empty()
        && !references.reply_source_ref.is_empty()
        && !references.reply_core_ref.is_empty()
        && !references.reply_source_artifact_ref.is_empty()
        && !references.reply_target_artifact_ref.is_empty()
        && !references.reply_edge_ref.is_empty()
        && !references.request_carrier_ref.is_empty()
        && !references.reply_carrier_ref.is_empty()
        && references.request_carrier_ref != references.reply_carrier_ref
        && !references.semantic_request_identity_ref.is_empty()
        && references.network_request_identity_ref == references.semantic_request_identity_ref
        && references.network_reply_linked_request_identity_ref
            == references.semantic_request_identity_ref
        && references.runtime_serve_request_identity_ref == references.semantic_request_identity_ref
        && references.runtime_write_request_identity_ref == references.semantic_request_identity_ref
        && references.runtime_receipt_linked_request_identity_ref
            == references.semantic_request_identity_ref
        && !references.network_request_occurrence_ref.is_empty()
        && !references.network_reply_occurrence_ref.is_empty()
        && references.network_request_occurrence_ref != references.network_reply_occurrence_ref
        && !references.runtime_serve_occurrence_ref.is_empty()
        && !references.runtime_write_occurrence_ref.is_empty()
        && !references.runtime_receipt_occurrence_ref.is_empty()
        && !references.requester_local_store_ref.is_empty()
        && !references.owner_local_store_ref.is_empty()
}

fn requester_observer_state_ref(
    runtime: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
) -> String {
    let summary = runtime.observer_safe_runtime_summary();
    let occurrences = runtime.observer_safe_semantic_occurrences();
    let pending = runtime
        .observer_safe_pending_owner_request_count()
        .to_string();
    let receipts = summary.accepted_inbound_receipt_count().to_string();
    let receipt_occurrences = occurrences.requester_local_receipt_count().to_string();
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/private-localnet/requester-observer-state/v1\0");
    for component in [
        runtime.local_store_identity_ref(),
        &pending,
        &receipts,
        &receipt_occurrences,
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component.as_bytes());
    }
    format!(
        "i3-private-localnet-requester-state-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn is_loopback_endpoint(endpoint: &str) -> bool {
    endpoint
        .parse::<SocketAddr>()
        .ok()
        .is_some_and(|address| address.ip() == IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn matches_slot_loci(actual: &[String], slot: I3LocalnetChildSlot) -> bool {
    actual.iter().cloned().collect::<BTreeSet<_>>()
        == slot
            .assigned_loci()
            .into_iter()
            .map(str::to_owned)
            .collect()
}

/// Decode only an M9-issued child's strict, reference-only terminal view
/// after checking the event belongs to the expected physical child. The
/// decoded value remains a non-authorizing candidate: the sibling view and
/// full actual process evidence must still agree before the runner returns.
fn provider_completion_observer_view(
    event: PrivateChildEvent,
    slot: I3LocalnetChildSlot,
) -> Result<Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate, ()> {
    let PrivateChildEvent::ProviderCompleted {
        slot: event_slot,
        assigned_loci,
        provider_control_consumed: true,
        tainted_image_consumed: true,
        requester_fixture_assertion_completed,
        tls_peer_verified: true,
        reciprocal_preface_verified: true,
        reliable_bidi_stream_count: 1,
        quic_datagrams_enabled: false,
        terminal_observer_view_frame,
    } = event
    else {
        return Err(());
    };
    if event_slot != slot
        || !matches_slot_loci(&assigned_loci, slot)
        || requester_fixture_assertion_completed != (slot == I3LocalnetChildSlot::ProcessA)
    {
        return Err(());
    }
    let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
        .decode_untrusted_provider_terminal_audit_observer_view(&terminal_observer_view_frame)
        .map_err(|_| ())?;
    if candidate.fixed_profile_ref() != FIXED_PROVIDER_TERMINAL_AUDIT_PROFILE_V2
        || candidate.fixed_schema_ref() != FIXED_PROVIDER_NETWORK_PROVENANCE_SCHEMA_V2
        || candidate.role() != slot.provider_role()
    {
        return Err(());
    }
    Ok(candidate)
}

/// Correlate a sealed observer-conformance completion with its actual child
/// slot without decoding or receiving any observer audit. This does not prove
/// a profile's individual verdict; that assertion stays inside the installed
/// runtime's opaque completion path.
fn provider_terminal_observation_conformance_completed(
    event: PrivateChildEvent,
    slot: I3LocalnetChildSlot,
) -> Result<(), ()> {
    let PrivateChildEvent::ProviderTerminalObservationConformanceCompleted { slot: event_slot } =
        event
    else {
        return Err(());
    };
    if event_slot != slot {
        return Err(());
    }
    Ok(())
}

/// Correlate a sealed provider-network conformance completion with its
/// physical child slot. The child-local runtime has already checked the
/// selected two-session schedule; this marker exports neither the selected
/// schedule nor a transport, semantic, or audit fact.
fn provider_network_conformance_completed(
    event: PrivateChildEvent,
    slot: I3LocalnetChildSlot,
) -> Result<(), ()> {
    let PrivateChildEvent::ProviderNetworkConformanceCompleted { slot: event_slot } = event else {
        return Err(());
    };
    if event_slot != slot {
        return Err(());
    }
    Ok(())
}

/// The fixed source-real path has exactly one request. Its M9-gated views
/// retain actual first-slot send reservation/completion and complete-receive
/// facts on their local endpoint. The join proves each local chain against the
/// same admitted descriptors and semantic request; it deliberately never
/// equates independently allocated endpoint-local occurrence references.
/// Exact fixture outcome checking remains inside A's private post-consume
/// assertion. This compares only decoded reference-only candidates, never a
/// raw value, path, grant, or witness.
fn provider_terminal_observer_views_match(
    requester: &Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
    executor: &Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
) -> bool {
    if requester.role() != Sys5I3ProviderChildRole::RequesterConsumer
        || executor.role() != Sys5I3ProviderChildRole::Executor
        || requester.fixed_profile_ref() != FIXED_PROVIDER_TERMINAL_AUDIT_PROFILE_V2
        || executor.fixed_profile_ref() != FIXED_PROVIDER_TERMINAL_AUDIT_PROFILE_V2
        || requester.fixed_schema_ref() != FIXED_PROVIDER_NETWORK_PROVENANCE_SCHEMA_V2
        || executor.fixed_schema_ref() != FIXED_PROVIDER_NETWORK_PROVENANCE_SCHEMA_V2
        || requester.request_descriptor() != executor.request_descriptor()
        || requester.result_descriptor() != executor.result_descriptor()
        || requester.request_descriptor().generated_edge_ref()
            == requester.result_descriptor().generated_edge_ref()
    {
        return false;
    }
    let [requester_row] = requester.rows() else {
        return false;
    };
    let [executor_row] = executor.rows() else {
        return false;
    };
    let (
        Some(requester_request_send_reservation),
        Some(requester_request_send_completed),
        Some(requester_result_receive),
        Some(requester_consume),
    ) = (
        first_provider_transport_slot(requester_row.request_send_reservation_occurrence_refs()),
        first_provider_transport_slot(requester_row.request_send_completed_occurrence_refs()),
        first_provider_transport_slot(requester_row.result_receive_occurrence_refs()),
        requester_row.local_consume_ref(),
    )
    else {
        return false;
    };
    let (
        Some(executor_request_receive),
        Some(executor_host_started),
        Some(executor_adapter_entry),
        Some(executor_outcome_retained),
        Some(executor_release),
        Some(executor_result_send_reservation),
        Some(executor_result_send_completed),
    ) = (
        first_provider_transport_slot(executor_row.request_receive_occurrence_refs()),
        executor_row.host_started_occurrence_ref(),
        executor_row.adapter_entry_occurrence_ref(),
        executor_row.outcome_retained_occurrence_ref(),
        executor_row.release_occurrence_ref(),
        first_provider_transport_slot(executor_row.result_send_reservation_occurrence_refs()),
        first_provider_transport_slot(executor_row.result_send_completed_occurrence_refs()),
    )
    else {
        return false;
    };
    let Some(outcome_class) = requester_row.outcome_class() else {
        return false;
    };
    let expected_executor_read_count = match outcome_class {
        Sys5I3ProviderTerminalOutcomeClass::ValuePresent
        | Sys5I3ProviderTerminalOutcomeClass::ProviderInvalidResult
            if executor_row.adapter_read_occurrence_ref().is_some() =>
        {
            1
        }
        // Resource and adapter failures can arise at lookup/open/read. Keep
        // the actual bounded-read occurrence, if any, rather than treating a
        // typed failure as proof that no provider read occurred. Individual
        // source-real fixtures still assert their selected count exactly.
        Sys5I3ProviderTerminalOutcomeClass::ProviderResourceNotFound
        | Sys5I3ProviderTerminalOutcomeClass::ProviderPolicyDenied
        | Sys5I3ProviderTerminalOutcomeClass::AdapterUnavailable => {
            if executor_row.adapter_read_occurrence_ref().is_some() {
                1
            } else {
                0
            }
        }
        Sys5I3ProviderTerminalOutcomeClass::ValuePresent
        | Sys5I3ProviderTerminalOutcomeClass::ProviderInvalidResult => return false,
    };
    if requester_row.semantic_request_ref() != executor_row.semantic_request_ref()
        || requester_row.provider_invocation_ref().is_some()
        || requester_row.host_started_occurrence_ref().is_some()
        || requester_row.adapter_entry_occurrence_ref().is_some()
        || requester_row.adapter_read_occurrence_ref().is_some()
        || requester_row.outcome_retained_occurrence_ref().is_some()
        || requester_row.release_occurrence_ref().is_some()
        || executor_row.local_consume_ref().is_some()
        || executor_row.provider_invocation_ref() != Some(executor_host_started)
        || executor_row.outcome_class() != Some(outcome_class)
        || requester_row
            .ordered_predecessor_refs()
            .iter()
            .map(String::as_str)
            .ne([
                requester_row.semantic_request_ref(),
                requester_request_send_reservation,
                requester_request_send_completed,
                requester_result_receive,
                requester_consume,
            ])
        || !provider_executor_predecessors_match(
            executor_row,
            PrivateProviderExecutorPredecessors {
                request_receive: executor_request_receive,
                host_started: executor_host_started,
                adapter_entry: executor_adapter_entry,
                expected_read_count: expected_executor_read_count,
                outcome_retained: executor_outcome_retained,
                release: executor_release,
                result_send_reservation: executor_result_send_reservation,
                result_send_completed: executor_result_send_completed,
            },
        )
    {
        return false;
    }
    let requester_counts = requester.counts();
    let executor_counts = executor.counts();
    requester_counts.request_count() == 1
        && requester_counts.reserved_count() == 0
        && requester_counts.rejected_before_call_count() == 0
        && requester_counts.call_started_count() == 0
        && requester_counts.physical_adapter_entry_count() == 0
        && requester_counts.actual_read_count() == 0
        && requester_counts.outcome_retained_count() == 0
        && requester_counts.released_count() == 0
        && requester_counts.consume_count() == 1
        && requester_counts.pending_count() == 0
        && executor_counts.request_count() == 1
        && executor_counts.reserved_count() == 1
        && executor_counts.rejected_before_call_count() == 0
        && executor_counts.call_started_count() == 1
        && executor_counts.physical_adapter_entry_count() == 1
        && executor_counts.actual_read_count() == expected_executor_read_count
        && executor_counts.outcome_retained_count() == 1
        && executor_counts.released_count() == 1
        && executor_counts.consume_count() == 0
        && executor_counts.pending_count() == 0
}

fn first_provider_transport_slot(slots: &[Option<String>; 2]) -> Option<&str> {
    match (slots[0].as_deref(), slots[1].as_deref()) {
        (Some(reference), None) if !reference.is_empty() => Some(reference),
        _ => None,
    }
}

struct PrivateProviderExecutorPredecessors<'reference> {
    request_receive: &'reference str,
    host_started: &'reference str,
    adapter_entry: &'reference str,
    expected_read_count: usize,
    outcome_retained: &'reference str,
    release: &'reference str,
    result_send_reservation: &'reference str,
    result_send_completed: &'reference str,
}

fn provider_executor_predecessors_match(
    row: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProviderTerminalAuditRow,
    expected_predecessors: PrivateProviderExecutorPredecessors<'_>,
) -> bool {
    let PrivateProviderExecutorPredecessors {
        request_receive,
        host_started,
        adapter_entry,
        expected_read_count,
        outcome_retained,
        release,
        result_send_reservation,
        result_send_completed,
    } = expected_predecessors;
    let mut expected = vec![
        row.semantic_request_ref(),
        request_receive,
        host_started,
        adapter_entry,
    ];
    match (expected_read_count, row.adapter_read_occurrence_ref()) {
        (1, Some(adapter_read)) => expected.push(adapter_read),
        (0, None) => {}
        _ => return false,
    }
    expected.extend([
        outcome_retained,
        release,
        result_send_reservation,
        result_send_completed,
    ]);
    row.ordered_predecessor_refs()
        .iter()
        .map(String::as_str)
        .eq(expected)
}

fn verify_actual_ready_endpoint_rebind(endpoint: &str) -> bool {
    endpoint
        .parse::<SocketAddr>()
        .ok()
        .is_some_and(|address| UdpSocket::bind(address).is_ok())
}

fn fresh_run_ref(cohort_ref: &str, first_spki_ref: &str, second_spki_ref: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/process-localnet/run/v1\0");
    for component in [cohort_ref, first_spki_ref, second_spki_ref] {
        hasher.update(
            u64::try_from(component.len())
                .expect("usize fits u64")
                .to_be_bytes(),
        );
        hasher.update(component.as_bytes());
    }
    format!("i3-process-localnet-run-sha256-v1:{:x}", hasher.finalize())
}

/// Reference-only identity for one provider QUIC transport attempt. The
/// actual source/Core/artifact lineage remains sealed in the opaque runtime
/// launch and is not an input to this physical TLS binding.
fn fresh_provider_transport_run_ref(first_spki_ref: &str, second_spki_ref: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/provider-localnet/run/v1\0");
    for component in [first_spki_ref, second_spki_ref] {
        hasher.update(
            u64::try_from(component.len())
                .expect("usize fits u64")
                .to_be_bytes(),
        );
        hasher.update(component.as_bytes());
    }
    format!("i3-provider-localnet-run-sha256-v1:{:x}", hasher.finalize())
}

fn spki_ref(spki: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/process-localnet/spki/v1\0");
    hasher.update(spki);
    format!("i3-process-localnet-spki-sha256-v1:{:x}", hasher.finalize())
}

fn generate_run_credentials() -> Result<RunCredentials, ()> {
    let mut params = CertificateParams::new(Vec::<String>::new()).map_err(|_| ())?;
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyCertSign,
    ];
    let ca_key = KeyPair::generate().map_err(|_| ())?;
    let ca_cert = params.self_signed(&ca_key).map_err(|_| ())?;
    let issuer = Issuer::new(params, ca_key);
    let ca_der = ca_cert.der().to_vec();
    let process_a = issue_leaf(&issuer)?;
    let process_b = issue_leaf(&issuer)?;
    let wrong_peer = issue_leaf(&issuer)?;
    Ok(RunCredentials {
        ca_der,
        process_a,
        process_b,
        wrong_peer,
    })
}

fn issue_leaf(issuer: &Issuer<'_, KeyPair>) -> Result<LeafMaterial, ()> {
    let mut params = CertificateParams::new(vec!["localhost".to_string()]).map_err(|_| ())?;
    params.extended_key_usages = vec![
        ExtendedKeyUsagePurpose::ServerAuth,
        ExtendedKeyUsagePurpose::ClientAuth,
    ];
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    let key = KeyPair::generate().map_err(|_| ())?;
    let certificate = params.signed_by(&key, issuer).map_err(|_| ())?;
    Ok(LeafMaterial {
        certificate_der: certificate.der().to_vec(),
        private_key_der: Zeroizing::new(key.serialize_der()),
        spki_ref: spki_ref(&key.subject_public_key_info()),
    })
}

fn probe_binary_path() -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    let debug_dir = current.parent()?.parent()?;
    let binary = debug_dir.join("mirrorea-i3-probe");
    binary.is_file().then_some(binary)
}

/// Binary-only dispatcher. It returns `None` when the argv belongs to the
/// older candidate harness, preserving its isolated private protocol.
pub(crate) fn run_private_localnet_child_from_args(args: Vec<String>) -> Option<bool> {
    match args.as_slice() {
        [value] if value == I3LocalnetChildSlot::ProcessA.child_arg() => {
            Some(run_private_localnet_child(I3LocalnetChildSlot::ProcessA).is_ok())
        }
        [value] if value == I3LocalnetChildSlot::ProcessB.child_arg() => {
            Some(run_private_localnet_child(I3LocalnetChildSlot::ProcessB).is_ok())
        }
        [value] if value == I3LocalnetChildSlot::ProcessA.provider_child_arg() => {
            Some(run_private_provider_localnet_child(I3LocalnetChildSlot::ProcessA).is_ok())
        }
        [value] if value == I3LocalnetChildSlot::ProcessB.provider_child_arg() => {
            Some(run_private_provider_localnet_child(I3LocalnetChildSlot::ProcessB).is_ok())
        }
        _ => None,
    }
}

fn run_private_localnet_child(fixed_slot: I3LocalnetChildSlot) -> Result<(), ()> {
    let result = run_private_localnet_child_inner(fixed_slot);
    if result.is_err() {
        let _ = emit_child_event(&PrivateChildEvent::UnknownLifecycleFailure { slot: fixed_slot });
    }
    result
}

/// Dedicated provider process branch. It deliberately bypasses the ordinary
/// `read_trusted_control`/decoded-control startup path: the runtime is the
/// only FD3 reader and will install only from its opaque inherited result.
fn run_private_provider_localnet_child(fixed_slot: I3LocalnetChildSlot) -> Result<(), ()> {
    let result = run_private_provider_localnet_child_inner(fixed_slot);
    if result.is_err() {
        let _ = emit_child_event(&PrivateChildEvent::UnknownLifecycleFailure { slot: fixed_slot });
    }
    result
}

fn run_private_provider_localnet_child_inner(fixed_slot: I3LocalnetChildSlot) -> Result<(), ()> {
    let image = read_tainted_image().map_err(|_| ())?;
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let image = codec.decode_untrusted_image(&image).map_err(|_| ())?;
    let manifest = image.observer_safe_manifest();
    let assigned_loci = manifest.assigned_loci();
    if !manifest.has_assigned_artifacts_only() || !matches_slot_loci(&assigned_loci, fixed_slot) {
        return Err(());
    }
    // This call solely owns FD3 on the provider branch. In particular, do
    // not call the ordinary `read_trusted_control`, decode its outer JSON, or
    // convert any generic decoded record into provider authority.
    let inherited = codec
        .take_inherited_provider_child_control_from_fd3()
        .map_err(|_| ())?;
    if inherited.role() != fixed_slot.provider_role() {
        return Err(());
    }
    let transport =
        decode_private_provider_child_transport_bootstrap(inherited.transport_bootstrap())?;
    if !provider_transport_bootstrap_matches_slot(&transport, fixed_slot) {
        return Err(());
    }
    let installed = codec
        .install_provider_from_inherited_control(image, inherited, fixed_slot.provider_role())
        .map_err(|_| ())?;
    let trusted = installed
        .bind_provider_localnet_transport(
            &transport.run_ref,
            &transport.local_spki_ref,
            &transport.peer_spki_ref,
        )
        .map_err(|_| ())?;
    let assigned_loci = fixed_slot
        .assigned_loci()
        .into_iter()
        .map(str::to_owned)
        .collect();
    let duration = Duration::from_millis(transport.timeout_millis);
    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| ())?;
    tokio_runtime.block_on(async move {
        match fixed_slot {
            I3LocalnetChildSlot::ProcessB => {
                run_provider_server_child(installed, trusted, transport, duration, assigned_loci)
                    .await
            }
            I3LocalnetChildSlot::ProcessA => {
                run_provider_client_child(installed, trusted, transport, duration, assigned_loci)
                    .await
            }
        }
    })
}

fn run_private_localnet_child_inner(fixed_slot: I3LocalnetChildSlot) -> Result<(), ()> {
    let image = read_tainted_image().map_err(|_| ())?;
    let raw_control = Zeroizing::new(read_trusted_control().map_err(|_| ())?);
    let control = decode_private_child_control(raw_control.as_ref())?;
    if control.stall_cleanup {
        thread::sleep(Duration::from_millis(control.timeout_millis.max(1)));
        return Err(());
    }
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let runtime_control = codec
        .decode_trusted_localnet_control(&control.trusted_runtime_control)
        .map_err(|_| ())?;
    // This fixed argv/FD association is checked before certificate creation,
    // endpoint bind, QUIC handshake, runtime bootstrap, or semantic state.
    if fixed_slot != control.slot || runtime_control.local_slot_name() != fixed_slot.slot_name() {
        emit_child_event(&PrivateChildEvent::rejected(
            PrivateChildRejection::StartBinding,
            true,
            0,
            0,
            0,
            0,
            0,
        ))
        .map_err(|_| ())?;
        return Ok(());
    }
    let image = codec.decode_untrusted_image(&image).map_err(|_| ())?;
    let manifest = image.observer_safe_manifest();
    let assigned_loci = manifest.assigned_loci();
    if !manifest.has_assigned_artifacts_only() || !matches_slot_loci(&assigned_loci, fixed_slot) {
        emit_child_event(&PrivateChildEvent::rejected(
            PrivateChildRejection::StartBinding,
            true,
            0,
            0,
            0,
            0,
            0,
        ))
        .map_err(|_| ())?;
        return Ok(());
    }
    // The image manifest is a set; render the accepted canonical deployment
    // order only after comparing that set to the checked slot allocation.
    let assigned_loci = fixed_slot
        .assigned_loci()
        .map(str::to_owned)
        .into_iter()
        .collect();
    let start_result = match control.late_ingress_profile {
            Some(
                I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
            ) => codec
                .validate_and_start_image_with_prestaged_lifecycle(image, runtime_control),
            None | Some(I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect) => {
                codec
                    .validate_and_start_image_with_localnet_control(image, runtime_control)
                    .map(|(runtime, runtime_control)| (runtime, runtime_control, None))
            }
    };
    let (runtime, runtime_control, admitted_lifecycle_stimulus) = match start_result {
        Ok(value) => value,
        Err(_) => {
            emit_child_event(&PrivateChildEvent::rejected(
                PrivateChildRejection::StartBinding,
                true,
                0,
                0,
                0,
                0,
                0,
            ))
            .map_err(|_| ())?;
            return Ok(());
        }
    };
    let duration = Duration::from_millis(control.timeout_millis.max(1));
    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| ())?;
    tokio_runtime.block_on(async move {
        match fixed_slot {
            I3LocalnetChildSlot::ProcessB => {
                run_server_child(
                    runtime,
                    runtime_control,
                    admitted_lifecycle_stimulus,
                    control,
                    duration,
                    assigned_loci,
                )
                .await
            }
            I3LocalnetChildSlot::ProcessA => {
                if admitted_lifecycle_stimulus.is_some() {
                    return Err(());
                }
                run_client_child(runtime, runtime_control, control, duration, assigned_loci).await
            }
        }
    })
}

fn read_tainted_image() -> io::Result<Vec<u8>> {
    let max = Sys5I3PrivateProcessCodec::private_provisional_v1()
        .limits()
        .max_image_bytes();
    let mut image = Vec::new();
    io::stdin()
        .take((max + 1).try_into().expect("limit fits"))
        .read_to_end(&mut image)?;
    if image.len() > max {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "image too large",
        ));
    }
    Ok(image)
}

fn read_trusted_control() -> io::Result<Vec<u8>> {
    // Check before constructing an owned descriptor. `from_raw_fd` has an
    // ownership precondition and must never be used to probe an absent FD.
    // SAFETY: `fcntl(F_GETFD)` only observes the fixed inherited descriptor.
    if unsafe { libc::fcntl(LOCALNET_CONTROL_FD, libc::F_GETFD) } == -1 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: FD 3 is created by the supervisor immediately before exec and
    // is consumed once here. No other runtime path opens or accepts this FD.
    let mut stream = unsafe { UnixStream::from_raw_fd(LOCALNET_CONTROL_FD) };
    let mut bytes = Vec::new();
    Read::by_ref(&mut stream)
        .take((MAX_TRUSTED_CONTROL_BYTES + 5) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() < 4 || bytes.len() > MAX_TRUSTED_CONTROL_BYTES + 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "control frame"));
    }
    let declared = u32::from_be_bytes(bytes[..4].try_into().expect("prefix")) as usize;
    if declared != bytes.len() - 4 || declared > MAX_TRUSTED_CONTROL_BYTES {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "control length"));
    }
    Ok(bytes.split_off(4))
}

fn take_registered_owner_lifecycle_ack_writer() -> io::Result<UnixStream> {
    // SAFETY: this fixed descriptor is installed only for ProcessB's selected
    // G2 launch. It is consumed once by the B-local receipt-gated emitter;
    // ProcessA and ordinary G1 launches never open it.
    if unsafe { libc::fcntl(LOCALNET_OWNER_LIFECYCLE_ACK_FD, libc::F_GETFD) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { UnixStream::from_raw_fd(LOCALNET_OWNER_LIFECYCLE_ACK_FD) })
}

fn write_registered_owner_lifecycle_ack(
    stream: &mut UnixStream,
    framed_ack: &[u8],
    replay_once: bool,
) -> io::Result<()> {
    if framed_ack.is_empty() || framed_ack.len() > MAX_TRUSTED_CONTROL_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "lifecycle ack frame",
        ));
    }
    stream.write_all(framed_ack)?;
    if replay_once {
        // The only replay control copies the exact receipt-produced frame on
        // the same registered B stream; it neither constructs fields nor
        // injects a detached caller-provided ACK.
        stream.write_all(framed_ack)?;
    }
    stream.flush()?;
    stream.shutdown(Shutdown::Write)
}

/// Resolve an actual generated-owner receive result. Only an observed
/// `Awaiting` admission may invoke the opaque host driver; the neutral probe
/// profile changes its one fixed clock input but never predicts the checked
/// gate outcome. The returned expiry evidence is copied from the runtime's
/// retained producer record, never reconstructed from counts or a carrier.
fn resolve_received_owner_reply(
    runtime: &mut mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    received: Option<mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessMessage>,
    request_identity_ref: &str,
    profile: Option<I3LocalnetOwnerAdmissionDriveProfile>,
) -> Result<PrivateResolvedOwnerReply, ()> {
    let reply = match received {
        Some(reply) => reply,
        None => {
            let owner_admission = runtime.observer_safe_owner_admission_summary();
            let summary = runtime.observer_safe_runtime_summary();
            if owner_admission.awaiting_count() != 1
                || owner_admission.expired_count() != 0
                || owner_admission.rejected_before_serve_count() != 0
                || owner_admission.serve_reserved_count() != 0
                || runtime.observer_safe_inbound_owner_request_tombstone_count() != 1
                || summary.served_owner_request_count() != 0
                || summary.actual_owner_write_count() != 0
            {
                return Err(());
            }
            let drivers = runtime.i3_admitted_owner_admission_host_drivers();
            let [driver] = drivers.as_slice() else {
                return Err(());
            };
            let next_tick = profile.map(|_| 1);
            runtime
                .drive_next_owner_admission(driver, next_tick)
                .map_err(|_| ())?
                .ok_or(())?
        }
    };
    let owner_admission = runtime.observer_safe_owner_admission_summary();
    let summary = runtime.observer_safe_runtime_summary();
    if let Some(decision) =
        runtime.observer_safe_owner_admission_expiry_decision(request_identity_ref)
    {
        let expiry = PrivateOwnerAdmissionExpiryEvidence {
            request_receive: None,
            decision_commitment_ref: decision.decision_commitment_ref().to_string(),
            decision_occurrence_ref: decision.decision_occurrence_ref().to_string(),
            generated_reply_count: 1,
            expired_count: owner_admission.expired_count(),
            owner_serve_count: summary.served_owner_request_count(),
            owner_mutation_count: summary.actual_owner_write_count(),
        };
        if owner_admission.awaiting_count() != 0
            || owner_admission.rejected_before_serve_count() != 0
            || owner_admission.serve_reserved_count() != 0
            || runtime.observer_safe_inbound_owner_request_tombstone_count() != 1
            || !expiry.is_exact_declared_expiry()
        {
            return Err(());
        }
        Ok(PrivateResolvedOwnerReply::DeclaredDeadlineExpired { reply, expiry })
    } else {
        // A source condition may still permit a successful serve after the
        // fixed neutral schedule. Keep the pre-existing strict success
        // invariants rather than treating the profile as an expected expiry.
        if owner_admission.awaiting_count() != 0
            || owner_admission.expired_count() != 0
            || owner_admission.rejected_before_serve_count() != 0
            || owner_admission.serve_reserved_count() != 0
            || runtime.observer_safe_inbound_owner_request_tombstone_count() != 1
            || summary.served_owner_request_count() != 1
            || summary.actual_owner_write_count() != 1
        {
            return Err(());
        }
        Ok(PrivateResolvedOwnerReply::Served(reply))
    }
}

/// B's only provider path: after opaque installation and reciprocal mTLS
/// preface validation, let the runtime-owned session decode/admit one request,
/// execute the bounded local effect, retain the result, and send that opaque
/// result. No owner-message path or probe-decoded carrier participates.
async fn run_provider_server_child(
    mut installed: Sys5I3InstalledProviderChildRuntime,
    trusted: Sys5I3TrustedProviderLocalnetControl,
    transport: PrivateProviderChildTransportBootstrap,
    timeout: Duration,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    if transport.endpoint.is_some() {
        return Err(());
    }
    install_ring()?;
    let (server_config, evidence) = server_config_from_transport_material(
        &transport.ca_der,
        &transport.leaf_cert_der,
        transport.leaf_key_der.as_ref(),
    )?;
    let endpoint =
        Endpoint::server(server_config, SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    let endpoint_address = endpoint.local_addr().map_err(|_| ())?.to_string();
    emit_child_event(&PrivateChildEvent::Ready {
        endpoint: endpoint_address,
    })
    .map_err(|_| ())?;
    if evidence.datagrams_enabled() {
        return Err(());
    }
    tokio::time::timeout(timeout, async {
        if installed.has_fixed_provider_network_conformance_profile() {
            return run_provider_network_server_sessions(&mut installed, trusted, &endpoint).await;
        }
        let connecting = endpoint.accept().await.ok_or(())?;
        let connection = connecting.await.map_err(|_| ())?;
        let mut session =
            mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::accept_provider(
                connection, trusted,
            )
            .await
            .map_err(|_| ())?;
        // Client sends first; the server validates it before releasing its
        // own preface, so no provider carrier is admitted first.
        session
            .receive_and_validate_peer_preface()
            .await
            .map_err(|_| ())?;
        session.send_local_preface().await.map_err(|_| ())?;
        let result = session
            .receive_provider_request_and_execute(&mut installed)
            .await
            .map_err(|_| ())?;
        session
            .send_provider_result(&mut installed, result)
            .await
            .map_err(|_| ())?;
        session.finish_send().map_err(|_| ())?;
        if !session.peer_spki_verified()
            || !session.peer_preface_verified()
            || session.reliable_bidi_stream_count() != 1
            || session.quic_datagrams_enabled()
        {
            return Err(());
        }
        match installed
            .complete_fixed_terminal_observation_conformance_if_selected()
            .map_err(|_| ())?
        {
            Some(_) => emit_child_event(
                &PrivateChildEvent::ProviderTerminalObservationConformanceCompleted {
                    slot: I3LocalnetChildSlot::ProcessB,
                },
            )
            .map_err(|_| ())?,
            None => {
                let audit = installed.provider_terminal_audit().map_err(|_| ())?;
                let terminal_observer_view_frame =
                    Sys5I3PrivateProcessCodec::private_provisional_v1()
                        .encode_provider_terminal_audit_observer_view(&audit)
                        .map_err(|_| ())?;
                emit_child_event(&PrivateChildEvent::ProviderCompleted {
                    slot: I3LocalnetChildSlot::ProcessB,
                    assigned_loci,
                    provider_control_consumed: true,
                    tainted_image_consumed: true,
                    requester_fixture_assertion_completed: false,
                    tls_peer_verified: true,
                    reciprocal_preface_verified: true,
                    reliable_bidi_stream_count: 1,
                    quic_datagrams_enabled: false,
                    terminal_observer_view_frame,
                })
                .map_err(|_| ())?;
            }
        }
        // A closes after it consumes the opaque result. Waiting here
        // prevents B from closing the connection before that checked
        // consume path observes the already-framed result.
        session.wait_for_peer_close().await;
        session.close();
        Ok::<_, ()>(())
    })
    .await
    .map_err(|_| ())?
}

/// A's matching provider path. It creates the request only from its opaque
/// installed runtime after reciprocal preface validation, then consumes the
/// opaque result locally. The raw result is not available to this event path.
async fn run_provider_client_child(
    mut installed: Sys5I3InstalledProviderChildRuntime,
    trusted: Sys5I3TrustedProviderLocalnetControl,
    transport: PrivateProviderChildTransportBootstrap,
    timeout: Duration,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    let endpoint_address = transport
        .endpoint
        .as_deref()
        .ok_or(())?
        .parse::<SocketAddr>()
        .map_err(|_| ())?;
    if endpoint_address.ip() != IpAddr::V4(Ipv4Addr::LOCALHOST) {
        return Err(());
    }
    install_ring()?;
    let mut endpoint = Endpoint::client(SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    let (client_config, evidence) = client_config_from_transport_material(
        &transport.ca_der,
        &transport.leaf_cert_der,
        transport.leaf_key_der.as_ref(),
    )?;
    endpoint.set_default_client_config(client_config);
    if evidence.datagrams_enabled() {
        return Err(());
    }
    tokio::time::timeout(timeout, async {
        if installed.has_fixed_provider_network_conformance_profile() {
            run_provider_network_client_sessions(
                &mut installed,
                trusted,
                &mut endpoint,
                endpoint_address,
            )
            .await?;
            if transport.requester_exit_nonzero_after_completed {
                std::process::exit(9);
            }
            return Ok::<_, ()>(());
        }
        let connecting = endpoint
            .connect(endpoint_address, "localhost")
            .map_err(|_| ())?;
        let connection = connecting.await.map_err(|_| ())?;
        let mut session =
            mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::connect_provider(
                connection, trusted,
            )
            .await
            .map_err(|_| ())?;
        session.send_local_preface().await.map_err(|_| ())?;
        session
            .receive_and_validate_peer_preface()
            .await
            .map_err(|_| ())?;
        let request = installed
            .begin_read_only_provider_request()
            .map_err(|_| ())?;
        session
            .send_provider_request(&mut installed, request)
            .await
            .map_err(|_| ())?;
        session.finish_send().map_err(|_| ())?;
        let receipt = session
            .receive_provider_result_and_consume(&mut installed)
            .await
            .map_err(|_| ())?;
        installed
            .complete_fixture_post_consume_assertion(&receipt)
            .map_err(|_| ())?;
        if !session.peer_spki_verified()
            || !session.peer_preface_verified()
            || session.reliable_bidi_stream_count() != 1
            || session.quic_datagrams_enabled()
        {
            return Err(());
        }
        let terminal_observation_conformance = installed
            .complete_fixed_terminal_observation_conformance_if_selected()
            .map_err(|_| ())?;
        session.close();
        // Remain online under the enclosing, unchanged child deadline long
        // enough for B to observe A's actual connection close after its
        // result has been consumed. `wait_idle` adds no independent timeout,
        // retry, or semantic acknowledgement.
        endpoint.wait_idle().await;
        match terminal_observation_conformance {
            Some(_) => emit_child_event(
                &PrivateChildEvent::ProviderTerminalObservationConformanceCompleted {
                    slot: I3LocalnetChildSlot::ProcessA,
                },
            )
            .map_err(|_| ())?,
            None => {
                let audit = installed.provider_terminal_audit().map_err(|_| ())?;
                let terminal_observer_view_frame =
                    Sys5I3PrivateProcessCodec::private_provisional_v1()
                        .encode_provider_terminal_audit_observer_view(&audit)
                        .map_err(|_| ())?;
                emit_child_event(&PrivateChildEvent::ProviderCompleted {
                    slot: I3LocalnetChildSlot::ProcessA,
                    assigned_loci,
                    provider_control_consumed: true,
                    tainted_image_consumed: true,
                    requester_fixture_assertion_completed: true,
                    tls_peer_verified: true,
                    reciprocal_preface_verified: true,
                    reliable_bidi_stream_count: 1,
                    quic_datagrams_enabled: false,
                    terminal_observer_view_frame,
                })
                .map_err(|_| ())?;
            }
        }
        if transport.requester_exit_nonzero_after_completed {
            std::process::exit(9);
        }
        Ok::<_, ()>(())
    })
    .await
    .map_err(|_| ())?
}

/// The sealed network-conformance path uses exactly the existing provider
/// session and its consuming reconnect control. The probe does not select a
/// schedule: every phase is delegated to the installed runtime, which checks
/// the inherited sealed profile before it changes a carrier or local ledger.
async fn run_provider_network_server_sessions(
    installed: &mut Sys5I3InstalledProviderChildRuntime,
    trusted: Sys5I3TrustedProviderLocalnetControl,
    endpoint: &Endpoint,
) -> Result<(), ()> {
    let connecting = endpoint.accept().await.ok_or(())?;
    let connection = connecting.await.map_err(|_| ())?;
    let mut first_session =
        mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::accept_provider(
            connection, trusted,
        )
        .await
        .map_err(|_| ())?;
    first_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    first_session.send_local_preface().await.map_err(|_| ())?;
    let result = first_session
        .receive_provider_request_and_execute(installed)
        .await
        .map_err(|_| ())?;
    first_session
        .drive_fixed_provider_network_first_result_send(installed, &result)
        .await
        .map_err(|_| ())?;
    first_session.finish_send().map_err(|_| ())?;
    if !provider_session_is_verified(&first_session, 1) {
        return Err(());
    }

    // A explicitly closes the first session only after its selected actual
    // receive/EOF phase. Keep B online until that close rather than relying
    // on `into_reconnect` dropping the old Quinn connection.
    first_session.wait_for_peer_close().await;
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut second_session = accept_provider_reconnect_session(endpoint, reconnect).await?;
    second_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    second_session.send_local_preface().await.map_err(|_| ())?;
    second_session
        .drive_fixed_provider_network_second_request_receive(installed)
        .await
        .map_err(|_| ())?;
    second_session
        .drive_fixed_provider_network_second_result_send(installed, &result)
        .await
        .map_err(|_| ())?;
    second_session.finish_send().map_err(|_| ())?;
    if !provider_session_is_verified(&second_session, 2) {
        return Err(());
    }
    if installed
        .complete_fixed_provider_network_conformance_if_selected()
        .map_err(|_| ())?
        .is_none()
    {
        return Err(());
    }
    emit_child_event(&PrivateChildEvent::ProviderNetworkConformanceCompleted {
        slot: I3LocalnetChildSlot::ProcessB,
    })
    .map_err(|_| ())?;
    // A closes after it reaches its own sealed completion. Waiting on B is
    // part of the unchanged enclosing child deadline and adds no retry.
    second_session.wait_for_peer_close().await;
    second_session.close();
    Ok(())
}

/// A's half of the same sealed two-session path. The runtime keeps each
/// profile's held/lost/replayed carrier and any typed rejection private; this
/// function only composes real Quinn connections in the fixed order.
async fn run_provider_network_client_sessions(
    installed: &mut Sys5I3InstalledProviderChildRuntime,
    trusted: Sys5I3TrustedProviderLocalnetControl,
    endpoint: &mut Endpoint,
    endpoint_address: SocketAddr,
) -> Result<(), ()> {
    let connection = endpoint
        .connect(endpoint_address, "localhost")
        .map_err(|_| ())?
        .await
        .map_err(|_| ())?;
    let mut first_session =
        mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::connect_provider(
            connection, trusted,
        )
        .await
        .map_err(|_| ())?;
    first_session.send_local_preface().await.map_err(|_| ())?;
    first_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    let request = installed
        .begin_read_only_provider_request()
        .map_err(|_| ())?;
    first_session
        .send_provider_request(installed, request)
        .await
        .map_err(|_| ())?;
    first_session.finish_send().map_err(|_| ())?;
    first_session
        .drive_fixed_provider_network_first_result_receive(installed)
        .await
        .map_err(|_| ())?;
    if !provider_session_is_verified(&first_session, 1) {
        return Err(());
    }

    // The first result phase has completed its sealed consume, discard, or
    // clean-FIN boundary. Closing here releases B's same-deadline peer-close
    // wait before either side consumes reconnect.
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut second_session =
        connect_provider_reconnect_session(endpoint, endpoint_address, reconnect).await?;
    second_session.send_local_preface().await.map_err(|_| ())?;
    second_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    second_session
        .drive_fixed_provider_network_second_request_send(installed)
        .await
        .map_err(|_| ())?;
    second_session.finish_send().map_err(|_| ())?;
    second_session
        .drive_fixed_provider_network_second_result_receive(installed)
        .await
        .map_err(|_| ())?;
    if !provider_session_is_verified(&second_session, 2) {
        return Err(());
    }
    if installed
        .complete_fixed_provider_network_conformance_if_selected()
        .map_err(|_| ())?
        .is_none()
    {
        return Err(());
    }
    second_session.close();
    endpoint.wait_idle().await;
    emit_child_event(&PrivateChildEvent::ProviderNetworkConformanceCompleted {
        slot: I3LocalnetChildSlot::ProcessA,
    })
    .map_err(|_| ())?;
    Ok(())
}

fn provider_session_is_verified(
    session: &mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    expected_generation: u8,
) -> bool {
    session.peer_spki_verified()
        && session.peer_preface_verified()
        && session.reliable_bidi_stream_count() == 1
        && !session.quic_datagrams_enabled()
        && session.session_attempt_generation() == expected_generation
}

async fn accept_provider_reconnect_session(
    endpoint: &Endpoint,
    reconnect: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicReconnect,
) -> Result<mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession, ()> {
    let connecting = endpoint.accept().await.ok_or(())?;
    let connection = connecting.await.map_err(|_| ())?;
    reconnect.accept_provider(connection).await.map_err(|_| ())
}

async fn connect_provider_reconnect_session(
    endpoint: &Endpoint,
    endpoint_address: SocketAddr,
    reconnect: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicReconnect,
) -> Result<mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession, ()> {
    let connection = endpoint
        .connect(endpoint_address, "localhost")
        .map_err(|_| ())?
        .await
        .map_err(|_| ())?;
    reconnect.connect_provider(connection).await.map_err(|_| ())
}

async fn run_server_child(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    trusted: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    admitted_lifecycle_stimulus: Option<
        mir_runtime::sys5_i3_process_runtime::Sys5I3AdmittedLifecycleStimulus,
    >,
    control: PrivateChildControl,
    timeout: Duration,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    if control.endpoint.is_some() {
        return Err(());
    }
    install_ring()?;
    let local_preface = trusted.localnet_preface();
    let run_ref = local_preface.run_ref().to_string();
    let cohort_provenance_ref = local_preface.cohort_provenance_ref().to_string();
    let (server_config, _transport_evidence) = server_config(&control)?;
    let endpoint =
        Endpoint::server(server_config, SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    let endpoint_address = endpoint.local_addr().map_err(|_| ())?.to_string();
    if control.adapter_delivery_profile
        == Some(I3LocalnetAdapterDeliveryProfile::EndpointClosedBeforeConnect)
    {
        // Close the actually bound endpoint before advertising its former
        // address. The supervisor therefore cannot race A's connect against
        // this B listener becoming available, and no new child protocol is
        // needed to establish the ordering.
        endpoint.close(0_u32.into(), b"endpoint unavailable");
        emit_child_event(&PrivateChildEvent::Ready {
            endpoint: endpoint_address,
        })
        .map_err(|_| ())?;
        let summary = runtime.observer_safe_runtime_summary();
        if summary.served_owner_request_count() != 0 || summary.actual_owner_write_count() != 0 {
            return Err(());
        }
        emit_child_event(&PrivateChildEvent::AdapterDeliveryFault {
            slot: I3LocalnetChildSlot::ProcessB,
            exec_confirmed: true,
            assigned_loci,
            trusted_control_consumed: true,
            tainted_image_consumed: true,
            tls_peer_verified: false,
            reciprocal_preface_verified: false,
            reliable_bidi_stream_count: 0,
            quic_datagrams_enabled: false,
            request_identity_ref: None,
            requester_pending_request_is_retained: None,
            adapter_delivery_failure: None,
            remote_admission: Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                owner_serve_count: summary.served_owner_request_count(),
                owner_mutation_count: summary.actual_owner_write_count(),
            }),
            semantic_admission_count: 0,
            unauthenticated_semantic_admission_count: 0,
            owner_mutation_count: summary.actual_owner_write_count(),
        })
        .map_err(|_| ())?;
        return Ok(());
    }
    emit_child_event(&PrivateChildEvent::Ready {
        endpoint: endpoint_address,
    })
    .map_err(|_| ())?;
    let result = tokio::time::timeout(timeout, async {
        let connecting = endpoint.accept().await.ok_or(())?;
        let connection = connecting.await.map_err(|_| ())?;
        let mut session = match mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::accept(
            connection, trusted,
        )
        .await
        {
            Ok(session) => session,
            Err(_) => {
                emit_child_event(&PrivateChildEvent::rejected(
                    PrivateChildRejection::PeerBinding,
                    true,
                    0,
                    1,
                    1,
                    0,
                    0,
                ))
                .map_err(|_| ())?;
                return Ok(());
            }
        };
        if session.receive_and_validate_peer_preface().await.is_err() {
            session.close();
            emit_child_event(&PrivateChildEvent::rejected(
                PrivateChildRejection::PeerBinding,
                true,
                0,
                1,
                1,
                0,
                0,
            ))
            .map_err(|_| ())?;
            return Ok(());
        }
        session.send_local_preface().await.map_err(|_| ())?;
        if let Some(profile) = control.retry_profile {
            if admitted_lifecycle_stimulus.is_some() {
                return Err(());
            }
            return run_server_retry_sessions(
                runtime,
                session,
                &endpoint,
                &control,
                profile,
                run_ref,
                assigned_loci,
            )
            .await;
        }
        if let Some(profile) = control.owner_reply_replay_profile {
            if admitted_lifecycle_stimulus.is_some() {
                return Err(());
            }
            return run_server_owner_reply_replay_sessions(
                runtime,
                session,
                &endpoint,
                &control,
                PrivateOwnerReplyReplayServerSessionPlan {
                    profile,
                    falsifier: control.owner_reply_replay_falsifier,
                    run_ref,
                    cohort_provenance_ref: cohort_provenance_ref.clone(),
                },
            )
            .await;
        }
        if let Some(profile) = control.late_ingress_profile {
            return run_server_late_ingress_sessions(
                runtime,
                session,
                &endpoint,
                &control,
                PrivateLateIngressServerSessionPlan {
                    profile,
                    run_ref,
                    assigned_loci,
                    admitted_lifecycle_stimulus,
                },
            )
            .await;
        }
        if admitted_lifecycle_stimulus.is_some() {
            return Err(());
        }
        let (reply, request_delivery) = match session
            .receive_and_admit_generated_message(&mut runtime)
            .await
        {
            Ok(received) => received,
            Err(mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::FrameRejected)
                if control.adapter_delivery_profile
                    == Some(
                        I3LocalnetAdapterDeliveryProfile::TruncateGeneratedRequestAfterBodyPrefix,
                    ) =>
            {
                let tls_peer_verified = session.peer_spki_verified();
                let reciprocal_preface_verified = session.peer_preface_verified();
                let reliable_bidi_stream_count = session.reliable_bidi_stream_count();
                let quic_datagrams_enabled = session.quic_datagrams_enabled();
                let summary = runtime.observer_safe_runtime_summary();
                if !tls_peer_verified
                    || !reciprocal_preface_verified
                    || reliable_bidi_stream_count != 1
                    || quic_datagrams_enabled
                    || summary.served_owner_request_count() != 0
                    || summary.actual_owner_write_count() != 0
                {
                    return Err(());
                }
                emit_child_event(&PrivateChildEvent::AdapterDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessB,
                    exec_confirmed: true,
                    assigned_loci: assigned_loci.clone(),
                    trusted_control_consumed: true,
                    tainted_image_consumed: true,
                    tls_peer_verified,
                    reciprocal_preface_verified,
                    reliable_bidi_stream_count,
                    quic_datagrams_enabled,
                    request_identity_ref: None,
                    requester_pending_request_is_retained: None,
                    adapter_delivery_failure: Some(I3LocalnetAdapterDeliveryFailure::FrameRejected),
                    remote_admission: Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                        owner_serve_count: summary.served_owner_request_count(),
                        owner_mutation_count: summary.actual_owner_write_count(),
                    }),
                    semantic_admission_count: 0,
                    unauthenticated_semantic_admission_count: 0,
                    owner_mutation_count: summary.actual_owner_write_count(),
                })
                .map_err(|_| ())?;
                session.close();
                return Ok(());
            }
            Err(_)
                if control.fault_profile
                    == Some(I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite) =>
            {
                let summary = runtime.observer_safe_runtime_summary();
                // This is an actual owner-local post-EOF summary.  It has no
                // request identity because the adapter admitted no frame.
                emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessB,
                    request_identity_ref: None,
                    requester_observation: None,
                    requester_pending_request_is_retained: None,
                    requester_local_wait: None,
                    remote_admission: Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                        owner_serve_count: summary.served_owner_request_count(),
                        owner_mutation_count: summary.actual_owner_write_count(),
                    }),
                    semantic_admission_count: 0,
                    owner_mutation_count: summary.actual_owner_write_count(),
                    retry_evidence: None,
                    owner_reply_replay_evidence: None,
                })
                .map_err(|_| ())?;
                session.close();
                return Ok(());
            }
            Err(_) => return Err(()),
        };
        if !session.peer_spki_verified()
            || !session.peer_preface_verified()
            || session.reliable_bidi_stream_count() != 1
            || session.quic_datagrams_enabled()
        {
            return Err(());
        }
        let request_identity = request_delivery.semantic_request_identity_ref().to_string();
        let (reply, owner_expiry) = match resolve_received_owner_reply(
            &mut runtime,
            reply,
            &request_identity,
            control.owner_admission_drive_profile,
        )? {
            PrivateResolvedOwnerReply::Served(reply) => (reply, None),
            PrivateResolvedOwnerReply::DeclaredDeadlineExpired { reply, expiry } => {
                (reply, Some(expiry))
            }
        };
        if control.fault_profile
            == Some(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit)
        {
            // The real owner runtime has admitted, served, and written the
            // request.  This selected schedule then closes the actual session
            // without publishing an owner terminal event, so the supervisor
            // retains remote unknown rather than fabricating a verdict.
            let _ = reply;
            session.close();
            return Ok(());
        }
        if control.fault_profile == Some(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission) {
            let summary = runtime.observer_safe_runtime_summary();
            let occurrences = runtime.observer_safe_semantic_occurrences();
            let request_identity = request_delivery.semantic_request_identity_ref().to_string();
            let owner_mutation_count = summary.actual_owner_write_count();
            if control.fault_audit_falsifier
                == Some(I3LocalnetFaultAuditFalsifier::ReplaceOwnerFaultWithRejectedTerminal)
            {
                // This replaces only the terminal observer record after the
                // real owner request has already been admitted and written.
                // It is deliberately not an ambiguity result: the supervisor
                // must reject an unexpected terminal shape for a normal
                // profile instead of treating it as remote unknown.
                emit_child_event(&PrivateChildEvent::rejected(
                    PrivateChildRejection::Lifecycle,
                    true,
                    1,
                    1,
                    1,
                    1,
                    owner_mutation_count,
                ))
                .map_err(|_| ())?;
                let _ = reply;
                session.close();
                return Ok(());
            }
            let owner_serve_occurrence_ref = occurrences
                .owner_serve_linearization_occurrence_ref(&request_identity)
                .map(str::to_string)
                .ok_or(())?;
            let owner_write_occurrence_ref = occurrences
                .actual_owner_write_occurrence_ref(&request_identity)
                .map(str::to_string);
            let mut request_receive: PrivateDeliveryEvidence = request_delivery.into();
            if control.fault_audit_falsifier
                == Some(I3LocalnetFaultAuditFalsifier::MutateOwnerRequestReceiveCoreRef)
            {
                // This test-only mutation is applied only to the serialized
                // owner observer event after the real carrier has already
                // been admitted and dispatched.  It cannot affect semantic
                // traffic, authority, or the owner mutation.
                request_receive.core_ref = "observer-falsifier:core-ref".to_string();
            }
            if control.fault_audit_falsifier
                == Some(I3LocalnetFaultAuditFalsifier::SetOwnerRequestReceiveLinkedRequestIdentity)
            {
                request_receive.linked_request_identity_ref =
                    Some("observer-falsifier:linked-request".to_string());
            }
            if control.fault_audit_falsifier
                == Some(I3LocalnetFaultAuditFalsifier::ClearOwnerRequestReceiveCarrierRef)
            {
                request_receive.carrier_ref.clear();
            }
            if control.fault_audit_falsifier
                == Some(I3LocalnetFaultAuditFalsifier::ClearOwnerRequestReceiveNetworkOccurrenceRef)
            {
                request_receive.network_occurrence_ref.clear();
            }
            emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessB,
                request_identity_ref: Some(request_identity),
                requester_observation: None,
                requester_pending_request_is_retained: None,
                requester_local_wait: None,
                remote_admission: Some(PrivateRemoteAdmissionEvidence::Admitted {
                    request_receive: Box::new(request_receive),
                    owner_serve_count: summary.served_owner_request_count(),
                    owner_mutation_count,
                    owner_serve_occurrence_ref,
                    owner_write_occurrence_ref,
                }),
                semantic_admission_count: 1,
                owner_mutation_count,
                retry_evidence: None,
                owner_reply_replay_evidence: None,
            })
            .map_err(|_| ())?;
            // The reply carrier exists in the owner runtime but is deliberately
            // not written to the stream.  Closing here models delivery loss
            // after actual owner admission without asserting a result.
            let _ = reply;
            session.close();
            return Ok(());
        }
        let reply_delivery = session
            .send_generated_message(reply)
            .await
            .map_err(|_| ())?;
        session.finish_send().map_err(|_| ())?;
        let tls_peer_verified = session.peer_spki_verified();
        let reciprocal_preface_verified = session.peer_preface_verified();
        let reliable_bidi_stream_count = session.reliable_bidi_stream_count();
        let quic_datagrams_enabled = session.quic_datagrams_enabled();
        let summary = runtime.observer_safe_runtime_summary();
        let occurrences = runtime.observer_safe_semantic_occurrences();
        let request_identity = request_delivery.semantic_request_identity_ref().to_string();
        let observer_evidence = PrivateChildObserverEvidence {
            request_received: Some(request_delivery.into()),
            reply_sent: Some(reply_delivery.into()),
            local_store_ref: runtime.local_store_identity_ref().to_string(),
            owner_serve_occurrence_ref: occurrences
                .owner_serve_linearization_occurrence_ref(&request_identity)
                .map(str::to_string),
            owner_write_occurrence_ref: occurrences
                .actual_owner_write_occurrence_ref(&request_identity)
                .map(str::to_string),
            owner_expiry,
            ..PrivateChildObserverEvidence::default()
        };
        emit_child_event(&PrivateChildEvent::Completed {
            slot: I3LocalnetChildSlot::ProcessB,
            exec_confirmed: true,
            assigned_loci,
            trusted_control_consumed: true,
            tainted_image_consumed: true,
            tls_peer_verified,
            reciprocal_preface_verified,
            reliable_bidi_stream_count,
            quic_datagrams_enabled,
            semantic_admission_count: 1,
            unauthenticated_semantic_admission_count: 0,
            network_receipt_frame_count: 0,
            generated_request_count: 0,
            served_count: summary.served_owner_request_count(),
            write_count: summary.actual_owner_write_count(),
            reply_count: 1,
            receipt_count: 0,
            runtime_occurrence_count: occurrences.owner_serve_linearization_count()
                + occurrences.actual_owner_write_count(),
            observer_evidence,
        })
        .map_err(|_| ())?;
        match control.terminal_lifecycle_falsifier {
            PrivateChildTerminalLifecycleFalsifier::ExitNonzeroAfterCompleted => {
                std::process::exit(9);
            }
            PrivateChildTerminalLifecycleFalsifier::HangAfterCompleted => {
                // Deliberately block outside Tokio cancellation so the
                // supervisor exercises its bounded external reaper rather
                // than this child's semantic/transport deadline.
                thread::sleep(Duration::from_secs(60));
                unreachable!("forced lifecycle reaper must terminate child");
            }
            PrivateChildTerminalLifecycleFalsifier::None
            | PrivateChildTerminalLifecycleFalsifier::RejectAfterCompleted => {}
        }
        session.wait_for_peer_close().await;
        session.close();
        Ok(())
    })
    .await;
    endpoint.close(0_u32.into(), b"completed");
    let _ = tokio::time::timeout(Duration::from_secs(1), endpoint.wait_idle()).await;
    result.map_err(|_| ())?
}

async fn run_client_child(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    trusted: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    control: PrivateChildControl,
    timeout: Duration,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    let endpoint_address = control
        .endpoint
        .as_deref()
        .ok_or(())?
        .parse::<SocketAddr>()
        .map_err(|_| ())?;
    install_ring()?;
    let local_preface = trusted.localnet_preface();
    let run_ref = local_preface.run_ref().to_string();
    let cohort_provenance_ref = local_preface.cohort_provenance_ref().to_string();
    let mut endpoint = Endpoint::client(SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    let (client_config, _transport_evidence) = client_config(&control)?;
    endpoint.set_default_client_config(client_config);
    let result = tokio::time::timeout(timeout, async {
        let endpoint_closed_before_connect = control.adapter_delivery_profile
            == Some(I3LocalnetAdapterDeliveryProfile::EndpointClosedBeforeConnect);
        let prepared_request = if control.emit_request_before_wrong_peer_rejection
            || endpoint_closed_before_connect
        {
            let request = runtime
                .emit_generated_owner_request("init_avatar_hp")
                .map_err(|_| ())?;
            let before = requester_observer_state_ref(&runtime);
            Some((request, before))
        } else {
            None
        };
        let endpoint_request_identity = prepared_request
            .as_ref()
            .filter(|_| endpoint_closed_before_connect)
            .map(|(request, _)| request.semantic_request_identity_ref().to_string());
        let connection = match endpoint.connect(endpoint_address, "localhost") {
            Ok(connecting) => {
                if endpoint_closed_before_connect {
                    match tokio::time::timeout(
                        endpoint_unavailable_connect_budget(timeout).ok_or(())?,
                        connecting,
                    )
                    .await
                    {
                        Ok(Ok(connection)) => connection,
                        Ok(Err(_)) | Err(_) => {
                            emit_endpoint_unavailable_terminal(
                                &runtime,
                                assigned_loci.clone(),
                                endpoint_request_identity.ok_or(())?,
                            )?;
                            return Ok(());
                        }
                    }
                } else {
                    connecting.await.map_err(|_| ())?
                }
            }
            Err(_) if endpoint_closed_before_connect => {
                emit_endpoint_unavailable_terminal(
                    &runtime,
                    assigned_loci.clone(),
                    endpoint_request_identity.ok_or(())?,
                )?;
                return Ok(());
            }
            Err(_) => return Err(()),
        };
        if endpoint_closed_before_connect {
            // A successful connection contradicts the B-before-Ready close
            // schedule. Do not relabel it as endpoint unavailability.
            return Err(());
        }
        let mut session =
            match mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession::connect(
                connection, trusted,
            )
            .await
            {
                Ok(session) => session,
                Err(error) => {
                    let evidence = if let Some((_request, before)) = prepared_request {
                        let after = requester_observer_state_ref(&runtime);
                        let peer_binding = error.peer_binding_evidence();
                        let expected_peer_spki_ref = peer_binding
                            .map(|value| value.expected_peer_spki_ref().to_string());
                        let actual_peer_spki_ref = peer_binding
                            .and_then(|value| value.actual_peer_spki_ref())
                            .map(str::to_string);
                        let wrong_peer_ca_validated_leaf_ref = peer_binding
                            .and_then(|value| value.ca_validated_peer_leaf_ref())
                            .map(str::to_string);
                        let exact_peer_rejection = matches!(
                            error,
                            mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::PeerBindingRejected(_)
                        );
                        PrivateChildRejectionEvidence {
                            real_wrong_peer_delivery_attempted: exact_peer_rejection
                                && actual_peer_spki_ref.is_some(),
                            wrong_peer_certificate_chains_to_run_ca: exact_peer_rejection
                                && wrong_peer_ca_validated_leaf_ref.is_some(),
                            wrong_peer_spki_differs_from_expected: expected_peer_spki_ref
                                .as_deref()
                                .zip(actual_peer_spki_ref.as_deref())
                                .is_some_and(|(expected, actual)| expected != actual),
                            requester_pending_request_is_retained: runtime
                                .observer_safe_pending_owner_request_count()
                                == 1,
                            requester_observer_state_before: before,
                            requester_observer_state_after: after,
                            adapter_rejection_kind: exact_peer_rejection.then_some(
                                I3LocalnetAdapterRejectionKind::PeerBindingRejected,
                            ),
                            wrong_peer_ca_validated_leaf_ref,
                            expected_peer_spki_ref,
                            actual_peer_spki_ref,
                            ..PrivateChildRejectionEvidence::default()
                        }
                    } else {
                        PrivateChildRejectionEvidence::default()
                    };
                    emit_child_event(&PrivateChildEvent::rejected_with_evidence(
                        PrivateChildRejection::PeerBinding,
                        true,
                        0,
                        1,
                        1,
                        0,
                        0,
                        evidence,
                    ))
                    .map_err(|_| ())?;
                    return Ok(());
                }
            };
        if control.inject_bad_preface {
            session
                .send_unbound_preface_for_private_falsifier()
                .await
                .map_err(|_| ())?;
        } else {
            session.send_local_preface().await.map_err(|_| ())?;
        }
        if session.receive_and_validate_peer_preface().await.is_err() {
            session.close();
            emit_child_event(&PrivateChildEvent::rejected(
                PrivateChildRejection::PeerBinding,
                true,
                0,
                1,
                1,
                0,
                0,
            ))
            .map_err(|_| ())?;
            return Ok(());
        }
        if let Some(profile) = control.retry_profile {
            let retry_plan = PrivateRetryClientSessionPlan {
                profile,
                run_ref,
                assigned_loci,
            };
            return run_client_retry_sessions(
                runtime,
                session,
                &endpoint,
                endpoint_address,
                &control,
                retry_plan,
            )
            .await;
        }
        if let Some(profile) = control.owner_reply_replay_profile {
            return run_client_owner_reply_replay_sessions(
                runtime,
                session,
                &endpoint,
                endpoint_address,
                PrivateOwnerReplyReplayClientSessionPlan {
                    profile,
                    falsifier: control.owner_reply_replay_falsifier,
                    run_ref,
                    cohort_provenance_ref: cohort_provenance_ref.clone(),
                },
            )
            .await;
        }
        if let Some(profile) = control.late_ingress_profile {
            return run_client_late_ingress_sessions(
                runtime,
                session,
                &endpoint,
                endpoint_address,
                PrivateLateIngressClientSessionPlan {
                    profile,
                    run_ref,
                    assigned_loci,
                    late_ingress_falsifier: control.late_ingress_falsifier,
                    tainted_owner_lifecycle_ack_candidate: control
                        .tainted_owner_lifecycle_ack_candidate
                        .clone(),
                },
            )
            .await;
        }
        let request = match prepared_request {
            Some((request, _)) => request,
            None => runtime
                .emit_generated_owner_request("init_avatar_hp")
                .map_err(|_| ())?,
        };
        let request_identity = request.semantic_request_identity_ref().to_string();
        if control.fault_profile
            == Some(I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite)
        {
            emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                request_identity_ref: Some(request_identity),
                requester_observation: Some(
                    I3LocalnetRequesterFaultObservation::RequestCarrierWriteNotAttempted,
                ),
                requester_pending_request_is_retained: None,
                requester_local_wait: None,
                remote_admission: None,
                semantic_admission_count: 0,
                owner_mutation_count: 0,
                retry_evidence: None,
                owner_reply_replay_evidence: None,
            })
            .map_err(|_| ())?;
            // No generated carrier enters the adapter in this schedule.  The
            // source-derived request stays requester-local and pending; the
            // profile neither retries it nor creates another identity.
            session.close();
            return Ok(());
        }
        let request_delivery = match control.adapter_delivery_profile {
            Some(I3LocalnetAdapterDeliveryProfile::CompleteGeneratedRequestInTwoWrites) => {
                session
                    .test_only_send_generated_message_with_frame_write_control(
                        request,
                        mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicGeneratedFrameWriteControl::CompleteInTwoWrites {
                            first_frame_fragment_len: 1,
                        },
                    )
                    .await
                    .map_err(|_| ())?
            }
            Some(
                I3LocalnetAdapterDeliveryProfile::TruncateGeneratedRequestAfterBodyPrefix,
            ) => {
                let request_identity_ref = request.semantic_request_identity_ref().to_string();
                match session
                    .test_only_send_generated_message_with_frame_write_control(
                        request,
                        mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicGeneratedFrameWriteControl::TruncateAfterBodyPrefix {
                            body_prefix_len: 1,
                        },
                    )
                    .await
                {
                    Err(mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::FrameRejected) => {
                        let tls_peer_verified = session.peer_spki_verified();
                        let reciprocal_preface_verified = session.peer_preface_verified();
                        let reliable_bidi_stream_count = session.reliable_bidi_stream_count();
                        let quic_datagrams_enabled = session.quic_datagrams_enabled();
                        if !tls_peer_verified
                            || !reciprocal_preface_verified
                            || reliable_bidi_stream_count != 1
                            || quic_datagrams_enabled
                            || request_identity_ref.is_empty()
                            || runtime.observer_safe_pending_owner_request_count() != 1
                            || runtime
                                .observer_safe_runtime_summary()
                                .accepted_inbound_receipt_count()
                                != 0
                        {
                            return Err(());
                        }
                        emit_child_event(&PrivateChildEvent::AdapterDeliveryFault {
                            slot: I3LocalnetChildSlot::ProcessA,
                            exec_confirmed: true,
                            assigned_loci: assigned_loci.clone(),
                            trusted_control_consumed: true,
                            tainted_image_consumed: true,
                            tls_peer_verified,
                            reciprocal_preface_verified,
                            reliable_bidi_stream_count,
                            quic_datagrams_enabled,
                            request_identity_ref: Some(request_identity_ref),
                            requester_pending_request_is_retained: Some(true),
                            adapter_delivery_failure: Some(
                                I3LocalnetAdapterDeliveryFailure::FrameRejected,
                            ),
                            remote_admission: None,
                            semantic_admission_count: 0,
                            unauthenticated_semantic_admission_count: 0,
                            owner_mutation_count: 0,
                        })
                        .map_err(|_| ())?;
                        // The actual FIN was emitted by the controlled
                        // write. Keep A's connection alive until B has read
                        // that EOF, rejected the incomplete frame, and
                        // closed; this is not an early local-loss schedule.
                        session.wait_for_peer_close().await;
                        session.close();
                        return Ok(());
                    }
                    Ok(_) | Err(_) => return Err(()),
                }
            }
            Some(I3LocalnetAdapterDeliveryProfile::EndpointClosedBeforeConnect) => {
                return Err(());
            }
            None => session.send_generated_message(request).await.map_err(|_| ())?,
        };
        session.finish_send().map_err(|_| ())?;
        let (receipt, reply_delivery) = match session
            .receive_and_admit_generated_message(&mut runtime)
            .await
        {
            Ok(received) => received,
            Err(_)
                if matches!(
                    control.fault_profile,
                    Some(
                        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission
                            | I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit
                    )
                ) =>
            {
                let summary_before = runtime.observer_safe_runtime_summary();
                let pending_before = runtime.observer_safe_pending_owner_request_count();
                let requester_pending_request_is_retained = pending_before == 1;
                if !requester_pending_request_is_retained
                    || summary_before.accepted_inbound_receipt_count() != 0
                    || summary_before.accepted_inbound_declared_owner_failure_count() != 0
                {
                    return Err(());
                }
                let requester_local_wait = match control.requester_local_wait_profile {
                    None => {
                        if control.requester_local_wait_falsifier.is_some() {
                            return Err(());
                        }
                        None
                    }
                    Some(I3LocalnetRequesterLocalWaitProfile::WaitLocallyAfterObservedLoss) => {
                        let wait_started = Instant::now();
                        tokio::time::sleep(REQUESTER_LOCAL_WAIT_MINIMUM).await;
                        let summary_after = runtime.observer_safe_runtime_summary();
                        let mut evidence = PrivateRequesterLocalWaitEvidence {
                            observed_elapsed: wait_started.elapsed(),
                            required_minimum_elapsed: REQUESTER_LOCAL_WAIT_MINIMUM,
                            pending_before,
                            pending_after: runtime.observer_safe_pending_owner_request_count(),
                            local_receipt_before: summary_before.accepted_inbound_receipt_count(),
                            local_receipt_after: summary_after.accepted_inbound_receipt_count(),
                            terminal_failure_before: summary_before
                                .accepted_inbound_declared_owner_failure_count(),
                            terminal_failure_after: summary_after
                                .accepted_inbound_declared_owner_failure_count(),
                        };
                        if !evidence.is_exact() {
                            return Err(());
                        }
                        match control.requester_local_wait_falsifier {
                            None => {}
                            Some(
                                I3LocalnetRequesterLocalWaitFalsifier::SetObservedElapsedBelowMinimum,
                            ) => evidence.observed_elapsed = Duration::ZERO,
                            Some(
                                I3LocalnetRequesterLocalWaitFalsifier::ClearPostWaitPendingObservation,
                            ) => evidence.pending_after = 0,
                        }
                        Some(evidence)
                    }
                };
                emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessA,
                    request_identity_ref: Some(
                        request_delivery.semantic_request_identity_ref().to_string(),
                    ),
                    requester_observation: Some(
                        I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved,
                    ),
                    requester_pending_request_is_retained: Some(
                        requester_pending_request_is_retained,
                    ),
                    requester_local_wait,
                    remote_admission: None,
                    semantic_admission_count: 0,
                    owner_mutation_count: 0,
                    retry_evidence: None,
                    owner_reply_replay_evidence: None,
                })
                .map_err(|_| ())?;
                session.close();
                return Ok(());
            }
            Err(_) => return Err(()),
        };
        let receipt = receipt.ok_or(())?;
        let terminal_failure_consumed = receipt.is_observer_safe_terminal_failure_consumed()
            && receipt.has_no_transportable_carrier();
        if !terminal_failure_consumed
            && (!receipt.is_observer_safe_typed_result_or_receipt()
                || !receipt.has_no_transportable_carrier())
        {
            return Err(());
        }
        let tls_peer_verified = session.peer_spki_verified();
        let reciprocal_preface_verified = session.peer_preface_verified();
        let reliable_bidi_stream_count = session.reliable_bidi_stream_count();
        let quic_datagrams_enabled = session.quic_datagrams_enabled();
        session.close();
        let summary = runtime.observer_safe_runtime_summary();
        let occurrences = runtime.observer_safe_semantic_occurrences();
        let request_identity = request_delivery.semantic_request_identity_ref().to_string();
        if terminal_failure_consumed {
            let terminal_occurrence = occurrences
                .requester_terminal_declared_owner_failure_occurrence_ref(&request_identity)
                .map(str::to_string)
                .ok_or(())?;
            if summary.accepted_inbound_declared_owner_failure_count() != 1
                || summary.accepted_inbound_receipt_count() != 0
                || occurrences.requester_terminal_declared_owner_failure_count() != 1
                || occurrences.requester_local_receipt_count() != 0
                || runtime.observer_safe_pending_owner_request_count() != 0
                || terminal_occurrence.is_empty()
            {
                return Err(());
            }
            let observer_evidence = PrivateChildObserverEvidence {
                request_sent: Some(request_delivery.into()),
                reply_received: Some(reply_delivery.into()),
                local_store_ref: runtime.local_store_identity_ref().to_string(),
                requester_terminal_failure_occurrence_ref: Some(terminal_occurrence),
                ..PrivateChildObserverEvidence::default()
            };
            emit_child_event(&PrivateChildEvent::Completed {
                slot: I3LocalnetChildSlot::ProcessA,
                exec_confirmed: true,
                assigned_loci,
                trusted_control_consumed: true,
                tainted_image_consumed: true,
                tls_peer_verified,
                reciprocal_preface_verified,
                reliable_bidi_stream_count,
                quic_datagrams_enabled,
                semantic_admission_count: 1,
                unauthenticated_semantic_admission_count: 0,
                network_receipt_frame_count: 0,
                generated_request_count: 1,
                served_count: 0,
                write_count: 0,
                reply_count: 0,
                receipt_count: 0,
                runtime_occurrence_count: occurrences.requester_terminal_declared_owner_failure_count(),
                observer_evidence,
            })
            .map_err(|_| ())?;
            session.wait_for_peer_close().await;
            session.close();
            return Ok(());
        }
        let observer_evidence = PrivateChildObserverEvidence {
            request_sent: Some(request_delivery.into()),
            reply_received: Some(reply_delivery.into()),
            local_store_ref: runtime.local_store_identity_ref().to_string(),
            requester_receipt_occurrence_ref: occurrences
                .requester_local_receipt_occurrence_ref(&request_identity)
                .map(str::to_string),
            ..PrivateChildObserverEvidence::default()
        };
        if control.terminal_lifecycle_falsifier
            == PrivateChildTerminalLifecycleFalsifier::RejectAfterCompleted
        {
            emit_child_event(&PrivateChildEvent::rejected_with_evidence(
                PrivateChildRejection::Lifecycle,
                true,
                0,
                1,
                1,
                1,
                0,
                PrivateChildRejectionEvidence::default(),
            ))
            .map_err(|_| ())?;
            return Ok(());
        }
        emit_child_event(&PrivateChildEvent::Completed {
            slot: I3LocalnetChildSlot::ProcessA,
            exec_confirmed: true,
            assigned_loci,
            trusted_control_consumed: true,
            tainted_image_consumed: true,
            tls_peer_verified,
            reciprocal_preface_verified,
            reliable_bidi_stream_count,
            quic_datagrams_enabled,
            semantic_admission_count: 1,
            unauthenticated_semantic_admission_count: 0,
            network_receipt_frame_count: 0,
            generated_request_count: 1,
            served_count: 0,
            write_count: 0,
            reply_count: 0,
            receipt_count: summary.accepted_inbound_receipt_count(),
            runtime_occurrence_count: occurrences.requester_local_receipt_count(),
            observer_evidence,
        })
        .map_err(|_| ())?;
        match control.terminal_lifecycle_falsifier {
            PrivateChildTerminalLifecycleFalsifier::ExitNonzeroAfterCompleted => {
                std::process::exit(9);
            }
            PrivateChildTerminalLifecycleFalsifier::HangAfterCompleted => {
                thread::sleep(Duration::from_secs(60));
                unreachable!("forced lifecycle reaper must terminate child");
            }
            PrivateChildTerminalLifecycleFalsifier::None
            | PrivateChildTerminalLifecycleFalsifier::RejectAfterCompleted => {}
        }
        Ok(())
    })
    .await;
    endpoint.close(0_u32.into(), b"completed");
    let _ = tokio::time::timeout(Duration::from_secs(1), endpoint.wait_idle()).await;
    result.map_err(|_| ())?
}

fn endpoint_unavailable_connect_budget(timeout: Duration) -> Option<Duration> {
    timeout
        .checked_sub(Duration::from_millis(1))
        .filter(|remaining| !remaining.is_zero())
        .map(|remaining| remaining.min(ENDPOINT_UNAVAILABLE_CONNECT_BUDGET))
}

fn emit_endpoint_unavailable_terminal(
    runtime: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    assigned_loci: Vec<String>,
    request_identity_ref: String,
) -> Result<(), ()> {
    if request_identity_ref.is_empty()
        || runtime.observer_safe_pending_owner_request_count() != 1
        || runtime
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count()
            != 0
    {
        return Err(());
    }
    emit_child_event(&PrivateChildEvent::AdapterDeliveryFault {
        slot: I3LocalnetChildSlot::ProcessA,
        exec_confirmed: true,
        assigned_loci,
        trusted_control_consumed: true,
        tainted_image_consumed: true,
        tls_peer_verified: false,
        reciprocal_preface_verified: false,
        reliable_bidi_stream_count: 0,
        quic_datagrams_enabled: false,
        request_identity_ref: Some(request_identity_ref),
        requester_pending_request_is_retained: Some(true),
        adapter_delivery_failure: Some(I3LocalnetAdapterDeliveryFailure::EndpointUnavailable),
        remote_admission: None,
        semantic_admission_count: 0,
        unauthenticated_semantic_admission_count: 0,
        owner_mutation_count: 0,
    })
    .map_err(|_| ())
}

fn retry_attempt_snapshot(
    runtime: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    request_identity_ref: &str,
) -> Option<PrivateRetryAttemptEvidence> {
    let summary = runtime
        .observer_safe_original_owner_request_attempts()
        .into_iter()
        .find(|summary| summary.semantic_request_identity_ref() == request_identity_ref)?;
    let reason = match summary.reason() {
        mir_runtime::sys5_i3_process_runtime::Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason::InitialDelivery => {
            I3LocalnetRetryAttemptReason::InitialDelivery
        }
        mir_runtime::sys5_i3_process_runtime::Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason::ReconnectRetry => {
            I3LocalnetRetryAttemptReason::ReconnectRetry
        }
    };
    Some(PrivateRetryAttemptEvidence {
        semantic_request_identity_ref: summary.semantic_request_identity_ref().to_string(),
        source_requester_locus: summary.source_requester_locus().to_string(),
        requester_binding_ref: summary.requester_binding_ref().to_string(),
        attempt_generation: summary.attempt_generation(),
        reason,
    })
}

async fn accept_reconnect_session(
    endpoint: &Endpoint,
    reconnect: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicReconnect,
) -> Result<mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession, ()> {
    let connecting = endpoint.accept().await.ok_or(())?;
    let connection = connecting.await.map_err(|_| ())?;
    reconnect.accept(connection).await.map_err(|_| ())
}

async fn connect_reconnect_session(
    endpoint: &Endpoint,
    endpoint_address: SocketAddr,
    reconnect: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicReconnect,
) -> Result<mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession, ()> {
    let connection = endpoint
        .connect(endpoint_address, "localhost")
        .map_err(|_| ())?
        .await
        .map_err(|_| ())?;
    reconnect.connect(connection).await.map_err(|_| ())
}

/// Immutable actual-child inputs for the one held session-one ingress on B.
/// Grouping them keeps the session routine focused on the retained ingress
/// state rather than widening the child protocol.
struct PrivateLateIngressServerSessionPlan {
    profile: I3LocalnetLateIngressProfile,
    run_ref: String,
    assigned_loci: Vec<String>,
    admitted_lifecycle_stimulus:
        Option<mir_runtime::sys5_i3_process_runtime::Sys5I3AdmittedLifecycleStimulus>,
}

async fn run_server_late_ingress_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    control: &PrivateChildControl,
    late_plan: PrivateLateIngressServerSessionPlan,
) -> Result<(), ()> {
    let PrivateLateIngressServerSessionPlan {
        profile,
        run_ref,
        assigned_loci,
        admitted_lifecycle_stimulus,
    } = late_plan;
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    if !first_session_peer_spki_verified
        || !first_session_reciprocal_preface_verified
        || first_session.session_attempt_generation() != 1
    {
        return Err(());
    }
    // The adapter owns the raw frame and decoded candidate. Probe code sees
    // only this session-one receiver record before a later runtime admission.
    let pending_ingress = first_session
        .receive_complete_pending_ingress()
        .await
        .map_err(|_| ())?;
    let retained = pending_ingress.observer_safe_evidence();
    let mut retained_observer_evidence = PrivateRetainedIngressEvidence {
        session_generation: retained.session_attempt_generation(),
        candidate_commitment_ref: retained.candidate_commitment_ref().to_string(),
        network_occurrence_ref: retained.network_occurrence_ref().to_string(),
    };
    if retained_observer_evidence.session_generation != 1
        || retained_observer_evidence
            .candidate_commitment_ref
            .is_empty()
        || retained_observer_evidence.network_occurrence_ref.is_empty()
    {
        return Err(());
    }
    let retained_ingress_repeat_acquisition_rejection = if control.late_ingress_falsifier
        == Some(I3LocalnetLateIngressFalsifier::RepeatRetainedIngressAcquisition)
    {
        match first_session.receive_complete_pending_ingress().await {
            Err(
                mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::LocalAttemptRejected,
            ) => Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected),
            _ => return Err(()),
        }
    } else {
        None
    };
    match control.late_ingress_falsifier {
        Some(I3LocalnetLateIngressFalsifier::ClearRetainedIngressCandidateCommitment) => {
            retained_observer_evidence.candidate_commitment_ref.clear();
        }
        Some(I3LocalnetLateIngressFalsifier::MutateRetainedIngressCandidateCommitment) => {
            retained_observer_evidence.candidate_commitment_ref =
                "observer-falsifier:retained-ingress-commitment".to_string();
        }
        _ => {}
    }
    let m9_lifecycle_source_derived = match profile {
        I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
            if admitted_lifecycle_stimulus.is_some() {
                return Err(());
            }
            None
        }
        I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
            // The retained session-one frame is already complete, but no
            // reconnect exists yet. Install and consume the receipt before
            // releasing session one so the requester cannot reach session
            // two before the real G2 withdrawal is locally installed.
            let stimulus = admitted_lifecycle_stimulus.ok_or(())?;
            let installed_ack = runtime
                .install_admitted_owner_capability_successor(stimulus)
                .map_err(|_| ())?;
            let source_derived = {
                let installed_lifecycle = runtime
                    .observer_safe_installed_owner_capability_lifecycle()
                    .ok_or(())?;
                if installed_lifecycle.origin()
                    != mir_runtime::sys5_i3_process_runtime::Sys5I3ObserverSafeLifecycleOrigin::M9AdmittedLifecycle
                    || installed_lifecycle.source_derived()
                {
                    return Err(());
                }
                // Retain only this observer-safe scalar. The receipt and
                // installed lifecycle borrow do not survive late admission.
                installed_lifecycle.source_derived()
            };
            let suppress_registered_b_ack = matches!(
                control.late_ingress_falsifier,
                Some(
                    I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout
                        | I3LocalnetLateIngressFalsifier::DropBRegisteredAck
                )
            );
            let replay_registered_b_ack = control.late_ingress_falsifier
                == Some(I3LocalnetLateIngressFalsifier::ReplayBRegisteredAck);
            if !suppress_registered_b_ack {
                let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
                let framed_ack = codec
                    .encode_installed_owner_lifecycle_ack(installed_ack)
                    .map_err(|_| ())?;
                let mut writer = take_registered_owner_lifecycle_ack_writer().map_err(|_| ())?;
                write_registered_owner_lifecycle_ack(
                    &mut writer,
                    &framed_ack,
                    replay_registered_b_ack,
                )
                .map_err(|_| ())?;
            }
            Some(source_derived)
        }
    };
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut reconnect_session = accept_reconnect_session(endpoint, reconnect).await?;
    reconnect_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    reconnect_session
        .send_local_preface()
        .await
        .map_err(|_| ())?;
    if !reconnect_session.peer_spki_verified()
        || !reconnect_session.peer_preface_verified()
        || reconnect_session.session_attempt_generation() != 2
    {
        return Err(());
    }

    match profile {
        I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
            let (reply, late_admission_delivery) = reconnect_session
                .admit_retained_pending_ingress(&mut runtime, pending_ingress)
                .map_err(|_| ())?;
            let reply = reply.ok_or(())?;
            let reply_delivery = reconnect_session
                .send_generated_message(reply)
                .await
                .map_err(|_| ())?;
            reconnect_session.finish_send().map_err(|_| ())?;
            // Local finish is not a receipt. Wait for the actual requester
            // close after its consumed receipt before completing B's report.
            reconnect_session.wait_for_peer_close().await;
            let summary = runtime.observer_safe_runtime_summary();
            let occurrences = runtime.observer_safe_semantic_occurrences();
            let request_identity_ref = late_admission_delivery
                .semantic_request_identity_ref()
                .to_string();
            let observer_evidence = PrivateChildObserverEvidence {
                request_received: Some(late_admission_delivery.clone().into()),
                reply_sent: Some(reply_delivery.into()),
                local_store_ref: runtime.local_store_identity_ref().to_string(),
                owner_serve_occurrence_ref: occurrences
                    .owner_serve_linearization_occurrence_ref(&request_identity_ref)
                    .map(str::to_string),
                owner_write_occurrence_ref: occurrences
                    .actual_owner_write_occurrence_ref(&request_identity_ref)
                    .map(str::to_string),
                ..PrivateChildObserverEvidence::default()
            };
            if summary.served_owner_request_count() != 1
                || summary.actual_owner_write_count() != 1
                || observer_evidence.owner_serve_occurrence_ref.is_none()
                || observer_evidence.owner_write_occurrence_ref.is_none()
            {
                return Err(());
            }
            emit_child_event(&PrivateChildEvent::LateIngress {
                slot: I3LocalnetChildSlot::ProcessB,
                terminal_outcome: PrivateLateIngressTerminalOutcome::Completed,
                exec_confirmed: true,
                assigned_loci,
                trusted_control_consumed: true,
                tainted_image_consumed: true,
                tls_peer_verified: reconnect_session.peer_spki_verified(),
                reciprocal_preface_verified: reconnect_session.peer_preface_verified(),
                reliable_bidi_stream_count: reconnect_session.reliable_bidi_stream_count(),
                quic_datagrams_enabled: reconnect_session.quic_datagrams_enabled(),
                semantic_admission_count: 1,
                unauthenticated_semantic_admission_count: 0,
                network_receipt_frame_count: 0,
                generated_request_count: 0,
                served_count: summary.served_owner_request_count(),
                write_count: summary.actual_owner_write_count(),
                reply_count: 1,
                receipt_count: 0,
                runtime_occurrence_count: occurrences.owner_serve_linearization_count()
                    + occurrences.actual_owner_write_count(),
                observer_evidence,
                late_ingress_evidence: PrivateChildLateIngressEvidence {
                    run_ref,
                    first_session_generation: 1,
                    reconnect_session_generation: reconnect_session.session_attempt_generation(),
                    first_session_peer_spki_verified,
                    first_session_reciprocal_preface_verified,
                    reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                    reconnect_session_reciprocal_preface_verified: reconnect_session
                        .peer_preface_verified(),
                    retained_session_one_ingress: Some(retained_observer_evidence),
                    retained_ingress_repeat_acquisition_rejection,
                    late_admission_delivery: Some(late_admission_delivery.into()),
                    owner_outcome: Some(PrivateLateIngressOwnerOutcome::Admitted {
                        owner_serve_count: summary.served_owner_request_count(),
                        owner_mutation_count: summary.actual_owner_write_count(),
                    }),
                    lifecycle_provenance: Some(
                        I3LocalnetLateIngressLifecycleProvenance::NotSelected,
                    ),
                    ..PrivateChildLateIngressEvidence::default()
                },
            })
            .map_err(|_| ())?;
        }
        I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
            let m9_lifecycle_source_derived = m9_lifecycle_source_derived.ok_or(())?;
            let error = match reconnect_session
                .admit_retained_pending_ingress(&mut runtime, pending_ingress)
            {
                Err(error) => error,
                Ok(_) => return Err(()),
            };
            let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt: _,
            } = error
            else {
                return Err(());
            };
            if error.kind()
                != mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
            {
                return Err(());
            }
            let summary = runtime.observer_safe_runtime_summary();
            if summary.served_owner_request_count() != 0 || summary.actual_owner_write_count() != 0
            {
                return Err(());
            }
            // B has emitted only its reconnect preface.  A local connection
            // close can race A's preface validation, so finish this stream
            // and wait for A's actual post-EOF pending observation/close.
            reconnect_session.finish_send().map_err(|_| ())?;
            reconnect_session.wait_for_peer_close().await;
            let owner_terminal_trusted_control_consumed = control.late_ingress_falsifier
                != Some(
                    I3LocalnetLateIngressFalsifier::SetOwnerLateIngressTerminalUnauthenticatedAdmissionAndClearTrustedControl,
                );
            let owner_terminal_unauthenticated_semantic_admission_count = if owner_terminal_trusted_control_consumed {
                0
            } else {
                1
            };
            emit_child_event(&PrivateChildEvent::LateIngress {
                slot: I3LocalnetChildSlot::ProcessB,
                terminal_outcome: PrivateLateIngressTerminalOutcome::HandledDeliveryFault,
                exec_confirmed: true,
                assigned_loci,
                trusted_control_consumed: owner_terminal_trusted_control_consumed,
                tainted_image_consumed: true,
                tls_peer_verified: reconnect_session.peer_spki_verified(),
                reciprocal_preface_verified: reconnect_session.peer_preface_verified(),
                reliable_bidi_stream_count: reconnect_session.reliable_bidi_stream_count(),
                quic_datagrams_enabled: reconnect_session.quic_datagrams_enabled(),
                semantic_admission_count: 0,
                unauthenticated_semantic_admission_count:
                    owner_terminal_unauthenticated_semantic_admission_count,
                network_receipt_frame_count: 0,
                generated_request_count: 0,
                served_count: summary.served_owner_request_count(),
                write_count: summary.actual_owner_write_count(),
                reply_count: 0,
                receipt_count: 0,
                runtime_occurrence_count: 0,
                observer_evidence: PrivateChildObserverEvidence {
                    local_store_ref: runtime.local_store_identity_ref().to_string(),
                    ..PrivateChildObserverEvidence::default()
                },
                late_ingress_evidence: PrivateChildLateIngressEvidence {
                    run_ref,
                    first_session_generation: 1,
                    reconnect_session_generation: reconnect_session.session_attempt_generation(),
                    first_session_peer_spki_verified,
                    first_session_reciprocal_preface_verified,
                    reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                    reconnect_session_reciprocal_preface_verified: reconnect_session
                        .peer_preface_verified(),
                    retained_session_one_ingress: Some(retained_observer_evidence),
                    owner_outcome: Some(
                        PrivateLateIngressOwnerOutcome::CarrierAdmissionRejected {
                            owner_serve_count: summary.served_owner_request_count(),
                            owner_mutation_count: summary.actual_owner_write_count(),
                        },
                    ),
                    lifecycle_provenance: Some(
                        I3LocalnetLateIngressLifecycleProvenance::M9AdmittedLifecycle,
                    ),
                    m9_lifecycle_source_derived: Some(m9_lifecycle_source_derived),
                    ..PrivateChildLateIngressEvidence::default()
                },
            })
            .map_err(|_| ())?;
        }
    }
    reconnect_session.close();
    Ok(())
}

async fn run_server_retry_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    control: &PrivateChildControl,
    profile: I3LocalnetRetryProfile,
    run_ref: String,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    if !first_session_peer_spki_verified
        || !first_session_reciprocal_preface_verified
        || first_session.session_attempt_generation() != 1
    {
        return Err(());
    }

    if control.retry_falsifier == Some(I3LocalnetRetryFalsifier::RetryOnInitialVerifiedSession) {
        // The requester rejects before any request frame exists.  This owner
        // event records only the actual clean first-session EOF; it is not a
        // synthetic second-session retry audit.
        if first_session
            .receive_and_admit_generated_message(&mut runtime)
            .await
            .is_ok()
        {
            // A first-session carrier would contradict the local generation
            // misuse path.  Do not turn that unexpected admission into a
            // profile-shaped zero report.
            return Err(());
        }
        let summary = runtime.observer_safe_runtime_summary();
        if summary.served_owner_request_count() != 0 || summary.actual_owner_write_count() != 0 {
            return Err(());
        }
        emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
            slot: I3LocalnetChildSlot::ProcessB,
            request_identity_ref: None,
            requester_observation: None,
            requester_pending_request_is_retained: None,
            requester_local_wait: None,
            remote_admission: Some(PrivateRemoteAdmissionEvidence::NoAdmission {
                owner_serve_count: summary.served_owner_request_count(),
                owner_mutation_count: summary.actual_owner_write_count(),
            }),
            semantic_admission_count: summary.served_owner_request_count(),
            owner_mutation_count: summary.actual_owner_write_count(),
            retry_evidence: None,
            owner_reply_replay_evidence: None,
        })
        .map_err(|_| ())?;
        first_session.close();
        return Ok(());
    }

    let (initial_request_delivery, initial_owner_admission, initial_owner_expiry): (
        Option<PrivateDeliveryEvidence>,
        Option<PrivateRemoteAdmissionEvidence>,
        Option<PrivateOwnerAdmissionExpiryEvidence>,
    ) = match profile {
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => {
            // The client closes the first checked session before any carrier
            // write.  Treat its EOF only as no admission; do not infer a
            // request identity or server result.
            if first_session
                .receive_and_admit_generated_message(&mut runtime)
                .await
                .is_ok()
            {
                return Err(());
            }
            let summary = runtime.observer_safe_runtime_summary();
            if summary.served_owner_request_count() != 0 || summary.actual_owner_write_count() != 0
            {
                return Err(());
            }
            (None, None, None)
        }
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
            let (reply, delivery) = first_session
                .receive_and_admit_generated_message(&mut runtime)
                .await
                .map_err(|_| ())?;
            let request_identity_ref = delivery.semantic_request_identity_ref().to_string();
            match resolve_received_owner_reply(
                &mut runtime,
                reply,
                &request_identity_ref,
                control.owner_admission_drive_profile,
            )? {
                PrivateResolvedOwnerReply::Served(_) => {
                    let summary = runtime.observer_safe_runtime_summary();
                    let occurrences = runtime.observer_safe_semantic_occurrences();
                    let owner_serve_occurrence_ref = occurrences
                        .owner_serve_linearization_occurrence_ref(&request_identity_ref)
                        .map(str::to_string)
                        .ok_or(())?;
                    let owner_write_occurrence_ref = occurrences
                        .actual_owner_write_occurrence_ref(&request_identity_ref)
                        .map(str::to_string);
                    let admission = PrivateRemoteAdmissionEvidence::Admitted {
                        request_receive: Box::new(delivery.clone().into()),
                        owner_serve_count: summary.served_owner_request_count(),
                        owner_mutation_count: summary.actual_owner_write_count(),
                        owner_serve_occurrence_ref,
                        owner_write_occurrence_ref,
                    };
                    (Some(delivery.into()), Some(admission), None)
                }
                PrivateResolvedOwnerReply::DeclaredDeadlineExpired {
                    reply: _,
                    mut expiry,
                } => {
                    // The owner did generate the reply, but this existing
                    // retry profile intentionally drops it before A can
                    // consume it. Retain B's actual first-session receive
                    // privately for the later strict audit join.
                    expiry.request_receive = Some(Box::new(delivery.clone().into()));
                    (Some(delivery.into()), None, Some(expiry))
                }
            }
        }
    };
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut reconnect_session = accept_reconnect_session(endpoint, reconnect).await?;
    reconnect_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    reconnect_session
        .send_local_preface()
        .await
        .map_err(|_| ())?;
    if !reconnect_session.peer_spki_verified()
        || !reconnect_session.peer_preface_verified()
        || reconnect_session.session_attempt_generation() != 2
    {
        return Err(());
    }

    match profile {
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => {
            let (reply, request_delivery) = reconnect_session
                .receive_and_admit_generated_message(&mut runtime)
                .await
                .map_err(|_| ())?;
            let reply = reply.ok_or(())?;
            let reply_delivery = reconnect_session
                .send_generated_message(reply)
                .await
                .map_err(|_| ())?;
            reconnect_session.finish_send().map_err(|_| ())?;
            let request_identity_ref = request_delivery.semantic_request_identity_ref().to_string();
            let summary = runtime.observer_safe_runtime_summary();
            let occurrences = runtime.observer_safe_semantic_occurrences();
            let owner_serve_occurrence_ref = occurrences
                .owner_serve_linearization_occurrence_ref(&request_identity_ref)
                .map(str::to_string)
                .ok_or(())?;
            let owner_write_occurrence_ref = occurrences
                .actual_owner_write_occurrence_ref(&request_identity_ref)
                .map(str::to_string);
            let retry_evidence = PrivateChildRetryEvidence {
                run_ref,
                first_session_generation: 1,
                reconnect_session_generation: reconnect_session.session_attempt_generation(),
                first_session_peer_spki_verified,
                first_session_reciprocal_preface_verified,
                reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                reconnect_session_reciprocal_preface_verified: reconnect_session
                    .peer_preface_verified(),
                reconnect_owner_outcome: Some(PrivateReconnectOwnerOutcome::Replied {
                    owner_serve_count: summary.served_owner_request_count(),
                    owner_mutation_count: summary.actual_owner_write_count(),
                    owner_serve_occurrence_ref: owner_serve_occurrence_ref.clone(),
                    owner_write_occurrence_ref: owner_write_occurrence_ref.clone(),
                }),
                ..PrivateChildRetryEvidence::default()
            };
            let observer_evidence = PrivateChildObserverEvidence {
                request_received: Some(request_delivery.into()),
                reply_sent: Some(reply_delivery.into()),
                local_store_ref: runtime.local_store_identity_ref().to_string(),
                owner_serve_occurrence_ref: Some(owner_serve_occurrence_ref),
                owner_write_occurrence_ref,
                retry_evidence: Some(retry_evidence),
                ..PrivateChildObserverEvidence::default()
            };
            emit_child_event(&PrivateChildEvent::Completed {
                slot: I3LocalnetChildSlot::ProcessB,
                exec_confirmed: true,
                assigned_loci,
                trusted_control_consumed: true,
                tainted_image_consumed: true,
                tls_peer_verified: reconnect_session.peer_spki_verified(),
                reciprocal_preface_verified: reconnect_session.peer_preface_verified(),
                reliable_bidi_stream_count: reconnect_session.reliable_bidi_stream_count(),
                quic_datagrams_enabled: reconnect_session.quic_datagrams_enabled(),
                semantic_admission_count: 1,
                unauthenticated_semantic_admission_count: 0,
                network_receipt_frame_count: 0,
                generated_request_count: 0,
                served_count: summary.served_owner_request_count(),
                write_count: summary.actual_owner_write_count(),
                reply_count: 1,
                receipt_count: 0,
                runtime_occurrence_count: occurrences.owner_serve_linearization_count()
                    + occurrences.actual_owner_write_count(),
                observer_evidence,
            })
            .map_err(|_| ())?;
            // `finish_send` only completes the local stream half.  Keep the
            // owner alive until the requester has actually consumed its
            // receipt and closed, as in the ordinary successful owner path.
            reconnect_session.wait_for_peer_close().await;
        }
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
            let error = match reconnect_session
                .receive_and_admit_generated_message(&mut runtime)
                .await
            {
                Err(error) => error,
                Ok(_) => return Err(()),
            };
            let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt,
            } = error
            else {
                return Err(());
            };
            if error.kind()
                != mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected
            {
                return Err(());
            }
            let summary = runtime.observer_safe_runtime_summary();
            let request_identity_ref = initial_request_delivery
                .as_ref()
                .map(|delivery| delivery.semantic_request_identity_ref.clone())
                .ok_or(())?;
            let reconnect_owner_outcome = if let Some(expiry) = initial_owner_expiry.as_ref() {
                let request_receive = expiry.request_receive.as_deref().ok_or(())?;
                let decision = runtime
                    .observer_safe_owner_admission_expiry_decision(&request_identity_ref)
                    .ok_or(())?;
                let owner_admission = runtime.observer_safe_owner_admission_summary();
                if !expiry.is_exact_declared_expiry()
                    || request_receive.semantic_request_identity_ref != request_identity_ref
                    || request_receive.candidate_commitment_ref.is_empty()
                    || initial_request_delivery.as_ref().is_none_or(|delivery| {
                        delivery.candidate_commitment_ref
                            != request_receive.candidate_commitment_ref
                    })
                    || decision.decision_commitment_ref() != expiry.decision_commitment_ref
                    || decision.decision_occurrence_ref() != expiry.decision_occurrence_ref
                    || owner_admission.awaiting_count() != 0
                    || owner_admission.expired_count() != 1
                    || owner_admission.rejected_before_serve_count() != 0
                    || owner_admission.serve_reserved_count() != 0
                    || runtime.observer_safe_inbound_owner_request_tombstone_count() != 1
                    || summary.served_owner_request_count() != 0
                    || summary.actual_owner_write_count() != 0
                {
                    return Err(());
                }
                PrivateReconnectOwnerOutcome::DuplicateRequestRejectedAfterDeclaredDeadlineExpired {
                    candidate_commitment_ref: rejected_attempt
                        .candidate_commitment_ref()
                        .to_string(),
                    network_occurrence_ref: rejected_attempt.network_occurrence_ref().to_string(),
                    owner_expired_count: owner_admission.expired_count(),
                    owner_serve_count: summary.served_owner_request_count(),
                    owner_mutation_count: summary.actual_owner_write_count(),
                }
            } else {
                PrivateReconnectOwnerOutcome::DuplicateRequestRejected {
                    candidate_commitment_ref: rejected_attempt
                        .candidate_commitment_ref()
                        .to_string(),
                    network_occurrence_ref: rejected_attempt.network_occurrence_ref().to_string(),
                    owner_serve_count: summary.served_owner_request_count(),
                    owner_mutation_count: summary.actual_owner_write_count(),
                }
            };
            let retry_evidence = PrivateChildRetryEvidence {
                run_ref,
                first_session_generation: 1,
                reconnect_session_generation: reconnect_session.session_attempt_generation(),
                first_session_peer_spki_verified,
                first_session_reciprocal_preface_verified,
                reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                reconnect_session_reciprocal_preface_verified: reconnect_session
                    .peer_preface_verified(),
                initial_request_delivery,
                initial_owner_admission: initial_owner_admission.clone(),
                initial_owner_expiry,
                reconnect_owner_outcome: Some(reconnect_owner_outcome),
                ..PrivateChildRetryEvidence::default()
            };
            emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessB,
                request_identity_ref: Some(request_identity_ref),
                requester_observation: None,
                requester_pending_request_is_retained: None,
                requester_local_wait: None,
                remote_admission: initial_owner_admission,
                semantic_admission_count: 1,
                owner_mutation_count: summary.actual_owner_write_count(),
                retry_evidence: Some(retry_evidence),
                owner_reply_replay_evidence: None,
            })
            .map_err(|_| ())?;
        }
    }
    reconnect_session.close();
    Ok(())
}

/// Immutable actual-child inputs for the one bounded reconnect route.
struct PrivateRetryClientSessionPlan {
    profile: I3LocalnetRetryProfile,
    run_ref: String,
    assigned_loci: Vec<String>,
}

/// Immutable B-local inputs for the bounded reply replay. The source-derived
/// reply itself stays inside the opaque private-QUIC candidate.
struct PrivateOwnerReplyReplayServerSessionPlan {
    profile: I3LocalnetOwnerReplyReplayProfile,
    falsifier: Option<I3LocalnetOwnerReplyReplayFalsifier>,
    run_ref: String,
    cohort_provenance_ref: String,
}

/// Immutable A-local inputs for the bounded reply replay. There is no source
/// resend or caller-provided reply/result on the successor session.
struct PrivateOwnerReplyReplayClientSessionPlan {
    profile: I3LocalnetOwnerReplyReplayProfile,
    falsifier: Option<I3LocalnetOwnerReplyReplayFalsifier>,
    run_ref: String,
    cohort_provenance_ref: String,
}

async fn run_server_owner_reply_replay_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    control: &PrivateChildControl,
    replay_plan: PrivateOwnerReplyReplayServerSessionPlan,
) -> Result<(), ()> {
    let PrivateOwnerReplyReplayServerSessionPlan {
        profile,
        falsifier,
        run_ref,
        cohort_provenance_ref,
    } = replay_plan;
    if profile != I3LocalnetOwnerReplyReplayProfile::ReplayKnownOwnerReplyOnVerifiedSuccessorSession
        || !first_session.peer_spki_verified()
        || !first_session.peer_preface_verified()
        || first_session.session_attempt_generation() != 1
        || cohort_provenance_ref.is_empty()
    {
        return Err(());
    }
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    let (reply, request_delivery) = first_session
        .receive_and_admit_generated_message(&mut runtime)
        .await
        .map_err(|_| ())?;
    let request_identity_ref = request_delivery.semantic_request_identity_ref().to_string();
    let first_request_receive = PrivateDeliveryEvidence::from(request_delivery);
    let (reply, mut owner_expiry) = match resolve_received_owner_reply(
        &mut runtime,
        reply,
        &request_identity_ref,
        control.owner_admission_drive_profile,
    )? {
        PrivateResolvedOwnerReply::Served(reply) => (reply, None),
        PrivateResolvedOwnerReply::DeclaredDeadlineExpired { reply, expiry } => {
            (reply, Some(expiry))
        }
    };
    if let Some(expiry) = owner_expiry.as_mut() {
        // The expiry producer record is joined only to this actual B receive;
        // it retains no host driver or clock input.
        expiry.request_receive = Some(Box::new(first_request_receive.clone()));
    }
    let owner_state_after_first = owner_reply_replay_owner_state(&runtime);
    if !owner_reply_replay_owner_state_is_exact(&owner_state_after_first, owner_expiry.is_some()) {
        return Err(());
    }
    let (first_reply_delivery, replay_candidate) = first_session
        .test_only_send_generated_reply_and_retain_replay_candidate(reply)
        .await
        .map_err(|_| ())?;
    first_session.finish_send().map_err(|_| ())?;
    let first_reply_send = PrivateDeliveryEvidence::from(first_reply_delivery);
    if first_request_receive.semantic_request_identity_ref != request_identity_ref
        || first_reply_send.semantic_request_identity_ref != request_identity_ref
        || first_reply_send.linked_request_identity_ref.as_deref() != Some(&request_identity_ref)
        || first_reply_send.network_occurrence_ref.is_empty()
        || first_reply_send.carrier_ref.is_empty()
    {
        return Err(());
    }
    // The accepted parent join requires A's emitted first-consumption record
    // before treating its close as this hand-off. A peer close alone can be
    // abnormal and proves no requester decision; no T0 label substitutes for
    // the independent A evidence.
    first_session.wait_for_peer_close().await;

    let (
        replay_session_generation,
        replay_session_peer_spki_verified,
        replay_session_preface_verified,
        replay_reply_send,
        receiver_rejection,
        replay_prewrite_rejection,
        owner_state_after_replay_rejection,
        token_original_first_reply_send_occurrence_ref,
        token_original_first_reply_carrier_ref,
    ) = match falsifier {
        Some(I3LocalnetOwnerReplyReplayFalsifier::ReplayOnInitialVerifiedSession) => {
            let error = match first_session
                .test_only_replay_retained_generated_reply(replay_candidate)
                .await
            {
                Err(error) => error,
                Ok(_) => return Err(()),
            };
            if !matches!(
                error,
                mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::LocalAttemptRejected
            ) {
                return Err(());
            }
            let after = owner_reply_replay_owner_state(&runtime);
            if after != owner_state_after_first {
                return Err(());
            }
            first_session.close();
            (
                None,
                None,
                None,
                None,
                None,
                Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected),
                after,
                first_reply_send.network_occurrence_ref.clone(),
                first_reply_send.carrier_ref.clone(),
            )
        }
        None => {
            first_session.close();
            let reconnect = first_session.into_reconnect();
            let mut replay_session = accept_reconnect_session(endpoint, reconnect).await?;
            replay_session.send_local_preface().await.map_err(|_| ())?;
            replay_session
                .receive_and_validate_peer_preface()
                .await
                .map_err(|_| ())?;
            if !replay_session.peer_spki_verified()
                || !replay_session.peer_preface_verified()
                || replay_session.session_attempt_generation() != 2
            {
                return Err(());
            }
            let replay_delivery = replay_session
                .test_only_replay_retained_generated_reply(replay_candidate)
                .await
                .map_err(|_| ())?;
            let original_send =
                PrivateDeliveryEvidence::from(replay_delivery.original_send_delivery().clone());
            let replay_reply_send =
                PrivateDeliveryEvidence::from(replay_delivery.replay_delivery().clone());
            if original_send != first_reply_send
                || !delivery_semantics_match(&first_reply_send, &replay_reply_send)
                || replay_reply_send.carrier_ref != first_reply_send.carrier_ref
                || replay_reply_send.candidate_commitment_ref
                    == first_reply_send.candidate_commitment_ref
                || replay_reply_send.network_occurrence_ref
                    == first_reply_send.network_occurrence_ref
            {
                return Err(());
            }
            replay_session.finish_send().map_err(|_| ())?;
            // A completes generic receiver admission/rejection before B
            // reports its post-replay owner state to the supervisor.
            replay_session.wait_for_peer_close().await;
            let after = owner_reply_replay_owner_state(&runtime);
            if after != owner_state_after_first {
                return Err(());
            }
            let generation = replay_session.session_attempt_generation();
            let peer_spki_verified = replay_session.peer_spki_verified();
            let preface_verified = replay_session.peer_preface_verified();
            replay_session.close();
            (
                Some(generation),
                Some(peer_spki_verified),
                Some(preface_verified),
                Some(replay_reply_send),
                None,
                None,
                after,
                original_send.network_occurrence_ref,
                original_send.carrier_ref,
            )
        }
    };
    let summary = runtime.observer_safe_runtime_summary();
    emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
        slot: I3LocalnetChildSlot::ProcessB,
        request_identity_ref: Some(request_identity_ref),
        requester_observation: None,
        requester_pending_request_is_retained: None,
        requester_local_wait: None,
        remote_admission: None,
        semantic_admission_count: 1,
        owner_mutation_count: summary.actual_owner_write_count(),
        retry_evidence: None,
        owner_reply_replay_evidence: Some(PrivateChildOwnerReplyReplayEvidence {
            run_ref,
            cohort_provenance_ref,
            first_session_generation: 1,
            first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified,
            replay_session_generation,
            replay_session_peer_spki_verified,
            replay_session_reciprocal_preface_verified: replay_session_preface_verified,
            first_request_receive: Some(first_request_receive),
            first_reply_send: Some(first_reply_send),
            token_original_first_reply_send_occurrence_ref: Some(
                token_original_first_reply_send_occurrence_ref,
            ),
            token_original_first_reply_carrier_ref: Some(token_original_first_reply_carrier_ref),
            owner_state_after_first: Some(owner_state_after_first),
            owner_state_after_replay_rejection: Some(owner_state_after_replay_rejection),
            owner_expiry,
            replay_reply_send,
            receiver_rejection,
            replay_prewrite_rejection,
            ..PrivateChildOwnerReplyReplayEvidence::default()
        }),
    })
    .map_err(|_| ())?;
    Ok(())
}

async fn run_client_owner_reply_replay_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    endpoint_address: SocketAddr,
    replay_plan: PrivateOwnerReplyReplayClientSessionPlan,
) -> Result<(), ()> {
    let PrivateOwnerReplyReplayClientSessionPlan {
        profile,
        falsifier,
        run_ref,
        cohort_provenance_ref,
    } = replay_plan;
    if profile != I3LocalnetOwnerReplyReplayProfile::ReplayKnownOwnerReplyOnVerifiedSuccessorSession
        || !first_session.peer_spki_verified()
        || !first_session.peer_preface_verified()
        || first_session.session_attempt_generation() != 1
        || cohort_provenance_ref.is_empty()
    {
        return Err(());
    }
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    let request = runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(|_| ())?;
    let request_identity_ref = request.semantic_request_identity_ref().to_string();
    let request_delivery = first_session
        .send_generated_message(request)
        .await
        .map_err(|_| ())?;
    first_session.finish_send().map_err(|_| ())?;
    let (receipt, first_reply_delivery) = first_session
        .receive_and_admit_generated_message(&mut runtime)
        .await
        .map_err(|_| ())?;
    let receipt = receipt.ok_or(())?;
    let terminal_failure_consumed = receipt.is_observer_safe_terminal_failure_consumed()
        && receipt.has_no_transportable_carrier();
    if !terminal_failure_consumed
        && (!receipt.is_observer_safe_typed_result_or_receipt()
            || !receipt.has_no_transportable_carrier())
    {
        return Err(());
    }
    let first_outcome = if terminal_failure_consumed {
        PrivateOwnerReplyReplayFirstOutcome::TerminalFailureConsumed
    } else {
        PrivateOwnerReplyReplayFirstOutcome::ReceiptConsumed
    };
    let requester_state_after_first =
        owner_reply_replay_requester_state(&runtime, &request_identity_ref);
    if !owner_reply_replay_requester_state_is_exact(
        &requester_state_after_first,
        terminal_failure_consumed,
    ) {
        return Err(());
    }
    let first_request_send = PrivateDeliveryEvidence::from(request_delivery);
    let first_reply_receive = PrivateDeliveryEvidence::from(first_reply_delivery);
    if first_request_send.semantic_request_identity_ref != request_identity_ref
        || first_reply_receive.semantic_request_identity_ref != request_identity_ref
        || first_reply_receive.linked_request_identity_ref.as_deref() != Some(&request_identity_ref)
    {
        return Err(());
    }
    // This close occurs only after generic admission has consumed A's first
    // receipt/terminal. B's session-one wait therefore cannot advance before
    // the known first decision exists in A's actual runtime.
    first_session.close();

    let (
        replay_session_generation,
        replay_session_peer_spki_verified,
        replay_session_preface_verified,
        receiver_rejection,
        requester_final_state,
    ) = match falsifier {
        Some(I3LocalnetOwnerReplyReplayFalsifier::ReplayOnInitialVerifiedSession) => (
            None,
            None,
            None,
            None,
            owner_reply_replay_requester_state(&runtime, &request_identity_ref),
        ),
        None => {
            let reconnect = first_session.into_reconnect();
            let mut replay_session =
                connect_reconnect_session(endpoint, endpoint_address, reconnect).await?;
            replay_session.send_local_preface().await.map_err(|_| ())?;
            replay_session
                .receive_and_validate_peer_preface()
                .await
                .map_err(|_| ())?;
            if !replay_session.peer_spki_verified()
                || !replay_session.peer_preface_verified()
                || replay_session.session_attempt_generation() != 2
            {
                return Err(());
            }
            let error = match replay_session
                .receive_and_admit_generated_message(&mut runtime)
                .await
            {
                Err(error) => error,
                Ok(_) => return Err(()),
            };
            let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt,
            } = error
            else {
                return Err(());
            };
            if error.kind()
                    != mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
                {
                    return Err(());
                }
            let after = owner_reply_replay_requester_state(&runtime, &request_identity_ref);
            if after != requester_state_after_first {
                return Err(());
            }
            let generation = replay_session.session_attempt_generation();
            let peer_spki_verified = replay_session.peer_spki_verified();
            let preface_verified = replay_session.peer_preface_verified();
            replay_session.close();
            (
                Some(generation),
                Some(peer_spki_verified),
                Some(preface_verified),
                Some(
                    PrivateOwnerReplyReplayReceiverRejection::CarrierAdmissionRejected {
                        rejected_candidate_commitment_ref: rejected_attempt
                            .candidate_commitment_ref()
                            .to_string(),
                        network_occurrence_ref: rejected_attempt
                            .network_occurrence_ref()
                            .to_string(),
                    },
                ),
                after,
            )
        }
    };
    if requester_final_state != requester_state_after_first {
        return Err(());
    }
    let summary = runtime.observer_safe_runtime_summary();
    emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
        slot: I3LocalnetChildSlot::ProcessA,
        request_identity_ref: Some(request_identity_ref),
        requester_observation: None,
        requester_pending_request_is_retained: None,
        requester_local_wait: None,
        remote_admission: None,
        semantic_admission_count: 1,
        owner_mutation_count: 0,
        retry_evidence: None,
        owner_reply_replay_evidence: Some(PrivateChildOwnerReplyReplayEvidence {
            run_ref,
            cohort_provenance_ref,
            first_session_generation: 1,
            first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified,
            replay_session_generation,
            replay_session_peer_spki_verified,
            replay_session_reciprocal_preface_verified: replay_session_preface_verified,
            first_request_send: Some(first_request_send),
            first_reply_receive: Some(first_reply_receive),
            requester_first_outcome: Some(first_outcome),
            requester_state_after_first: Some(requester_state_after_first),
            requester_final_state: Some(requester_final_state),
            receiver_rejection,
            ..PrivateChildOwnerReplyReplayEvidence::default()
        }),
    })
    .map_err(|_| ())?;
    if summary.accepted_inbound_receipt_count()
        + summary.accepted_inbound_declared_owner_failure_count()
        != 1
    {
        return Err(());
    }
    Ok(())
}

fn owner_reply_replay_requester_state(
    runtime: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    request_identity_ref: &str,
) -> PrivateOwnerReplyReplayRequesterState {
    let summary = runtime.observer_safe_runtime_summary();
    let occurrences = runtime.observer_safe_semantic_occurrences();
    PrivateOwnerReplyReplayRequesterState {
        pending_request_count: runtime.observer_safe_pending_owner_request_count(),
        receipt_count: summary.accepted_inbound_receipt_count(),
        terminal_failure_count: summary.accepted_inbound_declared_owner_failure_count(),
        receipt_occurrence_ref: occurrences
            .requester_local_receipt_occurrence_ref(request_identity_ref)
            .map(str::to_string),
        terminal_failure_occurrence_ref: occurrences
            .requester_terminal_declared_owner_failure_occurrence_ref(request_identity_ref)
            .map(str::to_string),
    }
}

fn owner_reply_replay_requester_state_is_exact(
    state: &PrivateOwnerReplyReplayRequesterState,
    terminal_failure_consumed: bool,
) -> bool {
    state.pending_request_count == 0
        && if terminal_failure_consumed {
            state.receipt_count == 0
                && state.terminal_failure_count == 1
                && state.receipt_occurrence_ref.is_none()
                && state
                    .terminal_failure_occurrence_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.is_empty())
        } else {
            state.receipt_count == 1
                && state.terminal_failure_count == 0
                && state.terminal_failure_occurrence_ref.is_none()
                && state
                    .receipt_occurrence_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.is_empty())
        }
}

fn owner_reply_replay_owner_state(
    runtime: &mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
) -> PrivateOwnerReplyReplayOwnerState {
    let summary = runtime.observer_safe_runtime_summary();
    let owner_admission = runtime.observer_safe_owner_admission_summary();
    PrivateOwnerReplyReplayOwnerState {
        tombstone_count: runtime.observer_safe_inbound_owner_request_tombstone_count(),
        served_owner_request_count: summary.served_owner_request_count(),
        owner_mutation_count: summary.actual_owner_write_count(),
        expired_owner_admission_count: owner_admission.expired_count(),
    }
}

fn owner_reply_replay_owner_state_is_exact(
    state: &PrivateOwnerReplyReplayOwnerState,
    declared_expiry: bool,
) -> bool {
    state.tombstone_count == 1
        && if declared_expiry {
            state.served_owner_request_count == 0
                && state.owner_mutation_count == 0
                && state.expired_owner_admission_count == 1
        } else {
            state.served_owner_request_count == 1
                && state.owner_mutation_count == 1
                && state.expired_owner_admission_count == 0
        }
}

/// Immutable actual-child inputs for one held session-one ingress. The source
/// request is emitted once on the first verified session; the second session
/// is receive-only from A's perspective.
struct PrivateLateIngressClientSessionPlan {
    profile: I3LocalnetLateIngressProfile,
    run_ref: String,
    assigned_loci: Vec<String>,
    late_ingress_falsifier: Option<I3LocalnetLateIngressFalsifier>,
    tainted_owner_lifecycle_ack_candidate: Option<Vec<u8>>,
}

async fn run_client_late_ingress_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    endpoint_address: SocketAddr,
    late_plan: PrivateLateIngressClientSessionPlan,
) -> Result<(), ()> {
    let PrivateLateIngressClientSessionPlan {
        profile,
        run_ref,
        assigned_loci,
        late_ingress_falsifier,
        tainted_owner_lifecycle_ack_candidate,
    } = late_plan;
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    if !first_session_peer_spki_verified
        || !first_session_reciprocal_preface_verified
        || first_session.session_attempt_generation() != 1
    {
        return Err(());
    }
    let request = runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(|_| ())?;
    let pending = runtime
        .into_original_owner_request_pending(request)
        .map_err(|_| ())?;
    let request_identity_ref = pending.semantic_request_identity_ref().to_string();
    let initial_sender_delivery = first_session
        .send_initial_original_owner_request(&mut runtime, &pending)
        .await
        .map_err(|_| ())?;
    first_session.finish_send().map_err(|_| ())?;
    // `finish_send` only closes A's local stream half. B closes session one
    // only after its adapter has retained the complete frame, which makes the
    // following peer-close wait the actual hand-off rather than a guessed
    // carrier delivery point.
    first_session.wait_for_peer_close().await;
    // This close follows the actual B-side retained-ingress hand-off.
    // `into_reconnect` preserves the checked peer/preface context but neither
    // exposes nor replays the source request.
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut reconnect_session =
        connect_reconnect_session(endpoint, endpoint_address, reconnect).await?;
    reconnect_session
        .send_local_preface()
        .await
        .map_err(|_| ())?;
    reconnect_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    if !reconnect_session.peer_spki_verified()
        || !reconnect_session.peer_preface_verified()
        || reconnect_session.session_attempt_generation() != 2
    {
        return Err(());
    }

    let outcome = reconnect_session
        .receive_and_admit_original_owner_reply(&mut runtime, pending)
        .await;
    let (
        terminal_outcome,
        requester_outcome,
        receipt_count,
        semantic_admission_count,
        reply_received,
        retained_pending,
    ) =
        match profile {
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect => {
                let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed {
                    receipt,
                    delivery,
                } = outcome
                else {
                    return Err(());
                };
                if !receipt.is_observer_safe_typed_result_or_receipt()
                    || !receipt.has_no_transportable_carrier()
                    || runtime.observer_safe_pending_owner_request_count() != 0
                {
                    return Err(());
                }
                (
                    PrivateLateIngressTerminalOutcome::Completed,
                    I3LocalnetLateIngressRequesterOutcome::ReceiptConsumed,
                    1,
                    1,
                    Some(PrivateDeliveryEvidence::from(delivery)),
                    None,
                )
            }
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission => {
                let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
                    pending: returned_pending,
                    // The session-two preface was independently verified
                    // above. B then finishes its preface-only stream after
                    // the actual carrier rejection, so this is the adapter's
                    // observed no-reply frame outcome, not a profile claim.
                    error: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::FrameRejected,
                } = outcome
                else {
                    return Err(());
                };
                if returned_pending.semantic_request_identity_ref() != request_identity_ref
                    || runtime.observer_safe_pending_owner_request_count() != 1
                {
                    return Err(());
                }
                (
                    PrivateLateIngressTerminalOutcome::HandledDeliveryFault,
                    I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved,
                    0,
                    0,
                    None,
                    Some(returned_pending),
                )
            }
        };
    let summary = runtime.observer_safe_runtime_summary();
    let occurrences = runtime.observer_safe_semantic_occurrences();
    let observer_evidence = PrivateChildObserverEvidence {
        request_sent: Some(initial_sender_delivery.clone().into()),
        reply_received,
        local_store_ref: runtime.local_store_identity_ref().to_string(),
        requester_receipt_occurrence_ref: occurrences
            .requester_local_receipt_occurrence_ref(&request_identity_ref)
            .map(str::to_string),
        ..PrivateChildObserverEvidence::default()
    };
    let trusted_control_consumed = late_ingress_falsifier
        != Some(I3LocalnetLateIngressFalsifier::ClearRequesterLateIngressTerminalTrustedControl);
    emit_child_event(&PrivateChildEvent::LateIngress {
        slot: I3LocalnetChildSlot::ProcessA,
        terminal_outcome,
        exec_confirmed: true,
        assigned_loci,
        trusted_control_consumed,
        tainted_image_consumed: true,
        tls_peer_verified: reconnect_session.peer_spki_verified(),
        reciprocal_preface_verified: reconnect_session.peer_preface_verified(),
        reliable_bidi_stream_count: reconnect_session.reliable_bidi_stream_count(),
        quic_datagrams_enabled: reconnect_session.quic_datagrams_enabled(),
        semantic_admission_count,
        unauthenticated_semantic_admission_count: 0,
        network_receipt_frame_count: 0,
        generated_request_count: 1,
        served_count: 0,
        write_count: 0,
        reply_count: 0,
        receipt_count: summary.accepted_inbound_receipt_count(),
        runtime_occurrence_count: occurrences.requester_local_receipt_count(),
        observer_evidence,
        late_ingress_evidence: PrivateChildLateIngressEvidence {
            run_ref,
            first_session_generation: 1,
            reconnect_session_generation: reconnect_session.session_attempt_generation(),
            first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified,
            reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
            reconnect_session_reciprocal_preface_verified: reconnect_session
                .peer_preface_verified(),
            initial_sender_delivery: Some(initial_sender_delivery.into()),
            requester_outcome: Some(requester_outcome),
            outbound_request_frame_write_count: Some(1),
            tainted_owner_lifecycle_ack_candidate,
            ..PrivateChildLateIngressEvidence::default()
        },
    })
    .map_err(|_| ())?;
    if summary.accepted_inbound_receipt_count() != receipt_count {
        return Err(());
    }
    // The opaque pending handle is intentionally kept to this point in the
    // G2 case: no
    // second send or local consumption occurs after the actual pending reply
    // outcome. It then drops with the child runtime, not as a new request.
    let _ = retained_pending;
    reconnect_session.close();
    Ok(())
}

async fn run_client_retry_sessions(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    mut first_session: mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicSession,
    endpoint: &Endpoint,
    endpoint_address: SocketAddr,
    control: &PrivateChildControl,
    retry_plan: PrivateRetryClientSessionPlan,
) -> Result<(), ()> {
    let PrivateRetryClientSessionPlan {
        profile,
        run_ref,
        assigned_loci,
    } = retry_plan;
    let first_session_peer_spki_verified = first_session.peer_spki_verified();
    let first_session_reciprocal_preface_verified = first_session.peer_preface_verified();
    if !first_session_peer_spki_verified
        || !first_session_reciprocal_preface_verified
        || first_session.session_attempt_generation() != 1
    {
        return Err(());
    }
    let request = runtime
        .emit_generated_owner_request("init_avatar_hp")
        .map_err(|_| ())?;
    let mut pending = runtime
        .into_original_owner_request_pending(request)
        .map_err(|_| ())?;
    let request_identity_ref = pending.semantic_request_identity_ref().to_string();

    if control.retry_falsifier == Some(I3LocalnetRetryFalsifier::RetryOnInitialVerifiedSession) {
        let error = match first_session
            .retry_original_owner_request_after_reconnect(&mut runtime, &pending)
            .await
        {
            Err(error) => error,
            Ok(_) => return Err(()),
        };
        let adapter_rejection_kind = match error {
            mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::LocalAttemptRejected => {
                I3LocalnetAdapterRejectionKind::LocalAttemptRejected
            }
            mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicError::PeerBindingRejected(_) => {
                I3LocalnetAdapterRejectionKind::PeerBindingRejected
            }
            _ => return Err(()),
        };
        let evidence = PrivateChildRejectionEvidence {
            requester_pending_request_is_retained: runtime
                .observer_safe_pending_owner_request_count()
                == 1,
            adapter_rejection_kind: Some(adapter_rejection_kind),
            requester_first_session_peer_spki_verified: first_session_peer_spki_verified,
            requester_first_session_reciprocal_preface_verified:
                first_session_reciprocal_preface_verified,
            ..PrivateChildRejectionEvidence::default()
        };
        emit_child_event(&PrivateChildEvent::rejected_with_evidence(
            PrivateChildRejection::Lifecycle,
            true,
            0,
            1,
            1,
            0,
            0,
            evidence,
        ))
        .map_err(|_| ())?;
        first_session.close();
        return Ok(());
    }

    let (initial_request_delivery, initial_attempt) = match profile {
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => (None, None),
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
            let delivery = first_session
                .send_initial_original_owner_request(&mut runtime, &pending)
                .await
                .map_err(|_| ())?;
            first_session.finish_send().map_err(|_| ())?;
            let attempt = retry_attempt_snapshot(&runtime, &request_identity_ref).ok_or(())?;
            (Some(PrivateDeliveryEvidence::from(delivery)), Some(attempt))
        }
    };
    if profile == I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply {
        // Do not infer remote admission from the schedule: wait until the
        // owner itself closes the first session without a reply, retaining
        // the opaque pending handle returned by the actual adapter outcome.
        let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
            pending: returned_pending,
            error: _,
        } = first_session
            .receive_and_admit_original_owner_reply(&mut runtime, pending)
            .await
        else {
            return Err(());
        };
        pending = returned_pending;
    }
    first_session.close();
    let reconnect = first_session.into_reconnect();
    let mut reconnect_session =
        connect_reconnect_session(endpoint, endpoint_address, reconnect).await?;
    reconnect_session
        .send_local_preface()
        .await
        .map_err(|_| ())?;
    reconnect_session
        .receive_and_validate_peer_preface()
        .await
        .map_err(|_| ())?;
    if !reconnect_session.peer_spki_verified()
        || !reconnect_session.peer_preface_verified()
        || reconnect_session.session_attempt_generation() != 2
    {
        return Err(());
    }
    let reconnect_delivery = reconnect_session
        .retry_original_owner_request_after_reconnect(&mut runtime, &pending)
        .await
        .map_err(|_| ())?;
    reconnect_session.finish_send().map_err(|_| ())?;
    let mut reconnect_attempt = retry_attempt_snapshot(&runtime, &request_identity_ref).ok_or(())?;
    let mut reconnect_observer_delivery: PrivateDeliveryEvidence = reconnect_delivery.into();
    if profile == I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply {
        match control.retry_audit_falsifier {
            Some(I3LocalnetRetryAuditFalsifier::MutateReconnectSenderCandidateCommitment) => {
                reconnect_observer_delivery.candidate_commitment_ref =
                    "observer-falsifier:reconnect-commitment".to_string();
            }
            Some(I3LocalnetRetryAuditFalsifier::ClearReconnectSenderCandidateCommitment) => {
                reconnect_observer_delivery.candidate_commitment_ref.clear();
            }
            Some(
                I3LocalnetRetryAuditFalsifier::UseInitialSessionCandidateCommitmentForReconnect,
            ) => {
                reconnect_observer_delivery.candidate_commitment_ref = initial_request_delivery
                    .as_ref()
                    .map(|delivery| delivery.candidate_commitment_ref.clone())
                    .ok_or(())?;
            }
            Some(I3LocalnetRetryAuditFalsifier::MutateReconnectRequesterBindingRef) => {
                reconnect_attempt.requester_binding_ref =
                    "observer-falsifier:reconnect-requester-binding".to_string();
            }
            None => {}
        }
    }
    match profile {
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => {
            let outcome = reconnect_session
                .receive_and_admit_original_owner_reply(&mut runtime, pending)
                .await;
            let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed {
                receipt,
                delivery: reply_delivery,
            } = outcome
            else {
                return Err(());
            };
            if !receipt.is_observer_safe_typed_result_or_receipt()
                || !receipt.has_no_transportable_carrier()
            {
                return Err(());
            }
            let summary = runtime.observer_safe_runtime_summary();
            let occurrences = runtime.observer_safe_semantic_occurrences();
            let retry_evidence = PrivateChildRetryEvidence {
                run_ref,
                first_session_generation: 1,
                reconnect_session_generation: reconnect_session.session_attempt_generation(),
                first_session_peer_spki_verified,
                first_session_reciprocal_preface_verified,
                reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                reconnect_session_reciprocal_preface_verified: reconnect_session
                    .peer_preface_verified(),
                reconnect_session_runtime_attempt: Some(reconnect_attempt),
                reconnect_request_delivery: Some(reconnect_observer_delivery),
                ..PrivateChildRetryEvidence::default()
            };
            let observer_evidence = PrivateChildObserverEvidence {
                request_sent: retry_evidence.reconnect_request_delivery.clone(),
                reply_received: Some(reply_delivery.into()),
                local_store_ref: runtime.local_store_identity_ref().to_string(),
                requester_receipt_occurrence_ref: occurrences
                    .requester_local_receipt_occurrence_ref(&request_identity_ref)
                    .map(str::to_string),
                retry_evidence: Some(retry_evidence),
                ..PrivateChildObserverEvidence::default()
            };
            emit_child_event(&PrivateChildEvent::Completed {
                slot: I3LocalnetChildSlot::ProcessA,
                exec_confirmed: true,
                assigned_loci,
                trusted_control_consumed: true,
                tainted_image_consumed: true,
                tls_peer_verified: reconnect_session.peer_spki_verified(),
                reciprocal_preface_verified: reconnect_session.peer_preface_verified(),
                reliable_bidi_stream_count: reconnect_session.reliable_bidi_stream_count(),
                quic_datagrams_enabled: reconnect_session.quic_datagrams_enabled(),
                semantic_admission_count: 1,
                unauthenticated_semantic_admission_count: 0,
                network_receipt_frame_count: 0,
                generated_request_count: 1,
                served_count: 0,
                write_count: 0,
                reply_count: 0,
                receipt_count: summary.accepted_inbound_receipt_count(),
                runtime_occurrence_count: occurrences.requester_local_receipt_count(),
                observer_evidence,
            })
            .map_err(|_| ())?;
        }
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
            let outcome = reconnect_session
                .receive_and_admit_original_owner_reply(&mut runtime, pending)
                .await;
            let mir_runtime::sys5_i3_private_quic::Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
                pending,
                error: _,
            } = outcome
            else {
                return Err(());
            };
            if runtime.observer_safe_pending_owner_request_count() != 1
                || pending.semantic_request_identity_ref() != request_identity_ref
            {
                return Err(());
            }
            let retry_evidence = PrivateChildRetryEvidence {
                run_ref,
                first_session_generation: 1,
                reconnect_session_generation: reconnect_session.session_attempt_generation(),
                first_session_peer_spki_verified,
                first_session_reciprocal_preface_verified,
                reconnect_session_peer_spki_verified: reconnect_session.peer_spki_verified(),
                reconnect_session_reciprocal_preface_verified: reconnect_session
                    .peer_preface_verified(),
                requester_pending_request_is_retained: true,
                first_session_runtime_attempt: initial_attempt,
                reconnect_session_runtime_attempt: Some(reconnect_attempt),
                initial_request_delivery,
                reconnect_request_delivery: Some(reconnect_observer_delivery),
                ..PrivateChildRetryEvidence::default()
            };
            emit_child_event(&PrivateChildEvent::HandledDeliveryFault {
                slot: I3LocalnetChildSlot::ProcessA,
                request_identity_ref: Some(request_identity_ref),
                requester_observation: Some(
                    I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved,
                ),
                requester_pending_request_is_retained: None,
                requester_local_wait: None,
                remote_admission: None,
                semantic_admission_count: 0,
                owner_mutation_count: 0,
                retry_evidence: Some(retry_evidence),
                owner_reply_replay_evidence: None,
            })
            .map_err(|_| ())?;
        }
    }
    reconnect_session.close();
    Ok(())
}

fn install_ring() -> Result<(), ()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| ())
}

fn server_config(
    control: &PrivateChildControl,
) -> Result<(quinn::ServerConfig, PrivateQuicTransportEvidence), ()> {
    server_config_from_transport_material(
        &control.ca_der,
        &control.leaf_cert_der,
        control.leaf_key_der.as_ref(),
    )
}

fn server_config_from_transport_material(
    ca_der: &[u8],
    leaf_cert_der: &[u8],
    leaf_key_der: &[u8],
) -> Result<(quinn::ServerConfig, PrivateQuicTransportEvidence), ()> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(ca_der.to_vec()))
        .map_err(|_| ())?;
    let verifier = WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .map_err(|_| ())?;
    let mut crypto = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from(leaf_cert_der.to_vec())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(leaf_key_der.to_vec())),
        )
        .map_err(|_| ())?;
    crypto.alpn_protocols = vec![PRIVATE_LOCALNET_ALPN.to_vec()];
    let mut configuration = quinn::ServerConfig::with_crypto(Arc::new(
        QuicServerConfig::try_from(crypto).map_err(|_| ())?,
    ));
    let transport = Arc::get_mut(&mut configuration.transport).ok_or(())?;
    let evidence = PrivateQuicTransportEvidence {
        datagram_receive_enabled: false,
        datagram_send_enabled: false,
    };
    transport
        .max_concurrent_bidi_streams(1_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    Ok((configuration, evidence))
}

fn client_config(
    control: &PrivateChildControl,
) -> Result<(quinn::ClientConfig, PrivateQuicTransportEvidence), ()> {
    client_config_from_transport_material(
        &control.ca_der,
        &control.leaf_cert_der,
        control.leaf_key_der.as_ref(),
    )
}

fn client_config_from_transport_material(
    ca_der: &[u8],
    leaf_cert_der: &[u8],
    leaf_key_der: &[u8],
) -> Result<(quinn::ClientConfig, PrivateQuicTransportEvidence), ()> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(ca_der.to_vec()))
        .map_err(|_| ())?;
    let mut crypto = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(
            vec![CertificateDer::from(leaf_cert_der.to_vec())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(leaf_key_der.to_vec())),
        )
        .map_err(|_| ())?;
    crypto.alpn_protocols = vec![PRIVATE_LOCALNET_ALPN.to_vec()];
    let mut configuration = quinn::ClientConfig::new(Arc::new(
        QuicClientConfig::try_from(crypto).map_err(|_| ())?,
    ));
    let mut transport = quinn::TransportConfig::default();
    let evidence = PrivateQuicTransportEvidence {
        datagram_receive_enabled: false,
        datagram_send_enabled: false,
    };
    transport
        .max_concurrent_bidi_streams(0_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    configuration.transport_config(Arc::new(transport));
    Ok((configuration, evidence))
}

fn emit_child_event(event: &PrivateChildEvent) -> io::Result<()> {
    serde_json::to_writer(io::stdout(), event).map_err(io::Error::other)?;
    io::stdout().write_all(b"\n")?;
    io::stdout().flush()
}

#[cfg(test)]
mod lifecycle_evidence_tests {
    use super::*;

    fn completed_owner_terminal() -> PrivateChildEvent {
        PrivateChildEvent::Completed {
            slot: I3LocalnetChildSlot::ProcessB,
            exec_confirmed: true,
            assigned_loci: I3LocalnetChildSlot::ProcessB
                .assigned_loci()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            trusted_control_consumed: true,
            tainted_image_consumed: true,
            tls_peer_verified: true,
            reciprocal_preface_verified: true,
            reliable_bidi_stream_count: 1,
            quic_datagrams_enabled: false,
            semantic_admission_count: 1,
            unauthenticated_semantic_admission_count: 0,
            network_receipt_frame_count: 1,
            generated_request_count: 0,
            served_count: 1,
            write_count: 0,
            reply_count: 0,
            receipt_count: 0,
            runtime_occurrence_count: 1,
            observer_evidence: PrivateChildObserverEvidence::default(),
        }
    }

    fn known_owner_terminal_error(owner_terminal: PrivateChildEvent) -> I3LocalnetRunError {
        let terminal_event = owner_terminal
            .terminal_event()
            .expect("the classifier accepts only a known owner terminal");
        unexpected_owner_terminal_failure(owner_terminal).into_error_with_terminal_events(
            true,
            vec![terminal_event],
            0,
            false,
            false,
            false,
            2,
            Duration::ZERO,
            Duration::ZERO,
            true,
            Duration::ZERO,
            None,
            None,
            None,
        )
    }

    #[test]
    fn unexpected_owner_terminals_retain_start_and_counts_when_admitted_without_mutation() {
        let cases = [
            (
                completed_owner_terminal(),
                I3LocalnetChildTerminalOutcome::Completed,
            ),
            (
                PrivateChildEvent::HandledDeliveryFault {
                    slot: I3LocalnetChildSlot::ProcessB,
                    request_identity_ref: None,
                    requester_observation: None,
                    requester_pending_request_is_retained: None,
                    requester_local_wait: None,
                    remote_admission: None,
                    semantic_admission_count: 1,
                    owner_mutation_count: 0,
                    retry_evidence: None,
                    owner_reply_replay_evidence: None,
                },
                I3LocalnetChildTerminalOutcome::HandledDeliveryFault,
            ),
            (
                PrivateChildEvent::rejected(PrivateChildRejection::Lifecycle, true, 1, 1, 1, 1, 0),
                I3LocalnetChildTerminalOutcome::Rejected,
            ),
        ];

        for (owner_terminal, expected_outcome) in cases {
            let error = known_owner_terminal_error(owner_terminal);
            assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
            let audit = error.rejection_audit();
            assert_eq!(audit.stage(), I3LocalnetFailureStage::AfterRemoteAdmission);
            assert_eq!(audit.child_owner_starts(), 1);
            assert_eq!(audit.semantic_admission_count(), 1);
            assert_eq!(audit.owner_mutation_count(), 0);
            assert_eq!(audit.child_terminal_event_count(), 1);
            let retained = &audit.child_terminal_events()[0];
            assert_eq!(retained.outcome(), expected_outcome);
            assert_eq!(retained.semantic_admission_count(), 1);
            assert_eq!(retained.owner_mutation_count(), 0);
        }
    }

    #[test]
    fn missing_post_ready_owner_terminal_is_lifecycle_evidence_rejected_after_owner_start() {
        let error = LocalnetFailure::lifecycle_evidence_rejected()
            .after_observed_owner_runtime_start()
            .into_error(true);

        assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
        let audit = error.rejection_audit();
        assert_eq!(
            audit.stage(),
            I3LocalnetFailureStage::LifecycleEvidenceRejected
        );
        assert_ne!(audit.stage(), I3LocalnetFailureStage::BeforeOwnerStart);
        assert_eq!(audit.child_owner_starts(), 1);
        assert_eq!(audit.semantic_admission_count(), 0);
        assert_eq!(audit.owner_mutation_count(), 0);
        assert_eq!(audit.child_terminal_event_count(), 0);
    }
}
