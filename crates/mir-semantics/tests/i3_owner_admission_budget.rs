use mir_ast::surface_v0::{FixtureSource, ParseErrorKind, parse_surface_v0};
use mir_semantics::{
    surface_v0_classification::{
        ClassificationKind, SurfaceV0ClassificationOptions, SurfaceV0DiagnosticKind,
        classify_surface_v0,
    },
    surface_v0_pipeline::{
        M7DiagnosticKind, OWNER_ADMISSION_DEADLINE_EXPIRED_FAILURE, OwnerRmwCheckedCore,
        check_and_elaborate_surface_v0, private_snapshot::SnapshotOwnerRmwCheckedCore,
    },
};
use serde_json::{Value, json};

const BUDGETED_SOURCE_PATH: &str = "tests/i3_owner_admission_budget/budgeted_owner_request.mir";
const PADDED_CONTROL_SOURCE_PATH: &str =
    "tests/i3_owner_admission_budget/padded_deadline_failure_control.mir";
const UNDERDECLARED_SOURCE_PATH: &str =
    "tests/i3_owner_admission_budget/underdeclared_budget_failure.mir";
const BOUNDS_SOURCE_PATH: &str = "tests/i3_owner_admission_budget/budget_bounds.mir";
const SHAPE_SOURCE_PATH: &str = "tests/i3_owner_admission_budget/budget_shape.mir";
const IDENTITY_SOURCE_PATH: &str = "tests/i3_owner_admission_budget/budget_identity.mir";
const OTHER_PROVENANCE_SOURCE_PATH: &str =
    "tests/i3_owner_admission_budget/other_budget_identity_provenance.mir";

const ORDINARY_OWNER_FAILURES: &str =
    "StaleMembership, MissingCapability, MissingWitness, RouteUnavailable";
const OWNER_FAILURES_WITH_DEADLINE: &str =
    "StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired";
const OWNER_ASSIGNMENT: &str = "player[target].hp = player[target].hp - player[self].atk";

fn owner_request_source(failure_row: &str, admission_clause: &str) -> String {
    owner_request_source_with_body(failure_row, admission_clause, "S", OWNER_ASSIGNMENT)
}

fn owner_request_source_with_body(
    failure_row: &str,
    admission_clause: &str,
    owner_locus: &str,
    assignment_body: &str,
) -> String {
    format!(
        "module Combat.OwnerAdmissionBudget

locus {owner_locus}
principal self
principal target
type Player

state player[id: Player] at {owner_locus} {{
  hp: Int
  atk: Int
}}

Role[self] at {owner_locus} {{
  when attack(target: Player) fails ({failure_row}){admission_clause} {{
    at {owner_locus} {{
      {assignment_body}
    }}
  }}
}}

with auth MembershipAuth

verify finite_refinement
"
    )
}

fn budgeted_checked_owner_core() -> OwnerRmwCheckedCore {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        BUDGETED_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 1"),
    ))
    .expect("budgeted source checks before private snapshotting");
    checked
        .evaluation("attack")
        .expect("budgeted checked source retains owner evaluation")
        .owner_rmw_core()
        .expect("budgeted checked source retains owner Core")
        .clone()
}

fn ordinary_checked_owner_core() -> OwnerRmwCheckedCore {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        PADDED_CONTROL_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, ""),
    ))
    .expect("ordinary padded source checks before private snapshotting");
    checked
        .evaluation("attack")
        .expect("ordinary checked source retains owner evaluation")
        .owner_rmw_core()
        .expect("ordinary checked source retains owner Core")
        .clone()
}

fn serialized_budgeted_owner_snapshot() -> (OwnerRmwCheckedCore, Value) {
    let core = budgeted_checked_owner_core();
    let snapshot = SnapshotOwnerRmwCheckedCore::from_checked(&core);
    let json = serde_json::to_value(snapshot).expect("private checked owner snapshot serializes");
    (core, json)
}

fn owner_budget_json(snapshot: &mut Value) -> &mut serde_json::Map<String, Value> {
    snapshot
        .as_object_mut()
        .expect("owner snapshot serializes as an object")
        .get_mut("owner_admission_budget")
        .expect("budgeted snapshot retains its typed condition")
        .as_object_mut()
        .expect("typed condition serializes as an object")
}

fn assert_tampered_snapshot_is_rejected(snapshot: Value, falsifier: &str) {
    if let Ok(snapshot) = serde_json::from_value::<SnapshotOwnerRmwCheckedCore>(snapshot) {
        assert!(
            snapshot.into_checked().is_err(),
            "{falsifier} must fail closed before restoring a checked owner Core"
        );
    }
}

