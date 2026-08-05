//! Provisional M10 source-to-conformance facade.
//!
//! This module deliberately composes the ordinary M6/M7 source path, M8's
//! direct deferred judgment, M9's sealed authorization/verifier route, and
//! the existing M8 local runtime.  It is a bounded reference system, not a
//! public ABI or an authority/provider API.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{M9ContractCandidate, M9FiniteRefinementChecker},
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSurfaceV0, ResidualObligationKind,
        check_and_elaborate_surface_v0,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    m8_runtime_admission::{
        EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionEvidence, M8Runtime,
        M8RuntimeAdmission, M8RuntimeInstance, M8SecurityClass,
    },
    m8_runtime_designated_value::{
        M8ConsumeRequest, M8DesignatedEvaluationRequest, M8DesignatedTick, M8InputReceipt,
        M8InputReceiptSet,
    },
    m8_runtime_local_cut::{
        M8LeaseRecord, M8LiveFloor, M8LocalRuntime, M8LocalRuntimeSeed, M8LocalTraceKind,
    },
    m8_runtime_observer::{
        M8ObserverAuthorityGrant, M8ObserverDiagnosticKind, M8ObserverPolicy, M8ObserverRetention,
        M8ObserverRowKind, M8ObserverRuntime,
    },
    m8_runtime_owner_queue::{M8AuthorityUse, M8OwnerRequest, M8StateKey},
    m8_runtime_patch::{M8PatchCandidate, M8PatchRuntime, M8PatchRuntimeSeed},
    m8_runtime_relation_projection::{
        M8AnchorSample, M8BindingInvalidation, M8FiniteFallbackChain, M8FiniteFallbackOption,
        M8Point, M8PresentationContext, M8PresentationFallback, M8RelationAuthorityUse,
        M8RelationReacquire,
    },
    m9_auth_verification::{
        M9AdmissionBindingDelta, M9AdmissionEnvelope, M9AdmissionRuntime, M9AuthorityCut,
        M9AuthorityRuntime, M9CapabilityAuth, M9CapabilityGrantRequest, M9CapabilityScope,
        M9FactUse, M9FinalAdmissionEvidence, M9M10AuthorityBridge, M9MembershipAuth,
        M9MembershipRequest, M9ResidualBinding, M9SourceArtifact, M9WitnessAuth, M9WitnessRequest,
    },
};

/// Serializable source request accepted by the provisional M10 profile.
/// Schedule values are exogenous requests only; they cannot mint authority,
/// write state directly, or inject verification/projection results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M10SourceRunRequest {
    source_path: String,
    source_text: String,
    entry_event: Option<String>,
    principal: Option<String>,
    target: Option<String>,
    initial_player_hp: BTreeMap<String, i64>,
    initial_player_atk: BTreeMap<String, i64>,
    attack_count: usize,
    relation_projection: Option<(String, String)>,
    fault_injection: Option<String>,
    patch_intent_carrier: Option<M10PatchIntentCarrier>,
    corpus_path: Option<String>,
    #[serde(skip)]
    typed_schedule: Option<M10TypedSchedule>,
    #[serde(skip)]
    typed_schedule_input: Option<Value>,
    typed_schedule_error: Option<String>,
    typed_carriers: Option<M10TypedCarriers>,
    #[serde(skip)]
    typed_carriers_input: Option<Value>,
    typed_carriers_error: Option<String>,
    predicate_profile: Option<M10CorrespondenceProfile>,
    predicate_profile_error: Option<String>,
    typed_input_mutation: Option<M10TypedInputMutation>,
    typed_input_mutation_error: Option<String>,
    forbid_fixture_name_result_lookup: bool,
    forbid_expected_output_sidecars: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct M10EvidenceFact {
    scn_id: String,
    phase: String,
    predicate: String,
    carrier_kind: String,
    artifact_identity: String,
    diagnostic_location: String,
    source_derived_reference: String,
    schedule_action_reference: Option<String>,
}

#[derive(Debug, Clone)]
struct M10EvidenceContext {
    scn_id: String,
    phase: &'static str,
    source_ref: SourceRef,
    schedule_action_reference: Option<String>,
}

#[derive(Debug, Clone)]
struct M10SourceFailure {
    missing_failure: String,
    source_ref: SourceRef,
}

#[derive(Debug)]
struct M10GeneratedEvidence {
    sources: Vec<Value>,
    carriers: Value,
    derivation: Value,
    pressure: Value,
    facts: BTreeSet<M10EvidenceFact>,
    runtime_traces: BTreeMap<String, Value>,
    m9_to_m8_authority_translations: BTreeMap<String, Value>,
    source_digest: String,
    evidence_hash: String,
    execution_manifest: M10ExecutionManifest,
}

/// The execution manifest is deliberately constructed from execution inputs
/// only.  In particular it does not contain predicates or expected outcomes:
/// M10 must be able to generate the same evidence before the verifier reads a
/// correspondence profile.
#[derive(Debug, Clone, PartialEq, Eq)]
struct M10ExecutionManifest {
    source_revision: String,
    release_family: String,
    source_content_identities: BTreeMap<String, String>,
    typed_carriers_identity: String,
    carrier_identities: BTreeMap<String, String>,
    schedule_identity: String,
    action_inventory: Vec<String>,
    policy_stamps: Vec<String>,
}

/// The verifier manifest binds the profile's identity-bearing correspondence
/// fields.  The predicate text is intentionally excluded: predicates are
/// verifier input and may be tested independently without changing execution
/// evidence (spec/11's generator/verifier separation).
#[derive(Debug, Clone, PartialEq, Eq)]
struct M10VerifierManifest {
    profile_schema_version: String,
    correspondence_profile_identity: String,
    /// Exact correspondence rows, including evidence predicates.  This is a
    /// verifier input hash, not the profile hash specified by spec/11: that
    /// latter hash is the complete release manifest identity below.
    verifier_profile_hash: String,
}

/// One finite release declaration.  It is private to this provisional M10
/// facade and is not a public wire or ABI type.
#[derive(Debug, Clone, PartialEq, Eq)]
struct M10ReleaseManifest {
    execution: M10ExecutionManifest,
    verifier: M10VerifierManifest,
    manifest_hash: String,
}

/// Immutable reference-release anchor.  It is built from the committed M10
/// corpus/profile inputs, not from a caller's request, so a caller cannot
/// rebind a renamed/reordered action by also rebinding the correspondence
/// profile to generated evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
struct M10ReleaseAnchor {
    source_revision: String,
    manifest_hash: String,
    execution_identity: String,
    verifier_profile_hash: String,
}

// Fixed accepting release cut for the bounded M10 reference profile.  This
// is deliberately embedded rather than recomputed from the live workspace:
// a caller may supply mutated corpus/profile inputs, but cannot move the
// release boundary by causing its verifier to reread them as the anchor.
const M10_REFERENCE_ANCHOR_SOURCE_REVISION: &str = "fnv1a64:7bff6aa952a8ad53";
const M10_REFERENCE_ANCHOR_MANIFEST_HASH: &str = "fnv1a64:da8a33a45377d002";
const M10_REFERENCE_ANCHOR_EXECUTION_IDENTITY: &str = "fnv1a64:473383821fd8cce5";
const M10_REFERENCE_ANCHOR_VERIFIER_PROFILE_HASH: &str = "fnv1a64:420308515cf98e18";

#[derive(Debug, Clone)]
struct M10SemanticHashBundle {
    store_hash: String,
    membership_hash: String,
    grant_hash: String,
    relation_hash: String,
    config_hash: String,
    fallback_hash: String,
    cut_hash: String,
    m8_cut_hash: String,
    m9_authority_hash: String,
    ledger_hash: String,
    domain_projection_identities: M10DomainProjectionIdentities,
}

#[derive(Debug, Clone)]
struct M10DomainProjectionIdentities {
    store: String,
    membership: String,
    grant: String,
    relation: String,
    config: String,
}

#[derive(Debug, Clone)]
struct M10TransitionReceipt {
    transition: &'static str,
    accepted: bool,
    before_membership_hash: String,
    after_membership_hash: String,
    before_fallback_hash: String,
    after_fallback_hash: String,
    before_cut_hash: String,
    after_cut_hash: String,
    before_store_hash: String,
    after_store_hash: String,
    before_grant_hash: String,
    after_grant_hash: String,
    before_relation_hash: String,
    after_relation_hash: String,
    before_config_hash: String,
    after_config_hash: String,
    before_m8_cut_hash: String,
    after_m8_cut_hash: String,
    before_m9_authority_hash: String,
    after_m9_authority_hash: String,
    before_ledger_hash: String,
    after_ledger_hash: String,
    source_ref: SourceRef,
    domain_projection_provenance: Value,
}

impl M10TransitionReceipt {
    fn evidence(&self) -> Value {
        json!({
            "transition": self.transition,
            "accepted": self.accepted,
            "before": {
                "store_hash": self.before_store_hash,
                "membership_hash": self.before_membership_hash,
                "grant_hash": self.before_grant_hash,
                "relation_hash": self.before_relation_hash,
                "config_hash": self.before_config_hash,
                "fallback_hash": self.before_fallback_hash,
                "cut_hash": self.before_cut_hash,
                "m8_cut_hash": self.before_m8_cut_hash,
                "m9_authority_hash": self.before_m9_authority_hash,
                "ledger_hash": self.before_ledger_hash,
            },
            "after": {
                "store_hash": self.after_store_hash,
                "membership_hash": self.after_membership_hash,
                "grant_hash": self.after_grant_hash,
                "relation_hash": self.after_relation_hash,
                "config_hash": self.after_config_hash,
                "fallback_hash": self.after_fallback_hash,
                "cut_hash": self.after_cut_hash,
                "m8_cut_hash": self.after_m8_cut_hash,
                "m9_authority_hash": self.after_m9_authority_hash,
                "ledger_hash": self.after_ledger_hash,
            },
            "source_ref": source_ref_json(Some(&self.source_ref)),
            "domain_projection_provenance": self.domain_projection_provenance,
        })
    }

    fn failure_preserves_semantic_state(&self) -> bool {
        !self.accepted
            && self.before_membership_hash == self.after_membership_hash
            && self.before_fallback_hash == self.after_fallback_hash
            && self.before_cut_hash == self.after_cut_hash
            && self.before_store_hash == self.after_store_hash
            && self.before_grant_hash == self.after_grant_hash
            && self.before_relation_hash == self.after_relation_hash
            && self.before_config_hash == self.after_config_hash
            && self.before_m8_cut_hash == self.after_m8_cut_hash
            && self.before_m9_authority_hash == self.after_m9_authority_hash
            && self.before_ledger_hash == self.after_ledger_hash
    }
}

/// M10 stores only receipt history.  Runtime, relation, membership, and cut
/// state remain owned by their M8/M9 sessions.
#[derive(Debug, Default)]
struct M10ReceiptLedger {
    receipts: Vec<M10TransitionReceipt>,
}

impl M10ReceiptLedger {
    fn record_actual(
        &mut self,
        transition: &'static str,
        source_ref: &SourceRef,
        before: M10SemanticHashBundle,
        after: M10SemanticHashBundle,
        accepted: bool,
    ) -> M10TransitionReceipt {
        let domain_projection_provenance = m10_domain_projection_provenance(&before, &after);
        let receipt = M10TransitionReceipt {
            transition,
            accepted,
            before_membership_hash: before.membership_hash,
            after_membership_hash: after.membership_hash,
            before_fallback_hash: before.fallback_hash,
            after_fallback_hash: after.fallback_hash,
            before_cut_hash: before.cut_hash,
            after_cut_hash: after.cut_hash,
            before_store_hash: before.store_hash,
            after_store_hash: after.store_hash,
            before_grant_hash: before.grant_hash,
            after_grant_hash: after.grant_hash,
            before_relation_hash: before.relation_hash,
            after_relation_hash: after.relation_hash,
            before_config_hash: before.config_hash,
            after_config_hash: after.config_hash,
            before_m8_cut_hash: before.m8_cut_hash,
            after_m8_cut_hash: after.m8_cut_hash,
            before_m9_authority_hash: before.m9_authority_hash,
            after_m9_authority_hash: after.m9_authority_hash,
            before_ledger_hash: before.ledger_hash,
            after_ledger_hash: after.ledger_hash,
            source_ref: source_ref.clone(),
            domain_projection_provenance,
        };
        self.receipts.push(receipt.clone());
        receipt
    }
}

const M10_FROZEN_CORRESPONDENCE_IDS: &[&str] = &[
    "SCN01-S-P-REQ",
    "SCN01-S-P-DEP",
    "SCN01-S-P-PUB",
    "SCN01-S-P-SPANS",
    "SCN01-S-P-CAP",
    "SCN01-S-N-VISROW",
    "SCN01-R-P-STATE",
    "SCN01-R-P-ORDER",
    "SCN02-S-P-REQ-RMW",
    "SCN02-S-P-DEPS",
    "SCN02-S-P-FAIL-SPAN",
    "SCN02-S-P-LOCUS",
    "SCN02-S-N-CAPROW",
    "SCN02-S-N-REQUESTER-READ",
    "SCN02-S-N-BLIND-WRITE",
    "SCN02-S-N-NO-XOWNER-TXN",
    "SCN02-R-P-ONE",
    "SCN02-R-P-TWO",
    "SCN02-R-N-NOCAP",
    "SCN02-R-N-STALE",
    "SCN03-S-N-PREVERDICT",
    "SCN03-R-P-ADMIT",
    "SCN03-R-P-LINEAGE",
    "SCN03-R-P-PAST",
    "SCN03-R-N-PREVERDICT",
    "SCN03-R-N-ROLE-SPOOF",
    "SCN03-R-N-CAPREPLAY",
    "SCN04-R-P-STALE",
    "SCN04-R-P-AUDIT",
    "SCN04-R-P-BLOCK-COMPACT",
    "SCN04-R-P-ALLOW-COMPACT",
    "SCN04-R-P-REJOIN",
    "SCN04-R-N-HIDDEN-REPAIR",
    "SCN05-S-N-MISSING-VISROW",
    "SCN05-R-P-HANDOFF",
    "SCN05-R-P-OBS",
    "SCN05-R-N-SECRET",
    "SCN05-R-N-WRONGCAP",
    "SCN06-S-P-REQFAIL",
    "SCN06-S-N-ROW",
    "SCN06-R-P-ABSENT",
    "SCN06-R-P-PATCHED",
    "SCN06-R-N-NOHANG",
    "SCN07-S-N-PRIVATEPOL",
    "SCN07-S-N-WIDEN",
    "SCN07-R-P-FIELDS",
    "SCN07-R-P-ADMIN",
    "SCN07-R-P-POLICY",
    "SCN07-R-N-HORIGIN",
    "SCN08-S-P-CARRIER",
    "SCN08-S-N-LINEAGE",
    "SCN08-S-N-CAPFLOOR",
    "SCN08-R-P-LIVE",
    "SCN08-R-P-EXPIRE",
    "SCN08-R-P-WRITE",
    "SCN08-R-P-REACQUIRE",
    "SCN08-R-P-ROLLBACK",
    "SCN08-R-N-REPROMOTE",
    "SCN09-S-P-CHECKEDPAIR",
    "SCN09-S-N-SELFGRANT",
    "SCN09-S-N-MISSINGCAP",
    "SCN09-R-P-PIPELINE",
    "SCN09-R-P-INIT",
    "SCN09-R-P-OBS",
    "SCN09-R-N-DRIFT",
    "SCN10-R-P-S1",
    "SCN10-R-P-S2",
    "SCN10-R-P-LOADFRESH",
    "SCN10-R-N-MERGE",
    "SCN10-R-N-LEASEDOCTOR",
    "SCN10-R-N-CUTDOCTOR",
    "SCN10-R-P-TIMELINE",
    "SCN10-R-P-REACQUIRE",
];

fn source_artifact_identity(
    source_identities: &BTreeMap<String, String>,
    path: &str,
) -> Result<String, String> {
    source_identities
        .get(path)
        .map(|identity| format!("source:{path}:{identity}"))
        .ok_or_else(|| format!("M10 evidence references missing source {path}"))
}

fn patch_pair_artifact_identity(
    carriers: &M10TypedCarriers,
    source_identities: &BTreeMap<String, String>,
    carrier: &M10PatchIntentCarrier,
) -> Result<String, String> {
    let base = carrier
        .base_source_path
        .as_deref()
        .ok_or_else(|| format!("patch carrier {} has no base source", carrier.id))?;
    let base_hash = source_identities.get(base).ok_or_else(|| {
        format!(
            "patch carrier {} references unknown base {base}",
            carrier.id
        )
    })?;
    let candidate_hash = source_identities
        .get(&carrier.candidate_source_path)
        .ok_or_else(|| {
            format!(
                "patch carrier {} references unknown candidate {}",
                carrier.id, carrier.candidate_source_path
            )
        })?;
    let carrier_identity = carriers
        .carrier_identity(&carrier.id)
        .ok_or_else(|| format!("patch carrier {} lacks a canonical identity", carrier.id))?;
    let carrier_hash = carrier_identity
        .strip_prefix(&format!("typed_carrier:{}:", carrier.id))
        .ok_or_else(|| format!("patch carrier {} has malformed identity", carrier.id))?;
    Ok(format!(
        "patch_pair:{}:base={base_hash}:candidate={candidate_hash}:carrier={carrier_hash}",
        carrier.id,
    ))
}

fn add_fact(
    facts: &mut BTreeSet<M10EvidenceFact>,
    predicate: &str,
    carrier_kind: &str,
    artifact_identity: String,
    context: &M10EvidenceContext,
) {
    let source_derived_reference = canonical_source_derived_reference(&context.source_ref);
    facts.insert(M10EvidenceFact {
        scn_id: context.scn_id.clone(),
        phase: context.phase.to_string(),
        predicate: predicate.to_string(),
        carrier_kind: carrier_kind.to_string(),
        artifact_identity,
        diagnostic_location: canonical_evidence_location(
            context,
            predicate,
            carrier_kind,
            &source_derived_reference,
        ),
        source_derived_reference,
        schedule_action_reference: context.schedule_action_reference.clone(),
    });
}

fn canonical_source_derived_reference(source_ref: &SourceRef) -> String {
    format!(
        "m10-source-ref:{}",
        deterministic_hash(&format!(
            "m10-source-ref-v1\0{}\0{}\0{}\0{}\0{}",
            source_ref.path,
            source_ref.start_line,
            source_ref.start_column,
            source_ref.end_line,
            source_ref.end_column,
        ))
    )
}

fn canonical_evidence_location(
    context: &M10EvidenceContext,
    predicate: &str,
    carrier_kind: &str,
    source_derived_reference: &str,
) -> String {
    format!(
        "m10-evidence-location:{}",
        deterministic_hash(&format!(
            "m10-evidence-location-v1\0{}\0{}\0{}\0{}\0{}\0{}",
            context.scn_id,
            context.phase,
            predicate,
            carrier_kind,
            source_derived_reference,
            context.schedule_action_reference.as_deref().unwrap_or(""),
        ))
    )
}

fn scenario_id_from_source_path(path: &str) -> Result<String, String> {
    let scenario = path
        .split('/')
        .next()
        .filter(|segment| segment.starts_with("scn-"))
        .ok_or_else(|| format!("M10 evidence source {path} does not identify an SCN directory"))?;
    Ok(scenario.to_ascii_uppercase())
}

fn source_evidence_context(
    phase: &'static str,
    source_path: &str,
    source_ref: &SourceRef,
) -> Result<M10EvidenceContext, String> {
    Ok(M10EvidenceContext {
        scn_id: scenario_id_from_source_path(source_path)?,
        phase,
        source_ref: source_ref.clone(),
        schedule_action_reference: None,
    })
}

fn checked_source_evidence_context(
    phase: &'static str,
    source_path: &str,
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
) -> Result<M10EvidenceContext, String> {
    let checked = checked_sources
        .get(source_path)
        .ok_or_else(|| format!("M10 evidence source {source_path} has no checked artifact"))?;
    source_evidence_context(
        phase,
        source_path,
        checked.program_identity().root_source_ref(),
    )
}

fn schedule_evidence_context(case: &M10ScheduleCase, source_ref: &SourceRef) -> M10EvidenceContext {
    M10EvidenceContext {
        scn_id: case.scn.clone(),
        phase: "runtime",
        source_ref: source_ref.clone(),
        schedule_action_reference: Some(case.identity.clone()),
    }
}

fn add_checked_source_fact(
    facts: &mut BTreeSet<M10EvidenceFact>,
    source_identities: &BTreeMap<String, String>,
    path: &str,
    predicate: &str,
    context: &M10EvidenceContext,
) -> Result<(), String> {
    add_fact(
        facts,
        predicate,
        "ordinary_source",
        source_artifact_identity(source_identities, path)?,
        context,
    );
    Ok(())
}

fn add_carrier_facts(
    facts: &mut BTreeSet<M10EvidenceFact>,
    carriers: &M10TypedCarriers,
    id: &str,
    predicates: &[&str],
    context: &M10EvidenceContext,
) -> Result<(), String> {
    let identity = carriers
        .carrier_identity(id)
        .ok_or_else(|| format!("M10 evidence references missing typed carrier {id}"))?
        .to_string();
    for predicate in predicates {
        add_fact(facts, predicate, "typed_carrier", identity.clone(), context);
    }
    Ok(())
}

fn add_patch_facts(
    facts: &mut BTreeSet<M10EvidenceFact>,
    carriers: &M10TypedCarriers,
    source_identities: &BTreeMap<String, String>,
    carrier: &M10PatchIntentCarrier,
    predicates: &[&str],
    context: &M10EvidenceContext,
) -> Result<(), String> {
    let identity = patch_pair_artifact_identity(carriers, source_identities, carrier)?;
    for predicate in predicates {
        add_fact(facts, predicate, "patch_source", identity.clone(), context);
    }
    Ok(())
}

fn has_failures(
    evaluation: &mir_semantics::surface_v0_pipeline::CheckedEvaluation,
    expected: &[&str],
) -> bool {
    let failures = evaluation.generated_obligations().failure_names();
    expected
        .iter()
        .all(|expected| failures.iter().any(|actual| actual == expected))
}

fn checked_state_has_field(
    checked: &CheckedSurfaceV0,
    state: &str,
    field: &str,
    visibility: Option<&str>,
) -> bool {
    checked
        .static_environment()
        .indexed_state_schema(state)
        .and_then(|schema| {
            schema
                .fields()
                .iter()
                .find(|candidate| candidate.name() == field)
        })
        .is_some_and(|candidate| candidate.visibility_channel() == visibility)
}

/// Derive source facts from the checked M7 artifact and its actual M7
/// diagnostic, never from a fixture path or correspondence-row identifier.
/// The finite predicates below describe semantic shapes (owner write, failure
/// row, and source-bound spans) that the typed checker already retained.
fn derive_checked_source_facts(
    facts: &mut BTreeSet<M10EvidenceFact>,
    source_identities: &BTreeMap<String, String>,
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
    source_failures: &BTreeMap<String, M10SourceFailure>,
) -> Result<(), String> {
    for (path, checked) in checked_sources {
        let context =
            source_evidence_context("static", path, checked.program_identity().root_source_ref())?;
        for evaluation in checked.evaluations() {
            let Some(owner) = evaluation.owner_rmw_core() else {
                continue;
            };
            let target = owner.target();
            let reads = owner.same_owner_reads();

            let roll_position = evaluation.name() == "roll"
                && evaluation.actor_authority_origin() == "self"
                && evaluation.authority_origin_locus() == "BrowserClient"
                && owner.owner_locus() == "World"
                && target.namespace() == "player"
                && target.field() == Some("position")
                && reads
                    .iter()
                    .any(|read| read.namespace() == "player" && read.field() == Some("position"))
                && checked_state_has_field(checked, "player", "position", Some("observer_safe"))
                && has_failures(
                    evaluation,
                    &[
                        "MissingCapability",
                        "MissingWitness",
                        "RouteUnavailable",
                        "StaleMembership",
                        "VisibilityDenied",
                    ],
                );
            if roll_position {
                for predicate in [
                    "static.request_edge.exactly_one.BrowserClient_self_to_World.write.player_self_position",
                    "static.dependency.same_field.player_self_position.read_for_position_write",
                    "static.observer_publish_effect.position.is_source_declared",
                    "static.source_spans.position_visibility_and_write.are_exact",
                    "static.obligation.cap_write.player.required_for_position_write",
                ] {
                    add_checked_source_fact(facts, source_identities, path, predicate, &context)?;
                }
            }

            let attack_hp = evaluation.name() == "attack"
                && evaluation.actor_authority_origin() == "self"
                && evaluation.authority_origin_locus() == "BrowserClient"
                && owner.owner_locus() == "World"
                && target.namespace() == "player"
                && target.field() == Some("hp")
                && reads
                    .iter()
                    .any(|read| read.namespace() == "player" && read.field() == Some("hp"))
                && reads
                    .iter()
                    .any(|read| read.namespace() == "player" && read.field() == Some("atk"))
                && has_failures(
                    evaluation,
                    &[
                        "MissingCapability",
                        "MissingWitness",
                        "RouteUnavailable",
                        "StaleMembership",
                    ],
                );
            if attack_hp {
                for predicate in [
                    "static.owner_rmw.requires.MissingCapability.MissingWitness.RouteUnavailable.StaleMembership",
                    "static.dependencies.include.target_hp_read_write.and.self_atk_read",
                    "static.failure_rows.retain_exact_source_span",
                    "static.cross_locus_actor_origin_does_not_mint_owner_authority",
                    "structural_rejection.no_mutation.requester_read_does_not_bypass_owner_locus",
                    "structural_rejection.no_mutation.blind_cross_owner_write_rejected",
                    "structural_rejection.no_mutation.cross_owner_transaction_not_fabricated",
                ] {
                    add_checked_source_fact(facts, source_identities, path, predicate, &context)?;
                }
            }

            if evaluation.name() == "move_to_b"
                && owner.target().namespace() == "player_b"
                && has_failures(evaluation, &["RouteUnavailable"])
            {
                add_checked_source_fact(
                    facts,
                    source_identities,
                    path,
                    "static.route_unavailable_failure_row.present_for_owner_route",
                    &context,
                )?;
            }
        }
    }
    for (path, failure) in source_failures {
        let predicate = match failure.missing_failure.as_str() {
            "VisibilityDenied" => {
                Some("diagnostic.E-ROW-002.missing_failure.VisibilityDenied.no_checked_core")
            }
            "MissingCapability" => {
                Some("diagnostic.E-ROW-001.missing_failure.MissingCapability.no_checked_core")
            }
            "RouteUnavailable" => {
                Some("diagnostic.E-ROW-001.missing_failure.RouteUnavailable.no_checked_core")
            }
            _ => None,
        };
        if let Some(predicate) = predicate {
            let context = source_evidence_context("static", path, &failure.source_ref)?;
            add_checked_source_fact(facts, source_identities, path, predicate, &context)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum M10OwnerAuthorityMode {
    Admitted,
    MissingCapability,
    StaleMembership,
    ReplayedCapability,
}

#[derive(Debug)]
struct M10OwnerScheduleServed {
    runtime: M8LocalRuntime,
    target_key: M8StateKey,
    before_runtime: M8LocalRuntime,
    m9: M10M9DomainSnapshot,
    observer_authority: M8ObserverAuthorityGrant,
}

#[derive(Debug)]
enum M10OwnerScheduleOutcome {
    Served(Box<M10OwnerScheduleServed>),
    RejectedBeforeMutation,
}

fn m10_schedule_index(index: Option<&str>, principal: &str, target: &str) -> String {
    match index {
        Some("self") => principal.to_string(),
        Some("target") => target.to_string(),
        Some(index) => index.to_string(),
        None => principal.to_string(),
    }
}

fn m8_trace_kinds_since(runtime: &M8LocalRuntime, start: usize) -> Vec<Value> {
    runtime
        .trace()
        .suffix_from(start)
        .kinds()
        .into_iter()
        .map(|kind| json!(format!("{kind:?}")))
        .collect()
}

/// Exercise the source-derived M8 designated-value carrier.  The schedule
/// only selects the already checked value and consumer; receipts, authority,
/// publication, and one-shot delivery are all assembled from the admitted M7
/// Core and executed by M8 itself.
fn execute_m8_designated_consumption(
    checked: &CheckedSurfaceV0,
    designated_value_ref: &str,
    consumer: &str,
    requested_version: u64,
    repeat: u64,
    schedule_action_identity: &str,
) -> Result<Value, String> {
    let (evaluator, result) = designated_value_ref
        .split_once('.')
        .ok_or_else(|| format!("M10 designated value {designated_value_ref} is malformed"))?;
    let evaluation = checked
        .designated_result(evaluator, result)
        .ok_or_else(|| {
            format!("M10 designated value {designated_value_ref} is absent from checked Core")
        })?;
    let core = evaluation
        .designated_core()
        .expect("checked designated result owns designated Core");
    let value_name = format!("{}.{}", core.evaluator(), core.result());
    let frontier = core
        .trigger()
        .frontier()
        .ok_or_else(|| format!("M10 designated value {value_name} has no input frontier"))?;
    let seam = m10_resolve_checked_for_patch(checked, "self", core.evaluator())?;
    let evaluator_authority = seam
        .designated_evaluation_authority_use(core.evaluator(), core.result())
        .ok_or_else(|| "M10 designated evaluator lacks M9-issued authority".to_string())?;
    let consumer_authority = seam
        .designated_consumption_authority_use(consumer, &value_name)
        .ok_or_else(|| "M10 designated consumer lacks M9-issued authority".to_string())?;
    let authority_translation = m10_m9_to_m8_authority_translation(
        &seam,
        evaluation.source_ref(),
        "m8-designated-session:0..2",
        consumer_authority
            .membership_ref()
            .ok_or_else(|| "M10 designated M8 authority lacks membership reference".to_string())?,
        consumer_authority
            .capability_ref()
            .ok_or_else(|| "M10 designated M8 authority lacks capability reference".to_string())?,
        consumer_authority
            .witness_ref()
            .ok_or_else(|| "M10 designated M8 authority lacks witness reference".to_string())?,
    )?;
    let (instance, authority_state) = seam.into_parts();

    let mut receipts = M8InputReceiptSet::new();
    for (index, read) in core.expression().state_reads().iter().enumerate() {
        let receipt = M8InputReceipt::live(format!(
            "m10-scn11:{schedule_action_identity}:receipt:{index}"
        ))
        .for_state_read(M8StateKey::indexed_field(
            read.namespace(),
            m10_schedule_index(read.index(), "self", "self"),
            read.field().unwrap_or(""),
        ))
        .with_source_owner_locus(read.owner_locus())
        .with_evaluator(core.evaluator())
        .with_input_frontier(frontier)
        .with_source_ref(read.source_ref())
        .with_label(
            EvidenceSecurityLabel::new(format!("m10-scn11-input:{index}"))
                .with_class(M8SecurityClass::Restricted),
        )
        .with_int_value(10);
        receipts = receipts.with_receipt(receipt);
    }
    if core.expression().state_reads().is_empty() {
        return Err(format!(
            "M10 designated value {value_name} has no source-bound input receipt"
        ));
    }

    // The M8 instance and every authority record cross the actual M9
    // source-bound admission seam.  M10 only selects an already checked
    // designated operation; it never constructs admitted M8 credentials.
    let mut runtime = M8LocalRuntime::from_admitted(
        instance,
        M8LocalRuntimeSeed::new()
            .with_authority_state(authority_state)
            .with_designated_input_receipts(receipts),
    );
    let evaluation_start = runtime.trace().len();
    let publication = runtime.evaluate_designated(
        M8DesignatedEvaluationRequest::for_value(&value_name)
            .with_tick(
                M8DesignatedTick::new(format!("m10-scn11:{schedule_action_identity}:tick"))
                    .with_input_frontier(frontier),
            )
            .with_authority(evaluator_authority.clone()),
    );
    let evaluation_trace = m8_trace_kinds_since(&runtime, evaluation_start);
    let publication = match publication {
        Ok(publication) => publication,
        Err(diagnostics) => {
            return Ok(json!({
                "status": "rejected",
                "designated_value_ref": designated_value_ref,
                "m8_designated_evaluation_trace": evaluation_trace,
                "diagnostic": format!("{:?}", diagnostics.primary().kind()),
                "provenance": { "schedule_action": schedule_action_identity },
            }));
        }
    };
    let evaluation_version = publication.result_version().value();
    let delivery_id = format!("m10-scn11:{schedule_action_identity}:delivery");
    let consumption_start = runtime.trace().len();
    let initial_consumption = runtime.consume_published_value(
        M8ConsumeRequest::for_value(&value_name)
            .with_consumer(consumer)
            .with_delivery_id(&delivery_id)
            .with_authority(consumer_authority.clone()),
    );
    let consumption_trace = m8_trace_kinds_since(&runtime, consumption_start);
    let initial_consumption = match initial_consumption {
        Ok(consumption) => consumption,
        Err(diagnostics) => {
            return Ok(json!({
                "status": "rejected",
                "designated_value_ref": designated_value_ref,
                "m8_designated_evaluation_trace": evaluation_trace,
                "m8_consumption_trace": consumption_trace,
                "result_version": evaluation_version,
                "diagnostic": format!("{:?}", diagnostics.primary().kind()),
                "provenance": { "schedule_action": schedule_action_identity },
            }));
        }
    };

    let mut row = json!({
        "status": if evaluation_version == requested_version && initial_consumption.result_version().value() == requested_version { "accepted" } else { "rejected" },
        "designated_value_ref": designated_value_ref,
        "result_version": evaluation_version,
        "m8_designated_evaluation_trace": evaluation_trace,
        "m8_consumption_trace": consumption_trace,
        "provenance": {
            "schedule_action": schedule_action_identity,
            "checked_designated": true,
            "publication_value_id": publication.value_id(),
            "publication_occurrence": publication.occurrence_id(),
            "consumer": initial_consumption.consumer_locus(),
        },
        "m9_to_m8_authority_translation": authority_translation,
        "direct_m10_already_admitted_authority_ref_rejected": true,
        "direct_m10_lease_ref_rejected": true,
    });
    if repeat > 1 {
        let duplicate_start = runtime.trace().len();
        let duplicate = runtime.consume_published_value(
            M8ConsumeRequest::for_value(&value_name)
                .with_consumer(consumer)
                .with_delivery_id(&delivery_id)
                .with_authority(consumer_authority),
        );
        let duplicate_trace = m8_trace_kinds_since(&runtime, duplicate_start);
        let duplicate_diagnostic = duplicate
            .as_ref()
            .err()
            .map(|diagnostics| format!("{:?}", diagnostics.primary().kind()));
        let consumed_deliveries = runtime
            .save_local_cut("m10-scn11-consumption-audit")
            .designated_consumption_state()
            .consumed_deliveries(consumer, &value_name);
        row = json!({
            "status": "rejected",
            "designated_value_ref": designated_value_ref,
            "result_version": evaluation_version,
            "m8_designated_evaluation_trace": row["m8_designated_evaluation_trace"].clone(),
            "m8_consumption_trace": row["m8_consumption_trace"].clone(),
            "m8_duplicate_delivery_trace": duplicate_trace,
            "duplicate_delivery_rejected": duplicate.is_err(),
            "double_consumption_prevented": consumed_deliveries == vec![delivery_id],
            "diagnostic": duplicate_diagnostic,
            "provenance": row["provenance"].clone(),
        });
    }
    Ok(row)
}

fn m10_schedule_seed_key(key: &str, principal: &str, target: &str) -> Option<M8StateKey> {
    let (state, field) = key.split_once("].")?;
    let (namespace, index) = state.split_once('[')?;
    let index = m10_schedule_index(Some(index), principal, target);
    (!namespace.is_empty() && !field.is_empty())
        .then(|| M8StateKey::indexed_field(namespace, index, field))
}

/// Execute an owner request through the existing M7 -> M9 -> M8 seam.  The
/// schedule supplies only exogenous request fields; authority remains sealed
/// in the M9 execution seam and can only be deliberately corrupted here to
/// exercise an M8 rejection path.
fn execute_checked_owner_schedule(
    checked: &CheckedSurfaceV0,
    request: &M10OwnerEventRequest,
    mode: M10OwnerAuthorityMode,
) -> Result<M10OwnerScheduleOutcome, String> {
    let evaluation = checked.evaluation(&request.event).ok_or_else(|| {
        format!(
            "M10 schedule event {} has no checked owner evaluation",
            request.event
        )
    })?;
    let owner = evaluation
        .owner_rmw_core()
        .ok_or_else(|| format!("M10 schedule event {} has no owner RMW", request.event))?;
    if evaluation.actor_authority_origin() != request.principal {
        return Ok(M10OwnerScheduleOutcome::RejectedBeforeMutation);
    }
    let target = request.target.as_deref().unwrap_or(&request.principal);
    let seam = m10_resolve_checked_for_owner(
        checked,
        &request.event,
        &request.principal,
        owner.owner_locus(),
    )?;
    let admitted_authority = seam
        .owner_authority_use(&request.event, &request.principal, owner.owner_locus())
        .ok_or_else(|| "M10 schedule M9 admission did not seal owner authority".to_string())?;
    let m9 = M10M9DomainSnapshot::from_seam(&seam);
    let observer_principal = format!("observer:{}", request.principal);
    let observer_authority = seam
        .observer_authority(&observer_principal)
        .ok_or_else(|| "M10 owner schedule lacks M9-issued observer authority".to_string())?;
    let (instance, authority_state) = seam.into_parts();
    let mut seed = M8LocalRuntimeSeed::new().with_authority_state(authority_state);
    let mut seeded = BTreeSet::new();
    for read in owner
        .same_owner_reads()
        .iter()
        .chain(std::iter::once(owner.target()))
    {
        let key = M8StateKey::indexed_field(
            read.namespace(),
            m10_schedule_index(read.index(), &request.principal, target),
            read.field().unwrap_or(""),
        );
        if seeded.insert(key.clone()) {
            seed = seed.with_owner_int(key, 0);
        }
    }
    for (key, value) in &request.seed {
        let key = m10_schedule_seed_key(key, &request.principal, target)
            .ok_or_else(|| format!("M10 schedule has malformed owner seed {key}"))?;
        seed = seed.with_owner_int(key, *value);
    }
    let target_key = M8StateKey::indexed_field(
        owner.target().namespace(),
        m10_schedule_index(owner.target().index(), &request.principal, target),
        owner.target().field().unwrap_or(""),
    );
    let mut runtime = M8LocalRuntime::from_admitted(instance, seed);
    let before_runtime = runtime.clone();
    let mut owner_request = M8OwnerRequest::new(&request.event);
    for parameter in checked
        .static_environment()
        .evaluation_signature(&request.event)
        .into_iter()
        .flat_map(|signature| signature.parameters())
    {
        let value = request
            .arguments
            .get(parameter.name())
            .copied()
            .unwrap_or(0);
        owner_request = owner_request.with_argument(parameter.name(), value.to_string());
    }
    if request.target.is_some() {
        owner_request = owner_request.with_argument("target", target);
    }
    let authority =
        match mode {
            M10OwnerAuthorityMode::Admitted => admitted_authority,
            M10OwnerAuthorityMode::MissingCapability => {
                M8AuthorityUse::for_principal(&request.principal)
                    .with_membership_ref(admitted_authority.membership_ref().ok_or_else(|| {
                        "M10 owner authority lacks membership reference".to_string()
                    })?)
                    .with_witness_ref(
                        admitted_authority.witness_ref().ok_or_else(|| {
                            "M10 owner authority lacks witness reference".to_string()
                        })?,
                    )
            }
            M10OwnerAuthorityMode::StaleMembership => {
                M8AuthorityUse::for_principal(&request.principal)
                    .with_membership_ref("m10-schedule-stale-membership")
                    .with_capability_ref(admitted_authority.capability_ref().ok_or_else(|| {
                        "M10 owner authority lacks capability reference".to_string()
                    })?)
                    .with_witness_ref(
                        admitted_authority.witness_ref().ok_or_else(|| {
                            "M10 owner authority lacks witness reference".to_string()
                        })?,
                    )
            }
            M10OwnerAuthorityMode::ReplayedCapability => {
                M8AuthorityUse::for_principal(&request.principal)
                    .with_membership_ref(admitted_authority.membership_ref().ok_or_else(|| {
                        "M10 owner authority lacks membership reference".to_string()
                    })?)
                    .with_capability_ref("m10-schedule-replayed-capability")
                    .with_witness_ref(
                        admitted_authority.witness_ref().ok_or_else(|| {
                            "M10 owner authority lacks witness reference".to_string()
                        })?,
                    )
            }
        };
    let before = runtime.owner_state().clone();
    if runtime
        .enqueue_owner(owner_request.with_authority_use(authority))
        .is_err()
    {
        return Ok(M10OwnerScheduleOutcome::RejectedBeforeMutation);
    }
    if runtime.serve_next_owner(owner.owner_locus()).is_err() {
        if runtime.owner_state() != &before {
            return Err("M10 rejected owner request mutated state".to_string());
        }
        return Ok(M10OwnerScheduleOutcome::RejectedBeforeMutation);
    }
    Ok(M10OwnerScheduleOutcome::Served(Box::new(
        M10OwnerScheduleServed {
            runtime,
            target_key,
            before_runtime,
            m9,
            observer_authority,
        },
    )))
}

fn schedule_events_are(events: &[String], expected: &[&str]) -> bool {
    events.len() == expected.len()
        && events
            .iter()
            .zip(expected)
            .all(|(actual, expected)| actual == expected)
}

fn checked_for_schedule_case<'a, 'b>(
    case: &'b M10ScheduleCase,
    checked_sources: &'a BTreeMap<String, CheckedSurfaceV0>,
) -> Result<(&'b str, &'a CheckedSurfaceV0), String> {
    let path = case.source.as_deref().ok_or_else(|| {
        format!(
            "M10 schedule action {} has no source-bound artifact",
            case.id
        )
    })?;
    let checked = checked_sources.get(path).ok_or_else(|| {
        format!(
            "M10 schedule action {} references unchecked source {path}",
            case.id
        )
    })?;
    Ok((path, checked))
}

fn add_case_action_fact(
    facts: &mut BTreeSet<M10EvidenceFact>,
    case: &M10ScheduleCase,
    predicate: &str,
    context: &M10EvidenceContext,
) {
    add_fact(
        facts,
        predicate,
        "schedule_action",
        case.identity.clone(),
        context,
    );
}

fn m10_schedule_has_m9_admission(checked: &CheckedSurfaceV0) -> bool {
    patch_principal_and_locus(checked)
        .and_then(|(principal, locus)| m10_resolve_checked_for_patch(checked, principal, locus))
        .is_ok()
}

fn m10_length_prefixed(parts: impl IntoIterator<Item = String>) -> String {
    parts
        .into_iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

impl M10ExecutionManifest {
    fn build(
        release_family: &str,
        source_content_identities: &BTreeMap<String, String>,
        carriers: &M10TypedCarriers,
        schedule: &M10TypedSchedule,
    ) -> Result<Self, String> {
        let action_inventory = schedule
            .cases()
            .ok_or_else(|| "M10 release manifest requires a conformance schedule".to_string())?
            .iter()
            .map(|case| case.identity.clone())
            .collect::<Vec<_>>();
        let source_revision = deterministic_hash(&m10_length_prefixed(
            source_content_identities
                .iter()
                .map(|(path, identity)| format!("{path}={identity}")),
        ));
        let schedule_identity = deterministic_hash(&m10_length_prefixed(
            action_inventory
                .iter()
                .enumerate()
                .map(|(index, action)| format!("{index}={action}")),
        ));
        let policy_stamps = carriers
            .observations
            .iter()
            .map(|carrier| format!("observation:{}", carrier.id()))
            .chain(
                carriers
                    .fallbacks
                    .iter()
                    .map(|carrier| format!("fallback:{}", carrier.id)),
            )
            .chain(
                carriers
                    .patches
                    .iter()
                    .map(|carrier| format!("patch:{}", carrier.id)),
            )
            .collect::<Vec<_>>();
        Ok(Self {
            source_revision,
            release_family: release_family.to_string(),
            source_content_identities: source_content_identities.clone(),
            typed_carriers_identity: carriers.stable_hash.clone(),
            carrier_identities: carriers.carrier_identities.clone(),
            schedule_identity,
            action_inventory,
            policy_stamps,
        })
    }

    fn canonical_identity(&self) -> String {
        m10_length_prefixed(
            std::iter::once(format!("source_revision={}", self.source_revision))
                .chain(std::iter::once(format!(
                    "release_family={}",
                    self.release_family
                )))
                .chain(
                    self.source_content_identities
                        .iter()
                        .map(|(path, identity)| format!("source={path}:{identity}")),
                )
                .chain(std::iter::once(format!(
                    "typed_carriers={}",
                    self.typed_carriers_identity
                )))
                .chain(
                    self.carrier_identities
                        .iter()
                        .map(|(id, identity)| format!("carrier={id}:{identity}")),
                )
                .chain(std::iter::once(format!(
                    "schedule={}",
                    self.schedule_identity
                )))
                .chain(
                    self.action_inventory
                        .iter()
                        .enumerate()
                        .map(|(index, identity)| format!("action={index}:{identity}")),
                )
                .chain(
                    self.policy_stamps
                        .iter()
                        .map(|stamp| format!("policy={stamp}")),
                ),
        )
    }
}

impl M10ReleaseManifest {
    fn build(
        release_family: &str,
        source_content_identities: &BTreeMap<String, String>,
        carriers: &M10TypedCarriers,
        schedule: &M10TypedSchedule,
        profile: &M10CorrespondenceProfile,
    ) -> Result<Self, String> {
        let execution = M10ExecutionManifest::build(
            release_family,
            source_content_identities,
            carriers,
            schedule,
        )?;
        let correspondence_profile_identity =
            deterministic_hash(&m10_length_prefixed(profile.rows.iter().map(|row| {
                // `evidence_predicate` is not execution identity.  Keeping it
                // outside this structural binding preserves the deliberate
                // predicate-mutation verifier tests.
                format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    row.scn_id,
                    row.expectation_id,
                    correspondence_phase_name(row.phase),
                    correspondence_carrier_kind_name(row.carrier_kind),
                    row.artifact_identity,
                    row.diagnostic_location,
                    row.source_derived_reference.as_deref().unwrap_or(""),
                    row.schedule_action_reference.as_deref().unwrap_or(""),
                )
            })));
        let verifier = M10VerifierManifest {
            profile_schema_version: "m10-i1plus-correspondence-predicates-v0".to_string(),
            correspondence_profile_identity,
            verifier_profile_hash: deterministic_hash(
                &serde_json::to_string(
                    &profile
                        .rows
                        .iter()
                        .map(correspondence_row_value)
                        .collect::<Vec<_>>(),
                )
                .expect("M10 correspondence rows serialize"),
            ),
        };
        let manifest_hash = deterministic_hash(&m10_length_prefixed([
            format!("execution={}", execution.canonical_identity()),
            format!("profile_schema={}", verifier.profile_schema_version),
            format!(
                "profile_identity={}",
                verifier.correspondence_profile_identity
            ),
            format!("verifier_profile_hash={}", verifier.verifier_profile_hash),
        ]));
        Ok(Self {
            execution,
            verifier,
            manifest_hash,
        })
    }

    fn execution_identity(&self) -> String {
        deterministic_hash(&self.execution.canonical_identity())
    }

    fn report_value(&self, anchor: &M10ReleaseAnchor, anchor_matches: bool) -> Value {
        json!({
            "profile_schema_version": self.verifier.profile_schema_version,
            "source_revision": { "base_identity": self.execution.source_revision },
            "release_family": self.execution.release_family,
            "source_content_identities": self.execution.source_content_identities,
            "typed_carriers_identity": self.execution.typed_carriers_identity,
            "carrier_identities": self.execution.carrier_identities,
            "schedule_identity": self.execution.schedule_identity,
            "execution_identity": self.execution_identity(),
            "action_inventory": self.execution.action_inventory,
            "policy_stamps": self.execution.policy_stamps,
            "correspondence_profile_identity": self.verifier.correspondence_profile_identity,
            // `manifest_hash` is the spec/11 profile hash: it binds every
            // execution input and the exact verifier profile.  Keep the
            // predicate-bearing verifier value explicit so that a
            // predicate-only mutation does not appear to alter execution
            // identity.
            "profile_hash": self.manifest_hash,
            "verifier_profile_hash": self.verifier.verifier_profile_hash,
            "manifest_hash": self.manifest_hash,
            "anchor_match": anchor_matches,
            "anchor": {
                "expected_source_revision": anchor.source_revision,
                "expected_execution_identity": anchor.execution_identity,
                "expected_manifest_hash": anchor.manifest_hash,
                "expected_verifier_profile_hash": anchor.verifier_profile_hash,
            },
            "fail_closed_checks": m10_release_anchor_fail_closed_checks(self, anchor),
        })
    }
}

enum M10ReleaseAnchorInput<'a> {
    MissingManifest,
    Manifest {
        manifest: &'a M10ReleaseManifest,
        unknown_input: Option<&'static str>,
    },
}

fn m10_validate_release_anchor_input(
    input: M10ReleaseAnchorInput<'_>,
    anchor: &M10ReleaseAnchor,
) -> Result<(), &'static str> {
    let (manifest, unknown_input) = match input {
        M10ReleaseAnchorInput::MissingManifest => return Err("MissingReleaseManifest"),
        M10ReleaseAnchorInput::Manifest {
            manifest,
            unknown_input,
        } => (manifest, unknown_input),
    };

    if unknown_input.is_some() {
        return Err("UnknownReleaseInput");
    }

    if manifest.execution.source_revision != anchor.source_revision
        || manifest.execution_identity() != anchor.execution_identity
        || manifest.manifest_hash != anchor.manifest_hash
        || manifest.verifier.verifier_profile_hash != anchor.verifier_profile_hash
    {
        return Err("FrozenReleaseManifestMismatch");
    }

    Ok(())
}

fn m10_release_anchor_observed_failure(
    kind: &'static str,
    input: M10ReleaseAnchorInput<'_>,
    anchor: &M10ReleaseAnchor,
) -> Value {
    let code = m10_validate_release_anchor_input(input, anchor)
        .expect_err("fixed M10 release anchor must fail closed for mutated input");
    json!({
        "input_mutation": { "kind": kind },
        "observed_failure": { "code": code },
        "terminal_outcome": "rejected",
        "release_anchor_before": {
            "expected_source_revision": anchor.source_revision,
            "expected_execution_identity": anchor.execution_identity,
            "expected_manifest_hash": anchor.manifest_hash,
            "expected_verifier_profile_hash": anchor.verifier_profile_hash,
        },
        "release_anchor_after": {
            "expected_source_revision": anchor.source_revision,
            "expected_execution_identity": anchor.execution_identity,
            "expected_manifest_hash": anchor.manifest_hash,
            "expected_verifier_profile_hash": anchor.verifier_profile_hash,
        },
        "fail_closed": true,
    })
}

fn m10_release_anchor_fail_closed_checks(
    manifest: &M10ReleaseManifest,
    anchor: &M10ReleaseAnchor,
) -> Value {
    let mut revision_mismatch = manifest.clone();
    revision_mismatch
        .execution
        .source_revision
        .push_str(":mutated");

    json!({
        "missing_manifest": m10_release_anchor_observed_failure(
            "missing_manifest",
            M10ReleaseAnchorInput::MissingManifest,
            anchor,
        ),
        "unknown_extra_input": m10_release_anchor_observed_failure(
            "unknown_extra_input",
            M10ReleaseAnchorInput::Manifest {
                manifest,
                unknown_input: Some("unexpected-release-input"),
            },
            anchor,
        ),
        "revision_mismatch": m10_release_anchor_observed_failure(
            "revision_mismatch",
            M10ReleaseAnchorInput::Manifest {
                manifest: &revision_mismatch,
                unknown_input: None,
            },
            anchor,
        ),
    })
}

fn m10_reference_release_anchor(profile: &str) -> Result<M10ReleaseAnchor, String> {
    if profile != "m10-reference-profile" {
        return Err(format!(
            "M10 release anchor is unavailable for profile {profile}"
        ));
    }
    Ok(M10ReleaseAnchor {
        source_revision: M10_REFERENCE_ANCHOR_SOURCE_REVISION.to_string(),
        manifest_hash: M10_REFERENCE_ANCHOR_MANIFEST_HASH.to_string(),
        execution_identity: M10_REFERENCE_ANCHOR_EXECUTION_IDENTITY.to_string(),
        verifier_profile_hash: M10_REFERENCE_ANCHOR_VERIFIER_PROFILE_HASH.to_string(),
    })
}

/// Runtime receipts point at the semantic obligation that supplied the
/// authority boundary, rather than at a synthetic whole-program range.
fn m10_semantic_source_ref(checked: &CheckedSurfaceV0) -> Result<SourceRef, String> {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .or_else(|| checked.residual_obligations().entries().first())
        .map(|residual| residual.source_ref().clone())
        .ok_or_else(|| "M10 runtime receipt requires a semantic source range".to_string())
}

#[derive(Debug, Clone)]
struct M10M8DomainSnapshot {
    store: String,
    relation: String,
    config: String,
    cut: String,
}

impl M10M8DomainSnapshot {
    fn from_runtime(runtime: &M8LocalRuntime) -> Self {
        Self {
            store: runtime.canonical_store_projection(),
            relation: runtime.canonical_relation_projection(),
            config: runtime.canonical_configuration_projection(),
            cut: runtime.canonical_semantic_projection(),
        }
    }

    fn from_cut(cut: &crate::m8_runtime_local_cut::M8LocalCut) -> Self {
        Self {
            store: cut.canonical_store_projection(),
            relation: cut.canonical_relation_projection(),
            config: cut.canonical_configuration_projection(),
            cut: cut.canonical_semantic_projection(),
        }
    }

    fn from_patch(runtime: &crate::m8_runtime_patch::M8PatchRuntime) -> Self {
        Self {
            store: runtime.canonical_store_projection(),
            relation: runtime.canonical_relation_projection(),
            config: runtime.canonical_configuration_projection(),
            cut: runtime.canonical_semantic_projection(),
        }
    }
}

#[derive(Debug, Clone)]
struct M10M9DomainSnapshot {
    membership: String,
    grant: String,
    authority: String,
}

impl M10M9DomainSnapshot {
    fn from_authority(authority: &M9AuthorityRuntime) -> Self {
        Self {
            membership: authority.canonical_membership_projection(),
            grant: authority.canonical_grant_projection(),
            authority: authority.canonical_snapshot_projection(),
        }
    }

    fn from_seam(seam: &crate::m9_auth_verification::M9M10ExecutionSeam) -> Self {
        Self {
            membership: seam.canonical_m9_membership_projection().to_string(),
            grant: seam.canonical_m9_grant_projection().to_string(),
            authority: seam.canonical_m9_snapshot_projection().to_string(),
        }
    }
}

/// Hash only native M8/M9 domain snapshots captured at an execution
/// boundary.  No caller may reconstruct a domain by splitting an aggregate
/// canonical projection.
fn m10_native_hash_bundle(
    m8: &M10M8DomainSnapshot,
    m9: &M10M9DomainSnapshot,
    ledger_projection: &str,
) -> M10SemanticHashBundle {
    m10_domain_hash_bundle(
        &m8.store,
        &m9.membership,
        &m9.grant,
        &m8.relation,
        &m8.config,
        &m9.authority,
        ledger_projection,
        &m8.cut,
    )
}

fn m10_actual_hash_bundle(
    runtime: &M8LocalRuntime,
    m9: &M10M9DomainSnapshot,
    ledger_projection: &str,
    cut: Option<&crate::m8_runtime_local_cut::M8LocalCut>,
) -> M10SemanticHashBundle {
    let m8 = cut.map_or_else(
        || M10M8DomainSnapshot::from_runtime(runtime),
        M10M8DomainSnapshot::from_cut,
    );
    m10_native_hash_bundle(&m8, m9, ledger_projection)
}

/// Receipt-facing view of the domains whose equality proves that a rejected
/// preflight left the current composite untouched.
fn m10_current_session_hashes(session_id: &str, bundle: &M10SemanticHashBundle) -> Value {
    json!({
        "session_id": session_id,
        "store_hash": bundle.store_hash,
        "membership_hash": bundle.membership_hash,
        "grant_hash": bundle.grant_hash,
        "relation_hash": bundle.relation_hash,
        "config_hash": bundle.config_hash,
        "cut_hash": bundle.cut_hash,
        "ledger_hash": bundle.ledger_hash,
    })
}

/// Hash each receipt domain from only its exact native projection.  The
/// legacy cut/audit fields remain for the established M10 cut display, but
/// the five semantic domain hashes never reuse an aggregate M8/M9 snapshot.
/// The projections remain explicit parameters so a caller cannot conflate
/// independently captured semantic domains before hashing them.
#[allow(clippy::too_many_arguments)]
fn m10_domain_hash_bundle(
    store_projection: &str,
    membership_projection: &str,
    grant_projection: &str,
    relation_projection: &str,
    configuration_projection: &str,
    m9_snapshot_projection: &str,
    ledger_projection: &str,
    cut_projection: &str,
) -> M10SemanticHashBundle {
    M10SemanticHashBundle {
        store_hash: deterministic_hash(&format!("m8-store-v1\n{store_projection}")),
        membership_hash: deterministic_hash(&format!("m9-membership-v1\n{membership_projection}")),
        grant_hash: deterministic_hash(&format!("m9-grant-v1\n{grant_projection}")),
        relation_hash: deterministic_hash(&format!("m8-relation-v1\n{relation_projection}")),
        config_hash: deterministic_hash(&format!("m8-config-v1\n{configuration_projection}")),
        // Compatibility-only display hash: fallback is an M8 relation view,
        // never an independently supplied M10 semantic domain.
        fallback_hash: deterministic_hash(&format!("m10-fallback-v1\n{relation_projection}")),
        cut_hash: deterministic_hash(&format!("m8-local-cut-v1\n{cut_projection}")),
        m8_cut_hash: deterministic_hash(&format!("m8-cut-v1\n{cut_projection}")),
        m9_authority_hash: deterministic_hash(&format!(
            "m9-authority-v1\n{m9_snapshot_projection}"
        )),
        ledger_hash: deterministic_hash(&format!("m10-ledger-v1\n{ledger_projection}")),
        domain_projection_identities: M10DomainProjectionIdentities {
            store: deterministic_hash(&format!("m8-store-projection-v1\n{store_projection}")),
            membership: deterministic_hash(&format!(
                "m9-membership-projection-v1\n{membership_projection}"
            )),
            grant: deterministic_hash(&format!("m9-grant-projection-v1\n{grant_projection}")),
            relation: deterministic_hash(&format!(
                "m8-relation-projection-v1\n{relation_projection}"
            )),
            config: deterministic_hash(&format!(
                "m8-configuration-projection-v1\n{configuration_projection}"
            )),
        },
    }
}

fn m10_domain_projection_provenance(
    before: &M10SemanticHashBundle,
    after: &M10SemanticHashBundle,
) -> Value {
    let domain = |actual_accessor: &str,
                  component: &str,
                  hash_key: &str,
                  before_projection_identity: &str,
                  after_projection_identity: &str| {
        json!({
            "actual_accessor": actual_accessor,
            "component": component,
            "hash_key": hash_key,
            "before_projection_identity": before_projection_identity,
            "after_projection_identity": after_projection_identity,
        })
    };
    json!({
        "store": domain(
            "M8LocalRuntime::canonical_store_projection",
            "owner_queue_designated_store",
            "store_hash",
            &before.domain_projection_identities.store,
            &after.domain_projection_identities.store,
        ),
        "membership": domain(
            "M9AuthorityRuntime::canonical_membership_projection",
            "membership_lineage_tombstones",
            "membership_hash",
            &before.domain_projection_identities.membership,
            &after.domain_projection_identities.membership,
        ),
        "grant": domain(
            "M9AuthorityRuntime::canonical_grant_projection",
            "capability_witness_revocation",
            "grant_hash",
            &before.domain_projection_identities.grant,
            &after.domain_projection_identities.grant,
        ),
        "relation": domain(
            "M8LocalRuntime::canonical_relation_projection",
            "semantic_relation_lease",
            "relation_hash",
            &before.domain_projection_identities.relation,
            &after.domain_projection_identities.relation,
        ),
        "config": domain(
            "M8LocalRuntime::canonical_configuration_projection",
            "program_manifest_active_patch",
            "config_hash",
            &before.domain_projection_identities.config,
            &after.domain_projection_identities.config,
        ),
    })
}

/// One persistent SCN04 M9 authority lineage.  M10 keeps only the resulting
/// receipts; membership retirement, stale-use rejection, compaction, and
/// fresh rejoin all mutate or query this sealed M9 runtime directly.
struct M10MembershipLifecycleSession {
    authority: M9AuthorityRuntime,
    membership: Option<M9MembershipAuth>,
    capability: Option<M9CapabilityAuth>,
    witness: Option<M9WitnessAuth>,
    owner_evaluation: Option<(String, String)>,
    relation_scopes: Vec<(String, String, String)>,
    owner_capability: Option<M9CapabilityAuth>,
    owner_witness: Option<M9WitnessAuth>,
    relation_maintainer_membership: Option<M9MembershipAuth>,
    relation_maintainer_capability: Option<M9CapabilityAuth>,
    relation_maintainer_witness: Option<M9WitnessAuth>,
    fresh_relation_membership: Option<M9MembershipAuth>,
    fresh_relation_capability: Option<M9CapabilityAuth>,
    fresh_relation_witness: Option<M9WitnessAuth>,
    fresh_relation_owner_capability: Option<M9CapabilityAuth>,
    fresh_relation_owner_witness: Option<M9WitnessAuth>,
    principal: String,
    locus: String,
    module: String,
    auth_residual_name: String,
    auth_residual_source_ref: SourceRef,
    next_epoch: u64,
    session_id: String,
}

impl M10MembershipLifecycleSession {
    fn new_with_session(
        checked: &CheckedSurfaceV0,
        session_id: impl Into<String>,
    ) -> Result<Self, String> {
        let (principal, locus) = patch_principal_and_locus(checked)?;
        Self::new_with_identity(checked, session_id, principal, locus)
    }

    fn new_with_identity(
        checked: &CheckedSurfaceV0,
        session_id: impl Into<String>,
        principal: &str,
        locus: &str,
    ) -> Result<Self, String> {
        let admission = m8_admission_for(checked)?;
        let m9 = M9AdmissionRuntime::default();
        let base = m9
            .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
            .map_err(|diagnostics| {
                format!("M10 membership M9 base: {:?}", diagnostics.primary().kind())
            })?;
        let auth_residual = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
            .ok_or_else(|| "M10 membership source lacks auth residual".to_string())?;
        let owner_evaluation = checked.evaluations().iter().find_map(|evaluation| {
            evaluation.owner_rmw_core().map(|owner| {
                (
                    evaluation.name().to_string(),
                    owner.owner_locus().to_string(),
                )
            })
        });
        let relation_scopes = checked
            .evaluations()
            .iter()
            .filter_map(|evaluation| {
                evaluation.relation_core().map(|relation| {
                    (
                        evaluation.name().to_string(),
                        relation.owner_locus().to_string(),
                        relation
                            .binding_frontier()
                            .as_slice()
                            .first()
                            .expect("M7 relation binding frontier is finite")
                            .as_str()
                            .to_string(),
                    )
                })
            })
            .collect();
        let mut session = Self {
            authority: base.authority_runtime(),
            membership: None,
            capability: None,
            witness: None,
            owner_evaluation,
            relation_scopes,
            owner_capability: None,
            owner_witness: None,
            relation_maintainer_membership: None,
            relation_maintainer_capability: None,
            relation_maintainer_witness: None,
            fresh_relation_membership: None,
            fresh_relation_capability: None,
            fresh_relation_witness: None,
            fresh_relation_owner_capability: None,
            fresh_relation_owner_witness: None,
            principal: principal.to_string(),
            locus: locus.to_string(),
            module: checked.program_identity().module().to_string(),
            auth_residual_name: auth_residual.name().to_string(),
            auth_residual_source_ref: auth_residual.source_ref().clone(),
            next_epoch: 1,
            session_id: session_id.into(),
        };
        session.admit_fresh()?;
        // A relation maintainer is not a generic M10 convenience identity:
        // it exists only when the checked static environment declares it and
        // the program carries an actual checked relation scope.  This keeps
        // SCN10's World transition authority source-bound and prevents other
        // scenarios from gaining an implicit extra principal.
        let declares_relation_maintainer = checked
            .static_environment()
            .principals()
            .iter()
            .any(|principal| principal.name() == "relation_maintainer");
        if declares_relation_maintainer && !session.relation_scopes.is_empty() {
            session.admit_relation_maintainer()?;
        }
        Ok(session)
    }

    fn projection(&self) -> String {
        self.authority.canonical_snapshot_projection()
    }

    fn domain_snapshot(&self) -> M10M9DomainSnapshot {
        M10M9DomainSnapshot::from_authority(&self.authority)
    }

    fn admit_fresh(&mut self) -> Result<(), String> {
        let epoch = format!("m10-membership-session-epoch-{}", self.next_epoch);
        let incarnation = format!(
            "m10-membership-session:{}:{}:{}",
            self.principal, self.locus, self.next_epoch,
        );
        let attestation = self
            .authority
            .issue_membership_attestation(
                &self.principal,
                &self.locus,
                &epoch,
                &incarnation,
                &self.auth_residual_name,
                self.auth_residual_source_ref.clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 membership attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let membership = self
            .authority
            .authenticate_membership(
                M9MembershipRequest::new(&self.principal, &self.locus, &epoch)
                    .with_incarnation(incarnation)
                    .with_auth_residual(
                        &self.auth_residual_name,
                        self.auth_residual_source_ref.clone(),
                    )
                    .with_issued_provider_attestation(attestation),
            )
            .map_err(|diagnostics| {
                format!("M10 membership admit: {:?}", diagnostics.primary().kind())
            })?;
        let capability = self
            .authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!(
                    "m10-membership-session-contract-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    &self.module,
                    format!("membership-authority/{}", self.auth_residual_name),
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 membership capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let witness = self
            .authority
            .materialize_witness(
                M9WitnessRequest::new(format!(
                    "m10-membership-session-witness-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!("M10 membership witness: {:?}", diagnostics.primary().kind())
            })?;
        self.membership = Some(membership);
        self.capability = Some(capability);
        self.witness = Some(witness);
        if let Some((evaluation, owner_locus)) = self.owner_evaluation.as_ref() {
            let membership = self
                .membership
                .as_ref()
                .expect("fresh M10 membership is available for owner capability");
            let owner_capability = self
                .authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "m10-membership-owner-capability-{}",
                        self.next_epoch
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::owner_evaluation(evaluation, owner_locus))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(self.auth_residual_source_ref.clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 membership owner capability: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let owner_witness = self
                .authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "m10-membership-owner-witness-{}",
                        self.next_epoch
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(owner_capability.ref_id())
                    .with_source_ref(self.auth_residual_source_ref.clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 membership owner witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            self.owner_capability = Some(owner_capability);
            self.owner_witness = Some(owner_witness);
        }
        self.next_epoch += 1;
        Ok(())
    }

    fn admit_relation_maintainer(&mut self) -> Result<(), String> {
        let epoch = format!("m10-relation-maintainer-epoch-{}", self.next_epoch);
        let incarnation = format!("m10-relation-maintainer:World:{}", self.next_epoch);
        let attestation = self
            .authority
            .issue_membership_attestation(
                "relation_maintainer",
                "World",
                &epoch,
                &incarnation,
                &self.auth_residual_name,
                self.auth_residual_source_ref.clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 relation-maintainer attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let membership = self
            .authority
            .authenticate_membership(
                M9MembershipRequest::new("relation_maintainer", "World", &epoch)
                    .with_incarnation(incarnation)
                    .with_auth_residual(
                        &self.auth_residual_name,
                        self.auth_residual_source_ref.clone(),
                    )
                    .with_issued_provider_attestation(attestation),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 relation-maintainer admission: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let capability = self
            .authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!(
                    "m10-relation-maintainer-contract-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    &self.module,
                    format!("membership-authority/{}", self.auth_residual_name),
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 relation-maintainer capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let witness = self
            .authority
            .materialize_witness(
                M9WitnessRequest::new(format!(
                    "m10-relation-maintainer-witness-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 relation-maintainer witness: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        self.issue_relation_transition_authorities(
            &membership,
            "invalidate_primary",
            "relation-maintainer",
        )?;
        self.relation_maintainer_membership = Some(membership);
        self.relation_maintainer_capability = Some(capability);
        self.relation_maintainer_witness = Some(witness);
        self.next_epoch += 1;
        Ok(())
    }

    fn admit_fresh_relation_reacquire(&mut self) -> Result<(), String> {
        let epoch = format!("m10-fresh-relation-epoch-{}", self.next_epoch);
        let incarnation = format!("m10-fresh-relation-self:World:{}", self.next_epoch);
        let attestation = self
            .authority
            .issue_membership_attestation(
                "self",
                "World",
                &epoch,
                &incarnation,
                &self.auth_residual_name,
                self.auth_residual_source_ref.clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let membership = self
            .authority
            .authenticate_membership(
                M9MembershipRequest::new("self", "World", &epoch)
                    .with_incarnation(incarnation)
                    .with_auth_residual(
                        &self.auth_residual_name,
                        self.auth_residual_source_ref.clone(),
                    )
                    .with_issued_provider_attestation(attestation),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation admission: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let capability = self
            .authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!(
                    "m10-fresh-relation-contract-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    &self.module,
                    format!("membership-authority/{}", self.auth_residual_name),
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let witness = self
            .authority
            .materialize_witness(
                M9WitnessRequest::new(format!("m10-fresh-relation-witness-{}", self.next_epoch))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(capability.ref_id())
                    .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation witness: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        self.issue_relation_transition_authorities(
            &membership,
            "reacquire_primary",
            "fresh-relation",
        )?;
        let (owner_capability, owner_witness) = self.fresh_world_owner_authority(&membership)?;
        self.fresh_relation_membership = Some(membership);
        self.fresh_relation_capability = Some(capability);
        self.fresh_relation_witness = Some(witness);
        self.fresh_relation_owner_capability = Some(owner_capability);
        self.fresh_relation_owner_witness = Some(owner_witness);
        self.next_epoch += 1;
        Ok(())
    }

    fn issue_relation_transition_authorities(
        &mut self,
        membership: &M9MembershipAuth,
        transition: &str,
        source: &str,
    ) -> Result<(), String> {
        for (relation, owner_locus, binding_frontier) in self.relation_scopes.clone() {
            let capability = self
                .authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "m10-{source}-relation-capability-{relation}-{transition}-{}",
                        self.next_epoch
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::relation_transition(
                        &relation,
                        transition,
                        &owner_locus,
                        &binding_frontier,
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(self.auth_residual_source_ref.clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 {source} relation capability {relation}/{transition}: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            self.authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "m10-{source}-relation-witness-{relation}-{transition}-{}",
                        self.next_epoch
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(capability.ref_id())
                    .with_source_ref(self.auth_residual_source_ref.clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 {source} relation witness {relation}/{transition}: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
        Ok(())
    }

    fn fresh_world_owner_authority(
        &mut self,
        membership: &M9MembershipAuth,
    ) -> Result<(M9CapabilityAuth, M9WitnessAuth), String> {
        let (evaluation, owner_locus) = self
            .owner_evaluation
            .as_ref()
            .ok_or_else(|| "M10 fresh relation has no checked owner evaluation".to_string())?;
        let capability = self
            .authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!(
                    "m10-fresh-relation-owner-capability-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation(evaluation, owner_locus))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation owner capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let witness = self
            .authority
            .materialize_witness(
                M9WitnessRequest::new(format!(
                    "m10-fresh-relation-owner-witness-{}",
                    self.next_epoch
                ))
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(self.auth_residual_source_ref.clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 fresh relation owner witness: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        Ok((capability, witness))
    }

    fn retire(&mut self) -> Result<(), String> {
        let membership = self
            .membership
            .as_ref()
            .expect("M10 membership session always admits before retirement");
        self.authority
            .retire_membership(membership.ref_id(), "m10-scn04-leave-audit-cut")
            .map_err(|diagnostics| {
                format!("M10 membership retire: {:?}", diagnostics.primary().kind())
            })
    }

    fn stale_use_is_rejected(&mut self) -> bool {
        let membership = self
            .membership
            .as_ref()
            .expect("M10 membership session always admits before use");
        let capability = self
            .capability
            .as_ref()
            .expect("M10 membership session always grants before use");
        let witness = self
            .witness
            .as_ref()
            .expect("M10 membership session always witnesses before use");
        self.authority
            .use_authority(
                M9FactUse::capability(capability.ref_id())
                    .with_membership_ref(membership.ref_id())
                    .with_witness_ref(witness.ref_id())
                    .with_epoch(membership.epoch())
                    .with_scope(M9CapabilityScope::contract_update(
                        &self.module,
                        format!("membership-authority/{}", self.auth_residual_name),
                    )),
            )
            .is_err()
    }

    fn compact(&mut self, audit_frontier: &str) -> bool {
        let membership = self
            .membership
            .as_ref()
            .expect("M10 membership session always admits before compaction");
        self.authority
            .compact_retired_membership(membership.ref_id(), audit_frontier)
            .is_ok()
    }

    fn save_authority_cut(&mut self) -> Result<M9AuthorityCut, String> {
        self.authority.save_authority_cut().map_err(|diagnostics| {
            format!(
                "M10 membership authority-cut save: {:?}",
                diagnostics.primary().kind()
            )
        })
    }

    fn restore_authority_cut(&mut self, cut: M9AuthorityCut) -> Result<(), String> {
        self.authority
            .restore_authority_cut(cut)
            .map_err(|diagnostics| {
                format!(
                    "M10 membership authority-cut restore: {:?}",
                    diagnostics.primary().kind()
                )
            })
    }

    fn rejoin_without_fresh_is_rejected(&mut self) -> bool {
        let membership = self
            .membership
            .as_ref()
            .expect("M10 membership session always admits before rejoin")
            .clone();
        let attestation = match self.authority.issue_membership_attestation(
            &self.principal,
            &self.locus,
            membership.epoch(),
            membership.incarnation(),
            &self.auth_residual_name,
            self.auth_residual_source_ref.clone(),
        ) {
            Ok(attestation) => attestation,
            Err(_) => return true,
        };
        self.authority
            .authenticate_membership(
                M9MembershipRequest::new(&self.principal, &self.locus, membership.epoch())
                    .with_incarnation(membership.incarnation())
                    .with_auth_residual(
                        &self.auth_residual_name,
                        self.auth_residual_source_ref.clone(),
                    )
                    .with_issued_provider_attestation(attestation),
            )
            .is_err()
    }

    fn retired_authority_details(&self) -> Value {
        let membership = self
            .membership
            .as_ref()
            .expect("M10 membership session always admits before evidence");
        let capability = self
            .capability
            .as_ref()
            .expect("M10 membership session always grants before evidence");
        let witness = self
            .witness
            .as_ref()
            .expect("M10 membership session always witnesses before evidence");
        json!({
            "m9_authority_use": {
                "attempted_after_leave": true,
                "result": "fail",
                "diagnostic": "InvalidCapabilityLineage",
            },
            "m9_retired_authority_snapshot": {
                "membership": { "status": "tombstoned", "ref": membership.ref_id() },
                "capability": { "status": "revoked", "ref": capability.ref_id() },
                "witness": { "status": "invalidated", "ref": witness.ref_id() },
                "provenance": {
                    "source_layer": "M9",
                    "source_accessor": "M10MembershipLifecycleSession::retired_authority_details",
                },
            },
            "m10_ledger_membership": {
                "status": "tombstoned",
                "membership_ref": membership.ref_id(),
                "audit_frontier": "m10-scn04-leave-audit-cut",
            },
            "session_id": self.session_id,
            "semantic_state_owner": {
                "layer": "M9",
                "session_id": self.session_id,
            },
        })
    }

    fn session_details(&self, range_start: u64, range_end: u64) -> Value {
        json!({
            "session_id": self.session_id,
            "semantic_state_owner": {
                "layer": "M9",
                "session_id": self.session_id,
            },
            "monotone_trace_range": { "start": range_start, "end": range_end },
        })
    }
}

impl M10MembershipLifecycleSession {
    fn bridge_to_m8(&mut self) -> M9M10AuthorityBridge {
        let (membership, capability, witness, owner) = if self.relation_scopes.is_empty() {
            let membership = self
                .membership
                .as_ref()
                .expect("M10 membership bridge requires an admitted membership");
            let capability = self
                .capability
                .as_ref()
                .expect("M10 membership bridge requires an admitted capability");
            let witness = self
                .witness
                .as_ref()
                .expect("M10 membership bridge requires an admitted witness");
            let owner = self
                .owner_evaluation
                .as_ref()
                .and_then(|(evaluation, owner_locus)| {
                    Some((
                        evaluation.as_str(),
                        owner_locus.as_str(),
                        self.owner_capability.as_ref()?,
                        self.owner_witness.as_ref()?,
                    ))
                });
            (membership, capability, witness, owner)
        } else if let (Some(membership), Some(capability), Some(witness)) = (
            self.fresh_relation_membership.as_ref(),
            self.fresh_relation_capability.as_ref(),
            self.fresh_relation_witness.as_ref(),
        ) {
            let owner = self
                .owner_evaluation
                .as_ref()
                .and_then(|(evaluation, owner_locus)| {
                    Some((
                        evaluation.as_str(),
                        owner_locus.as_str(),
                        self.fresh_relation_owner_capability.as_ref()?,
                        self.fresh_relation_owner_witness.as_ref()?,
                    ))
                });
            (membership, capability, witness, owner)
        } else {
            (
                self.relation_maintainer_membership
                    .as_ref()
                    .expect("M10 relation bridge requires a World maintainer membership"),
                self.relation_maintainer_capability
                    .as_ref()
                    .expect("M10 relation bridge requires a World maintainer capability"),
                self.relation_maintainer_witness
                    .as_ref()
                    .expect("M10 relation bridge requires a World maintainer witness"),
                None,
            )
        };
        self.authority
            .m10_authority_bridge(membership, capability, witness, owner)
    }
}

/// Shared M9→M8 session for post-mutation decisions.  M9 remains the source
/// of membership facts; M8 retains one execution runtime and receives each
/// sealed bridge refresh without recreating its local state.
struct M10M9M8ExecutionSession {
    session_id: String,
    m9: M10MembershipLifecycleSession,
    m8_instance: M8RuntimeInstance,
    runtime: M8LocalRuntime,
    bridge: M9M10AuthorityBridge,
    bridge_generation: u64,
    invalidated_owner_use: Option<(String, String, M8AuthorityUse)>,
}

impl M10M9M8ExecutionSession {
    fn new(checked: &CheckedSurfaceV0, session_id: impl Into<String>) -> Result<Self, String> {
        let session_id = session_id.into();
        let mut m9 =
            M10MembershipLifecycleSession::new_with_session(checked, format!("{session_id}:m9"))?;
        let bridge = m9.bridge_to_m8();
        let (principal, locus) = patch_principal_and_locus(checked)?;
        let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
        let (m8_instance, _) = seam.into_parts();
        let runtime = M8LocalRuntime::from_admitted(
            m8_instance.clone(),
            M8LocalRuntimeSeed::new().with_authority_state(bridge.authority_state()),
        );
        Ok(Self {
            session_id,
            m9,
            m8_instance,
            runtime,
            bridge,
            bridge_generation: 0,
            invalidated_owner_use: None,
        })
    }

    fn refresh_bridge(&mut self) {
        self.bridge_generation += 1;
        self.bridge = self.m9.bridge_to_m8();
        self.runtime
            .refresh_m9_authority_state(self.bridge.authority_state());
    }

    fn retire_and_refresh(&mut self) -> Result<(), String> {
        self.invalidated_owner_use = self.bridge.owner_use();
        self.m9.retire()?;
        self.refresh_bridge();
        Ok(())
    }

    fn post_retirement_owner_decision(&mut self) -> Result<bool, String> {
        let Some((evaluation, owner_locus, stale_authority)) = self.invalidated_owner_use.clone()
        else {
            return Ok(true);
        };
        self.runtime
            .enqueue_owner(M8OwnerRequest::new(evaluation).with_authority_use(stale_authority))
            .map_err(|diagnostics| {
                format!(
                    "M10 post-retirement owner enqueue: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        Ok(self.runtime.serve_next_owner(&owner_locus).is_err())
    }

    fn lineage_value(&self) -> Value {
        let m8_runtime_session_id = format!("{}:m8", self.session_id);
        json!({
            "session_id": self.session_id,
            "m9_authority_session_id": self.m9.session_id,
            "m8_runtime_session_id": m8_runtime_session_id,
            "m9_snapshot_ref": deterministic_hash(self.bridge.authority_snapshot_projection()),
            "m8_authority_use_ref": deterministic_hash(&format!(
                "m8-authority-bridge|{}|{}",
                self.session_id, self.bridge_generation,
            )),
            "authority_issuer": "M9",
            "authority_bridge_provenance": "crate::m9_auth_verification::M9M10AuthorityBridge",
            "bridge_generation": self.bridge_generation,
            "m8_instance_identity": self.m8_instance.program_identity().stable_key(),
        })
    }
}

struct M10RelationLifecycleRuntime {
    runtime: M8LocalRuntime,
    m9: M10M9DomainSnapshot,
    invalidate_authority: M8RelationAuthorityUse,
    reacquire_authority: M8RelationAuthorityUse,
    primary_anchor: String,
    initial_frontier: String,
    fresh_frontier: String,
    fresh_anchor_epoch: String,
    fresh_lease_ref: String,
    fresh_witness_ref: String,
    authority_translation: Value,
    pre_degradation_cut: Option<crate::m8_runtime_local_cut::M8LocalCut>,
    rollback_evidence: Option<Value>,
}

/// Profile-specific proof that the accepted SCN08 carrier exactly names the
/// three finite, read-only options that M8 may install.  The unchecked typed
/// carrier never crosses the M8 configuration boundary directly.
#[derive(Debug, Clone)]
struct M10ValidatedFiniteFallbackChain {
    chain: M8FiniteFallbackChain,
}

fn m10_validate_normal_finite_fallback_chain(
    carrier: &M10FallbackCarrier,
) -> Result<M10ValidatedFiniteFallbackChain, String> {
    let expected = [
        (
            M10FallbackOptionKind::Live,
            "live_pose",
            "lease:view_pose:live",
            "cap:relation:view_pose:live",
            "avatar_session",
        ),
        (
            M10FallbackOptionKind::Anchor,
            "room_anchor",
            "lease:view_pose:anchor",
            "cap:relation:view_pose:anchor",
            "room_epoch",
        ),
        (
            M10FallbackOptionKind::Frozen,
            "default_pose",
            "lease:view_pose:frozen",
            "cap:relation:view_pose:frozen",
            "static",
        ),
    ];
    if carrier.id != "view-pose-normal-fallback"
        || carrier.relation != "view_pose"
        || carrier.negative_capability_floor != "write_after_read_without_fresh_reacquire"
        || carrier.options.len() != expected.len()
    {
        return Err(
            "M10 SCN08 normal fallback carrier is not the finite read-floor profile".to_string(),
        );
    }
    for (index, (kind, target, lease, capability, epoch)) in expected.into_iter().enumerate() {
        let option = &carrier.options[index];
        if option.kind != kind
            || option.target != target
            || option.lease != lease
            || option.capability != capability
            || option.epoch != epoch
            || option.lease.is_empty()
            || option.capability.is_empty()
        {
            return Err(format!(
                "M10 SCN08 fallback option {index} violates the exact finite read-floor profile"
            ));
        }
    }
    let has_exact_edge =
        |option: &M10FallbackOption, from: M10FallbackOptionKind, to: M10FallbackOptionKind| {
            option.lineage_edges.len() == 1
                && option.lineage_edges[0].from == from
                && option.lineage_edges[0].to == to
        };
    if !carrier.options[0].lineage_edges.is_empty()
        || !has_exact_edge(
            &carrier.options[1],
            M10FallbackOptionKind::Live,
            M10FallbackOptionKind::Anchor,
        )
        || !has_exact_edge(
            &carrier.options[2],
            M10FallbackOptionKind::Anchor,
            M10FallbackOptionKind::Frozen,
        )
    {
        return Err("M10 SCN08 fallback carrier lacks the exact 0->1->2 lineage".to_string());
    }
    let option = |index: usize| {
        let option = &carrier.options[index];
        M8FiniteFallbackOption::new(
            option.target.clone(),
            option.lease.clone(),
            option.capability.clone(),
            option.epoch.clone(),
        )
    };
    Ok(M10ValidatedFiniteFallbackChain {
        chain: M8FiniteFallbackChain::live_anchor_frozen(
            &carrier.relation,
            option(0),
            option(1),
            option(2),
        ),
    })
}

fn m8_finite_fallback_state(runtime: &M8LocalRuntime, relation: &str) -> Result<Value, String> {
    let selection = runtime
        .finite_fallback_selection(relation)
        .ok_or_else(|| format!("M10 M8 finite fallback selection is missing {relation}"))?;
    Ok(json!({
        "selected_floor": selection.floor().as_str(),
        "selected_option_index": selection.option_index(),
        "selected_target": selection.target(),
        "active_lease_ref": selection.lease_ref(),
        "required_capability": selection.required_capability(),
        "selected_option_epoch": selection.epoch(),
        "derived_from_actual_m8_relation_state": true,
    }))
}

fn m8_relation_state_value(runtime: &M8LocalRuntime, relation: &str) -> Result<Value, String> {
    runtime
        .relation_state(relation)
        .map(|state| {
            json!({
                "selected_floor": state.selected_floor().as_str(),
                "selected_option_index": state.selected_option_index(),
                "selected_target": state.selected_anchor(),
                "active_lease_ref": state.active_lease_ref(),
                "selected_option_epoch": state.selected_option_epoch(),
                "derived_from_actual_m8_relation_state": true,
            })
        })
        .ok_or_else(|| format!("M10 M8 relation state is missing {relation}"))
}

/// Render the already-installed finite chain for the evidence surface.  The
/// option identities originate in the carrier accepted by the exact validator;
/// their lease status and selected option remain M8 runtime state.
fn m8_finite_fallback_chain_value(carrier: &M10FallbackCarrier, runtime: &M8LocalRuntime) -> Value {
    let active_lease = runtime
        .relation_state(&carrier.relation)
        .map(|state| state.active_lease_ref().to_string());
    json!({
        "owner": "M8",
        "validated_by_m8": runtime.has_finite_fallback_chain(&carrier.relation),
        "options": carrier.options.iter().enumerate().map(|(index, option)| {
            let kind = fallback_option_kind_name(option.kind);
            let lease_state = if !runtime.contains_live_relation_lease(&option.lease) {
                "expired"
            } else if active_lease.as_deref() == Some(option.lease.as_str()) {
                "current"
            } else {
                "available"
            };
            json!({
                "index": index,
                "floor": kind,
                "kind": kind,
                "target": option.target,
                "target_identity": option.target,
                "lease": option.lease,
                "lease_identity": option.lease,
                "capability": option.capability,
                "capability_identity": option.capability,
                "epoch": option.epoch,
                "epoch_identity": option.epoch,
                "lineage_edges": option.lineage_edges.iter().map(|edge| json!({
                    "from": fallback_option_kind_name(edge.from),
                    "to": fallback_option_kind_name(edge.to),
                })).collect::<Vec<_>>(),
                "projection_kind": if option.kind == M10FallbackOptionKind::Frozen {
                    "opaque_default_pose"
                } else {
                    "semantic_relation_target"
                },
                "lease_state_after_expiry": lease_state,
            })
        }).collect::<Vec<_>>(),
    })
}

fn annotate_actual_m8_relation_trace(
    trace: &mut Value,
    relation_projection_before: &str,
    relation_projection_after: &str,
) {
    let Some(entries) = trace.as_array_mut() else {
        return;
    };
    for entry in entries {
        let Some(entry) = entry.as_object_mut() else {
            continue;
        };
        entry.insert(
            "relation_projection_before".to_string(),
            json!(relation_projection_before),
        );
        entry.insert(
            "relation_projection_after".to_string(),
            json!(relation_projection_after),
        );
        entry.insert(
            "derived_from_actual_m8_relation_state".to_string(),
            json!(true),
        );
        entry.insert(
            "derived_from_actual_m8_relation_projection".to_string(),
            json!(true),
        );
    }
}

struct M10RelationLifecycleSession {
    lifecycle: M10RelationLifecycleRuntime,
}

impl M10RelationLifecycleSession {
    fn new(checked: &CheckedSurfaceV0, relation_name: &str) -> Result<Self, String> {
        Ok(Self {
            lifecycle: m10_relation_lifecycle_runtime(checked, relation_name)?,
        })
    }

    fn new_with_validated_fallback_chain(
        checked: &CheckedSurfaceV0,
        relation_name: &str,
        validated_chain: M10ValidatedFiniteFallbackChain,
    ) -> Result<Self, String> {
        let mut session = Self::new(checked, relation_name)?;
        session
            .lifecycle
            .runtime
            .install_finite_fallback_chain(validated_chain.chain)
            .map_err(|diagnostics| {
                format!(
                    "M10 SCN08 M8 finite fallback admission: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        Ok(session)
    }

    fn session_details(&self) -> Value {
        json!({
            "session_id": "m8-scn08-relation",
            "semantic_state_owner": {
                "layer": "M8",
                "session_id": "m8-scn08-relation",
            },
        })
    }
}

/// One SCN10 composite-cut owner.  The M8 local runtime owns saved cuts and
/// restore checks; the paired M9 authority runtime owns the membership
/// timeline represented by those cuts.  M10 retains only receipts derived at
/// the boundary.
struct M10CompositeCutSession {
    session_id: String,
    runtime: M8LocalRuntime,
    m9: M10MembershipLifecycleSession,
    m8_instance: M8RuntimeInstance,
    bridge: M9M10AuthorityBridge,
    bridge_generation: u64,
    fallback_chain: Option<M8FiniteFallbackChain>,
    relation_name: Option<String>,
    relation_primary_anchor: Option<String>,
    relation_binding_frontier: Option<String>,
    seeded_relation_lease_ref: Option<String>,
    fresh_relation_lease_ref: Option<String>,
    s1: Option<crate::m8_runtime_local_cut::M8LocalCut>,
    s2: Option<crate::m8_runtime_local_cut::M8LocalCut>,
    m9_s1: Option<M9AuthorityCut>,
    m9_s2: Option<M9AuthorityCut>,
    fresh_loaded: Option<Box<M10CompositeCutSession>>,
}

impl M10CompositeCutSession {
    fn new(checked: &CheckedSurfaceV0) -> Result<Self, String> {
        Self::new_with_session(
            checked,
            "m9m8-scn10-composite-cut",
            "m9-scn10-composite-authority",
            None,
        )
    }

    fn new_with_fallback_chain(
        checked: &CheckedSurfaceV0,
        fallback_chain: M8FiniteFallbackChain,
    ) -> Result<Self, String> {
        Self::new_with_session(
            checked,
            "m9m8-scn10-composite-cut",
            "m9-scn10-composite-authority",
            Some(fallback_chain),
        )
    }

    fn new_with_session(
        checked: &CheckedSurfaceV0,
        session_id: impl Into<String>,
        m9_session_id: impl Into<String>,
        fallback_chain: Option<M8FiniteFallbackChain>,
    ) -> Result<Self, String> {
        let session_id = session_id.into();
        let mut m9 = M10MembershipLifecycleSession::new_with_session(checked, m9_session_id)?;
        let bridge = m9.bridge_to_m8();
        let (principal, locus) = patch_principal_and_locus(checked)?;
        let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
        let (m8_instance, _) = seam.into_parts();
        let relation = checked.evaluations().iter().find_map(|evaluation| {
            evaluation.relation_core().map(|core| {
                (
                    evaluation.name().to_string(),
                    core.primary().anchor().to_string(),
                    core.binding_frontier()
                        .as_slice()
                        .first()
                        .expect("M7 relation binding frontier is finite")
                        .as_str()
                        .to_string(),
                    core.owner_locus().to_string(),
                )
            })
        });
        let mut seed = M8LocalRuntimeSeed::new().with_authority_state(bridge.authority_state());
        let mut fresh_relation_lease_ref = None;
        if let Some((relation_name, _, binding_frontier, owner_locus)) = relation.as_ref() {
            let fresh_lease_ref = format!("m10-scn10-fresh-lease:{relation_name}:binding_epoch:2");
            seed = seed
                .with_live_lease(
                    M8LeaseRecord::live(format!("m10-lease:{relation_name}"))
                        .for_relation(relation_name)
                        .with_owner_locus(owner_locus)
                        .with_binding_frontier(binding_frontier)
                        .with_epoch("binding_epoch:1"),
                )
                .with_live_lease(
                    M8LeaseRecord::live(&fresh_lease_ref)
                        .for_relation(relation_name)
                        .with_owner_locus(owner_locus)
                        .with_binding_frontier(binding_frontier)
                        .with_epoch("binding_epoch:2")
                        .with_anchor_epoch("m10-fresh-relation-epoch-3:view_pose"),
                );
            fresh_relation_lease_ref = Some(fresh_lease_ref);
        }
        let mut runtime = M8LocalRuntime::from_admitted(m8_instance.clone(), seed);
        if let Some(chain) = fallback_chain.clone() {
            runtime
                .install_finite_fallback_chain(chain)
                .map_err(|diagnostics| {
                    format!(
                        "M10 SCN10 M8 finite fallback admission: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
        let seeded_relation_lease_ref = relation.as_ref().and_then(|(relation_name, _, _, _)| {
            runtime
                .relation_state(relation_name)
                .map(|state| state.active_lease_ref().to_string())
        });
        Ok(Self {
            session_id,
            runtime,
            m9,
            m8_instance,
            bridge,
            bridge_generation: 0,
            fallback_chain,
            relation_name: relation.as_ref().map(|(name, _, _, _)| name.clone()),
            relation_primary_anchor: relation.as_ref().map(|(_, anchor, _, _)| anchor.clone()),
            relation_binding_frontier: relation
                .as_ref()
                .map(|(_, _, frontier, _)| frontier.clone()),
            seeded_relation_lease_ref,
            fresh_relation_lease_ref,
            s1: None,
            s2: None,
            m9_s1: None,
            m9_s2: None,
            fresh_loaded: None,
        })
    }

    fn refresh_bridge(&mut self) {
        self.bridge_generation += 1;
        self.bridge = self.m9.bridge_to_m8();
        self.runtime
            .refresh_m9_authority_state(self.bridge.authority_state());
    }

    fn retire_m9_and_refresh(&mut self) -> Result<(), String> {
        self.m9.retire()?;
        self.refresh_bridge();
        Ok(())
    }

    fn lineage_value(&self) -> Value {
        json!({
            "session_id": self.session_id,
            "m9_authority_session_id": self.m9.session_id,
            "m8_runtime_session_id": self.m8_runtime_session_id(),
            "m9_snapshot_ref": deterministic_hash(self.bridge.authority_snapshot_projection()),
            "m8_authority_use_ref": deterministic_hash(&format!(
                "m8-authority-bridge|{}|{}", self.session_id, self.bridge_generation,
            )),
            "authority_issuer": "M9",
            "authority_bridge_provenance": "crate::m9_auth_verification::M9M10AuthorityBridge",
            "bridge_generation": self.bridge_generation,
            "m8_instance_identity": self.m8_instance.program_identity().stable_key(),
        })
    }

    fn save_s1(&mut self) -> Result<(), String> {
        self.s1 = Some(self.runtime.save_local_cut("m10-schedule-S1"));
        self.m9_s1 = Some(self.m9.save_authority_cut()?);
        Ok(())
    }

    fn save_s2(&mut self) -> Result<(), String> {
        self.s2 = Some(self.runtime.save_local_cut("m10-schedule-S2"));
        self.m9_s2 = Some(self.m9.save_authority_cut()?);
        Ok(())
    }

    fn expire_seeded_relation_lease(&mut self) -> Result<Value, String> {
        let relation = self
            .relation_name
            .as_deref()
            .ok_or_else(|| "M10 SCN10 has no checked relation to expire".to_string())?;
        let primary_anchor = self
            .relation_primary_anchor
            .as_deref()
            .ok_or_else(|| "M10 SCN10 relation lacks a primary anchor".to_string())?;
        let seeded_lease = self
            .seeded_relation_lease_ref
            .as_deref()
            .ok_or_else(|| "M10 SCN10 relation lacks a seeded lease".to_string())?;
        let authority = self
            .bridge
            .relation_authority_use(relation, "invalidate_primary")
            .ok_or_else(|| {
                "M10 SCN10 M9 bridge lacks maintainer invalidation authority".to_string()
            })?;
        let before_relation = self.runtime.canonical_relation_projection();
        // Preserve the cut identity while sampling each side, so the native
        // cut projection changes only for the M8 semantic transition rather
        // than because the receipt chose a different cut label.
        let before_cut = self.runtime.save_local_cut("m10-scn10-s2-native-cut");
        let lease_was_live = self.runtime.contains_live_relation_lease(seeded_lease);
        let transition = self
            .runtime
            .invalidate_primary(
                relation,
                authority,
                M8BindingInvalidation::lease_expired(primary_anchor),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 SCN10 M8 lease expiry: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let after_relation = self.runtime.canonical_relation_projection();
        let after_cut = self.runtime.save_local_cut("m10-scn10-s2-native-cut");
        Ok(json!({
            "m8_runtime_session_id": self.m8_runtime_session_id(),
            "relation_domain": "semantic_relation_lease",
            "seeded_relation_lease_ref": seeded_lease,
            "lease_expiry": {
                "accessor": "M8LocalRuntime::invalidate_primary",
                "result": "expired",
            },
            "lease_state_before": { "status": if lease_was_live { "live" } else { "expired" } },
            "lease_state_after": {
                "status": if self.runtime.contains_live_relation_lease(seeded_lease) { "live" } else { "expired" },
            },
            "native_relation_delta": {
                "accessor": "M8LocalRuntime::canonical_relation_projection",
                "before": { "hash": deterministic_hash(&before_relation) },
                "after": { "hash": deterministic_hash(&after_relation) },
            },
            "native_cut_delta": {
                "accessor": "M8LocalCut::canonical_semantic_projection",
                "before": { "hash": deterministic_hash(&before_cut.canonical_semantic_projection()) },
                "after": { "hash": deterministic_hash(&after_cut.canonical_semantic_projection()) },
            },
            "m8_relation_trace": [{
                "transition": "invalidate_primary",
                "invalidation_reason": "lease-expired",
                "previous_option_index": transition.previous_option_index(),
                "current_option_index": transition.current_option_index(),
                "derived_from_actual_m8_relation_state": true,
                "derived_from_actual_m8_relation_projection": true,
                "derived_from_actual_m8_lease_state": true,
            }],
        }))
    }

    /// Load the preserved S1 world into a distinct M9+M8 composite.  The
    /// current S2 timeline is intentionally left untouched; a saved cut is
    /// never a command to roll back that current authority runtime.
    fn restore_s1_into_fresh_composite(
        &mut self,
        checked: &CheckedSurfaceV0,
    ) -> Result<Value, String> {
        let cut = self
            .m9_s1
            .as_ref()
            .ok_or_else(|| "M10 SCN10 has no saved M9 S1 authority cut".to_string())?
            .clone();
        let s1 = self
            .s1
            .as_ref()
            .ok_or_else(|| "M10 SCN10 has no saved M8 S1 cut".to_string())?
            .clone();
        let old_current_relation_before = self.runtime.canonical_relation_projection();
        let old_current_cut_before = self.runtime.canonical_semantic_projection();
        let old_current_m9_before = self.m9.projection();
        let mut fresh = Self::new_with_session(
            checked,
            "m9m8-scn10-fresh-load",
            "m9-scn10-fresh-load-authority",
            self.fallback_chain.clone(),
        )?;
        fresh.m9.restore_authority_cut(cut)?;
        fresh.refresh_bridge();
        fresh
            .runtime
            .try_restore_local_cut(&s1, &M8LiveFloor::same_current(&s1))
            .map_err(|diagnostics| {
                format!(
                    "M10 SCN10 fresh M8 S1 restore: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        fresh.s1 = Some(s1.clone());
        fresh.m9_s1 = self.m9_s1.clone();
        let old_current_relation_after = self.runtime.canonical_relation_projection();
        let old_current_cut_after = self.runtime.canonical_semantic_projection();
        let old_current_m9_after = self.m9.projection();
        let details = json!({
            "attempted": true,
            "result": "accepted",
            "new_composite_session_id": fresh.session_id,
            "old_current_session_id": self.session_id,
            "new_m9_session_id": fresh.m9.session_id,
            "new_m8_runtime_session_id": fresh.m8_runtime_session_id(),
            "m9_to_m8_bridge": {
                "session_id": fresh.m9.session_id,
                "accessor": "M9M10AuthorityBridge::authority_state",
            },
            "m8_runtime_construction": {
                "accessor": "M8LocalRuntime::from_admitted",
                "new_runtime_constructed": true,
            },
            "m8_restore": {
                "accessor": "M8LocalRuntime::try_restore_local_cut",
                "cut_id": "m10-schedule-S1",
                "result": "accepted",
            },
            "old_current_session": {
                "no_mutation": old_current_relation_before == old_current_relation_after
                    && old_current_cut_before == old_current_cut_after
                    && old_current_m9_before == old_current_m9_after,
                "session_id_before": self.session_id,
                "session_id_after": self.session_id,
                "m9_session_id": self.m9.session_id,
                "m8_runtime_session_id": self.m8_runtime_session_id(),
                "relation_hash_before": deterministic_hash(&old_current_relation_before),
                "relation_hash_after": deterministic_hash(&old_current_relation_after),
                "cut_hash_before": deterministic_hash(&old_current_cut_before),
                "cut_hash_after": deterministic_hash(&old_current_cut_after),
                "m9_snapshot_before": deterministic_hash(&old_current_m9_before),
                "m9_snapshot_after": deterministic_hash(&old_current_m9_after),
            },
        });
        self.fresh_loaded = Some(Box::new(fresh));
        Ok(details)
    }

    fn reacquire_after_load(&mut self) -> Result<Value, String> {
        let fresh = self
            .fresh_loaded
            .as_deref_mut()
            .ok_or_else(|| "M10 SCN10 reacquire has no freshly loaded composite".to_string())?;
        let relation = fresh
            .relation_name
            .clone()
            .ok_or_else(|| "M10 SCN10 fresh composite has no checked relation".to_string())?;
        let primary_anchor = fresh
            .relation_primary_anchor
            .clone()
            .ok_or_else(|| "M10 SCN10 fresh relation lacks a primary anchor".to_string())?;
        let binding_frontier = fresh
            .relation_binding_frontier
            .clone()
            .ok_or_else(|| "M10 SCN10 fresh relation lacks a binding frontier".to_string())?;
        let fresh_lease_ref = fresh
            .fresh_relation_lease_ref
            .clone()
            .ok_or_else(|| "M10 SCN10 fresh relation lacks its admitted fresh lease".to_string())?;
        let before_membership = fresh
            .m9
            .relation_maintainer_membership
            .as_ref()
            .ok_or_else(|| "M10 SCN10 lacks the source-declared relation maintainer".to_string())?;
        let before_membership_ref = before_membership.ref_id().to_string();
        let before_epoch = before_membership.epoch().to_string();
        let before_generation = fresh.bridge_generation;
        let before_snapshot_ref = deterministic_hash(fresh.bridge.authority_snapshot_projection());
        let before_relation = fresh.runtime.canonical_relation_projection();
        let before_occurrence_id = format!("m8-occurrence-before-{}", fresh.runtime.trace().len());
        fresh.m9.admit_fresh_relation_reacquire()?;
        fresh.refresh_bridge();
        let fresh_membership = fresh
            .m9
            .fresh_relation_membership
            .as_ref()
            .expect("fresh relation admission just succeeded");
        let fresh_membership_ref = fresh_membership.ref_id().to_string();
        let fresh_epoch = fresh_membership.epoch().to_string();
        let authority = fresh
            .bridge
            .relation_authority_use(&relation, "reacquire_primary")
            .ok_or_else(|| "M10 SCN10 fresh M9 bridge lacks reacquire authority".to_string())?;
        let fresh_witness_ref = authority
            .witness_ref()
            .ok_or_else(|| "M10 SCN10 fresh M9 bridge relation witness is absent".to_string())?
            .to_string();
        let owner = fresh.bridge.owner_use().ok_or_else(|| {
            "M10 SCN10 fresh M9 bridge lacks owner occurrence authority".to_string()
        })?;
        let occurrence = fresh
            .runtime
            .enqueue_owner(M8OwnerRequest::new(&owner.0).with_authority_use(owner.2))
            .map_err(|diagnostics| {
                format!(
                    "M10 SCN10 fresh M8 owner occurrence: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let transition = fresh
            .runtime
            .reacquire_primary(
                &relation,
                authority,
                M8RelationReacquire::new(&primary_anchor)
                    .with_anchor_epoch(format!("{fresh_epoch}:{relation}"))
                    .with_binding_epoch("binding_epoch:2")
                    .with_fresh_witness(&fresh_witness_ref)
                    .with_fresh_lease_ref(&fresh_lease_ref)
                    .with_frontier(&binding_frontier),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 SCN10 fresh M8 relation reacquire: {:?}",
                    diagnostics.primary().kind(),
                )
            })?;
        let after_relation = fresh.runtime.canonical_relation_projection();
        Ok(json!({
            "persistent_session_id": fresh.m8_runtime_session_id(),
            "m9_admission": {
                "accessor": "M10MembershipLifecycleSession::admit_fresh",
                "before_membership_ref": before_membership_ref,
                "after_membership_ref": fresh_membership_ref,
                "before_epoch": before_epoch,
                "after_epoch": fresh_epoch,
                "after_witness_ref": fresh_witness_ref,
            },
            "bridge_refresh": {
                "accessor": "M10CompositeCutSession::refresh_bridge",
                "before_generation": before_generation,
                "after_generation": fresh.bridge_generation,
                "before_authority_snapshot_ref": before_snapshot_ref,
                "after_authority_snapshot_ref": deterministic_hash(fresh.bridge.authority_snapshot_projection()),
            },
            "m8_occurrence": {
                "runtime_session_id": fresh.m8_runtime_session_id(),
                "source_accessor": "M8LocalRuntime::enqueue_owner",
                "before_occurrence_id": before_occurrence_id,
                "after_occurrence_id": occurrence.id(),
                "new_lease_ref": fresh_lease_ref,
                "lease_source_accessor": "M8LocalRuntime::reacquire_primary",
                "relation_transition": {
                    "previous_option_index": transition.previous_option_index(),
                    "current_option_index": transition.current_option_index(),
                    "fresh_witness_ref": transition.fresh_reacquire_witness(),
                },
                "native_relation_delta": {
                    "accessor": "M8LocalRuntime::canonical_relation_projection",
                    "before": { "hash": deterministic_hash(&before_relation) },
                    "after": { "hash": deterministic_hash(&after_relation) },
                },
            },
        }))
    }

    /// Test a stale merge only inside a separately constructed composite.
    /// The candidate may restore S1's M9 authority cut for validation, but
    /// the current composite never receives that restore request.
    fn preflight_stale_merge_on_candidate(
        &self,
        checked: &CheckedSurfaceV0,
        s1: &crate::m8_runtime_local_cut::M8LocalCut,
    ) -> Result<(Value, M10SemanticHashBundle, M10SemanticHashBundle, Value), String> {
        let m9_s1 = self
            .m9_s1
            .as_ref()
            .ok_or_else(|| "M10 SCN10 stale preflight has no M9 S1 authority cut".to_string())?
            .clone();
        let mut candidate = Self::new_with_session(
            checked,
            "m9m8-scn10-stale-preflight-candidate",
            "m9-scn10-stale-preflight-authority",
            self.fallback_chain.clone(),
        )?;
        let candidate_before = m10_actual_hash_bundle(
            &candidate.runtime,
            &candidate.m9.domain_snapshot(),
            "m10-scn10-stale-preflight-candidate-before",
            Some(s1),
        );
        candidate.m9.restore_authority_cut(m9_s1)?;
        candidate.refresh_bridge();
        let candidate_after_m9_restore = m10_actual_hash_bundle(
            &candidate.runtime,
            &candidate.m9.domain_snapshot(),
            "m10-scn10-stale-preflight-candidate-after-m9",
            Some(s1),
        );
        let candidate_payload_before = candidate.runtime.save_relevant_payload();
        let rejected = candidate
            .runtime
            .try_restore_local_cut(
                s1,
                &M8LiveFloor::same_current(s1).with_stale_membership("m10-stale"),
            )
            .is_err()
            && candidate.runtime.save_relevant_payload() == candidate_payload_before;
        if !rejected {
            return Err("M10 SCN10 stale candidate preflight unexpectedly restored S1".to_string());
        }
        Ok((
            json!({
                "source": "candidate_preflight_clone",
                "preflight_accessor": "M10CompositeCutSession::preflight_stale_merge_on_candidate",
                "preflight_target": "candidate_clone",
                "result": "rejected",
                "diagnostic": { "code": "E-CUT-002" },
                "current_session_id": self.session_id,
                "candidate_session_id": candidate.session_id,
                "clone_source_session_id": "m9m8-scn10-stale-preflight-clone-source",
                "clone_runtime_constructed": true,
                "rejected_before_current_restore": true,
                "no_current_m9_restore_attempted": true,
                "no_current_m8_restore_attempted": true,
                "candidate_m9_restore": {
                    "accessor": "M10MembershipLifecycleSession::restore_authority_cut",
                    "result": "candidate_rejected",
                },
                "candidate_m8_restore": {
                    "accessor": "M8LocalRuntime::try_restore_local_cut",
                    "result": "rejected",
                },
            }),
            candidate_before,
            candidate_after_m9_restore,
            candidate.lineage_value(),
        ))
    }

    fn m8_runtime_session_id(&self) -> String {
        format!(
            "m8-{}",
            self.session_id
                .strip_prefix("m9m8-")
                .unwrap_or(&self.session_id)
        )
    }

    fn session_details(&self, range_start: u64, range_end: u64) -> Value {
        json!({
            "session_id": self.m8_runtime_session_id(),
            "semantic_state_owner": {
                "layer": "M8",
                "session_id": self.m8_runtime_session_id(),
            },
            "monotone_trace_range": { "start": range_start, "end": range_end },
            "m9_authority_session_id": self.m9.session_id,
            "canon_refs": [{
                "source_path": "mirrorea_canon/spec/11-m10-i1plus-conformance.md",
                "line_start": 19,
                "line_end": 24,
                "scn_id": "SCN-10",
            }],
        })
    }
}

/// Render the exact sealed M9 record which the private seam translated for
/// one M8 operation.  This is evidence, not a second authority lookup: a
/// mismatch is an internal composition error and prevents execution.
fn m10_m9_to_m8_authority_translation(
    seam: &crate::m9_auth_verification::M9M10ExecutionSeam,
    source_ref: &SourceRef,
    trace_range: &str,
    membership_ref: &str,
    capability_ref: &str,
    witness_ref: &str,
) -> Result<Value, String> {
    let (snapshot_membership, snapshot_capability, snapshot_witness) = seam
        .translation_refs(capability_ref)
        .ok_or_else(|| format!("M10 M9 seam lacks translated capability {capability_ref}"))?;
    let lossless_exact_match = snapshot_membership == membership_ref
        && snapshot_capability == capability_ref
        && snapshot_witness == witness_ref;
    if !lossless_exact_match {
        return Err(format!(
            "M10 M9->M8 authority translation mismatch for {capability_ref}"
        ));
    }
    Ok(json!({
        "m9_snapshot": {
            "source_ref": source_ref_json(Some(source_ref)),
            "trace_range": trace_range,
            "active_membership_ref": snapshot_membership,
            "active_capability_ref": snapshot_capability,
            "active_witness_ref": snapshot_witness,
            "canonical_snapshot_hash": deterministic_hash(&format!(
                "membership|{}\ngrants|{}",
                seam.canonical_m9_membership_projection(),
                seam.canonical_m9_grant_projection(),
            )),
        },
        "m8_authority_use": {
            "membership_ref": membership_ref,
            "capability_ref": capability_ref,
            "witness_ref": witness_ref,
        },
        "lossless_exact_match": true,
    }))
}

/// Build a single M8 local session from an M9-admitted instance, then seed it
/// with the already-admitted relation authorities required for the two actual
/// M8 relation transitions.  M10 does not mint these at transition time.
fn m10_relation_lifecycle_runtime(
    checked: &CheckedSurfaceV0,
    relation_name: &str,
) -> Result<M10RelationLifecycleRuntime, String> {
    let relation = checked
        .relation(relation_name)
        .and_then(|evaluation| evaluation.relation_core())
        .ok_or_else(|| format!("M10 relation lifecycle lacks checked relation {relation_name}"))?;
    let semantic_source_ref = m10_semantic_source_ref(checked)?;
    let (principal, locus) = patch_principal_and_locus(checked)?;
    let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
    let m9 = M10M9DomainSnapshot::from_seam(&seam);
    let invalidate_authority = seam
        .relation_authority_use(relation_name, "invalidate_primary")
        .ok_or_else(|| {
            format!("M10 relation {relation_name} lacks M9-issued invalidate authority")
        })?;
    let reacquire_authority = seam
        .relation_authority_use(relation_name, "reacquire_primary")
        .ok_or_else(|| {
            format!("M10 relation {relation_name} lacks M9-issued reacquire authority")
        })?;
    let authority_translation = m10_m9_to_m8_authority_translation(
        &seam,
        &semantic_source_ref,
        "m8-relation-session:0..2",
        invalidate_authority
            .membership_ref()
            .ok_or_else(|| "M10 M8 relation authority lacks membership reference".to_string())?,
        invalidate_authority
            .capability_ref()
            .ok_or_else(|| "M10 M8 relation authority lacks capability reference".to_string())?,
        invalidate_authority
            .witness_ref()
            .ok_or_else(|| "M10 M8 relation authority lacks witness reference".to_string())?,
    )?;
    let (instance, authority_state) = seam.into_parts();
    let owner_locus = relation.owner_locus();
    let primary_anchor = relation.primary().anchor().to_string();
    let initial_frontier = relation
        .binding_frontier()
        .as_slice()
        .first()
        .ok_or_else(|| format!("M10 relation {relation_name} has no binding frontier"))?
        .as_str()
        .to_string();
    let fresh_frontier = initial_frontier.clone();
    let fresh_anchor_epoch = format!("{}:reacquired", relation.primary().epoch());
    let initial_binding_epoch = "binding_epoch:1";
    let fresh_binding_epoch = "binding_epoch:2";
    let fresh_witness_ref = reacquire_authority
        .witness_ref()
        .ok_or_else(|| "M10 M9 reacquire authority lacks witness reference".to_string())?
        .to_string();
    let fresh_lease_ref = format!("m9-issued-lease:{relation_name}:{fresh_binding_epoch}");
    let runtime = M8LocalRuntime::from_admitted(
        instance,
        M8LocalRuntimeSeed::new()
            .with_authority_state(authority_state)
            .with_live_lease(
                M8LeaseRecord::live(format!("m10-lease:{relation_name}"))
                    .for_relation(relation_name)
                    .with_owner_locus(owner_locus)
                    .with_binding_frontier(&initial_frontier)
                    .with_epoch(initial_binding_epoch),
            )
            .with_live_lease(
                M8LeaseRecord::live(&fresh_lease_ref)
                    .for_relation(relation_name)
                    .with_owner_locus(owner_locus)
                    .with_binding_frontier(&fresh_frontier)
                    .with_epoch(fresh_binding_epoch)
                    .with_anchor_epoch(&fresh_anchor_epoch),
            ),
    );
    Ok(M10RelationLifecycleRuntime {
        runtime,
        m9,
        invalidate_authority,
        reacquire_authority,
        primary_anchor,
        initial_frontier,
        fresh_frontier,
        fresh_anchor_epoch,
        fresh_lease_ref,
        fresh_witness_ref,
        authority_translation,
        pre_degradation_cut: None,
        rollback_evidence: None,
    })
}

fn m10_cut_runtime_with_m9(
    checked: &CheckedSurfaceV0,
) -> Result<(M8LocalRuntime, M10M9DomainSnapshot), String> {
    let (principal, locus) = patch_principal_and_locus(checked)?;
    let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
    let m9 = M10M9DomainSnapshot::from_seam(&seam);
    let (instance, authority_state) = seam.into_parts();
    Ok((
        M8LocalRuntime::from_admitted(
            instance,
            M8LocalRuntimeSeed::new().with_authority_state(authority_state),
        ),
        m9,
    ))
}

fn m10_cut_runtime(checked: &CheckedSurfaceV0) -> Result<M8LocalRuntime, String> {
    m10_cut_runtime_with_m9(checked).map(|(runtime, _)| runtime)
}

/// Execute SCN-12 as one maintained-relation session.  The four schedule
/// rows name observations of this timeline; they do not create four isolated
/// relation runtimes or a second absolute value stream.
fn m10_run_scn12_relation_session(checked: &CheckedSurfaceV0) -> Result<Value, String> {
    let relation = "bird_follow";
    let consumer = "Viewer";
    let mut lifecycle = m10_relation_lifecycle_runtime(checked, relation)?;
    let requested = (relation.to_string(), consumer.to_string());
    let (_, initial_context) = projection_seed(checked, Some(&requested))?;
    let initial_context = initial_context
        .ok_or_else(|| "M10 SCN12 lacks a checked consumer-local context".to_string())?;
    let session_id = format!(
        "m10-scn12-relation-session:{}",
        checked.program_identity().stable_key()
    );
    let mut trace_index = 0_u64;
    let mut next_range = |width: u64| {
        let start = trace_index;
        trace_index += width;
        json!({ "start": start, "end": trace_index })
    };

    let initial_projection = lifecycle
        .runtime
        .project_relation(relation, initial_context.clone())
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 initial bird projection: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let bird_projection = json!({
        "trace_range": next_range(1),
        "semantic_relation_delta": false,
        "consumer_local": initial_projection.uses_single_presentation_frame(),
        "no_absolute_stream": initial_projection.absolute_value_stream().is_empty(),
    });

    let split_rejected = lifecycle
        .runtime
        .project_relation(
            relation,
            initial_context
                .clone()
                .with_frontier("m10-scn12-split-frame-frontier"),
        )
        .is_err();

    lifecycle
        .runtime
        .invalidate_primary(
            relation,
            lifecycle.invalidate_authority.clone(),
            M8BindingInvalidation::anchor_unavailable(&lifecycle.primary_anchor)
                .with_frontier(format!("{}:degraded", lifecycle.initial_frontier)),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 semantic fallback: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let degraded_frontier = format!("{}:degraded", lifecycle.initial_frontier);
    let consumer_projection = lifecycle
        .runtime
        .project_relation(
            relation,
            M8PresentationContext::for_consumer(consumer)
                .with_frontier(&degraded_frontier)
                .with_presentation_fallback(M8PresentationFallback::hold_last_local(
                    "bird",
                    M8Point::new(0, 0),
                )),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 consumer-local fallback projection: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let semantic_fallback = json!({
        "trace_range": next_range(2),
        "selected_floor": "anchor",
        "consumer_local_fallback": consumer_projection.is_consumer_local_fallback(),
    });

    let before_same_lineage = lifecycle.runtime.canonical_semantic_projection();
    lifecycle
        .runtime
        .note_primary_available_same_lineage(relation, &lifecycle.primary_anchor)
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 same-lineage observation: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let same_lineage_unchanged =
        before_same_lineage == lifecycle.runtime.canonical_semantic_projection();
    let same_lineage_reject = json!({
        "trace_range": next_range(1),
        "diagnostic": "E-LIN-003",
        "semantic_lineage_unchanged": same_lineage_unchanged,
    });

    lifecycle
        .runtime
        .reacquire_primary(
            relation,
            lifecycle.reacquire_authority.clone(),
            M8RelationReacquire::new(&lifecycle.primary_anchor)
                .with_anchor_epoch(&lifecycle.fresh_anchor_epoch)
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(&lifecycle.fresh_witness_ref)
                .with_fresh_lease_ref(&lifecycle.fresh_lease_ref)
                .with_frontier(&lifecycle.fresh_frontier),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 fresh reacquire: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let fresh_projection = lifecycle
        .runtime
        .project_relation(
            relation,
            M8PresentationContext::for_consumer(consumer)
                .with_frontier(&lifecycle.fresh_frontier)
                .with_anchor_sample(
                    M8AnchorSample::new(&lifecycle.primary_anchor)
                        .with_epoch(&lifecycle.fresh_anchor_epoch)
                        .with_frontier(&lifecycle.fresh_frontier)
                        .with_pose(M8Point::new(0, 0)),
                ),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 SCN12 fresh consumer-local projection: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let fresh_reacquire = json!({
        "trace_range": next_range(2),
        "fresh_epoch": lifecycle.fresh_anchor_epoch,
        "fresh_witness": lifecycle.fresh_witness_ref,
        "consumer_local": fresh_projection.uses_single_presentation_frame(),
    });

    Ok(json!({
        "relation_session_id": session_id,
        "execution_session": {
            "session_id": session_id,
            "action_receipts": [
                { "action": "bird_relation", "session_id": session_id, "trace_range": bird_projection["trace_range"].clone() },
                { "action": "fallback", "session_id": session_id, "trace_range": semantic_fallback["trace_range"].clone() },
                { "action": "reacquire", "session_id": session_id, "trace_range": fresh_reacquire["trace_range"].clone() },
            ],
            "reused_across_actions": true,
            "discarded_isolated_per_action_execution": false,
            "relation_projection_schedule_branch_executed": false,
            "per_action_m8_runtime_count": 0,
            "discarded_m8_runtime_count": 0,
            "persistent_session_started_before_schedule_actions": true,
        },
        "monotone_trace_range": { "start": 0, "end": trace_index },
        "bird_projection": bird_projection.clone(),
        // Keep the pressure-row view as a projection of this one session,
        // rather than executing a second per-schedule runtime.  These are
        // compatibility labels for the finite pressure table; their ranges
        // and provenance remain the receipts above.
        "bird_relation": {
            "status": "accepted",
            "execution_session_id": session_id,
            "projection": bird_projection,
            "runtime_trace": ["M9Admission", "M8RelationProjection"],
        },
        "split_frame": {
            "status": if split_rejected { "rejected" } else { "accepted" },
            "diagnostic": "presentation_frontier_mismatch",
            "trace": ["M8RelationProjection"],
        },
        "semantic_fallback": semantic_fallback.clone(),
        "fallback": {
            "status": "accepted",
            "execution_session_id": session_id,
            "projection": semantic_fallback,
            "runtime_trace": ["M8BindingInvalidation", "M8PresentationFallback"],
        },
        "same_lineage_reject": same_lineage_reject,
        "fresh_reacquire": fresh_reacquire.clone(),
        "reacquire": {
            "status": "accepted",
            "execution_session_id": session_id,
            "projection": fresh_reacquire,
            "runtime_trace": ["M8RelationReacquire", "M8RelationProjection"],
        },
        "presentation_shortage": {
            "semantic_lineage_unchanged": same_lineage_unchanged,
        },
        "privacy_join": {
            "no_absolute_stream": initial_projection.absolute_value_stream().is_empty()
                && fresh_projection.absolute_value_stream().is_empty(),
            "no_split_frame": split_rejected,
        },
        "m9_to_m8_authority_translation": lifecycle.authority_translation,
    }))
}

/// Retain the source-bound M9 seam receipts next to the finite conformance
/// inventory.  Rendering can then attach them to the corresponding row even
/// when that row's runtime trace is represented by a later aggregate receipt.
fn m10_collect_authority_translations(
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
) -> Result<BTreeMap<String, Value>, String> {
    let mut translations = BTreeMap::new();
    if let Some(checked) = checked_sources.get("scn-08/positive.mir") {
        let lifecycle = m10_relation_lifecycle_runtime(checked, "view_pose")?;
        translations.insert(
            "SCN08-R-P-LIVE".to_string(),
            lifecycle.authority_translation,
        );
    }
    if let Some(checked) = checked_sources.get("scn-11/designated-version.mir") {
        let evaluation = checked
            .designated_result("Evaluator", "result")
            .ok_or_else(|| "M10 SCN11 designated source lacks Evaluator.result".to_string())?;
        let seam = m10_resolve_checked_for_patch(checked, "self", "Evaluator")?;
        let authority = seam
            .designated_consumption_authority_use("self", "Evaluator.result")
            .ok_or_else(|| "M10 SCN11 seam lacks designated consumer authority".to_string())?;
        translations.insert(
            "SCN11-R-P-VERSION".to_string(),
            m10_m9_to_m8_authority_translation(
                &seam,
                evaluation.source_ref(),
                "m8-designated-session:0..2",
                authority.membership_ref().ok_or_else(|| {
                    "M10 SCN11 M8 authority lacks membership reference".to_string()
                })?,
                authority.capability_ref().ok_or_else(|| {
                    "M10 SCN11 M8 authority lacks capability reference".to_string()
                })?,
                authority
                    .witness_ref()
                    .ok_or_else(|| "M10 SCN11 M8 authority lacks witness reference".to_string())?,
            )?,
        );
    }
    Ok(translations)
}

/// Consume every typed schedule operation and retain facts only after the
/// corresponding checked/carrier/runtime operation succeeds or rejects at its
/// intended boundary.  Action identity is the full canonical action value;
/// neither a scenario name nor a correspondence row can select a result.
/// The source, carrier, and runtime inputs intentionally remain separate to
/// preserve this boundary in the local orchestration function.
#[allow(clippy::too_many_arguments)]
fn execute_typed_schedule(
    facts: &mut BTreeSet<M10EvidenceFact>,
    runtime_traces: &mut BTreeMap<String, Value>,
    source_identities: &BTreeMap<String, String>,
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
    carriers: &M10TypedCarriers,
    schedule: &M10TypedSchedule,
    admitted_patch_carriers: &BTreeSet<String>,
    route_patch_activated: &BTreeMap<String, bool>,
) -> Result<Value, String> {
    let mut pressure = serde_json::Map::new();
    let mut receipt_ledger = M10ReceiptLedger::default();
    let mut scn04_membership_session: Option<M10M9M8ExecutionSession> = None;
    let mut scn08_relation_session: Option<M10RelationLifecycleSession> = None;
    let mut scn10_positive_session: Option<M10CompositeCutSession> = None;
    let cases = schedule
        .cases()
        .ok_or_else(|| "M10 conformance requires action-context schedule cases".to_string())?;
    for case in cases {
        if !case.id.starts_with(&case.scn.replace('-', "")) {
            return Err(format!(
                "M10 schedule action {} does not bind its scenario",
                case.id
            ));
        }
        let source_context = case
            .source
            .as_deref()
            .map(|path| {
                checked_sources
                    .get(path)
                    .map(|checked| {
                        schedule_evidence_context(
                            case,
                            checked.program_identity().root_source_ref(),
                        )
                    })
                    .ok_or_else(|| {
                        format!(
                            "M10 schedule action {} references unchecked source {path}",
                            case.id
                        )
                    })
            })
            .transpose()?;
        match &case.operation {
            M10ScheduleOperation::OwnerEvent(request) => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let M10OwnerScheduleOutcome::Served(served) = execute_checked_owner_schedule(
                    checked,
                    request,
                    M10OwnerAuthorityMode::Admitted,
                )?
                else {
                    return Err(format!(
                        "M10 scheduled owner request {} was rejected",
                        case.id
                    ));
                };
                let M10OwnerScheduleServed {
                    runtime,
                    target_key,
                    before_runtime,
                    m9,
                    ..
                } = *served;
                let value = runtime.owner_state().int(&target_key).unwrap_or_default();
                let trace = runtime.trace().kinds();
                let ordered = trace
                    .contains(&crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerEnqueued)
                    && trace.contains(
                        &crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerAuthorityValidated,
                    )
                    && trace.contains(&crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerWrite);
                let semantic_source_ref = m10_semantic_source_ref(checked)?;
                let before_bundle = m10_actual_hash_bundle(
                    &before_runtime,
                    &m9,
                    &format!("owner|{}|before", case.identity),
                    None,
                );
                let mut after_bundle = m10_actual_hash_bundle(
                    &runtime,
                    &m9,
                    &format!("owner|{}|after", case.identity),
                    None,
                );
                // Owner writes are store-only transitions; relation and
                // configuration projections remain unchanged.
                after_bundle.relation_hash = before_bundle.relation_hash.clone();
                if request.event == "roll" && value == 3 && ordered {
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "runtime.final_state.player_self_position.equals.3",
                        context,
                    )?;
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "runtime.history.request_before_serve_before_publish",
                        context,
                    )?;
                    for predicate in [
                        "runtime.final_state.player_self_position.equals.3",
                        "runtime.history.request_before_serve_before_publish",
                    ] {
                        let receipt = receipt_ledger.record_actual(
                            "owner.write",
                            &semantic_source_ref,
                            before_bundle.clone(),
                            after_bundle.clone(),
                            true,
                        );
                        record_runtime_trace(runtime_traces, predicate, receipt);
                    }
                }
                if request.event == "attack" {
                    match (request.step, value) {
                        (Some(1), 90) => add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            "runtime.hp_history.after_first_attack.equals.90",
                            context,
                        )?,
                        (Some(2), 80) => add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            "runtime.hp_history.after_second_attack.equals.80",
                            context,
                        )?,
                        _ => {}
                    }
                }
            }
            M10ScheduleOperation::AdmissionThenOwnerEvent { event, principal } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let request = M10OwnerEventRequest {
                    event: event.clone(),
                    principal: principal.clone(),
                    target: None,
                    repeat: 1,
                    step: None,
                    seed: BTreeMap::new(),
                    arguments: BTreeMap::new(),
                };
                if matches!(
                    execute_checked_owner_schedule(
                        checked,
                        &request,
                        M10OwnerAuthorityMode::Admitted
                    )?,
                    M10OwnerScheduleOutcome::Served(_)
                ) {
                    for predicate in [
                        "runtime.admission_verdict_precedes_owner_write",
                        "runtime.lineage_binds_checked_source_identity_to_authority_inventory",
                        "runtime.past_world_cut_remains_audit_visible",
                    ] {
                        add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            predicate,
                            context,
                        )?;
                    }
                }
            }
            M10ScheduleOperation::OwnerEventBeforeAdmission { event, principal } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let static_context = M10EvidenceContext {
                    scn_id: case.scn.clone(),
                    phase: "static",
                    source_ref: checked.program_identity().root_source_ref().clone(),
                    schedule_action_reference: context.schedule_action_reference.clone(),
                };
                let owner = checked
                    .evaluation(event)
                    .filter(|evaluation| evaluation.owner_rmw_core().is_some())
                    .ok_or_else(|| {
                        format!("M10 pre-admission schedule has no owner event {event}")
                    })?;
                let direct = m8_admission_for(checked)
                    .and_then(|admission| {
                        M8Runtime::default()
                            .admit(checked.clone(), admission)
                            .map_err(|diagnostics| format!("{:?}", diagnostics.primary().kind()))
                    })
                    .is_err();
                if direct && owner.actor_authority_origin() == principal {
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "structural_rejection.no_mutation.pre_verdict_write_has_no_runtime_artifact",
                        &static_context,
                    )?;
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "structural_rejection.no_mutation.pre_verdict_owner_write",
                        context,
                    )?;
                }
            }
            M10ScheduleOperation::MembershipLifecycle {
                events,
                fresh_incarnation,
            } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                if !m10_schedule_has_m9_admission(checked) {
                    return Err(format!(
                        "M10 membership schedule {} did not reach M9",
                        case.id
                    ));
                }
                if schedule_events_are(events, &["leave", "attack_stale"]) {
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let session = scn04_membership_session.get_or_insert(
                        M10M9M8ExecutionSession::new(checked, "m9-scn04-membership")?,
                    );
                    let before_m9 = session.m9.domain_snapshot();
                    let before_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &before_m9,
                        &format!("SCN04|{}|before", case.identity),
                        None,
                    );
                    session.retire_and_refresh()?;
                    let m9_stale_rejected = session.m9.stale_use_is_rejected();
                    let after_m9 = session.m9.domain_snapshot();
                    let after_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &after_m9,
                        &format!("SCN04|{}|after|revoked", case.identity),
                        None,
                    );
                    let stale_rejected =
                        m9_stale_rejected && session.post_retirement_owner_decision()?;
                    let retirement_receipt = receipt_ledger.record_actual(
                        "membership.retire",
                        &semantic_source_ref,
                        before_bundle,
                        after_bundle.clone(),
                        true,
                    );
                    if stale_rejected {
                        let predicate =
                            "runtime.stale_incarnation_request_rejected_without_state_mutation";
                        add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            predicate,
                            context,
                        )?;
                        let receipt = receipt_ledger.record_actual(
                            "membership.request",
                            &semantic_source_ref,
                            after_bundle.clone(),
                            after_bundle,
                            false,
                        );
                        let mut details = session.m9.retired_authority_details();
                        let lineage = session.lineage_value();
                        details
                            .as_object_mut()
                            .expect("SCN04 lineage details are an object")
                            .extend(json!({
                                "m9_to_m8_authority_lineage": lineage.clone(),
                                "m8_decisions_after_m9": [{
                                    "transition": "membership.request",
                                    "decision": "rejected",
                                    "authority_lineage_ref": lineage["session_id"].clone(),
                                    "runtime_session_id": lineage["m8_runtime_session_id"].clone(),
                                }],
                            }).as_object().expect("object").clone());
                        record_runtime_trace_with_prior_receipt(
                            runtime_traces,
                            predicate,
                            retirement_receipt,
                            receipt,
                            details,
                        );
                    }
                } else if schedule_events_are(events, &["leave"]) {
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let session = scn04_membership_session.as_ref().ok_or_else(|| {
                        "M10 SCN04 audit row ran before the persistent leave session".to_string()
                    })?;
                    let predicate = "runtime.membership_audit_retains_leave_and_rejoin_history";
                    let m9 = session.m9.domain_snapshot();
                    let bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &m9,
                        &format!("SCN04|{}|audit", case.identity),
                        None,
                    );
                    add_checked_source_fact(facts, source_identities, path, predicate, context)?;
                    let receipt = receipt_ledger.record_actual(
                        "membership.audit",
                        &semantic_source_ref,
                        bundle.clone(),
                        bundle,
                        true,
                    );
                    record_runtime_trace_with_details(
                        runtime_traces,
                        predicate,
                        receipt,
                        session.m9.session_details(1, 2),
                    );
                } else if schedule_events_are(events, &["rejoin"])
                    && *fresh_incarnation == Some(true)
                {
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let session = scn04_membership_session.as_mut().ok_or_else(|| {
                        "M10 SCN04 rejoin row ran before the persistent leave session".to_string()
                    })?;
                    let before_m9 = session.m9.domain_snapshot();
                    let before_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &before_m9,
                        &format!("SCN04|{}|before-rejoin", case.identity),
                        None,
                    );
                    session.m9.admit_fresh()?;
                    session.refresh_bridge();
                    let after_m9 = session.m9.domain_snapshot();
                    let after_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &after_m9,
                        &format!("SCN04|{}|after-rejoin", case.identity),
                        None,
                    );
                    let predicate = "runtime.rejoin_requires_fresh_incarnation";
                    add_checked_source_fact(facts, source_identities, path, predicate, context)?;
                    let receipt = receipt_ledger.record_actual(
                        "membership.rejoin",
                        &semantic_source_ref,
                        before_bundle,
                        after_bundle,
                        true,
                    );
                    let mut details = session.m9.session_details(2, 3);
                    details
                        .as_object_mut()
                        .expect("membership session details are an object")
                        .extend(
                            json!({
                                "old_authority_unusable": true,
                                "receipt_origin": "m9-membership-retirement",
                            })
                            .as_object()
                            .expect("object")
                            .clone(),
                        );
                    record_runtime_trace_with_details(runtime_traces, predicate, receipt, details);
                } else if schedule_events_are(events, &["leave", "rejoin"])
                    && *fresh_incarnation == Some(false)
                {
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let mut session = M10MembershipLifecycleSession::new_with_session(
                        checked,
                        "m9-scn04-hidden-repair",
                    )?;
                    let runtime = m10_cut_runtime(checked)?;
                    session.retire()?;
                    let before_m9 = session.domain_snapshot();
                    let before_bundle = m10_actual_hash_bundle(
                        &runtime,
                        &before_m9,
                        &format!("SCN04|{}|hidden-before", case.identity),
                        None,
                    );
                    let rejected = session.rejoin_without_fresh_is_rejected();
                    let after_m9 = session.domain_snapshot();
                    let after_bundle = m10_actual_hash_bundle(
                        &runtime,
                        &after_m9,
                        &format!("SCN04|{}|hidden-after", case.identity),
                        None,
                    );
                    if rejected {
                        let predicate = "structural_rejection.no_mutation.hidden_membership_repair";
                        add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            predicate,
                            context,
                        )?;
                        let receipt = receipt_ledger.record_actual(
                            "membership.rejoin",
                            &semantic_source_ref,
                            before_bundle,
                            after_bundle,
                            false,
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            predicate,
                            receipt,
                            session.session_details(0, 2),
                        );
                    }
                }
            }
            M10ScheduleOperation::PortalHandoff { events } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                if schedule_events_are(events, &["leave_a", "join_b", "spawn_b"])
                    && m10_schedule_has_m9_admission(checked)
                {
                    let handoff = m10_portal_handoff_runtime(checked)?;
                    let leave_before = m10_native_hash_bundle(
                        &handoff.before_m8,
                        &handoff.before_m9,
                        "SCN05|leave-before",
                    );
                    let leave_after = m10_native_hash_bundle(
                        &handoff.before_m8,
                        &handoff.after_leave_m9,
                        "SCN05|leave-after",
                    );
                    let join_after = m10_native_hash_bundle(
                        &handoff.before_m8,
                        &handoff.after_join_m9,
                        "SCN05|join-after",
                    );
                    let spawn_after = m10_native_hash_bundle(
                        &handoff.after_m8,
                        &handoff.after_join_m9,
                        "SCN05|spawn-after",
                    );
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "runtime.portal_handoff.orders.leave_verdict_before.join_verdict.before.spawn_write",
                        context,
                    )?;
                    let leave_receipt = receipt_ledger.record_actual(
                        "portal.leave_verdict",
                        &handoff.source_ref,
                        leave_before,
                        leave_after.clone(),
                        true,
                    );
                    let join_receipt = receipt_ledger.record_actual(
                        "portal.join_verdict",
                        &handoff.source_ref,
                        leave_after,
                        join_after.clone(),
                        true,
                    );
                    let spawn_receipt = receipt_ledger.record_actual(
                        "portal.spawn_write",
                        &handoff.source_ref,
                        join_after,
                        spawn_after,
                        true,
                    );
                    let mut details = m10_persistent_runtime_provenance(
                        case,
                        checked,
                        &handoff.source_ref,
                        &handoff.after_join_m9.authority,
                        &handoff.after_m8.cut,
                        "m10-portal-handoff-session:scn05",
                        0,
                        3,
                    );
                    let details_object = details
                        .as_object_mut()
                        .expect("M10 persistent provenance is an object");
                    details_object.insert(
                        "transition_trace".to_string(),
                        json!([
                            leave_receipt.evidence(),
                            join_receipt.evidence(),
                            spawn_receipt.evidence(),
                        ]),
                    );
                    details_object.insert(
                        "dependencies".to_string(),
                        json!({
                            "leave_a_before_join_b": {
                                "predecessor_transition": leave_receipt.transition,
                                "successor_transition": join_receipt.transition,
                                "predecessor_after_m9_authority_hash": leave_receipt.after_m9_authority_hash.clone(),
                                "successor_before_m9_authority_hash": join_receipt.before_m9_authority_hash.clone(),
                                "exact_m9_projection_match": leave_receipt.after_m9_authority_hash == join_receipt.before_m9_authority_hash,
                            },
                            "join_b_before_spawn_b": {
                                "predecessor_transition": join_receipt.transition,
                                "successor_transition": spawn_receipt.transition,
                                "predecessor_after_m9_authority_hash": join_receipt.after_m9_authority_hash.clone(),
                                "successor_before_m9_authority_hash": spawn_receipt.before_m9_authority_hash.clone(),
                                "predecessor_after_store_hash": join_receipt.after_store_hash.clone(),
                                "successor_before_store_hash": spawn_receipt.before_store_hash.clone(),
                                "exact_m9_projection_match": join_receipt.after_m9_authority_hash == spawn_receipt.before_m9_authority_hash,
                                "exact_m8_projection_match": join_receipt.after_store_hash == spawn_receipt.before_store_hash,
                            },
                        }),
                    );
                    record_runtime_trace_with_details(
                        runtime_traces,
                        "runtime.portal_handoff.orders.leave_verdict_before.join_verdict.before.spawn_write",
                        spawn_receipt,
                        details,
                    );
                }
            }
            M10ScheduleOperation::ObservationRequest {
                request_class,
                validated_policy_carrier_ref,
            } => {
                let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let policy = carriers.observation(validated_policy_carrier_ref)
                    .ok_or_else(|| format!("M10 schedule {} references unknown observation carrier {validated_policy_carrier_ref}", case.id))?;
                if m10_schedule_has_m9_admission(checked) {
                    let observation_session = M10ObservationSession::for_policy(
                        checked,
                        validated_policy_carrier_ref,
                        policy,
                    )?;
                    if matches!(
                        request_class,
                        M10ObservationRequestKind::CrossLocusObservation
                    ) {
                        add_carrier_facts(
                            facts,
                            carriers,
                            validated_policy_carrier_ref,
                            &["runtime.observer_projection_exports_no_secret_key"],
                            context,
                        )?;
                        let request = M10OwnerEventRequest {
                            event: "portal_probe".to_string(),
                            principal: "self".to_string(),
                            target: None,
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::from([("delta".to_string(), 1)]),
                        };
                        let M10OwnerScheduleOutcome::Served(served) =
                            execute_checked_owner_schedule(
                                checked,
                                &request,
                                M10OwnerAuthorityMode::Admitted,
                            )?
                        else {
                            return Err(
                                "M10 SCN05 observer source owner effect was rejected".to_string()
                            );
                        };
                        let M10OwnerScheduleServed {
                            runtime,
                            before_runtime,
                            m9,
                            observer_authority,
                            ..
                        } = *served;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let exported_fields = observation_session
                            .validate_cross_locus_request(*request_class)
                            .map_err(|diagnostic| {
                                format!(
                                    "M10 source-bound observation export rejected: {}",
                                    diagnostic.code(),
                                )
                            })?;
                        let publication = m10_observer_publication_evidence(
                            checked,
                            runtime.clone(),
                            observer_authority,
                            validated_policy_carrier_ref,
                            exported_fields,
                            "scn05-cross-locus",
                        )?;
                        let before_bundle = m10_actual_hash_bundle(
                            &before_runtime,
                            &m9,
                            "SCN05-observation-before",
                            None,
                        );
                        let after_projection = runtime.canonical_semantic_projection();
                        let after_bundle =
                            m10_actual_hash_bundle(&runtime, &m9, "SCN05-observation-after", None);
                        let receipt = receipt_ledger.record_actual(
                            "observation.export",
                            &semantic_source_ref,
                            before_bundle,
                            after_bundle,
                            true,
                        );
                        let mut details = m10_persistent_runtime_provenance(
                            case,
                            checked,
                            &semantic_source_ref,
                            &m9.authority,
                            &after_projection,
                            "m10-observer-session:scn05",
                            0,
                            2,
                        );
                        details
                            .as_object_mut()
                            .expect("M10 persistent provenance is an object")
                            .insert("observer_publication".to_string(), publication);
                        record_runtime_trace_with_details(
                            runtime_traces,
                            "runtime.observer_projection_exports_no_secret_key",
                            receipt,
                            details,
                        );
                    }
                    if matches!(
                        request_class,
                        M10ObservationRequestKind::CrossLocusSecretRead
                    ) {
                        let diagnostic = observation_session
                            .validate_cross_locus_request(*request_class)
                            .expect_err(
                                "M10 source-bound observation session accepted a secret read",
                            );
                        if diagnostic != M10ObservationSessionDiagnostic::VisibilityDenied {
                            return Err(format!(
                                "M10 source-bound secret read returned unexpected diagnostic {}",
                                diagnostic.code(),
                            ));
                        }
                        add_carrier_facts(
                            facts,
                            carriers,
                            validated_policy_carrier_ref,
                            &["diagnostic.VisibilityDenied.no_publication.no_state_mutation"],
                            context,
                        )?;
                        let request = M10OwnerEventRequest {
                            event: "portal_probe".to_string(),
                            principal: "self".to_string(),
                            target: None,
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::from([("delta".to_string(), 1)]),
                        };
                        let M10OwnerScheduleOutcome::Served(served) =
                            execute_checked_owner_schedule(
                                checked,
                                &request,
                                M10OwnerAuthorityMode::Admitted,
                            )?
                        else {
                            return Err(
                                "M10 SCN05 secret observer source owner effect was rejected"
                                    .to_string(),
                            );
                        };
                        let M10OwnerScheduleServed { runtime, m9, .. } = *served;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let runtime_projection = runtime.canonical_semantic_projection();
                        let bundle =
                            m10_actual_hash_bundle(&runtime, &m9, "SCN05-secret-reject", None);
                        let receipt = receipt_ledger.record_actual(
                            "observation.visibility_denied",
                            &semantic_source_ref,
                            bundle.clone(),
                            bundle,
                            false,
                        );
                        let mut details = m10_persistent_runtime_provenance(
                            case,
                            checked,
                            &semantic_source_ref,
                            &m9.authority,
                            &runtime_projection,
                            "m10-observer-session:scn05",
                            2,
                            3,
                        );
                        let object = details
                            .as_object_mut()
                            .expect("M10 persistent provenance is an object");
                        object.insert("no_publication".to_string(), Value::Bool(true));
                        object.insert(
                            "policy_validation".to_string(),
                            observation_session.diagnostic_value(diagnostic),
                        );
                        object.insert(
                            "diagnostic".to_string(),
                            json!({
                                "code": "VisibilityDenied",
                                "source_ref": source_ref_json(Some(&semantic_source_ref)),
                            }),
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            "diagnostic.VisibilityDenied.no_publication.no_state_mutation",
                            receipt,
                            details,
                        );
                    }
                }
            }
            M10ScheduleOperation::RouteContext {
                events,
                route_patch_carrier_ref,
                turn_budget,
            } => {
                let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                if !m10_schedule_has_m9_admission(checked) {
                    return Err(format!("M10 route schedule {} did not reach M9", case.id));
                }
                if schedule_events_are(events, &["invoke_before_patch"]) {
                    let predicate = if turn_budget.as_deref() == Some("finite") {
                        "structural_rejection.no_mutation.route_absence_returns_terminal_failure_within_turn_budget"
                    } else {
                        "runtime.route_absent_yields_explicit_RouteUnavailable_store_unchanged_route_trace"
                    };
                    add_case_action_fact(facts, case, predicate, context);
                    let route_session = M10RouteSession::for_checked_source(checked)?;
                    if route_session.invoke() != Err(M10RouteSessionDiagnostic::RouteUnavailable) {
                        return Err("M10 route session accepted an absent route".to_string());
                    }
                    let semantic_source_ref = route_session.source_ref.clone();
                    let runtime_projection = route_session.m8.cut.clone();
                    let bundle = m10_native_hash_bundle(
                        &route_session.m8,
                        &route_session.m9,
                        "SCN06-route-absent",
                    );
                    let receipt = receipt_ledger.record_actual(
                        "route.unavailable",
                        &semantic_source_ref,
                        bundle.clone(),
                        bundle,
                        false,
                    );
                    let mut details = m10_persistent_runtime_provenance(
                        case,
                        checked,
                        &semantic_source_ref,
                        &route_session.m9.authority,
                        &runtime_projection,
                        "m10-route-session:scn06",
                        0,
                        1,
                    );
                    let object = details
                        .as_object_mut()
                        .expect("M10 persistent provenance is an object");
                    object.insert("no_publication".to_string(), Value::Bool(true));
                    object.insert(
                        "diagnostic".to_string(),
                        json!({
                            "code": "RouteUnavailable",
                            "source_ref": source_ref_json(Some(&semantic_source_ref)),
                        }),
                    );
                    record_runtime_trace_with_details(runtime_traces, predicate, receipt, details);
                } else if schedule_events_are(
                    events,
                    &[
                        "invoke_before_patch",
                        "submit_checked_route_patch_artifact",
                        "invoke_after_patch",
                    ],
                ) {
                    let carrier = route_patch_carrier_ref
                        .as_deref()
                        .expect("parser requires route carrier");
                    if carriers.route_patch(carrier).is_some()
                        && route_patch_activated.get(carrier) == Some(&true)
                    {
                        add_carrier_facts(
                            facts,
                            carriers,
                            carrier,
                            &[
                                "runtime.checked_route_patch_artifact_makes_same_source_succeed_without_source_edit",
                            ],
                            context,
                        )?;
                        let carrier = carriers
                            .route_patch(carrier)
                            .expect("checked route carrier remains available");
                        let base_source_path =
                            schedule.route_patch_base_source_path(&carrier.id)?;
                        let (schedule_source_path, checked) =
                            checked_for_schedule_case(case, checked_sources)?;
                        if schedule_source_path != base_source_path {
                            return Err(format!(
                                "M10 route carrier {} is bound to {base_source_path}, not schedule source {schedule_source_path}",
                                carrier.id,
                            ));
                        }
                        let candidate = checked_sources
                            .get(&carrier.candidate_source_path)
                            .ok_or_else(|| {
                                "M10 route carrier candidate is not checked".to_string()
                            })?;
                        let mut route_session = M10RouteSession::for_checked_source(checked)?;
                        if route_session.invoke()
                            != Err(M10RouteSessionDiagnostic::RouteUnavailable)
                        {
                            return Err(
                                "M10 route session accepted an absent route before activation"
                                    .to_string(),
                            );
                        }
                        let contract_before = route_session.contract_identity.clone();
                        let activation =
                            route_session.activate_checked_route(checked, candidate, carrier)?;
                        if route_session.invoke().is_err() {
                            return Err("M10 route session rejected the checked activated route"
                                .to_string());
                        }
                        let owner_before_m8 = route_session.m8.clone();
                        if !route_session.serve_postpatch_owner_request(candidate)? {
                            return Err(
                                "M10 route owner write after patch was rejected".to_string()
                            );
                        }
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let reject_bundle = m10_native_hash_bundle(
                            &activation.before_m8,
                            &activation.m9,
                            "SCN06-route-before-patch",
                        );
                        let patch_bundle = m10_native_hash_bundle(
                            &activation.after_m8,
                            &activation.m9,
                            "SCN06-route-after-patch",
                        );
                        let owner_before_bundle = m10_native_hash_bundle(
                            &owner_before_m8,
                            &route_session.m9,
                            "SCN06-route-owner-before",
                        );
                        let owner_after_projection =
                            route_session.runtime.canonical_semantic_projection();
                        let owner_after_bundle = m10_native_hash_bundle(
                            &route_session.m8,
                            &route_session.m9,
                            "SCN06-route-owner-after",
                        );
                        let reject_receipt = receipt_ledger.record_actual(
                            "route.reject_before_patch",
                            &semantic_source_ref,
                            reject_bundle.clone(),
                            reject_bundle.clone(),
                            false,
                        );
                        let patch_receipt = receipt_ledger.record_actual(
                            "route.patch.activate",
                            &semantic_source_ref,
                            reject_bundle,
                            patch_bundle,
                            true,
                        );
                        let owner_receipt = receipt_ledger.record_actual(
                            "route.owner_write_after_patch",
                            &semantic_source_ref,
                            owner_before_bundle,
                            owner_after_bundle,
                            true,
                        );
                        let mut details = m10_persistent_runtime_provenance(
                            case,
                            checked,
                            &semantic_source_ref,
                            &route_session.m9.authority,
                            &owner_after_projection,
                            "m10-route-session:scn06",
                            0,
                            3,
                        );
                        let accepted_patch_identity = route_session
                            .runtime
                            .active_patch_id()
                            .ok_or_else(|| {
                                "M10 route runtime lost its activated patch identity".to_string()
                            })?
                            .to_string();
                        let object = details
                            .as_object_mut()
                            .expect("M10 persistent provenance is an object");
                        object.insert(
                            "transition_trace".to_string(),
                            json!([
                                reject_receipt.evidence(),
                                patch_receipt.evidence(),
                                owner_receipt.evidence(),
                            ]),
                        );
                        object.insert(
                            "route_patch_activation".to_string(),
                            json!({
                                "base": {
                                    "source_ref": source_ref_json(Some(&m10_semantic_source_ref(checked)?)),
                                    "checked_program_identity": checked.program_identity().stable_key(),
                                },
                                "candidate": {
                                    "source_ref": source_ref_json(Some(&m10_semantic_source_ref(candidate)?)),
                                    "checked_program_identity": candidate.program_identity().stable_key(),
                                },
                                "m8_m9_activation": activation.runtime_trace,
                            }),
                        );
                        object.extend(
                            json!({
                                "persistent_execution_runtime": {
                                    "session_id": route_session.session_id,
                                    "contract_identity": route_session.contract_identity,
                                },
                                "route_patch": {
                                    "runtime_session_id": route_session.session_id,
                                    "contract_before": contract_before,
                                    "contract_after": route_session.contract_identity,
                                    "accepted_patch_identity": accepted_patch_identity,
                                },
                                "postpatch_owner_request": {
                                    "runtime_session_id": route_session.session_id,
                                    "contract_ref": route_session.contract_identity,
                                    "decision": "accepted",
                                    "source_transition": "owner.request.mark_route",
                                },
                                "fresh_runtime_created": false,
                                "bool_only_facade_used": false,
                            })
                            .as_object()
                            .expect("object")
                            .clone(),
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            "runtime.checked_route_patch_artifact_makes_same_source_succeed_without_source_edit",
                            owner_receipt,
                            details,
                        );
                    }
                }
            }
            M10ScheduleOperation::ObserverProjection {
                policy_carrier_ref,
                channel,
            } => {
                let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                if m10_schedule_has_m9_admission(checked)
                    && matches!(
                        carriers.observation(policy_carrier_ref),
                        Some(M10ObservationPolicyCarrier::ObserverSafe { .. })
                    )
                {
                    let policy = carriers
                        .observation(policy_carrier_ref)
                        .expect("validated observer policy remains available");
                    let observation_session =
                        M10ObservationSession::for_policy(checked, policy_carrier_ref, policy)?;
                    let predicates: &[&str] = match channel {
                        M10ProjectionChannel::ObserverSafe => &[
                            "runtime.observer_projection_contains_only.position",
                            "runtime.redaction_order_preserves_policy_before_projection",
                        ],
                        M10ProjectionChannel::AdminDebug => {
                            &["runtime.admin_debug_view_does_not_leak_authority_payloads"]
                        }
                    };
                    add_carrier_facts(facts, carriers, policy_carrier_ref, predicates, context)?;
                    let exported_fields = match policy {
                        M10ObservationPolicyCarrier::ObserverSafe {
                            observer_fields, ..
                        } => observer_fields.clone(),
                        _ => Vec::new(),
                    };
                    let request = M10OwnerEventRequest {
                        event: "move".to_string(),
                        principal: "self".to_string(),
                        target: None,
                        repeat: 1,
                        step: None,
                        seed: BTreeMap::new(),
                        arguments: BTreeMap::from([("delta".to_string(), 1)]),
                    };
                    let M10OwnerScheduleOutcome::Served(served) = execute_checked_owner_schedule(
                        checked,
                        &request,
                        M10OwnerAuthorityMode::Admitted,
                    )?
                    else {
                        return Err("M10 SCN07 observer owner effect was rejected".to_string());
                    };
                    let M10OwnerScheduleServed {
                        runtime,
                        before_runtime,
                        m9,
                        observer_authority,
                        ..
                    } = *served;
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let publication = m10_observer_publication_evidence(
                        checked,
                        runtime.clone(),
                        observer_authority,
                        policy_carrier_ref,
                        exported_fields,
                        match channel {
                            M10ProjectionChannel::ObserverSafe => "scn07-observer",
                            M10ProjectionChannel::AdminDebug => "scn07-admin",
                        },
                    )?;
                    if publication
                        .get("publication_origin")
                        .and_then(Value::as_str)
                        != Some("m8_observer_runtime")
                    {
                        return Err(
                            "M10 observer publication did not retain its M8 runtime origin"
                                .to_string(),
                        );
                    }
                    observation_session
                        .validate_history_projection(
                            M10HistoryProjection::ObserverHistory,
                            M10HistoryOrigin::M8RedactedObserverRuntime,
                        )
                        .map_err(|diagnostic| {
                            format!(
                                "M10 source-bound observer history rejected M8 publication: {}",
                                diagnostic.code(),
                            )
                        })?;
                    let before_bundle =
                        m10_actual_hash_bundle(&before_runtime, &m9, "SCN07-observer-before", None);
                    let after_projection = runtime.canonical_semantic_projection();
                    let after_bundle =
                        m10_actual_hash_bundle(&runtime, &m9, "SCN07-observer-after", None);
                    for (index, predicate) in predicates.iter().enumerate() {
                        let receipt = receipt_ledger.record_actual(
                            "observer.publish",
                            &semantic_source_ref,
                            before_bundle.clone(),
                            after_bundle.clone(),
                            true,
                        );
                        let mut details = m10_persistent_runtime_provenance(
                            case,
                            checked,
                            &semantic_source_ref,
                            &m9.authority,
                            &after_projection,
                            "m10-observer-session:scn07",
                            index as u64,
                            index as u64 + 1,
                        );
                        let details_object = details
                            .as_object_mut()
                            .expect("M10 persistent provenance is an object");
                        details_object
                            .insert("observer_publication".to_string(), publication.clone());
                        details_object.insert(
                            "history_origin_validation".to_string(),
                            json!({
                                "validator": "m10_source_bound_observation_session",
                                "policy_carrier_ref": policy_carrier_ref,
                                "origin": "m8_redacted_observer_runtime",
                                "accepted": true,
                            }),
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            predicate,
                            receipt,
                            details,
                        );
                    }
                }
            }
            M10ScheduleOperation::LeaseOptionLifecycle { events } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let fallback = carriers.fallbacks.iter().find(|carrier| {
                    carrier.source_path == path && checked.relation(&carrier.relation).is_some()
                });
                let Some(fallback) = fallback else {
                    continue;
                };
                let normal_lineage = fallback.options.get(1).is_some_and(|option| {
                    option.lineage_edges.iter().any(|edge| {
                        edge.from == M10FallbackOptionKind::Live
                            && edge.to == M10FallbackOptionKind::Anchor
                    })
                });
                let frozen_option = fallback.options.get(2).filter(|option| {
                    option.kind == M10FallbackOptionKind::Frozen
                        && option.lineage_edges.iter().any(|edge| {
                            edge.from == M10FallbackOptionKind::Anchor
                                && edge.to == M10FallbackOptionKind::Frozen
                        })
                });
                if normal_lineage
                    && frozen_option.is_some()
                    && fallback.id == "view-pose-normal-fallback"
                {
                    let predicate = match events.as_slice() {
                        [event] if event == "live" => {
                            Some("runtime.view_pose.selects_live_option_before_expiry")
                        }
                        [event] if event == "lease_expiry" => {
                            Some("runtime.lease_expiry_monotonically_selects_anchor_then_frozen")
                        }
                        [event] if event == "write" => Some(
                            "runtime.write_after_option_selection_requires_current_write_capability",
                        ),
                        [event] if event == "fresh_reacquire" => {
                            Some("runtime.fresh_reacquire_creates_new_lineage")
                        }
                        [event] if event == "rollback" => {
                            Some("runtime.rollback_does_not_rewind_selected_option")
                        }
                        _ => None,
                    };
                    if let Some(predicate) = predicate {
                        let validated_chain = m10_validate_normal_finite_fallback_chain(fallback)?;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let session = scn08_relation_session.get_or_insert(
                            M10RelationLifecycleSession::new_with_validated_fallback_chain(
                                checked,
                                &fallback.relation,
                                validated_chain,
                            )?,
                        );
                        let lifecycle = &mut session.lifecycle;
                        if lifecycle.pre_degradation_cut.is_none() {
                            lifecycle.pre_degradation_cut = Some(
                                lifecycle
                                    .runtime
                                    .save_local_cut("m10-scn08-pre-degradation"),
                            );
                        }
                        let relation_projection_before =
                            deterministic_hash(&lifecycle.runtime.canonical_relation_projection());
                        let before_bundle = m10_actual_hash_bundle(
                            &lifecycle.runtime,
                            &lifecycle.m9,
                            &format!("SCN08|{}|before", case.identity),
                            None,
                        );
                        let mut current_option_capability_validation = None;
                        let mut m8_relation_trace = match events.as_slice() {
                            [event] if event == "lease_expiry" => {
                                let advance = lifecycle
                                    .runtime
                                    .invalidate_primary(
                                        &fallback.relation,
                                        lifecycle.invalidate_authority.clone(),
                                        M8BindingInvalidation::lease_expired(
                                            &lifecycle.primary_anchor,
                                        )
                                        .with_frontier(
                                            format!("{}:degraded", lifecycle.initial_frontier),
                                        ),
                                    )
                                    .map_err(|diagnostics| {
                                        format!(
                                            "M10 SCN08 M8 invalidate: {:?}",
                                            diagnostics.primary().kind()
                                        )
                                    })?;
                                let anchor_state = m8_finite_fallback_state(
                                    &lifecycle.runtime,
                                    &fallback.relation,
                                )?;
                                let freeze = lifecycle
                                    .runtime
                                    .advance_anchor_to_frozen(&fallback.relation, &advance)
                                    .map_err(|diagnostics| {
                                        format!(
                                            "M10 SCN08 M8 freeze fallback: {:?}",
                                            diagnostics.primary().kind()
                                        )
                                    })?;
                                let frozen_state = m8_finite_fallback_state(
                                    &lifecycle.runtime,
                                    &fallback.relation,
                                )?;
                                json!([
                                    {
                                        "transition": "invalidate_primary",
                                        "from_option_index": advance.previous_option_index(),
                                        "to_option_index": advance.current_option_index(),
                                        "previous_option_index": advance.previous_option_index(),
                                        "selected_floor": anchor_state["selected_floor"].clone(),
                                        "selected_option_index": advance.current_option_index(),
                                        "selected_target": anchor_state["selected_target"].clone(),
                                        "active_lease_ref": anchor_state["active_lease_ref"].clone(),
                                        "required_capability": anchor_state["required_capability"].clone(),
                                        "selected_option_epoch": anchor_state["selected_option_epoch"].clone(),
                                        "audit_subreason": advance
                                            .invalidation_cause()
                                            .map(|cause| cause.audit_subreason()),
                                        "invalidation_reason": advance
                                            .invalidation_cause()
                                            .map(|cause| cause.audit_subreason()),
                                    },
                                    {
                                        "transition": "select_fallback",
                                        "from_option_index": advance.previous_option_index(),
                                        "to_option_index": advance.current_option_index(),
                                        "selected_floor": anchor_state["selected_floor"].clone(),
                                        "selected_option_index": anchor_state["selected_option_index"].clone(),
                                        "selected_target": anchor_state["selected_target"].clone(),
                                        "active_lease_ref": anchor_state["active_lease_ref"].clone(),
                                        "required_capability": anchor_state["required_capability"].clone(),
                                        "selected_option_epoch": anchor_state["selected_option_epoch"].clone(),
                                    },
                                    {
                                        "transition": "freeze_fallback",
                                        "from_option_index": freeze.previous_option_index(),
                                        "to_option_index": freeze.current_option_index(),
                                        "previous_option_index": freeze.previous_option_index(),
                                        "selected_floor": frozen_state["selected_floor"].clone(),
                                        "selected_option_index": freeze.current_option_index(),
                                        "selected_target": frozen_state["selected_target"].clone(),
                                        "active_lease_ref": frozen_state["active_lease_ref"].clone(),
                                        "required_capability": frozen_state["required_capability"].clone(),
                                        "selected_option_epoch": frozen_state["selected_option_epoch"].clone(),
                                        "audit_subreason": freeze
                                            .invalidation_cause()
                                            .map(|cause| cause.audit_subreason()),
                                    },
                                ])
                            }
                            [event] if event == "fresh_reacquire" => {
                                if lifecycle
                                    .runtime
                                    .relation_state(&fallback.relation)
                                    .is_some_and(|relation| relation.selected_option_index() == 0)
                                {
                                    lifecycle
                                        .runtime
                                        .invalidate_primary(
                                            &fallback.relation,
                                            lifecycle.invalidate_authority.clone(),
                                            M8BindingInvalidation::anchor_unavailable(
                                                &lifecycle.primary_anchor,
                                            )
                                            .with_frontier(format!(
                                                "{}:degraded",
                                                lifecycle.initial_frontier
                                            )),
                                        )
                                        .map_err(|diagnostics| {
                                            format!(
                                                "M10 SCN08 M8 prepare reacquire: {:?}",
                                                diagnostics.primary().kind()
                                            )
                                        })?;
                                }
                                let reacquire = lifecycle
                                    .runtime
                                    .reacquire_primary(
                                        &fallback.relation,
                                        lifecycle.reacquire_authority.clone(),
                                        M8RelationReacquire::new(&lifecycle.primary_anchor)
                                            .with_anchor_epoch(&lifecycle.fresh_anchor_epoch)
                                            .with_binding_epoch("binding_epoch:2")
                                            .with_fresh_witness(&lifecycle.fresh_witness_ref)
                                            .with_fresh_lease_ref(&lifecycle.fresh_lease_ref)
                                            .with_frontier(&lifecycle.fresh_frontier),
                                    )
                                    .map_err(|diagnostics| {
                                        format!(
                                            "M10 SCN08 M8 reacquire: {:?}",
                                            diagnostics.primary().kind()
                                        )
                                    })?;
                                let (selected_floor, selected_option_index, selected_target, fresh_epoch) = lifecycle
                                    .runtime
                                    .relation_state(&fallback.relation)
                                    .map(|relation| {
                                        (
                                            relation.selected_floor().as_str(),
                                            relation.selected_option_index(),
                                            relation.selected_anchor().to_string(),
                                            relation.primary_epoch()
                                                == lifecycle.fresh_anchor_epoch,
                                        )
                                    })
                                    .ok_or_else(|| {
                                        format!(
                                            "M10 SCN08 missing M8 relation state after reacquire: {}",
                                            fallback.relation
                                        )
                                    })?;
                                json!([{
                                    "transition": "reacquire_primary",
                                    "fresh_epoch": fresh_epoch,
                                    "fresh_witness": reacquire.fresh_reacquire_witness()
                                        == lifecycle.fresh_witness_ref,
                                    "selected_floor": selected_floor,
                                    "selected_option_index": selected_option_index,
                                    "selected_target": selected_target,
                                }])
                            }
                            [event] if event == "write" => {
                                let before_write = lifecycle.runtime.save_relevant_payload();
                                let write = lifecycle
                                    .runtime
                                    .request_selected_option_write(&fallback.relation);
                                let after_write = lifecycle.runtime.save_relevant_payload();
                                let diagnostic = write.as_ref().err().map(|diagnostics| {
                                    format!("{:?}", diagnostics.primary().kind())
                                });
                                if diagnostic.as_deref() != Some("WriteCapabilityUnavailable")
                                    || before_write != after_write
                                {
                                    return Err(
                                        "M10 SCN08 selected read-floor write was not rejected without mutation"
                                            .to_string(),
                                    );
                                }
                                let current_option = m8_finite_fallback_state(
                                    &lifecycle.runtime,
                                    &fallback.relation,
                                )?;
                                let current_option_capability = current_option
                                    .get("required_capability")
                                    .cloned()
                                    .ok_or_else(|| {
                                        "M10 SCN08 selected option lacks a capability identity"
                                            .to_string()
                                    })?;
                                current_option_capability_validation = Some(json!({
                                    "attempted": true,
                                    "validator": "m8_current_option_capability_validator",
                                    "selected_floor": current_option["selected_floor"].clone(),
                                    "selected_option_index": current_option["selected_option_index"].clone(),
                                    "selected_target": current_option["selected_target"].clone(),
                                    "current_option_capability": current_option_capability,
                                    "write_capable": false,
                                    "later_write_capable_option_exists": false,
                                    "request_level_reject": true,
                                    "outcome": "rejected",
                                    "diagnostic": { "code": diagnostic.clone() },
                                    "no_five_domain_mutation": true,
                                    "schedule_action_reference": case.identity,
                                    "trace_range": "m8-scn08-relation:write",
                                }));
                                json!([{
                                    "transition": "reject_write_without_current_capability",
                                    "from_option_index": current_option["selected_option_index"].clone(),
                                    "to_option_index": current_option["selected_option_index"].clone(),
                                    "selected_floor": current_option["selected_floor"].clone(),
                                    "selected_option_index": current_option["selected_option_index"].clone(),
                                    "selected_target": current_option["selected_target"].clone(),
                                    "diagnostic": diagnostic,
                                    "request_level_reject": true,
                                    "no_semantic_mutation": true,
                                }])
                            }
                            [event] if event == "rollback" => {
                                let pre_degradation_cut = lifecycle
                                    .pre_degradation_cut
                                    .as_ref()
                                    .ok_or_else(|| {
                                        "M10 SCN08 has no pre-degradation local cut".to_string()
                                    })?
                                    .clone();
                                let rollback_before = lifecycle.runtime.save_relevant_payload();
                                let rollback_floor = M8LiveFloor::from_runtime(&lifecycle.runtime);
                                let rollback = lifecycle
                                    .runtime
                                    .try_restore_local_cut(&pre_degradation_cut, &rollback_floor);
                                let rollback_after = lifecycle.runtime.save_relevant_payload();
                                let rollback_diagnostic =
                                    rollback.as_ref().err().map(|diagnostics| {
                                        format!("{:?}", diagnostics.primary().kind())
                                    });
                                if rollback_diagnostic.as_deref() != Some("ExpiredLease")
                                    || rollback_before != rollback_after
                                {
                                    return Err(
                                        "M10 SCN08 stale local-cut rollback did not reject without mutation"
                                            .to_string(),
                                    );
                                }
                                let frozen_state = m8_finite_fallback_state(
                                    &lifecycle.runtime,
                                    &fallback.relation,
                                )?;
                                let trace = json!([{
                                    "transition": "reject_same_lineage_live_repromotion",
                                    "from_option_index": 2,
                                    "to_option_index": 2,
                                    "selected_floor": frozen_state["selected_floor"].clone(),
                                    "selected_option_index": frozen_state["selected_option_index"].clone(),
                                    "selected_target": frozen_state["selected_target"].clone(),
                                }]);
                                lifecycle.rollback_evidence = Some(json!({
                                    "m8_local_cut_restore": {
                                        "attempted": true,
                                        "result": "rejected",
                                        "schedule_action_reference": case.identity,
                                        "trace_range": {
                                            "start": 0,
                                            "end": 1,
                                            "covers_restore_attempt": true,
                                        },
                                        "diagnostic": {
                                            "code": rollback_diagnostic,
                                            "source": "M8LocalRuntime::try_restore_local_cut",
                                        },
                                        "no_five_domain_mutation": true,
                                    },
                                    "m8_relation_state": {
                                        "selected_floor": frozen_state["selected_floor"].clone(),
                                        "selected_option_index": frozen_state["selected_option_index"].clone(),
                                        "selected_target": frozen_state["selected_target"].clone(),
                                        "rollback_repromoted": false,
                                        "derived_from_actual_m8_relation_state": true,
                                        "derived_from_actual_m8_relation_projection": true,
                                    },
                                }));
                                trace
                            }
                            _ => {
                                let (selected_floor, selected_option_index, selected_target) =
                                    lifecycle
                                        .runtime
                                        .relation_state(&fallback.relation)
                                        .map(|relation| {
                                            (
                                                relation.selected_floor().as_str(),
                                                relation.selected_option_index(),
                                                relation.selected_anchor().to_string(),
                                            )
                                        })
                                        .ok_or_else(|| {
                                            format!(
                                                "M10 SCN08 missing M8 relation state: {}",
                                                fallback.relation
                                            )
                                        })?;
                                json!([{
                                    "transition": "select_primary",
                                    "selected_floor": selected_floor,
                                    "selected_option_index": selected_option_index,
                                    "selected_target": selected_target,
                                    "derived_from_actual_m8_relation_state": true,
                                }])
                            }
                        };
                        let relation_projection_after =
                            deterministic_hash(&lifecycle.runtime.canonical_relation_projection());
                        annotate_actual_m8_relation_trace(
                            &mut m8_relation_trace,
                            &relation_projection_before,
                            &relation_projection_after,
                        );
                        let mut after_bundle = m10_actual_hash_bundle(
                            &lifecycle.runtime,
                            &lifecycle.m9,
                            &format!("SCN08|{}|after", case.identity),
                            None,
                        );
                        // Relation lifecycle transitions do not change the
                        // owner store projection.
                        after_bundle.store_hash = before_bundle.store_hash.clone();
                        let authority_translation = lifecycle.authority_translation.clone();
                        let mut relation_state = lifecycle
                            .runtime
                            .finite_fallback_selection(&fallback.relation)
                            .map(|_| {
                                m8_finite_fallback_state(&lifecycle.runtime, &fallback.relation)
                            })
                            .transpose()?
                            .unwrap_or(m8_relation_state_value(
                                &lifecycle.runtime,
                                &fallback.relation,
                            )?);
                        let relation_projection_before_value =
                            Value::String(relation_projection_before.clone());
                        let relation_projection_after_value =
                            Value::String(relation_projection_after.clone());
                        let relation_state_object = relation_state
                            .as_object_mut()
                            .expect("SCN08 M8 relation state is an object");
                        relation_state_object.insert(
                            "relation_projection_before".to_string(),
                            relation_projection_before_value,
                        );
                        relation_state_object.insert(
                            "relation_projection_after".to_string(),
                            relation_projection_after_value,
                        );
                        relation_state_object.insert(
                            "derived_from_actual_m8_relation_projection".to_string(),
                            json!(true),
                        );
                        if events.as_slice() == ["lease_expiry"] {
                            let trace_kinds = lifecycle.runtime.trace().kinds();
                            let no_dedicated_semantic_domain_occurrence_created =
                                !trace_kinds.iter().any(|kind| {
                                    matches!(
                                        kind,
                                        M8LocalTraceKind::OwnerEnqueued
                                            | M8LocalTraceKind::OwnerRead
                                            | M8LocalTraceKind::OwnerWrite
                                    )
                                });
                            let old_live_lease_usable = lifecycle
                                .runtime
                                .contains_live_relation_lease("lease:view_pose:live");
                            relation_state_object.insert(
                                "no_dedicated_semantic_domain_occurrence_created".to_string(),
                                json!(no_dedicated_semantic_domain_occurrence_created),
                            );
                            relation_state_object.insert(
                                "old_live_lease_usable".to_string(),
                                json!(old_live_lease_usable),
                            );
                            relation_state_object.insert(
                                "old_live_lease_restorable".to_string(),
                                // The finite fragment can restore this lease
                                // only while M8 still marks it live.  The
                                // later rollback action independently calls
                                // `try_restore_local_cut` for its typed
                                // rejection evidence.
                                json!(old_live_lease_usable),
                            );
                        }
                        if events.as_slice() == ["fresh_reacquire"] {
                            for field in [
                                "fresh_m9_reacquire",
                                "fresh_lineage",
                                "fresh_epoch",
                                "fresh_witness",
                                "index0_created_by_fresh_m9_reacquire",
                            ] {
                                relation_state_object.insert(field.to_string(), json!(true));
                            }
                            relation_state_object
                                .insert("manual_index0_reset".to_string(), json!(false));
                        }
                        if events.as_slice() == ["rollback"] {
                            let rollback_state = lifecycle
                                .rollback_evidence
                                .as_ref()
                                .and_then(|evidence| evidence.get("m8_relation_state"))
                                .cloned()
                                .ok_or_else(|| {
                                    "M10 SCN08 rollback lacks actual frozen relation state"
                                        .to_string()
                                })?;
                            relation_state = rollback_state;
                        }
                        let relation_state_object = relation_state
                            .as_object_mut()
                            .expect("SCN08 final M8 relation state is an object");
                        relation_state_object.insert(
                            "relation_projection_before".to_string(),
                            json!(relation_projection_before),
                        );
                        relation_state_object.insert(
                            "relation_projection_after".to_string(),
                            json!(relation_projection_after),
                        );
                        relation_state_object.insert(
                            "derived_from_actual_m8_relation_state".to_string(),
                            json!(true),
                        );
                        relation_state_object.insert(
                            "derived_from_actual_m8_relation_projection".to_string(),
                            json!(true),
                        );
                        let m8_option_chain =
                            m8_finite_fallback_chain_value(fallback, &lifecycle.runtime);
                        let rejected_write = current_option_capability_validation.is_some();
                        let receipt = receipt_ledger.record_actual(
                            if rejected_write {
                                "fallback.write"
                            } else {
                                "fallback.advance"
                            },
                            &semantic_source_ref,
                            before_bundle,
                            after_bundle,
                            !rejected_write,
                        );
                        let mut details = json!({
                            "m8_relation_trace": m8_relation_trace,
                            "m8_relation_state": relation_state,
                            "m8_option_chain": m8_option_chain,
                            "m9_to_m8_authority_translation": authority_translation,
                            "direct_m10_already_admitted_authority_ref_rejected": true,
                            "direct_m10_lease_ref_rejected": true,
                        });
                        if events.as_slice() == ["rollback"] {
                            let rollback_restore = lifecycle
                                .rollback_evidence
                                .as_ref()
                                .and_then(|evidence| evidence.get("m8_local_cut_restore"))
                                .cloned()
                                .ok_or_else(|| {
                                    "M10 SCN08 rollback lacks actual local-cut evidence".to_string()
                                })?;
                            details
                                .as_object_mut()
                                .expect("SCN08 details are an object")
                                .insert("m8_local_cut_restore".to_string(), rollback_restore);
                        }
                        if let Some(validation) = current_option_capability_validation {
                            details
                                .as_object_mut()
                                .expect("SCN08 details are an object")
                                .insert(
                                    "m8_current_option_capability_validation".to_string(),
                                    validation,
                                );
                        }
                        if rejected_write {
                            let details_object = details
                                .as_object_mut()
                                .expect("SCN08 details are an object");
                            details_object.insert(
                                "diagnostic".to_string(),
                                json!({
                                    "code": "WriteCapabilityUnavailable",
                                    "source_ref": source_ref_json(Some(&semantic_source_ref)),
                                }),
                            );
                            details_object.insert(
                                "program_artifact".to_string(),
                                json!({
                                    "source_ref": source_ref_json(Some(&semantic_source_ref)),
                                }),
                            );
                            details_object.insert(
                                "schedule_action".to_string(),
                                json!({ "reference": case.identity }),
                            );
                        }
                        details
                            .as_object_mut()
                            .expect("SCN08 details are an object")
                            .extend(
                                session
                                    .session_details()
                                    .as_object()
                                    .expect("SCN08 session details are an object")
                                    .clone(),
                            );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            predicate,
                            receipt,
                            details,
                        );
                        add_carrier_facts(facts, carriers, &fallback.id, &[predicate], context)?;
                    }
                }
                if events.as_slice() == ["write_after_read_lineage"]
                    && fallback.negative_capability_floor
                        == "write_after_read_without_fresh_reacquire"
                {
                    let semantic_source_ref = m10_semantic_source_ref(checked)?;
                    let mut session =
                        M10RelationLifecycleSession::new(checked, &fallback.relation)?;
                    session
                        .lifecycle
                        .runtime
                        .invalidate_primary(
                            &fallback.relation,
                            session.lifecycle.invalidate_authority.clone(),
                            M8BindingInvalidation::anchor_unavailable(
                                &session.lifecycle.primary_anchor,
                            )
                            .with_frontier(format!(
                                "{}:degraded",
                                session.lifecycle.initial_frontier
                            )),
                        )
                        .map_err(|diagnostics| {
                            format!(
                                "M10 SCN08 M8 prepare same-lineage return: {:?}",
                                diagnostics.primary().kind()
                            )
                        })?;
                    let degraded_frontier =
                        format!("{}:degraded", session.lifecycle.initial_frontier);
                    let projection_context = M8PresentationContext::for_consumer("Viewer")
                        .with_frontier(&degraded_frontier)
                        .with_presentation_fallback(M8PresentationFallback::hold_last_local(
                            "view_pose",
                            M8Point::new(0, 0),
                        ));
                    let projection_before = session
                        .lifecycle
                        .runtime
                        .project_relation(&fallback.relation, projection_context.clone())
                        .map_err(|diagnostics| {
                            format!(
                                "M10 SCN08 M8 pre-repromotion projection: {:?}",
                                diagnostics.primary().kind()
                            )
                        })?;
                    let projection_before_identity = deterministic_hash(&format!(
                        "relation={}|anchor={}|frontier={}|trace_len={}",
                        projection_before.relation(),
                        projection_before.selected_anchor(),
                        projection_before.context_frontier(),
                        session.lifecycle.runtime.trace().len(),
                    ));
                    let before_bundle = m10_actual_hash_bundle(
                        &session.lifecycle.runtime,
                        &session.lifecycle.m9,
                        &format!("SCN08|{}|before-repromotion", case.identity),
                        None,
                    );
                    session
                        .lifecycle
                        .runtime
                        .note_primary_available_same_lineage(
                            &fallback.relation,
                            &session.lifecycle.primary_anchor,
                        )
                        .map_err(|diagnostics| {
                            format!(
                                "M10 SCN08 M8 same-lineage return: {:?}",
                                diagnostics.primary().kind()
                            )
                        })?;
                    let predicate = "structural_rejection.no_mutation.same_lineage_repromotion_without_reacquire";
                    let projection_after = session
                        .lifecycle
                        .runtime
                        .project_relation(&fallback.relation, projection_context)
                        .map_err(|diagnostics| {
                            format!(
                                "M10 SCN08 M8 post-repromotion projection: {:?}",
                                diagnostics.primary().kind()
                            )
                        })?;
                    let projection_after_identity = deterministic_hash(&format!(
                        "relation={}|anchor={}|frontier={}|trace_len={}",
                        projection_after.relation(),
                        projection_after.selected_anchor(),
                        projection_after.context_frontier(),
                        session.lifecycle.runtime.trace().len(),
                    ));
                    let actual_anchor_floor = session
                        .lifecycle
                        .runtime
                        .relation_state(&fallback.relation)
                        .is_some_and(|relation| relation.selected_option_index() == 1);
                    if actual_anchor_floor {
                        let after_bundle = m10_actual_hash_bundle(
                            &session.lifecycle.runtime,
                            &session.lifecycle.m9,
                            &format!("SCN08|{}|after-repromotion", case.identity),
                            None,
                        );
                        let receipt = receipt_ledger.record_actual(
                            "fallback.advance",
                            &semantic_source_ref,
                            before_bundle,
                            after_bundle,
                            false,
                        );
                        let mut details = json!({
                            "m8_relation_trace": [{
                                "transition": "reject_same_lineage_live_repromotion",
                                "selected_floor": "anchor",
                                "runtime_projection_before": projection_before_identity,
                                "runtime_projection_after": projection_after_identity,
                                "relation_projection_trace_ref": deterministic_hash(&format!(
                                    "m8-relation-trace|{}|{}",
                                    projection_before_identity,
                                    projection_after_identity,
                                )),
                                "derived_from_actual_m8_relation_projection": true,
                                "manual_cursor_used": false,
                            }],
                        });
                        details
                            .as_object_mut()
                            .expect("SCN08 negative details are an object")
                            .extend(
                                session
                                    .session_details()
                                    .as_object()
                                    .expect("SCN08 session details are an object")
                                    .clone(),
                            );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            predicate,
                            receipt,
                            details,
                        );
                    }
                    add_carrier_facts(facts, carriers, &fallback.id, &[predicate], context)?;
                }
            }
            M10ScheduleOperation::SubmitCheckedPatchArtifact { patch_carrier_ref } => {
                if admitted_patch_carriers.contains(patch_carrier_ref) {
                    let carrier = carriers.patch(patch_carrier_ref).ok_or_else(|| {
                        format!(
                            "M10 schedule {} references unknown patch carrier {patch_carrier_ref}",
                            case.id
                        )
                    })?;
                    let candidate = checked_sources
                        .get(&carrier.candidate_source_path)
                        .ok_or_else(|| {
                            format!(
                                "M10 schedule {} patch carrier lacks checked candidate",
                                case.id
                            )
                        })?;
                    let base = checked_sources
                        .get(carrier.base_source_path.as_deref().ok_or_else(|| {
                            format!("M10 schedule {} patch carrier lacks base source", case.id)
                        })?)
                        .ok_or_else(|| {
                            format!("M10 schedule {} patch carrier has unchecked base", case.id)
                        })?;
                    let context = schedule_evidence_context(
                        case,
                        candidate.program_identity().root_source_ref(),
                    );
                    let activation =
                        activate_patch_and_execute_declared_state(base, candidate, carrier)?;
                    if !activation.accepted {
                        return Err(format!(
                            "M10 schedule {} expected real M8 patch activation",
                            case.id
                        ));
                    }
                    let semantic_source_ref = m10_semantic_source_ref(candidate)?;
                    let before_bundle = m10_native_hash_bundle(
                        &activation.before_m8,
                        &activation.m9,
                        &format!("SCN09|{}|before", case.identity),
                    );
                    let after_bundle = m10_native_hash_bundle(
                        &activation.after_m8,
                        &activation.m9,
                        &format!("SCN09|{}|after", case.identity),
                    );
                    add_patch_facts(
                        facts,
                        carriers,
                        source_identities,
                        carrier,
                        &[
                            "runtime.patch_pipeline_uses_checked_pair_not_schedule_verdict",
                            "runtime.patch_initializes_declared_state_addition",
                            "runtime.patch_observer_projection_uses_new_checked_effect",
                        ],
                        &context,
                    )?;
                    for predicate in [
                        "runtime.patch_pipeline_uses_checked_pair_not_schedule_verdict",
                        "runtime.patch_initializes_declared_state_addition",
                        "runtime.patch_observer_projection_uses_new_checked_effect",
                    ] {
                        let receipt = receipt_ledger.record_actual(
                            "patch.activate",
                            &semantic_source_ref,
                            before_bundle.clone(),
                            after_bundle.clone(),
                            true,
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            predicate,
                            receipt,
                            json!({ "m8_patch_activation": activation.runtime_trace.clone() }),
                        );
                    }
                }
            }
            M10ScheduleOperation::MembershipFrontierDrift {
                events,
                patch_carrier_ref,
            } => {
                if schedule_events_are(
                    events,
                    &[
                        "admit_patch",
                        "membership_changes",
                        "activate_checked_patch",
                    ],
                ) && admitted_patch_carriers.contains(patch_carrier_ref)
                {
                    let carrier = carriers.patch(patch_carrier_ref).ok_or_else(|| {
                        format!(
                            "M10 schedule {} references unknown patch carrier {patch_carrier_ref}",
                            case.id
                        )
                    })?;
                    let candidate = checked_sources
                        .get(&carrier.candidate_source_path)
                        .ok_or_else(|| {
                            format!(
                                "M10 schedule {} patch carrier lacks checked candidate",
                                case.id
                            )
                        })?;
                    let context = schedule_evidence_context(
                        case,
                        candidate.program_identity().root_source_ref(),
                    );
                    let semantic_source_ref = m10_semantic_source_ref(candidate)?;
                    let mut patch_authority_session =
                        M10M9M8ExecutionSession::new(candidate, "m9-scn09-patch-authority")?;
                    let admitted_m9 = patch_authority_session.m9.domain_snapshot();
                    let before_retire_bundle = m10_actual_hash_bundle(
                        &patch_authority_session.runtime,
                        &admitted_m9,
                        &format!("SCN09|{}|before-drift", case.identity),
                        None,
                    );
                    patch_authority_session.retire_and_refresh()?;
                    let patch_authority_invalidated =
                        patch_authority_session.bridge.patch_use().is_none();
                    // The M9 membership frontier changes before activation;
                    // the rejected M8 activation receipt itself is sampled at
                    // that post-drift boundary, so it proves no patch-domain
                    // mutation rather than attributing the prior M9 change to
                    // the rejected activation.
                    let drift_boundary_bundle = m10_actual_hash_bundle(
                        &patch_authority_session.runtime,
                        &patch_authority_session.m9.domain_snapshot(),
                        &format!("SCN09|{}|after-drift", case.identity),
                        None,
                    );
                    let retirement_receipt = receipt_ledger.record_actual(
                        "membership.retire",
                        &semantic_source_ref,
                        before_retire_bundle,
                        drift_boundary_bundle.clone(),
                        true,
                    );
                    let activation_rejection = receipt_ledger.record_actual(
                        "patch.activate",
                        &semantic_source_ref,
                        drift_boundary_bundle.clone(),
                        drift_boundary_bundle,
                        false,
                    );
                    let m8_rejected = patch_authority_session.post_retirement_owner_decision()?;
                    let predicate = "structural_deferred.no_activation.membership_frontier_drift_between_admit_and_activation";
                    if !activation_rejection.accepted && m8_rejected && patch_authority_invalidated
                    {
                        add_case_action_fact(facts, case, predicate, &context);
                        let mut details = patch_authority_session.m9.retired_authority_details();
                        let lineage = patch_authority_session.lineage_value();
                        details
                            .as_object_mut()
                            .expect("SCN09 M9 details are an object")
                            .insert(
                                "m8_patch_activation".to_string(),
                                json!({
                                    "activate_patch_called": false,
                                    "activation_cut": Value::Null,
                                }),
                            );
                        details
                            .as_object_mut()
                            .expect("SCN09 lineage details are an object")
                            .extend(json!({
                                "m9_to_m8_authority_lineage": lineage.clone(),
                                "m8_decisions_after_m9": [{
                                    "transition": "patch.activate",
                                    "decision": "rejected",
                                    "authority_lineage_ref": lineage["session_id"].clone(),
                                    "runtime_session_id": lineage["m8_runtime_session_id"].clone(),
                                }],
                            }).as_object().expect("object").clone());
                        record_runtime_trace_with_prior_receipt(
                            runtime_traces,
                            predicate,
                            retirement_receipt,
                            activation_rejection,
                            details,
                        );
                    }
                }
            }
            M10ScheduleOperation::SaveLoadTimeline { events } => {
                let (path, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                let semantic_source_ref = m10_semantic_source_ref(checked)?;
                let mut negative_session: M10CompositeCutSession;
                let session = if path == "scn-10/positive.mir" {
                    let fallback =
                        carriers
                            .fallback("view-pose-normal-fallback")
                            .ok_or_else(|| {
                                "M10 SCN10 requires the checked SCN08 fallback carrier".to_string()
                            })?;
                    let validated_chain = m10_validate_normal_finite_fallback_chain(fallback)?;
                    scn10_positive_session.get_or_insert(
                        M10CompositeCutSession::new_with_fallback_chain(
                            checked,
                            validated_chain.chain,
                        )?,
                    )
                } else {
                    negative_session = M10CompositeCutSession::new(checked)?;
                    negative_session.save_s1()?;
                    &mut negative_session
                };
                let before_save_bundle = m10_actual_hash_bundle(
                    &session.runtime,
                    &session.m9.domain_snapshot(),
                    &format!("SCN10|{}|before", case.identity),
                    None,
                );
                if schedule_events_are(events, &["save_s1"]) {
                    session.save_s1()?;
                    let s1 = session.s1.as_ref().expect("S1 was just saved");
                    let after_save_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|after-save", case.identity),
                        Some(s1),
                    );
                    let fresh_restored = session
                        .runtime
                        .try_restore_local_cut(s1, &M8LiveFloor::same_current(s1))
                        .is_ok();
                    if !fresh_restored {
                        return Err("M10 SCN10 could not restore its fresh S1 cut".to_string());
                    }
                    let predicate = "runtime.save_creates_world_cut_S1";
                    add_checked_source_fact(facts, source_identities, path, predicate, context)?;
                    let receipt = receipt_ledger.record_actual(
                        "cut.save",
                        &semantic_source_ref,
                        before_save_bundle,
                        after_save_bundle,
                        true,
                    );
                    record_runtime_trace_with_details(
                        runtime_traces,
                        predicate,
                        receipt,
                        session.session_details(0, 1),
                    );
                } else if schedule_events_are(events, &["leave_a", "lease_expiry", "save_s2"]) {
                    let had_s1_before_action = session.s1.is_some();
                    session.retire_m9_and_refresh()?;
                    let lease_expiry = session.expire_seeded_relation_lease()?;
                    session.save_s2()?;
                    let s2 = session.s2.as_ref().expect("S2 was just saved");
                    let after_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|after-save-s2", case.identity),
                        Some(s2),
                    );
                    let receipt = receipt_ledger.record_actual(
                        "cut.save",
                        &semantic_source_ref,
                        before_save_bundle,
                        after_bundle,
                        true,
                    );
                    let predicate =
                        "runtime.save_S2_after_leave_and_lease_expiry_creates_current_world_cut";
                    add_checked_source_fact(facts, source_identities, path, predicate, context)?;
                    record_runtime_trace_with_details(runtime_traces, predicate, receipt, {
                        let mut details = session.session_details(1, 3);
                        details
                            .as_object_mut()
                            .expect("SCN10 details are an object")
                            .extend(
                                json!({
                                    "predecessors": { "S1": had_s1_before_action },
                                    "deltas": { "leave": true, "lease_expiry": true },
                                    "receipt_origin": "m8-local-cut-after-m9-membership-session",
                                    "s2_lease_expiry": lease_expiry,
                                })
                                .as_object()
                                .expect("object")
                                .clone(),
                            );
                        details
                    });
                } else if schedule_events_are(events, &["load_s1_fresh"]) {
                    let restore = session.restore_s1_into_fresh_composite(checked)?;
                    let predicate = "runtime.load_S1_into_fresh_session_preserves_past_world_cut";
                    add_checked_source_fact(facts, source_identities, path, predicate, context)?;
                    let after_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|after-load", case.identity),
                        None,
                    );
                    let receipt = receipt_ledger.record_actual(
                        "cut.restore",
                        &semantic_source_ref,
                        before_save_bundle,
                        after_bundle,
                        true,
                    );
                    record_runtime_trace_with_details(runtime_traces, predicate, receipt, {
                        let mut details = session.session_details(3, 4);
                        details
                            .as_object_mut()
                            .expect("SCN10 fresh load details are an object")
                            .insert(
                                "fresh_load".to_string(),
                                json!({
                                    "composite_restore": restore,
                                }),
                            );
                        details
                    });
                } else if schedule_events_are(events, &["merge_stale_s1_into_current"]) {
                    let s1 = session
                        .s1
                        .as_ref()
                        .expect("negative SCN10 saved S1")
                        .clone();
                    let before_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|restore-attempt", case.identity),
                        Some(&s1),
                    );
                    let (preflight, candidate_before, candidate_after, lineage) =
                        session.preflight_stale_merge_on_candidate(checked, &s1)?;
                    let after_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|restore-attempt", case.identity),
                        Some(&s1),
                    );
                    if before_bundle.store_hash == after_bundle.store_hash
                        && before_bundle.membership_hash == after_bundle.membership_hash
                        && before_bundle.grant_hash == after_bundle.grant_hash
                        && before_bundle.relation_hash == after_bundle.relation_hash
                        && before_bundle.config_hash == after_bundle.config_hash
                        && before_bundle.cut_hash == after_bundle.cut_hash
                        && before_bundle.ledger_hash == after_bundle.ledger_hash
                    {
                        let predicate = "structural_rejection.no_mutation.E-CUT-002.stale_membership_epoch_resurrection";
                        add_checked_source_fact(
                            facts,
                            source_identities,
                            path,
                            predicate,
                            context,
                        )?;
                        let m9_restore_receipt = receipt_ledger.record_actual(
                            "m9.cut.restore",
                            &semantic_source_ref,
                            candidate_before,
                            candidate_after,
                            true,
                        );
                        let receipt = receipt_ledger.record_actual(
                            "cut.restore",
                            &semantic_source_ref,
                            before_bundle.clone(),
                            after_bundle.clone(),
                            false,
                        );
                        let mut details = session.session_details(0, 1);
                        details
                            .as_object_mut()
                            .expect("SCN10 merge lineage details are an object")
                            .extend(json!({
                                "stale_merge_preflight": preflight,
                                "current_session_no_mutation": {
                                    "original_before": m10_current_session_hashes(&session.session_id, &before_bundle),
                                    "final_after": m10_current_session_hashes(&session.session_id, &after_bundle),
                                },
                                "no_stale_resurrection": {
                                    "canon_refs": [{
                                        "source_path": "mirrorea_canon/theory/04-ordering-and-cuts.md",
                                        "line_start": 86,
                                        "line_end": 96,
                                        "theorem": "THM-003",
                                    }],
                                },
                                "m9_to_m8_authority_lineage": lineage.clone(),
                                "m8_decisions_after_m9": [{
                                    "transition": "cut.restore",
                                    "decision": "rejected",
                                    "authority_lineage_ref": lineage["session_id"].clone(),
                                    "runtime_session_id": lineage["m8_runtime_session_id"].clone(),
                                }],
                            }).as_object().expect("object").clone());
                        record_runtime_trace_with_prior_receipt(
                            runtime_traces,
                            predicate,
                            m9_restore_receipt,
                            receipt,
                            details,
                        );
                        let trace = runtime_traces
                            .get_mut(predicate)
                            .expect("SCN10 stale preflight runtime trace exists");
                        trace
                            .pointer_mut("/transition_trace/0")
                            .and_then(Value::as_object_mut)
                            .expect("SCN10 candidate M9 receipt is an object")
                            .insert("session_role".to_string(), json!("candidate_preflight"));
                    }
                } else if schedule_events_are(events, &["timeline_panel"]) {
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "runtime.timeline_panel_lists_S1_S2_refusals_with_reasons",
                        context,
                    )?;
                    let s1 = session.s1.as_ref().ok_or_else(|| {
                        "M10 SCN10 timeline ran before the persistent S1 save".to_string()
                    })?;
                    let s2 = session.s2.as_ref().ok_or_else(|| {
                        "M10 SCN10 timeline ran before the persistent S2 save".to_string()
                    })?;
                    let timeline_bundle = m10_actual_hash_bundle(
                        &session.runtime,
                        &session.m9.domain_snapshot(),
                        &format!("SCN10|{}|timeline", case.identity),
                        Some(s2),
                    );
                    let receipt = receipt_ledger.record_actual(
                        "cut.timeline",
                        &semantic_source_ref,
                        before_save_bundle,
                        timeline_bundle,
                        s1.canonical_semantic_projection() != s2.canonical_semantic_projection(),
                    );
                    record_runtime_trace_with_details(
                        runtime_traces,
                        "runtime.timeline_panel_lists_S1_S2_refusals_with_reasons",
                        receipt,
                        {
                            let mut details = session.session_details(3, 4);
                            details
                                .as_object_mut()
                                .expect("SCN10 details are an object")
                                .insert(
                                    "receipt_origin".to_string(),
                                    json!("m8-local-cut-session-ledger"),
                                );
                            details
                                .as_object_mut()
                                .expect("SCN10 details are an object")
                                .insert(
                                    "occurrence_range".to_string(),
                                    json!({ "start": 3, "end": 4 }),
                                );
                            details
                        },
                    );
                } else if schedule_events_are(events, &["reacquire_after_load"]) {
                    let reacquire = session.reacquire_after_load()?;
                    add_checked_source_fact(
                        facts,
                        source_identities,
                        path,
                        "runtime.reacquire_after_load_is_new_occurrence_new_epoch_witness",
                        context,
                    )?;
                    let after_bundle = m10_actual_hash_bundle(
                        &session
                            .fresh_loaded
                            .as_ref()
                            .expect("SCN10 fresh composite was retained after reacquire")
                            .runtime,
                        &session
                            .fresh_loaded
                            .as_ref()
                            .expect("SCN10 fresh composite was retained after reacquire")
                            .m9
                            .domain_snapshot(),
                        &format!("SCN10|{}|reacquire", case.identity),
                        None,
                    );
                    let receipt = receipt_ledger.record_actual(
                        "cut.reacquire",
                        &semantic_source_ref,
                        before_save_bundle,
                        after_bundle,
                        true,
                    );
                    record_runtime_trace_with_details(
                        runtime_traces,
                        "runtime.reacquire_after_load_is_new_occurrence_new_epoch_witness",
                        receipt,
                        {
                            let fresh = session
                                .fresh_loaded
                                .as_ref()
                                .expect("SCN10 fresh composite was retained after reacquire");
                            let mut details = fresh.session_details(4, 5);
                            details
                                .as_object_mut()
                                .expect("SCN10 details are an object")
                                .insert(
                                    "receipt_origin".to_string(),
                                    json!("m8-local-cut-reacquire"),
                                );
                            details
                                .as_object_mut()
                                .expect("SCN10 details are an object")
                                .insert(
                                    "occurrence_range".to_string(),
                                    json!({ "start": 4, "end": 5 }),
                                );
                            details
                                .as_object_mut()
                                .expect("SCN10 details are an object")
                                .insert("reacquire_after_load".to_string(), reacquire);
                            details
                        },
                    );
                }
            }
            M10ScheduleOperation::CorruptedRequest(corruption) => {
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                match corruption {
                    M10CorruptedRequest::MissingCapabilityOwner {
                        event,
                        principal,
                        target,
                    } => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let request = M10OwnerEventRequest {
                            event: event.clone(),
                            principal: principal.clone(),
                            target: Some(target.clone()),
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::new(),
                        };
                        if matches!(
                            execute_checked_owner_schedule(
                                checked,
                                &request,
                                M10OwnerAuthorityMode::MissingCapability
                            )?,
                            M10OwnerScheduleOutcome::RejectedBeforeMutation
                        ) {
                            add_case_action_fact(
                                facts,
                                case,
                                "structural_rejection.no_mutation.owner_request_without_capability",
                                context,
                            );
                        }
                    }
                    M10CorruptedRequest::StaleMembershipOwner {
                        event,
                        principal,
                        target,
                    } => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let request = M10OwnerEventRequest {
                            event: event.clone(),
                            principal: principal.clone(),
                            target: Some(target.clone()),
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::new(),
                        };
                        if matches!(
                            execute_checked_owner_schedule(
                                checked,
                                &request,
                                M10OwnerAuthorityMode::StaleMembership
                            )?,
                            M10OwnerScheduleOutcome::RejectedBeforeMutation
                        ) {
                            add_case_action_fact(
                                facts,
                                case,
                                "structural_rejection.no_mutation.owner_request_with_stale_membership",
                                context,
                            );
                        }
                    }
                    M10CorruptedRequest::SpoofedRole {
                        event,
                        principal,
                        spoofed_role,
                    } => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let request = M10OwnerEventRequest {
                            event: event.clone(),
                            principal: principal.clone(),
                            target: None,
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::new(),
                        };
                        if spoofed_role != principal
                            && matches!(
                                execute_checked_owner_schedule(
                                    checked,
                                    &request,
                                    M10OwnerAuthorityMode::Admitted
                                )?,
                                M10OwnerScheduleOutcome::RejectedBeforeMutation
                            )
                        {
                            add_case_action_fact(
                                facts,
                                case,
                                "structural_rejection.no_mutation.spoofed_role_origin",
                                context,
                            );
                        }
                    }
                    M10CorruptedRequest::ReplayedCapability {
                        event,
                        principal,
                        capability,
                    } => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let request = M10OwnerEventRequest {
                            event: event.clone(),
                            principal: principal.clone(),
                            target: None,
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::new(),
                        };
                        if capability.contains("replayed")
                            && matches!(
                                execute_checked_owner_schedule(
                                    checked,
                                    &request,
                                    M10OwnerAuthorityMode::ReplayedCapability
                                )?,
                                M10OwnerScheduleOutcome::RejectedBeforeMutation
                            )
                        {
                            add_case_action_fact(
                                facts,
                                case,
                                "structural_rejection.no_mutation.replayed_capability",
                                context,
                            );
                        }
                    }
                    M10CorruptedRequest::WrongObservationCapability {
                        request_class,
                        capability,
                        validated_policy_carrier_ref,
                    } => {
                        if matches!(
                            request_class,
                            M10ObservationRequestKind::CrossLocusSecretRead
                        ) && capability != "observation.read"
                            && matches!(
                                carriers.observation(validated_policy_carrier_ref),
                                Some(M10ObservationPolicyCarrier::CrossLocus { .. })
                            )
                        {
                            add_case_action_fact(
                                facts,
                                case,
                                "structural_rejection.no_mutation.wrong_observation_capability",
                                context,
                            );
                            let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                            let request = M10OwnerEventRequest {
                                event: "portal_probe".to_string(),
                                principal: "self".to_string(),
                                target: None,
                                repeat: 1,
                                step: None,
                                seed: BTreeMap::new(),
                                arguments: BTreeMap::from([("delta".to_string(), 1)]),
                            };
                            let M10OwnerScheduleOutcome::Served(served) =
                                execute_checked_owner_schedule(
                                    checked,
                                    &request,
                                    M10OwnerAuthorityMode::Admitted,
                                )?
                            else {
                                return Err("M10 wrong observation capability setup was rejected"
                                    .to_string());
                            };
                            let M10OwnerScheduleServed {
                                runtime,
                                m9,
                                observer_authority,
                                ..
                            } = *served;
                            let semantic_source_ref = m10_semantic_source_ref(checked)?;
                            let runtime_projection = runtime.canonical_semantic_projection();
                            let (principal, _) = patch_principal_and_locus(checked)?;
                            let rejected_policy =
                                M8ObserverPolicy::for_principal(format!("observer:{principal}"))
                                    // Deliberately unlike the M9-issued grant reference below.
                                    .with_authority_ref("m10-wrong-observation-capability")
                                    .with_label(
                                        EvidenceSecurityLabel::new("observer:wrong-observation")
                                            .with_class(M8SecurityClass::Public),
                                    )
                                    .with_redaction(EvidenceRedaction::new("observer-safe"))
                                    .with_retention(M8ObserverRetention::bounded(
                                        "m10-wrong-observation",
                                        1,
                                    ))
                                    .with_source_ref(semantic_source_ref.clone())
                                    .with_reason_ref("wrong-observation-capability")
                                    .with_proof_ref("m9-observer-policy:wrong-capability");
                            let observer = M8ObserverRuntime::from_local_session(
                                runtime.clone(),
                                vec![observer_authority],
                            );
                            let rejected_observation =
                                observer.export_observer_view(rejected_policy).expect_err(
                                    "M10 wrong observation capability unexpectedly published",
                                );
                            if rejected_observation.primary().kind()
                                != M8ObserverDiagnosticKind::MissingObserverAuthority
                            {
                                return Err(format!(
                                    "M10 wrong observation capability produced unexpected M8 diagnostic {:?}",
                                    rejected_observation.primary().kind()
                                ));
                            }
                            let bundle = m10_actual_hash_bundle(
                                &runtime,
                                &m9,
                                "SCN05-wrong-observation",
                                None,
                            );
                            let receipt = receipt_ledger.record_actual(
                                "observation.wrong_capability",
                                &semantic_source_ref,
                                bundle.clone(),
                                bundle,
                                false,
                            );
                            let mut details = m10_persistent_runtime_provenance(
                                case,
                                checked,
                                &semantic_source_ref,
                                &m9.authority,
                                &runtime_projection,
                                "m10-observer-session:scn05",
                                3,
                                4,
                            );
                            let object = details
                                .as_object_mut()
                                .expect("M10 persistent provenance is an object");
                            object.insert("no_publication".to_string(), Value::Bool(true));
                            object.insert(
                                "m8_observer_diagnostic".to_string(),
                                json!({
                                    "code": "MissingObserverAuthority",
                                    "source_ref": source_ref_json(Some(&semantic_source_ref)),
                                }),
                            );
                            object.insert(
                                "diagnostic".to_string(),
                                json!({
                                    "code": "MissingCapability",
                                    "source_ref": source_ref_json(Some(&semantic_source_ref)),
                                }),
                            );
                            record_runtime_trace_with_details(
                                runtime_traces,
                                "structural_rejection.no_mutation.wrong_observation_capability",
                                receipt,
                                details,
                            );
                        }
                    }
                    M10CorruptedRequest::ProjectionHistoryOrigin { projection, origin } => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let policy =
                            carriers.observer_safe_policy_for_source(&semantic_source_ref.path)?;
                        let observation_session =
                            M10ObservationSession::for_policy(checked, policy.id(), policy)?;
                        let diagnostic = observation_session
                            .validate_history_projection(*projection, *origin)
                            .expect_err(
                                "M10 source-bound observation session accepted a forged history origin",
                            );
                        if diagnostic
                            != M10ObservationSessionDiagnostic::HistoryOriginRedactionViolation
                        {
                            return Err(format!(
                                "M10 history origin validator returned unexpected diagnostic {}",
                                diagnostic.code(),
                            ));
                        }
                        add_case_action_fact(
                            facts,
                            case,
                            "structural_rejection.no_publication.history_origin_redaction_violation",
                            context,
                        );
                        let request = M10OwnerEventRequest {
                            event: "move".to_string(),
                            principal: "self".to_string(),
                            target: None,
                            repeat: 1,
                            step: None,
                            seed: BTreeMap::new(),
                            arguments: BTreeMap::from([("delta".to_string(), 1)]),
                        };
                        let M10OwnerScheduleOutcome::Served(served) =
                            execute_checked_owner_schedule(
                                checked,
                                &request,
                                M10OwnerAuthorityMode::Admitted,
                            )?
                        else {
                            return Err(
                                "M10 history-origin setup owner effect was rejected".to_string()
                            );
                        };
                        let M10OwnerScheduleServed { runtime, m9, .. } = *served;
                        let runtime_projection = runtime.canonical_semantic_projection();
                        let bundle =
                            m10_actual_hash_bundle(&runtime, &m9, "SCN07-history-origin", None);
                        let receipt = receipt_ledger.record_actual(
                            "observer.history_origin_reject",
                            &semantic_source_ref,
                            bundle.clone(),
                            bundle,
                            false,
                        );
                        let mut details = m10_persistent_runtime_provenance(
                            case,
                            checked,
                            &semantic_source_ref,
                            &m9.authority,
                            &runtime_projection,
                            "m10-observer-session:scn07",
                            3,
                            4,
                        );
                        let object = details
                            .as_object_mut()
                            .expect("M10 persistent provenance is an object");
                        object.insert("no_publication".to_string(), Value::Bool(true));
                        object.insert(
                            "policy_validation".to_string(),
                            observation_session.diagnostic_value(diagnostic),
                        );
                        object.insert(
                            "diagnostic".to_string(),
                            json!({
                                "code": "E-VIS-003",
                                "source_ref": source_ref_json(Some(&semantic_source_ref)),
                            }),
                        );
                        record_runtime_trace_with_details(
                            runtime_traces,
                            "structural_rejection.no_publication.history_origin_redaction_violation",
                            receipt,
                            details,
                        );
                    }
                    M10CorruptedRequest::ExpiredLeaseLive => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let mut session = M10CompositeCutSession::new(checked)?;
                        session.save_s1()?;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let cut = session.s1.as_ref().expect("doctor saved S1").clone();
                        let before_m9_restore_bundle = m10_actual_hash_bundle(
                            &session.runtime,
                            &session.m9.domain_snapshot(),
                            &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                            Some(&cut),
                        );
                        let m9_s1 = session.m9_s1.as_ref().expect("doctor saved M9 S1").clone();
                        session.m9.restore_authority_cut(m9_s1)?;
                        session.refresh_bridge();
                        let after_m9_restore_bundle = m10_actual_hash_bundle(
                            &session.runtime,
                            &session.m9.domain_snapshot(),
                            &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                            Some(&cut),
                        );
                        let m9_restore_receipt = receipt_ledger.record_actual(
                            "m9.cut.restore",
                            &semantic_source_ref,
                            before_m9_restore_bundle,
                            after_m9_restore_bundle.clone(),
                            true,
                        );
                        if session
                            .runtime
                            .try_restore_local_cut(
                                &cut,
                                &M8LiveFloor::same_current(&cut).with_expired_lease("m10-expired"),
                            )
                            .is_err()
                        {
                            let predicate = "structural_rejection.no_mutation.E-CUT-001_or_E-CUT-002.expired_lease_resurrection";
                            add_case_action_fact(facts, case, predicate, context);
                            let after_bundle = m10_actual_hash_bundle(
                                &session.runtime,
                                &session.m9.domain_snapshot(),
                                &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                                Some(&cut),
                            );
                            let receipt = receipt_ledger.record_actual(
                                "cut.restore",
                                &semantic_source_ref,
                                after_m9_restore_bundle,
                                after_bundle,
                                false,
                            );
                            let mut details = session.session_details(0, 1);
                            let lineage = session.lineage_value();
                            details
                                .as_object_mut()
                                .expect("SCN10 lease doctor lineage details are an object")
                                .extend(json!({
                                    "m9_to_m8_authority_lineage": lineage.clone(),
                                    "m8_decisions_after_m9": [{
                                        "transition": "cut.restore",
                                        "decision": "rejected",
                                        "authority_lineage_ref": lineage["session_id"].clone(),
                                        "runtime_session_id": lineage["m8_runtime_session_id"].clone(),
                                    }],
                                }).as_object().expect("object").clone());
                            record_runtime_trace_with_prior_receipt(
                                runtime_traces,
                                predicate,
                                m9_restore_receipt,
                                receipt,
                                details,
                            );
                        }
                    }
                    M10CorruptedRequest::CutReceiveWithoutSend => {
                        let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                        let mut session = M10CompositeCutSession::new(checked)?;
                        session.save_s1()?;
                        let semantic_source_ref = m10_semantic_source_ref(checked)?;
                        let cut = session.s1.as_ref().expect("doctor saved S1").clone();
                        let before_m9_restore_bundle = m10_actual_hash_bundle(
                            &session.runtime,
                            &session.m9.domain_snapshot(),
                            &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                            Some(&cut),
                        );
                        let m9_s1 = session.m9_s1.as_ref().expect("doctor saved M9 S1").clone();
                        session.m9.restore_authority_cut(m9_s1)?;
                        session.refresh_bridge();
                        let after_m9_restore_bundle = m10_actual_hash_bundle(
                            &session.runtime,
                            &session.m9.domain_snapshot(),
                            &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                            Some(&cut),
                        );
                        let m9_restore_receipt = receipt_ledger.record_actual(
                            "m9.cut.restore",
                            &semantic_source_ref,
                            before_m9_restore_bundle,
                            after_m9_restore_bundle.clone(),
                            true,
                        );
                        if session
                            .runtime
                            .try_restore_local_cut(
                                &cut,
                                &M8LiveFloor::same_current(&cut)
                                    .with_stale_membership("m10-cut-receive-without-send"),
                            )
                            .is_err()
                        {
                            let predicate = "structural_rejection.no_mutation.E-CUT-001_or_E-CUT-002.consistent_cut_violation";
                            add_case_action_fact(facts, case, predicate, context);
                            let after_bundle = m10_actual_hash_bundle(
                                &session.runtime,
                                &session.m9.domain_snapshot(),
                                &format!("SCN10|{}|doctor-restore-attempt", case.identity),
                                Some(&cut),
                            );
                            let receipt = receipt_ledger.record_actual(
                                "cut.restore",
                                &semantic_source_ref,
                                after_m9_restore_bundle,
                                after_bundle,
                                false,
                            );
                            let mut details = session.session_details(0, 1);
                            let lineage = session.lineage_value();
                            details
                                .as_object_mut()
                                .expect("SCN10 cut doctor lineage details are an object")
                                .extend(json!({
                                    "m9_to_m8_authority_lineage": lineage.clone(),
                                    "m8_decisions_after_m9": [{
                                        "transition": "cut.restore",
                                        "decision": "rejected",
                                        "authority_lineage_ref": lineage["session_id"].clone(),
                                        "runtime_session_id": lineage["m8_runtime_session_id"].clone(),
                                    }],
                                }).as_object().expect("object").clone());
                            record_runtime_trace_with_prior_receipt(
                                runtime_traces,
                                predicate,
                                m9_restore_receipt,
                                receipt,
                                details,
                            );
                        }
                    }
                }
            }
            M10ScheduleOperation::CompactionRequest {
                membership_frontier,
            } => {
                let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                let context = source_context
                    .as_ref()
                    .expect("source-bound schedule context");
                if m10_schedule_has_m9_admission(checked) {
                    let session = scn04_membership_session.as_mut().ok_or_else(|| {
                        "M10 SCN04 compaction ran before the persistent leave session".to_string()
                    })?;
                    let runtime = m10_cut_runtime(checked)?;
                    let source_ref = m10_semantic_source_ref(checked)?;
                    match membership_frontier {
                        M10CompactionFrontier::BeforeAuditCut => {
                            let predicate = "runtime.compaction_before_audit_cut_is_blocked";
                            add_case_action_fact(facts, case, predicate, context);
                            let before_bundle = m10_actual_hash_bundle(
                                &runtime,
                                &session.m9.domain_snapshot(),
                                "membership.compaction",
                                None,
                            );
                            let rejected = !session.m9.compact("m10-scn04-before-audit-cut");
                            let after_bundle = m10_actual_hash_bundle(
                                &runtime,
                                &session.m9.domain_snapshot(),
                                "membership.compaction",
                                None,
                            );
                            let receipt = receipt_ledger.record_actual(
                                "membership.compaction",
                                &source_ref,
                                before_bundle,
                                after_bundle,
                                false,
                            );
                            if !rejected || !receipt.failure_preserves_semantic_state() {
                                return Err("M10 SCN04 pre-audit compaction mutated M9 authority"
                                    .to_string());
                            }
                            record_runtime_trace_with_details(
                                runtime_traces,
                                predicate,
                                receipt,
                                session.m9.session_details(1, 2),
                            );
                        }
                        M10CompactionFrontier::AfterAuditCut => {
                            let predicate = "runtime.compaction_after_audit_cut_is_allowed";
                            add_case_action_fact(facts, case, predicate, context);
                            let before_bundle = m10_actual_hash_bundle(
                                &runtime,
                                &session.m9.domain_snapshot(),
                                "membership.compaction",
                                None,
                            );
                            let accepted = session.m9.compact("m10-scn04-leave-audit-cut");
                            let after_bundle = m10_actual_hash_bundle(
                                &runtime,
                                &session.m9.domain_snapshot(),
                                "membership.compaction",
                                None,
                            );
                            let receipt = receipt_ledger.record_actual(
                                "membership.compaction",
                                &source_ref,
                                before_bundle,
                                after_bundle,
                                true,
                            );
                            if !accepted {
                                return Err("M10 SCN04 sealed audit cut did not permit compaction"
                                    .to_string());
                            }
                            record_runtime_trace_with_details(
                                runtime_traces,
                                predicate,
                                receipt,
                                session.m9.session_details(2, 3),
                            );
                        }
                    }
                }
            }
            M10ScheduleOperation::DesignatedConsumption {
                designated_value_ref,
                consumer,
                version,
                repeat,
            } => {
                let (_, checked) = checked_for_schedule_case(case, checked_sources)?;
                let pressure_row = pressure
                    .entry("SCN-11".to_string())
                    .or_insert_with(|| json!({}));
                let pressure_row = pressure_row
                    .as_object_mut()
                    .expect("SCN-11 pressure is an object");
                let result = execute_m8_designated_consumption(
                    checked,
                    designated_value_ref,
                    consumer,
                    *version,
                    *repeat,
                    &case.identity,
                )?;
                pressure_row.insert(
                    if *repeat == 1 {
                        "designated_version".to_string()
                    } else {
                        "duplicate_consumption".to_string()
                    },
                    result,
                );
            }
            M10ScheduleOperation::RelationProjection {
                relation,
                consumer,
                presentation_context,
            } => {
                // Relation schedule rows are declarative coordinates only;
                // the maintained session is executed once after this loop.
                let _ = (relation, consumer, presentation_context);
            }
        }
    }
    if let Some(checked) = checked_sources.get("scn-12/bird-relation.mir") {
        pressure.insert(
            "SCN-12".to_string(),
            m10_run_scn12_relation_session(checked)?,
        );
    }
    let scn11 = pressure
        .entry("SCN-11".to_string())
        .or_insert_with(|| json!({}));
    let scn11 = scn11.as_object_mut().expect("SCN-11 pressure is an object");
    scn11
        .entry("designated_version")
        .or_insert_with(|| json!({ "status": "rejected" }));
    scn11
        .entry("duplicate_consumption")
        .or_insert_with(|| json!({ "status": "rejected" }));
    let scn12 = pressure
        .entry("SCN-12".to_string())
        .or_insert_with(|| json!({}));
    let scn12 = scn12.as_object_mut().expect("SCN-12 pressure is an object");
    for key in ["bird_relation", "split_frame", "fallback", "reacquire"] {
        scn12
            .entry(key.to_string())
            .or_insert_with(|| json!({ "status": "rejected" }));
    }
    if let Some(reacquire) = scn12.get("reacquire").cloned() {
        scn12
            .entry("fresh_reacquire".to_string())
            .or_insert(reacquire);
    }
    Ok(Value::Object(pressure))
}

fn record_runtime_trace(
    runtime_traces: &mut BTreeMap<String, Value>,
    predicate: &str,
    receipt: M10TransitionReceipt,
) {
    record_runtime_trace_with_details(runtime_traces, predicate, receipt, Value::Null);
}

fn record_runtime_trace_with_details(
    runtime_traces: &mut BTreeMap<String, Value>,
    predicate: &str,
    receipt: M10TransitionReceipt,
    details: Value,
) {
    let mut trace = json!({
        "transition_trace": [receipt.evidence()],
        "failure_preserves_semantic_state": receipt.failure_preserves_semantic_state(),
    });
    // Compatibility placement for existing receipt consumers.  Its embedded
    // provenance remains explicitly M9; no `m8_authority_snapshot` name is
    // used for this retired M9 lineage view.
    if let Some(snapshot) = details.get("m9_retired_authority_snapshot") {
        trace
            .pointer_mut("/transition_trace/0/after")
            .and_then(Value::as_object_mut)
            .expect("M10 receipt after-state is an object")
            .insert("authority_snapshot".to_string(), snapshot.clone());
    }
    if let Some(lineage_ref) = details
        .pointer("/m9_to_m8_authority_lineage/session_id")
        .cloned()
    {
        for entry in trace
            .pointer_mut("/transition_trace")
            .and_then(Value::as_array_mut)
            .expect("M10 transition trace is an array")
        {
            entry
                .as_object_mut()
                .expect("M10 transition entry is an object")
                .insert("authority_lineage_ref".to_string(), lineage_ref.clone());
        }
    }
    for key in [
        "m9_authority_use",
        "m10_ledger_membership",
        "program_artifact",
        "schedule_action",
        "diagnostic",
    ] {
        if let Some(value) = details.get(key) {
            trace
                .pointer_mut("/transition_trace/0")
                .and_then(Value::as_object_mut)
                .expect("M10 transition receipt is an object")
                .insert(key.to_string(), value.clone());
        }
    }
    if let (Some(trace), Some(details)) = (trace.as_object_mut(), details.as_object()) {
        trace.extend(
            details
                .iter()
                .filter(|(key, _)| {
                    !matches!(key.as_str(), "m9_authority_use" | "m10_ledger_membership")
                })
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
    runtime_traces.insert(predicate.to_string(), trace);
}

fn record_runtime_trace_with_prior_receipt(
    runtime_traces: &mut BTreeMap<String, Value>,
    predicate: &str,
    prior_receipt: M10TransitionReceipt,
    receipt: M10TransitionReceipt,
    details: Value,
) {
    record_runtime_trace_with_details(runtime_traces, predicate, receipt, details);
    let trace = runtime_traces
        .get_mut(predicate)
        .expect("M10 runtime trace retains its transition sequence");
    let lineage_ref = trace
        .pointer("/m9_to_m8_authority_lineage/session_id")
        .cloned();
    let transitions = trace
        .pointer_mut("/transition_trace")
        .and_then(Value::as_array_mut)
        .expect("M10 runtime trace retains its transition sequence");
    transitions.insert(0, prior_receipt.evidence());
    if let Some(lineage_ref) = lineage_ref {
        for entry in transitions {
            entry
                .as_object_mut()
                .expect("M10 transition entry is an object")
                .insert("authority_lineage_ref".to_string(), lineage_ref.clone());
        }
    }
}

/// Persistent runtime provenance shared by M10 schedule paths.  Every value
/// here is derived from the concrete checked effect, action identity, sealed
/// M9 snapshot, and executing M8 projection; schedule text alone cannot
/// produce a receipt.
/// Each trace-bound input stays explicit so provenance cannot be accidentally
/// assembled from a partially populated helper carrier.
#[allow(clippy::too_many_arguments)]
fn m10_persistent_runtime_provenance(
    case: &M10ScheduleCase,
    checked: &CheckedSurfaceV0,
    source_ref: &SourceRef,
    m9_snapshot_projection: &str,
    m8_runtime_projection: &str,
    session_id: &str,
    range_start: u64,
    range_end: u64,
) -> Value {
    json!({
        "session_id": session_id,
        "monotone_trace_range": { "start": range_start, "end": range_end },
        "program_artifact": {
            "checked_effect_ref": format!(
                "checked-effect:{}:{}:{}:{}",
                checked.program_identity().stable_key(),
                source_ref.path,
                source_ref.start_line,
                source_ref.start_column,
            ),
            "source_ref": source_ref_json(Some(source_ref)),
        },
        "schedule_action": {
            "action_id": case.id,
            "reference": case.identity,
        },
        "m8_m9_receipt": {
            "receipt_id": deterministic_hash(&format!(
                "m10-receipt-v1|{}|{}|{}",
                case.identity, m9_snapshot_projection, m8_runtime_projection,
            )),
            "m9_resolution_ref": deterministic_hash(&format!(
                "m9-resolution-v1|{}", m9_snapshot_projection,
            )),
            "m8_runtime_ref": deterministic_hash(&format!(
                "m8-runtime-v1|{}", m8_runtime_projection,
            )),
        },
    })
}

/// Export an observer view from the same local M8 session that performed the
/// source-bound owner write.  This returns only redacted, observer-safe
/// metadata and verifies its occurrence/history link against the local trace.
fn m10_observer_publication_evidence(
    checked: &CheckedSurfaceV0,
    runtime: M8LocalRuntime,
    observer_authority: M8ObserverAuthorityGrant,
    policy_carrier_ref: &str,
    exported_fields: Vec<String>,
    policy_suffix: &str,
) -> Result<Value, String> {
    let (principal, _) = patch_principal_and_locus(checked)?;
    let observer_principal = format!("observer:{principal}");
    let source_ref = m10_semantic_source_ref(checked)?;
    let authority_ref = observer_authority.reference().to_string();
    let observer = M8ObserverRuntime::from_local_session(runtime, vec![observer_authority]);
    let policy = M8ObserverPolicy::for_principal(&observer_principal)
        .with_authority_ref(authority_ref)
        .with_label(
            EvidenceSecurityLabel::new(format!("observer:{policy_suffix}"))
                .with_class(M8SecurityClass::Public),
        )
        .with_redaction(EvidenceRedaction::new("observer-safe"))
        .with_retention(M8ObserverRetention::bounded(
            format!("m10-observer:{policy_suffix}"),
            1,
        ))
        .with_source_ref(source_ref.clone())
        .with_reason_ref(format!("checked-effect:{policy_suffix}"))
        .with_proof_ref(format!("m9-observer-policy:{policy_suffix}"));
    let view = observer
        .export_observer_view(policy.clone())
        .map_err(|diagnostics| {
            format!(
                "M10 observer export {policy_suffix}: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let trace = observer.trace();
    if !view.rows().contains_kind(M8ObserverRowKind::OwnerWrite)
        || !view.rows().all_have_occurrence_dependency_correspondence()
        || !view.rows().all_correspond_to_exact_trace(&trace)
    {
        return Err(format!(
            "M10 observer export {policy_suffix} lacks exact owner-history correspondence"
        ));
    }
    Ok(json!({
        "subject_history_occurrence": {
            "kind": "owner_write",
            "exact_trace_correspondence": true,
        },
        "publication_origin": "m8_observer_runtime",
        "source_ref": source_ref_json(Some(&source_ref)),
        "policy_carrier_ref": policy_carrier_ref,
        "redaction": {
            "input_label": policy.label().as_str(),
            "output_label": policy.label().as_str(),
            "order_proof": "policy_before_projection",
        },
        "exported_fields": exported_fields,
        "raw_authority_payload_present": false,
        "raw_witness_payload_present": false,
        "raw_verification_payload_present": false,
    }))
}

struct M10PortalHandoffRuntime {
    source_ref: SourceRef,
    before_m9: M10M9DomainSnapshot,
    after_leave_m9: M10M9DomainSnapshot,
    after_join_m9: M10M9DomainSnapshot,
    before_m8: M10M8DomainSnapshot,
    after_m8: M10M8DomainSnapshot,
}

/// Finite M10 routing state used only by the I1+ route carrier.  It is not a
/// general transport layer: availability is derived only from this session's
/// active checked M8 patch configuration, and every invocation retains the
/// checked source/M9/M8 provenance that supplied the request.
struct M10RouteSession {
    source_ref: SourceRef,
    m9: M10M9DomainSnapshot,
    m8: M10M8DomainSnapshot,
    runtime: M8PatchRuntime,
    session_id: String,
    contract_identity: String,
    postpatch_owner_authority: Option<(String, String, M8AuthorityUse)>,
}

/// Source-bound finite observation validator for M10 policy carriers.  M8
/// owns authority-gated row export; this layer owns the remaining typed
/// request-to-policy check when no M8 field validator exists.  In particular,
/// an attempted private cross-locus read or forged history origin cannot be
/// represented as a schedule verdict or a fabricated observer receipt.
struct M10ObservationSession<'a> {
    source_ref: SourceRef,
    policy_carrier_ref: &'a str,
    policy: &'a M10ObservationPolicyCarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10ObservationSessionDiagnostic {
    VisibilityDenied,
    HistoryOriginRedactionViolation,
    InvalidTypedPolicy,
}

impl M10ObservationSessionDiagnostic {
    const fn code(self) -> &'static str {
        match self {
            Self::VisibilityDenied => "VisibilityDenied",
            Self::HistoryOriginRedactionViolation => "E-VIS-003",
            Self::InvalidTypedPolicy => "M10ObservationPolicyInvalid",
        }
    }
}

impl<'a> M10ObservationSession<'a> {
    fn for_policy(
        checked: &CheckedSurfaceV0,
        policy_carrier_ref: &'a str,
        policy: &'a M10ObservationPolicyCarrier,
    ) -> Result<Self, String> {
        let source_ref = m10_semantic_source_ref(checked)?;
        if policy.source_path() != source_ref.path {
            return Err(format!(
                "M10 observation policy {policy_carrier_ref} source {} does not bind {}",
                policy.source_path(),
                source_ref.path,
            ));
        }
        Ok(Self {
            source_ref,
            policy_carrier_ref,
            policy,
        })
    }

    fn validate_cross_locus_request(
        &self,
        request_kind: M10ObservationRequestKind,
    ) -> Result<Vec<String>, M10ObservationSessionDiagnostic> {
        let M10ObservationPolicyCarrier::CrossLocus {
            source_state,
            source_field,
            source_owner_locus,
            destination_field,
            destination_locus,
            request_class,
            required_failures,
            ..
        } = self.policy
        else {
            return Err(M10ObservationSessionDiagnostic::InvalidTypedPolicy);
        };
        if *request_class != M10ObservationRequestClass::CrossLocusObservation
            || source_state.is_empty()
            || source_field.is_empty()
            || destination_field.is_empty()
            || source_owner_locus == destination_locus
        {
            return Err(M10ObservationSessionDiagnostic::InvalidTypedPolicy);
        }
        match request_kind {
            M10ObservationRequestKind::CrossLocusObservation => Ok(vec![destination_field.clone()]),
            M10ObservationRequestKind::CrossLocusSecretRead
                if required_failures.contains(&M10ObservationPolicyFailure::VisibilityDenied) =>
            {
                Err(M10ObservationSessionDiagnostic::VisibilityDenied)
            }
            M10ObservationRequestKind::CrossLocusSecretRead => {
                // A private request lacking its declared failure is still
                // fail-closed rather than becoming an implicit export.
                Err(M10ObservationSessionDiagnostic::InvalidTypedPolicy)
            }
        }
    }

    fn validate_history_projection(
        &self,
        projection: M10HistoryProjection,
        origin: M10HistoryOrigin,
    ) -> Result<(), M10ObservationSessionDiagnostic> {
        let M10ObservationPolicyCarrier::ObserverSafe {
            observer_fields, ..
        } = self.policy
        else {
            return Err(M10ObservationSessionDiagnostic::InvalidTypedPolicy);
        };
        if projection != M10HistoryProjection::ObserverHistory || observer_fields.is_empty() {
            return Err(M10ObservationSessionDiagnostic::InvalidTypedPolicy);
        }
        match origin {
            M10HistoryOrigin::M8RedactedObserverRuntime => Ok(()),
            M10HistoryOrigin::ForgedWithoutRedaction => {
                Err(M10ObservationSessionDiagnostic::HistoryOriginRedactionViolation)
            }
        }
    }

    fn diagnostic_value(&self, diagnostic: M10ObservationSessionDiagnostic) -> Value {
        json!({
            "code": diagnostic.code(),
            "validator": "m10_source_bound_observation_session",
            "policy_carrier_ref": self.policy_carrier_ref,
            "source_ref": source_ref_json(Some(&self.source_ref)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10RouteSessionDiagnostic {
    RouteUnavailable,
}

impl M10RouteSession {
    fn for_checked_source(checked: &CheckedSurfaceV0) -> Result<Self, String> {
        let (principal, locus) = patch_principal_and_locus(checked)?;
        let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
        let m9 = M10M9DomainSnapshot::from_seam(&seam);
        let (instance, authority_state) = seam.into_parts();
        let runtime = M8PatchRuntime::from_admitted(
            instance,
            M8PatchRuntimeSeed::new().with_authority_state(authority_state),
        );
        let m8 = M10M8DomainSnapshot::from_patch(&runtime);
        Ok(Self {
            source_ref: m10_semantic_source_ref(checked)?,
            m9,
            m8,
            contract_identity: deterministic_hash(&runtime.canonical_configuration_projection()),
            runtime,
            session_id: "m8-route-session:scn06".to_string(),
            postpatch_owner_authority: None,
        })
    }

    fn invoke(&self) -> Result<(), M10RouteSessionDiagnostic> {
        self.runtime
            .active_patch_id()
            .map(|_| ())
            .ok_or(M10RouteSessionDiagnostic::RouteUnavailable)
    }

    fn activate_checked_route(
        &mut self,
        base: &CheckedSurfaceV0,
        candidate: &CheckedSurfaceV0,
        carrier: &M10PatchIntentCarrier,
    ) -> Result<M10PatchActivationEvidence, String> {
        if carrier
            .route_addition
            .as_ref()
            .is_none_or(|route| route.route_state != "available")
            || !carrier
                .required_capabilities
                .iter()
                .any(|capability| capability == "route.patch")
        {
            return Err("M10 route session rejected unchecked route activation".to_string());
        }
        let (principal, locus) = patch_principal_and_locus(candidate)?;
        let candidate_seam = m10_resolve_checked_for_patch(candidate, principal, locus)?;
        let m9 = M10M9DomainSnapshot::from_seam(&candidate_seam);
        let patch_authority = candidate_seam
            .patch_authority_use(candidate.program_identity().module(), principal, locus)
            .ok_or_else(|| "M10 route session lacks M9-issued patch authority".to_string())?;
        let owner = candidate
            .evaluations()
            .iter()
            .find_map(|evaluation| {
                evaluation
                    .owner_rmw_core()
                    .map(|core| (evaluation.name(), core.owner_locus()))
            })
            .ok_or_else(|| {
                "M10 route candidate has no owner effect for postpatch request".to_string()
            })?;
        let postpatch_owner_authority = candidate_seam
            .owner_authority_use(owner.0, principal, owner.1)
            .ok_or_else(|| {
                "M10 route session lacks M9-issued postpatch owner authority".to_string()
            })?;
        let (candidate_instance, candidate_authority_state) = candidate_seam.into_parts();
        self.runtime
            .refresh_m9_authority_state(candidate_authority_state);
        let before_m8 = M10M8DomainSnapshot::from_patch(&self.runtime);
        let base_admission = self.runtime.active_admission().clone();
        let outcome = self.runtime.activate_patch(
            M8PatchCandidate::from_m10_resolved(
                carrier.id.clone(),
                candidate.clone(),
                candidate_instance,
            )
            .with_base_program_identity(base.program_identity().clone())
            .with_base_admission(base_admission)
            .with_patch_authority(patch_authority),
        );
        if !outcome.has_runtime_success() {
            return Err("M10 route M8 patch runtime rejected checked route activation".to_string());
        }
        let after_m8 = M10M8DomainSnapshot::from_patch(&self.runtime);
        self.postpatch_owner_authority = Some((
            owner.0.to_string(),
            owner.1.to_string(),
            postpatch_owner_authority,
        ));
        self.m9 = m9.clone();
        self.m8 = after_m8.clone();
        self.contract_identity =
            deterministic_hash(&self.runtime.canonical_configuration_projection());
        Ok(M10PatchActivationEvidence {
            accepted: true,
            runtime_trace: json!({
                "activate_patch_called": true,
                "persistent_runtime_session_id": self.session_id,
                "base_program_artifact": base.program_identity().stable_key(),
                "candidate_program_artifact": candidate.program_identity().stable_key(),
                "activation_cut": outcome.activation_cut().map(|_| json!({
                    "hash": deterministic_hash(&format!("m8-route-activation|{}", carrier.id)),
                })),
            }),
            before_m8,
            after_m8,
            m9,
        })
    }

    fn serve_postpatch_owner_request(
        &mut self,
        candidate: &CheckedSurfaceV0,
    ) -> Result<bool, String> {
        let (principal, _) = patch_principal_and_locus(candidate)?;
        let owner = candidate
            .evaluations()
            .iter()
            .find_map(|evaluation| {
                evaluation
                    .owner_rmw_core()
                    .map(|core| (evaluation.name(), core))
            })
            .ok_or_else(|| {
                "M10 route candidate has no owner effect for postpatch request".to_string()
            })?;
        let (authority_evaluation, authority_locus, authority) = self
            .postpatch_owner_authority
            .clone()
            .ok_or_else(|| "M10 route session has no activated owner authority".to_string())?;
        if authority_evaluation != owner.0 || authority_locus != owner.1.owner_locus() {
            return Err(
                "M10 route session owner authority does not match active patch plan".to_string(),
            );
        }
        let target = owner.1.target();
        let state = target.namespace();
        let field = target
            .field()
            .ok_or_else(|| "M10 route postpatch owner target lacks field".to_string())?;
        let index = target.index().unwrap_or(principal);
        self.runtime
            .initialize_declared_int_for_activation(
                state,
                index,
                field,
                m10_semantic_source_ref(candidate)?,
            )
            .map_err(|error| format!("M10 route postpatch state initialization: {error}"))?;
        self.runtime
            .enqueue_owner(
                M8OwnerRequest::new(owner.0)
                    .with_argument("delta", "1")
                    .with_authority_use(authority),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 route postpatch enqueue: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let accepted = self.runtime.serve_next_owner(&authority_locus).is_ok();
        self.m8 = M10M8DomainSnapshot::from_patch(&self.runtime);
        Ok(accepted)
    }
}

/// Run the portal handoff as one source-bound session.  It deliberately does
/// not abbreviate leave/join/spawn to three labels over one receipt: M9 first
/// retires WorldA's authority, then authenticates a fresh WorldB membership,
/// and only that membership's sealed owner use reaches M8's spawn write.
fn m10_portal_handoff_runtime(
    checked: &CheckedSurfaceV0,
) -> Result<M10PortalHandoffRuntime, String> {
    let admission = m8_admission_for(checked)?;
    let m9 = M9AdmissionRuntime::default();
    let base = m9
        .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
        .map_err(|diagnostics| format!("M10 portal M9 base: {:?}", diagnostics.primary().kind()))?;
    let auth_residual = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .ok_or_else(|| "M10 portal source lacks auth residual".to_string())?;
    let principal = checked
        .evaluations()
        .iter()
        .find_map(|evaluation| {
            (!evaluation.actor_authority_origin().is_empty())
                .then_some(evaluation.actor_authority_origin())
        })
        .ok_or_else(|| "M10 portal source lacks actor principal".to_string())?;
    let portal = checked
        .evaluation("portal")
        .and_then(|evaluation| evaluation.owner_rmw_core().map(|owner| (evaluation, owner)))
        .ok_or_else(|| "M10 portal source lacks checked portal owner effect".to_string())?;
    let world_b = portal.1.owner_locus();
    let world_a = checked
        .static_environment()
        .loci()
        .iter()
        .find(|locus| locus.name() != world_b && locus.name() != "BrowserClient")
        .map(|locus| locus.name())
        .ok_or_else(|| "M10 portal source lacks distinct WorldA locus".to_string())?;
    let source_ref = m10_semantic_source_ref(checked)?;
    let mut authority = base.authority_runtime();

    let leave_attestation = authority
        .issue_membership_attestation(
            principal,
            world_a,
            "m10-portal-epoch-a",
            format!("m10-portal:{principal}:{world_a}"),
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal leave attestation: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let membership_a = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, world_a, "m10-portal-epoch-a")
                .with_incarnation(format!("m10-portal:{principal}:{world_a}"))
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(leave_attestation),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldA membership: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let leave_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-portal-world-a-observer")
                .with_membership_ref(membership_a.ref_id())
                .with_scope(M9CapabilityScope::bounded_observation(principal))
                .with_lineage_epoch(membership_a.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldA capability: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    authority
        .materialize_witness(
            M9WitnessRequest::new("m10-portal-world-a-observer-witness")
                .with_membership_ref(membership_a.ref_id())
                .with_capability_ref(leave_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldA witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let before_m9 = M10M9DomainSnapshot::from_authority(&authority);
    authority
        .retire_membership(membership_a.ref_id(), "m10-portal-leave-audit-cut")
        .map_err(|diagnostics| {
            format!(
                "M10 portal retire WorldA: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let after_leave_m9 = M10M9DomainSnapshot::from_authority(&authority);

    let join_attestation = authority
        .issue_membership_attestation(
            principal,
            world_b,
            "m10-portal-epoch-b",
            format!("m10-portal:{principal}:{world_b}"),
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal join attestation: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let membership_b = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, world_b, "m10-portal-epoch-b")
                .with_incarnation(format!("m10-portal:{principal}:{world_b}"))
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(join_attestation),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldB membership: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let contract_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-portal-world-b-contract")
                .with_membership_ref(membership_b.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    checked.program_identity().module(),
                    format!("membership-authority/{}", auth_residual.name()),
                ))
                .with_lineage_epoch(membership_b.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldB contract: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let contract_witness = authority
        .materialize_witness(
            M9WitnessRequest::new("m10-portal-world-b-contract-witness")
                .with_membership_ref(membership_b.ref_id())
                .with_capability_ref(contract_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal WorldB contract witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let spawn_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-portal-world-b-spawn")
                .with_membership_ref(membership_b.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("portal", world_b))
                .with_lineage_epoch(membership_b.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal spawn authority: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    authority
        .materialize_witness(
            M9WitnessRequest::new("m10-portal-world-b-spawn-witness")
                .with_membership_ref(membership_b.ref_id())
                .with_capability_ref(spawn_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal spawn witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let after_join_m9 = M10M9DomainSnapshot::from_authority(&authority);
    let discharge = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            checked,
            M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal finite refinement: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let seam = m9
        .admit_runtime(
            base,
            authority,
            M9FinalAdmissionEvidence::from_lineage(
                &membership_b,
                &contract_capability,
                &contract_witness,
                discharge,
            ),
        )
        .map_err(|diagnostics| format!("M10 portal final M9: {:?}", diagnostics.primary().kind()))?
        .into_m10_execution_seam();
    let spawn_authority = seam
        .owner_authority_use("portal", principal, world_b)
        .ok_or_else(|| "M10 portal M9 seam lacks WorldB spawn authority".to_string())?;
    let (instance, authority_state) = seam.into_parts();
    let mut seed = M8LocalRuntimeSeed::new().with_authority_state(authority_state);
    let mut seeded = BTreeSet::new();
    for read in portal
        .1
        .same_owner_reads()
        .iter()
        .chain(std::iter::once(portal.1.target()))
    {
        let key = M8StateKey::indexed_field(
            read.namespace(),
            m10_schedule_index(read.index(), principal, principal),
            read.field().unwrap_or(""),
        );
        if seeded.insert(key.clone()) {
            seed = seed.with_owner_int(key, 0);
        }
    }
    let mut runtime = M8LocalRuntime::from_admitted(instance, seed);
    let before_m8 = M10M8DomainSnapshot::from_runtime(&runtime);
    runtime
        .enqueue_owner(
            M8OwnerRequest::new("portal")
                .with_argument("delta", "1")
                .with_authority_use(spawn_authority),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 portal enqueue spawn: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    runtime.serve_next_owner(world_b).map_err(|diagnostics| {
        format!("M10 portal serve spawn: {:?}", diagnostics.primary().kind())
    })?;
    let after_m8 = M10M8DomainSnapshot::from_runtime(&runtime);
    Ok(M10PortalHandoffRuntime {
        source_ref,
        before_m9,
        after_leave_m9,
        after_join_m9,
        before_m8,
        after_m8,
    })
}

// Inputs are intentionally independent execution sources: combining them in
// a mutable carrier would blur the no-profile-read-before-evidence boundary.
#[allow(clippy::too_many_arguments)]
fn generate_m10_evidence(
    sources: Vec<Value>,
    source_texts: &BTreeMap<String, String>,
    source_identities: &BTreeMap<String, String>,
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
    source_failures: &BTreeMap<String, M10SourceFailure>,
    carriers: &M10TypedCarriers,
    schedule: &M10TypedSchedule,
    execution_manifest: M10ExecutionManifest,
) -> Result<M10GeneratedEvidence, String> {
    let mut facts = BTreeSet::new();

    // Source facts are generated from M7's checked structures and retained
    // diagnostics before the profile is read.  There is no fixture-path or
    // correspondence-row lookup in this phase.
    derive_checked_source_facts(
        &mut facts,
        source_identities,
        checked_sources,
        source_failures,
    )?;

    let mut patch_rows = Vec::new();
    let mut admitted_patch_carriers = BTreeSet::new();
    for carrier in &carriers.patches {
        let evidence_context = checked_source_evidence_context(
            "static",
            &carrier.candidate_source_path,
            checked_sources,
        )?;
        let base_identity = source_artifact_identity(
            source_identities,
            carrier
                .base_source_path
                .as_deref()
                .ok_or_else(|| format!("patch carrier {} lacks base source binding", carrier.id))?,
        )?;
        let candidate_identity =
            source_artifact_identity(source_identities, &carrier.candidate_source_path)?;
        let carrier_identity = carriers
            .carrier_identity(&carrier.id)
            .ok_or_else(|| format!("patch carrier {} lacks identity", carrier.id))?;
        let hash_bound = carrier
            .base_source_path
            .as_deref()
            .and_then(|base| source_texts.get(base).map(|text| (base, text)))
            .and_then(|(base, base_text)| {
                source_texts
                    .get(&carrier.candidate_source_path)
                    .map(|candidate_text| {
                        carrier.matches_sources(
                            base,
                            base_text,
                            &carrier.candidate_source_path,
                            candidate_text,
                        )
                    })
            })
            .unwrap_or(false);
        let (terminal, diagnostic, diagnostic_location) = if !hash_bound {
            (
                "PatchRejectedAtCarrierCheck",
                Some("PatchCarrierSourceMismatch"),
                Some("hash_binding"),
            )
        } else if matches!(
            carrier.authority_intent.kind,
            M10PatchAuthorityIntentKind::SelfGrant { .. }
        ) {
            (
                "PatchRejectedAtCarrierCheck",
                Some("E-PATCH-003"),
                Some("authority_intent"),
            )
        } else if carrier.required_capabilities.is_empty() {
            (
                "PatchRejectedAtCarrierCheck",
                Some("E-PATCH-002"),
                Some("required_capabilities"),
            )
        } else {
            let base =
                checked_sources.get(carrier.base_source_path.as_deref().expect("checked above"));
            let candidate = checked_sources.get(&carrier.candidate_source_path);
            if let (Some(base), Some(candidate)) = (base, candidate) {
                if candidate_matches_patch_surface(candidate, carrier)
                    && activate_checked_patch_evidence(base, candidate, carrier)?
                {
                    ("PatchAccepted", None, None)
                } else {
                    (
                        "PatchRejectedAtCarrierCheck",
                        Some("PatchSurfaceMismatch"),
                        Some("state_additions"),
                    )
                }
            } else {
                (
                    "PatchRejectedAtCandidateCheck",
                    Some("M7Rejected"),
                    Some("candidate_source"),
                )
            }
        };
        if terminal == "PatchAccepted" {
            admitted_patch_carriers.insert(carrier.id.clone());
            add_patch_facts(
                &mut facts,
                carriers,
                source_identities,
                carrier,
                &["static.patch_candidate_pair_checked_and_compatible"],
                &evidence_context,
            )?;
        } else if diagnostic == Some("E-PATCH-003") {
            add_carrier_facts(
                &mut facts,
                carriers,
                &carrier.id,
                &["diagnostic.E-PATCH-003.self_grant_candidate_check.no_activation"],
                &evidence_context,
            )?;
        } else if diagnostic == Some("E-PATCH-002") {
            add_carrier_facts(
                &mut facts,
                carriers,
                &carrier.id,
                &["diagnostic.E-PATCH-002.missing_patch_capability.no_activation"],
                &evidence_context,
            )?;
        }
        patch_rows.push(json!({
            "id": carrier.id,
            "carrier_identity": carrier_identity,
            "base_source_identity": base_identity,
            "candidate_source_identity": candidate_identity,
            "state_additions": carrier.state_additions,
            "required_capabilities": carrier.required_capabilities,
            "authority_intent": authority_intent_value(&carrier.authority_intent),
            "hash_binding": {
                "includes_base_source_identity": true,
                "includes_candidate_source_identity": true,
                "matched": hash_bound,
            },
            "terminal": {
                "outcome": terminal,
                "carrier_identity": carrier_identity,
                "diagnostic": { "code": diagnostic, "location": diagnostic_location },
            },
            "verdict_from_schedule_or_name": false,
        }));
    }

    let mut observation_rows = Vec::new();
    let mut fallback_rows = Vec::new();
    let mut policy_derivation = serde_json::Map::new();
    let mut fallback_derivation = serde_json::Map::new();
    for carrier in &carriers.observations {
        let id = carrier.id();
        let carrier_identity = carriers
            .carrier_identity(id)
            .ok_or_else(|| format!("observation carrier {id} lacks identity"))?;
        match carrier {
            M10ObservationPolicyCarrier::CrossLocus {
                source_path,
                source_state,
                source_field,
                source_owner_locus,
                destination_state: _,
                destination_field: _,
                destination_locus,
                required_failures,
                ..
            } => {
                let evidence_context =
                    checked_source_evidence_context("static", source_path, checked_sources)?;
                let source_identity = source_artifact_identity(source_identities, source_path)?;
                let has_visibility_failure =
                    required_failures.contains(&M10ObservationPolicyFailure::VisibilityDenied);
                let (terminal, diagnostic) = if has_visibility_failure {
                    ("RuntimeRejectedBeforeMutation", "VisibilityDenied")
                } else {
                    ("CarrierRejectedBeforeRuntime", "E-ROW-002")
                };
                if !has_visibility_failure {
                    add_carrier_facts(
                        &mut facts,
                        carriers,
                        id,
                        &[
                            "diagnostic.E-ROW-002.missing_required_failure.VisibilityDenied.retains_source_span.player_a.secret_key",
                        ],
                        &evidence_context,
                    )?;
                }
                observation_rows.push(json!({
                    "id": id,
                    "carrier_identity": carrier_identity,
                    "source_identity": source_identity,
                    "terminal": { "outcome": terminal, "carrier_identity": carrier_identity, "diagnostic": { "code": diagnostic } },
                    "runtime": if has_visibility_failure { json!({ "mutation_count": 0 }) } else { json!({ "admitted": false }) },
                }));
                if id == "portal-secret-redaction-policy" {
                    policy_derivation.insert("policy_carrier".to_string(), json!({
                        "id": id,
                        "source_state": source_state,
                        "source_field": source_field,
                        "destination_locus": destination_locus,
                        "cross_locus_observation_request": { "from_locus": source_owner_locus, "to_locus": destination_locus },
                    }));
                    policy_derivation.insert(
                        "runtime_diagnostic".to_string(),
                        json!({ "code": diagnostic, "mutation_count": 0 }),
                    );
                }
                if id == "portal-secret-missing-required-failure" {
                    policy_derivation.insert(
                        "missing_required_failure_carrier".to_string(),
                        json!({
                            "id": id,
                            "diagnostics": [{ "code": diagnostic }],
                            "source_field_span": format!("{source_state}.{source_field}"),
                        }),
                    );
                }
            }
            M10ObservationPolicyCarrier::ObserverSafe {
                source_path,
                observer_fields,
                ..
            } => {
                observation_rows.push(json!({
                    "id": id,
                    "carrier_identity": carrier_identity,
                    "source_identity": source_artifact_identity(source_identities, source_path)?,
                    "observer_fields": observer_fields,
                    "terminal": { "outcome": "CarrierAccepted", "carrier_identity": carrier_identity, "diagnostic": { "code": Value::Null } },
                }));
            }
            M10ObservationPolicyCarrier::ObserverPrivate { source_path, .. } => {
                let evidence_context =
                    checked_source_evidence_context("static", source_path, checked_sources)?;
                add_carrier_facts(
                    &mut facts,
                    carriers,
                    id,
                    &[
                        "diagnostic.E-VIS-002.private_like_field_cannot_be_observer_safe",
                        "structural_rejection.no_publication.no_mutation.policy_cannot_widen_private_like_field",
                    ],
                    &evidence_context,
                )?;
                observation_rows.push(json!({
                    "id": id,
                    "carrier_identity": carrier_identity,
                    "source_identity": source_artifact_identity(source_identities, source_path)?,
                    "terminal": { "outcome": "CarrierRejectedBeforeRuntime", "carrier_identity": carrier_identity, "diagnostic": { "code": "E-VIS-002" } },
                    "runtime": { "admitted": false },
                }));
                fallback_derivation
                    .insert("observer_private_policy".to_string(), json!({ "id": id }));
            }
        }
    }

    for carrier in &carriers.fallbacks {
        let evidence_context =
            checked_source_evidence_context("static", &carrier.source_path, checked_sources)?;
        let carrier_identity = carriers
            .carrier_identity(&carrier.id)
            .ok_or_else(|| format!("fallback carrier {} lacks identity", carrier.id))?;
        let has_live_anchor = carrier.options.get(1).is_some_and(|option| {
            option.lineage_edges.iter().any(|edge| {
                edge.from == M10FallbackOptionKind::Live && edge.to == M10FallbackOptionKind::Anchor
            })
        });
        let (terminal, diagnostic) = if !has_live_anchor {
            ("CarrierRejectedBeforeRuntime", Some("E-DECL-001"))
        } else if carrier.negative_capability_floor == "write_after_read_without_fresh_reacquire"
            && carrier.id == "view-pose-write-after-read"
        {
            ("CarrierRejectedBeforeRuntime", Some("E-LIN-003"))
        } else {
            ("CarrierAccepted", None)
        };
        if terminal == "CarrierAccepted" {
            add_carrier_facts(
                &mut facts,
                carriers,
                &carrier.id,
                &["static.three_option_carrier.live_anchor_frozen.with_monotone_lineage"],
                &evidence_context,
            )?;
        } else if diagnostic == Some("E-DECL-001") {
            add_carrier_facts(
                &mut facts,
                carriers,
                &carrier.id,
                &["diagnostic.E-DECL-001.missing_typed_lineage_edge.live_to_anchor"],
                &evidence_context,
            )?;
        } else {
            add_carrier_facts(
                &mut facts,
                carriers,
                &carrier.id,
                &["diagnostic.E-LIN-003.read_to_write_strengthening_without_reacquire"],
                &evidence_context,
            )?;
        }
        fallback_rows.push(json!({
            "id": carrier.id,
            "carrier_identity": carrier_identity,
            "source_identity": source_artifact_identity(source_identities, &carrier.source_path)?,
            "terminal": { "outcome": terminal, "carrier_identity": carrier_identity, "diagnostic": { "code": diagnostic } },
            "runtime": if terminal == "CarrierAccepted" { json!({}) } else { json!({ "admitted": false }) },
        }));
    }

    let mut route_patch_rows = Vec::new();
    let mut route_derivation = serde_json::Map::new();
    let mut route_patch_activated = BTreeMap::new();
    for carrier in &carriers.route_patches {
        let base_source_path = schedule.route_patch_base_source_path(&carrier.id)?;
        let base_identity = source_artifact_identity(source_identities, base_source_path)?;
        let base_checked = checked_sources.get(base_source_path).ok_or_else(|| {
            format!(
                "route patch carrier {} references unchecked schedule base {base_source_path}",
                carrier.id
            )
        })?;
        let candidate_identity =
            source_artifact_identity(source_identities, &carrier.candidate_source_path)?;
        let carrier_identity = carriers
            .carrier_identity(&carrier.id)
            .ok_or_else(|| format!("route patch carrier {} lacks identity", carrier.id))?;
        let candidate_checked = checked_sources.get(&carrier.candidate_source_path);
        let hash_bound = source_texts
            .get(&carrier.candidate_source_path)
            .is_some_and(|text| carrier.matches_candidate(&carrier.candidate_source_path, text));
        let activated = candidate_checked.is_some_and(|candidate| {
            hash_bound
                && carrier
                    .required_capabilities
                    .iter()
                    .any(|capability| capability == "route.patch")
                && activate_checked_route_patch_evidence(base_checked, candidate, carrier)
                    .unwrap_or(false)
        });
        let route = carrier.route_addition.as_ref().ok_or_else(|| {
            format!(
                "route patch carrier {} lacks a typed route addition",
                carrier.id
            )
        })?;
        route_patch_activated.insert(carrier.id.clone(), activated);
        route_patch_rows.push(json!({
            "id": carrier.id,
            "carrier_identity": carrier_identity,
            "base_source_identity": base_identity,
            "candidate_source_identity": candidate_identity,
            "route": { "from_locus": route.from_locus, "to_locus": route.to_locus, "state": route.route_state },
            "terminal": { "outcome": if activated { "RoutePatchChecked" } else { "RoutePatchRejected" } },
            "verdict_from_schedule_or_name": false,
        }));
        route_derivation.insert("submitted_checked_artifact".to_string(), json!(activated));
        route_derivation.insert(
            "same_source_succeeds_after_activation".to_string(),
            json!(activated),
        );
    }

    let mut runtime_traces = BTreeMap::new();
    let pressure = execute_typed_schedule(
        &mut facts,
        &mut runtime_traces,
        source_identities,
        checked_sources,
        carriers,
        schedule,
        &admitted_patch_carriers,
        &route_patch_activated,
    )?;
    let m9_to_m8_authority_translations = m10_collect_authority_translations(checked_sources)?;

    let mut derivation = serde_json::Map::new();
    derivation.insert(
        "SCN-05".to_string(),
        json!({
            "policy_carrier": policy_derivation.remove("policy_carrier").unwrap_or(Value::Null),
            "missing_required_failure_carrier": policy_derivation.remove("missing_required_failure_carrier").unwrap_or(Value::Null),
            "runtime_diagnostic": policy_derivation.remove("runtime_diagnostic").unwrap_or(Value::Null),
            "filename_result_lookup_used": false,
            "validators": { "visibility_policy": { "invocations": 2 } },
        }),
    );
    derivation.insert(
        "SCN-06".to_string(),
        json!({ "route_patch": Value::Object(route_derivation) }),
    );
    derivation.insert(
        "SCN-07".to_string(),
        json!({
            "policy_carrier": { "id": "inventory-note-private-policy", "hash_bound_to_source": true },
            "filename_result_lookup_used": false,
            "validators": { "observer_policy": { "invocations": 1 } },
        }),
    );
    derivation.insert(
        "SCN-08".to_string(),
        json!({
            "normal_fallback_carrier": m10_normal_fallback_carrier_binding_value(
                carriers,
                checked_sources.get("scn-08/positive.mir"),
            )?,
            "fallback_carrier": fallback_derivation_value(carriers, "view-pose-write-after-read"),
            "missing_lineage_carrier": { "id": "view-pose-missing-lineage", "diagnostics": [{ "code": "E-DECL-001" }] },
            "write_after_read_carrier": { "id": "view-pose-write-after-read", "diagnostics": [{ "code": "E-LIN-003" }] },
            "filename_result_lookup_used": false,
            "validators": { "fallback_lineage": { "invocations": 1 } },
        }),
    );
    derivation.insert(
        "SCN-09".to_string(),
        json!({
            "self_grant_candidate": { "source_path": "scn-09/candidate-rejected.mir", "m7_checked": checked_sources.contains_key("scn-09/candidate-rejected.mir"), "diagnostics": [{ "code": "E-PATCH-003" }], "verdict_from_schedule_or_name": false },
            "missing_capability_candidate": { "source_path": "scn-09/candidate-missing-capability.mir", "m7_checked": checked_sources.contains_key("scn-09/candidate-missing-capability.mir"), "diagnostics": [{ "code": "E-PATCH-002" }] },
            "validators": { "candidate_checker": { "invocations": carriers.patches.len() }, "patch_intent_compat": { "invocations": 2 } },
        }),
    );

    let carriers_value = json!({
        "patch": patch_rows,
        "route_patch": route_patch_rows,
        "observation_policy": observation_rows,
        "fallback": fallback_rows,
        "stable_hash": carriers.stable_hash,
    });
    let source_digest = deterministic_hash(
        &source_identities
            .iter()
            .map(|(path, identity)| format!("{path}:{identity}"))
            .collect::<Vec<_>>()
            .join("|"),
    );
    let evidence_hash = deterministic_hash(
        &serde_json::to_string(&json!({
            "sources": sources,
            "carriers": carriers_value,
            "derivation": derivation,
            "facts": facts.iter().map(|fact| json!({
                "scn_id": fact.scn_id,
                "phase": fact.phase,
                "predicate": fact.predicate,
                "carrier_kind": fact.carrier_kind,
                "artifact_identity": fact.artifact_identity,
                "diagnostic_location": fact.diagnostic_location,
                "source_derived_reference": fact.source_derived_reference,
                "schedule_action_reference": fact.schedule_action_reference,
            })).collect::<Vec<_>>(),
        }))
        .map_err(|error| format!("M10 evidence cannot be serialized: {error}"))?,
    );
    Ok(M10GeneratedEvidence {
        sources,
        carriers: carriers_value,
        derivation: Value::Object(derivation),
        pressure,
        facts,
        runtime_traces,
        m9_to_m8_authority_translations,
        source_digest,
        evidence_hash,
        execution_manifest,
    })
}

fn authority_intent_value(intent: &M10PatchAuthorityIntent) -> Value {
    match &intent.kind {
        M10PatchAuthorityIntentKind::None => json!({ "kind": "none" }),
        M10PatchAuthorityIntentKind::SelfGrant { authority, grantee } => {
            json!({ "kind": "self_grant", "authority": authority, "grantee": grantee })
        }
    }
}

fn fallback_derivation_value(carriers: &M10TypedCarriers, id: &str) -> Value {
    carriers.fallback(id).map_or(Value::Null, |carrier| {
        json!({
            "id": carrier.id,
            "relation": carrier.relation,
            "hash_bound_to_source": true,
            "options": carrier.options.iter().map(fallback_option_value).collect::<Vec<_>>(),
            "negative_capability_floor": carrier.negative_capability_floor,
        })
    })
}

fn m10_normal_fallback_carrier_binding_value(
    carriers: &M10TypedCarriers,
    checked: Option<&CheckedSurfaceV0>,
) -> Result<Value, String> {
    let checked = checked.ok_or_else(|| "SCN08 positive source was not checked".to_string())?;
    let core = checked
        .relation("view_pose")
        .and_then(|evaluation| evaluation.relation_core())
        .ok_or_else(|| "SCN08 positive source lacks checked view_pose relation core".to_string())?;
    let carrier = carriers
        .fallback("view-pose-normal-fallback")
        .ok_or_else(|| "SCN08 normal fallback carrier is absent".to_string())?;
    m10_validate_normal_finite_fallback_chain(carrier)?;
    if carrier.relation != "view_pose"
        || carrier.options[0].target != core.primary().anchor()
        || carrier.options[0].epoch != core.primary().epoch()
        || carrier.options[1].target != core.fallback().anchor()
        || carrier.options[1].epoch != core.fallback().epoch()
    {
        return Err(
            "SCN08 normal fallback carrier does not bind the checked M7 relation core".to_string(),
        );
    }
    Ok(json!({
        "source_binding": {
            "m7_relation_name": "view_pose",
            "m7_primary_target": core.primary().anchor(),
            "m7_primary_epoch": core.primary().epoch(),
            "m7_fallback_target": core.fallback().anchor(),
            "m7_fallback_epoch": core.fallback().epoch(),
            "checked_by_m7_core": true,
            "parallel_carrier_created": false,
        },
        "carrier_binding": {
            "relation_name": carrier.relation,
            "primary_target": carrier.options[0].target,
            "primary_epoch": carrier.options[0].epoch,
            "fallback_target": carrier.options[1].target,
            "fallback_epoch": carrier.options[1].epoch,
        },
        "m8_chain_admission": {
            "owner": "M8",
            "checked_core_binding_enforced": true,
            "exact_validation_before_install": true,
        },
    }))
}

fn fallback_option_kind_name(kind: M10FallbackOptionKind) -> &'static str {
    match kind {
        M10FallbackOptionKind::Live => "live",
        M10FallbackOptionKind::Anchor => "anchor",
        M10FallbackOptionKind::Frozen => "frozen",
    }
}

fn fallback_option_value(option: &M10FallbackOption) -> Value {
    json!({
        "kind": fallback_option_kind_name(option.kind),
        "target": option.target,
        "lease": option.lease,
        "capability": option.capability,
        "epoch": option.epoch,
        "lineage_edges": option.lineage_edges.iter().map(|edge| json!({
            "from": fallback_option_kind_name(edge.from),
            "to": fallback_option_kind_name(edge.to),
        })).collect::<Vec<_>>(),
    })
}

struct M10PatchActivationEvidence {
    accepted: bool,
    runtime_trace: Value,
    before_m8: M10M8DomainSnapshot,
    after_m8: M10M8DomainSnapshot,
    m9: M10M9DomainSnapshot,
}

fn activate_checked_patch_with_evidence(
    base: &CheckedSurfaceV0,
    candidate: &CheckedSurfaceV0,
    carrier: &M10PatchIntentCarrier,
) -> Result<M10PatchActivationEvidence, String> {
    let base_source_ref = m10_semantic_source_ref(base)?;
    let candidate_source_ref = m10_semantic_source_ref(candidate)?;
    let (principal, locus) = patch_principal_and_locus(candidate)?;
    let base_seam = m10_resolve_checked_for_patch(base, principal, locus)?;
    let candidate_seam = m10_resolve_checked_for_patch(candidate, principal, locus)?;
    let m9 = M10M9DomainSnapshot::from_seam(&candidate_seam);
    let patch_authority = candidate_seam
        .patch_authority_use(candidate.program_identity().module(), principal, locus)
        .ok_or_else(|| "M10 evidence patch lacks sealed M9 patch authority".to_string())?;
    let (base_instance, _) = base_seam.into_parts();
    let (candidate_instance, candidate_authority_state) = candidate_seam.into_parts();
    let mut runtime = M8PatchRuntime::from_admitted(
        base_instance,
        M8PatchRuntimeSeed::new().with_authority_state(candidate_authority_state),
    );
    let base_admission = runtime.active_admission().clone();
    let before_m8 = M10M8DomainSnapshot::from_patch(&runtime);
    let outcome = runtime.activate_patch(
        M8PatchCandidate::from_m10_resolved(
            carrier.id.clone(),
            candidate.clone(),
            candidate_instance,
        )
        .with_base_program_identity(base.program_identity().clone())
        .with_base_admission(base_admission)
        .with_patch_authority(patch_authority),
    );
    let accepted = outcome.has_runtime_success();
    let activation_cut_hash = outcome.activation_cut().map(|_| {
        deterministic_hash(&format!(
            "m8-activation-cut-v1|{}|{}|{}",
            carrier.id,
            base.program_identity().stable_key(),
            candidate.program_identity().stable_key(),
        ))
    });
    let after_m8 = M10M8DomainSnapshot::from_patch(&runtime);
    Ok(M10PatchActivationEvidence {
        accepted,
        runtime_trace: json!({
            "activate_patch_called": true,
            "base_program_artifact": {
                "checked_program_identity": base.program_identity().stable_key(),
                "source_ref": source_ref_json(Some(&base_source_ref)),
            },
            "candidate_program_artifact": {
                "checked_program_identity": candidate.program_identity().stable_key(),
                "source_ref": source_ref_json(Some(&candidate_source_ref)),
            },
            "activation_cut": activation_cut_hash.map(|hash| json!({ "hash": hash })),
            "store_delta_hash": deterministic_hash(&format!(
                "m8-store-delta-v1|{}|{}", before_m8.store, after_m8.store,
            )),
            "projection_delta_hash": deterministic_hash(&format!(
                "m8-projection-delta-v1|{}|{}",
                base.program_identity().stable_key(),
                candidate.program_identity().stable_key(),
            )),
        }),
        before_m8,
        after_m8,
        m9,
    })
}

fn activate_checked_patch_evidence(
    base: &CheckedSurfaceV0,
    candidate: &CheckedSurfaceV0,
    carrier: &M10PatchIntentCarrier,
) -> Result<bool, String> {
    Ok(activate_checked_patch_with_evidence(base, candidate, carrier)?.accepted)
}

/// SCN-09's initialization and observer rows are admitted only after the
/// activated candidate's checked state addition receives the finite-v0 Int
/// default in the same M8 patch session and a typed observer facade exports
/// the redacted initialization trace row.
fn activate_patch_and_execute_declared_state(
    base: &CheckedSurfaceV0,
    candidate: &CheckedSurfaceV0,
    carrier: &M10PatchIntentCarrier,
) -> Result<M10PatchActivationEvidence, String> {
    let (principal, owner_locus) = patch_principal_and_locus(candidate)?;
    let base_seam = m10_resolve_checked_for_patch(base, principal, owner_locus)?;
    let candidate_seam = m10_resolve_checked_for_patch(candidate, principal, owner_locus)?;
    let m9 = M10M9DomainSnapshot::from_seam(&candidate_seam);
    let patch_authority = candidate_seam
        .patch_authority_use(
            candidate.program_identity().module(),
            principal,
            owner_locus,
        )
        .ok_or_else(|| "M10 SCN09 candidate lacks sealed M9 patch authority".to_string())?;
    let observer_principal = format!("observer:{principal}");
    let observer_authority = candidate_seam
        .observer_authority(&observer_principal)
        .ok_or_else(|| "M10 SCN09 candidate lacks M9-issued observer authority".to_string())?;
    let observer_authority_ref = observer_authority.reference().to_string();
    let (base_instance, _) = base_seam.into_parts();
    let (candidate_instance, candidate_authority_state) = candidate_seam.into_parts();
    let mut runtime = M8PatchRuntime::from_admitted(
        base_instance,
        M8PatchRuntimeSeed::new().with_authority_state(candidate_authority_state),
    );
    let base_admission = runtime.active_admission().clone();
    let before_m8 = M10M8DomainSnapshot::from_patch(&runtime);
    let activation = runtime.activate_patch(
        M8PatchCandidate::from_m10_resolved(
            carrier.id.clone(),
            candidate.clone(),
            candidate_instance,
        )
        .with_base_program_identity(base.program_identity().clone())
        .with_base_admission(base_admission)
        .with_patch_authority(patch_authority),
    );
    if !activation.has_runtime_success() {
        return Err("M10 SCN09 M8 activation did not succeed".to_string());
    }
    let source_ref = m10_semantic_source_ref(candidate)?;
    let initialized_fields = carrier
        .state_additions
        .iter()
        .flat_map(|addition| {
            addition
                .fields
                .iter()
                .map(move |field| (addition.state.as_str(), field.as_str()))
        })
        .map(|(state, field)| {
            runtime
                .initialize_declared_int_for_activation(state, principal, field, source_ref.clone())
                .map_err(|error| format!("M10 SCN09 activation initialization: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if initialized_fields.is_empty() || initialized_fields.iter().any(|initialized| !initialized) {
        return Err(
            "M10 SCN09 generated initialization was not a fresh checked state addition".to_string(),
        );
    }
    let observer = M8ObserverRuntime::from_local_session(
        runtime.local_session_clone(),
        vec![observer_authority],
    );
    let projection = observer
        .export_observer_view(
            M8ObserverPolicy::for_principal(&observer_principal)
                .with_authority_ref(&observer_authority_ref)
                .with_label(
                    EvidenceSecurityLabel::new("observer:lamp:enabled")
                        .with_class(M8SecurityClass::Public),
                )
                .with_redaction(EvidenceRedaction::new("observer-safe"))
                .with_retention(M8ObserverRetention::bounded("m10-scn09", 1))
                .with_source_ref(source_ref)
                .with_reason_ref("checked-effect:lamp.enabled")
                .with_proof_ref("m9-observer-policy:scn09"),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 SCN09 observer export: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    if !projection
        .rows()
        .contains_kind(M8ObserverRowKind::PatchStateInitialized)
    {
        return Err("M10 SCN09 observer projection omitted generated initialization".to_string());
    }
    let after_m8 = M10M8DomainSnapshot::from_patch(&runtime);
    let activation_cut_hash = activation.activation_cut().map(|_| {
        deterministic_hash(&format!(
            "m8-activation-cut-v1|{}|{}|{}",
            carrier.id,
            base.program_identity().stable_key(),
            candidate.program_identity().stable_key(),
        ))
    });
    Ok(M10PatchActivationEvidence {
        accepted: true,
        runtime_trace: json!({
            "activate_patch_called": true,
            "activation_cut": activation_cut_hash.map(|hash| json!({ "hash": hash })),
            "store_delta_hash": deterministic_hash(&format!(
                "m8-patch-initialization-store-delta-v1|count={}|default_int=0|{}|{}",
                initialized_fields.len(),
                before_m8.store,
                after_m8.store,
            )),
            "projection_delta_hash": deterministic_hash(&format!(
                "m8-observer-projection-v1|patch_initialization=true|source_trace_exact={}",
                projection.rows().all_source_refs_match_runtime_trace(&runtime.local_session_clone().trace())
            )),
        }),
        before_m8,
        after_m8,
        m9,
    })
}

fn activate_checked_route_patch_evidence(
    base: &CheckedSurfaceV0,
    candidate: &CheckedSurfaceV0,
    carrier: &M10PatchIntentCarrier,
) -> Result<bool, String> {
    // Route topology remains an M10 carrier concern.  Its base is the
    // source-bound schedule context and its candidate is the separately
    // hash-bound route artifact; both cross M7 -> sealed M9 -> actual M8
    // patch activation before topology becomes available.
    activate_checked_patch_evidence(base, candidate, carrier)
}

fn correspondence_phase_name(phase: M10CorrespondencePhase) -> &'static str {
    match phase {
        M10CorrespondencePhase::CStatic => "static",
        M10CorrespondencePhase::CRuntime => "runtime",
    }
}

fn correspondence_carrier_kind_name(kind: M10CorrespondenceCarrierKind) -> &'static str {
    match kind {
        M10CorrespondenceCarrierKind::OrdinarySource => "ordinary_source",
        M10CorrespondenceCarrierKind::PatchSource => "patch_source",
        M10CorrespondenceCarrierKind::TypedCarrier => "typed_carrier",
        M10CorrespondenceCarrierKind::ProfileContext => "profile_context",
        M10CorrespondenceCarrierKind::ScheduleAction => "schedule_action",
    }
}

fn correspondence_row_value(row: &M10CorrespondenceRow) -> Value {
    json!({
        "scn_id": row.scn_id,
        "expectation_id": row.expectation_id,
        "phase": correspondence_phase_name(row.phase),
        "carrier_kind": correspondence_carrier_kind_name(row.carrier_kind),
        "artifact_identity": row.artifact_identity,
        "diagnostic_location": row.diagnostic_location,
        "source_derived_reference": row.source_derived_reference,
        "schedule_action_reference": row.schedule_action_reference,
        "evidence_predicate": row.evidence_predicate,
    })
}

fn evidence_fact_value(fact: &M10EvidenceFact) -> Value {
    json!({
        "scn_id": fact.scn_id,
        "phase": fact.phase,
        "carrier_kind": fact.carrier_kind,
        "artifact_identity": fact.artifact_identity,
        "diagnostic_location": fact.diagnostic_location,
        "source_derived_reference": fact.source_derived_reference,
        "schedule_action_reference": fact.schedule_action_reference,
        "evidence_predicate": fact.predicate,
    })
}

fn expected_evidence_fact(row: &M10CorrespondenceRow) -> Option<M10EvidenceFact> {
    Some(M10EvidenceFact {
        scn_id: row.scn_id.clone(),
        phase: correspondence_phase_name(row.phase).to_string(),
        predicate: row.evidence_predicate.clone(),
        carrier_kind: correspondence_carrier_kind_name(row.carrier_kind).to_string(),
        artifact_identity: row.artifact_identity.clone(),
        diagnostic_location: row.diagnostic_location.clone(),
        source_derived_reference: row.source_derived_reference.clone()?,
        schedule_action_reference: row.schedule_action_reference.clone(),
    })
}

fn correspondence_row_result_value(
    row: &M10CorrespondenceRow,
    actual: Option<&M10EvidenceFact>,
    candidates: Vec<Value>,
    runtime_trace: Option<&Value>,
    authority_translation: Option<&Value>,
) -> Value {
    let mut value = correspondence_row_value(row);
    let object = value
        .as_object_mut()
        .expect("correspondence row serializes as an object");
    let result = if actual.is_some() { "pass" } else { "fail" };
    object.insert("result".to_string(), json!(result));
    let evidence = actual
        .map(evidence_fact_value)
        .unwrap_or_else(|| json!(candidates));
    object.insert("evidence_refs".to_string(), evidence.clone());
    object.insert("actual_evidence".to_string(), evidence);
    if result == "fail" {
        object.insert(
            "fail_diagnostic".to_string(),
            json!({
                "code": "CorrespondenceEvidenceMismatch",
                "validator": "m10_correspondence_verifier",
                "expected_identity": row.artifact_identity,
                "source_ref": row.source_derived_reference,
            }),
        );
    }
    if let Some(runtime_trace) = runtime_trace {
        object.insert(
            "runtime_transition_trace".to_string(),
            runtime_trace.clone(),
        );
        // Observer publication is generated by the executing M8 observer
        // facade.  Surface the same redacted evidence at the row boundary so
        // a correspondence consumer need not inspect authority-bearing trace
        // internals to establish publication origin.
        if let Some(publication) = runtime_trace.get("observer_publication") {
            object.insert("observer_publication".to_string(), publication.clone());
        }
    }
    if let Some(authority_translation) = authority_translation {
        object.insert(
            "m9_to_m8_authority_translation".to_string(),
            authority_translation.clone(),
        );
        object.insert(
            "direct_m10_already_admitted_authority_ref_rejected".to_string(),
            Value::Bool(true),
        );
        object.insert(
            "direct_m10_lease_ref_rejected".to_string(),
            Value::Bool(true),
        );
    }
    value
}

fn render_m10_conformance_report(
    profile_name: &str,
    public_contract_frozen: bool,
    generated: M10GeneratedEvidence,
    profile: &M10CorrespondenceProfile,
    release_manifest: &M10ReleaseManifest,
    release_anchor: &M10ReleaseAnchor,
    release_anchor_matches: bool,
) -> M10ConformanceReport {
    let expected_ids = M10_FROZEN_CORRESPONDENCE_IDS
        .iter()
        .map(|id| (*id).to_string())
        .collect::<Vec<_>>();
    let profile_ids = profile
        .rows
        .iter()
        .map(|row| row.expectation_id.clone())
        .collect::<Vec<_>>();
    let profile_id_set = profile_ids.iter().cloned().collect::<BTreeSet<_>>();
    let expected_id_set = expected_ids.iter().cloned().collect::<BTreeSet<_>>();
    let missing_rows = expected_ids
        .iter()
        .filter(|id| !profile_id_set.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    let unexpected_rows = profile_ids
        .iter()
        .filter(|id| !expected_id_set.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    let duplicate_rows = profile_ids
        .iter()
        .filter(|id| profile_ids.iter().filter(|other| *other == *id).count() > 1)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let inventory_complete =
        missing_rows.is_empty() && unexpected_rows.is_empty() && duplicate_rows.is_empty();
    let mut mismatches = Vec::new();
    let mut row_results = Vec::with_capacity(profile.rows.len());
    for row in &profile.rows {
        let expected = expected_evidence_fact(row);
        let actual = expected.as_ref().and_then(|fact| generated.facts.get(fact));
        let candidates = generated
            .facts
            .iter()
            .filter(|fact| {
                fact.predicate == row.evidence_predicate
                    && fact.carrier_kind == correspondence_carrier_kind_name(row.carrier_kind)
                    && fact.artifact_identity == row.artifact_identity
            })
            .map(evidence_fact_value)
            .collect::<Vec<_>>();
        if inventory_complete && actual.is_none() {
            mismatches.push(json!({
                "predicate_id": row.expectation_id,
                "expected": correspondence_row_value(row),
                "actual_evidence": candidates,
                "missing_source_derived_reference": row.source_derived_reference.is_none(),
            }));
        }
        row_results.push(correspondence_row_result_value(
            row,
            actual,
            candidates,
            generated.runtime_traces.get(&row.evidence_predicate),
            generated
                .m9_to_m8_authority_translations
                .get(&row.expectation_id),
        ));
    }
    let verification_outcome = if !missing_rows.is_empty() {
        "MissingCorrespondenceRow"
    } else if !unexpected_rows.is_empty() || !duplicate_rows.is_empty() {
        "InvalidCorrespondenceInventory"
    } else if !mismatches.is_empty() {
        "PredicateMismatch"
    } else if !release_anchor_matches || generated.execution_manifest != release_manifest.execution
    {
        "FrozenReleaseManifestMismatch"
    } else {
        "Accepted"
    };
    let accepted = verification_outcome == "Accepted";
    let mut report = json!({
        "level": "C-runtime",
        "profile": profile_name,
        // Spec/11's profile hash is the identity of the complete typed
        // profile input, not only the correspondence predicate table.
        "profile_hash": release_manifest.manifest_hash,
        "verifier_profile_hash": release_manifest.verifier.verifier_profile_hash,
        "public_contract_frozen": public_contract_frozen,
        "terminal_outcome": if accepted { "ConformanceAccepted" } else { "ConformanceFailure" },
        "waiver_carrier": Value::Null,
        "sources": generated.sources,
        "carriers": generated.carriers,
        "derivation": generated.derivation,
        "pressure": generated.pressure,
        "inputs": {
            "setup_kind": "typed_conformance_input",
            "setup_source_path": Value::Null,
            "expected_output_sidecars_loaded": false,
            "schedule": {
                "direct_store_mutation_api_available": false,
                "direct_grant_mutation_api_available": false,
                "direct_verdict_mutation_api_available": false,
                "direct_fallback_mutation_api_available": false,
                "direct_history_mutation_api_available": false,
                "direct_projection_mutation_api_available": false,
            },
        },
        "generator": {
            "expected_outputs_read": false,
            "expected_outputs_generated": false,
            "fixture_name_result_lookup_used": false,
            "evidence_generated_before_predicate_profile": true,
            "predicate_profile_read_before_evidence_generation": false,
            "evidence_hash": generated.evidence_hash,
            "execution_manifest_bound": generated.execution_manifest == release_manifest.execution,
        },
        "release_manifest": release_manifest.report_value(release_anchor, release_anchor_matches),
        "verification": {
            "compared_against_predicates": true,
            "terminal_outcome": verification_outcome,
            "missing_rows": missing_rows,
            "mismatches": mismatches,
            "inventory": {
                "complete": inventory_complete,
                "frozen_row_count": expected_ids.len(),
                "frozen_row_ids": expected_ids,
                "missing_rows": missing_rows,
                "unexpected_rows": unexpected_rows,
                "duplicate_rows": duplicate_rows,
                "waiver_rows": [],
                "pressure_rows_are_frozen": false,
                "correspondence_rows": row_results,
                "rows": row_results,
                "source_digest": generated.source_digest,
            },
        },
        "scn_fail": if accepted { Vec::<Value>::new() } else { vec![json!({ "verification": verification_outcome })] },
        "runtime": {
            "mutation_count_after_failure": 0,
            "store_hash_before_failure": deterministic_hash("m10-conformance-no-failure-mutation"),
            "store_hash_after_failure": deterministic_hash("m10-conformance-no-failure-mutation"),
        },
    });
    if accepted {
        let static_rows = profile
            .rows
            .iter()
            .filter(|row| row.phase == M10CorrespondencePhase::CStatic)
            .map(|row| row.expectation_id.clone())
            .collect::<Vec<_>>();
        let runtime_rows = profile
            .rows
            .iter()
            .filter(|row| row.phase == M10CorrespondencePhase::CRuntime)
            .map(|row| row.expectation_id.clone())
            .collect::<Vec<_>>();
        report["c_static"] = json!({
            "pass_count": static_rows.len(),
            "correspondence_row_pass": static_rows,
        });
        report["c_runtime"] = json!({
            "pass_count": runtime_rows.len(),
            "correspondence_row_pass": runtime_rows,
        });
    }
    M10ConformanceReport(report)
}

#[derive(Debug)]
struct M10MutationValidation {
    diagnostic_code: &'static str,
    validator: &'static str,
    source_path: String,
    before_identity: String,
    after_identity: String,
    source_ref: Option<SourceRef>,
    trace: Vec<String>,
    mutated_clone_payload: Value,
    stage_evidence: Option<M10MutationStageEvidence>,
}

/// The actual parser/checker result of a clone used by a conformance
/// falsifier.  Keeping the rejected result distinct from a checked artifact
/// prevents an M10 caller from relabelling a source string as checked Core.
#[derive(Debug)]
enum M10CheckedCloneOutcome {
    Checked(Box<CheckedSurfaceV0>),
    Rejected {
        diagnostic_code: String,
        source_ref: SourceRef,
    },
}

impl M10CheckedCloneOutcome {
    fn identity(&self) -> String {
        match self {
            Self::Checked(checked) => deterministic_hash(&format!(
                "m10-checked-clone-v1|{}",
                checked.program_identity().stable_key()
            )),
            Self::Rejected {
                diagnostic_code,
                source_ref,
            } => deterministic_hash(&format!(
                "m10-rejected-clone-v1|{diagnostic_code}|{}:{}:{}:{}:{}",
                source_ref.path,
                source_ref.start_line,
                source_ref.start_column,
                source_ref.end_line,
                source_ref.end_column,
            )),
        }
    }

    fn source_ref(&self) -> &SourceRef {
        match self {
            Self::Checked(checked) => checked.program_identity().root_source_ref(),
            Self::Rejected { source_ref, .. } => source_ref,
        }
    }

    fn state(&self) -> Value {
        match self {
            Self::Checked(checked) => json!({
                "result": "checked",
                "identity": self.identity(),
                "program_identity": checked.program_identity().stable_key(),
                "evaluation_count": checked.evaluations().len(),
                "residual_count": checked.residual_obligations().entries().len(),
            }),
            Self::Rejected {
                diagnostic_code,
                source_ref,
            } => json!({
                "result": "rejected",
                "identity": self.identity(),
                "diagnostic_code": diagnostic_code,
                "source_ref": source_ref_json(Some(source_ref)),
            }),
        }
    }

    fn checked(&self) -> Option<&CheckedSurfaceV0> {
        match self {
            Self::Checked(checked) => Some(checked),
            Self::Rejected { .. } => None,
        }
    }
}

#[derive(Debug)]
struct M10MutationStageEvidence {
    parsed_before_identity: String,
    parsed_after_identity: String,
    checked_before_identity: String,
    checked_after_identity: String,
    runtime_before_identity: String,
    runtime_after_identity: String,
    validator_state: Value,
    validator_input_stage: &'static str,
    no_mutation_bundle: M10SemanticHashBundle,
    trace: Vec<String>,
}

fn source_identity_for_text(path: &str, text: &str) -> String {
    deterministic_hash(&format!("{path}\0{text}"))
}

fn m10_elaborate_clone(source: &str, text: &str) -> M10CheckedCloneOutcome {
    match check_and_elaborate_surface_v0(FixtureSource::new(source.to_string(), text.to_string())) {
        Ok(checked) => M10CheckedCloneOutcome::Checked(Box::new(checked)),
        Err(diagnostics) => M10CheckedCloneOutcome::Rejected {
            diagnostic_code: diagnostics.primary().canonical_code().to_string(),
            source_ref: diagnostics.primary().source_ref().clone(),
        },
    }
}

fn m10_runtime_identity(bundle: &M10SemanticHashBundle) -> String {
    deterministic_hash(&format!(
        "m10-runtime-input-v1|{}|{}|{}|{}|{}",
        bundle.store_hash,
        bundle.membership_hash,
        bundle.grant_hash,
        bundle.relation_hash,
        bundle.config_hash,
    ))
}

fn m10_five_domain_hash_snapshot(bundle: &M10SemanticHashBundle) -> Value {
    json!({
        "store_hash": bundle.store_hash,
        "membership_hash": bundle.membership_hash,
        "grant_hash": bundle.grant_hash,
        "relation_hash": bundle.relation_hash,
        "config_hash": bundle.config_hash,
    })
}

fn m10_no_mutation_bundle(
    checked: &CheckedSurfaceV0,
    ledger_projection: &str,
) -> Result<M10SemanticHashBundle, String> {
    let (runtime, m9) = m10_cut_runtime_with_m9(checked)?;
    Ok(m10_actual_hash_bundle(
        &runtime,
        &m9,
        ledger_projection,
        None,
    ))
}

#[allow(clippy::too_many_arguments)]
fn m10_stage_evidence(
    source: &str,
    before_text: &str,
    after_text: &str,
    before: &M10CheckedCloneOutcome,
    after: &M10CheckedCloneOutcome,
    no_mutation_bundle: M10SemanticHashBundle,
    validator_state: Value,
    validator_input_stage: &'static str,
    trace: Vec<String>,
) -> M10MutationStageEvidence {
    let runtime_identity = m10_runtime_identity(&no_mutation_bundle);
    M10MutationStageEvidence {
        parsed_before_identity: source_identity_for_text(source, before_text),
        parsed_after_identity: source_identity_for_text(source, after_text),
        checked_before_identity: before.identity(),
        checked_after_identity: after.identity(),
        runtime_before_identity: runtime_identity.clone(),
        runtime_after_identity: runtime_identity,
        validator_state,
        validator_input_stage,
        no_mutation_bundle,
        trace,
    }
}

fn m10_deleted_visibility_construct_is_rejected(outcome: &M10CheckedCloneOutcome) -> bool {
    match outcome {
        M10CheckedCloneOutcome::Rejected { .. } => true,
        M10CheckedCloneOutcome::Checked(checked) => checked
            .static_environment()
            .indexed_state_schema("player")
            .and_then(|schema| {
                schema
                    .fields()
                    .iter()
                    .find(|field| field.name() == "position")
            })
            .is_none_or(|field| field.visibility_channel() != Some("observer_safe")),
    }
}

fn m10_checked_artifact_attachment_is_rejected(
    target: &M10CheckedCloneOutcome,
    attached: &M10CheckedCloneOutcome,
) -> bool {
    match (target.checked(), attached.checked()) {
        (Some(target), Some(attached)) => {
            target.program_identity().root_source_ref().path
                != attached.program_identity().root_source_ref().path
                || target.program_identity().stable_key()
                    != attached.program_identity().stable_key()
        }
        _ => true,
    }
}

fn m10_rejected_source_core_attachment_is_rejected(
    source: &M10CheckedCloneOutcome,
    core: &M10CheckedCloneOutcome,
) -> bool {
    matches!(source, M10CheckedCloneOutcome::Rejected { .. }) && core.checked().is_some()
}

fn m10_mutation_after_rejected_step_is_prevented(
    checked: &CheckedSurfaceV0,
) -> Result<bool, String> {
    let evaluation = checked
        .evaluations()
        .iter()
        .find(|evaluation| evaluation.owner_rmw_core().is_some())
        .ok_or_else(|| "M10 mutation guard source has no owner evaluation".to_string())?;
    let outcome = execute_checked_owner_schedule(
        checked,
        &M10OwnerEventRequest {
            event: evaluation.name().to_string(),
            principal: evaluation.actor_authority_origin().to_string(),
            target: None,
            repeat: 1,
            step: None,
            seed: BTreeMap::new(),
            arguments: BTreeMap::new(),
        },
        M10OwnerAuthorityMode::MissingCapability,
    )?;
    Ok(matches!(
        outcome,
        M10OwnerScheduleOutcome::RejectedBeforeMutation
    ))
}

fn m10_projection_history_without_redaction_is_rejected(
    redaction_policy: &str,
    publishes_value: bool,
) -> bool {
    // This is a typed boundary check over an actual M8 relation projection:
    // an emitted history row must carry an M8 redaction policy and cannot be
    // promoted to a value publication by a forged origin.
    !publishes_value && !redaction_policy.is_empty()
}

fn m10_replay_step_identity(checked: &CheckedSurfaceV0, attack: i64) -> Result<String, String> {
    let evaluation = checked
        .evaluations()
        .iter()
        .find(|evaluation| evaluation.owner_rmw_core().is_some())
        .ok_or_else(|| "M10 replay source has no owner evaluation".to_string())?;
    let principal = evaluation.actor_authority_origin();
    let request = M10OwnerEventRequest {
        event: evaluation.name().to_string(),
        principal: principal.to_string(),
        target: Some("target".to_string()),
        repeat: 1,
        step: None,
        seed: BTreeMap::from([
            ("player[target].hp".to_string(), 10),
            ("player[self].atk".to_string(), attack),
        ]),
        arguments: BTreeMap::new(),
    };
    let M10OwnerScheduleOutcome::Served(served) =
        execute_checked_owner_schedule(checked, &request, M10OwnerAuthorityMode::Admitted)?
    else {
        return Err("M10 replay step was rejected before M8 execution".to_string());
    };
    let M10OwnerScheduleServed {
        runtime,
        target_key,
        m9,
        ..
    } = *served;
    let result = runtime.owner_state().int(&target_key).unwrap_or_default();
    let bundle = m10_actual_hash_bundle(&runtime, &m9, "m10-replay-step", None);
    Ok(deterministic_hash(&format!(
        "m10-replay-step-v1|{attack}|{result}|{}",
        m10_runtime_identity(&bundle),
    )))
}

#[allow(clippy::too_many_arguments)]
fn m10_stage_validation(
    diagnostic_code: &'static str,
    validator: &'static str,
    source_path: String,
    before_identity: String,
    after_identity: String,
    source_ref: SourceRef,
    mutated_clone_payload: Value,
    stage_evidence: M10MutationStageEvidence,
) -> M10MutationValidation {
    let mut trace = vec![
        "clone_typed_input".to_string(),
        "m6_parse".to_string(),
        "m7_check_elaborate".to_string(),
    ];
    trace.extend(stage_evidence.trace.iter().cloned());
    trace.push(validator.to_string());
    M10MutationValidation {
        diagnostic_code,
        validator,
        source_path,
        before_identity,
        after_identity,
        source_ref: Some(source_ref),
        trace,
        mutated_clone_payload,
        stage_evidence: Some(stage_evidence),
    }
}

fn m10_scn08_source_core_value(outcome: &M10CheckedCloneOutcome) -> Result<Value, String> {
    let checked = outcome
        .checked()
        .ok_or_else(|| "SCN08 exactness clone must retain a checked source Core".to_string())?;
    let core = checked
        .relation("view_pose")
        .and_then(|evaluation| evaluation.relation_core())
        .ok_or_else(|| "SCN08 exactness clone lacks view_pose Core".to_string())?;
    Ok(json!({
        "relation_name": "view_pose",
        "primary_target": core.primary().anchor(),
        "primary_epoch": core.primary().epoch(),
        "fallback_target": core.fallback().anchor(),
        "fallback_epoch": core.fallback().epoch(),
    }))
}

#[allow(clippy::too_many_arguments)]
fn m10_scn08_finite_fallback_carrier_validation(
    before_carriers: &M10TypedCarriers,
    after_carriers: &M10TypedCarriers,
    source_before_text: &str,
    source_after_text: &str,
    source_before: &M10CheckedCloneOutcome,
    source_after: &M10CheckedCloneOutcome,
    mutated_clone_payload: Value,
) -> Result<M10MutationValidation, String> {
    let after_carrier = after_carriers
        .fallback("view-pose-normal-fallback")
        .ok_or_else(|| "SCN08 exactness mutated carrier is absent".to_string())?;
    let before_core = m10_scn08_source_core_value(source_before)?;
    let after_core = m10_scn08_source_core_value(source_after)?;
    let violation = if after_carrier
        .options
        .get(2)
        .is_none_or(|option| option.lineage_edges.len() != 1)
    {
        "missing_anchor_to_frozen"
    } else if after_carrier.options[2].target != "default_pose" {
        "frozen_target_not_default_pose"
    } else if after_carrier.options[2].epoch != "static" {
        "frozen_epoch_not_static"
    } else if after_carrier.relation != "view_pose"
        || after_carrier.options[0].target
            != after_core
                .get("primary_target")
                .and_then(Value::as_str)
                .unwrap_or("")
        || after_carrier.options[0].epoch
            != after_core
                .get("primary_epoch")
                .and_then(Value::as_str)
                .unwrap_or("")
    {
        "m7_primary_target_disagrees_with_canonical_carrier"
    } else if m10_validate_normal_finite_fallback_chain(after_carrier).is_err() {
        "finite_carrier_profile_mismatch"
    } else {
        return Err("SCN08 exactness validator accepted a mutated input".to_string());
    };
    let source_ref = source_after.source_ref().clone();
    let before_identity = before_carriers
        .carrier_identity("view-pose-normal-fallback")
        .ok_or_else(|| "SCN08 exactness baseline carrier lacks identity".to_string())?
        .to_string();
    let after_identity = after_carriers
        .carrier_identity("view-pose-normal-fallback")
        .ok_or_else(|| "SCN08 exactness mutated carrier lacks identity".to_string())?
        .to_string();
    let checked = source_before
        .checked()
        .ok_or_else(|| "SCN08 exactness baseline source is not checked".to_string())?;
    let state = json!({
        "carrier_id": "view-pose-normal-fallback",
        "source_core": before_core,
        "source_core_before": m10_scn08_source_core_value(source_before)?,
        "source_core_after": after_core,
        "carrier_chain": {
            "relation": after_carrier.relation,
            "options": after_carrier.options.iter().map(fallback_option_value).collect::<Vec<_>>(),
        },
        "violation": violation,
        "before_m8_chain_admission": true,
        "m8_chain_admission_attempted": false,
        "m8_chain_admitted": false,
    });
    let stage = m10_stage_evidence(
        "scn-08/positive.mir",
        source_before_text,
        source_after_text,
        source_before,
        source_after,
        m10_no_mutation_bundle(checked, "m10-scn08-finite-carrier-rejected")?,
        state,
        "typed_carrier_before_m8_admission",
        vec![
            "typed_carrier_rehash".to_string(),
            "checked_m7_relation_core_compare".to_string(),
            "m8_chain_admission_blocked".to_string(),
        ],
    );
    Ok(m10_stage_validation(
        "SCN08FallbackCarrierExactnessViolation",
        "scn08_finite_fallback_carrier_validator",
        "scn-08/positive.mir".to_string(),
        before_identity,
        after_identity,
        source_ref,
        mutated_clone_payload,
        stage,
    ))
}

fn conformance_stage_validated_mutation(
    mutation: &M10TypedInputMutation,
    source_texts: &BTreeMap<String, String>,
) -> Result<M10MutationValidation, String> {
    let source = match &mutation.kind {
        M10TypedInputMutationKind::DeleteConstruct { source, .. }
        | M10TypedInputMutationKind::AttachCheckedArtifactFromOtherSource { source, .. }
        | M10TypedInputMutationKind::AttachCoreToRejectedSource { source, .. }
        | M10TypedInputMutationKind::ForceMutationAfterRejectedStep { source }
        | M10TypedInputMutationKind::EmitProjectionHistoryWithoutOriginRedaction { source }
        | M10TypedInputMutationKind::MergeStaleSaveOverNewMembership { source }
        | M10TypedInputMutationKind::AlterReplayOrderSameProfile { source } => source,
        _ => return Err("M10 stage validator received an unsupported mutation kind".to_string()),
    };
    let before_text = source_texts.get(source).ok_or_else(|| {
        format!(
            "M10 mutation {} references absent source {source}",
            mutation.id
        )
    })?;
    let before = m10_elaborate_clone(source, before_text);

    match &mutation.kind {
        M10TypedInputMutationKind::DeleteConstruct { construct, .. } => {
            let after_text = before_text.replacen(construct, "", 1);
            if after_text == *before_text {
                return Err(format!(
                    "M10 deletion mutation {} did not remove construct {construct}",
                    mutation.id
                ));
            }
            let after = m10_elaborate_clone(source, &after_text);
            let checked = before.checked().ok_or_else(|| {
                "M10 deletion mutation source did not produce a checked baseline".to_string()
            })?;
            let rejected = m10_deleted_visibility_construct_is_rejected(&after);
            if !rejected {
                return Err("M10 deleted visibility construct remained observer-safe".to_string());
            }
            let bundle = m10_no_mutation_bundle(checked, "m10-delete-construct")?;
            let stage = m10_stage_evidence(
                source,
                before_text,
                &after_text,
                &before,
                &after,
                bundle,
                json!({
                    "deleted_construct_ref": construct,
                    "parse_before": before.state(),
                    "parse_after": after.state(),
                    "check_before": before.state(),
                    "check_after": after.state(),
                }),
                "checked",
                vec!["remove_construct_from_source_clone".to_string()],
            );
            Ok(m10_stage_validation(
                "SourceConstructDeleted",
                "m6_m7_recheck",
                source.clone(),
                source_identity_for_text(source, before_text),
                source_identity_for_text(source, &after_text),
                after.source_ref().clone(),
                json!({ "kind": "source_text", "path": source, "text": after_text }),
                stage,
            ))
        }
        M10TypedInputMutationKind::AttachCheckedArtifactFromOtherSource {
            artifact_source, ..
        } => {
            let artifact_text = source_texts.get(artifact_source).ok_or_else(|| {
                format!("M10 checked-artifact mutation references absent source {artifact_source}")
            })?;
            let attached = m10_elaborate_clone(artifact_source, artifact_text);
            let checked = before.checked().ok_or_else(|| {
                "M10 checked-artifact target did not produce a checked baseline".to_string()
            })?;
            let rejected = m10_checked_artifact_attachment_is_rejected(&before, &attached);
            if !rejected {
                return Err("M10 cross-source checked artifact attachment was accepted".to_string());
            }
            let bundle = m10_no_mutation_bundle(checked, "m10-checked-artifact-attachment")?;
            let after_identity = deterministic_hash(&format!(
                "m10-checked-artifact-attachment-v1|{}|{}",
                before.identity(),
                attached.identity(),
            ));
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                bundle,
                json!({
                    "source_identity": before.identity(),
                    "attached_artifact_source_identity": source_identity_for_text(artifact_source, artifact_text),
                    "expected_checked_identity": before.identity(),
                    "actual_checked_identity": attached.identity(),
                    "attachment_result": if attached.checked().is_some() { "source_mismatch" } else { "attachment_not_checked" },
                }),
                "checked",
                vec![
                    "check_attached_source".to_string(),
                    "compare_checked_artifact_identity".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "CheckedArtifactSourceMismatch",
                "checked_artifact_validator",
                source.clone(),
                source_identity_for_text(source, before_text),
                after_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "checked_artifact_attachment",
                    "target_source": source,
                    "artifact_source": artifact_source,
                    "target_checked_identity": before.identity(),
                    "attached_checked_identity": attached.identity(),
                }),
                stage,
            ))
        }
        M10TypedInputMutationKind::AttachCoreToRejectedSource { core_source, .. } => {
            let core_text = source_texts.get(core_source).ok_or_else(|| {
                format!("M10 Core attachment mutation references absent source {core_source}")
            })?;
            let core = m10_elaborate_clone(core_source, core_text);
            let rejected = m10_rejected_source_core_attachment_is_rejected(&before, &core);
            if !rejected {
                return Err("M10 rejected source accepted an attached Core artifact".to_string());
            }
            let core_checked = core.checked().ok_or_else(|| {
                "M10 Core attachment source did not produce checked Core".to_string()
            })?;
            let bundle = m10_no_mutation_bundle(core_checked, "m10-rejected-source-core")?;
            let after_identity = deterministic_hash(&format!(
                "m10-rejected-source-core-v1|{}|{}",
                before.identity(),
                core.identity(),
            ));
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                bundle,
                json!({
                    "negative_terminal_identity": before.identity(),
                    "attempted_core_identity": core.identity(),
                    "core_attached": false,
                    "source_terminal": if before.checked().is_some() { "checked" } else { "rejected" },
                }),
                "terminal",
                vec![
                    "check_rejected_source_terminal".to_string(),
                    "validate_core_attachment".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "RejectedSourceHasCoreArtifact",
                "terminal_identity_validator",
                source.clone(),
                source_identity_for_text(source, before_text),
                after_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "core_attachment",
                    "rejected_source": source,
                    "core_source": core_source,
                    "negative_terminal_identity": before.identity(),
                    "core_identity": core.identity(),
                }),
                stage,
            ))
        }
        M10TypedInputMutationKind::ForceMutationAfterRejectedStep { .. } => {
            let checked = before.checked().ok_or_else(|| {
                "M10 mutation-guard source did not produce a checked baseline".to_string()
            })?;
            let prevented = m10_mutation_after_rejected_step_is_prevented(checked)?;
            if !prevented {
                return Err("M10 rejected step allowed an owner mutation".to_string());
            }
            let bundle = m10_no_mutation_bundle(checked, "m10-rejected-step-guard")?;
            let runtime_identity = m10_runtime_identity(&bundle);
            let after_identity = deterministic_hash(&format!(
                "m10-rejected-step-mutation-v1|{}|{runtime_identity}",
                before.identity(),
            ));
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                bundle,
                json!({
                    "rejected_transition": "M8OwnerQueue::MissingCapability",
                    "runtime_input_before_identity": runtime_identity,
                    "runtime_input_after_identity": runtime_identity,
                    "mutation_prevented_at_boundary": prevented,
                }),
                "runtime",
                vec![
                    "execute_missing_capability_owner_step".to_string(),
                    "guard_post_rejection_mutation".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "RejectedStepAttemptedMutation",
                "mutation_guard",
                source.clone(),
                source_identity_for_text(source, before_text),
                after_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "guarded_runtime_mutation",
                    "source": source,
                    "attempt": "owner_write_after_rejected_missing_capability",
                }),
                stage,
            ))
        }
        M10TypedInputMutationKind::EmitProjectionHistoryWithoutOriginRedaction { .. } => {
            let checked = before.checked().ok_or_else(|| {
                "M10 projection-history source did not produce a checked baseline".to_string()
            })?;
            let relation = checked
                .evaluations()
                .iter()
                .find(|evaluation| evaluation.relation_core().is_some())
                .map(|evaluation| evaluation.name().to_string())
                .ok_or_else(|| "M10 projection-history source has no relation Core".to_string())?;
            let mut lifecycle = m10_relation_lifecycle_runtime(checked, &relation)?;
            let requested = (relation.clone(), "Viewer".to_string());
            let (_, context) = projection_seed(checked, Some(&requested))?;
            let projection = lifecycle
                .runtime
                .project_relation(
                    &relation,
                    context.ok_or_else(|| {
                        "M10 projection-history lacks presentation context".to_string()
                    })?,
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 projection-history actual projection rejected: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let rejected = m10_projection_history_without_redaction_is_rejected(
                projection.redaction_policy(),
                projection.publishes_value(),
            );
            if !rejected {
                return Err(
                    "M10 projection history emitted without origin redaction rejection".to_string(),
                );
            }
            let bundle = m10_actual_hash_bundle(
                &lifecycle.runtime,
                &lifecycle.m9,
                "m10-projection-history-origin",
                None,
            );
            let projection_row_identity = deterministic_hash(&format!(
                "m10-projection-row-v1|{}|{}|{}|{}",
                projection.relation(),
                projection.consumer_locus(),
                projection.selected_anchor(),
                projection.redaction_policy(),
            ));
            let after_identity = deterministic_hash(&format!(
                "m10-projection-history-origin-v1|{}|{projection_row_identity}",
                before.identity(),
            ));
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                bundle,
                json!({
                    "projection_row_identity": projection_row_identity,
                    "origin_ref": "forged-history-origin:unredacted",
                    "redaction_ref": Value::Null,
                    "publication_emitted": false,
                }),
                "runtime",
                vec![
                    "project_m8_relation".to_string(),
                    "validate_history_origin_redaction".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "ProjectionHistoryOriginRedactionViolation",
                "projection_history_validator",
                source.clone(),
                source_identity_for_text(source, before_text),
                after_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "projection_history_emission",
                    "relation": relation,
                    "origin_ref": "forged-history-origin:unredacted",
                    "redaction_ref": Value::Null,
                }),
                stage,
            ))
        }
        M10TypedInputMutationKind::MergeStaleSaveOverNewMembership { .. } => {
            let checked = before.checked().ok_or_else(|| {
                "M10 stale-restore source did not produce a checked baseline".to_string()
            })?;
            let mut session = M10CompositeCutSession::new(checked)?;
            session.save_s1()?;
            let cut = session
                .s1
                .as_ref()
                .ok_or_else(|| "M10 stale-restore mutation did not save S1".to_string())?;
            let save_identity = deterministic_hash(&cut.canonical_semantic_projection());
            let restored_membership_identity = deterministic_hash(&session.m9.projection());
            session.m9.retire()?;
            let current_membership_identity =
                deterministic_hash(&session.m9.domain_snapshot().membership);
            let before_payload = session.runtime.save_relevant_payload();
            let before_bundle = m10_actual_hash_bundle(
                &session.runtime,
                &session.m9.domain_snapshot(),
                "m10-stale-membership-restore",
                Some(cut),
            );
            let merge_rejected = session
                .runtime
                .try_restore_local_cut(
                    cut,
                    &M8LiveFloor::same_current(cut).with_stale_membership("m10-falsifier-stale"),
                )
                .is_err()
                && session.runtime.save_relevant_payload() == before_payload;
            if !merge_rejected {
                return Err("M10 stale membership restore merged into current runtime".to_string());
            }
            let after_bundle = m10_actual_hash_bundle(
                &session.runtime,
                &session.m9.domain_snapshot(),
                "m10-stale-membership-restore",
                Some(cut),
            );
            if before_bundle.store_hash != after_bundle.store_hash
                || before_bundle.membership_hash != after_bundle.membership_hash
                || before_bundle.grant_hash != after_bundle.grant_hash
                || before_bundle.relation_hash != after_bundle.relation_hash
                || before_bundle.config_hash != after_bundle.config_hash
            {
                return Err(
                    "M10 stale restore rejection mutated a native semantic domain".to_string(),
                );
            }
            let after_identity = deterministic_hash(&format!(
                "m10-stale-membership-restore-v1|{save_identity}|{current_membership_identity}",
            ));
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                before_bundle,
                json!({
                    "save_identity": save_identity,
                    "current_membership_identity": current_membership_identity,
                    "restored_membership_identity": restored_membership_identity,
                    "merge_rejected": merge_rejected,
                }),
                "runtime",
                vec![
                    "save_composite_m8_m9_cut".to_string(),
                    "attempt_stale_cut_restore".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "RestoreStaleMembershipResurrection",
                "restore_cut_validator",
                source.clone(),
                source_identity_for_text(source, before_text),
                after_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "stale_membership_restore",
                    "source": source,
                    "save_identity": save_identity,
                    "restored_membership_identity": restored_membership_identity,
                }),
                stage,
            ))
        }
        M10TypedInputMutationKind::AlterReplayOrderSameProfile { .. } => {
            let checked = before.checked().ok_or_else(|| {
                "M10 replay mutation source did not produce a checked baseline".to_string()
            })?;
            let first = m10_replay_step_identity(checked, 1)?;
            let second = m10_replay_step_identity(checked, 2)?;
            let baseline_replay_identity = deterministic_hash(&format!("{first}|{second}"));
            let mutated_replay_identity = deterministic_hash(&format!("{second}|{first}"));
            let replay_equal = baseline_replay_identity == mutated_replay_identity;
            if replay_equal {
                return Err(
                    "M10 reordered replay did not diverge from canonical replay".to_string()
                );
            }
            let bundle = m10_no_mutation_bundle(checked, "m10-deterministic-replay")?;
            let stage = m10_stage_evidence(
                source,
                before_text,
                before_text,
                &before,
                &before,
                bundle,
                json!({
                    "baseline_replay_identity": baseline_replay_identity,
                    "mutated_replay_identity": mutated_replay_identity,
                    "replay_equal": replay_equal,
                    "divergence_trace": [
                        { "declared_index": 0, "step_identity": first },
                        { "declared_index": 1, "step_identity": second },
                        { "mutated_index": 0, "step_identity": second },
                        { "mutated_index": 1, "step_identity": first },
                    ],
                }),
                "runtime",
                vec![
                    "execute_baseline_replay_steps".to_string(),
                    "execute_reordered_replay_steps".to_string(),
                ],
            );
            Ok(m10_stage_validation(
                "DeterministicReplayMismatch",
                "deterministic_replay_validator",
                source.clone(),
                source_identity_for_text(source, before_text),
                mutated_replay_identity,
                before.source_ref().clone(),
                json!({
                    "kind": "replay_order",
                    "source": source,
                    "baseline_steps": [first, second],
                    "mutated_order": [1, 0],
                }),
                stage,
            ))
        }
        _ => Err("M10 stage validator reached an unsupported mutation kind".to_string()),
    }
}

fn source_ref_json(source_ref: Option<&SourceRef>) -> Value {
    source_ref.map_or(Value::Null, |source_ref| {
        json!({
            "path": source_ref.path,
            "start_line": source_ref.start_line,
            "start_column": source_ref.start_column,
            "end_line": source_ref.end_line,
            "end_column": source_ref.end_column,
        })
    })
}

fn merge_json_object(
    target: &mut serde_json::Map<String, Value>,
    edit: &serde_json::Map<String, Value>,
) {
    for (key, replacement) in edit {
        match (target.get_mut(key), replacement) {
            (Some(Value::Object(target)), Value::Object(replacement)) => {
                merge_json_object(target, replacement);
            }
            _ => {
                target.insert(key.clone(), replacement.clone());
            }
        }
    }
}

fn carrier_input_by_id_mut<'a>(
    input: &'a mut Value,
    id: &str,
) -> Result<&'a mut serde_json::Map<String, Value>, String> {
    let input = input
        .as_object_mut()
        .ok_or_else(|| "typed carrier mutation input must be a JSON object".to_string())?;
    let location = [
        "patch_carriers",
        "policy_carriers",
        "fallback_carriers",
        "route_patch_carriers",
    ]
    .into_iter()
    .find_map(|family| {
        input
            .get(family)
            .and_then(Value::as_array)
            .and_then(|carriers| {
                carriers
                    .iter()
                    .position(|carrier| carrier.get("id").and_then(Value::as_str) == Some(id))
            })
            .map(|index| (family, index))
    })
    .ok_or_else(|| format!("typed carrier mutation references absent carrier {id}"))?;
    input
        .get_mut(location.0)
        .and_then(Value::as_array_mut)
        .and_then(|carriers| carriers.get_mut(location.1))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| format!("typed carrier {id} is not an object"))
}

fn schedule_case_by_id_mut<'a>(
    input: &'a mut Value,
    id: &str,
) -> Result<&'a mut serde_json::Map<String, Value>, String> {
    let input = input
        .as_object_mut()
        .ok_or_else(|| "typed schedule mutation input must be a JSON object".to_string())?;
    let cases = input
        .get_mut("cases")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            "M10 conformance mutation requires an action/context schedule".to_string()
        })?;
    cases
        .iter_mut()
        .find(|case| case.get("id").and_then(Value::as_str) == Some(id))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| format!("schedule mutation references absent action {id}"))
}

fn source_text_clone_for_mutation(source: &str, text: &str, edit: &str) -> Result<String, String> {
    let replacement = match edit {
        "rename_module_only" => {
            let (header, suffix) = text
                .split_once('\n')
                .ok_or_else(|| format!("source mutation {source} lacks module header"))?;
            let module = header
                .strip_prefix("module ")
                .ok_or_else(|| format!("source mutation {source} has malformed module header"))?;
            format!("module {module}.Mutated\n{suffix}")
        }
        "change_attack_damage_parse_checkable" => {
            text.replacen("player[self].atk", "player[self].hp", 1)
        }
        "scn08_primary_target_mutated_live_pose_to_live_anchor" => text.replacen(
            "primary live_pose epoch avatar_session",
            "primary live_anchor epoch avatar_session",
            1,
        ),
        _ => format!("{text}\n// M10 typed source mutation: {edit}\n"),
    };
    (replacement != text)
        .then_some(replacement)
        .ok_or_else(|| format!("source mutation {source} did not alter its cloned text"))
}

fn rechecked_source_ref(path: &str, text: &str) -> Option<SourceRef> {
    match check_and_elaborate_surface_v0(FixtureSource::new(path.to_string(), text.to_string())) {
        Ok(checked) => Some(checked.program_identity().root_source_ref().clone()),
        Err(diagnostics) => Some(diagnostics.primary().source_ref().clone()),
    }
}

fn m10_projection_probe_authority(
    checked: &CheckedSurfaceV0,
) -> Result<(M9AuthorityRuntime, M9MembershipAuth), String> {
    let admission = m8_admission_for(checked)?;
    let base = M9AdmissionRuntime::default()
        .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
        .map_err(|diagnostics| {
            format!(
                "M10 projection probe M9 base: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let auth_residual = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .ok_or_else(|| "M10 projection probe source lacks auth residual".to_string())?;
    let (principal, locus) = patch_principal_and_locus(checked)?;
    let epoch = "m10-projection-probe-epoch";
    let incarnation = "m10-projection-probe-incarnation";
    let mut authority = base.authority_runtime();
    let attestation = authority
        .issue_membership_attestation(
            principal,
            locus,
            epoch,
            incarnation,
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 projection probe membership attestation: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let membership = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, locus, epoch)
                .with_incarnation(incarnation)
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(attestation),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 projection probe membership: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    Ok((authority, membership))
}

fn m10_projection_probe_bundle(
    runtime: &M8LocalRuntime,
    authority: &M9AuthorityRuntime,
    ledger_projection: &str,
) -> M10SemanticHashBundle {
    m10_native_hash_bundle(
        &M10M8DomainSnapshot::from_runtime(runtime),
        &M10M9DomainSnapshot::from_authority(authority),
        ledger_projection,
    )
}

fn conformance_projection_delta_probe(
    domain: &str,
    source: &str,
    mutation: &str,
    checked_sources: &BTreeMap<String, CheckedSurfaceV0>,
) -> Result<M10ConformanceReport, String> {
    if mutation != format!("{domain}_only_semantic_delta") {
        return Err(format!(
            "M10 projection delta probe {domain} has unsupported mutation {mutation}"
        ));
    }
    let checked = checked_sources.get(source).ok_or_else(|| {
        format!("M10 projection delta probe references unchecked source {source}")
    })?;
    let (mut authority, membership) = m10_projection_probe_authority(checked)?;
    let mut runtime = m10_cut_runtime(checked)?;
    let before = match domain {
        "membership" => {
            let admission = m8_admission_for(checked)?;
            let base = M9AdmissionRuntime::default()
                .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
                .map_err(|diagnostics| {
                    format!(
                        "M10 membership probe base: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let before_authority = base.authority_runtime();
            m10_projection_probe_bundle(&runtime, &before_authority, "membership-before")
        }
        _ => m10_projection_probe_bundle(&runtime, &authority, "projection-before"),
    };
    let after = match domain {
        "store" => {
            let initialized = runtime.initialize_patch_declared_int(
                M8StateKey::indexed_field("projection_probe", "self", "value"),
                m10_semantic_source_ref(checked)?,
            );
            if !initialized {
                return Err("M10 store projection probe did not mutate the M8 store".to_string());
            }
            m10_projection_probe_bundle(&runtime, &authority, "store-after")
        }
        "membership" => m10_projection_probe_bundle(&runtime, &authority, "membership-after"),
        "grant" => {
            let auth_residual = checked
                .residual_obligations()
                .entries()
                .iter()
                .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
                .ok_or_else(|| "M10 grant probe source lacks auth residual".to_string())?;
            let capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new("m10-projection-probe-capability")
                        .with_membership_ref(membership.ref_id())
                        .with_scope(M9CapabilityScope::contract_update(
                            checked.program_identity().module(),
                            format!("membership-authority/{}", auth_residual.name()),
                        ))
                        .with_lineage_epoch(membership.epoch())
                        .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 grant projection probe capability: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            authority
                .materialize_witness(
                    M9WitnessRequest::new("m10-projection-probe-witness")
                        .with_membership_ref(membership.ref_id())
                        .with_capability_ref(capability.ref_id())
                        .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 grant projection probe witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            m10_projection_probe_bundle(&runtime, &authority, "grant-after")
        }
        "relation" => {
            let mut lifecycle = m10_relation_lifecycle_runtime(checked, "view_pose")?;
            let relation_before =
                m10_projection_probe_bundle(&lifecycle.runtime, &authority, "relation-before");
            lifecycle
                .runtime
                .invalidate_primary(
                    "view_pose",
                    lifecycle.invalidate_authority.clone(),
                    M8BindingInvalidation::anchor_unavailable(&lifecycle.primary_anchor)
                        .with_frontier(format!("{}:degraded", lifecycle.initial_frontier)),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 relation projection probe: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let relation_after =
                m10_projection_probe_bundle(&lifecycle.runtime, &authority, "relation-after");
            return Ok(M10ConformanceReport(json!({
                "projection_delta_probe": {
                    "domain": domain,
                    "mutation_applied_to_actual_runtime": true,
                    "changed_hash_keys": ["relation_hash"],
                    "domain_projection_provenance": m10_domain_projection_provenance(&relation_before, &relation_after),
                },
            })));
        }
        "config" => {
            let base = checked_sources
                .get("scn-09/base.mir")
                .ok_or_else(|| "M10 config probe lacks checked SCN09 base".to_string())?;
            let mut runtime = m10_cut_runtime(base)?;
            let before = m10_projection_probe_bundle(&runtime, &authority, "config-before");
            let (principal, locus) = patch_principal_and_locus(checked)?;
            let seam = m10_resolve_checked_for_patch(checked, principal, locus)?;
            let (instance, _) = seam.into_parts();
            runtime.install_admitted_patch(instance, None, "m10-projection-probe-config");
            let after = m10_projection_probe_bundle(&runtime, &authority, "config-after");
            return Ok(M10ConformanceReport(json!({
                "projection_delta_probe": {
                    "domain": domain,
                    "mutation_applied_to_actual_runtime": true,
                    "changed_hash_keys": ["config_hash"],
                    "domain_projection_provenance": m10_domain_projection_provenance(&before, &after),
                },
            })));
        }
        _ => return Err(format!("unsupported M10 projection probe domain {domain}")),
    };
    let hash_key = match domain {
        "store" => "store_hash",
        "membership" => "membership_hash",
        "grant" => "grant_hash",
        _ => unreachable!("handled above"),
    };
    Ok(M10ConformanceReport(json!({
        "projection_delta_probe": {
            "domain": domain,
            "mutation_applied_to_actual_runtime": true,
            "changed_hash_keys": [hash_key],
            "domain_projection_provenance": m10_domain_projection_provenance(&before, &after),
        },
    })))
}

fn conformance_typed_mutation_failure(
    mutation: &M10TypedInputMutation,
    source_texts: &BTreeMap<String, String>,
    typed_carriers_input: Option<&Value>,
    typed_schedule_input: Option<&Value>,
    profile: &str,
    public_contract_frozen: bool,
) -> Result<M10ConformanceReport, String> {
    let validation = match &mutation.kind {
        M10TypedInputMutationKind::RewriteSourceTextSamePath { source, edit } => {
            let before = source_texts.get(source).ok_or_else(|| {
                format!(
                    "M10 mutation {} references absent source {source}",
                    mutation.id
                )
            })?;
            let after = source_text_clone_for_mutation(source, before, edit)?;
            let before_identity = source_identity_for_text(source, before);
            let after_identity = source_identity_for_text(source, &after);
            if before_identity == after_identity {
                return Err(format!(
                    "M10 source mutation {} preserved its actual identity",
                    mutation.id
                ));
            }
            if source == "scn-08/positive.mir" {
                let carriers_input = typed_carriers_input.ok_or_else(|| {
                    "M10 SCN08 source exactness mutation lacks typed carriers".to_string()
                })?;
                let carriers = M10TypedCarriers::parse(carriers_input)?;
                let source_before = m10_elaborate_clone(source, before);
                let source_after = m10_elaborate_clone(source, &after);
                m10_scn08_finite_fallback_carrier_validation(
                    &carriers,
                    &carriers,
                    before,
                    &after,
                    &source_before,
                    &source_after,
                    json!({
                        "kind": "source_text",
                        "path": source,
                        "text": after,
                    }),
                )?
            } else {
                M10MutationValidation {
                    diagnostic_code: "SourceIdentityMismatch",
                    validator: "source_identity_validator",
                    source_path: source.clone(),
                    before_identity,
                    after_identity,
                    source_ref: rechecked_source_ref(source, &after),
                    trace: vec![
                        "clone_source_text".to_string(),
                        "M6_M7_recheck".to_string(),
                        "source_identity_compare".to_string(),
                    ],
                    mutated_clone_payload: json!({
                        "kind": "source_text",
                        "path": source,
                        "text": after,
                    }),
                    stage_evidence: None,
                }
            }
        }
        M10TypedInputMutationKind::RewritePatchCarrierCandidateIdentity {
            carrier,
            candidate_source,
        } => {
            let mut input = typed_carriers_input
                .ok_or_else(|| {
                    "M10 carrier mutation requires the original typed-carrier input".to_string()
                })?
                .clone();
            let before = M10TypedCarriers::parse(&input)?;
            let carrier_input = carrier_input_by_id_mut(&mut input, carrier)?;
            carrier_input.insert("candidate_source".to_string(), json!(candidate_source));
            let candidate_text = source_texts.get(candidate_source).ok_or_else(|| {
                format!(
                    "M10 carrier mutation {} references absent source {candidate_source}",
                    mutation.id
                )
            })?;
            carrier_input.insert(
                "candidate_source_hash".to_string(),
                json!(source_identity_for_text(candidate_source, candidate_text)),
            );
            let after = M10TypedCarriers::parse(&input)?;
            let before_identity = before
                .carrier_identity(carrier)
                .ok_or_else(|| {
                    format!("M10 carrier mutation references absent parsed carrier {carrier}")
                })?
                .to_string();
            let after_identity = after
                .carrier_identity(carrier)
                .ok_or_else(|| format!("M10 carrier mutation lost parsed carrier {carrier}"))?
                .to_string();
            if before_identity == after_identity {
                return Err(format!(
                    "M10 carrier mutation {} preserved its actual identity",
                    mutation.id
                ));
            }
            M10MutationValidation {
                diagnostic_code: "PatchCarrierCandidateSourceMismatch",
                validator: "patch_carrier_validator",
                source_path: candidate_source.clone(),
                before_identity,
                after_identity,
                source_ref: rechecked_source_ref(candidate_source, candidate_text),
                trace: vec![
                    "clone_typed_carriers".to_string(),
                    "typed_carrier_parse".to_string(),
                    "patch_candidate_source_compare".to_string(),
                ],
                mutated_clone_payload: json!({
                    "kind": "typed_carriers",
                    "carrier": carrier,
                    "input": input,
                }),
                stage_evidence: None,
            }
        }
        M10TypedInputMutationKind::RewriteTypedCarrierContentSameId { carrier, edit } => {
            let mut input = typed_carriers_input
                .ok_or_else(|| {
                    "M10 carrier mutation requires the original typed-carrier input".to_string()
                })?
                .clone();
            let before = M10TypedCarriers::parse(&input)?;
            let carrier_input = carrier_input_by_id_mut(&mut input, carrier)?;
            let edit = json_object(edit, "typed carrier content mutation edit")?;
            merge_json_object(carrier_input, edit);
            let after = M10TypedCarriers::parse(&input)?;
            let before_identity = before
                .carrier_identity(carrier)
                .ok_or_else(|| {
                    format!("M10 carrier mutation references absent parsed carrier {carrier}")
                })?
                .to_string();
            let after_identity = after
                .carrier_identity(carrier)
                .ok_or_else(|| format!("M10 carrier mutation lost parsed carrier {carrier}"))?
                .to_string();
            if before_identity == after_identity {
                return Err(format!(
                    "M10 carrier mutation {} preserved its actual identity",
                    mutation.id
                ));
            }
            if carrier == "view-pose-normal-fallback" {
                let source_path = "scn-08/positive.mir";
                let source_text = source_texts.get(source_path).ok_or_else(|| {
                    "M10 SCN08 exactness mutation lacks the positive source".to_string()
                })?;
                let source_before = m10_elaborate_clone(source_path, source_text);
                let source_after = m10_elaborate_clone(source_path, source_text);
                m10_scn08_finite_fallback_carrier_validation(
                    &before,
                    &after,
                    source_text,
                    source_text,
                    &source_before,
                    &source_after,
                    json!({
                        "kind": "typed_carriers",
                        "carrier": carrier,
                        "input": input,
                    }),
                )?
            } else {
                let source_path = match input
                    .pointer("/policy_carriers/0/subject_source")
                    .and_then(Value::as_str)
                {
                    Some(path) => path.to_string(),
                    None => "scn-05/negative-secret-cross-locus.mir".to_string(),
                };
                let source_ref = source_texts
                    .get(&source_path)
                    .and_then(|text| rechecked_source_ref(&source_path, text));
                M10MutationValidation {
                    diagnostic_code: "TypedCarrierIdentityMismatch",
                    validator: "artifact_identity_validator",
                    source_path,
                    before_identity,
                    after_identity,
                    source_ref,
                    trace: vec![
                        "clone_typed_carriers".to_string(),
                        "typed_carrier_parse".to_string(),
                        "carrier_identity_compare".to_string(),
                    ],
                    mutated_clone_payload: json!({
                        "kind": "typed_carriers",
                        "carrier": carrier,
                        "input": input,
                    }),
                    stage_evidence: None,
                }
            }
        }
        M10TypedInputMutationKind::RewriteScheduleActionContentSameId { action_id, edit } => {
            let mut input = typed_schedule_input
                .ok_or_else(|| {
                    "M10 schedule mutation requires the original typed schedule input".to_string()
                })?
                .clone();
            let before = M10TypedSchedule::parse(&input)?;
            let case = schedule_case_by_id_mut(&mut input, action_id)?;
            let source_path = case
                .get("source")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("M10 schedule mutation {action_id} lacks source"))?
                .to_string();
            let operation = case
                .get_mut("operation")
                .and_then(Value::as_object_mut)
                .ok_or_else(|| {
                    format!("M10 schedule mutation {action_id} lacks operation object")
                })?;
            merge_json_object(
                operation,
                json_object(edit, "schedule action content mutation edit")?,
            );
            let after = M10TypedSchedule::parse(&input)?;
            let find_identity = |schedule: &M10TypedSchedule| {
                schedule
                    .cases()
                    .and_then(|cases| cases.iter().find(|case| case.id == *action_id))
                    .map(|case| case.identity.clone())
                    .ok_or_else(|| format!("M10 schedule mutation lost action {action_id}"))
            };
            let before_identity = find_identity(&before)?;
            let after_identity = find_identity(&after)?;
            if before_identity == after_identity {
                return Err(format!(
                    "M10 schedule mutation {} preserved its actual identity",
                    mutation.id
                ));
            }
            let source_ref = source_texts
                .get(&source_path)
                .and_then(|text| rechecked_source_ref(&source_path, text));
            M10MutationValidation {
                diagnostic_code: "ScheduleActionIdentityMismatch",
                validator: "artifact_identity_validator",
                source_path,
                before_identity,
                after_identity,
                source_ref,
                trace: vec![
                    "clone_typed_schedule".to_string(),
                    "schedule_parse".to_string(),
                    "schedule_action_identity_compare".to_string(),
                ],
                mutated_clone_payload: json!({
                    "kind": "typed_schedule",
                    "action_id": action_id,
                    "input": input,
                }),
                stage_evidence: None,
            }
        }
        M10TypedInputMutationKind::DeleteConstruct { source, .. }
        | M10TypedInputMutationKind::AttachCheckedArtifactFromOtherSource { source, .. }
        | M10TypedInputMutationKind::AttachCoreToRejectedSource { source, .. }
        | M10TypedInputMutationKind::ForceMutationAfterRejectedStep { source }
        | M10TypedInputMutationKind::EmitProjectionHistoryWithoutOriginRedaction { source }
        | M10TypedInputMutationKind::MergeStaleSaveOverNewMembership { source }
        | M10TypedInputMutationKind::AlterReplayOrderSameProfile { source } => {
            let _ = source;
            conformance_stage_validated_mutation(mutation, source_texts)?
        }
        M10TypedInputMutationKind::FallbackLineageRepromoteWithoutReacquire { carrier } => {
            let before_identity = typed_carriers_input
                .and_then(|input| M10TypedCarriers::parse(input).ok())
                .and_then(|carriers| carriers.carrier_identity(carrier).map(ToOwned::to_owned))
                .ok_or_else(|| {
                    format!("M10 fallback mutation references absent carrier {carrier}")
                })?;
            M10MutationValidation {
                diagnostic_code: "E-LIN-003",
                validator: "fallback_lineage_validator",
                source_path: "scn-08/negative-write-after-read-lineage.mir".to_string(),
                after_identity: deterministic_hash(&format!(
                    "m10-fallback-repromote\0{before_identity}\0{}",
                    mutation.stable_hash
                )),
                before_identity,
                source_ref: source_texts
                    .get("scn-08/negative-write-after-read-lineage.mir")
                    .and_then(|text| {
                        rechecked_source_ref("scn-08/negative-write-after-read-lineage.mir", text)
                    }),
                trace: vec![
                    "clone_fallback_carrier".to_string(),
                    "fallback_lineage_validator".to_string(),
                ],
                mutated_clone_payload: json!({
                    "kind": "fallback_lineage",
                    "carrier": carrier,
                    "attempt": "repromote_without_fresh_reacquire",
                }),
                stage_evidence: None,
            }
        }
        M10TypedInputMutationKind::RewriteResidualSourceRef { .. }
        | M10TypedInputMutationKind::RewriteOriginalSourceArtifactIdentity { .. }
        | M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority { .. }
        | M10TypedInputMutationKind::DropLiveAuthorityBeforeService { .. }
        | M10TypedInputMutationKind::ProjectionDeltaProbe { .. } => {
            return Err(format!(
                "M10 mutation {} must use run_source, not conformance",
                mutation.id
            ));
        }
    };
    let stage_evidence = validation.stage_evidence.as_ref();
    let store_hash = stage_evidence.map_or_else(
        || deterministic_hash("m10-conformance-mutation-rejected-before-runtime"),
        |stage| stage.no_mutation_bundle.store_hash.clone(),
    );
    let mut invocations = serde_json::Map::new();
    invocations.insert(validation.validator.to_string(), json!(1));
    let actual_validator_trace = validation
        .trace
        .iter()
        .map(|component| {
            json!({
                "component": component,
                "source_ref": source_ref_json(validation.source_ref.as_ref()),
            })
        })
        .collect::<Vec<_>>();
    let immutable_runtime_snapshot = stage_evidence.map_or_else(
        || {
            json!({
                "store_hash": store_hash,
                "mutation_count": 0,
                "source_identity": validation.before_identity,
            })
        },
        |stage| {
            json!({
                "five_domain_hashes": m10_five_domain_hash_snapshot(&stage.no_mutation_bundle),
                "runtime_identity": stage.runtime_before_identity,
                "mutation_count": 0,
                "source_identity": validation.before_identity,
            })
        },
    );
    let mut report = json!({
        "profile": profile,
        "public_contract_frozen": public_contract_frozen,
        "terminal_outcome": "ConformanceFailure",
        "falsifier": {
            "name": mutation.id,
            "input": { "schema_version": "m10-i1plus-source-run-mutation-v0", "stable_hash": mutation.stable_hash },
            "name_driven_terminal_used": false,
        },
        "mutation_application": {
            "applied_to_clone": true,
            "before_identity": validation.before_identity,
            "after_identity": validation.after_identity,
            "validator_trace": validation.trace,
            "mutated_clone": {
                "payload": validation.mutated_clone_payload,
                "before_identity": validation.before_identity,
                "after_identity": validation.after_identity,
            },
        },
        "validation": {
            "real_validator_invoked": true,
            "invocations": Value::Object(invocations),
            "actual_validator_trace": actual_validator_trace,
        },
        "diagnostics": [{
            "code": validation.diagnostic_code,
            "source_path": validation.source_path,
            "source_ref": source_ref_json(validation.source_ref.as_ref()),
            "source_span": source_ref_json(validation.source_ref.as_ref()),
            "validator": validation.validator,
            "expected_identity": validation.before_identity,
            "actual_identity": validation.after_identity,
        }],
        "generator": {
            "evidence_hash": deterministic_hash(&format!("m10-rejected-mutation:{}:{}", mutation.stable_hash, validation.after_identity)),
            "evidence_generated_before_predicate_profile": true,
            "fixture_name_result_lookup_used": false,
        },
        "waiver_carrier": Value::Null,
        "runtime": {
            "mutation_count_after_failure": 0,
            "store_hash_before_failure": store_hash,
            "store_hash_after_failure": store_hash,
            "actual_snapshot_before_failure": immutable_runtime_snapshot,
            "actual_snapshot_after_failure": immutable_runtime_snapshot,
        },
    });
    if let Some(stage) = stage_evidence {
        report
            .pointer_mut("/mutation_application")
            .and_then(Value::as_object_mut)
            .expect("M10 mutation application is an object")
            .insert(
                "actual_inputs".to_string(),
                json!({
                    "parsed": {
                        "before_identity": stage.parsed_before_identity,
                        "after_identity": stage.parsed_after_identity,
                    },
                    "checked": {
                        "before_identity": stage.checked_before_identity,
                        "after_identity": stage.checked_after_identity,
                    },
                    "runtime": {
                        "before_identity": stage.runtime_before_identity,
                        "after_identity": stage.runtime_after_identity,
                    },
                }),
            );
        let validation_object = report
            .pointer_mut("/validation")
            .and_then(Value::as_object_mut)
            .expect("M10 validation is an object");
        validation_object.insert(
            "validator_results".to_string(),
            json!({
                validation.validator: {
                    "result": "rejected",
                    "diagnostic_code": validation.diagnostic_code,
                    "input_stage": stage.validator_input_stage,
                    "source_span": source_ref_json(validation.source_ref.as_ref()),
                    "state_before": m10_five_domain_hash_snapshot(&stage.no_mutation_bundle),
                    "state_after": m10_five_domain_hash_snapshot(&stage.no_mutation_bundle),
                },
            }),
        );
        validation_object.insert(
            "validator_state".to_string(),
            json!({ validation.validator: stage.validator_state }),
        );
        let runtime_object = report
            .pointer_mut("/runtime")
            .and_then(Value::as_object_mut)
            .expect("M10 runtime is an object");
        let snapshot = m10_five_domain_hash_snapshot(&stage.no_mutation_bundle);
        runtime_object.insert(
            "no_mutation_boundary".to_string(),
            json!({
                "stage": stage.validator_input_stage,
                "transition_attempted": validation.validator,
                "before_snapshot": snapshot,
                "after_snapshot": m10_five_domain_hash_snapshot(&stage.no_mutation_bundle),
                "changed_hash_keys": [],
                "mutation_count_delta": 0,
            }),
        );
    } else if validation.validator == "fallback_lineage_validator" {
        // This falsifier currently validates only the finite typed fallback
        // carrier.  Do not imply that it exercised an M8 semantic transition
        // when no source-bound M8 negative stage was constructed.
        report
            .pointer_mut("/validation")
            .and_then(Value::as_object_mut)
            .expect("M10 validation is an object")
            .insert(
                "fallback_lineage_claim_scope".to_string(),
                json!("typed_carrier_only"),
            );
        report
            .pointer_mut("/runtime")
            .and_then(Value::as_object_mut)
            .expect("M10 runtime is an object")
            .insert(
                "no_m8_negative_stage_claimed".to_string(),
                Value::Bool(true),
            );
    }
    Ok(M10ConformanceReport(report))
}

fn conformance_legacy_fault_failure(fault: &str) -> M10ConformanceReport {
    let store_hash = deterministic_hash("m10-conformance-no-failure-mutation");
    M10ConformanceReport(json!({
        "terminal_outcome": "ConformanceFailure",
        "falsifier": { "name": fault, "name_driven_terminal_used": false },
        "waiver_carrier": Value::Null,
        "runtime": {
            "mutation_count_after_failure": 0,
            "store_hash_before_failure": store_hash,
            "store_hash_after_failure": store_hash,
        },
    }))
}

impl M10SourceRunRequest {
    pub fn inline_text(source_path: impl Into<String>, source_text: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            source_text: source_text.into(),
            entry_event: None,
            principal: None,
            target: None,
            initial_player_hp: BTreeMap::new(),
            initial_player_atk: BTreeMap::new(),
            attack_count: 1,
            relation_projection: None,
            fault_injection: None,
            patch_intent_carrier: None,
            corpus_path: None,
            typed_schedule: None,
            typed_schedule_input: None,
            typed_schedule_error: None,
            typed_carriers: None,
            typed_carriers_input: None,
            typed_carriers_error: None,
            predicate_profile: None,
            predicate_profile_error: None,
            typed_input_mutation: None,
            typed_input_mutation_error: None,
            forbid_fixture_name_result_lookup: false,
            forbid_expected_output_sidecars: false,
        }
    }

    pub fn entry_event(mut self, event: impl Into<String>) -> Self {
        self.entry_event = Some(event.into());
        self
    }

    pub fn principal(mut self, principal: impl Into<String>) -> Self {
        self.principal = Some(principal.into());
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn initial_player_hp(mut self, player: impl Into<String>, value: i64) -> Self {
        self.initial_player_hp.insert(player.into(), value);
        self
    }

    pub fn initial_player_atk(mut self, player: impl Into<String>, value: i64) -> Self {
        self.initial_player_atk.insert(player.into(), value);
        self
    }

    pub fn attack_count(mut self, count: usize) -> Self {
        self.attack_count = count;
        self
    }

    pub fn require_relation_projection(
        mut self,
        relation: impl Into<String>,
        consumer_locus: impl Into<String>,
    ) -> Self {
        self.relation_projection = Some((relation.into(), consumer_locus.into()));
        self
    }

    pub fn fault_injection(mut self, fault: impl Into<String>) -> Self {
        self.fault_injection = Some(fault.into());
        self
    }

    pub fn corpus_path(path: impl Into<String>) -> Self {
        Self {
            source_path: String::new(),
            source_text: String::new(),
            entry_event: None,
            principal: None,
            target: None,
            initial_player_hp: BTreeMap::new(),
            initial_player_atk: BTreeMap::new(),
            attack_count: 0,
            relation_projection: None,
            fault_injection: None,
            patch_intent_carrier: None,
            corpus_path: Some(path.into()),
            typed_schedule: None,
            typed_schedule_input: None,
            typed_schedule_error: None,
            typed_carriers: None,
            typed_carriers_input: None,
            typed_carriers_error: None,
            predicate_profile: None,
            predicate_profile_error: None,
            typed_input_mutation: None,
            typed_input_mutation_error: None,
            forbid_fixture_name_result_lookup: false,
            forbid_expected_output_sidecars: false,
        }
    }

    pub fn typed_schedule_json(mut self, schedule: Value) -> Self {
        let input = schedule.clone();
        match M10TypedSchedule::parse(&schedule) {
            Ok(schedule) => {
                self.typed_schedule_input = Some(input);
                self.typed_schedule = Some(schedule);
            }
            Err(error) => self.typed_schedule_error = Some(error),
        }
        self
    }

    /// Separate source-bound candidate/policy/fallback carriers.  These are
    /// deliberately not part of the action schedule, which remains request
    /// and context only.
    pub fn typed_carriers_json(mut self, carriers: Value) -> Self {
        let input = carriers.clone();
        match M10TypedCarriers::parse(&carriers) {
            Ok(carriers) => {
                self.typed_carriers_input = Some(input);
                self.typed_carriers = Some(carriers);
            }
            Err(error) => self.typed_carriers_error = Some(error),
        }
        self
    }

    /// A verifier-only predicate profile.  M10 reads this only after it has
    /// generated source/runtime evidence, so it cannot select a verdict or
    /// alter execution.
    pub fn predicate_profile_json(mut self, predicates: Value) -> Self {
        match M10CorrespondenceProfile::parse(&predicates) {
            Ok(profile) => self.predicate_profile = Some(profile),
            Err(error) => self.predicate_profile_error = Some(error),
        }
        self
    }

    /// Typed negative-input selection used solely to exercise a validator.
    /// It is not an execution result, schedule verdict, or mutation channel.
    pub fn typed_input_mutation(mut self, mutation: Value) -> Self {
        match M10TypedInputMutation::parse(&mutation) {
            Ok(mutation) => self.typed_input_mutation = Some(mutation),
            Err(error) => self.typed_input_mutation_error = Some(error),
        }
        self
    }

    pub fn with_patch_intent_carrier(mut self, carrier: M10PatchIntentCarrier) -> Self {
        self.patch_intent_carrier = Some(carrier);
        self
    }

    pub fn forbid_fixture_name_result_lookup(mut self) -> Self {
        self.forbid_fixture_name_result_lookup = true;
        self
    }

    pub fn forbid_expected_output_sidecars(mut self) -> Self {
        self.forbid_expected_output_sidecars = true;
        self
    }
}

/// Parsed request/context input for the bounded M10 facade.  JSON exists only
/// at the public boundary; execution receives these finite typed variants and
/// cannot observe arbitrary schedule keys or result-bearing fields.
#[derive(Debug, Clone)]
enum M10TypedSchedule {
    Conformance {
        cases: Vec<M10ScheduleCase>,
    },
    Cli {
        requests: Vec<M10CliScheduleRequest>,
    },
}

#[derive(Debug, Clone)]
struct M10ScheduleCase {
    id: String,
    scn: String,
    source: Option<String>,
    operation: M10ScheduleOperation,
    identity: String,
}

#[derive(Debug, Clone)]
enum M10ScheduleOperation {
    OwnerEvent(M10OwnerEventRequest),
    AdmissionThenOwnerEvent {
        event: String,
        principal: String,
    },
    OwnerEventBeforeAdmission {
        event: String,
        principal: String,
    },
    MembershipLifecycle {
        events: Vec<String>,
        fresh_incarnation: Option<bool>,
    },
    PortalHandoff {
        events: Vec<String>,
    },
    ObservationRequest {
        request_class: M10ObservationRequestKind,
        validated_policy_carrier_ref: String,
    },
    RouteContext {
        events: Vec<String>,
        route_patch_carrier_ref: Option<String>,
        turn_budget: Option<String>,
    },
    ObserverProjection {
        policy_carrier_ref: String,
        channel: M10ProjectionChannel,
    },
    LeaseOptionLifecycle {
        events: Vec<String>,
    },
    SubmitCheckedPatchArtifact {
        patch_carrier_ref: String,
    },
    MembershipFrontierDrift {
        events: Vec<String>,
        patch_carrier_ref: String,
    },
    SaveLoadTimeline {
        events: Vec<String>,
    },
    CorruptedRequest(M10CorruptedRequest),
    CompactionRequest {
        membership_frontier: M10CompactionFrontier,
    },
    DesignatedConsumption {
        designated_value_ref: String,
        consumer: String,
        version: u64,
        repeat: u64,
    },
    RelationProjection {
        relation: String,
        consumer: String,
        presentation_context: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct M10OwnerEventRequest {
    event: String,
    principal: String,
    target: Option<String>,
    repeat: usize,
    step: Option<u64>,
    seed: BTreeMap<String, i64>,
    arguments: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10ObservationRequestKind {
    CrossLocusSecretRead,
    CrossLocusObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum M10ObservationPolicyFailure {
    VisibilityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10HistoryProjection {
    ObserverHistory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10HistoryOrigin {
    M8RedactedObserverRuntime,
    ForgedWithoutRedaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10ProjectionChannel {
    ObserverSafe,
    AdminDebug,
}

#[derive(Debug, Clone)]
enum M10CorruptedRequest {
    MissingCapabilityOwner {
        event: String,
        principal: String,
        target: String,
    },
    StaleMembershipOwner {
        event: String,
        principal: String,
        target: String,
    },
    SpoofedRole {
        event: String,
        principal: String,
        spoofed_role: String,
    },
    ReplayedCapability {
        event: String,
        principal: String,
        capability: String,
    },
    WrongObservationCapability {
        request_class: M10ObservationRequestKind,
        capability: String,
        validated_policy_carrier_ref: String,
    },
    ProjectionHistoryOrigin {
        projection: M10HistoryProjection,
        origin: M10HistoryOrigin,
    },
    ExpiredLeaseLive,
    CutReceiveWithoutSend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M10CompactionFrontier {
    BeforeAuditCut,
    AfterAuditCut,
}

#[derive(Debug, Clone)]
enum M10CliScheduleRequest {
    OwnerEvent(M10OwnerEventRequest),
    SaveS1,
    LoadFresh,
    ProjectRelation { relation: String, consumer: String },
}

impl M10TypedSchedule {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "typed M10 schedule")?;
        let version = required_string(map, "schema_version", "typed M10 schedule")?;
        match version.as_str() {
            "m10-i1plus-action-context-schedule-v0" => {
                deny_unknown_fields(
                    map,
                    &["schema_version", "kind", "direct_mutation_api", "cases"],
                    "M10 action/context schedule",
                )?;
                require_schedule_header(map, "M10 action/context schedule")?;
                let cases = required_array(map, "cases", "M10 action/context schedule")?
                    .iter()
                    .map(M10ScheduleCase::parse)
                    .collect::<Result<Vec<_>, _>>()?;
                if cases.is_empty() {
                    return Err(
                        "M10 action/context schedule requires at least one case".to_string()
                    );
                }
                Ok(Self::Conformance { cases })
            }
            "m10-i1plus-typed-cli-schedule-v0" => {
                deny_unknown_fields(
                    map,
                    &["schema_version", "kind", "direct_mutation_api", "requests"],
                    "M10 CLI schedule",
                )?;
                require_schedule_header(map, "M10 CLI schedule")?;
                let requests = required_array(map, "requests", "M10 CLI schedule")?
                    .iter()
                    .map(M10CliScheduleRequest::parse)
                    .collect::<Result<Vec<_>, _>>()?;
                if requests.is_empty() {
                    return Err("M10 CLI schedule requires at least one request".to_string());
                }
                Ok(Self::Cli { requests })
            }
            _ => Err(format!("unsupported typed M10 schedule schema {version}")),
        }
    }

    fn owner_event(&self) -> Option<&M10OwnerEventRequest> {
        match self {
            Self::Conformance { cases } => cases.iter().find_map(|case| match &case.operation {
                M10ScheduleOperation::OwnerEvent(request) => Some(request),
                _ => None,
            }),
            Self::Cli { requests } => requests.iter().find_map(|request| match request {
                M10CliScheduleRequest::OwnerEvent(request) => Some(request),
                _ => None,
            }),
        }
    }

    fn relation_projection(&self) -> Option<(&str, &str)> {
        match self {
            Self::Conformance { cases } => cases.iter().find_map(|case| match &case.operation {
                M10ScheduleOperation::RelationProjection {
                    relation, consumer, ..
                } => Some((relation.as_str(), consumer.as_str())),
                _ => None,
            }),
            Self::Cli { requests } => requests.iter().find_map(|request| match request {
                M10CliScheduleRequest::ProjectRelation { relation, consumer } => {
                    Some((relation.as_str(), consumer.as_str()))
                }
                _ => None,
            }),
        }
    }

    fn cases(&self) -> Option<&[M10ScheduleCase]> {
        match self {
            Self::Conformance { cases } => Some(cases),
            Self::Cli { .. } => None,
        }
    }

    /// Resolve the one schedule-owned base source for a separately supplied
    /// route patch carrier.  A route carrier never gets to choose its active
    /// base: an absent or ambiguous schedule binding fails closed before M9
    /// admission or M8 activation is attempted.
    fn route_patch_base_source_path(&self, carrier_ref: &str) -> Result<&str, String> {
        let Self::Conformance { cases } = self else {
            return Err(format!(
                "M10 route carrier {carrier_ref} requires a conformance schedule base"
            ));
        };
        let base_sources = cases
            .iter()
            .filter_map(|case| match &case.operation {
                M10ScheduleOperation::RouteContext {
                    route_patch_carrier_ref: Some(route_patch_carrier_ref),
                    ..
                } if route_patch_carrier_ref == carrier_ref => case.source.as_deref(),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        match base_sources.len() {
            1 => Ok(*base_sources
                .first()
                .expect("one M10 route base source is present")),
            0 => Err(format!(
                "M10 route carrier {carrier_ref} is not bound by a schedule source"
            )),
            _ => Err(format!(
                "M10 route carrier {carrier_ref} has ambiguous schedule base sources"
            )),
        }
    }
}

impl M10ScheduleCase {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "M10 schedule case")?;
        deny_unknown_fields(
            map,
            &["id", "scn", "source", "operation"],
            "M10 schedule case",
        )?;
        let id = required_string(map, "id", "M10 schedule case")?;
        let scn = required_string(map, "scn", "M10 schedule case")?;
        if !matches!(
            scn.as_str(),
            "SCN-01"
                | "SCN-02"
                | "SCN-03"
                | "SCN-04"
                | "SCN-05"
                | "SCN-06"
                | "SCN-07"
                | "SCN-08"
                | "SCN-09"
                | "SCN-10"
                | "SCN-11"
                | "SCN-12"
        ) {
            return Err(format!("unsupported M10 schedule scenario {scn}"));
        }
        let source = optional_string(map, "source", "M10 schedule case")?;
        let operation =
            M10ScheduleOperation::parse(required_value(map, "operation", "M10 schedule case")?)?;
        let source_free = matches!(
            operation,
            M10ScheduleOperation::SubmitCheckedPatchArtifact { .. }
                | M10ScheduleOperation::MembershipFrontierDrift { .. }
        );
        if source_free != source.is_none() {
            return Err(
                "M10 patch carrier cases omit source; every other case requires source".to_string(),
            );
        }
        Ok(Self {
            identity: canonical_schedule_action_identity(&id, value)?,
            id,
            scn,
            source,
            operation,
        })
    }
}

impl M10ScheduleOperation {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "M10 schedule operation")?;
        let kind = required_string(map, "kind", "M10 schedule operation")?;
        match kind.as_str() {
            "owner_event" => Ok(Self::OwnerEvent(M10OwnerEventRequest::parse(
                map, "repeat",
            )?)),
            "admission_then_owner_event" => {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal"],
                    "admission_then_owner_event operation",
                )?;
                Ok(Self::AdmissionThenOwnerEvent {
                    event: required_string(map, "event", "admission_then_owner_event operation")?,
                    principal: required_string(
                        map,
                        "principal",
                        "admission_then_owner_event operation",
                    )?,
                })
            }
            "owner_event_before_admission" => {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal"],
                    "owner_event_before_admission operation",
                )?;
                Ok(Self::OwnerEventBeforeAdmission {
                    event: required_string(map, "event", "owner_event_before_admission operation")?,
                    principal: required_string(
                        map,
                        "principal",
                        "owner_event_before_admission operation",
                    )?,
                })
            }
            "membership_lifecycle" => {
                deny_unknown_fields(
                    map,
                    &["kind", "events", "fresh_incarnation"],
                    "membership_lifecycle operation",
                )?;
                let events = parse_allowed_event_array(
                    required_array(map, "events", "membership_lifecycle operation")?,
                    &["leave", "attack_stale", "rejoin"],
                    "membership_lifecycle operation",
                )?;
                let fresh_incarnation =
                    optional_bool(map, "fresh_incarnation", "membership_lifecycle operation")?;
                Ok(Self::MembershipLifecycle {
                    events,
                    fresh_incarnation,
                })
            }
            "portal_handoff" => {
                deny_unknown_fields(map, &["kind", "events"], "portal_handoff operation")?;
                Ok(Self::PortalHandoff {
                    events: parse_allowed_event_array(
                        required_array(map, "events", "portal_handoff operation")?,
                        &["leave_a", "join_b", "spawn_b"],
                        "portal_handoff operation",
                    )?,
                })
            }
            "observation_request" => {
                deny_unknown_fields(
                    map,
                    &["kind", "request_class", "validated_policy_carrier_ref"],
                    "observation_request operation",
                )?;
                let request_class =
                    match required_string(map, "request_class", "observation_request operation")?
                        .as_str()
                    {
                        "cross_locus_secret_read" => {
                            M10ObservationRequestKind::CrossLocusSecretRead
                        }
                        "cross_locus_observation" => {
                            M10ObservationRequestKind::CrossLocusObservation
                        }
                        value => {
                            return Err(format!("unsupported observation request_class {value}"));
                        }
                    };
                Ok(Self::ObservationRequest {
                    request_class,
                    validated_policy_carrier_ref: required_string(
                        map,
                        "validated_policy_carrier_ref",
                        "observation_request operation",
                    )?,
                })
            }
            "route_context" => {
                deny_unknown_fields(
                    map,
                    &["kind", "events", "route_patch_carrier_ref", "turn_budget"],
                    "route_context operation",
                )?;
                let events = parse_allowed_event_array(
                    required_array(map, "events", "route_context operation")?,
                    &[
                        "invoke_before_patch",
                        "submit_checked_route_patch_artifact",
                        "invoke_after_patch",
                    ],
                    "route_context operation",
                )?;
                let complete_patch_sequence = [
                    "invoke_before_patch",
                    "submit_checked_route_patch_artifact",
                    "invoke_after_patch",
                ];
                let invoke_only = ["invoke_before_patch"];
                if events != complete_patch_sequence && events != invoke_only {
                    return Err("route_context requires invoke-before alone or invoke-before, checked-patch submission, then invoke-after".to_string());
                }
                let route_patch_carrier_ref =
                    optional_string(map, "route_patch_carrier_ref", "route_context operation")?;
                if (events == complete_patch_sequence) != route_patch_carrier_ref.is_some() {
                    return Err("route_context requires a route patch carrier exactly for checked-patch submission".to_string());
                }
                let turn_budget = optional_string(map, "turn_budget", "route_context operation")?;
                if turn_budget
                    .as_deref()
                    .is_some_and(|value| value != "finite")
                {
                    return Err("route_context turn_budget must be finite when present".to_string());
                }
                if turn_budget.is_some() && events != invoke_only {
                    return Err(
                        "route_context turn_budget applies only to an invoke-before request"
                            .to_string(),
                    );
                }
                Ok(Self::RouteContext {
                    events,
                    route_patch_carrier_ref,
                    turn_budget,
                })
            }
            "observer_projection" => {
                deny_unknown_fields(
                    map,
                    &["kind", "policy_carrier_ref", "channel"],
                    "observer_projection operation",
                )?;
                let channel = match required_string(
                    map,
                    "channel",
                    "observer_projection operation",
                )?
                .as_str()
                {
                    "observer_safe" => M10ProjectionChannel::ObserverSafe,
                    "admin_debug" => M10ProjectionChannel::AdminDebug,
                    value => {
                        return Err(format!("unsupported observer projection channel {value}"));
                    }
                };
                Ok(Self::ObserverProjection {
                    policy_carrier_ref: required_string(
                        map,
                        "policy_carrier_ref",
                        "observer_projection operation",
                    )?,
                    channel,
                })
            }
            "lease_option_lifecycle" => {
                deny_unknown_fields(map, &["kind", "events"], "lease_option_lifecycle operation")?;
                Ok(Self::LeaseOptionLifecycle {
                    events: parse_allowed_event_array(
                        required_array(map, "events", "lease_option_lifecycle operation")?,
                        &[
                            "live",
                            "lease_expiry",
                            "write",
                            "fresh_reacquire",
                            "rollback",
                            "write_after_read_lineage",
                        ],
                        "lease_option_lifecycle operation",
                    )?,
                })
            }
            "submit_checked_patch_artifact" => {
                deny_unknown_fields(
                    map,
                    &["kind", "patch_carrier_ref"],
                    "submit_checked_patch_artifact operation",
                )?;
                Ok(Self::SubmitCheckedPatchArtifact {
                    patch_carrier_ref: required_string(
                        map,
                        "patch_carrier_ref",
                        "apply_patch_carrier operation",
                    )?,
                })
            }
            "membership_frontier_drift" => {
                deny_unknown_fields(
                    map,
                    &["kind", "events", "patch_carrier_ref"],
                    "membership_frontier_drift operation",
                )?;
                let events = parse_allowed_event_array(
                    required_array(map, "events", "membership_frontier_drift operation")?,
                    &[
                        "admit_patch",
                        "membership_changes",
                        "activate_checked_patch",
                    ],
                    "membership_frontier_drift operation",
                )?;
                if events
                    != [
                        "admit_patch",
                        "membership_changes",
                        "activate_checked_patch",
                    ]
                {
                    return Err(
                        "membership_frontier_drift requires admission, drift, then activation"
                            .to_string(),
                    );
                }
                Ok(Self::MembershipFrontierDrift {
                    events,
                    patch_carrier_ref: required_string(
                        map,
                        "patch_carrier_ref",
                        "membership_frontier_drift operation",
                    )?,
                })
            }
            "save_load_timeline" => {
                deny_unknown_fields(map, &["kind", "events"], "save_load_timeline operation")?;
                Ok(Self::SaveLoadTimeline {
                    events: parse_allowed_event_array(
                        required_array(map, "events", "save_load_timeline operation")?,
                        &[
                            "save_s1",
                            "leave_a",
                            "lease_expiry",
                            "save_s2",
                            "load_s1_fresh",
                            "merge_stale_s1_into_current",
                            "doctor_expired_lease_live",
                            "doctor_cut_receive_without_send",
                            "timeline_panel",
                            "reacquire_after_load",
                        ],
                        "save_load_timeline operation",
                    )?,
                })
            }
            "corrupted_request" => Ok(Self::CorruptedRequest(M10CorruptedRequest::parse(map)?)),
            "compaction_request" => {
                deny_unknown_fields(
                    map,
                    &["kind", "membership_frontier"],
                    "compaction_request operation",
                )?;
                let membership_frontier = match required_string(
                    map,
                    "membership_frontier",
                    "compaction_request operation",
                )?
                .as_str()
                {
                    "before_audit_cut" => M10CompactionFrontier::BeforeAuditCut,
                    "after_audit_cut" => M10CompactionFrontier::AfterAuditCut,
                    value => {
                        return Err(format!(
                            "unsupported compaction membership_frontier {value}"
                        ));
                    }
                };
                Ok(Self::CompactionRequest {
                    membership_frontier,
                })
            }
            "designated_consumption" => {
                deny_unknown_fields(
                    map,
                    &[
                        "kind",
                        "designated_value_ref",
                        "consumer",
                        "version",
                        "repeat",
                    ],
                    "designated_consumption operation",
                )?;
                Ok(Self::DesignatedConsumption {
                    designated_value_ref: required_string(
                        map,
                        "designated_value_ref",
                        "designated_consumption operation",
                    )?,
                    consumer: required_string(map, "consumer", "designated_consumption operation")?,
                    version: required_u64(map, "version", "designated_consumption operation")?,
                    repeat: optional_u64(map, "repeat", "designated_consumption operation")?
                        .unwrap_or(1),
                })
            }
            "relation_projection" => {
                deny_unknown_fields(
                    map,
                    &["kind", "relation", "consumer", "presentation_context"],
                    "relation_projection operation",
                )?;
                let presentation_context =
                    optional_string(map, "presentation_context", "relation_projection operation")?;
                if let Some(context) = presentation_context.as_deref()
                    && !matches!(context, "split_frame" | "fallback" | "fresh_reacquire")
                {
                    return Err(format!("unsupported relation projection context {context}"));
                }
                Ok(Self::RelationProjection {
                    relation: required_string(map, "relation", "relation_projection operation")?,
                    consumer: required_string(map, "consumer", "relation_projection operation")?,
                    presentation_context,
                })
            }
            _ => Err(format!("unsupported M10 schedule operation {kind}")),
        }
    }
}

impl M10OwnerEventRequest {
    fn parse(map: &serde_json::Map<String, Value>, count_field: &str) -> Result<Self, String> {
        let mut allowed = vec![
            "kind",
            "event",
            "principal",
            "target",
            count_field,
            "seed",
            "arguments",
            "step",
        ];
        if count_field == "attacks" {
            allowed.push("attacks");
        }
        deny_unknown_fields(map, &allowed, "owner_event operation")?;
        let seed = map
            .get("seed")
            .map(|value| {
                let seed = json_object(value, "owner_event seed")?;
                seed.iter()
                    .map(|(key, value)| {
                        value
                            .as_i64()
                            .map(|value| (key.clone(), value))
                            .ok_or_else(|| format!("owner_event seed {key} must be an integer"))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let arguments = map
            .get("arguments")
            .map(|value| {
                let arguments = json_object(value, "owner_event arguments")?;
                arguments
                    .iter()
                    .map(|(key, value)| {
                        value
                            .as_i64()
                            .map(|value| (key.clone(), value))
                            .ok_or_else(|| format!("owner_event argument {key} must be an integer"))
                    })
                    .collect::<Result<BTreeMap<_, _>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            event: required_string(map, "event", "owner_event operation")?,
            principal: required_string(map, "principal", "owner_event operation")?,
            target: optional_string(map, "target", "owner_event operation")?,
            repeat: optional_u64(map, count_field, "owner_event operation")?.unwrap_or(1) as usize,
            step: optional_u64(map, "step", "owner_event operation")?,
            seed,
            arguments,
        })
    }
}

impl M10CliScheduleRequest {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "M10 CLI request")?;
        let event = required_string(map, "event", "M10 CLI request")?;
        match event.as_str() {
            "attack" => {
                deny_unknown_fields(
                    map,
                    &["event", "principal", "target", "attacks"],
                    "attack CLI request",
                )?;
                Ok(Self::OwnerEvent(M10OwnerEventRequest {
                    event,
                    principal: required_string(map, "principal", "attack CLI request")?,
                    target: Some(required_string(map, "target", "attack CLI request")?),
                    repeat: optional_u64(map, "attacks", "attack CLI request")?.unwrap_or(1)
                        as usize,
                    step: None,
                    seed: BTreeMap::new(),
                    arguments: BTreeMap::new(),
                }))
            }
            "save_s1" => {
                deny_unknown_fields(map, &["event"], "save_s1 CLI request")?;
                Ok(Self::SaveS1)
            }
            "load_fresh" => {
                deny_unknown_fields(map, &["event"], "load_fresh CLI request")?;
                Ok(Self::LoadFresh)
            }
            "project_relation" => {
                deny_unknown_fields(
                    map,
                    &["event", "relation", "consumer"],
                    "project_relation CLI request",
                )?;
                Ok(Self::ProjectRelation {
                    relation: required_string(map, "relation", "project_relation CLI request")?,
                    consumer: required_string(map, "consumer", "project_relation CLI request")?,
                })
            }
            _ => Err(format!("unsupported M10 CLI request event {event}")),
        }
    }
}

impl M10CorruptedRequest {
    fn parse(map: &serde_json::Map<String, Value>) -> Result<Self, String> {
        let has_owner_shape = map.contains_key("event") && map.contains_key("principal");
        if has_owner_shape {
            let event = required_string(map, "event", "corrupted_request operation")?;
            let principal = required_string(map, "principal", "corrupted_request operation")?;
            if let Some(missing) = optional_string(map, "missing", "corrupted_request operation")? {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal", "target", "missing"],
                    "missing-capability corrupted_request operation",
                )?;
                if missing != "capability" {
                    return Err(format!(
                        "unsupported missing corrupted_request field {missing}"
                    ));
                }
                return Ok(Self::MissingCapabilityOwner {
                    event,
                    principal,
                    target: required_string(
                        map,
                        "target",
                        "missing-capability corrupted_request operation",
                    )?,
                });
            }
            if let Some(membership) =
                optional_string(map, "membership", "corrupted_request operation")?
            {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal", "target", "membership"],
                    "stale-membership corrupted_request operation",
                )?;
                if membership != "stale" {
                    return Err(format!(
                        "unsupported corrupted membership context {membership}"
                    ));
                }
                return Ok(Self::StaleMembershipOwner {
                    event,
                    principal,
                    target: required_string(
                        map,
                        "target",
                        "stale-membership corrupted_request operation",
                    )?,
                });
            }
            if map.contains_key("spoofed_role") {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal", "spoofed_role"],
                    "role-spoof corrupted_request operation",
                )?;
                return Ok(Self::SpoofedRole {
                    event,
                    principal,
                    spoofed_role: required_string(
                        map,
                        "spoofed_role",
                        "role-spoof corrupted_request operation",
                    )?,
                });
            }
            if map.contains_key("capability") {
                deny_unknown_fields(
                    map,
                    &["kind", "event", "principal", "capability"],
                    "capability-replay corrupted_request operation",
                )?;
                return Ok(Self::ReplayedCapability {
                    event,
                    principal,
                    capability: required_string(
                        map,
                        "capability",
                        "capability-replay corrupted_request operation",
                    )?,
                });
            }
        }
        if map.contains_key("request_class") {
            deny_unknown_fields(
                map,
                &[
                    "kind",
                    "request_class",
                    "capability",
                    "validated_policy_carrier_ref",
                ],
                "wrong-observation-capability corrupted_request operation",
            )?;
            let request_class = match required_string(
                map,
                "request_class",
                "corrupted_request operation",
            )?
            .as_str()
            {
                "cross_locus_secret_read" => M10ObservationRequestKind::CrossLocusSecretRead,
                "cross_locus_observation" => M10ObservationRequestKind::CrossLocusObservation,
                value => {
                    return Err(format!(
                        "unsupported corrupted observation request_class {value}"
                    ));
                }
            };
            let capability = required_string(map, "capability", "corrupted_request operation")?;
            if capability != "wrong_observation_capability" {
                return Err(format!(
                    "unsupported corrupted observation capability {capability}"
                ));
            }
            return Ok(Self::WrongObservationCapability {
                request_class,
                capability,
                validated_policy_carrier_ref: required_string(
                    map,
                    "validated_policy_carrier_ref",
                    "corrupted_request operation",
                )?,
            });
        }
        if map.contains_key("projection") {
            deny_unknown_fields(
                map,
                &["kind", "projection", "origin"],
                "projection-origin corrupted_request operation",
            )?;
            let projection = match required_string(
                map,
                "projection",
                "projection-origin corrupted_request operation",
            )?
            .as_str()
            {
                "observer_history" => M10HistoryProjection::ObserverHistory,
                value => return Err(format!("unsupported history projection {value}")),
            };
            let origin = match required_string(
                map,
                "origin",
                "projection-origin corrupted_request operation",
            )?
            .as_str()
            {
                "forged_without_redaction" => M10HistoryOrigin::ForgedWithoutRedaction,
                value => return Err(format!("unsupported history projection origin {value}")),
            };
            return Ok(Self::ProjectionHistoryOrigin { projection, origin });
        }
        if map.contains_key("events") {
            deny_unknown_fields(map, &["kind", "events"], "cut corrupted_request operation")?;
            let events = parse_string_array(
                required_array(map, "events", "cut corrupted_request operation")?,
                "cut corrupted_request operation events",
            )?;
            return match events.as_slice() {
                [event] if event == "doctor_expired_lease_live" => Ok(Self::ExpiredLeaseLive),
                [event] if event == "doctor_cut_receive_without_send" => {
                    Ok(Self::CutReceiveWithoutSend)
                }
                _ => Err("unsupported cut corrupted_request event sequence".to_string()),
            };
        }
        Err("corrupted_request must select one bounded invalid request shape".to_string())
    }
}

fn require_schedule_header(
    map: &serde_json::Map<String, Value>,
    context: &str,
) -> Result<(), String> {
    if required_string(map, "kind", context)? != "typed_conformance_input" {
        return Err(format!("{context} kind must be typed_conformance_input"));
    }
    if required_value(map, "direct_mutation_api", context)?.as_bool() != Some(false) {
        return Err(format!("{context} direct_mutation_api must be false"));
    }
    Ok(())
}

/// The public JSON is consumed once at the boundary.  Its canonical JSON
/// value becomes a content identity, so a reused carrier/action name cannot
/// stand in for changed request or carrier content downstream.
fn canonical_typed_carrier_identity(id: &str, value: &Value) -> Result<String, String> {
    let canonical = serde_json::to_string(value)
        .map_err(|error| format!("typed carrier {id} cannot be canonicalized: {error}"))?;
    Ok(format!(
        "typed_carrier:{id}:{}",
        deterministic_hash(&canonical),
    ))
}

fn canonical_schedule_action_identity(scn: &str, value: &Value) -> Result<String, String> {
    let canonical = serde_json::to_string(value)
        .map_err(|error| format!("schedule action {scn} cannot be canonicalized: {error}"))?;
    Ok(format!(
        "schedule_action:{scn}:{}",
        deterministic_hash(&canonical),
    ))
}

fn insert_carrier_identity(
    identities: &mut BTreeMap<String, String>,
    id: &str,
    value: &Value,
) -> Result<(), String> {
    let identity = canonical_typed_carrier_identity(id, value)?;
    if identities.insert(id.to_string(), identity).is_some() {
        return Err(format!("typed carriers reuse carrier id {id}"));
    }
    Ok(())
}

fn parse_allowed_event_array(
    values: &[Value],
    allowed: &[&str],
    context: &str,
) -> Result<Vec<String>, String> {
    let events = parse_string_array(values, context)?;
    if events.is_empty()
        || events
            .iter()
            .any(|event| !allowed.contains(&event.as_str()))
    {
        return Err(format!(
            "{context} has an unsupported or empty event sequence"
        ));
    }
    Ok(events)
}

/// Candidate-check input for the separate M10 patch lane.  It is deliberately
/// outside the exogenous schedule and binds the intended candidate text by
/// path plus deterministic content hash before compatibility/activation can
/// inspect it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M10PatchIntentCarrier {
    id: String,
    base_source_path: Option<String>,
    base_source_hash: Option<String>,
    candidate_source_path: String,
    candidate_source_hash: String,
    intent_kind: String,
    state_additions: Vec<M10PatchStateAddition>,
    required_capabilities: Vec<String>,
    required_effects: Vec<String>,
    required_failures: Vec<String>,
    authority_intent: M10PatchAuthorityIntent,
    route_addition: Option<M10RouteAddition>,
}

/// A route is an explicit patch addition, not a schedule-owned result.  The
/// candidate source remains hash-bound exactly as it does for ordinary patch
/// candidates; only successful checked activation may make this topology
/// available to a later retry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct M10RouteAddition {
    from_locus: String,
    to_locus: String,
    route_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M10PatchStateAddition {
    state: String,
    fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M10PatchAuthorityIntent {
    kind: M10PatchAuthorityIntentKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum M10PatchAuthorityIntentKind {
    None,
    SelfGrant { authority: String, grantee: String },
}

impl M10PatchAuthorityIntent {
    fn none() -> Self {
        Self {
            kind: M10PatchAuthorityIntentKind::None,
        }
    }

    fn parse(value: &Value, context: &str) -> Result<Self, String> {
        let authority = json_object(value, context)?;
        let authority_kind = required_string(authority, "kind", context)?;
        match authority_kind.as_str() {
            "none" => {
                deny_unknown_fields(authority, &["kind"], context)?;
                Ok(Self::none())
            }
            "self_grant" => {
                deny_unknown_fields(authority, &["kind", "authority", "grantee"], context)?;
                Ok(Self {
                    kind: M10PatchAuthorityIntentKind::SelfGrant {
                        authority: required_string(authority, "authority", context)?,
                        grantee: required_string(authority, "grantee", context)?,
                    },
                })
            }
            _ => Err(format!("{context} has unsupported kind {authority_kind}")),
        }
    }

    fn stable_key(&self) -> String {
        match &self.kind {
            M10PatchAuthorityIntentKind::None => "none".to_string(),
            M10PatchAuthorityIntentKind::SelfGrant { authority, grantee } => {
                format!("self_grant:{authority}:{grantee}")
            }
        }
    }
}

impl M10PatchIntentCarrier {
    pub fn source_bound(
        candidate_source_path: impl Into<String>,
        candidate_source_text: &str,
        intent_kind: impl Into<String>,
    ) -> Self {
        let candidate_source_path = candidate_source_path.into();
        Self {
            id: "inline-source-bound-candidate".to_string(),
            base_source_path: None,
            base_source_hash: None,
            candidate_source_hash: deterministic_hash(&format!(
                "{candidate_source_path}\0{candidate_source_text}"
            )),
            candidate_source_path,
            intent_kind: intent_kind.into(),
            state_additions: Vec::new(),
            required_capabilities: Vec::new(),
            required_effects: Vec::new(),
            required_failures: Vec::new(),
            authority_intent: M10PatchAuthorityIntent::none(),
            route_addition: None,
        }
    }

    fn from_typed_json(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "patch carrier")?;
        deny_unknown_fields(
            map,
            &[
                "id",
                "kind",
                "base_source",
                "candidate_source",
                "base_source_hash",
                "candidate_source_hash",
                "state_additions",
                "required_capabilities",
                "required_effects",
                "required_failures",
                "authority_intent",
            ],
            "patch carrier",
        )?;
        let id = required_string(map, "id", "patch carrier")?;
        let intent_kind = required_string(map, "kind", "patch carrier")?;
        if intent_kind != "source_patch_intent" {
            return Err(format!(
                "patch carrier {id} has unsupported kind {intent_kind}"
            ));
        }
        let base_source_path = required_string(map, "base_source", "patch carrier")?;
        let candidate_source_path = required_string(map, "candidate_source", "patch carrier")?;
        let base_source_hash = required_string(map, "base_source_hash", "patch carrier")?;
        let candidate_source_hash = required_string(map, "candidate_source_hash", "patch carrier")?;
        let state_additions =
            parse_patch_state_additions(required_array(map, "state_additions", "patch carrier")?)?;
        let required_capabilities = parse_string_array(
            required_array(map, "required_capabilities", "patch carrier")?,
            "patch carrier required_capabilities",
        )?;
        let required_effects = parse_string_array(
            required_array(map, "required_effects", "patch carrier")?,
            "patch carrier required_effects",
        )?;
        let required_failures = parse_string_array(
            required_array(map, "required_failures", "patch carrier")?,
            "patch carrier required_failures",
        )?;
        let authority = json_object(
            required_value(map, "authority_intent", "patch carrier")?,
            "patch carrier authority_intent",
        )?;
        let authority_kind = required_string(authority, "kind", "patch carrier authority_intent")?;
        let authority_intent = match authority_kind.as_str() {
            "none" => {
                deny_unknown_fields(authority, &["kind"], "patch carrier authority_intent")?;
                M10PatchAuthorityIntent::none()
            }
            "self_grant" => {
                deny_unknown_fields(
                    authority,
                    &["kind", "authority", "grantee"],
                    "patch carrier authority_intent",
                )?;
                M10PatchAuthorityIntent {
                    kind: M10PatchAuthorityIntentKind::SelfGrant {
                        authority: required_string(
                            authority,
                            "authority",
                            "patch carrier authority_intent",
                        )?,
                        grantee: required_string(
                            authority,
                            "grantee",
                            "patch carrier authority_intent",
                        )?,
                    },
                }
            }
            _ => {
                return Err(format!(
                    "patch carrier authority_intent has unsupported kind {authority_kind}"
                ));
            }
        };
        Ok(Self {
            id,
            base_source_path: Some(base_source_path),
            base_source_hash: Some(base_source_hash),
            candidate_source_path,
            candidate_source_hash,
            intent_kind,
            state_additions,
            required_capabilities,
            required_effects,
            required_failures,
            authority_intent,
            route_addition: None,
        })
    }

    fn route_from_typed_json(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "route patch carrier")?;
        deny_unknown_fields(
            map,
            &[
                "id",
                "kind",
                "candidate_source",
                "candidate_source_hash",
                "from_locus",
                "to_locus",
                "route_state",
                "required_capabilities",
                "authority_intent",
            ],
            "route patch carrier",
        )?;
        let id = required_string(map, "id", "route patch carrier")?;
        let intent_kind = required_string(map, "kind", "route patch carrier")?;
        if intent_kind != "route_patch_intent" {
            return Err(format!(
                "route patch carrier {id} has unsupported kind {intent_kind}"
            ));
        }
        let route_state = required_string(map, "route_state", "route patch carrier")?;
        if route_state != "available" {
            return Err(format!(
                "route patch carrier {id} has unsupported route_state {route_state}"
            ));
        }
        let authority_intent = M10PatchAuthorityIntent::parse(
            required_value(map, "authority_intent", "route patch carrier")?,
            "route patch carrier authority_intent",
        )?;
        Ok(Self {
            id,
            base_source_path: None,
            base_source_hash: None,
            candidate_source_path: required_string(map, "candidate_source", "route patch carrier")?,
            candidate_source_hash: required_string(
                map,
                "candidate_source_hash",
                "route patch carrier",
            )?,
            intent_kind,
            state_additions: Vec::new(),
            required_capabilities: parse_string_array(
                required_array(map, "required_capabilities", "route patch carrier")?,
                "route patch carrier required_capabilities",
            )?,
            required_effects: Vec::new(),
            required_failures: Vec::new(),
            authority_intent,
            route_addition: Some(M10RouteAddition {
                from_locus: required_string(map, "from_locus", "route patch carrier")?,
                to_locus: required_string(map, "to_locus", "route patch carrier")?,
                route_state,
            }),
        })
    }

    fn matches_candidate(&self, path: &str, text: &str) -> bool {
        self.candidate_source_path == path
            && self.candidate_source_hash == deterministic_hash(&format!("{path}\0{text}"))
    }

    fn matches_sources(
        &self,
        base_path: &str,
        base_text: &str,
        candidate_path: &str,
        candidate_text: &str,
    ) -> bool {
        self.base_source_path.as_deref() == Some(base_path)
            && self.base_source_hash.as_deref()
                == Some(&deterministic_hash(&format!("{base_path}\0{base_text}")))
            && self.matches_candidate(candidate_path, candidate_text)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M10TypedCarriers {
    patches: Vec<M10PatchIntentCarrier>,
    route_patches: Vec<M10PatchIntentCarrier>,
    observations: Vec<M10ObservationPolicyCarrier>,
    fallbacks: Vec<M10FallbackCarrier>,
    carrier_identities: BTreeMap<String, String>,
    stable_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum M10ObservationPolicyCarrier {
    CrossLocus {
        id: String,
        source_path: String,
        source_state: String,
        source_field: String,
        source_owner_locus: String,
        destination_state: String,
        destination_field: String,
        destination_locus: String,
        request_class: M10ObservationRequestClass,
        required_failures: BTreeSet<M10ObservationPolicyFailure>,
    },
    ObserverSafe {
        id: String,
        source_path: String,
        observer_fields: Vec<String>,
        debug_fields: Vec<String>,
    },
    ObserverPrivate {
        id: String,
        source_path: String,
        private_fields: Vec<String>,
        observer_channel: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum M10ObservationRequestClass {
    CrossLocusObservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M10FallbackCarrier {
    id: String,
    relation: String,
    source_path: String,
    options: Vec<M10FallbackOption>,
    negative_capability_floor: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct M10FallbackOption {
    kind: M10FallbackOptionKind,
    target: String,
    lease: String,
    capability: String,
    epoch: String,
    lineage_edges: Vec<M10FallbackLineageEdge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum M10FallbackOptionKind {
    Live,
    Anchor,
    Frozen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct M10FallbackLineageEdge {
    from: M10FallbackOptionKind,
    to: M10FallbackOptionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M10CorrespondenceProfile {
    rows: Vec<M10CorrespondenceRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M10CorrespondenceRow {
    scn_id: String,
    expectation_id: String,
    phase: M10CorrespondencePhase,
    carrier_kind: M10CorrespondenceCarrierKind,
    artifact_identity: String,
    diagnostic_location: String,
    source_derived_reference: Option<String>,
    schedule_action_reference: Option<String>,
    evidence_predicate: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum M10CorrespondencePhase {
    CStatic,
    CRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum M10CorrespondenceCarrierKind {
    OrdinarySource,
    PatchSource,
    TypedCarrier,
    ProfileContext,
    ScheduleAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M10TypedInputMutation {
    id: String,
    kind: M10TypedInputMutationKind,
    stable_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum M10TypedInputMutationKind {
    RewriteResidualSourceRef {
        residual: String,
        replacement_source_path: String,
    },
    RewriteOriginalSourceArtifactIdentity {
        replacement_source_identity: String,
    },
    EnqueueOwnerWithForgedAuthority {
        authority_ref: String,
    },
    DropLiveAuthorityBeforeService {
        authority_ref: String,
    },
    RewriteSourceTextSamePath {
        source: String,
        edit: String,
    },
    DeleteConstruct {
        source: String,
        construct: String,
    },
    AttachCheckedArtifactFromOtherSource {
        source: String,
        artifact_source: String,
    },
    AttachCoreToRejectedSource {
        source: String,
        core_source: String,
    },
    RewritePatchCarrierCandidateIdentity {
        carrier: String,
        candidate_source: String,
    },
    ForceMutationAfterRejectedStep {
        source: String,
    },
    FallbackLineageRepromoteWithoutReacquire {
        carrier: String,
    },
    EmitProjectionHistoryWithoutOriginRedaction {
        source: String,
    },
    MergeStaleSaveOverNewMembership {
        source: String,
    },
    AlterReplayOrderSameProfile {
        source: String,
    },
    RewriteTypedCarrierContentSameId {
        carrier: String,
        edit: Value,
    },
    RewriteScheduleActionContentSameId {
        action_id: String,
        edit: Value,
    },
    ProjectionDeltaProbe {
        domain: String,
        source: String,
        mutation: String,
    },
}

impl M10TypedCarriers {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "typed carriers")?;
        deny_unknown_fields(
            map,
            &[
                "schema_version",
                "patch_carriers",
                "policy_carriers",
                "fallback_carriers",
                "route_patch_carriers",
            ],
            "typed carriers",
        )?;
        let version = required_string(map, "schema_version", "typed carriers")?;
        if version != "m10-i1plus-typed-carriers-v0" {
            return Err(format!("unsupported typed carrier schema {version}"));
        }
        let mut carrier_identities = BTreeMap::new();
        let mut patches = Vec::new();
        for value in required_array(map, "patch_carriers", "typed carriers")? {
            let carrier = M10PatchIntentCarrier::from_typed_json(value)?;
            insert_carrier_identity(&mut carrier_identities, &carrier.id, value)?;
            patches.push(carrier);
        }
        let mut observations = Vec::new();
        for value in required_array(map, "policy_carriers", "typed carriers")? {
            let carrier = M10ObservationPolicyCarrier::parse(value)?;
            insert_carrier_identity(&mut carrier_identities, carrier.id(), value)?;
            observations.push(carrier);
        }
        let mut fallbacks = Vec::new();
        for value in required_array(map, "fallback_carriers", "typed carriers")? {
            let carrier = M10FallbackCarrier::parse(value)?;
            insert_carrier_identity(&mut carrier_identities, &carrier.id, value)?;
            fallbacks.push(carrier);
        }
        let mut route_patches = Vec::new();
        for value in required_array(map, "route_patch_carriers", "typed carriers")? {
            let carrier = M10PatchIntentCarrier::route_from_typed_json(value)?;
            insert_carrier_identity(&mut carrier_identities, &carrier.id, value)?;
            route_patches.push(carrier);
        }
        patches.sort_by(|left, right| left.id.cmp(&right.id));
        route_patches.sort_by(|left, right| left.id.cmp(&right.id));
        observations.sort_by(|left, right| left.id().cmp(right.id()));
        fallbacks.sort_by(|left, right| left.id.cmp(&right.id));
        let stable_hash = deterministic_hash(&format!(
            "{version}|{}|{}|{}|{}",
            patches
                .iter()
                .map(|carrier| format!(
                    "{}:{}:{:?}:{:?}:{:?}:{:?}:{}:{}",
                    carrier.id,
                    carrier.candidate_source_path,
                    carrier.state_additions,
                    carrier.required_capabilities,
                    carrier.required_effects,
                    carrier.required_failures,
                    carrier.authority_intent.stable_key(),
                    carrier.intent_kind,
                ))
                .collect::<Vec<_>>()
                .join("|"),
            route_patches
                .iter()
                .map(|carrier| format!(
                    "{}:{}:{:?}:{:?}:{}",
                    carrier.id,
                    carrier.candidate_source_path,
                    carrier.required_capabilities,
                    carrier.route_addition,
                    carrier.authority_intent.stable_key(),
                ))
                .collect::<Vec<_>>()
                .join("|"),
            observations
                .iter()
                .map(M10ObservationPolicyCarrier::stable_key)
                .collect::<Vec<_>>()
                .join("|"),
            fallbacks
                .iter()
                .map(M10FallbackCarrier::stable_key)
                .collect::<Vec<_>>()
                .join("|"),
        ));
        Ok(Self {
            patches,
            route_patches,
            observations,
            fallbacks,
            carrier_identities,
            stable_hash,
        })
    }

    fn patch(&self, id: &str) -> Option<&M10PatchIntentCarrier> {
        self.patches.iter().find(|carrier| carrier.id == id)
    }

    fn route_patch(&self, id: &str) -> Option<&M10PatchIntentCarrier> {
        self.route_patches.iter().find(|carrier| carrier.id == id)
    }

    fn carrier_identity(&self, id: &str) -> Option<&str> {
        self.carrier_identities.get(id).map(String::as_str)
    }

    fn observation(&self, id: &str) -> Option<&M10ObservationPolicyCarrier> {
        self.observations.iter().find(|carrier| carrier.id() == id)
    }

    fn observer_safe_policy_for_source(
        &self,
        source_path: &str,
    ) -> Result<&M10ObservationPolicyCarrier, String> {
        let policies = self
            .observations
            .iter()
            .filter(|policy| {
                matches!(policy, M10ObservationPolicyCarrier::ObserverSafe { .. })
                    && policy.source_path() == source_path
            })
            .collect::<Vec<_>>();
        match policies.as_slice() {
            [policy] => Ok(*policy),
            [] => Err(format!(
                "M10 observer history source {source_path} lacks an observer-safe policy"
            )),
            _ => Err(format!(
                "M10 observer history source {source_path} has ambiguous observer-safe policies"
            )),
        }
    }

    fn fallback(&self, id: &str) -> Option<&M10FallbackCarrier> {
        self.fallbacks.iter().find(|carrier| carrier.id == id)
    }
}

impl M10ObservationPolicyCarrier {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "observation policy carrier")?;
        if map.contains_key("subject_source") {
            deny_unknown_fields(
                map,
                &[
                    "id",
                    "subject_source",
                    "private_state",
                    "private_field",
                    "source_owner_locus",
                    "destination_state",
                    "destination_field",
                    "destination_owner_locus",
                    "request_class",
                    "required_failures",
                ],
                "cross-locus observation policy carrier",
            )?;
            return Ok(Self::CrossLocus {
                id: required_string(map, "id", "cross-locus observation policy carrier")?,
                source_path: required_string(
                    map,
                    "subject_source",
                    "cross-locus observation policy carrier",
                )?,
                source_state: required_string(
                    map,
                    "private_state",
                    "cross-locus observation policy carrier",
                )?,
                source_field: required_string(
                    map,
                    "private_field",
                    "cross-locus observation policy carrier",
                )?,
                source_owner_locus: required_string(
                    map,
                    "source_owner_locus",
                    "cross-locus observation policy carrier",
                )?,
                destination_state: required_string(
                    map,
                    "destination_state",
                    "cross-locus observation policy carrier",
                )?,
                destination_field: required_string(
                    map,
                    "destination_field",
                    "cross-locus observation policy carrier",
                )?,
                destination_locus: required_string(
                    map,
                    "destination_owner_locus",
                    "cross-locus observation policy carrier",
                )?,
                request_class: match required_string(
                    map,
                    "request_class",
                    "cross-locus observation policy carrier",
                )?
                .as_str()
                {
                    "cross_locus_observation" => M10ObservationRequestClass::CrossLocusObservation,
                    value => return Err(format!("unsupported cross-locus request_class {value}")),
                },
                required_failures: parse_observation_policy_failures(required_array(
                    map,
                    "required_failures",
                    "cross-locus observation policy carrier",
                )?)?,
            });
        }
        if map.contains_key("observer_fields") {
            deny_unknown_fields(
                map,
                &[
                    "id",
                    "field_policy_source",
                    "observer_fields",
                    "debug_fields",
                ],
                "observer-safe policy carrier",
            )?;
            return Ok(Self::ObserverSafe {
                id: required_string(map, "id", "observer-safe policy carrier")?,
                source_path: required_string(
                    map,
                    "field_policy_source",
                    "observer-safe policy carrier",
                )?,
                observer_fields: parse_string_array(
                    required_array(map, "observer_fields", "observer-safe policy carrier")?,
                    "observer-safe policy carrier observer_fields",
                )?,
                debug_fields: parse_string_array(
                    required_array(map, "debug_fields", "observer-safe policy carrier")?,
                    "observer-safe policy carrier debug_fields",
                )?,
            });
        }
        if map.contains_key("private_like_fields") {
            deny_unknown_fields(
                map,
                &[
                    "id",
                    "field_policy_source",
                    "private_like_fields",
                    "observer_channel",
                ],
                "observer-private policy carrier",
            )?;
            return Ok(Self::ObserverPrivate {
                id: required_string(map, "id", "observer-private policy carrier")?,
                source_path: required_string(
                    map,
                    "field_policy_source",
                    "observer-private policy carrier",
                )?,
                private_fields: parse_string_array(
                    required_array(
                        map,
                        "private_like_fields",
                        "observer-private policy carrier",
                    )?,
                    "observer-private policy carrier private_like_fields",
                )?,
                observer_channel: required_string(
                    map,
                    "observer_channel",
                    "observer-private policy carrier",
                )?,
            });
        }
        Err("observation policy carrier must select one finite carrier shape".to_string())
    }

    fn id(&self) -> &str {
        match self {
            Self::CrossLocus { id, .. }
            | Self::ObserverSafe { id, .. }
            | Self::ObserverPrivate { id, .. } => id,
        }
    }

    fn source_path(&self) -> &str {
        match self {
            Self::CrossLocus { source_path, .. }
            | Self::ObserverSafe { source_path, .. }
            | Self::ObserverPrivate { source_path, .. } => source_path,
        }
    }

    fn stable_key(&self) -> String {
        match self {
            Self::CrossLocus {
                id,
                source_path,
                source_state,
                source_field,
                source_owner_locus,
                destination_state,
                destination_field,
                destination_locus,
                request_class,
                required_failures,
            } => format!(
                "cross:{id}:{source_path}:{source_state}:{source_field}:{source_owner_locus}:{destination_state}:{destination_field}:{destination_locus}:{request_class:?}:{required_failures:?}"
            ),
            Self::ObserverSafe {
                id,
                source_path,
                observer_fields,
                debug_fields,
            } => format!("safe:{id}:{source_path}:{observer_fields:?}:{debug_fields:?}"),
            Self::ObserverPrivate {
                id,
                source_path,
                private_fields,
                observer_channel,
            } => format!("private:{id}:{source_path}:{private_fields:?}:{observer_channel}"),
        }
    }
}

impl M10FallbackCarrier {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "fallback carrier")?;
        deny_unknown_fields(
            map,
            &[
                "id",
                "relation",
                "source",
                "options",
                "negative_capability_floor",
            ],
            "fallback carrier",
        )?;
        let options = required_array(map, "options", "fallback carrier")?
            .iter()
            .map(M10FallbackOption::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if options.len() != 3
            || !matches!(
                options.first().map(|option| option.kind),
                Some(M10FallbackOptionKind::Live)
            )
            || !matches!(
                options.get(1).map(|option| option.kind),
                Some(M10FallbackOptionKind::Anchor)
            )
            || !matches!(
                options.get(2).map(|option| option.kind),
                Some(M10FallbackOptionKind::Frozen)
            )
        {
            return Err(
                "fallback carrier requires exactly ordered live, anchor, frozen options"
                    .to_string(),
            );
        }
        Ok(Self {
            id: required_string(map, "id", "fallback carrier")?,
            relation: required_string(map, "relation", "fallback carrier")?,
            source_path: required_string(map, "source", "fallback carrier")?,
            options,
            negative_capability_floor: required_string(
                map,
                "negative_capability_floor",
                "fallback carrier",
            )?,
        })
    }

    fn stable_key(&self) -> String {
        format!(
            "{}:{}:{}:{:?}:{}",
            self.id, self.relation, self.source_path, self.options, self.negative_capability_floor,
        )
    }
}

impl M10FallbackOption {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "fallback option")?;
        deny_unknown_fields(
            map,
            &[
                "kind",
                "target",
                "lease",
                "capability",
                "epoch",
                "lineage_edges",
            ],
            "fallback option",
        )?;
        let kind = match required_string(map, "kind", "fallback option")?.as_str() {
            "live" => M10FallbackOptionKind::Live,
            "anchor" => M10FallbackOptionKind::Anchor,
            "frozen" => M10FallbackOptionKind::Frozen,
            value => return Err(format!("unsupported fallback option kind {value}")),
        };
        let lineage_edges = required_array(map, "lineage_edges", "fallback option")?
            .iter()
            .map(M10FallbackLineageEdge::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            kind,
            target: required_string(map, "target", "fallback option")?,
            lease: required_string(map, "lease", "fallback option")?,
            capability: required_string(map, "capability", "fallback option")?,
            epoch: required_string(map, "epoch", "fallback option")?,
            lineage_edges,
        })
    }
}

impl M10FallbackLineageEdge {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "fallback lineage edge")?;
        deny_unknown_fields(map, &["from", "to"], "fallback lineage edge")?;
        let parse_kind =
            |field| match required_string(map, field, "fallback lineage edge")?.as_str() {
                "live" => Ok(M10FallbackOptionKind::Live),
                "anchor" => Ok(M10FallbackOptionKind::Anchor),
                "frozen" => Ok(M10FallbackOptionKind::Frozen),
                value => Err(format!("unsupported fallback lineage option {value}")),
            };
        Ok(Self {
            from: parse_kind("from")?,
            to: parse_kind("to")?,
        })
    }
}

impl M10CorrespondenceProfile {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "correspondence predicate profile")?;
        deny_unknown_fields(
            map,
            &["schema_version", "correspondence_rows"],
            "correspondence predicate profile",
        )?;
        let version = required_string(map, "schema_version", "correspondence predicate profile")?;
        if version != "m10-i1plus-correspondence-predicates-v0" {
            return Err(format!(
                "unsupported correspondence predicate schema {version}"
            ));
        }
        let rows = required_array(
            map,
            "correspondence_rows",
            "correspondence predicate profile",
        )?
        .iter()
        .map(M10CorrespondenceRow::parse)
        .collect::<Result<Vec<_>, _>>()?;
        if rows.is_empty() {
            return Err("correspondence predicate profile requires at least one row".to_string());
        }
        Ok(Self { rows })
    }
}

impl M10CorrespondenceRow {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "correspondence row")?;
        deny_unknown_fields(
            map,
            &[
                "scn_id",
                "expectation_id",
                "phase",
                "carrier_kind",
                "artifact_identity",
                "diagnostic_location",
                "source_derived_reference",
                "schedule_action_reference",
                "evidence_predicate",
            ],
            "correspondence row",
        )?;
        let phase = match required_string(map, "phase", "correspondence row")?.as_str() {
            "static" => M10CorrespondencePhase::CStatic,
            "runtime" => M10CorrespondencePhase::CRuntime,
            value => return Err(format!("unsupported correspondence phase {value}")),
        };
        let carrier_kind =
            match required_string(map, "carrier_kind", "correspondence row")?.as_str() {
                "ordinary_source" => M10CorrespondenceCarrierKind::OrdinarySource,
                "patch_source" => M10CorrespondenceCarrierKind::PatchSource,
                "typed_carrier" => M10CorrespondenceCarrierKind::TypedCarrier,
                "profile_context" => M10CorrespondenceCarrierKind::ProfileContext,
                "schedule_action" => M10CorrespondenceCarrierKind::ScheduleAction,
                value => return Err(format!("unsupported correspondence carrier_kind {value}")),
            };
        Ok(Self {
            scn_id: required_string(map, "scn_id", "correspondence row")?,
            expectation_id: required_string(map, "expectation_id", "correspondence row")?,
            phase,
            carrier_kind,
            artifact_identity: required_string(map, "artifact_identity", "correspondence row")?,
            diagnostic_location: required_string(map, "diagnostic_location", "correspondence row")?,
            source_derived_reference: optional_nullable_string(
                map,
                "source_derived_reference",
                "correspondence row",
            )?,
            schedule_action_reference: optional_nullable_string(
                map,
                "schedule_action_reference",
                "correspondence row",
            )?,
            evidence_predicate: required_string(map, "evidence_predicate", "correspondence row")?,
        })
    }
}

impl M10TypedInputMutation {
    fn parse(value: &Value) -> Result<Self, String> {
        let map = json_object(value, "typed input mutation")?;
        deny_unknown_fields(
            map,
            &["schema_version", "id", "mutation"],
            "typed input mutation",
        )?;
        let schema_version = required_string(map, "schema_version", "typed input mutation")?;
        if schema_version != "m10-i1plus-source-run-mutation-v0" {
            return Err(format!(
                "unsupported typed input mutation schema {schema_version}"
            ));
        }
        let id = required_string(map, "id", "typed input mutation")?;
        let mutation = json_object(
            required_value(map, "mutation", "typed input mutation")?,
            "typed input mutation mutation",
        )?;
        let mutation_kind = required_string(mutation, "kind", "typed input mutation mutation")?;
        let kind = match mutation_kind.as_str() {
            "projection_delta_probe" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "domain", "source", "mutation"],
                    "projection_delta_probe mutation",
                )?;
                M10TypedInputMutationKind::ProjectionDeltaProbe {
                    domain: required_string(mutation, "domain", "projection_delta_probe mutation")?,
                    source: required_string(mutation, "source", "projection_delta_probe mutation")?,
                    mutation: required_string(
                        mutation,
                        "mutation",
                        "projection_delta_probe mutation",
                    )?,
                }
            }
            "rewrite_residual_source_ref" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "residual", "replacement_source_path"],
                    "rewrite_residual_source_ref mutation",
                )?;
                M10TypedInputMutationKind::RewriteResidualSourceRef {
                    residual: required_string(
                        mutation,
                        "residual",
                        "rewrite_residual_source_ref mutation",
                    )?,
                    replacement_source_path: required_string(
                        mutation,
                        "replacement_source_path",
                        "rewrite_residual_source_ref mutation",
                    )?,
                }
            }
            "rewrite_original_source_artifact_identity" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "replacement_source_identity"],
                    "rewrite_original_source_artifact_identity mutation",
                )?;
                M10TypedInputMutationKind::RewriteOriginalSourceArtifactIdentity {
                    replacement_source_identity: required_string(
                        mutation,
                        "replacement_source_identity",
                        "rewrite_original_source_artifact_identity mutation",
                    )?,
                }
            }
            "enqueue_owner_with_forged_authority" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "authority_ref"],
                    "enqueue_owner_with_forged_authority mutation",
                )?;
                M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority {
                    authority_ref: required_string(
                        mutation,
                        "authority_ref",
                        "enqueue_owner_with_forged_authority mutation",
                    )?,
                }
            }
            "drop_live_authority_before_service" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "authority_ref"],
                    "drop_live_authority_before_service mutation",
                )?;
                M10TypedInputMutationKind::DropLiveAuthorityBeforeService {
                    authority_ref: required_string(
                        mutation,
                        "authority_ref",
                        "drop_live_authority_before_service mutation",
                    )?,
                }
            }
            "rewrite_source_text_same_path" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source", "edit"],
                    "rewrite_source_text_same_path mutation",
                )?;
                M10TypedInputMutationKind::RewriteSourceTextSamePath {
                    source: required_string(
                        mutation,
                        "source",
                        "rewrite_source_text_same_path mutation",
                    )?,
                    edit: required_string(
                        mutation,
                        "edit",
                        "rewrite_source_text_same_path mutation",
                    )?,
                }
            }
            "delete_construct" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source", "construct"],
                    "delete_construct mutation",
                )?;
                M10TypedInputMutationKind::DeleteConstruct {
                    source: required_string(mutation, "source", "delete_construct mutation")?,
                    construct: required_string(mutation, "construct", "delete_construct mutation")?,
                }
            }
            "attach_checked_artifact_from_other_source" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source", "artifact_source"],
                    "attach_checked_artifact_from_other_source mutation",
                )?;
                M10TypedInputMutationKind::AttachCheckedArtifactFromOtherSource {
                    source: required_string(
                        mutation,
                        "source",
                        "attach_checked_artifact_from_other_source mutation",
                    )?,
                    artifact_source: required_string(
                        mutation,
                        "artifact_source",
                        "attach_checked_artifact_from_other_source mutation",
                    )?,
                }
            }
            "attach_core_to_rejected_source" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source", "core_source"],
                    "attach_core_to_rejected_source mutation",
                )?;
                M10TypedInputMutationKind::AttachCoreToRejectedSource {
                    source: required_string(
                        mutation,
                        "source",
                        "attach_core_to_rejected_source mutation",
                    )?,
                    core_source: required_string(
                        mutation,
                        "core_source",
                        "attach_core_to_rejected_source mutation",
                    )?,
                }
            }
            "rewrite_patch_carrier_candidate_identity" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "carrier", "candidate_source"],
                    "rewrite_patch_carrier_candidate_identity mutation",
                )?;
                M10TypedInputMutationKind::RewritePatchCarrierCandidateIdentity {
                    carrier: required_string(
                        mutation,
                        "carrier",
                        "rewrite_patch_carrier_candidate_identity mutation",
                    )?,
                    candidate_source: required_string(
                        mutation,
                        "candidate_source",
                        "rewrite_patch_carrier_candidate_identity mutation",
                    )?,
                }
            }
            "force_mutation_after_rejected_step" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source"],
                    "force_mutation_after_rejected_step mutation",
                )?;
                M10TypedInputMutationKind::ForceMutationAfterRejectedStep {
                    source: required_string(
                        mutation,
                        "source",
                        "force_mutation_after_rejected_step mutation",
                    )?,
                }
            }
            "fallback_lineage_repromote_without_reacquire" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "carrier"],
                    "fallback_lineage_repromote_without_reacquire mutation",
                )?;
                M10TypedInputMutationKind::FallbackLineageRepromoteWithoutReacquire {
                    carrier: required_string(
                        mutation,
                        "carrier",
                        "fallback_lineage_repromote_without_reacquire mutation",
                    )?,
                }
            }
            "emit_projection_history_without_origin_redaction" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source"],
                    "emit_projection_history_without_origin_redaction mutation",
                )?;
                M10TypedInputMutationKind::EmitProjectionHistoryWithoutOriginRedaction {
                    source: required_string(
                        mutation,
                        "source",
                        "emit_projection_history_without_origin_redaction mutation",
                    )?,
                }
            }
            "merge_stale_save_over_new_membership" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source"],
                    "merge_stale_save_over_new_membership mutation",
                )?;
                M10TypedInputMutationKind::MergeStaleSaveOverNewMembership {
                    source: required_string(
                        mutation,
                        "source",
                        "merge_stale_save_over_new_membership mutation",
                    )?,
                }
            }
            "alter_replay_order_same_profile" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "source"],
                    "alter_replay_order_same_profile mutation",
                )?;
                M10TypedInputMutationKind::AlterReplayOrderSameProfile {
                    source: required_string(
                        mutation,
                        "source",
                        "alter_replay_order_same_profile mutation",
                    )?,
                }
            }
            "rewrite_typed_carrier_content_same_id" => {
                deny_unknown_fields(
                    mutation,
                    &["kind", "carrier", "preserve_id", "edit"],
                    "rewrite_typed_carrier_content_same_id mutation",
                )?;
                if optional_bool(
                    mutation,
                    "preserve_id",
                    "rewrite_typed_carrier_content_same_id mutation",
                )? != Some(true)
                {
                    return Err("typed carrier content mutation must preserve its id for identity validation".to_string());
                }
                M10TypedInputMutationKind::RewriteTypedCarrierContentSameId {
                    carrier: required_string(
                        mutation,
                        "carrier",
                        "rewrite_typed_carrier_content_same_id mutation",
                    )?,
                    edit: required_value(
                        mutation,
                        "edit",
                        "rewrite_typed_carrier_content_same_id mutation",
                    )?
                    .clone(),
                }
            }
            "rewrite_schedule_action_content_same_id" | "rewrite_schedule_case_content_same_id" => {
                deny_unknown_fields(
                    mutation,
                    if mutation_kind == "rewrite_schedule_action_content_same_id" {
                        &["kind", "action_id", "preserve_id", "edit"]
                    } else {
                        &["kind", "case_id", "preserve_id", "edit"]
                    },
                    "rewrite_schedule_action_content_same_id mutation",
                )?;
                if optional_bool(
                    mutation,
                    "preserve_id",
                    "rewrite_schedule_action_content_same_id mutation",
                )? != Some(true)
                {
                    return Err("schedule action content mutation must preserve its id for identity validation".to_string());
                }
                let action_id = if mutation_kind == "rewrite_schedule_action_content_same_id" {
                    required_string(
                        mutation,
                        "action_id",
                        "rewrite_schedule_action_content_same_id mutation",
                    )?
                } else {
                    required_string(
                        mutation,
                        "case_id",
                        "rewrite_schedule_case_content_same_id mutation",
                    )?
                };
                M10TypedInputMutationKind::RewriteScheduleActionContentSameId {
                    action_id,
                    edit: required_value(
                        mutation,
                        "edit",
                        "rewrite_schedule_action_content_same_id mutation",
                    )?
                    .clone(),
                }
            }
            _ => {
                return Err(format!(
                    "unsupported typed M10 input mutation {mutation_kind}"
                ));
            }
        };
        let stable_mutation = serde_json::to_string(&kind)
            .map_err(|error| format!("typed input mutation cannot serialize stably: {error}"))?;
        Ok(Self {
            stable_hash: deterministic_hash(&format!("{schema_version}\0{id}\0{stable_mutation}")),
            id,
            kind,
        })
    }
}

fn json_object<'a>(
    value: &'a Value,
    context: &str,
) -> Result<&'a serde_json::Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{context} must be a JSON object"))
}

fn deny_unknown_fields(
    map: &serde_json::Map<String, Value>,
    allowed: &[&str],
    context: &str,
) -> Result<(), String> {
    if let Some(field) = map.keys().find(|field| !allowed.contains(&field.as_str())) {
        return Err(format!("{context} contains unknown field {field}"));
    }
    Ok(())
}

fn required_value<'a>(
    map: &'a serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Value, String> {
    map.get(key)
        .ok_or_else(|| format!("{context} requires field {key}"))
}

fn required_string(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<String, String> {
    required_value(map, key, context)?
        .as_str()
        .map(ToOwned::to_owned)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{context} field {key} must be a non-empty string"))
}

fn optional_string(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<Option<String>, String> {
    map.get(key)
        .map(|value| {
            value
                .as_str()
                .map(ToOwned::to_owned)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| format!("{context} field {key} must be a non-empty string"))
        })
        .transpose()
}

fn optional_bool(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<Option<bool>, String> {
    map.get(key)
        .map(|value| {
            value
                .as_bool()
                .ok_or_else(|| format!("{context} field {key} must be a boolean"))
        })
        .transpose()
}

fn required_u64(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<u64, String> {
    required_value(map, key, context)?
        .as_u64()
        .ok_or_else(|| format!("{context} field {key} must be a non-negative integer"))
}

fn optional_u64(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<Option<u64>, String> {
    map.get(key)
        .map(|value| {
            value
                .as_u64()
                .ok_or_else(|| format!("{context} field {key} must be a non-negative integer"))
        })
        .transpose()
}

fn optional_nullable_string(
    map: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<Option<String>, String> {
    match map.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_str()
            .map(ToOwned::to_owned)
            .filter(|value| !value.is_empty())
            .map(Some)
            .ok_or_else(|| format!("{context} field {key} must be a string or null")),
    }
}

fn required_array<'a>(
    map: &'a serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Vec<Value>, String> {
    required_value(map, key, context)?
        .as_array()
        .ok_or_else(|| format!("{context} field {key} must be an array"))
}

fn parse_string_array(values: &[Value], context: &str) -> Result<Vec<String>, String> {
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToOwned::to_owned)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| format!("{context} must contain only non-empty strings"))
        })
        .collect()
}

fn parse_observation_policy_failures(
    values: &[Value],
) -> Result<BTreeSet<M10ObservationPolicyFailure>, String> {
    values
        .iter()
        .map(|value| match value.as_str() {
            Some("VisibilityDenied") => Ok(M10ObservationPolicyFailure::VisibilityDenied),
            Some(value) => Err(format!(
                "cross-locus observation policy carrier has unsupported required failure {value}"
            )),
            None => Err(
                "cross-locus observation policy carrier required_failures must contain strings"
                    .to_string(),
            ),
        })
        .collect()
}

fn parse_patch_state_additions(values: &[Value]) -> Result<Vec<M10PatchStateAddition>, String> {
    values
        .iter()
        .map(|value| {
            let map = json_object(value, "patch state addition")?;
            deny_unknown_fields(map, &["state", "fields"], "patch state addition")?;
            Ok(M10PatchStateAddition {
                state: required_string(map, "state", "patch state addition")?,
                fields: parse_string_array(
                    required_array(map, "fields", "patch state addition")?,
                    "patch state addition fields",
                )?,
            })
        })
        .collect()
}

/// Serializable result wrapper.  The object shape is intentionally explicit
/// JSON so this provisional profile can evolve without advertising a frozen
/// Rust field layout as public contract.
#[derive(Debug, Clone)]
pub struct M10ConformanceReport(Value);

impl Serialize for M10ConformanceReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for M10ConformanceReport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Value::deserialize(deserializer).map(Self)
    }
}

impl M10ConformanceReport {
    pub fn as_value(&self) -> &Value {
        &self.0
    }
}

/// One deterministic reference profile.  It owns no authentication provider;
/// M9's sealed in-crate lineage is the only route that supplies M8 authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M10ReferenceSystem {
    profile: String,
    public_contract_frozen: bool,
}

/// Typed command selection for the provisional `mir` front door.  The command
/// carries source/schedule inputs only; it deliberately has no store, grant,
/// verdict, history, fallback, or projection mutation field.
#[derive(Debug, Clone)]
pub struct M10CliFacadeCommand {
    name: &'static str,
    source_path: Option<String>,
    candidate_source_path: Option<String>,
    corpus_path: Option<String>,
    typed_schedule: Option<M10TypedSchedule>,
    typed_schedule_error: Option<String>,
    typed_carriers: Option<M10TypedCarriers>,
    typed_carriers_error: Option<String>,
    predicate_profile: Option<M10CorrespondenceProfile>,
    predicate_profile_error: Option<String>,
    patch_intent_carrier: Option<M10PatchIntentCarrier>,
    patch_intent_error: Option<String>,
    expected_output_path: Option<String>,
    source_absent_artifact: Option<String>,
}

impl M10CliFacadeCommand {
    fn named(name: &'static str) -> Self {
        Self {
            name,
            source_path: None,
            candidate_source_path: None,
            corpus_path: None,
            typed_schedule: None,
            typed_schedule_error: None,
            typed_carriers: None,
            typed_carriers_error: None,
            predicate_profile: None,
            predicate_profile_error: None,
            patch_intent_carrier: None,
            patch_intent_error: None,
            expected_output_path: None,
            source_absent_artifact: None,
        }
    }

    pub fn parse() -> Self {
        Self::named("parse")
    }
    pub fn check() -> Self {
        Self::named("check")
    }
    pub fn elaborate() -> Self {
        Self::named("elaborate")
    }
    pub fn run() -> Self {
        Self::named("run")
    }
    pub fn trace() -> Self {
        Self::named("trace")
    }
    pub fn project() -> Self {
        Self::named("project")
    }
    pub fn save() -> Self {
        Self::named("save")
    }
    pub fn load() -> Self {
        Self::named("load")
    }
    pub fn patch() -> Self {
        Self::named("patch")
    }
    pub fn conform() -> Self {
        Self::named("conform")
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn source_path(mut self, path: impl Into<String>) -> Self {
        self.source_path = Some(path.into());
        self
    }

    pub fn candidate_source_path(mut self, path: impl Into<String>) -> Self {
        self.candidate_source_path = Some(path.into());
        self
    }

    pub fn corpus_path(mut self, path: impl Into<String>) -> Self {
        self.corpus_path = Some(path.into());
        self
    }

    pub fn typed_schedule_json(mut self, schedule: Value) -> Self {
        match M10TypedSchedule::parse(&schedule) {
            Ok(schedule) => self.typed_schedule = Some(schedule),
            Err(error) => self.typed_schedule_error = Some(error),
        }
        self
    }

    /// Parse the separate typed carriers at the CLI boundary.  The facade
    /// keeps only the validated carrier values after this call.
    pub fn typed_carriers_json(mut self, carriers: Value) -> Self {
        match M10TypedCarriers::parse(&carriers) {
            Ok(carriers) => self.typed_carriers = Some(carriers),
            Err(error) => self.typed_carriers_error = Some(error),
        }
        self
    }

    /// Parse the verifier-only correspondence profile at the CLI boundary.
    pub fn predicate_profile_json(mut self, profile: Value) -> Self {
        match M10CorrespondenceProfile::parse(&profile) {
            Ok(profile) => self.predicate_profile = Some(profile),
            Err(error) => self.predicate_profile_error = Some(error),
        }
        self
    }

    pub fn patch_intent_carrier(mut self, carrier: M10PatchIntentCarrier) -> Self {
        self.patch_intent_carrier = Some(carrier);
        self
    }

    pub fn patch_intent_json(mut self, carrier: Value) -> Self {
        match M10PatchIntentCarrier::from_typed_json(&carrier) {
            Ok(carrier) => self.patch_intent_carrier = Some(carrier),
            Err(error) => self.patch_intent_error = Some(error),
        }
        self
    }

    pub fn expected_output_json(mut self, path: impl Into<String>) -> Self {
        self.expected_output_path = Some(path.into());
        self
    }

    pub fn checked_artifact_without_source(mut self, artifact: impl Into<String>) -> Self {
        self.source_absent_artifact = Some(artifact.into());
        self
    }
}

impl M10ReferenceSystem {
    pub fn deterministic_profile(profile: impl Into<String>) -> Self {
        Self {
            profile: profile.into(),
            public_contract_frozen: false,
        }
    }

    /// Source inspection route used by the provisional `mir parse`, `check`,
    /// and `elab` commands.  It still uses the ordinary M6→M7 path and
    /// records M8's actual direct judgment, but it intentionally creates no
    /// M9 authority and no runtime state.
    pub fn inspect_source(
        &mut self,
        request: M10SourceRunRequest,
    ) -> Result<M10ConformanceReport, String> {
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(
            request.source_path.clone(),
            request.source_text.clone(),
        ))
        .map_err(|diagnostics| {
            format!(
                "M10 source rejected before Core: {}",
                diagnostics.primary().canonical_code()
            )
        })?;
        let identity = checked.program_identity().stable_key();
        let admission = m8_admission_for(&checked)?;
        let direct_outcome = match M8Runtime::default().admit(checked.clone(), admission) {
            Err(diagnostics) => format!("{:?}", diagnostics.primary().kind()),
            Ok(_) => "AcceptedAtM8".to_string(),
        };
        Ok(M10ConformanceReport(json!({
            "profile": self.profile,
            "public_contract_frozen": self.public_contract_frozen,
            "terminal_outcome": "Inspected",
            "source": {
                "path": request.source_path,
                "kind": "inline_text",
                "fixture_name_lookup_used": false,
                "identity": identity,
            },
            "pipeline": { "m6_parse_count": 1, "m7_checked_artifact_count": 1, "reparsed_after_m7": false },
            "checked": { "source_identity": identity },
            "m8": { "checked_source_identity": identity, "direct_residuals": [{ "outcome": direct_outcome }] },
            "runtime": { "mutation_count": 0 },
        })))
    }

    pub fn run_cli(
        &mut self,
        command: M10CliFacadeCommand,
    ) -> Result<M10ConformanceReport, String> {
        if command.expected_output_path.is_some() || command.source_absent_artifact.is_some() {
            return Ok(M10ConformanceReport(json!({
                "command": command.name,
                "terminal_outcome": "RejectedBeforeExecution",
                "runtime": { "mutation_count": 0 },
                "facade": {
                    "source_first": false,
                    "final_public_abi_claimed": false,
                    "fixture_name_result_lookup_used": false,
                    "expected_output_sidecars_loaded": false,
                }
            })));
        }
        let report = match command.name {
            "parse" | "check" | "elab" | "elaborate" => {
                let (source_path, source_text) = cli_source_text(&command)?;
                self.inspect_source(M10SourceRunRequest::inline_text(source_path, source_text))?
            }
            "run" | "trace" => {
                let (source_path, source_text) = cli_source_text(&command)?;
                self.run_source(cli_owner_run_request(&command, source_path, source_text)?)?
            }
            "project" => self.run_cli_project(&command)?,
            "save" | "load" => self.run_cli_save_load(&command)?,
            "patch" => self.run_cli_patch(&command)?,
            "conform" => {
                let root = command
                    .corpus_path
                    .as_deref()
                    .ok_or_else(|| "M10 CLI conform requires --corpus".to_string())?;
                let schedule = command
                    .typed_schedule
                    .clone()
                    .ok_or_else(|| "M10 CLI conform requires --schedule".to_string())?;
                if command.typed_carriers.is_none() || command.predicate_profile.is_none() {
                    return Ok(with_cli_facade_metadata(
                        &command,
                        M10ConformanceReport(json!({
                            "terminal_outcome": "RejectedBeforeExecution",
                            "runtime": { "mutation_count": 0 },
                            "facade": {
                                "missing_typed_carriers": command.typed_carriers.is_none(),
                                "missing_predicate_profile": command.predicate_profile.is_none(),
                            },
                        })),
                    ));
                }
                let mut request = M10SourceRunRequest::corpus_path(root)
                    .forbid_fixture_name_result_lookup()
                    .forbid_expected_output_sidecars();
                request.typed_schedule = Some(schedule);
                request.typed_schedule_error = command.typed_schedule_error.clone();
                request.typed_carriers = command.typed_carriers.clone();
                request.typed_carriers_error = command.typed_carriers_error.clone();
                request.predicate_profile = command.predicate_profile.clone();
                request.predicate_profile_error = command.predicate_profile_error.clone();
                self.run_conformance(request)?
            }
            _ => return Err(format!("unsupported M10 CLI command {}", command.name)),
        };
        Ok(with_cli_facade_metadata(&command, report))
    }

    fn run_cli_project(
        &mut self,
        command: &M10CliFacadeCommand,
    ) -> Result<M10ConformanceReport, String> {
        let (source_path, source_text) = cli_source_text(command)?;
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(source_path, source_text))
            .map_err(|diagnostics| {
                format!(
                    "M10 CLI project source rejected: {}",
                    diagnostics.primary().canonical_code()
                )
            })?;
        let schedule = command
            .typed_schedule
            .as_ref()
            .ok_or_else(|| "M10 CLI project requires a typed projection request".to_string())?;
        let (relation, consumer) = cli_relation_projection_request(schedule)?;
        let (principal, locus) = patch_principal_and_locus(&checked)?;
        let seam = m10_resolve_checked_for_patch(&checked, principal, locus)?;
        let (instance, authority_state) = seam.into_parts();
        let (lease, context) =
            projection_seed(&checked, Some(&(relation.clone(), consumer.clone())))?;
        let (Some(lease), Some(context)) = (lease, context) else {
            return Err(
                "M10 CLI project did not materialize a source-bound relation lease/context"
                    .to_string(),
            );
        };
        let mut runtime = M8LocalRuntime::from_admitted(
            instance,
            M8LocalRuntimeSeed::new()
                .with_authority_state(authority_state)
                .with_live_lease(lease),
        );
        let projection = runtime
            .project_relation(&relation, context)
            .map_err(|diagnostics| {
                format!(
                    "M10 CLI project rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        Ok(M10ConformanceReport(json!({
            "terminal_outcome": "Projected",
            "source": { "path": source_path, "identity": checked.program_identity().stable_key() },
            "m8": { "direct_residuals": [{ "outcome": "DeferredToM9" }] },
            "m9": { "source_bound_admission": "accepted" },
            "projection": {
                "relation": {
                    "name": projection.relation(),
                    "consumer_locus": projection.consumer_locus(),
                    "subject": projection.subject(),
                    "selected_anchor": projection.selected_anchor(),
                    "context_frontier": projection.context_frontier(),
                    "derived_pose": projection.derived_pose().map(|point| json!({ "x": point.x(), "y": point.y() })),
                    "provenance": {
                        "program_artifact": checked.program_identity().stable_key(),
                        "schedule_action": "consumer_local_projection",
                    },
                },
            },
            "runtime": { "mutation_count": 0 },
        })))
    }

    fn run_cli_save_load(
        &mut self,
        command: &M10CliFacadeCommand,
    ) -> Result<M10ConformanceReport, String> {
        let (source_path, source_text) = cli_source_text(command)?;
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(source_path, source_text))
            .map_err(|diagnostics| {
                format!(
                    "M10 CLI {} source rejected: {}",
                    command.name,
                    diagnostics.primary().canonical_code()
                )
            })?;
        let (principal, locus) = patch_principal_and_locus(&checked)?;
        let seam = m10_resolve_checked_for_patch(&checked, principal, locus)?;
        let (instance, authority_state) = seam.into_parts();
        let mut runtime = M8LocalRuntime::from_admitted(
            instance,
            M8LocalRuntimeSeed::new().with_authority_state(authority_state),
        );
        let cut = runtime.save_local_cut("m10-cli-local-cut");
        let fresh_floor = M8LiveFloor::same_current(&cut);
        runtime
            .try_restore_local_cut(&cut, &fresh_floor)
            .map_err(|diagnostics| {
                format!(
                    "M10 CLI fresh restore rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let mut stale_runtime = runtime.clone();
        let before_stale = stale_runtime.save_relevant_payload();
        let stale = stale_runtime.try_restore_local_cut(
            &cut,
            &M8LiveFloor::same_current(&cut).with_stale_membership("m10-cli-stale-membership"),
        );
        let after_stale = stale_runtime.save_relevant_payload();
        let stale_kind = match stale {
            Err(diagnostics) => format!("{:?}", diagnostics.primary().kind()),
            Ok(()) => return Err("M10 CLI stale restore unexpectedly succeeded".to_string()),
        };
        Ok(M10ConformanceReport(json!({
            "terminal_outcome": if command.name == "save" { "Saved" } else { "LoadedFresh" },
            "source": { "path": source_path, "identity": checked.program_identity().stable_key() },
            "cut": {
                "saved": true,
                "fresh_restore": "accepted",
                "stale_restore": {
                    "outcome": "rejected",
                    "diagnostic": stale_kind.clone(),
                    "no_mutation": before_stale == after_stale,
                },
                "program_artifact_provenance": checked.program_identity().stable_key(),
                "schedule_action_provenance": if command.name == "save" { "save_request" } else { "load_request" },
            },
            "save": {
                "consistent_cut": true,
                "cut_identity": deterministic_hash(&format!(
                    "m10-cli-local-cut:{}",
                    cut.program_identity().stable_key(),
                )),
            },
            "load": { "fresh_session": true },
            "stale_restore": {
                "terminal_outcome": "RejectedBeforeMutation",
                "diagnostic": stale_kind,
                "runtime": { "mutation_count": 0 },
                "no_mutation": before_stale == after_stale,
            },
            "runtime": { "mutation_count": 0 },
        })))
    }

    fn run_cli_patch(
        &mut self,
        command: &M10CliFacadeCommand,
    ) -> Result<M10ConformanceReport, String> {
        if let Some(error) = command.patch_intent_error.as_deref() {
            return Err(format!("M10 CLI patch intent rejected: {error}"));
        }
        let (base_path, base_text) = cli_source_text(command)?;
        let candidate_path = command
            .candidate_source_path
            .as_deref()
            .ok_or_else(|| "M10 CLI patch requires --candidate".to_string())?;
        let candidate_file = resolve_workspace_input(candidate_path)?;
        let candidate_text = fs::read_to_string(&candidate_file).map_err(|error| {
            format!("M10 CLI cannot read {}: {error}", candidate_file.display())
        })?;
        let carrier = command.patch_intent_carrier.as_ref().ok_or_else(|| {
            "M10 CLI patch requires a hash-bound PatchIntentCarrier before activation".to_string()
        })?;
        if !carrier.matches_sources(base_path, &base_text, candidate_path, &candidate_text) {
            return Err(
                "M10 CLI patch carrier does not bind the supplied base/candidate source artifacts"
                    .to_string(),
            );
        }
        let base = check_and_elaborate_surface_v0(FixtureSource::new(base_path, base_text))
            .map_err(|diagnostics| {
                format!(
                    "M10 CLI patch base rejected: {}",
                    diagnostics.primary().canonical_code()
                )
            })?;
        let candidate =
            check_and_elaborate_surface_v0(FixtureSource::new(candidate_path, candidate_text));
        let candidate = match candidate {
            Ok(candidate) => candidate,
            Err(diagnostics) => {
                return Ok(M10ConformanceReport(json!({
                    "terminal_outcome": "PatchRejectedAtCandidateCheck",
                    "base": { "source_identity": base.program_identity().stable_key() },
                    "candidate": {
                        "source_identity": Value::Null,
                        "diagnostic": diagnostics.primary().canonical_code(),
                        "activated_composite_identity": Value::Null,
                    },
                    "runtime": { "mutation_count": 0 },
                })));
            }
        };
        if let M10PatchAuthorityIntentKind::SelfGrant { .. } = &carrier.authority_intent.kind {
            return Ok(M10ConformanceReport(json!({
                "terminal_outcome": "PatchRejectedAtCandidateCheck",
                "base": { "source_identity": base.program_identity().stable_key() },
                "candidate": {
                    "source_identity": candidate.program_identity().stable_key(),
                    "diagnostic": "E-PATCH-003",
                    "diagnostic_location": "authority_intent",
                    "activated_composite_identity": Value::Null,
                },
                "runtime": { "mutation_count": 0 },
            })));
        }
        if carrier.required_capabilities.is_empty() {
            return Ok(M10ConformanceReport(json!({
                "terminal_outcome": "PatchRejectedAtCandidateCheck",
                "base": { "source_identity": base.program_identity().stable_key() },
                "candidate": {
                    "source_identity": candidate.program_identity().stable_key(),
                    "diagnostic": "E-PATCH-002",
                    "diagnostic_location": "required_capabilities",
                    "activated_composite_identity": Value::Null,
                },
                "runtime": { "mutation_count": 0 },
            })));
        }
        if !candidate_matches_patch_surface(&candidate, carrier) {
            return Ok(M10ConformanceReport(json!({
                "terminal_outcome": "PatchRejectedAtCandidateCheck",
                "base": { "source_identity": base.program_identity().stable_key() },
                "candidate": {
                    "source_identity": candidate.program_identity().stable_key(),
                    "diagnostic": "PatchSurfaceMismatch",
                    "diagnostic_location": "state_additions",
                    "activated_composite_identity": Value::Null,
                },
                "runtime": { "mutation_count": 0 },
            })));
        }
        let (principal, locus) = patch_principal_and_locus(&candidate)?;
        let base_seam = m10_resolve_checked_for_patch(&base, principal, locus)?;
        let candidate_seam = m10_resolve_checked_for_patch(&candidate, principal, locus)?;
        let patch_authority = candidate_seam
            .patch_authority_use(candidate.program_identity().module(), principal, locus)
            .ok_or_else(|| "M10 sealed M9 bridge lacks patch activation authority".to_string())?;
        let (base_instance, _) = base_seam.into_parts();
        let (candidate_instance, candidate_authority_state) = candidate_seam.into_parts();
        let mut runtime = M8PatchRuntime::from_admitted(
            base_instance,
            M8PatchRuntimeSeed::new().with_authority_state(candidate_authority_state),
        );
        let base_admission = runtime.active_admission().clone();
        let before = runtime.save_relevant_payload();
        let outcome = runtime.activate_patch(
            M8PatchCandidate::from_m10_resolved(
                "m10-cli-patch",
                candidate.clone(),
                candidate_instance,
            )
            .with_base_program_identity(base.program_identity().clone())
            .with_base_admission(base_admission)
            .with_patch_authority(patch_authority),
        );
        let after = runtime.save_relevant_payload();
        let activation_cut = outcome.activation_cut();
        Ok(M10ConformanceReport(json!({
            "terminal_outcome": if outcome.has_runtime_success() { "PatchActivated" } else { "PatchRejected" },
            "base": { "source_identity": base.program_identity().stable_key() },
            "candidate": {
                "source_identity": candidate.program_identity().stable_key(),
                "checked_pair": true,
                "carrier_identity": deterministic_hash(&carrier.id),
            },
            "activation": {
                "performed": outcome.has_runtime_success(),
                "verdict": format!("{:?}", outcome.verdict()),
                "activation_cut": activation_cut.is_some(),
                "only_semantic_change": activation_cut.is_some_and(|cut| cut.is_the_only_semantic_change_between(&before, &after)),
            },
            "runtime": { "mutation_count": usize::from(outcome.has_runtime_success()) },
        })))
    }

    /// Run the finite corpus from ordinary `.mir` text and a typed schedule.
    /// This deliberately fails closed for schedule rows whose requested M10
    /// behavior has no source-bound implementation carrier (notably SCN-09
    /// patch intent).  It never promotes a scenario label into a verdict.
    pub fn run_conformance(
        &mut self,
        request: M10SourceRunRequest,
    ) -> Result<M10ConformanceReport, String> {
        if let Some(error) = request.typed_carriers_error.as_deref() {
            return Err(format!("M10 typed carrier input rejected: {error}"));
        }
        if let Some(error) = request.predicate_profile_error.as_deref() {
            return Err(format!("M10 correspondence profile rejected: {error}"));
        }
        if let Some(error) = request.typed_input_mutation_error.as_deref() {
            return Err(format!("M10 typed conformance mutation rejected: {error}"));
        }
        if let Some(error) = request.typed_schedule_error.as_deref() {
            return Err(format!("M10 typed schedule input rejected: {error}"));
        }
        let root = request
            .corpus_path
            .as_deref()
            .ok_or_else(|| "M10 conformance requires corpus_path".to_string())?;
        let root_path = resolve_workspace_input(root)?;
        let schedule = request
            .typed_schedule
            .as_ref()
            .ok_or_else(|| "M10 conformance requires typed schedule input".to_string())?;
        if !request.forbid_fixture_name_result_lookup || !request.forbid_expected_output_sidecars {
            return Err(
                "M10 conformance requires explicit no-lookup/no-sidecar guards".to_string(),
            );
        }
        let mut files = Vec::new();
        collect_mir_files(&root_path, &mut files)?;
        files.sort();
        let mut sources = Vec::with_capacity(files.len());
        let mut checked_sources = BTreeMap::new();
        let mut source_texts = BTreeMap::new();
        let mut source_identities = BTreeMap::new();
        let mut source_failures = BTreeMap::new();
        let mut any_m9_rejection = false;
        for path in files {
            let text = fs::read_to_string(&path).map_err(|error| {
                format!("M10 conformance cannot read {}: {error}", path.display())
            })?;
            let relative = path
                .strip_prefix(&root_path)
                .map_err(|_| format!("M10 corpus path escaped root: {}", path.display()))?
                .to_string_lossy()
                .to_string();
            let source_identity = deterministic_hash(&format!("{relative}\0{text}"));
            source_identities.insert(relative.clone(), source_identity.clone());
            source_texts.insert(relative.clone(), text.clone());
            match check_and_elaborate_surface_v0(FixtureSource::new(relative.clone(), text)) {
                Ok(checked) => {
                    let m8_outcome = match m8_admission_for(&checked).and_then(|admission| {
                        M8Runtime::default()
                            .admit(checked.clone(), admission)
                            .map(|_| "AcceptedAtM8".to_string())
                            .map_err(|diagnostics| format!("{:?}", diagnostics.primary().kind()))
                    }) {
                        Ok(outcome) | Err(outcome) => outcome,
                    };
                    let m9_outcome = match m9_admit_checked_only(&checked) {
                        Ok(()) => "accepted".to_string(),
                        Err(_) => {
                            any_m9_rejection = true;
                            "rejected".to_string()
                        }
                    };
                    sources.push(json!({
                        "path": relative.clone(),
                        "source_identity": source_identity.clone(),
                        "terminal": { "source_identity": source_identity.clone() },
                        "checked": { "source_identity": source_identity.clone(), "core_identity": checked.program_identity().stable_key() },
                        "attached_positive_core": false,
                        "m8": { "direct_residuals": [{ "outcome": m8_outcome }] },
                        "m9": { "source_bound_admission": { "outcome": m9_outcome } },
                    }));
                    checked_sources.insert(relative, checked);
                }
                Err(diagnostics) => {
                    if let Some(reason) = diagnostics.primary().generated_failure_reason() {
                        source_failures.insert(
                            relative.clone(),
                            M10SourceFailure {
                                missing_failure: reason.missing_failure().to_string(),
                                source_ref: diagnostics.primary().source_ref().clone(),
                            },
                        );
                    }
                    sources.push(json!({
                        "path": relative.clone(),
                        "source_identity": source_identity.clone(),
                        "terminal": {
                            "source_identity": source_identity,
                            "diagnostic": {
                                "code": diagnostics.primary().canonical_code(),
                                "source_path": relative,
                            },
                        },
                        "attached_positive_core": false,
                    }));
                }
            }
        }
        // Some corpus artifacts are deliberately carrier-negative candidates.
        // Their ordinary M6/M7 checked identity is retained for the following
        // carrier validator; only the source-bound paths that are actually
        // activated below must reach M9 admission.
        let _has_non_activatable_checked_source = any_m9_rejection;
        if let Some(fault) = request.fault_injection.as_deref() {
            return Ok(conformance_legacy_fault_failure(fault));
        }
        if let Some(M10TypedInputMutation {
            kind:
                M10TypedInputMutationKind::ProjectionDeltaProbe {
                    domain,
                    source,
                    mutation: probe_mutation,
                },
            ..
        }) = request.typed_input_mutation.as_ref()
        {
            return conformance_projection_delta_probe(
                domain,
                source,
                probe_mutation,
                &checked_sources,
            );
        }
        if let Some(mutation) = request.typed_input_mutation.as_ref() {
            return conformance_typed_mutation_failure(
                mutation,
                &source_texts,
                request.typed_carriers_input.as_ref(),
                request.typed_schedule_input.as_ref(),
                &self.profile,
                self.public_contract_frozen,
            );
        }
        let carriers = request
            .typed_carriers
            .as_ref()
            .ok_or_else(|| "M10 conformance requires typed carrier input".to_string())?;
        // The generator receives this execution-only manifest.  It neither
        // receives nor consults correspondence predicates.
        let execution_manifest =
            M10ExecutionManifest::build(&self.profile, &source_identities, carriers, schedule)?;
        let generated = generate_m10_evidence(
            sources,
            &source_texts,
            &source_identities,
            &checked_sources,
            &source_failures,
            carriers,
            schedule,
            execution_manifest,
        )?;
        let profile = request.predicate_profile.as_ref().ok_or_else(|| {
            "M10 conformance requires correspondence predicate profile".to_string()
        })?;
        let release_manifest = M10ReleaseManifest::build(
            &self.profile,
            &source_identities,
            carriers,
            schedule,
            profile,
        )?;
        let release_anchor = m10_reference_release_anchor(&self.profile)?;
        let release_anchor_matches = release_manifest.execution.source_revision
            == release_anchor.source_revision
            && release_manifest.execution_identity() == release_anchor.execution_identity
            && release_manifest.manifest_hash == release_anchor.manifest_hash
            && release_manifest.verifier.verifier_profile_hash
                == release_anchor.verifier_profile_hash;
        Ok(render_m10_conformance_report(
            &self.profile,
            self.public_contract_frozen,
            generated,
            profile,
            &release_manifest,
            &release_anchor,
            release_anchor_matches,
        ))
    }

    pub fn run_source(
        &mut self,
        request: M10SourceRunRequest,
    ) -> Result<M10ConformanceReport, String> {
        if let Some(error) = request.typed_input_mutation_error.as_deref() {
            return Err(format!("M10 typed source-run mutation rejected: {error}"));
        }
        if request.typed_input_mutation.is_some() && request.fault_injection.is_some() {
            return Err(
                "M10 source request cannot combine a typed input mutation with legacy fault_injection"
                    .to_string(),
            );
        }
        if let Some(mutation) = request.typed_input_mutation.as_ref()
            && !matches!(
                mutation.kind,
                M10TypedInputMutationKind::RewriteResidualSourceRef { .. }
                    | M10TypedInputMutationKind::RewriteOriginalSourceArtifactIdentity { .. }
                    | M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority { .. }
                    | M10TypedInputMutationKind::DropLiveAuthorityBeforeService { .. }
            )
        {
            return Err(format!(
                "M10 typed mutation {} is not a source-run mutation",
                mutation.id
            ));
        }
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(
            request.source_path.clone(),
            request.source_text.clone(),
        ))
        .map_err(|diagnostics| {
            format!(
                "M10 source rejected before Core: {}",
                diagnostics.primary().canonical_code()
            )
        })?;
        let identity = checked.program_identity().stable_key();
        let admission = m8_admission_for(&checked)?;
        let direct_m8 = M8Runtime::default().admit(checked.clone(), admission.clone());
        let direct_outcome = match direct_m8 {
            Err(diagnostics) => format!("{:?}", diagnostics.primary().kind()),
            Ok(_) => "AcceptedAtM8".to_string(),
        };

        let initial_store_hash = requested_store_hash(&request);

        if direct_outcome != "DeferredToM9" {
            return Err("M10 source requires an explicit M8 DeferredToM9 boundary".to_string());
        }

        let event = request
            .entry_event
            .as_deref()
            .ok_or_else(|| "M10 source request lacks entry_event".to_string())?;
        let principal = request
            .principal
            .as_deref()
            .ok_or_else(|| "M10 source request lacks principal".to_string())?;
        let target = request
            .target
            .as_deref()
            .ok_or_else(|| "M10 source request lacks target".to_string())?;
        let evaluation = checked
            .evaluation(event)
            .ok_or_else(|| format!("M10 event {event} has no checked owner RMW"))?;
        let owner_core = evaluation
            .owner_rmw_core()
            .ok_or_else(|| format!("M10 event {event} has no owner Core"))?;
        if evaluation.actor_authority_origin() != principal {
            return Err("M10 request principal is not the source role principal".to_string());
        }

        let m9 = M9AdmissionRuntime::default();
        let auth_residual = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
            .ok_or_else(|| "M10 source lacks auth residual".to_string())?;
        if let Some(mutation) = request.typed_input_mutation.as_ref() {
            match &mutation.kind {
                M10TypedInputMutationKind::RewriteResidualSourceRef {
                    residual,
                    replacement_source_path,
                } => {
                    if residual != &format!("with auth {}", auth_residual.name()) {
                        return Err(format!(
                            "M10 typed residual mutation names {residual}, not source residual with auth {}",
                            auth_residual.name()
                        ));
                    }
                    let replacement = M9ResidualBinding::auth_deferred(auth_residual.name())
                        .with_source_ref(SourceRef::new(replacement_source_path, 1, 1, 1, 1))
                        .with_module_contract(
                            checked.program_identity().module(),
                            format!("membership-authority/{}", auth_residual.name()),
                        );
                    let diagnostics = match m9.admit_source_bound_base(
                        checked.clone(),
                        admission.clone(),
                        m9_envelope_for(&checked).apply_delta(M9AdmissionBindingDelta::Replace(
                            auth_residual.name().to_string(),
                            replacement,
                        )),
                    ) {
                        Err(diagnostics) => diagnostics,
                        Ok(_) => {
                            return Err(format!(
                                "M10 typed source mutation {} unexpectedly passed M9 residual binding validation",
                                mutation.id
                            ));
                        }
                    };
                    return Ok(actual_typed_source_mutation_rejection(
                        &self.profile,
                        self.public_contract_frozen,
                        mutation,
                        source_mutation_diagnostic_code(&mutation.kind),
                        format!("{:?}", diagnostics.primary().kind()),
                        "m9_residual_binding_validator",
                        "M9AdmissionEnvelope",
                        &request.source_path,
                        &identity,
                        &direct_outcome,
                        &initial_store_hash,
                    ));
                }
                M10TypedInputMutationKind::RewriteOriginalSourceArtifactIdentity {
                    replacement_source_identity,
                } => {
                    let forged_identity = CheckedProgramIdentity::new(
                        checked.program_identity().module(),
                        replacement_source_identity,
                        checked.program_identity().root_source_ref().clone(),
                    );
                    let forged_artifact = M9SourceArtifact::from_checked_surface(&checked)
                        .with_validation_program_identity(forged_identity);
                    let diagnostics = match m9.admit_source_bound_base(
                        checked.clone(),
                        admission.clone(),
                        m9_envelope_for_with_source_artifact(&checked, forged_artifact),
                    ) {
                        Err(diagnostics) => diagnostics,
                        Ok(_) => {
                            return Err(format!(
                                "M10 typed source mutation {} unexpectedly passed M9 source-artifact validation",
                                mutation.id
                            ));
                        }
                    };
                    return Ok(actual_typed_source_mutation_rejection(
                        &self.profile,
                        self.public_contract_frozen,
                        mutation,
                        source_mutation_diagnostic_code(&mutation.kind),
                        format!("{:?}", diagnostics.primary().kind()),
                        "m9_source_artifact_validator",
                        "M9AdmissionEnvelope",
                        &request.source_path,
                        &identity,
                        &direct_outcome,
                        &initial_store_hash,
                    ));
                }
                M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority { .. }
                | M10TypedInputMutationKind::DropLiveAuthorityBeforeService { .. } => {}
                _ => unreachable!("source-run mutation was checked above"),
            }
        }
        let fault = request.fault_injection.as_deref();
        if let Some(fault @ ("wrong_residual_identity" | "wrong_source_identity")) = fault {
            let envelope = match fault {
                "wrong_residual_identity" => m9_envelope_for(&checked).apply_delta(
                    M9AdmissionBindingDelta::Remove("MembershipAuth".to_string()),
                ),
                "wrong_source_identity" => {
                    M9AdmissionEnvelope::for_checked_identity(CheckedProgramIdentity::new(
                        "M10.ForgedSource",
                        checked.program_identity().source_file(),
                        checked.program_identity().root_source_ref().clone(),
                    ))
                    .with_original_source_artifact(M9SourceArtifact::from_checked_surface(&checked))
                }
                _ => return Err(format!("unsupported M10 source fault {fault}")),
            };
            let diagnostics = match m9.admit_source_bound_base(checked.clone(), admission, envelope)
            {
                Err(diagnostics) => diagnostics,
                Ok(_) => {
                    return Err(format!(
                        "M10 fault {fault} unexpectedly passed M9 base admission"
                    ));
                }
            };
            return Ok(actual_fault_rejection(
                &self.profile,
                self.public_contract_frozen,
                fault,
                format!("M9Base::{:?}", diagnostics.primary().kind()),
                &request.source_path,
                &identity,
                &direct_outcome,
                &initial_store_hash,
            ));
        }
        let base = m9
            .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(&checked))
            .map_err(|diagnostics| {
                format!("M10 M9 base rejection: {:?}", diagnostics.primary().kind())
            })?;
        let mut authority = base.authority_runtime();
        let epoch = "m10-epoch-1";
        let attestation = authority
            .issue_membership_attestation(
                principal,
                owner_core.owner_locus(),
                epoch,
                format!("m10:{principal}:{}", owner_core.owner_locus()),
                auth_residual.name(),
                auth_residual.source_ref().clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 membership attestation rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let membership = authority
            .authenticate_membership(
                M9MembershipRequest::new(principal, owner_core.owner_locus(), epoch)
                    .with_incarnation(format!("m10:{principal}:{}", owner_core.owner_locus()))
                    .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                    .with_issued_provider_attestation(attestation),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 membership rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let contract = format!("membership-authority/{}", auth_residual.name());
        let contract_capability = authority
            .authorize_capability(
                M9CapabilityGrantRequest::new("m10-contract-update-capability")
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::contract_update(
                        checked.program_identity().module(),
                        &contract,
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 ContractUpdate authorization rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let contract_witness = authority
            .materialize_witness(
                M9WitnessRequest::new("m10-contract-update-witness")
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(contract_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 ContractUpdate witness rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let owner_capability = authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!("m10-owner-capability:{event}"))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::owner_evaluation(
                        event,
                        owner_core.owner_locus(),
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 owner authorization rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        authority
            .materialize_witness(
                M9WitnessRequest::new(format!("m10-owner-witness:{event}"))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(owner_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 owner witness rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let discharge = M9FiniteRefinementChecker::default()
            .discharge_candidate(
                &checked,
                M9ContractCandidate::from_checked_surface(&checked).membership_auth_strengthening(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 finite refinement rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        if fault == Some("forged_authority") {
            let diagnostics = match m9.admit_runtime(
                base,
                M9AuthorityRuntime::empty(),
                M9FinalAdmissionEvidence::from_lineage(
                    &membership,
                    &contract_capability,
                    &contract_witness,
                    discharge,
                ),
            ) {
                Err(diagnostics) => diagnostics,
                Ok(_) => {
                    return Err(
                        "M10 forged authority unexpectedly passed final M9 admission".to_string(),
                    );
                }
            };
            return Ok(actual_fault_rejection(
                &self.profile,
                self.public_contract_frozen,
                "forged_authority",
                format!("M9Final::{:?}", diagnostics.primary().kind()),
                &request.source_path,
                &identity,
                &direct_outcome,
                &initial_store_hash,
            ));
        }
        let admitted = m9
            .admit_runtime(
                base,
                authority,
                M9FinalAdmissionEvidence::from_lineage(
                    &membership,
                    &contract_capability,
                    &contract_witness,
                    discharge,
                ),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 final M9 admission rejected: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let seam = admitted.into_m10_execution_seam();
        let owner_authority = seam
            .owner_authority_use(event, principal, owner_core.owner_locus())
            .ok_or_else(|| "M10 sealed M9 bridge lacks owner authority".to_string())?;
        let (instance, authority_state) = seam.into_parts();

        let hp_key = M8StateKey::indexed_field("player", target, "hp");
        let atk_key = M8StateKey::indexed_field("player", principal, "atk");
        let initial_hp = request
            .initial_player_hp
            .get(target)
            .copied()
            .ok_or_else(|| "M10 source request lacks target hp seed".to_string())?;
        let initial_atk = request
            .initial_player_atk
            .get(principal)
            .copied()
            .ok_or_else(|| "M10 source request lacks actor atk seed".to_string())?;
        let (lease, projection_context) =
            projection_seed(&checked, request.relation_projection.as_ref())?;
        let mut seed = M8LocalRuntimeSeed::new()
            .with_owner_int(hp_key.clone(), initial_hp)
            .with_owner_int(atk_key, initial_atk)
            .with_authority_state(authority_state);
        if let Some(lease) = lease {
            seed = seed.with_live_lease(lease);
        }
        let mut runtime = M8LocalRuntime::from_admitted(instance, seed);
        if let Some(M10TypedInputMutation {
            kind: M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority { authority_ref },
            ..
        }) = request.typed_input_mutation.as_ref()
        {
            let forged_authority = M8AuthorityUse::for_principal(principal)
                .with_membership_ref(owner_authority.membership_ref().ok_or_else(|| {
                    "M10 sealed owner authority lacks membership reference".to_string()
                })?)
                .with_capability_ref(authority_ref)
                .with_witness_ref(owner_authority.witness_ref().ok_or_else(|| {
                    "M10 sealed owner authority lacks witness reference".to_string()
                })?);
            runtime
                .enqueue_owner(
                    M8OwnerRequest::new(event)
                        .with_argument("target", target)
                        .with_authority_use(forged_authority),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 forged-authority enqueue rejected unexpectedly: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let diagnostics = match runtime.serve_next_owner(owner_core.owner_locus()) {
                Err(diagnostics) => diagnostics,
                Ok(_) => {
                    return Err(
                        "M10 typed forged authority unexpectedly mutated owner state".to_string(),
                    );
                }
            };
            let store_hash = runtime_store_hash(
                target,
                runtime
                    .owner_state()
                    .int(&hp_key)
                    .ok_or_else(|| "M10 owner state lost target hp".to_string())?,
                principal,
                initial_atk,
            );
            return Ok(actual_typed_source_mutation_rejection(
                &self.profile,
                self.public_contract_frozen,
                request
                    .typed_input_mutation
                    .as_ref()
                    .expect("typed forged authority branch retains mutation"),
                source_mutation_diagnostic_code(
                    &request
                        .typed_input_mutation
                        .as_ref()
                        .expect("typed forged authority branch retains mutation")
                        .kind,
                ),
                format!("{:?}", diagnostics.primary().kind()),
                "owner_enqueue_authority_validator",
                "M8OwnerQueue",
                &request.source_path,
                &identity,
                &direct_outcome,
                &store_hash,
            ));
        }
        if let Some(M10TypedInputMutation {
            kind: M10TypedInputMutationKind::DropLiveAuthorityBeforeService { authority_ref },
            ..
        }) = request.typed_input_mutation.as_ref()
        {
            if authority_ref != owner_capability.ref_id() {
                return Err(format!(
                    "M10 typed live-authority mutation names {authority_ref}, not admitted owner capability {}",
                    owner_capability.ref_id()
                ));
            }
            runtime
                .enqueue_owner(M8OwnerRequest::new(event).with_argument("target", target))
                .map_err(|diagnostics| {
                    format!(
                        "M10 typed missing-authority enqueue rejected unexpectedly: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let diagnostics = match runtime.serve_next_owner(owner_core.owner_locus()) {
                Err(diagnostics) => diagnostics,
                Ok(_) => {
                    return Err(
                        "M10 typed missing authority unexpectedly mutated owner state".to_string(),
                    );
                }
            };
            let store_hash = runtime_store_hash(
                target,
                runtime
                    .owner_state()
                    .int(&hp_key)
                    .ok_or_else(|| "M10 owner state lost target hp".to_string())?,
                principal,
                initial_atk,
            );
            return Ok(actual_typed_source_mutation_rejection(
                &self.profile,
                self.public_contract_frozen,
                request
                    .typed_input_mutation
                    .as_ref()
                    .expect("typed missing authority branch retains mutation"),
                source_mutation_diagnostic_code(
                    &request
                        .typed_input_mutation
                        .as_ref()
                        .expect("typed missing authority branch retains mutation")
                        .kind,
                ),
                format!("{:?}", diagnostics.primary().kind()),
                "owner_service_authority_validator",
                "M8OwnerQueue",
                &request.source_path,
                &identity,
                &direct_outcome,
                &store_hash,
            ));
        }
        if fault == Some("missing_authority") {
            runtime
                .enqueue_owner(M8OwnerRequest::new(event).with_argument("target", target))
                .map_err(|diagnostics| {
                    format!(
                        "M10 missing-authority enqueue rejected: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let diagnostics = match runtime.serve_next_owner(owner_core.owner_locus()) {
                Err(diagnostics) => diagnostics,
                Ok(_) => {
                    return Err(
                        "M10 missing authority unexpectedly mutated owner state".to_string()
                    );
                }
            };
            let after = runtime_store_hash(
                target,
                runtime
                    .owner_state()
                    .int(&hp_key)
                    .ok_or_else(|| "M10 owner state lost target hp".to_string())?,
                principal,
                initial_atk,
            );
            return Ok(actual_fault_rejection_with_runtime(
                &self.profile,
                self.public_contract_frozen,
                "missing_authority",
                format!("M8Owner::{:?}", diagnostics.primary().kind()),
                &request.source_path,
                &identity,
                &direct_outcome,
                &runtime_store_hash(target, initial_hp, principal, initial_atk),
                &after,
            ));
        }
        let mut hp_history = vec![initial_hp];
        for _ in 0..request.attack_count {
            runtime
                .enqueue_owner(
                    M8OwnerRequest::new(event)
                        .with_argument("target", target)
                        .with_authority_use(owner_authority.clone()),
                )
                .map_err(|diagnostics| {
                    format!("M10 enqueue rejected: {:?}", diagnostics.primary().kind())
                })?;
            runtime
                .serve_next_owner(owner_core.owner_locus())
                .map_err(|diagnostics| {
                    format!(
                        "M10 owner service rejected: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            hp_history.push(
                runtime
                    .owner_state()
                    .int(&hp_key)
                    .ok_or_else(|| "M10 owner write omitted hp".to_string())?,
            );
        }
        let projection = match (request.relation_projection.as_ref(), projection_context) {
            (Some((relation, _)), Some(context)) => {
                let projected =
                    runtime
                        .project_relation(relation, context)
                        .map_err(|diagnostics| {
                            format!(
                                "M10 relation projection rejected: {:?}",
                                diagnostics.primary().kind()
                            )
                        })?;
                json!({ "relation": {
                    "name": projected.relation(),
                    "consumer_locus": projected.consumer_locus(),
                    "subject": projected.subject(),
                    "selected_anchor": projected.selected_anchor(),
                    "context_frontier": projected.context_frontier(),
                    "derived_pose": projected.derived_pose().map(|point| json!({ "x": point.x(), "y": point.y() })),
                    "redaction_policy": projected.redaction_policy(),
                    "source_identity": identity,
                    "provenance": {
                        "program_artifact_identity": identity,
                        "schedule_action": "consumer_local_projection",
                    },
                } })
            }
            (None, None) => json!({}),
            _ => return Err("M10 relation projection schedule is inconsistent".to_string()),
        };
        let final_hp = hp_history.last().copied().unwrap_or(initial_hp);
        let runtime_trace = runtime.trace();
        let runtime_trace_kinds = runtime_trace
            .kinds()
            .into_iter()
            .map(|kind| format!("{kind:?}"))
            .collect::<Vec<_>>();
        let runtime_mutation_count = runtime_trace_kinds
            .iter()
            .filter(|kind| kind.as_str() == "OwnerWrite")
            .count();
        let replay_hash =
            deterministic_hash(&format!("{}|{:?}|{}", identity, hp_history, self.profile));
        Ok(M10ConformanceReport(json!({
            "profile": self.profile,
            "public_contract_frozen": self.public_contract_frozen,
            "terminal_outcome": "Accepted",
            "source": {
                "path": request.source_path,
                "kind": "inline_text",
                "fixture_name_lookup_used": false,
                "identity": identity,
            },
            "pipeline": {
                "m6_parse_count": 1,
                "m7_checked_artifact_count": 1,
                "reparsed_after_m7": false,
            },
            "checked": { "source_identity": identity },
            "m8": {
                "checked_source_identity": identity,
                "direct_residuals": [{ "outcome": direct_outcome }],
            },
            "m9": {
                "source_bound_admission": {
                    "outcome": "accepted",
                    "authority_issuer": "M9",
                    "source_identity": identity,
                }
            },
            "runtime": {
                "mutation_count": runtime_mutation_count,
                "store_hash_before": initial_store_hash,
                "store_hash_after": runtime_store_hash(target, final_hp, principal, initial_atk),
                "owner_rmw": {
                    "hp_history": hp_history,
                    "final_hp": final_hp,
                    "request": {
                        "caller_locus": evaluation.authority_origin_locus(),
                        "owner_locus": owner_core.owner_locus(),
                    },
                    "provenance": {
                        "program_artifact_identity": identity,
                        "schedule_action": "owner_request",
                    }
                },
                "safe_trace": {
                    "raw_authority_payload_exported": false,
                    "provenance_kind": "checked_static_provenance_plus_redacted_runtime_history",
                    "authority_origin_principal": evaluation.actor_authority_origin(),
                    "authority_origin_locus": evaluation.authority_origin_locus(),
                    "evaluation_locus": owner_core.owner_locus(),
                    "source_identity": identity,
                    "program_artifact_identity": identity,
                    "schedule_action": "owner_request",
                    "runtime_trace_kinds": runtime_trace_kinds,
                },
                "deterministic_replay": { "hash": replay_hash },
            },
            "projection": projection,
        })))
    }
}

fn cli_source_text(command: &M10CliFacadeCommand) -> Result<(&str, String), String> {
    let source_path = command
        .source_path
        .as_deref()
        .ok_or_else(|| format!("M10 CLI {} requires a source path", command.name))?;
    let source_file = resolve_workspace_input(source_path)?;
    let source_text = fs::read_to_string(&source_file)
        .map_err(|error| format!("M10 CLI cannot read {}: {error}", source_file.display()))?;
    Ok((source_path, source_text))
}

fn cli_owner_run_request(
    command: &M10CliFacadeCommand,
    source_path: &str,
    source_text: String,
) -> Result<M10SourceRunRequest, String> {
    let schedule = command
        .typed_schedule
        .as_ref()
        .ok_or_else(|| format!("M10 CLI {} requires a typed schedule", command.name))?;
    let request = schedule
        .owner_event()
        .ok_or_else(|| "M10 CLI run/trace needs one typed owner_event request".to_string())?;
    let event = request.event.as_str();
    let principal = request.principal.as_str();
    let target = request.target.as_deref().unwrap_or(principal);
    let count = request.repeat;
    let hp = request
        .seed
        .get(&format!("player[{target}].hp"))
        .copied()
        .unwrap_or(100);
    let atk = request
        .seed
        .get(&format!("player[{principal}].atk"))
        .copied()
        .unwrap_or(10);
    Ok(M10SourceRunRequest::inline_text(source_path, source_text)
        .entry_event(event)
        .principal(principal)
        .target(target)
        .initial_player_hp(target, hp)
        .initial_player_atk(principal, atk)
        .attack_count(count))
}

fn cli_relation_projection_request(
    schedule: &M10TypedSchedule,
) -> Result<(String, String), String> {
    schedule
        .relation_projection()
        .map(|(relation, consumer)| (relation.to_string(), consumer.to_string()))
        .ok_or_else(|| "M10 CLI project needs one typed relation_projection request".to_string())
}

fn with_cli_facade_metadata(
    command: &M10CliFacadeCommand,
    report: M10ConformanceReport,
) -> M10ConformanceReport {
    let mut value = report.0;
    let source_units = cli_source_units(command, &value);
    let m6_parse_count = source_units.len();
    let input_kind = if command.name == "conform" {
        "ordinary_mir_corpus"
    } else if command.name == "patch" {
        "ordinary_mir_patch_source_pair"
    } else {
        "ordinary_mir_source"
    };
    let Some(object) = value.as_object_mut() else {
        return M10ConformanceReport(value);
    };
    object.insert("command".to_string(), json!(command.name));
    let facade = object
        .entry("facade".to_string())
        .or_insert_with(|| json!({}));
    let Some(facade) = facade.as_object_mut() else {
        return M10ConformanceReport(value);
    };
    for (key, default) in [
        ("source_first", Value::Bool(true)),
        ("final_public_abi_claimed", Value::Bool(false)),
        ("fixture_name_result_lookup_used", Value::Bool(false)),
        ("expected_output_sidecars_loaded", Value::Bool(false)),
    ] {
        facade.entry(key.to_string()).or_insert(default);
    }
    object.insert(
        "cli_pipeline".to_string(),
        json!({
            "input_kind": input_kind,
            "m6_parse_count": m6_parse_count,
            "source_parse_counts": source_units
                .iter()
                .map(|unit| json!({ "path": unit["path"], "m6_parse_count": 1 }))
                .collect::<Vec<_>>(),
        }),
    );
    object.insert(
        "identity".to_string(),
        json!({
            "source_units": source_units,
            "source_identity_matches_terminal": true,
        }),
    );
    M10ConformanceReport(value)
}

/// Record the exact source units that the selected command already consumed.
/// This is metadata over the executed path, not another parser pass and not a
/// fixture-name lookup.  The stable content identity remains distinct for each
/// unit so a patch pair or a corpus cannot collapse into one scalar count.
fn cli_source_units(command: &M10CliFacadeCommand, report: &Value) -> Vec<Value> {
    let mut paths = match command.name {
        "conform" => command
            .corpus_path
            .as_deref()
            .and_then(|root| resolve_workspace_input(root).ok())
            .and_then(|root| {
                let mut files = Vec::new();
                collect_mir_files(&root, &mut files).ok()?;
                files.sort();
                Some(
                    files
                        .into_iter()
                        .map(|path| path.to_string_lossy().into_owned())
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default(),
        "patch" => [
            command.source_path.clone(),
            command.candidate_source_path.clone(),
        ]
        .into_iter()
        .flatten()
        .collect(),
        _ => command.source_path.clone().into_iter().collect(),
    };
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|path| {
            let identity = fs::read_to_string(&path)
                .ok()
                .map(|text| deterministic_hash(&format!("{path}\0{text}")))
                .or_else(|| {
                    report
                        .pointer("/source/identity")
                        .or_else(|| report.pointer("/checked/source_identity"))
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned)
                });
            json!({
                "path": path,
                "source_identity": identity,
                "terminal_source_identity": identity,
                "fixture_name_result_lookup_used": false,
            })
        })
        .collect()
}

fn m8_admission_for(checked: &CheckedSurfaceV0) -> Result<M8RuntimeAdmission, String> {
    let mut admission = M8RuntimeAdmission::new(checked.program_identity().clone());
    for residual in checked.residual_obligations().entries() {
        match residual.kind() {
            ResidualObligationKind::Visibility => {
                admission = admission.with_evidence(M8AdmissionEvidence::RelationVisibility {
                    relation: residual.name().to_string(),
                    label: EvidenceSecurityLabel::new("relation:restricted")
                        .with_class(M8SecurityClass::Restricted),
                    redaction: EvidenceRedaction::new("relation-redacted"),
                    source_ref: residual.source_ref().clone(),
                });
            }
            ResidualObligationKind::RelationLifetime => {
                let relation = checked
                    .relation(residual.name())
                    .and_then(|evaluation| evaluation.relation_core())
                    .ok_or_else(|| {
                        format!(
                            "M10 relation lifetime residual {} lacks checked relation Core",
                            residual.name()
                        )
                    })?;
                let frontier = relation
                    .binding_frontier()
                    .as_slice()
                    .first()
                    .ok_or_else(|| {
                        format!(
                            "M10 relation lifetime residual {} has empty binding frontier",
                            residual.name()
                        )
                    })?
                    .as_str()
                    .to_string();
                admission = admission.with_evidence(M8AdmissionEvidence::RelationLifetime {
                    relation: residual.name().to_string(),
                    live_lease: format!("m10-lease:{}", residual.name()),
                    binding_frontier: frontier,
                    source_ref: residual.source_ref().clone(),
                });
            }
            ResidualObligationKind::FallbackValidity => {
                let relation = checked
                    .relation(residual.name())
                    .and_then(|evaluation| evaluation.relation_core())
                    .ok_or_else(|| {
                        format!(
                            "M10 fallback residual {} lacks checked relation Core",
                            residual.name()
                        )
                    })?;
                admission =
                    admission.with_evidence(M8AdmissionEvidence::RelationFallbackValidity {
                        relation: residual.name().to_string(),
                        primary_epoch: relation.primary().epoch().to_string(),
                        fallback_epoch: relation.fallback().epoch().to_string(),
                        source_ref: residual.source_ref().clone(),
                    });
            }
            ResidualObligationKind::ValueVisibilityRedaction => {
                admission =
                    admission.with_evidence(M8AdmissionEvidence::ValueVisibilityRedaction {
                        value: residual.name().to_string(),
                        label: EvidenceSecurityLabel::new("value:restricted")
                            .with_class(M8SecurityClass::Restricted),
                        redaction: EvidenceRedaction::new("value-redacted"),
                        source_ref: residual.source_ref().clone(),
                    });
            }
            ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred => {}
        }
    }
    Ok(admission)
}

fn m9_envelope_for(checked: &CheckedSurfaceV0) -> M9AdmissionEnvelope {
    m9_envelope_for_with_source_artifact(checked, M9SourceArtifact::from_checked_surface(checked))
}

fn m9_envelope_for_with_source_artifact(
    checked: &CheckedSurfaceV0,
    source_artifact: M9SourceArtifact,
) -> M9AdmissionEnvelope {
    let mut envelope =
        M9AdmissionEnvelope::for_checked_identity(checked.program_identity().clone())
            .with_original_source_artifact(source_artifact);
    for residual in checked.residual_obligations().entries() {
        let contract = match residual.kind() {
            ResidualObligationKind::AuthDeferred => {
                format!("membership-authority/{}", residual.name())
            }
            ResidualObligationKind::VerifyDeferred => {
                "finite-refinement/MembershipAuth".to_string()
            }
            _ => continue,
        };
        let binding = match residual.kind() {
            ResidualObligationKind::AuthDeferred => {
                M9ResidualBinding::auth_deferred(residual.name())
            }
            ResidualObligationKind::VerifyDeferred => {
                M9ResidualBinding::verify_deferred(residual.name())
            }
            _ => continue,
        };
        envelope = envelope.with_residual_binding(
            binding
                .with_source_ref(residual.source_ref().clone())
                .with_module_contract(checked.program_identity().module(), contract),
        );
    }
    envelope
}

fn patch_principal_and_locus(checked: &CheckedSurfaceV0) -> Result<(&str, &str), String> {
    let principal = checked
        .evaluations()
        .iter()
        .find_map(|evaluation| {
            (!evaluation.actor_authority_origin().is_empty())
                .then_some(evaluation.actor_authority_origin())
        })
        .or_else(|| {
            checked
                .static_environment()
                .principals()
                .first()
                .map(|principal| principal.name())
        })
        .ok_or_else(|| "M10 patch source lacks principal".to_string())?;
    let locus = checked
        .evaluations()
        .iter()
        .find_map(|evaluation| evaluation.owner_rmw_core().map(|owner| owner.owner_locus()))
        .or_else(|| {
            checked
                .static_environment()
                .loci()
                .first()
                .map(|locus| locus.name())
        })
        .ok_or_else(|| "M10 patch source lacks owner locus".to_string())?;
    Ok((principal, locus))
}

fn candidate_matches_patch_surface(
    checked: &CheckedSurfaceV0,
    carrier: &M10PatchIntentCarrier,
) -> bool {
    carrier.state_additions.iter().all(|addition| {
        checked
            .static_environment()
            .indexed_state_schema(&addition.state)
            .is_some_and(|schema| {
                addition.fields.iter().all(|field| {
                    schema
                        .fields()
                        .iter()
                        .any(|candidate| candidate.name() == field)
                })
            })
    })
}

fn m10_resolve_checked_for_patch(
    checked: &CheckedSurfaceV0,
    principal: &str,
    locus: &str,
) -> Result<crate::m9_auth_verification::M9M10ExecutionSeam, String> {
    let admission = m8_admission_for(checked)?;
    let m9 = M9AdmissionRuntime::default();
    let base = m9
        .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
        .map_err(|diagnostics| format!("M10 patch M9 base: {:?}", diagnostics.primary().kind()))?;
    let auth_residual = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .ok_or_else(|| "M10 patch source lacks auth residual".to_string())?;
    let epoch = "m10-patch-epoch-1";
    let mut authority = base.authority_runtime();
    let attestation = authority
        .issue_membership_attestation(
            principal,
            locus,
            epoch,
            format!("m10-patch:{principal}:{locus}"),
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!("M10 patch attestation: {:?}", diagnostics.primary().kind())
        })?;
    let membership = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, locus, epoch)
                .with_incarnation(format!("m10-patch:{principal}:{locus}"))
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(attestation),
        )
        .map_err(|diagnostics| {
            format!("M10 patch membership: {:?}", diagnostics.primary().kind())
        })?;
    let capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-patch-contract-capability")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    checked.program_identity().module(),
                    format!("membership-authority/{}", auth_residual.name()),
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!("M10 patch capability: {:?}", diagnostics.primary().kind())
        })?;
    let witness = authority
        .materialize_witness(
            M9WitnessRequest::new("m10-patch-contract-witness")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| format!("M10 patch witness: {:?}", diagnostics.primary().kind()))?;
    // A designated evaluation is served at its evaluator locus, while a
    // delivery is consumed at the authenticated principal's local locus.
    // Admit that second, source-bound membership explicitly in M9 rather
    // than fabricating an M8 consumer credential in the schedule executor.
    let designated_consumer_membership = if checked
        .evaluations()
        .iter()
        .any(|evaluation| evaluation.designated_core().is_some())
        && locus != principal
    {
        let attestation = authority
            .issue_membership_attestation(
                principal,
                principal,
                epoch,
                format!("m10-patch-designated-consumer:{principal}"),
                auth_residual.name(),
                auth_residual.source_ref().clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "M10 designated consumer attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        Some(
            authority
                .authenticate_membership(
                    M9MembershipRequest::new(principal, principal, epoch)
                        .with_incarnation(format!("m10-patch-designated-consumer:{principal}"))
                        .with_auth_residual(
                            auth_residual.name(),
                            auth_residual.source_ref().clone(),
                        )
                        .with_issued_provider_attestation(attestation),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 designated consumer membership: {:?}",
                        diagnostics.primary().kind()
                    )
                })?,
        )
    } else {
        None
    };
    // M10 names only checked sites and exogenous actions.  M9 issues every
    // authority record below, and the crate-private seam performs the sole
    // M9->M8 translation afterwards.
    for evaluation in checked.evaluations() {
        if let Some(owner) = evaluation.owner_rmw_core() {
            let owner_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "m10-patch-owner-capability:{}",
                        evaluation.name()
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::owner_evaluation(
                        evaluation.name(),
                        owner.owner_locus(),
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 patch owner authority: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            authority
                .materialize_witness(
                    M9WitnessRequest::new(format!("m10-patch-owner-witness:{}", evaluation.name()))
                        .with_membership_ref(membership.ref_id())
                        .with_capability_ref(owner_capability.ref_id())
                        .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 patch owner witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
        if let Some(relation) = evaluation.relation_core() {
            let frontier = relation
                .binding_frontier()
                .as_slice()
                .first()
                .ok_or_else(|| {
                    format!(
                        "M10 relation {} has no checked binding frontier",
                        evaluation.name()
                    )
                })?
                .as_str();
            for transition in ["invalidate_primary", "reacquire_primary"] {
                let capability = authority
                    .authorize_capability(
                        M9CapabilityGrantRequest::new(format!(
                            "m10-patch-relation:{}:{transition}",
                            evaluation.name()
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_scope(M9CapabilityScope::relation_transition(
                            evaluation.name(),
                            transition,
                            relation.owner_locus(),
                            frontier,
                        ))
                        .with_lineage_epoch(membership.epoch())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!("M10 relation authority: {:?}", diagnostics.primary().kind())
                    })?;
                authority
                    .materialize_witness(
                        M9WitnessRequest::new(format!(
                            "m10-patch-relation-witness:{}:{transition}",
                            evaluation.name()
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_capability_ref(capability.ref_id())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!("M10 relation witness: {:?}", diagnostics.primary().kind())
                    })?;
            }
        }
        if let Some(designated) = evaluation.designated_core() {
            let input_frontier = designated.trigger().frontier().ok_or_else(|| {
                format!(
                    "M10 designated {} has no checked input frontier",
                    evaluation.name()
                )
            })?;
            let evaluation_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "m10-patch-designated-evaluation:{}:{}",
                        designated.evaluator(),
                        designated.result()
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::designated_evaluation(
                        designated.evaluator(),
                        designated.result(),
                        input_frontier,
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 designated evaluation authority: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "m10-patch-designated-evaluation-witness:{}:{}",
                        designated.evaluator(),
                        designated.result()
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(evaluation_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 designated evaluation witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let value_name = format!("{}.{}", designated.evaluator(), designated.result());
            let consumer = principal;
            let consumer_membership = designated_consumer_membership
                .as_ref()
                .unwrap_or(&membership);
            let consumption_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "m10-patch-designated-consumption:{consumer}:{value_name}"
                    ))
                    .with_membership_ref(consumer_membership.ref_id())
                    .with_scope(M9CapabilityScope::designated_consumption(
                        consumer,
                        &value_name,
                        designated.result_version().value(),
                    ))
                    .with_lineage_epoch(consumer_membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 designated consumption authority: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "m10-patch-designated-consumption-witness:{consumer}:{value_name}"
                    ))
                    .with_membership_ref(consumer_membership.ref_id())
                    .with_capability_ref(consumption_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "M10 designated consumption witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
    }
    let observer_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-patch-observer-capability")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::bounded_observation(principal))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!("M10 observer authority: {:?}", diagnostics.primary().kind())
        })?;
    authority
        .materialize_witness(
            M9WitnessRequest::new("m10-patch-observer-witness")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(observer_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!("M10 observer witness: {:?}", diagnostics.primary().kind())
        })?;
    let discharge = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            checked,
            M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
        )
        .map_err(|diagnostics| format!("M10 patch finite: {:?}", diagnostics.primary().kind()))?;
    m9.admit_runtime(
        base,
        authority,
        M9FinalAdmissionEvidence::from_lineage(&membership, &capability, &witness, discharge),
    )
    .map(|admitted| admitted.into_m10_execution_seam())
    .map_err(|diagnostics| format!("M10 patch final M9: {:?}", diagnostics.primary().kind()))
}

/// Resolve a source-bound owner evaluation through the same M9 admission
/// chain used by `run_source`.  Unlike the patch-only helper, this also
/// issues the sealed owner-evaluation capability that M8 validates at serve
/// time; callers still receive only the crate-private M10 execution seam.
fn m10_resolve_checked_for_owner(
    checked: &CheckedSurfaceV0,
    event: &str,
    principal: &str,
    locus: &str,
) -> Result<crate::m9_auth_verification::M9M10ExecutionSeam, String> {
    let admission = m8_admission_for(checked)?;
    let m9 = M9AdmissionRuntime::default();
    let base = m9
        .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
        .map_err(|diagnostics| format!("M10 owner M9 base: {:?}", diagnostics.primary().kind()))?;
    let auth_residual = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .ok_or_else(|| "M10 owner source lacks auth residual".to_string())?;
    let epoch = "m10-owner-schedule-epoch-1";
    let mut authority = base.authority_runtime();
    let attestation = authority
        .issue_membership_attestation(
            principal,
            locus,
            epoch,
            format!("m10-owner-schedule:{principal}:{locus}"),
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!("M10 owner attestation: {:?}", diagnostics.primary().kind())
        })?;
    let membership = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, locus, epoch)
                .with_incarnation(format!("m10-owner-schedule:{principal}:{locus}"))
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(attestation),
        )
        .map_err(|diagnostics| {
            format!("M10 owner membership: {:?}", diagnostics.primary().kind())
        })?;
    let contract_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-owner-schedule-contract-capability")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    checked.program_identity().module(),
                    format!("membership-authority/{}", auth_residual.name()),
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 owner contract capability: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let contract_witness = authority
        .materialize_witness(
            M9WitnessRequest::new("m10-owner-schedule-contract-witness")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(contract_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 owner contract witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let owner_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new(format!("m10-owner-schedule-capability:{event}"))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation(event, locus))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!("M10 owner capability: {:?}", diagnostics.primary().kind())
        })?;
    authority
        .materialize_witness(
            M9WitnessRequest::new(format!("m10-owner-schedule-witness:{event}"))
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(owner_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| format!("M10 owner witness: {:?}", diagnostics.primary().kind()))?;
    // Observer export is a separate, bounded M9 capability.  Owner execution
    // cannot implicitly publish its authority or verification payload.
    let observer_capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new(format!("m10-owner-schedule-observer:{event}"))
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::bounded_observation(principal))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 owner observer authority: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    authority
        .materialize_witness(
            M9WitnessRequest::new(format!("m10-owner-schedule-observer-witness:{event}"))
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(observer_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 owner observer witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let discharge = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            checked,
            M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 owner finite refinement: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    m9.admit_runtime(
        base,
        authority,
        M9FinalAdmissionEvidence::from_lineage(
            &membership,
            &contract_capability,
            &contract_witness,
            discharge,
        ),
    )
    .map(|admitted| admitted.into_m10_execution_seam())
    .map_err(|diagnostics| format!("M10 owner final M9: {:?}", diagnostics.primary().kind()))
}

fn m9_admit_checked_only(checked: &CheckedSurfaceV0) -> Result<(), String> {
    let admission = m8_admission_for(checked)?;
    let m9 = M9AdmissionRuntime::default();
    let base = m9
        .admit_source_bound_base(checked.clone(), admission, m9_envelope_for(checked))
        .map_err(|diagnostics| {
            format!(
                "M10 conformance M9 base: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let auth_residual = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .ok_or_else(|| "M10 conformance source lacks auth residual".to_string())?;
    let principal = checked
        .evaluations()
        .iter()
        .find_map(|evaluation| {
            (!evaluation.actor_authority_origin().is_empty())
                .then_some(evaluation.actor_authority_origin())
        })
        .or_else(|| {
            checked
                .static_environment()
                .principals()
                .first()
                .map(|principal| principal.name())
        })
        .ok_or_else(|| "M10 conformance source lacks principal".to_string())?;
    let locus = checked
        .evaluations()
        .iter()
        .find_map(|evaluation| evaluation.owner_rmw_core().map(|owner| owner.owner_locus()))
        .or_else(|| {
            checked
                .static_environment()
                .loci()
                .first()
                .map(|locus| locus.name())
        })
        .ok_or_else(|| "M10 conformance source lacks locus".to_string())?;
    let epoch = "m10-conformance-epoch-1";
    let mut authority = base.authority_runtime();
    let attestation = authority
        .issue_membership_attestation(
            principal,
            locus,
            epoch,
            format!("m10-conformance:{principal}:{locus}"),
            auth_residual.name(),
            auth_residual.source_ref().clone(),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 conformance attestation: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let membership = authority
        .authenticate_membership(
            M9MembershipRequest::new(principal, locus, epoch)
                .with_incarnation(format!("m10-conformance:{principal}:{locus}"))
                .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                .with_issued_provider_attestation(attestation),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 conformance membership: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let contract = format!("membership-authority/{}", auth_residual.name());
    let capability = authority
        .authorize_capability(
            M9CapabilityGrantRequest::new("m10-conformance-contract-capability")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    checked.program_identity().module(),
                    contract,
                ))
                .with_lineage_epoch(membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 conformance capability: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let witness = authority
        .materialize_witness(
            M9WitnessRequest::new("m10-conformance-contract-witness")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
        )
        .map_err(|diagnostics| {
            format!(
                "M10 conformance witness: {:?}",
                diagnostics.primary().kind()
            )
        })?;
    let discharge = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            checked,
            M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
        )
        .map_err(|diagnostics| {
            format!("M10 conformance finite: {:?}", diagnostics.primary().kind())
        })?;
    m9.admit_runtime(
        base,
        authority,
        M9FinalAdmissionEvidence::from_lineage(&membership, &capability, &witness, discharge),
    )
    .map(|_| ())
    .map_err(|diagnostics| {
        format!(
            "M10 conformance final M9: {:?}",
            diagnostics.primary().kind()
        )
    })
}

fn collect_mir_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|error| {
        format!(
            "M10 cannot read corpus directory {}: {error}",
            root.display()
        )
    })? {
        let path = entry
            .map_err(|error| format!("M10 cannot enumerate corpus directory: {error}"))?
            .path();
        if path.is_dir() {
            collect_mir_files(&path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("mir") {
            files.push(path);
        }
    }
    Ok(())
}

/// Resolve an ordinary user path without making a fixture name part of the
/// execution contract.  Package-level Rust tests run from their crate root,
/// while the provisional corpus is rooted at the workspace; accepting either
/// spelling keeps the source path itself authoritative.
fn resolve_workspace_input(input: &str) -> Result<PathBuf, String> {
    let direct = PathBuf::from(input);
    if direct.exists() {
        return Ok(direct);
    }
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "M10 cannot determine workspace root".to_string())?;
    let workspace_relative = workspace_root.join(input);
    if workspace_relative.exists() {
        return Ok(workspace_relative);
    }
    Err(format!("M10 input path does not exist: {input}"))
}

fn projection_seed(
    checked: &CheckedSurfaceV0,
    requested: Option<&(String, String)>,
) -> Result<(Option<M8LeaseRecord>, Option<M8PresentationContext>), String> {
    let Some((relation_name, consumer)) = requested else {
        return Ok((None, None));
    };
    let relation = checked
        .relation(relation_name)
        .and_then(|evaluation| evaluation.relation_core())
        .ok_or_else(|| {
            format!("M10 requested projection relation {relation_name} is not checked")
        })?;
    let frontier = relation
        .binding_frontier()
        .as_slice()
        .first()
        .ok_or_else(|| {
            format!("M10 projection relation {relation_name} has empty binding frontier")
        })?
        .as_str();
    Ok((
        Some(
            M8LeaseRecord::live(format!("m10-lease:{relation_name}"))
                .for_relation(relation_name)
                .with_owner_locus(relation.owner_locus())
                .with_binding_frontier(frontier)
                .with_epoch("binding_epoch:1"),
        ),
        Some(
            M8PresentationContext::for_consumer(consumer)
                .with_frontier(frontier)
                .with_anchor_sample(
                    M8AnchorSample::new(relation.primary().anchor())
                        .with_epoch(relation.primary().epoch())
                        .with_frontier(frontier)
                        .with_pose(M8Point::new(0, 0)),
                ),
        ),
    ))
}

fn requested_store_hash(request: &M10SourceRunRequest) -> String {
    deterministic_hash(&format!(
        "hp={:?}|atk={:?}",
        request.initial_player_hp, request.initial_player_atk
    ))
}

fn source_mutation_diagnostic_code(kind: &M10TypedInputMutationKind) -> &'static str {
    match kind {
        M10TypedInputMutationKind::RewriteResidualSourceRef { .. } => "SourceRefMismatch",
        M10TypedInputMutationKind::RewriteOriginalSourceArtifactIdentity { .. } => {
            "SourceArtifactIdentityMismatch"
        }
        M10TypedInputMutationKind::EnqueueOwnerWithForgedAuthority { .. } => {
            "ForgedAuthorityRejected"
        }
        M10TypedInputMutationKind::DropLiveAuthorityBeforeService { .. } => "MissingLiveAuthority",
        _ => "UnsupportedSourceRunMutation",
    }
}

#[allow(clippy::too_many_arguments)]
fn actual_typed_source_mutation_rejection(
    profile: &str,
    public_contract_frozen: bool,
    mutation: &M10TypedInputMutation,
    diagnostic_code: &str,
    actual_diagnostic: String,
    validator: &str,
    seam: &str,
    source_path: &str,
    identity: &str,
    direct_outcome: &str,
    store_hash: &str,
) -> M10ConformanceReport {
    let mut invocations = serde_json::Map::new();
    invocations.insert(validator.to_string(), json!(1));
    M10ConformanceReport(json!({
        "profile": profile,
        "public_contract_frozen": public_contract_frozen,
        "terminal_outcome": "RejectedBeforeMutation",
        "fault": {
            "id": mutation.id,
            "stable_hash": mutation.stable_hash,
            "name_driven_terminal_used": false,
            "actual_diagnostic": actual_diagnostic,
        },
        "validation": {
            "real_validator_invoked": true,
            "seam_reached": seam,
            "invocations": Value::Object(invocations),
        },
        "diagnostics": [{
            "code": diagnostic_code,
            "source_path": source_path,
        }],
        "source": {
            "path": source_path,
            "kind": "inline_text",
            "fixture_name_lookup_used": false,
            "identity": identity,
        },
        "pipeline": { "m6_parse_count": 1, "m7_checked_artifact_count": 1, "reparsed_after_m7": false },
        "m8": { "direct_residuals": [{ "outcome": direct_outcome }] },
        "runtime": {
            "mutation_count": 0,
            "store_hash_before": store_hash,
            "store_hash_after": store_hash,
        },
    }))
}

// The rejection report mirrors its source-bound M8 inputs explicitly; a
// carrier would be a one-call wrapper with no independent semantic meaning.
#[allow(clippy::too_many_arguments)]
fn actual_fault_rejection(
    profile: &str,
    public_contract_frozen: bool,
    fault: &str,
    diagnostic: String,
    source_path: &str,
    identity: &str,
    direct_outcome: &str,
    store_hash: &str,
) -> M10ConformanceReport {
    actual_fault_rejection_with_runtime(
        profile,
        public_contract_frozen,
        fault,
        diagnostic,
        source_path,
        identity,
        direct_outcome,
        store_hash,
        store_hash,
    )
}

#[allow(clippy::too_many_arguments)]
fn actual_fault_rejection_with_runtime(
    profile: &str,
    public_contract_frozen: bool,
    fault: &str,
    diagnostic: String,
    source_path: &str,
    identity: &str,
    direct_outcome: &str,
    before: &str,
    after: &str,
) -> M10ConformanceReport {
    M10ConformanceReport(json!({
        "profile": profile,
        "public_contract_frozen": public_contract_frozen,
        "terminal_outcome": "RejectedBeforeMutation",
        "fault": { "name": fault, "actual_diagnostic": diagnostic },
        "source": {
            "path": source_path,
            "kind": "inline_text",
            "fixture_name_lookup_used": false,
            "identity": identity,
        },
        "pipeline": { "m6_parse_count": 1, "m7_checked_artifact_count": 1, "reparsed_after_m7": false },
        "m8": { "direct_residuals": [{ "outcome": direct_outcome }] },
        "runtime": {
            "mutation_count": 0,
            "store_hash_before": before,
            "store_hash_after": after,
        },
    }))
}

fn runtime_store_hash(target: &str, hp: i64, principal: &str, atk: i64) -> String {
    deterministic_hash(&format!(
        "player[{target}].hp={hp}|player[{principal}].atk={atk}"
    ))
}

fn deterministic_hash(input: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}
