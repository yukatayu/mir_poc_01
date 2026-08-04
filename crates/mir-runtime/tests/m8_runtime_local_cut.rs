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
    M8ConsumeRequest, M8ConsumedDesignatedValue, M8DesignatedAuthorityUse,
    M8DesignatedDiagnosticKind, M8DesignatedEvaluationRequest, M8DesignatedTick, M8InputReceipt,
    M8InputReceiptSet, M8PresentationInterpolation, M8PublishedDesignatedValue,
};
use mir_runtime::m8_runtime_local_cut::{
    M8LeaseRecord, M8LiveFloor, M8LocalCut, M8LocalPatchLifecycleState,
    M8LocalRestoreDiagnosticKind, M8LocalRuntime, M8LocalRuntimeSeed, M8LocalSavePayload,
    M8LocalTrace, M8LocalTraceKind, M8ObserverFailureRow,
};
use mir_runtime::m8_runtime_owner_queue::{
    M8AuthorityUse, M8DeclaredFailure, M8EnqueueDiagnosticKind, M8Occurrence, M8OwnerRequest,
    M8ServeDiagnosticKind, M8ServeOutcome, M8StateKey,
};
use mir_runtime::m8_runtime_relation_projection::{
    M8AnchorSample, M8BindingInvalidation, M8Point, M8PresentationContext, M8RelationAuthorityUse,
    M8RelationProjection, M8RelationReacquire, M8RelationTransition,
};
use mir_semantics::{
    shared_model::{ResultVersion, SourceRef},
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedSurfaceV0, ResidualObligationKind,
        check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const UNIFIED_FIXTURE: &str = "m8_unified_local_cut_no_m9_residuals.mir";
const OWNER: &str = "S";
const CONSUMER: &str = "C";
const EVALUATOR: &str = "E";
const RELATION_NAME: &str = "bird_follow";
const VALUE_NAME: &str = "E.result";
const RESULT_NAME: &str = "result";
const INPUT_FRONTIER: &str = "F";
const BINDING_FRONTIER: &str = "bird_binding_frontier";
const DEGRADED_FRONTIER: &str = "bird_binding_frontier:degraded";
const REACQUIRED_FRONTIER: &str = "bird_binding_frontier:reacquired";
const RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:1";
const REACQUIRE_RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:2";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:owner_epoch1";
const ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:owner_epoch1";
const ATTACK_WITNESS_REF: &str = "witness:attack:S:self:owner_epoch1";
const ABSENT_ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:absent";
const ABSENT_ATTACK_WITNESS_REF: &str = "witness:attack:S:self:absent";
const RELATION_MEMBERSHIP_EPOCH1_REF: &str = "membership:self:S:relation-binding-epoch1";
const RELATION_MEMBERSHIP_EPOCH2_REF: &str = "membership:self:S:relation-binding-epoch2";
const INVALIDATE_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:binding_epoch1";
const INVALIDATE_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:witness_epoch1";
const REACQUIRE_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:reacquire_primary:binding_epoch2";
const REACQUIRE_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:reacquire_primary:witness_epoch2";
const EVALUATOR_MEMBERSHIP_REF: &str = "membership:self:E:eval_epoch1";
const EVALUATE_CAPABILITY_REF: &str = "cap:designated:evaluate:E.result:self:eval_epoch1";
const EVALUATE_WITNESS_REF: &str = "witness:designated:evaluate:E.result:self:eval_epoch1";
const ABSENT_EVALUATE_CAPABILITY_REF: &str = "cap:designated:evaluate:E.result:self:absent";
const ABSENT_EVALUATE_WITNESS_REF: &str = "witness:designated:evaluate:E.result:self:absent";
const CONSUMER_MEMBERSHIP_REF: &str = "membership:self:C:consume_epoch1";
const CONSUME_CAPABILITY_REF: &str = "cap:designated:consume:C:E.result:self:consume_epoch1";
const CONSUME_WITNESS_REF: &str = "witness:designated:consume:C:E.result:self:consume_epoch1";
const ABSENT_CONSUME_CAPABILITY_REF: &str = "cap:designated:consume:C:E.result:self:absent";
const ABSENT_CONSUME_WITNESS_REF: &str = "witness:designated:consume:C:E.result:self:absent";
const INPUT_RECEIPT_REF: &str = "receipt:S:player[self].atk:E:F:1";
const CONSUME_DELIVERY_ID: &str = "delivery:C:E.result:F:version1";

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

fn checked_unified_fixture() -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture(UNIFIED_FIXTURE);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("unified local-cut fixture checks through M7");
    (path, source, checked)
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

fn designated_input_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref_after(
        path,
        source,
        "designated evaluate E on tick F publish result",
        "player[self].atk",
    )
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
        .expect("exact four non-M9 evidence rows admit unified M8 local-cut fixture");
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
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("binding_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_principal("self")
                .with_locus(OWNER)
                .with_epoch("binding_epoch:2"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(INVALIDATE_RELATION_CAPABILITY_REF)
                .for_relation_transition(RELATION_NAME, "invalidate_primary")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_binding_epoch("binding_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(INVALIDATE_RELATION_WITNESS_REF)
                .for_capability(INVALIDATE_RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_epoch("witness_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(REACQUIRE_RELATION_CAPABILITY_REF)
                .for_relation_transition(RELATION_NAME, "reacquire_primary")
                .with_owner_locus(OWNER)
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_binding_epoch("binding_epoch:2"),
        )
        .with_witness_record(
            M8WitnessRecord::live(REACQUIRE_RELATION_WITNESS_REF)
                .for_capability(REACQUIRE_RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_epoch("witness_epoch:2"),
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

fn fresh_reacquire_relation_lease() -> M8LeaseRecord {
    M8LeaseRecord::live(REACQUIRE_RELATION_LEASE_REF)
        .for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_binding_frontier(REACQUIRED_FRONTIER)
        .with_epoch("binding_epoch:2")
        .with_anchor_epoch("primary_epoch:2")
}

fn assert_relation_admission_declares_lease_payload(evidence: &[M8AdmissionEvidence]) {
    let declared = evidence
        .iter()
        .find_map(|row| match row {
            M8AdmissionEvidence::RelationLifetime {
                relation,
                live_lease,
                binding_frontier,
                ..
            } if relation == RELATION_NAME => {
                Some((live_lease.as_str(), binding_frontier.as_str()))
            }
            _ => None,
        })
        .expect("relation lifetime admission evidence is retained");

    assert_eq!(declared.0, RELATION_LEASE_REF);
    assert_eq!(declared.1, BINDING_FRONTIER);
}

fn assert_observer_failure_row_policy_is_serialized(
    row: &M8ObserverFailureRow,
    payload_rows: &[serde_json::Value],
) {
    let label = row.label();
    let redaction = row.redaction();
    assert!(
        label
            .security_class()
            .is_at_least(M8SecurityClass::Restricted)
    );
    assert!(!label.as_str().is_empty());
    assert_ne!(label.as_str(), "unspecified");
    assert!(!redaction.as_str().is_empty());
    assert_ne!(redaction.as_str(), "unspecified");

    let failure_kind = row.failure_kind();
    let serialized = payload_rows
        .iter()
        .find(|value| {
            value.get("failure_family").and_then(|field| field.as_str())
                == Some(row.failure_family())
                && value.get("failure_kind").and_then(|field| field.as_str())
                    == Some(failure_kind.as_str())
        })
        .unwrap_or_else(|| {
            panic!(
                "serialized observer payload includes {} / {}",
                row.failure_family(),
                failure_kind
            )
        });
    assert_eq!(
        serialized.get("label").and_then(|field| field.as_str()),
        Some(label.as_str())
    );
    assert_eq!(
        serialized.get("redaction").and_then(|field| field.as_str()),
        Some(redaction.as_str())
    );
}

fn valid_owner_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ATTACK_CAPABILITY_REF)
        .with_witness_ref(ATTACK_WITNESS_REF)
}

fn missing_capability_owner_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ABSENT_ATTACK_CAPABILITY_REF)
        .with_witness_ref(ABSENT_ATTACK_WITNESS_REF)
}

fn invalidate_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_transition("invalidate_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
        .with_capability_ref(INVALIDATE_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:1")
        .with_witness_ref(INVALIDATE_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:1")
}

fn reacquire_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_transition("reacquire_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
        .with_capability_ref(REACQUIRE_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:2")
        .with_witness_ref(REACQUIRE_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:2")
}

fn evaluator_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_evaluator(EVALUATOR)
        .with_principal("self")
        .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
        .with_capability_ref(EVALUATE_CAPABILITY_REF)
        .with_witness_ref(EVALUATE_WITNESS_REF)
}

fn missing_evaluator_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_evaluator(EVALUATOR)
        .with_principal("self")
        .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
        .with_capability_ref(ABSENT_EVALUATE_CAPABILITY_REF)
        .with_witness_ref(ABSENT_EVALUATE_WITNESS_REF)
}

fn consume_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_consumer(CONSUMER)
        .with_principal("self")
        .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
        .with_capability_ref(CONSUME_CAPABILITY_REF)
        .with_witness_ref(CONSUME_WITNESS_REF)
}

fn missing_consumer_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_consumer(CONSUMER)
        .with_principal("self")
        .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
        .with_capability_ref(ABSENT_CONSUME_CAPABILITY_REF)
        .with_witness_ref(ABSENT_CONSUME_WITNESS_REF)
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

fn receipt_set(receipts: Vec<M8InputReceipt>) -> M8InputReceiptSet {
    let mut set = M8InputReceiptSet::new();
    for receipt in receipts {
        set = set.with_receipt(receipt);
    }
    set
}

fn owner_attack_request() -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(valid_owner_authority_use())
}

fn owner_attack_request_with_authority(authority: M8AuthorityUse) -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(authority)
}

fn evaluation_request() -> M8DesignatedEvaluationRequest {
    M8DesignatedEvaluationRequest::for_value(VALUE_NAME)
        .with_tick(M8DesignatedTick::new("tick:F:1").with_input_frontier(INPUT_FRONTIER))
        .with_authority(evaluator_authority_use())
}

fn evaluation_request_with_authority(
    authority: M8DesignatedAuthorityUse,
) -> M8DesignatedEvaluationRequest {
    M8DesignatedEvaluationRequest::for_value(VALUE_NAME)
        .with_tick(M8DesignatedTick::new("tick:F:1").with_input_frontier(INPUT_FRONTIER))
        .with_authority(authority)
}

fn consume_request() -> M8ConsumeRequest {
    M8ConsumeRequest::for_value(VALUE_NAME)
        .with_consumer(CONSUMER)
        .with_delivery_id(CONSUME_DELIVERY_ID)
        .with_authority(consume_authority_use())
}

fn consume_request_with_authority(authority: M8DesignatedAuthorityUse) -> M8ConsumeRequest {
    M8ConsumeRequest::for_value(VALUE_NAME)
        .with_consumer(CONSUMER)
        .with_delivery_id("delivery:C:E.result:F:invalid-authority")
        .with_authority(authority)
}

fn primary_relation_context() -> M8PresentationContext {
    M8PresentationContext::for_consumer(CONSUMER)
        .with_frontier(BINDING_FRONTIER)
        .with_anchor_sample(
            M8AnchorSample::new("perch_anchor")
                .with_epoch("primary_epoch")
                .with_frontier(BINDING_FRONTIER)
                .with_pose(M8Point::new(10, 20)),
        )
        .with_anchor_sample(
            M8AnchorSample::new("nest_anchor")
                .with_epoch("fallback_epoch")
                .with_frontier(BINDING_FRONTIER)
                .with_pose(M8Point::new(-30, 5)),
        )
}

fn degraded_relation_context() -> M8PresentationContext {
    M8PresentationContext::for_consumer(CONSUMER)
        .with_frontier(DEGRADED_FRONTIER)
        .with_anchor_sample(
            M8AnchorSample::new("nest_anchor")
                .with_epoch("fallback_epoch")
                .with_frontier(DEGRADED_FRONTIER)
                .with_pose(M8Point::new(-30, 5)),
        )
}

fn local_runtime_without_live_relation_lease() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8LocalRuntime,
) {
    local_runtime_with_relation_lease_records(Vec::new())
}

fn local_runtime_with_relation_lease_records(
    leases: Vec<M8LeaseRecord>,
) -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8LocalRuntime,
) {
    let (path, source, checked, admission, instance) = admitted_unified_instance();
    let input_ref = designated_input_source_ref(&path, &source);
    let mut seed = M8LocalRuntimeSeed::new()
        .with_owner_int(hp_key(), 100)
        .with_owner_int(atk_key(), 10)
        .with_authority_state(authority_state())
        .with_designated_input_receipts(receipt_set(vec![input_receipt(&input_ref)]));
    for lease in leases {
        seed = seed.with_live_lease(lease);
    }
    let runtime = M8LocalRuntime::from_admitted(instance, seed);
    (path, source, checked, admission, runtime)
}

