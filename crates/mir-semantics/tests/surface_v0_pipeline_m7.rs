use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::{DeferredFormKind, FixtureSource, SurfaceV0Span, parse_surface_v0};
use mir_semantics::{
    evaluation_materialization::{
        AuthorityOrigin, EvaluationPolicy, EvaluationSite, InputFrontier, Locus, Materialization,
        ObservationPolicy, OccurrenceId as M3OccurrenceId, Principal, SemanticForm, TriggerClock,
    },
    shared_model::{
        BindingActivationFrontier, OccurrenceId, ResultFrontier, ResultKey, ResultVersion,
        SourceRef,
    },
    surface_v0_classification::{
        ClassificationKind, CoreTemplateKind, SourceToCoreKind, SurfaceV0Classification,
        SurfaceV0ClassificationOptions, classify_surface_v0,
    },
    surface_v0_pipeline::{
        CheckedBinaryOperator, CheckedEvaluationKind, EffectEntry, EffectKind, GeneratedObligation,
        GeneratedObligationKind, M7DiagnosticKind, ResidualObligationKind,
        SurfaceV0PipelineDiagnostics, check_and_elaborate_surface_v0,
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

fn run_fixture(
    name: &str,
) -> Result<impl std::fmt::Debug + Clone + Eq, SurfaceV0PipelineDiagnostics> {
    let (path, source) = load_fixture(name);
    check_and_elaborate_surface_v0(FixtureSource::new(path, source))
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn byte_range_nth(source: &str, needle: &str, occurrence: usize) -> Range<usize> {
    let mut search_from = 0;
    for _ in 0..occurrence {
        let relative_start = source[search_from..]
            .find(needle)
            .unwrap_or_else(|| panic!("fixture contains occurrence {occurrence} of {needle:?}"));
        search_from += relative_start + needle.len();
    }
    let relative_start = source[search_from..]
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains occurrence {occurrence} of {needle:?}"));
    let start = search_from + relative_start;
    start..start + needle.len()
}

fn expected_source_ref_for_range(path: &str, source: &str, range: Range<usize>) -> SourceRef {
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

fn expected_source_ref(path: &str, source: &str, needle: &str) -> SourceRef {
    let range = byte_range(source, needle);
    expected_source_ref_for_range(path, source, range)
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

fn assert_nonblank_site(site: &EvaluationSite) {
    let value = match site {
        EvaluationSite::Owner(locus)
        | EvaluationSite::Locus(locus)
        | EvaluationSite::DesignatedEvaluator(locus) => locus.as_str(),
        EvaluationSite::Consumer(principal) => principal.as_str(),
        EvaluationSite::Provider(provider) => provider.as_str(),
    };
    assert!(
        !value.is_empty(),
        "evaluation site identity must be nonblank"
    );
}

fn assert_nonblank_authority_origin(origin: &AuthorityOrigin) {
    let value = match origin {
        AuthorityOrigin::Caller(principal) => principal.as_str(),
        AuthorityOrigin::OwnerTransition(locus) | AuthorityOrigin::AdmittedEvaluator(locus) => {
            locus.as_str()
        }
        AuthorityOrigin::AdmittedProvider(provider) => provider.as_str(),
    };
    assert!(
        !value.is_empty(),
        "authority origin identity must be nonblank"
    );
}

fn assert_single_diagnostic(
    diagnostics: &SurfaceV0PipelineDiagnostics,
    expected_kind: M7DiagnosticKind,
    path: &str,
    source: &str,
    lexeme: &str,
) {
    assert_eq!(diagnostics.entries().len(), 1);
    assert!(!diagnostics.has_executable_core());
    let primary = diagnostics.primary();
    assert_eq!(primary.kind(), expected_kind);
    assert_eq!(primary.span().lexeme(source), lexeme);
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(path, source, lexeme)
    );
}

fn assert_single_diagnostic_at_range(
    diagnostics: &SurfaceV0PipelineDiagnostics,
    expected_kind: M7DiagnosticKind,
    path: &str,
    source: &str,
    expected_range: Range<usize>,
) {
    assert_eq!(diagnostics.entries().len(), 1);
    assert!(!diagnostics.has_executable_core());
    let primary = diagnostics.primary();
    assert_eq!(primary.kind(), expected_kind);
    assert_eq!(
        primary.span().byte_range(),
        expected_range,
        "diagnostic must point at the offending declaration, not the first declaration"
    );
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref_for_range(path, source, expected_range)
    );
}

fn assert_effect_entry(
    entries: &[EffectEntry],
    expected_kind: EffectKind,
    expected_source_ref: &SourceRef,
    source: &str,
    lexeme: &str,
) {
    let entry = entries
        .iter()
        .find(|entry| entry.kind() == expected_kind)
        .unwrap_or_else(|| panic!("missing effect entry {expected_kind:?}"));
    assert_eq!(entry.source_ref(), expected_source_ref);
    assert_eq!(entry.source_lexeme(source), lexeme);
}

fn assert_obligation_entry(
    entries: &[GeneratedObligation],
    expected_kind: GeneratedObligationKind,
    expected_source_ref: &SourceRef,
    source: &str,
    lexeme: &str,
) {
    let entry = entries
        .iter()
        .find(|entry| entry.kind() == &expected_kind)
        .unwrap_or_else(|| panic!("missing obligation entry {expected_kind:?}"));
    assert_eq!(entry.source_ref(), expected_source_ref);
    assert_eq!(entry.source_lexeme(source), lexeme);
}

fn assert_complete_m6_span_evidence(
    retained: &SurfaceV0Classification,
    expected: &SurfaceV0Classification,
    span: &SurfaceV0Span,
    expected_kinds: Vec<SourceToCoreKind>,
) {
    assert_eq!(
        retained.source_ref_for_span(span),
        expected.source_ref_for_span(span),
        "checked M7 artifact must publish the accepted M6 source reference for this span"
    );
    let retained_entries = retained.source_to_core_map().entries_for_span(span);
    assert_eq!(retained_entries.kinds(), expected_kinds);
    assert!(retained_entries.all_source_spans_equal(span));
    assert_eq!(
        retained_entries,
        expected.source_to_core_map().entries_for_span(span),
        "checked M7 artifact must publish the actual accepted M6 source-to-Core map entries"
    );
}

#[test]
fn canonical_source_checks_owner_failure_row_and_source_map() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical source checks and elaborates");

    assert_eq!(checked.source_file(), path);
    assert_eq!(
        checked.consumed_m6_classification().kind(),
        ClassificationKind::OwnerRmwWithRelationAndDesignated
    );
    assert!(
        checked
            .consumed_m6_classification()
            .template_names()
            .iter()
            .any(|name| name == "attack")
    );
    let attack = checked.evaluation("attack").expect("attack evaluation");
    assert_eq!(attack.kind(), CheckedEvaluationKind::OwnerRmw);
    assert_eq!(attack.actor_authority_origin(), "self");
    assert_eq!(attack.owner_evaluation_locus(), "S");
    assert_eq!(
        attack.declared_failure_row().names(),
        vec![
            "StaleMembership",
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
        ]
    );
    assert_eq!(
        attack.generated_failure_row().names(),
        vec![
            "StaleMembership",
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
        ]
    );
    assert!(
        attack
            .generated_failure_row()
            .is_subset_of(attack.declared_failure_row())
    );

    let assignment_lexeme = "player[target].hp = player[target].hp - player[self].atk";
    let assignment_ref = expected_source_ref(&path, &source, assignment_lexeme);
    let source_entries = checked
        .source_map()
        .entries_for_lexeme(&source, assignment_lexeme)
        .expect("assignment has source map entries");
    assert_eq!(
        source_entries.kinds(),
        vec![
            SourceToCoreKind::OwnerRmw,
            SourceToCoreKind::OwnerLocalRead,
            SourceToCoreKind::OwnerLocalWrite,
        ]
    );
    assert_eq!(source_entries.source_ref(), &assignment_ref);
}

#[test]
fn checked_surface_publishes_complete_accepted_m6_classification_evidence() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical source parses before M6 classification");
    let expected = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("canonical source has accepted M6 classification");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("canonical source checks and elaborates");
    let retained: &SurfaceV0Classification = checked.consumed_m6_classification();

    assert_eq!(
        retained, &expected,
        "M7 must retain the complete accepted M6 classification artifact, not reconstruct a name/kind summary"
    );
    assert_eq!(retained.root_source_ref(), expected.root_source_ref());

    let assignment = ast
        .assignment("player[target].hp")
        .expect("attack assignment");
    assert_complete_m6_span_evidence(
        retained,
        &expected,
        assignment.span(),
        vec![
            SourceToCoreKind::OwnerRmw,
            SourceToCoreKind::OwnerLocalRead,
            SourceToCoreKind::OwnerLocalWrite,
        ],
    );
    assert_eq!(
        retained.core_template("attack"),
        expected.core_template("attack")
    );
    let owner_template = retained.core_template("attack").expect("owner template");
    assert_eq!(owner_template.kind(), CoreTemplateKind::OwnerRmw);
    assert_eq!(owner_template.source_span(), assignment.span());

    let relation = ast.relation("bird_follow").expect("maintained relation");
    assert_complete_m6_span_evidence(
        retained,
        &expected,
        relation.span(),
        vec![
            SourceToCoreKind::PublishRelation,
            SourceToCoreKind::ConsumerLocalProjection,
        ],
    );
    assert_eq!(
        retained.relation_template("bird_follow"),
        expected.relation_template("bird_follow")
    );
    let relation_template = retained
        .relation_template("bird_follow")
        .expect("relation template");
    assert_eq!(
        relation_template.kind(),
        CoreTemplateKind::MaintainedRelation
    );
    assert_eq!(
        relation_template.owner_publication_kind(),
        CoreTemplateKind::PublishRelation
    );
    assert_eq!(
        relation_template.consumer_projection_kind(),
        CoreTemplateKind::ConsumerLocalProjection
    );

    let designated = ast.designated_result("E", "result").expect("designated E");
    assert_complete_m6_span_evidence(
        retained,
        &expected,
        designated.span(),
        vec![SourceToCoreKind::DesignatedDecision],
    );
    assert_eq!(
        retained.designated_template("E", "result"),
        expected.designated_template("E", "result")
    );
    let designated_template = retained
        .designated_template("E", "result")
        .expect("designated template");
    assert_eq!(
        designated_template.kind(),
        CoreTemplateKind::DesignatedPublishValue
    );
    assert_eq!(designated_template.source_span(), designated.span());

    for form in ast.deferred_forms().entries() {
        let kind = match form.kind() {
            DeferredFormKind::WithAuth => CoreTemplateKind::DeferredWithAuth,
            DeferredFormKind::Verify => CoreTemplateKind::DeferredVerify,
        };
        assert_complete_m6_span_evidence(
            retained,
            &expected,
            form.span(),
            vec![SourceToCoreKind::DeferredPolicy],
        );
        assert_eq!(
            retained.deferred_template(kind, form.name()),
            expected.deferred_template(kind, form.name())
        );
    }
}

#[test]
fn owner_rmw_checked_core_retains_typed_target_expression_and_same_owner_reads() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("canonical source checks and elaborates");

    let attack = checked.evaluation("attack").expect("attack evaluation");
    let owner_core = attack.owner_rmw_core().expect("owner RMW checked Core");
    assert_eq!(owner_core.owner_locus(), "S");
    assert_eq!(owner_core.target().namespace(), "player");
    assert_eq!(owner_core.target().index(), Some("target"));
    assert_eq!(owner_core.target().field(), Some("hp"));
    assert_eq!(
        owner_core.target().source_lexeme(&source),
        "player[target].hp"
    );

    let expression = owner_core.expression();
    assert_eq!(
        expression.source_lexeme(&source),
        "player[target].hp - player[self].atk"
    );
    assert_eq!(expression.operator_chain(), vec!["-"]);
    assert!(expression.int_literals().is_empty());
    assert_eq!(expression.state_reads().len(), 2);
    assert!(
        expression
            .state_reads()
            .iter()
            .all(|read| read.owner_locus() == "S")
    );
    assert_eq!(expression.state_reads()[0].namespace(), "player");
    assert_eq!(expression.state_reads()[0].index(), Some("target"));
    assert_eq!(expression.state_reads()[0].field(), Some("hp"));
    assert_eq!(expression.state_reads()[1].namespace(), "player");
    assert_eq!(expression.state_reads()[1].index(), Some("self"));
    assert_eq!(expression.state_reads()[1].field(), Some("atk"));
    assert_eq!(owner_core.same_owner_reads(), expression.state_reads());
}

