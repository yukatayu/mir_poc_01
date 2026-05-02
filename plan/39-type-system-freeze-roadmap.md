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
- `P-A0-05` first checker-floor helper actualization, widened by `P-A0-16`, and complemented by `P-A0-17` helper-local acceptance floor, `P-A0-20` snapshot-selected floor, and `P-A0-21` anchor-handoff floor now exists via:
  - `scripts/alpha_lifetime_fallback_checker.py`
  - `scripts/alpha_lifetime_fallback_acceptance.py`
  - `scripts/alpha_lifetime_fallback_snapshot.py`
  - `scripts/alpha_lifetime_fallback_anchor_handoff.py`
  - `samples/alpha/lifetime-fallback/lif-01` and `lif-05..08*.expected.json`
  - `samples/alpha/lifetime-fallback/lif-02` / `03` / `04*.expected.json`
  - `samples/alpha/lifetime-fallback/lif-13*.expected.json`
  - `samples/alpha/lifetime-fallback/lif-11*.expected.json`
  - shared helper reuse through `scripts/current_l2_family_checker_support.py`
  - shared helper reuse through `scripts/current_l2_family_acceptance_support.py`
  - shared helper reuse through `scripts/current_l2_family_snapshot_support.py`
  - shared helper reuse through `scripts/current_l2_family_anchor_handoff_support.py`
  synthetic detached artifact comparison only; negative rows are confined to `reason_codes_scope = alpha-static-floor`, acceptance rows are confined to `acceptance_scope = alpha-acceptance-floor`, snapshot-selected rows are confined to `snapshot_scope = alpha-snapshot-selected-floor`, anchor-handoff rows are confined to `anchor_handoff_scope = alpha-anchor-handoff-floor`, and there is still no parser/runtime bridge claim

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
- `LIF-13` now uses a dedicated helper-local snapshot-selected artifact schema:
  `snapshot_scope = alpha-snapshot-selected-floor` only
- `LIF-11` now uses a dedicated helper-local anchor-handoff artifact schema:
  `anchor_handoff_scope = alpha-anchor-handoff-floor` only
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

- after `P-A0-22`, the current lifetime-family widening is intentionally limited to:
  - negative checker-floor rows `LIF-01/05..08`
  - helper-local synthetic acceptance rows `LIF-02/03/04`
  - helper-local synthetic snapshot-selected row `LIF-13`
  - helper-local synthetic anchor-handoff row `LIF-11`
- `P-A0-22` closes only a docs-first blocker split and keeps `LIF-15` non-actualized behind:
  - `remote_observe_scope = alpha-remote-observe-floor`
  - place identity / target identity / membership epoch / participant incarnation
  - freshness frontier / visibility frontier / read-observe capability
  - label-redaction policy / covariance relation / stale-frontier rejection line
- no acceptance-floor widening, snapshot-floor widening, anchor-handoff widening, or parser/runtime bridge claim is added
- next safe actualization package is not yet promoted; the next reopen must first choose whether the narrower line is:
  - dedicated `LIF-15` remote-observe carrier design / helper floor
  - or a separate non-lifetime carrier first
- queue authority remains `progress.md` / `tasks.md`
