use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::{
    DeferredForm, DeferredFormKind, FixtureSource, SurfaceV0File, parse_surface_v0,
};
use mir_semantics::{
    shared_model::{
        AuthorityObligation, BindingActivationFrontier, CoreOp, DiagnosticCode, GeneratedEdge,
        OccurrenceId, ResultFrontier, ResultKey, ResultVersion, SourceRef, StateKey, Value,
    },
    surface_v0_classification::{
        ClassificationKind, CoreTemplate, CoreTemplateKind, MatrixOutcomeKind, SourceToCoreKind,
        SurfaceV0ClassificationOptions, SurfaceV0DiagnosticKind, SurfaceV0MatrixSpec,
        classify_surface_v0, classify_surface_v0_matrix,
    },
};

const FIXTURE_DIR: &str = "tests/fixtures/surface-v0";

fn fixture_path(name: &str) -> String {
    format!("{FIXTURE_DIR}/{name}")
}

fn load_fixture(name: &str) -> (String, String) {
    let relative = fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn expected_source_ref(path: &str, source: &str, needle: &str) -> SourceRef {
    let range = byte_range(source, needle);
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

fn diagnostic_kind_name(kind: SurfaceV0DiagnosticKind) -> String {
    format!("{kind:?}")
}

fn m5_diagnostic_code_name(code: DiagnosticCode) -> String {
    format!("{code:?}")
}

fn deferred_form<'a>(
    ast: &'a SurfaceV0File,
    kind: DeferredFormKind,
    name: &str,
) -> &'a DeferredForm {
    ast.deferred_forms()
        .entries()
        .iter()
        .find(|form| form.kind() == kind && form.name() == name)
        .expect("fixture contains deferred marker")
}

fn single_fixture_matrix_outcome(fixture_name: &str) -> MatrixOutcomeKind {
    let matrix_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/mir-ast")
        .join(FIXTURE_DIR);
    let matrix = classify_surface_v0_matrix(
        SurfaceV0MatrixSpec::new(matrix_root).with_fixture_names([fixture_name]),
        SurfaceV0ClassificationOptions::default(),
    )
    .expect("matrix is built from real parse and classification");
    matrix
        .row(fixture_name.strip_suffix(".mir").unwrap_or(fixture_name))
        .expect("single fixture row exists")
        .outcome()
}

#[test]
fn classifies_canonical_attack_with_m5_source_refs_and_one_to_many_core_map() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical source parses before classification");
    let classified = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect("canonical source classifies");

    assert_eq!(
        classified.root_source_ref(),
        &SourceRef::new(path.clone(), 1, 1, 36, 1)
    );
    let assignment = parsed
        .assignment("player[target].hp")
        .expect("attack assignment");
    let assignment_ref = expected_source_ref(
        &path,
        &source,
        "player[target].hp = player[target].hp - player[self].atk",
    );
    assert_eq!(
        classified
            .source_ref_for_span(assignment.span())
            .expect("classification owns M6 span to M5 SourceRef conversion"),
        &assignment_ref
    );

    let core_edges = classified
        .source_to_core_map()
        .entries_for_span(assignment.span());
    assert_eq!(core_edges.len(), 3);
    assert_eq!(
        core_edges.kinds(),
        vec![
            SourceToCoreKind::OwnerRmw,
            SourceToCoreKind::OwnerLocalRead,
            SourceToCoreKind::OwnerLocalWrite,
        ]
    );
    assert!(core_edges.all_source_spans_equal(assignment.span()));

    let attack = classified
        .core_template("attack")
        .expect("attack core template");
    assert_eq!(attack.kind(), CoreTemplateKind::OwnerRmw);
    let core = attack
        .to_m5_core()
        .expect("owner RMW template lowers to M5 Core");
    assert_eq!(core.source_ref(), &assignment_ref);
    assert_eq!(core.generated_edges().len(), 2);
    assert!(matches!(
        core.generated_edges(),
        [
            GeneratedEdge::Request { .. },
            GeneratedEdge::OwnerWrite { .. }
        ]
    ));
    assert!(
        !core
            .generated_edges()
            .iter()
            .any(|edge| matches!(edge, GeneratedEdge::ReceiptUse { .. }))
    );
    assert!(
        core.authority_obligations()
            .iter()
            .any(|obligation| matches!(obligation, AuthorityObligation::Capability { .. }))
    );
    assert!(
        core.authority_obligations()
            .iter()
            .any(|obligation| matches!(obligation, AuthorityObligation::Witness { .. }))
    );
    assert!(
        !core
            .authority_obligations()
            .iter()
            .any(|obligation| matches!(obligation, AuthorityObligation::ReceiptRelease { .. }))
    );
    assert!(matches!(
        core.ops(),
        [CoreOp::OwnerReadModifyWrite { owner, command }]
            if owner.as_str() == "S"
                && matches!(
                    command,
                    mir_semantics::shared_model::OwnerCommand::Add { state, amount }
                        if *state == StateKey::field("player", mir_semantics::shared_model::FieldRef::new("hp"))
                            && *amount == Value::int(-1)
                )
    ));

    let authority = classified
        .authority_audit("attack")
        .expect("authority audit");
    assert!(authority.inner_at_locus("S").does_not_mint_authority());
    assert!(authority.required_authority().contains("MembershipAuth"));
}