#[test]
fn explicit_owner_budget_with_declared_deadline_failure_reaches_m6_and_m7() {
    let source = owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 1");

    let ast = parse_surface_v0(FixtureSource::new(BUDGETED_SOURCE_PATH, source.clone()))
        .expect("the source-declared owner budget is accepted by the AST parser");
    let ast_condition = ast
        .when("attack")
        .expect("budgeted owner request retains its source handler")
        .owner_admission_budget()
        .expect("within owner_ticks is retained by the source AST");
    assert_eq!(ast_condition.ticks(), 1);
    assert_eq!(ast_condition.span().lexeme(&source), "within owner_ticks 1");

    let m6 = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("the budgeted owner request has an ordinary lowerable M6 owner template");
    assert_eq!(m6.kind(), ClassificationKind::OwnerRmw);
    let m6_template = m6
        .core_template("attack")
        .expect("M6 retains a budgeted owner CoreTemplate");
    let m6_condition = m6_template
        .owner_admission_budget()
        .expect("M6 retains the AST admission condition as typed metadata");
    assert_eq!(m6_condition.budget_ticks(), 1);
    assert_eq!(m6_condition.owner_locus().as_str(), "S");
    assert_eq!(m6_condition.source_span(), ast_condition.span());
    assert_eq!(
        m6_condition.source_ref(),
        m6.source_ref_for_span(ast_condition.span())
            .expect("M6 condition span retains its exact source reference")
    );
    assert!(
        m6_template.to_m5_core().is_none(),
        "an annotated template cannot claim an unguarded complete M5 export"
    );
    let inspection = m6_template
        .m5_owner_rmw_subcomponent()
        .expect("inspection retains non-executable owner-RMW summary metadata");
    assert_eq!(
        inspection.source_ref(),
        m6.source_ref_for_span(m6_template.source_span())
            .expect("M6 owner template retains its source reference")
    );
    assert_eq!(inspection.operation_count(), 1);

    let checked = check_and_elaborate_surface_v0(FixtureSource::new(BUDGETED_SOURCE_PATH, source))
        .expect("the budgeted owner request reaches checked M7 elaboration");
    let attack = checked
        .evaluation("attack")
        .expect("budgeted source retains its checked owner evaluation");
    assert_eq!(
        attack.declared_failure_row().names(),
        vec![
            "StaleMembership",
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
            "DeadlineExpired",
        ]
    );
    assert_eq!(
        attack.generated_failure_row().names(),
        vec![
            "StaleMembership",
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
            "DeadlineExpired",
        ],
        "M7 generates DeadlineExpired only from the retained admission condition"
    );
    let checked_m6_condition = checked
        .consumed_m6_classification()
        .core_template("attack")
        .expect("M7 retains the accepted M6 owner template")
        .owner_admission_budget()
        .expect("M7 retains M6 owner-admission metadata without reconstructing the AST");
    let m7_condition = attack
        .owner_admission_budget()
        .expect("checked owner evaluation retains the budget condition");
    let m7_core_condition = attack
        .owner_rmw_core()
        .expect("checked owner evaluation retains its owner Core")
        .owner_admission_budget()
        .expect("checked owner Core retains the budget condition");
    assert_eq!(checked_m6_condition, m6_condition);
    assert_eq!(m7_condition, m6_condition);
    assert_eq!(m7_core_condition, m6_condition);
    assert_eq!(m7_condition.clock_domain(), m6_condition.clock_domain());
}

#[test]
fn deadline_failure_name_padding_without_budget_clause_remains_an_ordinary_owner_source() {
    let source = owner_request_source(OWNER_FAILURES_WITH_DEADLINE, "");

    let ast = parse_surface_v0(FixtureSource::new(
        PADDED_CONTROL_SOURCE_PATH,
        source.clone(),
    ))
    .expect("failure-name padding remains ordinary accepted source syntax");
    assert!(
        ast.when("attack")
            .expect("ordinary owner source retains its handler")
            .owner_admission_budget()
            .is_none(),
        "a DeadlineExpired name alone is not an admission condition"
    );
    let m6 = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("failure-name padding remains lowerable ordinary M6 source");
    let m6_template = m6
        .core_template("attack")
        .expect("ordinary source retains its M6 owner template");
    assert!(m6_template.owner_admission_budget().is_none());
    assert!(
        m6_template.to_m5_core().is_some(),
        "old unannotated templates retain their existing complete M5 export"
    );

    let checked =
        check_and_elaborate_surface_v0(FixtureSource::new(PADDED_CONTROL_SOURCE_PATH, source))
            .expect("a declared DeadlineExpired name alone remains an accepted ordinary source");
    let attack = checked
        .evaluation("attack")
        .expect("ordinary source retains its owner evaluation");
    assert_eq!(
        attack.generated_failure_row().names(),
        vec![
            "StaleMembership",
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
        ],
        "failure-name padding must not add a generated owner-admission failure"
    );
    assert!(attack.owner_admission_budget().is_none());
    assert!(
        attack
            .owner_rmw_core()
            .expect("ordinary checked owner evaluation retains Core")
            .owner_admission_budget()
            .is_none()
    );
}

