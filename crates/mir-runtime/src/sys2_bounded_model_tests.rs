use crate::sys2_bounded_model::{
    AuthorityLineage, AuthorityStatus, BadOutcome, Counterexample, ExecutionProfile, LitmusCase,
    ModelCheckReport, ModelState, ModelTrace, ModelTransition, ReplayError, RequiredEdge,
    Sys2BoundedModel,
};

const SYS2_BOUND: usize = 6;
const EVIDENCE_LABEL: &str = "model-checked-bounded";
const STORE_BUFFERING_ZERO_ZERO: &str = "r1=0,r2=0";
const ATTACK_LINEAGE: &str = "self:S:attack";
const HP_KEY: &str = "player[target].hp";

fn required_edges() -> Vec<RequiredEdge> {
    vec![
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
    ]
}

fn required_litmus_cases() -> Vec<LitmusCase> {
    vec![
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
    ]
}

fn full_sys2_model_with_bound(bound: usize) -> Sys2BoundedModel {
    Sys2BoundedModel::new()
        .with_profile(ExecutionProfile::OneOwnerWorker)
        .with_bound(bound)
        .with_required_edges(required_edges())
        .with_litmus_cases(required_litmus_cases())
}

fn full_sys2_model() -> Sys2BoundedModel {
    full_sys2_model_with_bound(SYS2_BOUND)
}

fn model_without_edge(edge: RequiredEdge) -> Sys2BoundedModel {
    Sys2BoundedModel::new()
        .with_profile(ExecutionProfile::OneOwnerWorker)
        .with_bound(SYS2_BOUND)
        .with_required_edges(
            required_edges()
                .into_iter()
                .filter(|candidate| *candidate != edge),
        )
        .with_litmus_cases(required_litmus_cases())
}

fn weak_memory_calibration(
    required_edges: impl IntoIterator<Item = RequiredEdge>,
) -> Sys2BoundedModel {
    Sys2BoundedModel::new()
        .with_profile(ExecutionProfile::WeakMemoryCalibration)
        .with_bound(SYS2_BOUND)
        .with_required_edges(required_edges)
        .with_litmus_case(LitmusCase::store_buffering_calibration())
}

fn assert_bounded_metadata(report: &ModelCheckReport) {
    assert_eq!(report.evidence_label(), EVIDENCE_LABEL);
    assert_eq!(report.bound(), SYS2_BOUND);
    assert!(
        report.abstraction_summary().contains("finite"),
        "SYS-2 model-check evidence must expose the finite abstraction, got: {}",
        report.abstraction_summary()
    );
    assert!(
        !report.abstraction_summary().contains("theorem")
            && !report.abstraction_summary().contains("proof"),
        "bounded model evidence must not be reported as proof/theorem: {}",
        report.abstraction_summary()
    );
}

fn assert_typed_state_space(report: &ModelCheckReport) {
    let initial_states: &[ModelState] = report.initial_states();
    let transitions: &[ModelTransition] = report.transition_relation();

    assert!(!initial_states.is_empty());
    assert!(!transitions.is_empty());
    assert!(
        transitions
            .iter()
            .all(
                |transition: &ModelTransition| transition.source_state_id().is_some()
                    && transition.target_state_id().is_some()
                    && transition.litmus_case().is_some()
            ),
        "SYS-2 must expose typed finite transitions, not a static edge->trace table"
    );
}

fn assert_attack_lineage(lineage: &AuthorityLineage) {
    assert_eq!(lineage.principal(), "self");
    assert_eq!(lineage.membership_epoch(), "epoch1");
    assert_eq!(
        lineage.membership_incarnation(),
        "incarnation:self:S:epoch1"
    );
    assert_eq!(lineage.capability_ref(), "cap:attack:S:self:epoch1");
    assert_eq!(lineage.witness_ref(), "witness:attack:S:self:epoch1");
}

fn assert_full_edge_authority_invariants(report: &ModelCheckReport) {
    assert!(report.no_source_free_authority_mints());
    assert!(report.no_stale_authority_use());
    assert!(report.rejected_authority_use_mutations().is_empty());

    let initial = report
        .initial_states()
        .iter()
        .find(|state: &&ModelState| state.authority_lineage(ATTACK_LINEAGE).is_some())
        .expect("SYS-2 initial states must include the owner attack authority lineage");
    let lineage = initial
        .authority_lineage(ATTACK_LINEAGE)
        .expect("owner attack lineage exists");
    assert_attack_lineage(lineage);
    assert_eq!(lineage.capability_status(), AuthorityStatus::Active);
    assert_eq!(lineage.witness_status(), AuthorityStatus::Active);
}

