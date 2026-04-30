# 586. current-l2 representative problem quickstart CLI mirror actualization

## 位置づけ

- historical Phase 6 / Package 112 closeout memory。
- `quickstart problem1|problem2` helper によって
  archived problem-bundle quickstart を helper-side summary でも mirrored していた導線を、
  historical quickstart-CLI memory として記録する。
- exhaustive tutorial surface、final public CLI / tutorial surface、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py quickstart problem1|problem2`
   は historical helper-local quickstart-CLI memory として扱い、
   current active compatibility front door には戻さない。
2. archived quickstart docs は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/`
   側の historical memory として読む。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- quickstart CLI mirror は、
  archived bundle quickstart を helper でも読めるようにしていた
  historical memory として保ってよい。
- helper command 自体は current active command surface に戻さず、
  surviving `smoke-all` / `closeout` current line と分けて扱う。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py quickstart problem1`
  - `python3 scripts/current_l2_guided_samples.py quickstart problem2 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`

## stop line

- exhaustive tutorial surface
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- archived quickstart parity memory
- broader sample catalog widening