fn local_runtime() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8LocalRuntime,
) {
    let (path, source, checked, admission, instance) = admitted_unified_instance();
    let input_ref = designated_input_source_ref(&path, &source);
    let runtime = M8LocalRuntime::from_admitted(
        instance,
        M8LocalRuntimeSeed::new()
            .with_owner_int(hp_key(), 100)
            .with_owner_int(atk_key(), 10)
            .with_authority_state(authority_state())
            .with_live_lease(live_relation_lease())
            .with_live_lease(fresh_reacquire_relation_lease())
            .with_designated_input_receipts(receipt_set(vec![input_receipt(&input_ref)])),
    );
    (path, source, checked, admission, runtime)
}

fn runtime_with_pending_second_attack() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8LocalRuntime,
    String,
    String,
) {
    let (path, source, checked, admission, mut runtime) = local_runtime();
    let first = runtime
        .enqueue_owner(owner_attack_request())
        .expect("first owner request enqueues from admitted checked plan");
    let second = runtime
        .enqueue_owner(owner_attack_request())
        .expect("second owner request enqueues from admitted checked plan");
    runtime
        .serve_next_owner(OWNER)
        .expect("first queued owner request serves before save");
    (
        path,
        source,
        checked,
        admission,
        runtime,
        first.id().to_string(),
        second.id().to_string(),
    )
}

