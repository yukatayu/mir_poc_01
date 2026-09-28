//! W4-C candidate controls for explicitly declared index parameters.
//! Principal/literal typing and runtime argument authority are separate cuts.
use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{M7DiagnosticKind, check_and_elaborate_surface_v0};

fn source(target_type: &str, other_type: &str) -> String {
    format!(
        "module Mirrorea.TypedOwnerIndex\n\
         locus A\nlocus S\nprincipal self\ntype Player\ntype Team\n\
         state player[id: Player] at S {{ hp: Int }}\n\
         Role[self] at A {{\n\
           when update(target: {target_type}, other: {other_type}) fails \
           (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {{\n\
             at S {{ player[target].hp = player[other].hp + 1 }}\n\
           }}\n}}\nwith auth MembershipAuth\nverify finite_refinement\n"
    )
}

#[test]
fn proof_first_owner_index_matching_parameters_check() {
    for index_type in ["Player", "Team", "Int"] {
        let text = source(index_type, index_type).replace(
            "state player[id: Player]",
            &format!("state player[id: {index_type}]"),
        );
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(
            "tests/inline/typed_owner_index.mir",
            text,
        ))
        .expect("matching explicit index parameters remain meaningful ordinary source");
        let core = checked.evaluations()[0].owner_rmw_core().unwrap();
        assert_eq!(core.target().index(), Some("target"));
        assert_eq!(core.same_owner_reads()[0].index(), Some("other"));
    }
}

#[test]
fn proof_first_owner_index_wrong_target_parameter_refuses() {
    let text = source("Team", "Player");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text.clone(),
    ))
    .expect_err("a declared Team parameter cannot index a Player state");
    assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
    assert_eq!(error.primary().span().lexeme(&text), "player[target].hp");
    assert!(!error.has_executable_core());
}

#[test]
fn proof_first_owner_index_wrong_rhs_parameter_refuses() {
    let text = source("Player", "Team");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text.clone(),
    ))
    .expect_err("every RHS state access retains its declared index type");
    assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
    assert_eq!(error.primary().span().lexeme(&text), "player[other].hp");
    assert!(!error.has_executable_core());
}

#[test]
fn proof_first_owner_index_duplicate_parameter_refuses() {
    let text = source("Player", "Player")
        .replace("other: Player", "target: Player")
        .replace("player[other].hp + 1", "1");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text.clone(),
    ))
    .expect_err("even identical duplicate parameter declarations cannot be selected by order");
    assert_eq!(
        error.primary().kind(),
        M7DiagnosticKind::DuplicateDeclaration
    );
    let second = text.match_indices("target: Player").nth(1).unwrap().0;
    assert_eq!(
        error.primary().span().byte_range(),
        second..second + "target: Player".len()
    );
    assert!(!error.has_executable_core());
}

#[test]
fn proof_first_owner_index_later_rhs_parameter_refuses() {
    let text = source("Player", "Player")
        .replace("other: Player)", "other: Player, later: Team)")
        .replace(
            "player[other].hp + 1",
            "player[other].hp + 1 - player[later].hp",
        );
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text.clone(),
    ))
    .expect_err("a valid first read cannot hide a later wrongly typed index");
    assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
    assert_eq!(error.primary().span().lexeme(&text), "player[later].hp");
    assert!(!error.has_executable_core());
}

#[test]
fn proof_first_owner_index_nonparameter_stays_outside_new_refinement() {
    let text = source("Player", "Player")
        .replace("player[target].hp", "player[self].hp")
        .replace("player[other].hp + 1", "player[self].hp + 1");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text,
    ))
    .expect("new explicit-parameter refinement preserves the existing principal-index path");
    assert_eq!(
        checked.evaluations()[0]
            .owner_rmw_core()
            .unwrap()
            .target()
            .index(),
        Some("self")
    );
    // This is regression coverage, not a new type/identity/authority certificate.
}

