# 574 — current L2 Problem 2 public-shape residual bundle matrix

## 位置づけ

- historical Phase 6 / Package 100 closeout memory。
- `matrix problem2` helper と
  Problem 2 representative / reserve / negative pair の bundle 読みを、
  2026-04-22 clean-sample migration 前の
  historical residual-matrix memory として記録する。
- final public witness/provider/artifact contract、
  exhaustive shared-space catalog、
  stronger fairness / replay profile を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py matrix problem2`
   は historical helper-local residual-bundle matrix memory として扱い、
   current active compatibility front door には戻さない。
2. Problem 2 representative / reserve / negative set
   `p07 / p08 / p09 / p13 / p14`
   の読み分けは、
   archived Problem 2 bundle doc memory と historical matrix memory に残してよい。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- `matrix problem2` は、
  `p07 / p08` representative pair、
  `p09` reserve route、
  `p13 / p14` negative pair をどう分けていたかの
  historical compare-floor memory として読む。
- witness/provider public-shape residual と
  order-handoff source wording residual を分けて読む意図は残してよいが、
  helper command 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py matrix problem2`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public witness schema
- final public provider receipt schema
- final emitted-handoff contract
- exhaustive shared-space catalog
- stronger fairness / replay / provider attestation profile

## retained-later line

- source wording / emitted schema split memory
- witness/provider public-shape split memory
- reserve strengthening reopen memory
