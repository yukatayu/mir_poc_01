//! Private I3-2 process-image and pre-socket process-runtime seam.
//!
//! This module deliberately has no public wire, socket, process launcher, or
//! authority issuer.  It lowers one already checked/projected/admitted local
//! program into per-slot images and transfers exact generated carriers by
//! value only so the subsequent transport milestone has a narrow boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

#[cfg(unix)]
use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::{
        io::{AsRawFd, FromRawFd},
        net::UnixStream,
    },
    time::{Duration, Instant},
};

#[cfg(test)]
use std::sync::{
    Mutex,
    mpsc::{Receiver, SyncSender},
};

use mir_semantics::surface_v0_classification::OwnerAdmissionBudgetCondition;
use serde::{
    Deserialize, Serialize,
    de::{self, Error as _, MapAccess, SeqAccess, Visitor},
};
use sha2::{Digest, Sha256};

#[path = "sys5_i3_process_snapshot.rs"]
mod process_snapshot;
#[path = "sys5_i3_provider_runtime.rs"]
mod provider_runtime;

use provider_runtime::{
    PrivateProviderRequestCarrierSnapshot, PrivateProviderResultCarrierSnapshot,
    Sys5I3ProviderExecution,
};
#[cfg(feature = "i3-process-test-seams")]
pub(crate) use provider_runtime::{
    Sys5I3PrivateProviderConsumedRequestReplay, Sys5I3PrivateProviderReleasedResultReplay,
};
pub(crate) use provider_runtime::{
    Sys5I3PrivateProviderTransportOccurrence, Sys5I3PrivateProviderTransportOccurrenceKind,
    Sys5I3PrivateProviderTransportWriteTicket,
};
pub use provider_runtime::{
    Sys5I3ProviderConsumeReceipt, Sys5I3ProviderRequestCarrier, Sys5I3ProviderResultCarrier,
    Sys5I3ProviderTerminalAudit, Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    Sys5I3ProviderTerminalAuditCounts, Sys5I3ProviderTerminalAuditRow,
    Sys5I3ProviderTerminalOutcomeClass, Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
};
#[cfg(feature = "i3-process-test-seams")]
pub use provider_runtime::{
    Sys5I3ProviderFixtureAssertionCompletion, Sys5I3ProviderLedgerTestFacts,
};
#[cfg(all(test, feature = "i3-process-test-seams"))]
pub use provider_runtime::{
    Sys5I3ProviderRevalidationTestPoint, Sys5I3ProviderRevalidationTestRetirement,
};

// A logical activation occurrence is local runtime evidence, not a process
// identifier, endpoint, session, or transport attempt.  It prevents two
// independently derived cohorts of the same checked source from sharing a
// local-store or request identity before I3-3 retry semantics exist.
static NEXT_PROCESS_COHORT_OCCURRENCE: AtomicU64 = AtomicU64::new(1);

// The dedicated provider child branch takes inherited FD3 exactly once per
// process. This protects against a later descriptor-number reuse becoming a
// second authority-bearing control read.
#[cfg(unix)]
static PROVIDER_INHERITED_CONTROL_FD3_TAKEN: AtomicBool = AtomicBool::new(false);

// This is a bounded, in-memory I3-3 duplicate-admission guard for the
// accepted local profile.  It is deliberately neither durable nor a retry,
// reconnect, or exactly-once policy; reaching the bound fails closed.
const MAX_INBOUND_OWNER_REQUEST_TOMBSTONES: usize = 64;

// Requester-local original-operation state is bounded independently from the
// owner duplicate ledger.  Neither reconnect nor a pending handle can evict
// or reset this map; exhaustion happens before source action submission.
const MAX_OUTBOUND_OWNER_REQUEST_PENDING: usize = 64;
const MAX_OUTBOUND_OWNER_TERMINAL_FAILURES: usize = 64;
const MAX_OUTBOUND_OWNER_REQUEST_ATTEMPTS: u8 = 2;

use crate::{
    checked_program_reference::{checked_program_identity_ref, is_checked_program_identity_ref},
    i3_read_only_provider_composite::{
        I3PrivateFixedProviderNetworkConformanceProfile,
        I3PrivateFixedTerminalObservationConformanceProfile,
        I3PrivateReadOnlyProviderFixtureAssertion, I3PrivateReadOnlyProviderResourceEnvelope,
        I3TrustedReadOnlyProviderFixtureSetup, TrustedFixtureContext,
    },
    m8_owner_admission_gate::{M8I3OwnerAdmissionIssuance, M8I3OwnerAdmissionPermit},
    m8_runtime_admission::{
        M8I3PrivateProviderComponentSnapshot, M8InstalledReadOnlyProviderEffectComponent,
        M8ReadOnlyProviderEffectComponentInventory,
    },
    m9_auth_verification::{
        M9I3PrivateAuthorityGenerationSnapshot, M9I3PrivateReadOnlyProviderRoleSnapshot,
        M9I3ReadOnlyProviderChildRole, M9I3ReadOnlyProviderLocalAuthority,
    },
    sys3_i3_private_snapshot::I3PrivateProviderStaticProjectionSnapshot,
    sys3_projection::{BackendProfile, CommunicationEdgeKind},
    sys4_dispatch::{
        FabricProgram, LocalFabric, LocusStep, ObserverSafeM9SemanticRowSets,
        SealedFabricAdmission, SourceAction, Sys4I3InstalledOwnerCapabilitySuccessorReceipt,
        Sys4I3OwnerCapabilitySuccessorCoordinator, Sys4I3OwnerRequestRevalidationFailure,
        Sys4I3PendingOwnerRequestBinding, Sys4I3PrivateProcessCarrierSnapshot,
        Sys4I3RestrictedOwnerCapabilitySuccessor, Sys4I3ValidatedOwnerReply,
        Sys4InactiveProviderAdmission, Sys4InactiveProviderRestrictedAdmission,
        Sys4InstalledProviderLocalFabric, Sys4ProcessCarrier,
    },
    sys5_local_slice::{Sys5I3AdapterCarrierContract, Sys5LocalProject},
};

#[cfg(feature = "i3-process-test-seams")]
use crate::sys4_dispatch::Sys4I3OwnerCapabilitySuccessorTamper;

/// Typed, fail-closed outcomes for the provisional I3-2 process seam.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProcessRuntimeErrorKind {
    MissingLocusAssignment,
    ExtraLocusAssignment,
    DuplicateLocusAssignment,
    DuplicateDeploymentSlot,
    EmptyDeploymentSlot,
    InsufficientDeploymentSlots,
    UnknownDeploymentSlot,
    ProcessImageAlreadyTaken,
    ForeignArtifact,
    ForeignEdgeContract,
    ImageIntegrityMismatch,
    ImageInventoryProvenanceMismatch,
    ProgramProjectionMismatch,
    MissingRequiredAuthorityEvidence,
    ForeignAuthorityEvidence,
    CohortParentProgramMismatch,
    CohortProjectionMismatch,
    CohortM9GenerationMismatch,
    CohortProvenanceMismatch,
    AuthorityClosureDigestMismatch,
    /// A Stage 2c inactive provider image no longer matched the parent-held
    /// current setup/effect authorization association. No image can revive it
    /// and no bootstrap/start work follows.
    InactiveProviderBindingNotCurrent,
    RuntimeBootstrapRejected,
    NoGeneratedOwnerRequest,
    NonOwnerServe,
    DirectRemoteStore,
    CarrierAdmissionRejected,
    /// A complete source/Core/route/lineage binding named an owner whose
    /// current membership is stale.  No duplicate outcome is disclosed.
    StaleMembership,
    /// A complete source/Core/route/lineage binding named an owner whose
    /// current capability is absent or revoked.  No duplicate outcome is
    /// disclosed.
    MissingCapability,
    /// A complete source/Core/route/lineage binding named an owner whose
    /// current witness is absent or no longer live.  No duplicate outcome is
    /// disclosed.
    MissingWitness,
    /// The bounded provider ledger cannot retain another reservation or
    /// outcome without evicting an earlier fact.
    ResourceExhausted,
    /// The B-local trusted adapter reached CallStarted but the selected
    /// logical resource was absent.
    ProviderResourceNotFound,
    /// The B-local trusted adapter reached CallStarted but rejected the
    /// selected resource kind or fixed physical policy.
    ProviderPolicyDenied,
    /// The B-local trusted adapter reached CallStarted but did not produce
    /// the exact bounded canonical i64 representation.
    ProviderInvalidResult,
    /// The B-local trusted adapter reached CallStarted but an operational
    /// failure prevented a typed physical result.
    AdapterUnavailable,
    /// A second delivery bound to an already reserved source-derived owner
    /// request identity.  It is intentionally not a stored-result return.
    DuplicateRequestRejected,
    /// The outer semantic identity matched a prior reservation but the exact
    /// private generated-carrier snapshot did not.  This never reuses the
    /// prior outcome.
    RequestIdentityBindingMismatch,
    /// The bounded in-memory duplicate ledger cannot accept another distinct
    /// request identity without eviction, so it rejects rather than forget.
    InboundRequestLedgerExhausted,
    /// A trusted owner-runtime clock control was foreign, stale, or moved
    /// backwards.  It makes no admission, queue, or store mutation.
    OwnerAdmissionClockRejected,
    /// The checked `start + budget` deadline cannot be represented as u64,
    /// so no Awaiting ledger entry is retained.
    OwnerAdmissionDeadlineOverflow,
    /// An opaque Awaiting/Reserved control did not belong to this exact live
    /// runtime ledger state.  It discloses no carrier or authority material.
    OwnerAdmissionResolutionRejected,
    /// The requester-local bounded original-operation table cannot retain a
    /// new emitted request without eviction.
    OutboundRequestLedgerExhausted,
    /// A locally validated declared owner-admission failure cannot be
    /// retained without evicting an earlier terminal outcome.  The matching
    /// requester pending entry remains intact.
    OutboundTerminalFailureLedgerExhausted,
    /// An opaque original-request pending handle did not name the retained
    /// requester-local operation, its exact carrier, or its source-selected
    /// requester locus.  This discloses no authority or transport detail.
    OriginalRequestPendingRejected,
    /// The original request has already used its fixed two delivery-attempt
    /// budget.  This is not a completed semantic result.
    OutboundAttemptBudgetExhausted,
    /// A parent-only owner-capability successor was not selected from the
    /// exact checked prelaunch owner-request contract, or an image/control
    /// handoff had already begun.
    LifecyclePrestageRejected,
    /// Tainted child lifecycle material did not match its independently
    /// retained trusted bootstrap expectation.
    LifecycleCandidateRejected,
    /// B could not install the exact restricted successor and commit its
    /// local M9 live floor; no receipt or parent publication follows.
    LifecycleInstallRejected,
    /// A lifecycle acknowledgement did not arrive through the registered B
    /// child completion route.
    LifecycleAckRejected,
    /// The parent ended a staged lifecycle without a consumable B completion.
    LifecyclePublicationIncomplete,
    MissingAuthoritativeState,
    OutboundExtractionRejected,
    /// The cfg(test)-only bounded handshake did not reach its deterministic
    /// post-structural validation point. It is not a runtime authority or
    /// lifecycle result.
    #[cfg(test)]
    TestOnlyInactiveProviderValidationSynchronizationTimedOut,
}

/// One observer-safe process-seam failure.  No raw M8/M9 material or source
/// text is included in the diagnostic surface.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessRuntimeError {
    kind: Sys5I3ProcessRuntimeErrorKind,
}

impl Sys5I3ProcessRuntimeError {
    fn new(kind: Sys5I3ProcessRuntimeErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3ProcessRuntimeErrorKind {
        self.kind
    }
}

/// Typed failures at the narrow trusted provider-launch boundary. These are
/// operational control failures, never source/provider result diagnostics.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProviderLaunchErrorKind {
    /// The feature-gated canonical source fixture could not reach the real
    /// checked/composite/binding/policy launch boundary. The error retains no
    /// source text, fixture path, result, authority, or control material.
    SourceCompositePreparationRejected,
    ProviderRuntimeActivationPending,
    InactiveProviderBindingNotCurrent,
    InvalidChildRole,
    ProviderChildImageAlreadyTaken,
    ProviderChildImageNotWritten,
    ProviderChildImageWriteRejected,
    ProviderChildControlAlreadyTaken,
    ProviderTransportBootstrapOversized,
    ProviderControlConstructionRejected,
    ProviderControlSerializationRejected,
    ProviderControlSnapshotOversized,
    ProviderControlDeadlineElapsed,
    ProviderControlWriteRejected,
    ProviderControlShutdownRejected,
    InheritedProviderControlRejected,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderLaunchError {
    kind: Sys5I3ProviderLaunchErrorKind,
}

impl Sys5I3ProviderLaunchError {
    pub(crate) const fn new(kind: Sys5I3ProviderLaunchErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3ProviderLaunchErrorKind {
        self.kind
    }
}

/// Observer-safe terminal status of the one bounded I3 owner-capability
/// lifecycle.  `None` means a genuine pre-stage is still awaiting an
/// installed B receipt; it is intentionally not exposed as a general
/// lifecycle-control state.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3LifecyclePublicationOutcome {
    NoPrestageSelected,
    G2Published,
    PublicationIncomplete,
}

/// Narrow observer-safe parent lifecycle summary.  It contains neither M8/
/// M9 inventory nor candidate/ack bytes, capability/witness material, or a
/// publisher handle.
#[doc(hidden)]
pub struct Sys5I3ObserverSafeLifecyclePublicationSummary {
    published_authority_generation_ref: String,
    publication_outcome: Option<Sys5I3LifecyclePublicationOutcome>,
}

/// Observer-safe origin classification for the one parent-prestaged owner
/// lifecycle.  It deliberately distinguishes this M9 admission from source
/// action, network delivery, and ordinary semantic authority.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ObserverSafeLifecycleOrigin {
    M9AdmittedLifecycle,
}

/// Bounded observer-safe proof that B installed the parent-staged successor.
/// It contains only origin, source-derived classification, and generation
/// reference—never candidate bytes, M8/M9 inventory, receipt, or publisher.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeInstalledLifecycle {
    successor_generation_ref: String,
    origin: Sys5I3ObserverSafeLifecycleOrigin,
    source_derived: bool,
}

impl Sys5I3ObserverSafeInstalledLifecycle {
    pub fn successor_generation_ref(&self) -> &str {
        &self.successor_generation_ref
    }

    pub const fn origin(&self) -> Sys5I3ObserverSafeLifecycleOrigin {
        self.origin
    }

    pub const fn source_derived(&self) -> bool {
        self.source_derived
    }
}

impl Sys5I3ObserverSafeLifecyclePublicationSummary {
    pub fn published_authority_generation_ref(&self) -> &str {
        &self.published_authority_generation_ref
    }

    pub const fn publication_outcome(&self) -> Option<Sys5I3LifecyclePublicationOutcome> {
        self.publication_outcome
    }
}

/// Feature-gated negative selectors for an M9-produced staged candidate.
/// These do not accept caller-provided authority facts and are absent from
/// ordinary lifecycle execution.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3OwnerCapabilitySuccessorTamper {
    AddUnrelatedOwnerLineage,
    ReanimateSelectedCapability,
    ReanimateSelectedWitness,
}

/// The owner-local duplicate guard's state.  It is narrower than the I3-3
/// request lifecycle: `Reserved` is intentionally retained if a later
/// SYS-4 handoff errors, where this layer cannot prove non-mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sys5I3InboundOwnerRequestTombstonePhase {
    Reserved,
    Received,
    Ambiguous,
    Awaiting,
    Expired,
    RejectedBeforeServe,
    ServeReserved,
}

/// Private, source-derived replay protection retained by an owner runtime.
/// The exact canonical snapshot bytes are checked only for equality; neither
/// they nor any prior outcome are exposed to a duplicate sender.
///
/// This intentionally has no `Debug` implementation: raw private payload
/// bytes must not become an observer/debug surface.
struct Sys5I3InboundOwnerRequestTombstone {
    carrier_snapshot_binding_bytes: Vec<u8>,
    phase: Sys5I3InboundOwnerRequestTombstonePhase,
    terminal_admission: Option<Sys5I3TerminalOwnerAdmission>,
}

/// A single ledger entry keeps replay disposition clone-free from live
/// authority handoff material.  No second issuer or request-ID map exists.
struct Sys5I3InboundOwnerRequestRecord {
    tombstone: Sys5I3InboundOwnerRequestTombstone,
    live_admission: Option<Sys5I3LiveOwnerAdmission>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sys5I3OwnerAdmissionClockKey {
    owner_locus: String,
    clock_domain: String,
}

struct Sys5I3OwnerAdmissionClockState {
    tick: u64,
}

/// A trusted runtime-control selector, not observer output or authority.  It
/// can name only one checked owner/domain already retained in this runtime.
#[doc(hidden)]
pub(crate) struct Sys5I3OwnerAdmissionClockHandle {
    runtime_binding_ref: String,
    runtime_instance: u64,
    clock_key: Sys5I3OwnerAdmissionClockKey,
}

/// The sole T0 control for advancing one checked owner clock and resolving
/// its deterministic next staged admission.  It is runtime-bound and opaque:
/// callers cannot construct it, select a request, or obtain the underlying
/// clock handle, Awaiting entry, reservation, permit, carrier, or authority.
///
/// This is a private I3-3 host-control seam, not a public clock, wire, or
/// provider API.  In particular, it carries no observer-safe tick projection
/// and derives no authority beyond the already admitted runtime condition.
#[doc(hidden)]
pub struct Sys5I3OwnerAdmissionHostDriver {
    clock_handle: Sys5I3OwnerAdmissionClockHandle,
}

/// A non-Clone opaque reference to one exact Awaiting record.  It contains no
/// carrier, permit, authority, condition, or caller-selected request fields.
#[doc(hidden)]
pub(crate) struct Sys5I3OwnerAdmissionAwaiting {
    runtime_binding_ref: String,
    runtime_instance: u64,
    clock_key: Sys5I3OwnerAdmissionClockKey,
    request_identity_ref: String,
}

/// The only owner-runtime handoff carrying an unverified linear permit.  It
/// cannot be cloned, saved, or re-created once consumed or dropped.
#[doc(hidden)]
pub(crate) struct Sys5I3OwnerAdmissionReserved {
    runtime_binding_ref: String,
    runtime_instance: u64,
    request_identity_ref: String,
    carrier: Sys4ProcessCarrier,
    pending: Sys4I3PendingOwnerRequestBinding,
    carrier_snapshot_binding_bytes: Vec<u8>,
    permit: M8I3OwnerAdmissionPermit,
}

/// The only two terminal results of resolving one exact Awaiting entry.  A
/// declared failure remains an ordinary checked OwnerReply carrier; it is not
/// a transport result or a receipt.
#[doc(hidden)]
pub(crate) enum Sys5I3OwnerAdmissionResolution {
    ServeReserved(Box<Sys5I3OwnerAdmissionReserved>),
    DeclaredOwnerFailure(Box<Sys5I3ProcessMessage>),
}

struct Sys5I3LiveOwnerAdmission {
    carrier: Sys4ProcessCarrier,
    pending: Sys4I3PendingOwnerRequestBinding,
    clock_key: Sys5I3OwnerAdmissionClockKey,
    start_tick: u64,
    deadline_tick: u64,
    issuance: M8I3OwnerAdmissionIssuance,
}

struct Sys5I3PreparedOwnerAdmission {
    clock_key: Sys5I3OwnerAdmissionClockKey,
    start_tick: u64,
    deadline_tick: u64,
    issuance: M8I3OwnerAdmissionIssuance,
}

/// Immutable stage material needed by the next typed failure producer.  It
/// retains checked source/Core/request/reply contract binding without a live
/// permit, carrier export, or authority issuer.
struct Sys5I3TerminalOwnerAdmission {
    semantic_request_identity_ref: String,
    carrier: Sys4ProcessCarrier,
    pending: Sys4I3PendingOwnerRequestBinding,
    clock_key: Sys5I3OwnerAdmissionClockKey,
    start_tick: u64,
    deadline_tick: u64,
    resolution_tick: u64,
    resolution_generation_ref: String,
    // Present only after SYS-4 has successfully constructed the one
    // gate-produced expiry reply carrier.  It records production, not any
    // later adapter send, receive, or requester consumption.
    declared_deadline_expiry: Option<Sys5I3ObserverSafeOwnerAdmissionExpiryDecision>,
    current_authority_failure: Option<Sys4I3OwnerRequestRevalidationFailure>,
    // A later use-time check after the one-use permit has been consumed must
    // not relabel the immutable resolution decision above.  It is retained
    // separately for the next typed failure producer.
    handoff_revalidation_tick: Option<u64>,
    handoff_revalidation_generation_ref: Option<String>,
    handoff_authority_failure: Option<Sys4I3OwnerRequestRevalidationFailure>,
}

struct Sys5I3RequesterTerminalDeclaredOwnerFailure {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "spec/16 retains the exact requester-terminal commitment while the current observer output exports only the correlated occurrence"
        )
    )]
    decision_commitment_ref: String,
    decision_occurrence_ref: String,
}

/// A typed observer-safe projection of one actually produced owner-side
/// `DeadlineExpired` reply.  These opaque references are correlation
/// evidence, not a clock control, carrier, capability, witness, or proof
/// that a transport send/receive/consume occurred.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeOwnerAdmissionExpiryDecision {
    decision_commitment_ref: String,
    decision_occurrence_ref: String,
}

impl Sys5I3ObserverSafeOwnerAdmissionExpiryDecision {
    pub fn decision_commitment_ref(&self) -> &str {
        &self.decision_commitment_ref
    }

    pub fn decision_occurrence_ref(&self) -> &str {
        &self.decision_occurrence_ref
    }
}

/// Counts only retained owner-admission lifecycle dispositions.  It never
/// exposes controls, identities, source payload, carrier, permit, or M9/M8
/// material.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Sys5I3ObserverSafeOwnerAdmissionSummary {
    awaiting_count: usize,
    expired_count: usize,
    rejected_before_serve_count: usize,
    serve_reserved_count: usize,
}

impl Sys5I3ObserverSafeOwnerAdmissionSummary {
    pub const fn awaiting_count(&self) -> usize {
        self.awaiting_count
    }

    pub const fn expired_count(&self) -> usize {
        self.expired_count
    }

    pub const fn rejected_before_serve_count(&self) -> usize {
        self.rejected_before_serve_count
    }

    pub const fn serve_reserved_count(&self) -> usize {
        self.serve_reserved_count
    }
}

/// Retained requester-local state for a source-emitted owner request.  The
/// exact carrier is deliberately not exported through the pending handle:
/// only the runtime can revalidate and encode it for an adapter-owned send.
struct Sys5I3OutboundOwnerRequestRecord {
    pending: Sys4I3PendingOwnerRequestBinding,
    exact_carrier: Sys4ProcessCarrier,
    pending_token_ref: String,
    started_attempts: u8,
    // The source-selected requester stays in `pending`; this fixed intent
    // records whether its most recent bounded attempt was original delivery
    // or the one authorized reconnect retry.  No caller text participates.
    last_started_attempt_kind: Option<Sys5I3OriginalOwnerRequestAttemptKind>,
    pending_handle_issued: bool,
}

/// A non-cloneable, opaque claim to one runtime-retained original owner
/// request.  Possession alone grants no retry: the runtime compares this
/// token with its own record, rechecks exact current binding/M9 authority,
/// and enforces the source-selected requester and attempt budget.
#[doc(hidden)]
pub struct Sys5I3OriginalOwnerRequestPending {
    semantic_request_identity_ref: String,
    runtime_token_ref: String,
}

impl Sys5I3OriginalOwnerRequestPending {
    /// An observer-safe source-derived identity for correlation only.  It is
    /// not a carrier, capability, retry authorization, or transport key.
    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }
}

/// Private attempt intent selected by the adapter API, not by caller text.
/// A reconnect retry is accepted only by a consuming second-session adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sys5I3OriginalOwnerRequestAttemptKind {
    InitialDelivery,
    ReconnectRetry,
}

/// Fixed reason recorded for a runtime-authorized original-request delivery
/// attempt.  It has no free-form/caller-provided branch and is not authority.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason {
    InitialDelivery,
    ReconnectRetry,
}

impl From<Sys5I3OriginalOwnerRequestAttemptKind>
    for Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason
{
    fn from(value: Sys5I3OriginalOwnerRequestAttemptKind) -> Self {
        match value {
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery => Self::InitialDelivery,
            Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry => Self::ReconnectRetry,
        }
    }
}

/// Observer-safe summary of the latest begun attempt for one retained
/// original request.  The requester binding is a domain-separated reference,
/// not an M8/M9 principal/capability/witness or a transport credential.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeOriginalOwnerRequestAttempt {
    semantic_request_identity_ref: String,
    source_requester_locus: String,
    requester_binding_ref: String,
    attempt_generation: u8,
    reason: Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason,
}

impl Sys5I3ObserverSafeOriginalOwnerRequestAttempt {
    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn source_requester_locus(&self) -> &str {
        &self.source_requester_locus
    }

    pub fn requester_binding_ref(&self) -> &str {
        &self.requester_binding_ref
    }

    pub const fn attempt_generation(&self) -> u8 {
        self.attempt_generation
    }

    pub const fn reason(&self) -> Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason {
        self.reason
    }
}

/// One runtime-authorized, one-use exact request frame.  It stays
/// crate-private so no caller can replay or inspect its carrier bytes.
pub(crate) struct Sys5I3AuthorizedOriginalOwnerRequestAttempt {
    semantic_request_identity_ref: String,
    runtime_token_ref: String,
    attempt_number: u8,
    attempt_kind: Sys5I3OriginalOwnerRequestAttemptKind,
    encoded_message_bytes: Vec<u8>,
}

impl Sys5I3AuthorizedOriginalOwnerRequestAttempt {
    pub(crate) fn encoded_message_bytes(&self) -> &[u8] {
        &self.encoded_message_bytes
    }

    pub(crate) fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }
}

/// Deployment input for one logical process slot.  It is intentionally only
/// a locus-to-slot-to-endpoint assignment, never a route, Core, authority,
/// state, or expected result.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3DeploymentSlot {
    slot_name: String,
    endpoint: String,
    // Preserve raw construction rows until deployment validation.  Collapsing
    // this to a set here would hide duplicate locus assignment before the
    // fail-closed boundary gets a chance to reject it.
    loci: Vec<String>,
}

impl Sys5I3DeploymentSlot {
    pub fn new<I, S>(slot_name: impl Into<String>, endpoint: impl Into<String>, loci: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            slot_name: slot_name.into(),
            endpoint: endpoint.into(),
            loci: loci.into_iter().map(Into::into).collect(),
        }
    }
}

/// Observer-only deployment view proving that deployment did not choose
/// semantic operations or authority.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3DeploymentManifest;

impl Sys5I3DeploymentManifest {
    pub const fn has_only_locus_slot_endpoint_assignments(&self) -> bool {
        true
    }
}

/// A complete checked deployment assignment.  Its only retained information
/// is the slot/endpoint/locus map plus observer-safe parent provenance refs.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3Deployment {
    slots: Vec<Sys5I3DeploymentSlot>,
    parent_checked_program_ref: String,
    parent_projection_ref: String,
}

impl Sys5I3Deployment {
    pub fn from_checked_project<I>(
        project: &Sys5LocalProject,
        slots: I,
    ) -> Result<Self, Sys5I3ProcessRuntimeError>
    where
        I: IntoIterator<Item = Sys5I3DeploymentSlot>,
    {
        let expected = project
            .semantic_summary()
            .loci
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let slots = slots.into_iter().collect::<Vec<_>>();
        validate_i3_deployment_slots(&expected, &slots)?;
        Ok(Self {
            slots,
            parent_checked_program_ref: project.checked_program_identity_ref().to_string(),
            parent_projection_ref: project.i3_parent_projection_ref(),
        })
    }

    pub fn observer_safe_manifest(&self) -> Sys5I3DeploymentManifest {
        Sys5I3DeploymentManifest
    }

    fn slot(&self, slot_name: &str) -> Option<&Sys5I3DeploymentSlot> {
        self.slots.iter().find(|slot| slot.slot_name == slot_name)
    }
}

/// Preserve the ordinary deployment cardinality and locus-assignment rules
/// when an inactive provider cohort is assembled before image handoff.
fn validate_i3_deployment_slots(
    expected: &BTreeSet<String>,
    slots: &[Sys5I3DeploymentSlot],
) -> Result<(), Sys5I3ProcessRuntimeError> {
    if slots.len() < 2 {
        return Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::InsufficientDeploymentSlots,
        ));
    }
    if slots.iter().any(|slot| slot.loci.is_empty()) {
        return Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::EmptyDeploymentSlot,
        ));
    }
    let mut slot_names = BTreeSet::new();
    if slots
        .iter()
        .any(|slot| !slot_names.insert(slot.slot_name.clone()))
    {
        return Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
        ));
    }
    let mut assigned = BTreeSet::new();
    for locus in slots.iter().flat_map(|slot| slot.loci.iter()) {
        if !expected.contains(locus) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ExtraLocusAssignment,
            ));
        }
        if !assigned.insert(locus.clone()) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::DuplicateLocusAssignment,
            ));
        }
    }
    if assigned != *expected {
        return Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::MissingLocusAssignment,
        ));
    }
    Ok(())
}

/// A compact source-free description of one executable artifact retained in
/// a process image.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessArtifact {
    locus: String,
    operation_id: String,
    kind: String,
    core_ref: String,
    fragment_ref: String,
    parent_checked_program_ref: String,
}

impl Sys5I3ProcessArtifact {
    pub fn locus(&self) -> &str {
        &self.locus
    }
}

/// One reference-only generated edge incident to a process image.  It has no
/// payload, witness, capability, or transport/session data.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3RetainedEdgeContract {
    source_locus: String,
    target_locus: String,
    edge_ref: String,
    operation_id: String,
    kind: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    parent_checked_program_ref: String,
}

impl Sys5I3RetainedEdgeContract {
    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }

    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    /// Observer-safe generated-contract lineage retained in the sealed image.
    /// These references are not a route, payload, authority, or public wire
    /// schema.
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn kind(&self) -> &str {
        &self.kind
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

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub const fn is_reference_only(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sys5I3SemanticRow {
    Artifact {
        locus: String,
        fragment_ref: String,
    },
    IncidentEdge {
        source_locus: String,
        target_locus: String,
        edge_ref: String,
    },
}

/// Observer-safe proof that the child has exactly the semantic-row boundary
/// required by its assigned artifacts and incident generated edges.  Raw M9
/// authority is retained only in the private SYS-4/M8 seed.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3RequiredLocalAuthorityClosure {
    assigned_loci: BTreeSet<String>,
    rows: Vec<Sys5I3SemanticRow>,
    semantic_row_digest_ref: String,
    opaque_digest_ref: String,
    opaque_cohort_ref: String,
}

impl Sys5I3RequiredLocalAuthorityClosure {
    pub const fn is_reference_only(&self) -> bool {
        true
    }

    pub fn is_exact_for_image(&self) -> bool {
        !self.rows.is_empty()
            && !self.semantic_row_digest_ref.is_empty()
            && self.opaque_digest_ref == self.recomputed_digest()
    }

    pub fn has_no_unassigned_semantic_rows(&self) -> bool {
        self.rows.iter().all(|row| match row {
            Sys5I3SemanticRow::Artifact { locus, .. } => self.assigned_loci.contains(locus),
            Sys5I3SemanticRow::IncidentEdge {
                source_locus,
                target_locus,
                ..
            } => {
                self.assigned_loci.contains(source_locus)
                    || self.assigned_loci.contains(target_locus)
            }
        })
    }

    pub fn opaque_digest_ref(&self) -> &str {
        &self.opaque_digest_ref
    }

    pub fn opaque_cohort_ref(&self) -> &str {
        &self.opaque_cohort_ref
    }

    fn recomputed_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/sys5/i3/process-authority-closure/v1\\0");
        hasher.update(format!(
            "{:?}{:?}{}",
            self.assigned_loci, self.rows, self.semantic_row_digest_ref
        ));
        format!(
            "sys5-i3-process-authority-closure-sha256-v1:{:x}",
            hasher.finalize()
        )
    }
}

/// Observer-only child-seed view.  The executable seed stays private and is
/// limited to selected M8/M9 plans; this view makes the no-publisher and
/// no-full-program properties explicit without exposing those plans.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeChildSeed {
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_occurrence_ref: String,
    required_local_authority_closure: Sys5I3RequiredLocalAuthorityClosure,
}

impl Sys5I3ObserverSafeChildSeed {
    pub const fn carries_authority_publisher_or_issuer(&self) -> bool {
        false
    }

    pub const fn carries_full_prepared_admission(&self) -> bool {
        false
    }

    pub const fn carries_full_fabric_program(&self) -> bool {
        false
    }

    pub fn required_local_authority_closure(&self) -> &Sys5I3RequiredLocalAuthorityClosure {
        &self.required_local_authority_closure
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn m9_generation_ref(&self) -> &str {
        &self.m9_generation_ref
    }

    fn cohort_occurrence_ref(&self) -> &str {
        &self.cohort_occurrence_ref
    }
}

/// Observer-safe designated remote-input closure for one child image.  The
/// typed request/receipt tuples remain in the sealed SYS-4 projection; this
/// view exposes only exact-pair facts and an opaque identity.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3DesignatedRemoteInputClosure {
    request_receipt_pair_count: usize,
    distinct_operation_count: usize,
    pairs_are_distinguished_beyond_operation: bool,
    opaque_digest_ref: String,
}

impl Sys5I3DesignatedRemoteInputClosure {
    pub const fn is_reference_only(&self) -> bool {
        true
    }

    pub fn is_exact_for_image(&self) -> bool {
        !self.opaque_digest_ref.is_empty()
    }

    pub const fn is_derived_from_request_receipt_edges(&self) -> bool {
        true
    }

    pub const fn request_receipt_pair_count(&self) -> usize {
        self.request_receipt_pair_count
    }

    pub const fn distinct_operation_count(&self) -> usize {
        self.distinct_operation_count
    }

    pub const fn pairs_are_distinguished_beyond_operation(&self) -> bool {
        self.pairs_are_distinguished_beyond_operation
    }

    pub const fn is_symmetric_empty_for_image(&self) -> bool {
        self.request_receipt_pair_count == 0
            && self.distinct_operation_count == 0
            && !self.pairs_are_distinguished_beyond_operation
    }
}

/// Process-image-only tamper choices used by the I3-2 RED conformance test.
/// They mutate a detached candidate and cannot provide a production input.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5I3ProcessImageTamper {
    AppendForeignArtifact(Sys5I3ProcessArtifact),
    AppendForeignEdgeContract(Sys5I3RetainedEdgeContract),
    CorruptImageIntegrity,
    RemoveProjectedDesignatedRemoteInputRequirement,
    MismatchProjectedDesignatedRemoteInputRequestReceipt,
    RemoveOneRequiredSemanticBinding,
    AppendSemanticRowForUnassignedLocus,
    MismatchedParentCheckedProgramRef,
    MismatchedProjectionRef,
    MismatchedM9GenerationRef,
    MismatchedAuthorityClosureDigest,
    RemoveActualRestrictedOwnerBindingFromPrivateSeed,
    RemoveActualDesignatedRemoteInputLineageFromPrivateSeed,
    SubstituteSameLocusArtifactAndRecomputeIntegrity(Sys5I3ProcessArtifact),
    SubstituteSameIncidentEdgeAndRecomputeIntegrity(Sys5I3RetainedEdgeContract),
    DuplicateArtifactRowAndRecomputeIntegrity,
    DuplicateEdgeContractRowAndRecomputeIntegrity,
    #[cfg(feature = "i3-process-test-seams")]
    MismatchPrestagedOwnerCapabilityLifecycleTargetSlot,
}

impl Sys5I3ProcessImageTamper {
    pub fn append_foreign_artifact(artifact: Sys5I3ProcessArtifact) -> Self {
        Self::AppendForeignArtifact(artifact)
    }

    pub fn append_foreign_edge_contract(contract: Sys5I3RetainedEdgeContract) -> Self {
        Self::AppendForeignEdgeContract(contract)
    }

    pub fn corrupt_image_integrity() -> Self {
        Self::CorruptImageIntegrity
    }

    pub fn remove_projected_designated_remote_input_requirement() -> Self {
        Self::RemoveProjectedDesignatedRemoteInputRequirement
    }

    pub fn mismatch_projected_designated_remote_input_request_receipt() -> Self {
        Self::MismatchProjectedDesignatedRemoteInputRequestReceipt
    }

    pub fn remove_one_required_semantic_binding() -> Self {
        Self::RemoveOneRequiredSemanticBinding
    }

    pub fn append_semantic_row_for_unassigned_locus() -> Self {
        Self::AppendSemanticRowForUnassignedLocus
    }

    pub fn mismatched_parent_checked_program_ref() -> Self {
        Self::MismatchedParentCheckedProgramRef
    }

    pub fn mismatched_projection_ref() -> Self {
        Self::MismatchedProjectionRef
    }

    pub fn mismatched_m9_generation_ref() -> Self {
        Self::MismatchedM9GenerationRef
    }

    pub fn mismatched_authority_closure_digest() -> Self {
        Self::MismatchedAuthorityClosureDigest
    }

    pub const fn remove_actual_restricted_owner_binding_from_private_seed() -> Self {
        Self::RemoveActualRestrictedOwnerBindingFromPrivateSeed
    }

    pub const fn remove_actual_designated_remote_input_lineage_from_private_seed() -> Self {
        Self::RemoveActualDesignatedRemoteInputLineageFromPrivateSeed
    }

    pub fn substitute_same_locus_artifact_and_recompute_integrity(
        artifact: Sys5I3ProcessArtifact,
    ) -> Self {
        Self::SubstituteSameLocusArtifactAndRecomputeIntegrity(artifact)
    }

    pub fn substitute_same_incident_edge_and_recompute_integrity(
        contract: Sys5I3RetainedEdgeContract,
    ) -> Self {
        Self::SubstituteSameIncidentEdgeAndRecomputeIntegrity(contract)
    }

    pub const fn duplicate_artifact_row_and_recompute_integrity() -> Self {
        Self::DuplicateArtifactRowAndRecomputeIntegrity
    }

    pub const fn duplicate_edge_contract_row_and_recompute_integrity() -> Self {
        Self::DuplicateEdgeContractRowAndRecomputeIntegrity
    }

    /// Negative-only B bootstrap falsifier.  It changes only the target-slot
    /// wrapper in an already M9-produced tainted lifecycle image; it cannot
    /// alter candidate M8/M9 facts, mint authority, or supply an issuer.
    #[cfg(feature = "i3-process-test-seams")]
    pub const fn mismatch_prestaged_owner_capability_lifecycle_target_slot() -> Self {
        Self::MismatchPrestagedOwnerCapabilityLifecycleTargetSlot
    }
}

/// Test-only mutations of a decoded inactive provider candidate. Each choice
/// changes one tainted descriptor and recomputes the candidate commitment; it
/// has no constructor for M8/M9 authority, a host binding, or an executable
/// runtime seed.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3InactiveProviderImageTamper {
    ChangeProviderRoleDescriptor,
    ChangeProviderEdgeDescriptor,
    ChangeRetainedLoweringAssociation,
    RemoveExcludedProviderLoweringAssociation,
    ChangeEndpointMetadata,
    StripAssignedLocus,
    WidenAssignedLocus,
    RemoveRequiredLegacyAuthorityLineage,
}

/// Tainted image-carried successor material.  It has no issuer and becomes
/// usable only after the matching trusted start binding is consumed for B.
struct Sys5I3PrestagedOwnerCapabilityLifecycle {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    candidate: Sys4I3RestrictedOwnerCapabilitySuccessor,
}

/// Observer-safe portion of the parent-held lifecycle expectation.  It is
/// separately transported on trusted bootstrap control and therefore cannot
/// be reconstructed from an image candidate alone.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys5I3ExpectedOwnerCapabilityLifecycle {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

/// A one-use B-local install stimulus created only after an untrusted image
/// has matched independently trusted bootstrap control.  It has no public
/// constructor, Clone, or Debug surface.
#[doc(hidden)]
pub struct Sys5I3AdmittedLifecycleStimulus {
    stage_identity_binding_ref: String,
    candidate: Sys4I3RestrictedOwnerCapabilitySuccessor,
}

/// B's post-install receipt wrapper.  It is intentionally opaque and
/// non-cloneable: future ACK emission consumes the SYS-4 receipt rather than
/// candidate fields or observer output.
#[doc(hidden)]
pub struct Sys5I3OwnerLifecycleInstallAck {
    stage_identity_binding_ref: String,
    receipt: Sys4I3InstalledOwnerCapabilitySuccessorReceipt,
}

/// Parent-only, non-cloneable registration derived while the exact B
/// candidate is staged.  It is intentionally not exposed to callers: the
/// cohort moves it directly into the owned actual-B Unix-stream reader.
#[cfg(all(unix, feature = "i3-process-test-seams"))]
struct Sys5I3RegisteredOwnerLifecycleAckRegistration {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

/// Opaque completion derived only by a reader that owns the registered B
/// child output stream.  It has no public constructor, Clone, Debug, raw
/// bytes, source facts, candidate, receipt, or authority material.
#[cfg(all(unix, feature = "i3-process-test-seams"))]
#[doc(hidden)]
pub struct Sys5I3RegisteredOwnerLifecycleCompletion {
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

/// Dedicated I3 test-adapter reader for the inherited B→parent ACK FD.  It
/// owns the OS stream, has no `Read`-generic/byte constructor, and can only
/// produce a completion by bounded-reading and decoding that actual stream.
#[cfg(all(unix, feature = "i3-process-test-seams"))]
#[doc(hidden)]
pub struct Sys5I3RegisteredOwnerLifecycleAckReader {
    stream: UnixStream,
    registration: Sys5I3RegisteredOwnerLifecycleAckRegistration,
    deadline: Instant,
    completion_consumed: bool,
    deadline_only: bool,
}

struct Sys5I3OrdinaryPrivateRuntimeSeed {
    program: FabricProgram,
    admission: SealedFabricAdmission,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_occurrence_ref: String,
    private_snapshot_binding_ref: String,
    prestaged_owner_capability_lifecycle: Option<Sys5I3PrestagedOwnerCapabilityLifecycle>,
}

/// The private runtime seed remains tagged through codec decode. Provider
/// images carry only static/descriptive correspondence and have no ordinary
/// M9 generation, FabricProgram, SealedFabricAdmission, or LocalFabric seed.
enum Sys5I3PrivateRuntimeSeed {
    Ordinary(Box<Sys5I3OrdinaryPrivateRuntimeSeed>),
    InactiveProvider(Box<Sys5I3InactiveProviderRuntimeSeed>),
}

impl Sys5I3PrivateRuntimeSeed {
    fn ordinary(&self) -> Option<&Sys5I3OrdinaryPrivateRuntimeSeed> {
        match self {
            Self::Ordinary(seed) => Some(seed),
            Self::InactiveProvider(_) => None,
        }
    }

    fn ordinary_mut(&mut self) -> Option<&mut Sys5I3OrdinaryPrivateRuntimeSeed> {
        match self {
            Self::Ordinary(seed) => Some(seed),
            Self::InactiveProvider(_) => None,
        }
    }

    fn inactive_provider(&self) -> Option<&Sys5I3InactiveProviderRuntimeSeed> {
        match self {
            Self::Ordinary(_) => None,
            Self::InactiveProvider(seed) => Some(seed),
        }
    }
}

#[derive(Debug, Clone)]
struct Sys5I3InactiveProviderRuntimeSeed {
    parent_checked_program_ref: String,
    static_snapshot: I3PrivateProviderStaticProjectionSnapshot,
    component_snapshot: M8I3PrivateProviderComponentSnapshot,
    legacy_authority_snapshot: M9I3PrivateAuthorityGenerationSnapshot,
    component_binding_ref: String,
}

/// Static provider descriptor carried by a tainted inactive image. It is not
/// an executable artifact, a carrier, an M9 generation, or a resource handle.
#[derive(Debug, Clone)]
struct Sys5I3InactiveProviderImageSeed {
    assigned_loci: BTreeSet<String>,
    parent_checked_program_ref: String,
    static_snapshot: I3PrivateProviderStaticProjectionSnapshot,
    component_snapshot: M8I3PrivateProviderComponentSnapshot,
    legacy_authority_snapshot: M9I3PrivateAuthorityGenerationSnapshot,
    component_binding_ref: String,
}

/// An immutable process image.  Its private runtime seed is already reduced
/// to assigned loci and incident generated edges; the full project admission
/// and global program have been dropped before this value is returned.
#[doc(hidden)]
pub struct Sys5I3ProcessImage {
    slot_name: String,
    endpoint: String,
    assigned_loci: BTreeSet<String>,
    executable_artifacts: Vec<Sys5I3ProcessArtifact>,
    required_edge_contracts: Vec<Sys5I3RetainedEdgeContract>,
    designated_remote_input_closure: Sys5I3DesignatedRemoteInputClosure,
    child_seed: Sys5I3ObserverSafeChildSeed,
    private_runtime_seed: Sys5I3PrivateRuntimeSeed,
    inactive_provider: Option<Sys5I3InactiveProviderImageSeed>,
    private_integrity_ref: String,
}

impl std::fmt::Debug for Sys5I3ProcessImage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3ProcessImage")
            .field("slot_name", &self.slot_name)
            .field("assigned_loci", &self.assigned_loci)
            .field(
                "executable_artifact_count",
                &self.executable_artifacts.len(),
            )
            .field(
                "incident_edge_contract_count",
                &self.required_edge_contracts.len(),
            )
            .finish_non_exhaustive()
    }
}

/// Observer-safe construction facts for one process cohort.  The counts are
/// fixed by the only constructor below: the coordinator performs one full
/// prepare/admission and one M9 generation, then drops those full values
/// after it has derived every child image.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessCohortSummary {
    full_admission_count: usize,
    authority_generation_count: usize,
    parent_checked_program_ref: String,
    projection_ref: String,
    activation_occurrence_ref: String,
    cohort_occurrence_ref: String,
}

/// A coordinator-retained expectation for exactly one child bootstrap. For a
/// tagged inactive provider image it retains private DTOs only for structural
/// equality at the expected-start boundary; decoding them never reconstructs
/// authority. Its `Debug` surface exposes only bounded routing facts.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ExpectedStartBinding {
    slot_name: String,
    assigned_loci: BTreeSet<String>,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_provenance_ref: String,
    image_integrity_ref: String,
    private_snapshot_binding_ref: String,
    expected_owner_capability_lifecycle: Option<Sys5I3ExpectedOwnerCapabilityLifecycle>,
    inactive_provider: Option<Sys5I3ExpectedInactiveProviderBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys5I3ExpectedInactiveProviderBinding {
    static_snapshot: I3PrivateProviderStaticProjectionSnapshot,
    component_snapshot: M8I3PrivateProviderComponentSnapshot,
    legacy_authority_snapshot: M9I3PrivateAuthorityGenerationSnapshot,
    component_binding_ref: String,
}

impl std::fmt::Debug for Sys5I3ExpectedStartBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3ExpectedStartBinding")
            .field("slot_name", &self.slot_name)
            .field("assigned_loci", &self.assigned_loci)
            .field(
                "parent_checked_program_ref",
                &self.parent_checked_program_ref,
            )
            .field("image_integrity_ref", &self.image_integrity_ref)
            .finish_non_exhaustive()
    }
}

/// One supervisor-issued control record for the finite two-child localnet
/// profile.  It deliberately travels on a distinct trusted control channel:
/// an image byte stream is never sufficient to start a process or to bind a
/// transport peer.  This is private/provisional evidence, not a package,
/// process, wire, or certificate ABI.
#[doc(hidden)]
pub struct Sys5I3TrustedLocalnetControl {
    expected_start_binding: Sys5I3ExpectedStartBinding,
    run_ref: String,
    local_slot_name: String,
    peer_slot_name: String,
    local_spki_ref: String,
    peer_spki_ref: String,
    peer_image_integrity_ref: String,
    peer_checked_program_ref: String,
    peer_projection_ref: String,
    peer_cohort_provenance_ref: String,
}

impl std::fmt::Debug for Sys5I3TrustedLocalnetControl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3TrustedLocalnetControl")
            .field("local_slot_name", &self.local_slot_name)
            .field("peer_slot_name", &self.peer_slot_name)
            .finish_non_exhaustive()
    }
}

/// The non-semantic, mutually-authenticated transport preface.  It binds a
/// QUIC peer to the exact finite run/image cohort before any decoded carrier
/// is allowed to reach a semantic admission boundary.  Transport identity is
/// delivery evidence only; it never grants M8/M9 authority.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sys5I3LocalnetPeerPreface {
    run_ref: String,
    local_slot_name: String,
    peer_slot_name: String,
    local_spki_ref: String,
    image_integrity_ref: String,
    checked_program_ref: String,
    projection_ref: String,
    cohort_provenance_ref: String,
}

impl Sys5I3LocalnetPeerPreface {
    pub fn run_ref(&self) -> &str {
        &self.run_ref
    }

    /// Reference-only cohort binding for observer-safe delivery diagnostics.
    /// It is transport evidence, never an authority or admission grant.
    pub fn cohort_provenance_ref(&self) -> &str {
        &self.cohort_provenance_ref
    }

    pub fn local_slot_name(&self) -> &str {
        &self.local_slot_name
    }

    pub fn peer_slot_name(&self) -> &str {
        &self.peer_slot_name
    }

    pub fn local_spki_ref(&self) -> &str {
        &self.local_spki_ref
    }

    /// Component-only private-QUIC fixture. It creates no control, transport
    /// session, authority, or ingress capability.
    #[cfg(test)]
    pub(crate) fn test_only_private_quic_preface_fixture() -> Self {
        Self {
            run_ref: "i3-3-retained-ingress-run-a".to_string(),
            local_slot_name: "process-b".to_string(),
            peer_slot_name: "process-a".to_string(),
            local_spki_ref: "owner-spki:i3-3-retained-ingress".to_string(),
            image_integrity_ref: "owner-image-integrity:i3-3-retained-ingress".to_string(),
            checked_program_ref: "checked-program:i3-3-retained-ingress".to_string(),
            projection_ref: "projection:i3-3-retained-ingress".to_string(),
            cohort_provenance_ref: "cohort:i3-3-retained-ingress".to_string(),
        }
    }

    /// Component-only wrong-run preface derived from the exact fixture. It
    /// does not accept an externally chosen control field.
    #[cfg(test)]
    pub(crate) fn test_only_with_different_run_ref(mut self) -> Self {
        self.run_ref = "i3-3-retained-ingress-run-b".to_string();
        self
    }

    /// Component-only wrong-image binding derived from the exact fixture. It
    /// does not accept an externally chosen control field.
    #[cfg(test)]
    pub(crate) fn test_only_with_different_image_integrity_ref(mut self) -> Self {
        self.image_integrity_ref = "owner-image-integrity:other".to_string();
        self
    }
}

/// Typed, observer-safe failure for a trusted localnet control or peer
/// binding.  No raw image, certificate, key, capability, witness, or source
/// text is rendered from this boundary.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3LocalnetControlErrorKind {
    MalformedControl,
    StartBindingRejected,
    PeerBindingRejected,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3LocalnetControlError {
    kind: Sys5I3LocalnetControlErrorKind,
}

impl Sys5I3LocalnetControlError {
    fn new(kind: Sys5I3LocalnetControlErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3LocalnetControlErrorKind {
        self.kind
    }
}

impl Sys5I3TrustedLocalnetControl {
    pub fn local_slot_name(&self) -> &str {
        &self.local_slot_name
    }

    pub fn peer_slot_name(&self) -> &str {
        &self.peer_slot_name
    }

    /// Exact delivery pin for the mutually authenticated peer.  This is
    /// evidence-only transport configuration; it neither is nor derives a
    /// semantic authority, membership, capability, or witness.
    pub fn expected_peer_spki_ref(&self) -> &str {
        &self.peer_spki_ref
    }

    #[cfg_attr(not(feature = "i3-private-quic"), allow(dead_code))]
    pub(crate) fn run_ref(&self) -> &str {
        &self.run_ref
    }

    pub fn localnet_preface(&self) -> Sys5I3LocalnetPeerPreface {
        Sys5I3LocalnetPeerPreface {
            run_ref: self.run_ref.clone(),
            local_slot_name: self.local_slot_name.clone(),
            peer_slot_name: self.peer_slot_name.clone(),
            local_spki_ref: self.local_spki_ref.clone(),
            image_integrity_ref: self.expected_start_binding.image_integrity_ref.clone(),
            checked_program_ref: self
                .expected_start_binding
                .parent_checked_program_ref
                .clone(),
            projection_ref: self.expected_start_binding.projection_ref.clone(),
            cohort_provenance_ref: self.expected_start_binding.cohort_provenance_ref.clone(),
        }
    }

    /// Checks the preface component of the delivery-origin binding.  This is
    /// intentionally crate-private: a claimed preface alone is forgeable and
    /// cannot mint an ingress capability.  The private QUIC adapter combines
    /// this check with an inspected live mTLS connection and its owned bidi
    /// stream before it invokes decoded semantic admission.
    #[cfg_attr(not(feature = "i3-private-quic"), allow(dead_code))]
    pub(crate) fn validate_peer_preface(
        &self,
        received: &Sys5I3LocalnetPeerPreface,
    ) -> Result<(), Sys5I3LocalnetControlError> {
        if received.run_ref != self.run_ref
            || received.local_slot_name != self.peer_slot_name
            || received.peer_slot_name != self.local_slot_name
            || received.local_spki_ref != self.peer_spki_ref
            || received.image_integrity_ref != self.peer_image_integrity_ref
            || received.checked_program_ref != self.peer_checked_program_ref
            || received.projection_ref != self.peer_projection_ref
            || received.cohort_provenance_ref != self.peer_cohort_provenance_ref
        {
            return Err(Sys5I3LocalnetControlError::new(
                Sys5I3LocalnetControlErrorKind::PeerBindingRejected,
            ));
        }
        Ok(())
    }
}

/// Provider-only trusted QUIC control created after an inherited FD3 install.
/// It retains its complete local start binding and the peer's exact identity
/// descriptor, but no peer M8/M9 snapshot, native envelope, source text, or
/// provider outcome. Its presence is not proof of supervision; the supported
/// probe owner remains the T0 operational trust boundary for control delivery.
#[doc(hidden)]
pub struct Sys5I3TrustedProviderLocalnetControl {
    run_ref: String,
    local_role: Sys5I3ProviderChildRole,
    peer_role: Sys5I3ProviderChildRole,
    local_spki_ref: String,
    peer_spki_ref: String,
    expected_start_binding: Sys5I3ExpectedStartBinding,
    peer_start_binding: PrivateProviderPeerStartBinding,
    #[cfg(feature = "i3-process-test-seams")]
    fixed_provider_network_conformance_enabled: bool,
}

impl std::fmt::Debug for Sys5I3TrustedProviderLocalnetControl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3TrustedProviderLocalnetControl")
            .field("local_role", &self.local_role)
            .field("peer_role", &self.peer_role)
            .field("local_slot_name", &self.expected_start_binding.slot_name)
            .field("peer_slot_name", &self.peer_start_binding.slot_name)
            .finish_non_exhaustive()
    }
}

/// The provider-specific non-semantic transport preface. It binds the one
/// selected QUIC peer to exact role/image/static component correspondence
/// before a distinct provider carrier reaches local M9 admission.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sys5I3ProviderLocalnetPeerPreface {
    run_ref: String,
    local_role: Sys5I3ProviderChildRole,
    peer_role: Sys5I3ProviderChildRole,
    local_slot_name: String,
    peer_slot_name: String,
    local_spki_ref: String,
    image_integrity_ref: String,
    checked_program_ref: String,
    component_binding_ref: String,
}

impl Sys5I3TrustedProviderLocalnetControl {
    pub fn expected_peer_spki_ref(&self) -> &str {
        &self.peer_spki_ref
    }

    pub(crate) fn run_ref(&self) -> &str {
        &self.run_ref
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) const fn permits_fixed_provider_network_conformance(&self) -> bool {
        self.fixed_provider_network_conformance_enabled
    }

    pub(crate) fn localnet_preface(
        &self,
    ) -> Result<Sys5I3ProviderLocalnetPeerPreface, Sys5I3LocalnetControlError> {
        let component_binding_ref = self
            .expected_start_binding
            .inactive_provider
            .as_ref()
            .map(|provider| provider.component_binding_ref.clone())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                Sys5I3LocalnetControlError::new(Sys5I3LocalnetControlErrorKind::MalformedControl)
            })?;
        Ok(Sys5I3ProviderLocalnetPeerPreface {
            run_ref: self.run_ref.clone(),
            local_role: self.local_role,
            peer_role: self.peer_role,
            local_slot_name: self.expected_start_binding.slot_name.clone(),
            peer_slot_name: self.peer_start_binding.slot_name.clone(),
            local_spki_ref: self.local_spki_ref.clone(),
            image_integrity_ref: self.expected_start_binding.image_integrity_ref.clone(),
            checked_program_ref: checked_program_identity_ref(
                &self.expected_start_binding.parent_checked_program_ref,
            ),
            component_binding_ref,
        })
    }

    pub(crate) fn validate_peer_preface(
        &self,
        received: &Sys5I3ProviderLocalnetPeerPreface,
    ) -> Result<(), Sys5I3LocalnetControlError> {
        if received.run_ref != self.run_ref
            || received.local_role != self.peer_role
            || received.peer_role != self.local_role
            || received.local_slot_name != self.peer_start_binding.slot_name
            || received.peer_slot_name != self.expected_start_binding.slot_name
            || received.local_spki_ref != self.peer_spki_ref
            || received.image_integrity_ref != self.peer_start_binding.image_integrity_ref
            || !is_checked_program_identity_ref(&received.checked_program_ref)
            || received.checked_program_ref != self.peer_start_binding.parent_checked_program_ref
            || received.component_binding_ref != self.peer_start_binding.component_binding_ref
        {
            return Err(Sys5I3LocalnetControlError::new(
                Sys5I3LocalnetControlErrorKind::PeerBindingRejected,
            ));
        }
        Ok(())
    }
}

impl Sys5I3ExpectedStartBinding {
    fn for_image(image: &Sys5I3ProcessImage) -> Self {
        let seed = image
            .private_runtime_seed
            .ordinary()
            .expect("ordinary image retains ordinary private seed");
        Self {
            slot_name: image.slot_name.clone(),
            assigned_loci: image.assigned_loci.clone(),
            parent_checked_program_ref: image.child_seed.parent_checked_program_ref.clone(),
            projection_ref: image.child_seed.projection_ref.clone(),
            m9_generation_ref: image.child_seed.m9_generation_ref.clone(),
            cohort_provenance_ref: image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
                .to_string(),
            image_integrity_ref: image.private_integrity_ref.clone(),
            private_snapshot_binding_ref: seed.private_snapshot_binding_ref.clone(),
            expected_owner_capability_lifecycle: seed
                .prestaged_owner_capability_lifecycle
                .as_ref()
                .map(|lifecycle| Sys5I3ExpectedOwnerCapabilityLifecycle {
                    target_slot_name: lifecycle.target_slot_name.clone(),
                    stage_identity_binding_ref: lifecycle.stage_identity_binding_ref.clone(),
                    prior_generation_ref: lifecycle.candidate.prior_generation_ref().to_string(),
                    successor_generation_ref: lifecycle
                        .candidate
                        .successor_generation_ref()
                        .to_string(),
                    candidate_binding_ref: lifecycle.candidate.candidate_binding_ref().to_string(),
                }),
            inactive_provider: None,
        }
    }

    fn for_inactive_provider_image(image: &Sys5I3ProcessImage) -> Option<Self> {
        let provider = image.inactive_provider.as_ref()?;
        Some(Self {
            slot_name: image.slot_name.clone(),
            assigned_loci: image.assigned_loci.clone(),
            parent_checked_program_ref: provider.parent_checked_program_ref.clone(),
            projection_ref: String::new(),
            // A composite association never occupies this ordinary M9
            // generation field. Provider validation uses the explicit tagged
            // binding below instead.
            m9_generation_ref: String::new(),
            cohort_provenance_ref: String::new(),
            image_integrity_ref: image.private_integrity_ref.clone(),
            private_snapshot_binding_ref: String::new(),
            expected_owner_capability_lifecycle: None,
            inactive_provider: Some(Sys5I3ExpectedInactiveProviderBinding {
                static_snapshot: provider.static_snapshot.clone(),
                component_snapshot: provider.component_snapshot.clone(),
                legacy_authority_snapshot: provider.legacy_authority_snapshot.clone(),
                component_binding_ref: provider.component_binding_ref.clone(),
            }),
        })
    }

    fn validate_image(&self, image: &Sys5I3ProcessImage) -> Result<(), Sys5I3ProcessRuntimeError> {
        if image.inactive_provider.is_some() || self.inactive_provider.is_some() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        let seed = image.private_runtime_seed.ordinary().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        if self.cohort_provenance_ref
            != image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        if self.parent_checked_program_ref != image.child_seed.parent_checked_program_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if self.projection_ref != image.child_seed.projection_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        if self.m9_generation_ref != image.child_seed.m9_generation_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        if self.slot_name != image.slot_name
            || self.assigned_loci != image.assigned_loci
            || self.image_integrity_ref != image.private_integrity_ref
            || self.private_snapshot_binding_ref != seed.private_snapshot_binding_ref
            || self.expected_owner_capability_lifecycle
                != seed
                    .prestaged_owner_capability_lifecycle
                    .as_ref()
                    .map(|lifecycle| Sys5I3ExpectedOwnerCapabilityLifecycle {
                        target_slot_name: lifecycle.target_slot_name.clone(),
                        stage_identity_binding_ref: lifecycle.stage_identity_binding_ref.clone(),
                        prior_generation_ref: lifecycle
                            .candidate
                            .prior_generation_ref()
                            .to_string(),
                        successor_generation_ref: lifecycle
                            .candidate
                            .successor_generation_ref()
                            .to_string(),
                        candidate_binding_ref: lifecycle
                            .candidate
                            .candidate_binding_ref()
                            .to_string(),
                    })
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        Ok(())
    }

    /// Bind a decoded tagged provider image to the distinct expected-start
    /// record carried through inherited FD3. This is structural only: the M9
    /// role record is installed separately and no ordinary runtime starts.
    fn validate_inactive_provider_image(
        &self,
        image: &Sys5I3ProcessImage,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let expected = self.inactive_provider.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let provider = image.inactive_provider.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let seed = image
            .private_runtime_seed
            .inactive_provider()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                )
            })?;
        if image.private_integrity_ref != image.recomputed_private_integrity()
            || image.private_integrity_ref != self.image_integrity_ref
            || image.slot_name != self.slot_name
            || image.assigned_loci != self.assigned_loci
            || provider.assigned_loci != image.assigned_loci
            || provider.parent_checked_program_ref != self.parent_checked_program_ref
            || provider.parent_checked_program_ref != seed.parent_checked_program_ref
            || provider.static_snapshot != expected.static_snapshot
            || provider.static_snapshot != seed.static_snapshot
            || provider.component_snapshot != expected.component_snapshot
            || provider.component_snapshot != seed.component_snapshot
            || provider.legacy_authority_snapshot != expected.legacy_authority_snapshot
            || provider.legacy_authority_snapshot != seed.legacy_authority_snapshot
            || provider.component_binding_ref != expected.component_binding_ref
            || provider.component_binding_ref != seed.component_binding_ref
            || provider.static_snapshot.declared_provider_lowering_count() != 4
            || !provider.component_snapshot.is_scoped_component_snapshot()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        Ok(())
    }
}

impl Sys5I3ProcessCohortSummary {
    pub const fn full_admission_count(&self) -> usize {
        self.full_admission_count
    }

    pub const fn authority_generation_count(&self) -> usize {
        self.authority_generation_count
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn activation_occurrence_ref(&self) -> &str {
        &self.activation_occurrence_ref
    }

    pub fn cohort_occurrence_ref(&self) -> &str {
        &self.cohort_occurrence_ref
    }
}

/// A one-shot coordinator result.  It holds derived child images and
/// observer-safe construction facts.  It never retains a full projected
/// program or prepared admission; the sole exception is one private M9
/// publisher held by the restricted successor coordinator until its one
/// staged lifecycle either receives a qualified B completion or remains
/// incomplete.  No child image receives that publisher.
#[doc(hidden)]
pub struct Sys5I3ProcessCohort {
    images: BTreeMap<String, Option<Sys5I3ProcessImage>>,
    expected_start_bindings: BTreeMap<String, Option<Sys5I3ExpectedStartBinding>>,
    summary: Sys5I3ProcessCohortSummary,
    authority_successor_coordinator: Sys4I3OwnerCapabilitySuccessorCoordinator,
    published_authority_generation_ref: String,
    lifecycle_publication_outcome: Option<Sys5I3LifecyclePublicationOutcome>,
    prestage_run_ref: Option<String>,
    #[cfg(all(unix, feature = "i3-process-test-seams"))]
    pending_owner_lifecycle_ack_registration: Option<Sys5I3RegisteredOwnerLifecycleAckRegistration>,
    #[cfg(all(unix, feature = "i3-process-test-seams"))]
    pending_owner_lifecycle_stage_identity_binding_ref: Option<String>,
}

/// Stage 2c's parent-held provider-image cohort. The actual M9 composite and
/// current setup association remain in this parent object; images carry only
/// tainted static/descriptive candidates and cannot start a runtime.
#[doc(hidden)]
pub(crate) struct Sys5I3InactiveProviderCohort {
    images: BTreeMap<String, Option<Sys5I3ProcessImage>>,
    expected_start_bindings: BTreeMap<String, Option<Sys5I3ExpectedStartBinding>>,
    checked: mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
    admission: Sys4InactiveProviderAdmission,
    setup_context: Arc<TrustedFixtureContext>,
    #[cfg(test)]
    inactive_validation_pause: Option<Sys5I3InactiveProviderValidationPause>,
}

/// The only two source-derived provider child roles. This routing descriptor
/// names no grant, result, source, or resource authority.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Sys5I3ProviderChildRole {
    RequesterConsumer,
    Executor,
}

impl Sys5I3ProviderChildRole {
    fn m9_role(self) -> M9I3ReadOnlyProviderChildRole {
        match self {
            Self::RequesterConsumer => M9I3ReadOnlyProviderChildRole::RequesterConsumer,
            Self::Executor => M9I3ReadOnlyProviderChildRole::Executor,
        }
    }

    fn from_m9_role(role: M9I3ReadOnlyProviderChildRole) -> Self {
        match role {
            M9I3ReadOnlyProviderChildRole::RequesterConsumer => Self::RequesterConsumer,
            M9I3ReadOnlyProviderChildRole::Executor => Self::Executor,
        }
    }
}

/// Non-authorizing transport route selected from an already checked provider
/// cohort. It deliberately contains neither image bytes nor trusted control.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderChildRoute {
    role: Sys5I3ProviderChildRole,
    slot_name: String,
    endpoint: String,
}

impl Sys5I3ProviderChildRoute {
    pub const fn role(&self) -> Sys5I3ProviderChildRole {
        self.role
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// Runtime-only result of consuming the real inherited provider FD3 frame.
/// It has no constructor or authority/control payload accessor.
#[doc(hidden)]
pub struct Sys5I3InheritedProviderChildControl {
    transport_bootstrap: Vec<u8>,
    role: Sys5I3ProviderChildRole,
    expected_start_binding: Sys5I3ExpectedStartBinding,
    peer_role: Sys5I3ProviderChildRole,
    peer_start_binding: PrivateProviderPeerStartBinding,
    m9_role_snapshot: M9I3PrivateReadOnlyProviderRoleSnapshot,
    expected_resource_runtime_nonce: [u8; 32],
    executor_resource_envelope: Option<I3PrivateReadOnlyProviderResourceEnvelope>,
    requester_fixture_assertion: Option<I3PrivateReadOnlyProviderFixtureAssertion>,
    terminal_observation_conformance_profile:
        Option<I3PrivateFixedTerminalObservationConformanceProfile>,
    provider_network_conformance_profile: Option<I3PrivateFixedProviderNetworkConformanceProfile>,
}

impl std::fmt::Debug for Sys5I3InheritedProviderChildControl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3InheritedProviderChildControl(..)")
    }
}

impl Sys5I3InheritedProviderChildControl {
    pub const fn role(&self) -> Sys5I3ProviderChildRole {
        self.role
    }

    /// This is the sole probe-visible part of a provider FD3 frame. It is
    /// restricted to physical transport bootstrap metadata and carries no M9
    /// fact, resource envelope, path, request, or result.
    pub fn transport_bootstrap(&self) -> &[u8] {
        &self.transport_bootstrap
    }
}

/// Installed child-local provider runtime. It is populated only after a
/// trusted inherited-control read and remains opaque to the probe.
#[doc(hidden)]
pub struct Sys5I3InstalledProviderChildRuntime {
    role: Sys5I3ProviderChildRole,
    expected_start_binding: Sys5I3ExpectedStartBinding,
    peer_role: Sys5I3ProviderChildRole,
    peer_start_binding: PrivateProviderPeerStartBinding,
    local_authority: M9I3ReadOnlyProviderLocalAuthority,
    _scoped_component: M8InstalledReadOnlyProviderEffectComponent,
    provider_local_fabric: Sys4InstalledProviderLocalFabric,
    executor_resource_envelope: Option<I3PrivateReadOnlyProviderResourceEnvelope>,
    requester_fixture_assertion: Option<I3PrivateReadOnlyProviderFixtureAssertion>,
    fixture_post_consume_assertion_completed: bool,
    provider_execution: Sys5I3ProviderExecution,
    terminal_observation_export_reserved: bool,
    #[cfg(feature = "i3-process-test-seams")]
    terminal_observation_conformance_profile:
        Option<I3PrivateFixedTerminalObservationConformanceProfile>,
    #[cfg(feature = "i3-process-test-seams")]
    terminal_observation_conformance_completed: bool,
    #[cfg(feature = "i3-process-test-seams")]
    provider_network_conformance_profile: Option<I3PrivateFixedProviderNetworkConformanceProfile>,
    #[cfg(feature = "i3-process-test-seams")]
    provider_network_conformance_facts: Sys5I3FixedProviderNetworkConformanceFacts,
    #[cfg(feature = "i3-process-test-seams")]
    provider_network_conformance_completed: bool,
}

/// Opaque completion of one sealed terminal-observation conformance
/// experiment. It carries no experiment selector, M9 diagnostic, audit,
/// result, reference, count, or proof of nonexecution.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub struct Sys5I3FixedTerminalObservationConformanceCompletion {
    _private: (),
}

#[cfg(feature = "i3-process-test-seams")]
impl std::fmt::Debug for Sys5I3FixedTerminalObservationConformanceCompletion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3FixedTerminalObservationConformanceCompletion(..)")
    }
}

/// Opaque completion of one sealed provider-network conformance experiment.
/// It discloses neither the selected fault profile nor a result, carrier,
/// occurrence, authority, or evidence record.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub struct Sys5I3FixedProviderNetworkConformanceCompletion {
    _private: (),
}

#[cfg(feature = "i3-process-test-seams")]
impl std::fmt::Debug for Sys5I3FixedProviderNetworkConformanceCompletion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3FixedProviderNetworkConformanceCompletion(..)")
    }
}

#[cfg(feature = "i3-process-test-seams")]
#[derive(Default)]
struct Sys5I3FixedProviderNetworkConformanceFacts {
    first_result_send_withheld: bool,
    first_result_send_completed: bool,
    first_result_receive_finished_without_frame: bool,
    first_result_consumed: bool,
    first_result_received_and_discarded_before_consume: bool,
    second_request_duplicate_rejected: bool,
    second_result_send_completed: bool,
    second_result_receive_finished_without_frame: bool,
    second_result_consumed: bool,
    second_result_duplicate_rejected: bool,
    effect_retired_after_call: bool,
    second_result_send_currentness_rejected: bool,
}

#[cfg(feature = "i3-process-test-seams")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3FixedProviderNetworkFirstResultSendAction {
    Withhold,
    Send,
}

#[cfg(feature = "i3-process-test-seams")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3FixedProviderNetworkFirstResultReceiveAction {
    ExpectFinishedWithoutFrame,
    Consume,
    ReadAndDiscardBeforeConsume,
}

#[cfg(feature = "i3-process-test-seams")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3FixedProviderNetworkSecondResultSendAction {
    Send,
    FinishWithoutFrame,
    ExpectCurrentnessRejection,
}

#[cfg(feature = "i3-process-test-seams")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3FixedProviderNetworkSecondResultReceiveAction {
    Consume,
    ExpectFinishedWithoutFrame,
    ExpectDuplicateRejection,
}

impl std::fmt::Debug for Sys5I3InstalledProviderChildRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3InstalledProviderChildRuntime(..)")
    }
}

/// A source-real local component pair for focused runtime-boundary tests.
/// It retains the trusted setup borrow and constructs the same restricted M9,
/// scoped M8, SYS-4, and provider-execution parts as an installed child, but
/// it neither reads FD3 nor represents an inherited-control installation.
/// The supervised probe remains the sole evidence path for full child install
/// and transport behavior.
#[cfg(test)]
pub(crate) struct Sys5I3SourceRealProviderLocalExecutionPair<'setup> {
    _setup_guard: &'setup I3TrustedReadOnlyProviderFixtureSetup,
    requester: Sys5I3ProviderLocalExecution,
    executor: Sys5I3ProviderLocalExecution,
}

#[cfg(test)]
impl<'setup> Sys5I3SourceRealProviderLocalExecutionPair<'setup> {
    pub(crate) fn requester_mut(&mut self) -> &mut Sys5I3ProviderLocalExecution {
        &mut self.requester
    }

    pub(crate) fn executor_mut(&mut self) -> &mut Sys5I3ProviderLocalExecution {
        &mut self.executor
    }
}

/// One local execution boundary built only from genuine restricted provider
/// parts. This is not an installed runtime and deliberately has no image,
/// control, transport, or authority constructor exposed to tests.
#[cfg(test)]
pub(crate) struct Sys5I3ProviderLocalExecution {
    role: Sys5I3ProviderChildRole,
    local_authority: M9I3ReadOnlyProviderLocalAuthority,
    _scoped_component: M8InstalledReadOnlyProviderEffectComponent,
    provider_local_fabric: Sys4InstalledProviderLocalFabric,
    executor_resource_envelope: Option<I3PrivateReadOnlyProviderResourceEnvelope>,
    provider_execution: Sys5I3ProviderExecution,
}

#[cfg(test)]
impl Sys5I3ProviderLocalExecution {
    pub(crate) fn begin_read_only_provider_request(
        &mut self,
    ) -> Result<Sys5I3ProviderRequestCarrier, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .begin_request(self.role, &mut self.local_authority)
    }

    pub(crate) fn admit_read_only_provider_request_and_execute(
        &mut self,
        request: Sys5I3ProviderRequestCarrier,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3ProcessRuntimeError> {
        self.provider_execution.admit_request_and_execute(
            self.role,
            &mut self.local_authority,
            self.executor_resource_envelope.as_ref(),
            request,
        )
    }

    pub(crate) fn admit_read_only_provider_result_and_consume(
        &mut self,
        result: Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3ProcessRuntimeError> {
        self.provider_execution.admit_result_and_consume(
            self.role,
            &mut self.local_authority,
            result,
        )
    }

    pub(crate) fn has_no_owner_execution_occurrences(&self) -> bool {
        self.provider_local_fabric
            .has_no_owner_execution_occurrences()
    }

    #[cfg(all(test, feature = "i3-process-test-seams"))]
    pub(crate) fn configure_revalidation_test_retirement(
        &mut self,
        point: Sys5I3ProviderRevalidationTestPoint,
        retirement: Sys5I3ProviderRevalidationTestRetirement,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .configure_revalidation_test_retirement(self.role, point, retirement)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn test_only_ledger_facts(&self) -> Sys5I3ProviderLedgerTestFacts {
        self.provider_execution.test_only_ledger_facts()
    }
}

impl Sys5I3InstalledProviderChildRuntime {
    /// Begin the distinct requester-to-executor provider carrier. This is
    /// intentionally not an owner request or a generated owner message.
    pub fn begin_read_only_provider_request(
        &mut self,
    ) -> Result<Sys5I3ProviderRequestCarrier, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .begin_request(self.role, &mut self.local_authority)
    }

    /// Admit a distinct provider request at the executor, reserve its only
    /// bounded entry, synchronously invoke the B-local adapter after
    /// CallStarted, then retain a typed result before release.
    pub fn admit_read_only_provider_request_and_execute(
        &mut self,
        request: Sys5I3ProviderRequestCarrier,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3ProcessRuntimeError> {
        self.provider_execution.admit_request_and_execute(
            self.role,
            &mut self.local_authority,
            self.executor_resource_envelope.as_ref(),
            request,
        )
    }

    /// Crate-private QUIC ingress. The adapter has already completed one
    /// physical stream read and allocated a run/session/direction/ordinal
    /// occurrence without inspecting the carrier. This method binds that
    /// occurrence only after exact static carrier admission and before the
    /// executor crosses the local host boundary.
    pub(crate) fn admit_read_only_provider_request_and_execute_from_transport(
        &mut self,
        request: Sys5I3ProviderRequestCarrier,
        request_receive: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .admit_request_and_execute_with_transport_occurrence(
                self.role,
                &mut self.local_authority,
                self.executor_resource_envelope.as_ref(),
                request,
                Some(request_receive),
            )
    }

    /// Admit and consume a distinct provider result exactly once. The receipt
    /// has no raw value or carrier accessor.
    pub fn admit_read_only_provider_result_and_consume(
        &mut self,
        result: Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3ProcessRuntimeError> {
        self.provider_execution.admit_result_and_consume(
            self.role,
            &mut self.local_authority,
            result,
        )
    }

    /// Crate-private QUIC ingress for the one completed result read. The
    /// opaque occurrence is retained only when this current requester-side
    /// consume accepts the exact carrier; decoded bytes never issue it.
    pub(crate) fn admit_read_only_provider_result_and_consume_from_transport(
        &mut self,
        result: Sys5I3ProviderResultCarrier,
        result_receive: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .admit_result_and_consume_with_transport_occurrence(
                self.role,
                &mut self.local_authority,
                result,
                Some(result_receive),
            )
    }

    /// Reserve exactly one requester send occurrence before the provider
    /// stream write. The returned opaque ticket can complete only that same
    /// retained request/slot after the adapter's complete write succeeds.
    pub(crate) fn reserve_provider_request_transport_send(
        &mut self,
        request: &Sys5I3ProviderRequestCarrier,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .reserve_request_send_transport_occurrence(self.role, request, occurrence)
    }

    /// Reserve exactly one executor result-send occurrence only after M9
    /// currentness has been revalidated against the retained released result.
    pub(crate) fn reserve_provider_result_transport_send(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .reserve_result_send_transport_occurrence(
                self.role,
                &mut self.local_authority,
                result,
                occurrence,
            )
    }

    /// Record one completed provider stream write at the ticket's retained
    /// slot. A ticket is move-only and cannot issue a second frame.
    pub(crate) fn complete_provider_transport_send(
        &mut self,
        ticket: Sys5I3PrivateProviderTransportWriteTicket,
        completed: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.provider_execution
            .complete_provider_transport_write(self.role, ticket, completed)
    }

    /// Complete the finite requester-side fixture assertion only after the
    /// current A-local consume retained this exact receipt. The expectation is
    /// carried in A's trusted inherited control; callers cannot supply a
    /// value, outcome, path, or fixture selector.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn complete_fixture_post_consume_assertion(
        &mut self,
        receipt: &Sys5I3ProviderConsumeReceipt,
    ) -> Result<Sys5I3ProviderFixtureAssertionCompletion, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if self.fixture_post_consume_assertion_completed {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ResourceExhausted,
            ));
        }
        let expected = self.requester_fixture_assertion.ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        let completion = self
            .provider_execution
            .verifies_fixture_post_consume_assertion(receipt, expected)?;
        if !self
            .provider_local_fabric
            .has_no_owner_execution_occurrences()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.fixture_post_consume_assertion_completed = true;
        Ok(completion)
    }

    /// Report only whether this installed child inherited one sealed
    /// provider-network conformance profile. The probe uses this routing bit
    /// before choosing its fixed two-session schedule; it exposes neither the
    /// selected profile nor any authority, carrier, result, or occurrence.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn has_fixed_provider_network_conformance_profile(&self) -> bool {
        self.provider_network_conformance_profile.is_some()
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn fixed_provider_network_first_result_send_action(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3FixedProviderNetworkFirstResultSendAction, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor
            || !self.provider_execution.has_exact_released_executor_facts()
            || self
                .provider_execution
                .prepare_released_result_replay(self.role, result)
                .is_err()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        match self.fixed_provider_network_profile()? {
            I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession => {
                self.provider_network_conformance_facts.first_result_send_withheld = true;
                Ok(Sys5I3FixedProviderNetworkFirstResultSendAction::Withhold)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
            | I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession => {
                Ok(Sys5I3FixedProviderNetworkFirstResultSendAction::Send)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend => {
                self.local_authority.retire_effect_authorization().map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
                self.provider_network_conformance_facts.first_result_send_withheld = true;
                self.provider_network_conformance_facts.effect_retired_after_call = true;
                Ok(Sys5I3FixedProviderNetworkFirstResultSendAction::Withhold)
            }
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_first_result_sent(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor
            || !matches!(
                self.fixed_provider_network_profile()?,
                I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
                    | I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
            )
            || !self
                .provider_execution
                .has_completed_transport_slot(self.role, result, 0)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .first_result_send_completed = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn fixed_provider_network_first_result_receive_action(
        &self,
    ) -> Result<Sys5I3FixedProviderNetworkFirstResultReceiveAction, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        match self.fixed_provider_network_profile()? {
            I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession
            | I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend => {
                Ok(Sys5I3FixedProviderNetworkFirstResultReceiveAction::ExpectFinishedWithoutFrame)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume => {
                Ok(
                    Sys5I3FixedProviderNetworkFirstResultReceiveAction::ReadAndDiscardBeforeConsume,
                )
            }
            I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession => {
                Ok(Sys5I3FixedProviderNetworkFirstResultReceiveAction::Consume)
            }
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_first_result_finished_without_frame(
        &mut self,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || !matches!(
                self.fixed_provider_network_profile()?,
                I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession
                    | I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend
            )
            || !self.provider_execution.has_exact_requester_pending_facts()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .first_result_receive_finished_without_frame = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_first_result_received_and_discarded_before_consume(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
            || !self.provider_execution.has_exact_requester_pending_facts()
            || !self
                .provider_execution
                .matches_exact_requester_pending_result(result)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .first_result_received_and_discarded_before_consume = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_first_result_consumed(
        &mut self,
        receipt: &Sys5I3ProviderConsumeReceipt,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.complete_fixture_post_consume_assertion(receipt)?;
        self.provider_network_conformance_facts
            .first_result_consumed = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn prepare_fixed_provider_network_second_request_replay(
        &self,
    ) -> Result<Option<Sys5I3PrivateProviderConsumedRequestReplay>, Sys5I3ProcessRuntimeError> {
        let profile = self.fixed_provider_network_profile()?;
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if profile != I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession {
            return Ok(None);
        }
        if !self
            .provider_network_conformance_facts
            .first_result_consumed
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_execution
            .prepare_consumed_request_replay(self.role)
            .map(Some)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn reserve_fixed_provider_network_second_request_send(
        &mut self,
        replay: &Sys5I3PrivateProviderConsumedRequestReplay,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
            || !self.provider_network_conformance_facts.first_result_consumed
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_execution
            .reserve_consumed_request_replay_transport_occurrence(self.role, replay, occurrence)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_request_duplicate_rejected(
        &mut self,
        request: &Sys5I3ProviderRequestCarrier,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
            || !self.provider_execution.has_exact_released_request(request)
            || !self.provider_execution.has_exact_released_executor_facts()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .second_request_duplicate_rejected = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn expects_fixed_provider_network_second_request_replay(
        &self,
    ) -> Result<bool, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        Ok(self.fixed_provider_network_profile()?
            == I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn fixed_provider_network_second_result_send_action(
        &self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3FixedProviderNetworkSecondResultSendAction, Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor
            || self
                .provider_execution
                .prepare_released_result_replay(self.role, result)
                .is_err()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        match self.fixed_provider_network_profile()? {
            I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession
                if self.provider_network_conformance_facts.first_result_send_withheld =>
            {
                Ok(Sys5I3FixedProviderNetworkSecondResultSendAction::Send)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
                if self.provider_network_conformance_facts.first_result_send_completed =>
            {
                Ok(Sys5I3FixedProviderNetworkSecondResultSendAction::FinishWithoutFrame)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
                if self.provider_network_conformance_facts.first_result_send_completed
                    && self
                        .provider_network_conformance_facts
                        .second_request_duplicate_rejected =>
            {
                Ok(Sys5I3FixedProviderNetworkSecondResultSendAction::Send)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend
                if self.provider_network_conformance_facts.first_result_send_withheld
                    && self.provider_network_conformance_facts.effect_retired_after_call =>
            {
                Ok(Sys5I3FixedProviderNetworkSecondResultSendAction::ExpectCurrentnessRejection)
            }
            _ => Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            )),
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn prepare_fixed_provider_network_second_result_replay(
        &self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3PrivateProviderReleasedResultReplay, Sys5I3ProcessRuntimeError> {
        match self.fixed_provider_network_second_result_send_action(result)? {
            Sys5I3FixedProviderNetworkSecondResultSendAction::Send
            | Sys5I3FixedProviderNetworkSecondResultSendAction::ExpectCurrentnessRejection => {}
            Sys5I3FixedProviderNetworkSecondResultSendAction::FinishWithoutFrame => {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
        }
        self.provider_execution
            .prepare_released_result_replay(self.role, result)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn reserve_fixed_provider_network_result_send(
        &mut self,
        replay: &Sys5I3PrivateProviderReleasedResultReplay,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        self.reserve_provider_result_transport_send(replay.result(), occurrence)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_result_sent(
        &mut self,
        replay: &Sys5I3PrivateProviderReleasedResultReplay,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let expected_slot = match self.fixed_provider_network_profile()? {
            I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession => 0,
            I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession => 1,
            _ => {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
        };
        if self.role != Sys5I3ProviderChildRole::Executor
            || !self.provider_execution.has_completed_transport_slot(
                self.role,
                replay.result(),
                expected_slot,
            )
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .second_result_send_completed = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_result_currentness_rejected(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::Executor
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend
            || !self.provider_network_conformance_facts.effect_retired_after_call
            || !self
                .provider_execution
                .has_empty_result_transport_slot(result, 0)
            || !self
                .provider_execution
                .has_empty_result_transport_slot(result, 1)
            || !self.provider_execution.has_exact_released_executor_facts()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .second_result_send_currentness_rejected = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn fixed_provider_network_second_result_receive_action(
        &self,
    ) -> Result<Sys5I3FixedProviderNetworkSecondResultReceiveAction, Sys5I3ProcessRuntimeError>
    {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        match self.fixed_provider_network_profile()? {
            I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession => {
                Ok(Sys5I3FixedProviderNetworkSecondResultReceiveAction::Consume)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
            | I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend => {
                Ok(Sys5I3FixedProviderNetworkSecondResultReceiveAction::ExpectFinishedWithoutFrame)
            }
            I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession => {
                Ok(Sys5I3FixedProviderNetworkSecondResultReceiveAction::ExpectDuplicateRejection)
            }
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_result_finished_without_frame(
        &mut self,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || !matches!(
                self.fixed_provider_network_profile()?,
                I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume
                    | I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend
            )
            || !self.provider_execution.has_exact_requester_pending_facts()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .second_result_receive_finished_without_frame = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_result_consumed(
        &mut self,
        receipt: &Sys5I3ProviderConsumeReceipt,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession
            || !self
                .provider_network_conformance_facts
                .first_result_receive_finished_without_frame
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.complete_fixture_post_consume_assertion(receipt)?;
        self.provider_network_conformance_facts
            .second_result_consumed = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn record_fixed_provider_network_second_result_duplicate_rejected(
        &mut self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.role != Sys5I3ProviderChildRole::RequesterConsumer
            || self.fixed_provider_network_profile()?
                != I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession
            || !self.provider_network_conformance_facts.first_result_consumed
            || !self.provider_execution.has_exact_consumed_result(result)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_facts
            .second_result_duplicate_rejected = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn complete_fixed_provider_network_conformance_if_selected(
        &mut self,
    ) -> Result<Option<Sys5I3FixedProviderNetworkConformanceCompletion>, Sys5I3ProcessRuntimeError>
    {
        let Some(profile) = self.provider_network_conformance_profile else {
            return Ok(None);
        };
        if self.provider_network_conformance_completed
            || self.terminal_observation_export_reserved
            || !self
                .provider_local_fabric
                .has_no_owner_execution_occurrences()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let facts = &self.provider_network_conformance_facts;
        let completed = match (self.role, profile) {
            (
                Sys5I3ProviderChildRole::Executor,
                I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession,
            ) => {
                facts.first_result_send_withheld
                    && facts.second_result_send_completed
                    && self.provider_execution.has_exact_released_executor_facts()
            }
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                I3PrivateFixedProviderNetworkConformanceProfile::HeldResultFirstSendOnSecondSession,
            ) => {
                facts.first_result_receive_finished_without_frame
                    && facts.second_result_consumed
                    && self.fixture_post_consume_assertion_completed
                    && self
                        .provider_execution
                        .has_completed_terminal_facts_for_role(self.role)
            }
            (
                Sys5I3ProviderChildRole::Executor,
                I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume,
            ) => {
                facts.first_result_send_completed
                    && self.provider_execution.has_exact_released_executor_facts()
            }
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                I3PrivateFixedProviderNetworkConformanceProfile::SentResultLostBeforeConsume,
            ) => {
                facts.first_result_received_and_discarded_before_consume
                    && facts.second_result_receive_finished_without_frame
                    && self.provider_execution.has_exact_requester_pending_facts()
            }
            (
                Sys5I3ProviderChildRole::Executor,
                I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession,
            ) => {
                facts.first_result_send_completed
                    && facts.second_request_duplicate_rejected
                    && facts.second_result_send_completed
                    && self.provider_execution.has_exact_released_executor_facts()
            }
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                I3PrivateFixedProviderNetworkConformanceProfile::DuplicateProviderDeliveryOnSecondSession,
            ) => {
                facts.first_result_consumed
                    && facts.second_result_duplicate_rejected
                    && self.fixture_post_consume_assertion_completed
                    && self
                        .provider_execution
                        .has_completed_terminal_facts_for_role(self.role)
            }
            (
                Sys5I3ProviderChildRole::Executor,
                I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend,
            ) => {
                facts.first_result_send_withheld
                    && facts.effect_retired_after_call
                    && facts.second_result_send_currentness_rejected
                    && self.provider_execution.has_exact_released_executor_facts()
            }
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                I3PrivateFixedProviderNetworkConformanceProfile::PostCallEffectRetiredBeforeHeldResultSend,
            ) => {
                facts.first_result_receive_finished_without_frame
                    && facts.second_result_receive_finished_without_frame
                    && self.provider_execution.has_exact_requester_pending_facts()
            }
        };
        if !completed {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let audit_error = match self.provider_terminal_audit() {
            Ok(_) => {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
            Err(error) => error,
        };
        if audit_error.kind() != Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected
            || self.terminal_observation_export_reserved
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        self.provider_network_conformance_completed = true;
        Ok(Some(Sys5I3FixedProviderNetworkConformanceCompletion {
            _private: (),
        }))
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn fixed_provider_network_profile(
        &self,
    ) -> Result<I3PrivateFixedProviderNetworkConformanceProfile, Sys5I3ProcessRuntimeError> {
        self.provider_network_conformance_profile.ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })
    }

    /// Run exactly one sealed, source-real terminal-observation conformance
    /// experiment after normal installed-child execution. A launch without a
    /// profile returns `None` and must follow the ordinary terminal-audit
    /// path; callers cannot use this method to make an ordinary denied audit
    /// look like a successful experiment.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn complete_fixed_terminal_observation_conformance_if_selected(
        &mut self,
    ) -> Result<
        Option<Sys5I3FixedTerminalObservationConformanceCompletion>,
        Sys5I3ProcessRuntimeError,
    > {
        let Some(profile) = self.terminal_observation_conformance_profile else {
            return Ok(None);
        };
        if self.terminal_observation_conformance_completed
            || !self
                .provider_execution
                .has_completed_terminal_facts_for_role(self.role)
            || (self.role == Sys5I3ProviderChildRole::RequesterConsumer
                && !self.fixture_post_consume_assertion_completed)
            || !self
                .provider_local_fabric
                .has_no_owner_execution_occurrences()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }

        match profile {
            I3PrivateFixedTerminalObservationConformanceProfile::RetireBeforeInitialPreflight => {
                self.local_authority
                    .retire_terminal_observation_authorization()
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                        )
                    })?;
                let error = match self.provider_terminal_audit() {
                    Ok(_) => {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                        ));
                    }
                    Err(error) => error,
                };
                if error.kind() != Sys5I3ProcessRuntimeErrorKind::MissingCapability
                    || self.terminal_observation_export_reserved
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    ));
                }
            }
            I3PrivateFixedTerminalObservationConformanceProfile::RetireAfterProjectionBeforeCommit => {
                let error = match self.provider_terminal_audit() {
                    Ok(_) => {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                        ));
                    }
                    Err(error) => error,
                };
                if error.kind() != Sys5I3ProcessRuntimeErrorKind::MissingCapability
                    || !self.terminal_observation_export_reserved
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    ));
                }
            }
            I3PrivateFixedTerminalObservationConformanceProfile::RepeatExport => {
                let audit = self.provider_terminal_audit()?;
                audit.encode_observer_view_body().map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
                let error = match self.provider_terminal_audit() {
                    Ok(_) => {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                        ));
                    }
                    Err(error) => error,
                };
                if error.kind() != Sys5I3ProcessRuntimeErrorKind::ResourceExhausted
                    || !self.terminal_observation_export_reserved
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    ));
                }
            }
            I3PrivateFixedTerminalObservationConformanceProfile::EffectRetiredObserverCurrent => {
                self.local_authority.retire_effect_authorization().map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
                // Retiring effect use after the retained call must reject an
                // actual next use at its ordinary local M9 boundary.  The
                // failed attempt is deliberately before any new ledger
                // reservation, adapter entry, or consume fact; terminal
                // observation stays separately current for the already
                // retained terminal fact.
                let facts_before = self.provider_execution.test_only_ledger_facts();
                let error = match self
                    .provider_execution
                    .attempt_post_effect_retirement_provider_use(
                        self.role,
                        &mut self.local_authority,
                    ) {
                    Ok(()) => {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                        ));
                    }
                    Err(error) => error,
                };
                if error.kind() != Sys5I3ProcessRuntimeErrorKind::MissingCapability
                    || facts_before != self.provider_execution.test_only_ledger_facts()
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    ));
                }
                let audit = self.provider_terminal_audit()?;
                audit.encode_observer_view_body().map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
            }
        }

        self.terminal_observation_conformance_completed = true;
        Ok(Some(Sys5I3FixedTerminalObservationConformanceCompletion {
            _private: (),
        }))
    }

    /// Project one fixed, reference-only terminal audit only after the
    /// separate installed M9 Observation lineage is current for this exact
    /// provider carrier. Effect-use authority is neither checked nor reused
    /// here: a retained post-call fact can remain observable after effect
    /// retirement when membership, contract, observer policy, and profile
    /// correspondence are still current.
    pub fn provider_terminal_audit(
        &mut self,
    ) -> Result<Sys5I3ProviderTerminalAudit, Sys5I3ProcessRuntimeError> {
        #[cfg(feature = "i3-process-test-seams")]
        if self.provider_network_conformance_profile.is_some() {
            // A sealed two-session conformance witness has no truthful normal
            // v2 terminal-audit chronology. It must complete without spending
            // an observer export allowance or emitting an audit candidate.
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if self.terminal_observation_export_reserved {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ResourceExhausted,
            ));
        }
        if !self
            .provider_local_fabric
            .has_no_owner_execution_occurrences()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let permit = self
            .local_authority
            .preflight_provider_terminal_observation_export(
                self.provider_execution.provider_carrier_binding(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::MissingCapability)
            })?;
        if !permit.permits_fixed_v2_terminal_export() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::MissingCapability,
            ));
        }
        // Spend the one fixed export allowance before allocating or encoding
        // the projection. A later projection or commit-currentness failure
        // deliberately does not refund it, so repeated attempts cannot turn
        // observer work into an unbounded retry surface.
        self.terminal_observation_export_reserved = true;
        let audit = self
            .provider_execution
            .terminal_audit_projection(self.role)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        #[cfg(feature = "i3-process-test-seams")]
        if self.terminal_observation_conformance_profile
            == Some(
                I3PrivateFixedTerminalObservationConformanceProfile::RetireAfterProjectionBeforeCommit,
            )
        {
            self.local_authority
                .retire_terminal_observation_authorization()
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
        }
        let commit_permit = self
            .local_authority
            .preflight_provider_terminal_observation_export(
                self.provider_execution.provider_carrier_binding(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::MissingCapability)
            })?;
        if !commit_permit.permits_fixed_v2_terminal_export() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::MissingCapability,
            ));
        }
        Ok(audit)
    }

    /// Bind only physical QUIC identity facts parsed by the provider child
    /// from its inherited transport blob. The installed runtime is the sole
    /// producer of this control, so a generic decoded image/control cannot
    /// create an executable provider transport session.
    pub fn bind_provider_localnet_transport(
        &self,
        run_ref: &str,
        local_spki_ref: &str,
        peer_spki_ref: &str,
    ) -> Result<Sys5I3TrustedProviderLocalnetControl, Sys5I3LocalnetControlError> {
        if run_ref.is_empty()
            || local_spki_ref.is_empty()
            || peer_spki_ref.is_empty()
            || local_spki_ref == peer_spki_ref
            || self.role == self.peer_role
            || self.expected_start_binding.slot_name == self.peer_start_binding.slot_name
            || self.expected_start_binding.inactive_provider.is_none()
            || !self.peer_start_binding.is_structurally_valid()
        {
            return Err(Sys5I3LocalnetControlError::new(
                Sys5I3LocalnetControlErrorKind::MalformedControl,
            ));
        }
        Ok(Sys5I3TrustedProviderLocalnetControl {
            run_ref: run_ref.to_string(),
            local_role: self.role,
            peer_role: self.peer_role,
            local_spki_ref: local_spki_ref.to_string(),
            peer_spki_ref: peer_spki_ref.to_string(),
            expected_start_binding: self.expected_start_binding.clone(),
            peer_start_binding: self.peer_start_binding.clone(),
            #[cfg(feature = "i3-process-test-seams")]
            fixed_provider_network_conformance_enabled: self
                .provider_network_conformance_profile
                .is_some(),
        })
    }
}

/// Opaque input to the sole supervised provider-localnet launcher. It owns
/// the real setup guard through child reaping and exposes only one-shot
/// privileged image/control operations, never authority or resource payloads.
#[doc(hidden)]
pub struct Sys5I3PreparedProviderLocalnetLaunch {
    cohort: Sys5I3InactiveProviderCohort,
    setup: I3TrustedReadOnlyProviderFixtureSetup,
    child_routes: BTreeMap<Sys5I3ProviderChildRole, Sys5I3ProviderChildRoute>,
    provider_peer_expected_start_bindings:
        BTreeMap<Sys5I3ProviderChildRole, Sys5I3ExpectedStartBinding>,
    image_written_roles: BTreeSet<Sys5I3ProviderChildRole>,
    control_attempted_roles: BTreeSet<Sys5I3ProviderChildRole>,
}

impl std::fmt::Debug for Sys5I3PreparedProviderLocalnetLaunch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3PreparedProviderLocalnetLaunch(..)")
    }
}

impl Sys5I3PreparedProviderLocalnetLaunch {
    pub(crate) fn from_inactive_provider_cohort(
        cohort: Sys5I3InactiveProviderCohort,
        setup: I3TrustedReadOnlyProviderFixtureSetup,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if !setup.matches_live_context(&cohort.setup_context)
            || !cohort.admission.has_current_composite_seal()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        let mut child_routes = BTreeMap::new();
        for role in [
            Sys5I3ProviderChildRole::RequesterConsumer,
            Sys5I3ProviderChildRole::Executor,
        ] {
            let route = cohort.provider_child_route(role).ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
            let role_snapshot = cohort
                .admission
                .i3_private_provider_role_snapshot(role.m9_role())
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
                    )
                })?;
            let route_loci = cohort
                .images
                .get(route.slot_name())
                .and_then(Option::as_ref)
                .map(|image| &image.assigned_loci)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                    )
                })?;
            if !role_snapshot.matches_provider_child_installation(
                role.m9_role(),
                cohort.checked.program_identity().stable_key().as_str(),
                &setup.i3_private_resource_runtime_nonce(),
                route_loci,
            ) {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
            if child_routes.insert(role, route).is_some() {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
        }
        let requester_route = child_routes
            .get(&Sys5I3ProviderChildRole::RequesterConsumer)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let executor_route = child_routes
            .get(&Sys5I3ProviderChildRole::Executor)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let requester_expected = cohort
            .expected_start_bindings
            .get(requester_route.slot_name())
            .and_then(Option::as_ref)
            .cloned()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let executor_expected = cohort
            .expected_start_bindings
            .get(executor_route.slot_name())
            .and_then(Option::as_ref)
            .cloned()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let provider_peer_expected_start_bindings = BTreeMap::from([
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                executor_expected,
            ),
            (Sys5I3ProviderChildRole::Executor, requester_expected),
        ]);
        Ok(Self {
            cohort,
            setup,
            child_routes,
            provider_peer_expected_start_bindings,
            image_written_roles: BTreeSet::new(),
            control_attempted_roles: BTreeSet::new(),
        })
    }

    fn is_current(&self) -> bool {
        self.setup.matches_live_context(&self.cohort.setup_context)
            && self.cohort.admission.has_current_composite_seal()
    }

    /// Report only whether this opaque launch was created by one of the four
    /// fixed source-real observer conformance factories. The profile itself,
    /// its expected outcome, and all authority facts remain private to the
    /// trusted control path. The probe uses this before spawning either child
    /// so an ordinary launch cannot enter its harness-only validation runner.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn has_fixed_terminal_observation_conformance_profile(&self) -> bool {
        self.is_current()
            && self
                .setup
                .i3_private_terminal_observation_conformance_profile()
                .is_some()
    }

    /// Report only whether this opaque launch was created by one of the fixed
    /// source-real provider-network conformance factories. The selected
    /// schedule, its expected terminal state, and all authority facts remain
    /// private to trusted setup and FD3 control. The probe uses this solely to
    /// reject ordinary launches before either child is spawned.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn has_fixed_provider_network_conformance_profile(&self) -> bool {
        self.is_current()
            && self
                .setup
                .i3_private_provider_network_conformance_profile()
                .is_some()
    }

    #[cfg(test)]
    pub(crate) fn test_only_peer_start_binding_descriptor_for_role(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<PrivateProviderPeerStartBinding, Sys5I3ProviderLaunchError> {
        let peer = self
            .provider_peer_expected_start_bindings
            .get(&role)
            .ok_or_else(|| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
                )
            })?;
        PrivateProviderPeerStartBinding::try_from(peer).map_err(|_| {
            Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
            )
        })
    }

    #[cfg(test)]
    pub(crate) fn test_only_peer_start_binding_descriptor_matches_parent_held_binding(
        &self,
        role: Sys5I3ProviderChildRole,
        candidate: &PrivateProviderPeerStartBinding,
    ) -> Result<(), Sys5I3ProviderLaunchError> {
        let parent_held_binding = self
            .provider_peer_expected_start_bindings
            .get(&role)
            .ok_or_else(|| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
                )
            })?;
        if candidate.matches_parent_held_binding(parent_held_binding) {
            Ok(())
        } else {
            Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
            ))
        }
    }

    /// Return only the selected physical routing facts for a provider child.
    /// The caller cannot obtain the corresponding image/control through this
    /// view.
    pub fn child_route(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<Sys5I3ProviderChildRoute, Sys5I3ProviderLaunchError> {
        if !self.is_current() {
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        self.child_routes.get(&role).cloned().ok_or_else(|| {
            Sys5I3ProviderLaunchError::new(Sys5I3ProviderLaunchErrorKind::InvalidChildRole)
        })
    }

    /// Consume and write exactly one tagged provider image after probe child
    /// registration. A failed write spends the image and never enables a
    /// later control write or image replay.
    pub fn write_provider_child_image_once<W: std::io::Write>(
        &mut self,
        role: Sys5I3ProviderChildRole,
        writer: &mut W,
    ) -> Result<(), Sys5I3ProviderLaunchError> {
        let route = self.child_route(role)?;
        let image = self
            .cohort
            .take_process_image(route.slot_name())
            .map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderChildImageAlreadyTaken,
                )
            })?;
        let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_image(image)
            .map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderChildImageWriteRejected,
                )
            })?;
        let result = writer
            .write_all(&bytes)
            .and_then(|_| writer.flush())
            .map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderChildImageWriteRejected,
                )
            });
        if result.is_ok() {
            self.image_written_roles.insert(role);
        }
        result
    }

    /// The actual Stage 3 control handoff writes one complete private frame
    /// directly to FD3 and shuts down its write side. `transport_bootstrap`
    /// is bounded physical routing material only; runtime alone encodes the
    /// authority-bearing frame and the probe never accepts a decoded control
    /// as authority.
    #[cfg(unix)]
    pub fn write_provider_child_control_once(
        &mut self,
        role: Sys5I3ProviderChildRole,
        fd3: &mut UnixStream,
        transport_bootstrap: &[u8],
        deadline: Instant,
    ) -> Result<(), Sys5I3ProviderLaunchError> {
        if !self.control_attempted_roles.insert(role) {
            let _ = fd3.shutdown(Shutdown::Write);
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderChildControlAlreadyTaken,
            ));
        }

        let write_result = (|| {
            if transport_bootstrap.len() > Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES {
                return Err(Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderTransportBootstrapOversized,
                ));
            }
            let route = self.child_route(role)?;
            if !self.image_written_roles.contains(&role) {
                return Err(Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderChildImageNotWritten,
                ));
            }
            let snapshot = self.provider_child_control_snapshot_for_route(
                role,
                &route,
                transport_bootstrap.to_vec(),
            )?;
            let body = serialize_bounded_provider_child_control_snapshot(&snapshot)?;
            let body_len = u32::try_from(body.len()).map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlSnapshotOversized,
                )
            })?;
            write_provider_control_frame_until(fd3, &body_len.to_be_bytes(), &body, deadline)
                .map_err(|failure| {
                    Sys5I3ProviderLaunchError::new(match failure {
                        ProviderControlFrameWriteFailure::DeadlineElapsed => {
                            Sys5I3ProviderLaunchErrorKind::ProviderControlDeadlineElapsed
                        }
                        ProviderControlFrameWriteFailure::StreamRejected => {
                            Sys5I3ProviderLaunchErrorKind::ProviderControlWriteRejected
                        }
                    })
                })
        })();
        // Every attempted provider control closes this write end, including a
        // construction, size, progress, or deadline failure. The launch has
        // already spent `role`, so no alternate FD can reissue the control.
        let shutdown_result = fd3.shutdown(Shutdown::Write);
        match (write_result, shutdown_result) {
            (Err(error), _) => Err(error),
            (Ok(()), Err(_)) => Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderControlShutdownRejected,
            )),
            (Ok(()), Ok(())) => Ok(()),
        }
    }

    fn provider_child_control_snapshot_for_route(
        &mut self,
        role: Sys5I3ProviderChildRole,
        route: &Sys5I3ProviderChildRoute,
        transport_bootstrap: Vec<u8>,
    ) -> Result<PrivateProviderChildControlSnapshot, Sys5I3ProviderLaunchError> {
        if route.role() != role {
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
            ));
        }
        let expected_start_binding = self
            .cohort
            .parent_held_expected_start_binding(route.slot_name())
            .map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderChildControlAlreadyTaken,
                )
            })?;
        let peer_role = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => Sys5I3ProviderChildRole::Executor,
            Sys5I3ProviderChildRole::Executor => Sys5I3ProviderChildRole::RequesterConsumer,
        };
        let peer_expected_start_binding = self
            .provider_peer_expected_start_bindings
            .get(&role)
            .cloned()
            .filter(|binding| binding.slot_name != expected_start_binding.slot_name)
            .ok_or_else(|| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
                )
            })?;
        let m9_role_snapshot = self
            .cohort
            .admission
            .i3_private_provider_role_snapshot(role.m9_role())
            .map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::InactiveProviderBindingNotCurrent,
                )
            })?;
        let expected_resource_runtime_nonce = self.setup.i3_private_resource_runtime_nonce();
        let requester_fixture_assertion = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                Some(self.setup.i3_private_requester_fixture_assertion())
            }
            Sys5I3ProviderChildRole::Executor => None,
        };
        let executor_resource_envelope = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => None,
            Sys5I3ProviderChildRole::Executor => Some(
                self.setup
                    .i3_private_executor_resource_envelope()
                    .map_err(|_| {
                        Sys5I3ProviderLaunchError::new(
                            Sys5I3ProviderLaunchErrorKind::InactiveProviderBindingNotCurrent,
                        )
                    })?,
            ),
        };
        let terminal_observation_conformance_profile = self
            .setup
            .i3_private_terminal_observation_conformance_profile();
        let provider_network_conformance_profile =
            self.setup.i3_private_provider_network_conformance_profile();
        let expected_start_binding =
            PrivateInactiveProviderExpectedStartBindingSnapshot::try_from(&expected_start_binding)
                .map_err(|_| {
                    Sys5I3ProviderLaunchError::new(
                        Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
                    )
                })?;
        let peer_start_binding = PrivateProviderPeerStartBinding::try_from(
            &peer_expected_start_binding,
        )
        .map_err(|_| {
            Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
            )
        })?;
        Ok(PrivateProviderChildControlSnapshot {
            version: PRIVATE_PROVIDER_CHILD_CONTROL_VERSION,
            role: role.m9_role(),
            expected_start_binding,
            peer_role: peer_role.m9_role(),
            peer_start_binding,
            m9_role_snapshot,
            expected_resource_runtime_nonce,
            executor_resource_envelope,
            requester_fixture_assertion,
            terminal_observation_conformance_profile,
            provider_network_conformance_profile,
            transport_bootstrap,
        })
    }

    #[cfg(test)]
    pub(crate) fn test_only_provider_child_control_snapshot_size_metrics(
        &mut self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<PrivateProviderChildControlSnapshotSizeMetrics, Sys5I3ProviderLaunchError> {
        let route = self.child_route(role)?;
        let snapshot = self.provider_child_control_snapshot_for_route(role, &route, Vec::new())?;
        PrivateProviderChildControlSnapshotSizeMetrics::from_snapshot(&snapshot)
    }

    /// Returns only the bounded serializer allocation capacity for one
    /// source-real control snapshot. The snapshot is still built by the
    /// consuming parent-held binding path; no bytes or authority facts escape.
    #[cfg(all(test, unix))]
    pub(crate) fn test_only_provider_child_control_serialized_capacity_for_role(
        &mut self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<usize, Sys5I3ProviderLaunchError> {
        let route = self.child_route(role)?;
        let snapshot = self.provider_child_control_snapshot_for_route(role, &route, Vec::new())?;
        Ok(serialize_bounded_provider_child_control_snapshot(&snapshot)?.capacity())
    }
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderControlFrameWriteFailure {
    DeadlineElapsed,
    StreamRejected,
}

#[cfg(unix)]
fn write_provider_control_frame_until(
    stream: &mut UnixStream,
    prefix: &[u8],
    body: &[u8],
    deadline: Instant,
) -> Result<(), ProviderControlFrameWriteFailure> {
    const MAX_CONTROL_WRITE_WINDOW: Duration = Duration::from_secs(15);
    if deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero() && *remaining <= MAX_CONTROL_WRITE_WINDOW)
        .is_none()
    {
        return Err(ProviderControlFrameWriteFailure::DeadlineElapsed);
    }
    stream
        .set_nonblocking(true)
        .map_err(|_| ProviderControlFrameWriteFailure::StreamRejected)?;
    for bytes in [prefix, body] {
        let mut written = 0;
        while written < bytes.len() {
            if Instant::now() >= deadline {
                return Err(ProviderControlFrameWriteFailure::DeadlineElapsed);
            }
            match stream.write(&bytes[written..]) {
                Ok(0) => return Err(ProviderControlFrameWriteFailure::StreamRejected),
                Ok(count) => {
                    written = written
                        .checked_add(count)
                        .ok_or(ProviderControlFrameWriteFailure::StreamRejected)?
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    wait_for_provider_control_writable(stream, deadline)?;
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return Err(ProviderControlFrameWriteFailure::StreamRejected),
            }
        }
    }
    if Instant::now() >= deadline {
        return Err(ProviderControlFrameWriteFailure::DeadlineElapsed);
    }
    stream
        .flush()
        .map_err(|_| ProviderControlFrameWriteFailure::StreamRejected)?;
    if Instant::now() >= deadline {
        return Err(ProviderControlFrameWriteFailure::DeadlineElapsed);
    }
    Ok(())
}

#[cfg(unix)]
fn wait_for_provider_control_writable(
    stream: &UnixStream,
    deadline: Instant,
) -> Result<(), ProviderControlFrameWriteFailure> {
    #[repr(C)]
    struct PollFd {
        fd: std::os::raw::c_int,
        events: std::os::raw::c_short,
        revents: std::os::raw::c_short,
    }

    unsafe extern "C" {
        fn poll(
            descriptors: *mut PollFd,
            descriptor_count: std::os::raw::c_ulong,
            timeout_millis: std::os::raw::c_int,
        ) -> std::os::raw::c_int;
    }

    const POLLOUT: std::os::raw::c_short = 0x0004;
    loop {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .filter(|remaining| !remaining.is_zero())
            .ok_or(ProviderControlFrameWriteFailure::DeadlineElapsed)?;
        let millis = remaining
            .as_millis()
            .saturating_add(u128::from(remaining.subsec_nanos() % 1_000_000 != 0))
            .min(std::os::raw::c_int::MAX as u128) as std::os::raw::c_int;
        let mut descriptor = PollFd {
            fd: stream.as_raw_fd(),
            events: POLLOUT,
            revents: 0,
        };
        // SAFETY: `descriptor` is valid for one poll entry, and `stream`
        // retains the borrowed descriptor for this bounded wait.
        let outcome = unsafe { poll(&mut descriptor, 1, millis.max(1)) };
        match outcome {
            1 if descriptor.revents & POLLOUT != 0 => return Ok(()),
            0 if Instant::now() >= deadline => {
                return Err(ProviderControlFrameWriteFailure::DeadlineElapsed);
            }
            0 => continue,
            value
                if value < 0
                    && std::io::Error::last_os_error().kind()
                        == std::io::ErrorKind::Interrupted =>
            {
                continue;
            }
            _ => return Err(ProviderControlFrameWriteFailure::StreamRejected),
        }
    }
}

#[cfg(test)]
struct Sys5I3InactiveProviderValidationPause {
    entered: SyncSender<()>,
    resume: Mutex<Receiver<()>>,
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the inactive cohort methods are intentionally pre-start only until activation is added"
    )
)]
impl Sys5I3InactiveProviderCohort {
    pub(crate) fn from_parent_inactive_provider_admission<I>(
        checked: mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
        admission: Sys4InactiveProviderAdmission,
        setup_context: Arc<TrustedFixtureContext>,
        slots: I,
    ) -> Result<Self, Sys5I3ProcessRuntimeError>
    where
        I: IntoIterator<Item = Sys5I3DeploymentSlot>,
    {
        if !setup_context.is_current()
            || admission.checked_program_identity() != checked.program_identity()
            || !admission.has_current_composite_seal()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        let slots = slots.into_iter().collect::<Vec<_>>();
        let expected_loci = admission
            .static_plan()
            .projection()
            .locus_order()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<BTreeSet<_>>();
        validate_i3_deployment_slots(&expected_loci, &slots)?;
        let mut images = BTreeMap::new();
        let mut expected_start_bindings = BTreeMap::new();
        for slot in &slots {
            if slot.slot_name.is_empty() || slot.endpoint.is_empty() || slot.loci.is_empty() {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::EmptyDeploymentSlot,
                ));
            }
            let slot_loci = slot.loci.iter().cloned().collect::<BTreeSet<_>>();
            let restricted = admission.restricted_to_loci(&slot_loci).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
            let image = Sys5I3ProcessImage::from_inactive_provider_restriction(
                slot,
                &checked.program_identity().stable_key(),
                &restricted,
            )?;
            let expected = Sys5I3ExpectedStartBinding::for_inactive_provider_image(&image)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                    )
                })?;
            if images.insert(slot.slot_name.clone(), Some(image)).is_some()
                || expected_start_bindings
                    .insert(slot.slot_name.clone(), Some(expected))
                    .is_some()
            {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
        }
        Ok(Self {
            images,
            expected_start_bindings,
            checked,
            admission,
            setup_context,
            #[cfg(test)]
            inactive_validation_pause: None,
        })
    }

    fn provider_child_route(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> Option<Sys5I3ProviderChildRoute> {
        let expected_loci = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                BTreeSet::from(["ParticipantA".to_string(), "ViewerC".to_string()])
            }
            Sys5I3ProviderChildRole::Executor => {
                BTreeSet::from(["ParticipantB".to_string(), "WorldAuthority".to_string()])
            }
        };
        self.images.values().flatten().find_map(|image| {
            (image.assigned_loci == expected_loci).then(|| Sys5I3ProviderChildRoute {
                role,
                slot_name: image.slot_name.clone(),
                endpoint: image.endpoint.clone(),
            })
        })
    }

    pub(crate) fn take_process_image(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        self.images
            .get_mut(slot_name)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
            })?
            .take()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken,
                )
            })
    }

    pub(crate) fn parent_held_expected_start_binding(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ExpectedStartBinding, Sys5I3ProcessRuntimeError> {
        self.expected_start_bindings
            .get_mut(slot_name)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
            })?
            .take()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })
    }

    pub(crate) fn validate_inactive_untrusted_image(
        &self,
        image: Sys5I3UntrustedProcessImage,
        expected: Sys5I3ExpectedStartBinding,
    ) -> Result<Sys5I3InactiveProviderImageValidation, Sys5I3ProcessRuntimeError> {
        if !self.setup_context.is_current() || !self.admission.has_current_composite_seal() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        let expected_provider = expected.inactive_provider.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let candidate = image.image;
        let provider = candidate.inactive_provider.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let provider_seed = candidate
            .private_runtime_seed
            .inactive_provider()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                )
            })?;
        let restricted = self
            .admission
            .restricted_to_loci(&candidate.assigned_loci)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                )
            })?;
        if candidate.private_integrity_ref != candidate.recomputed_private_integrity()
            || candidate.private_integrity_ref != expected.image_integrity_ref
            || candidate.slot_name != expected.slot_name
            || candidate.assigned_loci != expected.assigned_loci
            || provider.assigned_loci != candidate.assigned_loci
            || provider.parent_checked_program_ref != expected.parent_checked_program_ref
            || provider.parent_checked_program_ref != provider_seed.parent_checked_program_ref
            || provider.static_snapshot != provider_seed.static_snapshot
            || provider.component_snapshot != provider_seed.component_snapshot
            || provider.legacy_authority_snapshot != provider_seed.legacy_authority_snapshot
            || provider.component_binding_ref != provider_seed.component_binding_ref
            || provider.static_snapshot != expected_provider.static_snapshot
            || provider.component_snapshot != expected_provider.component_snapshot
            || provider.legacy_authority_snapshot != expected_provider.legacy_authority_snapshot
            || provider.component_binding_ref != expected_provider.component_binding_ref
            || !restricted.matches_parent(&self.admission)
            || provider.static_snapshot != *restricted.static_snapshot()
            || !provider
                .component_snapshot
                .matches_parent_component(restricted.component())
            || provider.legacy_authority_snapshot
                != restricted
                    .legacy_authority_generation()
                    .i3_private_snapshot()
            || provider.component_binding_ref != restricted.component_binding_ref()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        #[cfg(test)]
        if let Some(pause) = self.inactive_validation_pause.as_ref() {
            pause.entered.send(()).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::TestOnlyInactiveProviderValidationSynchronizationTimedOut,
                )
            })?;
            let resume = pause.resume.lock().map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::TestOnlyInactiveProviderValidationSynchronizationTimedOut,
                )
            })?;
            resume
                .recv_timeout(std::time::Duration::from_secs(5))
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::TestOnlyInactiveProviderValidationSynchronizationTimedOut,
                    )
                })?;
        }
        if !self.admission.has_current_composite_seal() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        if !self.setup_context.is_current() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        let candidate_assigned_loci = candidate.assigned_loci.clone();
        Ok(Sys5I3InactiveProviderImageValidation {
            assigned_loci: candidate_assigned_loci.clone(),
            checked_program_identity: self.checked.program_identity().clone(),
            exact_provider_static_coverage: provider
                .static_snapshot
                .matches_static_projection_restricted_to_loci(
                    self.admission.static_plan(),
                    &provider.assigned_loci,
                ),
            provider_lowering_count: restricted.declared_provider_lowering_count(),
            retained_non_provider_lowering_inventory: restricted.component().inventory(),
            binding_was_current_at_validation: true,
            composite_seal_association_at_validation: true,
            exact_translated_legacy_m9_authority_inventory: restricted
                .matches_m9_execution_restriction_for_assigned_loci(&self.admission),
            matches_m9_execution_restriction: restricted
                .matches_m9_execution_restriction_for_assigned_loci(&self.admission),
            assigned_provider_static_descriptors_only: provider.assigned_loci
                == candidate_assigned_loci,
        })
    }

    #[cfg(test)]
    pub(crate) fn test_only_pause_after_inactive_structural_validation(
        &mut self,
        entered: SyncSender<()>,
        resume: Receiver<()>,
    ) {
        self.inactive_validation_pause = Some(Sys5I3InactiveProviderValidationPause {
            entered,
            resume: Mutex::new(resume),
        });
    }

    #[cfg(test)]
    pub(crate) fn test_only_retire_effect_authorization(
        &mut self,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.admission.retire_effect_authorization().map_err(|_| {
            Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            )
        })
    }

    /// Build two focused local provider-execution components from the same
    /// source-real inactive cohort. This is deliberately narrower than an
    /// inherited-control installation: it neither consumes an image nor
    /// creates FD3 control, a child runtime, or transport evidence. Keeping
    /// `setup` borrowed makes the actual fixture context remain live for the
    /// entire local test pair instead of treating an `Arc` retirement flag as
    /// a substitute for the native setup guard.
    #[cfg(test)]
    pub(crate) fn test_only_source_real_provider_local_execution_pair<'setup>(
        &self,
        setup: &'setup I3TrustedReadOnlyProviderFixtureSetup,
    ) -> Result<Sys5I3SourceRealProviderLocalExecutionPair<'setup>, Sys5I3ProcessRuntimeError> {
        if !setup.matches_live_context(&self.setup_context)
            || !self.admission.has_current_composite_seal()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
            ));
        }
        Ok(Sys5I3SourceRealProviderLocalExecutionPair {
            _setup_guard: setup,
            requester: self.test_only_source_real_provider_local_execution_for_role(
                setup,
                Sys5I3ProviderChildRole::RequesterConsumer,
            )?,
            executor: self.test_only_source_real_provider_local_execution_for_role(
                setup,
                Sys5I3ProviderChildRole::Executor,
            )?,
        })
    }

    #[cfg(test)]
    fn test_only_source_real_provider_local_execution_for_role(
        &self,
        setup: &I3TrustedReadOnlyProviderFixtureSetup,
        role: Sys5I3ProviderChildRole,
    ) -> Result<Sys5I3ProviderLocalExecution, Sys5I3ProcessRuntimeError> {
        let route = self.provider_child_route(role).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        let assigned_loci = self
            .images
            .get(route.slot_name())
            .and_then(Option::as_ref)
            .map(|image| image.assigned_loci.clone())
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let restriction = self
            .admission
            .restricted_to_loci(&assigned_loci)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        if restriction.assigned_loci() != &assigned_loci
            || !restriction.matches_parent(&self.admission)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }

        let resource_runtime_nonce = setup.i3_private_resource_runtime_nonce();
        let role_snapshot = self
            .admission
            .i3_private_provider_role_snapshot(role.m9_role())
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        if !role_snapshot.matches_provider_child_installation(
            role.m9_role(),
            self.checked.program_identity().stable_key().as_str(),
            &resource_runtime_nonce,
            &assigned_loci,
        ) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let local_authority = role_snapshot
            .install_local_authority(
                role.m9_role(),
                self.checked.program_identity().stable_key().as_str(),
                &resource_runtime_nonce,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let scoped_component = restriction
            .component()
            .i3_private_provider_component_snapshot()
            .restore_for_inherited_provider_install(&resource_runtime_nonce)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        if scoped_component.component_binding_ref() != restriction.component_binding_ref() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let provider_local_fabric =
            Sys4InstalledProviderLocalFabric::from_inherited_provider_parts(
                restriction.static_snapshot().clone(),
                &scoped_component,
                restriction
                    .legacy_authority_generation()
                    .i3_private_snapshot(),
                assigned_loci,
                restriction.component_binding_ref(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        if provider_local_fabric.checked_program_identity() != self.checked.program_identity()
            || provider_local_fabric.component_binding_ref() != restriction.component_binding_ref()
            || !provider_local_fabric.has_exact_legacy_authority_generation()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let executor_resource_envelope = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => None,
            Sys5I3ProviderChildRole::Executor => {
                Some(setup.i3_private_executor_resource_envelope().map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
                    )
                })?)
            }
        };
        let provider_execution = Sys5I3ProviderExecution::from_installed_profile(
            &local_authority,
            &provider_local_fabric,
        )?;
        Ok(Sys5I3ProviderLocalExecution {
            role,
            local_authority,
            _scoped_component: scoped_component,
            provider_local_fabric,
            executor_resource_envelope,
            provider_execution,
        })
    }
}

/// T0-internal inspection surface for a successfully validated inactive
/// provider image. It carries no executable seed or M9 authority material.
#[doc(hidden)]
pub(crate) struct Sys5I3InactiveProviderImageValidation {
    assigned_loci: BTreeSet<String>,
    checked_program_identity: mir_semantics::surface_v0_pipeline::CheckedProgramIdentity,
    exact_provider_static_coverage: bool,
    provider_lowering_count: usize,
    retained_non_provider_lowering_inventory: M8ReadOnlyProviderEffectComponentInventory,
    binding_was_current_at_validation: bool,
    composite_seal_association_at_validation: bool,
    exact_translated_legacy_m9_authority_inventory: bool,
    matches_m9_execution_restriction: bool,
    assigned_provider_static_descriptors_only: bool,
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the inactive validation receipt exposes no active runtime behavior before its next consumer"
    )
)]
impl Sys5I3InactiveProviderImageValidation {
    pub(crate) fn activation_pending(&self) -> bool {
        true
    }

    pub(crate) fn provider_runtime_active(&self) -> bool {
        false
    }

    pub(crate) fn provider_call_count(&self) -> usize {
        0
    }

    /// The parent binding and effect authorization passed their checks at the
    /// validation boundary. This receipt does not assert continuing authority
    /// after validation returns.
    pub(crate) fn binding_was_current_at_validation(&self) -> bool {
        self.binding_was_current_at_validation
    }

    pub(crate) fn matches_checked_program_identity(
        &self,
        checked: &mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
    ) -> bool {
        self.checked_program_identity == *checked.program_identity()
            && self
                .retained_non_provider_lowering_inventory
                .matches_checked_program_identity(checked)
            && self.exact_provider_static_coverage
    }

    pub(crate) fn has_exact_provider_static_coverage(&self) -> bool {
        self.exact_provider_static_coverage
    }

    pub(crate) fn declared_provider_lowering_count(&self) -> usize {
        self.provider_lowering_count
    }

    pub(crate) fn retains_exact_non_provider_ordered_lowering_associations(
        &self,
        checked: &mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
    ) -> bool {
        self.checked_program_identity == *checked.program_identity()
            && self
                .retained_non_provider_lowering_inventory
                .retains_exact_non_provider_ordered_lowering_associations(checked)
    }

    /// The decoded candidate matched the parent-held composite seal when this
    /// receipt was produced. It is not a live effect-authorization query.
    pub(crate) fn has_retained_composite_seal_association_at_validation(&self) -> bool {
        self.composite_seal_association_at_validation
    }

    pub(crate) fn has_exact_translated_legacy_m9_authority_inventory(&self) -> bool {
        self.exact_translated_legacy_m9_authority_inventory
    }

    pub(crate) fn matches_m9_execution_restriction_for_assigned_loci(&self) -> bool {
        self.matches_m9_execution_restriction
    }

    pub(crate) fn retains_assigned_provider_static_descriptors_only(&self) -> bool {
        self.assigned_provider_static_descriptors_only
    }

    pub(crate) fn assigned_loci(&self) -> Vec<String> {
        self.assigned_loci.iter().cloned().collect()
    }
}

impl Sys5I3ProcessCohort {
    pub fn from_checked_project(
        project: &Sys5LocalProject,
        deployment: &Sys5I3Deployment,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if deployment.parent_checked_program_ref != project.checked_program_identity_ref() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if deployment.parent_projection_ref != project.i3_parent_projection_ref() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        let activation_occurrence_ref = fresh_occurrence_ref(
            "activation",
            &deployment.parent_checked_program_ref,
            &deployment.parent_projection_ref,
        );
        let cohort_occurrence_ref = fresh_occurrence_ref(
            "cohort",
            &deployment.parent_checked_program_ref,
            &deployment.parent_projection_ref,
        );

        // This is the sole full prepare/admission call for the cohort.  The
        // values are borrowed only while images are sealed below and are not
        // moved into `Self` or any child image.
        let prepared = project
            .prepare_canonical_local_st_admission()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let (coordinator_program, mut coordinator_admission) = prepared.into_parts_for_sys4();
        let mut images = BTreeMap::new();
        let mut expected_start_bindings = BTreeMap::new();
        for slot in &deployment.slots {
            let image = Sys5I3ProcessImage::from_coordinator_parts(
                project,
                deployment,
                &slot.slot_name,
                &coordinator_program,
                &coordinator_admission,
                &cohort_occurrence_ref,
            )?;
            let expected_start_binding = Sys5I3ExpectedStartBinding::for_image(&image);
            if images.insert(slot.slot_name.clone(), Some(image)).is_some() {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
            if expected_start_bindings
                .insert(slot.slot_name.clone(), Some(expected_start_binding))
                .is_some()
            {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
        }

        // Images must be complete before the parent retains its one genuine
        // M9 publisher.  The retained coordinator cannot regenerate a full
        // program/admission and is never handed to a child.
        let published_authority_generation_ref =
            coordinator_admission.m9_generation_ref().to_string();
        let authority_successor_coordinator = coordinator_admission
            .take_i3_owner_capability_successor_coordinator()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;

        Ok(Self {
            images,
            expected_start_bindings,
            summary: Sys5I3ProcessCohortSummary {
                full_admission_count: 1,
                authority_generation_count: 1,
                parent_checked_program_ref: deployment.parent_checked_program_ref.clone(),
                projection_ref: deployment.parent_projection_ref.clone(),
                activation_occurrence_ref,
                cohort_occurrence_ref,
            },
            authority_successor_coordinator,
            published_authority_generation_ref,
            lifecycle_publication_outcome: Some(
                Sys5I3LifecyclePublicationOutcome::NoPrestageSelected,
            ),
            prestage_run_ref: None,
            #[cfg(all(unix, feature = "i3-process-test-seams"))]
            pending_owner_lifecycle_ack_registration: None,
            #[cfg(all(unix, feature = "i3-process-test-seams"))]
            pending_owner_lifecycle_stage_identity_binding_ref: None,
        })
    }

    pub fn observer_safe_summary(&self) -> Sys5I3ProcessCohortSummary {
        self.summary.clone()
    }

    /// Observer-safe parent view of the one bounded owner-capability
    /// lifecycle.  A `None` terminal outcome means a pre-staged successor is
    /// still pending B installation/completion; it never exposes candidate
    /// or authority material.
    pub fn observer_safe_lifecycle_publication_summary(
        &self,
    ) -> Sys5I3ObserverSafeLifecyclePublicationSummary {
        Sys5I3ObserverSafeLifecyclePublicationSummary {
            published_authority_generation_ref: self.published_authority_generation_ref.clone(),
            publication_outcome: self.lifecycle_publication_outcome,
        }
    }

    /// Select the one exact source-checked owner-request contract that may
    /// retire its existing owner capability.  This remains a host lifecycle
    /// operation: it is complete before images or bindings leave the parent,
    /// and it cannot accept an operation, capability, witness, or generation
    /// selected by a child or deployment.
    pub fn prestage_owner_capability_revocation(
        &mut self,
        run_ref: &str,
        contract: &Sys5I3AdapterCarrierContract,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.prestage_owner_capability_revocation_inner(run_ref, contract, None)
    }

    /// Feature-gated negative-only successor predicate exerciser.  The
    /// tamper is applied only to an M9-produced candidate; it never accepts
    /// raw caller authority material or exposes a child issuer path.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn test_only_prestage_owner_capability_revocation_with_tamper(
        &mut self,
        run_ref: &str,
        contract: &Sys5I3AdapterCarrierContract,
        tamper: Sys5I3OwnerCapabilitySuccessorTamper,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.prestage_owner_capability_revocation_inner(run_ref, contract, Some(tamper))
    }

    fn prestage_owner_capability_revocation_inner(
        &mut self,
        run_ref: &str,
        contract: &Sys5I3AdapterCarrierContract,
        #[cfg(feature = "i3-process-test-seams")] tamper: Option<
            Sys5I3OwnerCapabilitySuccessorTamper,
        >,
        #[cfg(not(feature = "i3-process-test-seams"))] _tamper: Option<()>,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if run_ref.is_empty()
            || contract.edge_ref().is_empty()
            || contract.operation_id().is_empty()
            || contract.source_locus().is_empty()
            || contract.target_locus().is_empty()
            || contract.edge_kind() != "owner-request"
            || contract.checked_program_ref() != self.summary.parent_checked_program_ref
            || contract.full_retained_contract_fingerprint().is_empty()
            || !contract.checked_core_bound()
            || contract.transfers_authority()
            || contract.mints_authority_without_source()
            || self.lifecycle_publication_outcome.is_none()
            || self.prestage_run_ref.is_some()
            || !self.expected_start_bindings.values().all(Option::is_some)
            || self.images.values().any(Option::is_none)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
            ));
        }

        let matching_slots = self
            .images
            .iter()
            .filter_map(|(slot_name, image)| {
                let image = image.as_ref()?;
                (image.assigned_loci.contains(contract.target_locus())
                    && image.required_edge_contracts.iter().any(|candidate| {
                        candidate.edge_ref == contract.edge_ref()
                            && candidate.operation_id == contract.operation_id()
                            && candidate.kind == contract.edge_kind()
                            && candidate.source_locus == contract.source_locus()
                            && candidate.target_locus == contract.target_locus()
                            && candidate.core_ref == contract.core_ref()
                            && candidate.source_artifact_ref == contract.source_artifact_ref()
                            && candidate.target_artifact_ref == contract.target_artifact_ref()
                            && candidate.parent_checked_program_ref
                                == contract.checked_program_ref()
                    }))
                .then(|| slot_name.clone())
            })
            .collect::<Vec<_>>();
        let [target_slot_name] = matching_slots.as_slice() else {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
            ));
        };
        let target_slot_name = target_slot_name.clone();
        let target_image = self
            .images
            .get(&target_slot_name)
            .and_then(Option::as_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
                )
            })?;
        let target_seed = target_image
            .private_runtime_seed
            .ordinary()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
                )
            })?;
        #[cfg(feature = "i3-process-test-seams")]
        let staged = match tamper {
            Some(tamper) => self
                .authority_successor_coordinator
                .test_only_prestage_owner_capability_revocation_with_tamper(
                    &target_seed.program,
                    &target_seed.admission,
                    contract.operation_id(),
                    contract.target_locus(),
                    match tamper {
                        Sys5I3OwnerCapabilitySuccessorTamper::AddUnrelatedOwnerLineage => {
                            Sys4I3OwnerCapabilitySuccessorTamper::AddUnrelatedOwnerLineage
                        }
                        Sys5I3OwnerCapabilitySuccessorTamper::ReanimateSelectedCapability => {
                            Sys4I3OwnerCapabilitySuccessorTamper::ReanimateSelectedCapability
                        }
                        Sys5I3OwnerCapabilitySuccessorTamper::ReanimateSelectedWitness => {
                            Sys4I3OwnerCapabilitySuccessorTamper::ReanimateSelectedWitness
                        }
                    },
                ),
            None => self
                .authority_successor_coordinator
                .prestage_owner_capability_revocation(
                    &target_seed.program,
                    &target_seed.admission,
                    contract.operation_id(),
                    contract.target_locus(),
                ),
        };
        #[cfg(not(feature = "i3-process-test-seams"))]
        let staged = self
            .authority_successor_coordinator
            .prestage_owner_capability_revocation(
                &target_seed.program,
                &target_seed.admission,
                contract.operation_id(),
                contract.target_locus(),
            );
        let candidate = staged.map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected)
        })?;
        let stage_identity_binding_ref = owner_capability_lifecycle_stage_identity_binding_ref(
            run_ref,
            target_image,
            &candidate,
        );
        #[cfg(all(unix, feature = "i3-process-test-seams"))]
        let ack_registration = Sys5I3RegisteredOwnerLifecycleAckRegistration {
            target_slot_name: target_slot_name.clone(),
            stage_identity_binding_ref: stage_identity_binding_ref.clone(),
            prior_generation_ref: candidate.prior_generation_ref().to_string(),
            successor_generation_ref: candidate.successor_generation_ref().to_string(),
            candidate_binding_ref: candidate.candidate_binding_ref().to_string(),
        };

        let image = self
            .images
            .get_mut(&target_slot_name)
            .and_then(Option::as_mut)
            .expect("target image remained parent-held during prestage");
        let seed = image.private_runtime_seed.ordinary_mut().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected)
        })?;
        seed.prestaged_owner_capability_lifecycle = Some(Sys5I3PrestagedOwnerCapabilityLifecycle {
            target_slot_name: target_slot_name.clone(),
            stage_identity_binding_ref: stage_identity_binding_ref.clone(),
            candidate,
        });
        image.refresh_private_integrity();
        let expected_start_binding = Sys5I3ExpectedStartBinding::for_image(image);
        let expected = self
            .expected_start_bindings
            .get_mut(&target_slot_name)
            .expect("image and start-binding slot inventories are constructed together");
        *expected = Some(expected_start_binding);
        self.lifecycle_publication_outcome = None;
        self.prestage_run_ref = Some(run_ref.to_string());
        #[cfg(all(unix, feature = "i3-process-test-seams"))]
        {
            self.pending_owner_lifecycle_ack_registration = Some(ack_registration);
            self.pending_owner_lifecycle_stage_identity_binding_ref =
                Some(stage_identity_binding_ref);
        }
        Ok(())
    }

    /// Consume two parent-held bootstrap bindings only after exact pre-stage.
    /// The control pair contains the lifecycle expectation independently of
    /// the tainted image; it carries no publisher and grants no child issuer.
    pub fn split_trusted_localnet_controls_with_prestaged_lifecycle(
        &mut self,
        codec: &Sys5I3PrivateProcessCodec,
        run_ref: &str,
        first_slot_name: &str,
        first_spki_ref: &str,
        second_slot_name: &str,
        second_spki_ref: &str,
    ) -> Result<
        (Sys5I3TrustedLocalnetControl, Sys5I3TrustedLocalnetControl),
        Sys5I3ProcessRuntimeError,
    > {
        if self.lifecycle_publication_outcome.is_some()
            || self.prestage_run_ref.as_deref() != Some(run_ref)
            || first_slot_name.is_empty()
            || second_slot_name.is_empty()
            || first_slot_name == second_slot_name
            || first_spki_ref.is_empty()
            || second_spki_ref.is_empty()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
            ));
        }
        let first = self.parent_held_expected_start_binding(first_slot_name)?;
        let second = self.parent_held_expected_start_binding(second_slot_name)?;
        codec
            .split_trusted_localnet_controls(
                run_ref,
                first,
                first_spki_ref,
                second,
                second_spki_ref,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
                )
            })
    }

    /// Negative-only codec fixture: it serializes candidate-shaped fields
    /// without an installed receipt or registered child-FD provenance.  A
    /// successful decode remains tainted and cannot reach publication.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn test_only_encode_prestaged_owner_lifecycle_ack_candidate(
        &self,
        codec: &Sys5I3PrivateProcessCodec,
    ) -> Result<Vec<u8>, Sys5I3ProcessRuntimeError> {
        let lifecycle = self
            .images
            .values()
            .filter_map(Option::as_ref)
            .filter_map(|image| {
                image
                    .private_runtime_seed
                    .ordinary()
                    .and_then(|seed| seed.prestaged_owner_capability_lifecycle.as_ref())
            })
            .next()
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
                )
            })?;
        codec
            .encode_tainted_owner_lifecycle_ack_candidate(
                &lifecycle.stage_identity_binding_ref,
                lifecycle.candidate.prior_generation_ref(),
                lifecycle.candidate.successor_generation_ref(),
                lifecycle.candidate.candidate_binding_ref(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
                )
            })
    }

    /// Register the parent half of the one inherited B→parent ACK descriptor.
    /// This is intentionally available only to the actual-process test
    /// adapter on Unix.  It consumes the cohort-held registration exactly
    /// once, configures a finite OS read deadline, and never accepts bytes or
    /// a decoded ACK from its caller.
    #[cfg(all(unix, feature = "i3-process-test-seams"))]
    #[doc(hidden)]
    pub fn take_registered_owner_lifecycle_ack_reader(
        &mut self,
        owner_slot_name: &str,
        parent_stream: UnixStream,
        read_timeout: Duration,
    ) -> Result<Sys5I3RegisteredOwnerLifecycleAckReader, Sys5I3ProcessRuntimeError> {
        if read_timeout.is_zero()
            || self.lifecycle_publication_outcome.is_some()
            || self
                .pending_owner_lifecycle_ack_registration
                .as_ref()
                .is_none_or(|registration| {
                    registration.target_slot_name != owner_slot_name
                        || self
                            .pending_owner_lifecycle_stage_identity_binding_ref
                            .as_deref()
                            != Some(registration.stage_identity_binding_ref.as_str())
                })
            || self
                .pending_owner_lifecycle_stage_identity_binding_ref
                .as_deref()
                .is_none()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        let registration = self
            .pending_owner_lifecycle_ack_registration
            .take()
            .expect("checked registration exists before actual B reader construction");
        Sys5I3RegisteredOwnerLifecycleAckReader::from_registered_stream(
            parent_stream,
            registration,
            read_timeout,
            false,
        )
    }

    /// Publish the retained genuine M9 successor only from a completion that
    /// an owned registered-B stream reader produced.  There is deliberately
    /// no overload accepting ACK bytes, a decoded DTO, a slot label, or
    /// candidate fields from an observer/untrusted route.
    #[cfg(all(unix, feature = "i3-process-test-seams"))]
    #[doc(hidden)]
    pub fn publish_registered_owner_lifecycle_completion(
        &mut self,
        completion: Sys5I3RegisteredOwnerLifecycleCompletion,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.lifecycle_publication_outcome.is_some() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        if self
            .pending_owner_lifecycle_stage_identity_binding_ref
            .as_deref()
            != Some(completion.stage_identity_binding_ref.as_str())
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        let published_generation_ref = self
            .authority_successor_coordinator
            .publish_prestaged_owner_capability_revocation(
                &completion.prior_generation_ref,
                &completion.successor_generation_ref,
                &completion.candidate_binding_ref,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
            })?;
        self.published_authority_generation_ref = published_generation_ref;
        self.lifecycle_publication_outcome = Some(Sys5I3LifecyclePublicationOutcome::G2Published);
        self.prestage_run_ref = None;
        self.pending_owner_lifecycle_stage_identity_binding_ref = None;
        Ok(())
    }

    /// Record the terminal finite outcome when the registered B reader cannot
    /// yield its one completion (including a lost child ACK or rejected B
    /// bootstrap).  This publishes no successor, performs no rollback, and
    /// leaves the parent generation at G1; the retained staged publisher is
    /// dropped only with the bounded cohort lifetime.
    #[cfg(all(unix, feature = "i3-process-test-seams"))]
    #[doc(hidden)]
    pub fn mark_registered_owner_lifecycle_publication_incomplete(
        &mut self,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.lifecycle_publication_outcome.is_some() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecyclePublicationIncomplete,
            ));
        }
        self.lifecycle_publication_outcome =
            Some(Sys5I3LifecyclePublicationOutcome::PublicationIncomplete);
        self.prestage_run_ref = None;
        self.pending_owner_lifecycle_ack_registration = None;
        self.pending_owner_lifecycle_stage_identity_binding_ref = None;
        Ok(())
    }

    /// Retain the supervisor-controlled expectation before the child image
    /// is released.  It is deliberately derived from the already sealed
    /// image and contains only binding/provenance facts, never an authority
    /// seed or replacement admission path.
    pub fn parent_held_expected_start_binding(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ExpectedStartBinding, Sys5I3ProcessRuntimeError> {
        let binding = self
            .expected_start_bindings
            .get_mut(slot_name)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
            })?;
        binding.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken)
        })
    }

    /// Consume the one sealed image for a deployment slot.  Neither a cohort
    /// nor an image implements a duplication path, so a caller cannot start
    /// two owner runtimes from one derived local authority seed.
    pub fn take_process_image(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        let image = self.images.get_mut(slot_name).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
        })?;
        image.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken)
        })
    }
}

#[cfg(all(unix, feature = "i3-process-test-seams"))]
impl Sys5I3RegisteredOwnerLifecycleAckReader {
    fn from_registered_stream(
        stream: UnixStream,
        registration: Sys5I3RegisteredOwnerLifecycleAckRegistration,
        read_timeout: Duration,
        deadline_only: bool,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        let deadline = Instant::now().checked_add(read_timeout).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
        })?;
        Ok(Self {
            stream,
            registration,
            deadline,
            completion_consumed: false,
            deadline_only,
        })
    }

    /// Component-only real-UnixStream factory for the absolute-deadline
    /// regression. It cannot yield an owner-lifecycle completion, publish a
    /// candidate, or stand in for a registered B child route.
    #[doc(hidden)]
    pub fn test_only_for_absolute_deadline(
        stream: UnixStream,
        read_timeout: Duration,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if read_timeout.is_zero() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        Self::from_registered_stream(
            stream,
            Sys5I3RegisteredOwnerLifecycleAckRegistration {
                target_slot_name: "i3-test-deadline-only".to_string(),
                stage_identity_binding_ref: "i3-test-deadline-only".to_string(),
                prior_generation_ref: "i3-test-deadline-only".to_string(),
                successor_generation_ref: "i3-test-deadline-only".to_string(),
                candidate_binding_ref: "i3-test-deadline-only".to_string(),
            },
            read_timeout,
            true,
        )
    }

    fn deadline_expired(&self) -> bool {
        Instant::now() >= self.deadline
    }

    fn read_exact_before_deadline(
        &mut self,
        destination: &mut [u8],
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let mut read = 0;
        while read < destination.len() {
            let remaining = self
                .deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
                    )
                })?;
            self.stream.set_read_timeout(Some(remaining)).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
            })?;
            let count = self.stream.read(&mut destination[read..]).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
            })?;
            if count == 0 {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
                ));
            }
            read += count;
        }
        Ok(())
    }

    /// Read exactly one bounded framed ACK from the registered B child stream.
    /// This method owns both the stream and registration; it has no parameter
    /// for bytes or a tainted decoded candidate.  A second read attempt is a
    /// typed replay rejection even if a child wrote duplicate frame bytes.
    pub fn read_next_completion(
        &mut self,
        codec: &Sys5I3PrivateProcessCodec,
    ) -> Result<Sys5I3RegisteredOwnerLifecycleCompletion, Sys5I3ProcessRuntimeError> {
        if self.completion_consumed {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        if self.deadline_expired() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        let mut prefix = [0_u8; 4];
        self.read_exact_before_deadline(&mut prefix)?;
        let length = u32::from_be_bytes(prefix) as usize;
        if length > Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        let frame_length = length.checked_add(prefix.len()).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
        })?;
        let mut frame = Vec::with_capacity(frame_length);
        frame.extend_from_slice(&prefix);
        frame.resize(frame_length, 0);
        self.read_exact_before_deadline(&mut frame[prefix.len()..])?;
        if self.deadline_only {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        let tainted = codec.decode_owner_lifecycle_ack(&frame).map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected)
        })?;
        if self.deadline_expired()
            || tainted.stage_identity_binding_ref != self.registration.stage_identity_binding_ref
            || tainted.prior_generation_ref != self.registration.prior_generation_ref
            || tainted.successor_generation_ref != self.registration.successor_generation_ref
            || tainted.candidate_binding_ref != self.registration.candidate_binding_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            ));
        }
        self.completion_consumed = true;
        Ok(Sys5I3RegisteredOwnerLifecycleCompletion {
            stage_identity_binding_ref: tainted.stage_identity_binding_ref,
            prior_generation_ref: tainted.prior_generation_ref,
            successor_generation_ref: tainted.successor_generation_ref,
            candidate_binding_ref: tainted.candidate_binding_ref,
        })
    }
}

impl Sys5I3ProcessImage {
    /// Construct a tainted, inactive provider-image candidate from one
    /// parent-held restricted admission. This does not construct a
    /// `FabricProgram`, `SealedFabricAdmission`, M9 generation, executable
    /// artifact, or provider call path.
    fn from_inactive_provider_restriction(
        slot: &Sys5I3DeploymentSlot,
        parent_checked_program_ref: &str,
        restriction: &Sys4InactiveProviderRestrictedAdmission,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if slot.slot_name.is_empty()
            || slot.endpoint.is_empty()
            || slot.loci.is_empty()
            || slot.loci.iter().cloned().collect::<BTreeSet<_>>().len() != slot.loci.len()
            || slot.loci.iter().cloned().collect::<BTreeSet<_>>() != *restriction.assigned_loci()
            || parent_checked_program_ref.is_empty()
            || restriction.component_binding_ref().is_empty()
            || restriction.declared_provider_lowering_count() != 4
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        let assigned_loci = slot.loci.iter().cloned().collect::<BTreeSet<_>>();
        let provider = Sys5I3InactiveProviderImageSeed {
            assigned_loci: assigned_loci.clone(),
            parent_checked_program_ref: parent_checked_program_ref.to_string(),
            static_snapshot: restriction.static_snapshot().clone(),
            component_snapshot: restriction
                .component()
                .i3_private_provider_component_snapshot(),
            legacy_authority_snapshot: restriction
                .legacy_authority_generation()
                .i3_private_snapshot(),
            component_binding_ref: restriction.component_binding_ref().to_string(),
        };
        // The ordinary observer seed is deliberately inert for this tagged
        // image. No composite reference is placed in its M9-generation field;
        // all provider correspondence stays in `inactive_provider` below.
        let child_seed = Sys5I3ObserverSafeChildSeed {
            parent_checked_program_ref: parent_checked_program_ref.to_string(),
            projection_ref: String::new(),
            m9_generation_ref: String::new(),
            cohort_occurrence_ref: String::new(),
            required_local_authority_closure: Sys5I3RequiredLocalAuthorityClosure {
                assigned_loci: assigned_loci.clone(),
                rows: Vec::new(),
                semantic_row_digest_ref: String::new(),
                opaque_digest_ref: String::new(),
                opaque_cohort_ref: String::new(),
            },
        };
        let private_runtime_seed = Sys5I3PrivateRuntimeSeed::InactiveProvider(Box::new(
            Sys5I3InactiveProviderRuntimeSeed {
                parent_checked_program_ref: parent_checked_program_ref.to_string(),
                static_snapshot: provider.static_snapshot.clone(),
                component_snapshot: provider.component_snapshot.clone(),
                legacy_authority_snapshot: provider.legacy_authority_snapshot.clone(),
                component_binding_ref: provider.component_binding_ref.clone(),
            },
        ));
        let mut image = Self {
            slot_name: slot.slot_name.clone(),
            endpoint: slot.endpoint.clone(),
            assigned_loci,
            executable_artifacts: Vec::new(),
            required_edge_contracts: Vec::new(),
            designated_remote_input_closure: Sys5I3DesignatedRemoteInputClosure {
                request_receipt_pair_count: 0,
                distinct_operation_count: 0,
                pairs_are_distinguished_beyond_operation: false,
                opaque_digest_ref: String::new(),
            },
            child_seed,
            private_runtime_seed,
            inactive_provider: Some(provider),
            private_integrity_ref: String::new(),
        };
        image.refresh_private_integrity();
        Ok(image)
    }

    /// Construct one child image from the one coordinator-owned checked
    /// admission.  This is intentionally private: an image must not trigger
    /// a second full check/projection/admission or authority generation.
    fn from_coordinator_parts(
        project: &Sys5LocalProject,
        deployment: &Sys5I3Deployment,
        slot_name: &str,
        coordinator_program: &FabricProgram,
        coordinator_admission: &SealedFabricAdmission,
        cohort_occurrence_ref: &str,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        let slot = deployment.slot(slot_name).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
        })?;
        let assigned_loci = slot.loci.iter().cloned().collect::<BTreeSet<_>>();
        let executable_artifacts = project
            .semantic_summary()
            .artifacts
            .iter()
            .filter(|artifact| assigned_loci.contains(&artifact.locus))
            .map(|artifact| Sys5I3ProcessArtifact {
                locus: artifact.locus.clone(),
                operation_id: artifact.operation_id.clone(),
                kind: artifact.kind.clone(),
                core_ref: artifact.core_ref.clone(),
                fragment_ref: artifact.fragment_ref.clone(),
                parent_checked_program_ref: artifact.checked_program_identity.clone(),
            })
            .collect::<Vec<_>>();
        // Retain every incident generated edge, including inbound contracts.
        // The receiver will still resolve an inbound carrier independently
        // from its sealed local projection; this inventory is provenance
        // evidence, never a sender-selected route.
        let required_edge_contracts = project
            .semantic_summary()
            .generated_communication
            .iter()
            .filter(|edge| {
                assigned_loci.contains(&edge.from_locus) || assigned_loci.contains(&edge.to_locus)
            })
            .map(|edge| {
                let core_ref = edge.core_ref.clone().ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                    )
                })?;
                Ok(Sys5I3RetainedEdgeContract {
                    source_locus: edge.from_locus.clone(),
                    target_locus: edge.to_locus.clone(),
                    edge_ref: edge.edge_ref.clone(),
                    operation_id: edge.operation_id.clone(),
                    kind: edge.kind.clone(),
                    core_ref,
                    source_artifact_ref: edge.source_fragment_ref.clone(),
                    target_artifact_ref: edge.target_fragment_ref.clone(),
                    parent_checked_program_ref: edge.checked_program_identity.clone(),
                })
            })
            .collect::<Result<Vec<_>, Sys5I3ProcessRuntimeError>>()?;

        // The full program/admission belong solely to the coordinator.  A
        // child receives the layer-owned subset produced from them, never a
        // clone of the global FabricProgram or Sys5PreparedAdmission.
        let program = coordinator_program
            .restricted_to_loci(&assigned_loci)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let admission = coordinator_admission
            .restricted_to_process_program(&program)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let parent_checked_program_ref = deployment.parent_checked_program_ref.clone();
        let projection_ref = deployment.parent_projection_ref.clone();
        let m9_generation_ref = admission.m9_generation_ref().to_string();
        let private_snapshot_binding_ref = private_runtime_seed_binding_ref(&program, &admission)
            .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        let designated_inventory = program
            .i3_process_designated_remote_input_inventory()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
        let designated_remote_input_closure = Sys5I3DesignatedRemoteInputClosure {
            request_receipt_pair_count: designated_inventory.request_receipt_pair_count(),
            distinct_operation_count: designated_inventory.distinct_operation_count(),
            pairs_are_distinguished_beyond_operation: designated_inventory
                .pairs_are_distinguished_beyond_operation(),
            opaque_digest_ref: designated_inventory.opaque_digest_ref(),
        };
        let closure = authority_closure_for_image(
            &assigned_loci,
            &executable_artifacts,
            &required_edge_contracts,
            &parent_checked_program_ref,
            &projection_ref,
            &m9_generation_ref,
            cohort_occurrence_ref,
            &admission.observer_safe_m9_semantic_row_sets_clone(),
        );
        let child_seed = Sys5I3ObserverSafeChildSeed {
            parent_checked_program_ref: parent_checked_program_ref.clone(),
            projection_ref: projection_ref.clone(),
            m9_generation_ref: m9_generation_ref.clone(),
            cohort_occurrence_ref: cohort_occurrence_ref.to_string(),
            required_local_authority_closure: closure,
        };
        let private_runtime_seed =
            Sys5I3PrivateRuntimeSeed::Ordinary(Box::new(Sys5I3OrdinaryPrivateRuntimeSeed {
                program,
                admission,
                parent_checked_program_ref,
                projection_ref,
                m9_generation_ref,
                cohort_occurrence_ref: cohort_occurrence_ref.to_string(),
                private_snapshot_binding_ref,
                prestaged_owner_capability_lifecycle: None,
            }));
        let mut image = Self {
            slot_name: slot.slot_name.clone(),
            endpoint: slot.endpoint.clone(),
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_runtime_seed,
            inactive_provider: None,
            private_integrity_ref: String::new(),
        };
        image.refresh_private_integrity();
        Ok(image)
    }

    pub fn assigned_loci(&self) -> Vec<String> {
        self.assigned_loci.iter().cloned().collect()
    }

    pub fn executable_artifacts(&self) -> &[Sys5I3ProcessArtifact] {
        &self.executable_artifacts
    }

    pub fn required_edge_contracts(&self) -> &[Sys5I3RetainedEdgeContract] {
        &self.required_edge_contracts
    }

    pub fn observer_safe_manifest(&self) -> Sys5I3ProcessImageManifest {
        Sys5I3ProcessImageManifest
    }

    pub fn observer_safe_child_seed(&self) -> &Sys5I3ObserverSafeChildSeed {
        &self.child_seed
    }

    pub fn observer_safe_designated_remote_input_closure(
        &self,
    ) -> &Sys5I3DesignatedRemoteInputClosure {
        &self.designated_remote_input_closure
    }

    pub fn into_test_only_tamper(mut self, tamper: Sys5I3ProcessImageTamper) -> Self {
        // The production type deliberately has no clone path.  The detached
        // falsifier consumes the sole image, so an invalid candidate cannot
        // leave a second startable owner image behind.
        match tamper {
            Sys5I3ProcessImageTamper::AppendForeignArtifact(artifact) => {
                self.executable_artifacts.push(artifact);
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::AppendForeignEdgeContract(contract) => {
                self.required_edge_contracts.push(contract);
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::CorruptImageIntegrity => {
                self.private_integrity_ref = "sys5-i3-corrupt-image-integrity".to_string();
            }
            Sys5I3ProcessImageTamper::RemoveProjectedDesignatedRemoteInputRequirement => {
                if let Some(seed) = self.private_runtime_seed.ordinary_mut() {
                    let _ = seed
                        .program
                        .remove_i3_process_designated_requirement_for_test();
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchProjectedDesignatedRemoteInputRequestReceipt => {
                if let Some(seed) = self.private_runtime_seed.ordinary_mut() {
                    let _ = seed
                        .program
                        .mismatch_i3_process_designated_requirement_for_test();
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::RemoveOneRequiredSemanticBinding => {
                self.child_seed.required_local_authority_closure.rows.pop();
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = self
                    .child_seed
                    .required_local_authority_closure
                    .recomputed_digest();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::AppendSemanticRowForUnassignedLocus => {
                self.child_seed.required_local_authority_closure.rows.push(
                    Sys5I3SemanticRow::Artifact {
                        locus: "i3-unassigned-locus".to_string(),
                        fragment_ref: "i3-foreign-semantic-row".to_string(),
                    },
                );
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = self
                    .child_seed
                    .required_local_authority_closure
                    .recomputed_digest();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedParentCheckedProgramRef => {
                self.child_seed.parent_checked_program_ref =
                    "sys5-i3-mismatched-parent-program".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedProjectionRef => {
                self.child_seed.projection_ref = "sys5-i3-mismatched-projection".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedM9GenerationRef => {
                self.child_seed.m9_generation_ref = "sys5-i3-mismatched-m9-generation".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedAuthorityClosureDigest => {
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = "sys5-i3-mismatched-authority-closure".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::SubstituteSameLocusArtifactAndRecomputeIntegrity(
                artifact,
            ) => {
                if let Some(row) = self
                    .executable_artifacts
                    .iter_mut()
                    .find(|row| row.locus == artifact.locus)
                {
                    *row = artifact;
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::SubstituteSameIncidentEdgeAndRecomputeIntegrity(contract) => {
                if let Some(row) = self.required_edge_contracts.iter_mut().find(|row| {
                    row.source_locus == contract.source_locus
                        && row.target_locus == contract.target_locus
                }) {
                    *row = contract;
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::DuplicateArtifactRowAndRecomputeIntegrity => {
                if let Some(row) = self.executable_artifacts.first().cloned() {
                    self.executable_artifacts.push(row);
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::DuplicateEdgeContractRowAndRecomputeIntegrity => {
                if let Some(row) = self.required_edge_contracts.first().cloned() {
                    self.required_edge_contracts.push(row);
                }
                self.refresh_private_integrity();
            }
            #[cfg(feature = "i3-process-test-seams")]
            Sys5I3ProcessImageTamper::MismatchPrestagedOwnerCapabilityLifecycleTargetSlot => {
                if let Some(lifecycle) = self
                    .private_runtime_seed
                    .ordinary_mut()
                    .and_then(|seed| seed.prestaged_owner_capability_lifecycle.as_mut())
                {
                    lifecycle.target_slot_name =
                        "i3-test-mismatched-owner-lifecycle-target".to_string();
                    self.refresh_private_integrity();
                }
            }
            Sys5I3ProcessImageTamper::RemoveActualRestrictedOwnerBindingFromPrivateSeed => {
                if let Some(seed) = self.private_runtime_seed.ordinary_mut() {
                    let _ = seed
                        .admission
                        .remove_actual_restricted_owner_binding_for_i3_process_test();
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::RemoveActualDesignatedRemoteInputLineageFromPrivateSeed => {
                if let Some(seed) = self.private_runtime_seed.ordinary_mut() {
                    let _ = seed
                        .admission
                        .remove_actual_designated_remote_input_lineage_for_i3_process_test();
                }
                self.refresh_private_integrity();
            }
        }
        self
    }

    fn refresh_private_integrity(&mut self) {
        self.private_integrity_ref = self.recomputed_private_integrity();
    }

    fn recomputed_private_integrity(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/sys5/i3/process-image-integrity/v1\\0");
        let prestaged_lifecycle_binding = match &self.private_runtime_seed {
            Sys5I3PrivateRuntimeSeed::Ordinary(seed) => seed
                .prestaged_owner_capability_lifecycle
                .as_ref()
                .map(|lifecycle| {
                    (
                        lifecycle.target_slot_name.as_str(),
                        lifecycle.stage_identity_binding_ref.as_str(),
                        lifecycle.candidate.prior_generation_ref(),
                        lifecycle.candidate.successor_generation_ref(),
                        lifecycle.candidate.candidate_binding_ref(),
                    )
                }),
            Sys5I3PrivateRuntimeSeed::InactiveProvider(_) => None,
        };
        let inactive_provider_binding = self.inactive_provider.as_ref().map(|provider| {
            (
                &provider.assigned_loci,
                &provider.parent_checked_program_ref,
                &provider.static_snapshot,
                &provider.component_snapshot,
                &provider.legacy_authority_snapshot,
                &provider.component_binding_ref,
            )
        });
        hasher.update(format!(
            "{}{}{:?}{:?}{:?}{:?}{:?}{prestaged_lifecycle_binding:?}{inactive_provider_binding:?}",
            self.slot_name,
            self.endpoint,
            self.assigned_loci,
            self.executable_artifacts,
            self.required_edge_contracts,
            self.designated_remote_input_closure,
            self.child_seed,
        ));
        format!("sys5-i3-process-image-sha256-v1:{:x}", hasher.finalize())
    }

    fn validate_before_start(&self) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.private_integrity_ref != self.recomputed_private_integrity() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        if self.inactive_provider.is_some()
            || self.private_runtime_seed.inactive_provider().is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let seed = self.private_runtime_seed.ordinary().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        if seed
            .program
            .validate_i3_process_designated_requirements()
            .is_err()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        if self
            .executable_artifacts
            .iter()
            .any(|artifact| !self.assigned_loci.contains(&artifact.locus))
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignArtifact,
            ));
        }
        if self.required_edge_contracts.iter().any(|contract| {
            !self.assigned_loci.contains(&contract.source_locus)
                && !self.assigned_loci.contains(&contract.target_locus)
        }) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignEdgeContract,
            ));
        }
        if self.child_seed.parent_checked_program_ref != seed.parent_checked_program_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if self.child_seed.projection_ref != seed.projection_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        if self.child_seed.m9_generation_ref != seed.m9_generation_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        if self.child_seed.cohort_occurrence_ref() != seed.cohort_occurrence_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        let (expected_artifacts, expected_edges) =
            seed.program.i3_process_normalized_inventory_refs();
        let actual_artifacts = self
            .executable_artifacts
            .iter()
            .map(normalized_artifact_inventory_ref)
            .collect::<BTreeSet<_>>();
        let actual_edges = self
            .required_edge_contracts
            .iter()
            .map(normalized_edge_inventory_ref)
            .collect::<BTreeSet<_>>();
        let artifact_parent_matches = self.executable_artifacts.iter().all(|artifact| {
            artifact.parent_checked_program_ref == self.child_seed.parent_checked_program_ref
        });
        let edge_parent_matches = self.required_edge_contracts.iter().all(|contract| {
            contract.parent_checked_program_ref == self.child_seed.parent_checked_program_ref
        });
        if !artifact_parent_matches
            || !edge_parent_matches
            || actual_artifacts.len() != self.executable_artifacts.len()
            || actual_edges.len() != self.required_edge_contracts.len()
            || actual_artifacts != expected_artifacts
            || actual_edges != expected_edges
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch,
            ));
        }
        let designated_inventory = seed
            .program
            .i3_process_designated_remote_input_inventory()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
        let expected_designated_closure = Sys5I3DesignatedRemoteInputClosure {
            request_receipt_pair_count: designated_inventory.request_receipt_pair_count(),
            distinct_operation_count: designated_inventory.distinct_operation_count(),
            pairs_are_distinguished_beyond_operation: designated_inventory
                .pairs_are_distinguished_beyond_operation(),
            opaque_digest_ref: designated_inventory.opaque_digest_ref(),
        };
        if self.designated_remote_input_closure != expected_designated_closure {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        let closure = &self.child_seed.required_local_authority_closure;
        if !closure.has_no_unassigned_semantic_rows() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignAuthorityEvidence,
            ));
        }
        if closure.opaque_digest_ref != closure.recomputed_digest() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::AuthorityClosureDigestMismatch,
            ));
        }
        let expected = authority_closure_for_image(
            &self.assigned_loci,
            &self.executable_artifacts,
            &self.required_edge_contracts,
            &self.child_seed.parent_checked_program_ref,
            &self.child_seed.projection_ref,
            &self.child_seed.m9_generation_ref,
            self.child_seed.cohort_occurrence_ref(),
            &seed.admission.observer_safe_m9_semantic_row_sets_clone(),
        );
        if closure != &expected {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::MissingRequiredAuthorityEvidence,
            ));
        }
        Ok(())
    }
}

/// Observer-safe image manifest.  It intentionally makes the non-carriage
/// guarantees explicit rather than exporting image internals.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ProcessImageManifest;

impl Sys5I3ProcessImageManifest {
    pub const fn carries_source_text(&self) -> bool {
        false
    }

    pub const fn carries_host_path(&self) -> bool {
        false
    }

    pub const fn carries_expected_result(&self) -> bool {
        false
    }
}

fn normalized_artifact_inventory_ref(artifact: &Sys5I3ProcessArtifact) -> String {
    format!(
        "artifact|{}|{}|{}|{}|{}",
        artifact.locus,
        artifact.operation_id,
        artifact.kind,
        artifact.core_ref,
        artifact.fragment_ref
    )
}

fn normalized_edge_inventory_ref(contract: &Sys5I3RetainedEdgeContract) -> String {
    format!(
        "edge|{}|{}|{}|{}|{}|{}|{}|{}",
        contract.source_locus,
        contract.target_locus,
        contract.operation_id,
        contract.kind,
        contract.edge_ref,
        contract.core_ref,
        contract.source_artifact_ref,
        contract.target_artifact_ref,
    )
}

#[allow(clippy::too_many_arguments)]
fn authority_closure_for_image(
    assigned_loci: &BTreeSet<String>,
    artifacts: &[Sys5I3ProcessArtifact],
    contracts: &[Sys5I3RetainedEdgeContract],
    parent_checked_program_ref: &str,
    projection_ref: &str,
    m9_generation_ref: &str,
    cohort_occurrence_ref: &str,
    semantic_rows: &ObserverSafeM9SemanticRowSets,
) -> Sys5I3RequiredLocalAuthorityClosure {
    let mut rows = artifacts
        .iter()
        .map(|artifact| Sys5I3SemanticRow::Artifact {
            locus: artifact.locus.clone(),
            fragment_ref: artifact.fragment_ref.clone(),
        })
        .chain(
            contracts
                .iter()
                .map(|contract| Sys5I3SemanticRow::IncidentEdge {
                    source_locus: contract.source_locus.clone(),
                    target_locus: contract.target_locus.clone(),
                    edge_ref: contract.edge_ref.clone(),
                }),
        )
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| format!("{row:?}"));
    let mut cohort_hasher = Sha256::new();
    cohort_hasher.update(b"mirrorea/sys5/i3/process-cohort/v1\\0");
    cohort_hasher.update(parent_checked_program_ref);
    cohort_hasher.update(projection_ref);
    cohort_hasher.update(m9_generation_ref);
    cohort_hasher.update(cohort_occurrence_ref);
    let opaque_cohort_ref = format!(
        "sys5-i3-process-cohort-sha256-v1:{:x}",
        cohort_hasher.finalize()
    );
    let mut closure = Sys5I3RequiredLocalAuthorityClosure {
        assigned_loci: assigned_loci.clone(),
        rows,
        semantic_row_digest_ref: observer_safe_semantic_rows_digest(semantic_rows),
        opaque_digest_ref: String::new(),
        opaque_cohort_ref,
    };
    closure.opaque_digest_ref = closure.recomputed_digest();
    closure
}

fn fresh_occurrence_ref(
    domain: &str,
    parent_checked_program_ref: &str,
    projection_ref: &str,
) -> String {
    let ordinal = NEXT_PROCESS_COHORT_OCCURRENCE.fetch_add(1, Ordering::Relaxed);
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/local-occurrence/v1\\0");
    hasher.update(domain);
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(ordinal.to_le_bytes());
    format!(
        "sys5-i3-{domain}-occurrence-sha256-v1:{:x}",
        hasher.finalize()
    )
}

/// Bind the one pre-staged lifecycle to the independently retained parent and
/// B-bootstrap facts.  This is an opaque commitment, not a credential: only
/// the parent computes it while holding the genuine candidate, and it is
/// compared at every subsequent lifecycle boundary.
fn owner_capability_lifecycle_stage_identity_binding_ref(
    run_ref: &str,
    image: &Sys5I3ProcessImage,
    candidate: &Sys4I3RestrictedOwnerCapabilitySuccessor,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/owner-capability-lifecycle-stage/v1\0");
    for component in [
        run_ref,
        image.child_seed.parent_checked_program_ref.as_str(),
        image.child_seed.projection_ref.as_str(),
        image.child_seed.cohort_occurrence_ref.as_str(),
        image.slot_name.as_str(),
        image
            .child_seed
            .required_local_authority_closure
            .opaque_digest_ref(),
        candidate.operation(),
        candidate.owner_locus(),
        candidate.prior_generation_ref(),
        candidate.successor_generation_ref(),
        "owner-capability-revocation",
        candidate.candidate_binding_ref(),
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component.as_bytes());
    }
    hasher.update((image.assigned_loci.len() as u64).to_be_bytes());
    for locus in &image.assigned_loci {
        hasher.update((locus.len() as u64).to_be_bytes());
        hasher.update(locus.as_bytes());
    }
    format!(
        "sys5-i3-owner-capability-lifecycle-stage-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn observer_safe_semantic_rows_digest(rows: &ObserverSafeM9SemanticRowSets) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-authority-semantic-rows/v1\\0");
    hasher.update(format!("{rows:?}"));
    format!(
        "sys5-i3-process-authority-semantic-rows-sha256-v1:{:x}",
        hasher.finalize()
    )
}

/// Bind both independently restored private roots.  The sealed admission
/// commitment covers M8/M9/state facts; the projection commitment covers the
/// complete restricted executable structure rather than merely its route
/// fingerprint.  No source text, transport identity, or new authority enters
/// this value.
fn private_runtime_seed_binding_ref(
    program: &FabricProgram,
    admission: &SealedFabricAdmission,
) -> Result<String, ()> {
    let projection_binding = program.i3_private_projection_binding_ref()?;
    let admission_binding = admission.i3_private_snapshot_binding_ref();
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/private-runtime-seed-binding/v1\0");
    hasher.update(projection_binding);
    hasher.update(admission_binding);
    Ok(format!(
        "sys5-i3-private-runtime-seed-binding-sha256-v1:{:x}",
        hasher.finalize()
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sys5I3ProcessMessageKind {
    Request,
    Reply,
    Receipt,
    TerminalFailureConsumed,
}

/// One generated carrier moving by value between two process runtimes.  It
/// has no socket, connection, session, or certificate binding in G1.
#[doc(hidden)]
#[derive(Debug)]
pub struct Sys5I3ProcessMessage {
    kind: Sys5I3ProcessMessageKind,
    carrier: Option<Sys4ProcessCarrier>,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    // Present only on the local terminal-failure consumption result.  It is
    // copied from the retained requester ledger, never decoded from a peer.
    terminal_failure_decision_occurrence_ref: Option<String>,
    // Private admission provenance.  This binds a generated carrier to the
    // cohort occurrence which sealed its process images; it is neither an
    // authority fact nor a substitute for M9 validation.
    cohort_provenance_ref: String,
    identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
}

impl Sys5I3ProcessMessage {
    pub const fn transport_binding(&self) -> Option<()> {
        None
    }

    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn linked_request_identity_ref(&self) -> Option<&str> {
        self.linked_request_identity_ref.as_deref()
    }

    pub const fn is_observer_safe_typed_result_or_receipt(&self) -> bool {
        matches!(self.kind, Sys5I3ProcessMessageKind::Receipt)
    }

    /// A locally retained declared owner-admission failure was consumed.  It
    /// is deliberately distinct from a receipt or successful typed result.
    pub const fn is_observer_safe_terminal_failure_consumed(&self) -> bool {
        matches!(self.kind, Sys5I3ProcessMessageKind::TerminalFailureConsumed)
            && self.terminal_failure_decision_occurrence_ref.is_some()
    }

    pub fn has_no_transportable_carrier(&self) -> bool {
        matches!(
            self.kind,
            Sys5I3ProcessMessageKind::Receipt | Sys5I3ProcessMessageKind::TerminalFailureConsumed
        ) && self.carrier.is_none()
    }

    pub const fn observer_safe_identity_basis(
        &self,
    ) -> Sys5I3ObserverSafeSemanticRequestIdentityBasis {
        self.identity_basis
    }
}

/// Declared bounded limits for the private G2 codec.  They are an internal
/// fail-closed resource boundary, not a public wire-format commitment.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3PrivateProcessCodecLimits {
    max_image_bytes: usize,
    max_message_bytes: usize,
}

impl Sys5I3PrivateProcessCodecLimits {
    pub const fn max_image_bytes(&self) -> usize {
        self.max_image_bytes
    }

    pub const fn max_message_bytes(&self) -> usize {
        self.max_message_bytes
    }
}

/// Typed rejection classes for the private I3-2 codec.  These names and the
/// JSON representation are provisional implementation detail only.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3PrivateProcessCodecErrorKind {
    Malformed,
    Incomplete,
    Oversized,
    UnknownVersion,
    MissingRequiredCoreProvenance,
    ReceiptIsLocalOnly,
    TerminalFailureIsLocalOnly,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3PrivateProcessCodecError {
    kind: Sys5I3PrivateProcessCodecErrorKind,
}

impl Sys5I3PrivateProcessCodecError {
    fn new(kind: Sys5I3PrivateProcessCodecErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3PrivateProcessCodecErrorKind {
        self.kind
    }
}

/// A length-prefixed, private codec for the one-shot I3 child image and the
/// generated owner request/reply carrier.  It accepts only exact restricted
/// snapshots; decode returns untrusted candidates that cannot start a child
/// or enter a mailbox without a receiver-owned validation boundary.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sys5I3PrivateProcessCodec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProcessMessageEnvelope {
    version: u64,
    message: PrivateProcessMessageSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProcessMessageSnapshot {
    kind: PrivateProcessMessageKind,
    carrier: Sys4I3PrivateProcessCarrierSnapshot,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    cohort_provenance_ref: String,
}

/// Private nested provider carrier framing.  It deliberately has no route
/// through the public generated owner message codec and is decoded only by
/// the private QUIC adapter before the installed child validates exact local
/// M9/static correspondence.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProviderCarrierEnvelope {
    version: u8,
    carrier: PrivateProviderCarrierPayload,
}

#[derive(Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    tag = "kind",
    content = "payload",
    deny_unknown_fields
)]
enum PrivateProviderCarrierPayload {
    Request(PrivateProviderRequestCarrierSnapshot),
    Result(PrivateProviderResultCarrierSnapshot),
}

const PRIVATE_OWNER_LIFECYCLE_ACK_VERSION: u64 = 2;

/// Codec-decoded lifecycle ACK candidate.  This is deliberately tainted:
/// decoding correctly shaped fields cannot claim B installation, registered
/// child-stream origin, or authority publication.
#[doc(hidden)]
pub struct Sys5I3TaintedOwnerLifecycleAck {
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

impl Sys5I3TaintedOwnerLifecycleAck {
    /// This reports only that the finite decoded record retained all required
    /// nonempty fields; it exposes none of them and remains no proof of B
    /// installation, registered-FD origin, or publication authority.
    pub fn is_complete_tainted_candidate(&self) -> bool {
        !self.prior_generation_ref.is_empty()
            && !self.successor_generation_ref.is_empty()
            && !self.candidate_binding_ref.is_empty()
            && !self.stage_identity_binding_ref.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateOwnerLifecycleAckSnapshot {
    version: u64,
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

/// JSON DTO for a coordinator-retained start binding.  This remains inside
/// the private trusted-control codec; it is intentionally separate from the
/// tainted image DTO and cannot be recovered by image decoding alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateExpectedStartBindingSnapshot {
    slot_name: String,
    assigned_loci: Vec<String>,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_provenance_ref: String,
    image_integrity_ref: String,
    private_snapshot_binding_ref: String,
    expected_owner_capability_lifecycle: Option<PrivateExpectedOwnerCapabilityLifecycleSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateExpectedOwnerCapabilityLifecycleSnapshot {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    candidate_binding_ref: String,
}

impl From<&Sys5I3ExpectedStartBinding> for PrivateExpectedStartBindingSnapshot {
    fn from(binding: &Sys5I3ExpectedStartBinding) -> Self {
        Self {
            slot_name: binding.slot_name.clone(),
            assigned_loci: binding.assigned_loci.iter().cloned().collect(),
            parent_checked_program_ref: binding.parent_checked_program_ref.clone(),
            projection_ref: binding.projection_ref.clone(),
            m9_generation_ref: binding.m9_generation_ref.clone(),
            cohort_provenance_ref: binding.cohort_provenance_ref.clone(),
            image_integrity_ref: binding.image_integrity_ref.clone(),
            private_snapshot_binding_ref: binding.private_snapshot_binding_ref.clone(),
            expected_owner_capability_lifecycle: binding
                .expected_owner_capability_lifecycle
                .as_ref()
                .map(
                    |lifecycle| PrivateExpectedOwnerCapabilityLifecycleSnapshot {
                        target_slot_name: lifecycle.target_slot_name.clone(),
                        stage_identity_binding_ref: lifecycle.stage_identity_binding_ref.clone(),
                        prior_generation_ref: lifecycle.prior_generation_ref.clone(),
                        successor_generation_ref: lifecycle.successor_generation_ref.clone(),
                        candidate_binding_ref: lifecycle.candidate_binding_ref.clone(),
                    },
                ),
        }
    }
}

impl TryFrom<PrivateExpectedStartBindingSnapshot> for Sys5I3ExpectedStartBinding {
    type Error = ();

    fn try_from(snapshot: PrivateExpectedStartBindingSnapshot) -> Result<Self, Self::Error> {
        if snapshot.slot_name.is_empty()
            || snapshot.assigned_loci.is_empty()
            || snapshot.parent_checked_program_ref.is_empty()
            || snapshot.projection_ref.is_empty()
            || snapshot.m9_generation_ref.is_empty()
            || snapshot.cohort_provenance_ref.is_empty()
            || snapshot.image_integrity_ref.is_empty()
            || snapshot.private_snapshot_binding_ref.is_empty()
        {
            return Err(());
        }
        let expected_owner_capability_lifecycle = snapshot
            .expected_owner_capability_lifecycle
            .map(|lifecycle| {
                if lifecycle.target_slot_name.is_empty()
                    || lifecycle.stage_identity_binding_ref.is_empty()
                    || lifecycle.prior_generation_ref.is_empty()
                    || lifecycle.successor_generation_ref.is_empty()
                    || lifecycle.candidate_binding_ref.is_empty()
                {
                    return Err(());
                }
                Ok(Sys5I3ExpectedOwnerCapabilityLifecycle {
                    target_slot_name: lifecycle.target_slot_name,
                    stage_identity_binding_ref: lifecycle.stage_identity_binding_ref,
                    prior_generation_ref: lifecycle.prior_generation_ref,
                    successor_generation_ref: lifecycle.successor_generation_ref,
                    candidate_binding_ref: lifecycle.candidate_binding_ref,
                })
            })
            .transpose()?;
        let assigned_loci = snapshot.assigned_loci.into_iter().collect::<BTreeSet<_>>();
        if assigned_loci.is_empty() {
            return Err(());
        }
        Ok(Self {
            slot_name: snapshot.slot_name,
            assigned_loci,
            parent_checked_program_ref: snapshot.parent_checked_program_ref,
            projection_ref: snapshot.projection_ref,
            m9_generation_ref: snapshot.m9_generation_ref,
            cohort_provenance_ref: snapshot.cohort_provenance_ref,
            image_integrity_ref: snapshot.image_integrity_ref,
            private_snapshot_binding_ref: snapshot.private_snapshot_binding_ref,
            expected_owner_capability_lifecycle,
            inactive_provider: None,
        })
    }
}

/// The provider FD3 record keeps its expected binding in a distinct private
/// DTO. Ordinary start bindings intentionally cannot represent a tagged
/// provider image, and this conversion never defaults that tag away.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateInactiveProviderExpectedStartBindingSnapshot {
    slot_name: String,
    assigned_loci: Vec<String>,
    parent_checked_program_ref: String,
    image_integrity_ref: String,
    static_snapshot: I3PrivateProviderStaticProjectionSnapshot,
    component_snapshot: M8I3PrivateProviderComponentSnapshot,
    legacy_authority_snapshot: M9I3PrivateAuthorityGenerationSnapshot,
    component_binding_ref: String,
}

impl TryFrom<&Sys5I3ExpectedStartBinding> for PrivateInactiveProviderExpectedStartBindingSnapshot {
    type Error = ();

    fn try_from(binding: &Sys5I3ExpectedStartBinding) -> Result<Self, Self::Error> {
        let provider = binding.inactive_provider.as_ref().ok_or(())?;
        if binding.slot_name.is_empty()
            || binding.assigned_loci.is_empty()
            || binding.parent_checked_program_ref.is_empty()
            || binding.image_integrity_ref.is_empty()
            || provider.component_binding_ref.is_empty()
            || provider.static_snapshot.declared_provider_lowering_count() != 4
            || !provider.component_snapshot.is_scoped_component_snapshot()
        {
            return Err(());
        }
        Ok(Self {
            slot_name: binding.slot_name.clone(),
            assigned_loci: binding.assigned_loci.iter().cloned().collect(),
            parent_checked_program_ref: binding.parent_checked_program_ref.clone(),
            image_integrity_ref: binding.image_integrity_ref.clone(),
            static_snapshot: provider.static_snapshot.clone(),
            component_snapshot: provider.component_snapshot.clone(),
            legacy_authority_snapshot: provider.legacy_authority_snapshot.clone(),
            component_binding_ref: provider.component_binding_ref.clone(),
        })
    }
}

impl TryFrom<PrivateInactiveProviderExpectedStartBindingSnapshot> for Sys5I3ExpectedStartBinding {
    type Error = ();

    fn try_from(
        snapshot: PrivateInactiveProviderExpectedStartBindingSnapshot,
    ) -> Result<Self, Self::Error> {
        let assigned_loci = snapshot.assigned_loci.into_iter().collect::<BTreeSet<_>>();
        if snapshot.slot_name.is_empty()
            || assigned_loci.is_empty()
            || snapshot.parent_checked_program_ref.is_empty()
            || snapshot.image_integrity_ref.is_empty()
            || snapshot.component_binding_ref.is_empty()
            || snapshot.static_snapshot.declared_provider_lowering_count() != 4
            || !snapshot.component_snapshot.is_scoped_component_snapshot()
        {
            return Err(());
        }
        Ok(Sys5I3ExpectedStartBinding {
            slot_name: snapshot.slot_name,
            assigned_loci,
            parent_checked_program_ref: snapshot.parent_checked_program_ref,
            projection_ref: String::new(),
            m9_generation_ref: String::new(),
            cohort_provenance_ref: String::new(),
            image_integrity_ref: snapshot.image_integrity_ref,
            private_snapshot_binding_ref: String::new(),
            expected_owner_capability_lifecycle: None,
            inactive_provider: Some(Sys5I3ExpectedInactiveProviderBinding {
                static_snapshot: snapshot.static_snapshot,
                component_snapshot: snapshot.component_snapshot,
                legacy_authority_snapshot: snapshot.legacy_authority_snapshot,
                component_binding_ref: snapshot.component_binding_ref,
            }),
        })
    }
}

/// The peer portion of a provider FD3 record. It is derived only from the
/// coordinator's complete parent-held binding and preserves exactly the facts
/// consumed later by the peer-preface validator. It intentionally is not a
/// peer image or authority restore DTO.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PrivateProviderPeerStartBinding {
    slot_name: String,
    parent_checked_program_ref: String,
    image_integrity_ref: String,
    component_binding_ref: String,
}

impl PrivateProviderPeerStartBinding {
    fn is_structurally_valid(&self) -> bool {
        !self.slot_name.is_empty()
            && is_checked_program_identity_ref(&self.parent_checked_program_ref)
            && !self.image_integrity_ref.is_empty()
            && !self.component_binding_ref.is_empty()
    }

    #[cfg(test)]
    fn matches_parent_held_binding(&self, binding: &Sys5I3ExpectedStartBinding) -> bool {
        Self::try_from(binding).is_ok_and(|expected| expected == *self)
    }
}

impl TryFrom<&Sys5I3ExpectedStartBinding> for PrivateProviderPeerStartBinding {
    type Error = ();

    fn try_from(binding: &Sys5I3ExpectedStartBinding) -> Result<Self, Self::Error> {
        let provider = binding.inactive_provider.as_ref().ok_or(())?;
        // Keep the pre-existing assigned-locus guard at the trusted producer.
        // The compact peer record is not a full image DTO and cannot carry an
        // additional child-local locus set without recreating the duplication
        // this descriptor removes.
        if binding.slot_name.is_empty()
            || binding.assigned_loci.is_empty()
            || binding.parent_checked_program_ref.is_empty()
            || binding.image_integrity_ref.is_empty()
            || provider.component_binding_ref.is_empty()
            || provider.static_snapshot.declared_provider_lowering_count() != 4
            || !provider.component_snapshot.is_scoped_component_snapshot()
        {
            return Err(());
        }
        Ok(Self {
            slot_name: binding.slot_name.clone(),
            parent_checked_program_ref: checked_program_identity_ref(
                &binding.parent_checked_program_ref,
            ),
            image_integrity_ref: binding.image_integrity_ref.clone(),
            component_binding_ref: provider.component_binding_ref.clone(),
        })
    }
}

// Version 7 rejects every previous private FD3 record: v2 duplicated the
// peer's full scoped image/authority snapshot, v3 carried only the exact
// peer identity facts, v4 bound the requester-only finite T0 fixture
// assertion, v5 carried the sealed fixed observer-conformance profile, and
// v6 expands the sealed finite fixture-assertion vocabulary. Version 7 adds
// the independently sealed fixed provider-network conformance profile. No
// earlier private record is accepted as if it had selected either profile.
const PRIVATE_PROVIDER_CHILD_CONTROL_VERSION: u64 = 7;
// This is a provider-FD3-body-only ceiling. The accepted ordinary private
// message ceiling remains 64 KiB, while image and queue bounds stay separate.
// The handoff can transiently retain this bounded body, its strict JSON value,
// its decoded DTO, and the independently bounded child image; this limit is
// not an RSS or whole-process memory guarantee.
const PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES: usize = 5 * 1024 * 1024;

/// One whole FD3 control record for a provider child. It is decoded only by
/// the runtime's inherited-FD3 reader; there is no generic decoded-control
/// constructor for its authority-bearing fields.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProviderChildControlSnapshot {
    version: u64,
    role: M9I3ReadOnlyProviderChildRole,
    expected_start_binding: PrivateInactiveProviderExpectedStartBindingSnapshot,
    peer_role: M9I3ReadOnlyProviderChildRole,
    peer_start_binding: PrivateProviderPeerStartBinding,
    m9_role_snapshot: M9I3PrivateReadOnlyProviderRoleSnapshot,
    expected_resource_runtime_nonce: [u8; 32],
    executor_resource_envelope: Option<I3PrivateReadOnlyProviderResourceEnvelope>,
    requester_fixture_assertion: Option<I3PrivateReadOnlyProviderFixtureAssertion>,
    terminal_observation_conformance_profile:
        Option<I3PrivateFixedTerminalObservationConformanceProfile>,
    provider_network_conformance_profile: Option<I3PrivateFixedProviderNetworkConformanceProfile>,
    transport_bootstrap: Vec<u8>,
}

/// Bounded serialization sink for the one authority-bearing provider FD3
/// body. It checks a write's complete remaining budget before extending the
/// backing vector, so JSON serialization cannot first allocate an oversized
/// body and only then reject it.
#[cfg(unix)]
struct BoundedProviderControlJsonWriter {
    bytes: Vec<u8>,
    limit: usize,
    overflowed: bool,
}

#[cfg(unix)]
impl BoundedProviderControlJsonWriter {
    fn new(limit: usize) -> std::io::Result<Self> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(limit)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::Other))?;
        // `try_reserve_exact` asks the allocator for this exact bounded
        // policy capacity. Refuse an allocator result larger than the policy
        // rather than allowing later `extend_from_slice` growth to exceed it.
        if bytes.capacity() > limit {
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
        Ok(Self {
            bytes,
            limit,
            overflowed: false,
        })
    }

    fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(unix)]
impl Write for BoundedProviderControlJsonWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.bytes.len());
        if bytes.len() > remaining {
            self.overflowed = true;
            return Err(std::io::Error::from(std::io::ErrorKind::WriteZero));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(unix)]
fn serialize_bounded_provider_child_control_snapshot(
    snapshot: &PrivateProviderChildControlSnapshot,
) -> Result<Vec<u8>, Sys5I3ProviderLaunchError> {
    let mut writer = BoundedProviderControlJsonWriter::new(
        PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES,
    )
    .map_err(|_| {
        Sys5I3ProviderLaunchError::new(
            Sys5I3ProviderLaunchErrorKind::ProviderControlSerializationRejected,
        )
    })?;
    let serialization = serde_json::to_writer(&mut writer, snapshot);
    if writer.overflowed {
        return Err(Sys5I3ProviderLaunchError::new(
            Sys5I3ProviderLaunchErrorKind::ProviderControlSnapshotOversized,
        ));
    }
    serialization.map_err(|_| {
        Sys5I3ProviderLaunchError::new(
            Sys5I3ProviderLaunchErrorKind::ProviderControlSerializationRejected,
        )
    })?;
    Ok(writer.into_bytes())
}

/// Test-only non-sensitive size evidence for one production-built provider
/// control record. It contains only fixed field names and encoded lengths;
/// no DTO, bytes, path, grant, witness, or credential can be recovered from
/// it. The transport upper bound accounts for JSON's at-most-four-byte
/// encoding per one of the permitted 64 KiB physical transport bytes.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PrivateProviderChildControlSnapshotSizeMetrics {
    local_static_snapshot_bytes: usize,
    local_component_snapshot_bytes: usize,
    local_legacy_authority_snapshot_bytes: usize,
    role_m9_snapshot_bytes: usize,
    peer_start_binding_bytes: usize,
    whole_control_empty_transport_bytes: usize,
    whole_control_max_transport_upper_bound_bytes: usize,
}

#[cfg(test)]
impl PrivateProviderChildControlSnapshotSizeMetrics {
    fn from_snapshot(
        snapshot: &PrivateProviderChildControlSnapshot,
    ) -> Result<Self, Sys5I3ProviderLaunchError> {
        fn encoded_len<T: Serialize>(value: &T) -> Result<usize, Sys5I3ProviderLaunchError> {
            serde_json::to_vec(value)
                .map(|encoded| encoded.len())
                .map_err(|_| {
                    Sys5I3ProviderLaunchError::new(
                        Sys5I3ProviderLaunchErrorKind::ProviderControlSerializationRejected,
                    )
                })
        }

        let whole_control_empty_transport_bytes = encoded_len(snapshot)?;
        let transport_upper_bound = Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES
            .checked_mul(4)
            .ok_or_else(|| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlSerializationRejected,
                )
            })?;
        let whole_control_max_transport_upper_bound_bytes = whole_control_empty_transport_bytes
            .checked_add(transport_upper_bound)
            .ok_or_else(|| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::ProviderControlSerializationRejected,
                )
            })?;
        Ok(Self {
            local_static_snapshot_bytes: encoded_len(
                &snapshot.expected_start_binding.static_snapshot,
            )?,
            local_component_snapshot_bytes: encoded_len(
                &snapshot.expected_start_binding.component_snapshot,
            )?,
            local_legacy_authority_snapshot_bytes: encoded_len(
                &snapshot.expected_start_binding.legacy_authority_snapshot,
            )?,
            role_m9_snapshot_bytes: encoded_len(&snapshot.m9_role_snapshot)?,
            peer_start_binding_bytes: encoded_len(&snapshot.peer_start_binding)?,
            whole_control_empty_transport_bytes,
            whole_control_max_transport_upper_bound_bytes,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateTrustedLocalnetControlSnapshot {
    version: u64,
    expected_start_binding: PrivateExpectedStartBindingSnapshot,
    run_ref: String,
    local_slot_name: String,
    peer_slot_name: String,
    local_spki_ref: String,
    peer_spki_ref: String,
    peer_image_integrity_ref: String,
    peer_checked_program_ref: String,
    peer_projection_ref: String,
    peer_cohort_provenance_ref: String,
}

const PRIVATE_TRUSTED_LOCALNET_CONTROL_VERSION: u64 = 1;

/// Parse one untrusted JSON value without normalizing duplicate object keys.
/// `serde_json::Value` otherwise accepts duplicate members with last-write
/// wins semantics, which would erase the malformed input before the strict
/// private DTO boundary can reject it.
struct StrictJsonValue(serde_json::Value);

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonValueVisitor)
    }
}

struct StrictJsonValueVisitor;

impl<'de> Visitor<'de> for StrictJsonValueVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("strict JSON without duplicate object members")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("JSON number must be finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::String(
            value.to_string(),
        )))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StrictJsonValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(A::Error::custom("duplicate JSON object member"));
            }
            let value = map.next_value::<StrictJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Object(values)))
    }
}

/// Read exactly one provider FD3 body after its fixed prefix. This is private
/// parser work only: the sole production caller is the inherited-FD3 branch,
/// and parsing never constructs an installed runtime. The declared body size
/// is rejected before any body allocation; after allocation the exact body,
/// EOF, strict duplicate-key parsing, and closed DTO shape are all required.
#[cfg(unix)]
fn read_private_provider_child_control_frame<R: Read>(
    reader: &mut R,
) -> Result<PrivateProviderChildControlSnapshot, Sys5I3ProviderLaunchError> {
    fn rejected() -> Sys5I3ProviderLaunchError {
        Sys5I3ProviderLaunchError::new(
            Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
        )
    }

    let mut prefix = [0_u8; 4];
    reader.read_exact(&mut prefix).map_err(|_| rejected())?;
    let declared = u32::from_be_bytes(prefix) as usize;
    if declared > PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES {
        return Err(rejected());
    }

    let mut body = Vec::new();
    body.try_reserve_exact(declared).map_err(|_| rejected())?;
    body.resize(declared, 0);
    reader.read_exact(&mut body).map_err(|_| rejected())?;
    let mut trailing = [0_u8; 1];
    match reader.read(&mut trailing) {
        Ok(0) => {}
        Ok(_) | Err(_) => return Err(rejected()),
    }

    let value = strict_json_value(&body).map_err(|_| rejected())?;
    serde_json::from_value(value).map_err(|_| rejected())
}

pub(crate) fn strict_json_value(bytes: &[u8]) -> Result<serde_json::Value, ()> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictJsonValue::deserialize(&mut deserializer).map_err(|_| ())?;
    deserializer.end().map_err(|_| ())?;
    Ok(value.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateProcessMessageKind {
    Request,
    Reply,
}

/// Codec-decoded image that has no direct runtime constructor.  Its only
/// promotion path is `validate_and_start_image`, which requires the T0
/// coordinator's separately retained expected binding.
#[doc(hidden)]
pub struct Sys5I3UntrustedProcessImage {
    image: Sys5I3ProcessImage,
}

impl std::fmt::Debug for Sys5I3UntrustedProcessImage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3UntrustedProcessImage")
            .field("assigned_loci", &self.image.assigned_loci)
            .field(
                "executable_artifact_count",
                &self.image.executable_artifacts.len(),
            )
            .field(
                "incident_edge_contract_count",
                &self.image.required_edge_contracts.len(),
            )
            .finish_non_exhaustive()
    }
}

/// Observer-safe facts from a decoded image.  It deliberately omits the
/// executable projection/admission values and every raw M8/M9 datum.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3UntrustedProcessImageManifest {
    assigned_loci: Vec<String>,
    has_assigned_artifacts_only: bool,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_provenance_ref: String,
}

impl Sys5I3UntrustedProcessImage {
    pub fn observer_safe_manifest(&self) -> Sys5I3UntrustedProcessImageManifest {
        Sys5I3UntrustedProcessImageManifest {
            assigned_loci: self.image.assigned_loci(),
            has_assigned_artifacts_only: self
                .image
                .executable_artifacts
                .iter()
                .all(|artifact| self.image.assigned_loci.contains(&artifact.locus)),
            parent_checked_program_ref: self.image.child_seed.parent_checked_program_ref.clone(),
            projection_ref: self.image.child_seed.projection_ref.clone(),
            m9_generation_ref: self.image.child_seed.m9_generation_ref.clone(),
            cohort_provenance_ref: self
                .image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
                .to_string(),
        }
    }

    #[cfg(test)]
    pub(crate) fn into_test_only_inactive_provider_tamper(
        mut self,
        tamper: Sys5I3InactiveProviderImageTamper,
    ) -> Self {
        if self.image.inactive_provider.is_none() {
            return self;
        }
        match tamper {
            Sys5I3InactiveProviderImageTamper::ChangeProviderRoleDescriptor => {
                let _ = self.image.inactive_provider.as_mut().and_then(|provider| provider.static_snapshot.test_only_tamper(
                    crate::sys3_i3_private_snapshot::I3PrivateProviderStaticSnapshotTamper::ChangeRoleDescriptor,
                ).then_some(()));
            }
            Sys5I3InactiveProviderImageTamper::ChangeProviderEdgeDescriptor => {
                let _ = self.image.inactive_provider.as_mut().and_then(|provider| provider.static_snapshot.test_only_tamper(
                    crate::sys3_i3_private_snapshot::I3PrivateProviderStaticSnapshotTamper::ChangeEdgeDescriptor,
                ).then_some(()));
            }
            Sys5I3InactiveProviderImageTamper::ChangeRetainedLoweringAssociation => {
                let _ = self.image.inactive_provider.as_mut().and_then(|provider| {
                    provider
                        .component_snapshot
                        .test_only_change_retained_lowering_association()
                        .then_some(())
                });
            }
            Sys5I3InactiveProviderImageTamper::RemoveExcludedProviderLoweringAssociation => {
                let _ = self.image.inactive_provider.as_mut().and_then(|provider| {
                    provider
                        .static_snapshot
                        .test_only_tamper(
                            crate::sys3_i3_private_snapshot::I3PrivateProviderStaticSnapshotTamper::RemoveProviderSourceMapAssociation,
                        )
                        .then_some(())
                });
            }
            Sys5I3InactiveProviderImageTamper::ChangeEndpointMetadata => {
                self.image.endpoint.push_str(":i3-provider-tampered");
            }
            Sys5I3InactiveProviderImageTamper::RemoveRequiredLegacyAuthorityLineage => {
                let _ = self.image.inactive_provider.as_mut().and_then(|provider| {
                    provider
                        .legacy_authority_snapshot
                        .test_only_remove_required_legacy_authority_lineage()
                        .then_some(())
                });
            }
            Sys5I3InactiveProviderImageTamper::StripAssignedLocus => {
                if let Some(locus) = self.image.assigned_loci.iter().next().cloned() {
                    self.image.assigned_loci.remove(&locus);
                    if let Some(provider) = self.image.inactive_provider.as_mut() {
                        provider.assigned_loci.remove(&locus);
                    }
                }
            }
            Sys5I3InactiveProviderImageTamper::WidenAssignedLocus => {
                let widened = "i3-provider-unassigned-locus".to_string();
                self.image.assigned_loci.insert(widened.clone());
                if let Some(provider) = self.image.inactive_provider.as_mut() {
                    provider.assigned_loci.insert(widened);
                }
            }
        }
        self.image.refresh_private_integrity();
        self
    }
}

impl Sys5I3UntrustedProcessImageManifest {
    pub fn assigned_loci(&self) -> Vec<String> {
        self.assigned_loci.clone()
    }

    pub const fn has_assigned_artifacts_only(&self) -> bool {
        self.has_assigned_artifacts_only
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn m9_generation_ref(&self) -> &str {
        &self.m9_generation_ref
    }

    pub fn cohort_provenance_ref(&self) -> &str {
        &self.cohort_provenance_ref
    }

    pub const fn carries_source_text(&self) -> bool {
        false
    }

    pub const fn carries_host_path(&self) -> bool {
        false
    }

    pub const fn carries_expected_result(&self) -> bool {
        false
    }
}

/// Codec-decoded carrier candidate.  It contains private bytes-derived facts
/// only and cannot call `accept_inbound` until the target runtime resolves
/// them against its local image.
#[doc(hidden)]
pub struct Sys5I3UntrustedProcessMessage {
    message: PrivateProcessMessageSnapshot,
}

impl std::fmt::Debug for Sys5I3UntrustedProcessMessage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3UntrustedProcessMessage")
            .field("kind", &self.message.kind)
            .finish_non_exhaustive()
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3UntrustedProcessMessageManifest {
    is_request: bool,
    is_reply: bool,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
}

impl Sys5I3UntrustedProcessMessage {
    pub fn observer_safe_manifest(&self) -> Sys5I3UntrustedProcessMessageManifest {
        Sys5I3UntrustedProcessMessageManifest {
            is_request: matches!(self.message.kind, PrivateProcessMessageKind::Request),
            is_reply: matches!(self.message.kind, PrivateProcessMessageKind::Reply),
            semantic_request_identity_ref: self.message.semantic_request_identity_ref.clone(),
            linked_request_identity_ref: self.message.linked_request_identity_ref.clone(),
        }
    }
}

impl Sys5I3UntrustedProcessMessageManifest {
    pub const fn is_request(&self) -> bool {
        self.is_request
    }

    pub const fn is_reply(&self) -> bool {
        self.is_reply
    }

    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn linked_request_identity_ref(&self) -> Option<&str> {
        self.linked_request_identity_ref.as_deref()
    }
}

impl Sys5I3PrivateProcessCodec {
    // The exact restricted M8/M9 closure is intentionally carried rather
    // than recomputed in the child.  Keep a finite private bound large enough
    // for the accepted four-locus profile while still rejecting allocation
    // growth before JSON decoding.
    const MAX_IMAGE_BYTES: usize = 8 << 20;
    const MAX_MESSAGE_BYTES: usize = 1 << 16;

    pub const fn private_provisional_v1() -> Self {
        Self
    }

    /// The provider branch will be the sole FD3 reader for its one complete
    /// private frame. The ordinary inherited-control reader remains unchanged.
    /// Successful decoding produces only an opaque installation input; it
    /// cannot turn image bytes or a generic decoded control into authority.
    #[cfg(unix)]
    pub fn take_inherited_provider_child_control_from_fd3(
        &self,
    ) -> Result<Sys5I3InheritedProviderChildControl, Sys5I3ProviderLaunchError> {
        const INHERITED_PROVIDER_CONTROL_FD: std::os::raw::c_int = 3;
        const F_GETFD: std::os::raw::c_int = 1;

        unsafe extern "C" {
            fn fcntl(fd: std::os::raw::c_int, command: std::os::raw::c_int) -> std::os::raw::c_int;
        }

        if PROVIDER_INHERITED_CONTROL_FD3_TAKEN
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
            ));
        }
        // Check the fixed inherited descriptor before taking ownership. The
        // process runner provides this FD exactly once for the provider argv
        // branch; an absent or malformed FD is a typed bootstrap rejection.
        if unsafe { fcntl(INHERITED_PROVIDER_CONTROL_FD, F_GETFD) } == -1 {
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
            ));
        }
        // SAFETY: the descriptor was just observed valid and this provider
        // reader is the sole owner for the provider child branch.
        let mut stream = unsafe { UnixStream::from_raw_fd(INHERITED_PROVIDER_CONTROL_FD) };
        let snapshot = read_private_provider_child_control_frame(&mut stream)?;
        let expected_start_binding: Sys5I3ExpectedStartBinding =
            snapshot.expected_start_binding.try_into().map_err(|_| {
                Sys5I3ProviderLaunchError::new(
                    Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
                )
            })?;
        let peer_start_binding = snapshot.peer_start_binding;
        let role = Sys5I3ProviderChildRole::from_m9_role(snapshot.role);
        let peer_role = Sys5I3ProviderChildRole::from_m9_role(snapshot.peer_role);
        let envelope_matches_role = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                snapshot.executor_resource_envelope.is_none()
            }
            Sys5I3ProviderChildRole::Executor => snapshot
                .executor_resource_envelope
                .as_ref()
                .is_some_and(|envelope| {
                    envelope.matches_runtime_nonce(&snapshot.expected_resource_runtime_nonce)
                }),
        };
        let fixture_assertion_matches_role = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                snapshot.requester_fixture_assertion.is_some()
            }
            Sys5I3ProviderChildRole::Executor => snapshot.requester_fixture_assertion.is_none(),
        };
        if snapshot.version != PRIVATE_PROVIDER_CHILD_CONTROL_VERSION
            || role == peer_role
            || expected_start_binding.slot_name == peer_start_binding.slot_name
            || expected_start_binding.assigned_loci.is_empty()
            || !peer_start_binding.is_structurally_valid()
            || snapshot.transport_bootstrap.len() > Self::MAX_MESSAGE_BYTES
            || !snapshot
                .m9_role_snapshot
                .matches_provider_child_installation(
                    snapshot.role,
                    &expected_start_binding.parent_checked_program_ref,
                    &snapshot.expected_resource_runtime_nonce,
                    &expected_start_binding.assigned_loci,
                )
            || !envelope_matches_role
            || !fixture_assertion_matches_role
            || (snapshot.terminal_observation_conformance_profile.is_some()
                && snapshot.provider_network_conformance_profile.is_some())
            || {
                #[cfg(not(feature = "i3-process-test-seams"))]
                {
                    snapshot.terminal_observation_conformance_profile.is_some()
                        || snapshot.provider_network_conformance_profile.is_some()
                }
                #[cfg(feature = "i3-process-test-seams")]
                {
                    false
                }
            }
        {
            return Err(Sys5I3ProviderLaunchError::new(
                Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
            ));
        }
        Ok(Sys5I3InheritedProviderChildControl {
            transport_bootstrap: snapshot.transport_bootstrap,
            role,
            expected_start_binding,
            peer_role,
            peer_start_binding,
            m9_role_snapshot: snapshot.m9_role_snapshot,
            expected_resource_runtime_nonce: snapshot.expected_resource_runtime_nonce,
            executor_resource_envelope: snapshot.executor_resource_envelope,
            requester_fixture_assertion: snapshot.requester_fixture_assertion,
            terminal_observation_conformance_profile: snapshot
                .terminal_observation_conformance_profile,
            provider_network_conformance_profile: snapshot.provider_network_conformance_profile,
        })
    }

    /// Install a provider child only from the opaque result of the dedicated
    /// inherited-control reader. Generic decoded controls and ordinary image
    /// start paths remain incapable of reaching this branch.
    pub fn install_provider_from_inherited_control(
        &self,
        image: Sys5I3UntrustedProcessImage,
        control: Sys5I3InheritedProviderChildControl,
        role: Sys5I3ProviderChildRole,
    ) -> Result<Sys5I3InstalledProviderChildRuntime, Sys5I3ProcessRuntimeError> {
        let Sys5I3InheritedProviderChildControl {
            transport_bootstrap: _,
            role: control_role,
            expected_start_binding,
            peer_role,
            peer_start_binding,
            m9_role_snapshot,
            expected_resource_runtime_nonce,
            executor_resource_envelope,
            requester_fixture_assertion,
            terminal_observation_conformance_profile,
            provider_network_conformance_profile,
        } = control;
        #[cfg(not(feature = "i3-process-test-seams"))]
        if terminal_observation_conformance_profile.is_some()
            || provider_network_conformance_profile.is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if terminal_observation_conformance_profile.is_some()
            && provider_network_conformance_profile.is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if role != control_role
            || role == peer_role
            || expected_start_binding.slot_name == peer_start_binding.slot_name
            || expected_start_binding.assigned_loci.is_empty()
            || !peer_start_binding.is_structurally_valid()
            || expected_start_binding
                .validate_inactive_provider_image(&image.image)
                .is_err()
            || !m9_role_snapshot.matches_provider_child_installation(
                role.m9_role(),
                &expected_start_binding.parent_checked_program_ref,
                &expected_resource_runtime_nonce,
                &image.image.assigned_loci,
            )
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let local_authority = m9_role_snapshot
            .install_local_authority(
                role.m9_role(),
                &expected_start_binding.parent_checked_program_ref,
                &expected_resource_runtime_nonce,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let executor_resource_envelope = match (role, executor_resource_envelope) {
            (Sys5I3ProviderChildRole::RequesterConsumer, None) => None,
            (Sys5I3ProviderChildRole::Executor, Some(envelope))
                if envelope.matches_runtime_nonce(&expected_resource_runtime_nonce) =>
            {
                Some(envelope)
            }
            _ => {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
        };
        let requester_fixture_assertion = match (role, requester_fixture_assertion) {
            (Sys5I3ProviderChildRole::RequesterConsumer, Some(assertion)) => Some(assertion),
            (Sys5I3ProviderChildRole::Executor, None) => None,
            _ => {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                ));
            }
        };
        let scoped_component = image
            .image
            .inactive_provider
            .as_ref()
            .map(|provider| provider.component_snapshot.clone())
            .filter(M8I3PrivateProviderComponentSnapshot::is_scoped_component_snapshot)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?
            .restore_for_inherited_provider_install(&expected_resource_runtime_nonce)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let provider = image.image.inactive_provider.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        if scoped_component.component_binding_ref() != provider.component_binding_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let provider_local_fabric =
            Sys4InstalledProviderLocalFabric::from_inherited_provider_parts(
                provider.static_snapshot.clone(),
                &scoped_component,
                provider.legacy_authority_snapshot.clone(),
                image.image.assigned_loci.clone(),
                &provider.component_binding_ref,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        if provider_local_fabric
            .checked_program_identity()
            .stable_key()
            != expected_start_binding.parent_checked_program_ref
            || provider_local_fabric.assigned_loci() != &image.image.assigned_loci
            || provider_local_fabric.component_binding_ref() != provider.component_binding_ref
            || !provider_local_fabric.has_exact_legacy_authority_generation()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        let provider_execution = Sys5I3ProviderExecution::from_installed_profile(
            &local_authority,
            &provider_local_fabric,
        )?;
        Ok(Sys5I3InstalledProviderChildRuntime {
            role,
            expected_start_binding,
            peer_role,
            peer_start_binding,
            local_authority,
            _scoped_component: scoped_component,
            provider_local_fabric,
            executor_resource_envelope,
            requester_fixture_assertion,
            fixture_post_consume_assertion_completed: false,
            provider_execution,
            terminal_observation_export_reserved: false,
            #[cfg(feature = "i3-process-test-seams")]
            terminal_observation_conformance_profile,
            #[cfg(feature = "i3-process-test-seams")]
            terminal_observation_conformance_completed: false,
            #[cfg(feature = "i3-process-test-seams")]
            provider_network_conformance_profile,
            #[cfg(feature = "i3-process-test-seams")]
            provider_network_conformance_facts: Default::default(),
            #[cfg(feature = "i3-process-test-seams")]
            provider_network_conformance_completed: false,
        })
    }

    /// Turns two coordinator-held bindings into exactly two dedicated trusted
    /// controls.  The pair consumes both bindings, so the caller cannot reuse
    /// an image bootstrap after handing it to a child.  `run_ref` and SPKI
    /// references are delivery commitments only; M9 remains inside the sealed
    /// restricted admission restored from the checked image.
    pub fn split_trusted_localnet_controls(
        &self,
        run_ref: impl Into<String>,
        first: Sys5I3ExpectedStartBinding,
        first_spki_ref: impl Into<String>,
        second: Sys5I3ExpectedStartBinding,
        second_spki_ref: impl Into<String>,
    ) -> Result<
        (Sys5I3TrustedLocalnetControl, Sys5I3TrustedLocalnetControl),
        Sys5I3LocalnetControlError,
    > {
        let run_ref = run_ref.into();
        let first_spki_ref = first_spki_ref.into();
        let second_spki_ref = second_spki_ref.into();
        if run_ref.is_empty()
            || first_spki_ref.is_empty()
            || second_spki_ref.is_empty()
            || first.slot_name.is_empty()
            || second.slot_name.is_empty()
            || first.slot_name == second.slot_name
        {
            return Err(Sys5I3LocalnetControlError::new(
                Sys5I3LocalnetControlErrorKind::MalformedControl,
            ));
        }
        let first_control = Sys5I3TrustedLocalnetControl {
            run_ref: run_ref.clone(),
            local_slot_name: first.slot_name.clone(),
            peer_slot_name: second.slot_name.clone(),
            local_spki_ref: first_spki_ref.clone(),
            peer_spki_ref: second_spki_ref.clone(),
            peer_image_integrity_ref: second.image_integrity_ref.clone(),
            peer_checked_program_ref: second.parent_checked_program_ref.clone(),
            peer_projection_ref: second.projection_ref.clone(),
            peer_cohort_provenance_ref: second.cohort_provenance_ref.clone(),
            expected_start_binding: first,
        };
        let second_control = Sys5I3TrustedLocalnetControl {
            run_ref,
            local_slot_name: second.slot_name.clone(),
            peer_slot_name: first_control.local_slot_name.clone(),
            local_spki_ref: second_spki_ref,
            peer_spki_ref: first_spki_ref,
            peer_image_integrity_ref: first_control
                .expected_start_binding
                .image_integrity_ref
                .clone(),
            peer_checked_program_ref: first_control
                .expected_start_binding
                .parent_checked_program_ref
                .clone(),
            peer_projection_ref: first_control.expected_start_binding.projection_ref.clone(),
            peer_cohort_provenance_ref: first_control
                .expected_start_binding
                .cohort_provenance_ref
                .clone(),
            expected_start_binding: second,
        };
        Ok((first_control, second_control))
    }

    /// Serializes a coordinator-issued trusted control for its dedicated
    /// inherited control descriptor.  This private frame must never be
    /// multiplexed with, substituted for, or accepted from the tainted image
    /// byte descriptor.
    pub fn encode_trusted_localnet_control(
        &self,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let snapshot = PrivateTrustedLocalnetControlSnapshot {
            version: PRIVATE_TRUSTED_LOCALNET_CONTROL_VERSION,
            expected_start_binding: (&control.expected_start_binding).into(),
            run_ref: control.run_ref,
            local_slot_name: control.local_slot_name,
            peer_slot_name: control.peer_slot_name,
            local_spki_ref: control.local_spki_ref,
            peer_spki_ref: control.peer_spki_ref,
            peer_image_integrity_ref: control.peer_image_integrity_ref,
            peer_checked_program_ref: control.peer_checked_program_ref,
            peer_projection_ref: control.peer_projection_ref,
            peer_cohort_provenance_ref: control.peer_cohort_provenance_ref,
        };
        let body = serde_json::to_vec(&snapshot).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// Decodes only the private trusted-control record.  Successful decode is
    /// not process start: `validate_and_start_image_with_localnet_control`
    /// must still compare the separately received tainted image.
    pub fn decode_trusted_localnet_control(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3TrustedLocalnetControl, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let snapshot: PrivateTrustedLocalnetControlSnapshot = serde_json::from_value(value)
            .map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if snapshot.version != PRIVATE_TRUSTED_LOCALNET_CONTROL_VERSION
            || snapshot.run_ref.is_empty()
            || snapshot.local_slot_name.is_empty()
            || snapshot.peer_slot_name.is_empty()
            || snapshot.local_slot_name == snapshot.peer_slot_name
            || snapshot.local_spki_ref.is_empty()
            || snapshot.peer_spki_ref.is_empty()
            || snapshot.peer_image_integrity_ref.is_empty()
            || snapshot.peer_checked_program_ref.is_empty()
            || snapshot.peer_projection_ref.is_empty()
            || snapshot.peer_cohort_provenance_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        let expected_start_binding: Sys5I3ExpectedStartBinding =
            snapshot.expected_start_binding.try_into().map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if expected_start_binding.slot_name != snapshot.local_slot_name {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(Sys5I3TrustedLocalnetControl {
            expected_start_binding,
            run_ref: snapshot.run_ref,
            local_slot_name: snapshot.local_slot_name,
            peer_slot_name: snapshot.peer_slot_name,
            local_spki_ref: snapshot.local_spki_ref,
            peer_spki_ref: snapshot.peer_spki_ref,
            peer_image_integrity_ref: snapshot.peer_image_integrity_ref,
            peer_checked_program_ref: snapshot.peer_checked_program_ref,
            peer_projection_ref: snapshot.peer_projection_ref,
            peer_cohort_provenance_ref: snapshot.peer_cohort_provenance_ref,
        })
    }

    /// Decode a private ACK-shaped record only as tainted input.  There is no
    /// codec API that turns this value, its bytes, or its fields into an
    /// installed receipt or parent publication.
    pub fn decode_owner_lifecycle_ack(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3TaintedOwnerLifecycleAck, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let snapshot: PrivateOwnerLifecycleAckSnapshot =
            serde_json::from_value(value).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if snapshot.version != PRIVATE_OWNER_LIFECYCLE_ACK_VERSION
            || snapshot.stage_identity_binding_ref.is_empty()
            || snapshot.prior_generation_ref.is_empty()
            || snapshot.successor_generation_ref.is_empty()
            || snapshot.candidate_binding_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(Sys5I3TaintedOwnerLifecycleAck {
            stage_identity_binding_ref: snapshot.stage_identity_binding_ref,
            prior_generation_ref: snapshot.prior_generation_ref,
            successor_generation_ref: snapshot.successor_generation_ref,
            candidate_binding_ref: snapshot.candidate_binding_ref,
        })
    }

    /// Serialize the one child ACK only by consuming a post-install receipt.
    /// This does not publish anything: the parent still requires its owned,
    /// registered B output reader to obtain and validate this frame.
    pub fn encode_installed_owner_lifecycle_ack(
        &self,
        ack: Sys5I3OwnerLifecycleInstallAck,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let Sys5I3OwnerLifecycleInstallAck {
            stage_identity_binding_ref,
            receipt,
        } = ack;
        let body = serde_json::to_vec(&PrivateOwnerLifecycleAckSnapshot {
            version: PRIVATE_OWNER_LIFECYCLE_ACK_VERSION,
            stage_identity_binding_ref,
            prior_generation_ref: receipt.prior_generation_ref().to_string(),
            successor_generation_ref: receipt.successor_generation_ref().to_string(),
            candidate_binding_ref: receipt.candidate_binding_ref().to_string(),
        })
        .map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn encode_tainted_owner_lifecycle_ack_candidate(
        &self,
        stage_identity_binding_ref: &str,
        prior_generation_ref: &str,
        successor_generation_ref: &str,
        candidate_binding_ref: &str,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        if stage_identity_binding_ref.is_empty()
            || prior_generation_ref.is_empty()
            || successor_generation_ref.is_empty()
            || candidate_binding_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        let body = serde_json::to_vec(&PrivateOwnerLifecycleAckSnapshot {
            version: PRIVATE_OWNER_LIFECYCLE_ACK_VERSION,
            stage_identity_binding_ref: stage_identity_binding_ref.to_string(),
            prior_generation_ref: prior_generation_ref.to_string(),
            successor_generation_ref: successor_generation_ref.to_string(),
            candidate_binding_ref: candidate_binding_ref.to_string(),
        })
        .map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// The child-only promotion path: decoded image bytes and a separately
    /// delivered control must agree before a local fabric can start.  It also
    /// returns the sole gate able to turn a completed, peer-bound delivery
    /// into semantic carrier admission.
    pub fn validate_and_start_image_with_localnet_control(
        &self,
        image: Sys5I3UntrustedProcessImage,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<(Sys5I3ProcessRuntime, Sys5I3TrustedLocalnetControl), Sys5I3LocalnetControlError>
    {
        control
            .expected_start_binding
            .validate_image(&image.image)
            .map_err(|_| {
                Sys5I3LocalnetControlError::new(
                    Sys5I3LocalnetControlErrorKind::StartBindingRejected,
                )
            })?;
        let runtime = Sys5I3ProcessRuntime::start(image.image).map_err(|_| {
            Sys5I3LocalnetControlError::new(Sys5I3LocalnetControlErrorKind::StartBindingRejected)
        })?;
        Ok((runtime, control))
    }

    /// The only child bootstrap path that may transfer a pre-staged owner
    /// lifecycle candidate.  The image remains tainted until it matches its
    /// separately trusted binding; only the selected B control can then
    /// receive the opaque, non-cloneable install stimulus.  Neither child
    /// receives an M9 publisher.
    pub fn validate_and_start_image_with_prestaged_lifecycle(
        &self,
        mut image: Sys5I3UntrustedProcessImage,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<
        (
            Sys5I3ProcessRuntime,
            Sys5I3TrustedLocalnetControl,
            Option<Sys5I3AdmittedLifecycleStimulus>,
        ),
        Sys5I3LocalnetControlError,
    > {
        control
            .expected_start_binding
            .validate_image(&image.image)
            .map_err(|_| {
                Sys5I3LocalnetControlError::new(
                    Sys5I3LocalnetControlErrorKind::StartBindingRejected,
                )
            })?;
        let image_lifecycle = match &mut image.image.private_runtime_seed {
            Sys5I3PrivateRuntimeSeed::Ordinary(seed) => {
                seed.prestaged_owner_capability_lifecycle.take()
            }
            Sys5I3PrivateRuntimeSeed::InactiveProvider(_) => {
                return Err(Sys5I3LocalnetControlError::new(
                    Sys5I3LocalnetControlErrorKind::StartBindingRejected,
                ));
            }
        };
        let stimulus = match (
            image_lifecycle,
            control
                .expected_start_binding
                .expected_owner_capability_lifecycle
                .as_ref(),
        ) {
            (None, None) => None,
            (Some(lifecycle), Some(expected))
                if lifecycle.target_slot_name == control.local_slot_name
                    && lifecycle.target_slot_name == expected.target_slot_name
                    && lifecycle.stage_identity_binding_ref
                        == expected.stage_identity_binding_ref
                    && lifecycle.candidate.prior_generation_ref()
                        == expected.prior_generation_ref
                    && lifecycle.candidate.successor_generation_ref()
                        == expected.successor_generation_ref
                    && lifecycle.candidate.candidate_binding_ref()
                        == expected.candidate_binding_ref =>
            {
                Some(Sys5I3AdmittedLifecycleStimulus {
                    stage_identity_binding_ref: lifecycle.stage_identity_binding_ref,
                    candidate: lifecycle.candidate,
                })
            }
            _ => {
                return Err(Sys5I3LocalnetControlError::new(
                    Sys5I3LocalnetControlErrorKind::StartBindingRejected,
                ));
            }
        };
        // The candidate moved into the opaque B-local stimulus before fabric
        // bootstrap.  Re-seal the now publisher-free child image so ordinary
        // start validation cannot retain a second copy in its private seed.
        image.image.refresh_private_integrity();
        let runtime = Sys5I3ProcessRuntime::start(image.image).map_err(|_| {
            Sys5I3LocalnetControlError::new(Sys5I3LocalnetControlErrorKind::StartBindingRejected)
        })?;
        Ok((runtime, control, stimulus))
    }

    pub const fn limits(&self) -> Sys5I3PrivateProcessCodecLimits {
        Sys5I3PrivateProcessCodecLimits {
            max_image_bytes: Self::MAX_IMAGE_BYTES,
            max_message_bytes: Self::MAX_MESSAGE_BYTES,
        }
    }

    /// Consume the only startable image.  The returned bytes are untrusted
    /// delivery material; no image value remains available to start directly.
    pub fn encode_image(
        &self,
        image: Sys5I3ProcessImage,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let snapshot =
            process_snapshot::PrivateProcessImageSnapshot::from_image(image).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        let body = serde_json::to_vec(&snapshot).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_IMAGE_BYTES)
    }

    pub fn decode_untrusted_image(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3UntrustedProcessImage, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_IMAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.validate_image_json_shape(&value)?;
        let snapshot: process_snapshot::PrivateProcessImageSnapshot = serde_json::from_value(value)
            .map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        let image = snapshot.into_untrusted_image().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        Ok(Sys5I3UntrustedProcessImage { image })
    }

    pub fn validate_and_start_image(
        &self,
        candidate: Sys5I3UntrustedProcessImage,
        expected: Sys5I3ExpectedStartBinding,
    ) -> Result<Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeError> {
        expected.validate_image(&candidate.image)?;
        Sys5I3ProcessRuntime::start(candidate.image)
    }

    pub fn encode_outbound_message(
        &self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let kind = match message.kind {
            Sys5I3ProcessMessageKind::Request => PrivateProcessMessageKind::Request,
            Sys5I3ProcessMessageKind::Reply => PrivateProcessMessageKind::Reply,
            Sys5I3ProcessMessageKind::Receipt => {
                return Err(Sys5I3PrivateProcessCodecError::new(
                    Sys5I3PrivateProcessCodecErrorKind::ReceiptIsLocalOnly,
                ));
            }
            Sys5I3ProcessMessageKind::TerminalFailureConsumed => {
                return Err(Sys5I3PrivateProcessCodecError::new(
                    Sys5I3PrivateProcessCodecErrorKind::TerminalFailureIsLocalOnly,
                ));
            }
        };
        let carrier = message.carrier.as_ref().ok_or_else(|| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let carrier = carrier.i3_private_process_snapshot().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let envelope = PrivateProcessMessageEnvelope {
            version: process_snapshot::PRIVATE_PROCESS_SNAPSHOT_VERSION,
            message: PrivateProcessMessageSnapshot {
                kind,
                carrier,
                semantic_request_identity_ref: message.semantic_request_identity_ref,
                linked_request_identity_ref: message.linked_request_identity_ref,
                cohort_provenance_ref: message.cohort_provenance_ref,
            },
        };
        let body = serde_json::to_vec(&envelope).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// Encode one distinct provider request for the private QUIC adapter.
    /// This is crate-private: external callers cannot turn an arbitrary JSON
    /// record into a carrier accepted by an installed child.
    pub(crate) fn encode_provider_request(
        &self,
        request: &Sys5I3ProviderRequestCarrier,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let envelope = PrivateProviderCarrierEnvelope {
            version: 1,
            carrier: PrivateProviderCarrierPayload::Request(request.private_snapshot()),
        };
        let body = serde_json::to_vec(&envelope).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// Decode one tainted provider request. The result is usable only by the
    /// private QUIC receiver, whose installed role performs its own exact
    /// static/M9 admission before any host crossing.
    pub(crate) fn decode_untrusted_provider_request(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3ProviderRequestCarrier, Sys5I3PrivateProcessCodecError> {
        let envelope = self.decode_provider_carrier_envelope(bytes)?;
        let PrivateProviderCarrierPayload::Request(snapshot) = envelope.carrier else {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        };
        Sys5I3ProviderRequestCarrier::from_private_snapshot(snapshot).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })
    }

    /// Encode one distinct provider result for the private QUIC adapter.
    pub(crate) fn encode_provider_result(
        &self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let envelope = PrivateProviderCarrierEnvelope {
            version: 1,
            carrier: PrivateProviderCarrierPayload::Result(result.private_snapshot()),
        };
        let body = serde_json::to_vec(&envelope).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// Decode one tainted provider result. It carries no receipt and cannot
    /// consume a requester pending entry until the installed requester checks
    /// its actual current M9 role state.
    pub(crate) fn decode_untrusted_provider_result(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3PrivateProcessCodecError> {
        let envelope = self.decode_provider_carrier_envelope(bytes)?;
        let PrivateProviderCarrierPayload::Result(snapshot) = envelope.carrier else {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        };
        Sys5I3ProviderResultCarrier::from_private_snapshot(snapshot).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })
    }

    /// Encode the one M9-authorized, reference-only terminal audit emitted by
    /// an installed provider child. The audit value can arise only from the
    /// runtime's current M9 preflight; this codec does not issue a permit.
    #[doc(hidden)]
    pub fn encode_provider_terminal_audit_observer_view(
        &self,
        audit: &Sys5I3ProviderTerminalAudit,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let body = audit.encode_observer_view_body().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Oversized)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    /// Decode only a bounded, strict, reference-only child-output candidate.
    /// This cannot be used to issue M9 authority, recreate a terminal audit,
    /// or enter a provider runtime. The supervising probe must separately
    /// correlate it with its owned run, role, source facts, request joins, and
    /// physical child completion before accepting it as evidence.
    #[doc(hidden)]
    pub fn decode_untrusted_provider_terminal_audit_observer_view(
        &self,
        bytes: &[u8],
    ) -> Result<
        Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
        Sys5I3PrivateProcessCodecError,
    > {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        provider_runtime::decode_untrusted_provider_terminal_audit_observer_view_body(body)
            .map_err(|error| {
                let kind = match error {
                    provider_runtime::PrivateProviderTerminalAuditObserverViewDecodeError::Malformed => {
                        Sys5I3PrivateProcessCodecErrorKind::Malformed
                    }
                    provider_runtime::PrivateProviderTerminalAuditObserverViewDecodeError::Oversized => {
                        Sys5I3PrivateProcessCodecErrorKind::Oversized
                    }
                    provider_runtime::PrivateProviderTerminalAuditObserverViewDecodeError::UnknownVersion => {
                        Sys5I3PrivateProcessCodecErrorKind::UnknownVersion
                    }
                };
                Sys5I3PrivateProcessCodecError::new(kind)
            })
    }

    /// Encode the runtime-retained exact original request for one
    /// adapter-authorized attempt.  This is crate-private so ordinary callers
    /// cannot turn an opaque pending handle into cloneable carrier bytes.
    pub(crate) fn encode_retained_owner_request_attempt(
        &self,
        carrier: &Sys4ProcessCarrier,
        semantic_request_identity_ref: &str,
        cohort_provenance_ref: &str,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        if carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest
            || semantic_request_identity_ref.is_empty()
            || cohort_provenance_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        let carrier = carrier.i3_private_process_snapshot().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let envelope = PrivateProcessMessageEnvelope {
            version: process_snapshot::PRIVATE_PROCESS_SNAPSHOT_VERSION,
            message: PrivateProcessMessageSnapshot {
                kind: PrivateProcessMessageKind::Request,
                carrier,
                semantic_request_identity_ref: semantic_request_identity_ref.to_string(),
                linked_request_identity_ref: None,
                cohort_provenance_ref: cohort_provenance_ref.to_string(),
            },
        };
        let body = serde_json::to_vec(&envelope).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    pub fn decode_untrusted_message(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3UntrustedProcessMessage, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.validate_version(&value)?;
        let envelope: PrivateProcessMessageEnvelope =
            serde_json::from_value(value).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if envelope.message.semantic_request_identity_ref.is_empty()
            || envelope.message.cohort_provenance_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(Sys5I3UntrustedProcessMessage {
            message: envelope.message,
        })
    }

    fn frame_body(
        &self,
        body: Vec<u8>,
        limit: usize,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let total = body.len().checked_add(4).ok_or_else(|| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Oversized)
        })?;
        if total > limit || body.len() > u32::MAX as usize {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Oversized,
            ));
        }
        let mut framed = (body.len() as u32).to_be_bytes().to_vec();
        framed.extend(body);
        Ok(framed)
    }

    fn decode_provider_carrier_envelope(
        &self,
        bytes: &[u8],
    ) -> Result<PrivateProviderCarrierEnvelope, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let envelope: PrivateProviderCarrierEnvelope =
            serde_json::from_value(value).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if envelope.version != 1 {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::UnknownVersion,
            ));
        }
        Ok(envelope)
    }

    fn unframe_body<'a>(
        &self,
        bytes: &'a [u8],
        limit: usize,
    ) -> Result<&'a [u8], Sys5I3PrivateProcessCodecError> {
        if bytes.len() > limit {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Oversized,
            ));
        }
        if bytes.len() < 4 {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Incomplete,
            ));
        }
        let declared = u32::from_be_bytes(bytes[..4].try_into().expect("fixed prefix")) as usize;
        let available = bytes.len() - 4;
        if declared > limit.saturating_sub(4) {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Oversized,
            ));
        }
        if available < declared {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Incomplete,
            ));
        }
        if available != declared {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(&bytes[4..])
    }

    fn validate_version(
        &self,
        value: &serde_json::Value,
    ) -> Result<(), Sys5I3PrivateProcessCodecError> {
        let version = value
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if version != process_snapshot::PRIVATE_PROCESS_SNAPSHOT_VERSION {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::UnknownVersion,
            ));
        }
        Ok(())
    }

    fn validate_image_json_shape(
        &self,
        value: &serde_json::Value,
    ) -> Result<(), Sys5I3PrivateProcessCodecError> {
        self.validate_version(value)?;
        let Some(edges) = value
            .pointer("/image/required_edge_contracts")
            .and_then(serde_json::Value::as_array)
        else {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        };
        if edges.iter().any(|edge| {
            edge.get("core_ref")
                .and_then(serde_json::Value::as_str)
                .is_none_or(str::is_empty)
        }) {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::MissingRequiredCoreProvenance,
            ));
        }
        Ok(())
    }
}

/// The explicit semantic identity basis for local images and generated
/// carrier lineage.  Process IDs, addresses, sessions, and future transport
/// attempts deliberately do not participate.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeIdentityBasis;

impl Sys5I3ObserverSafeIdentityBasis {
    pub const fn includes_checked_program_ref(&self) -> bool {
        true
    }
    pub const fn includes_projection_ref(&self) -> bool {
        true
    }
    pub const fn includes_cohort_ref(&self) -> bool {
        true
    }
    pub const fn includes_logical_origin_ref(&self) -> bool {
        true
    }
    pub const fn includes_ordinal(&self) -> bool {
        true
    }
    pub const fn includes_process_id(&self) -> bool {
        false
    }
    pub const fn includes_network_identity(&self) -> bool {
        false
    }
}

/// Semantic request identity has a narrower basis than the process-local
/// store identity.  In particular, a v3 request is derived from the checked
/// carrier/request occurrence, not from the local store's logical origin or
/// ordinal.  Keeping the two observer claims separate prevents a runtime
/// bookkeeping reference from being reported as semantic provenance.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeSemanticRequestIdentityBasis;

impl Sys5I3ObserverSafeSemanticRequestIdentityBasis {
    pub const fn includes_checked_program_ref(&self) -> bool {
        true
    }

    pub const fn includes_projection_ref(&self) -> bool {
        true
    }

    pub const fn includes_cohort_ref(&self) -> bool {
        true
    }

    pub const fn includes_logical_origin_ref(&self) -> bool {
        false
    }

    pub const fn includes_ordinal(&self) -> bool {
        false
    }

    pub const fn includes_process_id(&self) -> bool {
        false
    }

    pub const fn includes_network_identity(&self) -> bool {
        false
    }
}

/// Observer-safe pending-carrier facts for the pre-transport runtime.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeOutboxSummary {
    pending_carrier_count: usize,
    generated_owner_operations: BTreeSet<String>,
}

impl Sys5I3ObserverSafeOutboxSummary {
    pub const fn pending_carrier_count(&self) -> usize {
        self.pending_carrier_count
    }

    pub fn contains_generated_owner_request(&self, operation_id: &str) -> bool {
        self.generated_owner_operations.contains(operation_id)
    }
}

/// Observer-safe proof that a started child retains no coordinator-only
/// authority publisher, full admission, or global FabricProgram.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeRuntimeSummary {
    served_owner_request_count: usize,
    actual_owner_write_count: usize,
    accepted_inbound_receipt_count: usize,
    accepted_inbound_declared_owner_failure_count: usize,
}

/// Exact, observer-safe semantic occurrence references retained by the
/// process runtime.  These are recorded from accepted SYS-4 steps; they are
/// deliberately not inferred from the aggregate counters above.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sys5I3ObserverSafeSemanticOccurrences {
    owner_serve_linearizations: BTreeMap<String, String>,
    actual_owner_writes: BTreeMap<String, String>,
    requester_local_receipts: BTreeMap<String, String>,
    requester_terminal_declared_owner_failures: BTreeMap<String, String>,
}

impl Sys5I3ObserverSafeSemanticOccurrences {
    pub fn owner_serve_linearization_count(&self) -> usize {
        self.owner_serve_linearizations.len()
    }

    pub fn actual_owner_write_count(&self) -> usize {
        self.actual_owner_writes.len()
    }

    pub fn requester_local_receipt_count(&self) -> usize {
        self.requester_local_receipts.len()
    }

    pub fn owner_serve_linearization_occurrence_ref(
        &self,
        request_identity_ref: &str,
    ) -> Option<&str> {
        self.owner_serve_linearizations
            .get(request_identity_ref)
            .map(String::as_str)
    }

    pub fn actual_owner_write_occurrence_ref(&self, request_identity_ref: &str) -> Option<&str> {
        self.actual_owner_writes
            .get(request_identity_ref)
            .map(String::as_str)
    }

    pub fn requester_local_receipt_occurrence_ref(
        &self,
        request_identity_ref: &str,
    ) -> Option<&str> {
        self.requester_local_receipts
            .get(request_identity_ref)
            .map(String::as_str)
    }

    pub fn requester_terminal_declared_owner_failure_count(&self) -> usize {
        self.requester_terminal_declared_owner_failures.len()
    }

    pub fn requester_terminal_declared_owner_failure_occurrence_ref(
        &self,
        request_identity_ref: &str,
    ) -> Option<&str> {
        self.requester_terminal_declared_owner_failures
            .get(request_identity_ref)
            .map(String::as_str)
    }
}

impl Sys5I3ObserverSafeRuntimeSummary {
    pub const fn carries_authority_publisher_or_issuer(&self) -> bool {
        false
    }

    pub const fn carries_full_admission_or_fabric_program(&self) -> bool {
        false
    }

    pub const fn served_owner_request_count(&self) -> usize {
        self.served_owner_request_count
    }

    pub const fn actual_owner_write_count(&self) -> usize {
        self.actual_owner_write_count
    }

    pub const fn accepted_inbound_receipt_count(&self) -> usize {
        self.accepted_inbound_receipt_count
    }

    pub const fn accepted_inbound_declared_owner_failure_count(&self) -> usize {
        self.accepted_inbound_declared_owner_failure_count
    }
}

/// Started process-local runtime.  It owns only selected loci and an
/// independent local fabric/store; it has no access to another image's
/// stores, artifacts, publisher, or full projection.
#[cfg_attr(
    not(feature = "i3-process-test-seams"),
    doc = r#"
```compile_fail
use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessRuntime;

// Normal safe Rust cannot invoke decoded semantic admission without the
// adapter-owned live QUIC connection and bidi stream.
let _ = Sys5I3ProcessRuntime::admit_decoded_process_message;
```
"#
)]
#[doc(hidden)]
pub struct Sys5I3ProcessRuntime {
    assigned_loci: BTreeSet<String>,
    local_store_identity_ref: String,
    identity_basis: Sys5I3ObserverSafeIdentityBasis,
    parent_checked_program_ref: String,
    projection_ref: String,
    cohort_ref: String,
    fabric: LocalFabric,
    // This is copied only from the live SYS-4 gate.  It distinguishes two
    // simultaneous starts from the same image whose stable store refs match.
    owner_admission_runtime_instance: u64,
    local_authoritative_mutation_count: usize,
    served_owner_request_count: usize,
    accepted_inbound_receipt_count: usize,
    accepted_inbound_declared_owner_failure_count: usize,
    semantic_occurrences: Sys5I3ObserverSafeSemanticOccurrences,
    // A requester-local claim for one emitted owner request.  It contains
    // only receiver-owned, source-derived route/provenance facts; it is not
    // a transport session, credential, or mutable remote-store handle.
    pending_outbound_owner_requests: BTreeMap<String, Sys5I3OutboundOwnerRequestRecord>,
    requester_terminal_declared_owner_failures:
        BTreeMap<String, Sys5I3RequesterTerminalDeclaredOwnerFailure>,
    // Bounded owner-local replay tombstones.  A key is retained after the
    // first reservation for this runtime lifetime; no reconnect or retry
    // path may clear it.
    inbound_owner_request_tombstones: BTreeMap<String, Sys5I3InboundOwnerRequestRecord>,
    // Per checked owner/domain time coordinates.  These are not transport or
    // requester clocks and cannot issue a permit by themselves.
    owner_admission_clocks: BTreeMap<Sys5I3OwnerAdmissionClockKey, Sys5I3OwnerAdmissionClockState>,
    installed_owner_capability_lifecycle: Option<Sys5I3ObserverSafeInstalledLifecycle>,
    #[cfg(feature = "i3-process-test-seams")]
    reject_next_outbound_extraction: bool,
    #[cfg(feature = "i3-process-test-seams")]
    reject_next_owner_admission_after_reservation: bool,
}

impl std::fmt::Debug for Sys5I3ProcessRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3ProcessRuntime")
            .field("assigned_loci", &self.assigned_loci)
            .field("local_store_identity_ref", &self.local_store_identity_ref)
            .field(
                "local_authoritative_mutation_count",
                &self.local_authoritative_mutation_count,
            )
            .field(
                "served_owner_request_count",
                &self.served_owner_request_count,
            )
            .finish_non_exhaustive()
    }
}

impl Sys5I3ProcessRuntime {
    /// Starts a trusted, already-typed G1 process image held by the local
    /// coordinator.  This is not a decoded-child bootstrap path: delivered
    /// image bytes must use `validate_and_start_image_with_localnet_control`,
    /// which binds the untrusted image to its separately retained control.
    pub fn start(image: Sys5I3ProcessImage) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if image.inactive_provider.is_some()
            || image.private_runtime_seed.inactive_provider().is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        }
        if image
            .private_runtime_seed
            .ordinary()
            .is_some_and(|seed| seed.prestaged_owner_capability_lifecycle.is_some())
        {
            // A staged image must move its candidate only through the
            // independently bound bootstrap path above.  Direct start cannot
            // silently discard, duplicate, or self-authorize that candidate.
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::LifecycleCandidateRejected,
            ));
        }
        image.validate_before_start()?;
        let parent_checked_program_ref = image.child_seed.parent_checked_program_ref.clone();
        let projection_ref = image.child_seed.projection_ref.clone();
        let cohort_ref = image
            .child_seed
            .required_local_authority_closure
            .opaque_cohort_ref()
            .to_string();
        let logical_origin_ref = logical_origin_ref(
            &image.slot_name,
            &image.assigned_loci,
            &parent_checked_program_ref,
            &projection_ref,
            &cohort_ref,
        );
        let local_store_identity_ref = process_store_identity_ref(
            &parent_checked_program_ref,
            &projection_ref,
            &cohort_ref,
            &logical_origin_ref,
            0,
        );
        let assigned_loci = image.assigned_loci.clone();
        let Sys5I3PrivateRuntimeSeed::Ordinary(seed) = image.private_runtime_seed else {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
            ));
        };
        let mut fabric = LocalFabric::bootstrap(seed.program, seed.admission, BackendProfile::St)
            .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        let owner_admission_runtime_instance = fabric
            .install_i3_owner_admission_gate(&local_store_identity_ref)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let owner_admission_clocks = fabric
            .i3_checked_owner_admission_conditions()
            .into_iter()
            .filter(|condition: &OwnerAdmissionBudgetCondition| {
                assigned_loci.contains(condition.owner_locus().as_str())
            })
            .map(|condition: OwnerAdmissionBudgetCondition| {
                (
                    Sys5I3OwnerAdmissionClockKey {
                        owner_locus: condition.owner_locus().as_str().to_string(),
                        clock_domain: condition.clock_domain().as_str().to_string(),
                    },
                    Sys5I3OwnerAdmissionClockState { tick: 0 },
                )
            })
            .collect::<BTreeMap<_, _>>();
        Ok(Self {
            assigned_loci,
            local_store_identity_ref,
            identity_basis: Sys5I3ObserverSafeIdentityBasis,
            parent_checked_program_ref,
            projection_ref,
            cohort_ref,
            fabric,
            owner_admission_runtime_instance,
            local_authoritative_mutation_count: 0,
            served_owner_request_count: 0,
            accepted_inbound_receipt_count: 0,
            accepted_inbound_declared_owner_failure_count: 0,
            semantic_occurrences: Sys5I3ObserverSafeSemanticOccurrences::default(),
            pending_outbound_owner_requests: BTreeMap::new(),
            requester_terminal_declared_owner_failures: BTreeMap::new(),
            inbound_owner_request_tombstones: BTreeMap::new(),
            owner_admission_clocks,
            installed_owner_capability_lifecycle: None,
            #[cfg(feature = "i3-process-test-seams")]
            reject_next_outbound_extraction: false,
            #[cfg(feature = "i3-process-test-seams")]
            reject_next_owner_admission_after_reservation: false,
        })
    }

    /// Consume the B-local stimulus only after bootstrap.  The SYS-4 helper
    /// rechecks the exact M9 prior, restricted M8 delta, and local live floor
    /// before replacing the B fabric.  Failure leaves this G1 runtime and all
    /// semantic counters untouched; success yields an opaque receipt for the
    /// separately registered child-output ACK path, not a publisher.
    pub fn install_admitted_owner_capability_successor(
        &mut self,
        stimulus: Sys5I3AdmittedLifecycleStimulus,
    ) -> Result<Sys5I3OwnerLifecycleInstallAck, Sys5I3ProcessRuntimeError> {
        let Sys5I3AdmittedLifecycleStimulus {
            stage_identity_binding_ref,
            candidate,
        } = stimulus;
        let receipt = self
            .fabric
            .install_i3_restricted_owner_capability_successor(candidate)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::LifecycleInstallRejected,
                )
            })?;
        let successor_generation_ref = receipt.successor_generation_ref().to_string();
        self.installed_owner_capability_lifecycle = Some(Sys5I3ObserverSafeInstalledLifecycle {
            successor_generation_ref,
            origin: Sys5I3ObserverSafeLifecycleOrigin::M9AdmittedLifecycle,
            source_derived: false,
        });
        Ok(Sys5I3OwnerLifecycleInstallAck {
            stage_identity_binding_ref,
            receipt,
        })
    }

    pub fn local_store_identity_ref(&self) -> &str {
        &self.local_store_identity_ref
    }

    pub const fn observer_safe_runtime_summary(&self) -> Sys5I3ObserverSafeRuntimeSummary {
        Sys5I3ObserverSafeRuntimeSummary {
            served_owner_request_count: self.served_owner_request_count,
            actual_owner_write_count: self.local_authoritative_mutation_count,
            accepted_inbound_receipt_count: self.accepted_inbound_receipt_count,
            accepted_inbound_declared_owner_failure_count: self
                .accepted_inbound_declared_owner_failure_count,
        }
    }

    /// Installed lifecycle evidence exists only after B has committed its
    /// local M9 live floor.  A failed or unstarted candidate yields `None`.
    pub fn observer_safe_installed_owner_capability_lifecycle(
        &self,
    ) -> Option<&Sys5I3ObserverSafeInstalledLifecycle> {
        self.installed_owner_capability_lifecycle.as_ref()
    }

    pub const fn local_authoritative_mutation_count(&self) -> usize {
        self.local_authoritative_mutation_count
    }

    pub const fn observer_safe_store_identity_basis(&self) -> Sys5I3ObserverSafeIdentityBasis {
        self.identity_basis
    }

    pub fn observer_safe_semantic_occurrences(&self) -> Sys5I3ObserverSafeSemanticOccurrences {
        self.semantic_occurrences.clone()
    }

    pub fn observer_safe_outbox_summary(&self) -> Sys5I3ObserverSafeOutboxSummary {
        let summary = self.fabric.i3_process_outbox_summary();
        Sys5I3ObserverSafeOutboxSummary {
            pending_carrier_count: summary.pending_carrier_count(),
            generated_owner_operations: summary.generated_owner_operations_for_sys5(),
        }
    }

    /// Count only requester-local pending owner requests.  This is an
    /// observer-safe state summary for finite delivery-failure diagnostics;
    /// it exposes neither a remote store nor request payload/binding.
    pub fn observer_safe_pending_owner_request_count(&self) -> usize {
        self.pending_outbound_owner_requests.len()
    }

    /// Total started delivery attempts retained across the bounded original
    /// requester-operation table.  This is a count only: it exposes no
    /// request identity, carrier bytes, source, or authority material.
    pub fn observer_safe_started_owner_request_attempt_count(&self) -> usize {
        self.pending_outbound_owner_requests
            .values()
            .map(|record| usize::from(record.started_attempts))
            .sum()
    }

    /// Latest started attempt per retained original request.  This supports
    /// bounded I3-3 failure/reconnect evidence without exposing an exact
    /// carrier, M8/M9 authority material, peer control, or caller-supplied
    /// reason text.
    pub fn observer_safe_original_owner_request_attempts(
        &self,
    ) -> Vec<Sys5I3ObserverSafeOriginalOwnerRequestAttempt> {
        self.pending_outbound_owner_requests
            .iter()
            .filter_map(|(semantic_request_identity_ref, record)| {
                record.last_started_attempt_kind.map(|kind| {
                    let source_requester_locus = record.pending.requester_locus().to_string();
                    Sys5I3ObserverSafeOriginalOwnerRequestAttempt {
                        semantic_request_identity_ref: semantic_request_identity_ref.clone(),
                        requester_binding_ref: observer_safe_requester_binding_ref(
                            &self.parent_checked_program_ref,
                            &self.projection_ref,
                            &self.cohort_ref,
                            semantic_request_identity_ref,
                            &source_requester_locus,
                        ),
                        source_requester_locus,
                        attempt_generation: record.started_attempts,
                        reason: kind.into(),
                    }
                })
            })
            .collect()
    }

    /// Count the retained, owner-local duplicate reservations without
    /// exposing request identities, bindings, or prior semantic results.
    pub fn observer_safe_inbound_owner_request_tombstone_count(&self) -> usize {
        self.inbound_owner_request_tombstones.len()
    }

    /// Counts retained C1/C2 dispositions without exporting a carrier,
    /// clock-control handle, pending identity, permit, or authority fact.
    pub fn observer_safe_owner_admission_summary(&self) -> Sys5I3ObserverSafeOwnerAdmissionSummary {
        self.inbound_owner_request_tombstones.values().fold(
            Sys5I3ObserverSafeOwnerAdmissionSummary::default(),
            |mut summary, record| {
                match record.tombstone.phase {
                    Sys5I3InboundOwnerRequestTombstonePhase::Awaiting => {
                        summary.awaiting_count += 1;
                    }
                    Sys5I3InboundOwnerRequestTombstonePhase::Expired => {
                        summary.expired_count += 1;
                    }
                    Sys5I3InboundOwnerRequestTombstonePhase::RejectedBeforeServe => {
                        summary.rejected_before_serve_count += 1;
                    }
                    Sys5I3InboundOwnerRequestTombstonePhase::ServeReserved => {
                        summary.serve_reserved_count += 1;
                    }
                    Sys5I3InboundOwnerRequestTombstonePhase::Reserved
                    | Sys5I3InboundOwnerRequestTombstonePhase::Received
                    | Sys5I3InboundOwnerRequestTombstonePhase::Ambiguous => {}
                }
                summary
            },
        )
    }

    /// Return only the producer-derived decision references retained for one
    /// exact expired request.  `None` covers every non-expiry disposition and
    /// any failure before SYS-4 successfully produced its reply carrier.
    /// This does not inspect or decode a carrier, recompute a decision, or
    /// expose ticks, controls, authority, capability, witness, or payload.
    #[doc(hidden)]
    pub fn observer_safe_owner_admission_expiry_decision(
        &self,
        semantic_request_identity_ref: &str,
    ) -> Option<Sys5I3ObserverSafeOwnerAdmissionExpiryDecision> {
        self.inbound_owner_request_tombstones
            .get(semantic_request_identity_ref)
            .filter(|record| {
                record.tombstone.phase == Sys5I3InboundOwnerRequestTombstonePhase::Expired
            })
            .and_then(|record| record.tombstone.terminal_admission.as_ref())
            .and_then(|terminal| terminal.declared_deadline_expiry.clone())
    }

    /// Trusted owner-runtime controls derived from checked staged conditions.
    /// This is crate-private specifically so an observer/devtool/serde path
    /// cannot turn a count projection into clock-control authority.
    pub(crate) fn i3_trusted_owner_admission_clock_handles(
        &self,
    ) -> Vec<Sys5I3OwnerAdmissionClockHandle> {
        self.owner_admission_clocks
            .keys()
            .cloned()
            .map(|clock_key| Sys5I3OwnerAdmissionClockHandle {
                runtime_binding_ref: self.local_store_identity_ref.clone(),
                runtime_instance: self.owner_admission_runtime_instance,
                clock_key,
            })
            .collect()
    }

    /// Derive the finite set of opaque T0 host drivers from the admitted
    /// runtime's checked owner-admission conditions.  This exposes neither
    /// clock handles nor source-selected request control, and unbudgeted
    /// source derives no driver.
    ///
    /// The name and Rust visibility are provisional and doc-hidden.  They
    /// make no public API, ABI, provider, or clock-framework commitment.
    #[doc(hidden)]
    pub fn i3_admitted_owner_admission_host_drivers(&self) -> Vec<Sys5I3OwnerAdmissionHostDriver> {
        self.i3_trusted_owner_admission_clock_handles()
            .into_iter()
            .map(|clock_handle| Sys5I3OwnerAdmissionHostDriver { clock_handle })
            .collect()
    }

    /// Advance the opaque driver's checked owner clock when requested, then
    /// resolve and hand off at most one deterministic Awaiting admission.
    ///
    /// `None` retains the current tick; `Some` is the sole private monotonic
    /// host-clock input.  The driver is validated before either an empty
    /// queue or a ledger entry is inspected.  The caller cannot name a
    /// request, carrier, permit, expected outcome, or retry.  A generated
    /// reply is returned only after the existing gate and SYS-4/M8 handoff
    /// have produced it; an empty queue returns `None`.
    #[doc(hidden)]
    pub fn drive_next_owner_admission(
        &mut self,
        driver: &Sys5I3OwnerAdmissionHostDriver,
        next_tick: Option<u64>,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        if let Some(next_tick) = next_tick {
            // This validates runtime binding/key before mutating the clock,
            // and rejects backward input before any ledger inspection.
            self.advance_owner_admission_clock(&driver.clock_handle, next_tick)?;
        }
        // `take_next...` always validates the exact runtime-bound key before
        // reading the ledger.  Thus `None` cannot make foreign or stale
        // control look like an empty queue.
        let Some(awaiting) = self.take_next_owner_admission_awaiting(&driver.clock_handle)? else {
            return Ok(None);
        };
        match self.resolve_staged_owner_admission(awaiting)? {
            Sys5I3OwnerAdmissionResolution::ServeReserved(reserved) => {
                self.handoff_reserved_owner_admission(*reserved).map(Some)
            }
            Sys5I3OwnerAdmissionResolution::DeclaredOwnerFailure(message) => Ok(Some(*message)),
        }
    }

    /// Advance exactly one checked owner/domain clock.  This does not choose
    /// a request, issue a permit, expire an entry, or invoke M8/SYS-4.
    pub(crate) fn advance_owner_admission_clock(
        &mut self,
        handle: &Sys5I3OwnerAdmissionClockHandle,
        next_tick: u64,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if handle.runtime_binding_ref != self.local_store_identity_ref
            || handle.runtime_instance != self.owner_admission_runtime_instance
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected,
            ));
        }
        let Some(clock) = self.owner_admission_clocks.get_mut(&handle.clock_key) else {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected,
            ));
        };
        if next_tick < clock.tick {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected,
            ));
        }
        // Equal ticks are intentionally a no-op and cannot resolve any
        // lifecycle decision implicitly.
        clock.tick = next_tick;
        Ok(())
    }

    /// Return the first exact Awaiting entry for this checked owner/domain in
    /// deterministic ledger order.  The caller cannot name a request ID.
    pub(crate) fn take_next_owner_admission_awaiting(
        &self,
        handle: &Sys5I3OwnerAdmissionClockHandle,
    ) -> Result<Option<Sys5I3OwnerAdmissionAwaiting>, Sys5I3ProcessRuntimeError> {
        if handle.runtime_binding_ref != self.local_store_identity_ref
            || handle.runtime_instance != self.owner_admission_runtime_instance
            || !self.owner_admission_clocks.contains_key(&handle.clock_key)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected,
            ));
        }
        Ok(self.inbound_owner_request_tombstones.iter().find_map(
            |(request_identity_ref, record)| {
                (record.tombstone.phase == Sys5I3InboundOwnerRequestTombstonePhase::Awaiting
                    && record
                        .live_admission
                        .as_ref()
                        .is_some_and(|live| live.clock_key == handle.clock_key))
                .then(|| Sys5I3OwnerAdmissionAwaiting {
                    runtime_binding_ref: self.local_store_identity_ref.clone(),
                    runtime_instance: self.owner_admission_runtime_instance,
                    clock_key: handle.clock_key.clone(),
                    request_identity_ref: request_identity_ref.clone(),
                })
            },
        ))
    }

    /// Resolve one opaque Awaiting entry.  Current authority is revalidated
    /// before the deadline decision.  On-time resolution consumes the sole
    /// ledger issuance token and returns a non-Clone Reserved handoff; expiry
    /// and current-authority failure retain terminal metadata in the same map.
    pub(crate) fn resolve_staged_owner_admission(
        &mut self,
        awaiting: Sys5I3OwnerAdmissionAwaiting,
    ) -> Result<Sys5I3OwnerAdmissionResolution, Sys5I3ProcessRuntimeError> {
        if awaiting.runtime_binding_ref != self.local_store_identity_ref
            || awaiting.runtime_instance != self.owner_admission_runtime_instance
            || !self
                .owner_admission_clocks
                .contains_key(&awaiting.clock_key)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            ));
        }
        let (pending, carrier, deadline_tick, clock_key_matches) = self
            .inbound_owner_request_tombstones
            .get(&awaiting.request_identity_ref)
            .and_then(|record| {
                (record.tombstone.phase == Sys5I3InboundOwnerRequestTombstonePhase::Awaiting)
                    .then_some(record.live_admission.as_ref())
                    .flatten()
                    .map(|live| {
                        (
                            &live.pending,
                            &live.carrier,
                            live.deadline_tick,
                            live.clock_key == awaiting.clock_key,
                        )
                    })
            })
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        if !clock_key_matches {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            ));
        }
        let resolution_tick = self
            .owner_admission_clocks
            .get(&awaiting.clock_key)
            .expect("validated owner-admission clock remains live")
            .tick;
        let resolution_generation_ref = self.fabric.i3_owner_admission_generation_ref().to_string();
        if let Err(failure) = self
            .fabric
            .revalidate_i3_bound_owner_request_authority(pending, carrier)
        {
            // A substituted carrier is not a current-authority decision for
            // the retained exact request.  It must leave the sole Awaiting
            // issuance available for its authentic source carrier.
            if failure == Sys4I3OwnerRequestRevalidationFailure::CarrierBindingMismatch {
                return Err(owner_request_revalidation_runtime_error(failure));
            }
            self.reject_awaiting_owner_admission_before_serve(
                &awaiting.request_identity_ref,
                failure,
                resolution_tick,
                resolution_generation_ref,
            )?;
            return Err(owner_request_revalidation_runtime_error(failure));
        }
        if resolution_tick >= deadline_tick {
            let message = self.expire_awaiting_owner_admission(
                &awaiting.request_identity_ref,
                resolution_tick,
                resolution_generation_ref,
            )?;
            return Ok(Sys5I3OwnerAdmissionResolution::DeclaredOwnerFailure(
                Box::new(message),
            ));
        }
        self.reserve_staged_owner_admission(
            awaiting.request_identity_ref,
            resolution_tick,
            resolution_generation_ref,
        )
        .map(|reserved| Sys5I3OwnerAdmissionResolution::ServeReserved(Box::new(reserved)))
    }

    /// Consume a Reserved handoff before it can enter the existing SYS-4/M8
    /// path.  Any failure drops the permit and leaves the ledger disposition
    /// ServeReserved, so a caller cannot reacquire or count a second serve.
    pub(crate) fn handoff_reserved_owner_admission(
        &mut self,
        reserved: Sys5I3OwnerAdmissionReserved,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        if reserved.runtime_binding_ref != self.local_store_identity_ref
            || reserved.runtime_instance != self.owner_admission_runtime_instance
            || !self.reserved_owner_admission_matches(&reserved)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            ));
        }
        let resolution_tick = self
            .inbound_owner_request_tombstones
            .get(&reserved.request_identity_ref)
            .and_then(|record| record.tombstone.terminal_admission.as_ref())
            .and_then(|terminal| self.owner_admission_clocks.get(&terminal.clock_key))
            .map(|clock| clock.tick)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        let resolution_generation_ref = self.fabric.i3_owner_admission_generation_ref().to_string();
        if let Err(failure) = self
            .fabric
            .revalidate_i3_bound_owner_request_authority(&reserved.pending, &reserved.carrier)
        {
            self.record_reserved_owner_admission_authority_failure(
                &reserved.request_identity_ref,
                failure,
                resolution_tick,
                resolution_generation_ref,
            );
            return Err(owner_request_revalidation_runtime_error(failure));
        }
        self.accept_reserved_owner_admission_handoff(reserved)
    }

    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn test_only_reject_next_outbound_extraction(&mut self) {
        self.reject_next_outbound_extraction = true;
    }

    /// Negative-only I3-3 test control.  It rejects immediately after the
    /// owner duplicate ledger has retained the exact request but before any
    /// SYS-4 handoff.  It does not simulate or claim a post-mutation failure;
    /// that ambiguity remains fail-closed in the normal downstream path.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn test_only_reject_next_owner_admission_after_reservation(&mut self) {
        self.reject_next_owner_admission_after_reservation = true;
    }

    /// Negative-only C2 control over the existing real post-dequeue M8
    /// rejection seam.  It accepts neither an authority nor a carrier: the
    /// borrowed Reserved handoff supplies only the already staged exact
    /// operation/locus needed to arm that one genuine backend failure.
    #[cfg(test)]
    pub(crate) fn test_only_reject_next_staged_owner_admission_handoff(
        &mut self,
        reserved: &Sys5I3OwnerAdmissionReserved,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if reserved.runtime_binding_ref != self.local_store_identity_ref
            || reserved.runtime_instance != self.owner_admission_runtime_instance
            || !self.reserved_owner_admission_matches(reserved)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            ));
        }
        self.fabric
            .m8_backend_test_support_mut()
            .reject_next_owner_operation_after_dequeue(
                reserved.carrier.envelope_id(),
                reserved.pending.operation_id(),
                reserved.pending.owner_locus(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })
    }

    pub fn emit_generated_owner_request(
        &mut self,
        operation_id: &str,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        if self.pending_outbound_owner_requests.len() >= MAX_OUTBOUND_OWNER_REQUEST_PENDING {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OutboundRequestLedgerExhausted,
            ));
        }
        let submission = self
            .fabric
            .submit_source_action(SourceAction::owner_operation(operation_id))
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        self.fabric
            .validate_outbound_process_carrier(submission.origin_locus(), submission.envelope_id())
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        #[cfg(feature = "i3-process-test-seams")]
        if self.reject_next_outbound_extraction {
            self.reject_next_outbound_extraction = false;
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OutboundExtractionRejected,
            ));
        }
        let carrier = self
            .fabric
            .take_outbound_process_carrier(submission.origin_locus(), submission.envelope_id())
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        if carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
            ));
        }
        let pending = self
            .fabric
            .i3_pending_owner_request_binding(&carrier)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        let request_identity_ref = pending.semantic_request_identity_ref(
            &self.parent_checked_program_ref,
            &self.projection_ref,
            &self.cohort_ref,
        );
        let pending_token_ref = original_owner_request_pending_token_ref(
            &self.local_store_identity_ref,
            &request_identity_ref,
            &carrier,
        )
        .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected)
        })?;
        if self
            .pending_outbound_owner_requests
            .insert(
                request_identity_ref.clone(),
                Sys5I3OutboundOwnerRequestRecord {
                    pending,
                    exact_carrier: carrier.clone(),
                    pending_token_ref,
                    started_attempts: 0,
                    last_started_attempt_kind: None,
                    pending_handle_issued: false,
                },
            )
            .is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
            ));
        }
        Ok(Sys5I3ProcessMessage {
            kind: Sys5I3ProcessMessageKind::Request,
            carrier: Some(carrier),
            semantic_request_identity_ref: request_identity_ref,
            linked_request_identity_ref: None,
            terminal_failure_decision_occurrence_ref: None,
            cohort_provenance_ref: self.cohort_ref.clone(),
            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
        })
    }

    /// Consume one source-emitted request message into the private retry
    /// route.  The returned handle carries no carrier bytes and cannot
    /// authorize a send by itself.  Ordinary one-shot delivery may continue
    /// to consume `Sys5I3ProcessMessage` directly, but cannot later recover a
    /// retry handle from that moved value.
    pub fn into_original_owner_request_pending(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3OriginalOwnerRequestPending, Sys5I3ProcessRuntimeError> {
        if !matches!(message.kind, Sys5I3ProcessMessageKind::Request)
            || message.linked_request_identity_ref.is_some()
            || message.cohort_provenance_ref != self.cohort_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        let carrier = message.carrier.as_ref().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            )
        })?;
        let supplied_snapshot =
            private_process_carrier_snapshot_binding_bytes(carrier).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
                )
            })?;
        let record = self
            .pending_outbound_owner_requests
            .get_mut(&message.semantic_request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
                )
            })?;
        if record.pending_handle_issued
            || private_process_carrier_snapshot_binding_bytes(&record.exact_carrier)
                .map_or(true, |exact| exact != supplied_snapshot)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        record.pending_handle_issued = true;
        Ok(Sys5I3OriginalOwnerRequestPending {
            semantic_request_identity_ref: message.semantic_request_identity_ref,
            runtime_token_ref: record.pending_token_ref.clone(),
        })
    }

    /// Validate the runtime-owned original operation before the private QUIC
    /// adapter may reserve one network attempt.  This repeats the strict
    /// current lineage gate before the pure M9/M8 check, so a historical
    /// carrier never gets a more specific authority error after withdrawal.
    pub(crate) fn authorize_original_owner_request_attempt(
        &self,
        pending_handle: &Sys5I3OriginalOwnerRequestPending,
        kind: Sys5I3OriginalOwnerRequestAttemptKind,
    ) -> Result<Sys5I3AuthorizedOriginalOwnerRequestAttempt, Sys5I3ProcessRuntimeError> {
        let record = self
            .pending_outbound_owner_requests
            .get(&pending_handle.semantic_request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
                )
            })?;
        if !record.pending_handle_issued
            || record.pending_token_ref != pending_handle.runtime_token_ref
            || !self
                .assigned_loci
                .contains(record.pending.requester_locus())
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        let required_started_attempts = match kind {
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery => {
                (record.started_attempts == 0).then_some(())
            }
            // A pre-write connection/preface failure has not begun a frame
            // effect and leaves the initial budget unused.  The consuming
            // reconnect may therefore be its first actual send, or may be
            // the second after an ambiguous initial write.
            Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry => {
                (record.started_attempts <= 1).then_some(())
            }
        };
        if record.started_attempts >= MAX_OUTBOUND_OWNER_REQUEST_ATTEMPTS {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OutboundAttemptBudgetExhausted,
            ));
        }
        if required_started_attempts.is_none() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }

        // Resolve the current local contract before looking at the historical
        // record.  A revoked/superseded lineage is a generic carrier failure,
        // not an oracle for an otherwise captured M9 capability.
        let current = self
            .fabric
            .i3_pending_owner_request_binding(&record.exact_carrier)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        if current != record.pending {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
            ));
        }
        self.fabric
            .revalidate_i3_bound_owner_request_authority(&record.pending, &record.exact_carrier)
            .map_err(owner_request_revalidation_runtime_error)?;
        let encoded_message_bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_retained_owner_request_attempt(
                &record.exact_carrier,
                &pending_handle.semantic_request_identity_ref,
                &self.cohort_ref,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        let attempt_number = record.started_attempts.checked_add(1).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OutboundAttemptBudgetExhausted,
            )
        })?;
        Ok(Sys5I3AuthorizedOriginalOwnerRequestAttempt {
            semantic_request_identity_ref: pending_handle.semantic_request_identity_ref.clone(),
            runtime_token_ref: pending_handle.runtime_token_ref.clone(),
            attempt_number,
            attempt_kind: kind,
            encoded_message_bytes,
        })
    }

    /// Record that an adapter-owned write has become ambiguous by beginning
    /// its actual frame effect.  The original request remains pending after a
    /// frame failure; only its bounded delivery-attempt budget advances.
    pub(crate) fn commit_authorized_original_owner_request_attempt(
        &mut self,
        authorization: &Sys5I3AuthorizedOriginalOwnerRequestAttempt,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let record = self
            .pending_outbound_owner_requests
            .get_mut(authorization.semantic_request_identity_ref())
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
                )
            })?;
        if !record.pending_handle_issued
            || record.pending_token_ref != authorization.runtime_token_ref
            || record.started_attempts.checked_add(1) != Some(authorization.attempt_number)
            || authorization.attempt_number > MAX_OUTBOUND_OWNER_REQUEST_ATTEMPTS
            || !matches!(
                (record.started_attempts, authorization.attempt_kind),
                (0, Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery)
                    | (0, Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry)
                    | (1, Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry)
            )
            || !matches!(
                (record.last_started_attempt_kind, authorization.attempt_kind),
                (None, Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery)
                    | (None, Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry)
                    | (
                        Some(Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery),
                        Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry,
                    )
            )
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        record.started_attempts = authorization.attempt_number;
        record.last_started_attempt_kind = Some(authorization.attempt_kind);
        Ok(())
    }

    /// Admission companion for a caller-held original pending handle.  The
    /// candidate's claimed identity is compared with that trusted local
    /// handle only as an early reject; it cannot join an operation until the
    /// usual sealed carrier and pending-reply checks succeed.  On success
    /// `accept_inbound` removes the runtime-owned pending record exactly once.
    pub(crate) fn admit_decoded_original_owner_reply(
        &mut self,
        candidate: Sys5I3UntrustedProcessMessage,
        pending_handle: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        if !matches!(candidate.message.kind, PrivateProcessMessageKind::Reply)
            || candidate.message.semantic_request_identity_ref
                != pending_handle.semantic_request_identity_ref
            || candidate.message.linked_request_identity_ref.as_deref()
                != Some(pending_handle.semantic_request_identity_ref.as_str())
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        let record = self
            .pending_outbound_owner_requests
            .get(&pending_handle.semantic_request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
                )
            })?;
        if !record.pending_handle_issued
            || record.pending_token_ref != pending_handle.runtime_token_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected,
            ));
        }
        self.admit_decoded_process_message(candidate)
    }

    pub fn accept_inbound(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        // Fail before carrier/M9 admission.  A transport-independent cohort
        // provenance namespace prevents two same-source local activations
        // from crossing their sealed child images, but grants no authority.
        if message.cohort_provenance_ref != self.cohort_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        match message.kind {
            Sys5I3ProcessMessageKind::Request => {
                let carrier = message.carrier.ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
                if carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let pending = self
                    .fabric
                    .i3_pending_owner_request_binding(&carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                let expected_request_identity_ref = pending.semantic_request_identity_ref(
                    &self.parent_checked_program_ref,
                    &self.projection_ref,
                    &self.cohort_ref,
                );
                if message.semantic_request_identity_ref != expected_request_identity_ref
                    || message.linked_request_identity_ref.is_some()
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                if !self.assigned_loci.contains(carrier.target_locus()) {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::NonOwnerServe,
                    ));
                }
                let request_identity_ref = message.semantic_request_identity_ref;
                let carrier_snapshot_binding_bytes =
                    private_process_carrier_snapshot_binding_bytes(&carrier).map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                self.fabric
                    .revalidate_i3_bound_owner_request_authority(&pending, &carrier)
                    .map_err(|failure| {
                        Sys5I3ProcessRuntimeError::new(match failure {
                            Sys4I3OwnerRequestRevalidationFailure::CarrierBindingMismatch => {
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
                            }
                            Sys4I3OwnerRequestRevalidationFailure::StaleMembership => {
                                Sys5I3ProcessRuntimeErrorKind::StaleMembership
                            }
                            Sys4I3OwnerRequestRevalidationFailure::MissingCapability => {
                                Sys5I3ProcessRuntimeErrorKind::MissingCapability
                            }
                            Sys4I3OwnerRequestRevalidationFailure::MissingWitness => {
                                Sys5I3ProcessRuntimeErrorKind::MissingWitness
                            }
                        })
                    })?;
                // Duplicate identity/binding and finite-ledger capacity take
                // precedence over later clock arithmetic.  A duplicate at
                // an overflowing tick remains a duplicate.
                self.preflight_inbound_owner_request_reservation(
                    &request_identity_ref,
                    &carrier_snapshot_binding_bytes,
                )?;
                let staged_owner_admission = self.prepare_owner_admission_stage(
                    &pending,
                    &carrier,
                    &request_identity_ref,
                    &carrier_snapshot_binding_bytes,
                )?;
                self.reserve_inbound_owner_request(
                    &request_identity_ref,
                    carrier_snapshot_binding_bytes,
                )?;

                #[cfg(feature = "i3-process-test-seams")]
                if self.reject_next_owner_admission_after_reservation {
                    self.reject_next_owner_admission_after_reservation = false;
                    self.mark_inbound_owner_request_tombstone(
                        &request_identity_ref,
                        Sys5I3InboundOwnerRequestTombstonePhase::Ambiguous,
                    );
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }

                if let Some(staged_owner_admission) = staged_owner_admission {
                    self.stage_reserved_owner_admission(
                        &request_identity_ref,
                        carrier,
                        pending,
                        staged_owner_admission,
                    )?;
                    // A checked budget condition stages only its exact source
                    // request.  It has no reply, queue occurrence, M8 use, or
                    // store mutation until a later trusted clock resolution.
                    return Ok(None);
                }

                // Reserve before SYS-4 can enqueue/linearize the carrier.
                // If any later step fails, retain the tombstone as ambiguous:
                // this layer must not turn an uncertain post-handoff result
                // into permission to replay the semantic request.
                match self.accept_reserved_inbound_owner_request(carrier, &request_identity_ref) {
                    Ok(reply) => {
                        self.mark_inbound_owner_request_tombstone(
                            &request_identity_ref,
                            Sys5I3InboundOwnerRequestTombstonePhase::Received,
                        );
                        Ok(Some(reply))
                    }
                    Err(error) => {
                        self.mark_inbound_owner_request_tombstone(
                            &request_identity_ref,
                            Sys5I3InboundOwnerRequestTombstonePhase::Ambiguous,
                        );
                        Err(error)
                    }
                }
            }
            Sys5I3ProcessMessageKind::Reply => {
                let carrier = message.carrier.ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
                if carrier.edge_kind() != CommunicationEdgeKind::OwnerReplyReceipt {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let request_identity_ref = message.semantic_request_identity_ref.clone();
                if message.linked_request_identity_ref.as_deref()
                    != Some(request_identity_ref.as_str())
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let pending = self
                    .pending_outbound_owner_requests
                    .get(&request_identity_ref)
                    .map(|record| record.pending.clone())
                    .ok_or_else(|| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                let validated_reply = self
                    .fabric
                    .validate_i3_pending_owner_reply(&pending, &carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                // A validated terminal failure must be retainable before
                // SYS-4 admits and dequeues its reply carrier.  Otherwise a
                // full terminal ledger would record a phantom local dequeue
                // despite preserving the genuine requester pending entry.
                if matches!(
                    &validated_reply,
                    Sys4I3ValidatedOwnerReply::DeclaredDeadlineExpired { .. }
                ) {
                    if self
                        .requester_terminal_declared_owner_failures
                        .contains_key(&request_identity_ref)
                    {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        ));
                    }
                    if self.requester_terminal_declared_owner_failures.len()
                        >= MAX_OUTBOUND_OWNER_TERMINAL_FAILURES
                    {
                        return Err(Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::OutboundTerminalFailureLedgerExhausted,
                        ));
                    }
                }
                if !self.assigned_loci.contains(carrier.target_locus()) {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let step = self
                    .fabric
                    .accept_inbound_process_carrier(carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                match validated_reply {
                    Sys4I3ValidatedOwnerReply::Success => {
                        if step.receipt().is_none()
                            || step.declared_owner_deadline_expired().is_some()
                        {
                            return Err(Sys5I3ProcessRuntimeError::new(
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                            ));
                        }
                        let receipt_occurrence = observer_safe_process_occurrence_ref(
                            "requester-local-receipt",
                            &request_identity_ref,
                            step.consumed_envelope_id(),
                            step.locus_dequeue_occurrence_id(),
                        );
                        self.semantic_occurrences
                            .requester_local_receipts
                            .insert(request_identity_ref.clone(), receipt_occurrence);
                        self.accepted_inbound_receipt_count = self
                            .accepted_inbound_receipt_count
                            .checked_add(1)
                            .ok_or_else(|| {
                                Sys5I3ProcessRuntimeError::new(
                                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                                )
                            })?;
                        self.pending_outbound_owner_requests
                            .remove(&request_identity_ref);
                        Ok(Some(Sys5I3ProcessMessage {
                            kind: Sys5I3ProcessMessageKind::Receipt,
                            carrier: None,
                            semantic_request_identity_ref: message
                                .semantic_request_identity_ref
                                .clone(),
                            linked_request_identity_ref: Some(
                                message.semantic_request_identity_ref,
                            ),
                            terminal_failure_decision_occurrence_ref: None,
                            cohort_provenance_ref: self.cohort_ref.clone(),
                            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
                        }))
                    }
                    Sys4I3ValidatedOwnerReply::DeclaredDeadlineExpired {
                        decision_commitment_ref,
                    } => {
                        let failure = step.declared_owner_deadline_expired().ok_or_else(|| {
                            Sys5I3ProcessRuntimeError::new(
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                            )
                        })?;
                        if step.receipt().is_some()
                            || failure.decision_commitment_ref() != decision_commitment_ref
                        {
                            return Err(Sys5I3ProcessRuntimeError::new(
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                            ));
                        }
                        let occurrence = observer_safe_process_occurrence_ref(
                            "requester-terminal-declared-owner-failure",
                            &request_identity_ref,
                            failure.decision_occurrence_ref(),
                            step.locus_dequeue_occurrence_id(),
                        );
                        self.requester_terminal_declared_owner_failures.insert(
                            request_identity_ref.clone(),
                            Sys5I3RequesterTerminalDeclaredOwnerFailure {
                                decision_commitment_ref,
                                decision_occurrence_ref: occurrence.clone(),
                            },
                        );
                        self.semantic_occurrences
                            .requester_terminal_declared_owner_failures
                            .insert(request_identity_ref.clone(), occurrence);
                        self.accepted_inbound_declared_owner_failure_count = self
                            .accepted_inbound_declared_owner_failure_count
                            .checked_add(1)
                            .ok_or_else(|| {
                                Sys5I3ProcessRuntimeError::new(
                                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                                )
                            })?;
                        // Terminal retention is committed before removing the
                        // original pending entry; this is deliberately not a
                        // receipt or a successful owner execution.
                        self.pending_outbound_owner_requests
                            .remove(&request_identity_ref);
                        Ok(Some(Sys5I3ProcessMessage {
                            kind: Sys5I3ProcessMessageKind::TerminalFailureConsumed,
                            carrier: None,
                            semantic_request_identity_ref: message
                                .semantic_request_identity_ref
                                .clone(),
                            linked_request_identity_ref: Some(
                                message.semantic_request_identity_ref,
                            ),
                            terminal_failure_decision_occurrence_ref: self
                                .requester_terminal_declared_owner_failures
                                .get(&request_identity_ref)
                                .map(|record| record.decision_occurrence_ref.clone()),
                            cohort_provenance_ref: self.cohort_ref.clone(),
                            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
                        }))
                    }
                }
            }
            Sys5I3ProcessMessageKind::Receipt => {
                // A consumed owner reply completes requester-locally.  There
                // is no invented third carrier back to the owner and no
                // receipt admission count to inflate.
                let _ = message;
                Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                ))
            }
            Sys5I3ProcessMessageKind::TerminalFailureConsumed => {
                Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                ))
            }
        }
    }

    fn prepare_owner_admission_stage(
        &self,
        pending: &Sys4I3PendingOwnerRequestBinding,
        carrier: &Sys4ProcessCarrier,
        request_identity_ref: &str,
        carrier_snapshot_binding_bytes: &[u8],
    ) -> Result<Option<Sys5I3PreparedOwnerAdmission>, Sys5I3ProcessRuntimeError> {
        let Some(condition) = pending.owner_admission_budget() else {
            return Ok(None);
        };
        let clock_key = Sys5I3OwnerAdmissionClockKey {
            owner_locus: condition.owner_locus().as_str().to_string(),
            clock_domain: condition.clock_domain().as_str().to_string(),
        };
        let start_tick = self
            .owner_admission_clocks
            .get(&clock_key)
            .map(|clock| clock.tick)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        let deadline_tick = start_tick
            .checked_add(condition.budget_ticks())
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionDeadlineOverflow,
                )
            })?;
        let issuance = self
            .fabric
            .stage_i3_owner_admission_issuance(
                pending,
                carrier,
                request_identity_ref,
                &self.local_store_identity_ref,
                carrier_snapshot_binding_bytes.to_vec(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        Ok(Some(Sys5I3PreparedOwnerAdmission {
            clock_key,
            start_tick,
            deadline_tick,
            issuance,
        }))
    }

    fn stage_reserved_owner_admission(
        &mut self,
        request_identity_ref: &str,
        carrier: Sys4ProcessCarrier,
        pending: Sys4I3PendingOwnerRequestBinding,
        staged: Sys5I3PreparedOwnerAdmission,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let record = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        if record.tombstone.phase != Sys5I3InboundOwnerRequestTombstonePhase::Reserved
            || record.live_admission.is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            ));
        }
        record.tombstone.phase = Sys5I3InboundOwnerRequestTombstonePhase::Awaiting;
        record.live_admission = Some(Sys5I3LiveOwnerAdmission {
            carrier,
            pending,
            clock_key: staged.clock_key,
            start_tick: staged.start_tick,
            deadline_tick: staged.deadline_tick,
            issuance: staged.issuance,
        });
        Ok(())
    }

    fn reject_awaiting_owner_admission_before_serve(
        &mut self,
        request_identity_ref: &str,
        failure: Sys4I3OwnerRequestRevalidationFailure,
        resolution_tick: u64,
        resolution_generation_ref: String,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let record = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        let live = record.live_admission.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            )
        })?;
        record.tombstone.phase = Sys5I3InboundOwnerRequestTombstonePhase::RejectedBeforeServe;
        record.tombstone.terminal_admission = Some(Sys5I3TerminalOwnerAdmission {
            semantic_request_identity_ref: request_identity_ref.to_string(),
            carrier: live.carrier,
            pending: live.pending,
            clock_key: live.clock_key,
            start_tick: live.start_tick,
            deadline_tick: live.deadline_tick,
            resolution_tick,
            resolution_generation_ref,
            declared_deadline_expiry: None,
            current_authority_failure: Some(failure),
            handoff_revalidation_tick: None,
            handoff_revalidation_generation_ref: None,
            handoff_authority_failure: None,
        });
        // Dropping the issuance is deliberate: a revoked Awaiting request
        // cannot obtain another permit after an authority change.
        Ok(())
    }

    fn expire_awaiting_owner_admission(
        &mut self,
        request_identity_ref: &str,
        resolution_tick: u64,
        resolution_generation_ref: String,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        // Fail known-invalid expiry before terminalizing the live issuance.
        // The gate owns both the current-authority proof and the checked
        // deadline arithmetic; no raw tick or carrier field can be turned
        // into a failure reply by this caller.
        {
            let record = self
                .inbound_owner_request_tombstones
                .get(request_identity_ref)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                    )
                })?;
            let live = record.live_admission.as_ref().ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
            self.fabric
                .can_declare_i3_owner_admission_deadline_expired(
                    &live.issuance,
                    &live.pending,
                    &live.carrier,
                    request_identity_ref,
                    &self.local_store_identity_ref,
                    record.tombstone.carrier_snapshot_binding_bytes.clone(),
                    live.start_tick,
                    resolution_tick,
                )
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
        }
        let record = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        let live = record.live_admission.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
            )
        })?;
        let carrier_snapshot_binding_bytes =
            record.tombstone.carrier_snapshot_binding_bytes.clone();
        record.tombstone.phase = Sys5I3InboundOwnerRequestTombstonePhase::Expired;
        record.tombstone.terminal_admission = Some(Sys5I3TerminalOwnerAdmission {
            semantic_request_identity_ref: request_identity_ref.to_string(),
            carrier: live.carrier.clone(),
            pending: live.pending.clone(),
            clock_key: live.clock_key.clone(),
            start_tick: live.start_tick,
            deadline_tick: live.deadline_tick,
            resolution_tick,
            resolution_generation_ref,
            declared_deadline_expiry: None,
            current_authority_failure: None,
            handoff_revalidation_tick: None,
            handoff_revalidation_generation_ref: None,
            handoff_authority_failure: None,
        });
        // Terminal retention commits before consuming the issuance to create
        // the existing checked reply carrier.  A later construction failure
        // leaves `Expired` retained and cannot reissue the token.
        let produced = self
            .fabric
            .declare_i3_owner_admission_deadline_expired(
                live.issuance,
                &live.pending,
                &live.carrier,
                request_identity_ref,
                &self.local_store_identity_ref,
                carrier_snapshot_binding_bytes,
                live.start_tick,
                resolution_tick,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        let (reply, decision_commitment_ref, decision_occurrence_ref) = produced.into_parts();
        let message = Sys5I3ProcessMessage {
            kind: Sys5I3ProcessMessageKind::Reply,
            carrier: Some(reply),
            semantic_request_identity_ref: request_identity_ref.to_string(),
            linked_request_identity_ref: Some(request_identity_ref.to_string()),
            terminal_failure_decision_occurrence_ref: None,
            cohort_provenance_ref: self.cohort_ref.clone(),
            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
        };
        let terminal = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
            .filter(|record| {
                record.tombstone.phase == Sys5I3InboundOwnerRequestTombstonePhase::Expired
            })
            .and_then(|record| record.tombstone.terminal_admission.as_mut())
            .filter(|terminal| {
                terminal.semantic_request_identity_ref == request_identity_ref
                    && terminal.declared_deadline_expiry.is_none()
            })
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
        terminal.declared_deadline_expiry = Some(Sys5I3ObserverSafeOwnerAdmissionExpiryDecision {
            decision_commitment_ref,
            decision_occurrence_ref,
        });
        Ok(message)
    }

    fn reserve_staged_owner_admission(
        &mut self,
        request_identity_ref: String,
        resolution_tick: u64,
        resolution_generation_ref: String,
    ) -> Result<Sys5I3OwnerAdmissionReserved, Sys5I3ProcessRuntimeError> {
        // Consume no issuance token and mutate no ledger state until the
        // freshly revalidated binding is feasible.  G1/G2 may differ only in
        // generation: SYS-4 checks every immutable request/carrier/contract
        // field here and a later handoff remains generation-exact.
        {
            let record = self
                .inbound_owner_request_tombstones
                .get(&request_identity_ref)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                    )
                })?;
            if record.tombstone.phase != Sys5I3InboundOwnerRequestTombstonePhase::Awaiting {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                ));
            }
            let live = record.live_admission.as_ref().ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
            self.fabric
                .can_issue_i3_owner_admission_permit(
                    &live.issuance,
                    &live.pending,
                    &live.carrier,
                    &request_identity_ref,
                    &self.local_store_identity_ref,
                    record.tombstone.carrier_snapshot_binding_bytes.clone(),
                )
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
        }
        let (live, carrier_snapshot_binding_bytes) = {
            let record = self
                .inbound_owner_request_tombstones
                .get_mut(&request_identity_ref)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                    )
                })?;
            if record.tombstone.phase != Sys5I3InboundOwnerRequestTombstonePhase::Awaiting {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                ));
            }
            let live = record.live_admission.take().ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected,
                )
            })?;
            record.tombstone.phase = Sys5I3InboundOwnerRequestTombstonePhase::ServeReserved;
            record.tombstone.terminal_admission = Some(Sys5I3TerminalOwnerAdmission {
                semantic_request_identity_ref: request_identity_ref.clone(),
                carrier: live.carrier.clone(),
                pending: live.pending.clone(),
                clock_key: live.clock_key.clone(),
                start_tick: live.start_tick,
                deadline_tick: live.deadline_tick,
                resolution_tick,
                resolution_generation_ref,
                declared_deadline_expiry: None,
                current_authority_failure: None,
                handoff_revalidation_tick: None,
                handoff_revalidation_generation_ref: None,
                handoff_authority_failure: None,
            });
            (
                live,
                record.tombstone.carrier_snapshot_binding_bytes.clone(),
            )
        };
        let permit = self
            .fabric
            .issue_i3_owner_admission_permit(
                live.issuance,
                &live.pending,
                &live.carrier,
                &request_identity_ref,
                &self.local_store_identity_ref,
                carrier_snapshot_binding_bytes.clone(),
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        Ok(Sys5I3OwnerAdmissionReserved {
            runtime_binding_ref: self.local_store_identity_ref.clone(),
            runtime_instance: self.owner_admission_runtime_instance,
            request_identity_ref,
            carrier: live.carrier,
            pending: live.pending,
            carrier_snapshot_binding_bytes,
            permit,
        })
    }

    fn reserved_owner_admission_matches(&self, reserved: &Sys5I3OwnerAdmissionReserved) -> bool {
        self.inbound_owner_request_tombstones
            .get(&reserved.request_identity_ref)
            .is_some_and(|record| {
                record.tombstone.phase == Sys5I3InboundOwnerRequestTombstonePhase::ServeReserved
                    && record.tombstone.carrier_snapshot_binding_bytes
                        == reserved.carrier_snapshot_binding_bytes
                    && record
                        .tombstone
                        .terminal_admission
                        .as_ref()
                        .is_some_and(|terminal| {
                            terminal.semantic_request_identity_ref == reserved.request_identity_ref
                                && terminal.pending == reserved.pending
                                && terminal.carrier.envelope_id() == reserved.carrier.envelope_id()
                                && terminal.start_tick <= terminal.resolution_tick
                                && terminal.resolution_tick < terminal.deadline_tick
                                && !terminal.resolution_generation_ref.is_empty()
                                && terminal.current_authority_failure.is_none()
                        })
            })
    }

    fn record_reserved_owner_admission_authority_failure(
        &mut self,
        request_identity_ref: &str,
        failure: Sys4I3OwnerRequestRevalidationFailure,
        resolution_tick: u64,
        resolution_generation_ref: String,
    ) {
        if let Some(terminal) = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
            .and_then(|record| record.tombstone.terminal_admission.as_mut())
        {
            terminal.handoff_revalidation_tick = Some(resolution_tick);
            terminal.handoff_revalidation_generation_ref = Some(resolution_generation_ref);
            terminal.handoff_authority_failure = Some(failure);
        }
    }

    fn accept_reserved_owner_admission_handoff(
        &mut self,
        reserved: Sys5I3OwnerAdmissionReserved,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        let Sys5I3OwnerAdmissionReserved {
            request_identity_ref,
            carrier,
            pending,
            carrier_snapshot_binding_bytes,
            permit,
            ..
        } = reserved;
        let target_locus = carrier.target_locus().to_string();
        let step = self
            .fabric
            .accept_i3_owner_request_with_permit(
                carrier,
                &pending,
                &request_identity_ref,
                &self.local_store_identity_ref,
                carrier_snapshot_binding_bytes,
                permit,
            )
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        let reply =
            self.finalize_accepted_owner_step(&target_locus, &request_identity_ref, step)?;
        // A successful finalizer has extracted and validated the reply and
        // recorded the actual SYS-4/M8 occurrences.  Only now is this no
        // longer an outstanding one-use reservation.
        self.mark_inbound_owner_request_tombstone(
            &request_identity_ref,
            Sys5I3InboundOwnerRequestTombstonePhase::Received,
        );
        Ok(reply)
    }

    fn reserve_inbound_owner_request(
        &mut self,
        request_identity_ref: &str,
        carrier_snapshot_binding_bytes: Vec<u8>,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        self.preflight_inbound_owner_request_reservation(
            request_identity_ref,
            &carrier_snapshot_binding_bytes,
        )?;
        self.inbound_owner_request_tombstones.insert(
            request_identity_ref.to_string(),
            Sys5I3InboundOwnerRequestRecord {
                tombstone: Sys5I3InboundOwnerRequestTombstone {
                    carrier_snapshot_binding_bytes,
                    phase: Sys5I3InboundOwnerRequestTombstonePhase::Reserved,
                    terminal_admission: None,
                },
                live_admission: None,
            },
        );
        Ok(())
    }

    /// Check the existing bounded inbound ledger without changing it.  C1/C2
    /// invokes this before deadline arithmetic so a duplicate/capacity
    /// result cannot be masked by a temporal failure.
    fn preflight_inbound_owner_request_reservation(
        &self,
        request_identity_ref: &str,
        carrier_snapshot_binding_bytes: &[u8],
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        if let Some(existing) = self
            .inbound_owner_request_tombstones
            .get(request_identity_ref)
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                if existing.tombstone.carrier_snapshot_binding_bytes.as_slice()
                    == carrier_snapshot_binding_bytes
                {
                    Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected
                } else {
                    Sys5I3ProcessRuntimeErrorKind::RequestIdentityBindingMismatch
                },
            ));
        }
        if self.inbound_owner_request_tombstones.len() >= MAX_INBOUND_OWNER_REQUEST_TOMBSTONES {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InboundRequestLedgerExhausted,
            ));
        }
        Ok(())
    }

    fn mark_inbound_owner_request_tombstone(
        &mut self,
        request_identity_ref: &str,
        phase: Sys5I3InboundOwnerRequestTombstonePhase,
    ) {
        let Some(record) = self
            .inbound_owner_request_tombstones
            .get_mut(request_identity_ref)
        else {
            debug_assert!(
                false,
                "reserved inbound owner request must retain its tombstone"
            );
            return;
        };
        record.tombstone.phase = phase;
    }

    fn accept_reserved_inbound_owner_request(
        &mut self,
        carrier: Sys4ProcessCarrier,
        request_identity_ref: &str,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        let target_locus = carrier.target_locus().to_string();
        let step = self
            .fabric
            .accept_inbound_process_carrier(carrier)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        self.finalize_accepted_owner_step(&target_locus, request_identity_ref, step)
    }

    /// Apply the common post-SYS-4 step accounting only after a genuine
    /// accepted owner handoff.  Both ordinary admission and the private
    /// budget-gated route must derive their counters and occurrence evidence
    /// from the same `LocusStep`; neither infers a write from a reservation.
    fn finalize_accepted_owner_step(
        &mut self,
        target_locus: &str,
        request_identity_ref: &str,
        step: LocusStep,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        self.served_owner_request_count = self
            .served_owner_request_count
            .checked_add(1)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        let reply_envelope_id = step.reply_envelope_id().to_string();
        if reply_envelope_id.is_empty() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
            ));
        }
        let reply = self
            .fabric
            .take_outbound_process_carrier(target_locus, &reply_envelope_id)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        if reply.edge_kind() != CommunicationEdgeKind::OwnerReplyReceipt {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
            ));
        }
        let serve_occurrence = observer_safe_process_occurrence_ref(
            "owner-serve-linearization",
            request_identity_ref,
            step.m8_serve_node_id(),
            step.consumed_envelope_id(),
        );
        self.semantic_occurrences
            .owner_serve_linearizations
            .insert(request_identity_ref.to_string(), serve_occurrence);
        if let Some(actual_owner_write_occurrence_id) = step.actual_owner_write_occurrence_id() {
            self.local_authoritative_mutation_count = self
                .local_authoritative_mutation_count
                .checked_add(1)
                .ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
            let write_occurrence = observer_safe_process_occurrence_ref(
                "owner-actual-write",
                request_identity_ref,
                actual_owner_write_occurrence_id,
                step.locus_dequeue_occurrence_id(),
            );
            self.semantic_occurrences
                .actual_owner_writes
                .insert(request_identity_ref.to_string(), write_occurrence);
        }
        Ok(Sys5I3ProcessMessage {
            kind: Sys5I3ProcessMessageKind::Reply,
            carrier: Some(reply),
            semantic_request_identity_ref: request_identity_ref.to_string(),
            linked_request_identity_ref: Some(request_identity_ref.to_string()),
            terminal_failure_decision_occurrence_ref: None,
            cohort_provenance_ref: self.cohort_ref.clone(),
            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
        })
    }

    /// Receiver-owned admission core.  It is crate-private so only the
    /// feature-gated private QUIC adapter, after owning and inspecting a live
    /// connection and its bidi stream, can use it in a normal build.  The
    /// explicitly enabled codec regression seam below is the sole exception.
    /// It validates cohort provenance before touching SYS-4, then binds every
    /// static carrier field and owner-request M9 lineage to this local sealed
    /// image before mailbox/store mutation.
    #[cfg_attr(not(feature = "i3-private-quic"), allow(dead_code))]
    pub(crate) fn admit_decoded_process_message(
        &mut self,
        candidate: Sys5I3UntrustedProcessMessage,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        let message = candidate.message;
        if message.cohort_provenance_ref != self.cohort_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        let (kind, expected_edge_kind) = match message.kind {
            PrivateProcessMessageKind::Request => (
                Sys5I3ProcessMessageKind::Request,
                CommunicationEdgeKind::OwnerRequest,
            ),
            PrivateProcessMessageKind::Reply => (
                Sys5I3ProcessMessageKind::Reply,
                CommunicationEdgeKind::OwnerReplyReceipt,
            ),
        };
        let carrier = self
            .fabric
            .bind_i3_untrusted_process_carrier(message.carrier, expected_edge_kind)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        self.accept_inbound(Sys5I3ProcessMessage {
            kind,
            carrier: Some(carrier),
            semantic_request_identity_ref: message.semantic_request_identity_ref,
            linked_request_identity_ref: message.linked_request_identity_ref,
            terminal_failure_decision_occurrence_ref: None,
            cohort_provenance_ref: message.cohort_provenance_ref,
            identity_basis: Sys5I3ObserverSafeSemanticRequestIdentityBasis,
        })
    }

    /// Feature-gated direct decoded ingress retained for the G2a/G2b codec
    /// falsifiers.  The probe explicitly compiles this seam for negative
    /// bootstrap controls, while current normal localnet child ingress uses
    /// the private QUIC adapter rather than this entry point.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub fn admit_untrusted_message(
        &mut self,
        candidate: Sys5I3UntrustedProcessMessage,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        self.admit_decoded_process_message(candidate)
    }

    pub fn attempt_owner_serve(
        &mut self,
        request: &Sys5I3ProcessMessage,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let target = request
            .carrier
            .as_ref()
            .map(Sys4ProcessCarrier::target_locus)
            .unwrap_or_default();
        if !self.assigned_loci.contains(target) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::NonOwnerServe,
            ));
        }
        Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::DirectRemoteStore,
        ))
    }

    pub fn authoritative_i64_state(
        &self,
        state: &str,
        index: &str,
        field: &str,
    ) -> Result<i64, Sys5I3ProcessRuntimeError> {
        self.assigned_loci
            .iter()
            .find_map(|locus| {
                self.fabric
                    .semantic_snapshot()
                    .int(locus, state, index, field)
            })
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState,
                )
            })
    }
}

fn observer_safe_process_occurrence_ref(
    kind: &str,
    request_identity_ref: &str,
    source_occurrence_ref: &str,
    admission_occurrence_ref: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-semantic-occurrence/v1\\0");
    hasher.update(kind);
    hasher.update(request_identity_ref);
    hasher.update(source_occurrence_ref);
    hasher.update(admission_occurrence_ref);
    format!(
        "sys5-i3-process-semantic-occurrence-sha256-v1:{:x}",
        hasher.finalize()
    )
}

/// Canonicalize the exact private generated-carrier snapshot without exposing
/// it or treating it as a wire/API identity.  The ledger retains these bytes
/// only within its fixed 64-entry bound, so equality needs no standalone hash
/// collision assumption.
fn private_process_carrier_snapshot_binding_bytes(
    carrier: &Sys4ProcessCarrier,
) -> Result<Vec<u8>, ()> {
    let snapshot = carrier.i3_private_process_snapshot().map_err(|_| ())?;
    let bytes = serde_json::to_vec(&snapshot).map_err(|_| ())?;
    (bytes.len() <= Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES)
        .then_some(bytes)
        .ok_or(())
}

fn owner_request_revalidation_runtime_error(
    failure: Sys4I3OwnerRequestRevalidationFailure,
) -> Sys5I3ProcessRuntimeError {
    Sys5I3ProcessRuntimeError::new(match failure {
        Sys4I3OwnerRequestRevalidationFailure::CarrierBindingMismatch => {
            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
        }
        Sys4I3OwnerRequestRevalidationFailure::StaleMembership => {
            Sys5I3ProcessRuntimeErrorKind::StaleMembership
        }
        Sys4I3OwnerRequestRevalidationFailure::MissingCapability => {
            Sys5I3ProcessRuntimeErrorKind::MissingCapability
        }
        Sys4I3OwnerRequestRevalidationFailure::MissingWitness => {
            Sys5I3ProcessRuntimeErrorKind::MissingWitness
        }
    })
}

/// Bind an opaque local pending handle to both this runtime store and the
/// exact source-emitted carrier.  It is never sent on the network or used as
/// semantic provenance; the runtime still owns the only equality check.
fn original_owner_request_pending_token_ref(
    local_store_identity_ref: &str,
    semantic_request_identity_ref: &str,
    carrier: &Sys4ProcessCarrier,
) -> Result<String, ()> {
    let carrier_snapshot = private_process_carrier_snapshot_binding_bytes(carrier)?;
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/original-owner-request-pending/v1\0");
    for component in [
        local_store_identity_ref.as_bytes(),
        semantic_request_identity_ref.as_bytes(),
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component);
    }
    hasher.update((carrier_snapshot.len() as u64).to_be_bytes());
    hasher.update(carrier_snapshot);
    Ok(format!(
        "sys5-i3-original-owner-request-pending-sha256-v1:{:x}",
        hasher.finalize()
    ))
}

fn observer_safe_requester_binding_ref(
    parent_checked_program_ref: &str,
    projection_ref: &str,
    cohort_ref: &str,
    semantic_request_identity_ref: &str,
    source_requester_locus: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/original-owner-request-requester/v1\0");
    for component in [
        parent_checked_program_ref,
        projection_ref,
        cohort_ref,
        semantic_request_identity_ref,
        source_requester_locus,
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component.as_bytes());
    }
    format!(
        "sys5-i3-original-owner-request-requester-sha256-v1:{:x}",
        hasher.finalize()
    )
}

#[cfg(test)]
#[path = "sys5_i3_owner_admission_tests.rs"]
mod sys5_i3_owner_admission_tests;

#[cfg(all(test, unix))]
#[path = "sys5_i3_provider_control_tests.rs"]
mod sys5_i3_provider_control_tests;

fn logical_origin_ref(
    slot_name: &str,
    assigned_loci: &BTreeSet<String>,
    parent_checked_program_ref: &str,
    projection_ref: &str,
    cohort_ref: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/logical-origin/v1\\0");
    hasher.update(slot_name);
    hasher.update(format!("{assigned_loci:?}"));
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(cohort_ref);
    format!("sys5-i3-logical-origin-sha256-v1:{:x}", hasher.finalize())
}

fn process_store_identity_ref(
    parent_checked_program_ref: &str,
    projection_ref: &str,
    cohort_ref: &str,
    logical_origin_ref: &str,
    ordinal: usize,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-local-store/v2\\0");
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(cohort_ref);
    hasher.update(logical_origin_ref);
    hasher.update(ordinal.to_le_bytes());
    format!("sys5-i3-local-store-sha256-v2:{:x}", hasher.finalize())
}

#[cfg(test)]
mod i3_owner_request_ledger_tests {
    use super::*;

    use crate::sys5_local_slice::{Sys5SourceInput, build_project};

    const LEDGER_FIXTURE_PATH: &str = "tests/inline/sys5_i3_request_ledger.mir";
    const LEDGER_FIXTURE_SOURCE: &str =
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

    fn owner_and_exact_source_request_snapshot() -> (Sys5I3ProcessRuntime, String, Vec<u8>) {
        let project = build_project(Sys5SourceInput::inline(
            LEDGER_FIXTURE_PATH,
            LEDGER_FIXTURE_SOURCE,
        ))
        .expect("the ordinary checked source is available before a private ledger test");
        let deployment = Sys5I3Deployment::from_checked_project(
            &project,
            [
                Sys5I3DeploymentSlot::new(
                    "ledger-requester",
                    "127.0.0.1:41001",
                    ["ParticipantA", "ViewerC"],
                ),
                Sys5I3DeploymentSlot::new(
                    "ledger-owner",
                    "127.0.0.1:41002",
                    ["WorldAuthority", "ParticipantB"],
                ),
            ],
        )
        .expect("the checked source maps every locus to one nonempty private slot");
        let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
            .expect("one checked cohort derives both source-bound runtime images");
        let mut requester = Sys5I3ProcessRuntime::start(
            cohort
                .take_process_image("ledger-requester")
                .expect("the requester image is taken once"),
        )
        .expect("the checked requester image starts locally for this ledger-only test");
        let owner = Sys5I3ProcessRuntime::start(
            cohort
                .take_process_image("ledger-owner")
                .expect("the owner image is taken once"),
        )
        .expect("the checked owner image starts locally for this ledger-only test");
        let request = requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect("ordinary checked source emits the exact owner request");
        let request_identity = request.semantic_request_identity_ref().to_string();
        let exact_snapshot = private_process_carrier_snapshot_binding_bytes(
            request
                .carrier
                .as_ref()
                .expect("the generated owner request retains its private carrier internally"),
        )
        .expect("the real generated owner carrier has one bounded opaque snapshot");
        assert_eq!(
            owner.observer_safe_inbound_owner_request_tombstone_count(),
            0,
            "the owner ledger starts empty before its first private reservation"
        );
        (owner, request_identity, exact_snapshot)
    }

    #[test]
    fn i3_same_source_identity_with_a_different_snapshot_rejects_without_replacing_its_tombstone() {
        let (mut owner, request_identity, exact_snapshot) =
            owner_and_exact_source_request_snapshot();
        owner
            .reserve_inbound_owner_request(&request_identity, exact_snapshot.clone())
            .expect("the first source identity reserves its exact private snapshot");

        let mut different_snapshot = exact_snapshot;
        different_snapshot.push(0);
        assert_eq!(
            owner
                .reserve_inbound_owner_request(&request_identity, different_snapshot)
                .expect_err(
                    "the same semantic identity with another snapshot must not reuse the retained reservation",
                )
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::RequestIdentityBindingMismatch
        );
        assert_eq!(
            owner.observer_safe_inbound_owner_request_tombstone_count(),
            1,
            "a binding mismatch must retain the original reservation rather than overwrite it"
        );

        // This test reaches only the private ledger comparison. Normal
        // decoded ingress rejects an altered carrier at its earlier fresh
        // source/current-binding gate; this neither constructs one nor
        // creates authority for it.
    }
}

#[cfg(test)]
mod i3_original_owner_request_retry_tests {
    use super::*;

    use crate::sys5_local_slice::{Sys5SourceInput, build_project};

    const RETRY_FIXTURE_PATH: &str = "tests/inline/sys5_i3_original_owner_request_retry.mir";
    const RETRY_FIXTURE_SOURCE: &str =
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

    fn source_generated_pending_fixture() -> (
        Sys5I3ProcessRuntime,
        Sys5I3ProcessRuntime,
        Sys5I3PrivateProcessCodec,
        Sys5I3OriginalOwnerRequestPending,
        String,
    ) {
        let project = build_project(Sys5SourceInput::inline(
            RETRY_FIXTURE_PATH,
            RETRY_FIXTURE_SOURCE,
        ))
        .expect("the ordinary checked source is available before a runtime-only retry test");
        let deployment = Sys5I3Deployment::from_checked_project(
            &project,
            [
                Sys5I3DeploymentSlot::new(
                    "retry-requester",
                    "127.0.0.1:42001",
                    ["ParticipantA", "ViewerC"],
                ),
                Sys5I3DeploymentSlot::new(
                    "retry-owner",
                    "127.0.0.1:42002",
                    ["WorldAuthority", "ParticipantB"],
                ),
            ],
        )
        .expect("the checked source maps every locus to one nonempty retry-test slot");
        let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
            .expect("one checked cohort derives both source-bound retry-test images");
        let mut requester = Sys5I3ProcessRuntime::start(
            cohort
                .take_process_image("retry-requester")
                .expect("the requester image is taken once"),
        )
        .expect("the checked requester image starts locally for this runtime-only retry test");
        let owner = Sys5I3ProcessRuntime::start(
            cohort
                .take_process_image("retry-owner")
                .expect("the owner image is taken once"),
        )
        .expect("the checked owner image starts locally for this runtime-only retry test");
        let source_request = requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect("ordinary checked source emits the original owner request");
        let request_identity = source_request.semantic_request_identity_ref().to_string();
        let pending = requester
            .into_original_owner_request_pending(source_request)
            .expect("only the exact source-emitted request becomes the opaque pending handle");
        assert_eq!(pending.semantic_request_identity_ref(), request_identity);
        assert_eq!(requester.observer_safe_pending_owner_request_count(), 1);
        (
            requester,
            owner,
            Sys5I3PrivateProcessCodec::private_provisional_v1(),
            pending,
            request_identity,
        )
    }

    #[test]
    fn i3_runtime_only_original_request_allows_one_initial_and_one_exact_reconnect_attempt() {
        let (mut requester, owner, _codec, pending, request_identity) =
            source_generated_pending_fixture();

        assert_eq!(
            owner
                .authorize_original_owner_request_attempt(
                    &pending,
                    Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
                )
                .err()
                .expect("a pending handle names no operation in a foreign owner runtime")
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected
        );

        let initial = requester
            .authorize_original_owner_request_attempt(
                &pending,
                Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
            )
            .expect(
                "the source-generated opaque pending handle authorizes the first runtime attempt",
            );
        assert_eq!(initial.semantic_request_identity_ref(), request_identity);
        let initial_bytes = initial.encoded_message_bytes().to_vec();
        requester
            .commit_authorized_original_owner_request_attempt(&initial)
            .expect("beginning the initial frame effect commits only attempt generation one");

        let initial_attempts = requester.observer_safe_original_owner_request_attempts();
        assert_eq!(initial_attempts.len(), 1);
        let initial_observation = &initial_attempts[0];
        assert_eq!(
            initial_observation.semantic_request_identity_ref(),
            request_identity
        );
        assert_eq!(initial_observation.source_requester_locus(), "ParticipantA");
        assert_eq!(initial_observation.attempt_generation(), 1);
        assert_eq!(
            initial_observation.reason(),
            Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason::InitialDelivery
        );
        assert!(!initial_observation.requester_binding_ref().is_empty());
        assert_ne!(
            initial_observation.requester_binding_ref(),
            pending.runtime_token_ref.as_str(),
            "the observer-safe requester binding is not the opaque pending token"
        );
        assert_eq!(
            requester.observer_safe_started_owner_request_attempt_count(),
            1
        );

        assert_eq!(
            requester
                .authorize_original_owner_request_attempt(
                    &pending,
                    Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
                )
                .err()
                .expect("a second initial delivery is not a reconnect retry")
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected
        );

        let reconnect = requester
            .authorize_original_owner_request_attempt(
                &pending,
                Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry,
            )
            .expect("the retained original operation authorizes exactly one reconnect attempt");
        assert_eq!(reconnect.semantic_request_identity_ref(), request_identity);
        assert_eq!(
            reconnect.encoded_message_bytes(),
            initial_bytes.as_slice(),
            "a reconnect attempt retains the exact original private request bytes"
        );
        requester
            .commit_authorized_original_owner_request_attempt(&reconnect)
            .expect("the reconnect attempt consumes the second and final runtime attempt");

        let reconnect_attempts = requester.observer_safe_original_owner_request_attempts();
        assert_eq!(reconnect_attempts.len(), 1);
        let reconnect_observation = &reconnect_attempts[0];
        assert_eq!(
            reconnect_observation.semantic_request_identity_ref(),
            request_identity
        );
        assert_eq!(
            reconnect_observation.source_requester_locus(),
            "ParticipantA"
        );
        assert_eq!(reconnect_observation.attempt_generation(), 2);
        assert_eq!(
            reconnect_observation.reason(),
            Sys5I3ObserverSafeOriginalOwnerRequestAttemptReason::ReconnectRetry
        );
        assert!(!reconnect_observation.requester_binding_ref().is_empty());
        assert_ne!(
            reconnect_observation.requester_binding_ref(),
            pending.runtime_token_ref.as_str(),
            "the observer-safe retry record excludes the opaque pending token"
        );
        assert_eq!(
            requester.observer_safe_started_owner_request_attempt_count(),
            2
        );

        assert_eq!(
            requester
                .authorize_original_owner_request_attempt(
                    &pending,
                    Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry,
                )
                .err()
                .expect("a third actual delivery attempt exceeds the fixed runtime budget")
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::OutboundAttemptBudgetExhausted
        );
        assert_eq!(requester.observer_safe_pending_owner_request_count(), 1);

        // This covers only the runtime-owned pending/authorization state
        // machine. It does not create a QUIC session or establish a live
        // reconnect/transport proof.
    }

    #[test]
    fn i3_runtime_only_original_reply_rejection_preserves_pending_until_one_checked_receipt() {
        let (mut requester, mut owner, codec, pending, request_identity) =
            source_generated_pending_fixture();

        let initial = requester
            .authorize_original_owner_request_attempt(
                &pending,
                Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
            )
            .expect("the source-generated pending handle authorizes the initial runtime attempt");
        let request_bytes = initial.encoded_message_bytes().to_vec();
        requester
            .commit_authorized_original_owner_request_attempt(&initial)
            .expect("the runtime marks one begun initial attempt before owner admission");
        let reply = owner
            .admit_decoded_process_message(
                codec.decode_untrusted_message(&request_bytes).expect(
                    "the retained exact request bytes decode only as an untrusted candidate",
                ),
            )
            .expect("the checked owner admits the exact runtime-authorized request")
            .expect("the checked owner produces one typed reply");
        let reply_bytes = codec
            .encode_outbound_message(reply)
            .expect("the checked owner reply encodes through the private codec");

        let mut rejected_reply = codec
            .decode_untrusted_message(&reply_bytes)
            .expect("the checked reply decodes before its negative-input link omission");
        rejected_reply.message.linked_request_identity_ref = None;
        let attempts_before_rejection = requester.observer_safe_original_owner_request_attempts();
        assert_eq!(
            requester
                .admit_decoded_original_owner_reply(rejected_reply, &pending)
                .expect_err(
                    "a reply without the exact original-request link cannot consume pending"
                )
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected
        );
        assert_eq!(requester.observer_safe_pending_owner_request_count(), 1);
        assert_eq!(
            requester.observer_safe_original_owner_request_attempts(),
            attempts_before_rejection,
            "rejecting a candidate reply does not replace the retained original operation"
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            0
        );

        let receipt = requester
            .admit_decoded_original_owner_reply(
                codec.decode_untrusted_message(&reply_bytes).expect(
                    "the checked reply remains an untrusted input until requester admission",
                ),
                &pending,
            )
            .expect("the exact checked reply admits for its retained original pending handle")
            .expect("one accepted reply produces one requester-local receipt");
        assert!(receipt.is_observer_safe_typed_result_or_receipt());
        assert!(receipt.has_no_transportable_carrier());
        assert_eq!(receipt.semantic_request_identity_ref(), request_identity);
        assert_eq!(
            receipt.linked_request_identity_ref(),
            Some(request_identity.as_str())
        );
        assert_eq!(requester.observer_safe_pending_owner_request_count(), 0);
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            1
        );

        assert_eq!(
            requester
                .admit_decoded_original_owner_reply(
                    codec
                        .decode_untrusted_message(&reply_bytes)
                        .expect("the stale checked reply remains syntactically untrusted input"),
                    &pending,
                )
                .expect_err("a stale reply cannot recreate a consumed original pending operation")
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::OriginalRequestPendingRejected
        );
        assert_eq!(requester.observer_safe_pending_owner_request_count(), 0);
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            1,
            "a stale reply cannot mint a second local receipt"
        );

        // The negative reply is derived from an actual checked owner reply
        // and removes only its required outer request link. This is a
        // runtime-only reply-binding falsifier, not a fabricated pending,
        // authority, or live QUIC/session scenario.
    }
}