fn assert_tampered_replay_is_not_same_bad_state(counterexample: &Counterexample) {
    let transitions = counterexample.transitions();
    assert!(
        transitions.len() >= 2,
        "counterexample must contain enough typed transitions to detect table-shaped traces"
    );

    let mut dropped = transitions.to_vec();
    dropped.remove(0);
    assert_replay_fails_or_does_not_reach_bad_state(counterexample, dropped);

    let mut reordered = transitions.to_vec();
    reordered.swap(0, 1);
    assert_replay_fails_or_does_not_reach_bad_state(counterexample, reordered);
}

fn assert_replay_fails_or_does_not_reach_bad_state(
    counterexample: &Counterexample,
    transitions: Vec<ModelTransition>,
) {
    let replay: Result<ModelTrace, ReplayError> =
        ModelTrace::replay(counterexample.initial_state().clone(), transitions);
    match replay {
        Ok(trace) => {
            assert_ne!(trace.reached_state(), counterexample.reached_state());
            assert!(
                !counterexample.bad_predicate().holds(trace.reached_state()),
                "tampered replay must not reach the same concrete bad predicate"
            );
        }
        Err(error) => {
            assert!(error.is_invalid_transition_sequence());
        }
    }
}

fn assert_counterexample(
    report: &ModelCheckReport,
    missing_edge: RequiredEdge,
    bad_outcome: BadOutcome,
) -> Counterexample {
    assert_bounded_metadata(report);
    assert_eq!(report.profile(), ExecutionProfile::OneOwnerWorker);
    assert!(
        report.has_violations(),
        "removing {missing_edge:?} must violate SYS-2"
    );
    let counterexample: Counterexample = report
        .counterexamples()
        .iter()
        .find(|candidate: &&Counterexample| {
            candidate.missing_required_edge() == missing_edge
                && candidate.bad_outcome() == bad_outcome
        })
        .cloned()
        .unwrap_or_else(|| {
            panic!("missing concrete counterexample for {missing_edge:?}/{bad_outcome:?}")
        });
    assert_eq!(counterexample.bad_predicate().outcome(), bad_outcome);
    assert!(
        !counterexample
            .bad_predicate()
            .holds(counterexample.initial_state()),
        "bad predicate must not already hold in the initial state"
    );
    assert!(
        counterexample
            .bad_predicate()
            .holds(counterexample.reached_state()),
        "bad predicate must be checked against the reached typed state"
    );
    assert!(
        counterexample.trace().events().len() >= 2,
        "counterexample for {missing_edge:?} must include a concrete trace"
    );
    assert!(
        counterexample.trace().mentions_required_edge(missing_edge),
        "counterexample trace must name missing edge {missing_edge:?}: {:?}",
        counterexample.trace().events()
    );
    let replayed: ModelTrace = ModelTrace::replay(
        counterexample.initial_state().clone(),
        counterexample.transitions().to_vec(),
    )
    .expect("counterexample typed transitions must replay");
    assert_eq!(replayed.reached_state(), counterexample.reached_state());
    assert_tampered_replay_is_not_same_bad_state(&counterexample);
    counterexample
}

#[test]
fn full_edge_sys2_model_has_complete_typed_bounded_search() {
    let report = full_sys2_model().check();

    assert_bounded_metadata(&report);
    assert_eq!(report.profile(), ExecutionProfile::OneOwnerWorker);
    assert_typed_state_space(&report);
    assert!(report.search_complete_within_bound());
    assert!(report.passes_all_litmus());
    assert!(report.visited_state_count() > 0);
    assert!(report.transition_count() > 0);
    assert_eq!(report.litmus_count(), required_litmus_cases().len());
    assert!(
        report.violations().is_empty(),
        "complete high-level SYS-2 edge set should satisfy selected finite litmus cases: {:?}",
        report.violations()
    );
    assert_full_edge_authority_invariants(&report);
    assert!(
        report
            .case_report("capability_revoke_use_race")
            .expect("revocation litmus report exists")
            .rejected_state_mutations()
            .is_empty(),
        "full-edge revocation handling must reject stale g0 use without mutating semantic state"
    );
}

#[test]
fn bound_zero_or_insufficient_search_cannot_claim_pass() {
    let zero_bound = full_sys2_model_with_bound(0).check();

    assert_eq!(zero_bound.evidence_label(), EVIDENCE_LABEL);
    assert_eq!(zero_bound.profile(), ExecutionProfile::OneOwnerWorker);
    assert_eq!(zero_bound.bound(), 0);
    assert!(!zero_bound.search_complete_within_bound());
    assert!(!zero_bound.passes_all_litmus());
    assert!(zero_bound.bound_status().is_insufficient());
}

