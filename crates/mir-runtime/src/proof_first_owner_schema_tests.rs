//! Bounded actual source/Core/schema projection controls, not network evidence.
use crate::sys3_i3_private_snapshot::I3PrivateProjectionSnapshot;
use crate::sys3_projection::{
    DeclaredLogicalTopology, GlobalProjectionResult, ProjectedOperationFragmentKind,
    ProjectionDiagnosticKind, project_checked_core, verify_projection,
};
use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{
    CheckedExpressionTree, CheckedSurfaceV0, TypedStateRead, check_and_elaborate_surface_v0,
};
use std::collections::BTreeSet;

fn source_body(body: &str) -> String {
    let source = "module Mirrorea.SchemaReads\n\
        locus A\nlocus S\nprincipal self\ntype Player\n\
        state shield[id: Player] at S {\n\
          hp: Int\nsecret_sibling: Int\nvisible observer_safe fields (hp)\n}\n\
        state unused[id: Player] at S { hp: Int }\n\
        state player[id: Player] at S { hp: Int }\n\
        state armor[id: Player] at S { hp: Int }\n\
        Role[self] at A {\n\
          when refresh(target: Player, amount: Int) fails \
          (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {\n\
            at S { player[target].hp = shield[target].hp + amount }\n\
          }\n}\nwith auth MembershipAuth\nverify finite_refinement\n";
    source.replace("player[target].hp = shield[target].hp + amount", body)
}

fn checked_body(body: &str) -> CheckedSurfaceV0 {
    let source = source_body(body);
    check_and_elaborate_surface_v0(FixtureSource::new("tests/inline/schema_reads.mir", source))
        .unwrap()
}

fn checked() -> CheckedSurfaceV0 {
    checked_body("player[target].hp = shield[target].hp + amount")
}

fn projection() -> GlobalProjectionResult {
    let checked = checked();
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    project_checked_core(&checked, &topology).unwrap()
}

#[test]
fn proof_first_owner_schema_keeps_source_order_and_no_duplicate_schemas() {
    let checked = checked_body(
        "player[target].hp = shield[target].hp + armor[target].hp - shield[self].hp + amount",
    );
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    let projected = project_checked_core(&checked, &topology).unwrap();
    assert_schema_objects(
        &checked,
        &projected,
        "refresh",
        &["shield", "player", "armor"],
    );
    verify_projection(&checked, &topology, &projected).unwrap();
    let restricted = projected.restricted_to_loci(&BTreeSet::from(["S".to_string()]));
    let image: I3PrivateProjectionSnapshot = serde_json::from_slice(
        &serde_json::to_vec(&restricted.to_i3_private_snapshot().unwrap()).unwrap(),
    )
    .unwrap();
    let restored = GlobalProjectionResult::from_i3_private_snapshot(image).unwrap();
    assert_eq!(restored, restricted);
    assert_schema_objects(
        &checked,
        &restored,
        "refresh",
        &["shield", "player", "armor"],
    );
}

#[test]
fn proof_first_owner_schema_constant_rhs_retains_only_target() {
    let checked = checked_body("player[target].hp = 7");
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    let projected = project_checked_core(&checked, &topology).unwrap();
    let fragment = projected
        .locus_program("S")
        .unwrap()
        .operations()
        .single("refresh", ProjectedOperationFragmentKind::OwnerRmwExecution)
        .unwrap();
    assert_eq!(
        fragment
            .local_state_schemas()
            .iter()
            .map(|s| s.name())
            .collect::<Vec<_>>(),
        ["player"]
    );
    assert!(
        fragment
            .owner_rmw_checked_core()
            .unwrap()
            .same_owner_reads()
            .is_empty()
    );
}

