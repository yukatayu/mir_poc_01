# Report 1162 — P-A1-08 Product Prototype Recut Blocker

- Date: 2026-05-03
- Author / agent: Codex
- Scope: docs-only blocker/recut package for `P-A1-08` on the practical alpha-1 line
- Decision levels touched: practical alpha-1 stage wording, `plan/44` roadmap memory, snapshot/dashboard package promotion state
- 日本語要約: `P-A1-08` はそのまま promote すると overclaim になるため、実装へ進まずに blocker/recut package として固定した。review 3 系統の一致 finding は、現行 practical carriers だけで safe に compose できるのは local/Docker + layer attach + local save/load + devtools export の thin bundle までであり、`specs/18` が product prototype に要求している `custom Mir avatar runtime` と `unsupported runtime fallback` は current practical root に未actualize だというものだった。そのため、repo snapshot には `P-A1-08` recut-required を記録し、next reopen point を「thin product-preview bundle へ recut」か「practical `AV-A1-*` carrier を先に切るか」の user decision として残した。

## Objective

Record the newly discovered stage inconsistency before any `P-A1-08` implementation claim is made. Synchronize repository memory and current snapshots so they stop implying that `P-A1-08` can be promoted immediately under its current wording, and stop at a precise user-decision blocker as required by the plan-consult rule.

## Scope and assumptions

- This is a docs-only blocker package. No runtime/checker/hot-plug/transport/save-load code is widened.
- Scope is limited to blocker recording for the practical alpha-1 line:
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - new blocker report
- Normative `specs/18` wording is not edited in this package because the honest next move is still open between recut and prior avatar carrier work.
- The blocker is not “Docker/local transport is missing.” Existing practical carriers already cover that floor.
- The blocker is specifically the gap between current practical carriers and the current `P-A1-08` wording around avatar/product semantics.

## Start state / dirty state

- Work resumed on `main` after `P-A1-07` had already been closed and pushed.
- The worktree was clean at blocker-package start.
- The immediate trigger was the `P-A1-08` planning review after `P-A1-07`, where current snapshots still described `P-A1-08` as the next promoted package with no package-local blocker.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `docs/reports/1161-p-a1-07-local-save-load-command.md`
- `sub-agent-pro/alpha-1/02-practical-alpha1-definition.md`
- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`

## Actions taken

1. Re-read the practical alpha-1 stage roadmap, definition, sample matrix, validation plan, and current repo snapshots around `P-A1-08`.
2. Ran three read-only reviews focused on:
   - semantics / stage honesty,
   - runtime/helper composition feasibility,
   - docs/progress/sample consistency.
3. Compared the `P-A1-08` wording in `specs/18` and `plan/44` against the current practical carriers already actualized in the repo.
4. Identified the exact mismatch:
   current practical carriers can honestly compose local/Docker + layer attach + local save/load + devtools export, but do not yet actualize `custom Mir avatar runtime` or `unsupported runtime fallback`.
5. Chose not to promote or implement `P-A1-08` under the old wording.
6. Updated repository memory and current snapshots so they record `P-A1-08` as recut-required rather than “promoted next with no package-local blocker.”

## Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/1162-p-a1-08-recut-blocker.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M JST'
git status --short
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- `date`: `2026-05-03 23:32 JST`
- Review findings converged:
  - current practical carriers are sufficient for a thin product-like bundle over local/Docker + layer attach + local save/load + devtools export
  - they are not sufficient for the current `P-A1-08` wording that implies `custom Mir avatar runtime` and `unsupported runtime fallback`
  - the safest next shape is a recut to a thinner product-preview bundle, or a prior practical `AV-A1-*` carrier package
- Validation passed:
  - `python3 -m unittest scripts.tests.test_validate_docs`
    - `Ran 11 tests`
    - `OK`
  - `python3 scripts/check_source_hierarchy.py`
    - `required: 73`
    - `present: 73`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1163 numbered report(s).`
  - `cargo fmt --check`: passed
  - `git diff --check`: passed

## What changed in understanding

- The remaining problem is not transport, save/load, or devtools composition by itself. Those lanes can already be composed honestly as a thin bundle.
- The real mismatch is that the current `P-A1-08` wording inherits avatar/product expectations that the practical root does not yet satisfy.
- The correct next action is therefore not “implement the product prototype anyway,” but “recut the product prototype package or cut the avatar carrier first.”

## Open questions

- Should `P-A1-08` be recut into a **first practical product prototype preview** that explicitly narrows avatar scope to `HP-A1-06` placeholder object preview companion evidence?
- Or should practical `AV-A1-*` carriers be implemented first so the current `specs/18` / `plan/44` wording can remain literal?

## Suggested next prompt

Choose one of these two paths:

1. Recut `P-A1-08` into a first practical product-preview bundle over one world package plus companion layer/object packages, with avatar scope narrowed to `HP-A1-06` placeholder preview companion evidence.
2. Keep the current product-prototype wording and implement practical `AV-A1-*` carriers first.

## Plan update status

`plan/` 更新済み: `plan/44-practical-alpha1-roadmap.md` に `P-A1-08` current wording が recut-required であることと、2 つの safe next shapes を追記した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 current snapshot が `P-A1-08` blocked/recut-required 状態を反映するように同期した。

## progress.md update status

`progress.md` 更新済み: current blockers、next autonomous package、3-axis `着手可能/要仕様確認`、PA1-8 row、recent log を blocker state に同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-08` を promoted next から blocked-for-recut へ移し、ordered current work と avatar blocker wording を更新した。

## samples_progress.md update status

`samples_progress.md` 更新済み: practical stage header、active package text、toolchain summary、`PA1-8` row、recent validation、`PH0` freshness を blocker state に同期した。

## Reviewer findings and follow-up

- `Raman`:
  current wording の `P-A1-08` は avatar/product lane を overclaim しており、current practical root に `AV-A1-*` family がないまま promote すべきでない、という finding を採用した。
- `Aquinas`:
  safe next shape は monolithic package ではなく、one world package + companion layer/object packages + thin orchestrator + aggregate bundle report だという finding を採用した。
- `Mendel`:
  current snapshots / latest report / plan wording の衝突を指摘し、promote より先に recut/blocker sync が必要だという finding を採用した。

## Skipped validations and reasons

- No Rust, Python runtime, Docker, or practical sample implementation was changed in this blocker package, so lane-local runtime/integration floors were not rerun.
- `specs/18` itself was not edited because the honest next move remains open between recut and prior avatar-carrier work; this package records the blocker rather than freezing the answer silently.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Raman` completed the semantics/stage-honesty review and is pending closure
- `Aquinas` completed the runtime/helper feasibility review and is pending closure
- `Mendel` completed the docs/progress/sample consistency review and is pending closure
