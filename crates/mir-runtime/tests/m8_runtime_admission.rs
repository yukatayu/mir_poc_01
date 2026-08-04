use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::m8_runtime_admission::{
    EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionDiagnosticKind, M8AdmissionDiagnostics,
    M8AdmissionEvidence, M8Runtime, M8RuntimeAdmission, M8RuntimeInstance, M8SecurityClass,
    RuntimeLoweringKind,
};
use mir_semantics::{
    evaluation_materialization::{EvaluationPolicy, ObservationPolicy},
    shared_model::{ResultVersion, SourceRef},
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedProgramIdentity, CheckedStaticEnvironment, CheckedSurfaceV0,
        ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn checked_surface_fixture(name: &str) -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture(name);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("surface-v0 fixture checks through M7 before M8 admission");
    (path, source, checked)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1_u32;
    let mut column = 1_u32;
    for byte in source[..byte_offset].bytes() {
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
    let range = byte_range(source, lexeme);
    let (start_line, start_column) = line_column(source, range.start);
    let (end_line, end_column) = line_column(source, range.end);
    SourceRef::new(
        path.to_owned(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}

fn residual_source_ref(
    checked: &CheckedSurfaceV0,
    kind: ResidualObligationKind,
    name: &str,
) -> SourceRef {
    let entry = checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|entry| entry.kind() == kind && entry.name() == name)
        .unwrap_or_else(|| panic!("missing residual {kind:?}/{name}"));
    entry.source_ref().clone()
}

fn assert_structural_owner_only_static_environment(
    identity: &CheckedProgramIdentity,
    environment: &CheckedStaticEnvironment,
) {
    assert_eq!(identity.module(), "Combat.M7.OwnerOnlyNoResiduals");
    assert_eq!(
        identity.source_file(),
        "tests/fixtures/surface-v0/m7_owner_only_no_residuals.mir"
    );
    assert_eq!(identity.root_source_ref().path, identity.source_file());
    assert!(!identity.stable_key().is_empty());

    assert_eq!(environment.module(), identity.module());
    assert_eq!(
        environment
            .loci()
            .iter()
            .map(|decl| decl.name())
            .collect::<Vec<_>>(),
        vec!["S"]
    );
    assert_eq!(
        environment
            .principals()
            .iter()
            .map(|decl| decl.name())
            .collect::<Vec<_>>(),
        vec!["self", "target"]
    );
    assert_eq!(
        environment
            .types()
            .iter()
            .map(|decl| decl.name())
            .collect::<Vec<_>>(),
        vec!["Player"]
    );

    let player = environment
        .indexed_state_schema("player")
        .expect("player indexed state schema is retained");
    assert_eq!(player.name(), "player");
    assert_eq!(player.index_name(), "id");
    assert_eq!(player.index_type(), "Player");
    assert_eq!(player.owner_locus(), "S");
    assert_eq!(
        player
            .fields()
            .iter()
            .map(|field| (field.name(), field.type_name()))
            .collect::<Vec<_>>(),
        vec![("hp", "Int"), ("atk", "Int")]
    );

    let attack = environment
        .evaluation_signature("attack")
        .expect("owner evaluation signature is retained");
    assert_eq!(attack.name(), "attack");
    assert_eq!(attack.kind(), CheckedEvaluationKind::OwnerRmw);
    assert_eq!(attack.actor(), Some("self"));
    assert_eq!(attack.owner_locus(), Some("S"));
    assert_eq!(
        attack
            .parameters()
            .iter()
            .map(|param| (param.name(), param.type_name()))
            .collect::<Vec<_>>(),
        vec![("target", "Player")]
    );
    assert!(
        environment
            .evaluation_signatures()
            .iter()
            .any(|signature| signature.name() == "attack")
    );
}

fn empty_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
}

fn relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: "bird_follow".into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:consumer-visible"),
        redaction: EvidenceRedaction::new("consumer:C"),
        source_ref,
    }
}

