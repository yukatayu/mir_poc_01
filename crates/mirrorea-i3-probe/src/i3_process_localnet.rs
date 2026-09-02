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
        fd::{AsRawFd, FromRawFd},
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
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3PrivateProcessCodec, Sys5I3ProcessCohort,
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

const PROCESS_A_SLOT: &str = "process-a";
const PROCESS_B_SLOT: &str = "process-b";
const PROCESS_A_LOCI: [&str; 2] = ["ParticipantA", "ViewerC"];
const PROCESS_B_LOCI: [&str; 2] = ["WorldAuthority", "ParticipantB"];
const ACTIVE_I2_LOGICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const LOCALNET_CONTROL_FD: i32 = 3;
const MAX_TRUSTED_CONTROL_BYTES: usize = 512 * 1024;
const MAX_CHILD_EVENT_BYTES: usize = 64 * 1024;
const PRIVATE_LOCALNET_ALPN: &[u8] = b"mirrorea-i3-process-localnet-v1";
// The reaper reserve is part of the finite lifecycle allowance: it leaves
// room to observe a post-kill exit without exceeding the caller's total main
// deadline plus reaper allowance under suite load.
const LIFECYCLE_REAP_RESERVE: Duration = Duration::from_millis(100);

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
}

/// The finite child report outcome retained even when the aggregate run fails.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetChildTerminalOutcome {
    Completed,
    Rejected,
}

/// Observer-safe counters from one terminal child report.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetChildTerminalEvent {
    outcome: I3LocalnetChildTerminalOutcome,
    semantic_admission_count: usize,
    owner_mutation_count: usize,
    observed_exit_status_code: Option<i32>,
    was_force_killed: bool,
}

impl I3LocalnetChildTerminalEvent {
    pub const fn outcome(&self) -> I3LocalnetChildTerminalOutcome {
        self.outcome
    }

    pub const fn semantic_admission_count(&self) -> usize {
        self.semantic_admission_count
    }

    pub const fn owner_mutation_count(&self) -> usize {
        self.owner_mutation_count
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
}

impl I3ProcessLocalnetRequest {
    pub fn from_ordinary_source_path(path: impl Into<PathBuf>) -> Self {
        Self {
            ordinary_source_path: path.into(),
            deadline: Duration::from_secs(15),
            reaper_allowance: Duration::from_secs(1),
            falsifier: None,
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
    fixed_child_control_descriptors_preserved: bool,
    child_owner_starts: usize,
    quic_certificate_initializations: usize,
    quic_handshake_count: usize,
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
    wrong_peer_ca_validated_leaf_ref: Option<String>,
    expected_peer_spki_ref: Option<String>,
    actual_peer_spki_ref: Option<String>,
    child_terminal_events: Vec<I3LocalnetChildTerminalEvent>,
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
    pub const fn fixed_child_control_descriptors_preserved(&self) -> bool {
        self.fixed_child_control_descriptors_preserved
    }
    pub const fn child_owner_starts(&self) -> usize {
        self.child_owner_starts
    }
    pub const fn quic_certificate_initializations(&self) -> usize {
        self.quic_certificate_initializations
    }
    pub const fn quic_handshake_count(&self) -> usize {
        self.quic_handshake_count
    }
    pub const fn semantic_admission_count(&self) -> usize {
        self.semantic_admission_count
    }
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
    pub fn wrong_peer_ca_validated_leaf_ref(&self) -> Option<&str> {
        self.wrong_peer_ca_validated_leaf_ref.as_deref()
    }
    pub fn expected_peer_spki_ref(&self) -> Option<&str> {
        self.expected_peer_spki_ref.as_deref()
    }
    pub fn actual_peer_spki_ref(&self) -> Option<&str> {
        self.actual_peer_spki_ref.as_deref()
    }
    pub fn child_terminal_events(&self) -> &[I3LocalnetChildTerminalEvent] {
        &self.child_terminal_events
    }
    pub fn child_terminal_event_count(&self) -> usize {
        self.child_terminal_events.len()
    }
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
}

impl I3LocalnetRunError {
    fn new(kind: I3LocalnetRunErrorKind, rejection_audit: I3LocalnetRejectionAudit) -> Self {
        Self {
            kind,
            rejection_audit,
        }
    }

    pub const fn kind(&self) -> I3LocalnetRunErrorKind {
        self.kind
    }
    pub fn rejection_audit(&self) -> I3LocalnetRejectionAudit {
        self.rejection_audit.clone()
    }
}

struct LocalnetFailure {
    kind: I3LocalnetRunErrorKind,
    stage: I3LocalnetFailureStage,
    child_rejection: Option<Box<PrivateChildEvent>>,
    evidence: LocalnetRejectionEvidence,
    lifecycle_rejection_cause: Option<I3LocalnetLifecycleRejectionCause>,
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
        }
    }

