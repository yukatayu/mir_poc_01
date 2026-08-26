//! Bounded, executable model-checking evidence for the SYS-2 backend contract.
//!
//! This module deliberately models a finite selected fragment.  It is neither a
//! theorem prover nor a claim about every scheduler or hardware memory model.
//! The result records typed states and transitions so a reported counterexample
//! can be replayed from its initial state rather than being a hand-written trace.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// The backend profile being calibrated by the finite model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExecutionProfile {
    /// The deterministic reference event-loop profile.
    SingleThread,
    /// One owner worker and mailbox per semantic owner.
    OneOwnerWorker,
    /// A deliberately weak-memory calibration, separate from an OW claim.
    WeakMemoryCalibration,
}

/// A Mir-level edge whose removal is tested in the selected finite fragment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RequiredEdge {
    OwnerRequestServe,
    PublishObserve,
    WitnessCreateUse,
    CapabilityGrantUse,
    RevocationVisibility,
    PatchActivationVisibility,
    CutSaveQuiescence,
    RelationEpochSample,
    SameOwnerReadsFromCoherence,
    PresentationGapNonmutation,
}

/// A bounded negative outcome.  This is intentionally a state predicate, not a
/// label attached to a pre-computed trace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BadOutcome {
    ServeWithoutPriorRequest,
    ObservedBeforePublish,
    WitnessUseBeforeCreate,
    CapabilityUseBeforeGrant,
    StaleServeAfterRevocation,
    RequestCrossesPatchActivationFrontier,
    MutationEscapesSaveCut,
    RelationSampleMixesEpochs,
    SameOwnerRmwReadsFromStaleWrite,
    PresentationGapMutatesSemanticLineage,
}

/// Whether a capability or witness is usable in the finite state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AuthorityStatus {
    Active,
    Stale,
    Revoked,
}

/// The authority data which must travel with the selected owner operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityLineage {
    principal: String,
    membership_epoch: String,
    membership_incarnation: String,
    capability_ref: String,
    witness_ref: String,
    witness_capability_ref: String,
    capability_status: AuthorityStatus,
    witness_status: AuthorityStatus,
    revocation_generation: Option<usize>,
}

impl AuthorityLineage {
    fn attack_authority() -> Self {
        Self {
            principal: "self".to_owned(),
            membership_epoch: "epoch1".to_owned(),
            membership_incarnation: "incarnation:self:S:epoch1".to_owned(),
            capability_ref: "cap:attack:S:self:epoch1".to_owned(),
            witness_ref: "witness:attack:S:self:epoch1".to_owned(),
            witness_capability_ref: "cap:attack:S:self:epoch1".to_owned(),
            capability_status: AuthorityStatus::Active,
            witness_status: AuthorityStatus::Active,
            revocation_generation: None,
        }
    }

    fn forged_source_free_attempt() -> Self {
        Self {
            principal: "forged".to_owned(),
            membership_epoch: "untrusted-epoch".to_owned(),
            membership_incarnation: "untrusted-incarnation".to_owned(),
            capability_ref: "forged-capability".to_owned(),
            witness_ref: "forged-witness".to_owned(),
            witness_capability_ref: "forged-capability".to_owned(),
            capability_status: AuthorityStatus::Active,
            witness_status: AuthorityStatus::Active,
            revocation_generation: None,
        }
    }

    pub fn principal(&self) -> &str {
        &self.principal
    }

    pub fn membership_epoch(&self) -> &str {
        &self.membership_epoch
    }

    pub fn membership_incarnation(&self) -> &str {
        &self.membership_incarnation
    }

    pub fn capability_ref(&self) -> &str {
        &self.capability_ref
    }

    pub fn witness_ref(&self) -> &str {
        &self.witness_ref
    }

    pub fn witness_capability_ref(&self) -> &str {
        &self.witness_capability_ref
    }

    pub fn capability_status(&self) -> AuthorityStatus {
        self.capability_status
    }

    pub fn witness_status(&self) -> AuthorityStatus {
        self.witness_status
    }

    pub fn revocation_generation(&self) -> Option<usize> {
        self.revocation_generation
    }
}

/// A deterministic in-process fingerprint used to link a recorded transition
/// to the exact source and target state observed by the finite explorer.
#[derive(Clone, Debug, Eq, PartialEq)]
struct StateFingerprint(String);

/// A rejected authority attempt.  Its requested lineage is recorded, but it is
/// deliberately not inserted into semantic authority state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedAuthorityAttempt {
    reason: String,
    attempted_lineage: AuthorityLineage,
    pre_semantic_state: StateFingerprint,
    post_semantic_state: StateFingerprint,
}

impl RejectedAuthorityAttempt {
    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn attempted_lineage(&self) -> &AuthorityLineage {
        &self.attempted_lineage
    }

    pub fn preserves_semantic_state(&self) -> bool {
        self.pre_semantic_state == self.post_semantic_state
    }
}

/// A typed finite model state.  Its public query methods are the evidence
/// surface; the transition machinery remains internal so callers cannot mint a
/// semantic state or authority through this model API.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelState {
    request_counts: BTreeMap<String, usize>,
    serve_counts: BTreeMap<String, usize>,
    semantic_values: BTreeMap<String, i64>,
    semantic_generations: BTreeMap<String, usize>,
    last_mutation_source_generations: BTreeMap<String, usize>,
    published_versions: BTreeMap<String, usize>,
    observed_versions: BTreeMap<(String, String), usize>,
    authority_lineages: BTreeMap<String, AuthorityLineage>,
    used_authority_generations: BTreeMap<String, usize>,
    request_patch_generations: BTreeMap<String, usize>,
    active_patch_generation: Option<usize>,
    save_cut_generation: Option<usize>,
    mutations_after_save_cut: BTreeSet<String>,
    relation_sample_epochs: BTreeMap<(String, String), (String, String)>,
    reads_from_versions: BTreeMap<(String, String), usize>,
    coherence_versions: BTreeMap<String, usize>,
    presentation_gap_counts: BTreeMap<String, usize>,
    semantic_lineage_changed_by_presentation_gap: BTreeSet<String>,
    observable_outcomes: BTreeSet<String>,
    rejected_authority_use_mutations: Vec<String>,
    rejected_authority_attempts: Vec<RejectedAuthorityAttempt>,
    stale_patch_rejection_count: usize,
    stale_patch_execution_count: usize,
    store_buffers: BTreeMap<String, BTreeMap<String, i64>>,
    store_memory: BTreeMap<String, i64>,
    store_memory_sources: BTreeMap<String, String>,
    store_read_values: BTreeMap<String, i64>,
    store_reads_from: BTreeMap<String, String>,
    markers: BTreeSet<String>,
    completed_actions: BTreeSet<String>,
}

impl ModelState {
    fn initial() -> Self {
        let mut semantic_values = BTreeMap::new();
        semantic_values.insert("player[target].hp".to_owned(), 100);
        let mut semantic_generations = BTreeMap::new();
        semantic_generations.insert("player[target].hp".to_owned(), 0);
        let mut authority_lineages = BTreeMap::new();
        authority_lineages.insert(
            "self:S:attack".to_owned(),
            AuthorityLineage::attack_authority(),
        );
        let mut store_memory = BTreeMap::new();
        store_memory.insert("x".to_owned(), 0);
        store_memory.insert("y".to_owned(), 0);
        let mut store_memory_sources = BTreeMap::new();
        store_memory_sources.insert("x".to_owned(), "initial:x".to_owned());
        store_memory_sources.insert("y".to_owned(), "initial:y".to_owned());
        Self {
            request_counts: BTreeMap::new(),
            serve_counts: BTreeMap::new(),
            semantic_values,
            semantic_generations,
            last_mutation_source_generations: BTreeMap::new(),
            published_versions: BTreeMap::new(),
            observed_versions: BTreeMap::new(),
            authority_lineages,
            used_authority_generations: BTreeMap::new(),
            request_patch_generations: BTreeMap::new(),
            active_patch_generation: Some(0),
            save_cut_generation: None,
            mutations_after_save_cut: BTreeSet::new(),
            relation_sample_epochs: BTreeMap::new(),
            reads_from_versions: BTreeMap::new(),
            coherence_versions: BTreeMap::new(),
            presentation_gap_counts: BTreeMap::new(),
            semantic_lineage_changed_by_presentation_gap: BTreeSet::new(),
            observable_outcomes: BTreeSet::new(),
            rejected_authority_use_mutations: Vec::new(),
            rejected_authority_attempts: Vec::new(),
            stale_patch_rejection_count: 0,
            stale_patch_execution_count: 0,
            store_buffers: BTreeMap::new(),
            store_memory,
            store_memory_sources,
            store_read_values: BTreeMap::new(),
            store_reads_from: BTreeMap::new(),
            markers: BTreeSet::new(),
            completed_actions: BTreeSet::new(),
        }
    }

