# 572 — current L2 guided problem sample entrypoints and runner

## 位置づけ

- historical Phase 6 / Package 98 closeout memory。
- 2026-04-22 clean-sample migration 前に使っていた
  `show problem1|problem2` / `run problem1|problem2 [--all]`
  helper と prototype README 導線を、
  archived guided-entrypoint memory として記録する。
- final public CLI、final public tutorial surface、
  final public parser / checker / runtime API、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py show problem1|problem2`
   と
   `python3 scripts/current_l2_guided_samples.py run problem1|problem2 [--all]`
   は、2026-04-22 clean-sample migration 前の
   historical helper-local entrypoint memory として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical prototype README memory は
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/README.md`
   と
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/README.md`
   に残し、
   current active sample root とは別に読む。

## current recommendation

- guided problem sample entrypoint / runner は、
  clean-near-end suite へ移る前に
  representative / reserve / negative sample をどう導入していたかの
  historical memory として保ってよい。
- archived prototype README と retired `show` / `run` helper は
  helper-local / non-production memory であり、
  current active sample surface に戻さない。
- current repo で representative floor を見るときは、
  active clean-near-end sample suite と
  `list / smoke-all / closeout` の compatibility front door を優先する。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py show problem1`
  - `python3 scripts/current_l2_guided_samples.py show problem2`
  - `python3 scripts/current_l2_guided_samples.py run problem1`
  - `python3 scripts/current_l2_guided_samples.py run problem2 --all`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/README.md`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/README.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public CLI
- final public tutorial surface
- exhaustive sample catalog
- final public parser / checker / runtime API
- final public verifier contract

## retained-later line

- Problem 1 theorem/model-check public-seam residual memory
- Problem 2 witness/provider/public-shape residual memory
