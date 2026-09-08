//! Private Stage 3 provider carrier execution.
//!
//! This is a direct consumer of the installed SYS-4 provider profile and the
//! already-issued M9 role authority.  It deliberately contains no source
//! admission, image/bootstrap, transport, or authority issuer.  The QUIC
//! adapter moves the opaque request/result carriers below between the two
//! installed children.

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Read, Write},
    str,
};

#[cfg(target_os = "linux")]
use std::{
    ffi::CString,
    os::{fd::FromRawFd, unix::ffi::OsStrExt},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    i3_read_only_provider_composite::{
        I3PrivateReadOnlyProviderFixtureAssertion, I3PrivateReadOnlyProviderResourceEnvelope,
    },
    m9_auth_verification::{
        M9_I3_PROVIDER_TERMINAL_OBSERVATION_MAX_BODY_BYTES,
        M9_I3_PROVIDER_TERMINAL_OBSERVATION_MAX_ROWS,
        M9_I3_PROVIDER_TERMINAL_OBSERVATION_PROFILE_V2,
        M9_I3_PROVIDER_TERMINAL_OBSERVATION_SCHEMA_V2,
        M9_I3_PROVIDER_TERMINAL_OBSERVATION_TRANSPORT_OCCURRENCES_PER_KIND,
        M9I3PrivateProviderCarrierBinding, M9I3ReadOnlyProviderLocalAuthority,
        M9I3ReadOnlyProviderRevalidationFailure, M9I3ReadOnlyProviderRevalidationUse,
    },
    sys4_dispatch::{Sys4InstalledProviderLocalFabric, Sys4ProviderCarrierDescriptor},
};

use super::{Sys5I3ProcessRuntimeError, Sys5I3ProcessRuntimeErrorKind, Sys5I3ProviderChildRole};

const MAX_PROVIDER_LEDGER_ENTRIES: usize = 64;
const MAX_PROVIDER_TERMINAL_AUDIT_ROWS: usize =
    M9_I3_PROVIDER_TERMINAL_OBSERVATION_MAX_ROWS as usize;
const MAX_PROVIDER_TERMINAL_AUDIT_BYTES: usize =
    M9_I3_PROVIDER_TERMINAL_OBSERVATION_MAX_BODY_BYTES as usize;
const PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND: usize =
    M9_I3_PROVIDER_TERMINAL_OBSERVATION_TRANSPORT_OCCURRENCES_PER_KIND as usize;
const MAX_PROVIDER_READ_BYTES: usize = 32;
const PROVIDER_READ_SENTINEL_BYTES: usize = MAX_PROVIDER_READ_BYTES + 1;
const PRIVATE_PROVIDER_CARRIER_VERSION: u8 = 1;

/// One distinct requester-to-executor carrier.  Construction is restricted
/// to the installed requester runtime or the crate-private tainted codec.
/// It is neither an owner request nor a FabricReceipt alias.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderRequestCarrier {
    binding: M9I3PrivateProviderCarrierBinding,
    request_descriptor: Sys4ProviderCarrierDescriptor,
    request_ordinal: u64,
    request_identity_ref: String,
}

impl std::fmt::Debug for Sys5I3ProviderRequestCarrier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProviderRequestCarrier(..)")
    }
}

/// One distinct executor-to-requester result carrier.  Its private outcome
/// never appears in `Debug`, observer events, or source-independent APIs.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderResultCarrier {
    binding: M9I3PrivateProviderCarrierBinding,
    request_descriptor: Sys4ProviderCarrierDescriptor,
    result_descriptor: Sys4ProviderCarrierDescriptor,
    request_identity_ref: String,
    outcome: PrivateProviderOutcome,
}

impl std::fmt::Debug for Sys5I3ProviderResultCarrier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProviderResultCarrier(..)")
    }
}

/// A local retained completion.  It has no raw-result or carrier getter; the
/// finite test-driver selector is added only at the child test boundary.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderConsumeReceipt {
    request_identity_ref: String,
    outcome: PrivateProviderOutcome,
}

/// Opaque completion of the finite requester-side fixture assertion. It
/// exposes neither the selected expectation nor any provider result.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub struct Sys5I3ProviderFixtureAssertionCompletion {
    _private: (),
}

#[cfg(feature = "i3-process-test-seams")]
impl std::fmt::Debug for Sys5I3ProviderFixtureAssertionCompletion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProviderFixtureAssertionCompletion(..)")
    }
}

/// A finite local test point at which the already-installed M9 authority is
/// retired by its real monotone transition. This selector is process-local,
/// has no wire form, and is absent outside the I3 test-seam build.
#[cfg(all(test, feature = "i3-process-test-seams"))]
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProviderRevalidationTestPoint {
    BeforeReserve,
    AfterReserveBeforeCallStarted,
    AfterCallBeforeRelease,
    BeforeConsume,
}

/// One actual local M9 transition selectable at a finite test point. It does
/// not fabricate a failure, capability, witness, membership, or outcome.
#[cfg(all(test, feature = "i3-process-test-seams"))]
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProviderRevalidationTestRetirement {
    EffectCapability,
    EffectWitness,
    Membership,
}

#[cfg(all(test, feature = "i3-process-test-seams"))]
#[derive(Clone, Copy)]
struct Sys5I3ProviderRevalidationTestHook {
    point: Sys5I3ProviderRevalidationTestPoint,
    retirement: Sys5I3ProviderRevalidationTestRetirement,
}

/// Raw-free facts derived from the one retained provider ledger. They are
/// available only to the I3 test seam and neither authorize observation nor
/// expose a request, result, host path, or authority fact.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ProviderLedgerTestFacts {
    capacity: usize,
    retained_entry_count: usize,
    requester_pending_count: usize,
    executor_reserved_count: usize,
    executor_call_started_count: usize,
    executor_rejected_before_call_count: usize,
    executor_outcome_retained_count: usize,
    executor_released_count: usize,
    requester_consumed_count: usize,
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3ProviderLedgerTestFacts {
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub const fn retained_entry_count(&self) -> usize {
        self.retained_entry_count
    }

    pub const fn requester_pending_count(&self) -> usize {
        self.requester_pending_count
    }

    pub const fn executor_reserved_count(&self) -> usize {
        self.executor_reserved_count
    }

    pub const fn executor_call_started_count(&self) -> usize {
        self.executor_call_started_count
    }

    pub const fn executor_rejected_before_call_count(&self) -> usize {
        self.executor_rejected_before_call_count
    }

    pub const fn executor_outcome_retained_count(&self) -> usize {
        self.executor_outcome_retained_count
    }

    pub const fn executor_released_count(&self) -> usize {
        self.executor_released_count
    }

    pub const fn requester_consumed_count(&self) -> usize {
        self.requester_consumed_count
    }
}

impl std::fmt::Debug for Sys5I3ProviderConsumeReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProviderConsumeReceipt(..)")
    }
}

/// Fixed, reference-only terminal projection for one installed provider
/// child. Construction is gated by a distinct current M9 Observation permit;
/// it contains no raw provider value, native path, source text, credential,
/// authority material, timing, or encoded payload.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5I3ProviderTerminalAudit {
    profile_ref: &'static str,
    schema_ref: &'static str,
    role: Sys5I3ProviderChildRole,
    request_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    result_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    rows: Vec<Sys5I3ProviderTerminalAuditRow>,
    request_count: usize,
    reserved_count: usize,
    rejected_before_call_count: usize,
    call_started_count: usize,
    physical_adapter_entry_count: usize,
    actual_read_count: usize,
    outcome_retained_count: usize,
    released_count: usize,
    consume_count: usize,
    pending_count: usize,
}

impl std::fmt::Debug for Sys5I3ProviderTerminalAudit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3ProviderTerminalAudit(..)")
    }
}

impl Sys5I3ProviderTerminalAudit {
    fn empty() -> Self {
        Self {
            profile_ref: M9_I3_PROVIDER_TERMINAL_OBSERVATION_PROFILE_V2,
            schema_ref: M9_I3_PROVIDER_TERMINAL_OBSERVATION_SCHEMA_V2,
            role: Sys5I3ProviderChildRole::RequesterConsumer,
            request_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs::empty(),
            result_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs::empty(),
            rows: Vec::new(),
            request_count: 0,
            reserved_count: 0,
            rejected_before_call_count: 0,
            call_started_count: 0,
            physical_adapter_entry_count: 0,
            actual_read_count: 0,
            outcome_retained_count: 0,
            released_count: 0,
            consume_count: 0,
            pending_count: 0,
        }
    }
}

/// Fixed reference-only source/Core/artifact/edge correspondence in a
/// provider terminal observation candidate. It is neither authority nor a
/// source-program reconstruction input.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sys5I3ProviderTerminalAuditCarrierDescriptorRefs {
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    generated_edge_ref: String,
}

impl Sys5I3ProviderTerminalAuditCarrierDescriptorRefs {
    fn empty() -> Self {
        Self {
            source_ref: String::new(),
            core_ref: String::new(),
            source_artifact_ref: String::new(),
            target_artifact_ref: String::new(),
            generated_edge_ref: String::new(),
        }
    }

    fn from_descriptor(value: &Sys4ProviderCarrierDescriptor) -> Self {
        Self {
            source_ref: value.source_ref().to_string(),
            core_ref: value.core_ref().to_string(),
            source_artifact_ref: value.source_artifact_ref().to_string(),
            target_artifact_ref: value.target_artifact_ref().to_string(),
            generated_edge_ref: value.edge_ref().to_string(),
        }
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

    pub fn generated_edge_ref(&self) -> &str {
        &self.generated_edge_ref
    }
}

/// One bounded, reference-only terminal row. Its local occurrence references
/// derive from admitted semantic identity; its transport references derive
/// from adapter-owned run/session/direction/ordinal facts. Neither hashes or
/// otherwise encodes a provider value.
#[doc(hidden)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sys5I3ProviderTerminalAuditRow {
    semantic_request_ref: String,
    host_started_occurrence_ref: Option<String>,
    outcome_retained_occurrence_ref: Option<String>,
    release_occurrence_ref: Option<String>,
    local_consume_ref: Option<String>,
    adapter_entry_occurrence_ref: Option<String>,
    adapter_read_occurrence_ref: Option<String>,
    request_send_reservation_occurrence_refs:
        [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    request_send_completed_occurrence_refs:
        [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    request_receive_occurrence_refs: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_send_reservation_occurrence_refs:
        [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_send_completed_occurrence_refs:
        [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_receive_occurrence_refs: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    ordered_predecessor_refs: Vec<String>,
    outcome_class: Option<Sys5I3ProviderTerminalOutcomeClass>,
}

impl Sys5I3ProviderTerminalAuditRow {
    pub fn semantic_request_ref(&self) -> &str {
        &self.semantic_request_ref
    }

    pub fn provider_invocation_ref(&self) -> Option<&str> {
        self.host_started_occurrence_ref()
    }

    pub fn host_started_occurrence_ref(&self) -> Option<&str> {
        self.host_started_occurrence_ref.as_deref()
    }

    pub fn outcome_retained_occurrence_ref(&self) -> Option<&str> {
        self.outcome_retained_occurrence_ref.as_deref()
    }

    pub fn release_occurrence_ref(&self) -> Option<&str> {
        self.release_occurrence_ref.as_deref()
    }

    pub fn local_consume_ref(&self) -> Option<&str> {
        self.local_consume_ref.as_deref()
    }

    pub fn adapter_entry_occurrence_ref(&self) -> Option<&str> {
        self.adapter_entry_occurrence_ref.as_deref()
    }

    pub fn adapter_read_occurrence_ref(&self) -> Option<&str> {
        self.adapter_read_occurrence_ref.as_deref()
    }

    pub fn request_send_reservation_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.request_send_reservation_occurrence_refs
    }

    pub fn request_send_completed_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.request_send_completed_occurrence_refs
    }

    pub fn request_receive_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.request_receive_occurrence_refs
    }

    pub fn result_send_reservation_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.result_send_reservation_occurrence_refs
    }

