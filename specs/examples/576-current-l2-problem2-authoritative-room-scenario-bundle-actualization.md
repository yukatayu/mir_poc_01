# 576 — current L2 Problem 2 authoritative-room scenario bundle actualization

## 位置づけ

- historical Phase 6 / Package 102 closeout memory。
- `bundle problem2` helper と
  Problem 2 authoritative-room scenario bundle doc / Lean artifact / anchor docs 導線を、
  2026-04-22 clean-sample migration 前の
  historical scenario-bundle memory として記録する。
- final public witness/provider/artifact contract、
  exhaustive shared-space catalog、
  final source wording / emitted schema を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py bundle problem2`
   は historical helper-local scenario-bundle memory として扱い、
   current active compatibility front door には戻さない。
2. Problem 2 bundle doc / prototype README / Lean artifact memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/README.md`
   `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/p07-dice-late-join-visible-history/README.md`
   と related archived paths に残してよい。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- `bundle problem2` は、
  `p07 / p08` representative pair、
  `p09` reserve route、
  `p13 / p14` negative pair を
  authoritative-room scenario bundle としてどう辿っていたかの
  historical memory として読む。
- archived bundle doc / prototype README / Lean artifact 導線は残してよいが、
  helper command 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/README.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public witness schema
- final public provider receipt schema
- final public witness/provider/artifact contract
- exhaustive shared-space catalog

## retained-later line

- parser-side companion archived compare floor
- source wording / emitted schema split memory
- witness/provider public-shape split memory