fn runtime_after_designated_publication() -> M8LocalRuntime {
    let (_, _, _, _, mut runtime) = local_runtime();
    runtime
        .evaluate_designated(evaluation_request())
        .expect("designated value publishes before rollback checks");
    runtime
}

fn assert_restore_rejects_without_payload_or_cut_mutation(
    runtime: &mut M8LocalRuntime,
    cut: &M8LocalCut,
    floor: M8LiveFloor,
    expected_kind: M8LocalRestoreDiagnosticKind,
) {
    let before_payload = runtime.save_relevant_payload();
    let before_cut = cut.clone();
    let before_trace_len = runtime.trace().len();

    let diagnostics = runtime
        .try_restore_local_cut(cut, &floor)
        .expect_err("stale local cut must reject with typed restore diagnostics");

    assert_eq!(diagnostics.primary().kind(), expected_kind);
    assert_eq!(runtime.save_relevant_payload(), before_payload);
    assert_eq!(cut, &before_cut);
    let failure_suffix = runtime.trace().suffix_from(before_trace_len);
    assert_eq!(
        failure_suffix.kinds(),
        vec![M8LocalTraceKind::RestoreRejected]
    );
    assert_eq!(
        failure_suffix.restore_diagnostic_kinds(),
        vec![expected_kind]
    );
    assert!(failure_suffix.all_entries_are_outside_saved_payload());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalReplayEvidence {
    first_occurrence: M8Occurrence,
    second_occurrence: M8Occurrence,
    first_owner_outcome: M8ServeOutcome,
    second_owner_outcome: M8ServeOutcome,
    relation_transition: M8RelationTransition,
    relation_projection: M8RelationProjection,
    designated_publication: M8PublishedDesignatedValue,
    designated_consumption: M8ConsumedDesignatedValue,
    save_payload_before_cut: M8LocalSavePayload,
    cut_payload: M8LocalSavePayload,
    cut: M8LocalCut,
    trace_prefix: M8LocalTrace,
}

fn apply_final_deterministic_replay_log(runtime: &mut M8LocalRuntime) -> FinalReplayEvidence {
    let first_occurrence = runtime
        .enqueue_owner(owner_attack_request())
        .expect("first owner request enqueues from admitted checked plan");
    let second_occurrence = runtime
        .enqueue_owner(owner_attack_request())
        .expect("second owner request enqueues from admitted checked plan");
    let first_owner_outcome = runtime
        .serve_next_owner(OWNER)
        .expect("first owner request serves in FIFO order");
    let second_owner_outcome = runtime
        .serve_next_owner(OWNER)
        .expect("second owner request observes the first write");
    assert_eq!(first_owner_outcome.read_int(&hp_key()), Some(100));
    assert_eq!(first_owner_outcome.written_int(&hp_key()), Some(90));
    assert_eq!(second_owner_outcome.read_int(&hp_key()), Some(90));
    assert_eq!(second_owner_outcome.written_int(&hp_key()), Some(80));

    let relation_transition = runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("relation invalidation advances the runtime to fallback");
    let relation_projection = runtime
        .project_relation(RELATION_NAME, degraded_relation_context())
        .expect("fallback relation projection is derived from runtime relation state");
    let designated_publication = runtime
        .evaluate_designated(evaluation_request())
        .expect("designated value publishes from the same runtime session");
    let designated_consumption = runtime
        .consume_published_value(consume_request())
        .expect("designated value consumes exactly once in the same session");
    runtime
        .attach_presentation_interpolation(
            VALUE_NAME,
            ResultVersion::new(1),
            M8PresentationInterpolation::for_consumer(CONSUMER)
                .with_frame("render-frame:final-replay")
                .with_display_hint_int(99),
        )
        .expect("presentation interpolation is executable but not cut state");

    let save_payload_before_cut = runtime.save_relevant_payload();
    let cut = runtime.save_local_cut("cut:final-deterministic-replay");
    let cut_payload = cut.save_relevant_payload();
    let trace_prefix = cut.trace_prefix();

    FinalReplayEvidence {
        first_occurrence,
        second_occurrence,
        first_owner_outcome,
        second_owner_outcome,
        relation_transition,
        relation_projection,
        designated_publication,
        designated_consumption,
        save_payload_before_cut,
        cut_payload,
        cut,
        trace_prefix,
    }
}

#[test]
fn final_deterministic_replay_matches_across_independent_unified_local_runtime_sessions() {
    let (path_a, source_a, checked_a, admission_a, mut runtime_a) = local_runtime();
    let (path_b, source_b, checked_b, admission_b, mut runtime_b) = local_runtime();
    let (_, _, _, _, instance_for_shape) = admitted_unified_instance();
    assert_unified_fixture_shape_and_exact_admission(
        &path_a,
        &source_a,
        &checked_a,
        &admission_a,
        &instance_for_shape,
    );
    assert_eq!(path_a, path_b);
    assert_eq!(source_a, source_b);
    assert_eq!(checked_a.program_identity(), checked_b.program_identity());
    assert_eq!(admission_a, admission_b);

    let evidence_a = apply_final_deterministic_replay_log(&mut runtime_a);
    let evidence_b = apply_final_deterministic_replay_log(&mut runtime_b);

    assert_eq!(evidence_a, evidence_b);
    assert_eq!(
        evidence_a.save_payload_before_cut,
        evidence_b.save_payload_before_cut
    );
    assert_eq!(
        runtime_a.save_relevant_payload(),
        runtime_b.save_relevant_payload()
    );
    assert_eq!(evidence_a.save_payload_before_cut, evidence_a.cut_payload);
    assert_eq!(evidence_b.save_payload_before_cut, evidence_b.cut_payload);
    assert_eq!(evidence_a.cut_payload, evidence_b.cut_payload);
    assert_eq!(evidence_a.cut, evidence_b.cut);
    assert_eq!(evidence_a.trace_prefix, evidence_b.trace_prefix);
    assert_eq!(runtime_a.trace(), evidence_a.trace_prefix);
    assert_eq!(runtime_b.trace(), evidence_b.trace_prefix);

    assert_eq!(
        evidence_a.first_owner_outcome.read_int(&hp_key()),
        Some(100)
    );
    assert_eq!(
        evidence_a.first_owner_outcome.written_int(&hp_key()),
        Some(90)
    );
    assert_eq!(
        evidence_a.second_owner_outcome.read_int(&hp_key()),
        Some(90)
    );
    assert_eq!(
        evidence_a.second_owner_outcome.written_int(&hp_key()),
        Some(80)
    );
    assert_eq!(evidence_a.cut.owner_state().int(&hp_key()), Some(80));
    assert!(evidence_a.cut.pending_owner_fifo(OWNER).is_empty());
    assert_eq!(evidence_a.relation_transition.previous_option_index(), 0);
    assert_eq!(evidence_a.relation_transition.current_option_index(), 1);
    assert_eq!(evidence_a.relation_projection.relation(), RELATION_NAME);
    assert_eq!(evidence_a.relation_projection.consumer_locus(), CONSUMER);
    assert_eq!(
        evidence_a.relation_projection.selected_anchor(),
        "nest_anchor"
    );
    assert_eq!(
        evidence_a.relation_projection.context_frontier(),
        DEGRADED_FRONTIER
    );
    let relation = evidence_a
        .cut
        .relation_state(RELATION_NAME)
        .expect("final replay cut includes fallback relation state");
    assert_eq!(relation.selected_option_index(), 1);
    assert_eq!(relation.activation_frontier(), DEGRADED_FRONTIER);
    assert_eq!(evidence_a.designated_publication.int_value(), Some(11));
    assert_eq!(
        evidence_a.designated_publication.result_version(),
        ResultVersion::new(1)
    );
    assert_eq!(evidence_a.designated_consumption.int_value(), Some(11));
    assert_eq!(
        evidence_a.designated_consumption.result_version(),
        ResultVersion::new(1)
    );
    assert_eq!(
        evidence_a
            .cut
            .designated_version_store()
            .version(VALUE_NAME),
        Some(ResultVersion::new(1))
    );
    assert_eq!(
        evidence_a
            .cut
            .designated_consumption_state()
            .consumed_deliveries(CONSUMER, VALUE_NAME),
        vec![CONSUME_DELIVERY_ID]
    );

    assert_eq!(
        evidence_a.trace_prefix.kinds(),
        vec![
            M8LocalTraceKind::OwnerEnqueued,
            M8LocalTraceKind::OwnerEnqueued,
            M8LocalTraceKind::OwnerAuthorityValidated,
            M8LocalTraceKind::OwnerRead,
            M8LocalTraceKind::OwnerWrite,
            M8LocalTraceKind::OwnerAuthorityValidated,
            M8LocalTraceKind::OwnerRead,
            M8LocalTraceKind::OwnerWrite,
            M8LocalTraceKind::RelationPrimaryInvalidated,
            M8LocalTraceKind::RelationOptionAdvanced,
            M8LocalTraceKind::DesignatedAuthorityValidated,
            M8LocalTraceKind::DesignatedInputReceiptValidated,
            M8LocalTraceKind::DesignatedValuePublished,
            M8LocalTraceKind::DesignatedConsumerAuthorityValidated,
            M8LocalTraceKind::DesignatedValueConsumed,
            M8LocalTraceKind::LocalCutSaved,
        ]
    );
    assert!(evidence_a.trace_prefix.node_ids_are_unique());
    assert!(evidence_a.trace_prefix.node_indexes_are_monotone());
    assert!(!evidence_a.trace_prefix.has_self_edges());
    assert!(
        evidence_a
            .trace_prefix
            .dependencies_only_name_earlier_nodes()
    );
    assert!(!evidence_a.cut.contains_presentation_contexts());
    assert!(!evidence_a.cut.contains_presentation_policies());
    assert!(!evidence_a.cut.contains_presentation_interpolations());
}

#[test]
fn exact_relation_live_lease_inventory_allows_projection_and_semantic_transition() {
    let (_, _, _, _, mut runtime) = local_runtime();
    let before_payload = runtime.save_relevant_payload();
    let before_relation = runtime
        .relation_state(RELATION_NAME)
        .expect("relation state exists before projection")
        .clone();

    runtime
        .project_relation(RELATION_NAME, primary_relation_context())
        .expect("exact declared live lease allows projection");
    assert_eq!(runtime.save_relevant_payload(), before_payload);
    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("projection leaves relation state unchanged"),
        &before_relation
    );

    let transition = runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("exact declared live lease allows semantic transition");
    assert_eq!(transition.previous_option_index(), 0);
    assert_eq!(transition.current_option_index(), 1);
    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("transition advances relation state")
            .activation_frontier(),
        DEGRADED_FRONTIER
    );
}

