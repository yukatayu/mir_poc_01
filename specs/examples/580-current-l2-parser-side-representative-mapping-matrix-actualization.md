# 580. current-l2 parser-side representative mapping matrix actualization

## 位置づけ

- current Phase 6 / Package 106 closeout。
- `p06 / p07 / p08` representative slice について、
  original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report
  の対応を docs / helper / traceability の 3 点で同じ読みに揃える。
- final public parser / checker / runtime API や exhaustive sample catalog を固める task ではない。

## この package で固定する current cut

1. repo-local helper `python3 scripts/current_l2_guided_samples.py mapping`
   を actualize し、representative slice の 5 層対応を pretty / json で読めるようにする。
2. `samples/prototype/current-l2-parser-companion/README.md`
   に representative mapping 表を追加し、helper 出力と同じ読みを sample 側にも置く。
3. traceability / roadmap / progress / task snapshot を Package 106 close に同期する。

## current recommendation

- representative slice の readable mapping は、
  `p06 / p07 / p08` に限定した helper-local matrix として actualize してよい。
- current first line は bundle helper だけに依存させず、
  parser companion README / helper command / traceability addendum の 3 点で同じ導線を保つ。
- exhaustive catalog、reserve / negative sample までの mapping widening、
  final public tutorial surface は still later に残す。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py mapping`
  - `python3 scripts/current_l2_guided_samples.py mapping --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- sample docs:
  - `samples/prototype/current-l2-parser-companion/README.md`

## stop line

- exhaustive sample catalog
- final public tutorial surface
- final public parser / checker / runtime API

## retained-later line

- reserve / negative sample widening (`p09 / p13 / p14`)
- parser companion から full `Program` lowering への public bridge
- final parser diagnostics / span-rich contract
