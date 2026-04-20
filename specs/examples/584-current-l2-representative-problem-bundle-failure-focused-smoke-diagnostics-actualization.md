# 584. current-l2 representative problem bundle failure-focused smoke diagnostics actualization

## 位置づけ

- current Phase 6 / Package 110 closeout。
- `smoke-all` helper を compact summary のまま保ちつつ、
  failure 時だけ failing point を narrow に surfacing する。
- exhaustive workflow automation、aggregate CI contract、final public CLI、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py smoke-all`
   は、Problem 1 / Problem 2 の representative smoke summary を維持しつつ、
   failure 時に `failed step` / `failed command` / `failed return code` /
   `failure excerpt` を compact に返してよい。
2. `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   は、aggregate diagnostics 用に
   `failed_command` / `failed_return_code` / `failed_output_excerpt`
   を machine-readable に返してよい。
3. `smoke-all` 自体は aggregate failure を隠さず、
   問題のどこかが failed のときは non-zero exit で止まってよい。
4. `samples/problem-bundles/README.md`
   は、aggregate smoke helper が failure diagnostics を返すことと、
   repo-local regression entrypoint として使う読みを同期してよい。

## current recommendation

- per-problem `smoke` と `smoke-all` の両方を残し、
  current closeout line では `smoke-all` を
  representative verification loop の shortest aggregate entrypoint として使う。
- failure diagnostics は helper-local compact summary に留め、
  long-form log retention、artifact bundling、installed-binary contract には広げない。
- public tutorial や CI fixed surface へはまだ上げず、
  repo-local once-through closeout line を見失わないための diagnostics hardening として読む。

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

- long-form failure log retention
- artifact bundling / emitted failure receipt
- final public onboarding / tutorial flow
- final public theorem/model-check / witness-provider contract
