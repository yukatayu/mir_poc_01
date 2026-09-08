use mir_ast::surface_v0::{FixtureSource, SyntaxKind, parse_surface_v0};
use mir_semantics::{
    evaluation_materialization::{
        AuthorityOrigin, EvaluationSite, Locus, Materialization, Principal, SemanticForm,
        TriggerClock,
    },
    m9_finite_refinement::{
        M9ContractCandidate, M9FiniteEffectKind, M9FiniteRefinementChecker,
        M9FiniteRefinementErrorKind, M9ReadOnlyProviderEffectContract,
        M9ReadOnlyProviderEffectContractError,
    },
    surface_v0_classification::{
        CoreTemplateKind, SourceToCoreKind, SurfaceV0ClassificationOptions, classify_surface_v0,
    },
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedSurfaceV0, EffectKind, M7DiagnosticKind,
        ResidualObligationKind, check_and_elaborate_surface_v0,
        private_snapshot::{SnapshotOwnerRmwCheckedCore, SnapshotReadOnlyProviderEffectCore},
    },
    surface_v0_provider_effect::ReadOnlyProviderAdapterProfile,
};
use serde_json::{Value, json};

const PROVIDER_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const LOCAL_TOY_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const REQUIRED_PROVIDER_FAILURES: [&str; 10] = [
    "StaleMembership",
    "MissingCapability",
    "MissingWitness",
    "VisibilityDenied",
    "RouteUnavailable",
    "ProviderResourceNotFound",
    "AdapterUnavailable",
    "ProviderInvalidResult",
    "ProviderPolicyDenied",
    "ResourceExhausted",
];
const REQUIRED_PROVIDER_FAILURE_ROW: &str = "(StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, ProviderResourceNotFound, AdapterUnavailable, ProviderInvalidResult, ProviderPolicyDenied, ResourceExhausted)";
const NORMALIZED_PROVIDER_FAILURES: [&str; 10] = [
    "AdapterUnavailable",
    "MissingCapability",
    "MissingWitness",
    "ProviderInvalidResult",
    "ProviderPolicyDenied",
    "ProviderResourceNotFound",
    "ResourceExhausted",
    "RouteUnavailable",
    "StaleMembership",
    "VisibilityDenied",
];

fn provider_source() -> &'static str {
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir")
}

fn local_toy_source() -> &'static str {
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir")
}

fn provider_declaration(source: &str) -> &str {
    let start = source
        .find("effect sample_value by self at ParticipantA")
        .expect("fixture has the selected provider declaration");
    let end = source[start..]
        .find("\n\nrelation bird_follow")
        .map(|offset| start + offset)
        .expect("provider declaration precedes the ordinary relation control");
    &source[start..end]
}

fn provider_source_with_declaration(declaration: &str) -> String {
    let source = provider_source();
    let current = provider_declaration(source);
    source.replacen(current, declaration, 1)
}

fn provider_source_without_failure(missing: &str) -> String {
    let retained = REQUIRED_PROVIDER_FAILURES
        .iter()
        .copied()
        .filter(|failure| *failure != missing)
        .collect::<Vec<_>>()
        .join(", ");
    provider_source().replacen(REQUIRED_PROVIDER_FAILURE_ROW, &format!("({retained})"), 1)
}

fn checked_provider_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, provider_source()))
        .expect("the selected provider source reaches checked M7 before runtime admission")
}

fn checked_provider_core()
-> mir_semantics::surface_v0_provider_effect::CheckedReadOnlyProviderEffectCore {
    checked_provider_source()
        .read_only_provider_effect("sample_value")
        .expect("checked source retains the provider evaluation")
        .read_only_provider_effect_core()
        .expect("provider evaluation retains its distinct checked Core")
        .clone()
}

fn assert_tampered_provider_snapshot_is_rejected(snapshot: Value, falsifier: &str) {
    if let Ok(snapshot) = serde_json::from_value::<SnapshotReadOnlyProviderEffectCore>(snapshot) {
        assert!(
            snapshot.into_checked().is_err(),
            "{falsifier} must fail closed before restoring a checked provider Core"
        );
    }
}