#[test]
fn relation_live_lease_inventory_shape_must_match_declared_relation_before_projection_or_transition()
 {
    let scenarios = vec![
        ("no lease", Vec::new()),
        ("bare lease", vec![M8LeaseRecord::live(RELATION_LEASE_REF)]),
        (
            "wrong relation",
            vec![
                M8LeaseRecord::live(RELATION_LEASE_REF)
                    .for_relation("other_relation")
                    .with_owner_locus(OWNER)
                    .with_binding_frontier(BINDING_FRONTIER)
                    .with_epoch("binding_epoch:1"),
            ],
        ),
        (
            "wrong owner locus",
            vec![
                M8LeaseRecord::live(RELATION_LEASE_REF)
                    .for_relation(RELATION_NAME)
                    .with_owner_locus("OtherOwner")
                    .with_binding_frontier(BINDING_FRONTIER)
                    .with_epoch("binding_epoch:1"),
            ],
        ),
        (
            "wrong binding frontier",
            vec![
                M8LeaseRecord::live(RELATION_LEASE_REF)
                    .for_relation(RELATION_NAME)
                    .with_owner_locus(OWNER)
                    .with_binding_frontier("other_binding_frontier")
                    .with_epoch("binding_epoch:1"),
            ],
        ),
        (
            "wrong binding epoch",
            vec![
                M8LeaseRecord::live(RELATION_LEASE_REF)
                    .for_relation(RELATION_NAME)
                    .with_owner_locus(OWNER)
                    .with_binding_frontier(BINDING_FRONTIER)
                    .with_epoch("binding_epoch:forged"),
            ],
        ),
    ];

    for (scenario, leases) in scenarios {
        let (_, _, _, _, mut runtime) = local_runtime_with_relation_lease_records(leases);
        let before_payload = runtime.save_relevant_payload();
        let before_relation = runtime
            .relation_state(RELATION_NAME)
            .unwrap_or_else(|| panic!("relation state exists before {scenario} lease validation"))
            .clone();

        let projection = match runtime.project_relation(RELATION_NAME, primary_relation_context()) {
            Ok(_) => panic!("projection must fail closed for {scenario} relation lease inventory"),
            Err(diagnostics) => diagnostics,
        };
        assert_eq!(
            projection.primary().kind(),
            mir_runtime::m8_runtime_relation_projection::M8ProjectionDiagnosticKind::MissingLiveRelationLease
        );
        assert_eq!(runtime.save_relevant_payload(), before_payload);
        assert_eq!(
            runtime
                .relation_state(RELATION_NAME)
                .expect("failed projection preserves relation state"),
            &before_relation
        );

        let transition = match runtime.invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        ) {
            Ok(_) => {
                panic!(
                    "semantic transition must fail closed for {scenario} relation lease inventory"
                )
            }
            Err(diagnostics) => diagnostics,
        };
        assert_eq!(
            transition.primary().kind(),
            mir_runtime::m8_runtime_relation_projection::M8RelationDiagnosticKind::MissingLiveRelationLease
        );
        assert_eq!(runtime.save_relevant_payload(), before_payload);
        assert_eq!(
            runtime
                .relation_state(RELATION_NAME)
                .expect("failed transition preserves relation state"),
            &before_relation
        );
    }
}

