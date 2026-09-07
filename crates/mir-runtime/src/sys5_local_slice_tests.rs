use std::{fs, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

use crate::{
    m9_auth_verification::{
        M9FiniteLocalAdmissionCandidate, M9FiniteLocalAdmissionErrorKind,
        M9FiniteLocalAdmissionFact, M9RuntimeExecutionSeam,
    },
    sys3_projection::{
        BackendProfile, CommunicationEdgeKind, DeclaredLogicalTopology, GlobalProjectionResult,
        project_checked_core,
    },
    sys4_dispatch::LocalFabric,
    sys5_local_slice::{
        Sys5LocalAdmissionErrorKind, Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile,
        Sys5RelationBootstrapPolicy, Sys5RelationLifecycleKind, Sys5SourceInput, build_project,
    },
};

const SYS5_LOCAL_TOY_PATH: &str = "tests/inline/sys5_local_toy_admission_surface_v0.mir";
const SYS5_OWNER_ADMISSION_BUDGET_PATH: &str =
    "tests/inline/sys5_owner_admission_budget_adapter_contract.mir";

const SYS5_LOCAL_TOY_SOURCE: &str = r#"
module Mirrorea.Sys5.LocalToyAdmission

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

const SYS5_PRIVATE_FIELD_SOURCE: &str = r#"
module Mirrorea.Sys5.PrivateFieldDebugRedaction

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

fn owner_admission_budget_adapter_source(ticks: u64, include_clause: bool) -> String {
    let clause = if include_clause {
        format!(" within owner_ticks {ticks}")
    } else {
        String::new()
    };
    format!(
        "module Mirrorea.Sys5.OwnerAdmissionBudgetAdapter

locus A
locus S
principal self
principal target
type Player

state player[id: Player] at S {{
  hp: Int
  atk: Int
}}

Role[self] at A {{
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired){clause} {{
    at S {{
      player[target].hp = player[target].hp - player[self].atk
    }}
  }}
}}

with auth MembershipAuth

verify finite_refinement
"
    )
}

fn valid_admission_request() -> Sys5LocalAdmissionRequest {
    source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    ))
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn source_declared_memberships(request: Sys5LocalAdmissionRequest) -> Sys5LocalAdmissionRequest {
    request
        .with_source_declared_membership(
            "self",
            "ParticipantA",
            "epoch:sys5-local-a",
            "incarnation:self:ParticipantA:epoch:sys5-local-a",
        )
        .with_source_declared_membership(
            "self",
            "ParticipantB",
            "epoch:sys5-local-b",
            "incarnation:self:ParticipantB:epoch:sys5-local-b",
        )
        .with_source_declared_membership(
            "self",
            "ViewerC",
            "epoch:sys5-local-c",
            "incarnation:self:ViewerC:epoch:sys5-local-c",
        )
}

