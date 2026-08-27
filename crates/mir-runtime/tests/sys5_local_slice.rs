use mir_runtime::sys5_local_slice::{Sys5LocalSliceError, Sys5SourceInput, build_project};

const SYS5_LOCAL_TOY_PATH: &str = "tests/inline/sys5_local_toy_surface_v0.mir";

macro_rules! assert_summary_row {
    ($rows:expr, $locus:expr, $kind:expr, $operation_id:expr $(,)?) => {
        assert!(
            $rows.iter().any(|row| row.locus == $locus
                && row.kind == $kind
                && row.operation_id == $operation_id
                && row.derived_from_checked_core),
            "missing checked-Core-derived artifact summary row: locus={}, kind={}, operation_id={}",
            $locus,
            $kind,
            $operation_id
        );
    };
}

macro_rules! assert_summary_edge {
    ($rows:expr, $kind:expr, $from:expr, $to:expr, $operation_id:expr $(,)?) => {
        assert!(
            $rows.iter().any(|row| row.kind == $kind
                && row.from_locus == $from
                && row.to_locus == $to
                && row.operation_id == $operation_id
                && row.derived_from_checked_core
                && !row.transfers_authority),
            "missing checked-Core-derived generated communication edge: {}->{} {} {}",
            $from,
            $to,
            $kind,
            $operation_id
        );
    };
}

macro_rules! assert_nonzero_source_span {
    ($span:expr, $label:expr) => {
        assert!(
            $span.start_line > 0,
            "{} start_line must be nonzero",
            $label
        );
        assert!(
            $span.start_column > 0,
            "{} start_column must be nonzero",
            $label
        );
        assert!($span.end_line > 0, "{} end_line must be nonzero", $label);
        assert!(
            $span.end_column > 0,
            "{} end_column must be nonzero",
            $label
        );
        assert!(
            ($span.end_line, $span.end_column) >= ($span.start_line, $span.start_column),
            "{} span end must not precede start",
            $label
        );
    };
}

macro_rules! assert_complete_artifact_provenance {
    ($rows:expr, $locus:expr, $kind:expr, $operation_id:expr, $source_path:expr $(,)?) => {{
        let row = $rows
            .iter()
            .find(|row| {
                row.locus == $locus && row.kind == $kind && row.operation_id == $operation_id
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing artifact provenance row: locus={}, kind={}, operation_id={}",
                    $locus, $kind, $operation_id
                )
            });
        assert_eq!(row.source_path, $source_path);
        assert_nonzero_source_span!(row.source_span, "artifact source span");
        assert!(
            !row.core_ref.is_empty(),
            "artifact core_ref must be present"
        );
        assert!(
            !row.fragment_ref.is_empty(),
            "artifact fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "artifact checked_program_identity must be present"
        );
        row
    }};
}

macro_rules! assert_complete_mapping {
    ($rows:expr, $operation_id:expr, $core_kind:expr, $artifact_locus:expr, $artifact_kind:expr, $source_path:expr $(,)?) => {{
        let row = $rows
            .iter()
            .find(|row| {
                row.operation_id == $operation_id
                    && row.core_kind == $core_kind
                    && row.artifact_locus == $artifact_locus
                    && row.artifact_kind == $artifact_kind
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing source/Core/artifact mapping: operation_id={}, core_kind={}, artifact_locus={}, artifact_kind={}",
                    $operation_id, $core_kind, $artifact_locus, $artifact_kind
                )
            });
        assert_eq!(row.source_path, $source_path);
        assert_nonzero_source_span!(row.source_span, "mapping source span");
        assert!(!row.core_ref.is_empty(), "mapping core_ref must be present");
        assert!(
            !row.fragment_ref.is_empty(),
            "mapping fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "mapping checked_program_identity must be present"
        );
        row
    }};
}

macro_rules! assert_complete_communication_provenance {
    ($rows:expr, $kind:expr, $from:expr, $to:expr, $operation_id:expr, $source_path:expr $(,)?) => {{
        let row = $rows
            .iter()
            .find(|row| {
                row.kind == $kind
                    && row.from_locus == $from
                    && row.to_locus == $to
                    && row.operation_id == $operation_id
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing generated communication provenance: {}->{} {} {}",
                    $from, $to, $kind, $operation_id
                )
            });
        assert_eq!(row.source_path, $source_path);
        assert_nonzero_source_span!(row.source_span, "communication source span");
        assert!(
            !row.edge_ref.is_empty(),
            "communication edge_ref must be present"
        );
        assert!(
            row.core_ref
                .as_ref()
                .is_some_and(|core_ref| !core_ref.is_empty()),
            "checked-Core-derived communication core_ref must be present"
        );
        assert!(
            !row.source_fragment_ref.is_empty(),
            "communication source_fragment_ref must be present"
        );
        assert!(
            !row.target_fragment_ref.is_empty(),
            "communication target_fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "communication checked_program_identity must be present"
        );
        row
    }};
}