#[test]
fn checked_core_retains_left_associated_ordered_expression_tree_with_spans() {
    let (path, source) = load_fixture("m7_ordered_expression_tree.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("ordered expression source checks and elaborates");

    let attack = checked.evaluation("attack").expect("attack evaluation");
    let owner_tree = attack
        .owner_rmw_core()
        .expect("owner RMW checked Core")
        .expression()
        .tree();
    assert!(owner_tree.is_m8_consumable());
    assert_eq!(
        owner_tree.source_lexeme(&source),
        "player[self].hp - player[self].atk + 1"
    );
    assert_eq!(owner_tree.operator(), Some(CheckedBinaryOperator::Add));
    assert_eq!(
        owner_tree.left().operator(),
        Some(CheckedBinaryOperator::Subtract)
    );
    assert_eq!(
        owner_tree.left().left().source_lexeme(&source),
        "player[self].hp"
    );
    assert_eq!(
        owner_tree.left().right().source_lexeme(&source),
        "player[self].atk"
    );
    assert_eq!(owner_tree.right().int_literal().unwrap().value(), 1);
    assert_eq!(
        owner_tree.left().source_lexeme(&source),
        "player[self].hp - player[self].atk"
    );

    let designated = checked
        .designated_result("E", "result")
        .expect("designated result evaluation");
    let designated_tree = designated
        .designated_core()
        .expect("designated checked Core")
        .expression()
        .tree();
    assert!(designated_tree.is_m8_consumable());
    assert_eq!(
        designated_tree.source_lexeme(&source),
        "player[self].hp - 1 + player[self].atk"
    );
    assert_eq!(designated_tree.operator(), Some(CheckedBinaryOperator::Add));
    assert_eq!(
        designated_tree.left().operator(),
        Some(CheckedBinaryOperator::Subtract)
    );
    assert_eq!(
        designated_tree.left().left().source_lexeme(&source),
        "player[self].hp"
    );
    assert_eq!(
        designated_tree
            .left()
            .right()
            .int_literal()
            .unwrap()
            .value(),
        1
    );
    assert_eq!(
        designated_tree.right().source_lexeme(&source),
        "player[self].atk"
    );
    assert_eq!(
        designated_tree.left().source_lexeme(&source),
        "player[self].hp - 1"
    );
}

#[test]
fn relation_retains_projection_binding_frontier_and_visibility_residuals() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("canonical source checks and elaborates");

    let relation = checked
        .relation("bird_follow")
        .expect("maintained relation evaluation");
    assert_eq!(relation.kind(), CheckedEvaluationKind::PublishRelation);
    assert!(relation.publishes_relation_carrier());
    assert_eq!(relation.consumer_projection_locus(), Some("C"));
    assert_eq!(
        relation.consumer_projection_kind(),
        CheckedEvaluationKind::ConsumerLocalProjection
    );
    assert_eq!(
        relation.binding_frontier(),
        &BindingActivationFrontier::from_ordered_occurrences(vec![OccurrenceId::new(
            "bird_binding_frontier"
        )])
        .expect("single binding activation frontier")
    );

    let residuals = checked.residual_obligations();
    assert!(residuals.contains_kind(ResidualObligationKind::Visibility));
    assert!(residuals.contains_kind(ResidualObligationKind::RelationLifetime));
    assert!(
        residuals
            .for_kind_and_name(ResidualObligationKind::Visibility, "bird_follow")
            .expect("relation visibility residual")
            .source_lexeme(&source)
            .contains("relation bird_follow")
    );
    assert!(
        residuals
            .for_kind_and_name(ResidualObligationKind::RelationLifetime, "bird_follow")
            .expect("relation lifetime residual")
            .is_non_executable()
    );
    let fallback = residuals
        .for_kind_and_name(ResidualObligationKind::FallbackValidity, "bird_follow")
        .expect("relation fallback-validity residual");
    assert!(fallback.is_non_executable());
    assert_ne!(
        ResidualObligationKind::FallbackValidity,
        ResidualObligationKind::RelationLifetime
    );
}

