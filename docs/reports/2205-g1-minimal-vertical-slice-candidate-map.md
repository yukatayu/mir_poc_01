# Report 2205 — G1 minimal vertical slice candidate map

- Date: 2026-07-04 17:44 JST
- Author / agent: Codex
- Scope: LAB repository memory, snapshot docs, docs validators, and report
- Decision levels touched: L1/L2 references only; no canon decision changed

## Objective

Create a safe LAB-only map for the minimal source-first static slice that can
feed later G1 ordinary-assignment work after the `plan/120` repo-triage recut.

## Scope and assumptions

The source hierarchy remains unchanged: `mirrorea_canon/` is normative, and
`plan/`, legacy `specs/`, samples, helpers, reports, Rust code, and Lean
statement drafts are LAB evidence or repository memory.

The slice is a static source-first consequence map, not an end-to-end runtime
slice, conformance slice, release slice, implementation package, or product
slice.

## Start state / dirty state

Start state was clean on `main` with `main...origin/main`. The previous package
had already pushed `7d79739919cdc3dd2c9a3790bc12dc803d56edbc`.

The task baseline was recorded with the Discord report skill before edits.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/69-consultation-synthesis-and-management-roadmap.md`
- `plan/71-g1-ordinary-assignment-target.md`
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md`
- `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `plan/120-repo-triage-recut-matrix.md`
- `plan/90-source-traceability.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`

## Actions taken

- Added `plan/121-g1-minimal-vertical-slice-candidate-map.md` as LAB
  repository memory.
- Defined the planning label `G1-MVS-ASSIGNMENT-STATIC` and descriptive name
  `source-first static assignment spine`.
- Mapped the static vertical path:
  `.mir` fixture -> parser / AST evidence -> indexed-state and role-context
  static checks -> Surface-to-Core elaboration -> consequence inventory ->
  LAB-only Lean statement-shape guards.
- Limited the candidate to Surface syntax, indexed-state owner/keyspace
  pressure, owner-directed write, RHS dependency, visible publish / observe,
  failure-row containment, authority-obligation carrier, source spans, and
  OBL-001/020/021 statement-boundary evidence.
- Added explicit stop lines for role admission, fallback, cut/save-load,
  projection, hot-plug, devtools ABI, runtime, product, transport, provider,
  public API, and domain-vocabulary promotion.
- Added narrow executable evidence rows `ELAB-02`, `ELAB-05`, `ELAB-07`,
  `ELAB-09`, `ELAB-10`, `ELAB-11`, and `ELAB-12`.
- Updated `plan/00-index.md`, `plan/90-source-traceability.md`,
  `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `scripts/README.md`, `scripts/validate_docs.py`,
  `scripts/check_source_hierarchy.py`, and
  `scripts/tests/test_validate_docs.py`.
- Consulted a read-only sub-agent and ChatGPT Pro Oracle for overclaim risk.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/121-g1-minimal-vertical-slice-candidate-map.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2205-g1-minimal-vertical-slice-candidate-map.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `rg -n "minimal|vertical|slice|ordinary|assignment|G1|SCN|keep-core|useful-floor|Immediate scheduling|Open questions|ELAB" ...`
- `rg --files plan`
- `sed -n ...` for consulted docs listed above
- `oracle status`
- `ask-chatgpt-pro ... --file ...`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py --format json`
- `git diff --check`

One early `rg` command used incorrect plan filenames and returned path errors;
it was rerun with the actual filenames.

One later stale-range search included shell backticks in the pattern and printed
shell warnings before the corrected search. No files were changed by that
mistake.

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py` passed:
  documentation scaffold complete, 1356 numbered reports found before this
  report was added.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 37 tests OK.
- `python3 scripts/check_source_hierarchy.py --format json` passed:
  status `ok`, required `661`, present `661`, missing `0`, repo root `.`.
- `git diff --check` passed.
- ChatGPT Pro Oracle session `we-are-working-in-home` completed with Pro
  Extended selected and recommended a LAB-only "static source-first consequence
  map" rather than an executable/runtime slice.
- Sub-agent `019f2c4b-38ad-7a72-b1b2-e2afab876fd5` recommended the same narrow
  slice and identified `plan/90-source-traceability.md`, root snapshots, and
  validator registration as update points.

## What changed in understanding

The next safe follow-through is not the broad `plan/69` minimal slice. After
`plan/119` and `plan/120`, the current slice must be narrowed to ordinary
assignment static consequences only.

The current overall plan remains in Macro 0 / T0-G0 rebaseline, while the
new candidate gives a bridge into later G1 statement/evidence refinement.

## Open questions

- Should the next package refine `THM001StatementDraft.lean`, or first write a
  narrow SCN exact static slice manifest?
- Does OBL-001 need a more explicit abstract predicate for visible-write
  publish / observe consequences?
- Should a later canon proposal introduce a non-domain mental model for
  ordinary assignment before T1?

## Suggested next prompt

Use `plan/121` to choose the next G1 follow-through package: either a narrow
SCN exact static slice manifest or OBL-001 predicate refinement. Keep it
LAB-only unless a canon proposal is explicitly requested.

## Plan update status

`plan/` 更新済み:

- Added `plan/121-g1-minimal-vertical-slice-candidate-map.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added `plan/120` / `plan/121` to the current Surface/G1 LAB-memory summary
  without changing any completion claim.

## progress.md update status

`progress.md` 更新済み:

- Updated timestamp to `2026-07-04 17:44 JST`.
- Added the current G1 minimal vertical slice note.
- Added a recent log entry for this package.

## tasks.md update status

`tasks.md` 更新済み:

- Updated timestamp to `2026-07-04 17:44 JST`.
- Added the `plan/121` current holding-state note.
- Added `G1-MVS static slice follow-through` to candidate next strategy
  packages.
- Updated validator/scaffold range wording to `plan/00..121` /
  `plan/39..121`.

## samples_progress.md update status

`samples_progress.md` 更新不要:

- No runnable sample status, workflow readiness, command, sample path, debug
  surface, or blocker changed.

## Reviewer findings and follow-up

Reviewer sub-agent `019f2c56-516e-7fa0-82a2-0d1c766c1914` completed with no
findings. It confirmed that `plan/121` remains LAB-only, that root snapshots,
`plan/00-index.md`, `plan/90-source-traceability.md`, scripts docs, and
validator/test registrations are synchronized, and that the older
`plan/118..119` wording it found is historical `progress.md` log text rather
than current status.

## Skipped validations and reasons

No implementation, sample helper, Cargo, or Lean files were changed. Therefore
Cargo tests, sample helper execution, and Lean compilation were not run for
this docs-only planning package.

## Commit / push status

Primary package commit pushed:

- `b02820d1284e9da119703930e06fc82a9a804c52`
  (`Add G1 minimal vertical slice map`)

After push, `HEAD` and `origin/main` both resolved to
`b02820d1284e9da119703930e06fc82a9a804c52`.

This report-status update will be committed and pushed separately as a
closeout-only follow-up.

## Sub-agent session close status

- Code-mapping sub-agent `019f2c4b-38ad-7a72-b1b2-e2afab876fd5` completed and
  was closed.
- Reviewer sub-agent `019f2c56-516e-7fa0-82a2-0d1c766c1914` completed and was
  closed.
- Oracle session `we-are-working-in-home` completed; no external output is
  normative unless mirrored into repo files.
