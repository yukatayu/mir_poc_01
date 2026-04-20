# 585. current-l2 representative problem bundle quickstart walkthrough hardening

## 位置づけ

- current Phase 6 / Package 111 closeout。
- `samples/problem-bundles/problem1|problem2` を、
  representative sample guide であるだけでなく
  doc 単体でも最短導線を読める quickstart bundle に harden する。
- exhaustive tutorial surface、final public onboarding、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `samples/problem-bundles/problem1-typed-theorem-model-check.md`
   は `smoke problem1` → `matrix problem1` → `bundle problem1` →
   parser companion inspector の 4 段 quickstart を持ってよい。
2. `samples/problem-bundles/problem2-order-handoff-shared-space.md`
   は `smoke problem2` → `matrix problem2` → `bundle problem2` →
   parser companion inspector の 4 段 quickstart を持ってよい。
3. 各 step には `見るべき結果` を短く添え、
   representative / reserve / negative / stop line の読みを
   doc 単体でも取り違えにくくしてよい。
4. `samples/problem-bundles/README.md`
   は bundle doc 冒頭の `最短 quickstart` を index 側から案内してよい。

## current recommendation

- current sample-side quickstart は、
  exhaustive tutorial ではなく
  representative sample を見失わないための 1 画面 walkthrough として保つ。
- `smoke-all` は problem 横断の aggregate entrypoint、
  bundle doc 冒頭の quickstart は problem ごとの sample-side entrypoint として使い分ける。
- parser companion inspector は final grammar adoption ではなく、
  thin experimental companion surface の parser-side carrier 確認として読む。

## actualized evidence

- sample docs:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
  - `samples/problem-bundles/README.md`
- tests:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- exhaustive tutorial surface
- exhaustive sample catalog
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- quickstart CLI mirror
- broader sample catalog widening
- final public onboarding flow
