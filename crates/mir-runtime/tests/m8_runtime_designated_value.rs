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
    M8ConsumeRequest, M8DesignatedAuthorityUse, M8DesignatedDiagnosticKind,
    M8DesignatedEvaluationRequest, M8DesignatedReplayLog, M8DesignatedReplayReport,
    M8DesignatedRuntime, M8DesignatedSeed, M8DesignatedTick, M8DesignatedTraceKind, M8InputReceipt,
    M8InputReceiptSet, M8PresentationInterpolation,
};
use mir_runtime::m8_runtime_owner_queue::M8StateKey;
use mir_semantics::{
    shared_model::{ResultVersion, SourceRef},
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const VALUE_NAME: &str = "E.result";
const EVALUATOR: &str = "E";
const RESULT_NAME: &str = "result";
const CONSUMER: &str = "C";
const SOURCE_OWNER: &str = "S";
const INPUT_FRONTIER: &str = "F";
const RESULT_VERSION: u64 = 1;
const INPUT_LABEL: &str = "input:player[self].atk:S:F";
const VALUE_LABEL: &str = "value:E.result:publish";
const REDACTION_POLICY: &str = "conservative";
const INPUT_RECEIPT_REF: &str = "receipt:S:player[self].atk:E:F:1";
const WRONG_SOURCE_RECEIPT_REF: &str = "receipt:C:player[self].atk:E:F:1";
const WRONG_FRONTIER_RECEIPT_REF: &str = "receipt:S:player[self].atk:E:other-F:1";
const STALE_RECEIPT_REF: &str = "receipt:S:player[self].atk:E:F:stale";
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

fn checked_designated_fixture() -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture("designated_tick_publish_result.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("designated fixture checks through M7 before M8 admission");
    (path, source, checked)
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

fn input_label(security_class: M8SecurityClass) -> EvidenceSecurityLabel {
    EvidenceSecurityLabel::new(INPUT_LABEL).with_class(security_class)
}

fn output_label(security_class: M8SecurityClass) -> EvidenceSecurityLabel {
    EvidenceSecurityLabel::new(VALUE_LABEL).with_class(security_class)
}

fn designated_visibility_evidence(
    source_ref: SourceRef,
    security_class: M8SecurityClass,
) -> M8AdmissionEvidence {
    M8AdmissionEvidence::ValueVisibilityRedaction {
        value: VALUE_NAME.into(),
        label: output_label(security_class),
        redaction: EvidenceRedaction::new(REDACTION_POLICY),
        source_ref,
    }
}

fn designated_admission_for(
    checked: &CheckedSurfaceV0,
    output_class: M8SecurityClass,
) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone()).with_evidence(
        designated_visibility_evidence(
            residual_source_ref(
                checked,
                ResidualObligationKind::ValueVisibilityRedaction,
                VALUE_NAME,
            ),
            output_class,
        ),
    )
}

fn admitted_designated_instance_with_output_class(
    output_class: M8SecurityClass,
) -> (String, String, CheckedSurfaceV0, M8RuntimeInstance) {
    let (path, source, checked) = checked_designated_fixture();
    let admission = designated_admission_for(&checked, output_class);
    let instance = M8Runtime::default()
        .admit(checked.clone(), admission)
        .expect("exact value visibility/redaction evidence admits through M8 Phase 1");
    (path, source, checked, instance)
}

fn admitted_designated_instance() -> (String, String, CheckedSurfaceV0, M8RuntimeInstance) {
    admitted_designated_instance_with_output_class(M8SecurityClass::Private)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
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

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
    let range = byte_range(source, lexeme);
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

fn designated_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "designated evaluate E on tick F publish result = player[self].atk + 1",
    )
}

fn input_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(path, source, "player[self].atk")
}

fn atk_key() -> M8StateKey {
    M8StateKey::indexed_field("player", "self", "atk")
}

fn designated_authority_state() -> M8AuthorityState {
    M8AuthorityState::new()
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
                .with_result_version(ResultVersion::new(RESULT_VERSION))
                .with_epoch("consume_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(CONSUME_WITNESS_REF)
                .for_capability(CONSUME_CAPABILITY_REF)
                .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
                .with_epoch("consume_epoch:1"),
        )
}

fn evaluator_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_evaluator(EVALUATOR)
        .with_principal("self")
        .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
        .with_capability_ref(EVALUATE_CAPABILITY_REF)
        .with_witness_ref(EVALUATE_WITNESS_REF)
}