fn target_principal_full_membership_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "target",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:target:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "target",
        "ParticipantA",
        "epoch:sys5-local-a",
        "incarnation:target:ParticipantA:epoch:sys5-local-a",
    )
    .with_source_declared_membership(
        "target",
        "ParticipantB",
        "epoch:sys5-local-b",
        "incarnation:target:ParticipantB:epoch:sys5-local-b",
    )
    .with_source_declared_membership(
        "target",
        "ViewerC",
        "epoch:sys5-local-c",
        "incarnation:target:ViewerC:epoch:sys5-local-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn missing_participant_b_membership_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-local-a",
        "incarnation:self:ParticipantA:epoch:sys5-local-a",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-local-c",
        "incarnation:self:ViewerC:epoch:sys5-local-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn valid_ow1_admission_request() -> Sys5LocalAdmissionRequest {
    source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::Ow1,
    ))
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn canonical_m9_facts_for(principal: &str) -> Vec<M9FiniteLocalAdmissionFact> {
    vec![
        M9FiniteLocalAdmissionFact::anchor_membership(
            principal,
            "WorldAuthority",
            "epoch:sys5-local-1",
            format!("incarnation:{principal}:WorldAuthority:epoch:sys5-local-1"),
        ),
        M9FiniteLocalAdmissionFact::source_declared_membership(
            principal,
            "ParticipantA",
            "epoch:sys5-local-a",
            format!("incarnation:{principal}:ParticipantA:epoch:sys5-local-a"),
        ),
        M9FiniteLocalAdmissionFact::source_declared_membership(
            principal,
            "ParticipantB",
            "epoch:sys5-local-b",
            format!("incarnation:{principal}:ParticipantB:epoch:sys5-local-b"),
        ),
        M9FiniteLocalAdmissionFact::source_declared_membership(
            principal,
            "ViewerC",
            "epoch:sys5-local-c",
            format!("incarnation:{principal}:ViewerC:epoch:sys5-local-c"),
        ),
        M9FiniteLocalAdmissionFact::relation_bootstrap_fresh_at_admission("bird_follow"),
        M9FiniteLocalAdmissionFact::auth_discharge("MembershipAuth"),
        M9FiniteLocalAdmissionFact::optional_verification_discharge("finite_refinement"),
    ]
}

fn checked_and_projection() -> (CheckedSurfaceV0, GlobalProjectionResult) {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE.to_string(),
    ))
    .expect("canonical SYS-5 toy source checks for direct M9 boundary tests");
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("canonical SYS-5 topology has unique declared loci");
    let projection = project_checked_core(&checked, &topology)
        .expect("canonical SYS-5 checked Core projects for direct M9 boundary tests");
    (checked, projection)
}