#[test]
fn relation_checked_core_retains_m8_consumable_relation_shape() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("canonical source checks and elaborates");

    let relation = checked
        .relation("bird_follow")
        .expect("maintained relation evaluation");
    let relation_core = relation.relation_core().expect("relation checked Core");
    assert_eq!(relation_core.owner_locus(), "S");
    assert_eq!(relation_core.subject(), "bird");
    assert_eq!(relation_core.subject_type(), "Player");
    assert_eq!(
        relation_core.binding_frontier(),
        &BindingActivationFrontier::from_ordered_occurrences(vec![OccurrenceId::new(
            "bird_binding_frontier"
        )])
        .expect("single binding activation frontier")
    );
    assert_eq!(relation_core.consumer_projection_locus(), Some("C"));
    assert!(relation_core.publishes_relation_carrier());

    let primary = relation_core.primary();
    assert_eq!(primary.anchor(), "perch_anchor");
    assert_eq!(primary.epoch(), "primary_epoch");
    assert_eq!(primary.transform().kind(), "translate");
    assert_eq!(primary.transform().translation(), Some((3, -2)));

    let fallback = relation_core.fallback();
    assert_eq!(fallback.anchor(), "nest_anchor");
    assert_eq!(fallback.epoch(), "fallback_epoch");
    assert_eq!(fallback.transform().kind(), "identity");
    assert_eq!(fallback.transform().translation(), Some((0, 0)));
}

