# Report 2177 — Plan scaffold guard coverage refresh

- Date: 2026-07-04 13:08 JST
- Author / agent: Codex
- Scope: documentation scaffold guard maintenance
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Synchronize the documentation scaffold guards with current `plan/` repository
memory by requiring `plan/97..119` in both `scripts/validate_docs.py` and
`scripts/check_source_hierarchy.py`.

## Scope and assumptions

Scope:

- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `plan/00-index.md`
- `scripts/README.md`
- snapshot logs that track guard maintenance

Assumptions:

- `plan/97..119` are current LAB repository-memory files referenced by
  `progress.md` / `tasks.md`, not throwaway drafts.
- Requiring their structural presence does not promote them to canon and does
  not claim G1 exit, proof completion, conformance, sample workflow readiness,
  or implementation completion.
- The validators still check presence / reader-facing drift shape only; they do
  not validate semantic correctness of the plan contents.

## Start state / dirty state

Package 39 started from clean `HEAD == origin/main == 5ca710ed`.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Compared actual `plan/` files with the explicit required-path lists in
  `scripts/validate_docs.py` and `scripts/check_source_hierarchy.py`.
- Added a regression expectation for `plan/97..119` to
  `test_required_scaffold_includes_surface_mir_rebaseline_docs`.
- Confirmed the regression test failed before implementation because the
  required set omitted current plan files.
- Added `plan/97..119` to both required-path lists.
- Updated `scripts/README.md` to describe `check_source_hierarchy.py` as
  covering `plan/39..119`.
- Added the previously omitted full-filename entries for `plan/106..108` and
  `plan/118..119` to `plan/00-index.md`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` as maintenance
  snapshot entries.

## Files changed

- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `plan/00-index.md`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2177-plan-scaffold-guard-coverage-refresh.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_validate_docs.TestValidateDocs.test_required_scaffold_includes_surface_mir_rebaseline_docs`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_required_scaffold_includes_surface_mir_rebaseline_docs`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- read-only reviewer sub-agent `James` for adjacent drift audit

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs` passed: 31 tests.
- `python3 scripts/validate_docs.py` passed and found 1329 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  625/625.
- `git diff --check` passed.
- Read-only reviewer sub-agent validation also passed for the same four command
  families and reported no edits.

TDD RED evidence:

- The first targeted unittest invocation used the wrong class name and did not
  reach the test.
- The corrected targeted unittest invocation failed because current required
  docs did not include `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`.

TDD GREEN evidence:

- The corrected targeted unittest passed after adding `plan/97..119` to both
  guard required-path lists.

## What changed in understanding

The previous maintenance refresh correctly noticed stale `scripts/README.md`
wording, but the deeper guard drift was real: both validators still stopped at
`plan/96` even though current snapshots actively rely on `plan/97..119`.

## Open questions

- Whether future plan-file additions should be discovered by a range/generator
  helper rather than manually duplicated in two required-path lists remains
  open. This package keeps the existing explicit-list style.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, prioritizing small guard
drift fixes that keep reader-facing status and actual validation boundaries in
sync.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md` now lists the previously omitted detailed filenames for
  `plan/106..108` and `plan/118..119`.
- This is index maintenance only. No roadmap, semantics, open-question,
  source-traceability, or repository memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the `plan/97..119` scaffold guard coverage
  refresh and updated the top `最終更新` timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Updated the current holding-state maintenance note so the source-hierarchy
  guard coverage reads as `plan/39..119`, not `plan/39..96`.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not
  change.

## Reviewer findings and follow-up

Read-only reviewer sub-agent `James` confirmed that the current working tree
resolved the `plan/97..119` guard drift. It found one adjacent index gap:
`plan/00-index.md` named `plan/106..108` and `plan/117..119` in the short list,
but the later detailed full-filename list stopped at `plan/105` and omitted
`plan/106`, `plan/107`, `plan/108`, `plan/118`, and `plan/119`. This follow-up
was accepted and fixed in this package.

## Skipped validations and reasons

No executable sample validation is required for this guard/list maintenance
update. Docs validators, unit tests for `validate_docs`, source-hierarchy check,
and whitespace checks are run before commit.

## Commit / push status

Commit and push pending at this report update step.

## Sub-agent session close status

Reviewer sub-agent `James` completed read-only review and was closed after its
findings were recorded.
