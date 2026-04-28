#![doc = r#"
# mir-runtime

Single-process execution/runtime skeleton for Mir experiments.

This crate currently exposes a **non-production current L2 runtime skeleton**.
It intentionally stays thin: parser carrier evidence lives in `mir-ast`, semantic
evaluation lives in `mir-semantics`, and this crate only wires the compile path
between them. Runtime-side hot-plug work also remains at a narrow
report/skeleton layer rather than a completed engine.
"#]

pub mod clean_near_end;
pub mod current_l2;
pub mod current_l2_cli;
pub mod hotplug_runtime;

pub fn crate_name() -> &'static str {
    "mir_runtime"
}
