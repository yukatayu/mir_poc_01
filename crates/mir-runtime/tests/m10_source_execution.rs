use mir_runtime::m10_reference_system::{M10ReferenceSystem, M10SourceRunRequest};
use serde_json::{Value, json};

const INLINE_M10_SOURCE: &str = r#"
module M10.Temp.AttackRelation

locus World
locus BrowserClient
locus C
principal self
principal target
type Player

state player[id: Player] at World {
  hp: Int
  atk: Int
  visible observer_safe fields (hp)
}

Role[self] at BrowserClient {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, VisibilityDenied) {
    at World {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}

relation bird_follow at World {
  subject bird: Player
  primary perch_anchor epoch primary_epoch transform translate(3, -2)
  fallback nest_anchor epoch fallback_epoch transform identity
  bind frontier bird_binding_frontier
  publish relation
  project at C local
}

with auth MembershipAuth
verify finite_refinement
"#;

fn run_inline_source() -> Value {
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report = system
        .run_source(
            M10SourceRunRequest::inline_text(
                "tmp/m10/attack_relation_source_execution.mir",
                INLINE_M10_SOURCE,
            )
            .entry_event("attack")
            .principal("self")
            .target("target")
            .initial_player_hp("target", 100)
            .initial_player_atk("self", 10)
            .attack_count(2)
            .require_relation_projection("bird_follow", "C"),
        )
        .expect("M10 inline ordinary source executes");
    serde_json::to_value(report).expect("M10 source run report is serializable")
}

fn assert_json_pointer_eq(value: &Value, pointer: &str, expected: Value) {
    assert_eq!(
        value.pointer(pointer),
        Some(&expected),
        "unexpected value at {pointer}: {value:#}"
    );
}

#[test]
fn temp_ordinary_source_flows_once_through_m6_m7_m8_m9_trace_and_projection() {
    let value = run_inline_source();

    assert_json_pointer_eq(
        &value,
        "/source/path",
        json!("tmp/m10/attack_relation_source_execution.mir"),
    );
    assert_json_pointer_eq(&value, "/source/kind", json!("inline_text"));
    assert_json_pointer_eq(&value, "/source/fixture_name_lookup_used", json!(false));
    assert_json_pointer_eq(&value, "/pipeline/m6_parse_count", json!(1));
    assert_json_pointer_eq(&value, "/pipeline/m7_checked_artifact_count", json!(1));
    assert_json_pointer_eq(&value, "/pipeline/reparsed_after_m7", json!(false));
    assert_json_pointer_eq(
        &value,
        "/m8/direct_residuals/0/outcome",
        json!("DeferredToM9"),
    );
    assert_json_pointer_eq(
        &value,
        "/m9/source_bound_admission/outcome",
        json!("accepted"),
    );
    assert_json_pointer_eq(
        &value,
        "/m9/source_bound_admission/authority_issuer",
        json!("M9"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/owner_rmw/hp_history",
        json!([100, 90, 80]),
    );
    assert_json_pointer_eq(&value, "/runtime/owner_rmw/final_hp", json!(80));
    assert_json_pointer_eq(
        &value,
        "/runtime/owner_rmw/request/caller_locus",
        json!("BrowserClient"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/owner_rmw/request/owner_locus",
        json!("World"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/semantic_kernel/owner_path",
        json!("SemanticRuntimeKernel::from_m9_execution_seam"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/semantic_kernel/lifecycle",
        json!(["request", "serve", "reply", "receive_receipt"]),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/semantic_kernel/m8_runtime_owned",
        json!(true),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/safe_trace/raw_authority_payload_exported",
        json!(false),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/safe_trace/authority_origin_principal",
        json!("self"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/safe_trace/authority_origin_locus",
        json!("BrowserClient"),
    );
    assert_json_pointer_eq(
        &value,
        "/runtime/safe_trace/evaluation_locus",
        json!("World"),
    );
    assert_json_pointer_eq(&value, "/projection/relation/name", json!("bird_follow"));
    assert_json_pointer_eq(&value, "/projection/relation/consumer_locus", json!("C"));

    let source_identity = value.pointer("/source/identity").expect("source identity");
    assert_eq!(
        value.pointer("/checked/source_identity"),
        Some(source_identity)
    );
    assert_eq!(
        value.pointer("/m8/checked_source_identity"),
        Some(source_identity)
    );
    assert_eq!(
        value.pointer("/m9/source_bound_admission/source_identity"),
        Some(source_identity)
    );
    assert_eq!(
        value.pointer("/runtime/safe_trace/source_identity"),
        Some(source_identity)
    );
    assert_eq!(
        value.pointer("/projection/relation/source_identity"),
        Some(source_identity)
    );

    let first_replay = value
        .pointer("/runtime/deterministic_replay/hash")
        .expect("replay hash");
    let second = run_inline_source();
    assert_eq!(
        second.pointer("/runtime/deterministic_replay/hash"),
        Some(first_replay),
        "same source/profile must replay deterministically"
    );
}

#[test]
fn wrong_source_residual_or_authority_rejects_before_runtime_mutation() {
    for (fault, mutation, diagnostic, validator, seam) in [
        (
            "wrong_residual_source_ref",
            json!({"kind": "rewrite_residual_source_ref", "residual": "with auth MembershipAuth", "replacement_source_path": "tmp/m10/forged_source.mir"}),
            "SourceRefMismatch",
            "m9_residual_binding_validator",
            "M9AdmissionEnvelope",
        ),
        (
            "wrong_source_identity_envelope",
            json!({"kind": "rewrite_original_source_artifact_identity", "replacement_source_identity": "forged-source-identity"}),
            "SourceArtifactIdentityMismatch",
            "m9_source_artifact_validator",
            "M9AdmissionEnvelope",
        ),
        (
            "forged_authority_at_enqueue",
            json!({"kind": "enqueue_owner_with_forged_authority", "authority_ref": "forged:owner:authority"}),
            "ForgedAuthorityRejected",
            "SemanticRuntimeKernel::validate_owner_carrier",
            "SemanticRuntimeKernelPreAdmission",
        ),
        (
            "missing_live_authority_at_service",
            json!({"kind": "drop_live_authority_before_service", "authority_ref": "m10-owner-capability:attack"}),
            "MissingLiveAuthority",
            "SemanticRuntimeKernel::validate_owner_carrier",
            "SemanticRuntimeKernelPreAdmission",
        ),
    ] {
        let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
        let report = system
            .run_source(
                M10SourceRunRequest::inline_text(
                    "tmp/m10/attack_relation_source_execution.mir",
                    INLINE_M10_SOURCE,
                )
                .entry_event("attack")
                .principal("self")
                .target("target")
                .initial_player_hp("target", 100)
                .initial_player_atk("self", 10)
                .typed_input_mutation(json!({
                    "schema_version": "m10-i1plus-source-run-mutation-v0",
                    "id": fault,
                    "mutation": mutation,
                })),
            )
            .expect("M10 fault row returns typed rejection report");
        let value = serde_json::to_value(report).expect("M10 rejection report is serializable");

        assert_json_pointer_eq(&value, "/terminal_outcome", json!("RejectedBeforeMutation"));
        assert_json_pointer_eq(&value, "/fault/id", json!(fault));
        assert_json_pointer_eq(&value, "/fault/name_driven_terminal_used", json!(false));
        assert_json_pointer_eq(&value, "/validation/real_validator_invoked", json!(true));
        assert_json_pointer_eq(&value, "/validation/seam_reached", json!(seam));
        assert_json_pointer_eq(
            &value,
            &format!("/validation/invocations/{validator}"),
            json!(1),
        );
        assert_json_pointer_eq(&value, "/diagnostics/0/code", json!(diagnostic));
        assert_json_pointer_eq(
            &value,
            "/diagnostics/0/source_path",
            json!("tmp/m10/attack_relation_source_execution.mir"),
        );
        assert_json_pointer_eq(&value, "/runtime/mutation_count", json!(0));
        assert!(
            value.pointer("/runtime/semantic_kernel").is_none(),
            "rejected pre-admission carrier must not claim accepted ordinary-run kernel evidence"
        );
        assert_eq!(
            value.pointer("/runtime/store_hash_before"),
            value.pointer("/runtime/store_hash_after"),
            "fault {fault} must not mutate runtime store"
        );
    }
}