#[test]
fn fresh_reacquire_epoch_and_frontier_must_match_live_fresh_lease_inventory() {
    let (_, _, _, _, mut exact_runtime) = local_runtime();
    exact_runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("relation advances to fallback before exact reacquire");
    let accepted = exact_runtime
        .reacquire_primary(
            RELATION_NAME,
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier(REACQUIRED_FRONTIER),
        )
        .expect("exact pre-admitted fresh authority and live lease evidence reacquires primary");
    assert_eq!(accepted.previous_option_index(), 1);
    assert_eq!(accepted.current_option_index(), 0);
    assert_eq!(
        exact_runtime
            .relation_state(RELATION_NAME)
            .expect("exact reacquire restores primary")
            .activation_frontier(),
        REACQUIRED_FRONTIER
    );

    for (anchor_epoch, frontier) in [
        ("primary_epoch:forged", REACQUIRED_FRONTIER),
        ("primary_epoch:2", "bird_binding_frontier:forged"),
    ] {
        let (_, _, _, _, mut runtime) = local_runtime();
        runtime
            .invalidate_primary(
                RELATION_NAME,
                invalidate_relation_authority_use(),
                M8BindingInvalidation::anchor_unavailable("perch_anchor")
                    .with_frontier(DEGRADED_FRONTIER),
            )
            .expect("relation advances to fallback before forged reacquire");
        let before_payload = runtime.save_relevant_payload();
        let before_relation = runtime
            .relation_state(RELATION_NAME)
            .expect("relation state exists before forged reacquire")
            .clone();

        let rejected = runtime
            .reacquire_primary(
                RELATION_NAME,
                reacquire_relation_authority_use(),
                M8RelationReacquire::new("perch_anchor")
                    .with_anchor_epoch(anchor_epoch)
                    .with_binding_epoch("binding_epoch:2")
                    .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                    .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                    .with_frontier(frontier),
            )
            .expect_err("fresh reacquire must reject forged epoch/frontier values");
        assert_eq!(
            rejected.primary().kind(),
            mir_runtime::m8_runtime_relation_projection::M8RelationDiagnosticKind::MissingRelationAuthority
        );
        assert_eq!(runtime.save_relevant_payload(), before_payload);
        assert_eq!(
            runtime
                .relation_state(RELATION_NAME)
                .expect("forged reacquire preserves relation state"),
            &before_relation
        );
    }
}

#[test]
fn relation_admission_declares_lease_payload_but_operations_require_current_live_inventory() {
    let (_, _, _, admission, mut runtime) = local_runtime_without_live_relation_lease();
    assert_relation_admission_declares_lease_payload(admission.evidence());

    let cut_without_inventory = runtime.save_local_cut("cut:relation:declared-lease-no-inventory");
    assert_relation_admission_declares_lease_payload(
        cut_without_inventory.admission_provenance().evidence(),
    );
    assert!(
        !cut_without_inventory
            .lease_inventory()
            .contains_live(RELATION_LEASE_REF)
    );

    let before_payload = runtime.save_relevant_payload();
    let before_relation = runtime
        .relation_state(RELATION_NAME)
        .expect("relation state exists before lease validation")
        .clone();

    let projection = runtime
        .project_relation(RELATION_NAME, primary_relation_context())
        .expect_err("projection must fail closed when the declared lease is not live");
    assert_eq!(
        projection.primary().kind(),
        mir_runtime::m8_runtime_relation_projection::M8ProjectionDiagnosticKind::MissingLiveRelationLease
    );
    assert_eq!(runtime.save_relevant_payload(), before_payload);
    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("failed projection preserves relation state"),
        &before_relation
    );

    let transition = runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect_err("semantic relation transition must fail closed without a live lease");
    assert_eq!(
        transition.primary().kind(),
        mir_runtime::m8_runtime_relation_projection::M8RelationDiagnosticKind::MissingLiveRelationLease
    );
    assert_eq!(runtime.save_relevant_payload(), before_payload);
    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("failed transition preserves relation state"),
        &before_relation
    );

    let (_, _, _, exact_admission, mut exact_runtime) =
        local_runtime_with_relation_lease_records(vec![live_relation_lease()]);
    assert_relation_admission_declares_lease_payload(exact_admission.evidence());
    let cut_with_inventory = exact_runtime.save_local_cut("cut:relation:declared-lease-live");
    assert!(
        cut_with_inventory
            .lease_inventory()
            .contains_live(RELATION_LEASE_REF)
    );
    exact_runtime
        .project_relation(RELATION_NAME, primary_relation_context())
        .expect("exact current live inventory allows projection");
    exact_runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("exact current live inventory allows semantic relation transition");
}