// This mutates only a decoded candidate. A raw private snapshot is not an
// authority credential, and decoding it is not executable-image admission.
fn remove_read_schema(value: &mut serde_json::Value, fields: &[&str]) -> usize {
    match value {
        serde_json::Value::Object(object) => object
            .iter_mut()
            .map(|(key, value)| {
                if fields.contains(&key.as_str()) {
                    let schemas = value.as_array_mut().unwrap();
                    let before = schemas.len();
                    schemas.retain(|schema| schema["name"] != "shield");
                    before - schemas.len()
                } else {
                    remove_read_schema(value, fields)
                }
            })
            .sum(),
        serde_json::Value::Array(values) => values
            .iter_mut()
            .map(|v| remove_read_schema(v, fields))
            .sum(),
        _ => 0,
    }
}

#[test]
fn proof_first_owner_schema_erasure_is_rejected_by_source_verifier() {
    let checked = checked();
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    let projected = project_checked_core(&checked, &topology).unwrap();
    assert_schemas(&projected);
    verify_projection(&checked, &topology, &projected).unwrap();
    for fields in [
        vec!["local_state_schemas"],
        vec!["checked_local_state_schemas"],
        vec!["local_state_schemas", "checked_local_state_schemas"],
    ] {
        let mut raw = serde_json::to_value(projected.to_i3_private_snapshot().unwrap()).unwrap();
        assert!(remove_read_schema(&mut raw, &fields) > 0);
        let image: I3PrivateProjectionSnapshot = serde_json::from_value(raw).unwrap();
        let candidate = GlobalProjectionResult::from_i3_private_snapshot(image).unwrap();
        assert_ne!(candidate, projected);
        assert_eq!(
            verify_projection(&checked, &topology, &candidate)
                .unwrap_err()
                .primary()
                .kind(),
            ProjectionDiagnosticKind::StructuralMismatch
        );
    }
}

fn assert_schemas(projection: &GlobalProjectionResult) {
    assert_schema_objects(&checked(), projection, "refresh", &["shield", "player"]);
}

fn tree_reads<'a>(tree: &'a CheckedExpressionTree, out: &mut Vec<&'a TypedStateRead>) {
    match tree {
        CheckedExpressionTree::StateRead(read) => out.push(read),
        CheckedExpressionTree::ParameterRead { .. } | CheckedExpressionTree::IntegerLiteral(_) => {}
        CheckedExpressionTree::Binary { left, right, .. } => {
            tree_reads(left, out);
            tree_reads(right, out);
        }
    }
}

fn assert_schema_objects(
    checked: &CheckedSurfaceV0,
    projection: &GlobalProjectionResult,
    operation: &str,
    expected: &[&str],
) {
    let owner = projection.locus_program("S").unwrap();
    let fragment = owner
        .operations()
        .single(operation, ProjectedOperationFragmentKind::OwnerRmwExecution)
        .unwrap();
    let schemas: Vec<_> = fragment
        .local_state_schemas()
        .iter()
        .map(|s| s.name())
        .collect();
    assert_eq!(
        schemas, expected,
        "every actually read namespace must retain its checked schema; unrelated state stays out"
    );
    let expected_objects: Vec<_> = expected
        .iter()
        .map(|name| {
            checked
                .static_environment()
                .indexed_state_schema(name)
                .unwrap()
                .clone()
        })
        .collect();
    assert_eq!(
        fragment.local_state_schemas(),
        expected_objects,
        "index type, source refs and unread sibling fields/visibility remain exact"
    );
    let core = fragment.owner_rmw_checked_core().unwrap();
    let mut reads = Vec::new();
    tree_reads(core.expression().tree(), &mut reads);
    assert_eq!(reads, core.same_owner_reads().iter().collect::<Vec<_>>());
    for read in std::iter::once(core.target()).chain(reads) {
        let schema = fragment
            .local_state_schemas()
            .iter()
            .find(|s| s.name() == read.namespace())
            .unwrap();
        assert_eq!(schema.owner_locus(), read.owner_locus());
        let field = schema
            .fields()
            .iter()
            .find(|f| Some(f.name()) == read.field())
            .unwrap();
        assert_eq!(field.type_name(), read.value_type());
    }
}