#[test]
fn ordinary_provider_source_is_checked() {
    let result =
        check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, provider_source()));
    assert!(
        result.is_ok(),
        "declared provider effect must check: {result:?}"
    );
}

#[test]
fn provider_effect_ast_retains_each_selected_source_coordinate_and_span() {
    let source = provider_source();
    let ast = parse_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, source))
        .expect("the selected provider declaration is ordinary accepted source syntax");

    assert_eq!(ast.read_only_provider_effects().len(), 1);
    let effect = ast
        .read_only_provider_effect()
        .expect("the source AST retains the provider declaration");
    assert_eq!(effect.name(), "sample_value");
    assert_eq!(effect.requester_principal(), "self");
    assert_eq!(effect.requester_locus(), "ParticipantA");
    assert_eq!(effect.adapter_profile(), "read_int");
    assert_eq!(effect.executor_locus(), "WorldAuthority");
    assert_eq!(effect.logical_resource_slot(), "sample_input");
    assert_eq!(effect.result_type(), "Int");
    assert_eq!(effect.observation_label(), "observer_safe");
    assert_eq!(effect.max_bytes(), 32);
    assert_eq!(effect.max_calls(), 64);
    assert_eq!(
        effect
            .failures()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        REQUIRED_PROVIDER_FAILURES.to_vec()
    );

    assert_eq!(effect.requester_principal_span().lexeme(source), "self");
    assert_eq!(effect.requester_locus_span().lexeme(source), "ParticipantA");
    assert_eq!(effect.adapter_profile_span().lexeme(source), "read_int");
    assert_eq!(
        effect.executor_locus_span().lexeme(source),
        "WorldAuthority"
    );
    assert_eq!(
        effect.logical_resource_slot_span().lexeme(source),
        "sample_input"
    );
    assert_eq!(effect.result_type_span().lexeme(source), "Int");
    assert_eq!(
        effect.observation_label_span().lexeme(source),
        "observer_safe"
    );
    assert_eq!(effect.max_bytes_span().lexeme(source), "32");
    assert_eq!(effect.max_calls_span().lexeme(source), "64");
    assert_eq!(
        effect.failures_span().lexeme(source),
        "(StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, ProviderResourceNotFound, AdapterUnavailable, ProviderInvalidResult, ProviderPolicyDenied, ResourceExhausted)"
    );
    assert_eq!(
        effect.span().lexeme(source).lines().next(),
        Some("effect sample_value by self at ParticipantA")
    );
    assert!(effect.span().is_child_of(ast.root().span()));
    assert!(
        ast.find_node(SyntaxKind::ReadOnlyProviderEffect, "sample_value")
            .is_some(),
        "the provider effect is a distinct AST node, not a rewritten owner handler"
    );
}

