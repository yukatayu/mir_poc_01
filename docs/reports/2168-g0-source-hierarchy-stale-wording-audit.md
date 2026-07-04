# Report 2168 — G0 source-hierarchy stale wording audit

- Date: 2026-07-04 10:52 JST
- Author / agent: Codex
- Scope: docs-only focused stale wording audit after `plan/119`
- Decision levels touched: no canon decision changed; LAB wording cleanup only

## Objective

Run the narrow stale wording audit identified by `plan/119`: find and correct
current LAB/root docs that could still re-promote legacy `specs/`, helper
closeouts, sample evidence, Lean compile checks, or old wording into canon
status or workflow-ready runtime status.

## Scope and assumptions

Scope is limited to wording in root docs, snapshot docs, repository memory,
traceability rows, and the runnable sample dashboard. The package does not edit
`mirrorea_canon/`, does not change any canon L0/L1 source, does not create an
ADR, does not resolve an OPEN item, does not close G0 or G1..G7, does not move
the project out of T0, does not move any OBL or THM status, does not claim proof
discharge, does not claim conformance, does not change implementation behavior,
does not change runnable rows, and does not promote sample / helper / report /
Lean evidence to status.

Working assumption: stale wording can be corrected mechanically when the
correct source-hierarchy reading is already fixed by canon and `plan/70` /
`plan/119`. No historical LAB evidence is rewritten beyond the local wording
needed to prevent current-reader confusion.

## Start state / dirty state

Start state was clean on `main` at `67cf89cd`, matching `origin/main`, after
the `plan/119` package and report-status update were committed and pushed.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
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
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/01-status-at-a-glance.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/90-source-traceability.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`

## Actions taken

- Updated `README.md` reading order to put `mirrorea_canon/README.md`,
  `mirrorea_canon/MAP.md`, and task-specific canon files before LAB follow-up
  docs.
- Demoted legacy `specs/` / `plan/` references in `Documentation.md` to LAB
  package-line memory / roadmap memory wording.
- Split `Documentation.md` runnable workflow wording from the Surface
  evidence-closed alpha line.
- Rewrote `samples_progress.md` opening focus text into workflow focus,
  evidence-only focus, and non-claims.
- Corrected legacy source-hierarchy wording in `plan/01`, `plan/07`,
  `plan/09`, `plan/19`, and `plan/57`.
- Fenced the `plan/70` read/write/dependency row with `plan/119` wording:
  narrow G1 support only unless a later gate explicitly opens G4 observation;
  no runtime graph or event-model widening.
- Added `LAB:` prefixes for `progress.md` and `tasks.md` in the `plan/119`
  source traceability row.
- Updated `plan/119`, `progress.md`, and `tasks.md` with the audit follow-up.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/01-status-at-a-glance.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/90-source-traceability.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `docs/reports/2168-g0-source-hierarchy-stale-wording-audit.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- Multiple `rg` source-hierarchy / overclaim scans across root docs, snapshots,
  `plan/`, `samples_progress.md`, and `AGENTS.md`
- `nl -ba ... | sed -n ...` on the touched docs
- `git diff --stat`
- `git diff -- ...` on touched files
- `date '+%Y-%m-%d %H:%M %Z'`
- `git rev-parse --short HEAD`
- `git rev-parse --short origin/main`
- `git diff --name-only`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`
- Changed-file endpoint leak scan over `git ls-files --modified --others --exclude-standard`
- Stale wording scan for legacy `specs/` normative-source patterns

## Evidence / outputs / test results

Local validation passed:

- `python3 scripts/check_source_hierarchy.py`
  - required: 602
  - present: 602
  - missing: 0
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 1320 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs`
  - 20 tests passed.
- `git diff --check`
  - passed with no output.
- Changed-file endpoint leak scan
  - `no endpoint matches in changed files`
- Stale wording scan for legacy `specs/` normative-source patterns
  - only remaining non-report hit is the intentional rejected-pattern row in
    `plan/70`: `Legacy specs as current normative source` -> `superseded`.
  - the report itself repeats that phrase only to document the intentional
    remaining hit.

## What changed in understanding