#[test]
fn budget_clause_without_declared_deadline_failure_is_a_typed_failure_row_error() {
    let source = owner_request_source(ORDINARY_OWNER_FAILURES, " within owner_ticks 1");

    let diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(UNDERDECLARED_SOURCE_PATH, source))
            .expect_err("an opted-in owner budget must declare DeadlineExpired");
    let primary = diagnostics.primary();
    assert_eq!(
        primary.kind(),
        M7DiagnosticKind::GeneratedFailureNotDeclared
    );
    assert_eq!(primary.canonical_code(), "E-ROW-001");
    assert_eq!(
        primary
            .generated_failure_reason()
            .expect("underdeclared generated failure retains its missing member")
            .missing_failure(),
        OWNER_ADMISSION_DEADLINE_EXPIRED_FAILURE
    );
}

#[test]
fn owner_budget_literals_outside_the_finite_range_reject_at_the_literal() {
    for literal in ["0", "65536", "18446744073709551616"] {
        let source = owner_request_source(
            OWNER_FAILURES_WITH_DEADLINE,
            &format!(" within owner_ticks {literal}"),
        );
        let diagnostics = parse_surface_v0(FixtureSource::new(BOUNDS_SOURCE_PATH, source.clone()))
            .expect_err("outside-range owner budget does not parse as an ordinary integer");
        let primary = diagnostics.primary();
        assert_eq!(
            primary.kind(),
            ParseErrorKind::OwnerAdmissionBudgetOutOfRange
        );
        assert_eq!(primary.span().lexeme(&source), literal);
    }
}

#[test]
fn finite_owner_budget_upper_bound_reaches_checked_m7() {
    let source = owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 65535");
    let ast = parse_surface_v0(FixtureSource::new(BOUNDS_SOURCE_PATH, source.clone()))
        .expect("the finite owner-budget upper bound is accepted by the AST parser");
    assert_eq!(
        ast.when("attack")
            .expect("upper-bound source retains its handler")
            .owner_admission_budget()
            .expect("upper-bound source retains its admission clause")
            .ticks(),
        65_535
    );
    let m6 = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("the finite upper bound remains lowerable M6 source");
    assert_eq!(
        m6.core_template("attack")
            .expect("M6 retains the upper-bound owner template")
            .owner_admission_budget()
            .expect("M6 retains the finite upper-bound condition")
            .budget_ticks(),
        65_535
    );
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(BOUNDS_SOURCE_PATH, source))
        .expect("the finite upper bound reaches checked M7");
    assert_eq!(
        checked
            .evaluation("attack")
            .expect("M7 retains the upper-bound owner evaluation")
            .owner_admission_budget()
            .expect("M7 retains the finite upper-bound condition")
            .budget_ticks(),
        65_535
    );
}

#[test]
fn owner_budget_requires_exactly_one_lowerable_owner_assignment_at_the_clause() {
    let source = owner_request_source_with_body(
        OWNER_FAILURES_WITH_DEADLINE,
        " within owner_ticks 1",
        "S",
        "player[target].hp = player[target].hp - player[self].atk\n      player[target].atk = player[target].atk - player[self].atk",
    );
    let ast = parse_surface_v0(FixtureSource::new(SHAPE_SOURCE_PATH, source.clone()))
        .expect("the source shape parses before M6 decides lowerability");
    let diagnostics = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect_err(
            "budgeted handler with two owner assignments must not become a multi-owner deadline",
        );
    let primary = diagnostics.primary();
    assert_eq!(
        primary.kind(),
        SurfaceV0DiagnosticKind::OwnerAdmissionBudgetRequiresExactlyOneLowerableOwnerAssignment
    );
    assert_eq!(primary.span().lexeme(&source), "within owner_ticks 1");
}