    pub fn request_count(&self, operation: &str) -> usize {
        self.request_counts.get(operation).copied().unwrap_or(0)
    }

    pub fn serve_count(&self, operation: &str) -> usize {
        self.serve_counts.get(operation).copied().unwrap_or(0)
    }

    pub fn semantic_generation(&self, key: &str) -> Option<usize> {
        self.semantic_generations.get(key).copied()
    }

    pub fn int(&self, key: &str) -> Option<i64> {
        self.semantic_values.get(key).copied()
    }

    pub fn last_mutation_source_generation(&self, key: &str) -> Option<usize> {
        self.last_mutation_source_generations.get(key).copied()
    }

    pub fn published_version(&self, value: &str) -> Option<usize> {
        self.published_versions.get(value).copied()
    }

    pub fn observed_version(&self, value: &str, observer: &str) -> Option<usize> {
        self.observed_versions
            .get(&(value.to_owned(), observer.to_owned()))
            .copied()
    }

    pub fn authority_lineage(&self, lineage: &str) -> Option<&AuthorityLineage> {
        self.authority_lineages.get(lineage)
    }

    pub fn used_authority_generation(&self, lineage: &str) -> Option<usize> {
        self.used_authority_generations.get(lineage).copied()
    }

    pub fn active_patch_generation(&self) -> Option<usize> {
        self.active_patch_generation
    }

    pub fn request_patch_generation(&self, operation: &str) -> Option<usize> {
        self.request_patch_generations.get(operation).copied()
    }

    pub fn save_cut_generation(&self) -> Option<usize> {
        self.save_cut_generation
    }

    pub fn has_mutation_after_save_cut(&self, key: &str) -> bool {
        self.mutations_after_save_cut.contains(key)
    }

    pub fn relation_sample_epochs(&self, relation: &str, observer: &str) -> Option<(&str, &str)> {
        self.relation_sample_epochs
            .get(&(relation.to_owned(), observer.to_owned()))
            .map(|(primary, fallback)| (primary.as_str(), fallback.as_str()))
    }

    pub fn reads_from_version(&self, operation: &str, key: &str) -> Option<usize> {
        self.reads_from_versions
            .get(&(operation.to_owned(), key.to_owned()))
            .copied()
    }

    pub fn coherence_version(&self, key: &str) -> Option<usize> {
        self.coherence_versions.get(key).copied()
    }

    pub fn presentation_gap_count(&self, observer: &str) -> usize {
        self.presentation_gap_counts
            .get(observer)
            .copied()
            .unwrap_or(0)
    }

    pub fn semantic_lineage_changed_by_presentation_gap(&self, relation: &str) -> bool {
        self.semantic_lineage_changed_by_presentation_gap
            .contains(relation)
    }

    pub fn rejected_authority_use_mutations(&self) -> &[String] {
        &self.rejected_authority_use_mutations
    }

    pub fn rejected_authority_attempts(&self) -> &[RejectedAuthorityAttempt] {
        &self.rejected_authority_attempts
    }

    pub fn stale_patch_rejection_count(&self) -> usize {
        self.stale_patch_rejection_count
    }

    pub fn stale_patch_execution_count(&self) -> usize {
        self.stale_patch_execution_count
    }

    pub fn store_read(&self, register: &str) -> Option<i64> {
        self.store_read_values.get(register).copied()
    }

    pub fn store_reads_from(&self, register: &str) -> Option<&str> {
        self.store_reads_from.get(register).map(String::as_str)
    }

    fn marker(&self, marker: &str) -> bool {
        self.markers.contains(marker)
    }

    fn state_fingerprint(&self) -> StateFingerprint {
        StateFingerprint(format!("{self:?}"))
    }