The main remaining stale wording risk was not in `progress.md`, `tasks.md`, or
`plan/119`; those were already mostly aligned. The risk was in compressed reader
entry points and older LAB memory:

- `README.md` still routed readers from the root doc directly into legacy
  `specs/` / `plan/` as the default next path.
- `Documentation.md` compressed many historical lines and could make legacy
  `specs/` / `plan/` sound authoritative even under a correct canon banner.
- `samples_progress.md` compressed workflow-ready rows, evidence-closed rows,
  and G1 LAB addenda into one opening sentence.
- Older `plan/01`, `plan/07`, `plan/09`, `plan/19`, and `plan/57` still had direct
  `specs/`-as-source wording.

The corrected reading is now explicit in the touched current docs:
`mirrorea_canon/` is canon, legacy `specs/` are LAB evidence / historical
package-line memory, Surface alpha rows are evidence-closed unless a row is
explicitly workflow-ready, and helper / code artifacts are not semantic source
of truth.

## Open questions

- Should a later mechanical linter detect stale `specs/`-as-normative wording,
  or is focused manual grep sufficient for now?
- Should `Documentation.md` be structurally shortened later to reduce future
  drift risk, or does the current compact snapshot remain worth the density?

## Suggested next prompt

Continue self-driven work by either adding a small source-hierarchy wording
lint, or returning to a concrete G1 support package only if a specific gap is
identified. Do not edit canon or widen runtime / observation behavior by
default.

## Plan update status

`plan/` 更新済み:

- Updated `plan/01`, `plan/07`, `plan/09`, `plan/19`, `plan/57`, `plan/70`,
  `plan/90`, and `plan/119`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Demoted legacy `specs/` / `plan/` wording to LAB memory.
- Split runnable workflow wording from the Surface evidence-closed alpha line.

## progress.md update status

`progress.md` 更新済み:

- Added the focused source-hierarchy stale wording audit note and recent log
  entry.

## tasks.md update status

`tasks.md` 更新済み:

- Added the focused source-hierarchy stale wording audit holding-state bullet
  and maintenance-row update.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Rewrote the opening focus sentence into workflow focus, evidence-only focus,
  and non-claims. No row, command, blocker, validation anchor, or sample status
  changed.

## Reviewer findings and follow-up

Read-only sub-agent Sartre found six stale wording risks after the local grep
pass: `README.md` reading order, `Documentation.md` legacy authoritative verbs,
`Documentation.md` runnable/evidence compression, `plan/70` G1/G4
read/write/dependency follow-up wording, `plan/90` missing `LAB:` prefixes for
snapshot sources, and `samples_progress.md` opening-status compression.

Follow-up applied in all six areas. Sartre also found no issue in
`progress.md`, `tasks.md`, `plan/119`, or `AGENTS.md` regarding the main
non-claims.

Final reviewer Nietzsche found five closeout issues: `README.md` reading-order
canon paths were not fully qualified, existing README future-axis lines still
used `規範判断は specs/...`, several `plan/07` detached-exporter rows still used
`specs/examples` as `正本`, `samples_progress.md` had moved the Full System V1
bounded release-check lane into evidence-only wording, and this report still
contained final-review pending language. Follow-up applied: canon paths are now
fully qualified as `mirrorea_canon/...`; README and `plan/01` legacy lines now
say LAB evidence; `plan/07` uses legacy LAB anchor wording; Full System V1 is
back in workflow focus in `samples_progress.md`; and this report records the
actual final review state. Nietzsche ran read-only diff / grep / line
inspection plus `git diff --check`, which passed.

## Skipped validations and reasons

Cargo, Lean, Surface helper suites, runtime helpers, conformance helpers, and
sample dashboards are not planned for this package. The package is docs-only
and changes no Rust source, Lean source, sample fixture, expected JSON, helper
behavior, runnable command, runnable row, or sample status. No Cargo, Lean,
Surface helper, runtime, conformance, or sample success is claimed by this
report.

## Commit / push status

Pending.

## Sub-agent session close status

Sartre returned completed read-only findings and was closed. Final reviewer
Nietzsche returned final review findings and remains open only until this
package is committed or another reviewer is needed. No sub-agent edits were
made.
