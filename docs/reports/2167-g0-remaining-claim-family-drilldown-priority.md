# Report 2167 — G0 remaining claim-family drilldown priority map

- Date: 2026-07-04 10:38 JST
- Author / agent: Codex
- Scope: docs-only LAB scheduling / priority package for remaining `plan/70`
  claim-family rows after `plan/118`
- Decision levels touched: no canon decision changed; LAB repository memory only

## Objective

Classify the remaining `plan/70` LAB-to-canon claim-family rows after the
ordinary Surface assignment drilldown in `plan/118`, so future autonomous work
does not treat every high-risk row as an immediate line-level drilldown target.

## Scope and assumptions

Scope is limited to LAB docs, repository-memory indexing, snapshot docs, and
this report. The package does not edit `mirrorea_canon/`, does not change any
canon L0/L1 source, does not create an ADR, does not resolve an OPEN item, does
not close G0 or G1..G7, does not move the project out of T0, does not move any
OBL or THM status, does not claim proof discharge, does not claim C-static /
C-runtime / C-distributed conformance, does not freeze a grammar / API / Core
IR / diagnostic / repair / runtime / transport / projection / provider ABI,
does not claim implementation completion, and does not promote helper, sample,
report, Oracle, sub-agent, or Lean compile-check evidence to canon.

Working assumption: after `plan/118`, no remaining `plan/70` row is an
immediate default drilldown target. The only narrow exception is the
read/write/dependency support row, and only if a concrete G1 ordinary-assignment
support gap is later found. That exception must not widen into G4 observation
semantics, runtime graph machinery, or event-model restatement.

## Start state / dirty state

Start state was clean on `main` at `a80b93f8`, matching `origin/main`, after
the `plan/118` package and its report-status update were committed and pushed.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/adr/ADR-0012.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/theory/00-overview.md`
- `mirrorea_canon/architecture/01-strata.md`
- `mirrorea_canon/architecture/02-boundary-contracts.md`
- `plan/00-index.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md`
- `docs/reports/2166-g0-g1-ordinary-assignment-claim-family-drilldown.md`

## Actions taken

- Added `plan/119-g0-remaining-claim-family-drilldown-priority.md` as a
  LAB-only priority map for the remaining `plan/70` rows.
- Updated `plan/70` safe next actions so future work uses `plan/119` before
  opening another `plan/70` drilldown.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `README.md`, `Documentation.md`, `progress.md`, and `tasks.md` to
  reflect the remaining-claim-family priority map without changing canon status.
- Integrated read-only sub-agent review: remaining rows are not mandatory next;
  the read/write/dependency row is only an optional narrow G1 support drilldown
  if a concrete support gap remains.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/90-source-traceability.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `docs/reports/2167-g0-remaining-claim-family-drilldown-priority.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `nl -ba plan/119-g0-remaining-claim-family-drilldown-priority.md | sed -n '1,240p'`
- `nl -ba plan/70-lab-to-canon-reconciliation-ledger.md | sed -n '55,110p'`
- `nl -ba progress.md | sed -n '1,130p'`
- `nl -ba tasks.md | sed -n '1,120p'`
- `nl -ba tasks.md | sed -n '330,385p'`
- `test -e docs/reports/2167-g0-remaining-claim-family-drilldown-priority.md; echo $?`
- `find docs/reports -maxdepth 1 -type f -printf '%f\n' | sort -V | tail -n 10`
- `nl -ba README.md | sed -n '1,120p'`
- `nl -ba Documentation.md | sed -n '55,90p'`
- `sed -n '1,240p' docs/reports/2166-g0-g1-ordinary-assignment-claim-family-drilldown.md`
- `git diff --stat`
- `git diff -- plan/119-g0-remaining-claim-family-drilldown-priority.md tasks.md progress.md`
- `date '+%Y-%m-%d %H:%M %Z'`
- `git rev-parse --short HEAD`
- `git rev-parse --short origin/main`
- `git diff --name-only`
- `git ls-files --others --exclude-standard`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`
- Changed-file endpoint leak scan over `git ls-files --modified --others --exclude-standard`
- `git add README.md Documentation.md progress.md tasks.md plan/00-index.md plan/70-lab-to-canon-reconciliation-ledger.md plan/90-source-traceability.md plan/119-g0-remaining-claim-family-drilldown-priority.md docs/reports/2167-g0-remaining-claim-family-drilldown-priority.md`
- Staged-file endpoint leak scan over `git diff --cached --name-only --diff-filter=ACM`
- `git commit --no-gpg-sign -m "Add remaining claim priority map"`
- `git push`

## Evidence / outputs / test results

Local validation passed:

- `python3 scripts/check_source_hierarchy.py`
  - required: 602
  - present: 602
  - missing: 0
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 1319 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs`
  - 20 tests passed.
