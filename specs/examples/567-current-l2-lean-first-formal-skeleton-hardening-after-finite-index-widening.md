# 567. current L2 Lean-first formal skeleton hardening after finite-index widening

## 位置づけ

- historical Phase 5 / Package 93 closeout memory。
- current Lean front door は
  `samples/lean/foundations/` と
  `samples/lean/clean-near-end/`
  に置く。
- pre-clean-near-end representative generated stub corpus
  `p15 / p16` は
  `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/`
  の archived appendix として保ってよい。
- final public theorem contract、
  concrete production prover binding、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `samples/lean/foundations/` は
   actual small proof fragment を置く mechanization-ready core として読む。
2. `samples/lean/clean-near-end/` は
   active clean sample suite から生成した theorem stub corpus として読む。
3. `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/` は
   pre-clean-near-end representative sample set widening
   `p06 / p07 / p08 / p09 / p10 / p11 / p12 / p13 / p14 / p15 / p16`
   を保持する historical appendix として読む。
4. generated stub は Lean に受理されても
   completed theorem discharge ではなく
   artifact well-formedness / bridge alignment evidence に留める。

## current recommendation

- current Lean explanation は
  foundation / active clean-near-end generated stub / old archived corpus
  の 3-way split を崩さない。
- `scripts/current_l2_lean_sample_sync.py` の current sync target は
  `samples/lean/clean-near-end/` と `samples/lean/manifest.json` に留める。
- `p15 / p16` widening は
  archived old corpus の historical memory であり、
  active generated surface と混ぜない。

## actualized evidence

- current sync / explanation anchors:
  - `samples/lean/README.md`
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
  - `samples/lean/manifest.json`
- foundation:
  - `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  - `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.md`
- active clean-near-end generated stub floor:
  - `samples/lean/clean-near-end/04_capture_escape_rejected/`
  - `samples/lean/clean-near-end/05_cost_bound_rejected/`
- archived pre-clean-near-end generated corpus:
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p15-typed-capture-escape-rejected/`
  - `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p16-typed-remote-call-budget-exceeded/`

## stop line

- production prover binding
- final proof object public contract
- final public theorem result contract
- final public verifier contract
- archived old corpus の active surface 復帰

## retained-later line

- proof object public schema を先に public surface へ上げる案
- stronger typed source principal を先に Lean side へ寄せる案
- concrete theorem prover brand / discharge transport を先に publicize する案
