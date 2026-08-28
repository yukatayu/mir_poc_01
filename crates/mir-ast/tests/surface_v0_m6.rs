use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::{
    BoundedBinaryOperator, DeferredFormKind, FixtureSource, ParseErrorKind, RelationTransform,
    SyntaxKind, parse_surface_v0,
};

const FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const SYS5_ANCHOR_LOCUS_FIXTURE: &str = "sys5_relation_anchor_locus_boundary.mir";

fn fixture_path(name: &str) -> String {
    format!("{FIXTURE_DIR}/{name}")
}

fn load_fixture(name: &str) -> (String, String) {
    let relative = fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn byte_range_after(source: &str, anchor: &str, needle: &str) -> Range<usize> {
    let anchor_start = source
        .find(anchor)
        .unwrap_or_else(|| panic!("fixture contains anchor {anchor:?}"));
    let relative_start = source[anchor_start..]
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?} after {anchor:?}"));
    let start = anchor_start + relative_start;
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

fn byte_range_nth_after_offset(
    source: &str,
    byte_start: usize,
    needle: &str,
    occurrence: usize,
) -> Range<usize> {
    let mut search_from = byte_start;
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

fn parse_error_kind_name(kind: ParseErrorKind) -> String {
    format!("{kind:?}")
}

fn token_lexemes<'a>(
    source: &'a str,
    expression: &'a mir_ast::surface_v0::BoundedExpression,
) -> Vec<&'a str> {
    expression
        .tokens()
        .iter()
        .map(|token| token.span().lexeme(source))
        .collect()
}

#[test]
fn parses_surface_v0_bundle_with_canonical_file_and_child_spans() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical surface-v0 source parses");

    let root = ast.root();
    assert_eq!(root.kind(), SyntaxKind::Module);
    assert_eq!(root.span().file(), path);
    assert_eq!(root.span().byte_range(), 0..source.len());
    assert_eq!(root.span().start_line_column(), (1, 1));
    assert_eq!(root.span().end_line_column(), (36, 1));
    assert_eq!(ast.module().name(), "Combat.M6");

    assert_eq!(ast.locus("S").expect("locus S").span().file(), path);
    assert_eq!(ast.locus("C").expect("locus C").span().file(), path);
    assert_eq!(
        ast.principal("self")
            .expect("principal self")
            .span()
            .byte_range(),
        byte_range(&source, "principal self")
    );
    assert_eq!(
        ast.state("player")
            .expect("indexed state")
            .span()
            .byte_range(),
        byte_range(
            &source,
            "state player[id: Player] at S {\n  hp: Int\n  atk: Int\n}"
        )
    );

    let role = ast
        .find_node(SyntaxKind::RoleInstance, "Role[self] at S")
        .expect("role instance node");
    assert_eq!(
        role.span().byte_range(),
        byte_range(&source, "Role[self] at S")
    );
    assert_eq!(role.children().len(), 1);

    let attack = ast.when("attack").expect("attack event");
    assert_eq!(
        attack.span().byte_range(),
        byte_range(
            &source,
            "when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable)"
        )
    );
    assert_eq!(attack.parameters()[0].name(), "target");
    assert_eq!(attack.parameters()[0].type_name(), "Player");

    let assignment = ast
        .assignment("player[target].hp")
        .expect("attack assignment");
    assert_eq!(assignment.owner_locus(), "S");
    assert_eq!(
        assignment.span().byte_range(),
        byte_range(
            &source,
            "player[target].hp = player[target].hp - player[self].atk"
        )
    );
    assert!(assignment.span().is_child_of(root.span()));

    let relation = ast.relation("bird_follow").expect("maintained relation");
    assert_eq!(relation.owner_locus(), "S");
    assert_eq!(relation.primary().anchor(), "perch_anchor");
    assert_eq!(relation.fallback().anchor(), "nest_anchor");
    assert_eq!(relation.binding_frontier(), "bird_binding_frontier");
    assert_eq!(relation.publish_materialization(), "publish-relation");
    assert_eq!(relation.consumer_projection_locus(), Some("C"));

    let designated = ast.designated_result("E", "result").expect("designated E");
    assert_eq!(designated.tick_frontier(), "F");
    assert_eq!(designated.materialization(), "publish-value");
    assert_eq!(designated.span().file(), path);

    assert!(
        ast.deferred_forms()
            .contains(DeferredFormKind::WithAuth, "MembershipAuth")
    );
    assert!(
        ast.deferred_forms()
            .contains(DeferredFormKind::Verify, "finite_refinement")
    );
}