const SYS5_LOCAL_TOY_SOURCE: &str = r#"
module Mirrorea.Sys5.LocalToy

locus WorldAuthority
locus ParticipantA
locus ParticipantB
locus ViewerC
principal self
principal target
type Player
type Bird

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
  visible observer_safe fields (hp)
}

state participant_input[id: Player] at ParticipantA {
  focus: Int
  visible observer_safe fields (focus)
}

state bird_pose[id: Bird] at ParticipantB {
  x: Int
  y: Int
  visible observer_safe fields (x, y)
}

Role[self] at ParticipantA {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }
}

relation bird_follow at ParticipantB {
  subject bird: Bird
  primary participant_a_shoulder epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

const SYS5_TWO_OWNER_RMW_SOURCE: &str = r#"
module Mirrorea.Sys5.TwoOwnerRmwSameOwner

locus WorldAuthority
locus ParticipantA
principal self
principal target
type Player

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
  visible observer_safe fields (hp)
}

Role[self] at ParticipantA {
  when strike(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }

  when heal(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp + avatar[self].atk
    }
  }
}
"#;

const SYS5_PRIVATE_FIELD_SOURCE: &str = r#"
module Mirrorea.Sys5.PrivateFieldRedaction

locus WorldAuthority
locus ParticipantA
locus ParticipantB
locus ViewerC
principal self
principal target
type Player
type Bird

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
  private_secret_field: Int
  visible observer_safe fields (hp)
}

state participant_input[id: Player] at ParticipantA {
  focus: Int
  visible observer_safe fields (focus)
}

state bird_pose[id: Bird] at ParticipantB {
  x: Int
  y: Int
  visible observer_safe fields (x, y)
}

Role[self] at ParticipantA {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }
}