    pub fn result_send_completed_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.result_send_completed_occurrence_refs
    }

    pub fn result_receive_occurrence_refs(
        &self,
    ) -> &[Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND] {
        &self.result_receive_occurrence_refs
    }

    pub fn ordered_predecessor_refs(&self) -> &[String] {
        &self.ordered_predecessor_refs
    }

    pub const fn outcome_class(&self) -> Option<Sys5I3ProviderTerminalOutcomeClass> {
        self.outcome_class
    }
}

/// Fixed raw-value-free terminal outcome classification. `ValuePresent` says
/// only that an internal value existed; it carries neither that value nor a
/// value-derived identifier.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sys5I3ProviderTerminalOutcomeClass {
    ValuePresent,
    ProviderResourceNotFound,
    ProviderPolicyDenied,
    ProviderInvalidResult,
    AdapterUnavailable,
}

/// Fixed retained counts for one decoded terminal-observation candidate.
#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ProviderTerminalAuditCounts {
    request_count: usize,
    reserved_count: usize,
    rejected_before_call_count: usize,
    call_started_count: usize,
    physical_adapter_entry_count: usize,
    actual_read_count: usize,
    outcome_retained_count: usize,
    released_count: usize,
    consume_count: usize,
    pending_count: usize,
}

impl Sys5I3ProviderTerminalAuditCounts {
    pub const fn request_count(&self) -> usize {
        self.request_count
    }
    pub const fn reserved_count(&self) -> usize {
        self.reserved_count
    }
    pub const fn rejected_before_call_count(&self) -> usize {
        self.rejected_before_call_count
    }
    pub const fn call_started_count(&self) -> usize {
        self.call_started_count
    }
    pub const fn physical_adapter_entry_count(&self) -> usize {
        self.physical_adapter_entry_count
    }
    pub const fn actual_read_count(&self) -> usize {
        self.actual_read_count
    }
    pub const fn outcome_retained_count(&self) -> usize {
        self.outcome_retained_count
    }
    pub const fn released_count(&self) -> usize {
        self.released_count
    }
    pub const fn consume_count(&self) -> usize {
        self.consume_count
    }
    pub const fn pending_count(&self) -> usize {
        self.pending_count
    }
}

/// A decoded child-output candidate. It is structurally bounded and
/// reference-only, but it is never an issued M9 permit, proof, or audit: the
/// supervising runtime must still correlate it with its owned run and child.
#[doc(hidden)]
pub struct Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
    profile_ref: String,
    schema_ref: String,
    role: Sys5I3ProviderChildRole,
    request_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    result_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    rows: Vec<Sys5I3ProviderTerminalAuditRow>,
    counts: Sys5I3ProviderTerminalAuditCounts,
}

impl std::fmt::Debug for Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate(..)")
    }
}

impl Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
    pub fn fixed_profile_ref(&self) -> &str {
        &self.profile_ref
    }

    pub fn fixed_schema_ref(&self) -> &str {
        &self.schema_ref
    }

    pub const fn role(&self) -> Sys5I3ProviderChildRole {
        self.role
    }

    pub fn request_descriptor(&self) -> &Sys5I3ProviderTerminalAuditCarrierDescriptorRefs {
        &self.request_descriptor
    }

    pub fn result_descriptor(&self) -> &Sys5I3ProviderTerminalAuditCarrierDescriptorRefs {
        &self.result_descriptor
    }

    pub fn rows(&self) -> &[Sys5I3ProviderTerminalAuditRow] {
        &self.rows
    }

    pub const fn counts(&self) -> Sys5I3ProviderTerminalAuditCounts {
        self.counts
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProviderTerminalAuditObserverViewWire {
    version: u8,
    profile_ref: String,
    schema_ref: String,
    role: Sys5I3ProviderChildRole,
    request_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    result_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    rows: Vec<Sys5I3ProviderTerminalAuditRow>,
    request_count: usize,
    reserved_count: usize,
    rejected_before_call_count: usize,
    call_started_count: usize,
    physical_adapter_entry_count: usize,
    actual_read_count: usize,
    outcome_retained_count: usize,
    released_count: usize,
    consume_count: usize,
    pending_count: usize,
}

/// Borrowed producer form of the strict observer-view wire record. Keeping
/// this distinct from the decoded owned wire avoids cloning the bounded rows
/// merely to count or encode them.
#[derive(Serialize)]
struct PrivateProviderTerminalAuditObserverViewWireRef<'a> {
    version: u8,
    profile_ref: &'a str,
    schema_ref: &'a str,
    role: Sys5I3ProviderChildRole,
    request_descriptor: &'a Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    result_descriptor: &'a Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
    rows: &'a [Sys5I3ProviderTerminalAuditRow],
    request_count: usize,
    reserved_count: usize,
    rejected_before_call_count: usize,
    call_started_count: usize,
    physical_adapter_entry_count: usize,
    actual_read_count: usize,
    outcome_retained_count: usize,
    released_count: usize,
    consume_count: usize,
    pending_count: usize,
}

/// A fixed-capacity body writer for the private terminal observer view. It
/// reserves the full protocol body bound exactly once before serialization;
/// writes verify remaining capacity before extending, so serde cannot trigger
/// geometric `Vec` growth beyond the fixed body limit.
struct BoundedProviderTerminalAuditBodyWriter {
    body: Vec<u8>,
}

impl BoundedProviderTerminalAuditBodyWriter {
    fn new() -> io::Result<Self> {
        let mut body = Vec::new();
        body.try_reserve_exact(MAX_PROVIDER_TERMINAL_AUDIT_BYTES)
            .map_err(|_| io::Error::other("terminal observer body allocation rejected"))?;
        if body.capacity() > MAX_PROVIDER_TERMINAL_AUDIT_BYTES {
            return Err(io::Error::other(
                "terminal observer body allocation exceeds fixed bound",
            ));
        }
        Ok(Self { body })
    }

    fn into_body(self) -> io::Result<Vec<u8>> {
        if self.body.len() > MAX_PROVIDER_TERMINAL_AUDIT_BYTES
            || self.body.capacity() > MAX_PROVIDER_TERMINAL_AUDIT_BYTES
        {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "terminal observer body exceeds fixed bound",
            ));
        }
        Ok(self.body)
    }
}

impl Write for BoundedProviderTerminalAuditBodyWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = MAX_PROVIDER_TERMINAL_AUDIT_BYTES
            .checked_sub(self.body.len())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::WriteZero,
                    "terminal observer body exceeds fixed bound",
                )
            })?;
        if bytes.len() > remaining {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "terminal observer body exceeds fixed bound",
            ));
        }
        self.body.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A no-allocation sibling used at projection time. It proves the bounded
/// JSON representation fits before the installed child performs its final M9
/// currentness commit; it is not an observer payload or a second wire path.
struct CountingProviderTerminalAuditBodyWriter {
    length: usize,
}

impl CountingProviderTerminalAuditBodyWriter {
    const fn new() -> Self {
        Self { length: 0 }
    }
}

impl Write for CountingProviderTerminalAuditBodyWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.length = self
            .length
            .checked_add(bytes.len())
            .filter(|length| *length <= MAX_PROVIDER_TERMINAL_AUDIT_BYTES)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::WriteZero,
                    "terminal observer body exceeds fixed bound",
                )
            })?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PrivateProviderTerminalAuditObserverViewDecodeError {
    Malformed,
    Oversized,
    UnknownVersion,
}

impl Sys5I3ProviderTerminalAudit {
    fn observer_view_wire(&self) -> PrivateProviderTerminalAuditObserverViewWireRef<'_> {
        PrivateProviderTerminalAuditObserverViewWireRef {
            version: 2,
            profile_ref: self.profile_ref,
            schema_ref: self.schema_ref,
            role: self.role,
            request_descriptor: &self.request_descriptor,
            result_descriptor: &self.result_descriptor,
            rows: &self.rows,
            request_count: self.request_count,
            reserved_count: self.reserved_count,
            rejected_before_call_count: self.rejected_before_call_count,
            call_started_count: self.call_started_count,
            physical_adapter_entry_count: self.physical_adapter_entry_count,
            actual_read_count: self.actual_read_count,
            outcome_retained_count: self.outcome_retained_count,
            released_count: self.released_count,
            consume_count: self.consume_count,
            pending_count: self.pending_count,
        }
    }

    /// Check the fixed observer-view encoding with no body allocation. This
    /// leaves final bounded body construction to the codec after the caller
    /// has committed current M9 observation authorization.
    pub(crate) fn observer_view_body_fits_bound(&self) -> Result<(), ()> {
        if self.profile_ref != M9_I3_PROVIDER_TERMINAL_OBSERVATION_PROFILE_V2
            || self.schema_ref != M9_I3_PROVIDER_TERMINAL_OBSERVATION_SCHEMA_V2
            || !terminal_rows_are_well_formed(&self.rows, self.role)
        {
            return Err(());
        }
        let mut writer = CountingProviderTerminalAuditBodyWriter::new();
        serde_json::to_writer(&mut writer, &self.observer_view_wire()).map_err(|_| ())
    }

    pub(crate) fn encode_observer_view_body(&self) -> Result<Vec<u8>, ()> {
        let mut writer = BoundedProviderTerminalAuditBodyWriter::new().map_err(|_| ())?;
        serde_json::to_writer(&mut writer, &self.observer_view_wire()).map_err(|_| ())?;
        writer.into_body().map_err(|_| ())
    }
}

pub(crate) fn decode_untrusted_provider_terminal_audit_observer_view_body(
    bytes: &[u8],
) -> Result<
    Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate,
    PrivateProviderTerminalAuditObserverViewDecodeError,