#[test]
fn parser_retains_designated_result_consumer_ref_and_spans() {
    let (path, source) = load_fixture("designated_result_consume_three_locus.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("designated consume source parses");

    let consumer = ast
        .designated_result_consumer("E", "result", "C")
        .expect("designated result consumer is explicit AST");
    assert_eq!(consumer.evaluator(), "E");
    assert_eq!(consumer.result(), "result");
    assert_eq!(consumer.consumer_locus(), "C");
    assert_eq!(
        consumer.span().byte_range(),
        byte_range(&source, "designated consume E.result at C")
    );
    assert_eq!(
        consumer.result_ref_span().byte_range(),
        byte_range(&source, "E.result")
    );
    assert_eq!(
        consumer.consumer_locus_span().byte_range(),
        byte_range_after(&source, "designated consume", "C")
    );
    assert_eq!(consumer.span().file(), path);
    assert!(consumer.result_ref_span().is_child_of(consumer.span()));
    assert!(consumer.consumer_locus_span().is_child_of(consumer.span()));
}

#[test]
fn parser_retains_bounded_assignment_designated_exprs_and_relation_transforms_with_spans() {
    let (path, source) = load_fixture("canonical_attack_bundle.mir");
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical surface-v0 source parses");

    let assignment = ast
        .assignment("player[target].hp")
        .expect("attack assignment");
    let assignment_expr = assignment.expression();
    assert_eq!(
        assignment_expr.span().byte_range(),
        byte_range(&source, "player[target].hp - player[self].atk")
    );
    assert_eq!(assignment_expr.state_refs().len(), 2);
    assert_eq!(assignment_expr.state_refs()[0].base(), "player");
    assert_eq!(assignment_expr.state_refs()[0].index(), Some("target"));
    assert_eq!(assignment_expr.state_refs()[0].field(), Some("hp"));
    assert_eq!(
        assignment_expr.state_refs()[0].span().byte_range(),
        byte_range_nth(&source, "player[target].hp", 1)
    );
    assert_eq!(assignment_expr.state_refs()[1].base(), "player");
    assert_eq!(assignment_expr.state_refs()[1].index(), Some("self"));
    assert_eq!(assignment_expr.state_refs()[1].field(), Some("atk"));
    assert_eq!(
        assignment_expr.state_refs()[1].span().byte_range(),
        byte_range(&source, "player[self].atk")
    );
    assert!(assignment_expr.int_literals().is_empty());
    assert_eq!(assignment_expr.binary_ops().len(), 1);
    assert_eq!(
        assignment_expr.binary_ops()[0].operator(),
        BoundedBinaryOperator::Subtract
    );
    assert_eq!(
        assignment_expr.binary_ops()[0].span().byte_range(),
        byte_range_after(&source, "player[target].hp -", "-")
    );

    let designated = ast.designated_result("E", "result").expect("designated E");
    let designated_expr = designated.expression();
    assert_eq!(
        designated_expr.span().byte_range(),
        byte_range(&source, "player[self].atk + 1")
    );
    assert_eq!(designated_expr.state_refs().len(), 1);
    assert_eq!(designated_expr.state_refs()[0].base(), "player");
    assert_eq!(designated_expr.state_refs()[0].index(), Some("self"));
    assert_eq!(designated_expr.state_refs()[0].field(), Some("atk"));
    assert_eq!(
        designated_expr.state_refs()[0].span().byte_range(),
        byte_range_after(&source, "designated evaluate", "player[self].atk")
    );
    assert_eq!(designated_expr.int_literals().len(), 1);
    assert_eq!(designated_expr.int_literals()[0].value(), 1);
    assert_eq!(
        designated_expr.int_literals()[0].span().byte_range(),
        byte_range_after(&source, "player[self].atk +", "1")
    );
    assert_eq!(designated_expr.binary_ops().len(), 1);
    assert_eq!(
        designated_expr.binary_ops()[0].operator(),
        BoundedBinaryOperator::Add
    );
    assert_eq!(
        designated_expr.binary_ops()[0].span().byte_range(),
        byte_range_after(&source, "designated evaluate", "+")
    );

    let relation = ast.relation("bird_follow").expect("maintained relation");
    assert_eq!(
        relation.primary().transform(),
        &RelationTransform::Translate { x: 3, y: -2 }
    );
    assert_eq!(
        relation.primary().transform_span().byte_range(),
        byte_range(&source, "translate(3, -2)")
    );
    assert_eq!(
        relation.fallback().transform(),
        &RelationTransform::Identity
    );
    assert_eq!(
        relation.fallback().transform_span().byte_range(),
        byte_range(&source, "identity")
    );
}

#[test]
fn parser_retains_provisional_internal_relation_anchor_loci_and_spans() {
    let (path, source) = load_fixture(SYS5_ANCHOR_LOCUS_FIXTURE);
    let ast = parse_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("provisional internal relation anchor-locus source parses");

    let relation = ast.relation("bird_follow").expect("maintained relation");
    assert_eq!(relation.owner_locus(), "ParticipantB");
    assert_eq!(relation.primary().anchor(), "participant_a_shoulder");
    assert_eq!(relation.primary().anchor_locus(), Some("ParticipantA"));
    assert_eq!(
        relation
            .primary()
            .anchor_locus_span()
            .expect("primary anchor locus has a span")
            .byte_range(),
        byte_range_after(&source, "primary participant_a_shoulder at", "ParticipantA")
    );
    assert_eq!(relation.fallback().anchor(), "participant_b_shoulder");
    assert_eq!(relation.fallback().anchor_locus(), Some("ParticipantB"));
    assert_eq!(
        relation
            .fallback()
            .anchor_locus_span()
            .expect("fallback anchor locus has a span")
            .byte_range(),
        byte_range_after(
            &source,
            "fallback participant_b_shoulder at",
            "ParticipantB"
        )
    );
    assert!(
        relation.primary().span().is_child_of(relation.span()),
        "explicit anchor locus remains part of the relation source span"
    );
    assert!(
        relation.fallback().span().is_child_of(relation.span()),
        "explicit fallback locus remains part of the relation source span"
    );
}

#[test]
fn parser_preserves_broad_ordered_expression_tokens_before_m7_finite_rejection() {
    let (path, source) = load_fixture("m7_unsupported_punctuation_expression.mir");
    let ast = parse_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("M6 parser accepts broad expression token collector input");

    let assignment = ast
        .assignment("player[self].hp")
        .expect("punctuation expression assignment");
    let expression = assignment.expression();
    let expression_anchor = byte_range(&source, "player[self].hp =");
    let expected_expression_range = byte_range_nth_after_offset(
        &source,
        expression_anchor.end,
        "(player[self].hp + (player[self].atk, 1))",
        0,
    );
    assert_eq!(expression.span().byte_range(), expected_expression_range);
    assert_eq!(
        token_lexemes(&source, expression),
        vec![
            "(", "player", "[", "self", "]", ".", "hp", "+", "(", "player", "[", "self", "]", ".",
            "atk", ",", "1", ")", ")",
        ]
    );
    assert_eq!(
        expression.tokens()[0].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, "(", 0)
    );
    assert_eq!(
        expression.tokens()[15].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, ",", 0)
    );
    assert_eq!(
        expression.tokens()[18].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, ")", 1)
    );
}

