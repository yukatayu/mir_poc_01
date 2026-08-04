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
use mir_runtime::m8_runtime_local_cut::M8LeaseRecord;
use mir_runtime::m8_runtime_observer::{
    M8ObserverAuthorityGrant, M8ObserverDiagnosticKind, M8ObserverPolicy, M8ObserverRetention,
    M8ObserverRowKind, M8ObserverRuntime, M8ObserverRuntimeSeed,
};
use mir_runtime::m8_runtime_owner_queue::{M8AuthorityUse, M8OwnerRequest, M8StateKey};
use mir_runtime::m8_runtime_relation_projection::{M8BindingInvalidation, M8RelationAuthorityUse};
use mir_semantics::{
    shared_model::SourceRef,
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
const OBSERVER: &str = "auditor";
const RELATION_NAME: &str = "bird_follow";
const VALUE_NAME: &str = "E.result";
const RESULT_NAME: &str = "result";
const INPUT_FRONTIER: &str = "F";
const BINDING_FRONTIER: &str = "bird_binding_frontier";
const DEGRADED_FRONTIER: &str = "bird_binding_frontier:degraded";
const RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:1";
const REACQUIRED_FRONTIER: &str = "bird_binding_frontier:reacquired";
const REACQUIRE_RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:2";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:owner_epoch1";
const ATTACK_CAPABILITY_REF: &str = "cap:attack:S:self:owner_epoch1";
const ATTACK_WITNESS_REF: &str = "witness:attack:S:self:owner_epoch1";
const RELATION_MEMBERSHIP_REF: &str = "membership:self:S:relation-binding-epoch1";
const RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:binding_epoch1";
const RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:witness_epoch1";
const EVALUATOR_MEMBERSHIP_REF: &str = "membership:self:E:eval_epoch1";
const EVALUATE_CAPABILITY_REF: &str = "cap:designated:evaluate:E.result:self:eval_epoch1";
const EVALUATE_WITNESS_REF: &str = "witness:designated:evaluate:E.result:self:eval_epoch1";
const OBSERVER_AUTHORITY_REF: &str = "observer:auditor:bounded-export:epoch1";
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

fn checked_unified_fixture() -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture(UNIFIED_FIXTURE);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("unified observer fixture checks through M7");
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

fn observer_source_ref(path: &str, source: &str) -> SourceRef {
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
        .expect("exact four non-M9 evidence rows admit unified M8 observer fixture");
    (path, source, checked, admission, instance)
}

fn assert_unified_fixture_shape_and_exact_admission(
    path: &str,
    source: &str,
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
    instance: &M8RuntimeInstance,
) {
    assert_eq!(
        checked.program_identity().module(),
        "Combat.M8.UnifiedLocalCut"
    );
    assert!(source.contains(&format!("project at {CONSUMER} local")));
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
                &relation_source_ref(path, source),
            ),
            (
                ResidualObligationKind::RelationLifetime,
                RELATION_NAME,
                &relation_source_ref(path, source),
            ),
            (
                ResidualObligationKind::FallbackValidity,
                RELATION_NAME,
                &relation_source_ref(path, source),
            ),
            (
                ResidualObligationKind::ValueVisibilityRedaction,
                VALUE_NAME,
                &designated_source_ref(path, source),
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
}

fn observer_authority() -> M8ObserverAuthorityGrant {
    M8ObserverAuthorityGrant::already_admitted(OBSERVER_AUTHORITY_REF)
        .for_principal(OBSERVER)
        .with_max_security_class(M8SecurityClass::Restricted)
        .with_epoch("observer_epoch:1")
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

fn input_receipt(source_ref: &SourceRef) -> M8InputReceipt {
    M8InputReceipt::live(INPUT_RECEIPT_REF)
        .for_state_read(atk_key())
        .with_source_owner_locus(OWNER)
        .with_evaluator(EVALUATOR)
        .with_input_frontier(INPUT_FRONTIER)
        .with_source_ref(source_ref.clone())
        .with_label(
            EvidenceSecurityLabel::new("input:player[self].atk:S:F")
                .with_class(M8SecurityClass::Private),
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

fn observer_runtime() -> (
    String,
    String,
    CheckedSurfaceV0,
    M8RuntimeAdmission,
    M8ObserverRuntime,
) {
    let (path, source, checked, admission, instance) = admitted_unified_instance();
    let input_ref = designated_input_source_ref(&path, &source);
    let runtime = M8ObserverRuntime::from_admitted(
        instance,
        M8ObserverRuntimeSeed::new()
            .with_owner_int(hp_key(), 100)
            .with_owner_int(atk_key(), 10)
            .with_authority_state(authority_state())
            .with_live_lease(live_relation_lease())
            .with_live_lease(fresh_reacquire_relation_lease())
            .with_observer_authority(observer_authority())
            .with_designated_input_receipts(receipt_set(vec![input_receipt(&input_ref)])),
    );
    (path, source, checked, admission, runtime)
}

fn valid_owner_authority_use() -> M8AuthorityUse {
    M8AuthorityUse::for_principal("self")
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_capability_ref(ATTACK_CAPABILITY_REF)
        .with_witness_ref(ATTACK_WITNESS_REF)
}

fn invalidate_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_transition("invalidate_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_REF)
        .with_capability_ref(RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:1")
        .with_witness_ref(RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:1")
}

fn owner_attack_request() -> M8OwnerRequest {
    M8OwnerRequest::new("attack")
        .with_argument("target", "target")
        .with_authority_use(valid_owner_authority_use())
}

fn evaluation_request() -> M8DesignatedEvaluationRequest {
    M8DesignatedEvaluationRequest::for_value(VALUE_NAME)
        .with_tick(M8DesignatedTick::new("tick:F:1").with_input_frontier(INPUT_FRONTIER))
        .with_authority(
            M8DesignatedAuthorityUse::for_evaluator(EVALUATOR)
                .with_principal("self")
                .with_membership_ref(EVALUATOR_MEMBERSHIP_REF)
                .with_capability_ref(EVALUATE_CAPABILITY_REF)
                .with_witness_ref(EVALUATE_WITNESS_REF),
        )
}

fn observer_policy(source_ref: SourceRef) -> M8ObserverPolicy {
    M8ObserverPolicy::for_principal(OBSERVER)
        .with_authority_ref(OBSERVER_AUTHORITY_REF)
        .with_label(
            EvidenceSecurityLabel::new("observer:auditor:bounded-export")
                .with_class(M8SecurityClass::Restricted),
        )
        .with_redaction(EvidenceRedaction::new("redact-private-dependencies"))
        .with_retention(M8ObserverRetention::bounded("retention:phase4b", 64))
        .with_source_ref(source_ref)
        .with_reason_ref("reason:observer:phase4b")
        .with_proof_ref("proof:observer:finite-correspondence")
}

fn prepare_observable_runtime(runtime: &mut M8ObserverRuntime) {
    runtime
        .enqueue_owner(owner_attack_request())
        .expect("observer runtime uses real owner queue plan");
    runtime
        .serve_next_owner(OWNER)
        .expect("owner write exists for observer export");
    runtime
        .invalidate_primary(
            RELATION_NAME,
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier(DEGRADED_FRONTIER),
        )
        .expect("relation lineage change exists for observer export");
    runtime
        .evaluate_designated(evaluation_request())
        .expect("private designated result exists for observer redaction");
}

#[test]
fn authorized_observer_export_requires_typed_policy_and_is_state_preserving_deterministic() {
    let (path, source, checked, admission, mut runtime) = observer_runtime();
    let (_, _, _, _, shape_instance) = admitted_unified_instance();
    assert_unified_fixture_shape_and_exact_admission(
        &path,
        &source,
        &checked,
        &admission,
        &shape_instance,
    );
    prepare_observable_runtime(&mut runtime);
    let source_ref = observer_source_ref(&path, &source);
    let policy = observer_policy(source_ref.clone());
    let before = runtime.save_relevant_payload();

    let first = runtime
        .export_observer_view(policy.clone())
        .expect("authorized observer receives typed export");
    let second = runtime
        .export_observer_view(policy)
        .expect("same observer policy exports deterministically");

    assert_eq!(runtime.save_relevant_payload(), before);
    assert_eq!(first, second);
    assert_eq!(first.policy().observer_principal(), OBSERVER);
    assert_eq!(first.policy().authority_ref(), Some(OBSERVER_AUTHORITY_REF));
    assert_eq!(
        first.policy().label().security_class(),
        M8SecurityClass::Restricted
    );
    assert_eq!(
        first.policy().redaction().as_str(),
        "redact-private-dependencies"
    );
    assert_eq!(first.policy().retention().name(), "retention:phase4b");
    assert_eq!(first.policy().source_ref(), &source_ref);
    assert_eq!(first.policy().reason_ref(), Some("reason:observer:phase4b"));
    assert_eq!(
        first.policy().proof_ref(),
        Some("proof:observer:finite-correspondence")
    );
    assert!(first.rows().contains_kind(M8ObserverRowKind::OwnerWrite));
    assert!(
        first
            .rows()
            .contains_kind(M8ObserverRowKind::RelationLineage)
    );
    assert!(
        first
            .rows()
            .contains_kind(M8ObserverRowKind::DesignatedValue)
    );
    assert!(first.rows().all_have_occurrence_dependency_correspondence());
    let trace = runtime.trace();
    assert!(first.rows().all_correspond_to_exact_trace(&trace));
    assert!(first.rows().all_source_refs_match_runtime_trace(&trace));
}

#[test]
fn unauthorized_or_debug_only_observer_cannot_export_private_raw_values() {
    let (path, source, _, _, mut runtime) = observer_runtime();
    prepare_observable_runtime(&mut runtime);
    let before = runtime.save_relevant_payload();
    let debug_only_policy = M8ObserverPolicy::for_principal(OBSERVER)
        .with_debug_provider_name("devtools")
        .with_package_name("debug-package")
        .with_label(
            EvidenceSecurityLabel::new("observer:debug-only").with_class(M8SecurityClass::Public),
        )
        .with_redaction(EvidenceRedaction::new("none"))
        .with_retention(M8ObserverRetention::bounded("retention:debug-only", 1))
        .with_source_ref(observer_source_ref(&path, &source))
        .with_reason_ref("reason:debug-provider-name-is-not-authority");

    let diagnostics = runtime
        .export_observer_view(debug_only_policy)
        .expect_err("debug/provider/package names are not observer authority");

    assert_eq!(
        diagnostics.primary().kind(),
        M8ObserverDiagnosticKind::MissingObserverAuthority
    );
    assert_eq!(runtime.save_relevant_payload(), before);
    assert!(!diagnostics.contains_raw_value_for(VALUE_NAME));
    assert!(!diagnostics.contains_raw_value_for_state_key(&atk_key()));
    assert!(diagnostics.secret_fields().is_empty());

    let export = runtime
        .export_observer_view(observer_policy(observer_source_ref(&path, &source)))
        .expect("authorized restricted observer receives redacted rows");
    assert!(export.rows().contains_redacted_subject(VALUE_NAME));
    assert!(!export.rows().contains_raw_value_for(VALUE_NAME));
    assert!(!export.rows().contains_raw_value_for_state_key(&atk_key()));
}

#[test]
fn relation_observer_label_cannot_weaken_join_of_private_dependencies() {
    let (path, source, _, _, mut runtime) = observer_runtime();
    prepare_observable_runtime(&mut runtime);
    let before = runtime.save_relevant_payload();
    let underclassified_policy = observer_policy(observer_source_ref(&path, &source))
        .with_relation_label_override(
            RELATION_NAME,
            EvidenceSecurityLabel::new("observer:relation:underclassified")
                .with_class(M8SecurityClass::Restricted),
        )
        .with_relation_input_label(
            RELATION_NAME,
            EvidenceSecurityLabel::new("observer:relation:private-input")
                .with_class(M8SecurityClass::Private),
        );

    let diagnostics = runtime
        .export_observer_view(underclassified_policy)
        .expect_err("derived relation label below input join must reject");

    assert_eq!(
        diagnostics.primary().kind(),
        M8ObserverDiagnosticKind::RelationLabelWouldWeakenInputJoin
    );
    assert_eq!(
        diagnostics.primary().source_ref(),
        &observer_source_ref(&path, &source)
    );
    assert_eq!(
        diagnostics.primary().reason_ref(),
        Some("reason:observer:phase4b")
    );
    assert_eq!(
        diagnostics.primary().proof_ref(),
        Some("proof:observer:finite-correspondence")
    );
    assert_eq!(runtime.save_relevant_payload(), before);
}

#[test]
fn redacted_rows_keep_occurrence_dependency_correspondence_without_secret_fields() {
    let (path, source, _, _, mut runtime) = observer_runtime();
    prepare_observable_runtime(&mut runtime);
    let trace = runtime.trace();
    let export = runtime
        .export_observer_view(observer_policy(observer_source_ref(&path, &source)))
        .expect("authorized observer receives redacted correspondence rows");

    let redacted_value = export
        .rows()
        .redacted_subject(VALUE_NAME)
        .expect("private designated value is represented by a redacted row");
    let occurrence_id = redacted_value
        .occurrence_id()
        .expect("redacted row carries the actual runtime trace node id");
    assert!(!occurrence_id.starts_with("redacted:"));
    assert!(
        trace.contains_node_id(occurrence_id),
        "observer row occurrence_id must name an actual runtime trace node"
    );
    assert!(!redacted_value.dependency_ids().is_empty());
    for dependency_id in redacted_value.dependency_ids() {
        assert!(!dependency_id.starts_with("redacted-dependency:"));
        assert!(
            trace.contains_edge(dependency_id, occurrence_id),
            "observer row dependency_ids must name actual incoming trace edges"
        );
    }
    assert!(redacted_value.corresponds_to(trace.clone()));
    assert!(redacted_value.corresponds_to_exact_trace(&trace));
    assert_eq!(
        trace.source_ref_for_node_id(occurrence_id),
        Some(redacted_value.source_ref())
    );
    assert_eq!(
        redacted_value.source_ref(),
        &designated_source_ref(&path, &source)
    );
    assert_ne!(
        redacted_value.source_ref(),
        &observer_source_ref(&path, &source)
    );
    assert_eq!(
        redacted_value.label().security_class(),
        M8SecurityClass::Restricted
    );
    assert_eq!(
        redacted_value.redaction().as_str(),
        "redact-private-dependencies"
    );
    assert!(!redacted_value.contains_secret_field("raw_value"));
    assert!(!redacted_value.contains_secret_field("raw_authority_payload"));
    assert!(!redacted_value.contains_secret_field("witness_payload"));
    assert!(!export.rows().contains_raw_value_for(VALUE_NAME));
    assert!(export.rows().all_correspond_to_exact_trace(&trace));
    assert!(export.rows().all_source_refs_match_runtime_trace(&trace));
}
