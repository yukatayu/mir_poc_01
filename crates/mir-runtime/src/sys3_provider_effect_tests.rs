use std::collections::BTreeSet;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::M9ReadOnlyProviderEffectCoverage,
    surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::{
        CheckedSurfaceV0, EffectKind, GeneratedObligationKind, check_and_elaborate_surface_v0,
    },
};
use serde_json::{Map, Value};

use crate::sys3_i3_private_snapshot::{
    I3PrivateProjectionSnapshot, I3PrivateProjectionSnapshotError,
    I3PrivateProviderStaticProjectionSnapshot,
};
use crate::sys4_dispatch::{FabricProgram, Sys4DiagnosticKind};

use super::{
    CarrierProvenanceKind, CommunicationEdge, CommunicationEdgeKind, DeclaredLogicalTopology,
    GlobalProjectionResult, ProjectedOperationFragmentKind, ProjectionDiagnosticKind,
    ReadOnlyProviderEffectStaticProjection, RuntimeSeamRequirementKind, SeamAuthorityKind,
    project_checked_core, project_read_only_provider_effect_static,
    verify_read_only_provider_effect_static_projection,
};

const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");
const LEGACY_CONTROL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const LEGACY_CONTROL_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const ORDINARY_PROVIDER_SNAPSHOT_REJECTION: &str =
    "read-only provider static projection requires its dedicated snapshot";

fn checked_provider_effect_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the ordinary mixed provider-effect source checks before static projection")
}

fn checked_legacy_control_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        LEGACY_CONTROL_SOURCE_PATH,
        LEGACY_CONTROL_SOURCE,
    ))
    .expect("the canonical I2 control source checks before legacy inventory comparison")
}

fn provider_effect_topology(checked: &CheckedSurfaceV0) -> DeclaredLogicalTopology {
    DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the fixture's full checked topology remains a valid static projection topology")
}

fn canonical_legacy_projection_snapshot_json() -> (GlobalProjectionResult, Value) {
    let checked = checked_legacy_control_source();
    let projection = project_checked_core(&checked, &provider_effect_topology(&checked))
        .expect("the genuine legacy control source projects through ordinary SYS-3");
    let snapshot = projection
        .to_i3_private_snapshot()
        .expect("the genuine legacy control projection exports an ordinary private snapshot");
    let serialized =
        serde_json::to_value(snapshot).expect("the genuine ordinary private snapshot serializes");
    (projection, serialized)
}

fn restore_ordinary_projection_snapshot(
    serialized: Value,
) -> Result<GlobalProjectionResult, I3PrivateProjectionSnapshotError> {
    let snapshot: I3PrivateProjectionSnapshot = serde_json::from_value(serialized)
        .expect("the scoped ordinary snapshot falsifier remains syntactically decodable");
    GlobalProjectionResult::from_i3_private_snapshot(snapshot)
}

fn assert_ordinary_provider_snapshot_rejected(
    result: Result<GlobalProjectionResult, I3PrivateProjectionSnapshotError>,
    case: &str,
) {
    match result {
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
            kind: ORDINARY_PROVIDER_SNAPSHOT_REJECTION,
        }) => {}
        Ok(_) => panic!("{case}: ordinary executable snapshot unexpectedly restored"),
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant { kind }) => {
            panic!("{case}: ordinary snapshot used unexpected unsupported variant {kind}")
        }
        Err(I3PrivateProjectionSnapshotError::UnsupportedVersion { .. }) => {
            panic!("{case}: ordinary snapshot rejected at unsupported-version validation")
        }
        Err(I3PrivateProjectionSnapshotError::DuplicateMapKey { .. }) => {
            panic!("{case}: ordinary snapshot rejected at duplicate-map validation")
        }
        Err(I3PrivateProjectionSnapshotError::DuplicateSetMember { .. }) => {
            panic!("{case}: ordinary snapshot rejected at duplicate-set validation")
        }
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch { reason }) => {
            panic!(
                "{case}: ordinary snapshot rejected at unrelated structural validation: {reason}"
            )
        }
        Err(I3PrivateProjectionSnapshotError::SemanticSnapshot) => {
            panic!("{case}: ordinary snapshot rejected at unrelated semantic-snapshot validation")
        }
    }
}

fn canonical_provider_static_snapshot_json() -> Value {
    provider_static_snapshot_json(&checked_provider_effect_source())
}

fn ordinary_snapshot_owner_request_edge_mut(snapshot: &mut Value) -> &mut Map<String, Value> {
    for edge in snapshot
        .pointer_mut("/communication_plan/edges")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual communication-plan edge vector")
    {
        if edge.get("kind").and_then(Value::as_str) == Some("owner_request") {
            let carrier = edge
                .get("carrier_contract")
                .and_then(Value::as_object)
                .expect("ordinary owner request retains its carrier contract");
            if carrier.get("edge_kind").and_then(Value::as_str) == Some("owner_request")
                && carrier.get("lifecycle_kind").and_then(Value::as_str) == Some("owner_request")
                && !carrier.contains_key("read_only_provider_effect_details")
            {
                return edge
                    .as_object_mut()
                    .expect("ordinary owner-request edge record remains an object");
            }
        }
    }
    panic!("genuine legacy snapshot retains an ordinary owner-request edge")
}

fn ordinary_snapshot_owner_request_fragment_mut(snapshot: &mut Value) -> &mut Map<String, Value> {
    for locus_program in snapshot
        .pointer_mut("/locus_programs")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual locus-program vector")
    {
        let Some(operations) = locus_program
            .get_mut("value")
            .and_then(Value::as_object_mut)
            .and_then(|program| program.get_mut("operations"))
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        for fragment in operations {
            if fragment.get("kind").and_then(Value::as_str) == Some("owner_request_invocation")
                && fragment.pointer("/placement/kind").and_then(Value::as_str)
                    == Some("owner_request")
                && fragment
                    .pointer("/checked_core_identity/fragment_kind")
                    .and_then(Value::as_str)
                    == Some("owner_request_invocation")
            {
                return fragment
                    .as_object_mut()
                    .expect("ordinary owner-request fragment remains an object");
            }
        }
    }
    panic!("genuine legacy snapshot retains an ordinary owner-request fragment")
}

fn ordinary_snapshot_handler_with_fragment_identity_mut(
    snapshot: &mut Value,
) -> &mut Map<String, Value> {
    for handler in snapshot
        .pointer_mut("/effect_handler_plan/handlers")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual effect-handler vector")
    {
        if handler
            .pointer("/checked_core_identity/fragment_kind")
            .and_then(Value::as_str)
            .is_some()
        {
            return handler
                .as_object_mut()
                .expect("ordinary effect-handler entry remains an object");
        }
    }
    panic!("genuine legacy snapshot retains an effect handler with a fragment identity")
}

