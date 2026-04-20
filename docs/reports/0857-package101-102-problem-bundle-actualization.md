# 0857 — Package 101 / 102 problem bundle actualization

## Objective

Package 101 theorem-first pilot bundle と Package 102 authoritative-room scenario bundle を、
`scripts/current_l2_guided_samples.py` の `bundle` subcommand、sample README、日本語 explanation、
spec / plan / progress / tasks の snapshot 同期まで含めて close する。

## Scope and assumptions

- `specs/` を規範正本とし、`plan/` を repository memory、`progress.md` / `tasks.md` を snapshot として扱う。
- helper は repo-local / non-production の bundle 導線であり、final public contract や final grammar へ上げない。
- representative sample corpus や Lean artifact corpus 自体は増やさず、既存 corpus の導線整理に絞る。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md`
- `specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md`
- `specs/examples/509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`
- `specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md`
- `specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md`
- `specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `scripts/current_l2_guided_samples.py` に `bundle problem1|problem2` を追加し、
   representative sample、Lean artifact、anchor docs / reports、stop line を 1 画面で辿れる helper を actualize した。
2. `scripts/tests/test_current_l2_guided_samples.py` に bundle helper の RED/GREEN test を追加した。
3. sample README 2 本を更新し、`bundle problem1` / `bundle problem2` の日本語導線を追加した。
4. Problem 1 / Problem 2 の bundle actualization を `specs/examples/575` と `576` に記録した。
5. `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、`specs/00`、`specs/11`、`specs/12`、`plan/90` を同期し、
   Package 101 / 102 close 後の next queue を experimental parser-side companion surface bundle / thin bridge line に更新した。

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - 12 tests passed
- `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - Problem 1 representative sample / Lean artifact / doc anchor / stop line を確認
- `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - Problem 2 representative pair / reserve / negative / Lean artifact / doc anchor / stop line を確認
- `python3 scripts/current_l2_guided_samples.py bundle problem1 --format json`
  - bundle manifest JSON を確認
- `python3 scripts/current_l2_guided_samples.py bundle problem2 --format json`
  - bundle manifest JSON を確認
- `python3 scripts/validate_docs.py`
  - documentation scaffold passed
- `git diff --check`
  - no whitespace / conflict marker issue

## What changed in understanding

- Problem 1 / Problem 2 ともに、current representative sample と residual matrix までは既に揃っていたが、
  Lean artifact / spec-report anchor / stop line を 1 本で辿る helper がないため、
  「実際にどこまで行けるか」の把握が still fragmented だった。
- `bundle` helper を足すことで、current first line と retained-later stop line を同時に visible に保てるようになった。
- self-driven next line は docs bundle closeout の後、parser-side thin experimental surface と parser-to-helper thin bridge へ戻すのが自然になった。

## Open questions

- next parser-side package で、どの representative sample を thin experimental surface の first slice に採るか。
- parser-side carrier から current helper summary へ何を最小 bridge にするか。
- final public parser/checker/runtime API や final public witness/provider/theorem contract には引き続き入らないことを、どの程度 sample 側で visible にするか。

## Suggested next prompt

Package 103 / 104 として、parser-side thin experimental surface bundle と parser-side carrier から current helper summary への thin bridge を self-driven に actualize してください。 representative sample は Problem 1 なら `p06`、Problem 2 なら `p07 / p08` を first slice にし、final grammar / final public API へは上げないでください。