#[test]
fn classifies_designated_and_relation_templates_without_collapsing_frontiers() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path, source)).expect("source parses");
    let classified = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect("source classifies");

    let designated = classified
        .designated_template("E", "result")
        .expect("designated result template");
    assert_eq!(designated.kind(), CoreTemplateKind::DesignatedPublishValue);
    assert_eq!(
        designated.result_frontier(),
        &ResultFrontier::from_ordered_results(vec![ResultKey::new("F")])
            .expect("single tick result frontier")
    );
    assert_eq!(designated.result_version(), ResultVersion::new(1));
    assert!(designated.preserves_duplicate_version());

    let relation = classified
        .relation_template("bird_follow")
        .expect("relation template");
    assert_eq!(relation.kind(), CoreTemplateKind::MaintainedRelation);
    assert_eq!(
        relation.binding_frontier(),
        &BindingActivationFrontier::from_ordered_occurrences(vec![OccurrenceId::new(
            "bird_binding_frontier"
        )])
        .expect("single binding activation frontier")
    );
    assert_eq!(
        relation.owner_publication_kind(),
        CoreTemplateKind::PublishRelation
    );
    assert!(relation.published_relation_carrier().is_some());
    assert_eq!(relation.consumer_projection_locus(), Some("C"));
    assert_eq!(
        relation.consumer_projection_kind(),
        CoreTemplateKind::ConsumerLocalProjection
    );
}

#[test]
fn classifies_with_auth_and_verify_as_span_tracked_non_executable_deferred_templates() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical source parses before classification");
    let classified = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect("canonical source classifies");

    let with_auth_form = deferred_form(&parsed, DeferredFormKind::WithAuth, "MembershipAuth");
    let verify_form = deferred_form(&parsed, DeferredFormKind::Verify, "finite_refinement");

    let with_auth: &CoreTemplate = classified
        .deferred_template(CoreTemplateKind::DeferredWithAuth, "MembershipAuth")
        .expect("with auth marker is a visible typed deferred template");
    assert_eq!(with_auth.kind(), CoreTemplateKind::DeferredWithAuth);
    assert_eq!(with_auth.source_span(), with_auth_form.span());
    assert_eq!(with_auth.authority_requirement(), Some("MembershipAuth"));

    let verify: &CoreTemplate = classified
        .deferred_template(CoreTemplateKind::DeferredVerify, "finite_refinement")
        .expect("verify marker is a visible typed deferred template");
    assert_eq!(verify.kind(), CoreTemplateKind::DeferredVerify);
    assert_eq!(verify.source_span(), verify_form.span());
    assert!(verify.authority_requirement().is_none());

    assert_eq!(
        with_auth_form.span().lexeme(&source),
        "with auth MembershipAuth"
    );
    assert_eq!(
        classified
            .source_ref_for_span(with_auth_form.span())
            .expect("with auth marker has M5 SourceRef visibility"),
        &expected_source_ref(&path, &source, "with auth MembershipAuth")
    );
    let with_auth_source_map_entries = classified
        .source_to_core_map()
        .entries_for_span(with_auth_form.span());
    assert_eq!(
        with_auth_source_map_entries.kinds(),
        vec![SourceToCoreKind::DeferredPolicy]
    );
    assert!(with_auth.is_non_executable());
    assert!(with_auth.to_m5_core().is_none());
    assert!(!with_auth.grants_authority());
    assert!(!with_auth.emits_effect());
    assert!(!with_auth.mutates_state());
    assert!(!with_auth.emits_verdict());

    assert_eq!(
        verify_form.span().lexeme(&source),
        "verify finite_refinement"
    );
    assert_eq!(
        classified
            .source_ref_for_span(verify_form.span())
            .expect("verify marker has M5 SourceRef visibility"),
        &expected_source_ref(&path, &source, "verify finite_refinement")
    );
    let verify_source_map_entries = classified
        .source_to_core_map()
        .entries_for_span(verify_form.span());
    assert_eq!(
        verify_source_map_entries.kinds(),
        vec![SourceToCoreKind::DeferredPolicy]
    );
    assert!(verify.is_non_executable());
    assert!(verify.to_m5_core().is_none());
    assert!(!verify.grants_authority());
    assert!(!verify.emits_effect());
    assert!(!verify.mutates_state());
    assert!(!verify.emits_verdict());
}

