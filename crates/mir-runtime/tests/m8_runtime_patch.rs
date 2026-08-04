use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::m8_runtime_admission::{
    EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionEvidence, M8Runtime, M8RuntimeAdmission,
    M8RuntimeInstance, M8SecurityClass,
};
use mir_runtime::m8_runtime_authority::{
    M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
};
use mir_runtime::m8_runtime_designated_value::{
    M8DesignatedAuthorityUse, M8DesignatedEvaluationRequest, M8DesignatedTick, M8InputReceipt,
    M8InputReceiptSet,
};
use mir_runtime::m8_runtime_local_cut::M8LiveFloor;
use mir_runtime::m8_runtime_owner_queue::{M8AuthorityUse, M8OwnerRequest, M8StateKey};
use mir_runtime::m8_runtime_patch::{
    M8LeaseRecord, M8PatchActivationCutKind, M8PatchAuthorityUse, M8PatchCandidate,
    M8PatchDiagnosticKind, M8PatchLifecycleKind, M8PatchRuntime, M8PatchRuntimeSeed,
    M8PatchVerdictKind,
};
use mir_semantics::{
    shared_model::{ResultVersion, SourceRef},
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedProgramIdentity, CheckedSurfaceV0, ResidualObligationKind,
        check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const UNIFIED_FIXTURE: &str = "m8_unified_local_cut_no_m9_residuals.mir";
const PATCH_PLUS_TWO_FIXTURE: &str = "m8_unified_patch_designated_plus_two.mir";
const DEFERRED_FIXTURE: &str = "m7_residual_cannot_execute.mir";
const OWNER: &str = "S";
const CONSUMER: &str = "C";
const EVALUATOR: &str = "E";
const RELATION_NAME: &str = "bird_follow";
const VALUE_NAME: &str = "E.result";
const RESULT_NAME: &str = "result";
const INPUT_FRONTIER: &str = "F";
const BINDING_FRONTIER: &str = "bird_binding_frontier";
const RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:1";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:owner_epoch1";
const ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:owner_epoch1";
const ATTACK_WITNESS_REF: &str = "witness:attack:S:self:owner_epoch1";
const PATCH_MEMBERSHIP_REF: &str = "membership:self:S:patch_activation_epoch1";
const PATCH_CAPABILITY_REF: &str =
    "cap:patch:activate:Combat.M8.UnifiedLocalCut:self:patch_activation_epoch1";
const PATCH_WITNESS_REF: &str =
    "witness:patch:activate:Combat.M8.UnifiedLocalCut:self:patch_activation_epoch1";
const ABSENT_PATCH_CAPABILITY_REF: &str =
    "cap:patch:activate:Combat.M8.UnifiedLocalCut:self:absent";
const ABSENT_PATCH_WITNESS_REF: &str =
    "witness:patch:activate:Combat.M8.UnifiedLocalCut:self:absent";
const RELATION_MEMBERSHIP_REF: &str = "membership:self:S:relation-binding-epoch1";
const RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:binding_epoch1";
const RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:witness_epoch1";
const EVALUATOR_MEMBERSHIP_REF: &str = "membership:self:E:eval_epoch1";
const EVALUATE_CAPABILITY_REF: &str = "cap:designated:evaluate:E.result:self:eval_epoch1";
const EVALUATE_WITNESS_REF: &str = "witness:designated:evaluate:E.result:self:eval_epoch1";
const CONSUMER_MEMBERSHIP_REF: &str = "membership:self:C:consume_epoch1";
const CONSUME_CAPABILITY_REF: &str = "cap:designated:consume:C:E.result:self:consume_epoch1";
const CONSUME_WITNESS_REF: &str = "witness:designated:consume:C:E.result:self:consume_epoch1";
const INPUT_RECEIPT_REF: &str = "receipt:S:player[self].atk:E:F:1";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn checked_surface_fixture(name: &str) -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture(name);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("surface-v0 fixture checks through M7");
    (path, source, checked)
}