#[test]
fn source_derived_finite_admission_inventory_bootstraps_st_with_empty_source_bound_seed() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");
    let project_identity = project.checked_program_identity_ref();
    assert_sys5_checked_program_sha256_ref(project_identity);
    let modified_source = SYS5_LOCAL_TOY_SOURCE.replace(
        "participant_input[self].focus + 1",
        "participant_input[self].focus + 2",
    );
    let modified = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        modified_source,
    ))
    .expect("modified source still checks and projects");
    assert_ne!(
        project_identity,
        modified.checked_program_identity_ref(),
        "checked program identity must change when the source text changes"
    );

    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("source-derived admission should seal a complete finite SYS-5 inventory");

    let summary = prepared.observer_safe_admission_summary();
    assert_eq!(summary.checked_program_identity_ref(), project_identity);
    assert_eq!(summary.runtime_profile(), Sys5LocalRuntimeProfile::St);
    assert!(summary.is_source_derived());
    assert!(summary.is_complete_for_projection());
    assert!(!summary.public_api_or_wire_contract());
    assert!(summary.derived_from_sealed_admission());
    assert_eq!(
        summary.raw_input_rejection_profile(),
        "sys5-finite-admission-request-surface"
    );
    assert!(!summary.raw_input_rejection_is_runtime_evidence());
    assert!(!summary.sealed_inventory_digest().is_empty());
    assert!(!summary.sealed_inventory_attestation_ref().is_empty());
    let counts = summary.sealed_inventory_counts();
    assert_eq!(counts.owner_rmw(), 1);
    assert_eq!(counts.relation_transitions(), 2);
    assert_eq!(counts.designated_evaluators(), 1);
    assert_eq!(counts.designated_remote_inputs(), 1);
    assert_eq!(counts.named_consumers(), 1);

    let auth = summary
        .auth_discharge("MembershipAuth")
        .expect("MembershipAuth residual is explicitly discharged");
    assert!(auth.is_discharged());
    assert!(auth.has_source_ref());
    assert!(auth.has_m9_evidence_ref());
    assert!(!auth.grants_runtime_authority_by_name_only());

    let verify = summary
        .verification_discharge("finite_refinement")
        .expect("finite_refinement optional verifier is separately discharged");
    assert!(verify.is_discharged());
    assert!(verify.has_source_ref());
    assert!(verify.has_finite_refinement_evidence_ref());
    assert!(!verify.is_merged_into_auth());

    let inventory = prepared.observer_safe_inventory();
    let sealed_attestation = prepared.sealed_inventory_attestation();
    assert_eq!(inventory.checked_program_identity_ref(), project_identity);
    assert_eq!(
        sealed_attestation.checked_program_identity_ref(),
        project_identity
    );
    assert!(!sealed_attestation.digest().is_empty());
    assert_eq!(sealed_attestation.owner_rmw_count(), 1);
    assert_eq!(sealed_attestation.relation_transition_count(), 2);
    assert_eq!(sealed_attestation.designated_evaluator_count(), 1);
    assert_eq!(sealed_attestation.designated_remote_input_count(), 1);
    assert_eq!(sealed_attestation.named_consumer_count(), 1);
    assert!(sealed_attestation.exact_row_set_match());
    assert!(inventory.matches_sealed_attestation(sealed_attestation));
    assert!(sealed_attestation.covers_source_inventory(inventory));
    assert_eq!(inventory.owner_rmw_operation_ids(), ["attack"]);
    assert!(inventory.contains_owner_rmw("attack", "self", "ParticipantA", "WorldAuthority"));
    for lifecycle in [
        Sys5RelationLifecycleKind::Invalidate,
        Sys5RelationLifecycleKind::FreshReacquire,
    ] {
        let row = inventory
            .relation_lifecycle("bird_follow", lifecycle)
            .expect("relation lifecycle row is source-derived into the sealed inventory");
        assert_eq!(row.bootstrap_policy(), "bounded-local-bootstrap");
        assert!(!row.core_derived());
        assert!(!row.grants_authority());
        assert!(!row.accepts_raw_lease_or_ref());
        assert!(inventory.contains_relation_lifecycle("bird_follow", lifecycle));
    }
    assert!(inventory.contains_designated_evaluator("WorldAuthority.result", "WorldAuthority"));
    assert!(inventory.contains_designated_remote_input(
        "WorldAuthority.result",
        0,
        "ParticipantA",
        "WorldAuthority"
    ));
    assert!(inventory.contains_named_consumer("WorldAuthority.result", "ViewerC"));
    assert!(inventory.covers_every_generated_remote_input());
    assert!(inventory.covers_every_relation_lifecycle_row());

    let observer_json =
        serde_json::to_string(summary).expect("observer-safe admission summary serializes");
    assert_contains_all(
        &observer_json,
        &[
            "checked_program_identity",
            "MembershipAuth",
            "finite_refinement",
            "attack",
            "bird_follow",
            "WorldAuthority.result",
            "ViewerC",
            "raw_input_rejection_profile",
            "sys5-finite-admission-request-surface",
            "derived_from_sealed_admission",
        ],
    );
    assert_contains_none(
        &observer_json,
        &[
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_credential",
            "raw_scope",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "raw_lease",
            "lease_ref",
            "route_override",
            "expected_result",
            "rejected_untrusted_admission_fields",
            "rejected_raw_authority_payloads",
            "source_text",
            "avatar[target].hp = avatar[target].hp - avatar[self].atk",
        ],
    );

    let (program, admission) = prepared.into_parts_for_sys4();
    assert_eq!(
        admission
            .observer_safe_m9_summary()
            .checked_program_identity()
            .stable_key(),
        program.checked_program_identity().stable_key()
    );
    assert!(
        admission
            .initial_state_seed()
            .int("WorldAuthority", "avatar", "self", "hp")
            .is_none(),
        "SYS-5 admission uses an empty source-bound seed; it must not inject state"
    );

    let fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("prepared source-derived admission plus empty seed bootstraps SYS-4 ST");
    assert_eq!(
        fabric.locus_names(),
        ["ParticipantA", "ParticipantB", "ViewerC", "WorldAuthority"]
    );
}

