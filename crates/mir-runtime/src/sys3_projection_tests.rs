use std::{
    collections::{BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::{BindingActivationFrontier, SourceRef},
    surface_v0_pipeline::{
        CheckedEvaluation, CheckedProgramIdentity, CheckedSurfaceV0, EffectKind,
        RelationAnchorCore, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

use crate::{
    m8_runtime_admission::{
        EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionEvidence, M8Runtime,
        M8RuntimeAdmission,
    },
    m8_runtime_authority::{
        M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
    },
    m8_runtime_relation_projection::{
        M8AnchorSample, M8LeaseInventory, M8LeaseRecord, M8Point, M8PresentationContext,
        M8RelationProjectionSeed, M8RestrictionPolicy, M8Transform2,
    },
    sys3_projection::{
        BackendEligibility, BackendIneligibilityReason, BackendProfile, CommunicationEdgeKind,
        DeclaredLogicalTopology, EffectHandlerKind, GlobalProjectionResult, LocusOperationKind,
        PersistenceResponsibilityKind, ProjectedRelationAnchor, ProjectionDiagnosticKind,
        ProjectionDiagnostics, ProjectionRelationGraph, RelationAnchorRole, RelationGraphClaim,
        RelationGraphEdgeSeed, RuntimeAdmissionStatus, StaticProjectionReadiness,
        project_checked_core, verify_projection,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const FOUR_LOCUS_FIXTURE: &str = "sys3_projection_four_locus.mir";
const SINGLE_OWNER_FIXTURE: &str = "m7_owner_only_no_residuals.mir";
const CANONICAL_FIXTURE: &str = "canonical_attack_bundle.mir";
const RELATION_ONLY_FIXTURE: &str = "maintained_bird_relation.mir";
const EXTENSION_PRESSURE_FIXTURE: &str = "sys3_projection_relation_extension_pressure.mir";
const RELATION_NAME: &str = "bird_follow";
const RELATION_OWNER: &str = "S";
const RELATION_LEASE_REF: &str = "bird_binding_frontier/live";
const RELATION_MEMBERSHIP_REF: &str = "membership:self:S:relation-binding-epoch1";
const RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:binding_epoch1";
const RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:witness_epoch1";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn load_checked_fixture(name: &str) -> CheckedSurfaceV0 {
    let (relative, source) = load_surface_fixture(name);
    check_and_elaborate_surface_v0(FixtureSource::new(relative, source))
        .expect("SYS-3 projection tests start from real parsed and checked source")
}

fn topology<const N: usize>(
    identity: &CheckedProgramIdentity,
    loci: [&'static str; N],
) -> DeclaredLogicalTopology {
    DeclaredLogicalTopology::try_new(identity.clone(), loci)
        .expect("test topology has unique declared loci")
}

fn locus_names(result: &GlobalProjectionResult) -> Vec<&str> {
    result.locus_order()
}

fn checked_eval<'a>(checked: &'a CheckedSurfaceV0, name: &str) -> &'a CheckedEvaluation {
    checked
        .evaluation(name)
        .unwrap_or_else(|| panic!("missing checked owner evaluation {name}"))
}

fn checked_relation<'a>(checked: &'a CheckedSurfaceV0, name: &str) -> &'a CheckedEvaluation {
    checked
        .relation(name)
        .unwrap_or_else(|| panic!("missing checked relation {name}"))
}

fn checked_designated<'a>(
    checked: &'a CheckedSurfaceV0,
    evaluator: &str,
    result: &str,
) -> &'a CheckedEvaluation {
    checked
        .designated_result(evaluator, result)
        .unwrap_or_else(|| panic!("missing checked designated result {evaluator}.{result}"))
}

fn residual_source_ref(
    checked: &CheckedSurfaceV0,
    kind: ResidualObligationKind,
    name: &str,
) -> SourceRef {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|entry| entry.kind() == kind && entry.name() == name)
        .unwrap_or_else(|| panic!("missing residual {kind:?}/{name}"))
        .source_ref()
        .clone()
}

fn effect_kinds(evaluation: &CheckedEvaluation) -> Vec<EffectKind> {
    evaluation
        .effect_row()
        .entries()
        .iter()
        .map(|entry| entry.kind())
        .collect()
}

fn assert_projection_diag<T>(
    actual: Result<T, ProjectionDiagnostics>,
    expected: ProjectionDiagnosticKind,
) -> ProjectionDiagnostics {
    let diagnostics = actual.expect_err("projection should fail with typed diagnostics");
    assert_eq!(diagnostics.primary().kind(), expected);
    assert!(
        diagnostics.partial_result().is_none(),
        "invalid topology or verifier failure must not expose a partial projection"
    );
    diagnostics
}