> {
    if bytes.len() > MAX_PROVIDER_TERMINAL_AUDIT_BYTES {
        return Err(PrivateProviderTerminalAuditObserverViewDecodeError::Oversized);
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let wire = PrivateProviderTerminalAuditObserverViewWire::deserialize(&mut deserializer)
        .map_err(|_| PrivateProviderTerminalAuditObserverViewDecodeError::Malformed)?;
    deserializer
        .end()
        .map_err(|_| PrivateProviderTerminalAuditObserverViewDecodeError::Malformed)?;
    if wire.version != 2 {
        return Err(PrivateProviderTerminalAuditObserverViewDecodeError::UnknownVersion);
    }
    let candidate = Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
        profile_ref: wire.profile_ref,
        schema_ref: wire.schema_ref,
        role: wire.role,
        request_descriptor: wire.request_descriptor,
        result_descriptor: wire.result_descriptor,
        rows: wire.rows,
        counts: Sys5I3ProviderTerminalAuditCounts {
            request_count: wire.request_count,
            reserved_count: wire.reserved_count,
            rejected_before_call_count: wire.rejected_before_call_count,
            call_started_count: wire.call_started_count,
            physical_adapter_entry_count: wire.physical_adapter_entry_count,
            actual_read_count: wire.actual_read_count,
            outcome_retained_count: wire.outcome_retained_count,
            released_count: wire.released_count,
            consume_count: wire.consume_count,
            pending_count: wire.pending_count,
        },
    };
    candidate
        .has_well_formed_fixed_profile()
        .then_some(candidate)
        .ok_or(PrivateProviderTerminalAuditObserverViewDecodeError::Malformed)
}

impl Sys5I3UntrustedProviderTerminalAuditObserverViewCandidate {
    fn has_well_formed_fixed_profile(&self) -> bool {
        let counts = self.counts;
        self.profile_ref == M9_I3_PROVIDER_TERMINAL_OBSERVATION_PROFILE_V2
            && self.schema_ref == M9_I3_PROVIDER_TERMINAL_OBSERVATION_SCHEMA_V2
            && !self.rows.is_empty()
            && self.rows.len() <= MAX_PROVIDER_TERMINAL_AUDIT_ROWS
            && [
                counts.request_count,
                counts.reserved_count,
                counts.rejected_before_call_count,
                counts.call_started_count,
                counts.physical_adapter_entry_count,
                counts.actual_read_count,
                counts.outcome_retained_count,
                counts.released_count,
                counts.consume_count,
                counts.pending_count,
            ]
            .into_iter()
            .all(|count| count <= MAX_PROVIDER_LEDGER_ENTRIES)
            && counts.reserved_count <= counts.request_count
            && counts.rejected_before_call_count <= counts.reserved_count
            && counts.call_started_count <= counts.reserved_count
            && counts.physical_adapter_entry_count <= counts.call_started_count
            && counts.actual_read_count <= counts.physical_adapter_entry_count
            && counts.outcome_retained_count <= counts.call_started_count
            && counts.released_count <= counts.outcome_retained_count
            && counts.consume_count <= counts.request_count
            && counts.pending_count <= counts.request_count
            && descriptor_refs_are_well_formed(&self.request_descriptor)
            && descriptor_refs_are_well_formed(&self.result_descriptor)
            && terminal_rows_are_well_formed(&self.rows, self.role)
    }
}

fn descriptor_refs_are_well_formed(
    descriptor: &Sys5I3ProviderTerminalAuditCarrierDescriptorRefs,
) -> bool {
    [
        descriptor.source_ref(),
        descriptor.core_ref(),
        descriptor.source_artifact_ref(),
        descriptor.target_artifact_ref(),
        descriptor.generated_edge_ref(),
    ]
    .into_iter()
    .all(reference_shape_is_well_formed)
}

fn terminal_rows_are_well_formed(
    rows: &[Sys5I3ProviderTerminalAuditRow],
    role: Sys5I3ProviderChildRole,
) -> bool {
    let mut semantic_request_refs = std::collections::BTreeSet::new();
    rows.iter().all(|row| {
        if !semantic_request_ref_is_well_formed(row.semantic_request_ref())
            || !semantic_request_refs.insert(row.semantic_request_ref())
        {
            return false;
        }
        let mut expected_predecessors = vec![row.semantic_request_ref()];
        let mut occurrence_refs = std::collections::BTreeSet::new();
        let transport = [
            row.request_send_reservation_occurrence_refs(),
            row.request_send_completed_occurrence_refs(),
            row.request_receive_occurrence_refs(),
            row.result_send_reservation_occurrence_refs(),
            row.result_send_completed_occurrence_refs(),
            row.result_receive_occurrence_refs(),
        ];
        // The fixed v2 observer profile represents only the one normal
        // delivery chain. The second retained transport slot is reserved for
        // sealed fault conformance and has no truthful v2 chronology yet, so
        // neither a producer nor an untrusted decoded candidate may flatten
        // it into this normal ordered-predecessor projection.
        if transport
            .iter()
            .any(|slots| slots.iter().skip(1).any(Option::is_some))
        {
            return false;
        }
        if transport
            .iter()
            .flat_map(|slots| slots.iter())
            .any(|reference| {
                reference.as_deref().is_some_and(|reference| {
                    !provider_network_occurrence_ref_is_well_formed(reference)
                })
            })
        {
            return false;
        }
        for reference in transport
            .iter()
            .flat_map(|slots| slots.iter())
            .filter_map(Option::as_deref)
        {
            if !occurrence_refs.insert(reference) {
                return false;
            }
        }
        let local = [
            row.host_started_occurrence_ref(),
            row.adapter_entry_occurrence_ref(),
            row.adapter_read_occurrence_ref(),
            row.outcome_retained_occurrence_ref(),
            row.release_occurrence_ref(),
            row.local_consume_ref(),
        ];
        for reference in local.into_iter().flatten() {
            if !provider_local_occurrence_ref_is_well_formed(reference)
                || !occurrence_refs.insert(reference)
            {
                return false;
            }
        }
        let required = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                row.request_send_reservation_occurrence_refs()[0].is_some()
                    && row.request_send_completed_occurrence_refs()[0].is_some()
                    && row.result_receive_occurrence_refs()[0].is_some()
                    && row.local_consume_ref().is_some()
                    && row
                        .request_receive_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row
                        .result_send_reservation_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row
                        .result_send_completed_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row.host_started_occurrence_ref().is_none()
                    && row.adapter_entry_occurrence_ref().is_none()
                    && row.adapter_read_occurrence_ref().is_none()
                    && row.outcome_retained_occurrence_ref().is_none()
                    && row.release_occurrence_ref().is_none()
            }
            Sys5I3ProviderChildRole::Executor => {
                row.request_receive_occurrence_refs()[0].is_some()
                    && row.host_started_occurrence_ref().is_some()
                    && row.adapter_entry_occurrence_ref().is_some()
                    && row.outcome_retained_occurrence_ref().is_some()
                    && row.release_occurrence_ref().is_some()
                    && row.result_send_reservation_occurrence_refs()[0].is_some()
                    && row.result_send_completed_occurrence_refs()[0].is_some()
                    && row
                        .request_send_reservation_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row
                        .request_send_completed_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row
                        .result_receive_occurrence_refs()
                        .iter()
                        .all(Option::is_none)
                    && row.local_consume_ref().is_none()
            }
        };
        match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.request_send_reservation_occurrence_refs(),
                );
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.request_send_completed_occurrence_refs(),
                );
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.result_receive_occurrence_refs(),
                );
                if let Some(reference) = row.local_consume_ref() {
                    expected_predecessors.push(reference);
                }
            }
            Sys5I3ProviderChildRole::Executor => {
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.request_receive_occurrence_refs(),
                );
                for reference in [
                    row.host_started_occurrence_ref(),
                    row.adapter_entry_occurrence_ref(),
                    row.adapter_read_occurrence_ref(),
                    row.outcome_retained_occurrence_ref(),
                    row.release_occurrence_ref(),
                ]
                .into_iter()
                .flatten()
                {
                    expected_predecessors.push(reference);
                }
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.result_send_reservation_occurrence_refs(),
                );
                append_transport_occurrence_refs(
                    &mut expected_predecessors,
                    row.result_send_completed_occurrence_refs(),
                );
            }
        }
        required
            && row
                .ordered_predecessor_refs()
                .iter()
                .map(String::as_str)
                .eq(expected_predecessors)
    })
}

fn append_transport_occurrence_refs<'a>(
    predecessors: &mut Vec<&'a str>,
    slots: &'a [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
) {
    predecessors.extend(slots.iter().filter_map(Option::as_deref));
}

fn reference_shape_is_well_formed(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 512
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'-' | b'_' | b'.'))
}

fn semantic_request_ref_is_well_formed(value: &str) -> bool {
    value.starts_with("i3-provider-request-occurrence-sha256-v1:")
        && reference_shape_is_well_formed(value)
}

fn provider_local_occurrence_ref_is_well_formed(value: &str) -> bool {
    value.starts_with("i3-provider-terminal-local-occurrence-sha256-v2:")
        && reference_shape_is_well_formed(value)
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    tag = "kind",
    content = "payload",
    deny_unknown_fields
)]
enum PrivateProviderOutcome {
    Value(i64),
    ProviderResourceNotFound,
    ProviderPolicyDenied,
    ProviderInvalidResult,
    AdapterUnavailable,
}

impl PrivateProviderOutcome {
    fn terminal_class(&self) -> Sys5I3ProviderTerminalOutcomeClass {
        match self {
            Self::Value(_) => Sys5I3ProviderTerminalOutcomeClass::ValuePresent,
            Self::ProviderResourceNotFound => {
                Sys5I3ProviderTerminalOutcomeClass::ProviderResourceNotFound
            }
            Self::ProviderPolicyDenied => Sys5I3ProviderTerminalOutcomeClass::ProviderPolicyDenied,
            Self::ProviderInvalidResult => {
                Sys5I3ProviderTerminalOutcomeClass::ProviderInvalidResult
            }
            Self::AdapterUnavailable => Sys5I3ProviderTerminalOutcomeClass::AdapterUnavailable,
        }
    }
}

#[derive(Clone)]
struct PrivateProviderReadOutcome {
    outcome: PrivateProviderOutcome,
    #[cfg(test)]
    physical_adapter_entered: bool,
    actual_read: bool,
}

/// The six fixed provider transport fact kinds retained by the v2 terminal
/// projection.  The QUIC adapter allocates a reference from its run/session /
/// direction/ordinal before decoding; this runtime binds it only once the
/// matching admitted carrier reaches the corresponding transition.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys5I3PrivateProviderTransportOccurrenceKind {
    RequestSendReservation,
    RequestSendCompleted,
    RequestReceive,
    ResultSendReservation,
    ResultSendCompleted,
    ResultReceive,
}

/// Opaque adapter-produced occurrence material.  It intentionally has no
/// public constructor or reference getter: a decoded carrier, a test, or an
/// observer cannot manufacture transport provenance through this type.
#[derive(Clone)]
pub(crate) struct Sys5I3PrivateProviderTransportOccurrence {
    kind: Sys5I3PrivateProviderTransportOccurrenceKind,
    occurrence_ref: String,
}

impl std::fmt::Debug for Sys5I3PrivateProviderTransportOccurrence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3PrivateProviderTransportOccurrence(..)")
    }
}

impl Sys5I3PrivateProviderTransportOccurrence {
    pub(crate) fn from_private_quic(
        kind: Sys5I3PrivateProviderTransportOccurrenceKind,
        occurrence_ref: String,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        provider_network_occurrence_ref_is_well_formed(&occurrence_ref)
            .then_some(Self {
                kind,
                occurrence_ref,
            })
            .ok_or_else(rejected)
    }
}

/// A runtime-issued reservation for exactly one already-retained send fact.
/// Completing it records the write completion at the same fixed slot; it is
/// not a transport retry permit and exposes neither a carrier nor a raw ref.
pub(crate) struct Sys5I3PrivateProviderTransportWriteTicket {
    request_identity_ref: String,
    kind: Sys5I3PrivateProviderTransportOccurrenceKind,
    slot: usize,
    occurrence_ref: String,
}

