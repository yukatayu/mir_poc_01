# 587. current-l2 representative problem quickstart parity checks actualization

## 位置づけ

- historical Phase 6 / Package 113 closeout memory。
- `quickstart-parity` helper により
  archived sample bundle doc と archived quickstart CLI mirror の parity を見ていた導線を、
  historical quickstart-parity memory として記録する。
- exhaustive tutorial validation、final public CLI / tutorial surface、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py quickstart-parity`
   は historical helper-local parity memory として扱い、
   current active compatibility front door には戻さない。
2. parity target は archived problem-bundle docs と archived quickstart memory に留める。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。

## current recommendation

- quickstart parity check は、
  archived quickstart doc / helper mirror の drift suppression をしていた
  historical memory として保ってよい。
- helper command 自体は current active command surface に戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py quickstart-parity`
  - `python3 scripts/current_l2_guided_samples.py quickstart-parity --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- exhaustive tutorial validation
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- reopen-map historical memory
- broader sample catalog widening