fn assert_verify_diag(
    actual: Result<(), ProjectionDiagnostics>,
    expected: ProjectionDiagnosticKind,
) {
    let diagnostics = actual.expect_err("verification should reject the mutated projection");
    assert_eq!(diagnostics.primary().kind(), expected);
    assert!(diagnostics.partial_result().is_none());
}

fn assert_projected_anchor_matches_checked(
    projected: &ProjectedRelationAnchor,
    checked: &RelationAnchorCore,
) {
    assert_eq!(projected.anchor(), checked.anchor());
    assert_eq!(projected.epoch(), checked.epoch());
    assert_eq!(projected.transform().kind(), checked.transform().kind());
    assert_eq!(
        projected.transform().translation(),
        checked.transform().translation()
    );
    assert!(
        projected.source_ref().path.ends_with(CANONICAL_FIXTURE)
            || projected.source_ref().path.ends_with(RELATION_ONLY_FIXTURE)
            || projected
                .source_ref()
                .path
                .ends_with(EXTENSION_PRESSURE_FIXTURE),
        "relation anchor projection remains source-bound"
    );
}

fn binding_frontier_name(frontier: &BindingActivationFrontier) -> &str {
    frontier
        .as_slice()
        .first()
        .expect("relation has a finite binding frontier")
        .as_str()
}

fn primary_relation_context(relation: &CheckedEvaluation) -> M8PresentationContext {
    let core = relation
        .relation_core()
        .expect("checked relation has relation Core");
    let primary = core.primary();
    let fallback = core.fallback();
    let frontier = binding_frontier_name(core.binding_frontier());

    M8PresentationContext::for_consumer(
        core.consumer_projection_locus()
            .expect("relation has consumer projection locus"),
    )
    .with_frontier(frontier)
    .with_anchor_sample(
        M8AnchorSample::new(primary.anchor())
            .with_epoch(primary.epoch())
            .with_frontier(frontier)
            .with_pose(M8Point::new(10, 20))
            .with_policy(M8RestrictionPolicy::Public),
    )
    .with_anchor_sample(
        M8AnchorSample::new(fallback.anchor())
            .with_epoch(fallback.epoch())
            .with_frontier(frontier)
            .with_pose(M8Point::new(-30, 5))
            .with_policy(M8RestrictionPolicy::Private),
    )
}

fn relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: RELATION_NAME.into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:consumer-visible"),
        redaction: EvidenceRedaction::new("consumer:C"),
        source_ref,
    }
}

fn relation_lifetime_evidence(
    relation: &CheckedEvaluation,
    source_ref: SourceRef,
) -> M8AdmissionEvidence {
    let core = relation
        .relation_core()
        .expect("checked relation has relation Core");
    M8AdmissionEvidence::RelationLifetime {
        relation: RELATION_NAME.into(),
        live_lease: RELATION_LEASE_REF.into(),
        binding_frontier: binding_frontier_name(core.binding_frontier()).into(),
        source_ref,
    }
}

fn relation_fallback_evidence(
    relation: &CheckedEvaluation,
    source_ref: SourceRef,
) -> M8AdmissionEvidence {
    let core = relation
        .relation_core()
        .expect("checked relation has relation Core");
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: RELATION_NAME.into(),
        primary_epoch: core.primary().epoch().into(),
        fallback_epoch: core.fallback().epoch().into(),
        source_ref,
    }
}

fn relation_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    let relation = checked_relation(checked, RELATION_NAME);
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            RELATION_NAME,
        )))
        .with_evidence(relation_lifetime_evidence(
            relation,
            residual_source_ref(
                checked,
                ResidualObligationKind::RelationLifetime,
                RELATION_NAME,
            ),
        ))
        .with_evidence(relation_fallback_evidence(
            relation,
            residual_source_ref(
                checked,
                ResidualObligationKind::FallbackValidity,
                RELATION_NAME,
            ),
        ))
}

fn relation_authority_state() -> M8AuthorityState {
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(RELATION_OWNER)
                .with_epoch("binding_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(RELATION_CAPABILITY_REF)
                .for_relation_transition(RELATION_NAME, "invalidate_primary")
                .with_owner_locus(RELATION_OWNER)
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_REF)
                .with_binding_epoch("binding_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(RELATION_WITNESS_REF)
                .for_capability(RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_REF)
                .with_epoch("witness_epoch:1"),
        )
}

fn relation_lease_inventory(relation: &CheckedEvaluation) -> M8LeaseInventory {
    let core = relation
        .relation_core()
        .expect("checked relation has relation Core");
    M8LeaseInventory::default().with_live_lease(
        M8LeaseRecord::live(RELATION_LEASE_REF)
            .for_relation(RELATION_NAME)
            .with_owner_locus(RELATION_OWNER)
            .with_binding_frontier(binding_frontier_name(core.binding_frontier()))
            .with_epoch("binding_epoch:1"),
    )
}

