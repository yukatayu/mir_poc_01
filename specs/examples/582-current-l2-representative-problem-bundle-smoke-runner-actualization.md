# 582. current-l2 representative problem bundle smoke runner actualization

## 位置づけ

- historical Phase 6 / Package 108 closeout memory。
- `smoke problem1|problem2` helper により
  representative bundle guide の主要 command 群を順に再現していた導線を、
  2026-04-22 clean-sample migration 前の historical smoke-runner memory として記録する。
- final public CLI、aggregate CI contract、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py smoke problem1|problem2`
   は historical helper-local smoke-runner memory として扱い、
   current active compatibility front door には戻さない。
2. current live compatibility helper は
   Problem 1 / Problem 2 individual smoke を再導入せず、
   `smoke-all` のみを current active command に残す。
3. historical bundle docs は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/`
   側の closeout memory として読む。

## current recommendation

- `smoke problem1|problem2` は、
  problem-local walkthrough を helper から再生していた
  historical smoke-runner memory として保ってよい。
- current repo では individual smoke command を戻さず、
  `smoke-all` を active compatibility front door に留める。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py smoke problem1`
  - `python3 scripts/current_l2_guided_samples.py smoke problem2`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- exhaustive workflow automation
- aggregate CI contract
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- surviving `smoke-all` aggregate summary
- failure-focused diagnostics hardening memory