#[test]
fn parser_preserves_canon_m6_expr_token_punctuation_before_m7_rejection() {
    let (path, source) = load_fixture("m7_unsupported_expr_token_set_punctuation.mir");
    let ast = parse_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("M6 parser accepts every Canon M6ExprToken in the broad collector");

    let assignment = ast
        .assignment("player[self].hp")
        .expect("punctuation token-set assignment");
    let expression = assignment.expression();
    let expression_anchor = byte_range(&source, "player[self].hp =");
    let expected_expression_range = byte_range_nth_after_offset(
        &source,
        expression_anchor.end,
        "[ player[self].hp : player[self].atk = . ]",
        0,
    );
    assert_eq!(expression.span().byte_range(), expected_expression_range);
    assert_eq!(
        token_lexemes(&source, expression),
        vec![
            "[", "player", "[", "self", "]", ".", "hp", ":", "player", "[", "self", "]", ".",
            "atk", "=", ".", "]",
        ]
    );
    assert_eq!(
        expression.tokens()[0].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, "[", 0)
    );
    assert_eq!(
        expression.tokens()[7].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, ":", 0)
    );
    assert_eq!(
        expression.tokens()[14].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, "=", 0)
    );
    assert_eq!(
        expression.tokens()[15].span().byte_range(),
        byte_range_nth_after_offset(&source, expected_expression_range.start, ".", 2)
    );
}