#[test]
fn provider_effect_m6_template_and_edges_retain_the_exact_selected_source_facts() {
    let source = provider_source();
    let ast = parse_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, source))
        .expect("the selected provider source parses before M6 classification");
    let effect = ast
        .read_only_provider_effect()
        .expect("the AST retains the selected provider declaration");
    let m6 = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("the selected provider source has a lowerable M6 provider template");
    let template = m6
        .read_only_provider_effect_template("sample_value")
        .expect("M6 retains a distinct provider template");

    assert_eq!(template.kind(), CoreTemplateKind::ReadOnlyProviderEffect);
    assert_eq!(template.name(), "sample_value");
    assert_eq!(template.requester_principal(), "self");
    assert_eq!(template.requester_locus(), "ParticipantA");
    assert_eq!(template.executor_locus(), "WorldAuthority");
    assert_eq!(template.logical_resource_slot(), "sample_input");
    assert_eq!(
        template.adapter_profile(),
        Some(ReadOnlyProviderAdapterProfile::ReadInt)
    );
    assert_eq!(template.result_type(), "Int");
    assert_eq!(template.observation_label(), "observer_safe");
    assert_eq!(template.allowance().max_bytes(), 32);
    assert_eq!(template.allowance().max_calls(), 64);
    assert_eq!(
        template
            .declared_failures()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        REQUIRED_PROVIDER_FAILURES.to_vec()
    );

    assert_eq!(template.declaration_span(), effect.span());
    assert_eq!(
        template.requester_principal_span(),
        effect.requester_principal_span()
    );
    assert_eq!(
        template.requester_locus_span(),
        effect.requester_locus_span()
    );
    assert_eq!(template.executor_locus_span(), effect.executor_locus_span());
    assert_eq!(
        template.logical_resource_slot_span(),
        effect.logical_resource_slot_span()
    );
    assert_eq!(
        template.adapter_profile_span(),
        effect.adapter_profile_span()
    );
    assert_eq!(template.result_type_span(), effect.result_type_span());
    assert_eq!(
        template.observation_label_span(),
        effect.observation_label_span()
    );
    assert_eq!(template.max_bytes_span(), effect.max_bytes_span());
    assert_eq!(template.max_calls_span(), effect.max_calls_span());
    assert_eq!(template.failures_span(), effect.failures_span());

    let edges = m6.source_to_core_map().entries_for_span(effect.span());
    assert_eq!(
        edges.kinds(),
        vec![
            SourceToCoreKind::ReadOnlyProviderEffectRequest,
            SourceToCoreKind::ReadOnlyProviderEffectInvocation,
            SourceToCoreKind::ReadOnlyProviderEffectResult,
            SourceToCoreKind::ReadOnlyProviderEffectResultConsume,
        ]
    );
    assert!(edges.all_source_spans_equal(effect.span()));
    assert_eq!(
        m6.source_ref_for_span(effect.span()),
        m6.source_ref_for_span(template.declaration_span()),
        "M6 source identity is bound to the declaration rather than an inferred owner action"
    );
    assert!(
        m6.core_template("sample_value").is_none(),
        "a provider declaration cannot masquerade as an existing owner-RMW M5 Core template"
    );
}

#[test]
fn old_local_toy_source_retains_no_implicit_provider_declaration_or_template() {
    let ast = parse_surface_v0(FixtureSource::new(
        LOCAL_TOY_SOURCE_PATH,
        local_toy_source(),
    ))
    .expect("the existing I2 local-toy control remains ordinary accepted source");
    assert!(ast.read_only_provider_effect().is_none());
    assert!(ast.read_only_provider_effects().is_empty());

    let m6 = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .expect("the existing I2 local-toy control remains lowerable M6 source");
    assert!(
        m6.read_only_provider_effect_template("sample_value")
            .is_none()
    );
    assert!(
        m6.core_template("attack").is_some(),
        "the old owner action remains an owner template rather than becoming a provider declaration"
    );
}

#[test]
fn old_local_toy_identity_failure_rows_and_private_owner_snapshot_remain_provider_free() {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        LOCAL_TOY_SOURCE_PATH,
        local_toy_source(),
    ))
    .expect("the existing I2 local-toy control still checks");
    let attack = checked
        .evaluation("attack")
        .expect("the prior owner evaluation remains present");
    let owner_core = attack
        .owner_rmw_core()
        .expect("the prior owner evaluation retains only its owner Core");

    assert!(
        checked
            .program_identity()
            .structural_entries()
            .iter()
            .all(|entry| !entry.contains("read-only-provider-effect"))
    );
    assert_eq!(
        attack.declared_failure_row().names(),
        vec![
            "StaleMembership".to_string(),
            "MissingCapability".to_string(),
            "MissingWitness".to_string(),
            "VisibilityDenied".to_string(),
            "RouteUnavailable".to_string(),
        ]
    );
    assert_eq!(
        attack.generated_failure_row().names(),
        vec![
            "StaleMembership".to_string(),
            "MissingCapability".to_string(),
            "MissingWitness".to_string(),
            "RouteUnavailable".to_string(),
            "VisibilityDenied".to_string(),
        ],
        "the pre-provider generated owner row retains the original base-row then observer addition order"
    );
    assert!(checked.source_map().entries().iter().all(|entry| {
        !matches!(
            entry.kind(),
            SourceToCoreKind::ReadOnlyProviderEffectRequest
                | SourceToCoreKind::ReadOnlyProviderEffectInvocation
                | SourceToCoreKind::ReadOnlyProviderEffectResult
                | SourceToCoreKind::ReadOnlyProviderEffectResultConsume
        )
    }));

    let serialized = serde_json::to_value(SnapshotOwnerRmwCheckedCore::from_checked(owner_core))
        .expect("the existing private owner snapshot still serializes");
    assert!(
        serialized
            .as_object()
            .expect("owner snapshot serializes as an object")
            .keys()
            .all(|key| !key.contains("provider"))
    );
    let restored: SnapshotOwnerRmwCheckedCore = serde_json::from_value(serialized)
        .expect("the existing private owner snapshot remains readable");
    assert_eq!(
        restored
            .into_checked()
            .expect("the existing private owner snapshot retains its prior meaning"),
        *owner_core
    );
}

