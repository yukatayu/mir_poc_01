//! Source-real Stage 3 provider execution contract.
//!
//! The runtime factory owns the canonical checked sample, trusted fixture,
//! M9 admission, binding, and policy. The probe receives only its opaque
//! prepared launch plus physical supervision bounds; it cannot inject a
//! resource path, semantic result, grant, image, control, or fault outcome.

#![allow(unused_crate_dependencies)]

use mir_runtime::{
    I3ReadOnlyProviderFixtureProfile, prepare_source_real_i3_read_only_provider_localnet_launch,
    prepare_source_real_i3_read_only_provider_localnet_launch_duplicate_provider_delivery_on_second_session,
    prepare_source_real_i3_read_only_provider_localnet_launch_effect_retired_observer_current,
    prepare_source_real_i3_read_only_provider_localnet_launch_held_result_first_send_on_second_session,
    prepare_source_real_i3_read_only_provider_localnet_launch_observer_repeat_export,
    prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_after_projection_before_commit,
    prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_before_preflight,
    prepare_source_real_i3_read_only_provider_localnet_launch_post_call_effect_retired_before_held_result_send,
    prepare_source_real_i3_read_only_provider_localnet_launch_sent_result_lost_before_consume,
    prepare_source_real_i3_read_only_provider_localnet_launch_with_fixture_profile,
    sys5_i3_process_runtime::Sys5I3ProviderTerminalOutcomeClass,
};
use mirrorea_i3_probe::{
    I3ReadOnlyProviderLocalnetFailureStage, I3ReadOnlyProviderLocalnetRequest,
    I3ReadOnlyProviderLocalnetRun, I3ReadOnlyProviderLocalnetRunErrorKind,
    run_i3_read_only_provider_localnet, run_i3_read_only_provider_network_conformance,
    run_i3_read_only_provider_terminal_observer_conformance,
};

const FIXED_PROVIDER_TERMINAL_AUDIT_V2: &str = "fixed-provider-terminal-audit-v2";
const FIXED_PROVIDER_NETWORK_PROVENANCE_V2: &str =
    "fixed-provider-terminal-audit-network-provenance-v2";

fn first_normal_transport_slot(slots: &[Option<String>; 2]) -> Option<&str> {
    match (slots[0].as_deref(), slots[1].as_deref()) {
        (Some(reference), None) if !reference.is_empty() => Some(reference),
        _ => None,
    }
}