#[test]
fn direct_m9_boundary_rejects_unknown_membership_fact_before_validated_candidate_or_authority() {
    let (checked, projection) = checked_and_projection();
    let mut facts = canonical_m9_facts_for("self");
    facts.push(M9FiniteLocalAdmissionFact::source_declared_membership(
        "self",
        "ForeignLocus",
        "epoch:sys5-local-foreign",
        "incarnation:self:ForeignLocus:epoch:sys5-local-foreign",
    ));

    let err = M9FiniteLocalAdmissionCandidate::from_checked(&checked, &projection, facts)
        .expect_err("unknown principal/locus facts must not become a validated M9 candidate");
    assert_eq!(
        err.kind(),
        M9FiniteLocalAdmissionErrorKind::UnknownPrincipalOrLocus
    );
    assert!(err.rejected_before_validated_candidate());
    assert!(err.rejected_before_execution_seam());
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_execution_seam().is_none());
}

#[test]
fn direct_m9_boundary_rejects_target_handler_principal_before_validated_candidate_or_authority() {
    let (checked, projection) = checked_and_projection();

    let err = M9FiniteLocalAdmissionCandidate::from_checked(
        &checked,
        &projection,
        canonical_m9_facts_for("target"),
    )
    .expect_err("target has complete rows but is not the unique checked owner actor origin");
    assert_eq!(
        err.kind(),
        M9FiniteLocalAdmissionErrorKind::PrincipalPolicyMismatch
    );
    assert!(err.rejected_before_validated_candidate());
    assert!(err.rejected_before_execution_seam());
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_execution_seam().is_none());
}

#[test]
fn m9_execution_seam_accepts_only_validated_finite_local_candidate() {
    let (checked, projection) = checked_and_projection();
    let validated = M9FiniteLocalAdmissionCandidate::from_checked(
        &checked,
        &projection,
        canonical_m9_facts_for("self"),
    )
    .expect("canonical facts produce an opaque validated finite local candidate");

    let seam = M9RuntimeExecutionSeam::admit_validated_finite_local_candidate(validated)
        .expect("execution seam accepts the validated candidate, not raw fact slices");
    assert!(seam.has_complete_final_residual_discharge());

    let m9_source = runtime_source("m9_auth_verification.rs");
    let compact = m9_source.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_contains_all(
        &compact,
        &[
            "struct M9FiniteLocalAdmissionCandidate",
            "struct M9ValidatedFiniteLocalAdmissionCandidate",
            "admit_validated_finite_local_candidate",
        ],
    );
    assert_contains_none(
        &compact,
        &[
            "pub fn from_unchecked",
            "pub fn new_unchecked",
            "pub fn from_raw_fact_slices",
            "pub fn from_raw_strings",
            "pub struct M9ValidatedFiniteLocalAdmissionCandidate { pub",
            "pub(crate) struct M9ValidatedFiniteLocalAdmissionCandidate { pub(crate)",
            "admit_sys5_source_derived_inventory(",
            "admit_validated_sys5",
            "Sys5",
            "sys5_",
            "source_declared_memberships: &[",
            "requested_principal: &str",
            "requested_locus: &str",
            "requested_epoch: &str",
            "requested_incarnation: &str",
        ],
    );
}

#[test]
fn public_source_carrying_debug_views_redact_source_text_private_fields_and_raw_authority() {
    assert!(
        SYS5_PRIVATE_FIELD_SOURCE.contains("private_secret_field")
            && SYS5_PRIVATE_FIELD_SOURCE
                .contains("avatar[target].hp = avatar[target].hp - avatar[self].atk")
            && SYS5_PRIVATE_FIELD_SOURCE.contains("participant_input[self].focus + 1"),
        "privacy RED source must contain private field and source-expression falsifiers"
    );
    let input = Sys5SourceInput::inline(
        "tests/inline/sys5_private_debug_redaction.mir",
        SYS5_PRIVATE_FIELD_SOURCE,
    );
    assert_debug_redacts_sensitive_surface_and_authority(&format!("{input:?}"));

    let project = build_project(input).expect("private-field source checks and projects");
    assert_debug_redacts_sensitive_surface_and_authority(&format!("{project:?}"));

    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("private-field source prepares finite admission before Debug redaction check");
    assert_debug_redacts_sensitive_surface_and_authority(&format!("{prepared:?}"));
}

