# 595. current-l2 representative problem reopen-map split closeout sync

## 位置づけ

- current Phase 6 / Package 122 closeout。
- representative problem reopen-map から stale な `next split packages` 表示を外し、
  split-package closeout 後の residual public-seam maintenance 読みへ同期する。
- final public theorem/model-check/witness-provider contract や final parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem1|problem2`
   は remaining mixed gate / true user-spec residual を保ったまま、
   `split package closeout` を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py reopen-map problem1|problem2 --format json`
   は `closed_split_packages` を machine-readable に返してよい。
3. reopen-map の public manifest は `split_packages` の next-queue 読みを helper public surface から外し、
   split helper command 自体は `split problem* ...` 側に残してよい。

## current recommendation

- representative reopen-map は stale な next split queue を見せず、
  split-package closeout 済みであることを helper surface から読めるようにする。
- split helper command 自体は separate helper として保持し、
  reopen-map 側では remaining mixed gate / true user-spec residual の再読みに集中させる。
- reopen-map の current cut は residual public-seam maintenance の public helper mirror に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`

## stop line

- final public theorem/model-check/witness-provider contract
- final public parser / checker / runtime API
- exhaustive shared-space catalog

## retained-later line

- split helper command family
- remaining mixed-gate compression after split closeout
- true user-spec residual