fn private_relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: "bird_follow".into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:private")
            .with_class(M8SecurityClass::Private),
        redaction: EvidenceRedaction::new("relation:redact-private"),
        source_ref,
    }
}

fn relation_lifetime_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationLifetime {
        relation: "bird_follow".into(),
        live_lease: "bird_binding_frontier/live".into(),
        binding_frontier: "bird_binding_frontier".into(),
        source_ref,
    }
}

fn relation_lifetime_evidence_with_frontier(
    source_ref: SourceRef,
    binding_frontier: &str,
) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationLifetime {
        relation: "bird_follow".into(),
        live_lease: "bird_binding_frontier/live".into(),
        binding_frontier: binding_frontier.into(),
        source_ref,
    }
}

fn relation_fallback_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: "bird_follow".into(),
        primary_epoch: "primary_epoch".into(),
        fallback_epoch: "fallback_epoch".into(),
        source_ref,
    }
}

fn relation_fallback_evidence_with_epochs(
    source_ref: SourceRef,
    primary_epoch: &str,
    fallback_epoch: &str,
) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: "bird_follow".into(),
        primary_epoch: primary_epoch.into(),
        fallback_epoch: fallback_epoch.into(),
        source_ref,
    }
}

fn complete_relation_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            "bird_follow",
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            "bird_follow",
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            "bird_follow",
        )))
}

fn relation_admission_with_visibility_rows(
    checked: &CheckedSurfaceV0,
    visibility_rows: Vec<M8AdmissionEvidence>,
) -> M8RuntimeAdmission {
    let mut admission = M8RuntimeAdmission::new(checked.program_identity().clone());
    for row in visibility_rows {
        admission = admission.with_evidence(row);
    }
    admission
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            "bird_follow",
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            "bird_follow",
        )))
}

fn designated_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::ValueVisibilityRedaction {
        value: "E.result".into(),
        label: EvidenceSecurityLabel::new("value:E.result:publish"),
        redaction: EvidenceRedaction::new("conservative"),
        source_ref,
    }
}

fn deferred_auth_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::AuthDeferred {
        name: "MembershipAuth".into(),
        authority_label: "membership-authority/MembershipAuth".into(),
        source_ref,
    }
}

fn deferred_verify_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::VerifyDeferred {
        name: "finite_refinement".into(),
        theorem: "finite_refinement".into(),
        witness_schema: "m9-proof-witness-required".into(),
        source_ref,
    }
}

fn admit_without_source_ast_or_fixture_name(
    runtime: &M8Runtime,
    checked: CheckedSurfaceV0,
    admission: M8RuntimeAdmission,
) -> Result<M8RuntimeInstance, M8AdmissionDiagnostics> {
    runtime.admit(checked, admission)
}

#[test]
fn owner_only_residual_free_artifact_admits_without_source_ast_or_fixture_and_lowers_deterministically()
 {
    let (_, _, checked) = checked_surface_fixture("m7_owner_only_no_residuals.mir");
    assert_structural_owner_only_static_environment(
        checked.program_identity(),
        checked.static_environment(),
    );
    assert_eq!(
        checked
            .evaluations()
            .iter()
            .map(|evaluation| (evaluation.name(), evaluation.kind()))
            .collect::<Vec<_>>(),
        vec![("attack", CheckedEvaluationKind::OwnerRmw)]
    );
    assert!(checked.residual_obligations().entries().is_empty());

    let first = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        empty_admission_for(&checked),
    )
    .expect("residual-free owner artifact admits into M8");
    let (_, _, checked_again) = checked_surface_fixture("m7_owner_only_no_residuals.mir");
    let second = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked_again.clone(),
        empty_admission_for(&checked_again),
    )
    .expect("same checked input admits deterministically");

    assert_eq!(first.program_identity(), checked.program_identity());
    assert!(first.runtime_alias().starts_with("runtime-admitted:"));
    assert_ne!(first.runtime_alias(), first.program_identity().stable_key());
    assert!(first.is_runtime_admitted());
    assert_eq!(
        first.ordered_lowering().entries(),
        second.ordered_lowering().entries()
    );
    assert_eq!(
        first.ordered_lowering().kinds(),
        vec![
            RuntimeLoweringKind::OwnerRequest,
            RuntimeLoweringKind::OwnerLocalRead,
            RuntimeLoweringKind::OwnerWrite,
        ]
    );
    assert!(first.admission_evidence().entries().is_empty());
}

