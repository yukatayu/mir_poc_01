#![doc = r#"
# mir-runtime

Single-process execution/runtime skeleton for Mir experiments.

This crate currently exposes a **non-production current L2 runtime skeleton**.
It intentionally stays thin: parser carrier evidence lives in `mir-ast`, semantic
evaluation lives in `mir-semantics`, and this crate only wires the compile path
between them.
"#]

pub mod current_l2;
pub mod current_l2_cli;

pub fn crate_name() -> &'static str {
    "mir_runtime"
}
