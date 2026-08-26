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
pub mod clean_near_end;
pub mod current_l2;
pub mod current_l2_cli;
pub mod full_system_v1_local_split;
pub mod full_system_v1_projection;
pub mod full_system_v1_provider_admission;
pub mod full_system_v1_renderer_pose_backend;
pub mod full_system_v1_session;
pub mod hotplug_runtime;
pub mod m10_reference_system;
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

#[cfg(test)]
mod sys1_runtime_kernel_tests;
#[cfg(test)]
mod sys2_bounded_model_tests;
#[cfg(test)]
mod sys2_execution_backend_tests;
#[cfg(test)]
mod sys3_projection_tests;

pub fn crate_name() -> &'static str {
    "mir_runtime"
}