#[test]
fn checked_provider_effect_retains_exact_core_axes_rows_source_map_and_identity() {
    let checked = checked_provider_source();
    let provider = checked
        .read_only_provider_effect("sample_value")
        .expect("M7 retains the distinct provider evaluation");
    let core = provider
        .read_only_provider_effect_core()
        .expect("M7 retains the provider checked Core");

    assert_eq!(
        provider.kind(),
        CheckedEvaluationKind::ReadOnlyProviderEffect
    );
    assert_eq!(provider.name(), "sample_value");
    assert_eq!(provider.actor_authority_origin(), "self");
    assert_eq!(provider.authority_origin_locus(), "ParticipantA");
    assert_eq!(provider.owner_evaluation_locus(), "");
    assert_eq!(core.operation(), "sample_value");
    assert_eq!(core.requester_principal(), "self");
    assert_eq!(core.requester_locus(), "ParticipantA");
    assert_eq!(core.executor_locus(), "WorldAuthority");
    assert_eq!(core.result_consumer_locus(), "ParticipantA");
    assert_eq!(
        core.adapter_profile(),
        ReadOnlyProviderAdapterProfile::ReadInt
    );
    assert_eq!(core.logical_resource_slot(), "sample_input");
    assert_eq!(core.result_type(), "Int");
    assert_eq!(core.observation_label(), "observer_safe");
    assert_eq!(core.allowance().max_bytes(), 32);
    assert_eq!(core.allowance().max_calls(), 64);
    assert_eq!(
        core.declared_failures()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        REQUIRED_PROVIDER_FAILURES.to_vec()
    );
    assert!(core.effect_use_is_required());

    assert_eq!(
        provider.declared_failure_row().names(),
        REQUIRED_PROVIDER_FAILURES
            .iter()
            .map(|failure| (*failure).to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        provider.generated_failure_row().names(),
        REQUIRED_PROVIDER_FAILURES
            .iter()
            .map(|failure| (*failure).to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        provider.evaluation_axes().semantic_form(),
        SemanticForm::Computation
    );
    assert_eq!(
        provider.evaluation_axes().evaluation_site(),
        &EvaluationSite::Locus(Locus::new("WorldAuthority"))
    );
    assert_eq!(
        provider.evaluation_axes().trigger(),
        TriggerClock::OnRequest
    );
    assert_eq!(
        provider.evaluation_axes().authority_origin(),
        &AuthorityOrigin::Caller(Principal::new("self"))
    );
    assert_eq!(
        provider.evaluation_axes().materialization(),
        Materialization::PublishValue
    );
    assert!(
        provider
            .authority_requirements()
            .requires_read_only_provider_effect_authority(
                "sample_value",
                "self",
                "WorldAuthority",
                "ParticipantA"
            )
    );

    assert_eq!(
        provider
            .effect_row()
            .entries()
            .iter()
            .map(|entry| entry.kind())
            .collect::<Vec<_>>(),
        vec![
            EffectKind::ReadOnlyProviderEffectRequest,
            EffectKind::ReadOnlyProviderEffectInvocation,
            EffectKind::ReadOnlyProviderEffectResult,
            EffectKind::ReadOnlyProviderEffectResultConsume,
        ]
    );
    assert!(
        provider
            .effect_row()
            .entries()
            .iter()
            .all(|entry| entry.source_ref() == core.source_ref())
    );
    assert_eq!(
        provider.generated_obligations().failure_names(),
        REQUIRED_PROVIDER_FAILURES
            .iter()
            .map(|failure| (*failure).to_string())
            .collect::<Vec<_>>()
    );
    assert!(
        provider
            .generated_obligations()
            .contains_provider_effect_authorization()
    );
    assert!(
        provider
            .generated_obligations()
            .contains_evaluation(CheckedEvaluationKind::ReadOnlyProviderEffect)
    );
    assert!(!provider.generated_obligations().contains_authority());
    assert!(
        provider
            .generated_obligations()
            .entries()
            .iter()
            .all(|obligation| !obligation.grants_authority_success())
    );

    let source_map = checked
        .source_map()
        .entries_for_lexeme(provider_source(), provider_declaration(provider_source()))
        .expect("checked source map retains each provider source-to-Core edge");
    assert_eq!(
        source_map.kinds(),
        vec![
            SourceToCoreKind::ReadOnlyProviderEffectRequest,
            SourceToCoreKind::ReadOnlyProviderEffectInvocation,
            SourceToCoreKind::ReadOnlyProviderEffectResult,
            SourceToCoreKind::ReadOnlyProviderEffectResultConsume,
        ]
    );
    assert_eq!(source_map.source_ref(), core.source_ref());
    assert_eq!(
        source_map.core_refs(),
        vec![
            "sample_value:provider-effect-request:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-invocation:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-result:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-result-consume:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
        ]
    );
    let expected_provider_identity_prefix = format!(
        "evaluation:sample_value:ReadOnlyProviderEffect:read-only-provider-effect:sample_value:self:ParticipantA:WorldAuthority:ParticipantA:ReadInt:sample_input:Int:observer_safe:{:?}:{:?}:",
        core.allowance(),
        core.source_ref(),
    );
    assert!(
        checked
            .program_identity()
            .structural_entries()
            .iter()
            .any(|entry| entry.starts_with(&expected_provider_identity_prefix))
    );
}

#[test]
fn provider_effect_neither_masquerades_as_owner_rmw_nor_erases_the_execution_blocker() {
    let checked = checked_provider_source();
    let provider = checked
        .read_only_provider_effect("sample_value")
        .expect("M7 retains the provider evaluation");

    assert!(provider.owner_rmw_core().is_none());
    assert!(provider.relation_core().is_none());
    assert!(provider.designated_core().is_none());
    assert!(provider.designated_result_consumer_core().is_none());
    assert!(provider.effect_row().entries().iter().all(|entry| {
        !matches!(
            entry.kind(),
            EffectKind::OwnerRequest | EffectKind::OwnerLocalRead | EffectKind::OwnerWrite
        )
    }));
    assert!(
        checked
            .residual_obligations()
            .contains_kind(ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported)
    );
    assert!(!checked.execution_is_admissible());
    let diagnostics = checked
        .require_execution_admission()
        .expect_err("stage one has no provider exporter or runtime execution path");
    assert_eq!(
        diagnostics.primary().kind(),
        M7DiagnosticKind::ResidualCannotExecute
    );
}

#[test]
fn each_missing_required_provider_failure_is_a_typed_generated_failure_error() {
    for missing in REQUIRED_PROVIDER_FAILURES {
        let source = provider_source_without_failure(missing);
        let diagnostics =
            check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, source))
                .expect_err("every generated provider failure must be explicitly declared");
        let primary = diagnostics.primary();
        assert_eq!(
            primary.kind(),
            M7DiagnosticKind::GeneratedFailureNotDeclared
        );
        assert_eq!(
            primary
                .generated_failure_reason()
                .expect("the typed failure diagnostic retains the absent provider atom")
                .missing_failure(),
            missing
        );
    }
}