fn ordinary_snapshot_source_map_fragment_identity_mut(
    snapshot: &mut Value,
) -> &mut Map<String, Value> {
    for entry in snapshot
        .pointer_mut("/projected_source_map")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual projected-source map vector")
    {
        let Some(value) = entry.get_mut("value") else {
            continue;
        };
        if value
            .pointer("/checked_core_identity/fragment_kind")
            .and_then(Value::as_str)
            .is_some()
        {
            return value
                .as_object_mut()
                .expect("ordinary projected-source map value remains an object");
        }
    }
    panic!("genuine legacy snapshot retains a projected-source fragment identity")
}

fn ordinary_snapshot_observation_edge_identity_mut(
    snapshot: &mut Value,
) -> &mut Map<String, Value> {
    for row in snapshot
        .pointer_mut("/observation_plan/rows")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual observation-row vector")
    {
        if row
            .get("edge_identity")
            .and_then(Value::as_object)
            .is_some()
        {
            return row
                .get_mut("edge_identity")
                .and_then(Value::as_object_mut)
                .expect("ordinary observation row retains its edge identity");
        }
    }
    panic!("genuine legacy snapshot retains an observation edge identity")
}

fn ordinary_snapshot_relation_residual_mut(snapshot: &mut Value) -> &mut Map<String, Value> {
    for relation in snapshot
        .pointer_mut("/relation_graph/relations")
        .and_then(Value::as_array_mut)
        .expect("ordinary snapshot retains its actual relation vector")
    {
        let Some(residuals) = relation
            .get_mut("value")
            .and_then(Value::as_object_mut)
            .and_then(|value| value.get_mut("residual_source_refs"))
            .and_then(Value::as_array_mut)
        else {
            continue;
        };
        if let Some(residual) = residuals.first_mut() {
            return residual
                .as_object_mut()
                .expect("ordinary relation residual remains an object");
        }
    }
    panic!("genuine legacy snapshot retains a relation runtime residual")
}

fn json_value_at(value: &Value, pointer: &str, description: &str) -> Value {
    value
        .pointer(pointer)
        .cloned()
        .unwrap_or_else(|| panic!("provider static snapshot retains {description} at {pointer}"))
}

fn provider_static_request_edge(snapshot: &Value) -> &Value {
    snapshot
        .pointer("/projection/communication_plan/edges")
        .and_then(Value::as_array)
        .expect("provider static snapshot retains its actual communication-plan edge vector")
        .iter()
        .find(|edge| {
            edge.get("kind").and_then(Value::as_str) == Some("read_only_provider_effect_request")
        })
        .expect("provider static snapshot retains its actual request carrier")
}

fn provider_static_service_fragment(snapshot: &Value) -> &Value {
    snapshot
        .pointer("/projection/locus_programs")
        .and_then(Value::as_array)
        .expect("provider static snapshot retains its actual locus-program vector")
        .iter()
        .filter_map(|entry| entry.get("value"))
        .filter_map(Value::as_object)
        .filter_map(|program| program.get("operations"))
        .filter_map(Value::as_array)
        .flatten()
        .find(|fragment| {
            fragment.get("kind").and_then(Value::as_str)
                == Some("read_only_provider_effect_service")
        })
        .expect("provider static snapshot retains its actual service fragment")
}

fn provider_static_invocation_effect(snapshot: &Value) -> Value {
    snapshot
        .pointer("/provider_coverage/effects")
        .and_then(Value::as_array)
        .expect("provider static snapshot retains its checked provider effect inventory")
        .iter()
        .find(|kind| kind.as_str() == Some("read_only_provider_effect_invocation"))
        .cloned()
        .expect("provider static snapshot retains the invocation effect marker")
}

fn canonical_static_provider_projection(
    checked: &CheckedSurfaceV0,
) -> ReadOnlyProviderEffectStaticProjection {
    let topology = provider_effect_topology(checked);
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(checked)
        .expect("full checked source derives its non-authorizing provider coverage");
    let static_projection = project_read_only_provider_effect_static(checked, &topology, coverage)
        .expect("the dedicated static entry projects checked provider coverage without execution");
    verify_read_only_provider_effect_static_projection(checked, &topology, &static_projection)
        .expect("the canonical static projection retains every checked source association");
    static_projection
}

fn provider_static_snapshot_json(checked: &CheckedSurfaceV0) -> Value {
    let static_projection = canonical_static_provider_projection(checked);
    let snapshot = static_projection
        .to_i3_private_static_snapshot()
        .expect("canonical provider static projection exports its dedicated private snapshot");
    serde_json::to_value(snapshot)
        .expect("canonical provider static snapshot serializes for scoped falsification")
}

fn restore_provider_static_snapshot(
    serialized: Value,
    checked: &CheckedSurfaceV0,
) -> Result<ReadOnlyProviderEffectStaticProjection, I3PrivateProjectionSnapshotError> {
    let snapshot: I3PrivateProviderStaticProjectionSnapshot = serde_json::from_value(serialized)
        .expect("scoped provider static snapshot falsifier remains syntactically decodable");
    ReadOnlyProviderEffectStaticProjection::from_i3_private_static_snapshot(
        snapshot,
        checked,
        &provider_effect_topology(checked),
    )
}

fn provider_snapshot_edges_mut(snapshot: &mut Value) -> &mut Vec<Value> {
    snapshot
        .pointer_mut("/projection/communication_plan/edges")
        .and_then(Value::as_array_mut)
        .expect("provider static snapshot retains its actual communication-plan edge vector")
}

fn is_provider_effect_snapshot_edge(edge: &Value) -> bool {
    matches!(
        edge.get("kind").and_then(Value::as_str),
        Some("read_only_provider_effect_request" | "read_only_provider_effect_result")
    )
}

fn is_provider_effect_snapshot_fragment(fragment: &Value) -> bool {
    matches!(
        fragment.get("kind").and_then(Value::as_str),
        Some(
            "read_only_provider_effect_requester"
                | "read_only_provider_effect_service"
                | "read_only_provider_effect_result_consumer"
        )
    )
}

fn ordinary_snapshot_from_static_snapshot_json(snapshot: &Value) -> I3PrivateProjectionSnapshot {
    serde_json::from_value(
        snapshot
            .get("projection")
            .cloned()
            .expect("dedicated static snapshot retains its inner ordinary projection DTO"),
    )
    .expect("the canonical inner projection remains syntactically decodable as an ordinary DTO")
}

