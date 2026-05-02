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
- `P-A0-05` first checker-floor helper actualization, widened by `P-A0-16`, and complemented by `P-A0-17` helper-local acceptance floor now exists via:
  - `scripts/alpha_lifetime_fallback_checker.py`
  - `scripts/alpha_lifetime_fallback_acceptance.py`
  - `samples/alpha/lifetime-fallback/lif-01` and `lif-05..08*.expected.json`
  - `samples/alpha/lifetime-fallback/lif-02` / `03` / `04*.expected.json`
  - shared helper reuse through `scripts/current_l2_family_checker_support.py`
  - shared helper reuse through `scripts/current_l2_family_acceptance_support.py`
  synthetic detached artifact comparison only; negative rows are confined to `reason_codes_scope = alpha-static-floor`, positive rows are confined to `acceptance_scope = alpha-acceptance-floor`, and there is still no parser/runtime bridge claim

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

- bridge selected Alpha-0 lifetime sample IDs to a non-public sidecar-driven checker floor through `expected_static.checked_reason_codes`
- keep lease-expiry / freshness cutoff as runtime/admissibility line until dedicated checker cluster exists
- keep parser bridge explicit
- preserve `nonproduction` wording

### next cut after first bridge

- add Alpha-0 sample-family aware fixture / source mapping if/when a dedicated artifact emitter exists
- widen static reason-code clusters beyond the current selected static rows (`LIF-01`, `LIF-05..08`) only when a new checked carrier is explicitly fixed
- selected positive rows now use an explicit helper-local acceptance artifact schema:
  `LIF-02/03/04` only
- keep broader positive/remote/runtime-sensitive rows planned until new semantics justify widening
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

## next reopen point

- after `P-A0-19`, the current lifetime-family widening is intentionally limited to:
  - negative checker-floor rows `LIF-01/05..08`
  - helper-local synthetic acceptance rows `LIF-02/03/04`
- `P-A0-19` closes only a docs-first remaining-row inventory for:
  - `LIF-11` anchor/deletion outcome semantics
  - `LIF-13` selected-option snapshot semantics
  - `LIF-15` remote freshness/membership/frontier carrier
- inventory closeout does not widen the acceptance floor or create a parser/runtime bridge
- next safe reopen requires the first row-specific actualization chosen from that inventory:
  - either proof that one row still fits a narrow helper-local carrier without new semantics
  - or a dedicated runtime/remote bridge for that row without overclaim
- queue authority remains `progress.md` / `tasks.md`
