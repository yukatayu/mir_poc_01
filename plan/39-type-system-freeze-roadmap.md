# plan/39 — type-system freeze roadmap

## purpose

この文書は、Alpha-0 line における
`specs/13-type-system-lifetime-fallback.md`
の repository-memory roadmap を置く。

ここで保持するのは current alpha-local freeze と checker/sample next line であり、
final parser grammar や final public remote-reference API ではない。

## current repo state

- existing current-L2 line already has parser-free fixture and checker-floor support for:
  - same-lineage floor
  - underdeclared lineage / target stop
  - capability strengthening stop
  - try/fallback structural floor
- active implementation anchors:
  - `crates/mir-ast/src/current_l2.rs`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-runtime/src/current_l2.rs`
  - `crates/mir-ast/tests/fixtures/current-l2/`
- Alpha-0 sample family / docs taxonomy / scope wording for Mirrorea Spaces alpha is now present via `samples/alpha/` and snapshot-doc sync

## decisions mirrored from specs/13

- fallback extends access-path availability, not target lifetime
- guarded chain inheritance is explicit
- underdeclared fallback is static error in current alpha-local theory
- no re-promotion within same semantic lineage
- read-only covariance may exist; mutable/read-write carrier remains invariant
- remote cross-place value is guarded observed access path, not raw pointer

## checker implementation roadmap

### first safe cut

reuse current `current_l2` checker/runtime skeleton rather than freezing a new public checker crate.

- bridge Alpha-0 lifetime sample IDs to existing lineage / capability / missing-structure checks
- keep lease-expiry / freshness cutoff as runtime/admissibility line until dedicated checker cluster exists
- keep parser bridge explicit
- preserve `nonproduction` wording

### next cut after first bridge

- add Alpha-0 sample-family aware fixture / source mapping
- widen static reason-code clusters beyond current lineage/target/capability trio when sample floor requires it
- add explicit no-re-promotion / no-resurrection check rows if executable semantics floor reaches them

### kept-later

- final `mir-checker` public crate
- final public diagnostics schema
- full dependent lifetime reasoning
- public remote-reference API

## sample roadmap

sample family roots:

- `samples/alpha/lifetime-fallback/`

minimum rows to keep visible:

- dangling raw ref reject
- explicit inherit chain valid
- underdeclared lineage error
- capability strengthening reject
- write-after-read-only reject
- no re-promotion / no resurrection contrasts
- bird/sparkle inheritance contrast

## Lean / proof roadmap

current proof obligation reading is still repository-memory only.

- no successful dangling read
- no write through expired/read-only option
- no implicit chain propagation
- no hidden resurrection by rollback/load

actual mechanization widening remains later than first checker skeleton.

## deferred questions

- final syntax for `inherit_chain` / `snapshot_selected`
- exact capability lattice beyond current floor
- region polymorphism
- dependent lifetime discharge
- public checker / verifier handoff contract

## next package

- immediate next package after docs/sample sync:
  `P-A0-05` checker skeleton first cut
