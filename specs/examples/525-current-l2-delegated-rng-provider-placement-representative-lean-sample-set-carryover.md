# 525 — current L2 delegated RNG provider placement representative Lean sample-set carry-over

## 位置づけ

- historical Package 58 first widening closeout memory。
- `p09-dice-delegated-rng-provider-placement`
  は、2026-04-22 clean-sample migration 前の
  delegated RNG / provider placement compare anchor として保ってよい。
- current active sample-side compare floor は
  `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
  に置く。
- final public provider receipt schema、
  final witness/provider/artifact contract、
  final public theorem/model-check contract を fixed する task ではない。

## current recommendation

- archived `p09` は
  provider placement と authority placement separation の
  historical compare anchor として読む。
- current execution / inspection / Lean sync front door は
  clean-near-end `05_delegated_rng_service`
  と active `samples/lean/clean-near-end/05_delegated_rng_service/`
  に置く。
- broader coverage widening は helper-local / repo-local compare floor に留める。

## actualized carry-over memory

| anchor | historical role | current compare floor |
|---|---|---|
| `p09-dice-delegated-rng-provider-placement` | delegated provider placement reached prototype | `05_delegated_rng_service` |

## actualized evidence

- archived compare anchor:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt`
- current clean-near-end compare floor:
  - `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
- active / archived Lean evidence:
  - `samples/lean/clean-near-end/05_delegated_rng_service/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p09-dice-delegated-rng-provider-placement/`
  - `scripts/current_l2_lean_sample_sync.py`
- runtime / preview / model-check / theorem tests
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
  - `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
  - `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`

## stop line

- archived compare anchor の active path 復帰
- final public provider receipt schema
- final witness/provider/artifact contract
- final public theorem/model-check contract

## retained-later line

- helper/CLI hardening beyond current sync/export route
- broader theorem/model-check/order-handoff widening beyond this carry-over