#[test]
fn designated_result_preserves_result_frontier_version_and_publish_value() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("canonical source checks and elaborates");

    let designated = checked
        .designated_result("E", "result")
        .expect("designated result evaluation");
    assert_eq!(
        designated.kind(),
        CheckedEvaluationKind::DesignatedPublishValue
    );
    assert_eq!(
        designated.result_frontier(),
        &ResultFrontier::from_ordered_results(vec![ResultKey::new("F")])
            .expect("single result frontier")
    );
    assert_eq!(designated.result_version(), ResultVersion::new(1));
    assert!(designated.publishes_value());

    let value_visibility = checked
        .residual_obligations()
        .for_kind_and_name(ResidualObligationKind::ValueVisibilityRedaction, "E.result")
        .expect("designated value visibility/redaction residual");
    assert!(value_visibility.is_non_executable());
}

#[test]
fn designated_checked_core_retains_tick_value_expression_and_remote_input_dependencies() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("canonical source checks and elaborates");

    let designated = checked
        .designated_result("E", "result")
        .expect("designated result evaluation");
    let designated_core = designated
        .designated_core()
        .expect("designated checked Core");
    assert_eq!(designated_core.evaluator(), "E");
    assert_eq!(designated_core.result(), "result");
    assert_eq!(designated_core.trigger().kind(), "logical-tick");
    assert_eq!(designated_core.trigger().frontier(), Some("F"));
    assert_eq!(
        designated_core.result_frontier(),
        &ResultFrontier::from_ordered_results(vec![ResultKey::new("F")])
            .expect("single result frontier")
    );
    assert_eq!(
        designated_core.input_frontier(),
        &InputFrontier::from_ordered_producers(vec![M3OccurrenceId::new("F")])
            .expect("single M3 input frontier")
    );
    assert_eq!(designated_core.result_version(), ResultVersion::new(1));
    let expected_eval_policy = EvaluationPolicy::declared_deterministic("inferred:E.result");
    let expected_observation_policy = ObservationPolicy::declared("conservative");
    assert_eq!(designated_core.evaluation_policy(), &expected_eval_policy);
    assert!(designated_core.evaluation_policy().deterministic);
    assert_eq!(
        designated_core.observation_policy(),
        &expected_observation_policy
    );
    assert_eq!(
        designated_core.policy_stamp(),
        &expected_eval_policy.stamp_with(&expected_observation_policy)
    );
    assert_eq!(designated_core.materialization().kind(), "publish-value");
    assert_eq!(
        designated_core.expression().source_lexeme(&source),
        "player[self].atk + 1"
    );
    assert_eq!(designated_core.expression().operator_chain(), vec!["+"]);
    assert_eq!(designated_core.expression().int_literals(), vec![1]);
    assert_eq!(designated_core.expression().state_reads().len(), 1);
    assert_eq!(
        designated_core.expression().state_reads()[0].owner_locus(),
        "S"
    );

    let dependencies = designated_core.generated_remote_input_dependencies();
    assert_eq!(dependencies.len(), 1);
    let dependency = &dependencies[0];
    assert_eq!(dependency.designated_evaluator(), "E");
    assert_eq!(
        dependency.requester_site(),
        &EvaluationSite::DesignatedEvaluator(Locus::new("E"))
    );
    assert_eq!(
        dependency.authority_origin(),
        &AuthorityOrigin::AdmittedEvaluator(Locus::new("E"))
    );
    assert_eq!(dependency.source_owner_locus(), "S");
    assert_eq!(dependency.typed_state_read().namespace(), "player");
    assert_eq!(dependency.typed_state_read().index(), Some("self"));
    assert_eq!(dependency.typed_state_read().field(), Some("atk"));
    assert_eq!(dependency.typed_state_read().value_type(), "Int");
    assert_eq!(dependency.request().source_owner_locus(), "S");
    assert_eq!(
        dependency.request().typed_state_read(),
        dependency.typed_state_read()
    );
    assert_eq!(dependency.receipt_use().source_owner_locus(), "S");
    assert_eq!(
        dependency.receipt_use().typed_state_read(),
        dependency.typed_state_read()
    );
}