- `git diff --check`
  - passed with no output.
- Changed-file endpoint leak scan
  - `no endpoint matches in changed files`

## What changed in understanding

`plan/118` closes the need for an immediate ordinary-assignment claim-family
drilldown, but it should not automatically create a queue of all remaining
`plan/70` rows. The safer current management posture is a priority map:
canonized / separability rows need only stale wording audits; later G2..G7 rows
should wait for prerequisite gate context; evidence-only rows should be cited
as command/report evidence, not status; process-only rows remain advisory.

The read/write/dependency row is the one remaining G1-adjacent row that may
justify a narrow support drilldown, but only after a concrete ordinary
assignment support gap is found and only without widening into observation or
runtime graph semantics.

## Open questions

- Which later gate, if any, should become the next promoted theory line after
  G1 ordinary-assignment / diagnostic explanation boundaries are stable enough?
- Should `plan/70` and `plan/119` later become machine-readable ledgers, or is
  human-readable repository memory sufficient until a G0 close review?
- Should canon receive a short mental-model clarification proposal later, or is
  the current canon wording sufficient until human review requests movement?

## Suggested next prompt

Continue self-driven work with a focused stale wording audit or a concrete G1
support package only if a specific gap is identified. Do not drill later-gate
rows, edit canon, or widen runtime / observation behavior by default.

## Plan update status

`plan/` 更新済み:

- Added `plan/119-g0-remaining-claim-family-drilldown-priority.md`.
- Updated `plan/70`, `plan/00-index.md`, and `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added the remaining claim-family priority map to the Surface / G1 LAB memory
  snapshot while keeping the non-claim boundary explicit.

## progress.md update status

`progress.md` 更新済み:

- Added a current milestone note and recent log entry for `plan/119`.

## tasks.md update status

`tasks.md` 更新済み:

- Added a current holding-state bullet for `plan/119`.
- Updated the remaining LAB claim-family drilldown candidate to say no remaining
  row is an immediate default target, with only a narrow G1 read/write/dependency
  support exception if a concrete gap is found.

## samples_progress.md update status

`samples_progress.md 更新不要`:

- No runnable sample status, command, row count, blocker, debug surface, or
  validation anchor changed.

## Reviewer findings and follow-up

Read-only sub-agent Locke confirmed the current canon phase is still T0/G0,
that `plan/118` was the correct first drilldown because ordinary assignment is
the current G1 pressure case, and that remaining non-ordinary rows should not
be drilled automatically. Locke classified indexed-state/admission as G3,
lifetime/fallback as G2, cut/save as G5, observation as G4, projection/backend
as G6, and hot-plug as G7; source hierarchy, project-axis vocabulary, strata
separation, and subsystem separability were classified as already canonized
for current purposes. Locke also warned that Product Alpha / workflow rows,
Lean stubs, and Oracle/sub-agent outputs should remain evidence/process only.

Follow-up applied: `plan/119`, `tasks.md`, and `progress.md` now preserve the
one narrow optional exception for read/write/dependency support only if a
concrete G1 ordinary-assignment support gap remains.

Final reviewer Poincare found one issue: stale pending closeout language in
this report. That finding is resolved by this edit. Poincare found no
canon/status overclaim, no G0 or G1..G7 exit claim, no OBL/proof/conformance /
implementation/sample status movement, and no G1 read/write/dependency
exception widening into G4 observation, runtime graph machinery, or event-model
semantics. Poincare's residual risk was limited to not running Cargo, Lean,
runtime, or sample suites for this docs-only diff.

## Skipped validations and reasons

Cargo, Lean, Surface helper suites, runtime helpers, conformance helpers, and
sample dashboards are not planned for this package. The package is docs-only
and changes no Rust source, Lean source, sample fixture, expected JSON, helper
behavior, runnable command, or sample status. No Cargo, Lean, Surface helper,
runtime, conformance, or sample success is claimed by this report.

## Commit / push status

Package commit / push completed:

- Commit: `bedead89` (`Add remaining claim priority map`)
- Push: completed to `origin/main`

This report-status update will be committed and pushed immediately after this
edit.

## Sub-agent session close status

Locke returned completed mapping findings and was closed. Poincare returned
final review findings and was closed. Both were read-only for this package, and
no sub-agent edits were made.