    fn semantic_fingerprint(&self) -> StateFingerprint {
        StateFingerprint(format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
            self.request_counts,
            self.serve_counts,
            self.semantic_values,
            self.semantic_generations,
            self.last_mutation_source_generations,
            self.published_versions,
            self.observed_versions,
            self.authority_lineages,
            self.used_authority_generations,
        ))
    }

    fn record_rejected_authority_attempt(
        &mut self,
        reason: &str,
        attempted_lineage: AuthorityLineage,
    ) {
        let pre_semantic_state = self.semantic_fingerprint();
        let post_semantic_state = self.semantic_fingerprint();
        self.rejected_authority_attempts
            .push(RejectedAuthorityAttempt {
                reason: reason.to_owned(),
                attempted_lineage,
                pre_semantic_state,
                post_semantic_state,
            });
    }

    fn store_buffer_write(&mut self, worker: &str, key: &str, value: i64) {
        self.store_buffers
            .entry(worker.to_owned())
            .or_default()
            .insert(key.to_owned(), value);
    }

    fn flush_store_buffer(&mut self, worker: &str, key: &str) -> Result<(), String> {
        let value = self
            .store_buffers
            .get_mut(worker)
            .and_then(|buffer| buffer.remove(key))
            .ok_or_else(|| format!("{worker} has no buffered {key} write to flush"))?;
        self.store_memory.insert(key.to_owned(), value);
        self.store_memory_sources
            .insert(key.to_owned(), format!("{worker}:{key}@flush"));
        Ok(())
    }

    fn record_store_read(&mut self, register: &str, key: &str) -> Result<(), String> {
        let value = self
            .store_memory
            .get(key)
            .copied()
            .ok_or_else(|| format!("store key {key} is not modelled"))?;
        let source = self
            .store_memory_sources
            .get(key)
            .cloned()
            .ok_or_else(|| format!("store source for {key} is not modelled"))?;
        self.store_read_values.insert(register.to_owned(), value);
        self.store_reads_from.insert(register.to_owned(), source);
        if let (Some(left), Some(right)) = (
            self.store_read_values.get("r1"),
            self.store_read_values.get("r2"),
        ) {
            self.observable_outcomes
                .insert(format!("r1={left},r2={right}"));
        }
        Ok(())
    }

    fn set_semantic(&mut self, key: &str, value: i64, generation: usize, source_generation: usize) {
        self.semantic_values.insert(key.to_owned(), value);
        self.semantic_generations.insert(key.to_owned(), generation);
        self.last_mutation_source_generations
            .insert(key.to_owned(), source_generation);
    }

    fn attack_authority_mut(&mut self) -> Result<&mut AuthorityLineage, String> {
        self.authority_lineages
            .get_mut("self:S:attack")
            .ok_or_else(|| "missing selected attack authority lineage".to_owned())
    }

    fn apply(&self, action: Action) -> Result<Self, String> {
        let key = action.key();
        if self.completed_actions.contains(key) {
            return Err(format!("action {key} already executed"));
        }

        let mut next = self.clone();
        match action {
            Action::ArmUnrequestedServe => {
                next.markers.insert("unrequested_serve_armed".to_owned());
            }
            Action::ServeWithoutRequest => {
                require(
                    next.marker("unrequested_serve_armed"),
                    "unrequested serve was not armed",
                )?;
                require(
                    next.request_count("attack") == 0,
                    "request would make this a valid serve",
                )?;
                increment(&mut next.serve_counts, "attack");
            }
            Action::OwnerRequest => {
                increment(&mut next.request_counts, "attack");
            }
            Action::OwnerServe => {
                require(
                    next.request_count("attack") > next.serve_count("attack"),
                    "owner serve requires a prior request",
                )?;
                increment(&mut next.serve_counts, "attack");
                next.set_semantic("player[target].hp", 90, 1, 1);
            }
            Action::PublishResult => {
                next.published_versions.insert("result".to_owned(), 1);
            }
            Action::ObserveBeforePublish => {
                require(
                    next.published_version("result").is_none(),
                    "pre-publish observation requires no published result",
                )?;
                next.observed_versions
                    .insert(("result".to_owned(), "ViewerC".to_owned()), 0);
                next.markers.insert("observed_before_publish".to_owned());
            }
            Action::ObservePublishedResult => {
                require(
                    next.published_version("result") == Some(1),
                    "observation requires a published result",
                )?;
                next.observed_versions
                    .insert(("result".to_owned(), "ViewerC".to_owned()), 1);
            }
            Action::ArmStaleWitness => {
                next.attack_authority_mut()?.witness_status = AuthorityStatus::Stale;
                next.markers.insert("stale_witness_armed".to_owned());
            }
            Action::UseStaleWitness => {
                require(
                    next.marker("stale_witness_armed"),
                    "stale witness use was not armed",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| lineage.witness_status == AuthorityStatus::Stale),
                    "selected witness is not stale",
                )?;
                next.markers.insert("stale_witness_used".to_owned());
            }
            Action::CreateWitness => {
                next.markers.insert("witness_created".to_owned());
            }
            Action::UseCreatedWitness => {
                require(
                    next.marker("witness_created"),
                    "witness use requires witness creation",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| lineage.witness_status == AuthorityStatus::Active),
                    "created witness must be active",
                )?;
            }
            Action::ArmCapabilityMismatch => {
                let lineage = next.attack_authority_mut()?;
                lineage.capability_status = AuthorityStatus::Revoked;
                lineage.witness_capability_ref = "cap:attack:S:other:epoch1".to_owned();
                next.markers.insert("capability_mismatch_armed".to_owned());
            }
            Action::UseMismatchedCapability => {
                require(
                    next.marker("capability_mismatch_armed"),
                    "capability mismatch use was not armed",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| {
                            lineage.capability_status == AuthorityStatus::Revoked
                                && lineage.capability_ref != lineage.witness_capability_ref
                        }),
                    "selected capability lineage is not mismatched",
                )?;
                next.markers.insert("mismatched_capability_used".to_owned());
            }
            Action::GrantCapability => {
                next.markers.insert("capability_granted".to_owned());
            }
            Action::UseGrantedCapability => {
                require(
                    next.marker("capability_granted"),
                    "capability use requires a grant",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| {
                            lineage.capability_status == AuthorityStatus::Active
                                && lineage.capability_ref == lineage.witness_capability_ref
                        }),
                    "granted capability lineage is invalid",
                )?;
            }
            Action::EnqueueGenerationZero => {
                next.markers.insert("g0_enqueued".to_owned());
            }
            Action::RevokePublishGenerationOne => {
                require(
                    next.marker("g0_enqueued"),
                    "revocation requires a queued g0 request",
                )?;
                let lineage = next.attack_authority_mut()?;
                lineage.capability_status = AuthorityStatus::Revoked;
                lineage.revocation_generation = Some(1);
                next.markers.insert("g1_revoked".to_owned());
            }
            Action::StaleServeWriteGenerationZero => {
                require(
                    next.marker("g0_enqueued"),
                    "stale serve requires queued g0 request",
                )?;
                require(
                    next.marker("g1_revoked"),
                    "stale serve requires published revoke g1",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| {
                            lineage.capability_status == AuthorityStatus::Revoked
                        }),
                    "stale serve requires revoked authority",
                )?;
                next.used_authority_generations
                    .insert("self:S:attack".to_owned(), 0);
                next.set_semantic("player[target].hp", 90, 1, 0);
            }
            Action::RejectStaleServe => {
                require(
                    next.marker("g0_enqueued"),
                    "stale rejection requires queued g0 request",
                )?;
                require(
                    next.marker("g1_revoked"),
                    "stale rejection requires published revoke g1",
                )?;
                require(
                    next.authority_lineage("self:S:attack")
                        .is_some_and(|lineage| {
                            lineage.capability_status == AuthorityStatus::Revoked
                        }),
                    "stale rejection requires revoked authority",
                )?;
                next.record_rejected_authority_attempt(
                    "stale-generation-use-after-revoke",
                    next.authority_lineage("self:S:attack")
                        .expect("selected attack lineage exists")
                        .clone(),
                );
            }
            Action::RecordPatchRequestGenerationZero => {
                next.request_patch_generations
                    .insert("attack".to_owned(), 0);
                next.markers.insert("patch_request_g0".to_owned());
            }
            Action::ActivatePatchGenerationOne => {
                require(
                    next.marker("patch_request_g0"),
                    "patch activation needs the selected request frontier",
                )?;
                next.active_patch_generation = Some(1);
            }
            Action::RejectStalePatchRequest => {
                require(
                    next.active_patch_generation == Some(1),
                    "patch rejection requires active patch g1",
                )?;
                require(
                    next.request_patch_generation("attack") == Some(0),
                    "patch rejection requires stale request g0",
                )?;
                next.stale_patch_rejection_count += 1;
            }
            Action::ExecuteStalePatchRequest => {
                require(
                    next.active_patch_generation == Some(1),
                    "stale execution requires active patch g1",
                )?;
                require(
                    next.request_patch_generation("attack") == Some(0),
                    "stale execution requires request g0",
                )?;
                next.stale_patch_execution_count += 1;
            }
            Action::SaveCutGenerationOne => {
                next.save_cut_generation = Some(1);
            }
            Action::MutateAfterSaveCut => {
                require(
                    next.save_cut_generation == Some(1),
                    "mutation needs a save cut",
                )?;
                next.set_semantic("player[target].hp", 90, 1, 1);
                next.mutations_after_save_cut
                    .insert("player[target].hp".to_owned());
            }
            Action::RejectMutationAfterSaveCut => {
                require(
                    next.save_cut_generation == Some(1),
                    "rejection needs a save cut",
                )?;
            }
            Action::PrepareFallbackEpochZero => {
                next.markers.insert("fallback_epoch_0".to_owned());
            }
            Action::SampleMixedRelationEpochs => {
                require(
                    next.marker("fallback_epoch_0"),
                    "mixed sample needs fallback epoch zero",
                )?;
                next.relation_sample_epochs.insert(
                    ("bird_follow".to_owned(), "ViewerC".to_owned()),
                    ("primary_epoch:1".to_owned(), "fallback_epoch:0".to_owned()),
                );
            }
            Action::PrepareRelationEpochOne => {
                next.markers.insert("relation_epoch_1".to_owned());
            }
            Action::SampleCoherentRelationEpochs => {
                require(
                    next.marker("relation_epoch_1"),
                    "coherent sample needs relation epoch one",
                )?;
                next.relation_sample_epochs.insert(
                    ("bird_follow".to_owned(), "ViewerC".to_owned()),
                    ("primary_epoch:1".to_owned(), "fallback_epoch:1".to_owned()),
                );
            }
            Action::FirstOwnerRmw => {
                next.coherence_versions
                    .insert("player[target].hp".to_owned(), 1);
                next.set_semantic("player[target].hp", 90, 1, 1);
                next.markers.insert("first_owner_rmw".to_owned());
            }
            Action::SecondOwnerRmwReadsStale => {
                require(
                    next.marker("first_owner_rmw"),
                    "second RMW requires first owner RMW",
                )?;
                require(
                    next.coherence_version("player[target].hp") == Some(1),
                    "coherence must advance before stale reads-from check",
                )?;
                next.reads_from_versions
                    .insert(("attack#2".to_owned(), "player[target].hp".to_owned()), 0);
            }
            Action::SecondOwnerRmwReadsCoherent => {
                require(
                    next.marker("first_owner_rmw"),
                    "second RMW requires first owner RMW",
                )?;
                require(
                    next.coherence_version("player[target].hp") == Some(1),
                    "coherence must advance before reads-from check",
                )?;
                next.reads_from_versions
                    .insert(("attack#2".to_owned(), "player[target].hp".to_owned()), 1);
            }
            Action::PresentationGap => {
                increment(&mut next.presentation_gap_counts, "ViewerC");
            }
            Action::MutateLineageForPresentationGap => {
                require(
                    next.presentation_gap_count("ViewerC") == 1,
                    "presentation lineage mutation needs a gap",
                )?;
                next.semantic_lineage_changed_by_presentation_gap
                    .insert("bird_follow".to_owned());
            }
            Action::PreserveLineageForPresentationGap => {
                require(
                    next.presentation_gap_count("ViewerC") == 1,
                    "presentation preservation needs a gap",
                )?;
            }
            Action::ForgeSourceFreeAuthorityAttempt => {
                next.record_rejected_authority_attempt(
                    "source-free-authority-mint",
                    AuthorityLineage::forged_source_free_attempt(),
                );
            }
            Action::StoreBufferWriteLeft => {
                next.store_buffer_write("left", "x", 1);
                next.markers.insert("store_left_written".to_owned());
            }
            Action::StoreBufferWriteRight => {
                next.store_buffer_write("right", "y", 1);
                next.markers.insert("store_right_written".to_owned());
            }
            Action::StoreFlushLeft => {
                require(
                    next.marker("store_left_written"),
                    "left buffered write is missing",
                )?;
                next.flush_store_buffer("left", "x")?;
                next.markers.insert("store_left_flushed".to_owned());
            }
            Action::StoreFlushRight => {
                require(
                    next.marker("store_right_written"),
                    "right buffered write is missing",
                )?;
                next.flush_store_buffer("right", "y")?;
                next.markers.insert("store_right_flushed".to_owned());
            }
            Action::StoreReadLeftRelaxed => {
                require(
                    next.marker("store_left_written"),
                    "left program-order write is missing",
                )?;
                next.record_store_read("r1", "y")?;
            }
            Action::StoreReadRightRelaxed => {
                require(
                    next.marker("store_right_written"),
                    "right program-order write is missing",
                )?;
                next.record_store_read("r2", "x")?;
            }
            Action::StoreReadLeftAfterPublish => {
                require(
                    next.marker("store_left_written"),
                    "left program-order write is missing",
                )?;
                require(
                    next.marker("store_right_flushed"),
                    "publish visibility to left is missing",
                )?;
                next.record_store_read("r1", "y")?;
            }
            Action::StoreReadRightAfterPublish => {
                require(
                    next.marker("store_right_written"),
                    "right program-order write is missing",
                )?;
                require(
                    next.marker("store_left_flushed"),
                    "publish visibility to right is missing",
                )?;
                next.record_store_read("r2", "x")?;
            }
        }
        next.completed_actions.insert(key.to_owned());
        Ok(next)
    }
}

