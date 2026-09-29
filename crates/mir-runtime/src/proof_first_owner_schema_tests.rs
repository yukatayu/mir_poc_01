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

// Provisional exact ordinary-call domain: actual checked source and executor,
// test-admitted M9; no external authentication/network/restore claim.
pub(crate) fn invocation_source_fixture(
    text: &str,
) -> (
    crate::sys4_dispatch::FabricProgram,
    crate::sys4_dispatch::SealedFabricAdmission,
) {
    invocation_source_fixture_at(text, "tests/inline/invocation.mir")
}

pub(crate) fn invocation_source_fixture_at(
    text: &str,
    path: &str,
) -> (
    crate::sys4_dispatch::FabricProgram,
    crate::sys4_dispatch::SealedFabricAdmission,
) {
    use crate::m9_auth_verification::M9RuntimeExecutionSeam;
    use crate::sys4_dispatch::{FabricProgram, SealedFabricAdmission, Sys4InitialStateSeed};
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, text)).unwrap();
    let topology =
        DeclaredLogicalTopology::try_new(checked.program_identity().clone(), ["A", "S"]).unwrap();
    let projection = project_checked_core(&checked, &topology).unwrap();
    let program = FabricProgram::from_projection(projection).unwrap();
    let seam = M9RuntimeExecutionSeam::test_real_admitted_sys4_fabric_seam(&checked).unwrap();
    let seed = Sys4InitialStateSeed::for_checked_program(checked.program_identity().clone())
        .with_int("S", "player", "self", "hp", 100)
        .with_int("S", "player", "self", "atk", 10)
        .with_int("S", "player", "target", "hp", 200)
        .with_int("S", "player", "target", "atk", 30);
    let admission = SealedFabricAdmission::from_m9_execution_seam(&program, seam, seed).unwrap();
    (program, admission)
}
const INVOCATION_SOURCE: &str =
    include_str!("../../mir-ast/tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir");

fn invocation_expect(
    text: &str,
    rows: &[(&str, &str)],
    success: bool,
    self_hp: i64,
    target_hp: i64,
) {
    use crate::sys3_projection::BackendProfile;
    use crate::sys4_dispatch::{LocalFabric, SourceAction};
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        let (program, admission) = invocation_source_fixture(text);
        let mut fabric = LocalFabric::bootstrap(program, admission, profile).unwrap();
        let mut action = SourceAction::owner_operation("attack");
        for (name, value) in rows {
            action = action.with_argument(*name, *value);
        }
        let result = fabric.dispatch_source_action(action);
        assert_eq!(
            result.is_ok(),
            success,
            "profile={profile:?} rows={rows:?} actual={:?}",
            result.as_ref().map(|_| ())
        );
        let state = fabric.semantic_snapshot();
        assert_eq!(state.int("S", "player", "self", "hp"), Some(self_hp));
        assert_eq!(state.int("S", "player", "target", "hp"), Some(target_hp));
    }
}