#[test]
fn evaluation_axes_are_typed_orthogonal_and_nonblank() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("canonical source checks and elaborates");

    let attack = checked.evaluation("attack").expect("attack evaluation");
    let attack_axes = attack.evaluation_axes();
    assert_eq!(attack_axes.semantic_form(), SemanticForm::State);
    assert_eq!(
        attack_axes.evaluation_site(),
        &EvaluationSite::Owner(Locus::new("S"))
    );
    assert_eq!(attack_axes.trigger(), TriggerClock::OnEvent);
    assert_eq!(
        attack_axes.authority_origin(),
        &AuthorityOrigin::Caller(Principal::new("self"))
    );
    assert_eq!(attack_axes.materialization(), Materialization::Store);
    assert_nonblank_site(attack_axes.evaluation_site());
    assert_nonblank_authority_origin(attack_axes.authority_origin());

    let relation = checked
        .relation("bird_follow")
        .expect("maintained relation evaluation");
    let relation_axes = relation.evaluation_axes();
    assert_eq!(relation_axes.semantic_form(), SemanticForm::Relation);
    assert_eq!(
        relation_axes.evaluation_site(),
        &EvaluationSite::Owner(Locus::new("S"))
    );
    assert_eq!(relation_axes.trigger(), TriggerClock::FrontierAdvance);
    assert_eq!(
        relation_axes.authority_origin(),
        &AuthorityOrigin::OwnerTransition(Locus::new("S"))
    );
    assert_eq!(
        relation_axes.materialization(),
        Materialization::PublishRelation
    );
    assert_nonblank_site(relation_axes.evaluation_site());
    assert_nonblank_authority_origin(relation_axes.authority_origin());

    let designated = checked
        .designated_result("E", "result")
        .expect("designated result evaluation");
    let designated_axes = designated.evaluation_axes();
    assert_eq!(designated_axes.semantic_form(), SemanticForm::Value);
    assert_eq!(
        designated_axes.evaluation_site(),
        &EvaluationSite::DesignatedEvaluator(Locus::new("E"))
    );
    assert_eq!(designated_axes.trigger(), TriggerClock::LogicalTick);
    assert_eq!(
        designated_axes.authority_origin(),
        &AuthorityOrigin::AdmittedEvaluator(Locus::new("E"))
    );
    assert_eq!(
        designated_axes.materialization(),
        Materialization::PublishValue
    );
    assert_nonblank_site(designated_axes.evaluation_site());
    assert_nonblank_authority_origin(designated_axes.authority_origin());
}