#[test]
fn store_buffering_zero_zero_is_calibration_not_implicit_failure() {
    let relaxed_report = weak_memory_calibration(
        required_edges()
            .into_iter()
            .filter(|edge| *edge != RequiredEdge::PublishObserve),
    )
    .check();

    assert_bounded_metadata(&relaxed_report);
    assert_eq!(
        relaxed_report.profile(),
        ExecutionProfile::WeakMemoryCalibration
    );
    assert!(!relaxed_report.claims_sequential_consistency());
    assert!(
        relaxed_report
            .case_report("store_buffering_calibration")
            .expect("store-buffering report exists")
            .allows_observable_outcome(STORE_BUFFERING_ZERO_ZERO),
        "0/0 is legal without a declared publish->observe edge"
    );
    assert!(
        relaxed_report.violations().is_empty(),
        "store-buffering calibration must not invent a Mir edge"
    );

    let publication_report = weak_memory_calibration([RequiredEdge::PublishObserve]).check();

    assert_bounded_metadata(&publication_report);
    assert_eq!(
        publication_report.profile(),
        ExecutionProfile::WeakMemoryCalibration
    );
    assert!(!publication_report.claims_sequential_consistency());
    assert!(
        publication_report
            .case_report("store_buffering_calibration")
            .expect("store-buffering report exists")
            .forbids_observable_outcome(STORE_BUFFERING_ZERO_ZERO),
        "declared publish->observe edge must forbid the 0/0 store-buffering outcome"
    );
}

#[test]
fn missing_owner_request_serve_edge_reports_serve_without_request() {
    let report = model_without_edge(RequiredEdge::OwnerRequestServe).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::OwnerRequestServe,
        BadOutcome::ServeWithoutPriorRequest,
    );

    assert_eq!(counterexample.reached_state().request_count("attack"), 0);
    assert_eq!(counterexample.reached_state().serve_count("attack"), 1);
    assert_eq!(
        counterexample.reached_state().semantic_generation(HP_KEY),
        Some(0),
        "serving without an admitted request must not be encoded as a valid owner mutation"
    );
}

#[test]
fn missing_publish_observe_edge_reports_stale_observation() {
    let report = model_without_edge(RequiredEdge::PublishObserve).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::PublishObserve,
        BadOutcome::ObservedBeforePublish,
    );

    let reached = counterexample.reached_state();
    assert_eq!(reached.published_version("result"), Some(1));
    assert_eq!(reached.observed_version("result", "ViewerC"), Some(0));
    assert!(
        reached.observed_version("result", "ViewerC") < reached.published_version("result"),
        "stale observation must be a concrete version violation"
    );
}

#[test]
fn missing_witness_create_use_edge_reports_stale_lineage_without_rejected_mutation() {
    let report = model_without_edge(RequiredEdge::WitnessCreateUse).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::WitnessCreateUse,
        BadOutcome::WitnessUseBeforeCreate,
    );

    let lineage = counterexample
        .reached_state()
        .authority_lineage(ATTACK_LINEAGE)
        .expect("counterexample must carry typed attack authority lineage");
    assert_attack_lineage(lineage);
    assert_eq!(lineage.witness_status(), AuthorityStatus::Stale);
    assert_eq!(lineage.capability_status(), AuthorityStatus::Active);
    assert!(
        counterexample
            .reached_state()
            .rejected_authority_use_mutations()
            .is_empty(),
        "rejected stale witness use must not mutate semantic owner state"
    );
}

#[test]
fn missing_capability_grant_use_edge_reports_mismatched_lineage_without_rejected_mutation() {
    let report = model_without_edge(RequiredEdge::CapabilityGrantUse).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::CapabilityGrantUse,
        BadOutcome::CapabilityUseBeforeGrant,
    );

    let lineage = counterexample
        .reached_state()
        .authority_lineage(ATTACK_LINEAGE)
        .expect("counterexample must carry typed attack authority lineage");
    assert_eq!(lineage.principal(), "self");
    assert_eq!(lineage.membership_epoch(), "epoch1");
    assert_eq!(lineage.capability_status(), AuthorityStatus::Revoked);
    assert_eq!(lineage.witness_status(), AuthorityStatus::Active);
    assert_ne!(
        lineage.capability_ref(),
        lineage.witness_capability_ref(),
        "missing grant->use edge must expose a typed capability/witness lineage mismatch"
    );
    assert!(
        counterexample
            .reached_state()
            .rejected_authority_use_mutations()
            .is_empty(),
        "rejected mismatched capability use must not mutate semantic owner state"
    );
}