#[test]
fn provider_failure_row_rejects_extra_or_duplicate_atoms_after_all_required_atoms_are_present() {
    for malformed_row in [
        format!("({}, Extra)", REQUIRED_PROVIDER_FAILURES.join(", ")),
        format!(
            "({}, ResourceExhausted)",
            REQUIRED_PROVIDER_FAILURES.join(", ")
        ),
    ] {
        let source = provider_source().replacen(REQUIRED_PROVIDER_FAILURE_ROW, &malformed_row, 1);
        let diagnostics =
            check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, source))
                .expect_err("the provider failure row is exactly the fixed distinct ten-atom row");
        assert_eq!(
            diagnostics.primary().kind(),
            M7DiagnosticKind::ReadOnlyProviderEffectProfileMismatch
        );
    }
}

#[test]
fn duplicate_provider_declaration_and_owner_name_collision_reject_at_the_provider_boundary() {
    let declaration = provider_declaration(provider_source());
    let duplicated = provider_source().replacen(
        "\n\nrelation bird_follow",
        &format!("\n\n{declaration}\n\nrelation bird_follow"),
        1,
    );
    let duplicate_diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, duplicated))
            .expect_err("at most one provider declaration is admitted per checked program");
    assert_eq!(
        duplicate_diagnostics.primary().kind(),
        M7DiagnosticKind::DuplicateReadOnlyProviderEffect
    );

    let colliding = provider_source_with_declaration(&declaration.replacen(
        "effect sample_value",
        "effect attack",
        1,
    ));
    let collision_diagnostics =
        check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, colliding))
            .expect_err("a provider declaration cannot reuse an existing owner-RMW operation name");
    assert_eq!(
        collision_diagnostics.primary().kind(),
        M7DiagnosticKind::ReadOnlyProviderEffectProfileMismatch
    );
}