fn run_source_real_provider_fixture(
    profile: I3ReadOnlyProviderFixtureProfile,
) -> I3ReadOnlyProviderLocalnetRun {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_with_fixture_profile(profile)
            .expect("the finite T0 fixture profile prepares an opaque actual A/B launch");
    run_i3_read_only_provider_localnet(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect("A's opaque post-consume fixture assertion and the normal supervised QUIC run complete")
}

fn assert_safe_source_real_terminal_outcome(
    run: &I3ReadOnlyProviderLocalnetRun,
    expected_outcome: Sys5I3ProviderTerminalOutcomeClass,
    expected_executor_actual_read_count: usize,
) {
    let requester = run.requester_terminal_observer_view();
    let executor = run.executor_terminal_observer_view();
    let [requester_row] = requester.rows() else {
        panic!("the completed requester has exactly one reference-only terminal row");
    };
    let [executor_row] = executor.rows() else {
        panic!("the completed executor has exactly one reference-only terminal row");
    };

    assert!(
        requester.fixed_profile_ref() == FIXED_PROVIDER_TERMINAL_AUDIT_V2
            && executor.fixed_profile_ref() == FIXED_PROVIDER_TERMINAL_AUDIT_V2
            && requester.fixed_schema_ref() == FIXED_PROVIDER_NETWORK_PROVENANCE_V2
            && executor.fixed_schema_ref() == FIXED_PROVIDER_NETWORK_PROVENANCE_V2,
        "the paired terminal candidates retain only the fixed v2 network-provenance profile and schema"
    );
    assert!(
        requester.role() != executor.role()
            && requester.request_descriptor() == executor.request_descriptor()
            && requester.result_descriptor() == executor.result_descriptor()
            && requester_row.semantic_request_ref() == executor_row.semantic_request_ref(),
        "the paired A/B candidates join through opposite roles plus the exact admitted descriptors and semantic request"
    );

    let Some(a_request_send_reservation) =
        first_normal_transport_slot(requester_row.request_send_reservation_occurrence_refs())
    else {
        panic!("A retains only its first request-send reservation occurrence");
    };
    let Some(a_request_send_completed) =
        first_normal_transport_slot(requester_row.request_send_completed_occurrence_refs())
    else {
        panic!("A retains only its first completed request-send occurrence");
    };
    let Some(a_result_receive) =
        first_normal_transport_slot(requester_row.result_receive_occurrence_refs())
    else {
        panic!("A retains only its first result-receive occurrence");
    };
    let Some(a_local_consume) = requester_row.local_consume_ref() else {
        panic!("A retains its completed local consume occurrence");
    };
    assert!(
        requester_row
            .ordered_predecessor_refs()
            .iter()
            .map(String::as_str)
            .eq([
                requester_row.semantic_request_ref(),
                a_request_send_reservation,
                a_request_send_completed,
                a_result_receive,
                a_local_consume,
            ]),
        "A retains the exact first-slot request-send, result-receive, and consume causal order"
    );

    let Some(b_request_receive) =
        first_normal_transport_slot(executor_row.request_receive_occurrence_refs())
    else {
        panic!("B retains only its first request-receive occurrence");
    };
    let Some(b_host_started) = executor_row.host_started_occurrence_ref() else {
        panic!("B retains its actual host-start occurrence");
    };
    let Some(b_adapter_entry) = executor_row.adapter_entry_occurrence_ref() else {
        panic!("B retains its actual adapter-entry occurrence");
    };
    let Some(b_outcome_retained) = executor_row.outcome_retained_occurrence_ref() else {
        panic!("B retains its post-host outcome occurrence");
    };
    let Some(b_release) = executor_row.release_occurrence_ref() else {
        panic!("B retains its release occurrence");
    };
    let Some(b_result_send_reservation) =
        first_normal_transport_slot(executor_row.result_send_reservation_occurrence_refs())
    else {
        panic!("B retains only its first result-send reservation occurrence");
    };
    let Some(b_result_send_completed) =
        first_normal_transport_slot(executor_row.result_send_completed_occurrence_refs())
    else {
        panic!("B retains only its first completed result-send occurrence");
    };
    assert!(
        executor_row.provider_invocation_ref() == Some(b_host_started),
        "the compatibility invocation reference is the retained actual host-start occurrence"
    );
    let b_adapter_read = match expected_executor_actual_read_count {
        0 => {
            assert!(
                executor_row.adapter_read_occurrence_ref().is_none(),
                "a no-read typed outcome retains no adapter-read occurrence"
            );
            None
        }
        1 => Some(
            executor_row
                .adapter_read_occurrence_ref()
                .expect("a successful physical read retains its adapter-read occurrence"),
        ),
        _ => panic!("the finite source-real profiles have zero or one actual B reads"),
    };
    let mut expected_executor_predecessors = vec![
        executor_row.semantic_request_ref(),
        b_request_receive,
        b_host_started,
        b_adapter_entry,
    ];
    if let Some(b_adapter_read) = b_adapter_read {
        expected_executor_predecessors.push(b_adapter_read);
    }
    expected_executor_predecessors.extend([
        b_outcome_retained,
        b_release,
        b_result_send_reservation,
        b_result_send_completed,
    ]);
    assert!(
        executor_row
            .ordered_predecessor_refs()
            .iter()
            .map(String::as_str)
            .eq(expected_executor_predecessors),
        "B retains the exact first-slot receive, host, outcome, release, and result-send causal order"
    );

    assert!(
        requester_row.outcome_class() == Some(expected_outcome)
            && executor_row.outcome_class() == Some(expected_outcome),
        "the two correlated observer candidates retain the expected safe typed terminal class"
    );
    let requester_counts = requester.counts();
    assert!(
        requester_counts.request_count() == 1
            && requester_counts.reserved_count() == 0
            && requester_counts.rejected_before_call_count() == 0
            && requester_counts.call_started_count() == 0
            && requester_counts.physical_adapter_entry_count() == 0
            && requester_counts.actual_read_count() == 0
            && requester_counts.outcome_retained_count() == 0
            && requester_counts.released_count() == 0
            && requester_counts.consume_count() == 1
            && requester_counts.pending_count() == 0,
        "A retains only the completed local consume, never executor-side work or a pending request"
    );
    let executor_counts = executor.counts();
    assert!(
        executor_counts.request_count() == 1
            && executor_counts.reserved_count() == 1
            && executor_counts.rejected_before_call_count() == 0
            && executor_counts.call_started_count() == 1
            && executor_counts.physical_adapter_entry_count() == 1
            && executor_counts.actual_read_count() == expected_executor_actual_read_count
            && executor_counts.outcome_retained_count() == 1
            && executor_counts.released_count() == 1
            && executor_counts.consume_count() == 0
            && executor_counts.pending_count() == 0,
        "B retains one bounded host lifecycle with no rejected-before-call or pending state"
    );
}

#[test]
fn source_real_provider_two_process_round_trip_uses_supervised_quic() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the canonical checked provider source prepares one opaque A/B launch");
    let _run = run_i3_read_only_provider_localnet(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the canonical provider source must complete the normal A/B supervised QUIC round trip",
    );
}