#[test]
fn missing_revocation_visibility_edge_reports_stale_serve_write() {
    let report = model_without_edge(RequiredEdge::RevocationVisibility).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::RevocationVisibility,
        BadOutcome::StaleServeAfterRevocation,
    );

    assert_eq!(
        counterexample.trace().event_names(),
        ["enqueue@g0", "revoke_publish@g1", "stale_serve_write@g0",],
        "primary SYS-2 revocation counterexample must be enqueue@g0 -> revoke publish@g1 -> stale serve write"
    );
    let reached = counterexample.reached_state();
    let lineage = reached
        .authority_lineage(ATTACK_LINEAGE)
        .expect("revocation counterexample carries typed lineage");
    assert_eq!(lineage.capability_status(), AuthorityStatus::Revoked);
    assert_eq!(lineage.revocation_generation(), Some(1));
    assert_eq!(reached.used_authority_generation(ATTACK_LINEAGE), Some(0));
    assert_eq!(reached.semantic_generation(HP_KEY), Some(1));
    assert_eq!(reached.int(HP_KEY), Some(90));
    assert_eq!(
        reached.last_mutation_source_generation(HP_KEY),
        Some(0),
        "bad state is a stale g0 owner write after revoke publish at g1"
    );

    let full_report = full_sys2_model().check();
    assert!(
        !full_report
            .case_report("capability_revoke_use_race")
            .expect("revocation litmus report exists")
            .has_semantic_mutation_after_revocation(HP_KEY),
        "full-edge revocation path must reject stale g0 use instead of mutating hp"
    );
}

#[test]
fn missing_patch_activation_visibility_edge_reports_stale_patch_frontier() {
    let report = model_without_edge(RequiredEdge::PatchActivationVisibility).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::PatchActivationVisibility,
        BadOutcome::RequestCrossesPatchActivationFrontier,
    );

    assert_eq!(
        counterexample.reached_state().active_patch_generation(),
        Some(1)
    );
    assert_eq!(
        counterexample
            .reached_state()
            .request_patch_generation("attack"),
        Some(0),
        "bad patch race must be a concrete stale-frontier request"
    );
}

#[test]
fn missing_cut_save_quiescence_edge_reports_save_mutation_race() {
    let report = model_without_edge(RequiredEdge::CutSaveQuiescence).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::CutSaveQuiescence,
        BadOutcome::MutationEscapesSaveCut,
    );

    assert_eq!(
        counterexample.reached_state().save_cut_generation(),
        Some(1)
    );
    assert!(
        counterexample
            .reached_state()
            .has_mutation_after_save_cut(HP_KEY),
        "bad save/cut race must record a concrete post-cut owner mutation"
    );
}

#[test]
fn missing_relation_epoch_sample_edge_reports_mixed_epoch_sample() {
    let report = model_without_edge(RequiredEdge::RelationEpochSample).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::RelationEpochSample,
        BadOutcome::RelationSampleMixesEpochs,
    );

    assert_eq!(
        counterexample
            .reached_state()
            .relation_sample_epochs("bird_follow", "ViewerC"),
        Some(("primary_epoch:1", "fallback_epoch:0")),
        "bad relation sample must expose mixed typed relation epochs"
    );
}

#[test]
fn missing_same_owner_coherence_edge_reports_rmw_reads_from_break() {
    let report = model_without_edge(RequiredEdge::SameOwnerReadsFromCoherence).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::SameOwnerReadsFromCoherence,
        BadOutcome::SameOwnerRmwReadsFromStaleWrite,
    );

    assert_eq!(
        counterexample
            .reached_state()
            .reads_from_version("attack#2", HP_KEY),
        Some(0),
        "second same-owner RMW must expose the stale reads-from version"
    );
    assert_eq!(
        counterexample.reached_state().coherence_version(HP_KEY),
        Some(1),
        "owner-local coherence order must have already advanced"
    );
}

#[test]
fn missing_presentation_gap_nonmutation_edge_reports_semantic_mutation() {
    let report = model_without_edge(RequiredEdge::PresentationGapNonmutation).check();
    let counterexample = assert_counterexample(
        &report,
        RequiredEdge::PresentationGapNonmutation,
        BadOutcome::PresentationGapMutatesSemanticLineage,
    );

    assert_eq!(
        counterexample
            .reached_state()
            .presentation_gap_count("ViewerC"),
        1
    );
    assert!(
        counterexample
            .reached_state()
            .semantic_lineage_changed_by_presentation_gap("bird_follow"),
        "presentation gap counterexample must expose a concrete semantic lineage mutation"
    );
}