#[test]
fn bird_relation_admission_requires_visibility_lifetime_and_fallback_evidence_with_exact_identity_and_source_refs()
 {
    let (path, source, checked) = checked_surface_fixture("maintained_bird_relation.mir");
    let relation_lexeme = "relation bird_follow at S {\n  subject bird: Bird\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}";
    let relation_ref = expected_source_ref(&path, &source, relation_lexeme);
    assert_eq!(
        checked
            .residual_obligations()
            .entries()
            .iter()
            .map(|entry| (entry.kind(), entry.name(), entry.source_ref()))
            .collect::<Vec<_>>(),
        vec![
            (
                ResidualObligationKind::Visibility,
                "bird_follow",
                &relation_ref
            ),
            (
                ResidualObligationKind::RelationLifetime,
                "bird_follow",
                &relation_ref
            ),
            (
                ResidualObligationKind::FallbackValidity,
                "bird_follow",
                &relation_ref
            ),
        ]
    );

    let admitted = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        complete_relation_admission_for(&checked),
    )
    .expect("complete relation evidence admits the runtime instance");
    assert_eq!(admitted.program_identity(), checked.program_identity());
    assert!(
        admitted
            .admission_evidence()
            .contains_residual(ResidualObligationKind::Visibility, "bird_follow")
    );
    assert!(
        admitted
            .admission_evidence()
            .contains_residual(ResidualObligationKind::RelationLifetime, "bird_follow")
    );
    assert!(
        admitted
            .admission_evidence()
            .contains_residual(ResidualObligationKind::FallbackValidity, "bird_follow")
    );

    let missing_fallback = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(relation_ref.clone()))
        .with_evidence(relation_lifetime_evidence(relation_ref.clone()));
    let missing = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        missing_fallback,
    )
    .expect_err("relation admission rejects missing fallback evidence");
    assert_eq!(
        missing.primary().kind(),
        M8AdmissionDiagnosticKind::MissingResidualEvidence
    );
    assert_eq!(
        missing.primary().residual_kind(),
        Some(ResidualObligationKind::FallbackValidity)
    );
    assert_eq!(missing.primary().residual_name(), Some("bird_follow"));
    assert_eq!(missing.primary().source_ref(), &relation_ref);

    let wrong_source = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(SourceRef::new(
            "tests/fixtures/surface-v0/wrong.mir",
            1,
            1,
            1,
            2,
        )))
        .with_evidence(relation_lifetime_evidence(relation_ref.clone()))
        .with_evidence(relation_fallback_evidence(relation_ref.clone()));
    let wrong_source_diagnostics = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        wrong_source,
    )
    .expect_err("relation admission rejects wrong source-bound evidence");
    assert_eq!(
        wrong_source_diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::SourceRefMismatch
    );
    assert_eq!(
        wrong_source_diagnostics.primary().residual_kind(),
        Some(ResidualObligationKind::Visibility)
    );
    assert_eq!(
        wrong_source_diagnostics.primary().expected_source_ref(),
        Some(&relation_ref)
    );

    let wrong_identity = CheckedProgramIdentity::new(
        "Combat.M6.Relation.Other",
        checked.program_identity().source_file(),
        checked.program_identity().root_source_ref().clone(),
    );
    let wrong_program = M8RuntimeAdmission::new(wrong_identity)
        .with_evidence(relation_visibility_evidence(relation_ref.clone()))
        .with_evidence(relation_lifetime_evidence(relation_ref.clone()))
        .with_evidence(relation_fallback_evidence(relation_ref));
    let wrong_program_diagnostics =
        admit_without_source_ast_or_fixture_name(&M8Runtime::default(), checked, wrong_program)
            .expect_err("relation admission rejects mismatched checked program identity");
    assert_eq!(
        wrong_program_diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::ProgramIdentityMismatch
    );
}