fn assert_provider_edge_static_only(edge: &CommunicationEdge) {
    let carrier = edge.carrier_contract();
    assert_eq!(carrier.origin_principal_template(), None);
    assert_eq!(carrier.origin_locus_template(), None);
    assert_eq!(carrier.target_owner_locus_template(), None);
    assert_eq!(carrier.owner_admission_budget(), None);
    assert_eq!(carrier.designated_remote_input_dependency(), None);
    assert_eq!(carrier.result_version(), None);
    assert_eq!(carrier.input_frontier(), None);
    assert_eq!(carrier.result_frontier(), None);
    assert_eq!(carrier.observation_policy(), None);
    assert_eq!(carrier.policy_stamp(), None);
    assert_eq!(carrier.static_retry_contract(), None);
    assert!(
        carrier.read_only_provider_effect_details().is_some(),
        "the provider carrier retains checked profile details without inheriting designated details"
    );
    assert!(
        carrier.i3_adapter_static_facts().is_none(),
        "provider carriers remain outside the ordinary executable I3 adapter algebra"
    );
    assert!(!carrier.transfers_authority());
    assert!(!carrier.mints_authority_without_source());
    assert_eq!(
        carrier
            .authority_requirements()
            .runtime_seam_requirements()
            .rows(),
        &[
            (
                RuntimeSeamRequirementKind::ProviderEffectMembershipEpochIncarnation,
                Some(GeneratedObligationKind::ProviderEffectAuthorization),
                CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                Some(SeamAuthorityKind::ProviderEffectMembership),
            ),
            (
                RuntimeSeamRequirementKind::ProviderEffectUseCapability,
                Some(GeneratedObligationKind::ProviderEffectAuthorization),
                CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                Some(SeamAuthorityKind::ProviderEffectUseCapability),
            ),
            (
                RuntimeSeamRequirementKind::ProviderEffectUseWitness,
                Some(GeneratedObligationKind::ProviderEffectAuthorization),
                CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                Some(SeamAuthorityKind::ProviderEffectUseWitness),
            ),
        ],
        "provider membership, capability, and witness remain exact static effect requirements"
    );
}

fn provider_snapshot_fragment_operations_mut<'snapshot>(
    snapshot: &'snapshot mut Value,
    locus: &str,
) -> &'snapshot mut Vec<Value> {
    snapshot
        .pointer_mut("/projection/locus_programs")
        .and_then(Value::as_array_mut)
        .expect("provider static snapshot retains its actual locus-program vector")
        .iter_mut()
        .find(|entry| entry.get("key").and_then(Value::as_str) == Some(locus))
        .and_then(|entry| entry.get_mut("value"))
        .and_then(Value::as_object_mut)
        .and_then(|program| program.get_mut("operations"))
        .and_then(Value::as_array_mut)
        .expect("provider static snapshot retains the selected actual locus operations")
}

fn is_provider_effect_lowering(kind: SourceToCoreKind) -> bool {
    matches!(
        kind,
        SourceToCoreKind::ReadOnlyProviderEffectRequest
            | SourceToCoreKind::ReadOnlyProviderEffectInvocation
            | SourceToCoreKind::ReadOnlyProviderEffectResult
            | SourceToCoreKind::ReadOnlyProviderEffectResultConsume
    )
}

fn is_provider_effect_fragment(kind: ProjectedOperationFragmentKind) -> bool {
    matches!(
        kind,
        ProjectedOperationFragmentKind::ReadOnlyProviderEffectRequester
            | ProjectedOperationFragmentKind::ReadOnlyProviderEffectService
            | ProjectedOperationFragmentKind::ReadOnlyProviderEffectResultConsumer
    )
}

fn is_provider_effect_edge(kind: CommunicationEdgeKind) -> bool {
    matches!(
        kind,
        CommunicationEdgeKind::ReadOnlyProviderEffectRequest
            | CommunicationEdgeKind::ReadOnlyProviderEffectResult
    )
}

fn legacy_fragment_inventory(
    projection: &GlobalProjectionResult,
) -> Vec<(String, ProjectedOperationFragmentKind, String)> {
    let mut inventory = projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .filter(|fragment| !is_provider_effect_fragment(fragment.fragment_kind()))
        .map(|fragment| {
            (
                fragment.operation_id().to_string(),
                fragment.fragment_kind(),
                fragment.locus_tag().as_str().to_string(),
            )
        })
        .collect::<Vec<_>>();
    inventory.sort();
    inventory
}

fn legacy_edge_inventory(
    projection: &GlobalProjectionResult,
) -> Vec<(String, CommunicationEdgeKind, String, String)> {
    let mut inventory = projection
        .communication_plan()
        .edges()
        .iter()
        .filter(|edge| !is_provider_effect_edge(edge.kind()))
        .map(|edge| {
            (
                edge.operation_id().to_string(),
                edge.kind(),
                edge.source_locus().to_string(),
                edge.target_locus().to_string(),
            )
        })
        .collect::<Vec<_>>();
    inventory.sort();
    inventory
}

