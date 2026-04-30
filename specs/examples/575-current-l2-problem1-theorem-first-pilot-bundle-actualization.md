# 575 — current L2 Problem 1 theorem-first pilot bundle actualization

## 位置づけ

- historical Phase 6 / Package 101 closeout memory。
- `bundle problem1` helper と
  Problem 1 theorem-first bundle doc / Lean artifact / anchor docs 導線を、
  2026-04-22 clean-sample migration 前の
  historical theorem-first bundle memory として記録する。
- final public theorem contract、
  concrete theorem prover brand adoption、
  final public verifier contract、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py bundle problem1`
   は historical helper-local theorem-first bundle memory として扱い、
   current active compatibility front door には戻さない。
2. Problem 1 bundle doc / prototype README / Lean artifact memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/README.md`
   `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p06-typed-proof-owner-handoff/README.md`
   以下の archived path に残してよい。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- `bundle problem1` は、
  representative `p06` と supporting `p10 / p11 / p12 / p15 / p16` を
  theorem-first bundle としてどう辿っていたかの
  historical memory として読む。
- archived bundle doc / prototype README / Lean artifact 導線は残してよいが、
  helper command 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem1 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-typed-proof-model-check/README.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem contract
- concrete theorem prover brand adoption
- final public verifier contract
- final public parser / checker / runtime API

## retained-later line

- parser-side companion archived compare floor
- theorem public-contract split memory
- emitted-artifact reopen memory