fn increment(values: &mut BTreeMap<String, usize>, key: &str) {
    *values.entry(key.to_owned()).or_insert(0) += 1;
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    condition.then_some(()).ok_or_else(|| message.to_owned())
}

/// A predicate evaluated against a reached [`ModelState`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadPredicate {
    outcome: BadOutcome,
}

impl BadPredicate {
    pub fn outcome(&self) -> BadOutcome {
        self.outcome
    }

    pub fn holds(&self, state: &ModelState) -> bool {
        match self.outcome {
            BadOutcome::ServeWithoutPriorRequest => {
                state.request_count("attack") == 0
                    && state.serve_count("attack") == 1
                    && state.semantic_generation("player[target].hp") == Some(0)
            }
            BadOutcome::ObservedBeforePublish => {
                state.published_version("result") == Some(1)
                    && state.observed_version("result", "ViewerC") == Some(0)
                    && state.marker("observed_before_publish")
            }
            BadOutcome::WitnessUseBeforeCreate => {
                state
                    .authority_lineage("self:S:attack")
                    .is_some_and(|lineage| {
                        lineage.witness_status == AuthorityStatus::Stale
                            && lineage.capability_status == AuthorityStatus::Active
                    })
                    && state.marker("stale_witness_used")
            }
            BadOutcome::CapabilityUseBeforeGrant => {
                state
                    .authority_lineage("self:S:attack")
                    .is_some_and(|lineage| {
                        lineage.capability_status == AuthorityStatus::Revoked
                            && lineage.witness_status == AuthorityStatus::Active
                            && lineage.capability_ref != lineage.witness_capability_ref
                    })
                    && state.marker("mismatched_capability_used")
            }
            BadOutcome::StaleServeAfterRevocation => {
                state
                    .authority_lineage("self:S:attack")
                    .is_some_and(|lineage| {
                        lineage.capability_status == AuthorityStatus::Revoked
                            && lineage.revocation_generation == Some(1)
                    })
                    && state.used_authority_generation("self:S:attack") == Some(0)
                    && state.semantic_generation("player[target].hp") == Some(1)
                    && state.last_mutation_source_generation("player[target].hp") == Some(0)
            }
            BadOutcome::RequestCrossesPatchActivationFrontier => {
                state.active_patch_generation() == Some(1)
                    && state.request_patch_generation("attack") == Some(0)
                    && state.stale_patch_execution_count() == 1
            }
            BadOutcome::MutationEscapesSaveCut => {
                state.save_cut_generation() == Some(1)
                    && state.has_mutation_after_save_cut("player[target].hp")
            }
            BadOutcome::RelationSampleMixesEpochs => state
                .relation_sample_epochs("bird_follow", "ViewerC")
                .is_some_and(|(primary, fallback)| primary != fallback),
            BadOutcome::SameOwnerRmwReadsFromStaleWrite => {
                state.reads_from_version("attack#2", "player[target].hp") == Some(0)
                    && state.coherence_version("player[target].hp") == Some(1)
            }
            BadOutcome::PresentationGapMutatesSemanticLineage => {
                state.presentation_gap_count("ViewerC") == 1
                    && state.semantic_lineage_changed_by_presentation_gap("bird_follow")
            }
        }
    }
}

/// A recorded typed event in a replayable trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelEvent {
    name: String,
    required_edge: Option<RequiredEdge>,
}

impl ModelEvent {
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A state transition in the finite relation.  The action is private: callers
/// can inspect relation identity but cannot construct source-free transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelTransition {
    source_state_id: Option<usize>,
    target_state_id: Option<usize>,
    source_state_fingerprint: StateFingerprint,
    target_state_fingerprint: StateFingerprint,
    litmus_case: Option<String>,
    required_edge: Option<RequiredEdge>,
    event_name: String,
    action: Action,
}

impl ModelTransition {
    fn new(
        source_state_id: usize,
        target_state_id: usize,
        litmus_case: &str,
        required_edge: Option<RequiredEdge>,
        action: Action,
        source_state: &ModelState,
        target_state: &ModelState,
    ) -> Self {
        Self {
            source_state_id: Some(source_state_id),
            target_state_id: Some(target_state_id),
            source_state_fingerprint: source_state.state_fingerprint(),
            target_state_fingerprint: target_state.state_fingerprint(),
            litmus_case: Some(litmus_case.to_owned()),
            required_edge,
            event_name: action.event_name().to_owned(),
            action,
        }
    }

    pub fn source_state_id(&self) -> Option<usize> {
        self.source_state_id
    }

    pub fn target_state_id(&self) -> Option<usize> {
        self.target_state_id
    }

    pub fn litmus_case(&self) -> Option<&str> {
        self.litmus_case.as_deref()
    }
}

/// An invalid edited or reordered transition sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayError {
    InvalidTransitionSequence { index: usize, reason: String },
}

impl ReplayError {
    pub fn is_invalid_transition_sequence(&self) -> bool {
        matches!(self, Self::InvalidTransitionSequence { .. })
    }
}

/// A trace reconstructed by applying typed transitions from an initial state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelTrace {
    reached_state: ModelState,
    events: Vec<ModelEvent>,
}

impl ModelTrace {
    pub fn replay(
        initial_state: ModelState,
        transitions: Vec<ModelTransition>,
    ) -> Result<Self, ReplayError> {
        let mut reached_state = initial_state.clone();
        let mut events = Vec::with_capacity(transitions.len());
        for (index, transition) in transitions.into_iter().enumerate() {
            if transition.source_state_id.is_none()
                || transition.target_state_id.is_none()
                || transition.litmus_case.is_none()
            {
                return Err(ReplayError::InvalidTransitionSequence {
                    index,
                    reason: "transition has no typed relation identity".to_owned(),
                });
            }
            if reached_state.state_fingerprint() != transition.source_state_fingerprint {
                return Err(ReplayError::InvalidTransitionSequence {
                    index,
                    reason: "transition source fingerprint does not match replay state".to_owned(),
                });
            }
            reached_state = reached_state
                .apply(transition.action)
                .map_err(|reason| ReplayError::InvalidTransitionSequence { index, reason })?;
            if reached_state.state_fingerprint() != transition.target_state_fingerprint {
                return Err(ReplayError::InvalidTransitionSequence {
                    index,
                    reason: "transition action did not reach its recorded target state".to_owned(),
                });
            }
            events.push(ModelEvent {
                name: transition.event_name,
                required_edge: transition.required_edge,
            });
        }
        Ok(Self {
            reached_state,
            events,
        })
    }

    fn from_transitions(initial_state: ModelState, transitions: Vec<ModelTransition>) -> Self {
        Self::replay(initial_state, transitions)
            .expect("the explorer only records transitions admitted by the relation")
    }

    pub fn reached_state(&self) -> &ModelState {
        &self.reached_state
    }

    pub fn events(&self) -> &[ModelEvent] {
        &self.events
    }

    pub fn event_names(&self) -> Vec<&str> {
        self.events.iter().map(|event| event.name()).collect()
    }

    pub fn mentions_required_edge(&self, edge: RequiredEdge) -> bool {
        self.events
            .iter()
            .any(|event| event.required_edge == Some(edge))
    }
}

/// A reachable bad predicate plus the exact finite trace which reaches it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Counterexample {
    missing_required_edge: RequiredEdge,
    bad_predicate: BadPredicate,
    initial_state: ModelState,
    reached_state: ModelState,
    transitions: Vec<ModelTransition>,
    trace: ModelTrace,
}

impl Counterexample {
    pub fn missing_required_edge(&self) -> RequiredEdge {
        self.missing_required_edge
    }

    pub fn bad_outcome(&self) -> BadOutcome {
        self.bad_predicate.outcome()
    }

    pub fn bad_predicate(&self) -> &BadPredicate {
        &self.bad_predicate
    }

    pub fn initial_state(&self) -> &ModelState {
        &self.initial_state
    }

    pub fn reached_state(&self) -> &ModelState {
        &self.reached_state
    }

    pub fn transitions(&self) -> &[ModelTransition] {
        &self.transitions
    }

    pub fn trace(&self) -> &ModelTrace {
        &self.trace
    }
}