#[test]
fn finite_admission_rejects_unknown_source_declared_principal_and_locus_before_authority_issuance()
{
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");

    let unknown_principal =
        source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
            "intruder",
            "WorldAuthority",
            "epoch:sys5-local-1",
            "incarnation:intruder:WorldAuthority:epoch:sys5-local-1",
            Sys5LocalRuntimeProfile::St,
        ))
        .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
        .with_auth_discharge("MembershipAuth")
        .with_optional_verification_discharge("finite_refinement");
    let principal_err = project
        .prepare_finite_admission(unknown_principal)
        .expect_err("unknown principal must fail before authority issuance");
    assert_eq!(
        principal_err.kind(),
        Sys5LocalAdmissionErrorKind::UnknownPrincipal
    );
    assert!(principal_err.rejected_before_authority_issuance());
    assert!(principal_err.partial_admission().is_none());

    let unknown_locus = source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
        "self",
        "ForeignLocus",
        "epoch:sys5-local-1",
        "incarnation:self:ForeignLocus:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    ))
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement");
    let locus_err = project
        .prepare_finite_admission(unknown_locus)
        .expect_err("unknown locus must fail before authority issuance");
    assert_eq!(locus_err.kind(), Sys5LocalAdmissionErrorKind::UnknownLocus);
    assert!(locus_err.rejected_before_authority_issuance());
    assert!(locus_err.partial_admission().is_none());
}

#[test]
fn finite_admission_rejects_missing_required_membership_before_authority_issuance() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");

    let err = project
        .prepare_finite_admission(missing_participant_b_membership_request())
        .expect_err("omitting ParticipantB membership must fail before authority issuance");
    assert_eq!(
        err.kind(),
        Sys5LocalAdmissionErrorKind::MissingRequiredMembership
    );
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_admission().is_none());
}

#[test]
fn finite_admission_rejects_handler_principal_mismatch_before_authority_issuance() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");

    let err = project
        .prepare_finite_admission(target_principal_full_membership_request())
        .expect_err("finite profile handler principal must match checked owner actor origin");
    assert_eq!(
        err.kind(),
        Sys5LocalAdmissionErrorKind::PrincipalPolicyMismatch
    );
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_admission().is_none());
}

#[test]
fn finite_admission_rejects_missing_relation_bootstrap_policy_before_authority_issuance() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");
    let missing_policy = source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    ))
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement");

    let err = project
        .prepare_finite_admission(missing_policy)
        .expect_err("relation bootstrap policy must be explicit");
    assert_eq!(
        err.kind(),
        Sys5LocalAdmissionErrorKind::MissingRelationBootstrapPolicy
    );
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_admission().is_none());
}

#[test]
fn finite_admission_rejects_ow1_backend_before_authority_issuance_for_canonical_four_locus() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");

    let err = project
        .prepare_finite_admission(valid_ow1_admission_request())
        .expect_err("canonical four-locus SYS-5 admission is not OW1-eligible yet");
    assert_eq!(err.kind(), Sys5LocalAdmissionErrorKind::BackendIneligible);
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_admission().is_none());
}

#[test]
fn finite_admission_rejects_missing_optional_verification_discharge_fail_closed() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 toy source checks and projects");
    let missing_verify = source_declared_memberships(Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    ))
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth");

    let err = project
        .prepare_finite_admission(missing_verify)
        .expect_err("omitting finite_refinement discharge must fail closed");
    assert_eq!(
        err.kind(),
        Sys5LocalAdmissionErrorKind::MissingVerificationDischarge
    );
    assert!(err.rejected_before_authority_issuance());
    assert!(err.partial_admission().is_none());
}