#[test]
fn effect_and_generated_obligation_rows_are_public_enumerable_and_source_bound() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical source checks and elaborates");

    let assignment_lexeme = "player[target].hp = player[target].hp - player[self].atk";
    let assignment_ref = expected_source_ref(&path, &source, assignment_lexeme);
    let attack = checked.evaluation("attack").expect("attack evaluation");
    let attack_effects = attack.effect_row().entries();
    assert_effect_entry(
        attack_effects,
        EffectKind::OwnerRequest,
        &assignment_ref,
        &source,
        assignment_lexeme,
    );
    assert_effect_entry(
        attack_effects,
        EffectKind::OwnerLocalRead,
        &assignment_ref,
        &source,
        assignment_lexeme,
    );
    assert_effect_entry(
        attack_effects,
        EffectKind::OwnerWrite,
        &assignment_ref,
        &source,
        assignment_lexeme,
    );

    let attack_obligations = attack.generated_obligations().entries();
    for failure in [
        "StaleMembership",
        "MissingCapability",
        "MissingWitness",
        "RouteUnavailable",
    ] {
        assert_obligation_entry(
            attack_obligations,
            GeneratedObligationKind::Failure(failure.to_string()),
            &assignment_ref,
            &source,
            assignment_lexeme,
        );
    }
    assert_obligation_entry(
        attack_obligations,
        GeneratedObligationKind::Capability,
        &assignment_ref,
        &source,
        assignment_lexeme,
    );
    assert_obligation_entry(
        attack_obligations,
        GeneratedObligationKind::Witness,
        &assignment_ref,
        &source,
        assignment_lexeme,
    );
    assert_obligation_entry(
        attack_obligations,
        GeneratedObligationKind::Evaluation(CheckedEvaluationKind::OwnerRmw),
        &assignment_ref,
        &source,
        assignment_lexeme,
    );

    let relation_lexeme = "relation bird_follow at S {\n  subject bird: Player\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}";
    let relation_ref = expected_source_ref(&path, &source, relation_lexeme);
    let relation = checked
        .relation("bird_follow")
        .expect("maintained relation evaluation");
    assert_effect_entry(
        relation.effect_row().entries(),
        EffectKind::RelationPublish,
        &relation_ref,
        &source,
        relation_lexeme,
    );
    assert_obligation_entry(
        relation.generated_obligations().entries(),
        GeneratedObligationKind::Authority,
        &relation_ref,
        &source,
        relation_lexeme,
    );
    assert_obligation_entry(
        relation.generated_obligations().entries(),
        GeneratedObligationKind::Evaluation(CheckedEvaluationKind::PublishRelation),
        &relation_ref,
        &source,
        relation_lexeme,
    );

    let designated_lexeme = "designated evaluate E on tick F publish result = player[self].atk + 1";
    let designated_ref = expected_source_ref(&path, &source, designated_lexeme);
    let designated = checked
        .designated_result("E", "result")
        .expect("designated result evaluation");
    let designated_effects = designated.effect_row().entries();
    assert_effect_entry(
        designated_effects,
        EffectKind::DesignatedRemoteRequest,
        &designated_ref,
        &source,
        designated_lexeme,
    );
    assert_effect_entry(
        designated_effects,
        EffectKind::DesignatedReceiptUse,
        &designated_ref,
        &source,
        designated_lexeme,
    );
    assert_effect_entry(
        designated_effects,
        EffectKind::DesignatedValuePublish,
        &designated_ref,
        &source,
        designated_lexeme,
    );
    let designated_obligations = designated.generated_obligations().entries();
    assert_obligation_entry(
        designated_obligations,
        GeneratedObligationKind::AdmittedEvaluatorAuthority,
        &designated_ref,
        &source,
        designated_lexeme,
    );
    assert_obligation_entry(
        designated_obligations,
        GeneratedObligationKind::Evaluation(CheckedEvaluationKind::DesignatedPublishValue),
        &designated_ref,
        &source,
        designated_lexeme,
    );
    assert!(
        designated_obligations
            .iter()
            .all(|entry| !entry.grants_authority_success()),
        "admitted evaluator remains an obligation, not authority evidence success"
    );
}

#[test]
fn source_to_core_map_is_total_and_stably_enumerable_for_checked_surface_spans() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("canonical source checks and elaborates");

    let entries = checked.source_map().entries();
    assert!(!entries.is_empty());
    for (index, entry) in entries.iter().enumerate() {
        assert_eq!(entry.ordinal(), index);
        assert!(!entry.core_ref().is_empty());
        assert_eq!(entry.source_ref().path, checked.source_file());
    }
    assert!(
        entries
            .windows(2)
            .all(|pair| pair[0].stable_key() < pair[1].stable_key())
    );

    for lexeme in [
        "player[target].hp = player[target].hp - player[self].atk",
        "relation bird_follow at S {\n  subject bird: Player\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}",
        "designated evaluate E on tick F publish result = player[self].atk + 1",
        "with auth MembershipAuth",
        "verify finite_refinement",
    ] {
        let mapped = entries
            .iter()
            .filter(|entry| entry.source_lexeme(&source) == lexeme)
            .collect::<Vec<_>>();
        assert!(
            !mapped.is_empty(),
            "{lexeme:?} must have source-to-Core entries"
        );
    }
}

#[test]
fn auth_and_verify_are_residual_only_without_execution_side_effects() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("canonical source checks and elaborates");

    let auth = checked
        .residual_obligations()
        .for_kind_and_name(ResidualObligationKind::AuthDeferred, "MembershipAuth")
        .expect("auth residual");
    assert_eq!(auth.source_lexeme(&source), "with auth MembershipAuth");
    assert_eq!(auth.required_authority(), Some("MembershipAuth"));
    assert!(auth.is_non_executable());
    assert!(!auth.grants_authority());
    assert!(!auth.emits_effect());
    assert!(!auth.mutates_state());
    assert!(!auth.emits_verdict());

    let verify = checked
        .residual_obligations()
        .for_kind_and_name(ResidualObligationKind::VerifyDeferred, "finite_refinement")
        .expect("verify residual");
    assert_eq!(verify.source_lexeme(&source), "verify finite_refinement");
    assert!(verify.required_authority().is_none());
    assert!(verify.is_non_executable());
    assert!(!verify.grants_authority());
    assert!(!verify.emits_effect());
    assert!(!verify.mutates_state());
    assert!(!verify.emits_verdict());
}

#[test]
fn same_source_input_elaboration_is_eq_and_deterministic() {
    let first = run_fixture("canonical_attack_bundle.mir").expect("first elaboration");
    let second = run_fixture("canonical_attack_bundle.mir").expect("second elaboration");
    assert_eq!(first, second);
}

#[test]
fn m6_invalid_source_is_forwarded_as_typed_diagnostic_without_executable_core() {
    let (path, source) = load_fixture("cross_owner_without_receipt.mir");
    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect_err("M6 static diagnostic is forwarded");
    assert_single_diagnostic(
        &diagnostics,
        M7DiagnosticKind::CrossOwnerOperandRequiresReceipt,
        &path,
        &source,
        "enemy[target].atk",
    );
}