/// A move-only, crate-private replay capsule for the one source-real request
/// already consumed by the requester. It is created only from that retained
/// ledger entry and never exposes carrier bytes to a caller.
#[cfg(feature = "i3-process-test-seams")]
pub(crate) struct Sys5I3PrivateProviderConsumedRequestReplay {
    request: Sys5I3ProviderRequestCarrier,
}

#[cfg(feature = "i3-process-test-seams")]
impl std::fmt::Debug for Sys5I3PrivateProviderConsumedRequestReplay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3PrivateProviderConsumedRequestReplay(..)")
    }
}

/// A move-only, crate-private replay capsule for the one executor result
/// retained by the actual completed call. It cannot be manufactured from
/// decoded bytes or a caller-selected outcome.
#[cfg(feature = "i3-process-test-seams")]
pub(crate) struct Sys5I3PrivateProviderReleasedResultReplay {
    result: Sys5I3ProviderResultCarrier,
}

#[cfg(feature = "i3-process-test-seams")]
impl std::fmt::Debug for Sys5I3PrivateProviderReleasedResultReplay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3PrivateProviderReleasedResultReplay(..)")
    }
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3PrivateProviderConsumedRequestReplay {
    pub(crate) fn request(&self) -> &Sys5I3ProviderRequestCarrier {
        &self.request
    }
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3PrivateProviderReleasedResultReplay {
    pub(crate) fn result(&self) -> &Sys5I3ProviderResultCarrier {
        &self.result
    }
}

impl std::fmt::Debug for Sys5I3PrivateProviderTransportWriteTicket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Sys5I3PrivateProviderTransportWriteTicket(..)")
    }
}

#[derive(Clone, Default)]
struct ProviderLedgerOccurrences {
    request_send_reservation: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    request_send_completed: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    request_receive: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_send_reservation: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_send_completed: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    result_receive: [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    host_started: Option<String>,
    adapter_entry: Option<String>,
    adapter_read: Option<String>,
    outcome_retained: Option<String>,
    released: Option<String>,
    local_consume: Option<String>,
}

#[derive(Clone)]
enum ProviderLedgerEntry {
    RequesterPending {
        request: Sys5I3ProviderRequestCarrier,
        occurrences: ProviderLedgerOccurrences,
    },
    ExecutorReserved {
        request: Sys5I3ProviderRequestCarrier,
        occurrences: ProviderLedgerOccurrences,
    },
    ExecutorCallStarted {
        request: Sys5I3ProviderRequestCarrier,
        occurrences: ProviderLedgerOccurrences,
    },
    ExecutorRejectedBeforeCall {
        request: Sys5I3ProviderRequestCarrier,
        occurrences: ProviderLedgerOccurrences,
    },
    ExecutorOutcomeRetained {
        request: Sys5I3ProviderRequestCarrier,
        read: PrivateProviderReadOutcome,
        occurrences: ProviderLedgerOccurrences,
    },
    ExecutorReleased {
        request: Sys5I3ProviderRequestCarrier,
        read: PrivateProviderReadOutcome,
        occurrences: ProviderLedgerOccurrences,
    },
    RequesterConsumed {
        request: Sys5I3ProviderRequestCarrier,
        outcome: PrivateProviderOutcome,
        occurrences: ProviderLedgerOccurrences,
    },
}

/// The one finite provider ledger for one installed child.  There is no
/// separate replay/outcome/tombstone budget and no reconnect reset path.
pub(crate) struct Sys5I3ProviderExecution {
    binding: M9I3PrivateProviderCarrierBinding,
    request_descriptor: Sys4ProviderCarrierDescriptor,
    result_descriptor: Sys4ProviderCarrierDescriptor,
    ledger: BTreeMap<String, ProviderLedgerEntry>,
    next_request_ordinal: u64,
    next_local_occurrence: u64,
    executing: bool,
    audit: Sys5I3ProviderTerminalAudit,
    #[cfg(all(test, feature = "i3-process-test-seams"))]
    revalidation_test_hook: Option<Sys5I3ProviderRevalidationTestHook>,
    #[cfg(all(test, feature = "i3-process-test-seams"))]
    revalidation_test_hook_was_configured: bool,
}

impl Sys5I3ProviderExecution {
    pub(crate) fn from_installed_profile(
        local_authority: &M9I3ReadOnlyProviderLocalAuthority,
        provider_local_fabric: &Sys4InstalledProviderLocalFabric,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        let binding = local_authority.provider_carrier_binding();
        let request_descriptor = provider_local_fabric
            .provider_request_descriptor()
            .map_err(|_| rejected())?;
        let result_descriptor = provider_local_fabric
            .provider_result_descriptor()
            .map_err(|_| rejected())?;
        if !descriptors_match_binding(&binding, &request_descriptor, &result_descriptor) {
            return Err(rejected());
        }
        Ok(Self {
            binding,
            request_descriptor,
            result_descriptor,
            ledger: BTreeMap::new(),
            next_request_ordinal: 0,
            next_local_occurrence: 0,
            executing: false,
            audit: Sys5I3ProviderTerminalAudit::empty(),
            #[cfg(all(test, feature = "i3-process-test-seams"))]
            revalidation_test_hook: None,
            #[cfg(all(test, feature = "i3-process-test-seams"))]
            revalidation_test_hook_was_configured: false,
        })
    }