#[test]
fn source_real_value_41_provider_fixture_completes_with_a_safe_present_terminal() {
    let run = run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::Value41);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ValuePresent,
        1,
    );
}

#[test]
fn source_real_value_negative_7_provider_fixture_completes_with_a_safe_present_terminal() {
    let run = run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::ValueNegative7);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ValuePresent,
        1,
    );
}

#[test]
fn source_real_declared_target_absent_provider_fixture_completes_with_not_found_terminal() {
    let run =
        run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::DeclaredTargetAbsent);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ProviderResourceNotFound,
        0,
    );
}

#[test]
fn source_real_noncanonical_integer_provider_fixture_retains_safe_invalid_result_terminal() {
    let run =
        run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::NoncanonicalInteger);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ProviderInvalidResult,
        1,
    );
}

#[test]
fn source_real_33_byte_provider_fixture_retains_safe_invalid_result_terminal() {
    let run =
        run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::ReadBoundary33Bytes);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ProviderInvalidResult,
        1,
    );
}

#[test]
fn source_real_directory_provider_fixture_retains_safe_policy_denied_terminal() {
    let run =
        run_source_real_provider_fixture(I3ReadOnlyProviderFixtureProfile::DeclaredTargetDirectory);
    assert_safe_source_real_terminal_outcome(
        &run,
        Sys5I3ProviderTerminalOutcomeClass::ProviderPolicyDenied,
        0,
    );
}

#[test]
fn source_real_requester_nonzero_after_terminal_is_never_provider_success() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the canonical checked provider source prepares one opaque A/B launch");
    let error = match run_i3_read_only_provider_localnet(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults()
            .with_requester_exit_nonzero_after_completed(),
    ) {
        Ok(_) => panic!(
            "a requester that exits nonzero only after real consume and terminal emission cannot yield provider success"
        ),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
        "the physical post-terminal exit is a provider launch rejection"
    );
    assert_eq!(
        error.stage(),
        I3ReadOnlyProviderLocalnetFailureStage::NaturalReap,
        "the supervisor rejects the nonzero exit at the natural-reap boundary"
    );
}

#[test]
fn source_real_observer_retired_before_preflight_completes_only_the_fixed_conformance_route() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_before_preflight()
        .expect("the sealed preflight-retirement source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_terminal_observer_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the actual FD3-installed and QUIC-supervised preflight-retirement route reaches its fixed opaque completion",
    );
}

#[test]
fn source_real_observer_retired_after_projection_before_commit_completes_only_the_fixed_conformance_route()
 {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_after_projection_before_commit()
        .expect("the sealed post-projection-retirement source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_terminal_observer_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the actual FD3-installed and QUIC-supervised post-projection-retirement route reaches its fixed opaque completion",
    );
}

#[test]
fn source_real_observer_repeat_export_completes_only_the_fixed_conformance_route() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch_observer_repeat_export()
        .expect("the sealed repeat-export source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_terminal_observer_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the actual FD3-installed and QUIC-supervised repeat-export route reaches its fixed opaque completion",
    );
}

