use crate::sys2_bounded_model::{
    BadOutcome, Counterexample, LitmusCase, ModelCheckReport, RequiredEdge, Sys2BoundedModel,
};

const SYS2_BOUND: usize = 6;
const EVIDENCE_LABEL: &str = "model-checked-bounded";
const STORE_BUFFERING_ZERO_ZERO: &str = "r1=0,r2=0";

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

fn full_sys2_model() -> Sys2BoundedModel {
    Sys2BoundedModel::new()
        .with_profile("OW")
        .with_bound(SYS2_BOUND)
        .with_required_edges(required_edges())
        .with_litmus_cases(required_litmus_cases())
}

fn model_without_edge(edge: RequiredEdge) -> Sys2BoundedModel {
    Sys2BoundedModel::new()
        .with_profile("OW")
        .with_bound(SYS2_BOUND)
        .with_required_edges(
            required_edges()
                .into_iter()
                .filter(|candidate| *candidate != edge),
        )
        .with_litmus_cases(required_litmus_cases())
}

fn assert_bounded_metadata(report: &ModelCheckReport) {
    assert_eq!(report.evidence_label(), EVIDENCE_LABEL);
    assert_eq!(report.profile(), "OW");
    assert_eq!(report.bound(), SYS2_BOUND);
    assert!(
        report.abstraction().contains("finite"),
        "SYS-2 model-check evidence must expose the finite abstraction, got: {}",
        report.abstraction()
    );
    assert!(
        !report.abstraction().contains("theorem") && !report.abstraction().contains("proof"),
        "bounded model evidence must not be reported as proof/theorem: {}",
        report.abstraction()
    );
}

fn assert_counterexample(
    report: &ModelCheckReport,
    missing_edge: RequiredEdge,
    bad_outcome: BadOutcome,
) -> Counterexample {
    assert_bounded_metadata(report);
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
    assert!(
        counterexample.trace().events().len() >= 2,
        "counterexample for {missing_edge:?} must include a concrete trace"
    );
    assert!(
        counterexample.trace().mentions_required_edge(missing_edge),
        "counterexample trace must name missing edge {missing_edge:?}: {:?}",
        counterexample.trace().events()
    );
    counterexample
}

#[test]
fn full_edge_sys2_model_has_no_violations_and_nonzero_search() {
    let report = full_sys2_model().check();

    assert_bounded_metadata(&report);
    assert!(report.visited_state_count() > 0);
    assert!(report.transition_count() > 0);
    assert_eq!(report.litmus_count(), required_litmus_cases().len());
    assert!(
        report.violations().is_empty(),
        "complete high-level SYS-2 edge set should satisfy selected finite litmus cases: {:?}",
        report.violations()
    );
}

#[test]
fn store_buffering_zero_zero_is_calibration_not_implicit_failure() {
    let relaxed_report = Sys2BoundedModel::new()
        .with_profile("OW")
        .with_bound(SYS2_BOUND)
        .with_required_edges(
            required_edges()
                .into_iter()
                .filter(|edge| *edge != RequiredEdge::PublishObserve),
        )
        .with_litmus_case(LitmusCase::store_buffering_calibration())
        .check();

    assert_bounded_metadata(&relaxed_report);
    assert!(
        relaxed_report
            .allowed_outcomes("store_buffering_calibration")
            .contains(STORE_BUFFERING_ZERO_ZERO),
        "0/0 is legal without a declared publish->observe edge"
    );
    assert!(
        relaxed_report.violations().is_empty(),
        "store-buffering calibration must not invent a Mir edge"
    );

    let publication_report = Sys2BoundedModel::new()
        .with_profile("OW")
        .with_bound(SYS2_BOUND)
        .with_required_edges([RequiredEdge::PublishObserve])
        .with_litmus_case(LitmusCase::store_buffering_calibration())
        .check();

    assert_bounded_metadata(&publication_report);
    assert!(
        publication_report
            .forbidden_outcomes("store_buffering_calibration")
            .contains(STORE_BUFFERING_ZERO_ZERO),
        "declared publish->observe edge must forbid the 0/0 store-buffering outcome"
    );
}

#[test]
fn missing_owner_request_serve_edge_reports_serve_without_request() {
    let report = model_without_edge(RequiredEdge::OwnerRequestServe).check();

    assert_counterexample(
        &report,
        RequiredEdge::OwnerRequestServe,
        BadOutcome::ServeWithoutPriorRequest,
    );
}

#[test]
fn missing_publish_observe_edge_reports_stale_observation() {
    let report = model_without_edge(RequiredEdge::PublishObserve).check();

    assert_counterexample(
        &report,
        RequiredEdge::PublishObserve,
        BadOutcome::ObservedBeforePublish,
    );
}

#[test]
fn missing_witness_create_use_edge_reports_use_before_witness() {
    let report = model_without_edge(RequiredEdge::WitnessCreateUse).check();

    assert_counterexample(
        &report,
        RequiredEdge::WitnessCreateUse,
        BadOutcome::WitnessUseBeforeCreate,
    );
}

#[test]
fn missing_capability_grant_use_edge_reports_ungranted_use() {
    let report = model_without_edge(RequiredEdge::CapabilityGrantUse).check();

    assert_counterexample(
        &report,
        RequiredEdge::CapabilityGrantUse,
        BadOutcome::CapabilityUseBeforeGrant,
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
}

#[test]
fn missing_patch_activation_visibility_edge_reports_stale_patch_frontier() {
    let report = model_without_edge(RequiredEdge::PatchActivationVisibility).check();

    assert_counterexample(
        &report,
        RequiredEdge::PatchActivationVisibility,
        BadOutcome::RequestCrossesPatchActivationFrontier,
    );
}

#[test]
fn missing_cut_save_quiescence_edge_reports_save_mutation_race() {
    let report = model_without_edge(RequiredEdge::CutSaveQuiescence).check();

    assert_counterexample(
        &report,
        RequiredEdge::CutSaveQuiescence,
        BadOutcome::MutationEscapesSaveCut,
    );
}

#[test]
fn missing_relation_epoch_sample_edge_reports_mixed_epoch_sample() {
    let report = model_without_edge(RequiredEdge::RelationEpochSample).check();

    assert_counterexample(
        &report,
        RequiredEdge::RelationEpochSample,
        BadOutcome::RelationSampleMixesEpochs,
    );
}

#[test]
fn missing_same_owner_coherence_edge_reports_rmw_reads_from_break() {
    let report = model_without_edge(RequiredEdge::SameOwnerReadsFromCoherence).check();

    assert_counterexample(
        &report,
        RequiredEdge::SameOwnerReadsFromCoherence,
        BadOutcome::SameOwnerRmwReadsFromStaleWrite,
    );
}

#[test]
fn missing_presentation_gap_nonmutation_edge_reports_semantic_mutation() {
    let report = model_without_edge(RequiredEdge::PresentationGapNonmutation).check();

    assert_counterexample(
        &report,
        RequiredEdge::PresentationGapNonmutation,
        BadOutcome::PresentationGapMutatesSemanticLineage,
    );
}
