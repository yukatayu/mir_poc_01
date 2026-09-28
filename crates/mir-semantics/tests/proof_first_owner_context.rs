use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

fn checked(visible: &str, rhs: &str) -> CheckedSurfaceV0 {
    let text = format!(
        "module Mirrorea.OwnerContext\n\
         locus A\nlocus S\nprincipal self\ntype Player\n\
         state player[id: Player] at S {{\n\
           hp: Int\nshown: Int\n{visible}\n}}\n\
         Role[self] at A {{\n\
           when refresh(target: Player, amount: Int) fails \
           (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {{\n\
             at S {{ player[target].shown = {rhs} }}\n\
           }}\n}}\nwith auth MembershipAuth\nverify finite_refinement\n"
    );
    check_and_elaborate_surface_v0(FixtureSource::new("tests/inline/owner_context.mir", text))
        .expect("bounded ordinary owner assignment checks")
}

// Actual parser -> M6 -> M7 evidence for the context consumed by a later
// proof-first adapter. A supplied numeric label table is not substituted here.
#[test]
fn checked_owner_context_retains_private_reads_and_declared_visibility_separately() {
    let artifact = checked(
        "visible observer_safe fields (shown)",
        "player[self].hp + amount",
    );
    let schema = &artifact.static_environment().indexed_state_schemas()[0];
    assert_eq!(schema.name(), "player");
    assert_eq!(schema.owner_locus(), "S");
    assert_eq!(schema.index_type(), "Player");
    let private = schema.fields().iter().find(|f| f.name() == "hp").unwrap();
    let visible = schema
        .fields()
        .iter()
        .find(|f| f.name() == "shown")
        .unwrap();
    assert_eq!(private.type_name(), "Int");
    assert_eq!(private.visibility_channel(), None);
    assert_eq!(visible.visibility_channel(), Some("observer_safe"));
    let core = artifact.evaluations()[0].owner_rmw_core().unwrap();
    assert_eq!(core.target().namespace(), "player");
    assert_eq!(core.target().index(), Some("target"));
    assert_eq!(core.target().field(), Some("shown"));
    let read = &core.same_owner_reads()[0];
    assert_eq!(read.namespace(), "player");
    assert_eq!(read.index(), Some("self"));
    assert_eq!(read.field(), Some("hp"));
    assert_eq!(read.owner_locus(), "S");
    assert_ne!(read.source_ref(), core.target().source_ref());
    // M7 retains both facts; accepting this artifact alone is not a proof of
    // secret-to-public noninterference or an observer release authorization.
}

#[test]
fn changing_visibility_changes_checked_identity_without_changing_the_owner_expression() {
    let private = checked("", "player[self].hp + amount");
    let visible = checked(
        "visible observer_safe fields (shown)",
        "player[self].hp + amount",
    );
    assert_ne!(private.program_identity(), visible.program_identity());
    for artifact in [&private, &visible] {
        let core = artifact.evaluations()[0].owner_rmw_core().unwrap();
        assert_eq!(core.target().field(), Some("shown"));
        assert_eq!(core.same_owner_reads()[0].field(), Some("hp"));
    }
    assert!(
        private.static_environment().indexed_state_schemas()[0]
            .fields()
            .iter()
            .all(|field| field.visibility_channel().is_none())
    );
}