#[test]
fn unified_saved_trace_retains_typed_failure_rows_from_all_m8_local_families() {
    let (_, _, _, _, mut runtime) = local_runtime();

    let unknown = runtime
        .enqueue_owner(
            M8OwnerRequest::new("unknown_attack").with_authority_use(valid_owner_authority_use()),
        )
        .expect_err("unknown owner enqueue rejects through M8LocalRuntime");
    assert_eq!(
        unknown.primary().kind(),
        M8EnqueueDiagnosticKind::UnknownEvaluation
    );

    let invalid_owner = runtime
        .enqueue_owner(owner_attack_request_with_authority(
            missing_capability_owner_authority_use(),
        ))
        .expect("known owner request with bad authority still gets an occurrence");
    let owner_failure = runtime
        .serve_next_owner(OWNER)
        .expect_err("serve rejects a queued occurrence with missing capability");
    assert_eq!(
        owner_failure.primary().kind(),
        M8ServeDiagnosticKind::DeclaredFailure(M8DeclaredFailure::MissingCapability)
    );

    runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("baseline relation invalidation reaches fallback before forged reacquire");
    let before_forged_relation = runtime
        .relation_state(RELATION_NAME)
        .expect("relation state exists before forged reacquire")
        .clone();
    let forged_relation = runtime
        .reacquire_primary(
            RELATION_NAME,
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness("witness:relation:bird_follow:forged")
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier(REACQUIRED_FRONTIER),
        )
        .expect_err("forged relation reacquire witness rejects");
    assert_eq!(
        forged_relation.primary().kind(),
        mir_runtime::m8_runtime_relation_projection::M8RelationDiagnosticKind::MissingRelationAuthority
    );
    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("forged relation transition preserves state"),
        &before_forged_relation
    );

    let invalid_evaluation = runtime
        .evaluate_designated(evaluation_request_with_authority(
            missing_evaluator_authority_use(),
        ))
        .expect_err("invalid designated evaluator authority rejects");
    assert_eq!(
        invalid_evaluation.primary().kind(),
        M8DesignatedDiagnosticKind::MissingEvaluatorAuthority
    );

    runtime
        .evaluate_designated(evaluation_request())
        .expect("valid designated publication exists before invalid consumption");
    let before_invalid_consume_store = runtime.designated_result_store().clone();
    let invalid_consumption = runtime
        .consume_published_value(consume_request_with_authority(
            missing_consumer_authority_use(),
        ))
        .expect_err("invalid designated consumer authority rejects");
    assert_eq!(
        invalid_consumption.primary().kind(),
        M8DesignatedDiagnosticKind::MissingConsumerAuthority
    );
    assert_eq!(
        runtime.designated_result_store(),
        &before_invalid_consume_store
    );

    let cut = runtime.save_local_cut("cut:unified:typed-failure-rows");
    let trace = cut.trace_prefix();
    let observer_rows = trace.observer_failure_rows();
    let observer_payload = observer_rows.to_redacted_structural_json();
    let observer_payload_json: serde_json::Value =
        serde_json::from_str(&observer_payload).expect("observer failure rows serialize as JSON");
    let observer_payload_rows = observer_payload_json
        .as_array()
        .expect("observer failure rows serialize as an array");

    let unknown_observed = observer_rows
        .owner_enqueue(M8EnqueueDiagnosticKind::UnknownEvaluation)
        .expect("observer projection retains unknown enqueue failure row");
    assert_eq!(unknown_observed.failure_family(), "owner_enqueue");
    assert_eq!(unknown_observed.failure_kind(), "UnknownEvaluation");
    assert_eq!(
        unknown_observed.source_ref(),
        unknown.primary().source_ref()
    );
    assert_eq!(unknown_observed.operation_occurrence_id(), None);
    assert!(unknown_observed.dependencies().is_empty());
    assert!(unknown_observed.authority_refs_are_redacted());
    assert_observer_failure_row_policy_is_serialized(unknown_observed, observer_payload_rows);

    let serve_observed = observer_rows
        .owner_serve(M8ServeDiagnosticKind::DeclaredFailure(
            M8DeclaredFailure::MissingCapability,
        ))
        .expect("observer projection retains invalid serve authority failure row");
    assert_eq!(serve_observed.failure_family(), "owner_serve");
    assert_eq!(
        serve_observed.failure_kind(),
        "DeclaredFailure::MissingCapability"
    );
    assert_eq!(
        serve_observed.source_ref(),
        owner_failure.primary().source_ref()
    );
    assert_eq!(
        serve_observed.operation_occurrence_id(),
        Some(invalid_owner.id())
    );
    assert!(serve_observed.has_actual_earlier_dependencies(&trace));
    assert!(serve_observed.authority_refs_are_redacted());
    assert_observer_failure_row_policy_is_serialized(serve_observed, observer_payload_rows);

    let relation_observed = observer_rows
        .relation_transition(
            mir_runtime::m8_runtime_relation_projection::M8RelationDiagnosticKind::MissingRelationAuthority,
        )
        .expect("observer projection retains forged relation transition failure row");
    assert_eq!(relation_observed.failure_family(), "relation_transition");
    assert_eq!(relation_observed.failure_kind(), "MissingRelationAuthority");
    assert_eq!(
        relation_observed.source_ref(),
        forged_relation.primary().source_ref()
    );
    assert!(relation_observed.has_actual_earlier_dependencies(&trace));
    assert!(relation_observed.authority_refs_are_redacted());
    assert_observer_failure_row_policy_is_serialized(relation_observed, observer_payload_rows);

    let eval_observed = observer_rows
        .designated_evaluation(M8DesignatedDiagnosticKind::MissingEvaluatorAuthority)
        .expect("observer projection retains invalid designated evaluation failure row");
    assert_eq!(eval_observed.failure_family(), "designated_evaluation");
    assert_eq!(eval_observed.failure_kind(), "MissingEvaluatorAuthority");
    assert_eq!(
        eval_observed.source_ref(),
        invalid_evaluation.primary().source_ref()
    );
    assert!(eval_observed.authority_refs_are_redacted());
    assert_observer_failure_row_policy_is_serialized(eval_observed, observer_payload_rows);

    let consume_observed = observer_rows
        .designated_consumption(M8DesignatedDiagnosticKind::MissingConsumerAuthority)
        .expect("observer projection retains invalid designated consumption failure row");
    assert_eq!(consume_observed.failure_family(), "designated_consumption");
    assert_eq!(consume_observed.failure_kind(), "MissingConsumerAuthority");
    assert_eq!(
        consume_observed.source_ref(),
        invalid_consumption.primary().source_ref()
    );
    assert!(consume_observed.has_actual_earlier_dependencies(&trace));
    assert!(consume_observed.authority_refs_are_redacted());
    assert_observer_failure_row_policy_is_serialized(consume_observed, observer_payload_rows);

    for raw_ref in [
        OWNER_MEMBERSHIP_REF,
        ABSENT_ATTACK_CAPABILITY_REF,
        ABSENT_ATTACK_WITNESS_REF,
        RELATION_MEMBERSHIP_EPOCH2_REF,
        REACQUIRE_RELATION_CAPABILITY_REF,
        REACQUIRE_RELATION_WITNESS_REF,
        ABSENT_EVALUATE_CAPABILITY_REF,
        ABSENT_EVALUATE_WITNESS_REF,
        ABSENT_CONSUME_CAPABILITY_REF,
        ABSENT_CONSUME_WITNESS_REF,
    ] {
        assert!(
            !observer_payload.contains(raw_ref),
            "observer projection must redact raw authority ref {raw_ref}"
        );
    }
    assert!(observer_payload.contains("failure_family"));
    assert!(observer_payload.contains("source_ref"));
    assert!(observer_payload.contains("dependencies"));
    assert!(observer_payload.contains("label"));
    assert!(observer_payload.contains("redaction"));
}