#[test]
fn owner_budget_does_not_mask_an_existing_unsupported_owner_shape() {
    let source = "module Combat.OwnerAdmissionBudgetUnsupportedShape

locus S
locus T
principal self
principal target
type Player

state player[id: Player] at S {
  hp: Int
  atk: Int
}

Role[self] at S {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {
    at T {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}

with auth MembershipAuth

verify finite_refinement
";
    let ast = parse_surface_v0(FixtureSource::new(SHAPE_SOURCE_PATH, source))
        .expect("unsupported owner shape remains syntactically valid before M6");
    let diagnostics = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect_err("the budget annotation must not hide the existing cross-owner shape error");
    let primary = diagnostics.primary();
    assert_eq!(
        primary.kind(),
        SurfaceV0DiagnosticKind::CrossOwnerWriteTargetOutsideActionLocus
    );
    assert_eq!(primary.span().lexeme(source), "player[target].hp");
}

#[test]
fn checked_budget_identity_does_not_alias_budget_presence_owner_or_source_provenance() {
    let one_tick = check_and_elaborate_surface_v0(FixtureSource::new(
        IDENTITY_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 1"),
    ))
    .expect("one-tick budgeted source checks");
    let two_ticks = check_and_elaborate_surface_v0(FixtureSource::new(
        IDENTITY_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 2"),
    ))
    .expect("two-tick budgeted source checks");
    let unannotated = check_and_elaborate_surface_v0(FixtureSource::new(
        IDENTITY_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, ""),
    ))
    .expect("unannotated padded source checks");
    let other_owner = check_and_elaborate_surface_v0(FixtureSource::new(
        IDENTITY_SOURCE_PATH,
        owner_request_source_with_body(
            OWNER_FAILURES_WITH_DEADLINE,
            " within owner_ticks 1",
            "T",
            OWNER_ASSIGNMENT,
        ),
    ))
    .expect("same budget at a different owner checks");
    let other_provenance = check_and_elaborate_surface_v0(FixtureSource::new(
        OTHER_PROVENANCE_SOURCE_PATH,
        owner_request_source(OWNER_FAILURES_WITH_DEADLINE, " within owner_ticks 1"),
    ))
    .expect("same budget with distinct source provenance checks");

    assert_ne!(one_tick.program_identity(), two_ticks.program_identity());
    assert_ne!(one_tick.program_identity(), unannotated.program_identity());
    assert_ne!(one_tick.program_identity(), other_owner.program_identity());
    assert_ne!(
        one_tick.program_identity(),
        other_provenance.program_identity()
    );
    assert!(
        one_tick
            .program_identity()
            .structural_entries()
            .iter()
            .any(|entry| {
                entry.contains(":owner-admission-budget:")
                    && entry.contains("source_span=")
                    && entry.contains("source_ref=")
            }),
        "an opted-in identity must retain an explicit source-span fragment alongside its source reference"
    );
    assert!(
        unannotated
            .program_identity()
            .structural_entries()
            .iter()
            .all(|entry| !entry.contains(":owner-admission-budget:")),
        "legacy identity must not acquire an implicit absent-condition fragment"
    );
}

#[test]
fn private_owner_snapshot_round_trip_preserves_the_exact_typed_budget_condition() {
    let (core, serialized) = serialized_budgeted_owner_snapshot();
    assert!(
        serialized
            .as_object()
            .expect("owner snapshot serializes as an object")
            .contains_key("owner_admission_budget"),
        "budgeted checked Core must retain its condition through the private snapshot"
    );

    let restored: SnapshotOwnerRmwCheckedCore =
        serde_json::from_value(serialized).expect("private snapshot deserializes");
    let restored = restored
        .into_checked()
        .expect("exact private snapshot restores its checked owner Core");
    assert_eq!(restored, core);
    assert_eq!(
        restored.owner_admission_budget(),
        core.owner_admission_budget(),
        "round-trip must preserve the exact typed condition, not a name-derived summary"
    );
}

#[test]
fn ordinary_owner_snapshot_omits_the_new_budget_field_and_round_trips_unchanged() {
    let core = ordinary_checked_owner_core();
    let snapshot = SnapshotOwnerRmwCheckedCore::from_checked(&core);
    let serialized = serde_json::to_value(snapshot).expect("ordinary owner snapshot serializes");
    assert!(
        !serialized
            .as_object()
            .expect("ordinary owner snapshot serializes as an object")
            .contains_key("owner_admission_budget"),
        "legacy unannotated owner Core must not acquire a snapshot field by default"
    );

    let restored: SnapshotOwnerRmwCheckedCore =
        serde_json::from_value(serialized).expect("legacy snapshot remains readable");
    assert_eq!(
        restored
            .into_checked()
            .expect("legacy snapshot restores its prior checked Core"),
        core
    );
}

#[test]
fn private_owner_snapshot_rejects_tampered_budget_binding_components() {
    let (_, mut wrong_clock_domain) = serialized_budgeted_owner_snapshot();
    owner_budget_json(&mut wrong_clock_domain).insert("clock_domain".to_string(), json!("Other"));
    assert_tampered_snapshot_is_rejected(wrong_clock_domain, "noncanonical clock domain");

    let (_, mut wrong_owner) = serialized_budgeted_owner_snapshot();
    owner_budget_json(&mut wrong_owner).insert("owner_locus".to_string(), json!("T"));
    assert_tampered_snapshot_is_rejected(wrong_owner, "owner locus different from owner Core");

    let (_, mut zero_budget) = serialized_budgeted_owner_snapshot();
    owner_budget_json(&mut zero_budget).insert("budget_ticks".to_string(), json!(0));
    assert_tampered_snapshot_is_rejected(zero_budget, "zero budget");

    let (_, mut mismatched_source_ref) = serialized_budgeted_owner_snapshot();
    let condition = owner_budget_json(&mut mismatched_source_ref);
    let source_ref = condition
        .get_mut("source_ref")
        .expect("budget condition serializes its source reference")
        .as_object_mut()
        .expect("source reference serializes as an object");
    let start_column = source_ref
        .get("start_column")
        .and_then(Value::as_u64)
        .expect("source reference retains its start column");
    source_ref.insert("start_column".to_string(), json!(start_column + 1));
    assert_tampered_snapshot_is_rejected(
        mismatched_source_ref,
        "source reference inconsistent with the retained source span",
    );
}

#[test]
fn private_owner_snapshot_rejects_unknown_source_span_fields() {
    let (_, mut serialized) = serialized_budgeted_owner_snapshot();
    owner_budget_json(&mut serialized)
        .get_mut("source_span")
        .expect("budget condition serializes its source span")
        .as_object_mut()
        .expect("source span serializes as an object")
        .insert("unexpected_snapshot_field".to_string(), json!(true));

    assert_tampered_snapshot_is_rejected(serialized, "unknown source-span field");
}

#[test]
fn private_owner_snapshot_rejects_inverted_source_span_byte_range() {
    let (_, mut serialized) = serialized_budgeted_owner_snapshot();
    let condition = owner_budget_json(&mut serialized);
    let source_span = condition
        .get_mut("source_span")
        .expect("budget condition serializes its source span")
        .as_object_mut()
        .expect("source span serializes as an object");
    let byte_end = source_span
        .get("byte_end")
        .and_then(Value::as_u64)
        .expect("source span retains its byte end");
    source_span.insert("byte_start".to_string(), json!(byte_end + 1));

    assert_tampered_snapshot_is_rejected(serialized, "inverted source-span byte range");
}

#[test]
fn changed_valid_budget_snapshot_is_only_a_component_not_whole_artifact_admission() {
    let (core, mut serialized) = serialized_budgeted_owner_snapshot();
    let original_condition = core
        .owner_admission_budget()
        .expect("budgeted owner Core retains its source condition");
    let condition = owner_budget_json(&mut serialized);
    condition.insert("budget_ticks".to_string(), json!(2));
    let source_span = condition
        .get_mut("source_span")
        .expect("budget condition serializes its source span")
        .as_object_mut()
        .expect("source span serializes as an object");
    let byte_start = source_span
        .get("byte_start")
        .and_then(Value::as_u64)
        .expect("source span retains its byte start");
    source_span.insert("byte_start".to_string(), json!(byte_start + 1));

    let changed: SnapshotOwnerRmwCheckedCore =
        serde_json::from_value(serialized).expect("changed finite budget remains snapshot-shaped");
    let changed = changed
        .into_checked()
        .expect("a self-consistent component can restore as a different checked Core");
    assert_ne!(changed, core);
    assert_eq!(
        changed
            .owner_admission_budget()
            .expect("changed component retains a condition")
            .budget_ticks(),
        2
    );
    assert_ne!(
        changed
            .owner_admission_budget()
            .expect("changed component retains a condition"),
        original_condition,
        "a source-free component retains a changed source span distinctly from the checked original"
    );
    assert_ne!(
        changed
            .owner_admission_budget()
            .expect("changed component retains a condition")
            .source_span(),
        original_condition.source_span()
    );
    // This DTO restores only `OwnerRmwCheckedCore`; it carries neither a
    // `CheckedSurfaceV0` identity nor an execution-admission result.  The
    // later SYS3 binder owns comparison against the expected whole artifact.
}