#[test]
fn role_action_locus_mismatch_rejects_before_rhs_owner_checks_without_core_success() {
    let (path, source) = load_fixture("owner_action_locus_mismatch.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("owner/action locus mismatch is syntactically valid");

    let diagnostics = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect_err("nested action locus must reject classification before Core success");

    assert_eq!(diagnostics.entries().len(), 1);
    let primary = diagnostics.primary();
    assert_eq!(
        primary.kind(),
        SurfaceV0DiagnosticKind::OwnerActionLocusMismatch
    );
    assert_eq!(primary.m5_code(), DiagnosticCode::OwnerActionLocusMismatch);
    assert_ne!(
        primary.kind(),
        SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt
    );
    assert_eq!(primary.span().lexeme(&source), "at T");
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(&path, &source, "at T")
    );
}

#[test]
fn role_actor_must_be_literal_self_through_real_parser_and_matrix_classification() {
    let (path, source) = load_fixture("role_actor_not_self.mir");
    let diagnostics =
        parse_surface_v0(FixtureSource::new(path.clone(), source.clone())).unwrap_err();
    let primary = diagnostics.primary();
    assert_eq!(
        format!("{:?}", primary.kind()),
        "RoleActorMustBeLiteralSelf"
    );
    assert_eq!(primary.span().file(), path);
    assert_eq!(primary.span().lexeme(&source), "attacker");

    let outcome = single_fixture_matrix_outcome("role_actor_not_self.mir");
    assert!(matches!(
        outcome,
        MatrixOutcomeKind::Diagnostic(kind)
            if diagnostic_kind_name(kind) == "RoleActorMustBeLiteralSelf"
    ));
}

#[test]
fn target_state_outside_action_locus_is_cross_owner_write_diagnostic_without_core_success() {
    let (path, source) = load_fixture("cross_owner_write_target.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("cross-owner write target source is syntactically valid");

    let diagnostics = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect_err("target state outside action locus must reject classification");

    assert_eq!(diagnostics.entries().len(), 1);
    let primary = diagnostics.primary();
    assert_eq!(
        diagnostic_kind_name(primary.kind()),
        "CrossOwnerWriteTargetOutsideActionLocus"
    );
    assert_eq!(
        m5_diagnostic_code_name(primary.m5_code()),
        "CrossOwnerWriteTargetOutsideActionLocus"
    );
    assert_ne!(
        primary.kind(),
        SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt
    );
    assert_eq!(primary.span().lexeme(&source), "enemy[target].hp");
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(&path, &source, "enemy[target].hp")
    );
}

#[test]
fn fieldless_assignment_target_is_typed_diagnostic_without_panic_or_core_success() {
    let (path, source) = load_fixture("fieldless_assignment_target.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("fieldless assignment target source is syntactically valid");

    let classification = std::panic::catch_unwind(|| {
        classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
    });
    assert!(
        classification.is_ok(),
        "fieldless target must return a typed diagnostic instead of panicking"
    );
    let diagnostics = classification
        .unwrap()
        .expect_err("fieldless assignment target must reject classification before Core success");

    assert_eq!(diagnostics.entries().len(), 1);
    let primary = diagnostics.primary();
    assert_eq!(
        diagnostic_kind_name(primary.kind()),
        "FieldlessAssignmentTarget"
    );
    assert_eq!(
        m5_diagnostic_code_name(primary.m5_code()),
        "FieldlessAssignmentTarget"
    );
    assert_eq!(primary.span().lexeme(&source), "player[target]");
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(&path, &source, "player[target]")
    );
}

#[test]
fn cross_owner_operand_without_receipt_is_typed_diagnostic_with_span() {
    let (path, source) = load_fixture("cross_owner_without_receipt.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("cross-owner source is syntactically valid");
    let diagnostics = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect_err("cross-owner operand requires explicit receipt");
    let primary = diagnostics.primary();
    assert_eq!(
        primary.m5_code(),
        DiagnosticCode::CrossOwnerOperandRequiresReceipt
    );
    assert_eq!(
        primary.kind(),
        SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt
    );
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(&path, &source, "enemy[target].atk")
    );
    assert_eq!(primary.span().lexeme(&source), "enemy[target].atk");
}