#[test]
fn unknown_provider_principal_requester_locus_and_executor_locus_have_distinct_typed_diagnostics() {
    for (declaration, expected_kind, expected_lexeme) in [
        (
            provider_declaration(provider_source()).replacen("by self", "by stranger", 1),
            M7DiagnosticKind::ReadOnlyProviderEffectProfileMismatch,
            "stranger",
        ),
        (
            provider_declaration(provider_source()).replacen(
                "at ParticipantA\n  invoke",
                "at UnknownRequester\n  invoke",
                1,
            ),
            M7DiagnosticKind::UndefinedReadOnlyProviderRequesterLocus,
            "UnknownRequester",
        ),
        (
            provider_declaration(provider_source()).replacen(
                "read_int at WorldAuthority",
                "read_int at UnknownExecutor",
                1,
            ),
            M7DiagnosticKind::UndefinedReadOnlyProviderExecutorLocus,
            "UnknownExecutor",
        ),
    ] {
        let source = provider_source_with_declaration(&declaration);
        let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(
            PROVIDER_SOURCE_PATH,
            source.clone(),
        ))
        .expect_err("an unresolved provider coordinate must fail static checking");
        let primary = diagnostics.primary();
        assert_eq!(primary.kind(), expected_kind);
        assert_eq!(primary.span().lexeme(&source), expected_lexeme);
    }
}

#[test]
fn provider_profile_result_label_and_allowance_are_exact_not_clamped_or_reinterpreted() {
    for (declaration, expected_lexeme) in [
        (
            provider_declaration(provider_source()).replacen(
                "invoke read_int",
                "invoke write_int",
                1,
            ),
            "write_int",
        ),
        (
            provider_declaration(provider_source()).replacen("returns Int", "returns Bool", 1),
            "Bool",
        ),
        (
            provider_declaration(provider_source()).replacen(
                "visible observer_safe",
                "visible private",
                1,
            ),
            "private",
        ),
        (
            provider_declaration(provider_source()).replacen("bytes 32", "bytes 31", 1),
            "31",
        ),
        (
            provider_declaration(provider_source()).replacen("calls 64", "calls 65", 1),
            "65",
        ),
        (
            provider_declaration(provider_source()).replacen(
                "read_int at WorldAuthority",
                "read_int at ParticipantA",
                1,
            ),
            "ParticipantA",
        ),
    ] {
        let source = provider_source_with_declaration(&declaration);
        let diagnostics = check_and_elaborate_surface_v0(FixtureSource::new(
            PROVIDER_SOURCE_PATH,
            source.clone(),
        ))
        .expect_err("a mixed-shape provider declaration cannot become an accepted profile");
        let primary = diagnostics.primary();
        assert_eq!(
            primary.kind(),
            M7DiagnosticKind::ReadOnlyProviderEffectProfileMismatch
        );
        assert_eq!(primary.span().lexeme(&source), expected_lexeme);
    }
}