fn relation_projection_seed(relation: &CheckedEvaluation) -> M8RelationProjectionSeed {
    let core = relation
        .relation_core()
        .expect("checked relation has relation Core");
    M8RelationProjectionSeed::new()
        .with_authority_state(relation_authority_state())
        .with_live_leases(relation_lease_inventory(relation))
        .with_relation_policy(RELATION_NAME, M8RestrictionPolicy::Restricted)
        .with_subject_policy(core.subject(), M8RestrictionPolicy::Restricted)
        .with_anchor_policy(core.primary().anchor(), M8RestrictionPolicy::Public)
        .with_anchor_policy(core.fallback().anchor(), M8RestrictionPolicy::Private)
}

fn m8_transform_from_projected_anchor(anchor: &ProjectedRelationAnchor) -> M8Transform2 {
    match (anchor.transform().kind(), anchor.transform().translation()) {
        ("identity", _) => M8Transform2::identity(),
        ("translate", Some((x, y))) => M8Transform2::translate(x, y),
        other => panic!("unsupported checked relation transform in SYS-3 RED test: {other:?}"),
    }
}

fn collect_projection_sources() -> Vec<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    let file = manifest_dir.join("src/sys3_projection.rs");
    if file.exists() {
        sources.push(file);
    }

    let dir = manifest_dir.join("src/sys3_projection");
    if !dir.exists() {
        return sources;
    }

    let mut queue = VecDeque::from([dir]);
    while let Some(next) = queue.pop_front() {
        for entry in fs::read_dir(&next).expect("sys3 projection dir is readable") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                queue.push_back(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}

fn relative_to_crate(path: &Path) -> String {
    path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
        .unwrap_or(path)
        .display()
        .to_string()
}

#[test]
fn four_locus_fixture_projects_owner_requests_without_runtime_admission_claim() {
    let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);
    assert!(
        checked.execution_is_admissible(),
        "four-locus SYS-3 fixture is residual-free before projection"
    );

    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["V", "T", "A", "S"]),
    )
    .expect("residual-free checked Core projects into per-locus artifacts");

    assert_eq!(
        result.checked_program_identity(),
        checked.program_identity()
    );
    assert_eq!(
        result.projection_identity().checked_program_identity(),
        checked.program_identity()
    );
    assert_eq!(result.static_readiness(), StaticProjectionReadiness::Ready);
    assert_eq!(
        result.runtime_admission_status(),
        RuntimeAdmissionStatus::AwaitingRuntimeSeam,
        "SYS-3 projection readiness is not runtime admission"
    );
    assert_eq!(locus_names(&result), vec!["A", "S", "T", "V"]);

    let actor = result.locus_program("A").expect("actor locus artifact");
    assert!(actor.has_operation("attack_s", LocusOperationKind::OwnerRequestStub));
    assert!(actor.has_operation("attack_t", LocusOperationKind::OwnerRequestStub));
    assert!(!actor.has_operation("attack_s", LocusOperationKind::OwnerRmwEvaluation));
    assert!(!actor.has_operation("attack_t", LocusOperationKind::OwnerRmwEvaluation));

    let owner_s = result.locus_program("S").expect("S owner artifact");
    assert!(owner_s.has_operation("attack_s", LocusOperationKind::OwnerRmwEvaluation));
    assert!(!owner_s.has_operation("attack_s", LocusOperationKind::OwnerRequestStub));

    let owner_t = result.locus_program("T").expect("T owner artifact");
    assert!(owner_t.has_operation("attack_t", LocusOperationKind::OwnerRmwEvaluation));
    assert!(!owner_t.has_operation("attack_t", LocusOperationKind::OwnerRequestStub));

    let viewer = result
        .locus_program("V")
        .expect("declared but unused viewer");
    assert!(
        viewer.is_empty_artifact(),
        "declared loci remain visible even when no Core operation is placed there"
    );

    let plan = result.communication_plan();
    let expected_edges = [
        ("attack_s", CommunicationEdgeKind::OwnerRequest, "A", "S"),
        (
            "attack_s",
            CommunicationEdgeKind::OwnerReplyReceipt,
            "S",
            "A",
        ),
        ("attack_t", CommunicationEdgeKind::OwnerRequest, "A", "T"),
        (
            "attack_t",
            CommunicationEdgeKind::OwnerReplyReceipt,
            "T",
            "A",
        ),
    ];
    for (operation, kind, source, target) in expected_edges {
        let edge = plan
            .single_edge(operation, kind, source, target)
            .unwrap_or_else(|| panic!("{source}->{target} {kind:?} edge is derived"));
        assert!(edge.is_derived_from_checked_core());
        assert!(edge.core_ref().is_some());
        assert!(edge.source_ref().path.ends_with(FOUR_LOCUS_FIXTURE));
        assert!(
            !edge.transfers_authority(),
            "request/reply carriers never transfer authority by receipt"
        );
    }
}