/// One selected litmus case.  It is an executable finite abstraction, not a
/// substitute for the production scheduler or a general concurrency theorem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LitmusCase {
    name: &'static str,
    kind: LitmusKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LitmusKind {
    OwnerRequestServe,
    StoreBuffering,
    PublicationObservation,
    WitnessCreationUse,
    CapabilityRevokeUse,
    PatchActivateRequest,
    SaveCutMutation,
    RelationEpochSample,
    SameOwnerRmw,
    PresentationGap,
}

impl LitmusCase {
    pub fn owner_request_serve_message_passing() -> Self {
        Self::new(
            "owner_request_serve_message_passing",
            LitmusKind::OwnerRequestServe,
        )
    }

    pub fn store_buffering_calibration() -> Self {
        Self::new("store_buffering_calibration", LitmusKind::StoreBuffering)
    }

    pub fn publication_observation() -> Self {
        Self::new(
            "publication_observation",
            LitmusKind::PublicationObservation,
        )
    }

    pub fn witness_creation_use() -> Self {
        Self::new("witness_creation_use", LitmusKind::WitnessCreationUse)
    }

    pub fn capability_revoke_use_race() -> Self {
        Self::new(
            "capability_revoke_use_race",
            LitmusKind::CapabilityRevokeUse,
        )
    }

    pub fn patch_activate_request_race() -> Self {
        Self::new(
            "patch_activate_request_race",
            LitmusKind::PatchActivateRequest,
        )
    }

    pub fn save_cut_mutation_race() -> Self {
        Self::new("save_cut_mutation_race", LitmusKind::SaveCutMutation)
    }

    pub fn relation_epoch_sample_race() -> Self {
        Self::new(
            "relation_epoch_sample_race",
            LitmusKind::RelationEpochSample,
        )
    }

    pub fn same_owner_two_request_rmw() -> Self {
        Self::new("same_owner_two_request_rmw", LitmusKind::SameOwnerRmw)
    }

    pub fn presentation_gap_nonmutation() -> Self {
        Self::new("presentation_gap_nonmutation", LitmusKind::PresentationGap)
    }

    fn new(name: &'static str, kind: LitmusKind) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

/// Per-case finite observation evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseReport {
    observable_outcomes: BTreeSet<String>,
    rejected_state_mutations: Vec<String>,
    semantic_mutations_after_revocation: BTreeSet<String>,
    explored_state_count: usize,
    explored_transition_count: usize,
    terminal_state_count: usize,
    expected_action_count: usize,
    covered_action_count: usize,
}

impl CaseReport {
    pub fn allows_observable_outcome(&self, outcome: &str) -> bool {
        self.observable_outcomes.contains(outcome)
    }

    pub fn forbids_observable_outcome(&self, outcome: &str) -> bool {
        !self.allows_observable_outcome(outcome)
    }

    pub fn rejected_state_mutations(&self) -> &[String] {
        &self.rejected_state_mutations
    }

    pub fn has_semantic_mutation_after_revocation(&self, key: &str) -> bool {
        self.semantic_mutations_after_revocation.contains(key)
    }

    pub fn explored_state_count(&self) -> usize {
        self.explored_state_count
    }

    pub fn explored_transition_count(&self) -> usize {
        self.explored_transition_count
    }

    pub fn terminal_state_count(&self) -> usize {
        self.terminal_state_count
    }

    pub fn expected_action_count(&self) -> usize {
        self.expected_action_count
    }

    pub fn covered_action_count(&self) -> usize {
        self.covered_action_count
    }

    pub fn has_complete_action_coverage(&self) -> bool {
        self.expected_action_count == self.covered_action_count
    }
}

/// The bounded result.  `evidence_label` intentionally classifies this as
/// model-checked bounded evidence rather than proof evidence.
#[derive(Clone, Debug)]
pub struct ModelCheckReport {
    profile: ExecutionProfile,
    bound: usize,
    initial_states: Vec<ModelState>,
    transition_relation: Vec<ModelTransition>,
    visited_state_count: usize,
    search_complete_within_bound: bool,
    counterexamples: Vec<Counterexample>,
    case_reports: BTreeMap<String, CaseReport>,
    no_source_free_authority_mints: bool,
    no_stale_authority_use: bool,
    rejected_authority_use_mutations: Vec<String>,
    rejected_authority_attempts: Vec<RejectedAuthorityAttempt>,
}

impl ModelCheckReport {
    pub fn evidence_label(&self) -> &'static str {
        "model-checked-bounded"
    }

    pub fn abstraction_summary(&self) -> &'static str {
        "finite selected SYS-2 state exploration; no general scheduler or memory-model claim"
    }

    pub fn profile(&self) -> ExecutionProfile {
        self.profile
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    pub fn initial_states(&self) -> &[ModelState] {
        &self.initial_states
    }

    pub fn transition_relation(&self) -> &[ModelTransition] {
        &self.transition_relation
    }

    pub fn search_complete_within_bound(&self) -> bool {
        self.search_complete_within_bound
    }

    pub fn bound_status(&self) -> BoundStatus {
        if self.search_complete_within_bound {
            BoundStatus::Complete
        } else {
            BoundStatus::Insufficient
        }
    }

    pub fn passes_all_litmus(&self) -> bool {
        self.search_complete_within_bound && self.counterexamples.is_empty()
    }

    pub fn visited_state_count(&self) -> usize {
        self.visited_state_count
    }

    pub fn transition_count(&self) -> usize {
        self.transition_relation.len()
    }

    pub fn litmus_count(&self) -> usize {
        self.case_reports.len()
    }

    pub fn has_violations(&self) -> bool {
        !self.counterexamples.is_empty()
    }

    pub fn violations(&self) -> &[Counterexample] {
        &self.counterexamples
    }

    pub fn counterexamples(&self) -> &[Counterexample] {
        &self.counterexamples
    }

    pub fn case_report(&self, name: &str) -> Option<&CaseReport> {
        self.case_reports.get(name)
    }

    pub fn claims_sequential_consistency(&self) -> bool {
        false
    }

    pub fn no_source_free_authority_mints(&self) -> bool {
        self.no_source_free_authority_mints
    }

    pub fn no_stale_authority_use(&self) -> bool {
        self.no_stale_authority_use
    }

    pub fn rejected_authority_use_mutations(&self) -> &[String] {
        &self.rejected_authority_use_mutations
    }

    pub fn rejected_authority_attempts(&self) -> &[RejectedAuthorityAttempt] {
        &self.rejected_authority_attempts
    }

    /// Compares only selected semantic outcomes.  It deliberately excludes the
    /// backend profile and transition IDs, so ST/OW agreement is not a claim
    /// that their implementations or schedules are identical.
    pub fn selected_semantic_results_match(&self, other: &Self) -> bool {
        self.case_reports
            .iter()
            .map(|(name, case)| {
                (
                    name,
                    &case.observable_outcomes,
                    &case.rejected_state_mutations,
                    &case.semantic_mutations_after_revocation,
                )
            })
            .eq(other.case_reports.iter().map(|(name, case)| {
                (
                    name,
                    &case.observable_outcomes,
                    &case.rejected_state_mutations,
                    &case.semantic_mutations_after_revocation,
                )
            }))
            && self
                .counterexamples
                .iter()
                .map(|counterexample| {
                    (
                        counterexample.missing_required_edge,
                        counterexample.bad_outcome(),
                        counterexample.reached_state.semantic_fingerprint(),
                    )
                })
                .eq(other.counterexamples.iter().map(|counterexample| {
                    (
                        counterexample.missing_required_edge,
                        counterexample.bad_outcome(),
                        counterexample.reached_state.semantic_fingerprint(),
                    )
                }))
    }

    /// A deterministic run fingerprint for this finite explorer.  It is an
    /// executable repeatability check, not a release identity or proof hash.
    pub fn deterministic_fingerprint(&self) -> String {
        format!(
            "{:?}|{:?}|{:?}|{:?}",
            self.case_reports,
            self.counterexamples,
            self.transition_relation,
            self.rejected_authority_attempts,
        )
    }
}

/// Whether the configured bound exhausted the finite selected state space.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundStatus {
    Complete,
    Insufficient,
}

impl BoundStatus {
    pub fn is_insufficient(&self) -> bool {
        matches!(self, Self::Insufficient)
    }
}

/// Builder for the selected finite SYS-2 model.
#[derive(Clone, Debug)]
pub struct Sys2BoundedModel {
    profile: ExecutionProfile,
    bound: usize,
    required_edges: BTreeSet<RequiredEdge>,
    litmus_cases: Vec<LitmusCase>,
}

impl Sys2BoundedModel {
    pub fn new() -> Self {
        Self {
            profile: ExecutionProfile::SingleThread,
            bound: 0,
            required_edges: BTreeSet::new(),
            litmus_cases: Vec::new(),
        }
    }