#[test]
fn relation_admission_payload_must_match_checked_core_and_retains_private_evidence() {
    let (_, _, checked) = checked_surface_fixture("maintained_bird_relation.mir");
    let visibility_ref =
        residual_source_ref(&checked, ResidualObligationKind::Visibility, "bird_follow");
    let lifetime_ref = residual_source_ref(
        &checked,
        ResidualObligationKind::RelationLifetime,
        "bird_follow",
    );
    let fallback_ref = residual_source_ref(
        &checked,
        ResidualObligationKind::FallbackValidity,
        "bird_follow",
    );

    let wrong_frontier = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(private_relation_visibility_evidence(visibility_ref.clone()))
        .with_evidence(relation_lifetime_evidence_with_frontier(
            lifetime_ref.clone(),
            "other_binding_frontier",
        ))
        .with_evidence(relation_fallback_evidence(fallback_ref.clone()));
    let diagnostics = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        wrong_frontier,
    )
    .expect_err("relation lifetime evidence with a non-Core binding frontier must reject");
    assert_eq!(
        diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::RelationEvidencePayloadMismatch
    );
    assert_eq!(
        diagnostics.primary().residual_kind(),
        Some(ResidualObligationKind::RelationLifetime)
    );
    assert_eq!(diagnostics.primary().residual_name(), Some("bird_follow"));
    assert_eq!(diagnostics.primary().source_ref(), &lifetime_ref);

    let wrong_epochs = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(private_relation_visibility_evidence(visibility_ref.clone()))
        .with_evidence(relation_lifetime_evidence(lifetime_ref.clone()))
        .with_evidence(relation_fallback_evidence_with_epochs(
            fallback_ref.clone(),
            "other_primary_epoch",
            "other_fallback_epoch",
        ));
    let diagnostics = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        wrong_epochs,
    )
    .expect_err("fallback validity evidence whose epochs differ from Core must reject");
    assert_eq!(
        diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::RelationEvidencePayloadMismatch
    );
    assert_eq!(
        diagnostics.primary().residual_kind(),
        Some(ResidualObligationKind::FallbackValidity)
    );
    assert_eq!(diagnostics.primary().residual_name(), Some("bird_follow"));
    assert_eq!(diagnostics.primary().source_ref(), &fallback_ref);

    let admitted = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        M8RuntimeAdmission::new(checked.program_identity().clone())
            .with_evidence(private_relation_visibility_evidence(visibility_ref))
            .with_evidence(relation_lifetime_evidence(lifetime_ref))
            .with_evidence(relation_fallback_evidence(fallback_ref)),
    )
    .expect("exact relation evidence admits and remains inspectable");
    let plan = admitted
        .relation_plan("bird_follow")
        .expect("admitted relation plan is publicly inspectable for evidence audit");
    assert_eq!(
        plan.visibility_label().security_class(),
        M8SecurityClass::Private
    );
    assert_eq!(
        plan.visibility_label().as_str(),
        "relation:bird_follow:private"
    );
    assert_eq!(plan.redaction().as_str(), "relation:redact-private");
    assert_eq!(plan.live_lease_ref(), "bird_binding_frontier/live");
    assert_eq!(plan.binding_frontier(), "bird_binding_frontier");
    assert_eq!(plan.primary_epoch(), "primary_epoch");
    assert_eq!(plan.fallback_epoch(), "fallback_epoch");
}