#[test]
fn i3_owner_admission_budget_adapter_contract_preserves_checked_metadata_and_changes_its_fingerprint()
 {
    let one_tick_source = owner_admission_budget_adapter_source(1, true);
    let checked_one = check_and_elaborate_surface_v0(FixtureSource::new(
        SYS5_OWNER_ADMISSION_BUDGET_PATH,
        one_tick_source.clone(),
    ))
    .expect("one-tick source checks before the SYS-5 adapter facade reads its projection");
    let expected_one = checked_one
        .evaluation("attack")
        .expect("one-tick source has its checked owner evaluation")
        .owner_admission_budget()
        .expect("one-tick checked owner evaluation retains its budget");
    let one_tick_project = build_project(Sys5SourceInput::inline(
        SYS5_OWNER_ADMISSION_BUDGET_PATH,
        one_tick_source,
    ))
    .expect("one-tick source projects through the SYS-5 facade");
    let one_tick_projection = one_tick_project.projected_result_for_i2_evidence();
    let one_tick_request_edge = one_tick_projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("one-tick source has an owner request edge");
    let one_tick_reply_edge = one_tick_projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerReplyReceipt, "S", "A")
        .expect("one-tick source has an owner reply/receipt edge");
    let one_tick_request = one_tick_project
        .i3_adapter_carrier_contract(one_tick_request_edge.edge_ref())
        .expect("one-tick owner request obtains its source-bound adapter contract");
    let one_tick_reply = one_tick_project
        .i3_adapter_carrier_contract(one_tick_reply_edge.edge_ref())
        .expect("one-tick owner reply obtains its source-bound adapter contract");
    let restored_request = one_tick_request
        .owner_admission_budget()
        .expect("adapter owner request retains a private typed budget snapshot")
        .clone()
        .into_checked(expected_one.owner_locus().as_str())
        .expect("adapter request snapshot restores only against the checked owner locus");
    let restored_reply = one_tick_reply
        .owner_admission_budget()
        .expect("adapter owner reply retains a private typed budget snapshot")
        .clone()
        .into_checked(expected_one.owner_locus().as_str())
        .expect("adapter reply snapshot restores only against the checked owner locus");
    assert_eq!(&restored_request, expected_one);
    assert_eq!(&restored_reply, expected_one);
    assert_eq!(restored_request, restored_reply);

    let two_tick_project = build_project(Sys5SourceInput::inline(
        SYS5_OWNER_ADMISSION_BUDGET_PATH,
        owner_admission_budget_adapter_source(2, true),
    ))
    .expect("two-tick source projects through the SYS-5 facade");
    let two_tick_request_edge = two_tick_project
        .projected_result_for_i2_evidence()
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("two-tick source has an owner request edge");
    let two_tick_request = two_tick_project
        .i3_adapter_carrier_contract(two_tick_request_edge.edge_ref())
        .expect("two-tick owner request obtains its source-bound adapter contract");
    assert_ne!(
        one_tick_request.full_retained_contract_fingerprint(),
        two_tick_request.full_retained_contract_fingerprint(),
        "changing only the checked budget must change the exact retained adapter fingerprint"
    );

    let padded_project = build_project(Sys5SourceInput::inline(
        SYS5_OWNER_ADMISSION_BUDGET_PATH,
        owner_admission_budget_adapter_source(1, false),
    ))
    .expect("unannotated DeadlineExpired padding remains projectable ordinary source");
    let padded_projection = padded_project.projected_result_for_i2_evidence();
    let padded_request_edge = padded_projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerRequest, "A", "S")
        .expect("padded source has an owner request edge");
    let padded_reply_edge = padded_projection
        .communication_plan()
        .single_edge("attack", CommunicationEdgeKind::OwnerReplyReceipt, "S", "A")
        .expect("padded source has an owner reply/receipt edge");
    assert!(
        padded_project
            .i3_adapter_carrier_contract(padded_request_edge.edge_ref())
            .expect("padded request obtains an adapter contract")
            .owner_admission_budget()
            .is_none(),
        "padding must not manufacture an adapter request budget"
    );
    assert!(
        padded_project
            .i3_adapter_carrier_contract(padded_reply_edge.edge_ref())
            .expect("padded reply obtains an adapter contract")
            .owner_admission_budget()
            .is_none(),
        "padding must not manufacture an adapter reply budget"
    );
}

