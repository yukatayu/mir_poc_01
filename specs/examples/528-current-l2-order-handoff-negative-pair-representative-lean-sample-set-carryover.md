# 528 — current L2 order-handoff negative pair representative Lean sample-set carry-over

## 位置づけ

- historical Package 58 broader theorem-side widening memory。
- `p13-dice-late-join-missing-publication-witness` と
  `p14-dice-late-join-handoff-before-publication`
  は、2026-04-22 clean-sample migration 前の
  archived negative-pair carry-over anchors として保ってよい。
- current active sample-side compare floor は
  `samples/clean-near-end/order-handoff/02_missing_witness_rejected.mir`
  と
  `samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir`
  に置く。
- final public theorem contract、final proof-object public schema、
  final parser/public API、final source wording を fixed する task ではない。

## current recommendation

- archived `p13 / p14` は
  theorem-side negative-pair carry-over historical anchors として読む。
- current Lean / verification front door は
  active clean-near-end negative pair と
  active `samples/lean/clean-near-end/02_missing_witness_rejected/` /
  `03_handoff_before_publication_rejected/`
  に置く。
- archived old Lean corpus は
  `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/`
  appendix に留め、active generated front door と混ぜない。

## actualized carry-over memory

| anchor | historical role | current compare floor |
|---|---|---|
| `p13-dice-late-join-missing-publication-witness` | static rejection carry-over anchor | `02_missing_witness_rejected` |
| `p14-dice-late-join-handoff-before-publication` | static rejection carry-over anchor | `03_handoff_before_publication_rejected` |

## actualized evidence

- prior helper-local static stop
  - `specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md`
- theorem-side actual Lean execution regression
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- active / archived Lean evidence
  - `samples/lean/clean-near-end/02_missing_witness_rejected/`
  - `samples/lean/clean-near-end/03_handoff_before_publication_rejected/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p13-dice-late-join-missing-publication-witness/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p14-dice-late-join-handoff-before-publication/`
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`

## stop line

- archived carry-over anchor の active path 復帰
- final public theorem contract
- proof-object public schema
- final public verifier contract
- final parser/public API
- final source wording / final emitted-artifact contract
- broader theorem-side / IFC helper widening beyond this carry-over