#[test]
fn i3_provider_effect_static_projection_derives_full_source_coverage_and_keeps_generic_projection_rejected()
 {
    let checked = checked_provider_effect_source();
    let topology = provider_effect_topology(&checked);
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(&checked)
        .expect("full checked source derives its non-authorizing provider coverage");
    let static_projection = project_read_only_provider_effect_static(&checked, &topology, coverage)
        .expect("the dedicated static entry projects checked provider coverage without execution");
    verify_read_only_provider_effect_static_projection(&checked, &topology, &static_projection)
        .expect("the canonical static projection retains every checked source association");
    match FabricProgram::from_projection(static_projection.projection().clone()) {
        Err(diagnostics)
            if diagnostics.primary().kind() == Sys4DiagnosticKind::ProgramProjectionMismatch => {}
        Ok(_) => {
            panic!("provider static projection unexpectedly passed SYS-4 executable preflight")
        }
        Err(diagnostics) => panic!(
            "provider static projection reached SYS-4 with wrong diagnostic {:?}",
            diagnostics.primary().kind()
        ),
    }

    assert_eq!(
        static_projection.checked_program_identity(),
        checked.program_identity(),
        "static provider coverage retains the full checked-program identity"
    );
    assert!(
        static_projection.activation_pending(),
        "static coverage does not claim provider activation, permission, or executable admission"
    );

    let coverage = static_projection.provider_coverage();
    assert_eq!(coverage.program_identity(), checked.program_identity());
    assert!(
        coverage.matches_checked(&checked),
        "static coverage must retain exact checked Core/source associations rather than a provider-present summary"
    );

    let expected_associations = checked
        .source_map()
        .entries()
        .iter()
        .filter(|entry| entry.kind() != SourceToCoreKind::DeferredPolicy)
        .collect::<Vec<_>>();
    let actual_associations = coverage.executable_source_map_associations();
    assert_eq!(actual_associations.len(), expected_associations.len());
    for (actual, expected) in actual_associations.iter().zip(expected_associations) {
        assert_eq!(actual.kind(), expected.kind());
        assert_eq!(actual.core_ref(), expected.core_ref());
        assert_eq!(actual.source_ref(), expected.source_ref());
    }

    let expected_provider_associations = checked
        .source_map()
        .entries()
        .iter()
        .filter(|entry| is_provider_effect_lowering(entry.kind()))
        .collect::<Vec<_>>();
    let actual_provider_associations = coverage.provider_source_map_associations();
    assert_eq!(actual_provider_associations.len(), 4);
    assert_eq!(
        actual_provider_associations.len(),
        expected_provider_associations.len()
    );
    for (actual, expected) in actual_provider_associations
        .iter()
        .zip(expected_provider_associations)
    {
        assert_eq!(actual.kind(), expected.kind());
        assert_eq!(actual.core_ref(), expected.core_ref());
        assert_eq!(actual.source_ref(), expected.source_ref());
    }

    let checked_effect = checked
        .read_only_provider_effect("sample_value")
        .expect("the ordinary fixture retains its checked provider operation");
    let checked_core = checked_effect
        .read_only_provider_effect_core()
        .expect("the provider operation retains its checked Core");
    let actual_projection = static_projection.projection();
    let actual_fragments = actual_projection.sys4_artifact_fragments();
    let provider_fragments = actual_fragments
        .entries()
        .iter()
        .filter(|fragment| is_provider_effect_fragment(fragment.fragment_kind()))
        .collect::<Vec<_>>();
    assert_eq!(provider_fragments.len(), 3);

    let requester = provider_fragments
        .iter()
        .copied()
        .find(|fragment| {
            fragment.fragment_kind()
                == ProjectedOperationFragmentKind::ReadOnlyProviderEffectRequester
        })
        .expect("the actual projection retains the provider requester fragment");
    let service = provider_fragments
        .iter()
        .copied()
        .find(|fragment| {
            fragment.fragment_kind()
                == ProjectedOperationFragmentKind::ReadOnlyProviderEffectService
        })
        .expect("the actual projection retains the provider service fragment");
    let result_consumer = provider_fragments
        .iter()
        .copied()
        .find(|fragment| {
            fragment.fragment_kind()
                == ProjectedOperationFragmentKind::ReadOnlyProviderEffectResultConsumer
        })
        .expect("the actual projection retains the provider result-consumer fragment");
    for fragment in [requester, service, result_consumer] {
        assert_eq!(fragment.operation_id(), checked_core.operation());
        assert_eq!(fragment.source_ref(), checked_core.source_ref());
        assert_eq!(
            fragment.read_only_provider_effect_core(),
            Some(checked_core)
        );
    }
    assert_eq!(
        requester.locus_tag().as_str(),
        checked_core.requester_locus()
    );
    assert_eq!(service.locus_tag().as_str(), checked_core.executor_locus());
    assert_eq!(
        result_consumer.locus_tag().as_str(),
        checked_core.result_consumer_locus()
    );
    assert_ne!(
        requester.fragment_ref(),
        result_consumer.fragment_ref(),
        "requester and result-consumer remain distinct actual fragments even when they share ParticipantA"
    );

    let provider_edges = actual_projection
        .communication_plan()
        .edges()
        .iter()
        .filter(|edge| is_provider_effect_edge(edge.kind()))
        .collect::<Vec<_>>();
    assert_eq!(provider_edges.len(), 2);
    let request_edge = provider_edges
        .iter()
        .copied()
        .find(|edge| edge.kind() == CommunicationEdgeKind::ReadOnlyProviderEffectRequest)
        .expect("the actual projection retains the provider request edge");
    let result_edge = provider_edges
        .iter()
        .copied()
        .find(|edge| edge.kind() == CommunicationEdgeKind::ReadOnlyProviderEffectResult)
        .expect("the actual projection retains the provider result edge");
    for edge in [request_edge, result_edge] {
        assert_eq!(edge.operation_id(), checked_core.operation());
        assert_eq!(edge.source_ref(), checked_core.source_ref());
        assert_eq!(
            edge.checked_core_identity().source_ref(),
            checked_core.source_ref()
        );
        assert!(edge.is_derived_from_checked_core());
        assert!(!edge.transfers_authority());
        assert!(!edge.edge_ref().is_empty());
        assert_provider_edge_static_only(edge);
    }
    assert_eq!(request_edge.source_locus(), checked_core.requester_locus());
    assert_eq!(request_edge.target_locus(), checked_core.executor_locus());
    assert_eq!(result_edge.source_locus(), checked_core.executor_locus());
    assert_eq!(
        result_edge.target_locus(),
        checked_core.result_consumer_locus()
    );

    let legacy_checked = checked_legacy_control_source();
    let legacy_topology = provider_effect_topology(&legacy_checked);
    let legacy_projection = project_checked_core(&legacy_checked, &legacy_topology)
        .expect("the unchanged I2 control projects through generic SYS-3");
    assert_eq!(
        legacy_fragment_inventory(actual_projection),
        legacy_fragment_inventory(&legacy_projection),
        "provider static lowering retains the entire existing legacy fragment inventory"
    );
    assert_eq!(
        legacy_edge_inventory(actual_projection),
        legacy_edge_inventory(&legacy_projection),
        "provider static lowering retains the entire existing legacy generated-edge inventory"
    );

    let generic_rejection = project_checked_core(&checked, &topology)
        .expect_err("generic SYS-3 projection remains closed to provider-bearing sources");
    assert_eq!(
        generic_rejection.primary().kind(),
        ProjectionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile
    );
    assert!(generic_rejection.partial_result().is_none());
}

