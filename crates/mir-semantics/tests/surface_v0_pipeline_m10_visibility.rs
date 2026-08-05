use std::ops::Range;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    evaluation_materialization::{AuthorityOrigin, EvaluationSite, Locus, Principal},
    shared_model::SourceRef,
    surface_v0_pipeline::{EffectKind, M7DiagnosticKind, check_and_elaborate_surface_v0},
};

const VISIBLE_ROLL_SOURCE: &str = r#"
module M10.Visibility.Roll

locus World
locus BrowserClient
principal self
type Participant

state player[p: Participant] at World {
  position: Int
  hp: Int
  visible observer_safe fields (position)
}

Role[self] at BrowserClient {
  when roll(draw: Int) fails (MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied) {
    at World {
      player[self].position = player[self].position + draw
    }
  }
}
"#;

const PRIVATE_HP_SOURCE: &str = r#"
module M10.Visibility.PrivateHp

locus World
locus BrowserClient
principal self
type Participant

state player[p: Participant] at World {
  position: Int
  hp: Int
  visible observer_safe fields (position)
}

Role[self] at BrowserClient {
  when damage(draw: Int) fails (MissingCapability, MissingWitness, RouteUnavailable, StaleMembership) {
    at World {
      player[self].hp = player[self].hp - draw
    }
  }
}
"#;

const MISSING_VISIBILITY_FAILURE_SOURCE: &str = r#"
module M10.Visibility.MissingVisibilityFailure

locus World
locus BrowserClient
principal self
type Participant

state player[p: Participant] at World {
  position: Int
  hp: Int
  visible observer_safe fields (position)
}

Role[self] at BrowserClient {
  when roll(draw: Int) fails (MissingCapability, MissingWitness, RouteUnavailable, StaleMembership) {
    at World {
      player[self].position = player[self].position + draw
    }
  }
}
"#;

const MISSING_CAPABILITY_FAILURE_SOURCE: &str = r#"
module M10.Visibility.MissingCapabilityFailure

locus World
locus BrowserClient
principal self
principal target
type Player

state player[p: Player] at World {
  hp: Int
  atk: Int
}

Role[self] at BrowserClient {
  when attack(target: Player) fails (StaleMembership, MissingWitness, RouteUnavailable) {
    at World {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}
"#;

const MISSING_ROUTE_UNAVAILABLE_FAILURE_SOURCE: &str = r#"
module M10.Visibility.MissingRouteUnavailableFailure

locus World
locus BrowserClient
principal self
type Player

state player[p: Player] at World {
  hp: Int
}

Role[self] at BrowserClient {
  when move(delta: Int) fails (MissingCapability, MissingWitness, StaleMembership) {
    at World {
      player[self].hp = player[self].hp + delta
    }
  }
}
"#;

const ACTION_LOCUS_STATE_OWNER_MISMATCH_SOURCE: &str = r#"
module M10.Visibility.ActionLocusStateOwnerMismatch

locus World
locus OtherWorld
locus BrowserClient
principal self
type Participant

state player[p: Participant] at World {
  position: Int
  visible observer_safe fields (position)
}

Role[self] at BrowserClient {
  when bad_roll(draw: Int) fails (MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied) {
    at OtherWorld {
      player[self].position = player[self].position + draw
    }
  }
}
"#;

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("source contains {needle:?}"));
    start..start + needle.len()
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut column = 1u32;
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
    SourceRef::new(path, start_line, start_column, end_line, end_column)
}

#[test]
fn visible_owner_rmw_generates_source_bound_observer_publish_effect() {
    let source_path = "tmp/m10/visible_roll.mir";
    let checked =
        check_and_elaborate_surface_v0(FixtureSource::new(source_path, VISIBLE_ROLL_SOURCE))
            .expect("visible owner RMW checks through M7");

    let roll = checked.evaluation("roll").expect("roll evaluation");
    let owner_core = roll.owner_rmw_core().expect("owner RMW checked core");
    assert_eq!(owner_core.owner_locus(), "World");
    assert_eq!(owner_core.target().namespace(), "player");
    assert_eq!(owner_core.target().field(), Some("position"));
    assert_eq!(roll.actor_authority_origin(), "self");
    assert_eq!(roll.authority_origin_locus(), "BrowserClient");
    assert_eq!(roll.owner_evaluation_locus(), "World");
    assert_eq!(owner_core.authority_origin_locus(), "BrowserClient");
    assert_eq!(
        roll.evaluation_axes().evaluation_site(),
        &EvaluationSite::Owner(Locus::new("World"))
    );
    assert_eq!(
        roll.evaluation_axes().authority_origin(),
        &AuthorityOrigin::Caller(Principal::new("self"))
    );
    assert!(
        roll.effect_row()
            .contains_request_to_owner("BrowserClient", "World"),
        "source-to-Core OwnerRequest effects retain caller_locus BrowserClient and owner World separately"
    );
    assert!(
        owner_core
            .same_owner_reads()
            .iter()
            .all(|read| read.owner_locus() == "World"),
        "RHS owner-private reads execute at World"
    );
    assert!(
        roll.effect_row()
            .entries()
            .iter()
            .all(|entry| entry.kind() != EffectKind::ActorReadReply),
        "RHS owner-private reads must not be returned to the actor"
    );

    let assignment = "player[self].position = player[self].position + draw";
    let publish = roll
        .effect_row()
        .entries()
        .iter()
        .find(|entry| entry.kind() == EffectKind::ObserverPublish)
        .expect("visible field write generates observer publish");
    assert_eq!(
        publish.source_ref(),
        &expected_source_ref(source_path, VISIBLE_ROLL_SOURCE, assignment)
    );
    assert_eq!(publish.source_lexeme(VISIBLE_ROLL_SOURCE), assignment);
    assert_eq!(publish.redaction_label(), "observer_safe");
    assert_eq!(publish.field(), Some("position"));
    assert_eq!(publish.failure(), Some("VisibilityDenied"));
}