#[test]
fn backend_requirements_keep_st_and_ow1_eligibility_precise() {
    let two_owner = load_checked_fixture(FOUR_LOCUS_FIXTURE);
    let two_owner_result: GlobalProjectionResult = project_checked_core(
        &two_owner,
        &topology(two_owner.program_identity(), ["A", "S", "T", "V"]),
    )
    .expect("two-owner static projection succeeds");

    let two_owner_backend = two_owner_result.backend_requirements();
    assert!(two_owner_backend.supports(BackendProfile::St));
    assert_eq!(
        two_owner_backend.eligibility(BackendProfile::Ow1),
        BackendEligibility::Ineligible {
            reason: BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { count: 2 },
        },
        "OW1 is not claimed for a two-owner/source-owner artifact set"
    );

    let one_owner = load_checked_fixture(SINGLE_OWNER_FIXTURE);
    let one_owner_result: GlobalProjectionResult =
        project_checked_core(&one_owner, &topology(one_owner.program_identity(), ["S"]))
            .expect("exact-one-owner static projection succeeds");
    let one_owner_backend = one_owner_result.backend_requirements();
    assert!(one_owner_backend.supports(BackendProfile::St));
    assert_eq!(
        one_owner_backend.eligibility(BackendProfile::Ow1),
        BackendEligibility::Eligible,
        "OW1 is eligible only when the combined owner/source-owner locus set has one member"
    );
    assert_eq!(
        one_owner_result.runtime_admission_status(),
        RuntimeAdmissionStatus::AwaitingRuntimeSeam
    );
}

#[test]
fn topology_validation_is_exact_identity_bound_and_core_referenced() {
    let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);

    assert_projection_diag(
        DeclaredLogicalTopology::try_new(
            checked.program_identity().clone(),
            ["A", "S", "S", "T", "V"],
        ),
        ProjectionDiagnosticKind::DuplicateLocus,
    );

    let missing_t = topology(checked.program_identity(), ["A", "S", "V"]);
    assert_projection_diag(
        project_checked_core(&checked, &missing_t),
        ProjectionDiagnosticKind::MissingRequiredLocus,
    );

    let unknown_x = topology(checked.program_identity(), ["A", "S", "T", "V", "X"]);
    assert_projection_diag(
        project_checked_core(&checked, &unknown_x),
        ProjectionDiagnosticKind::UnknownDeclaredLocus,
    );

    let canonical = load_checked_fixture(CANONICAL_FIXTURE);
    let identity_mismatch = topology(canonical.program_identity(), ["A", "S", "T", "V"]);
    assert_projection_diag(
        project_checked_core(&checked, &identity_mismatch),
        ProjectionDiagnosticKind::CheckedProgramIdentityMismatch,
    );

    assert_projection_diag(
        project_checked_core(
            &canonical,
            &topology(canonical.program_identity(), ["S", "E"]),
        ),
        ProjectionDiagnosticKind::MissingRequiredLocus,
    );
    assert_projection_diag(
        project_checked_core(
            &canonical,
            &topology(canonical.program_identity(), ["S", "C"]),
        ),
        ProjectionDiagnosticKind::MissingRequiredLocus,
    );
    assert_projection_diag(
        project_checked_core(
            &canonical,
            &topology(canonical.program_identity(), ["S", "C", "E", "X"]),
        ),
        ProjectionDiagnosticKind::UnknownDeclaredLocus,
    );
}

#[test]
fn projection_is_pure_deterministic_order_invariant_and_owns_checked_fragments() {
    let (checked_identity, retained_projection): (CheckedProgramIdentity, GlobalProjectionResult) = {
        let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);
        let unordered = topology(checked.program_identity(), ["T", "V", "A", "S"]);
        let ordered = topology(checked.program_identity(), ["A", "S", "T", "V"]);

        let first: GlobalProjectionResult = project_checked_core(&checked, &unordered)
            .expect("first projection from checked Core succeeds");
        let second: GlobalProjectionResult = project_checked_core(&checked, &ordered)
            .expect("same checked Core projects identically regardless of topology input order");

        assert_eq!(first, second);
        (checked.program_identity().clone(), first.clone())
    };

    assert_eq!(
        retained_projection.checked_program_identity(),
        &checked_identity,
        "projection result keeps its own checked identity after the checked source is dropped"
    );
    assert!(
        retained_projection
            .projected_source_map()
            .all_entries_source_core_artifact_bound_to_source(FOUR_LOCUS_FIXTURE)
    );
    assert!(
        retained_projection
            .locus_program("S")
            .expect("S artifact remains inspectable")
            .checked_fragments()
            .owner_operations()
            .contains(&"attack_s")
    );
}

