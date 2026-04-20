# 583. current-l2 representative problem bundle aggregate smoke summary actualization

## 位置づけ

- current Phase 6 / Package 109 closeout。
- `smoke problem1` / `smoke problem2` の representative smoke 実行結果を、
  `smoke-all` で compact に俯瞰できる current cut を actualize する。
- exhaustive workflow automation、aggregate CI contract、final public CLI を作る task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py smoke-all`
   を actualize し、Problem 1 / Problem 2 の smoke 成否と step inventory を
   1 コマンドで compact に読めるようにする。
2. `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   を actualize し、repo-local verification summary を machine-readable に読めるようにする。
3. `samples/problem-bundles/README.md`
   に `smoke-all` を shortest aggregate entrypoint として追記し、
   sample bundle index と helper command の読みを揃える。

## current recommendation

- per-problem `smoke` は残し、aggregate 側は compact status 読みだけに留める。
- current package の価値は、
  representative sample bundle verification の入口を 1 本に圧縮しつつ、
  Problem 1 / Problem 2 の個別 smoke route を壊さないことにある。
- aggregate summary は CI fixed surface ではなく、
  repo-local once-through closeout line を追うための helper-local verification cut として読む。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py smoke-all`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- sample docs:
  - `samples/problem-bundles/README.md`

## stop line

- exhaustive workflow automation
- aggregate CI / installed-binary contract
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- aggregate failure-focused diagnostics hardening
- mixed gate に残る theorem/model-check / witness-provider public contract
- final public onboarding / tutorial flow
