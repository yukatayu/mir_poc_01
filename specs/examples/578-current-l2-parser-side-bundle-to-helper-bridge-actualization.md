# 578 — current L2 parser-side bundle-to-helper bridge actualization

## 位置づけ

- historical Phase 6 / Package 104 closeout memory。
- `bundle problem1|problem2` helper に
  `parser_companion_path` を見せていた導線を、
  2026-04-22 clean-sample migration 前の
  historical bundle-to-helper bridge memory として記録する。
- final public parser / checker / runtime API、
  full `Program` lowering、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py bundle problem1|problem2`
   は historical helper-local bridge memory として扱い、
   current active compatibility front door には戻さない。
2. representative slice `p06 / p07 / p08`
   の parser companion bridge は、
   archived bundle doc memory と archived parser-companion sample set の対応として残してよい。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- bundle helper から parser-companion path を見せていた読みは、
  representative archived compare floor をどう辿っていたかの
  historical readability memory として保ってよい。
- bridge memory は残してよいが、
  `bundle` helper 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/README.md`

## stop line

- final public parser / checker / runtime API
- full `Program` lowering
- final public verifier contract

## retained-later line

- parser-side archived compare-floor inspection
- parser-side mapping memory