#[test]
fn i3_provider_effect_static_projection_rejects_same_count_edge_association_snapshot() {
    let checked = checked_provider_effect_source();
    let mut serialized = provider_static_snapshot_json(&checked);
    let original_target_ref = {
        let edges = provider_snapshot_edges_mut(&mut serialized);
        assert_eq!(
            edges
                .iter()
                .filter(|edge| is_provider_effect_snapshot_edge(edge))
                .count(),
            2,
            "the canonical static snapshot contains exactly the two actual provider edges alongside retained legacy edges"
        );
        let request_target_ref = edges
            .iter()
            .find(|edge| {
                edge.get("kind").and_then(Value::as_str)
                    == Some("read_only_provider_effect_request")
            })
            .and_then(|edge| edge.get("target_fragment_ref"))
            .and_then(Value::as_str)
            .expect("canonical request edge retains its service fragment association")
            .to_string();
        let result_target_ref = edges
            .iter()
            .find(|edge| {
                edge.get("kind").and_then(Value::as_str) == Some("read_only_provider_effect_result")
            })
            .and_then(|edge| edge.get("target_fragment_ref"))
            .and_then(Value::as_str)
            .expect("canonical result edge retains its consumer fragment association")
            .to_string();
        assert_ne!(
            request_target_ref, result_target_ref,
            "the two canonical edges join distinct actual target fragments"
        );
        let request = edges
            .iter_mut()
            .find(|edge| {
                edge.get("kind").and_then(Value::as_str)
                    == Some("read_only_provider_effect_request")
            })
            .and_then(Value::as_object_mut)
            .expect("canonical snapshot retains a mutable request edge record");
        request.insert(
            "target_fragment_ref".to_string(),
            Value::String(result_target_ref),
        );
        request_target_ref
    };
    let replaced_target_ref = provider_snapshot_edges_mut(&mut serialized)
        .iter()
        .find(|edge| {
            edge.get("kind").and_then(Value::as_str) == Some("read_only_provider_effect_request")
        })
        .and_then(|edge| edge.get("target_fragment_ref"))
        .and_then(Value::as_str)
        .expect("the scoped falsifier retains a syntactically valid request edge")
        .to_string();
    assert_ne!(
        replaced_target_ref, original_target_ref,
        "the falsifier changes only the request edge's actual fragment association"
    );
    assert_eq!(
        provider_snapshot_edges_mut(&mut serialized)
            .iter()
            .filter(|edge| is_provider_effect_snapshot_edge(edge))
            .count(),
        2,
        "the same-count falsifier retains both canonical provider edge rows"
    );

    assert!(matches!(
        restore_provider_static_snapshot(serialized, &checked),
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "provider static snapshot must retain exact generated projection associations"
        })
    ));
}

#[test]
fn i3_provider_effect_static_projection_rejects_tampered_static_coverage_snapshot() {
    let checked = checked_provider_effect_source();
    let canonical = canonical_static_provider_projection(&checked);
    let mut serialized = serde_json::to_value(
        canonical
            .to_i3_private_static_snapshot()
            .expect("canonical provider static projection exports its private snapshot"),
    )
    .expect("canonical provider static snapshot serializes");
    let restored = restore_provider_static_snapshot(serialized.clone(), &checked)
        .expect("the untampered provider static snapshot restores to its canonical projection");
    assert_eq!(restored, canonical);

    let requester_locus = serialized
        .pointer("/provider_coverage/requester_locus")
        .and_then(Value::as_str)
        .expect("canonical provider coverage retains its requester locus")
        .to_string();
    let executor_locus = serialized
        .pointer("/provider_coverage/executor_locus")
        .and_then(Value::as_str)
        .expect("canonical provider coverage retains its executor locus")
        .to_string();
    assert_ne!(
        requester_locus, executor_locus,
        "the canonical provider contract keeps requester and executor at distinct loci"
    );
    let executor = serialized
        .pointer_mut("/provider_coverage/executor_locus")
        .expect("canonical provider coverage exposes the scoped executor-locus field");
    *executor = Value::String(requester_locus);

    assert!(matches!(
        restore_provider_static_snapshot(serialized, &checked),
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "provider static snapshot coverage must match the full checked source"
        })
    ));
}

#[test]
fn i3_provider_effect_static_projection_rejects_omitted_or_duplicated_provider_snapshot_entries() {
    let checked = checked_provider_effect_source();
    let canonical = provider_static_snapshot_json(&checked);

    let mut omitted_result_consumer = canonical.clone();
    let operations: &mut Vec<Value> =
        provider_snapshot_fragment_operations_mut(&mut omitted_result_consumer, "ParticipantA");
    let before_omission = operations.len();
    let result_consumer_index = operations
        .iter()
        .position(|fragment: &Value| {
            fragment.get("kind").and_then(Value::as_str)
                == Some("read_only_provider_effect_result_consumer")
        })
        .expect("canonical static snapshot retains the actual result-consumer fragment");
    operations.remove(result_consumer_index);
    assert_eq!(
        operations.len(),
        before_omission - 1,
        "the omission falsifier removes only the canonical result-consumer fragment"
    );
    assert!(matches!(
        restore_provider_static_snapshot(omitted_result_consumer, &checked),
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "provider static snapshot must retain exact generated projection associations"
        })
    ));

    let mut duplicated_request_edge = canonical;
    let edges = provider_snapshot_edges_mut(&mut duplicated_request_edge);
    let request_edge = edges
        .iter()
        .find(|edge| {
            edge.get("kind").and_then(Value::as_str) == Some("read_only_provider_effect_request")
        })
        .expect("canonical static snapshot retains its actual request edge")
        .clone();
    let before_duplication = edges.len();
    let provider_edges_before_duplication = edges
        .iter()
        .filter(|edge| is_provider_effect_snapshot_edge(edge))
        .count();
    edges.push(request_edge);
    assert_eq!(
        edges.len(),
        before_duplication + 1,
        "the duplicate falsifier adds only a second copy of an actual generated request edge"
    );
    assert_eq!(
        edges
            .iter()
            .filter(|edge| is_provider_effect_snapshot_edge(edge))
            .count(),
        provider_edges_before_duplication + 1,
        "the duplicate falsifier adds one provider edge without changing legacy rows"
    );
    assert!(matches!(
        restore_provider_static_snapshot(duplicated_request_edge, &checked),
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "provider static snapshot must retain exact generated projection associations"
        })
    ));
}

#[test]
fn i3_provider_effect_static_projection_rejects_ordinary_executable_snapshot_export_and_restore() {
    let checked = checked_provider_effect_source();
    let static_projection = canonical_static_provider_projection(&checked);
    assert!(matches!(
        static_projection.projection().to_i3_private_snapshot(),
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
            kind: "read-only provider static projection requires its dedicated snapshot"
        })
    ));

    let static_snapshot = static_projection
        .to_i3_private_static_snapshot()
        .expect("the source-derived static projection exports only through its dedicated snapshot");
    let static_snapshot_json =
        serde_json::to_value(static_snapshot).expect("dedicated static snapshot serializes");
    let ordinary_snapshot = ordinary_snapshot_from_static_snapshot_json(&static_snapshot_json);
    assert!(matches!(
        GlobalProjectionResult::from_i3_private_snapshot(ordinary_snapshot),
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
            kind: "read-only provider static projection requires its dedicated snapshot"
        })
    ));
}

