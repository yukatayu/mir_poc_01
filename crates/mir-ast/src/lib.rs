#![doc = r#"
# mir-ast

Syntax trees and normalized language structures for Mir.

This crate currently exposes a **non-production current L2 parser carrier floor**
for the Phase 6 front-half compile-ready checkpoint.

It intentionally keeps the accepted surface narrow:
- stage 1 option/chain declarations
- stage 2 try/fallback structural shape

Stage 3 request clauses, predicate fragments, span-rich diagnostics, and final grammar
remain later work.
"#]

pub mod current_l2;

pub fn crate_name() -> &'static str {
    "mir_ast"
}
