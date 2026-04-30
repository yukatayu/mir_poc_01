# 573 — current L2 Problem 1 public-seam residual bundle matrix

## 位置づけ

- historical Phase 6 / Package 99 closeout memory。
- `matrix problem1` helper と
  Problem 1 representative/supporting bundle 読みを、
  2026-04-22 clean-sample migration 前の
  historical residual-matrix memory として記録する。
- final public theorem/model-check contract、
  final public checker artifact、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py matrix problem1`
   は historical helper-local residual-bundle matrix memory として扱い、
   current active compatibility front door には戻さない。
2. Problem 1 representative/supporting set
   `p06 / p10 / p11 / p12 / p15 / p16`
   の読み分けは、
   archived Problem 1 bundle doc memory と historical matrix memory に残してよい。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- `matrix problem1` は、
  `p06` representative と
  `p10 / p11 / p12 / p15 / p16` supporting set を
  theorem/model-check public seam residual としてどう分けていたかの
  historical compare-floor memory として読む。
- typed / theorem / model-check residual を Problem 1 の中で分けて読む意図は残してよいが、
  helper command 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py matrix problem1`
  - `python3 scripts/current_l2_guided_samples.py matrix problem1 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem result object
- consumer-shaped theorem payload public contract
- first settled property language
- concrete theorem/model-check tool binding
- final public checker artifact
- final public verifier contract

## retained-later line

- typed source principal actual adoption
- theorem public-contract split memory
- model-check public-contract split memory