#[test]
fn duplicate_or_conflicting_relation_admission_evidence_rejects_order_independently() {
    let (_, _, checked) = checked_surface_fixture("maintained_bird_relation.mir");
    let visibility_ref =
        residual_source_ref(&checked, ResidualObligationKind::Visibility, "bird_follow");

    for admission in [
        relation_admission_with_visibility_rows(
            &checked,
            vec![
                relation_visibility_evidence(visibility_ref.clone()),
                relation_visibility_evidence(visibility_ref.clone()),
            ],
        ),
        relation_admission_with_visibility_rows(
            &checked,
            vec![
                private_relation_visibility_evidence(visibility_ref.clone()),
                private_relation_visibility_evidence(visibility_ref.clone()),
            ],
        ),
    ] {
        let diagnostics = admit_without_source_ast_or_fixture_name(
            &M8Runtime::default(),
            checked.clone(),
            admission,
        )
        .expect_err("exact duplicate relation evidence must reject deterministically");
        assert_eq!(
            diagnostics.primary().kind(),
            M8AdmissionDiagnosticKind::DuplicateResidualEvidence
        );
        assert_eq!(
            diagnostics.primary().residual_kind(),
            Some(ResidualObligationKind::Visibility)
        );
        assert_eq!(diagnostics.primary().residual_name(), Some("bird_follow"));
        assert_eq!(diagnostics.primary().source_ref(), &visibility_ref);
    }

    for admission in [
        relation_admission_with_visibility_rows(
            &checked,
            vec![
                relation_visibility_evidence(visibility_ref.clone()),
                private_relation_visibility_evidence(visibility_ref.clone()),
            ],
        ),
        relation_admission_with_visibility_rows(
            &checked,
            vec![
                private_relation_visibility_evidence(visibility_ref.clone()),
                relation_visibility_evidence(visibility_ref.clone()),
            ],
        ),
    ] {
        let diagnostics = admit_without_source_ast_or_fixture_name(
            &M8Runtime::default(),
            checked.clone(),
            admission,
        )
        .expect_err("conflicting relation evidence must reject independent of row order");
        assert_eq!(
            diagnostics.primary().kind(),
            M8AdmissionDiagnosticKind::ConflictingResidualEvidence
        );
        assert_eq!(
            diagnostics.primary().residual_kind(),
            Some(ResidualObligationKind::Visibility)
        );
        assert_eq!(diagnostics.primary().residual_name(), Some("bird_follow"));
        assert_eq!(diagnostics.primary().source_ref(), &visibility_ref);
    }
}

#[test]
fn designated_admission_requires_value_visibility_redaction_and_preserves_m7_policy_frontiers_and_stamp()
 {
    let (path, source, checked) = checked_surface_fixture("designated_tick_publish_result.mir");
    let designated_lexeme = "designated evaluate E on tick F publish result = player[self].atk + 1";
    let designated_ref = expected_source_ref(&path, &source, designated_lexeme);
    assert_eq!(
        checked
            .residual_obligations()
            .entries()
            .iter()
            .map(|entry| (entry.kind(), entry.name(), entry.source_ref()))
            .collect::<Vec<_>>(),
        vec![(
            ResidualObligationKind::ValueVisibilityRedaction,
            "E.result",
            &designated_ref
        )]
    );

    let checked_designated = checked
        .designated_result("E", "result")
        .expect("M7 designated result exists")
        .designated_core()
        .expect("M7 designated checked Core exists");
    let expected_eval_policy = EvaluationPolicy::declared_deterministic("inferred:E.result");
    let expected_observation_policy = ObservationPolicy::declared("conservative");
    assert_eq!(
        checked_designated.evaluation_policy(),
        &expected_eval_policy
    );
    assert_eq!(
        checked_designated.observation_policy(),
        &expected_observation_policy
    );
    assert_eq!(checked_designated.result_version(), ResultVersion::new(1));

    let admitted = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        M8RuntimeAdmission::new(checked.program_identity().clone())
            .with_evidence(designated_visibility_evidence(designated_ref)),
    )
    .expect("designated value visibility/redaction evidence admits runtime instance");
    let admitted_value = admitted
        .designated_value("E.result")
        .expect("runtime instance retains admitted designated value");
    assert_eq!(
        admitted_value.result_frontier(),
        checked_designated.result_frontier()
    );
    assert_eq!(
        admitted_value.input_frontier(),
        checked_designated.input_frontier()
    );
    assert_eq!(
        admitted_value.evaluation_policy(),
        checked_designated.evaluation_policy()
    );
    assert_eq!(
        admitted_value.observation_policy(),
        checked_designated.observation_policy()
    );
    assert_eq!(
        admitted_value.policy_stamp(),
        checked_designated.policy_stamp()
    );
}

