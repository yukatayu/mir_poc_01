# 582. current-l2 representative problem bundle smoke runner actualization

## 位置づけ

- current Phase 6 / Package 108 closeout。
- Problem 1 / Problem 2 の representative sample bundle guide に書いた主要 command 群を、
  repo-local helper から順に smoke 実行できる current cut を actualize する。
- final public CLI、exhaustive workflow automation、CI 固定 surface を作る task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py smoke problem1`
   を actualize し、Problem 1 の representative line
   `runtime -> matrix -> bundle -> parser-side inspector -> mapping`
   を 1 本の smoke command で再現できるようにする。
2. `python3 scripts/current_l2_guided_samples.py smoke problem2`
   を actualize し、Problem 2 の representative line
   `runtime pair -> matrix -> bundle -> parser-side inspector pair -> mapping`
   を 1 本の smoke command で再現できるようにする。
3. helper / tests / sample guide / snapshot / traceability を Package 108 close に同期する。

## current recommendation

- smoke runner は representative line の drift suppression に集中し、
  bundle guide 全文の workflow automation や exhaustive scenario 実行には広げない。
- smoke step は pretty explanation よりも reproducible command chain を優先し、
  docs 側では `samples/problem-bundles/` と helper command の 2 本で同じ導線を保つ。
- representative pair のまま narrow に保つことで、
  Problem 1 の theorem-first line と Problem 2 の authoritative-room line を
  1 コマンドずつで再確認できる current cut が自然である。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py smoke problem1`
  - `python3 scripts/current_l2_guided_samples.py smoke problem2`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- existing bundle / mapping / parser-side anchor:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
  - `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
  - `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`

## stop line

- exhaustive workflow automation
- aggregate CI contract / installed-binary packaging
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- Problem 1 / Problem 2 をまとめた compact smoke summary
- mixed gate に残る theorem/model-check / witness-provider public contract
- final public onboarding / tutorial flow