#[test]
fn proof_first_owner_index_conflicting_duplicates_refuse_in_both_orders() {
    for (signature, second) in [
        (
            "target: Player, target: Team, other: Player",
            "target: Team",
        ),
        (
            "target: Team, target: Player, other: Player",
            "target: Player",
        ),
        ("target: Player, other: Player, other: Team", "other: Team"),
        (
            "target: Player, other: Team, other: Player",
            "other: Player",
        ),
    ] {
        let text = source("Player", "Player").replace("target: Player, other: Player", signature);
        let error = check_and_elaborate_surface_v0(FixtureSource::new(
            "tests/inline/typed_owner_index.mir",
            text.clone(),
        ))
        .expect_err("duplicate index parameters cannot select a compatible first occurrence");
        assert_eq!(
            error.primary().kind(),
            M7DiagnosticKind::DuplicateDeclaration
        );
        let start = text.find(second).unwrap();
        assert_eq!(
            error.primary().span().byte_range(),
            start..start + second.len()
        );
    }
}

#[test]
fn proof_first_owner_index_matching_name_rule_also_checks_self_parameter() {
    for (ty, accepted) in [("Player", true), ("Team", false)] {
        let text = source(ty, "Player").replace("target", "self");
        let result = check_and_elaborate_surface_v0(FixtureSource::new(
            "tests/inline/typed_owner_index.mir",
            text.clone(),
        ));
        if accepted {
            assert!(
                result.is_ok(),
                "matching declaration preserves this finite source path"
            );
        } else {
            let error = result
                .expect_err("principal spelling is not an exemption for a matching parameter");
            assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
            assert_eq!(error.primary().span().lexeme(&text), "player[self].hp");
        }
    }
}

#[test]
fn proof_first_owner_index_uses_each_namespace_and_enclosing_handler() {
    let text = source("Player", "Team")
        .replace(
            "Role[self]",
            "state squad[id: Team] at S { hp: Int }\nRole[self]",
        )
        .replace("player[other].hp", "squad[other].hp");
    assert!(
        check_and_elaborate_surface_v0(FixtureSource::new(
            "tests/inline/typed_owner_index.mir",
            text.clone(),
        ))
        .is_ok()
    );
    let wrong = text.replace("other: Team", "other: Player");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        wrong.clone(),
    ))
    .expect_err("RHS index type is taken from its own namespace");
    assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
    assert_eq!(error.primary().span().lexeme(&wrong), "squad[other].hp");

    let second = "when update_team(target: Team, other: Team) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {\nat S { squad[target].hp = squad[other].hp + 1 }\n}\n";
    let two = text.replace("}\nwith auth", &format!("{second}}}\nwith auth"));
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        two,
    ))
    .expect("the same parameter spelling can have different types in different handlers");
    assert_eq!(checked.evaluations().len(), 2);
}

#[test]
fn proof_first_owner_index_keeps_scalar_and_unused_duplicate_boundary_explicit() {
    for (signature, rhs) in [
        (
            "target: Player, other: Player, unused: Player, unused: Team",
            "player[other].hp + 1",
        ),
        (
            "target: Player, other: Player, amount: Int, amount: Player",
            "amount",
        ),
    ] {
        let text = source("Player", "Player")
            .replace("target: Player, other: Player", signature)
            .replace("player[other].hp + 1", rhs);
        assert!(
            check_and_elaborate_surface_v0(FixtureSource::new(
                "tests/inline/typed_owner_index.mir",
                text,
            ))
            .is_ok(),
            "index-only refinement is not a global parameter-uniqueness check"
        );
    }
    // Retains the existing finite scalar lookup boundary; it is not a claim
    // that such a signature is suitable for the later typed call-admission cut.
}

#[test]
fn proof_first_owner_index_diagnostic_precedence_is_explicit() {
    let text = source("Team", "Player").replace("player[target].hp", "player[target].missing");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        text.clone(),
    ))
    .unwrap_err();
    assert_eq!(error.primary().kind(), M7DiagnosticKind::TypeMismatch);
    assert_eq!(
        error.primary().span().lexeme(&text),
        "player[target].missing"
    );

    let missing_failure = source("Team", "Player").replace("MissingWitness, ", "");
    let error = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/typed_owner_index.mir",
        missing_failure,
    ))
    .unwrap_err();
    assert_eq!(
        error.primary().kind(),
        M7DiagnosticKind::GeneratedFailureNotDeclared
    );
}