#[test]
fn canonical_bundle_projects_relation_and_designated_structure_but_blocks_runtime_admission() {
    let checked = load_checked_fixture(CANONICAL_FIXTURE);
    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["S", "C", "E"]),
    )
    .expect("canonical checked Core projects structurally even with residual obligations");

    assert_eq!(locus_names(&result), vec!["C", "E", "S"]);
    assert_eq!(result.static_readiness(), StaticProjectionReadiness::Ready);
    assert_eq!(
        result.runtime_admission_status(),
        RuntimeAdmissionStatus::BlockedByResidual
    );

    let owner = result.locus_program("S").expect("owner locus artifact");
    let consumer = result.locus_program("C").expect("consumer locus artifact");
    let evaluator = result.locus_program("E").expect("evaluator locus artifact");

    assert!(owner.has_operation("attack", LocusOperationKind::OwnerRmwEvaluation));
    assert!(owner.has_operation("bird_follow", LocusOperationKind::RelationPublication));
    assert!(consumer.has_operation("bird_follow", LocusOperationKind::ConsumerLocalProjection));
    assert!(!consumer.has_operation("bird_follow", LocusOperationKind::DirectStoreMutation));
    assert!(evaluator.has_operation(
        "E.result",
        LocusOperationKind::DesignatedEvaluationExpression
    ));
    assert!(!owner.has_operation(
        "E.result",
        LocusOperationKind::DesignatedEvaluationExpression
    ));

    let communication = result.communication_plan();
    assert!(communication.has_edge(
        "bird_follow",
        CommunicationEdgeKind::RelationProjectionPublication,
        "S",
        "C"
    ));
    assert!(!communication.has_edge(
        "bird_follow",
        CommunicationEdgeKind::AbsoluteValueStream,
        "S",
        "C"
    ));
    assert!(communication.has_edge(
        "E.result",
        CommunicationEdgeKind::DesignatedInputRequest,
        "E",
        "S"
    ));
    assert!(communication.has_edge(
        "E.result",
        CommunicationEdgeKind::DesignatedInputReceipt,
        "S",
        "E"
    ));
    assert_eq!(
        communication.count_edges(
            "E.result",
            CommunicationEdgeKind::DesignatedInputRequest,
            "E",
            "S"
        ),
        1,
        "designated evaluator requests each remote source-owner input once"
    );
}

#[test]
fn effect_handler_plan_is_source_core_bound_and_not_a_generic_provider_registry() {
    let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);
    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["A", "S", "T", "V"]),
    )
    .expect("four-locus projection succeeds");

    let handlers = result.effect_handler_plan();
    assert!(!handlers.has_generic_provider_registry());

    for (operation, owner) in [("attack_s", "S"), ("attack_t", "T")] {
        let checked_eval = checked_eval(&checked, operation);
        let handler = handlers
            .single_handler(operation, EffectHandlerKind::OwnerService, owner)
            .unwrap_or_else(|| panic!("{owner} services {operation}"));
        assert!(handler.is_source_bound());
        assert!(handler.core_ref().is_some());
        assert_eq!(handler.source_ref(), checked_eval.source_ref());
        assert_eq!(handler.effect_row().kinds(), effect_kinds(checked_eval));
        assert_eq!(
            handler.declared_failure_row().names(),
            checked_eval.declared_failure_row().names()
        );
        assert_eq!(
            handler.generated_failure_row().names(),
            checked_eval.generated_failure_row().names()
        );
    }

    let canonical = load_checked_fixture(CANONICAL_FIXTURE);
    let canonical_projection: GlobalProjectionResult = project_checked_core(
        &canonical,
        &topology(canonical.program_identity(), ["S", "C", "E"]),
    )
    .expect("canonical projection succeeds structurally");
    let canonical_handlers = canonical_projection.effect_handler_plan();
    let designated = checked_designated(&canonical, "E", "result");
    let designated_core = designated
        .designated_core()
        .expect("designated Core retained");
    let source_service = canonical_handlers
        .single_handler("E.result", EffectHandlerKind::DesignatedSourceService, "S")
        .expect("source owner S services designated remote input");
    assert!(source_service.is_source_bound());
    let dependency_source_ref = designated_core.generated_remote_input_dependencies()[0]
        .typed_state_read()
        .source_ref();
    assert_eq!(source_service.source_ref(), &dependency_source_ref,);
    assert!(source_service.core_ref().is_some());

    let evaluator_handler = canonical_handlers
        .single_handler("E.result", EffectHandlerKind::DesignatedEvaluator, "E")
        .expect("only E owns designated expression evaluation");
    assert_eq!(evaluator_handler.source_ref(), designated.source_ref());
    assert_eq!(
        evaluator_handler.effect_row().kinds(),
        effect_kinds(designated)
    );
    assert!(
        canonical_handlers.all_handlers_for_operation_with_kind_are_at_locus(
            "E.result",
            EffectHandlerKind::DesignatedEvaluator,
            "E",
        )
    );
}