fn checked_unified_fixture() -> (String, String, CheckedSurfaceV0) {
    checked_surface_fixture(UNIFIED_FIXTURE)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn byte_range_after(source: &str, anchor: &str, needle: &str) -> Range<usize> {
    let anchor_start = source
        .find(anchor)
        .unwrap_or_else(|| panic!("fixture contains anchor {anchor:?}"));
    let relative = source[anchor_start..]
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?} after {anchor:?}"));
    let start = anchor_start + relative;
    start..start + needle.len()
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1_u32;
    let mut column = 1_u32;
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

fn source_ref_for_range(path: &str, source: &str, range: Range<usize>) -> SourceRef {
    let (start_line, start_column) = line_column(source, range.start);
    let (end_line, end_column) = line_column(source, range.end);
    SourceRef::new(
        path.to_owned(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
    source_ref_for_range(path, source, byte_range(source, lexeme))
}

fn expected_source_ref_after(path: &str, source: &str, anchor: &str, lexeme: &str) -> SourceRef {
    source_ref_for_range(path, source, byte_range_after(source, anchor, lexeme))
}

fn residual_source_ref(
    checked: &CheckedSurfaceV0,
    kind: ResidualObligationKind,
    name: &str,
) -> SourceRef {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|entry| entry.kind() == kind && entry.name() == name)
        .unwrap_or_else(|| panic!("missing residual {kind:?}/{name}"))
        .source_ref()
        .clone()
}

fn relation_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "relation bird_follow at S {\n  subject bird: Player\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}",
    )
}

fn designated_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "designated evaluate E on tick F publish result = player[self].atk + 1",
    )
}

fn designated_plus_two_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "designated evaluate E on tick F publish result = player[self].atk + 2",
    )
}

fn designated_input_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref_after(
        path,
        source,
        "designated evaluate E on tick F publish result",
        "player[self].atk",
    )
}

fn patch_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(path, source, "module Combat.M8.UnifiedLocalCut")
}

fn hp_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "target", "hp")
}

fn atk_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "self", "atk")
}

fn relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: RELATION_NAME.into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:consumer-visible")
            .with_class(M8SecurityClass::Restricted),
        redaction: EvidenceRedaction::new("consumer:C"),
        source_ref,
    }
}

fn relation_lifetime_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationLifetime {
        relation: RELATION_NAME.into(),
        live_lease: RELATION_LEASE_REF.into(),
        binding_frontier: BINDING_FRONTIER.into(),
        source_ref,
    }
}

fn relation_fallback_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: RELATION_NAME.into(),
        primary_epoch: "primary_epoch".into(),
        fallback_epoch: "fallback_epoch".into(),
        source_ref,
    }
}

fn designated_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::ValueVisibilityRedaction {
        value: VALUE_NAME.into(),
        label: EvidenceSecurityLabel::new("value:E.result:publish")
            .with_class(M8SecurityClass::Private),
        redaction: EvidenceRedaction::new("conservative"),
        source_ref,
    }
}

fn deferred_auth_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::AuthDeferred {
        name: "MembershipAuth".into(),
        authority_label: "membership-authority/MembershipAuth".into(),
        source_ref,
    }
}

fn deferred_verify_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::VerifyDeferred {
        name: "finite_refinement".into(),
        theorem: "finite_refinement".into(),
        witness_schema: "m9-proof-witness-required".into(),
        source_ref,
    }
}

fn complete_unified_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            RELATION_NAME,
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            RELATION_NAME,
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            RELATION_NAME,
        )))
        .with_evidence(designated_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::ValueVisibilityRedaction,
            VALUE_NAME,
        )))
}

fn deferred_m9_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(deferred_auth_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::AuthDeferred,
            "MembershipAuth",
        )))
        .with_evidence(deferred_verify_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::VerifyDeferred,
            "finite_refinement",
        )))
}

fn admitted_unified_instance() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8RuntimeInstance,
) {
    let (path, source, checked) = checked_unified_fixture();
    let admission = complete_unified_admission_for(&checked);
    let instance = M8Runtime::default()
        .admit(checked.clone(), admission.clone())
        .expect("exact four non-M9 evidence rows admit unified M8 patch fixture");
    (path, source, checked, admission, instance)
}

