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

The next parser-side follow-up package is fixed as:
- stage 3 shared single attachment frame
  - option-local `admit:` multiline extraction
  - request-local `require:` / `ensure:` multiline extraction
  - existing predicate fragment parser reuse
- stage 3 request clause fixed two-slot suite
  - `require_fragment_text` / `ensure_fragment_text`
  - perform-owner structural floor
  - predicate fragment parser reuse
- stage 3 perform head structural carrier
  - `op`
  - `On(target)` / `Via(chain_ref)`
  - request clause suite bundle attachment reuse
- stage 3 perform-head / request-clause thin bundle attachment
  - `Stage3RequestHeadClauseBundle`
  - `RequestLocalTwoSlotSuite`
  - perform head / request clause suite parser reuse

These stage 3 parsers are non-production reopen-package carriers, not a final
public parser API.

Span-rich diagnostics, final grammar, and full public parser integration remain
later work.
"#]

pub mod current_l2;

pub fn crate_name() -> &'static str {
    "mir_ast"
}