#[test]
fn parser_retains_left_associated_ordered_expression_tree_with_spans() {
    let (path, source) = load_fixture("m7_ordered_expression_tree.mir");
    let ast = parse_surface_v0(FixtureSource::new(path, source.clone()))
        .expect("ordered expression fixture parses");

    let assignment = ast
        .assignment("player[self].hp")
        .expect("ordered owner assignment");
    let assignment_tree = assignment.expression().tree();
    assert_eq!(
        assignment_tree.source_lexeme(&source),
        "player[self].hp - player[self].atk + 1"
    );
    assert_eq!(assignment_tree.operator(), Some(BoundedBinaryOperator::Add));
    assert_eq!(
        assignment_tree.left().operator(),
        Some(BoundedBinaryOperator::Subtract)
    );
    assert_eq!(
        assignment_tree.left().left().source_lexeme(&source),
        "player[self].hp"
    );
    assert_eq!(
        assignment_tree.left().right().source_lexeme(&source),
        "player[self].atk"
    );
    assert_eq!(assignment_tree.right().int_literal().unwrap().value(), 1);
    assert_eq!(
        assignment_tree.left().span().byte_range(),
        byte_range(&source, "player[self].hp - player[self].atk")
    );

    let designated = ast.designated_result("E", "result").expect("designated E");
    let designated_tree = designated.expression().tree();
    assert_eq!(
        designated_tree.source_lexeme(&source),
        "player[self].hp - 1 + player[self].atk"
    );
    assert_eq!(designated_tree.operator(), Some(BoundedBinaryOperator::Add));
    assert_eq!(
        designated_tree.left().operator(),
        Some(BoundedBinaryOperator::Subtract)
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
        designated_tree.left().span().byte_range(),
        byte_range(&source, "player[self].hp - 1")
    );
}

#[test]
fn parser_rejects_transport_occurrence_and_envelope_surface() {
    for (fixture, rejected_lexeme, expected_kind) in [
        (
            "reject_send_syntax.mir",
            "send",
            ParseErrorKind::UnsupportedTransportSyntax,
        ),
        (
            "reject_receive_syntax.mir",
            "receive",
            ParseErrorKind::UnsupportedTransportSyntax,
        ),
        (
            "reject_occurrence_syntax.mir",
            "occurrence",
            ParseErrorKind::UnsupportedOccurrenceSyntax,
        ),
        (
            "reject_envelope_syntax.mir",
            "envelope",
            ParseErrorKind::UnsupportedEnvelopeSyntax,
        ),
    ] {
        let (path, source) = load_fixture(fixture);
        let diagnostics =
            parse_surface_v0(FixtureSource::new(path.clone(), source.clone())).unwrap_err();
        let primary = diagnostics.primary();
        assert_eq!(primary.kind(), expected_kind);
        assert_eq!(primary.span().file(), path);
        assert_eq!(
            primary.span().byte_range(),
            byte_range(&source, rejected_lexeme)
        );
        assert_eq!(primary.span().lexeme(&source), rejected_lexeme);
    }
}

#[test]
fn parser_rejects_tokens_outside_m6_expr_token_set_with_typed_spans() {
    for (fixture, rejected_lexeme, expected_kind) in [
        (
            "reject_oversize_int_literal.mir",
            "9223372036854775808",
            ParseErrorKind::IntegerLiteralOutOfRange,
        ),
        (
            "reject_stray_expr_token.mir",
            "*",
            ParseErrorKind::UnexpectedSyntax,
        ),
    ] {
        let (path, source) = load_fixture(fixture);
        let diagnostics =
            parse_surface_v0(FixtureSource::new(path.clone(), source.clone())).unwrap_err();
        let primary = diagnostics.primary();
        assert_eq!(primary.kind(), expected_kind);
        assert_eq!(primary.span().file(), path);
        assert_eq!(
            primary.span().byte_range(),
            byte_range(&source, rejected_lexeme)
        );
        assert_eq!(primary.span().lexeme(&source), rejected_lexeme);
    }
}

#[test]
fn parser_rejects_non_literal_self_role_actor() {
    let (path, source) = load_fixture("role_actor_not_self.mir");
    let diagnostics =
        parse_surface_v0(FixtureSource::new(path.clone(), source.clone())).unwrap_err();
    let primary = diagnostics.primary();
    assert_eq!(
        parse_error_kind_name(primary.kind()),
        "RoleActorMustBeLiteralSelf"
    );
    assert_eq!(primary.span().file(), path);
    assert_eq!(
        primary.span().byte_range(),
        byte_range_after(&source, "Role[", "attacker")
    );
    assert_eq!(primary.span().lexeme(&source), "attacker");
}