fn assert_unified_fixture_shape_and_exact_admission(
    path: &str,
    source: &str,
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
    instance: &M8RuntimeInstance,
) {
    let relation_ref = relation_source_ref(path, source);
    let designated_ref = designated_source_ref(path, source);
    assert_eq!(
        checked.program_identity().module(),
        "Combat.M8.UnifiedLocalCut"
    );
    assert_eq!(
        checked
            .evaluations()
            .iter()
            .map(|evaluation| evaluation.kind())
            .collect::<Vec<_>>(),
        vec![
            CheckedEvaluationKind::OwnerRmw,
            CheckedEvaluationKind::PublishRelation,
            CheckedEvaluationKind::DesignatedPublishValue,
        ]
    );
    assert_eq!(
        checked
            .residual_obligations()
            .entries()
            .iter()
            .map(|entry| (entry.kind(), entry.name(), entry.source_ref()))
            .collect::<Vec<_>>(),
        vec![
            (
                ResidualObligationKind::Visibility,
                RELATION_NAME,
                &relation_ref,
            ),
            (
                ResidualObligationKind::RelationLifetime,
                RELATION_NAME,
                &relation_ref,
            ),
            (
                ResidualObligationKind::FallbackValidity,
                RELATION_NAME,
                &relation_ref,
            ),
            (
                ResidualObligationKind::ValueVisibilityRedaction,
                VALUE_NAME,
                &designated_ref,
            ),
        ]
    );
    assert_eq!(admission.program_identity(), checked.program_identity());
    assert_eq!(admission.evidence().len(), 4);
    assert_eq!(instance.program_identity(), checked.program_identity());
    assert_eq!(instance.admission_evidence().entries().len(), 4);
}

