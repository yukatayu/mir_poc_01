# 581. current-l2 explained representative problem sample bundles

## 位置づけ

- historical Phase 6 / Package 107 closeout memory。
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/` guide と
  `bundle problem1|problem2` helper によって
  representative sample を explained bundle として読んでいた導線を、
  2026-04-22 clean-sample migration 前の historical bundle-index memory として記録する。
- final public tutorial surface、final public parser / checker / runtime API、
  final public verifier contract、exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. explained bundle docs は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
   と Problem 1 / Problem 2 guide に archived memory として残してよい。
2. `python3 scripts/current_l2_guided_samples.py bundle problem1|problem2`
   は historical helper-local bundle-index memory として扱い、
   current active compatibility front door には戻さない。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- explained representative bundle は、
  prototype / Lean artifact / parser companion / helper command を
  どう 1 本の guide に束ねていたかの historical bundle-index memory として読む。
- archived bundle docs は残してよいが、
  `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/` archived guide path や `bundle` helper を current active sample surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- exhaustive tutorial expansion
- exhaustive sample catalog
- final public parser / checker / runtime API
- final public verifier contract
- exhaustive shared-space catalog

## retained-later line

- surviving `smoke-all` compatibility front door
- clean-near-end current representative suite
