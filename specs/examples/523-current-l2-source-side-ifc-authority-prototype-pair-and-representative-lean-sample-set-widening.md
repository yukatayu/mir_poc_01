# 523 — current L2 source-side IFC authority prototype pair and representative Lean sample-set widening

## 位置づけ

- historical Package 56 closeout memory。
- `p10-typed-authorized-fingerprint-declassification` と
  `p11-typed-unauthorized-fingerprint-release`
  は、2026-04-22 clean-sample migration 前の
  archived source-side IFC authority compare-anchor pair として保ってよい。
- current active sample-side compare floor は
  `samples/clean-near-end/typing/01_authorized_declassification.mir`
  と
  `samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir`
  に置く。
- final typed source principal、final IFC syntax、
  final public verifier contract を fixed する task ではない。

## current recommendation

- archived `p10 / p11` pair は
  source-side explicit authority success / failure historical compare anchor として読む。
- current execution / inspection / Lean sync front door は
  clean-near-end typing `01 / 02`
  と active `samples/lean/clean-near-end/`
  に置く。
- current checker-adjacent principal は維持し、
  stronger typed source principal early promotion はしない。

## actualized pair memory

| anchor | historical role | current compare floor |
|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | explicit authority declassification success prototype | `01_authorized_declassification` |
| `p11-typed-unauthorized-fingerprint-release` | authority-miss negative prototype | `02_unauthorized_declassification_rejected` |

## actualized evidence

- archived source-side compare anchors:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.host-plan.json`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.host-plan.json`
- current clean-near-end compare floor:
  - `samples/clean-near-end/typing/01_authorized_declassification.mir`
  - `samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir`
- active Lean front door:
  - `samples/lean/clean-near-end/01_authorized_declassification/`
  - `samples/lean/clean-near-end/02_unauthorized_declassification_rejected/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p10-typed-authorized-fingerprint-declassification/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p11-typed-unauthorized-fingerprint-release/`
  - `scripts/current_l2_lean_sample_sync.py`

## stop line

- archived prototype anchor の active path 復帰
- final typed source principal
- final IFC syntax
- final public verifier contract

## retained later line

- label-flow negative exhaustive corpus
- stronger typed source principal
- final typed calculus
- concrete theorem/model-check production binding