#[test]
fn deferred_auth_and_verify_evidence_remain_deferred_to_m9_without_runtime_success_authority_or_verdict()
 {
    let (path, source, checked) = checked_surface_fixture("m7_residual_cannot_execute.mir");
    let auth_ref = expected_source_ref(&path, &source, "with auth MembershipAuth");
    let verify_ref = expected_source_ref(&path, &source, "verify finite_refinement");
    assert!(checked.evaluations().is_empty());
    assert_eq!(
        checked
            .residual_obligations()
            .entries()
            .iter()
            .map(|entry| (entry.kind(), entry.name(), entry.source_ref()))
            .collect::<Vec<_>>(),
        vec![
            (
                ResidualObligationKind::AuthDeferred,
                "MembershipAuth",
                &auth_ref
            ),
            (
                ResidualObligationKind::VerifyDeferred,
                "finite_refinement",
                &verify_ref
            ),
        ]
    );

    let diagnostics = admit_without_source_ast_or_fixture_name(
        &M8Runtime::default(),
        checked.clone(),
        M8RuntimeAdmission::new(checked.program_identity().clone())
            .with_evidence(deferred_auth_evidence(auth_ref))
            .with_evidence(deferred_verify_evidence(verify_ref)),
    )
    .expect_err("M8 must not turn deferred M9 evidence into runtime success");
    assert_eq!(
        diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::DeferredToM9
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
}

#[test]
fn checked_artifact_exposes_evaluations_program_identity_static_environment_and_source_bound_residual_rows()
 {
    let (path, source, checked) = checked_surface_fixture("canonical_attack_bundle.mir");
    assert_eq!(checked.program_identity().module(), "Combat.M6");
    assert_eq!(
        checked
            .evaluations()
            .iter()
            .map(|evaluation| evaluation.kind())
            .collect::<Vec<_>>(),
        vec![
            CheckedEvaluationKind::OwnerRmw,
            CheckedEvaluationKind::PublishRelation,
            CheckedEvaluationKind::DesignatedPublishValue,
        ]
    );
    let environment = checked.static_environment();
    assert_eq!(environment.module(), "Combat.M6");
    assert!(environment.indexed_state_schema("player").is_some());
    assert!(environment.evaluation_signature("attack").is_some());
    assert!(environment.evaluation_signature("bird_follow").is_some());
    assert!(environment.evaluation_signature("E.result").is_some());

    let relation_ref = expected_source_ref(
        &path,
        &source,
        "relation bird_follow at S {\n  subject bird: Player\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}",
    );
    let designated_ref = expected_source_ref(
        &path,
        &source,
        "designated evaluate E on tick F publish result = player[self].atk + 1",
    );
    let auth_ref = expected_source_ref(&path, &source, "with auth MembershipAuth");
    let verify_ref = expected_source_ref(&path, &source, "verify finite_refinement");
    assert_eq!(
        checked
            .residual_obligations()
            .entries()
            .iter()
            .map(|entry| (entry.kind(), entry.name(), entry.source_ref()))
            .collect::<Vec<_>>(),
        vec![
            (
                ResidualObligationKind::Visibility,
                "bird_follow",
                &relation_ref
            ),
            (
                ResidualObligationKind::RelationLifetime,
                "bird_follow",
                &relation_ref
            ),
            (
                ResidualObligationKind::FallbackValidity,
                "bird_follow",
                &relation_ref
            ),
            (
                ResidualObligationKind::ValueVisibilityRedaction,
                "E.result",
                &designated_ref
            ),
            (
                ResidualObligationKind::AuthDeferred,
                "MembershipAuth",
                &auth_ref
            ),
            (
                ResidualObligationKind::VerifyDeferred,
                "finite_refinement",
                &verify_ref
            ),
        ]
    );
}