#[test]
fn proof_first_owner_schema_aggregate_is_an_occurrence_inventory() {
    let source = source_body("player[target].hp = shield[target].hp").replace(
        "with auth", "Role[self] at A {\n\
          when reset_shield(target: Player) fails \
          (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {\n\
            at S { shield[target].hp = 7 }\n\
          }\n}\nwith auth",
    );
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/schema_overlap.mir",
        source,
    ))
    .unwrap();
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    let projected = project_checked_core(&checked, &topology).unwrap();
    assert_schema_objects(&checked, &projected, "refresh", &["shield", "player"]);
    assert_schema_objects(&checked, &projected, "reset_shield", &["shield"]);
    let inventory = &projected
        .locus_program("S")
        .unwrap()
        .checked_fragments()
        .local_state_schemas;
    assert_eq!(
        inventory.iter().map(|s| s.name()).collect::<Vec<_>>(),
        ["player", "shield", "shield"]
    );
    // Two operation copies do not imply two declarations in the original source.
    assert_eq!(
        checked
            .static_environment()
            .indexed_state_schemas()
            .iter()
            .filter(|s| s.name() == "shield")
            .count(),
        1
    );
    verify_projection(&checked, &topology, &projected).unwrap();
    let image: I3PrivateProjectionSnapshot = serde_json::from_slice(
        &serde_json::to_vec(&projected.to_i3_private_snapshot().unwrap()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        GlobalProjectionResult::from_i3_private_snapshot(image).unwrap(),
        projected
    );
}

#[test]
fn proof_first_owner_schema_includes_same_owner_read_namespace() {
    let projected = projection();
    assert_schemas(&projected);
    let requester = projected.locus_program("A").unwrap();
    let fragment = requester
        .operations()
        .single(
            "refresh",
            ProjectedOperationFragmentKind::OwnerRequestInvocation,
        )
        .unwrap();
    assert!(fragment.local_state_schemas().is_empty());
}

#[test]
fn proof_first_owner_schema_survives_restriction_and_private_snapshot() {
    let projected = projection().restricted_to_loci(&BTreeSet::from(["S".to_string()]));
    let bytes = serde_json::to_vec(&projected.to_i3_private_snapshot().unwrap()).unwrap();
    let image: I3PrivateProjectionSnapshot = serde_json::from_slice(&bytes).unwrap();
    let restored = GlobalProjectionResult::from_i3_private_snapshot(image).unwrap();
    assert_eq!(restored, projected);
    assert_schemas(&restored);
    assert!(restored.locus_program("A").is_none());
}

#[test]
fn proof_first_owner_schema_provider_static_path_uses_same_complete_schema_selection() {
    use crate::sys3_projection::{
        project_read_only_provider_effect_static,
        verify_read_only_provider_effect_static_projection,
    };
    use mir_semantics::m9_finite_refinement::M9ReadOnlyProviderEffectCoverage;

    let source =
        include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir")
            .replace(
                "state participant_input",
                "state shield[id: Player] at WorldAuthority { hp: Int }\n\nstate participant_input",
            )
            .replace(
                "avatar[target].hp = avatar[target].hp - avatar[self].atk",
                "avatar[target].hp = shield[target].hp - avatar[self].atk",
            );
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/provider_schema.mir",
        source,
    ))
    .unwrap();
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        checked
            .static_environment()
            .loci()
            .iter()
            .map(|locus| locus.name()),
    )
    .unwrap();
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(&checked).unwrap();
    let plan = project_read_only_provider_effect_static(&checked, &topology, coverage).unwrap();
    verify_read_only_provider_effect_static_projection(&checked, &topology, &plan).unwrap();
    let fragment = plan
        .projection()
        .locus_program("WorldAuthority")
        .unwrap()
        .operations()
        .single("attack", ProjectedOperationFragmentKind::OwnerRmwExecution)
        .unwrap();
    assert_eq!(
        fragment
            .local_state_schemas()
            .iter()
            .map(|s| s.name())
            .collect::<Vec<_>>(),
        ["avatar", "shield"]
    );
    assert!(
        plan.activation_pending(),
        "schema retention issues no provider authority"
    );
}