relation bird_follow at ParticipantB {
  subject bird: Bird
  primary participant_a_shoulder epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

#[test]
fn build_project_derives_provisional_observer_safe_four_locus_summary_from_inline_source() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("SYS-5 build/project should parse, check, and project the inline Surface-v0 source");

    let summary = project.semantic_summary();

    assert_eq!(summary.profile_name, "sys5-local-slice");
    assert_eq!(
        summary.profile_status,
        "provisional-no-compatibility-promise"
    );
    assert!(!summary.public_api_or_wire_contract);
    assert!(!summary.requires_runtime_execution);

    assert_eq!(
        summary.loci,
        ["ParticipantA", "ParticipantB", "ViewerC", "WorldAuthority"]
    );
    assert_eq!(summary.loci.len(), 4);

    assert_summary_row!(
        summary.artifacts,
        "ParticipantA",
        "owner-request-invocation",
        "attack",
    );
    assert_summary_row!(
        summary.artifacts,
        "WorldAuthority",
        "owner-rmw-evaluation",
        "attack",
    );
    assert_summary_row!(
        summary.artifacts,
        "ParticipantA",
        "designated-remote-input-service",
        "WorldAuthority.result",
    );
    assert_summary_row!(
        summary.artifacts,
        "WorldAuthority",
        "designated-evaluation",
        "WorldAuthority.result",
    );
    assert_summary_row!(
        summary.artifacts,
        "ViewerC",
        "designated-result-consumer",
        "WorldAuthority.result",
    );
    assert_summary_row!(
        summary.artifacts,
        "ParticipantB",
        "relation-publication",
        "bird_follow",
    );
    assert_summary_row!(
        summary.artifacts,
        "ViewerC",
        "consumer-local-relation-projection",
        "bird_follow",
    );
    assert_complete_artifact_provenance!(
        summary.artifacts,
        "WorldAuthority",
        "owner-rmw-evaluation",
        "attack",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_artifact_provenance!(
        summary.artifacts,
        "ViewerC",
        "designated-result-consumer",
        "WorldAuthority.result",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_artifact_provenance!(
        summary.artifacts,
        "ViewerC",
        "consumer-local-relation-projection",
        "bird_follow",
        SYS5_LOCAL_TOY_PATH,
    );
    for row in &summary.artifacts {
        assert_eq!(row.source_path, SYS5_LOCAL_TOY_PATH);
        assert_nonzero_source_span!(row.source_span, "artifact source span");
        assert!(
            !row.core_ref.is_empty(),
            "artifact core_ref must be present"
        );
        assert!(
            !row.fragment_ref.is_empty(),
            "artifact fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "artifact checked_program_identity must be present"
        );
    }

    assert_summary_edge!(
        summary.generated_communication,
        "owner-request",
        "ParticipantA",
        "WorldAuthority",
        "attack",
    );
    assert_summary_edge!(
        summary.generated_communication,
        "owner-reply-receipt",
        "WorldAuthority",
        "ParticipantA",
        "attack",
    );
    assert_summary_edge!(
        summary.generated_communication,
        "designated-input-request",
        "WorldAuthority",
        "ParticipantA",
        "WorldAuthority.result",
    );
    assert_summary_edge!(
        summary.generated_communication,
        "designated-input-receipt",
        "ParticipantA",
        "WorldAuthority",
        "WorldAuthority.result",
    );
    assert_summary_edge!(
        summary.generated_communication,
        "designated-result-delivery",
        "WorldAuthority",
        "ViewerC",
        "WorldAuthority.result",
    );
    assert_summary_edge!(
        summary.generated_communication,
        "relation-projection-publication",
        "ParticipantB",
        "ViewerC",
        "bird_follow",
    );
    assert_complete_communication_provenance!(
        summary.generated_communication,
        "owner-request",
        "ParticipantA",
        "WorldAuthority",
        "attack",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_communication_provenance!(
        summary.generated_communication,
        "designated-input-request",
        "WorldAuthority",
        "ParticipantA",
        "WorldAuthority.result",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_communication_provenance!(
        summary.generated_communication,
        "designated-result-delivery",
        "WorldAuthority",
        "ViewerC",
        "WorldAuthority.result",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_communication_provenance!(
        summary.generated_communication,
        "relation-projection-publication",
        "ParticipantB",
        "ViewerC",
        "bird_follow",
        SYS5_LOCAL_TOY_PATH,
    );
    for row in &summary.generated_communication {
        assert_eq!(row.source_path, SYS5_LOCAL_TOY_PATH);
        assert_nonzero_source_span!(row.source_span, "communication source span");
        assert!(
            !row.edge_ref.is_empty(),
            "communication edge_ref must be present"
        );
        assert!(
            !row.source_fragment_ref.is_empty(),
            "communication source_fragment_ref must be present"
        );
        assert!(
            !row.target_fragment_ref.is_empty(),
            "communication target_fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "communication checked_program_identity must be present"
        );
        if row.derived_from_checked_core {
            assert!(
                row.core_ref
                    .as_ref()
                    .is_some_and(|core_ref| !core_ref.is_empty()),
                "checked-Core-derived communication core_ref must be present"
            );
        }
    }

    assert_complete_mapping!(
        summary.source_core_artifact_mappings,
        "attack",
        "OwnerRmw",
        "WorldAuthority",
        "owner-rmw-evaluation",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_mapping!(
        summary.source_core_artifact_mappings,
        "WorldAuthority.result",
        "DesignatedResultConsume",
        "ViewerC",
        "designated-result-consumer",
        SYS5_LOCAL_TOY_PATH,
    );
    assert_complete_mapping!(
        summary.source_core_artifact_mappings,
        "bird_follow",
        "MaintainedRelation",
        "ViewerC",
        "consumer-local-relation-projection",
        SYS5_LOCAL_TOY_PATH,
    );
    for row in &summary.source_core_artifact_mappings {
        assert_eq!(row.source_path, SYS5_LOCAL_TOY_PATH);
        assert_nonzero_source_span!(row.source_span, "mapping source span");
        assert!(!row.core_ref.is_empty(), "mapping core_ref must be present");
        assert!(
            !row.fragment_ref.is_empty(),
            "mapping fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "mapping checked_program_identity must be present"
        );
    }

    assert!(
        summary
            .auth_residuals
            .iter()
            .any(|row| row.authority == "MembershipAuth"
                && row.status == "residual"
                && !row.grants_runtime_authority)
    );
    assert!(
        summary
            .verification_residuals
            .iter()
            .any(|row| row.verifier == "finite_refinement"
                && row.status == "residual"
                && row.discharge == "optional")
    );

    assert_eq!(
        summary.observer_safety,
        "observer-safe-no-raw-authority-capability-witness-payload"
    );
    let observer_view =
        serde_json::to_string(&project.observer_safe_view()).expect("observer view serializes");
    assert_contains_all(
        &observer_view,
        &[
            "source:tests/inline/sys5_local_toy_surface_v0.mir",
            "core:OwnerRmw",
            "artifact:WorldAuthority:owner-rmw-evaluation",
            "edge:ParticipantA->WorldAuthority:owner-request",
            "core:DesignatedResultConsume",
            "edge:WorldAuthority->ViewerC:designated-result-delivery",
            "core:MaintainedRelation",
            "edge:ParticipantB->ViewerC:relation-projection-publication",
            "auth:MembershipAuth:residual",
            "verify:finite_refinement:residual",
            "profile:sys5-local-slice:provisional-no-compatibility-promise",
            "core-ref:",
            "artifact-ref:",
            "edge-ref:",
        ],
    );
    assert_contains_none(
        &observer_view,
        &[
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_credential",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "grant_payload",
            "credential_payload",
            "sys4_dispatch",
            "LocalFabric",
            "FabricProgram",
            "Sys4TraceEntry",
        ],
    );
    let summary_json = serde_json::to_string(summary).expect("summary serializes");
    assert_contains_none(
        &summary_json,
        &[
            "sys4_dispatch",
            "LocalFabric",
            "FabricProgram",
            "Sys4TraceEntry",
        ],
    );
}

#[test]
fn build_project_keeps_same_owner_rmw_mapping_identities_distinct_and_complete() {
    let project = build_project(Sys5SourceInput::inline(
        "tests/inline/sys5_two_owner_rmw_same_owner.mir",
        SYS5_TWO_OWNER_RMW_SOURCE,
    ))
    .expect("two owner RMW operations at one owner locus should check and project");
    let summary = project.semantic_summary();

    let owner_mappings = summary
        .source_core_artifact_mappings
        .iter()
        .filter(|row| {
            row.artifact_locus == "WorldAuthority" && row.artifact_kind == "owner-rmw-evaluation"
        })
        .collect::<Vec<_>>();
    assert_eq!(
        owner_mappings.len(),
        2,
        "same-owner RMW projection must retain one complete mapping per operation"
    );

    let strike = owner_mappings
        .iter()
        .copied()
        .find(|row| row.operation_id == "strike")
        .expect("strike owner mapping exists");
    let heal = owner_mappings
        .iter()
        .copied()
        .find(|row| row.operation_id == "heal")
        .expect("heal owner mapping exists");

    for row in [strike, heal] {
        assert_eq!(
            row.source_path,
            "tests/inline/sys5_two_owner_rmw_same_owner.mir"
        );
        assert_eq!(row.core_kind, "OwnerRmw");
        assert_eq!(row.artifact_locus, "WorldAuthority");
        assert_eq!(row.artifact_kind, "owner-rmw-evaluation");
        assert_nonzero_source_span!(row.source_span, "same-owner RMW mapping source span");
        assert!(!row.core_ref.is_empty(), "mapping core_ref must be present");
        assert!(
            !row.fragment_ref.is_empty(),
            "mapping fragment_ref must be present"
        );
        assert!(
            !row.checked_program_identity.is_empty(),
            "mapping checked_program_identity must be present"
        );
    }
    assert_ne!(strike.operation_id, heal.operation_id);
    assert_ne!(
        strike.core_ref, heal.core_ref,
        "distinct owner operations must not collapse to one Core ref"
    );
    assert_ne!(
        strike.fragment_ref, heal.fragment_ref,
        "distinct owner operations must not collapse to one artifact fragment ref"
    );
    assert_eq!(
        strike.checked_program_identity, heal.checked_program_identity,
        "distinct mappings from one source must still bind the same checked program identity"
    );
}

#[test]
fn build_project_rejects_invalid_logical_source_paths_before_surface_checking() {
    build_project(Sys5SourceInput::inline(
        "tests/inline/valid_repo_logical_path.mir",
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("repo-logical relative paths remain valid logical provenance");

    for invalid_path in [
        "/tmp/sys5_local_toy_surface_v0.mir",
        "../sys5_local_toy_surface_v0.mir",
        "tests/../sys5_local_toy_surface_v0.mir",
        r"C:\Users\alice\toy.mir",
        "C:/Users/alice/toy.mir",
        r"\\server\share\toy.mir",
        "//server/share/toy.mir",
        "   ",
    ] {
        match build_project(Sys5SourceInput::inline(invalid_path, SYS5_LOCAL_TOY_SOURCE)) {
            Ok(_) => panic!("invalid logical source path `{invalid_path}` unexpectedly succeeded"),
            Err(err) => assert_eq!(
                err,
                Sys5LocalSliceError::InvalidLogicalSourcePath,
                "unexpected error for invalid logical source path `{invalid_path}`"
            ),
        }
    }
}

#[test]
fn semantic_summary_redacts_private_source_text_payloads_and_opaque_identities() {
    assert!(
        SYS5_PRIVATE_FIELD_SOURCE.contains("private_secret_field"),
        "privacy source must contain the private field falsifier"
    );
    let project = build_project(Sys5SourceInput::inline(
        "tests/inline/sys5_private_field_redaction.mir",
        SYS5_PRIVATE_FIELD_SOURCE,
    ))
    .expect("privacy source should check and project before redaction assertions");
    let summary = project.semantic_summary();

    let summary_json = serde_json::to_string(summary).expect("summary serializes");
    assert_contains_all(
        &summary_json,
        &[
            "sys5-local-slice",
            "provisional-no-compatibility-promise",
            "checked_program_identity",
            "core_ref",
            "fragment_ref",
            "edge_ref",
        ],
    );
    assert_contains_none(
        &summary_json,
        &[
            "module Mirrorea.Sys5.PrivateFieldRedaction",
            "locus WorldAuthority",
            "visible observer_safe fields",
            "participant_input[self].focus + 1",
            "avatar[target].hp = avatar[target].hp - avatar[self].atk",
            "private_secret_field",
            "private_secret_field: Int",
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_credential",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "private_payload",
            "grant_payload",
            "credential_payload",
            "source_text",
            "source contents",
            "source_contents",
            "structural_entries",
        ],
    );

    for identity in summary
        .artifacts
        .iter()
        .map(|row| &row.checked_program_identity)
        .chain(
            summary
                .generated_communication
                .iter()
                .map(|row| &row.checked_program_identity),
        )
        .chain(
            summary
                .source_core_artifact_mappings
                .iter()
                .map(|row| &row.checked_program_identity),
        )
    {
        assert!(
            !identity.is_empty(),
            "checked_program_identity must be present"
        );
        assert_contains_none(
            identity,
            &[
                "structural_entries",
                "source_text",
                "source contents",
                "source_contents",
                "module Mirrorea",
                "private_secret_field",
                "participant_input[self].focus + 1",
                "avatar[target].hp = avatar[target].hp - avatar[self].atk",
            ],
        );
    }
}

#[test]
fn observer_safe_view_redacts_source_text_and_private_field_names() {
    assert!(
        SYS5_PRIVATE_FIELD_SOURCE.contains("private_secret_field"),
        "privacy source must contain the private field falsifier"
    );
    let project = build_project(Sys5SourceInput::inline(
        "tests/inline/sys5_private_field_redaction.mir",
        SYS5_PRIVATE_FIELD_SOURCE,
    ))
    .expect("privacy source should check and project before redaction assertions");

    let observer_view =
        serde_json::to_string(&project.observer_safe_view()).expect("observer view serializes");
    assert_contains_all(
        &observer_view,
        &[
            "profile:sys5-local-slice:provisional-no-compatibility-promise",
            "core-ref:",
            "artifact-ref:",
            "edge-ref:",
        ],
    );
    assert_contains_none(
        &observer_view,
        &[
            "module Mirrorea.Sys5.PrivateFieldRedaction",
            "visible observer_safe fields",
            "participant_input[self].focus + 1",
            "avatar[target].hp = avatar[target].hp - avatar[self].atk",
            "private_secret_field",
            "private_secret_field: Int",
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_credential",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "private_payload",
            "grant_payload",
            "credential_payload",
        ],
    );
}

fn assert_contains_all(text: &str, expected_fragments: &[&str]) {
    for fragment in expected_fragments {
        assert!(
            text.contains(fragment),
            "observer-safe view missing stable semantic fragment `{fragment}`"
        );
    }
}

fn assert_contains_none(text: &str, denied_fragments: &[&str]) {
    for fragment in denied_fragments {
        assert!(
            !text.contains(fragment),
            "observer-safe view leaked denied fragment `{fragment}`"
        );
    }
}