#[test]
fn i3_provider_effect_static_projection_rejects_provider_details_hidden_as_legacy_in_ordinary_snapshot()
 {
    let checked = checked_provider_effect_source();
    let mut static_snapshot_json = provider_static_snapshot_json(&checked);

    let legacy_fragment_kind = static_snapshot_json
        .pointer("/projection/locus_programs")
        .and_then(Value::as_array)
        .expect("canonical static snapshot retains its actual locus programs")
        .iter()
        .filter_map(|entry| entry.get("value"))
        .filter_map(Value::as_object)
        .filter_map(|program| program.get("operations"))
        .filter_map(Value::as_array)
        .flatten()
        .find(|fragment| !is_provider_effect_snapshot_fragment(fragment))
        .and_then(|fragment| fragment.get("kind"))
        .and_then(Value::as_str)
        .expect("the mixed source retains at least one canonical legacy fragment kind")
        .to_string();
    let legacy_edge_kind = static_snapshot_json
        .pointer("/projection/communication_plan/edges")
        .and_then(Value::as_array)
        .expect("canonical static snapshot retains its actual communication-plan edges")
        .iter()
        .find(|edge| {
            !is_provider_effect_snapshot_edge(edge)
                && edge.get("kind").and_then(Value::as_str) != Some("absolute_value_stream")
        })
        .and_then(|edge| edge.get("kind"))
        .and_then(Value::as_str)
        .expect("the mixed source retains at least one canonical legacy edge kind")
        .to_string();

    let mut hidden_fragment_count = 0;
    for locus_program in static_snapshot_json
        .pointer_mut("/projection/locus_programs")
        .and_then(Value::as_array_mut)
        .expect("canonical static snapshot retains mutable actual locus programs")
    {
        for fragment in locus_program
            .get_mut("value")
            .and_then(Value::as_object_mut)
            .and_then(|program| program.get_mut("operations"))
            .and_then(Value::as_array_mut)
            .expect("every canonical locus program retains its operation vector")
        {
            if is_provider_effect_snapshot_fragment(fragment) {
                fragment
                    .as_object_mut()
                    .expect("canonical fragment is an object")
                    .insert(
                        "kind".to_string(),
                        Value::String(legacy_fragment_kind.clone()),
                    );
                hidden_fragment_count += 1;
            }
        }
    }
    let mut hidden_edge_count = 0;
    for edge in provider_snapshot_edges_mut(&mut static_snapshot_json) {
        if is_provider_effect_snapshot_edge(edge) {
            edge.as_object_mut()
                .expect("canonical edge is an object")
                .insert("kind".to_string(), Value::String(legacy_edge_kind.clone()));
            hidden_edge_count += 1;
        }
    }
    assert_eq!(
        hidden_fragment_count, 3,
        "the falsifier conceals all three actual provider fragment discriminants while retaining their placements"
    );
    assert_eq!(
        hidden_edge_count, 2,
        "the falsifier conceals both actual provider edge discriminants while retaining their carrier details"
    );

    let ordinary_snapshot = ordinary_snapshot_from_static_snapshot_json(&static_snapshot_json);
    assert!(matches!(
        GlobalProjectionResult::from_i3_private_snapshot(ordinary_snapshot),
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
            kind: "read-only provider static projection requires its dedicated snapshot"
        })
    ));
}

#[test]
fn i3_ordinary_snapshot_rejects_provider_effect_row_hidden_in_legacy_owner_request() {
    let (canonical, mut serialized) = canonical_legacy_projection_snapshot_json();
    assert_eq!(
        restore_ordinary_projection_snapshot(serialized.clone())
            .expect("the unmodified genuine legacy snapshot restores positively"),
        canonical
    );

    let owner_request = ordinary_snapshot_owner_request_edge_mut(&mut serialized);
    assert_eq!(
        owner_request.get("kind").and_then(Value::as_str),
        Some("owner_request"),
        "the outer communication edge remains an ordinary owner request"
    );
    let carrier = owner_request
        .get_mut("carrier_contract")
        .and_then(Value::as_object_mut)
        .expect("ordinary owner-request snapshot retains a mutable carrier contract");
    assert_eq!(
        carrier.get("edge_kind").and_then(Value::as_str),
        Some("owner_request")
    );
    assert_eq!(
        carrier.get("lifecycle_kind").and_then(Value::as_str),
        Some("owner_request")
    );
    assert!(
        !carrier.contains_key("read_only_provider_effect_details"),
        "the legacy carrier starts without provider detail payload"
    );
    assert_eq!(carrier.get("designated_result_details"), Some(&Value::Null));
    let effect_row = carrier
        .get_mut("effect_row")
        .and_then(Value::as_array_mut)
        .expect("ordinary owner-request carrier retains its canonical effect row");
    assert!(
        !effect_row
            .iter()
            .any(|kind| { kind.as_str() == Some("read_only_provider_effect_invocation") }),
        "the genuine legacy carrier starts without the provider invocation effect"
    );
    effect_row.push(Value::String(
        "read_only_provider_effect_invocation".to_string(),
    ));

    assert_ordinary_provider_snapshot_rejected(
        restore_ordinary_projection_snapshot(serialized),
        "provider invocation effect hidden in legacy owner-request carrier",
    );
}

#[test]
fn i3_ordinary_snapshot_rejects_provider_fragment_identity_hidden_in_legacy_owner_request() {
    let (canonical, mut serialized) = canonical_legacy_projection_snapshot_json();
    assert_eq!(
        restore_ordinary_projection_snapshot(serialized.clone())
            .expect("the unmodified genuine legacy snapshot restores positively"),
        canonical
    );

    let owner_request = ordinary_snapshot_owner_request_fragment_mut(&mut serialized);
    assert_eq!(
        owner_request.get("kind").and_then(Value::as_str),
        Some("owner_request_invocation"),
        "the outer fragment remains an ordinary owner-request invocation"
    );
    assert_eq!(
        owner_request
            .get("placement")
            .and_then(Value::as_object)
            .and_then(|placement| placement.get("kind"))
            .and_then(Value::as_str),
        Some("owner_request"),
        "the owner-request placement remains unchanged"
    );
    let identity = owner_request
        .get_mut("checked_core_identity")
        .and_then(Value::as_object_mut)
        .expect("ordinary owner-request fragment retains its checked Core identity");
    assert_eq!(
        identity.get("fragment_kind").and_then(Value::as_str),
        Some("owner_request_invocation")
    );
    assert_eq!(identity.get("edge_kind"), Some(&Value::Null));
    identity.insert(
        "fragment_kind".to_string(),
        Value::String("read_only_provider_effect_service".to_string()),
    );

    assert_ordinary_provider_snapshot_rejected(
        restore_ordinary_projection_snapshot(serialized),
        "provider fragment identity hidden in legacy owner-request fragment",
    );
}

