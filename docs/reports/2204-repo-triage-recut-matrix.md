# Report 2204 — Repo triage recut matrix

- Date: 2026-07-04 17:24 JST
- Author / agent: Codex
- Scope: Macro 0 maintenance / LAB evidence classification
- Decision levels touched: none; LAB repository memory only

## Objective

Classify existing Product Alpha, Full System V1, and Surface evidence for the
next theory / management recut without moving files, deleting archives, editing
canon, or promoting helper/sample success to implementation-state completion.

## Scope and assumptions

- Scope is docs-first classification only.
- The classification is a LAB management overlay over existing repo vocabulary:
  `workflow-ready`, `product-release-candidate`, `evidence-closed`,
  `boundary-fixed`, `planned`, `LAB-evidence-only`, and later-gate status.
- `archive-exploration` means retained exploration / inventory evidence, not an
  archive move.
- `postpone/drop-from-current-recut` means dropped from the immediate theory
  recut, not file deletion.
- `useful-floor` is added as a safer label for runnable compatibility /
  adoption / regression anchors that should not be called core semantics.
- No sample status, workflow status, semantics, ABI, proof status, or canon
  claim is intended to change.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `a0055666cdc53a8615d8f0b8209ee7d4a2ef813a`
- Start state: clean and matched `origin/main`.

## Documents consulted

- `AGENTS.md` instructions supplied in the task context
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/plan/01-phases.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/69-consultation-synthesis-and-management-roadmap.md`
- `plan/70-lab-to-canon-reconciliation-ledger.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/full-system-v1/README.md`
- `samples/full-system-v1-surface/README.md`
- `scripts/README.md`
- `docs/reports/2203-shared-practical-failure-path-redaction.md`

## Actions taken

- Started a Discord task baseline before the package.
- Used a code-mapper sub-agent for a read-only mapping of Product Alpha / Full
  System V1 / Surface triage surfaces and closed that sub-agent after use.
- Started an Oracle consult through ChatGPT 5.5 Pro Extended and waited for the
  result before closing the classification.
- Added `plan/120-repo-triage-recut-matrix.md` as LAB repository memory.
- Registered `plan/120` in `plan/00-index.md`,
  `scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, and
  `scripts/tests/test_validate_docs.py`.
- Updated `scripts/README.md` to mirror that current numbered plan scaffold now
  covers `plan/00..120`.
- Updated `progress.md` and `tasks.md` with the completed triage matrix and
  non-claim status.
- Left `Documentation.md` and `samples_progress.md` unchanged because no
  front-door reader summary, workflow status, sample status, or runnable command
  changed.

## Files changed

- `plan/120-repo-triage-recut-matrix.md`
- `plan/00-index.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2204-repo-triage-recut-matrix.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
ask-chatgpt-pro ... --file README.md --file Documentation.md --file progress.md --file tasks.md --file samples_progress.md --file mirrorea_canon/meta/source-hierarchy.md --file mirrorea_canon/plan/01-phases.md --file plan/69-consultation-synthesis-and-management-roadmap.md --file plan/70-lab-to-canon-reconciliation-ledger.md --file plan/119-g0-remaining-claim-family-drilldown-priority.md
python3 scripts/new_report.py --slug repo-triage-recut-matrix
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py --format json
git diff --check
```

Additional read / inspection commands used `sed`, `rg`, `git status`, and
`git diff --stat` over the files listed above.

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1356
  numbered reports found.
- `python3 -m unittest scripts.tests.test_validate_docs`: 37 tests passed.
- `python3 scripts/check_source_hierarchy.py --format json`: status `ok`,
  required 660, present 660, missing 0, repo root display `.`.
- `git diff --check`: passed.
- Oracle agreed the package is safe only as Macro 0 maintenance / reading aid,
  and recommended multi-axis classification with source authority, evidence
  maturity, reuse disposition, semantic stratum, gate/timing, non-claim guard,
  and validation anchor.
- Oracle warned against treating `archive` as a file operation, `drop` as
  deletion, `keep-core-idea` as canon adoption, or validation as semantic proof.
  `plan/120` was adjusted to include `useful-floor` for runnable evidence.
- The code-mapper sub-agent independently identified the same core risks:
  triage labels must be layered over existing repo vocabulary, helper success
  must not become status, domain vocabulary must not become core semantics, and
  archive language must not imply file movement.

## What changed in understanding

The safe output is not a judgment about which old work is "true." It is a
citation discipline: for each evidence family, record how it may be reused
without overclaiming. Product Alpha and release-check evidence are usually a
`useful-floor`; Surface elaboration and source-authority rows are immediate
`keep-core-idea` pressure; planned-only projection / engine inventory is mostly
`archive-exploration`; final public runtime / transport / ABI / distribution
claims remain `postpone/drop-from-current-recut`.

## Open questions

- Whether a later package should make the triage matrix machine-readable.
- Which `keep-core-idea` rows should seed the first minimal source-first
  vertical slice after G1 ordinary-assignment boundaries stabilize.
- Whether Product Alpha release-check evidence should remain only a
  compatibility / adoption anchor or later split into a non-alpha demo root.
- Which Product Alpha operational roots should remain examples if broader
  final catalog breadth is reopened.

## Suggested next prompt

Continue Macro 0 / G1 preparation by selecting either a narrow G1
ordinary-assignment proof-boundary refinement, a focused stale wording audit
triggered by touched docs, or a review of which `plan/120` keep-core rows should
feed the next minimal vertical slice.

## Plan update status

`plan/` 更新済み: added `plan/120-repo-triage-recut-matrix.md` and registered it
in `plan/00-index.md`.

## Documentation.md update status

`Documentation.md` 更新不要: the front-door reader snapshot already says Product
Alpha, Full System V1, and Surface are bounded / LAB evidence and not final
product, runtime, ABI, or proof status.

## progress.md update status

`progress.md` 更新済み: added the current repo-triage recut note and a 2026-07-04
17:24 JST recent log entry.

## tasks.md update status

`tasks.md` 更新済み: recorded `plan/120` in current holding state and changed the
candidate row into a follow-up / reserve package.

## samples_progress.md update status

`samples_progress.md` 更新不要: no workflow status, evidence class, sample path,
validation command, blocker, or runnable dashboard row changed.

## Reviewer findings and follow-up

- Code-mapper sub-agent `019f2c33-65d3-7353-abae-0183cf33f049` returned a
  read-only map and was closed. Findings were incorporated into `plan/120`.
- Oracle consult returned after about 8.5 minutes. Its main correction was to
  avoid using `keep-core-idea` for every runnable workflow; follow-up added
  `useful-floor` and adjusted Product Alpha / release-check / operational rows.
- Reviewer sub-agent `019f2c42-ff71-7d30-973b-ffdddffeceab` found no semantic
  or source-hierarchy blocker. It found that the report omitted
  `validate_docs.py` / `git diff --check` from command and evidence sections,
  and that `postpone/drop-from-current-recut` had a spelling drift in
  `plan/00-index.md` and `progress.md`. This follow-up fixed both issues.

## Skipped validations and reasons

- Heavy Product Alpha, Full System V1, and Surface release-check suites were not
  rerun before initial report write because this package changes only docs /
  scaffold registration and does not touch sample roots, helpers, matrices,
  expected JSON, Cargo code, or CLI behavior.
- `samples_progress.md` was not updated for the same reason.

## Commit / push status

Pending at report write.

## Sub-agent session close status

Code-mapper sub-agent `019f2c33-65d3-7353-abae-0183cf33f049` was closed after
its read-only report was incorporated.