#[test]
fn provider_identity_changes_when_a_retained_source_coordinate_changes() {
    let original = checked_provider_source();
    let changed_source =
        provider_source().replacen("resource sample_input", "resource other_input", 1);
    let changed =
        check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, changed_source))
            .expect(
                "a distinct source-declared logical resource remains a typed static coordinate",
            );

    assert_ne!(original.program_identity(), changed.program_identity());
    assert_ne!(
        original
            .read_only_provider_effect("sample_value")
            .expect("original provider evaluation")
            .read_only_provider_effect_core()
            .expect("original provider Core")
            .logical_resource_slot(),
        changed
            .read_only_provider_effect("sample_value")
            .expect("changed provider evaluation")
            .read_only_provider_effect_core()
            .expect("changed provider Core")
            .logical_resource_slot()
    );
}

#[test]
fn m9_provider_contract_retains_checked_coordinates_but_cannot_issue_authority_or_execution() {
    let checked = checked_provider_source();
    let contract = M9ReadOnlyProviderEffectContract::try_from_checked(&checked, "sample_value")
        .expect("M9 retains the source-derived provider contract");

    assert_eq!(contract.program_identity(), checked.program_identity());
    assert_eq!(contract.operation(), "sample_value");
    assert_eq!(contract.requester_principal(), "self");
    assert_eq!(contract.requester_locus(), "ParticipantA");
    assert_eq!(contract.executor_locus(), "WorldAuthority");
    assert_eq!(contract.result_consumer_locus(), "ParticipantA");
    assert_eq!(
        contract.adapter_profile(),
        ReadOnlyProviderAdapterProfile::ReadInt
    );
    assert_eq!(contract.logical_resource_slot(), "sample_input");
    assert_eq!(contract.allowance().max_bytes(), 32);
    assert_eq!(contract.allowance().max_calls(), 64);
    assert_eq!(contract.observation_label(), "observer_safe");
    assert_eq!(contract.failures(), NORMALIZED_PROVIDER_FAILURES.to_vec());
    assert_eq!(
        contract.effects(),
        vec![
            M9FiniteEffectKind::ReadOnlyProviderEffectRequest,
            M9FiniteEffectKind::ReadOnlyProviderEffectInvocation,
            M9FiniteEffectKind::ReadOnlyProviderEffectResult,
            M9FiniteEffectKind::ReadOnlyProviderEffectResultConsume,
        ]
    );
    assert!(!contract.grants_authority());

    let refinement = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            &checked,
            M9ContractCandidate::from_checked_surface(&checked),
        )
        .expect_err("the generic M9 verifier cannot turn the provider contract into execution");
    assert_eq!(
        refinement.primary().kind(),
        M9FiniteRefinementErrorKind::ReadOnlyProviderEffectRequiresDedicatedRuntime
    );

    let old_checked = check_and_elaborate_surface_v0(FixtureSource::new(
        LOCAL_TOY_SOURCE_PATH,
        local_toy_source(),
    ))
    .expect("the old local-toy control still checks");
    assert_eq!(
        M9ReadOnlyProviderEffectContract::try_from_checked(&old_checked, "sample_value"),
        Err(M9ReadOnlyProviderEffectContractError::MissingCheckedEffect)
    );
}