    pub(crate) fn begin_request(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
    ) -> Result<Sys5I3ProviderRequestCarrier, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(rejected());
        }
        revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::RequestOrConsume,
        )?;
        self.ensure_capacity()?;
        let ordinal = self.next_request_ordinal;
        self.next_request_ordinal = self
            .next_request_ordinal
            .checked_add(1)
            .ok_or_else(resource_exhausted)?;
        let request_identity_ref =
            request_identity_ref(&self.binding, ordinal).map_err(|_| rejected())?;
        if self.ledger.contains_key(&request_identity_ref) {
            return Err(resource_exhausted());
        }
        let request = Sys5I3ProviderRequestCarrier {
            binding: self.binding.clone(),
            request_descriptor: self.request_descriptor.clone(),
            request_ordinal: ordinal,
            request_identity_ref: request_identity_ref.clone(),
        };
        self.ledger.insert(
            request_identity_ref.clone(),
            ProviderLedgerEntry::RequesterPending {
                request: request.clone(),
                occurrences: ProviderLedgerOccurrences::default(),
            },
        );
        self.audit.request_count = self.audit.request_count.saturating_add(1);
        Ok(request)
    }

    pub(crate) fn admit_request_and_execute(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        envelope: Option<&I3PrivateReadOnlyProviderResourceEnvelope>,
        request: Sys5I3ProviderRequestCarrier,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3ProcessRuntimeError> {
        self.admit_request_and_execute_with_transport_occurrence(
            role,
            local_authority,
            envelope,
            request,
            None,
        )
    }

    pub(crate) fn admit_request_and_execute_with_transport_occurrence(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        envelope: Option<&I3PrivateReadOnlyProviderResourceEnvelope>,
        request: Sys5I3ProviderRequestCarrier,
        request_receive: Option<Sys5I3PrivateProviderTransportOccurrence>,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::Executor || envelope.is_none() {
            return Err(rejected());
        }
        if request_receive.as_ref().is_some_and(|occurrence| {
            occurrence.kind != Sys5I3PrivateProviderTransportOccurrenceKind::RequestReceive
        }) {
            return Err(rejected());
        }
        if !request_matches_profile(&request, &self.binding, &self.request_descriptor) {
            return Err(request_binding_mismatch());
        }
        #[cfg(all(test, feature = "i3-process-test-seams"))]
        self.run_revalidation_test_hook(
            role,
            local_authority,
            Sys5I3ProviderRevalidationTestPoint::BeforeReserve,
        )?;
        revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::Host,
        )?;
        if self.executing {
            return Err(resource_exhausted());
        }
        // Fixed adapter input storage is reserved before the ledger reaches
        // CallStarted. No allocation is attempted after that commitment.
        let mut input_buffer = [0_u8; PROVIDER_READ_SENTINEL_BYTES];
        match self.ledger.get(&request.request_identity_ref) {
            Some(ProviderLedgerEntry::ExecutorReserved {
                request: retained, ..
            })
            | Some(ProviderLedgerEntry::ExecutorCallStarted {
                request: retained, ..
            })
            | Some(ProviderLedgerEntry::ExecutorOutcomeRetained {
                request: retained, ..
            })
            | Some(ProviderLedgerEntry::ExecutorReleased {
                request: retained, ..
            }) if retained == &request => {
                return Err(duplicate_request());
            }
            Some(ProviderLedgerEntry::ExecutorRejectedBeforeCall {
                request: retained, ..
            }) if retained == &request => {
                return Err(duplicate_request());
            }
            Some(_) => return Err(request_binding_mismatch()),
            None => self.ensure_capacity()?,
        }
        let mut reserved_occurrences = ProviderLedgerOccurrences::default();
        if let Some(occurrence) = request_receive {
            reserve_transport_occurrence_slot(
                &mut reserved_occurrences.request_receive,
                occurrence.occurrence_ref,
            )?;
        }
        self.ledger.insert(
            request.request_identity_ref.clone(),
            ProviderLedgerEntry::ExecutorReserved {
                request: request.clone(),
                occurrences: reserved_occurrences,
            },
        );
        self.audit.request_count = self.audit.request_count.saturating_add(1);
        self.audit.reserved_count = self.audit.reserved_count.saturating_add(1);
        let mut occurrences = self
            .entry_occurrences(&request.request_identity_ref)
            .ok_or_else(rejected)?
            .clone();

        // Revalidate after reservation and immediately before the committed
        // synchronous host crossing. There is no await, dispatch, or retry
        // gap between CallStarted and the bounded adapter read below.
        #[cfg(all(test, feature = "i3-process-test-seams"))]
        self.run_revalidation_test_hook(
            role,
            local_authority,
            Sys5I3ProviderRevalidationTestPoint::AfterReserveBeforeCallStarted,
        )?;
        if let Err(error) = revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::Host,
        ) {
            self.ledger.insert(
                request.request_identity_ref.clone(),
                ProviderLedgerEntry::ExecutorRejectedBeforeCall {
                    request,
                    occurrences,
                },
            );
            self.audit.rejected_before_call_count =
                self.audit.rejected_before_call_count.saturating_add(1);
            return Err(error);
        }
        occurrences.host_started =
            Some(self.reserve_local_occurrence_ref(&request.request_identity_ref, "host-started")?);
        self.ledger.insert(
            request.request_identity_ref.clone(),
            ProviderLedgerEntry::ExecutorCallStarted {
                request: request.clone(),
                occurrences: occurrences.clone(),
            },
        );
        self.executing = true;
        self.audit.call_started_count = self.audit.call_started_count.saturating_add(1);

        occurrences.adapter_entry = Some(
            self.reserve_local_occurrence_ref(&request.request_identity_ref, "adapter-entry")?,
        );

        let read = read_fixed_provider_resource(
            envelope.expect("executor envelope was checked before CallStarted"),
            &mut input_buffer,
            &mut self.audit,
        );
        self.executing = false;
        if read.actual_read {
            occurrences.adapter_read = Some(
                self.reserve_local_occurrence_ref(&request.request_identity_ref, "adapter-read")?,
            );
        }
        occurrences.outcome_retained = Some(
            self.reserve_local_occurrence_ref(&request.request_identity_ref, "outcome-retained")?,
        );
        self.ledger.insert(
            request.request_identity_ref.clone(),
            ProviderLedgerEntry::ExecutorOutcomeRetained {
                request: request.clone(),
                read: read.clone(),
                occurrences: occurrences.clone(),
            },
        );
        self.audit.outcome_retained_count = self.audit.outcome_retained_count.saturating_add(1);

        // The outcome remains retained even when the post-call release check
        // fails. It can neither prove no call nor authorize a later retry.
        #[cfg(all(test, feature = "i3-process-test-seams"))]
        self.run_revalidation_test_hook(
            role,
            local_authority,
            Sys5I3ProviderRevalidationTestPoint::AfterCallBeforeRelease,
        )?;
        revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::Host,
        )?;
        occurrences.released =
            Some(self.reserve_local_occurrence_ref(&request.request_identity_ref, "released")?);
        self.ledger.insert(
            request.request_identity_ref.clone(),
            ProviderLedgerEntry::ExecutorReleased {
                request: request.clone(),
                read: read.clone(),
                occurrences,
            },
        );
        self.audit.released_count = self.audit.released_count.saturating_add(1);
        Ok(Sys5I3ProviderResultCarrier {
            binding: self.binding.clone(),
            request_descriptor: self.request_descriptor.clone(),
            result_descriptor: self.result_descriptor.clone(),
            request_identity_ref: request.request_identity_ref,
            outcome: read.outcome,
        })
    }

    pub(crate) fn admit_result_and_consume(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        result: Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3ProcessRuntimeError> {
        self.admit_result_and_consume_with_transport_occurrence(role, local_authority, result, None)
    }

    pub(crate) fn admit_result_and_consume_with_transport_occurrence(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        result: Sys5I3ProviderResultCarrier,
        result_receive: Option<Sys5I3PrivateProviderTransportOccurrence>,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(rejected());
        }
        if result_receive.as_ref().is_some_and(|occurrence| {
            occurrence.kind != Sys5I3PrivateProviderTransportOccurrenceKind::ResultReceive
        }) {
            return Err(rejected());
        }
        if !result_matches_profile(
            &result,
            &self.binding,
            &self.request_descriptor,
            &self.result_descriptor,
        ) {
            return Err(request_binding_mismatch());
        }
        #[cfg(all(test, feature = "i3-process-test-seams"))]
        self.run_revalidation_test_hook(
            role,
            local_authority,
            Sys5I3ProviderRevalidationTestPoint::BeforeConsume,
        )?;
        revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::RequestOrConsume,
        )?;
        let (pending_request, mut occurrences) = match self.ledger.get(&result.request_identity_ref)
        {
            Some(ProviderLedgerEntry::RequesterPending {
                request,
                occurrences,
            }) if request.binding == self.binding
                && request.request_descriptor == self.request_descriptor =>
            {
                (request.clone(), occurrences.clone())
            }
            Some(ProviderLedgerEntry::RequesterConsumed {
                request, outcome, ..
            }) if request.binding == self.binding
                && request.request_descriptor == self.request_descriptor
                && request.request_identity_ref == result.request_identity_ref
                && outcome == &result.outcome =>
            {
                return Err(duplicate_request());
            }
            Some(ProviderLedgerEntry::RequesterConsumed { .. }) => {
                return Err(request_binding_mismatch());
            }
            Some(_) => return Err(request_binding_mismatch()),
            None => return Err(request_binding_mismatch()),
        };
        // The pending reservation already occupies the only ledger slot that
        // becomes this terminal fact; retain the decision before removing the
        // pending state, so a duplicate cannot reconsume it.
        let receipt = Sys5I3ProviderConsumeReceipt {
            request_identity_ref: result.request_identity_ref.clone(),
            outcome: result.outcome.clone(),
        };
        if let Some(occurrence) = result_receive {
            reserve_transport_occurrence_slot(
                &mut occurrences.result_receive,
                occurrence.occurrence_ref,
            )?;
        }
        occurrences.local_consume =
            Some(self.reserve_local_occurrence_ref(&result.request_identity_ref, "local-consume")?);
        self.ledger.insert(
            result.request_identity_ref.clone(),
            ProviderLedgerEntry::RequesterConsumed {
                request: pending_request,
                outcome: result.outcome,
                occurrences,
            },
        );
        self.audit.consume_count = self.audit.consume_count.saturating_add(1);
        Ok(receipt)
    }

    pub(crate) fn provider_carrier_binding(&self) -> &M9I3PrivateProviderCarrierBinding {
        &self.binding
    }

    pub(crate) fn reserve_request_send_transport_occurrence(
        &mut self,
        role: Sys5I3ProviderChildRole,
        request: &Sys5I3ProviderRequestCarrier,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::RequesterConsumer
            || occurrence.kind
                != Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendReservation
            || !request_matches_profile(request, &self.binding, &self.request_descriptor)
            || !matches!(
                self.ledger.get(&request.request_identity_ref),
                Some(ProviderLedgerEntry::RequesterPending { request: retained, .. })
                    if retained == request
            )
        {
            return Err(rejected());
        }
        let reservation_ref = occurrence.occurrence_ref.clone();
        let slot = reserve_transport_occurrence_slot(
            &mut self
                .entry_occurrences_mut(&request.request_identity_ref)
                .ok_or_else(rejected)?
                .request_send_reservation,
            occurrence.occurrence_ref,
        )?;
        Ok(Sys5I3PrivateProviderTransportWriteTicket {
            request_identity_ref: request.request_identity_ref.clone(),
            kind: Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted,
            slot,
            occurrence_ref: reservation_ref,
        })
    }

    /// Recover only the exact requester carrier already consumed through the
    /// first transport slot. This narrow replay capsule is for the sealed
    /// two-session conformance route; normal request sending remains limited
    /// to `RequesterPending` above.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn prepare_consumed_request_replay(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<Sys5I3PrivateProviderConsumedRequestReplay, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::RequesterConsumer {
            return Err(rejected());
        }
        let Some(ProviderLedgerEntry::RequesterConsumed {
            request,
            occurrences,
            ..
        }) = self.ledger.values().next()
        else {
            return Err(rejected());
        };
        if self.ledger.len() != 1
            || !request_matches_profile(request, &self.binding, &self.request_descriptor)
            || occurrences.request_send_reservation[0].is_none()
            || occurrences.request_send_completed[0].is_none()
            || occurrences.request_send_reservation[1].is_some()
            || occurrences.request_send_completed[1].is_some()
        {
            return Err(rejected());
        }
        Ok(Sys5I3PrivateProviderConsumedRequestReplay {
            request: request.clone(),
        })
    }

    /// Reserve the sole second transport slot for the exact retained consumed
    /// request. This does not create a pending request, a new semantic
    /// operation, or a second invocation allowance.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn reserve_consumed_request_replay_transport_occurrence(
        &mut self,
        role: Sys5I3ProviderChildRole,
        replay: &Sys5I3PrivateProviderConsumedRequestReplay,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::RequesterConsumer
            || occurrence.kind
                != Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendReservation
            || !request_matches_profile(replay.request(), &self.binding, &self.request_descriptor)
        {
            return Err(rejected());
        }
        let request_identity_ref = replay.request.request_identity_ref.clone();
        let occurrences = self
            .entry_occurrences_mut(&request_identity_ref)
            .ok_or_else(rejected)?;
        if occurrences.request_send_reservation[0].is_none()
            || occurrences.request_send_completed[0].is_none()
            || occurrences.request_send_reservation[1].is_some()
            || occurrences.request_send_completed[1].is_some()
        {
            return Err(rejected());
        }
        let reservation_ref = occurrence.occurrence_ref.clone();
        let slot = reserve_transport_occurrence_slot(
            &mut occurrences.request_send_reservation,
            occurrence.occurrence_ref,
        )?;
        if slot != 1 {
            return Err(rejected());
        }
        Ok(Sys5I3PrivateProviderTransportWriteTicket {
            request_identity_ref,
            kind: Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted,
            slot,
            occurrence_ref: reservation_ref,
        })
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn prepare_released_result_replay(
        &self,
        role: Sys5I3ProviderChildRole,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<Sys5I3PrivateProviderReleasedResultReplay, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::Executor
            || !result_matches_profile(
                result,
                &self.binding,
                &self.request_descriptor,
                &self.result_descriptor,
            )
            || !matches!(
                self.ledger.get(&result.request_identity_ref),
                Some(ProviderLedgerEntry::ExecutorReleased { request, read, .. })
                    if request.request_identity_ref == result.request_identity_ref
                        && read.outcome == result.outcome
            )
        {
            return Err(rejected());
        }
        Ok(Sys5I3PrivateProviderReleasedResultReplay {
            result: result.clone(),
        })
    }

    pub(crate) fn reserve_result_send_transport_occurrence(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        result: &Sys5I3ProviderResultCarrier,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<Sys5I3PrivateProviderTransportWriteTicket, Sys5I3ProcessRuntimeError> {
        if role != Sys5I3ProviderChildRole::Executor
            || occurrence.kind
                != Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendReservation
            || !result_matches_profile(
                result,
                &self.binding,
                &self.request_descriptor,
                &self.result_descriptor,
            )
            || !matches!(
                self.ledger.get(&result.request_identity_ref),
                Some(ProviderLedgerEntry::ExecutorReleased { request, read, .. })
                    if request.request_identity_ref == result.request_identity_ref
                        && read.outcome == result.outcome
            )
        {
            return Err(rejected());
        }
        // A retained result is not a permanent write permit. Revalidate the
        // executor's actual M9 host/effect lineage immediately before the
        // first provider result-send reservation; a later transport slice may
        // hold this result across sessions, but cannot use stale authority.
        revalidate_provider_use(
            local_authority,
            &self.binding,
            M9I3ReadOnlyProviderRevalidationUse::Host,
        )?;
        let reservation_ref = occurrence.occurrence_ref.clone();
        let slot = reserve_transport_occurrence_slot(
            &mut self
                .entry_occurrences_mut(&result.request_identity_ref)
                .ok_or_else(rejected)?
                .result_send_reservation,
            occurrence.occurrence_ref,
        )?;
        Ok(Sys5I3PrivateProviderTransportWriteTicket {
            request_identity_ref: result.request_identity_ref.clone(),
            kind: Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted,
            slot,
            occurrence_ref: reservation_ref,
        })
    }

    pub(crate) fn complete_provider_transport_write(
        &mut self,
        role: Sys5I3ProviderChildRole,
        ticket: Sys5I3PrivateProviderTransportWriteTicket,
        occurrence: Sys5I3PrivateProviderTransportOccurrence,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let expected = match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted
            }
            Sys5I3ProviderChildRole::Executor => {
                Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted
            }
        };
        if ticket.kind != expected || occurrence.kind != expected {
            return Err(rejected());
        }
        let occurrences = self
            .entry_occurrences_mut(&ticket.request_identity_ref)
            .ok_or_else(rejected)?;
        let reservation_slots = match expected {
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted => {
                &occurrences.request_send_reservation
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted => {
                &occurrences.result_send_reservation
            }
            _ => return Err(rejected()),
        };
        if reservation_slots
            .get(ticket.slot)
            .and_then(Option::as_deref)
            != Some(ticket.occurrence_ref.as_str())
        {
            return Err(rejected());
        }
        let slots = match expected {
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted => {
                &mut occurrences.request_send_completed
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted => {
                &mut occurrences.result_send_completed
            }
            _ => return Err(rejected()),
        };
        complete_transport_occurrence_slot(slots, ticket.slot, occurrence.occurrence_ref)
    }

    fn entry_occurrences(&self, request_identity_ref: &str) -> Option<&ProviderLedgerOccurrences> {
        match self.ledger.get(request_identity_ref)? {
            ProviderLedgerEntry::RequesterPending { occurrences, .. }
            | ProviderLedgerEntry::ExecutorReserved { occurrences, .. }
            | ProviderLedgerEntry::ExecutorCallStarted { occurrences, .. }
            | ProviderLedgerEntry::ExecutorRejectedBeforeCall { occurrences, .. }
            | ProviderLedgerEntry::ExecutorOutcomeRetained { occurrences, .. }
            | ProviderLedgerEntry::ExecutorReleased { occurrences, .. }
            | ProviderLedgerEntry::RequesterConsumed { occurrences, .. } => Some(occurrences),
        }
    }

    fn entry_occurrences_mut(
        &mut self,
        request_identity_ref: &str,
    ) -> Option<&mut ProviderLedgerOccurrences> {
        match self.ledger.get_mut(request_identity_ref)? {
            ProviderLedgerEntry::RequesterPending { occurrences, .. }
            | ProviderLedgerEntry::ExecutorReserved { occurrences, .. }
            | ProviderLedgerEntry::ExecutorCallStarted { occurrences, .. }
            | ProviderLedgerEntry::ExecutorRejectedBeforeCall { occurrences, .. }
            | ProviderLedgerEntry::ExecutorOutcomeRetained { occurrences, .. }
            | ProviderLedgerEntry::ExecutorReleased { occurrences, .. }
            | ProviderLedgerEntry::RequesterConsumed { occurrences, .. } => Some(occurrences),
        }
    }

    fn reserve_local_occurrence_ref(
        &mut self,
        request_identity_ref: &str,
        event: &str,
    ) -> Result<String, Sys5I3ProcessRuntimeError> {
        let ordinal = self.next_local_occurrence;
        self.next_local_occurrence = self
            .next_local_occurrence
            .checked_add(1)
            .ok_or_else(resource_exhausted)?;
        Ok(provider_local_occurrence_ref(
            request_identity_ref,
            event,
            ordinal,
        ))
    }

    /// Check only the finite retained terminal state required before the
    /// sealed installed-child observer conformance route runs. This is not a
    /// general audit query and exposes no ledger row, request, result, or
    /// authority fact.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_completed_terminal_facts_for_role(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> bool {
        match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                self.audit.request_count == 1
                    && self.audit.consume_count == 1
                    && self.ledger.len() == 1
                    && self
                        .ledger
                        .values()
                        .all(|entry| matches!(entry, ProviderLedgerEntry::RequesterConsumed { .. }))
            }
            Sys5I3ProviderChildRole::Executor => {
                self.audit.request_count == 1
                    && self.audit.reserved_count == 1
                    && self.audit.call_started_count == 1
                    && self.audit.outcome_retained_count == 1
                    && self.audit.released_count == 1
                    && self.ledger.len() == 1
                    && self
                        .ledger
                        .values()
                        .all(|entry| matches!(entry, ProviderLedgerEntry::ExecutorReleased { .. }))
            }
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_exact_requester_pending_facts(&self) -> bool {
        self.audit.request_count == 1
            && self.audit.consume_count == 0
            && self.ledger.len() == 1
            && matches!(
                self.ledger.values().next(),
                Some(ProviderLedgerEntry::RequesterPending { .. })
            )
    }

    /// Check the exact currently pending requester carrier without admitting
    /// or consuming it. The sealed two-session conformance route uses this
    /// only after the adapter completed a real result-frame read and before
    /// discarding that frame at the semantic boundary; it creates no receive
    /// occurrence, receipt, or result authority.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn matches_exact_requester_pending_result(
        &self,
        result: &Sys5I3ProviderResultCarrier,
    ) -> bool {
        result_matches_profile(
            result,
            &self.binding,
            &self.request_descriptor,
            &self.result_descriptor,
        ) && matches!(
            self.ledger.get(&result.request_identity_ref),
            Some(ProviderLedgerEntry::RequesterPending { request, .. })
                if request.binding == self.binding
                    && request.request_descriptor == self.request_descriptor
                    && request.request_identity_ref == result.request_identity_ref
        )
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_exact_released_executor_facts(&self) -> bool {
        self.audit.request_count == 1
            && self.audit.reserved_count == 1
            && self.audit.call_started_count == 1
            && self.audit.outcome_retained_count == 1
            && self.audit.released_count == 1
            && self.ledger.len() == 1
            && matches!(
                self.ledger.values().next(),
                Some(ProviderLedgerEntry::ExecutorReleased { .. })
            )
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_completed_transport_slot(
        &self,
        role: Sys5I3ProviderChildRole,
        result: &Sys5I3ProviderResultCarrier,
        slot: usize,
    ) -> bool {
        let Some(occurrences) = self.entry_occurrences(&result.request_identity_ref) else {
            return false;
        };
        role == Sys5I3ProviderChildRole::Executor
            && result_matches_profile(
                result,
                &self.binding,
                &self.request_descriptor,
                &self.result_descriptor,
            )
            && occurrences
                .result_send_reservation
                .get(slot)
                .is_some_and(Option::is_some)
            && occurrences
                .result_send_completed
                .get(slot)
                .is_some_and(Option::is_some)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_empty_result_transport_slot(
        &self,
        result: &Sys5I3ProviderResultCarrier,
        slot: usize,
    ) -> bool {
        let Some(occurrences) = self.entry_occurrences(&result.request_identity_ref) else {
            return false;
        };
        result_matches_profile(
            result,
            &self.binding,
            &self.request_descriptor,
            &self.result_descriptor,
        ) && occurrences
            .result_send_reservation
            .get(slot)
            .is_some_and(Option::is_none)
            && occurrences
                .result_send_completed
                .get(slot)
                .is_some_and(Option::is_none)
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_exact_released_request(
        &self,
        request: &Sys5I3ProviderRequestCarrier,
    ) -> bool {
        matches!(
            self.ledger.get(&request.request_identity_ref),
            Some(ProviderLedgerEntry::ExecutorReleased { request: retained, .. })
                if retained == request
        )
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_exact_consumed_result(&self, result: &Sys5I3ProviderResultCarrier) -> bool {
        matches!(
            self.ledger.get(&result.request_identity_ref),
            Some(ProviderLedgerEntry::RequesterConsumed { request, outcome, .. })
                if request.binding == self.binding
                    && request.request_descriptor == self.request_descriptor
                    && request.request_identity_ref == result.request_identity_ref
                    && outcome == &result.outcome
        )
    }

    /// Exercise the exact next-use boundary after a sealed conformance
    /// profile retires effect authority. A rejects before creating a new
    /// requester ledger entry; B rejects at its real host-use revalidation
    /// before any reservation or adapter entry. This is not a caller-selected
    /// request or a fabricated outcome path.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn attempt_post_effect_retirement_provider_use(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        match role {
            Sys5I3ProviderChildRole::RequesterConsumer => {
                self.begin_request(role, local_authority).map(|_| ())
            }
            Sys5I3ProviderChildRole::Executor => revalidate_provider_use(
                local_authority,
                &self.binding,
                M9I3ReadOnlyProviderRevalidationUse::Host,
            ),
        }
    }

    #[cfg(all(test, feature = "i3-process-test-seams"))]
    pub(crate) fn configure_revalidation_test_retirement(
        &mut self,
        role: Sys5I3ProviderChildRole,
        point: Sys5I3ProviderRevalidationTestPoint,
        retirement: Sys5I3ProviderRevalidationTestRetirement,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let role_matches_point = match point {
            Sys5I3ProviderRevalidationTestPoint::BeforeReserve
            | Sys5I3ProviderRevalidationTestPoint::AfterReserveBeforeCallStarted
            | Sys5I3ProviderRevalidationTestPoint::AfterCallBeforeRelease => {
                role == Sys5I3ProviderChildRole::Executor
            }
            Sys5I3ProviderRevalidationTestPoint::BeforeConsume => {
                role == Sys5I3ProviderChildRole::RequesterConsumer
            }
        };
        // A fixture can select one real retirement exactly once. Consuming
        // the hook intentionally leaves this monotone bit set, so a test
        // cannot turn the local execution boundary into a rearmable fault
        // scheduler.
        if !role_matches_point || self.revalidation_test_hook_was_configured {
            return Err(rejected());
        }
        self.revalidation_test_hook =
            Some(Sys5I3ProviderRevalidationTestHook { point, retirement });
        self.revalidation_test_hook_was_configured = true;
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn test_only_ledger_facts(&self) -> Sys5I3ProviderLedgerTestFacts {
        let mut facts = Sys5I3ProviderLedgerTestFacts {
            capacity: MAX_PROVIDER_LEDGER_ENTRIES,
            retained_entry_count: self.ledger.len(),
            requester_pending_count: 0,
            executor_reserved_count: 0,
            executor_call_started_count: 0,
            executor_rejected_before_call_count: 0,
            executor_outcome_retained_count: 0,
            executor_released_count: 0,
            requester_consumed_count: 0,
        };
        for entry in self.ledger.values() {
            match entry {
                ProviderLedgerEntry::RequesterPending { .. } => {
                    facts.requester_pending_count += 1;
                }
                ProviderLedgerEntry::ExecutorReserved { .. } => {
                    facts.executor_reserved_count += 1;
                }
                ProviderLedgerEntry::ExecutorCallStarted { .. } => {
                    facts.executor_call_started_count += 1;
                }
                ProviderLedgerEntry::ExecutorRejectedBeforeCall { .. } => {
                    facts.executor_rejected_before_call_count += 1;
                }
                ProviderLedgerEntry::ExecutorOutcomeRetained { .. } => {
                    facts.executor_outcome_retained_count += 1;
                }
                ProviderLedgerEntry::ExecutorReleased { .. } => {
                    facts.executor_released_count += 1;
                }
                ProviderLedgerEntry::RequesterConsumed { .. } => {
                    facts.requester_consumed_count += 1;
                }
            }
        }
        facts
    }

    #[cfg(all(test, feature = "i3-process-test-seams"))]
    fn run_revalidation_test_hook(
        &mut self,
        role: Sys5I3ProviderChildRole,
        local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
        point: Sys5I3ProviderRevalidationTestPoint,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let Some(hook) = self.revalidation_test_hook else {
            return Ok(());
        };
        if hook.point != point {
            return Ok(());
        }
        self.revalidation_test_hook = None;
        let expected_role = match point {
            Sys5I3ProviderRevalidationTestPoint::BeforeReserve
            | Sys5I3ProviderRevalidationTestPoint::AfterReserveBeforeCallStarted
            | Sys5I3ProviderRevalidationTestPoint::AfterCallBeforeRelease => {
                Sys5I3ProviderChildRole::Executor
            }
            Sys5I3ProviderRevalidationTestPoint::BeforeConsume => {
                Sys5I3ProviderChildRole::RequesterConsumer
            }
        };
        if role != expected_role {
            return Err(rejected());
        }
        let retired = match hook.retirement {
            Sys5I3ProviderRevalidationTestRetirement::EffectCapability => {
                local_authority.retire_effect_authorization()
            }
            Sys5I3ProviderRevalidationTestRetirement::EffectWitness => {
                local_authority.retire_effect_witness_authorization()
            }
            Sys5I3ProviderRevalidationTestRetirement::Membership => {
                local_authority.retire_membership_authorization()
            }
        };
        retired.map_err(|_| rejected())
    }

    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn verifies_fixture_post_consume_assertion(
        &self,
        receipt: &Sys5I3ProviderConsumeReceipt,
        expected: I3PrivateReadOnlyProviderFixtureAssertion,
    ) -> Result<Sys5I3ProviderFixtureAssertionCompletion, Sys5I3ProcessRuntimeError> {
        let retained = match self.ledger.get(&receipt.request_identity_ref) {
            Some(ProviderLedgerEntry::RequesterConsumed {
                request, outcome, ..
            }) if request.binding == self.binding
                && request.request_descriptor == self.request_descriptor
                && outcome == &receipt.outcome =>
            {
                outcome
            }
            _ => return Err(request_binding_mismatch()),
        };
        fixture_assertion_matches(expected, retained)
            .then_some(Sys5I3ProviderFixtureAssertionCompletion { _private: () })
            .ok_or_else(rejected)
    }

    /// Construct the fixed reference-only terminal projection from retained
    /// provider facts. The caller must obtain the separate M9 Observation
    /// permit before invoking this method; this execution component owns the
    /// fixed profile and encoded-size cap, not an observer-policy issuer.
    pub(crate) fn terminal_audit_projection(
        &self,
        role: Sys5I3ProviderChildRole,
    ) -> Result<Sys5I3ProviderTerminalAudit, ()> {
        let mut rows = self
            .ledger
            .values()
            .map(terminal_audit_row_for_ledger_entry)
            .collect::<Vec<_>>();
        rows.sort_by_key(|(ordinal, _)| *ordinal);
        let rows = rows.into_iter().map(|(_, row)| row).collect::<Vec<_>>();
        if rows.is_empty() || rows.len() > MAX_PROVIDER_TERMINAL_AUDIT_ROWS {
            return Err(());
        }
        let audit = Sys5I3ProviderTerminalAudit {
            profile_ref: M9_I3_PROVIDER_TERMINAL_OBSERVATION_PROFILE_V2,
            schema_ref: M9_I3_PROVIDER_TERMINAL_OBSERVATION_SCHEMA_V2,
            role,
            request_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs::from_descriptor(
                &self.request_descriptor,
            ),
            result_descriptor: Sys5I3ProviderTerminalAuditCarrierDescriptorRefs::from_descriptor(
                &self.result_descriptor,
            ),
            rows,
            request_count: self.audit.request_count,
            reserved_count: self.audit.reserved_count,
            rejected_before_call_count: self.audit.rejected_before_call_count,
            call_started_count: self.audit.call_started_count,
            physical_adapter_entry_count: self.audit.physical_adapter_entry_count,
            actual_read_count: self.audit.actual_read_count,
            outcome_retained_count: self.audit.outcome_retained_count,
            released_count: self.audit.released_count,
            consume_count: self.audit.consume_count,
            pending_count: self
                .ledger
                .values()
                .filter(|entry| matches!(entry, ProviderLedgerEntry::RequesterPending { .. }))
                .count(),
        };
        audit.observer_view_body_fits_bound()?;
        Ok(audit)
    }

    fn ensure_capacity(&self) -> Result<(), Sys5I3ProcessRuntimeError> {
        (self.ledger.len() < MAX_PROVIDER_LEDGER_ENTRIES)
            .then_some(())
            .ok_or_else(resource_exhausted)
    }
}

fn terminal_audit_row_for_ledger_entry(
    entry: &ProviderLedgerEntry,
) -> (u64, Sys5I3ProviderTerminalAuditRow) {
    let (request, occurrences, outcome) = match entry {
        ProviderLedgerEntry::RequesterPending {
            request,
            occurrences,
        }
        | ProviderLedgerEntry::ExecutorReserved {
            request,
            occurrences,
        }
        | ProviderLedgerEntry::ExecutorCallStarted {
            request,
            occurrences,
        }
        | ProviderLedgerEntry::ExecutorRejectedBeforeCall {
            request,
            occurrences,
        } => (request, occurrences, None),
        ProviderLedgerEntry::ExecutorOutcomeRetained {
            request,
            read,
            occurrences,
        }
        | ProviderLedgerEntry::ExecutorReleased {
            request,
            read,
            occurrences,
        } => (request, occurrences, Some(&read.outcome)),
        ProviderLedgerEntry::RequesterConsumed {
            request,
            outcome,
            occurrences,
        } => (request, occurrences, Some(outcome)),
    };
    let semantic_request_ref = request.request_identity_ref.clone();
    let mut ordered_predecessor_refs = vec![semantic_request_ref.clone()];
    for reference in occurrences
        .request_send_reservation
        .iter()
        .chain(occurrences.request_send_completed.iter())
        .chain(occurrences.request_receive.iter())
        .chain([
            &occurrences.host_started,
            &occurrences.adapter_entry,
            &occurrences.adapter_read,
            &occurrences.outcome_retained,
            &occurrences.released,
        ])
        .chain(occurrences.result_send_reservation.iter())
        .chain(occurrences.result_send_completed.iter())
        .chain(occurrences.result_receive.iter())
        .chain([&occurrences.local_consume])
        .filter_map(Option::as_ref)
    {
        ordered_predecessor_refs.push(reference.clone());
    }
    (
        request.request_ordinal,
        Sys5I3ProviderTerminalAuditRow {
            semantic_request_ref,
            host_started_occurrence_ref: occurrences.host_started.clone(),
            outcome_retained_occurrence_ref: occurrences.outcome_retained.clone(),
            release_occurrence_ref: occurrences.released.clone(),
            local_consume_ref: occurrences.local_consume.clone(),
            adapter_entry_occurrence_ref: occurrences.adapter_entry.clone(),
            adapter_read_occurrence_ref: occurrences.adapter_read.clone(),
            request_send_reservation_occurrence_refs: occurrences.request_send_reservation.clone(),
            request_send_completed_occurrence_refs: occurrences.request_send_completed.clone(),
            request_receive_occurrence_refs: occurrences.request_receive.clone(),
            result_send_reservation_occurrence_refs: occurrences.result_send_reservation.clone(),
            result_send_completed_occurrence_refs: occurrences.result_send_completed.clone(),
            result_receive_occurrence_refs: occurrences.result_receive.clone(),
            ordered_predecessor_refs,
            outcome_class: outcome.map(PrivateProviderOutcome::terminal_class),
        },
    )
}

fn reserve_transport_occurrence_slot(
    slots: &mut [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    occurrence_ref: String,
) -> Result<usize, Sys5I3ProcessRuntimeError> {
    if !provider_network_occurrence_ref_is_well_formed(&occurrence_ref) {
        return Err(rejected());
    }
    let Some((slot, target)) = slots
        .iter_mut()
        .enumerate()
        .find(|(_, slot)| slot.is_none())
    else {
        return Err(resource_exhausted());
    };
    *target = Some(occurrence_ref);
    Ok(slot)
}

fn complete_transport_occurrence_slot(
    slots: &mut [Option<String>; PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND],
    slot: usize,
    occurrence_ref: String,
) -> Result<(), Sys5I3ProcessRuntimeError> {
    if !provider_network_occurrence_ref_is_well_formed(&occurrence_ref)
        || slot >= PROVIDER_TRANSPORT_OCCURRENCES_PER_KIND
        || slots[slot].is_some()
    {
        return Err(rejected());
    }
    slots[slot] = Some(occurrence_ref);
    Ok(())
}

/// Domain-separate real local transitions from the admitted semantic request
/// reference and one retained local ordinal. This never hashes a provider
/// value, native path, source text, credential, capability, or witness.
fn provider_local_occurrence_ref(
    request_identity_ref: &str,
    event_kind: &str,
    ordinal: u64,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/provider/terminal-local-occurrence/v2\0");
    hasher.update((request_identity_ref.len() as u64).to_be_bytes());
    hasher.update(request_identity_ref.as_bytes());
    hasher.update((event_kind.len() as u64).to_be_bytes());
    hasher.update(event_kind.as_bytes());
    hasher.update(ordinal.to_be_bytes());
    format!(
        "i3-provider-terminal-local-occurrence-sha256-v2:{:x}",
        hasher.finalize()
    )
}

fn provider_network_occurrence_ref_is_well_formed(value: &str) -> bool {
    value.starts_with("i3-provider-network-occurrence-sha256-v1:")
        && reference_shape_is_well_formed(value)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PrivateProviderRequestCarrierSnapshot {
    version: u8,
    binding: M9I3PrivateProviderCarrierBinding,
    request_descriptor: Sys4ProviderCarrierDescriptor,
    request_ordinal: u64,
    request_identity_ref: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PrivateProviderResultCarrierSnapshot {
    version: u8,
    binding: M9I3PrivateProviderCarrierBinding,
    request_descriptor: Sys4ProviderCarrierDescriptor,
    result_descriptor: Sys4ProviderCarrierDescriptor,
    request_identity_ref: String,
    outcome: PrivateProviderOutcome,
}

impl Sys5I3ProviderRequestCarrier {
    pub(crate) fn private_snapshot(&self) -> PrivateProviderRequestCarrierSnapshot {
        PrivateProviderRequestCarrierSnapshot {
            version: PRIVATE_PROVIDER_CARRIER_VERSION,
            binding: self.binding.clone(),
            request_descriptor: self.request_descriptor.clone(),
            request_ordinal: self.request_ordinal,
            request_identity_ref: self.request_identity_ref.clone(),
        }
    }

    pub(crate) fn from_private_snapshot(
        snapshot: PrivateProviderRequestCarrierSnapshot,
    ) -> Result<Self, ()> {
        if snapshot.version != PRIVATE_PROVIDER_CARRIER_VERSION
            || snapshot.request_identity_ref.is_empty()
            || request_identity_ref(&snapshot.binding, snapshot.request_ordinal)?
                != snapshot.request_identity_ref
        {
            return Err(());
        }
        Ok(Self {
            binding: snapshot.binding,
            request_descriptor: snapshot.request_descriptor,
            request_ordinal: snapshot.request_ordinal,
            request_identity_ref: snapshot.request_identity_ref,
        })
    }
}

impl Sys5I3ProviderResultCarrier {
    pub(crate) fn private_snapshot(&self) -> PrivateProviderResultCarrierSnapshot {
        PrivateProviderResultCarrierSnapshot {
            version: PRIVATE_PROVIDER_CARRIER_VERSION,
            binding: self.binding.clone(),
            request_descriptor: self.request_descriptor.clone(),
            result_descriptor: self.result_descriptor.clone(),
            request_identity_ref: self.request_identity_ref.clone(),
            outcome: self.outcome.clone(),
        }
    }

    pub(crate) fn from_private_snapshot(
        snapshot: PrivateProviderResultCarrierSnapshot,
    ) -> Result<Self, ()> {
        if snapshot.version != PRIVATE_PROVIDER_CARRIER_VERSION
            || snapshot.request_identity_ref.is_empty()
        {
            return Err(());
        }
        Ok(Self {
            binding: snapshot.binding,
            request_descriptor: snapshot.request_descriptor,
            result_descriptor: snapshot.result_descriptor,
            request_identity_ref: snapshot.request_identity_ref,
            outcome: snapshot.outcome,
        })
    }
}

fn descriptors_match_binding(
    binding: &M9I3PrivateProviderCarrierBinding,
    request: &Sys4ProviderCarrierDescriptor,
    result: &Sys4ProviderCarrierDescriptor,
) -> bool {
    request.operation_id() == binding.operation()
        && request.source_locus() == binding.requester_locus()
        && request.target_locus() == binding.executor_locus()
        && result.operation_id() == binding.operation()
        && result.source_locus() == binding.executor_locus()
        && result.target_locus() == binding.result_consumer_locus()
}

fn request_matches_profile(
    request: &Sys5I3ProviderRequestCarrier,
    binding: &M9I3PrivateProviderCarrierBinding,
    request_descriptor: &Sys4ProviderCarrierDescriptor,
) -> bool {
    !request.request_identity_ref.is_empty()
        && request.binding == *binding
        && request.request_descriptor == *request_descriptor
        && request_identity_ref(&request.binding, request.request_ordinal)
            .is_ok_and(|identity| identity == request.request_identity_ref)
}

fn result_matches_profile(
    result: &Sys5I3ProviderResultCarrier,
    binding: &M9I3PrivateProviderCarrierBinding,
    request_descriptor: &Sys4ProviderCarrierDescriptor,
    result_descriptor: &Sys4ProviderCarrierDescriptor,
) -> bool {
    !result.request_identity_ref.is_empty()
        && result.binding == *binding
        && result.request_descriptor == *request_descriptor
        && result.result_descriptor == *result_descriptor
}

fn read_fixed_provider_resource(
    envelope: &I3PrivateReadOnlyProviderResourceEnvelope,
    bytes: &mut [u8; PROVIDER_READ_SENTINEL_BYTES],
    audit: &mut Sys5I3ProviderTerminalAudit,
) -> PrivateProviderReadOutcome {
    audit.physical_adapter_entry_count = audit.physical_adapter_entry_count.saturating_add(1);
    let resource_path = envelope.private_resource_path();
    // Metadata and native open happen only after CallStarted. The no-follow
    // open below closes the symlink race rather than treating a pre-open
    // metadata result as a resource binding proof.
    match fs::symlink_metadata(&resource_path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {}
        Ok(_) => return provider_read_outcome(PrivateProviderOutcome::ProviderPolicyDenied, false),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderResourceNotFound, false);
        }
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderPolicyDenied, false);
        }
        Err(_) => return provider_read_outcome(PrivateProviderOutcome::AdapterUnavailable, false),
    }
    let mut file = match open_regular_no_follow(&resource_path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderResourceNotFound, false);
        }
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderPolicyDenied, false);
        }
        Err(_) => return provider_read_outcome(PrivateProviderOutcome::AdapterUnavailable, false),
    };
    let metadata = match file.metadata() {
        Ok(metadata) if metadata.file_type().is_file() => metadata,
        Ok(_) => return provider_read_outcome(PrivateProviderOutcome::ProviderPolicyDenied, false),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderResourceNotFound, false);
        }
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            return provider_read_outcome(PrivateProviderOutcome::ProviderPolicyDenied, false);
        }
        Err(_) => return provider_read_outcome(PrivateProviderOutcome::AdapterUnavailable, false),
    };
    let _ = metadata;
    read_bounded_provider_stream(&mut file, bytes, audit)
}