type LegacySnapshotMutation = fn(&mut Value, &Value);

struct LegacyTypedMarkerMutation {
    name: &'static str,
    mutate: LegacySnapshotMutation,
}

fn mutate_legacy_carrier_lifecycle(snapshot: &mut Value, provider_static: &Value) {
    let lifecycle = json_value_at(
        provider_static_request_edge(provider_static),
        "/carrier_contract/lifecycle_kind",
        "provider request lifecycle marker",
    );
    let carrier = ordinary_snapshot_owner_request_edge_mut(snapshot)
        .get_mut("carrier_contract")
        .and_then(Value::as_object_mut)
        .expect("ordinary owner request retains a mutable carrier contract");
    carrier.insert("lifecycle_kind".to_string(), lifecycle);
}

fn mutate_legacy_carrier_edge_kind(snapshot: &mut Value, provider_static: &Value) {
    let edge_kind = json_value_at(
        provider_static_request_edge(provider_static),
        "/carrier_contract/edge_kind",
        "provider request carrier edge-kind marker",
    );
    let carrier = ordinary_snapshot_owner_request_edge_mut(snapshot)
        .get_mut("carrier_contract")
        .and_then(Value::as_object_mut)
        .expect("ordinary owner request retains a mutable carrier contract");
    carrier.insert("edge_kind".to_string(), edge_kind);
}

fn mutate_legacy_carrier_seam_requirements(snapshot: &mut Value, provider_static: &Value) {
    let rows = json_value_at(
        provider_static_request_edge(provider_static),
        "/carrier_contract/authority_requirements/requirements/rows",
        "provider request membership/capability/witness seam rows",
    );
    let carrier = ordinary_snapshot_owner_request_edge_mut(snapshot)
        .get_mut("carrier_contract")
        .and_then(Value::as_object_mut)
        .expect("ordinary owner request retains a mutable carrier contract");
    carrier
        .get_mut("authority_requirements")
        .and_then(Value::as_object_mut)
        .and_then(|authority| authority.get_mut("requirements"))
        .and_then(Value::as_object_mut)
        .expect("ordinary owner request retains runtime seam requirements")
        .insert("rows".to_string(), rows);
}

fn mutate_legacy_fragment_semantic_obligations(snapshot: &mut Value, provider_static: &Value) {
    let obligations = json_value_at(
        provider_static_service_fragment(provider_static),
        "/semantic_obligations",
        "provider service semantic obligations",
    );
    ordinary_snapshot_owner_request_fragment_mut(snapshot)
        .insert("semantic_obligations".to_string(), obligations);
}

fn mutate_legacy_fragment_runtime_requirements(snapshot: &mut Value, provider_static: &Value) {
    let requirements = json_value_at(
        provider_static_service_fragment(provider_static),
        "/runtime_seam_requirements",
        "provider service runtime seam requirements",
    );
    ordinary_snapshot_owner_request_fragment_mut(snapshot)
        .insert("runtime_seam_requirements".to_string(), requirements);
}

fn mutate_legacy_fragment_authority_requirements(snapshot: &mut Value, provider_static: &Value) {
    let requirements = json_value_at(
        provider_static_service_fragment(provider_static),
        "/authority_requirements",
        "provider service authority requirements",
    );
    ordinary_snapshot_owner_request_fragment_mut(snapshot)
        .insert("authority_requirements".to_string(), requirements);
}

fn mutate_legacy_effect_handler_effect(snapshot: &mut Value, provider_static: &Value) {
    let effect = provider_static_invocation_effect(provider_static);
    let effect_row = ordinary_snapshot_handler_with_fragment_identity_mut(snapshot)
        .get_mut("effect_row")
        .and_then(Value::as_array_mut)
        .expect("ordinary effect handler retains its canonical effect row");
    effect_row.push(effect);
}

fn mutate_legacy_effect_handler_identity(snapshot: &mut Value, provider_static: &Value) {
    let fragment_kind = json_value_at(
        provider_static_service_fragment(provider_static),
        "/checked_core_identity/fragment_kind",
        "provider service fragment identity marker",
    );
    ordinary_snapshot_handler_with_fragment_identity_mut(snapshot)
        .get_mut("checked_core_identity")
        .and_then(Value::as_object_mut)
        .expect("ordinary effect handler retains its checked Core identity")
        .insert("fragment_kind".to_string(), fragment_kind);
}

fn mutate_legacy_projected_source_map_identity(snapshot: &mut Value, provider_static: &Value) {
    let fragment_kind = json_value_at(
        provider_static_service_fragment(provider_static),
        "/checked_core_identity/fragment_kind",
        "provider service fragment identity marker",
    );
    ordinary_snapshot_source_map_fragment_identity_mut(snapshot)
        .get_mut("checked_core_identity")
        .and_then(Value::as_object_mut)
        .expect("ordinary projected-source entry retains its checked Core identity")
        .insert("fragment_kind".to_string(), fragment_kind);
}

fn mutate_legacy_observation_edge_kind(snapshot: &mut Value, provider_static: &Value) {
    let edge_kind = json_value_at(
        provider_static_request_edge(provider_static),
        "/kind",
        "provider request edge marker",
    );
    ordinary_snapshot_observation_edge_identity_mut(snapshot).insert("kind".to_string(), edge_kind);
}

fn mutate_legacy_relation_runtime_residual(snapshot: &mut Value, provider_static: &Value) {
    let residual_kind = json_value_at(
        provider_static,
        "/provider_coverage/runtime_requirement/kind",
        "provider runtime residual marker",
    );
    ordinary_snapshot_relation_residual_mut(snapshot).insert("kind".to_string(), residual_kind);
}