    fn lifecycle_with_cause(cause: I3LocalnetLifecycleRejectionCause) -> Self {
        let mut failure = Self::lifecycle();
        failure.lifecycle_rejection_cause = Some(cause);
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
            false,
            false,
            false,
            0,
            Duration::ZERO,
            Duration::ZERO,
            false,
            Duration::ZERO,
        )
    }

    #[allow(clippy::too_many_arguments)] // private audit preserves independent lifecycle observations.
    fn into_error_with_terminal_events(
        self,
        all_children_reaped: bool,
        child_terminal_events: Vec<I3LocalnetChildTerminalEvent>,
        completed_child_exited_nonzero: bool,
        completed_child_ignored_graceful_completion: bool,
        completed_child_was_force_killed_after_reaper_allowance: bool,
        spawned_child_count: usize,
        observed_supervised_process_lifecycle_elapsed: Duration,
        observed_supervised_process_lifecycle_bound: Duration,
        zero_exit_reap_observed_within_deadline: bool,
        captured_zero_exit_reap_observation_elapsed: Duration,
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
                fixed_control_descriptor_preserved,
                owner_start_count,
                certificate_initialization_count,
                handshake_count,
                _child_semantic_admission_count,
                _child_owner_mutation_count,
                true,
                child_evidence,
            ),
            // No child event means no evidence for an operational claim.
            // This path is used only for local pre-spawn/setup rejection.
            None | Some(_) => (
                false,
                0,
                0,
                0,
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
                structurally_valid_completed_child_report_observed: child_terminal_events
                    .iter()
                    .any(|event| event.outcome == I3LocalnetChildTerminalOutcome::Completed),
                no_orphan_child_pids: all_children_reaped,
                child_terminal_events,
                completed_child_exited_nonzero,
                completed_child_ignored_graceful_completion,
                completed_child_was_force_killed_after_reaper_allowance,
                spawned_child_count,
                observed_supervised_process_lifecycle_elapsed,
                observed_supervised_process_lifecycle_bound,
                zero_exit_reap_observed_within_deadline,
                captured_zero_exit_reap_observation_elapsed,
            },
        )
    }

    fn into_supervisor_error(
        mut self,
        supervisor: &LocalnetSupervisor,
        all_children_reaped: bool,
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
        self.into_error_with_terminal_events(
            all_children_reaped,
            supervisor.terminal_events(),
            completed_nonzero,
            completed_hung,
            completed_hung,
            supervisor.children.len(),
            supervisor.observed_lifecycle_elapsed(),
            supervisor.lifecycle_bound(),
            supervisor.zero_exit_reap_observed_within_deadline(),
            supervisor.captured_zero_exit_reap_observation_elapsed(),
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
    timeout_millis: u64,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        matches!(self, Self::Completed { .. } | Self::Rejected { .. })
    }

    fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    fn terminal_event(&self) -> Option<I3LocalnetChildTerminalEvent> {
        match self {
            Self::Completed {
                semantic_admission_count,
                write_count,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                outcome: I3LocalnetChildTerminalOutcome::Completed,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *write_count,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::Rejected {
                semantic_admission_count,
                owner_mutation_count,
                ..
            } => Some(I3LocalnetChildTerminalEvent {
                outcome: I3LocalnetChildTerminalOutcome::Rejected,
                semantic_admission_count: *semantic_admission_count,
                owner_mutation_count: *owner_mutation_count,
                observed_exit_status_code: None,
                was_force_killed: false,
            }),
            Self::Ready { .. } => None,
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
        let Self::Completed {
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
        } = self
        else {
            return None;
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
}

#[derive(Clone)]
struct SupervisorLineageEvidence {
    ordinary_source_build_count: usize,
    admission_count: usize,
    m9_generation_count: usize,
    request_contract: Sys5I3AdapterCarrierContract,
    reply_contract: Sys5I3AdapterCarrierContract,
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
    let first_binding = cohort
        .parent_held_expected_start_binding(PROCESS_A_SLOT)
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let second_binding = cohort
        .parent_held_expected_start_binding(PROCESS_B_SLOT)
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let first_image = codec
        .encode_image(
            cohort
                .take_process_image(PROCESS_A_SLOT)
                .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?,
        )
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let second_image = codec
        .encode_image(
            cohort
                .take_process_image(PROCESS_B_SLOT)
                .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?,
        )
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let lineage = SupervisorLineageEvidence {
        ordinary_source_build_count,
        admission_count: summary.full_admission_count(),
        m9_generation_count: summary.authority_generation_count(),
        request_contract: project_adapter_contract(&project, "owner-request")
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?,
        reply_contract: project_adapter_contract(&project, "owner-reply-receipt")
            .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?,
    };
    let credentials =
        generate_run_credentials().map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;
    let run_ref = fresh_run_ref(
        summary.cohort_occurrence_ref(),
        &credentials.process_a.spki_ref,
        &credentials.process_b.spki_ref,
    );
    let (first_control, second_control) = codec
        .split_trusted_localnet_controls(
            run_ref,
            first_binding,
            credentials.process_a.spki_ref.clone(),
            second_binding,
            credentials.process_b.spki_ref.clone(),
        )
        .map_err(|_| LocalnetFailure::lifecycle().into_error(true))?;

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
    };
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
                lineage,
            )
        }
    };
    match outcome {
        Ok(run) => Ok(run),
        Err(failure) => {
            let cleanup = supervisor.cleanup_after_failure();
            Err(failure.into_supervisor_error(&supervisor, cleanup))
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::result_large_err)] // preserves the private audit error across setup helpers.
fn run_positive_or_peer_falsifier(
    supervisor: &mut LocalnetSupervisor,
    codec: &Sys5I3PrivateProcessCodec,
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
    lineage: SupervisorLineageEvidence,
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
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    let process_b = supervisor
        .spawn(I3LocalnetChildSlot::ProcessB, second_image, server_control)
        .map_err(|_| LocalnetFailure::lifecycle())?;
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
        timeout_millis: deadline.as_millis().try_into().unwrap_or(u64::MAX),
    };
    let process_a = supervisor
        .spawn(I3LocalnetChildSlot::ProcessA, first_image, client_control)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let a_event = supervisor
        .next_event(process_a)
        .map_err(|_| LocalnetFailure::lifecycle())?;
    let b_event = supervisor
        .next_event(process_b)
        .map_err(|_| LocalnetFailure::lifecycle())?;
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
            rejection @ PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::PeerBinding,
                ..
            },
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
            rejection @ PrivateChildEvent::Rejected {
                rejection: PrivateChildRejection::StartBinding,
                ..
            },
        ) => {
            return Err(LocalnetFailure::from_child(
                I3LocalnetRunErrorKind::StartBindingRejected,
                I3LocalnetFailureStage::BeforeOwnerStart,
                rejection,
            ));
        }
        (a, b) => match (a.into_completed(), b.into_completed()) {
            (Some(a), Some(b))
                if a.slot == I3LocalnetChildSlot::ProcessA
                    && b.slot == I3LocalnetChildSlot::ProcessB =>
            {
                (a, b)
            }
            _ => return Err(LocalnetFailure::lifecycle()),
        },
    };
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
        joined_observer_evidence(&lineage, &a, &b).ok_or_else(LocalnetFailure::lifecycle)?;
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
        let child_fd = child_control.as_raw_fd();
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
                if libc::dup2(child_fd, LOCALNET_CONTROL_FD) == -1 {
                    return Err(io::Error::last_os_error());
                }
                if libc::fcntl(LOCALNET_CONTROL_FD, libc::F_SETFD, 0) == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command.spawn()?;
        drop(child_control);
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
    if path
        .to_string_lossy()
        .ends_with(ACTIVE_I2_LOGICAL_SOURCE_PATH)
    {
        ACTIVE_I2_LOGICAL_SOURCE_PATH
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
    let slot = match args.as_slice() {
        [value] if value == I3LocalnetChildSlot::ProcessA.child_arg() => {
            I3LocalnetChildSlot::ProcessA
        }
        [value] if value == I3LocalnetChildSlot::ProcessB.child_arg() => {
            I3LocalnetChildSlot::ProcessB
        }
        _ => return None,
    };
    Some(run_private_localnet_child(slot).is_ok())
}

