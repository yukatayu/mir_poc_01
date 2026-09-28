use mir_ast::surface_v0::{FixtureSource, parse_surface_v0};
use mir_semantics::{
    shared_model::SourceRef, surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::check_and_elaborate_surface_v0,
};

// This checks actual parser/classifier/checker provenance. It does not assert
// that the runtime accepts or executes every multi-assignment source below.
#[test]
fn each_assignment_keeps_its_actual_source_origin() {
    for body in [
        "at S { a[self].hp = 34 }\nat T { b[self].hp = 35 }",
        "at T { b[self].hp = 35 }\nat S { a[self].hp = 34 }",
        "at S { a[self].hp = 34 }\nat S { a[self].hp = 34 }",
    ] {
        let text = format!(
            "module Mirrorea.SourceOrigin\nlocus A\nlocus S\nlocus T\nprincipal self\ntype Player\nstate a[id: Player] at S {{ hp: Int }}\nstate b[id: Player] at T {{ hp: Int }}\nRole[self] at A {{\nwhen refresh() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {{\n{body}\n}}\n}}\nwith auth MembershipAuth\nverify finite_refinement\n"
        );
        let source = FixtureSource::new("tests/inline/source_origin.mir", &text);
        let parsed = parse_surface_v0(source.clone()).expect("source parses");
        let checked = check_and_elaborate_surface_v0(source).expect("source checks");
        assert_eq!(parsed.assignments().len(), 2);
        assert_eq!(checked.evaluations().len(), 2);
        assert_ne!(
            parsed.assignments()[0].span(),
            parsed.assignments()[1].span()
        );
        for (assignment, evaluation) in parsed.assignments().iter().zip(checked.evaluations()) {
            let site = assignment.span().source_ref_data();
            let (sl, sc, el, ec) = site.line_columns();
            let expected = SourceRef::new(site.path(), sl, sc, el, ec);
            assert_eq!(evaluation.source_ref(), &expected);
            assert_eq!(
                evaluation.source_lexeme(&text),
                assignment.span().lexeme(&text)
            );
            let core = evaluation.owner_rmw_core().unwrap();
            assert_eq!(core.owner_locus(), assignment.owner_locus());
            assert_eq!(
                core.expression().source_lexeme(&text),
                assignment.expression().span().lexeme(&text)
            );
            for effect in evaluation.effect_row().entries() {
                assert_eq!(effect.source_ref(), &expected);
            }
            for obligation in evaluation.generated_obligations().entries() {
                assert_eq!(obligation.source_ref(), &expected);
            }
            for kind in [
                SourceToCoreKind::OwnerRmw,
                SourceToCoreKind::OwnerLocalRead,
                SourceToCoreKind::OwnerLocalWrite,
            ] {
                assert_eq!(
                    checked
                        .source_map()
                        .entries()
                        .iter()
                        .filter(|entry| entry.kind() == kind && entry.source_ref() == &expected)
                        .count(),
                    1
                );
            }
        }
    }
}