fn invalid_evaluator_authority_use() -> M8DesignatedAuthorityUse {
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

fn invalid_consumer_authority_use() -> M8DesignatedAuthorityUse {
    M8DesignatedAuthorityUse::for_consumer(CONSUMER)
        .with_principal("self")
        .with_membership_ref(CONSUMER_MEMBERSHIP_REF)
        .with_capability_ref(ABSENT_CONSUME_CAPABILITY_REF)
        .with_witness_ref(ABSENT_CONSUME_WITNESS_REF)
}

fn input_receipt_with_class(
    source_ref: &SourceRef,
    security_class: M8SecurityClass,
) -> M8InputReceipt {
    M8InputReceipt::live(INPUT_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(SOURCE_OWNER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier(INPUT_FRONTIER)
        .with_source_ref(source_ref.clone())
        .with_label(input_label(security_class))
        .with_int_value(10)
}

fn valid_input_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    input_receipt_with_class(source_ref, M8SecurityClass::Restricted)
}

fn private_input_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    input_receipt_with_class(source_ref, M8SecurityClass::Private)
}

fn wrong_source_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    M8InputReceipt::live(WRONG_SOURCE_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(CONSUMER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier(INPUT_FRONTIER)
        .with_source_ref(source_ref.clone())
        .with_label(input_label(M8SecurityClass::Restricted))
        .with_int_value(10)
}

fn wrong_frontier_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    M8InputReceipt::live(WRONG_FRONTIER_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(SOURCE_OWNER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier("other-F")
        .with_source_ref(source_ref.clone())
        .with_label(input_label(M8SecurityClass::Restricted))
        .with_int_value(10)
}

fn stale_input_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    M8InputReceipt::stale(STALE_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(SOURCE_OWNER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier(INPUT_FRONTIER)
        .with_source_ref(source_ref.clone())
        .with_label(input_label(M8SecurityClass::Restricted))
        .with_int_value(10)
}

fn receipt_set(receipts: Vec<M8InputReceipt>) -> M8InputReceiptSet {
    let mut set = M8InputReceiptSet::new();
    for receipt in receipts {
        set = set.with_receipt(receipt);
    }
    set
}

fn designated_runtime_with_output_class_and_receipts(
    output_class: M8SecurityClass,
    receipts: Vec<M8InputReceipt>,
) -> M8DesignatedRuntime {
    let (_, _, _, instance) = admitted_designated_instance_with_output_class(output_class);
    instance.into_designated_values(
        M8DesignatedSeed::new()
            .with_authority_state(designated_authority_state())
            .with_input_receipts(receipt_set(receipts)),
    )
}

fn designated_runtime_with_receipts(receipts: Vec<M8InputReceipt>) -> M8DesignatedRuntime {
    designated_runtime_with_output_class_and_receipts(M8SecurityClass::Private, receipts)
}

fn designated_runtime(source_ref: &SourceRef) -> M8DesignatedRuntime {
    designated_runtime_with_receipts(vec![valid_input_receipt(source_ref)])
}

fn evaluation_tick() -> M8DesignatedTick {
    M8DesignatedTick::new("tick:F:1").with_input_frontier(INPUT_FRONTIER)
}

fn evaluation_request(authority: M8DesignatedAuthorityUse) -> M8DesignatedEvaluationRequest {
    M8DesignatedEvaluationRequest::for_value(VALUE_NAME)
        .with_tick(evaluation_tick())
        .with_authority(authority)
}

fn consume_request(authority: M8DesignatedAuthorityUse) -> M8ConsumeRequest {
    M8ConsumeRequest::for_value(VALUE_NAME)
        .with_consumer(CONSUMER)
        .with_delivery_id(CONSUME_DELIVERY_ID)
        .with_authority(authority)
}

#[test]
fn designated_evaluation_requires_source_owner_bound_input_receipt_without_hidden_owner_read_or_communication()
 {
    let (path, source, checked, _) = admitted_designated_instance();
    let designated_ref = designated_source_ref(&path, &source);
    let input_ref = input_source_ref(&path, &source);
    assert_eq!(
        residual_source_ref(
            &checked,
            ResidualObligationKind::ValueVisibilityRedaction,
            VALUE_NAME
        ),
        designated_ref
    );

    let receipt = valid_input_receipt(&input_ref);
    assert_eq!(receipt.source_owner_locus(), SOURCE_OWNER);
    assert_eq!(receipt.evaluator(), EVALUATOR);
    assert_eq!(receipt.input_frontier(), INPUT_FRONTIER);
    assert_eq!(receipt.source_ref(), &input_ref);
    assert_eq!(receipt.label().as_str(), INPUT_LABEL);
    assert_eq!(
        receipt.label().security_class(),
        M8SecurityClass::Restricted
    );
    assert_eq!(receipt.int_value(), Some(10));
    assert_eq!(
        receipt.generated_dependency_path().source_owner_locus(),
        SOURCE_OWNER
    );
    assert_eq!(receipt.generated_dependency_path().evaluator(), EVALUATOR);
    assert_eq!(
        receipt.generated_dependency_path().input_frontier(),
        INPUT_FRONTIER
    );
    assert_eq!(receipt.generated_dependency_path().source_ref(), &input_ref);

    for (receipts, expected_kind) in [
        (Vec::new(), M8DesignatedDiagnosticKind::MissingInputReceipt),
        (
            vec![wrong_source_receipt(&input_ref)],
            M8DesignatedDiagnosticKind::InputReceiptSourceMismatch,
        ),
        (
            vec![wrong_frontier_receipt(&input_ref)],
            M8DesignatedDiagnosticKind::InputReceiptFrontierMismatch,
        ),
    ] {
        let mut runtime = designated_runtime_with_receipts(receipts);
        let before = runtime.semantic_snapshot();
        let diagnostics = runtime
            .evaluate_designated(evaluation_request(evaluator_authority_use()))
            .expect_err("receipt missing or not bound to S/E/F rejects");

        assert_eq!(diagnostics.primary().kind(), expected_kind);
        assert_eq!(diagnostics.primary().source_ref(), &designated_ref);
        assert_eq!(runtime.semantic_snapshot(), before);
        assert!(
            runtime
                .result_store()
                .success_publications(VALUE_NAME)
                .is_empty()
        );
        assert!(
            runtime
                .result_store()
                .published_values(VALUE_NAME)
                .is_empty()
        );
        assert!(runtime.owner_store_reads().is_empty());
        assert!(runtime.hidden_communications().is_empty());
    }
}

#[test]
fn valid_designated_evaluation_publishes_one_retained_policy_version_with_authority_and_visibility()
{
    let (path, source, checked, instance) = admitted_designated_instance();
    let input_ref = input_source_ref(&path, &source);
    let checked_designated = checked
        .designated_result(EVALUATOR, RESULT_NAME)
        .expect("M7 designated result exists")
        .designated_core()
        .expect("M7 designated checked Core exists");
    let admitted_value = instance
        .designated_value(VALUE_NAME)
        .expect("M8 Phase 1 retains designated value metadata")
        .clone();
    assert_eq!(
        admitted_value.visibility_label().security_class(),
        M8SecurityClass::Private
    );
    let mut runtime = instance.into_designated_values(
        M8DesignatedSeed::new()
            .with_authority_state(designated_authority_state())
            .with_input_receipts(receipt_set(vec![valid_input_receipt(&input_ref)])),
    );

    let publication = runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect("valid authority and source-owner receipt evaluate designated value");

    assert_eq!(publication.value_name(), VALUE_NAME);
    assert_eq!(publication.evaluator(), EVALUATOR);
    assert_eq!(publication.result(), RESULT_NAME);
    assert_eq!(publication.logical_tick().input_frontier(), INPUT_FRONTIER);
    assert_eq!(
        publication.input_frontier(),
        admitted_value.input_frontier()
    );
    assert_eq!(
        publication.result_frontier(),
        admitted_value.result_frontier()
    );
    assert_eq!(
        publication.result_version(),
        ResultVersion::new(RESULT_VERSION)
    );
    assert_eq!(
        publication.result_version(),
        checked_designated.result_version()
    );
    assert_eq!(
        publication.evaluation_policy(),
        admitted_value.evaluation_policy()
    );
    assert_eq!(
        publication.observation_policy(),
        admitted_value.observation_policy()
    );
    assert_eq!(publication.policy_stamp(), admitted_value.policy_stamp());
    assert_eq!(publication.int_value(), Some(11));
    assert_eq!(
        publication.authority().capability_ref(),
        Some(EVALUATE_CAPABILITY_REF)
    );
    assert_eq!(
        publication.authority().witness_ref(),
        Some(EVALUATE_WITNESS_REF)
    );
    assert_eq!(publication.visibility_label().as_str(), VALUE_LABEL);
    assert_eq!(
        publication.visibility_label().security_class(),
        M8SecurityClass::Private
    );
    assert_eq!(
        publication.input_security_class_join(),
        M8SecurityClass::Restricted
    );
    assert!(
        publication
            .visibility_label()
            .security_class()
            .is_at_least(publication.input_security_class_join())
    );
    assert_eq!(publication.redaction().as_str(), REDACTION_POLICY);
    assert_eq!(
        runtime
            .trace()
            .causal_chain_for(publication.occurrence_id()),
        vec![
            M8DesignatedTraceKind::AuthorityValidated,
            M8DesignatedTraceKind::InputReceiptValidated,
            M8DesignatedTraceKind::ValuePublished,
        ]
    );
    assert!(
        runtime
            .trace()
            .authority_precedes_receipt_provenance_access(publication.occurrence_id())
    );
    assert!(
        runtime
            .trace()
            .authority_precedes_receipt_value_access(publication.occurrence_id())
    );
    assert_eq!(
        runtime
            .result_store()
            .success_publications(VALUE_NAME)
            .len(),
        1
    );
    assert!(runtime.authority_state().issued_by_m8().is_empty());
}

#[test]
fn duplicate_designated_evaluation_for_same_frontier_is_idempotent_without_second_success_publication()
 {
    let (path, source, _, _) = admitted_designated_instance();
    let input_ref = input_source_ref(&path, &source);
    let mut runtime = designated_runtime(&input_ref);

    let first = runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect("first evaluation publishes value");
    let second = runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect("duplicate evaluation returns stored value");

    assert_eq!(first.value_id(), second.value_id());
    assert_eq!(first.int_value(), second.int_value());
    assert_eq!(first.result_version(), second.result_version());
    assert_eq!(
        runtime
            .result_store()
            .success_publications(VALUE_NAME)
            .len(),
        1
    );
    assert_eq!(
        runtime
            .trace()
            .kinds()
            .into_iter()
            .filter(|kind| *kind == M8DesignatedTraceKind::ValuePublished)
            .count(),
        1
    );
}

#[test]
fn consumer_consumes_published_decision_once_without_semantic_reevaluation_or_presentation_mutation()
 {
    let (path, source, _, _) = admitted_designated_instance();
    let input_ref = input_source_ref(&path, &source);
    let mut runtime = designated_runtime(&input_ref);
    runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect("value is published before consumption");
    let stored_before_consume = runtime
        .result_store()
        .published_value(VALUE_NAME, ResultVersion::new(RESULT_VERSION))
        .expect("published value is stored")
        .clone();

    let consumed = runtime
        .consume_published_value(consume_request(consume_authority_use()))
        .expect("consumer has pre-admitted consumption capability");

    assert_eq!(consumed.consumer_locus(), CONSUMER);
    assert_eq!(consumed.value_name(), VALUE_NAME);
    assert_eq!(consumed.int_value(), Some(11));
    assert_eq!(
        consumed.result_version(),
        ResultVersion::new(RESULT_VERSION)
    );
    assert_eq!(
        consumed.authority().capability_ref(),
        Some(CONSUME_CAPABILITY_REF)
    );
    assert!(!consumed.reevaluated_semantics());
    assert!(
        !runtime
            .consumer_api(CONSUMER)
            .can_semantically_reevaluate(VALUE_NAME)
    );
    assert_eq!(
        runtime
            .result_store()
            .published_value(VALUE_NAME, ResultVersion::new(RESULT_VERSION)),
        Some(&stored_before_consume)
    );

    let snapshot_before_presentation = runtime.semantic_snapshot();
    let store_before_presentation = runtime.result_store().clone();
    let version_before_presentation = runtime.version_store().clone();
    let consumption_before_presentation = runtime.consumption_state().clone();
    runtime
        .attach_presentation_interpolation(
            VALUE_NAME,
            ResultVersion::new(RESULT_VERSION),
            M8PresentationInterpolation::for_consumer(CONSUMER)
                .with_frame("render-frame:2")
                .with_display_hint_int(99),
        )
        .expect("presentation metadata can be attached locally");
    assert_eq!(runtime.semantic_snapshot(), snapshot_before_presentation);
    assert_eq!(runtime.result_store(), &store_before_presentation);
    assert_eq!(runtime.version_store(), &version_before_presentation);
    assert_eq!(
        runtime.consumption_state(),
        &consumption_before_presentation
    );
    assert_eq!(
        runtime
            .result_store()
            .published_value(VALUE_NAME, ResultVersion::new(RESULT_VERSION)),
        Some(&stored_before_consume)
    );

    let before_duplicate = runtime.semantic_snapshot();
    let duplicate = runtime
        .consume_published_value(consume_request(consume_authority_use()))
        .expect_err("same delivery cannot consume the semantic value twice");
    assert_eq!(
        duplicate.primary().kind(),
        M8DesignatedDiagnosticKind::AlreadyConsumed
    );
    assert_eq!(runtime.semantic_snapshot(), before_duplicate);
    assert_eq!(
        runtime
            .consumption_state()
            .consumed_deliveries(CONSUMER, VALUE_NAME),
        vec![CONSUME_DELIVERY_ID]
    );
}

#[test]
fn invalid_consumer_authority_does_not_reveal_whether_designated_value_was_published() {
    fn invalid_consume_kind_preserving_state(
        runtime: &mut M8DesignatedRuntime,
    ) -> M8DesignatedDiagnosticKind {
        let before_result_store = runtime.result_store().clone();
        let before_receipts = runtime.receipt_state().clone();
        let before_consumption = runtime.consumption_state().clone();
        let before_success_consumptions = runtime
            .trace()
            .kinds()
            .into_iter()
            .filter(|kind| *kind == M8DesignatedTraceKind::ValueConsumed)
            .count();

        let diagnostics = runtime
            .consume_published_value(consume_request(invalid_consumer_authority_use()))
            .expect_err("invalid consumer authority must not consume the designated value");
        let kind = diagnostics.primary().kind();

        assert_eq!(runtime.result_store(), &before_result_store);
        assert_eq!(runtime.receipt_state(), &before_receipts);
        assert_eq!(runtime.consumption_state(), &before_consumption);
        assert_eq!(
            runtime
                .trace()
                .kinds()
                .into_iter()
                .filter(|kind| *kind == M8DesignatedTraceKind::ValueConsumed)
                .count(),
            before_success_consumptions
        );
        assert!(
            runtime
                .trace()
                .contains_failure(M8DesignatedTraceKind::ConsumptionRejected, kind)
        );
        kind
    }

    let (path, source, _, _) = admitted_designated_instance();
    let input_ref = input_source_ref(&path, &source);
    let mut unpublished_runtime = designated_runtime(&input_ref);
    let mut published_runtime = designated_runtime(&input_ref);
    published_runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect("published branch has a stored value before invalid consumption");

    let unpublished_kind = invalid_consume_kind_preserving_state(&mut unpublished_runtime);
    let published_kind = invalid_consume_kind_preserving_state(&mut published_runtime);

    assert_eq!(
        [unpublished_kind, published_kind],
        [
            M8DesignatedDiagnosticKind::MissingConsumerAuthority,
            M8DesignatedDiagnosticKind::MissingConsumerAuthority,
        ],
        "invalid consumer authority must be checked before publication existence is observable"
    );
}

#[test]
fn designated_evaluation_failure_preserves_result_and_consumption_state_except_typed_failure_trace()
{
    let (path, source, _, _) = admitted_designated_instance();
    let input_ref = input_source_ref(&path, &source);

    for (receipts, authority, expected_kind) in [
        (
            Vec::new(),
            evaluator_authority_use(),
            M8DesignatedDiagnosticKind::MissingInputReceipt,
        ),
        (
            vec![stale_input_receipt(&input_ref)],
            evaluator_authority_use(),
            M8DesignatedDiagnosticKind::StaleInputReceipt,
        ),
        (
            vec![valid_input_receipt(&input_ref)],
            invalid_evaluator_authority_use(),
            M8DesignatedDiagnosticKind::MissingEvaluatorAuthority,
        ),
    ] {
        let mut runtime = designated_runtime_with_receipts(receipts);
        let before_store = runtime.result_store().clone();
        let before_consumption = runtime.consumption_state().clone();
        let before_receipts = runtime.receipt_state().clone();
        let before_successes = runtime.trace().success_publication_count();

        let diagnostics = runtime
            .evaluate_designated(evaluation_request(authority))
            .expect_err("failed designated evaluation must be typed and non-mutating");

        assert_eq!(diagnostics.primary().kind(), expected_kind);
        assert_eq!(runtime.result_store(), &before_store);
        assert_eq!(runtime.consumption_state(), &before_consumption);
        assert_eq!(runtime.receipt_state(), &before_receipts);
        assert_eq!(
            runtime.trace().success_publication_count(),
            before_successes
        );
        assert!(
            runtime
                .result_store()
                .success_publications(VALUE_NAME)
                .is_empty()
        );
        assert!(
            runtime
                .trace()
                .contains_failure(M8DesignatedTraceKind::EvaluationFailed, expected_kind)
        );
        if expected_kind == M8DesignatedDiagnosticKind::MissingEvaluatorAuthority {
            assert!(
                !runtime
                    .trace()
                    .kinds()
                    .contains(&M8DesignatedTraceKind::InputReceiptValidated)
            );
            assert!(
                !runtime
                    .trace()
                    .contains_receipt_provenance_or_value_access()
            );
        }
    }

    let mut underclassified_runtime = designated_runtime_with_output_class_and_receipts(
        M8SecurityClass::Restricted,
        vec![private_input_receipt(&input_ref)],
    );
    let before_store = underclassified_runtime.result_store().clone();
    let before_consumption = underclassified_runtime.consumption_state().clone();
    let before_receipts = underclassified_runtime.receipt_state().clone();
    let before_successes = underclassified_runtime.trace().success_publication_count();
    let diagnostics = underclassified_runtime
        .evaluate_designated(evaluation_request(evaluator_authority_use()))
        .expect_err("output class below input class join rejects before success publication");

    assert_eq!(
        diagnostics.primary().kind(),
        M8DesignatedDiagnosticKind::OutputVisibilityWouldWeakenInput
    );
    assert_eq!(underclassified_runtime.result_store(), &before_store);
    assert_eq!(
        underclassified_runtime.consumption_state(),
        &before_consumption
    );
    assert_eq!(underclassified_runtime.receipt_state(), &before_receipts);
    assert_eq!(
        underclassified_runtime.trace().success_publication_count(),
        before_successes
    );
    assert!(
        underclassified_runtime
            .result_store()
            .success_publications(VALUE_NAME)
            .is_empty()
    );
    assert!(underclassified_runtime.trace().contains_failure(
        M8DesignatedTraceKind::EvaluationFailed,
        M8DesignatedDiagnosticKind::OutputVisibilityWouldWeakenInput
    ));
}

#[test]
fn designated_replay_is_exact_for_same_artifact_authority_receipt_tick_frontier_and_consumption_log()
 {
    fn replay_report() -> M8DesignatedReplayReport {
        let (path, source, _, instance) = admitted_designated_instance();
        let input_ref = input_source_ref(&path, &source);
        instance
            .into_designated_values(
                M8DesignatedSeed::new()
                    .with_authority_state(designated_authority_state())
                    .with_input_receipts(receipt_set(vec![valid_input_receipt(&input_ref)])),
            )
            .run_replay(
                M8DesignatedReplayLog::new()
                    .with_evaluation(evaluation_request(evaluator_authority_use()))
                    .with_consumption(consume_request(consume_authority_use()))
                    .with_consumption(consume_request(consume_authority_use())),
            )
    }

    let first = replay_report();
    let second = replay_report();

    assert_eq!(first.result_store(), second.result_store());
    assert_eq!(first.version_store(), second.version_store());
    assert_eq!(first.receipt_state(), second.receipt_state());
    assert_eq!(first.consumption_state(), second.consumption_state());
    assert_eq!(first.trace(), second.trace());
    assert_eq!(
        first.trace().authority_reference_count(),
        second.trace().authority_reference_count()
    );
    assert!(first.trace().contains_failure(
        M8DesignatedTraceKind::ConsumptionRejected,
        M8DesignatedDiagnosticKind::AlreadyConsumed
    ));
    assert!(first.trace().node_indexes_are_monotone());
    assert!(!first.trace().has_self_edges());
    assert!(first.trace().dependencies_only_name_earlier_nodes());
}