#[test]
fn unified_local_cut_save_contains_real_m8_state_and_excludes_presentation() {
    let (path, source, checked, admission, mut runtime, _, second_id) =
        runtime_with_pending_second_attack();
    let (_, _, _, _, instance_for_shape) = admitted_unified_instance();
    assert_unified_fixture_shape_and_exact_admission(
        &path,
        &source,
        &checked,
        &admission,
        &instance_for_shape,
    );

    runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("real unified runtime advances relation to fallback");
    runtime
        .evaluate_designated(evaluation_request())
        .expect("real unified runtime publishes designated value");
    runtime
        .consume_published_value(consume_request())
        .expect("real unified runtime consumes designated value once");
    runtime
        .project_relation(RELATION_NAME, degraded_relation_context())
        .expect("presentation projection is executable but not saved");
    runtime
        .attach_presentation_interpolation(
            VALUE_NAME,
            ResultVersion::new(1),
            M8PresentationInterpolation::for_consumer(CONSUMER)
                .with_frame("render-frame:2")
                .with_display_hint_int(99),
        )
        .expect("presentation interpolation is executable but not saved");

    let cut = runtime.save_local_cut("cut:unified:after-real-actions");

    assert_eq!(cut.program_identity(), checked.program_identity());
    assert_eq!(
        cut.admission_provenance().program_identity(),
        admission.program_identity()
    );
    assert_eq!(cut.admission_provenance().evidence(), admission.evidence());
    assert!(cut.admission_provenance().uses_structural_equality());
    assert!(!cut.admission_provenance().uses_hash_fingerprint());
    assert_eq!(cut.owner_state().int(&hp_key()), Some(90));
    assert_eq!(cut.owner_state().int(&atk_key()), Some(10));
    assert_eq!(cut.pending_owner_fifo(OWNER), vec![second_id.as_str()]);
    assert_eq!(cut.owner_counters().next_owner_occurrence_index(), 2);
    assert!(
        cut.authority_inventory()
            .contains_membership(OWNER_MEMBERSHIP_REF)
    );
    assert!(
        cut.authority_inventory()
            .contains_capability(ATTACK_CAPABILITY_REF)
    );
    assert!(
        cut.authority_inventory()
            .contains_witness(ATTACK_WITNESS_REF)
    );
    assert!(cut.lease_inventory().contains_live(RELATION_LEASE_REF));
    let relation = cut
        .relation_state(RELATION_NAME)
        .expect("cut includes owner-held relation state");
    assert_eq!(relation.selected_option_index(), 1);
    assert_eq!(relation.selected_anchor(), "nest_anchor");
    assert_eq!(relation.activation_frontier(), DEGRADED_FRONTIER);
    assert!(relation.lineage_contains("advance"));
    assert_eq!(
        cut.designated_receipt_state()
            .receipt(INPUT_RECEIPT_REF)
            .expect("cut includes designated receipt")
            .label()
            .security_class(),
        M8SecurityClass::Restricted
    );
    assert!(
        cut.designated_result_store()
            .published_value(VALUE_NAME, ResultVersion::new(1))
            .is_some()
    );
    assert_eq!(
        cut.designated_version_store().version(VALUE_NAME),
        Some(ResultVersion::new(1))
    );
    assert_eq!(
        cut.designated_consumption_state()
            .consumed_deliveries(CONSUMER, VALUE_NAME),
        vec![CONSUME_DELIVERY_ID]
    );
    assert_eq!(
        cut.patch_lifecycle().state(),
        M8LocalPatchLifecycleState::Placeholder
    );
    assert!(cut.patch_lifecycle().rows().is_empty());
    assert_eq!(
        cut.trace_prefix(),
        runtime.trace().prefix(cut.trace_prefix().len())
    );
    assert_eq!(
        cut.trace_prefix().kinds(),
        vec![
            M8LocalTraceKind::OwnerEnqueued,
            M8LocalTraceKind::OwnerEnqueued,
            M8LocalTraceKind::OwnerAuthorityValidated,
            M8LocalTraceKind::OwnerRead,
            M8LocalTraceKind::OwnerWrite,
            M8LocalTraceKind::RelationPrimaryInvalidated,
            M8LocalTraceKind::RelationOptionAdvanced,
            M8LocalTraceKind::DesignatedAuthorityValidated,
            M8LocalTraceKind::DesignatedInputReceiptValidated,
            M8LocalTraceKind::DesignatedValuePublished,
            M8LocalTraceKind::DesignatedConsumerAuthorityValidated,
            M8LocalTraceKind::DesignatedValueConsumed,
            M8LocalTraceKind::LocalCutSaved,
        ]
    );
    assert!(cut.trace_prefix().node_indexes_are_monotone());
    assert!(!cut.trace_prefix().has_self_edges());
    assert!(cut.trace_prefix().dependencies_only_name_earlier_nodes());
    assert!(!cut.contains_presentation_contexts());
    assert!(!cut.contains_presentation_policies());
    assert!(!cut.contains_presentation_interpolations());
}