#[test]
fn owner_failure_rows_preserve_exact_checked_core_order() {
    let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);
    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["A", "S", "T", "V"]),
    )
    .expect("four-locus projection succeeds");

    for (operation, owner) in [("attack_s", "S"), ("attack_t", "T")] {
        let checked_eval = checked_eval(&checked, operation);
        let expected = checked_eval.generated_failure_row().names();
        assert_eq!(
            expected,
            [
                "StaleMembership",
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
            ],
            "test fixture itself keeps the accepted checked Core order"
        );

        let artifact = result.locus_program(owner).expect("owner artifact");
        assert_eq!(
            artifact
                .generated_failures(operation)
                .expect("failure row")
                .names(),
            expected,
            "projection must not sort or otherwise normalize checked failure row order"
        );
        assert_eq!(
            artifact
                .declared_failures(operation)
                .expect("declared row")
                .names(),
            checked_eval.declared_failure_row().names()
        );
    }
}

#[test]
fn relation_graph_preserves_checked_core_shape_and_residual_refs() {
    let checked = load_checked_fixture(CANONICAL_FIXTURE);
    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["S", "C", "E"]),
    )
    .expect("canonical projection succeeds structurally");

    let relation_eval = checked_relation(&checked, "bird_follow");
    let relation_core = relation_eval
        .relation_core()
        .expect("checked relation Core retained");
    let relation = result
        .relation_graph()
        .relation("bird_follow")
        .expect("relation graph keeps the maintained relation");

    assert_eq!(relation.owner_locus(), relation_core.owner_locus());
    assert_eq!(relation.subject(), relation_core.subject());
    assert_eq!(relation.subject_type(), relation_core.subject_type());
    assert_eq!(
        relation.binding_frontier(),
        relation_core.binding_frontier(),
        "binding frontier is the exact checked Core frontier"
    );
    assert_eq!(
        relation.consumer_locus(),
        relation_core.consumer_projection_locus()
    );
    assert_projected_anchor_matches_checked(relation.primary_anchor(), relation_core.primary());
    assert_projected_anchor_matches_checked(relation.fallback_anchor(), relation_core.fallback());
    assert_eq!(
        relation.residual_source_ref(ResidualObligationKind::Visibility),
        Some(&residual_source_ref(
            &checked,
            ResidualObligationKind::Visibility,
            "bird_follow",
        ))
    );
    assert_eq!(
        relation.residual_source_ref(ResidualObligationKind::RelationLifetime),
        Some(&residual_source_ref(
            &checked,
            ResidualObligationKind::RelationLifetime,
            "bird_follow",
        ))
    );
    assert_eq!(
        relation.residual_source_ref(ResidualObligationKind::FallbackValidity),
        Some(&residual_source_ref(
            &checked,
            ResidualObligationKind::FallbackValidity,
            "bird_follow",
        ))
    );
}