/// Classify the one bounded physical read after the caller has entered the
/// adapter and performed its resource lookup/open/kind checks. Keeping this
/// loop separate preserves the exact successful-read accounting for both the
/// real `File` path and focused local I/O failure evidence.
fn read_bounded_provider_stream<R: Read>(
    reader: &mut R,
    bytes: &mut [u8; PROVIDER_READ_SENTINEL_BYTES],
    audit: &mut Sys5I3ProviderTerminalAudit,
) -> PrivateProviderReadOutcome {
    let mut used = 0;
    // `actual_read` means a successful bounded `Read` response, including an
    // EOF response. It records host activity once per invocation—not bytes or
    // chunks—and remains true if a later read reports an error.
    let mut actual_read_observed = false;
    loop {
        match reader.read(&mut bytes[used..]) {
            Ok(0) => {
                observe_actual_read(audit, &mut actual_read_observed);
                break;
            }
            Ok(read) => {
                observe_actual_read(audit, &mut actual_read_observed);
                used += read;
                if used == PROVIDER_READ_SENTINEL_BYTES {
                    return provider_read_outcome(
                        PrivateProviderOutcome::ProviderInvalidResult,
                        actual_read_observed,
                    );
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return provider_read_outcome(
                    PrivateProviderOutcome::ProviderResourceNotFound,
                    actual_read_observed,
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                return provider_read_outcome(
                    PrivateProviderOutcome::ProviderPolicyDenied,
                    actual_read_observed,
                );
            }
            Err(_) => {
                return provider_read_outcome(
                    PrivateProviderOutcome::AdapterUnavailable,
                    actual_read_observed,
                );
            }
        }
    }
    provider_read_outcome(
        parse_canonical_i64(&bytes[..used])
            .map(PrivateProviderOutcome::Value)
            .unwrap_or(PrivateProviderOutcome::ProviderInvalidResult),
        actual_read_observed,
    )
}

fn observe_actual_read(audit: &mut Sys5I3ProviderTerminalAudit, observed: &mut bool) {
    if !*observed {
        audit.actual_read_count = audit.actual_read_count.saturating_add(1);
        *observed = true;
    }
}

fn provider_read_outcome(
    outcome: PrivateProviderOutcome,
    actual_read: bool,
) -> PrivateProviderReadOutcome {
    PrivateProviderReadOutcome {
        outcome,
        #[cfg(test)]
        physical_adapter_entered: true,
        actual_read,
    }
}

#[cfg(feature = "i3-process-test-seams")]
fn fixture_assertion_matches(
    expected: I3PrivateReadOnlyProviderFixtureAssertion,
    actual: &PrivateProviderOutcome,
) -> bool {
    matches!(
        (expected, actual),
        (
            I3PrivateReadOnlyProviderFixtureAssertion::ValueZero,
            PrivateProviderOutcome::Value(0)
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::Value41,
            PrivateProviderOutcome::Value(41)
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::ValueNegative7,
            PrivateProviderOutcome::Value(-7)
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::DeclaredTargetAbsent,
            PrivateProviderOutcome::ProviderResourceNotFound
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::NoncanonicalInteger,
            PrivateProviderOutcome::ProviderInvalidResult
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::ReadBoundary33Bytes,
            PrivateProviderOutcome::ProviderInvalidResult
        ) | (
            I3PrivateReadOnlyProviderFixtureAssertion::DeclaredTargetDirectory,
            PrivateProviderOutcome::ProviderPolicyDenied
        )
    )
}

fn parse_canonical_i64(bytes: &[u8]) -> Option<i64> {
    let text = str::from_utf8(bytes).ok()?;
    let digits = text.strip_suffix('\n').unwrap_or(text);
    if digits.is_empty() || digits.contains('\n') {
        return None;
    }
    let value = digits.parse::<i64>().ok()?;
    (value.to_string() == digits).then_some(value)
}

fn request_identity_ref(
    binding: &M9I3PrivateProviderCarrierBinding,
    ordinal: u64,
) -> Result<String, ()> {
    let encoded = serde_json::to_vec(binding).map_err(|_| ())?;
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/provider/request-occurrence/v1\0");
    hasher.update((encoded.len() as u64).to_be_bytes());
    hasher.update(encoded);
    hasher.update(ordinal.to_be_bytes());
    Ok(format!(
        "i3-provider-request-occurrence-sha256-v1:{:x}",
        hasher.finalize()
    ))
}

#[cfg(target_os = "linux")]
fn open_regular_no_follow(path: &std::path::Path) -> std::io::Result<File> {
    const O_RDONLY: std::os::raw::c_int = 0;
    const O_NOFOLLOW: std::os::raw::c_int = 0o400_000;
    const O_CLOEXEC: std::os::raw::c_int = 0o2_000_000;

    unsafe extern "C" {
        fn open(
            path: *const std::os::raw::c_char,
            flags: std::os::raw::c_int,
        ) -> std::os::raw::c_int;
    }

    let path = CString::new(path.as_os_str().as_bytes())?;
    // SAFETY: `path` is NUL-terminated for this call; a successful descriptor
    // is transferred exactly once into the File below.
    let fd = unsafe { open(path.as_ptr(), O_RDONLY | O_NOFOLLOW | O_CLOEXEC) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: a successful `open` returns exactly one owned descriptor.
    Ok(unsafe { File::from_raw_fd(fd) })
}

#[cfg(not(target_os = "linux"))]
fn open_regular_no_follow(_path: &std::path::Path) -> std::io::Result<File> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "provider adapter requires Linux no-follow open",
    ))
}