#[test]
fn same_current_floor_restore_roundtrips_and_resumes_pending_owner_fifo_deterministically() {
    let (_, _, _, _, runtime, first_id, second_id) = runtime_with_pending_second_attack();
    let cut = runtime.save_local_cut("cut:pending-second-owner-request");
    let floor = M8LiveFloor::same_current(&cut);
    let (_, _, _, _, mut restored) = local_runtime();

    restored
        .try_restore_local_cut(&cut, &floor)
        .expect("same-current live floor restores the saved local cut");

    assert_eq!(
        restored.save_relevant_payload(),
        cut.save_relevant_payload()
    );
    assert!(restored.trace().starts_with(cut.trace_prefix()));
    assert_eq!(restored.pending_owner_fifo(OWNER), vec![second_id.as_str()]);
    assert_eq!(restored.owner_state().int(&hp_key()), Some(90));
    let resumed = restored
        .serve_next_owner(OWNER)
        .expect("restored runtime resumes pending second owner request");
    assert_eq!(resumed.read_int(&hp_key()), Some(90));
    assert_eq!(resumed.written_int(&hp_key()), Some(80));
    assert_eq!(restored.owner_state().int(&hp_key()), Some(80));

    let fresh = restored
        .enqueue_owner(owner_attack_request())
        .expect("restored occurrence counters resume without collision");
    assert_ne!(fresh.id(), first_id);
    assert_ne!(fresh.id(), second_id);
    assert!(restored.trace().node_ids_are_unique());
}

#[test]
fn restore_rejects_authority_or_lease_non_live_floor_without_saved_payload_mutation() {
    let (_, _, _, _, runtime, _, _) = runtime_with_pending_second_attack();
    let cut = runtime.save_local_cut("cut:authority-floor");

    for (floor, expected_kind) in [
        (
            M8LiveFloor::same_current(&cut).with_stale_membership(OWNER_MEMBERSHIP_REF),
            M8LocalRestoreDiagnosticKind::StaleMembership,
        ),
        (
            M8LiveFloor::same_current(&cut).with_revoked_capability(ATTACK_CAPABILITY_REF),
            M8LocalRestoreDiagnosticKind::RevokedCapability,
        ),
        (
            M8LiveFloor::same_current(&cut).with_stale_witness(ATTACK_WITNESS_REF),
            M8LocalRestoreDiagnosticKind::StaleWitness,
        ),
        (
            M8LiveFloor::same_current(&cut).with_expired_lease(RELATION_LEASE_REF),
            M8LocalRestoreDiagnosticKind::ExpiredLease,
        ),
    ] {
        let (_, _, _, _, mut restore_target) = local_runtime();
        assert_restore_rejects_without_payload_or_cut_mutation(
            &mut restore_target,
            &cut,
            floor,
            expected_kind,
        );
    }
}

#[test]
fn restore_rejects_designated_consumption_or_result_version_rollback_without_payload_mutation() {
    enum RollbackScenario {
        ConsumedDelivery,
        ResultVersion,
    }

    for scenario in [
        RollbackScenario::ConsumedDelivery,
        RollbackScenario::ResultVersion,
    ] {
        let mut runtime = runtime_after_designated_publication();
        let older_cut = runtime.save_local_cut("cut:designated:before-floor-advance");
        let floor = match scenario {
            RollbackScenario::ConsumedDelivery => {
                runtime
                    .consume_published_value(consume_request())
                    .expect("live floor advances consumed delivery after the older cut");
                M8LiveFloor::from_runtime(&runtime)
            }
            RollbackScenario::ResultVersion => M8LiveFloor::same_current(&older_cut)
                .with_result_version_floor(VALUE_NAME, ResultVersion::new(2)),
        };
        let expected_kind = match scenario {
            RollbackScenario::ConsumedDelivery => {
                M8LocalRestoreDiagnosticKind::ConsumedDeliveryRollback
            }
            RollbackScenario::ResultVersion => M8LocalRestoreDiagnosticKind::ResultVersionRollback,
        };

        assert_restore_rejects_without_payload_or_cut_mutation(
            &mut runtime,
            &older_cut,
            floor,
            expected_kind,
        );
        assert!(
            runtime
                .designated_result_store()
                .published_value(VALUE_NAME, ResultVersion::new(1))
                .is_some()
        );
    }
}

#[test]
fn restore_rejects_old_relation_lineage_without_relation_mutation_or_cut_rewrite() {
    let (_, _, _, _, mut runtime) = local_runtime();
    let older_cut = runtime.save_local_cut("cut:relation:old-lineage");
    runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("relation advances before newer floor");
    runtime
        .reacquire_primary(
            RELATION_NAME,
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier(REACQUIRED_FRONTIER),
        )
        .expect("fresh reacquire establishes a newer relation lineage floor");
    let before_relation = runtime
        .relation_state(RELATION_NAME)
        .expect("current relation state exists")
        .clone();
    let floor = M8LiveFloor::from_runtime(&runtime);

    assert_restore_rejects_without_payload_or_cut_mutation(
        &mut runtime,
        &older_cut,
        floor,
        M8LocalRestoreDiagnosticKind::OldRelationLineage,
    );

    assert_eq!(
        runtime
            .relation_state(RELATION_NAME)
            .expect("rejected restore keeps current relation state"),
        &before_relation
    );
    assert_eq!(
        older_cut
            .relation_state(RELATION_NAME)
            .expect("saved old relation state remains readable")
            .activation_frontier(),
        BINDING_FRONTIER
    );
}