#[test]
fn proof_first_invocation_exact_positive_target_and_alias() {
    invocation_expect(INVOCATION_SOURCE, &[("target", "self")], true, 90, 200);
    invocation_expect(INVOCATION_SOURCE, &[("target", "target")], true, 100, 190);
    // with_argument is documented replacement in a map, not raw duplicate rows.
    invocation_expect(
        INVOCATION_SOURCE,
        &[("target", "target"), ("target", "self")],
        true,
        90,
        200,
    );
}
#[test]
fn proof_first_invocation_missing_target_refuses() {
    invocation_expect(INVOCATION_SOURCE, &[], false, 100, 200);
}
#[test]
fn proof_first_invocation_extra_cannot_override_nonparameter() {
    invocation_expect(
        INVOCATION_SOURCE,
        &[("target", "self"), ("self", "target")],
        false,
        100,
        200,
    );
}
#[test]
fn proof_first_invocation_rhs_capture_requires_actual_argument() {
    let text = INVOCATION_SOURCE.replace(
        "player[target].hp = player[target].hp - player[self].atk",
        "player[self].hp = player[self].hp - player[target].atk",
    );
    invocation_expect(&text, &[("target", "target")], true, 70, 200);
    invocation_expect(&text, &[], false, 100, 200);
}
#[test]
fn proof_first_invocation_unused_parameter_and_duplicate_domain() {
    let text = INVOCATION_SOURCE.replace("target: Player)", "target: Player, unused: Int)");
    invocation_expect(&text, &[("target", "self")], false, 100, 200);
    // This is an exact-name/domain guard, deliberately not scalar validation.
    invocation_expect(
        &text,
        &[("target", "self"), ("unused", "not-an-int")],
        true,
        90,
        200,
    );
    let duplicate = INVOCATION_SOURCE.replace(
        "target: Player)",
        "target: Player, unused: Int, unused: Int)",
    );
    invocation_expect(
        &duplicate,
        &[("target", "self"), ("unused", "1")],
        false,
        100,
        200,
    );
}
#[test]
fn proof_first_invocation_empty_signature_and_scalar_boundary() {
    let text = INVOCATION_SOURCE.replace("target: Player", "");
    invocation_expect(&text, &[], true, 100, 190);
    invocation_expect(&text, &[("self", "target")], false, 100, 200);
    let scalar = INVOCATION_SOURCE
        .replace("target: Player)", "target: Player, delta: Int)")
        .replace("player[self].atk", "player[self].atk + delta");
    invocation_expect(
        &scalar,
        &[("target", "self"), ("delta", "7")],
        true,
        97,
        200,
    );
    invocation_expect(&scalar, &[("target", "self")], false, 100, 200);
    invocation_expect(
        &scalar,
        &[("target", "self"), ("delta", "bad-int")],
        false,
        100,
        200,
    );
}
#[test]
fn proof_first_invocation_actual_private_roundtrip_and_erasure() {
    use crate::sys3_projection::BackendProfile;
    use crate::sys4_dispatch::{
        LocalFabric, SealedFabricAdmission, SourceAction, Sys4I3PrivateSealedAdmissionSnapshot,
    };
    let (program, admission) = invocation_source_fixture(INVOCATION_SOURCE);
    let original = serde_json::to_value(admission.i3_private_snapshot().unwrap()).unwrap();
    let signature = original
        .pointer("/instance/owner_execution_plans/0/signature")
        .expect("complete retained source signature");
    assert_eq!(
        signature
            .get("parameters")
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let restored = SealedFabricAdmission::from_i3_private_snapshot(
        serde_json::from_value(original.clone()).unwrap(),
        &program,
    )
    .unwrap();
    let mut fabric = LocalFabric::bootstrap(program.clone(), restored, BackendProfile::St).unwrap();
    fabric
        .dispatch_source_action(
            SourceAction::owner_operation("attack").with_argument("target", "self"),
        )
        .unwrap();
    assert_eq!(
        fabric.semantic_snapshot().int("S", "player", "self", "hp"),
        Some(90)
    );
    for change in [
        "empty-signature",
        "target",
        "missing-field",
        "signature-source",
    ] {
        let mut altered = original.clone();
        match change {
            "empty-signature" => {
                *altered
                    .pointer_mut("/instance/owner_execution_plans/0/signature/parameters")
                    .unwrap() = serde_json::json!([])
            }
            "target" => {
                *altered
                    .pointer_mut("/instance/owner_execution_plans/0/target/index")
                    .unwrap() = serde_json::json!("self")
            }
            "missing-field" => {
                altered
                    .pointer_mut("/instance/owner_execution_plans/0")
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove("signature");
            }
            "signature-source" => {
                *altered
                    .pointer_mut("/instance/owner_execution_plans/0/signature/name")
                    .unwrap() = serde_json::json!("other")
            }
            _ => unreachable!(),
        }
        match serde_json::from_value::<Sys4I3PrivateSealedAdmissionSnapshot>(altered) {
            Err(_) => assert_eq!(change, "missing-field"),
            Ok(image) => assert!(
                SealedFabricAdmission::from_i3_private_snapshot(image, &program).is_err(),
                "accepted {change}"
            ),
        }
    }
}

// Test-only observation of the real common M8 body-entry point. It records no
// semantic event and changes no guard/result. The path selects the subscription
// for this test, so unrelated concurrent tests are not counted as its body use.
struct InvocationCounts {
    counts: [std::sync::atomic::AtomicUsize; 5],
    argument_maps: std::sync::Mutex<Vec<std::collections::BTreeMap<String, String>>>,
}
static INVOCATION_BODY_PROBES: std::sync::Mutex<
    std::collections::BTreeMap<String, std::sync::Arc<InvocationCounts>>,
> = std::sync::Mutex::new(std::collections::BTreeMap::new());
pub(crate) fn invocation_body_entered(
    source: &mir_semantics::shared_model::SourceRef,
    arguments: &std::collections::BTreeMap<String, String>,
) {
    invocation_accessed(source, 0);
    let counts = INVOCATION_BODY_PROBES
        .lock()
        .unwrap()
        .get(&source.path)
        .cloned();
    if let Some(counts) = counts {
        counts.argument_maps.lock().unwrap().push(arguments.clone());
    }
}
pub(crate) fn invocation_accessed(source: &mir_semantics::shared_model::SourceRef, kind: usize) {
    let counts = INVOCATION_BODY_PROBES
        .lock()
        .unwrap()
        .get(&source.path)
        .cloned();
    if let Some(counts) = counts {
        counts.counts[kind].fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
pub(crate) struct InvocationBodyProbe {
    path: String,
    counts: std::sync::Arc<InvocationCounts>,
}
impl InvocationBodyProbe {
    pub(crate) fn for_path(path: &str) -> Self {
        let counts = std::sync::Arc::new(InvocationCounts {
            counts: std::array::from_fn(|_| std::sync::atomic::AtomicUsize::new(0)),
            argument_maps: std::sync::Mutex::new(Vec::new()),
        });
        let mut held = INVOCATION_BODY_PROBES.lock().unwrap();
        assert!(!held.contains_key(path), "test source paths must be unique");
        held.insert(path.to_string(), counts.clone());
        Self {
            path: path.to_string(),
            counts,
        }
    }
    fn count(&self) -> usize {
        self.counts()[0]
    }
    pub(crate) fn counts(&self) -> [usize; 5] {
        std::array::from_fn(|i| self.counts.counts[i].load(std::sync::atomic::Ordering::SeqCst))
    }
    pub(crate) fn argument_maps(&self) -> Vec<std::collections::BTreeMap<String, String>> {
        self.counts.argument_maps.lock().unwrap().clone()
    }
    pub(crate) fn reset(&self) {
        self.counts.argument_maps.lock().unwrap().clear();
        for count in self.counts.counts.iter() {
            count.store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
impl Drop for InvocationBodyProbe {
    fn drop(&mut self) {
        INVOCATION_BODY_PROBES.lock().unwrap().remove(&self.path);
    }
}

#[test]
fn proof_first_invocation_refusal_never_enters_body_on_st_or_worker() {
    use crate::sys3_projection::BackendProfile;
    use crate::sys4_dispatch::{LocalFabric, SourceAction};
    let path = "tests/inline/invocation_body_probe.mir";
    let probe = InvocationBodyProbe::for_path(path);
    let mut expected = 0;
    for profile in [BackendProfile::St, BackendProfile::Ow1] {
        for (rows, success) in [
            (vec![], false),
            (vec![("target", "self"), ("self", "target")], false),
            (vec![("target", "self")], true),
        ] {
            let (program, admission) = invocation_source_fixture_at(INVOCATION_SOURCE, path);
            let mut fabric = LocalFabric::bootstrap(program, admission, profile).unwrap();
            let mut action = SourceAction::owner_operation("attack");
            for (name, value) in rows {
                action = action.with_argument(name, value);
            }
            let result = fabric.dispatch_source_action(action);
            assert_eq!(result.is_ok(), success);
            expected += usize::from(success);
            assert_eq!(
                probe.count(),
                expected,
                "malformed invocation must not enter the real M8 body"
            );
        }
    }
}

fn invocation_authority() -> crate::m8_runtime_authority::M8AuthorityState {
    use crate::m8_runtime_authority::{
        M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
    };
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted("member")
                .with_principal("self")
                .with_locus("S")
                .with_epoch("one"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted("cap")
                .for_owner_evaluation("attack")
                .with_owner_locus("S")
                .with_principal("self")
                .with_membership_ref("member")
                .with_epoch("one"),
        )
        .with_witness_record(
            M8WitnessRecord::live("witness")
                .for_capability("cap")
                .with_membership_ref("member")
                .with_epoch("one"),
        )
}
fn invocation_authority_use(capability: &str) -> crate::m8_runtime_owner_queue::M8AuthorityUse {
    crate::m8_runtime_owner_queue::M8AuthorityUse::for_principal("self")
        .with_membership_ref("member")
        .with_capability_ref(capability)
        .with_witness_ref("witness")
}
const INVOCATION_LOCAL_SOURCE: &str =
    include_str!("../../mir-ast/tests/fixtures/surface-v0/m7_owner_only_no_residuals.mir");
fn invocation_m8_instance(
    text: &str,
    path: &str,
) -> crate::m8_runtime_admission::M8RuntimeInstance {
    use crate::m8_runtime_admission::{M8Runtime, M8RuntimeAdmission};
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, text)).unwrap();
    let admission = M8RuntimeAdmission::new(checked.program_identity().clone());
    M8Runtime::default().admit(checked, admission).unwrap()
}
fn invocation_m8_execution(
    text: &str,
    path: &str,
    target_present: bool,
) -> crate::m8_runtime_owner_queue::M8RuntimeExecution {
    use crate::m8_runtime_owner_queue::{M8ExecutionSeed, M8StateKey};
    let mut seed = M8ExecutionSeed::new()
        .with_int(M8StateKey::indexed_field("player", "self", "hp"), 100)
        .with_int(M8StateKey::indexed_field("player", "self", "atk"), 10)
        .with_authority_state(invocation_authority());
    if target_present {
        seed = seed
            .with_int(M8StateKey::indexed_field("player", "target", "hp"), 200)
            .with_int(M8StateKey::indexed_field("player", "target", "atk"), 30);
    }
    invocation_m8_instance(text, path).into_execution(seed)
}
#[test]
fn proof_first_invocation_actual_access_state_authority_and_revocation() {
    use crate::m8_runtime_authority::M8CapabilityGrant;
    use crate::m8_runtime_owner_queue::{M8DeclaredFailure, M8OwnerRequest, M8QueueTraceKind};
    let path = "tests/inline/invocation_access_frame.mir";
    let probe = InvocationBodyProbe::for_path(path);
    let unused = INVOCATION_LOCAL_SOURCE.replace("target: Player)", "target: Player, unused: Int)");
    let duplicate = INVOCATION_LOCAL_SOURCE.replace(
        "target: Player)",
        "target: Player, unused: Int, unused: Int)",
    );
    for target_present in [false, true] {
        for (text, rows) in [
            (INVOCATION_LOCAL_SOURCE, vec![]),
            (
                INVOCATION_LOCAL_SOURCE,
                vec![("target", "self"), ("self", "target")],
            ),
            (unused.as_str(), vec![("target", "self")]),
            (
                duplicate.as_str(),
                vec![("target", "self"), ("unused", "1")],
            ),
        ] {
            for capability in ["cap", "missing-cap"] {
                let mut runtime = invocation_m8_execution(text, path, target_present);
                let before = runtime.snapshot();
                probe.reset();
                let mut request = M8OwnerRequest::new("attack")
                    .with_authority_use(invocation_authority_use(capability));
                for (name, value) in &rows {
                    request = request.with_argument(*name, *value);
                }
                runtime.try_enqueue(request).expect(
                    "malformed inputs and unauthorized inputs must not reach presence preflight",
                );
                let error = runtime.serve_next_owner("S").unwrap_err();
                let expected = if capability == "cap" {
                    M8DeclaredFailure::RouteUnavailable
                } else {
                    M8DeclaredFailure::MissingCapability
                };
                assert_eq!(error.outcome().failure(), Some(expected));
                assert_eq!(
                    runtime.snapshot(),
                    before,
                    "compare actual full M8 snapshot, including authority/presence"
                );
                assert_eq!(
                    probe.counts(),
                    [0; 5],
                    "no body, materialization, state read/write, body event for {rows:?}/{capability}"
                );
                assert!(!runtime.trace().kinds().iter().any(|k| matches!(
                    k,
                    M8QueueTraceKind::OwnerRead | M8QueueTraceKind::OwnerWrite
                )));
                assert!(runtime.owner_queue("S").occurrence_ids().is_empty());
            }
        }
    }
    // Reauthorization at service: enqueue used the current valid authority.
    // Both a valid and a malformed domain must now fail with that authority error.
    for supplied in [false, true] {
        let mut runtime = invocation_m8_execution(INVOCATION_LOCAL_SOURCE, path, true);
        let mut request =
            M8OwnerRequest::new("attack").with_authority_use(invocation_authority_use("cap"));
        if supplied {
            request = request.with_argument("target", "self");
        }
        runtime.try_enqueue(request).unwrap();
        runtime.snapshot.replace_authority_state(
            invocation_authority().with_capability_grant(M8CapabilityGrant::revoked("cap")),
        );
        let before = runtime.snapshot();
        probe.reset();
        assert_eq!(
            runtime
                .serve_next_owner("S")
                .unwrap_err()
                .outcome()
                .failure(),
            Some(M8DeclaredFailure::MissingCapability)
        );
        assert_eq!(runtime.snapshot(), before);
        assert_eq!(probe.counts(), [0; 5]);
    }
    // An accessed malformed scalar has a valid name domain and may enter/read
    // before its existing parse failure. Do not overclaim the domain frame.
    let scalar = INVOCATION_LOCAL_SOURCE
        .replace("target: Player)", "target: Player, delta: Int)")
        .replace("player[self].atk", "player[self].atk + delta");
    let mut runtime = invocation_m8_execution(&scalar, path, true);
    let before = runtime.snapshot();
    probe.reset();
    runtime
        .try_enqueue(
            M8OwnerRequest::new("attack")
                .with_authority_use(invocation_authority_use("cap"))
                .with_argument("target", "self")
                .with_argument("delta", "bad"),
        )
        .unwrap();
    assert_eq!(
        runtime
            .serve_next_owner("S")
            .unwrap_err()
            .outcome()
            .failure(),
        Some(M8DeclaredFailure::RouteUnavailable)
    );
    assert_eq!(runtime.snapshot(), before);
    let counts = probe.counts();
    assert_eq!(counts[0], 1);
    assert!(counts[2] > 0);
    assert_eq!(counts[3], 0);
    assert_eq!(counts[4], 0);
}
#[test]
fn proof_first_invocation_checked_event_uniqueness_and_declared_self() {
    use mir_semantics::surface_v0_pipeline::M7DiagnosticKind;
    // ast.when selects by globally unique event name on every checked artifact.
    let start = INVOCATION_LOCAL_SOURCE.find("Role[self]").unwrap();
    let duplicate = format!(
        "{}\n{}",
        INVOCATION_LOCAL_SOURCE,
        &INVOCATION_LOCAL_SOURCE[start..]
    );
    let rejected =
        check_and_elaborate_surface_v0(FixtureSource::new("duplicate-event.mir", duplicate))
            .unwrap_err();
    assert_eq!(rejected.primary().kind(), M7DiagnosticKind::DuplicateEvent);
    // Existing finite source semantics classify an explicitly declared self as
    // a parameter. This does not alter the actor identity or grant authority.
    let text = INVOCATION_SOURCE.replace("target: Player)", "target: Player, self: Player)");
    invocation_expect(
        &text,
        &[("target", "self"), ("self", "target")],
        true,
        70,
        200,
    );
    invocation_expect(&text, &[("target", "self")], false, 100, 200);
}

fn invocation_local_seed() -> crate::m8_runtime_local_cut::M8LocalRuntimeSeed {
    use crate::m8_runtime_local_cut::M8LocalRuntimeSeed;
    use crate::m8_runtime_owner_queue::M8StateKey;
    M8LocalRuntimeSeed::new()
        .with_owner_int(M8StateKey::indexed_field("player", "self", "hp"), 100)
        .with_owner_int(M8StateKey::indexed_field("player", "self", "atk"), 10)
        .with_authority_state(invocation_authority())
}
fn invocation_request(unused: bool) -> crate::m8_runtime_owner_queue::M8OwnerRequest {
    let request = crate::m8_runtime_owner_queue::M8OwnerRequest::new("attack")
        .with_argument("target", "self")
        .with_authority_use(invocation_authority_use("cap"));
    if unused {
        request.with_argument("unused", "1")
    } else {
        request
    }
}
#[test]
fn proof_first_invocation_clone_restriction_and_pending_cut_keep_signature() {
    use crate::m8_runtime_local_cut::{M8LiveFloor, M8LocalRuntime};
    use crate::m8_runtime_owner_queue::M8DeclaredFailure;
    let source = INVOCATION_LOCAL_SOURCE.replace("target: Player)", "target: Player, unused: Int)");
    for (text, required) in [(INVOCATION_LOCAL_SOURCE, false), (source.as_str(), true)] {
        let instance = invocation_m8_instance(text, "tests/inline/invocation_cut.mir");
        for current in [
            instance.clone(),
            instance.restricted_to_loci(&BTreeSet::from(["S".to_string()])),
        ] {
            let mut runtime =
                M8LocalRuntime::from_admitted(current.clone(), invocation_local_seed());
            runtime.enqueue_owner(invocation_request(required)).unwrap();
            runtime.enqueue_owner(invocation_request(false)).unwrap();
            let cut = runtime.save_local_cut("pending-signature");
            let floor = M8LiveFloor::same_current(&cut);
            let mut restored = M8LocalRuntime::from_admitted(current, invocation_local_seed());
            restored.try_restore_local_cut(&cut, &floor).unwrap();
            assert_eq!(
                restored.save_relevant_payload(),
                cut.save_relevant_payload()
            );
            // Cloning a restored session retains the pending map, plan and full rows.
            for mut copy in [restored.clone(), restored] {
                copy.serve_next_owner("S").unwrap();
                let before = copy.owner_state().clone();
                let outcome = copy.serve_next_owner("S");
                if required {
                    assert_eq!(
                        outcome.unwrap_err().outcome().failure(),
                        Some(M8DeclaredFailure::RouteUnavailable)
                    );
                    assert_eq!(copy.owner_state(), &before);
                } else {
                    outcome.unwrap();
                }
                let payload = copy.save_relevant_payload();
                assert!(
                    copy.try_restore_local_cut(&cut, &floor.clone().with_revoked_capability("cap"))
                        .is_err()
                );
                assert_eq!(
                    copy.save_relevant_payload(),
                    payload,
                    "failed floor restore preserves all saved state, plans and pending maps"
                );
            }
        }
    }
    let base = invocation_m8_instance(INVOCATION_LOCAL_SOURCE, "tests/inline/invocation_cut.mir");
    let current = invocation_m8_instance(&source, "tests/inline/invocation_cut.mir");
    let old =
        M8LocalRuntime::from_admitted(base, invocation_local_seed()).save_local_cut("old-source");
    let mut runtime = M8LocalRuntime::from_admitted(current, invocation_local_seed());
    let before = runtime.save_relevant_payload();
    assert!(
        runtime
            .try_restore_local_cut(&old, &M8LiveFloor::same_current(&old))
            .is_err()
    );
    assert_eq!(runtime.save_relevant_payload(), before);
}
#[test]
fn proof_first_invocation_patch_installs_current_signature_and_refuses_pending() {
    use crate::m8_runtime_admission::M8RuntimeAdmission;
    use crate::m8_runtime_authority::{M8CapabilityGrant, M8WitnessRecord};
    use crate::m8_runtime_local_cut::M8LiveFloor;
    use crate::m8_runtime_owner_queue::{M8DeclaredFailure, M8StateKey};
    use crate::m8_runtime_patch::{
        M8PatchAuthorityUse, M8PatchCandidate, M8PatchDiagnosticKind, M8PatchRuntime,
        M8PatchRuntimeSeed,
    };
    use crate::m9_auth_verification::M9RuntimeExecutionSeam;
    for resolved in [false, true] {
        let text = if resolved {
            INVOCATION_SOURCE
        } else {
            INVOCATION_LOCAL_SOURCE
        };
        let next = text.replace("target: Player)", "target: Player, unused: Int)");
        let path = "tests/inline/invocation_patch.mir";
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(path, text)).unwrap();
        let candidate = check_and_elaborate_surface_v0(FixtureSource::new(path, next)).unwrap();
        let base = if resolved {
            M9RuntimeExecutionSeam::test_real_admitted_sys4_fabric_seam(&checked)
                .unwrap()
                .into_parts()
                .0
        } else {
            invocation_m8_instance(text, path)
        };
        let module = checked.program_identity().module().to_string();
        // Test-admitted patch authority; this tests retention at M8, not issuance.
        let authority = invocation_authority()
            .with_capability_grant(
                M8CapabilityGrant::already_admitted("patch-cap")
                    .for_patch_activation(&module)
                    .with_owner_locus("S")
                    .with_principal("self")
                    .with_membership_ref("member")
                    .with_epoch("one"),
            )
            .with_witness_record(
                M8WitnessRecord::live("patch-witness")
                    .for_capability("patch-cap")
                    .with_membership_ref("member")
                    .with_epoch("one"),
            );
        let mut runtime = M8PatchRuntime::from_admitted(
            base,
            M8PatchRuntimeSeed::new()
                .with_owner_int(M8StateKey::indexed_field("player", "self", "hp"), 100)
                .with_owner_int(M8StateKey::indexed_field("player", "self", "atk"), 10)
                .with_authority_state(authority),
        );
        let patch = if resolved {
            let instance = M9RuntimeExecutionSeam::test_real_admitted_sys4_fabric_seam(&candidate)
                .unwrap()
                .into_parts()
                .0;
            M8PatchCandidate::from_m10_resolved("signature-change", candidate.clone(), instance)
        } else {
            M8PatchCandidate::from_checked_admitted(
                "signature-change",
                candidate.clone(),
                M8RuntimeAdmission::new(candidate.program_identity().clone()),
            )
        }
        .with_base_program_identity(runtime.active_program_identity().clone())
        .with_base_admission(runtime.active_admission().clone())
        .with_patch_authority(
            M8PatchAuthorityUse::for_patch_program(&module)
                .with_owner_locus("S")
                .with_principal("self")
                .with_membership_ref("member")
                .with_capability_ref("patch-cap")
                .with_witness_ref("patch-witness"),
        );
        runtime.enqueue_owner(invocation_request(false)).unwrap();
        let pending = runtime.save_relevant_payload();
        let refused = runtime.activate_patch(patch.clone());
        assert_eq!(
            refused.primary_diagnostic().kind(),
            M8PatchDiagnosticKind::NonQuiescentSession
        );
        assert_eq!(runtime.save_relevant_payload(), pending);
        runtime.serve_next_owner("S").unwrap();
        let old_cut = runtime.save_local_cut("old-patch-source");
        let accepted = runtime.activate_patch(patch);
        assert!(accepted.has_runtime_success(), "{accepted:?}");
        let new_cut = runtime.save_local_cut("new-patch-source");
        for mut local in [runtime.local_session_clone()] {
            local.enqueue_owner(invocation_request(false)).unwrap();
            let before = local.owner_state().clone();
            assert_eq!(
                local.serve_next_owner("S").unwrap_err().outcome().failure(),
                Some(M8DeclaredFailure::RouteUnavailable)
            );
            assert_eq!(local.owner_state(), &before);
            local.enqueue_owner(invocation_request(true)).unwrap();
            local.serve_next_owner("S").unwrap();
        }
        let current = runtime.save_relevant_payload();
        assert!(
            runtime
                .try_restore_local_cut(
                    &old_cut,
                    &M8LiveFloor::same_current(&old_cut).with_revoked_capability("cap")
                )
                .is_err()
        );
        assert_eq!(
            runtime.save_relevant_payload(),
            current,
            "staged plan installation cannot leak after failed restore"
        );
        runtime
            .try_restore_local_cut(&new_cut, &M8LiveFloor::same_current(&new_cut))
            .unwrap();
        runtime.enqueue_owner(invocation_request(false)).unwrap();
        assert_eq!(
            runtime
                .serve_next_owner("S")
                .unwrap_err()
                .outcome()
                .failure(),
            Some(M8DeclaredFailure::RouteUnavailable)
        );
        runtime.enqueue_owner(invocation_request(true)).unwrap();
        runtime.serve_next_owner("S").unwrap();
    }
}
