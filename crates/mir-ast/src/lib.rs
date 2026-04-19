#![doc = r#"
# mir-ast

Syntax trees and normalized language structures for Mir.

This crate currently exposes a **non-production current L2 parser carrier floor**
for the Phase 6 front-half compile-ready checkpoint and its narrow second-tranche
reopen package.

The current first tranche keeps the accepted surface narrow:
- stage 1 option/chain declarations
- stage 2 try/fallback structural shape

A narrow second tranche is also fixed for:
- stage 3 declaration-side admit attached slot
- stage 3 minimal predicate fragment

These stage 3 parsers are non-production reopen-package carriers, not a final
public parser API.

Stage 3 request clause suites, perform head publicization, span-rich
diagnostics, shared single attachment framing, and final grammar remain later
work.
"#]

pub mod current_l2;

pub fn crate_name() -> &'static str {
    "mir_ast"
}
