#![doc = r#"
# mir-ast

Syntax trees and normalized language structures for Mir.

This crate currently exposes a **non-production current L2 parser carrier floor**
for the Phase 6 front-half compile-ready checkpoint.

It intentionally keeps the accepted surface narrow:
- stage 1 option/chain declarations
- stage 2 try/fallback structural shape
- stage 3 declaration-side admit attached slot
- stage 3 minimal isolated predicate fragment
- stage 3 shared single attachment frame extraction bridge

Stage 3 request clause suites, perform head publicization,
span-rich diagnostics, and final grammar remain later work.
"#]

pub mod current_l2;

pub fn crate_name() -> &'static str {
    "mir_ast"
}