fn run_private_localnet_child(fixed_slot: I3LocalnetChildSlot) -> Result<(), ()> {
    let result = run_private_localnet_child_inner(fixed_slot);
    if result.is_err() {
        let _ = emit_child_event(&PrivateChildEvent::rejected(
            PrivateChildRejection::Lifecycle,
            false,
            0,
            0,
            0,
            0,
            0,
        ));
    }
    result
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
    let (runtime, runtime_control) =
        match codec.validate_and_start_image_with_localnet_control(image, runtime_control) {
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
                run_server_child(runtime, runtime_control, control, duration, assigned_loci).await
            }
            I3LocalnetChildSlot::ProcessA => {
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

async fn run_server_child(
    mut runtime: mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime,
    trusted: mir_runtime::sys5_i3_process_runtime::Sys5I3TrustedLocalnetControl,
    control: PrivateChildControl,
    timeout: Duration,
    assigned_loci: Vec<String>,
) -> Result<(), ()> {
    if control.endpoint.is_some() {
        return Err(());
    }
    install_ring()?;
    let (server_config, _transport_evidence) = server_config(&control)?;
    let endpoint =
        Endpoint::server(server_config, SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    emit_child_event(&PrivateChildEvent::Ready {
        endpoint: endpoint.local_addr().map_err(|_| ())?.to_string(),
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
        let (reply, request_delivery) = session
            .receive_and_admit_generated_message(&mut runtime)
            .await
            .map_err(|_| ())?;
        let reply = reply.ok_or(())?;
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
    let mut endpoint = Endpoint::client(SocketAddr::from(([127, 0, 0, 1], 0))).map_err(|_| ())?;
    let (client_config, _transport_evidence) = client_config(&control)?;
    endpoint.set_default_client_config(client_config);
    let result = tokio::time::timeout(timeout, async {
        let prepared_request = if control.emit_request_before_wrong_peer_rejection {
            let request = runtime
                .emit_generated_owner_request("init_avatar_hp")
                .map_err(|_| ())?;
            let before = requester_observer_state_ref(&runtime);
            Some((request, before))
        } else {
            None
        };
        let connection = endpoint
            .connect(endpoint_address, "localhost")
            .map_err(|_| ())?
            .await
            .map_err(|_| ())?;
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
        let request = match prepared_request {
            Some((request, _)) => request,
            None => runtime
                .emit_generated_owner_request("init_avatar_hp")
                .map_err(|_| ())?,
        };
        let request_delivery = session
            .send_generated_message(request)
            .await
            .map_err(|_| ())?;
        session.finish_send().map_err(|_| ())?;
        let (receipt, reply_delivery) = session
            .receive_and_admit_generated_message(&mut runtime)
            .await
            .map_err(|_| ())?;
        let receipt = receipt.ok_or(())?;
        if !receipt.is_observer_safe_typed_result_or_receipt()
            || !receipt.has_no_transportable_carrier()
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

fn install_ring() -> Result<(), ()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| ())
}

fn server_config(
    control: &PrivateChildControl,
) -> Result<(quinn::ServerConfig, PrivateQuicTransportEvidence), ()> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(control.ca_der.clone()))
        .map_err(|_| ())?;
    let verifier = WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .map_err(|_| ())?;
    let mut crypto = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from(control.leaf_cert_der.clone())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(control.leaf_key_der.to_vec())),
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
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(control.ca_der.clone()))
        .map_err(|_| ())?;
    let mut crypto = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(
            vec![CertificateDer::from(control.leaf_cert_der.clone())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(control.leaf_key_der.to_vec())),
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