#[test]
fn relation_cannot_classify_as_absolute_pose_publication_or_consumer_mutation() {
    for (fixture, expected_kind, lexeme) in [
        (
            "relation_absolute_pose_publication.mir",
            SurfaceV0DiagnosticKind::RelationMustPublishRelationCarrier,
            "publish value absolute_pose",
        ),
        (
            "relation_consumer_mutation.mir",
            SurfaceV0DiagnosticKind::ConsumerRelationMutationDenied,
            "relation bird_follow mutate binding",
        ),
    ] {
        let (path, source) = load_fixture(fixture);
        let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect("negative relation source is syntactically valid");
        let diagnostics = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
            .expect_err("relation classification rejects forbidden materialization");
        let primary = diagnostics.primary();
        assert_eq!(primary.kind(), expected_kind);
        assert_eq!(
            primary.source_ref(),
            &expected_source_ref(&path, &source, lexeme)
        );
        assert_eq!(primary.span().lexeme(&source), lexeme);
    }
}

#[test]
fn unresolved_and_ambiguous_names_are_diagnostics_with_source_spans() {
    let (path, source) = load_fixture("unresolved_ambiguous_names.mir");
    let parsed = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("unresolved names are classification diagnostics, not parse failures");
    let diagnostics = classify_surface_v0(&parsed, SurfaceV0ClassificationOptions::default())
        .expect_err("unresolved and ambiguous names reject classification");

    let unresolved = diagnostics
        .by_kind(SurfaceV0DiagnosticKind::UnresolvedName)
        .expect("unresolved name diagnostic");
    assert_eq!(unresolved.span().lexeme(&source), "UnknownPlayer");
    assert_eq!(
        unresolved.source_ref(),
        &expected_source_ref(&path, &source, "UnknownPlayer")
    );

    let ambiguous = diagnostics
        .by_kind(SurfaceV0DiagnosticKind::AmbiguousName)
        .expect("ambiguous name diagnostic");
    assert_eq!(ambiguous.span().lexeme(&source), "Player");
    assert_eq!(
        ambiguous.source_ref(),
        &expected_source_ref(&path, &source, "Player")
    );
}

#[test]
fn ten_scenario_matrix_uses_real_parse_and_classification() {
    let matrix_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/mir-ast")
        .join(FIXTURE_DIR);
    let matrix = classify_surface_v0_matrix(
        SurfaceV0MatrixSpec::new(matrix_root).with_fixture_names([
            "canonical_attack_bundle.mir",
            "cross_owner_without_receipt.mir",
            "maintained_bird_relation.mir",
            "designated_tick_publish_result.mir",
            "relation_absolute_pose_publication.mir",
            "relation_consumer_mutation.mir",
            "reject_send_syntax.mir",
            "reject_receive_syntax.mir",
            "reject_occurrence_syntax.mir",
            "reject_envelope_syntax.mir",
        ]),
        SurfaceV0ClassificationOptions::default(),
    )
    .expect("matrix is built from real parse and classification");

    assert_eq!(matrix.rows().len(), 10);
    assert!(matrix.rows().all_used_real_parse());
    assert!(matrix.rows().all_used_real_classification());
    assert_eq!(
        matrix
            .row("canonical_attack_bundle")
            .expect("canonical row")
            .outcome(),
        MatrixOutcomeKind::Accepted(ClassificationKind::OwnerRmwWithRelationAndDesignated)
    );
    assert_eq!(
        matrix
            .row("cross_owner_without_receipt")
            .expect("cross-owner row")
            .outcome(),
        MatrixOutcomeKind::Diagnostic(SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt)
    );
    assert_eq!(
        matrix
            .row("maintained_bird_relation")
            .expect("relation row")
            .outcome(),
        MatrixOutcomeKind::Accepted(ClassificationKind::MaintainedRelationWithFallback)
    );
    assert_eq!(
        matrix
            .row("designated_tick_publish_result")
            .expect("designated row")
            .outcome(),
        MatrixOutcomeKind::Accepted(ClassificationKind::DesignatedPublishValue)
    );
    assert_eq!(
        matrix
            .row("relation_absolute_pose_publication")
            .expect("absolute-pose row")
            .outcome(),
        MatrixOutcomeKind::Diagnostic(SurfaceV0DiagnosticKind::RelationMustPublishRelationCarrier)
    );
    assert_eq!(
        matrix
            .row("relation_consumer_mutation")
            .expect("consumer-mutation row")
            .outcome(),
        MatrixOutcomeKind::Diagnostic(SurfaceV0DiagnosticKind::ConsumerRelationMutationDenied)
    );
}