#[test]
fn private_provider_core_snapshot_round_trips_exactly_without_authority_or_runtime_fields() {
    let core = checked_provider_core();
    let snapshot = SnapshotReadOnlyProviderEffectCore::from_checked(&core);
    let serialized = serde_json::to_value(snapshot).expect("private provider Core serializes");
    let object = serialized
        .as_object()
        .expect("private provider Core serializes as an object");
    for required in [
        "operation",
        "requester_principal",
        "requester_locus",
        "executor_locus",
        "result_consumer_locus",
        "adapter_profile",
        "logical_resource_slot",
        "result_type",
        "observation_label",
        "max_bytes",
        "max_calls",
        "declared_failures",
        "source_ref",
    ] {
        assert!(object.contains_key(required), "snapshot retains {required}");
    }
    assert!(object.keys().all(|key| {
        !matches!(
            key.as_str(),
            "grant" | "capability" | "provider_identity" | "host_path" | "result_value"
        )
    }));

    let restored: SnapshotReadOnlyProviderEffectCore =
        serde_json::from_value(serialized).expect("private provider Core deserializes");
    assert_eq!(
        restored
            .into_checked()
            .expect("the exact private snapshot restores its checked Core"),
        core
    );
}

#[test]
fn private_provider_core_snapshot_rejects_tampered_profile_and_unknown_fields() {
    let core = checked_provider_core();
    let snapshot = SnapshotReadOnlyProviderEffectCore::from_checked(&core);
    let serialized = serde_json::to_value(snapshot).expect("private provider Core serializes");

    for (field, value, falsifier) in [
        (
            "result_consumer_locus",
            json!("WorldAuthority"),
            "a consumer different from the requester",
        ),
        (
            "adapter_profile",
            json!("write_int"),
            "an unsupported adapter profile",
        ),
        ("max_bytes", json!(31), "a noncanonical byte allowance"),
        ("max_calls", json!(65), "a noncanonical call allowance"),
        (
            "declared_failures",
            json!(["StaleMembership"]),
            "an incomplete provider failure row",
        ),
    ] {
        let mut tampered = serialized.clone();
        tampered
            .as_object_mut()
            .expect("private provider snapshot is an object")
            .insert(field.to_string(), value);
        assert_tampered_provider_snapshot_is_rejected(tampered, falsifier);
    }

    let mut unknown_field = serialized;
    unknown_field
        .as_object_mut()
        .expect("private provider snapshot is an object")
        .insert("unexpected_snapshot_field".to_string(), json!(true));
    assert!(
        serde_json::from_value::<SnapshotReadOnlyProviderEffectCore>(unknown_field).is_err(),
        "private snapshot decoding rejects unknown fields before Core restoration"
    );
}

#[test]
fn private_provider_core_snapshot_rejects_source_inexpressible_identifiers() {
    let core = checked_provider_core();
    let serialized = serde_json::to_value(SnapshotReadOnlyProviderEffectCore::from_checked(&core))
        .expect("the valid source-derived provider snapshot serializes");
    let valid: SnapshotReadOnlyProviderEffectCore =
        serde_json::from_value(serialized.clone()).expect("the valid snapshot deserializes");
    assert_eq!(
        valid
            .into_checked()
            .expect("ordinary alphabetic and underscore-bearing source identifiers restore"),
        core
    );

    for invalid_locus in ["not a locus", "", "0requester", "érequester", "bad-name"] {
        let mut tampered = serialized.clone();
        let fields = tampered
            .as_object_mut()
            .expect("private provider snapshot is an object");
        fields.insert("requester_locus".to_string(), json!(invalid_locus));
        fields.insert("result_consumer_locus".to_string(), json!(invalid_locus));
        assert_tampered_provider_snapshot_is_rejected(
            tampered,
            "a source-inexpressible requester and matching result consumer locus",
        );
    }

    for (field, value, falsifier) in [
        (
            "executor_locus",
            json!(""),
            "an empty executor locus that is distinct only by accident",
        ),
        ("operation", json!("/tmp/x"), "a path-shaped operation name"),
        (
            "logical_resource_slot",
            json!("/tmp/x"),
            "a path-shaped logical resource slot",
        ),
    ] {
        let mut tampered = serialized.clone();
        tampered
            .as_object_mut()
            .expect("private provider snapshot is an object")
            .insert(field.to_string(), value);
        assert_tampered_provider_snapshot_is_rejected(tampered, falsifier);
    }
}
