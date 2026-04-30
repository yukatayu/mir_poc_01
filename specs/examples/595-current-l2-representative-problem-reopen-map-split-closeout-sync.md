# 595. current-l2 representative problem reopen-map split closeout sync

## 位置づけ

- historical Phase 6 / Package 122 closeout memory。
- representative problem reopen-map から stale な `next split packages` 表示を外し、
  split-package closeout 後の residual public-seam maintenance 読みへ同期していた historical memory を記録する。
- final public theorem/model-check/witness-provider contract や final parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem1|problem2`
   は 2026-04-22 clean-sample migration 前の historical split-closeout summary entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical reopen-map manifest memory では `split_packages` の next-queue 読みを外し、
   `split package closeout` と `closed_split_packages` を helper memory として保持してよい。

## current recommendation

- representative reopen-map は stale な next split queue を見せず、
  split-package closeout 済みであることを helper memory から読めるようにしていた historical cut として保持する。
- split helper command 自体は separate helper memory として保持し、
  reopen-map 側では remaining mixed gate / true user-spec residual の再読みに集中していた historical reading に留める。
- historical `reopen-map problem1|problem2` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem/model-check/witness-provider contract
- final public parser / checker / runtime API
- exhaustive shared-space catalog

## retained-later line

- split helper command family
- remaining mixed-gate compression after split closeout
- true user-spec residual