#[test]
fn project_then_evaluate_relation_fragment_matches_existing_m8_projection() {
    let checked = load_checked_fixture(RELATION_ONLY_FIXTURE);
    let relation_eval = checked_relation(&checked, "bird_follow");
    let relation_core = relation_eval
        .relation_core()
        .expect("relation fixture has checked relation Core");
    assert_eq!(
        relation_core.binding_frontier(),
        &BindingActivationFrontier::from_ordered_occurrences(
            relation_core.binding_frontier().as_slice().to_vec(),
        )
        .expect("checked relation frontier is already finite")
    );

    let result: GlobalProjectionResult =
        project_checked_core(&checked, &topology(checked.program_identity(), ["S", "C"]))
            .expect("relation-only checked Core projects structurally");

    let retained = result
        .relation_graph()
        .relation("bird_follow")
        .expect("projection retains the relation fragment");
    assert_eq!(retained.owner_locus(), relation_core.owner_locus());
    assert_eq!(retained.subject(), relation_core.subject());
    assert_eq!(retained.subject_type(), relation_core.subject_type());
    assert_eq!(
        retained.binding_frontier(),
        relation_core.binding_frontier()
    );
    assert_eq!(
        retained.consumer_locus(),
        relation_core.consumer_projection_locus()
    );
    assert_projected_anchor_matches_checked(retained.primary_anchor(), relation_core.primary());
    assert_projected_anchor_matches_checked(retained.fallback_anchor(), relation_core.fallback());

    let mut runtime = M8Runtime::default()
        .admit(checked.clone(), relation_admission_for(&checked))
        .expect("exact source-bound relation residual evidence admits the cloned checked input")
        .into_relation_projection(relation_projection_seed(relation_eval));
    let before = runtime.semantic_snapshot();
    let projection = runtime
        .project_relation("bird_follow", primary_relation_context(relation_eval))
        .expect("M8 adapter projects the primary relation sample");

    assert_eq!(projection.subject(), retained.subject());
    assert_eq!(
        projection.selected_anchor(),
        retained.primary_anchor().anchor()
    );
    assert_eq!(
        projection.relative_transform(),
        &m8_transform_from_projected_anchor(retained.primary_anchor())
    );
    assert_eq!(projection.anchor_pose(), Some(M8Point::new(10, 20)));
    let expected_derived = retained
        .primary_anchor()
        .transform()
        .translation()
        .map(|(x, y)| M8Point::new(10 + x, 20 + y));
    assert_eq!(projection.derived_pose(), expected_derived);
    assert!(!projection.is_consumer_local_fallback());
    assert_eq!(
        runtime.semantic_snapshot(),
        before,
        "M8 consumer-local projection does not mutate semantic relation state"
    );
}

#[test]
fn observation_and_persistence_plans_cover_runtime_correspondence_and_canonical_state() {
    let checked = load_checked_fixture(CANONICAL_FIXTURE);
    let result: GlobalProjectionResult = project_checked_core(
        &checked,
        &topology(checked.program_identity(), ["S", "C", "E"]),
    )
    .expect("canonical projection succeeds structurally");

    let observation = result.observation_plan();
    assert!(!observation.rows().is_empty());
    assert!(
        observation.all_rows_source_core_artifact_bound_observer_safe_and_redacted(),
        "every observation row is source/Core/artifact/runtime-occurrence bound and redacted"
    );

    let persistence = result.persistence_plan();
    let relation_state = persistence
        .responsibilities_for_relation("bird_follow")
        .expect("canonical relation has persistence responsibilities")
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(relation_state.contains(&PersistenceResponsibilityKind::RelationBindingFrontier));
    assert!(relation_state.contains(&PersistenceResponsibilityKind::RelationSelectedFallback));
    assert!(relation_state.contains(&PersistenceResponsibilityKind::RelationResidualEvidenceRefs));

    let designated_state = persistence
        .responsibilities_for_designated_result("E.result")
        .expect("canonical designated result has persistence responsibilities")
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(designated_state.contains(&PersistenceResponsibilityKind::DesignatedResultVersion));
    assert!(
        designated_state.contains(&PersistenceResponsibilityKind::DesignatedReceiptConsumption)
    );
    assert!(designated_state.contains(&PersistenceResponsibilityKind::DesignatedInputFrontier));

    let global = persistence
        .global_obligations()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(global.contains(&PersistenceResponsibilityKind::ResidualObligationState));
    assert!(global.contains(&PersistenceResponsibilityKind::LocalCut));
}

#[test]
fn verifier_recomputes_projection_and_rejects_mutated_clones() {
    let checked = load_checked_fixture(FOUR_LOCUS_FIXTURE);
    let topology = topology(checked.program_identity(), ["A", "S", "T", "V"]);
    let projection: GlobalProjectionResult =
        project_checked_core(&checked, &topology).expect("canonical projection succeeds");
    verify_projection(&checked, &topology, &projection).expect("canonical projection verifies");

    let mut missing_edge = projection.clone();
    missing_edge.for_test_remove_derived_edge("attack_s", CommunicationEdgeKind::OwnerRequest);
    assert_verify_diag(
        verify_projection(&checked, &topology, &missing_edge),
        ProjectionDiagnosticKind::MissingDerivedEdge,
    );

    let mut extra_edge = projection.clone();
    extra_edge.for_test_insert_non_derived_edge(
        "manual-debug-edge",
        CommunicationEdgeKind::OwnerRequest,
        "A",
        "V",
        "attack_s",
    );
    assert_verify_diag(
        verify_projection(&checked, &topology, &extra_edge),
        ProjectionDiagnosticKind::ExtraNonDerivedEdge,
    );

    let mut moved_owner = projection.clone();
    moved_owner.for_test_move_owner_operation("attack_s", "S", "A");
    assert_verify_diag(
        verify_projection(&checked, &topology, &moved_owner),
        ProjectionDiagnosticKind::OwnerOperationMoved,
    );

    let canonical = load_checked_fixture(CANONICAL_FIXTURE);
    let mut identity_mismatch = projection.clone();
    identity_mismatch
        .for_test_replace_checked_program_identity(canonical.program_identity().clone());
    assert_verify_diag(
        verify_projection(&checked, &topology, &identity_mismatch),
        ProjectionDiagnosticKind::CheckedProgramIdentityMismatch,
    );

    let mut source_map_mismatch = projection;
    source_map_mismatch.for_test_rewrite_projected_source_ref(
        "artifact:S:attack_s:owner-rmw",
        SourceRef::new("tests/fixtures/surface-v0/forged.mir", 1, 1, 1, 1),
    );
    assert_verify_diag(
        verify_projection(&checked, &topology, &source_map_mismatch),
        ProjectionDiagnosticKind::SourceMapMismatch,
    );
}