#[test]
fn source_real_effect_retired_with_current_observer_completes_only_the_fixed_conformance_route() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_effect_retired_observer_current()
            .expect(
                "the sealed effect-retired observer-current factory prepares one opaque A/B launch",
            );
    let _completion = run_i3_read_only_provider_terminal_observer_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the actual FD3-installed and QUIC-supervised effect-retired observer-current route reaches its fixed opaque completion",
    );
}

#[test]
fn source_real_held_result_first_send_on_second_session_completes_network_conformance() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_held_result_first_send_on_second_session()
            .expect("the sealed held-result source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_network_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect(
        "the held original result completes only through its first verified second-session send",
    );
}

#[test]
fn source_real_sent_result_lost_before_consume_completes_network_conformance() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_sent_result_lost_before_consume()
            .expect(
                "the sealed sent-but-unconsumed source-real factory prepares one opaque A/B launch",
            );
    let _completion = run_i3_read_only_provider_network_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect("the sent original result remains unconsumed through the selected network conformance route");
}

#[test]
fn source_real_duplicate_provider_delivery_on_second_session_completes_network_conformance() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_duplicate_provider_delivery_on_second_session()
            .expect("the sealed duplicate-delivery source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_network_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect("the verified second-session replay is rejected by normal receiver admission without a second execution");
}

#[test]
fn source_real_post_call_effect_retirement_before_held_result_send_completes_network_conformance() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_post_call_effect_retired_before_held_result_send()
            .expect("the sealed post-call-retirement source-real factory prepares one opaque A/B launch");
    let _completion = run_i3_read_only_provider_network_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    )
    .expect("the retained post-call outcome completes the selected effect-retirement network conformance route");
}

#[test]
fn source_real_normal_launch_cannot_enter_network_conformance() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the ordinary sealed source-real factory prepares one opaque A/B launch");
    let error = match run_i3_read_only_provider_network_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    ) {
        Ok(_) => panic!("an ordinary provider launch must not enter the network conformance route"),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
        "the ordinary launch is rejected at the shared safe outer boundary"
    );
    assert_eq!(
        error.stage(),
        I3ReadOnlyProviderLocalnetFailureStage::Preflight,
        "network mode confusion rejects before credentials or child spawn"
    );
}

#[test]
fn source_real_network_conformance_launch_cannot_enter_normal_provider_runner() {
    let launch =
        prepare_source_real_i3_read_only_provider_localnet_launch_held_result_first_send_on_second_session()
            .expect("the sealed network-conformance factory prepares one opaque A/B launch");
    let error = match run_i3_read_only_provider_localnet(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    ) {
        Ok(_) => panic!("a network-conformance launch must not enter the ordinary provider runner"),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
        "the network-conformance launch is rejected at the shared safe outer boundary"
    );
    assert_eq!(
        error.stage(),
        I3ReadOnlyProviderLocalnetFailureStage::Preflight,
        "network mode confusion rejects before credentials or child spawn"
    );
}

#[test]
fn source_real_normal_launch_cannot_enter_terminal_observer_conformance() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the ordinary sealed source-real factory prepares one opaque A/B launch");
    let error = match run_i3_read_only_provider_terminal_observer_conformance(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    ) {
        Ok(_) => panic!(
            "an ordinary provider launch must not enter the terminal-observer conformance route"
        ),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
        "the ordinary launch is rejected at the shared safe outer boundary"
    );
    assert_eq!(
        error.stage(),
        I3ReadOnlyProviderLocalnetFailureStage::Preflight,
        "mode confusion rejects before credentials or child spawn"
    );
}

#[test]
fn source_real_terminal_observer_launch_cannot_enter_normal_provider_runner() {
    let launch = prepare_source_real_i3_read_only_provider_localnet_launch_observer_retired_before_preflight()
        .expect("the sealed observer conformance factory prepares one opaque A/B launch");
    let error = match run_i3_read_only_provider_localnet(
        launch,
        I3ReadOnlyProviderLocalnetRequest::bounded_defaults(),
    ) {
        Ok(_) => panic!(
            "a terminal-observer conformance launch must not enter the ordinary provider runner"
        ),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ReadOnlyProviderLocalnetRunErrorKind::ProviderLaunchRejected,
        "the observer conformance launch is rejected at the shared safe outer boundary"
    );
    assert_eq!(
        error.stage(),
        I3ReadOnlyProviderLocalnetFailureStage::Preflight,
        "mode confusion rejects before credentials or child spawn"
    );
}