fn rejected() -> Sys5I3ProcessRuntimeError {
    Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
}

fn revalidate_provider_use(
    local_authority: &mut M9I3ReadOnlyProviderLocalAuthority,
    binding: &M9I3PrivateProviderCarrierBinding,
    use_: M9I3ReadOnlyProviderRevalidationUse,
) -> Result<(), Sys5I3ProcessRuntimeError> {
    local_authority
        .revalidate_provider_carrier(binding, use_)
        .map_err(|failure| {
            Sys5I3ProcessRuntimeError::new(match failure {
                M9I3ReadOnlyProviderRevalidationFailure::StaleMembership => {
                    Sys5I3ProcessRuntimeErrorKind::StaleMembership
                }
                M9I3ReadOnlyProviderRevalidationFailure::MissingCapability => {
                    Sys5I3ProcessRuntimeErrorKind::MissingCapability
                }
                M9I3ReadOnlyProviderRevalidationFailure::MissingWitness => {
                    Sys5I3ProcessRuntimeErrorKind::MissingWitness
                }
            })
        })
}

fn resource_exhausted() -> Sys5I3ProcessRuntimeError {
    Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ResourceExhausted)
}

fn duplicate_request() -> Sys5I3ProcessRuntimeError {
    Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected)
}

fn request_binding_mismatch() -> Sys5I3ProcessRuntimeError {
    Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RequestIdentityBindingMismatch)
}

#[cfg(test)]
#[path = "sys5_i3_provider_runtime_tests.rs"]
mod sys5_i3_provider_runtime_tests;