#[test]
fn finite_admission_request_surface_accepts_no_raw_authority_route_state_or_expected_result() {
    let source = runtime_source("sys5_local_slice.rs");
    let request_surface = request_surface_source(&source);
    let compact = request_surface
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    assert_contains_all(
        &compact,
        &[
            "struct Sys5LocalAdmissionRequest",
            "source_declared",
            "principal",
            "locus",
            "epoch",
            "incarnation",
            "runtime_profile",
            "with_source_declared_membership",
            "with_relation_bootstrap_policy",
            "FreshAtAdmission",
        ],
    );
    assert_contains_none(
        &compact,
        &[
            "with_raw_scope",
            "with_raw_capability",
            "with_capability_ref",
            "with_raw_witness",
            "with_witness_ref",
            "with_route",
            "with_route_override",
            "with_state",
            "with_seed_value",
            "with_expected_result",
            "with_raw_lease",
            "with_lease_ref",
            "expected_result:",
            "raw_lease:",
            "lease_ref:",
            "raw_authority_payload:",
            "raw_capability_payload:",
            "raw_witness_payload:",
        ],
    );
}

fn request_surface_source(source: &str) -> &str {
    let start = source
        .find("struct Sys5LocalAdmissionRequest")
        .expect("SYS-5 finite admission request type is defined");
    let end = source[start..]
        .find("struct Sys5PreparedAdmission")
        .map(|offset| start + offset)
        .expect("Sys5PreparedAdmission marks the next top-level admission type");
    &source[start..end]
}

fn assert_sys5_checked_program_sha256_ref(actual: &str) {
    let Some(digest) = actual.strip_prefix("sys5-checked-program-sha256-v1:") else {
        panic!("checked program ref must use sys5 sha256 prefix, got `{actual}`");
    };
    assert_eq!(
        digest.len(),
        64,
        "checked program sha256 digest must be 64 lowercase hex chars"
    );
    assert!(
        digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "checked program sha256 digest must be lowercase hex"
    );
}

fn assert_debug_redacts_sensitive_surface_and_authority(debug: &str) {
    assert_contains_none(
        debug,
        &[
            "module Mirrorea.Sys5.PrivateFieldDebugRedaction",
            "locus WorldAuthority",
            "visible observer_safe fields",
            "private_secret_field",
            "private_secret_field: Int",
            "participant_input[self].focus + 1",
            "avatar[target].hp = avatar[target].hp - avatar[self].atk",
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_credential",
            "raw_scope",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "raw_lease",
            "lease_ref",
            "route_override",
            "expected_result",
            "source_text",
            "source_text:",
            "source contents",
            "source_contents",
        ],
    );
}

fn runtime_source(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(relative);
    fs::read_to_string(path).expect("runtime source is readable")
}

fn assert_contains_all(text: &str, expected_fragments: &[&str]) {
    for fragment in expected_fragments {
        assert!(
            text.contains(fragment),
            "text missing intended fragment `{fragment}`"
        );
    }
}

fn assert_contains_none(text: &str, denied_fragments: &[&str]) {
    for fragment in denied_fragments {
        assert!(
            !text.contains(fragment),
            "text leaked or accepted denied fragment `{fragment}`"
        );
    }
}
