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
pub mod hotplug_runtime;
pub mod practical_alpha05_host_io;
pub mod practical_alpha05_session;
pub mod practical_alpha08_hotplug_session;
pub mod practical_alpha09_devtools;
pub mod practical_alpha1_avatar;
pub mod practical_alpha1_hotplug;
pub mod practical_alpha1_local_runtime;
pub mod practical_alpha1_save_load;
pub mod practical_alpha1_transport;
pub mod product_alpha1_session;

pub fn crate_name() -> &'static str {
    "mir_runtime"
}