    pub fn with_profile(mut self, profile: ExecutionProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn with_bound(mut self, bound: usize) -> Self {
        self.bound = bound;
        self
    }

    pub fn with_required_edges(
        mut self,
        required_edges: impl IntoIterator<Item = RequiredEdge>,
    ) -> Self {
        self.required_edges = required_edges.into_iter().collect();
        self
    }

    pub fn with_litmus_case(mut self, litmus_case: LitmusCase) -> Self {
        self.litmus_cases.push(litmus_case);
        self
    }

    pub fn with_litmus_cases(mut self, litmus_cases: impl IntoIterator<Item = LitmusCase>) -> Self {
        self.litmus_cases.extend(litmus_cases);
        self
    }

    /// Exhaustively explores every enabled interleaving of each finite case up
    /// to `bound`.  Counterexamples are discovered by evaluating a typed state
    /// predicate after a real transition, never by attaching an outcome to an
    /// edge name.
    pub fn check(&self) -> ModelCheckReport {
        let initial = ModelState::initial();
        let initial_states = vec![initial.clone()];
        let mut states = vec![initial.clone()];
        let mut transition_relation = Vec::new();
        let mut counterexamples = Vec::new();
        let mut case_reports = BTreeMap::new();
        let mut search_complete_within_bound = true;

        for litmus_case in &self.litmus_cases {
            let plan = CasePlan::for_case(litmus_case, &self.required_edges, self.profile);
            if self.bound < plan.actions.len() {
                search_complete_within_bound = false;
            }
            let explored = explore_case(
                &initial,
                litmus_case.name(),
                &plan,
                self.bound,
                &mut states,
                &mut transition_relation,
            );
            if !explored.case_report.has_complete_action_coverage()
                || explored.case_report.explored_transition_count()
                    < explored.case_report.expected_action_count()
            {
                search_complete_within_bound = false;
            }

            if let Some((predicate, trace, reached_state)) = explored.first_bad {
                let missing_required_edge = plan
                    .missing_edge
                    .expect("only a missing edge creates a selected bad predicate");
                counterexamples.push(Counterexample {
                    missing_required_edge,
                    bad_predicate: predicate,
                    initial_state: initial.clone(),
                    reached_state,
                    transitions: trace.clone(),
                    trace: ModelTrace::from_transitions(initial.clone(), trace),
                });
            }
            case_reports.insert(litmus_case.name().to_owned(), explored.case_report);
        }

        let rejected_authority_attempts: Vec<_> = states
            .iter()
            .flat_map(|state| state.rejected_authority_attempts.iter().cloned())
            .collect();
        let no_source_free_authority_mints = rejected_authority_attempts.iter().any(|attempt| {
            attempt.reason() == "source-free-authority-mint"
                && attempt.attempted_lineage().principal() == "forged"
                && attempt.preserves_semantic_state()
        }) && states.iter().all(|state| {
            state.authority_lineages.len() == 1
                && state
                    .authority_lineage("self:S:attack")
                    .is_some_and(|lineage| {
                        lineage.principal == "self"
                            && lineage.membership_epoch == "epoch1"
                            && lineage.membership_incarnation == "incarnation:self:S:epoch1"
                            && !lineage.capability_ref.is_empty()
                            && !lineage.witness_ref.is_empty()
                    })
        });
        let no_stale_authority_use = counterexamples.iter().all(|counterexample| {
            counterexample.bad_outcome() != BadOutcome::StaleServeAfterRevocation
        });
        let rejected_authority_use_mutations = states
            .iter()
            .flat_map(|state| state.rejected_authority_use_mutations.iter().cloned())
            .collect();

        ModelCheckReport {
            profile: self.profile,
            bound: self.bound,
            initial_states,
            transition_relation,
            visited_state_count: states.len(),
            search_complete_within_bound,
            counterexamples,
            case_reports,
            no_source_free_authority_mints,
            no_stale_authority_use,
            rejected_authority_use_mutations,
            rejected_authority_attempts,
        }
    }
}

impl Default for Sys2BoundedModel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
struct CasePlan {
    required_edge: RequiredEdge,
    missing_edge: Option<RequiredEdge>,
    bad_outcome: Option<BadOutcome>,
    actions: Vec<Action>,
}

impl CasePlan {
    fn for_case(
        litmus_case: &LitmusCase,
        required_edges: &BTreeSet<RequiredEdge>,
        profile: ExecutionProfile,
    ) -> Self {
        let (required_edge, bad_outcome, safe_actions, unsafe_actions) = match litmus_case.kind {
            LitmusKind::OwnerRequestServe => (
                RequiredEdge::OwnerRequestServe,
                BadOutcome::ServeWithoutPriorRequest,
                vec![Action::OwnerRequest, Action::OwnerServe],
                vec![Action::ArmUnrequestedServe, Action::ServeWithoutRequest],
            ),
            LitmusKind::StoreBuffering => {
                let actions = if required_edges.contains(&RequiredEdge::PublishObserve)
                    || profile != ExecutionProfile::WeakMemoryCalibration
                {
                    vec![
                        Action::StoreBufferWriteLeft,
                        Action::StoreBufferWriteRight,
                        Action::StoreFlushLeft,
                        Action::StoreFlushRight,
                        Action::StoreReadLeftAfterPublish,
                        Action::StoreReadRightAfterPublish,
                    ]
                } else {
                    vec![
                        Action::StoreBufferWriteLeft,
                        Action::StoreBufferWriteRight,
                        Action::StoreReadLeftRelaxed,
                        Action::StoreReadRightRelaxed,
                    ]
                };
                return Self {
                    required_edge: RequiredEdge::PublishObserve,
                    missing_edge: None,
                    bad_outcome: None,
                    actions,
                };
            }
            LitmusKind::PublicationObservation => (
                RequiredEdge::PublishObserve,
                BadOutcome::ObservedBeforePublish,
                vec![Action::PublishResult, Action::ObservePublishedResult],
                vec![Action::ObserveBeforePublish, Action::PublishResult],
            ),
            LitmusKind::WitnessCreationUse => (
                RequiredEdge::WitnessCreateUse,
                BadOutcome::WitnessUseBeforeCreate,
                vec![Action::CreateWitness, Action::UseCreatedWitness],
                vec![Action::ArmStaleWitness, Action::UseStaleWitness],
            ),
            LitmusKind::CapabilityRevokeUse => (
                RequiredEdge::CapabilityGrantUse,
                BadOutcome::CapabilityUseBeforeGrant,
                vec![
                    Action::GrantCapability,
                    Action::UseGrantedCapability,
                    Action::EnqueueGenerationZero,
                    Action::RevokePublishGenerationOne,
                    Action::RejectStaleServe,
                    Action::ForgeSourceFreeAuthorityAttempt,
                ],
                vec![
                    Action::ArmCapabilityMismatch,
                    Action::UseMismatchedCapability,
                ],
            ),
            LitmusKind::PatchActivateRequest => (
                RequiredEdge::PatchActivationVisibility,
                BadOutcome::RequestCrossesPatchActivationFrontier,
                vec![
                    Action::RecordPatchRequestGenerationZero,
                    Action::ActivatePatchGenerationOne,
                    Action::RejectStalePatchRequest,
                ],
                vec![
                    Action::RecordPatchRequestGenerationZero,
                    Action::ActivatePatchGenerationOne,
                    Action::ExecuteStalePatchRequest,
                ],
            ),
            LitmusKind::SaveCutMutation => (
                RequiredEdge::CutSaveQuiescence,
                BadOutcome::MutationEscapesSaveCut,
                vec![
                    Action::SaveCutGenerationOne,
                    Action::RejectMutationAfterSaveCut,
                ],
                vec![Action::SaveCutGenerationOne, Action::MutateAfterSaveCut],
            ),
            LitmusKind::RelationEpochSample => (
                RequiredEdge::RelationEpochSample,
                BadOutcome::RelationSampleMixesEpochs,
                vec![
                    Action::PrepareRelationEpochOne,
                    Action::SampleCoherentRelationEpochs,
                ],
                vec![
                    Action::PrepareFallbackEpochZero,
                    Action::SampleMixedRelationEpochs,
                ],
            ),
            LitmusKind::SameOwnerRmw => (
                RequiredEdge::SameOwnerReadsFromCoherence,
                BadOutcome::SameOwnerRmwReadsFromStaleWrite,
                vec![Action::FirstOwnerRmw, Action::SecondOwnerRmwReadsCoherent],
                vec![Action::FirstOwnerRmw, Action::SecondOwnerRmwReadsStale],
            ),
            LitmusKind::PresentationGap => (
                RequiredEdge::PresentationGapNonmutation,
                BadOutcome::PresentationGapMutatesSemanticLineage,
                vec![
                    Action::PresentationGap,
                    Action::PreserveLineageForPresentationGap,
                ],
                vec![
                    Action::PresentationGap,
                    Action::MutateLineageForPresentationGap,
                ],
            ),
        };

        if litmus_case.kind == LitmusKind::CapabilityRevokeUse
            && required_edges.contains(&RequiredEdge::CapabilityGrantUse)
            && !required_edges.contains(&RequiredEdge::RevocationVisibility)
        {
            return Self {
                required_edge: RequiredEdge::RevocationVisibility,
                missing_edge: Some(RequiredEdge::RevocationVisibility),
                bad_outcome: Some(BadOutcome::StaleServeAfterRevocation),
                actions: vec![
                    Action::EnqueueGenerationZero,
                    Action::RevokePublishGenerationOne,
                    Action::StaleServeWriteGenerationZero,
                ],
            };
        }

        if required_edges.contains(&required_edge) {
            Self {
                required_edge,
                missing_edge: None,
                bad_outcome: None,
                actions: safe_actions,
            }
        } else {
            Self {
                required_edge,
                missing_edge: Some(required_edge),
                bad_outcome: Some(bad_outcome),
                actions: unsafe_actions,
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ExploredCase {
    first_bad: Option<(BadPredicate, Vec<ModelTransition>, ModelState)>,
    case_report: CaseReport,
}

#[derive(Clone, Debug)]
struct ExplorationNode {
    state: ModelState,
    state_id: usize,
    transitions: Vec<ModelTransition>,
}

fn explore_case(
    initial: &ModelState,
    litmus_name: &str,
    plan: &CasePlan,
    bound: usize,
    states: &mut Vec<ModelState>,
    transition_relation: &mut Vec<ModelTransition>,
) -> ExploredCase {
    let initial_id = intern_state(states, initial.clone());
    let mut queue = VecDeque::from([ExplorationNode {
        state: initial.clone(),
        state_id: initial_id,
        transitions: Vec::new(),
    }]);
    let mut terminals = Vec::new();
    let mut first_bad = None;
    let mut local_states = vec![initial.clone()];
    let mut local_transition_count = 0usize;
    let mut covered_actions = BTreeSet::new();

    while let Some(node) = queue.pop_front() {
        if node.transitions.len() >= bound {
            terminals.push(node.state);
            continue;
        }

        let mut admitted = 0usize;
        for action in &plan.actions {
            let Ok(next_state) = node.state.apply(*action) else {
                continue;
            };
            admitted += 1;
            local_transition_count += 1;
            covered_actions.insert(action.key());
            if !local_states.iter().any(|state| state == &next_state) {
                local_states.push(next_state.clone());
            }
            let target_state_id = intern_state(states, next_state.clone());
            let transition = ModelTransition::new(
                node.state_id,
                target_state_id,
                litmus_name,
                Some(plan.required_edge),
                *action,
                &node.state,
                &next_state,
            );
            let mut transitions = node.transitions.clone();
            transitions.push(transition.clone());
            transition_relation.push(transition);

            if let Some(outcome) = plan.bad_outcome {
                let predicate = BadPredicate { outcome };
                if predicate.holds(&next_state) && first_bad.is_none() {
                    first_bad = Some((predicate, transitions.clone(), next_state.clone()));
                }
            }

            queue.push_back(ExplorationNode {
                state: next_state,
                state_id: target_state_id,
                transitions,
            });
        }
        if admitted == 0 {
            terminals.push(node.state);
        }
    }

    if terminals.is_empty() {
        terminals.push(initial.clone());
    }
    let mut unique_terminals = Vec::new();
    for terminal in terminals {
        if !unique_terminals.iter().any(|state| state == &terminal) {
            unique_terminals.push(terminal);
        }
    }
    let terminal_state_count = unique_terminals.len();
    let mut observable_outcomes = BTreeSet::new();
    let mut rejected_state_mutations = Vec::new();
    let mut semantic_mutations_after_revocation = BTreeSet::new();
    for terminal in unique_terminals {
        observable_outcomes.extend(terminal.observable_outcomes.iter().cloned());
        rejected_state_mutations.extend(terminal.rejected_authority_use_mutations.iter().cloned());
        if terminal
            .authority_lineage("self:S:attack")
            .is_some_and(|lineage| lineage.capability_status == AuthorityStatus::Revoked)
            && terminal.semantic_generation("player[target].hp") == Some(1)
        {
            semantic_mutations_after_revocation.insert("player[target].hp".to_owned());
        }
    }
    rejected_state_mutations.sort();
    rejected_state_mutations.dedup();
    ExploredCase {
        first_bad,
        case_report: CaseReport {
            observable_outcomes,
            rejected_state_mutations,
            semantic_mutations_after_revocation,
            explored_state_count: local_states.len(),
            explored_transition_count: local_transition_count,
            terminal_state_count,
            expected_action_count: plan.actions.len(),
            covered_action_count: covered_actions.len(),
        },
    }
}

fn intern_state(states: &mut Vec<ModelState>, candidate: ModelState) -> usize {
    if let Some(index) = states.iter().position(|state| state == &candidate) {
        index
    } else {
        states.push(candidate);
        states.len() - 1
    }
}

/// The internal finite transition alphabet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    ArmUnrequestedServe,
    ServeWithoutRequest,
    OwnerRequest,
    OwnerServe,
    PublishResult,
    ObserveBeforePublish,
    ObservePublishedResult,
    ArmStaleWitness,
    UseStaleWitness,
    CreateWitness,
    UseCreatedWitness,
    ArmCapabilityMismatch,
    UseMismatchedCapability,
    GrantCapability,
    UseGrantedCapability,
    EnqueueGenerationZero,
    RevokePublishGenerationOne,
    StaleServeWriteGenerationZero,
    RejectStaleServe,
    RecordPatchRequestGenerationZero,
    ActivatePatchGenerationOne,
    RejectStalePatchRequest,
    ExecuteStalePatchRequest,
    SaveCutGenerationOne,
    MutateAfterSaveCut,
    RejectMutationAfterSaveCut,
    PrepareFallbackEpochZero,
    SampleMixedRelationEpochs,
    PrepareRelationEpochOne,
    SampleCoherentRelationEpochs,
    FirstOwnerRmw,
    SecondOwnerRmwReadsStale,
    SecondOwnerRmwReadsCoherent,
    PresentationGap,
    MutateLineageForPresentationGap,
    PreserveLineageForPresentationGap,
    ForgeSourceFreeAuthorityAttempt,
    StoreBufferWriteLeft,
    StoreBufferWriteRight,
    StoreFlushLeft,
    StoreFlushRight,
    StoreReadLeftRelaxed,
    StoreReadRightRelaxed,
    StoreReadLeftAfterPublish,
    StoreReadRightAfterPublish,
}

impl Action {
    fn key(self) -> &'static str {
        match self {
            Self::ArmUnrequestedServe => "arm_unrequested_serve",
            Self::ServeWithoutRequest => "serve_without_request",
            Self::OwnerRequest => "owner_request",
            Self::OwnerServe => "owner_serve",
            Self::PublishResult => "publish_result",
            Self::ObserveBeforePublish => "observe_before_publish",
            Self::ObservePublishedResult => "observe_published_result",
            Self::ArmStaleWitness => "arm_stale_witness",
            Self::UseStaleWitness => "use_stale_witness",
            Self::CreateWitness => "create_witness",
            Self::UseCreatedWitness => "use_created_witness",
            Self::ArmCapabilityMismatch => "arm_capability_mismatch",
            Self::UseMismatchedCapability => "use_mismatched_capability",
            Self::GrantCapability => "grant_capability",
            Self::UseGrantedCapability => "use_granted_capability",
            Self::EnqueueGenerationZero => "enqueue_g0",
            Self::RevokePublishGenerationOne => "revoke_publish_g1",
            Self::StaleServeWriteGenerationZero => "stale_serve_write_g0",
            Self::RejectStaleServe => "reject_stale_serve_g0",
            Self::RecordPatchRequestGenerationZero => "record_patch_request_g0",
            Self::ActivatePatchGenerationOne => "activate_patch_g1",
            Self::RejectStalePatchRequest => "reject_stale_patch_request",
            Self::ExecuteStalePatchRequest => "execute_stale_patch_request",
            Self::SaveCutGenerationOne => "save_cut_g1",
            Self::MutateAfterSaveCut => "mutate_after_save_cut",
            Self::RejectMutationAfterSaveCut => "reject_mutation_after_save_cut",
            Self::PrepareFallbackEpochZero => "prepare_fallback_epoch_0",
            Self::SampleMixedRelationEpochs => "sample_mixed_relation_epochs",
            Self::PrepareRelationEpochOne => "prepare_relation_epoch_1",
            Self::SampleCoherentRelationEpochs => "sample_coherent_relation_epochs",
            Self::FirstOwnerRmw => "first_owner_rmw",
            Self::SecondOwnerRmwReadsStale => "second_owner_rmw_reads_stale",
            Self::SecondOwnerRmwReadsCoherent => "second_owner_rmw_reads_coherent",
            Self::PresentationGap => "presentation_gap",
            Self::MutateLineageForPresentationGap => "mutate_lineage_for_presentation_gap",
            Self::PreserveLineageForPresentationGap => "preserve_lineage_for_presentation_gap",
            Self::ForgeSourceFreeAuthorityAttempt => "reject_source_free_authority_mint",
            Self::StoreBufferWriteLeft => "store_buffer_write_left",
            Self::StoreBufferWriteRight => "store_buffer_write_right",
            Self::StoreFlushLeft => "store_flush_left",
            Self::StoreFlushRight => "store_flush_right",
            Self::StoreReadLeftRelaxed => "store_read_left_relaxed",
            Self::StoreReadRightRelaxed => "store_read_right_relaxed",
            Self::StoreReadLeftAfterPublish => "store_read_left_after_publish",
            Self::StoreReadRightAfterPublish => "store_read_right_after_publish",
        }
    }

    fn event_name(self) -> &'static str {
        match self {
            Self::EnqueueGenerationZero => "enqueue@g0",
            Self::RevokePublishGenerationOne => "revoke_publish@g1",
            Self::StaleServeWriteGenerationZero => "stale_serve_write@g0",
            _ => self.key(),
        }
    }
}

#[cfg(test)]
mod review_regression_tests {
    use super::*;

