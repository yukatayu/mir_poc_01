#![doc = r#"
# mir-runtime

Single-process execution/runtime skeleton for Mir experiments.

This crate currently exposes a **non-production current L2 runtime skeleton**.
It intentionally stays thin: parser carrier evidence lives in `mir-ast`, semantic
evaluation lives in `mir-semantics`, and this crate only wires the compile path
between them. Runtime-side hot-plug work currently includes a narrow
request/verdict-to-engine-state projection layer over admitted carriers and the
logical runtime substrate, not a completed engine, rollback/migration protocol,
distributed activation ordering, or a final public ABI.
"#]

pub mod alpha_avatar_runtime;
pub mod alpha_layer_insertion_runtime;
pub mod alpha_local_runtime;
pub mod alpha_network_runtime;
pub(crate) mod checked_program_reference;
pub mod clean_near_end;
pub mod current_l2;
pub mod current_l2_cli;
pub mod full_system_v1_local_split;
pub mod full_system_v1_projection;
pub mod full_system_v1_provider_admission;
pub mod full_system_v1_renderer_pose_backend;
pub mod full_system_v1_session;
pub mod hotplug_runtime;
pub(crate) mod i3_read_only_provider_composite;
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub use i3_read_only_provider_composite::{
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
};
pub mod m10_reference_system;
pub(crate) mod m8_owner_admission_gate;
pub mod m8_runtime_admission;
pub mod m8_runtime_authority;
pub mod m8_runtime_designated_value;
pub mod m8_runtime_local_cut;
pub mod m8_runtime_observer;
pub mod m8_runtime_owner_queue;
pub mod m8_runtime_patch;
pub mod m8_runtime_relation_projection;
pub mod m9_auth_verification;
pub mod posegraph_runtime;
pub mod practical_alpha05_host_io;
pub mod practical_alpha05_session;
pub mod practical_alpha08_hotplug_session;
pub mod practical_alpha09_devtools;
pub mod practical_alpha1_avatar;
pub mod practical_alpha1_hotplug;
pub mod practical_alpha1_local_runtime;
pub mod practical_alpha1_save_load;
pub mod practical_alpha1_transport;
pub mod product_alpha1_devtools;
pub mod product_alpha1_session;
pub mod product_alpha1_transport;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod semantic_runtime_kernel;
pub mod surface_source_patch_hotplug;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod sys2_bounded_model;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod sys2_execution_backend;
#[doc(hidden)]
pub(crate) mod sys3_i3_private_snapshot;
pub(crate) mod sys3_projection;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod sys4_dispatch;
/// Private I3-2 QUIC ingress.  It owns an inspected mTLS connection and its
/// one bidi stream before a decoded carrier can reach the local runtime.
#[cfg(feature = "i3-private-quic")]
#[doc(hidden)]
pub mod sys5_i3_private_quic;
/// Private Row20 process-local cut custody seam. It is feature-gated and
/// deliberately exposes no general process/session attachment route.
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
#[doc(hidden)]
pub(crate) mod sys5_i3_process_local_cut;
#[doc(hidden)]
pub mod sys5_i3_process_runtime;
/// Provisional, non-executing SYS-5 build/project facade.
///
/// This exposes observer-safe summaries derived from the checked Core and the
/// internal SYS-3 projection.  A separate CLI may consume it through Rust
/// visibility during this profile, but that grants neither a compatibility,
/// public ABI, nor wire-format promise.
#[doc(hidden)]
pub mod sys5_local_slice;
/// Provisional SYS-5 local workflow over prechecked/projected artifacts.
///
/// This is an internal profile facade, not a public API, ABI, or wire
/// contract.  It intentionally accepts only already checked/projected source
/// and sealed admission candidates.
#[doc(hidden)]
pub mod sys5_local_workflow;
/// Provisional finite SYS-6 I2 conformance producer and verifier.
///
/// This is an internal profile facade. Its Rust names and JSON report are not
/// a public API, ABI, or wire compatibility promise.
#[doc(hidden)]
pub mod sys6_i2_conformance;

#[cfg(test)]
mod i3_read_only_provider_composite_tests;
#[cfg(test)]
mod sys1_runtime_kernel_tests;
#[cfg(test)]
mod sys2_bounded_model_tests;
#[cfg(test)]
mod sys2_execution_backend_tests;
#[cfg(test)]
mod sys3_projection_tests;
#[cfg(test)]
#[allow(clippy::bool_assert_comparison, clippy::too_many_arguments)]
mod sys4_dispatch_tests;
#[cfg(test)]
mod sys5_i3_provider_composite_tests;
#[cfg(test)]
mod sys5_i3_provider_execution_tests;
#[cfg(test)]
mod sys5_local_cut_patch_tests;
#[cfg(test)]
mod sys5_local_slice_tests;
#[cfg(test)]
mod sys5_local_workflow_tests;
#[cfg(test)]
mod sys5_relation_dispatch_tests;
#[cfg(test)]
mod sys5_vertical_slice_tests;
#[cfg(test)]
mod sys6_i2_conformance_tests;

pub fn crate_name() -> &'static str {
    "mir_runtime"
}
