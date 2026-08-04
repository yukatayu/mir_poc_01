use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::{
    DeferredFormKind, FixtureSource, ParseErrorKind, SyntaxKind, parse_surface_v0,
};

const FIXTURE_DIR: &str = "tests/fixtures/surface-v0";

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

fn parse_error_kind_name(kind: ParseErrorKind) -> String {
    format!("{kind:?}")
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