    fn selected_model(profile: ExecutionProfile) -> Sys2BoundedModel {
        Sys2BoundedModel::new()
            .with_profile(profile)
            .with_bound(6)
            .with_required_edges([
                RequiredEdge::OwnerRequestServe,
                RequiredEdge::PublishObserve,
                RequiredEdge::WitnessCreateUse,
                RequiredEdge::CapabilityGrantUse,
                RequiredEdge::RevocationVisibility,
                RequiredEdge::PatchActivationVisibility,
                RequiredEdge::CutSaveQuiescence,
                RequiredEdge::RelationEpochSample,
                RequiredEdge::SameOwnerReadsFromCoherence,
                RequiredEdge::PresentationGapNonmutation,
            ])
            .with_litmus_cases([
                LitmusCase::owner_request_serve_message_passing(),
                LitmusCase::store_buffering_calibration(),
                LitmusCase::publication_observation(),
                LitmusCase::witness_creation_use(),
                LitmusCase::capability_revoke_use_race(),
                LitmusCase::patch_activate_request_race(),
                LitmusCase::save_cut_mutation_race(),
                LitmusCase::relation_epoch_sample_race(),
                LitmusCase::same_owner_two_request_rmw(),
                LitmusCase::presentation_gap_nonmutation(),
            ])
    }