#[test]
fn private_owner_rmw_does_not_generate_observer_publish_effect() {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tmp/m10/private_hp.mir",
        PRIVATE_HP_SOURCE,
    ))
    .expect("private owner RMW still checks as an owner-local write");

    let damage = checked.evaluation("damage").expect("damage evaluation");
    assert!(
        damage
            .effect_row()
            .entries()
            .iter()
            .all(|entry| entry.kind() != EffectKind::ObserverPublish),
        "unlisted fields remain private and must not publish observer rows"
    );
    assert!(
        damage
            .effect_row()
            .entries()
            .iter()
            .any(|entry| entry.kind() == EffectKind::OwnerWrite),
        "private write still materializes as owner state mutation"
    );
}

#[test]
fn missing_visibility_denied_failure_rejects_at_assignment_span_with_erow002() {
    let source_path = "tmp/m10/missing_visibility_failure.mir";
    let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(
        source_path,
        MISSING_VISIBILITY_FAILURE_SOURCE,
    ))
    .expect_err("generated VisibilityDenied must be declared");

    let primary = diagnostics.primary();
    let assignment = "player[self].position = player[self].position + draw";
    assert_eq!(
        primary.kind(),
        M7DiagnosticKind::GeneratedFailureNotDeclared
    );
    assert_eq!(primary.canonical_code(), "E-ROW-002");
    let missing = primary
        .generated_failure_reason()
        .expect("E-ROW-002 exposes the missing generated failure");
    assert_eq!(missing.missing_failure(), "VisibilityDenied");
    assert_eq!(
        primary.span().lexeme(MISSING_VISIBILITY_FAILURE_SOURCE),
        assignment
    );
    assert_eq!(
        primary.source_ref(),
        &expected_source_ref(source_path, MISSING_VISIBILITY_FAILURE_SOURCE, assignment)
    );
    assert!(!diagnostics.has_executable_core());
}

#[test]
fn generated_failure_codes_distinguish_authority_route_from_visibility() {
    for (source_path, source, expected_code, expected_missing, expected_lexeme) in [
        (
            "tmp/m10/missing_capability_failure.mir",
            MISSING_CAPABILITY_FAILURE_SOURCE,
            "E-ROW-001",
            "MissingCapability",
            "fails (StaleMembership, MissingWitness, RouteUnavailable)",
        ),
        (
            "tmp/m10/missing_route_unavailable_failure.mir",
            MISSING_ROUTE_UNAVAILABLE_FAILURE_SOURCE,
            "E-ROW-001",
            "RouteUnavailable",
            "fails (MissingCapability, MissingWitness, StaleMembership)",
        ),
        (
            "tmp/m10/missing_visibility_failure.mir",
            MISSING_VISIBILITY_FAILURE_SOURCE,
            "E-ROW-002",
            "VisibilityDenied",
            "player[self].position = player[self].position + draw",
        ),
    ] {
        let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(source_path, source))
            .expect_err("underdeclared generated failure rejects with typed reason");
        let primary = diagnostics.primary();
        assert_eq!(
            primary.kind(),
            M7DiagnosticKind::GeneratedFailureNotDeclared
        );
        assert_eq!(primary.canonical_code(), expected_code);
        let missing = primary
            .generated_failure_reason()
            .expect("generated failure diagnostic exposes the missing failure");
        assert_eq!(missing.missing_failure(), expected_missing);
        assert_eq!(primary.span().lexeme(source), expected_lexeme);
    }
}

#[test]
fn action_locus_must_match_state_owner_but_role_locus_may_differ_from_owner_locus() {
    let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(
        "tmp/m10/action_locus_state_owner_mismatch.mir",
        ACTION_LOCUS_STATE_OWNER_MISMATCH_SOURCE,
    ))
    .expect_err("nested action locus that is not the state owner rejects");

    let primary = diagnostics.primary();
    assert_eq!(
        primary.kind(),
        M7DiagnosticKind::CrossOwnerWriteTargetOutsideActionLocus
    );
    assert_ne!(primary.kind(), M7DiagnosticKind::OwnerActionLocusMismatch);
    assert_eq!(
        primary
            .span()
            .lexeme(ACTION_LOCUS_STATE_OWNER_MISMATCH_SOURCE),
        "player[self].position"
    );
}