#[test]
fn relation_graph_extension_pressure_is_finite_typed_and_source_bound() {
    let checked = load_checked_fixture(EXTENSION_PRESSURE_FIXTURE);
    let bird = checked_relation(&checked, "bird_follow");
    let shadow = checked_relation(&checked, "shadow_follow");

    let graph: ProjectionRelationGraph = ProjectionRelationGraph::try_new_for_test(
        RelationGraphClaim::FiniteTypedExtensionBoundary,
        [
            RelationGraphEdgeSeed::typed_extension_dependency_between_checked_anchors(
                (bird, RelationAnchorRole::Primary),
                (bird, RelationAnchorRole::Fallback),
            ),
            RelationGraphEdgeSeed::typed_extension_dependency_between_checked_anchors(
                (bird, RelationAnchorRole::Fallback),
                (shadow, RelationAnchorRole::Primary),
            ),
        ],
    )
    .expect("finite checked relation-anchor dependency chain is accepted");

    assert_eq!(
        graph.claim(),
        RelationGraphClaim::FiniteTypedExtensionBoundary
    );
    assert!(!graph.claims_arbitrary_dag_theorem());
    assert!(!graph.claims_ordinary_source_nested_relation_semantics());
    assert!(graph.is_acyclic());
    assert_eq!(graph.max_dependency_depth(), 2);
    assert_eq!(graph.typed_dependency_edge_count(), 2);
    assert!(graph.has_typed_dependency_edge(
        ("bird_follow", RelationAnchorRole::Primary),
        ("bird_follow", RelationAnchorRole::Fallback),
    ));
    assert!(graph.has_typed_dependency_edge(
        ("bird_follow", RelationAnchorRole::Fallback),
        ("shadow_follow", RelationAnchorRole::Primary),
    ));
    assert_ne!(
        graph.node_id_for_relation_anchor("bird_follow", RelationAnchorRole::Primary),
        graph.node_id_for_relation_anchor("shadow_follow", RelationAnchorRole::Primary),
        "the same anchor lexeme remains scoped by checked relation provenance"
    );
    assert!(
        graph.all_typed_dependency_endpoints_are_checked_core_source_bound_to(
            EXTENSION_PRESSURE_FIXTURE
        ),
        "all typed dependency endpoints remain tied to actual checked relation Core/source refs"
    );

    assert_projection_diag(
        ProjectionRelationGraph::try_new_for_test(
            RelationGraphClaim::FiniteTypedExtensionBoundary,
            [
                RelationGraphEdgeSeed::typed_extension_dependency_between_checked_anchors(
                    (bird, RelationAnchorRole::Primary),
                    (shadow, RelationAnchorRole::Primary),
                ),
                RelationGraphEdgeSeed::typed_extension_dependency_between_checked_anchors(
                    (shadow, RelationAnchorRole::Primary),
                    (bird, RelationAnchorRole::Primary),
                ),
            ],
        ),
        ProjectionDiagnosticKind::RelationGraphCycle,
    );
}

#[test]
fn production_projection_module_does_not_depend_on_conformance_or_runtime_facades() {
    let sources = collect_projection_sources();
    if sources.is_empty() {
        return;
    }

    let forbidden = [
        "FullSystemV1",
        "full_system_v1",
        "M10",
        "m10_reference_system",
        "m8_runtime_",
        "M8Runtime",
        "M8RuntimeAdmission",
        "M8AdmissionEvidence",
        "M8RelationProjection",
        "M8Authority",
        "semantic_runtime_kernel",
        "sys2_execution_backend",
        "ExecutionProfile::Ow1",
        "sync_channel",
        "mailbox",
        "serde",
        "MessageEnvelope",
        "PublicEnvelope",
    ];

    for path in sources {
        let module = fs::read_to_string(&path).expect("sys3 projection source is readable");
        for token in forbidden {
            assert!(
                !module.contains(token),
                "SYS-3 projection must not depend on {token}: {}",
                relative_to_crate(&path)
            );
        }
    }
}