#[test]
fn m6_classification_diagnostics_take_precedence_over_m7_failure_row_checks() {
    let (path, source) = load_fixture("m7_cross_owner_and_underdeclared_failure.mir");
    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect_err("M6 classification must run before M7 finite checks");
    assert_single_diagnostic(
        &diagnostics,
        M7DiagnosticKind::CrossOwnerOperandRequiresReceipt,
        &path,
        &source,
        "enemy[target].atk",
    );
}

#[test]
fn m7_rejects_unsupported_finite_expression_after_m6_token_collector_accepts_it() {
    let (path, source) = load_fixture("m7_unsupported_punctuation_expression.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("M6 broad token collector accepts punctuation expression");
    assert!(
        ast.assignment("player[self].hp")
            .expect("punctuation assignment")
            .expression()
            .tokens()
            .iter()
            .any(|token| token.span().lexeme(&source) == ",")
    );

    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect_err("M7 finite checker rejects unsupported expression shape");
    assert_single_diagnostic_at_range(
        &diagnostics,
        M7DiagnosticKind::UnsupportedExpression,
        &path,
        &source,
        byte_range(&source, "(player[self].hp + (player[self].atk, 1))"),
    );
}

#[test]
fn m7_rejects_canon_m6_expr_token_punctuation_as_typed_unsupported_expression() {
    let (path, source) = load_fixture("m7_unsupported_expr_token_set_punctuation.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("M6 broad token collector accepts Canon punctuation tokens");
    let expression = ast
        .assignment("player[self].hp")
        .expect("punctuation token-set assignment")
        .expression();
    assert_eq!(
        expression
            .tokens()
            .iter()
            .map(|token| token.span().lexeme(&source))
            .collect::<Vec<_>>(),
        vec![
            "[", "player", "[", "self", "]", ".", "hp", ":", "player", "[", "self", "]", ".",
            "atk", "=", ".", "]",
        ]
    );

    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect_err("M7 finite checker rejects non-finite broad punctuation expression");
    assert_single_diagnostic_at_range(
        &diagnostics,
        M7DiagnosticKind::UnsupportedExpression,
        &path,
        &source,
        byte_range(&source, "[ player[self].hp : player[self].atk = . ]"),
    );
}

#[test]
fn m7_rejects_underdeclared_rows_duplicate_declarations_and_unknown_fields() {
    for (fixture, expected_kind, lexeme) in [
        (
            "m7_underdeclared_failure_row.mir",
            M7DiagnosticKind::GeneratedFailureNotDeclared,
            "fails (StaleMembership)",
        ),
        (
            "m7_unknown_state_field.mir",
            M7DiagnosticKind::UnknownStateField,
            "shield",
        ),
    ] {
        let (path, source) = load_fixture(fixture);
        let diagnostics =
            check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
                .expect_err("{fixture} rejects with typed M7 diagnostic");
        assert_single_diagnostic(&diagnostics, expected_kind, &path, &source, lexeme);
    }

    let (path, source) = load_fixture("m7_duplicate_declaration.mir");
    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
            .expect_err("duplicate declaration rejects with typed M7 diagnostic");
    assert_single_diagnostic_at_range(
        &diagnostics,
        M7DiagnosticKind::DuplicateDeclaration,
        &path,
        &source,
        byte_range_nth(&source, "locus S", 1),
    );
}

#[test]
fn finite_m7_checks_reject_type_and_declaration_consistency_gaps() {
    for (fixture, expected_kind, lexeme, occurrence) in [
        (
            "m7_type_mismatch.mir",
            M7DiagnosticKind::TypeMismatch,
            "player[self].name",
            0,
        ),
        (
            "m7_arithmetic_requires_int.mir",
            M7DiagnosticKind::ArithmeticRequiresInt,
            "+",
            0,
        ),
        (
            "m7_undefined_state_index_type.mir",
            M7DiagnosticKind::UndefinedStateIndexType,
            "Player",
            0,
        ),
        (
            "m7_undefined_state_field_type.mir",
            M7DiagnosticKind::UndefinedStateFieldType,
            "HitPoints",
            0,
        ),
        (
            "m7_undefined_relation_subject_type.mir",
            M7DiagnosticKind::UndefinedRelationSubjectType,
            "Bird",
            0,
        ),
        (
            "m7_undefined_owner_locus.mir",
            M7DiagnosticKind::UndefinedOwnerLocus,
            "MissingOwner",
            0,
        ),
        (
            "m7_undefined_consumer_locus.mir",
            M7DiagnosticKind::UndefinedConsumerLocus,
            "MissingConsumer",
            0,
        ),
        (
            "m7_undeclared_self_principal.mir",
            M7DiagnosticKind::UndefinedSelfPrincipal,
            "self",
            0,
        ),
        (
            "m7_undefined_role_locus.mir",
            M7DiagnosticKind::UndefinedRoleEvaluationLocus,
            "MissingRoleLocus",
            0,
        ),
        (
            "m7_duplicate_state_field.mir",
            M7DiagnosticKind::DuplicateStateField,
            "hp: Int",
            1,
        ),
        (
            "m7_designated_unknown_state_field.mir",
            M7DiagnosticKind::UnknownStateField,
            "hp",
            0,
        ),
        (
            "m7_duplicate_event.mir",
            M7DiagnosticKind::DuplicateEvent,
            "when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable)",
            1,
        ),
        (
            "m7_duplicate_relation.mir",
            M7DiagnosticKind::DuplicateRelation,
            "relation bird_follow at S",
            1,
        ),
        (
            "m7_duplicate_designated.mir",
            M7DiagnosticKind::DuplicateDesignated,
            "designated evaluate E on tick F publish result = player[self].atk + 1",
            1,
        ),
        (
            "m7_duplicate_deferred.mir",
            M7DiagnosticKind::DuplicateDeferred,
            "verify finite_refinement",
            1,
        ),
    ] {
        let (path, source) = load_fixture(fixture);
        let diagnostics =
            check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
                .expect_err("{fixture} rejects with typed M7 diagnostic");
        assert_single_diagnostic_at_range(
            &diagnostics,
            expected_kind,
            &path,
            &source,
            byte_range_nth(&source, lexeme, occurrence),
        );
    }
}

#[test]
fn residual_only_source_checks_static_but_rejects_explicit_execution_admission() {
    let (path, source) = load_fixture("m7_residual_cannot_execute.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("residual-only source still checks as a static artifact");

    let auth = checked
        .residual_obligations()
        .for_kind_and_name(ResidualObligationKind::AuthDeferred, "MembershipAuth")
        .expect("auth residual");
    assert_eq!(auth.source_lexeme(&source), "with auth MembershipAuth");
    assert!(auth.is_non_executable());

    let verify = checked
        .residual_obligations()
        .for_kind_and_name(ResidualObligationKind::VerifyDeferred, "finite_refinement")
        .expect("verify residual");
    assert_eq!(verify.source_lexeme(&source), "verify finite_refinement");
    assert!(verify.is_non_executable());

    assert!(!checked.execution_is_admissible());
    let diagnostics = checked
        .require_execution_admission()
        .expect_err("residual-only artifact cannot be admitted for runtime execution");
    assert_single_diagnostic(
        &diagnostics,
        M7DiagnosticKind::ResidualCannotExecute,
        &path,
        &source,
        "verify finite_refinement",
    );
}

#[test]
fn ten_row_m7_fixture_matrix_uses_real_parse_check_and_elaboration() {
    let rows = [
        ("canonical_attack_bundle.mir", Ok(())),
        ("maintained_bird_relation.mir", Ok(())),
        ("designated_tick_publish_result.mir", Ok(())),
        (
            "cross_owner_without_receipt.mir",
            Err(M7DiagnosticKind::CrossOwnerOperandRequiresReceipt),
        ),
        (
            "relation_absolute_pose_publication.mir",
            Err(M7DiagnosticKind::RelationMustPublishRelationCarrier),
        ),
        (
            "relation_consumer_mutation.mir",
            Err(M7DiagnosticKind::ConsumerRelationMutationDenied),
        ),
        (
            "m7_underdeclared_failure_row.mir",
            Err(M7DiagnosticKind::GeneratedFailureNotDeclared),
        ),
        (
            "m7_duplicate_declaration.mir",
            Err(M7DiagnosticKind::DuplicateDeclaration),
        ),
        (
            "m7_unknown_state_field.mir",
            Err(M7DiagnosticKind::UnknownStateField),
        ),
        ("m7_residual_cannot_execute.mir", Ok(())),
    ];

    let mut accepted = 0;
    let mut rejected = 0;
    for (fixture, expected) in rows {
        let (path, source) = load_fixture(fixture);
        let actual = check_and_elaborate_surface_v0(FixtureSource::new(path, source));
        match expected {
            Ok(()) => {
                actual.unwrap_or_else(|diagnostics: SurfaceV0PipelineDiagnostics| {
                    panic!(
                        "{fixture} should accept, got {:?}",
                        diagnostics.primary().kind()
                    )
                });
                accepted += 1;
            }
            Err(expected_kind) => {
                let diagnostics =
                    actual.expect_err("{fixture} should reject with typed M7 diagnostic");
                rejected += 1;
                assert_eq!(diagnostics.primary().kind(), expected_kind, "{fixture}");
                assert!(!diagnostics.has_executable_core(), "{fixture}");
            }
        }
    }
    assert_eq!(accepted, 4);
    assert_eq!(rejected, 6);
    assert_eq!(accepted + rejected, 10);
}

#[test]
fn execution_admission_succeeds_only_for_checked_artifacts_without_residual_obligations() {
    let (path, source) = load_fixture("m7_owner_only_no_residuals.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("owner-only source checks and elaborates");

    assert!(checked.residual_obligations().is_empty());
    assert!(checked.execution_is_admissible());
    checked
        .require_execution_admission()
        .expect("residual-free checked artifact is admissible for execution");

    let (path, source) = load_fixture("m7_residual_cannot_execute.mir");
    let residual_checked = check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("residual-only source still checks as a static artifact");
    assert!(!residual_checked.residual_obligations().is_empty());
    assert!(!residual_checked.execution_is_admissible());
    assert!(matches!(
        residual_checked
            .require_execution_admission()
            .expect_err("residual artifact cannot be admitted")
            .primary()
            .kind(),
        M7DiagnosticKind::ResidualCannotExecute
    ));
}