    #[test]
    fn replay_rejects_reordering_of_commuting_store_writes() {
        let initial = ModelState::initial();
        let after_left = initial
            .apply(Action::StoreBufferWriteLeft)
            .expect("left buffer write is admitted");
        let after_both = after_left
            .apply(Action::StoreBufferWriteRight)
            .expect("right buffer write is admitted after left");
        let left = ModelTransition::new(
            0,
            1,
            "store_buffering_calibration",
            Some(RequiredEdge::PublishObserve),
            Action::StoreBufferWriteLeft,
            &initial,
            &after_left,
        );
        let right = ModelTransition::new(
            1,
            2,
            "store_buffering_calibration",
            Some(RequiredEdge::PublishObserve),
            Action::StoreBufferWriteRight,
            &after_left,
            &after_both,
        );

        assert_eq!(after_both.markers.len(), 2);
        let replay = ModelTrace::replay(initial, vec![right, left]);
        assert!(
            matches!(replay, Err(error) if error.is_invalid_transition_sequence()),
            "transition relation identity must reject a reordered but state-commuting trace"
        );
    }

    #[test]
    fn stale_patch_rejection_is_not_stale_patch_execution() {
        let initial = ModelState::initial();
        let requested = initial
            .apply(Action::RecordPatchRequestGenerationZero)
            .expect("g0 request is admitted");
        let activated = requested
            .apply(Action::ActivatePatchGenerationOne)
            .expect("g1 patch activates");
        let rejected = activated
            .apply(Action::RejectStalePatchRequest)
            .expect("stale g0 request is rejected");

        assert!(
            !BadPredicate {
                outcome: BadOutcome::RequestCrossesPatchActivationFrontier,
            }
            .holds(&rejected),
            "rejection must not be classified as stale execution"
        );
        assert_eq!(rejected.stale_patch_rejection_count(), 1);
        assert_eq!(rejected.stale_patch_execution_count(), 0);
    }

    #[test]
    fn store_buffering_uses_buffer_flush_and_reads_from_state() {
        let relaxed = ModelState::initial()
            .apply(Action::StoreBufferWriteLeft)
            .and_then(|state| state.apply(Action::StoreBufferWriteRight))
            .and_then(|state| state.apply(Action::StoreReadLeftRelaxed))
            .and_then(|state| state.apply(Action::StoreReadRightRelaxed))
            .expect("finite weak calibration run is admitted");
        assert_eq!(relaxed.store_read("r1"), Some(0));
        assert_eq!(relaxed.store_reads_from("r1"), Some("initial:y"));
        assert_eq!(relaxed.store_read("r2"), Some(0));
        assert_eq!(relaxed.store_reads_from("r2"), Some("initial:x"));

        let published = ModelState::initial()
            .apply(Action::StoreBufferWriteLeft)
            .and_then(|state| state.apply(Action::StoreBufferWriteRight))
            .and_then(|state| state.apply(Action::StoreFlushLeft))
            .and_then(|state| state.apply(Action::StoreFlushRight))
            .and_then(|state| state.apply(Action::StoreReadLeftAfterPublish))
            .and_then(|state| state.apply(Action::StoreReadRightAfterPublish))
            .expect("declared publication visibility run is admitted");
        assert_eq!(published.store_read("r1"), Some(1));
        assert_eq!(published.store_reads_from("r1"), Some("right:y@flush"));
        assert_eq!(published.store_read("r2"), Some(1));
        assert_eq!(published.store_reads_from("r2"), Some("left:x@flush"));
    }

    #[test]
    fn pre_publish_observation_is_reached_before_later_publication() {
        let pre_publish = ModelState::initial()
            .apply(Action::ObserveBeforePublish)
            .expect("missing publish-observe edge admits pre-publish observation");
        assert_eq!(pre_publish.published_version("result"), None);
        assert_eq!(pre_publish.observed_version("result", "ViewerC"), Some(0));
        let reached = pre_publish
            .apply(Action::PublishResult)
            .expect("later publication is admitted");
        assert!(
            BadPredicate {
                outcome: BadOutcome::ObservedBeforePublish,
            }
            .holds(&reached)
        );
    }

    #[test]
    fn selected_st_and_ow_results_match_with_deterministic_case_coverage() {
        let st = selected_model(ExecutionProfile::SingleThread).check();
        let ow = selected_model(ExecutionProfile::OneOwnerWorker).check();
        let ow_repeat = selected_model(ExecutionProfile::OneOwnerWorker).check();

        assert!(st.selected_semantic_results_match(&ow));
        assert_eq!(
            ow.deterministic_fingerprint(),
            ow_repeat.deterministic_fingerprint()
        );
        assert!(ow.no_source_free_authority_mints());
        assert!(ow.rejected_authority_attempts().iter().any(|attempt| {
            attempt.reason() == "source-free-authority-mint"
                && attempt.attempted_lineage().principal() == "forged"
                && attempt.preserves_semantic_state()
        }));
        let direct_rejection = ModelState::initial()
            .apply(Action::ForgeSourceFreeAuthorityAttempt)
            .expect("forged source-free authority is rejected, not admitted");
        assert_eq!(direct_rejection.rejected_authority_attempts().len(), 1);
        assert!(direct_rejection.rejected_authority_attempts()[0].preserves_semantic_state());
        for case in [
            "owner_request_serve_message_passing",
            "store_buffering_calibration",
            "publication_observation",
            "witness_creation_use",
            "capability_revoke_use_race",
            "patch_activate_request_race",
            "save_cut_mutation_race",
            "relation_epoch_sample_race",
            "same_owner_two_request_rmw",
            "presentation_gap_nonmutation",
        ] {
            let case = ow.case_report(case).expect("selected case exists");
            assert!(case.explored_state_count() > 1);
            assert!(case.terminal_state_count() > 0);
            assert_eq!(case.covered_action_count(), case.expected_action_count());
            assert!(case.explored_transition_count() >= case.expected_action_count());
        }
        let owner = ow
            .case_report("owner_request_serve_message_passing")
            .expect("owner case exists");
        assert_eq!(owner.expected_action_count(), 2);
        assert_eq!(owner.explored_state_count(), 3);
        assert_eq!(owner.explored_transition_count(), 2);
        assert_eq!(owner.terminal_state_count(), 1);
    }
}