fn authority_state() -> M8AuthorityState {
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted(OWNER_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("owner_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(ATTACK_CAPABILITY_REF)
                .for_owner_evaluation("attack")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("owner_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(ATTACK_WITNESS_REF)
                .for_capability(ATTACK_CAPABILITY_REF)
                .with_membership_ref(OWNER_MEMBERSHIP_REF)
                .with_epoch("owner_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(PATCH_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("patch_activation_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(PATCH_CAPABILITY_REF)
                .for_patch_activation("Combat.M8.UnifiedLocalCut")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(PATCH_MEMBERSHIP_REF)
                .with_epoch("patch_activation_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(PATCH_WITNESS_REF)
                .for_capability(PATCH_CAPABILITY_REF)
                .with_membership_ref(PATCH_MEMBERSHIP_REF)
                .with_epoch("patch_activation_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("binding_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(RELATION_CAPABILITY_REF)
                .for_relation_transition(RELATION_NAME, "invalidate_primary")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_REF)
                .with_binding_epoch("binding_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(RELATION_WITNESS_REF)
                .for_capability(RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_REF)
                .with_epoch("witness_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(EVALUATOR_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(EVALUATOR)
                .with_epoch("eval_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(EVALUATE_CAPABILITY_REF)
                .for_designated_evaluation(EVALUATOR, RESULT_NAME)
                .with_evaluator_locus(EVALUATOR)
                .with_principal("self")
                .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
                .with_input_frontier(INPUT_FRONTIER)
                .with_epoch("eval_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(EVALUATE_WITNESS_REF)
                .for_capability(EVALUATE_CAPABILITY_REF)
                .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
                .with_epoch("eval_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(CONSUMER_MEMBERSHIP_REF)
                .with_principal("self")
                .with_locus(CONSUMER)
                .with_epoch("consume_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(CONSUME_CAPABILITY_REF)
                .for_designated_consumption(CONSUMER, VALUE_NAME)
                .with_consumer_locus(CONSUMER)
                .with_principal("self")
                .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
                .with_result_version(ResultVersion::new(1))
                .with_epoch("consume_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(CONSUME_WITNESS_REF)
                .for_capability(CONSUME_CAPABILITY_REF)
                .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
                .with_epoch("consume_epoch:1"),
        )
}

fn live_relation_lease() -> M8LeaseRecord {
    M8LeaseRecord::live(RELATION_LEASE_REF)
        .for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_binding_frontier(BINDING_FRONTIER)
        .with_epoch("binding_epoch:1")
}

fn input_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    M8InputReceipt::live(INPUT_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(OWNER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier(INPUT_FRONTIER)
        .with_source_ref(source_ref.clone())
        .with_label(
            EvidenceSecurityLabel::new("input:player[self].atk:S:F")
                .with_class(M8SecurityClass::Restricted),
        )
        .with_int_value(10)
}

fn valid_owner_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ATTACK_CAPABILITY_REF)
        .with_witness_ref(ATTACK_WITNESS_REF)
}

fn owner_attack_request() -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(valid_owner_authority_use())
}

fn evaluator_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_evaluator(EVALUATOR)
        .with_principal("self")
        .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
        .with_capability_ref(EVALUATE_CAPABILITY_REF)
        .with_witness_ref(EVALUATE_WITNESS_REF)
}

fn evaluation_request() -> M8DesignatedEvaluationRequest {
    M8DesignatedEvaluationRequest::for_value(VALUE_NAME)
        .with_tick(M8DesignatedTick::new("tick:F:1").with_input_frontier(INPUT_FRONTIER))
        .with_authority(evaluator_authority_use())
}

fn receipt_set(receipts: Vec<M8InputReceipt>) -> M8InputReceiptSet {
    let mut set = M8InputReceiptSet::new();
    for receipt in receipts {
        set = set.with_receipt(receipt);
    }
    set
}

fn patch_authority_use() -> M8PatchAuthorityUse {
    M8PatchAuthorityUse::for_patch_program("Combat.M8.UnifiedLocalCut")
        .with_owner_locus(OWNER)
        .with_principal("self")
        .with_membership_ref(PATCH_MEMBERSHIP_REF)
        .with_capability_ref(PATCH_CAPABILITY_REF)
        .with_witness_ref(PATCH_WITNESS_REF)
}

fn invalid_patch_authority_use() -> M8PatchAuthorityUse {
    M8PatchAuthorityUse::for_patch_program("Combat.M8.UnifiedLocalCut")
        .with_owner_locus(OWNER)
        .with_principal("self")
        .with_membership_ref(PATCH_MEMBERSHIP_REF)
        .with_capability_ref(ABSENT_PATCH_CAPABILITY_REF)
        .with_witness_ref(ABSENT_PATCH_WITNESS_REF)
}

fn provider_name_only_patch_authority_use() -> M8PatchAuthorityUse {
    M8PatchAuthorityUse::for_patch_program("Combat.M8.UnifiedLocalCut")
        .with_provider_name("devtools")
        .with_package_name("debug-package")
        .with_principal("self")
}

fn patch_runtime_with_hp(
    hp: i64,
) -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8PatchRuntime,
) {
    let (path, source, checked, admission, instance) = admitted_unified_instance();
    let input_ref = designated_input_source_ref(&path, &source);
    let runtime = M8PatchRuntime::from_admitted(
        instance,
        M8PatchRuntimeSeed::new()
            .with_owner_int(hp_key(), hp)
            .with_owner_int(atk_key(), 10)
            .with_authority_state(authority_state())
            .with_live_lease(live_relation_lease())
            .with_designated_input_receipts(receipt_set(vec![input_receipt(&input_ref)])),
    );
    (path, source, checked, admission, runtime)
}

fn patch_runtime() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8PatchRuntime,
) {
    patch_runtime_with_hp(100)
}

fn accepted_candidate(
    patch_id: &str,
    path: &str,
    source: &str,
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
) -> M8PatchCandidate {
    M8PatchCandidate::from_checked_admitted(patch_id, checked.clone(), admission.clone())
        .with_base_program_identity(checked.program_identity().clone())
        .with_base_admission(admission.clone())
        .with_source_ref(patch_source_ref(path, source))
        .with_reason_ref("reason:phase4b:compatible")
        .with_patch_authority(patch_authority_use())
}

fn wrong_identity(checked: &CheckedSurfaceV0) -> CheckedProgramIdentity {
    CheckedProgramIdentity::new(
        "Combat.M8.OtherPatchBase",
        checked.program_identity().source_file(),
        checked.program_identity().root_source_ref().clone(),
    )
}

#[test]
fn accepted_patch_candidate_records_lifecycle_and_activation_cut_before_semantic_change() {
    let (path, source, checked, admission, mut runtime) = patch_runtime();
    let (_, _, _, _, shape_instance) = admitted_unified_instance();
    assert_unified_fixture_shape_and_exact_admission(
        &path,
        &source,
        &checked,
        &admission,
        &shape_instance,
    );
    let before = runtime.save_relevant_payload();
    let candidate = accepted_candidate(
        "patch:accepted:compatible",
        &path,
        &source,
        &checked,
        &admission,
    );
    assert!(!candidate.accepts_raw_eval());

    let outcome = runtime.activate_patch(candidate.clone());

    assert_eq!(outcome.verdict(), M8PatchVerdictKind::Accepted);
    assert_eq!(
        outcome.lifecycle().kinds(),
        vec![
            M8PatchLifecycleKind::CandidateBound,
            M8PatchLifecycleKind::Parsed,
            M8PatchLifecycleKind::Checked,
            M8PatchLifecycleKind::Elaborated,
            M8PatchLifecycleKind::Compatible,
            M8PatchLifecycleKind::RuntimeAdmitted,
            M8PatchLifecycleKind::ActivationCut,
        ]
    );
    assert!(!outcome.lifecycle().contains_raw_eval());
    assert_eq!(outcome.source_ref(), candidate.source_ref());
    assert_eq!(outcome.reason_ref(), Some("reason:phase4b:compatible"));
    let activation_cut = outcome
        .activation_cut()
        .expect("accepted patch records a bounded activation cut");
    assert_eq!(
        activation_cut.kind(),
        M8PatchActivationCutKind::SingleSession
    );
    assert_eq!(
        runtime.active_program_identity(),
        candidate.checked_program_identity()
    );
    assert_ne!(runtime.save_relevant_payload(), before);
    assert!(
        activation_cut
            .is_the_only_semantic_change_between(&before, &runtime.save_relevant_payload())
    );
    let (_, _, _, _, different_seed_runtime) = patch_runtime_with_hp(77);
    assert!(
        !activation_cut.is_the_only_semantic_change_between(
            &different_seed_runtime.save_relevant_payload(),
            &runtime.save_relevant_payload(),
        ),
        "activation-only comparison must not accept unrelated runtime seed deltas"
    );
}

#[test]
fn accepted_patch_installs_candidate_checked_plans_and_local_cut_restores_candidate_execution() {
    let (_, _, base_checked, base_admission, mut runtime) = patch_runtime();
    let (patch_path, patch_source, patch_checked) = checked_surface_fixture(PATCH_PLUS_TWO_FIXTURE);
    let patch_admission = complete_unified_admission_for(&patch_checked);
    let patch_instance = M8Runtime::default()
        .admit(patch_checked.clone(), patch_admission.clone())
        .expect("patch candidate is parsed, checked, and admitted with exact M8 evidence");
    assert_eq!(
        patch_instance.program_identity(),
        patch_checked.program_identity()
    );
    assert_eq!(
        patch_checked.program_identity().module(),
        base_checked.program_identity().module()
    );
    assert_ne!(
        patch_checked.program_identity().source_file(),
        base_checked.program_identity().source_file()
    );
    assert_eq!(
        designated_plus_two_source_ref(&patch_path, &patch_source),
        residual_source_ref(
            &patch_checked,
            ResidualObligationKind::ValueVisibilityRedaction,
            VALUE_NAME,
        )
    );
    let candidate_input_ref = designated_input_source_ref(&patch_path, &patch_source);
    let candidate = M8PatchCandidate::from_checked_admitted(
        "patch:plus-two-designated",
        patch_checked.clone(),
        patch_admission.clone(),
    )
    .with_base_program_identity(base_checked.program_identity().clone())
    .with_base_admission(base_admission.clone())
    .with_source_ref(patch_source_ref(&patch_path, &patch_source))
    .with_reason_ref("reason:phase4b:actual-plus-two")
    .with_patch_authority(patch_authority_use())
    .with_designated_input_receipts(receipt_set(vec![input_receipt(&candidate_input_ref)]));

    let outcome = runtime.activate_patch(candidate.clone());

    assert_eq!(outcome.verdict(), M8PatchVerdictKind::Accepted);
    assert_eq!(
        runtime.active_program_identity(),
        patch_checked.program_identity()
    );
    assert_eq!(runtime.active_admission(), &patch_admission);
    let published = runtime
        .evaluate_designated(evaluation_request())
        .expect("active candidate designated plan executes through M8PatchRuntime");
    assert_eq!(published.int_value(), Some(12));

    let cut = runtime.save_local_cut("cut:patch:plus-two-designated");
    assert_eq!(cut.program_identity(), patch_checked.program_identity());
    assert_eq!(
        cut.admission_provenance().evidence(),
        patch_admission.evidence()
    );
    assert!(
        cut.patch_lifecycle()
            .rows()
            .iter()
            .any(|row| row == "activated:patch:plus-two-designated")
    );

    let (_, _, _, _, mut restored) = patch_runtime();
    let floor = M8LiveFloor::same_current(&cut);
    restored
        .try_restore_local_cut(&cut, &floor)
        .expect("same-current live floor restores active patch candidate cut");
    assert_eq!(
        restored.active_program_identity(),
        patch_checked.program_identity()
    );
    assert_eq!(restored.active_admission(), &patch_admission);
    let restored_published = restored
        .evaluate_designated(evaluation_request())
        .expect("restored patch runtime still executes the candidate plan");
    assert_eq!(restored_published.int_value(), Some(12));
}

#[test]
fn patch_activation_requires_quiescent_single_session_cut_without_pending_owner_work() {
    let (path, source, checked, admission, mut runtime) = patch_runtime();
    let pending = runtime
        .enqueue_owner(owner_attack_request())
        .expect("pending owner work exists before patch activation");
    let before_payload = runtime.semantic_payload_without_patch_lifecycle();
    let candidate = accepted_candidate(
        "patch:pending-owner-work",
        &path,
        &source,
        &checked,
        &admission,
    );

    let outcome = runtime.activate_patch(candidate);

    assert_ne!(outcome.verdict(), M8PatchVerdictKind::Accepted);
    assert!(
        matches!(
            outcome.verdict(),
            M8PatchVerdictKind::Deferred | M8PatchVerdictKind::Rejected
        ),
        "pending old-owner work must defer or reject without mutation"
    );
    assert_eq!(
        outcome.primary_diagnostic().kind(),
        M8PatchDiagnosticKind::NonQuiescentSession
    );
    assert!(outcome.activation_cut().is_none());
    assert_eq!(
        runtime.semantic_payload_without_patch_lifecycle(),
        before_payload
    );
    assert_eq!(runtime.pending_owner_fifo(OWNER), vec![pending.id()]);
}

#[test]
fn patch_activation_requires_preexisting_patch_authority_not_provider_strings() {
    let (path, source, checked, admission, _) = patch_runtime();
    let source_ref = patch_source_ref(&path, &source);

    let missing_authority = M8PatchCandidate::from_checked_admitted(
        "patch:missing-authority",
        checked.clone(),
        admission.clone(),
    )
    .with_base_program_identity(checked.program_identity().clone())
    .with_base_admission(admission.clone())
    .with_source_ref(source_ref.clone())
    .with_reason_ref("reason:missing-patch-authority");

    let invalid_authority = accepted_candidate(
        "patch:invalid-authority",
        &path,
        &source,
        &checked,
        &admission,
    )
    .with_patch_authority(invalid_patch_authority_use())
    .with_reason_ref("reason:invalid-patch-authority");

    let provider_name_only = M8PatchCandidate::from_checked_admitted(
        "patch:provider-name-only",
        checked.clone(),
        admission.clone(),
    )
    .with_base_program_identity(checked.program_identity().clone())
    .with_base_admission(admission.clone())
    .with_source_ref(source_ref)
    .with_reason_ref("reason:provider-name-is-not-patch-authority")
    .with_patch_authority(provider_name_only_patch_authority_use());

    for candidate in [missing_authority, invalid_authority, provider_name_only] {
        let (_, _, _, _, mut runtime) = patch_runtime();
        let before_payload = runtime.semantic_payload_without_patch_lifecycle();
        let before_lifecycle_len = runtime.patch_lifecycle().len();

        let outcome = runtime.activate_patch(candidate);

        assert_eq!(outcome.verdict(), M8PatchVerdictKind::Rejected);
        assert_eq!(
            outcome.primary_diagnostic().kind(),
            M8PatchDiagnosticKind::MissingPatchAuthority
        );
        assert!(outcome.activation_cut().is_none());
        assert!(
            !outcome
                .lifecycle()
                .contains(M8PatchLifecycleKind::RuntimeAdmitted)
        );
        assert!(
            !outcome
                .lifecycle()
                .contains(M8PatchLifecycleKind::ActivationCut)
        );
        assert_eq!(
            runtime.semantic_payload_without_patch_lifecycle(),
            before_payload
        );
        assert_eq!(runtime.patch_lifecycle().len(), before_lifecycle_len + 1);
        assert_eq!(
            runtime.patch_lifecycle().last_diagnostic_kind(),
            Some(M8PatchDiagnosticKind::MissingPatchAuthority)
        );
    }
}

#[test]
fn rejected_patch_candidates_fail_closed_with_typed_rows_and_no_semantic_mutation() {
    let (path, source, checked, admission, _) = patch_runtime();

    for (candidate, expected_kind) in [
        (
            M8PatchCandidate::unknown_reference(
                "patch:unknown",
                patch_source_ref(&path, &source),
                "reason:unknown-candidate",
            ),
            M8PatchDiagnosticKind::UnknownCandidate,
        ),
        (
            accepted_candidate("patch:stale", &path, &source, &checked, &admission)
                .mark_stale_against_current_identity(),
            M8PatchDiagnosticKind::StaleCandidate,
        ),
        (
            M8PatchCandidate::from_checked_unadmitted(
                "patch:unadmitted",
                checked.clone(),
                patch_source_ref(&path, &source),
                "reason:no-m8-admission",
            ),
            M8PatchDiagnosticKind::UnadmittedCandidate,
        ),
    ] {
        let (_, _, _, _, mut runtime) = patch_runtime();
        let before_payload = runtime.semantic_payload_without_patch_lifecycle();
        let before_lifecycle_len = runtime.patch_lifecycle().len();

        let outcome = runtime.activate_patch(candidate);

        assert_eq!(outcome.verdict(), M8PatchVerdictKind::Rejected);
        assert_eq!(outcome.primary_diagnostic().kind(), expected_kind);
        assert!(outcome.source_ref().path.ends_with(UNIFIED_FIXTURE));
        assert!(outcome.reason_ref().is_some());
        assert_eq!(
            runtime.semantic_payload_without_patch_lifecycle(),
            before_payload
        );
        assert_eq!(runtime.patch_lifecycle().len(), before_lifecycle_len + 1);
        assert_eq!(
            runtime.patch_lifecycle().last_diagnostic_kind(),
            Some(expected_kind)
        );
        assert!(
            runtime
                .patch_lifecycle()
                .last_source_ref()
                .path
                .ends_with(UNIFIED_FIXTURE)
        );
    }
}

#[test]
fn patch_candidate_with_m9_auth_or_verify_residual_is_deferred_without_hidden_success() {
    let (_, _, _, _, mut runtime) = patch_runtime();
    let (deferred_path, deferred_source, deferred_checked) =
        checked_surface_fixture(DEFERRED_FIXTURE);
    let deferred_admission = deferred_m9_admission_for(&deferred_checked);
    let candidate = M8PatchCandidate::from_checked_deferred_to_m9(
        "patch:needs-m9",
        deferred_checked.clone(),
        deferred_admission,
    )
    .with_base_program_identity(runtime.active_program_identity().clone())
    .with_source_ref(expected_source_ref(
        &deferred_path,
        &deferred_source,
        "with auth MembershipAuth",
    ))
    .with_reason_ref("reason:m9-auth-or-verify-residual");
    let before_payload = runtime.semantic_payload_without_patch_lifecycle();

    let outcome = runtime.activate_patch(candidate);

    assert_eq!(outcome.verdict(), M8PatchVerdictKind::Deferred);
    assert_eq!(
        outcome.primary_diagnostic().kind(),
        M8PatchDiagnosticKind::DeferredToM9
    );
    assert!(outcome.lifecycle().contains(M8PatchLifecycleKind::Deferred));
    assert!(outcome.activation_cut().is_none());
    assert!(!outcome.has_runtime_success());
    assert!(!outcome.grants_authority());
    assert!(!outcome.emits_verdict());
    assert_eq!(
        runtime.semantic_payload_without_patch_lifecycle(),
        before_payload
    );
}

#[test]
fn structural_identity_or_admission_mismatch_rejects_before_activation_cut() {
    let (path, source, checked, admission, _) = patch_runtime();

    for (candidate, expected_kind) in [
        (
            accepted_candidate("patch:wrong-base", &path, &source, &checked, &admission)
                .with_base_program_identity(wrong_identity(&checked)),
            M8PatchDiagnosticKind::StructuralIdentityMismatch,
        ),
        (
            accepted_candidate(
                "patch:wrong-admission",
                &path,
                &source,
                &checked,
                &admission,
            )
            .with_base_admission(M8RuntimeAdmission::new(checked.program_identity().clone())),
            M8PatchDiagnosticKind::AdmissionProvenanceMismatch,
        ),
    ] {
        let (_, _, _, _, mut runtime) = patch_runtime();
        let before_payload = runtime.semantic_payload_without_patch_lifecycle();
        let before_lifecycle_len = runtime.patch_lifecycle().len();

        let outcome = runtime.activate_patch(candidate);

        assert_eq!(outcome.verdict(), M8PatchVerdictKind::Rejected);
        assert_eq!(outcome.primary_diagnostic().kind(), expected_kind);
        assert!(outcome.activation_cut().is_none());
        assert_eq!(
            runtime.semantic_payload_without_patch_lifecycle(),
            before_payload
        );
        assert_eq!(runtime.patch_lifecycle().len(), before_lifecycle_len + 1);
        assert_eq!(
            runtime.patch_lifecycle().last_diagnostic_kind(),
            Some(expected_kind)
        );
    }
}

#[test]
fn candidate_admission_is_revalidated_for_required_relation_and_designated_evidence() {
    let (path, source, checked, admission, _) = patch_runtime();
    let missing_relation_evidence = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(designated_visibility_evidence(residual_source_ref(
            &checked,
            ResidualObligationKind::ValueVisibilityRedaction,
            VALUE_NAME,
        )));
    let missing_designated_evidence = M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            &checked,
            ResidualObligationKind::Visibility,
            RELATION_NAME,
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            &checked,
            ResidualObligationKind::RelationLifetime,
            RELATION_NAME,
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            &checked,
            ResidualObligationKind::FallbackValidity,
            RELATION_NAME,
        )));

    for (patch_id, candidate_admission) in [
        ("patch:missing-relation-evidence", missing_relation_evidence),
        (
            "patch:missing-designated-evidence",
            missing_designated_evidence,
        ),
    ] {
        let (_, _, _, _, mut runtime) = patch_runtime();
        let before_payload = runtime.semantic_payload_without_patch_lifecycle();
        let before_lifecycle_len = runtime.patch_lifecycle().len();
        let candidate =
            M8PatchCandidate::from_checked_admitted(patch_id, checked.clone(), candidate_admission)
                .with_base_program_identity(checked.program_identity().clone())
                .with_base_admission(admission.clone())
                .with_source_ref(patch_source_ref(&path, &source))
                .with_reason_ref("reason:candidate-admission-revalidation")
                .with_patch_authority(patch_authority_use());

        let outcome = runtime.activate_patch(candidate);

        assert_eq!(outcome.verdict(), M8PatchVerdictKind::Rejected);
        assert_eq!(
            outcome.primary_diagnostic().kind(),
            M8PatchDiagnosticKind::AdmissionProvenanceMismatch
        );
        assert!(outcome.activation_cut().is_none());
        assert!(
            !outcome
                .lifecycle()
                .contains(M8PatchLifecycleKind::RuntimeAdmitted)
        );
        assert!(
            !outcome
                .lifecycle()
                .contains(M8PatchLifecycleKind::ActivationCut)
        );
        assert_eq!(
            runtime.semantic_payload_without_patch_lifecycle(),
            before_payload
        );
        assert_eq!(runtime.patch_lifecycle().len(), before_lifecycle_len + 1);
    }
}