#[test]
fn i3_ordinary_snapshot_rejects_every_remaining_provider_typed_marker_category() {
    let (canonical_legacy, legacy_snapshot) = canonical_legacy_projection_snapshot_json();
    assert_eq!(
        restore_ordinary_projection_snapshot(legacy_snapshot.clone())
            .expect("the unmodified genuine legacy snapshot restores positively"),
        canonical_legacy
    );
    let provider_static = canonical_provider_static_snapshot_json();
    let cases = [
        LegacyTypedMarkerMutation {
            name: "carrier lifecycle",
            mutate: mutate_legacy_carrier_lifecycle,
        },
        LegacyTypedMarkerMutation {
            name: "carrier edge kind",
            mutate: mutate_legacy_carrier_edge_kind,
        },
        LegacyTypedMarkerMutation {
            name: "carrier membership/capability/witness seam rows",
            mutate: mutate_legacy_carrier_seam_requirements,
        },
        LegacyTypedMarkerMutation {
            name: "fragment semantic obligations",
            mutate: mutate_legacy_fragment_semantic_obligations,
        },
        LegacyTypedMarkerMutation {
            name: "fragment runtime seam requirements",
            mutate: mutate_legacy_fragment_runtime_requirements,
        },
        LegacyTypedMarkerMutation {
            name: "fragment authority requirements",
            mutate: mutate_legacy_fragment_authority_requirements,
        },
        LegacyTypedMarkerMutation {
            name: "effect-handler effect row",
            mutate: mutate_legacy_effect_handler_effect,
        },
        LegacyTypedMarkerMutation {
            name: "effect-handler checked Core identity",
            mutate: mutate_legacy_effect_handler_identity,
        },
        LegacyTypedMarkerMutation {
            name: "projected-source-map checked Core identity",
            mutate: mutate_legacy_projected_source_map_identity,
        },
        LegacyTypedMarkerMutation {
            name: "observation edge identity kind",
            mutate: mutate_legacy_observation_edge_kind,
        },
        LegacyTypedMarkerMutation {
            name: "relation runtime residual",
            mutate: mutate_legacy_relation_runtime_residual,
        },
    ];

    for case in cases {
        let mut candidate = legacy_snapshot.clone();
        (case.mutate)(&mut candidate, &provider_static);
        assert_ordinary_provider_snapshot_rejected(
            restore_ordinary_projection_snapshot(candidate),
            case.name,
        );
    }
}

#[test]
fn i3_provider_effect_marker_cannot_enter_adapter_static_facts_or_sys4_preflight() {
    let (mut legacy_projection, _) = canonical_legacy_projection_snapshot_json();
    let (original_static_facts, original_fingerprint, static_facts_after, fingerprint_after) = {
        let edge = legacy_projection
            .communication_plan_mut()
            .edges
            .iter_mut()
            .find(|edge| edge.kind == CommunicationEdgeKind::OwnerRequest)
            .expect("genuine legacy projection retains an ordinary owner-request carrier");
        let carrier = &mut edge.carrier_contract;
        let original_static_facts = carrier.i3_adapter_static_facts().is_some();
        let original_fingerprint = carrier
            .i3_probe_owner_request_fingerprint_component()
            .is_some();
        carrier
            .effect_row
            .kinds
            .push(EffectKind::ReadOnlyProviderEffectInvocation);
        (
            original_static_facts,
            original_fingerprint,
            carrier.i3_adapter_static_facts().is_some(),
            carrier
                .i3_probe_owner_request_fingerprint_component()
                .is_some(),
        )
    };
    assert!(
        original_static_facts && original_fingerprint,
        "the genuine unchanged owner carrier remains eligible for its existing adapter/fingerprint paths"
    );
    let sys4_preflight = FabricProgram::from_projection(legacy_projection);
    match (static_facts_after, fingerprint_after, sys4_preflight) {
        (false, false, Err(diagnostics))
            if diagnostics.primary().kind() == Sys4DiagnosticKind::ProgramProjectionMismatch => {}
        (true, _, _) => {
            panic!("provider marker unexpectedly retained ordinary I3 adapter static facts")
        }
        (_, true, _) => {
            panic!("provider marker unexpectedly retained an owner-request fingerprint component")
        }
        (false, false, Ok(_)) => {
            panic!("provider marker unexpectedly passed SYS-4 executable preflight")
        }
        (false, false, Err(diagnostics)) => {
            panic!(
                "provider marker reached SYS-4 with wrong diagnostic {:?}",
                diagnostics.primary().kind()
            )
        }
    }
}

#[test]
fn i3_provider_effect_viewer_restriction_keeps_full_source_profile_out_of_export_and_sys4() {
    let checked = checked_provider_effect_source();
    let static_projection = canonical_static_provider_projection(&checked);
    let full_projection = static_projection.projection();
    let viewer_only = full_projection.restricted_to_loci(&BTreeSet::from(["ViewerC".to_string()]));

    assert_eq!(
        viewer_only.checked_program_identity(),
        full_projection.checked_program_identity(),
        "restriction retains the full checked source identity rather than inventing a viewer-only program"
    );
    assert_eq!(
        viewer_only.checked_program_identity(),
        checked.program_identity(),
        "the retained identity stays bound to the ordinary provider-effect source"
    );
    assert!(
        !viewer_only
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .any(|fragment| is_provider_effect_fragment(fragment.fragment_kind())),
        "the ViewerC cut deliberately omits all provider-local static fragments"
    );
    assert!(
        !viewer_only
            .communication_plan()
            .edges()
            .iter()
            .any(|edge| is_provider_effect_edge(edge.kind())),
        "the ViewerC cut deliberately omits both provider request/result edges"
    );

    match viewer_only.to_i3_private_snapshot() {
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
            kind: ORDINARY_PROVIDER_SNAPSHOT_REJECTION,
        }) => {}
        Ok(_) => {
            panic!("viewer-only provider cut unexpectedly exported an ordinary executable snapshot")
        }
        Err(I3PrivateProjectionSnapshotError::UnsupportedVariant { kind }) => {
            panic!("viewer-only provider cut used unexpected unsupported variant {kind}")
        }
        Err(I3PrivateProjectionSnapshotError::StructuralMismatch { reason }) => {
            panic!("viewer-only provider cut rejected at unrelated structural validation: {reason}")
        }
        Err(I3PrivateProjectionSnapshotError::UnsupportedVersion { .. })
        | Err(I3PrivateProjectionSnapshotError::DuplicateMapKey { .. })
        | Err(I3PrivateProjectionSnapshotError::DuplicateSetMember { .. })
        | Err(I3PrivateProjectionSnapshotError::SemanticSnapshot) => {
            panic!("viewer-only provider cut rejected at unrelated ordinary snapshot validation")
        }
    }
    match FabricProgram::from_projection(viewer_only) {
        Err(diagnostics)
            if diagnostics.primary().kind() == Sys4DiagnosticKind::ProgramProjectionMismatch => {}
        Ok(_) => panic!("viewer-only provider cut unexpectedly passed SYS-4 executable preflight"),
        Err(diagnostics) => panic!(
            "viewer-only provider cut reached SYS-4 with wrong diagnostic {:?}",
            diagnostics.primary().kind()
        ),
    }
}
