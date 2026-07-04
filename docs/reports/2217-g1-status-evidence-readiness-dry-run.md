# 2217 - G1 status evidence readiness dry-run

## Objective

Record a LAB docs/validation-only dry-run for the OBL-001 / OBL-020 /
OBL-021 evidence checks that a future G1 status proposal packet would cite.

## Scope and assumptions

Scope:

- add `plan/132-g1-status-evidence-readiness-dry-run.md`;
- run direct Lean compile-checks for the three OBL-001 / OBL-020 / OBL-021
  LAB statement drafts;
- run the existing statement sync guard unit tests;
- scan the three drafts for admitted stubs or placeholder theorem bodies;
- synchronize root docs, repository-memory indexes, progress snapshots,
  runnable sample dashboard text, and validator scaffolds.

Assumptions:

- `mirrorea_canon/` remains the normative source;
- legacy `specs/`, `plan/`, samples, tests, reports, Rust code, and Lean drafts
  outside `mirrorea_canon/` remain LAB evidence / repository memory;
- this package may strengthen current LAB evidence readiness only.

Out of scope:

- canon edit;
- G0 exit;
- T0 -> T1 transition;
- G1 exit;
- G2..G7 exit;
- requested status selection;
- status proposal submission;
- metatheory ledger movement;
- OBL completion;
- proof skeleton completion;
- proof discharge;
- C-static / C-runtime / C-distributed conformance;
- executable row addition;
- Lean predicate refinement;
- runtime / transport / diagnostic / repair / API / grammar freeze;
- sample status relabel.

## Start state / dirty state

Start state:

- branch: `main`;
- start `HEAD`: `bfb7d22d Record G1 status outline commit`;
- `main` matched `origin/main`;
- worktree was clean before this package's edits.

During this package, only the P79 dry-run and documentation/scaffold files were
modified.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `samples/lean/lab-statements/README.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`

## Actions taken

- Added `plan/132-g1-status-evidence-readiness-dry-run.md`.
- Recorded the dry-run target, direct commands, results, evidence
  classification, remaining blockers, non-claims, and next allowed move.
- Registered `plan/132` in `plan/00-index.md`.
- Registered `plan/132` in `plan/90-source-traceability.md`.
- Registered `plan/132` in `scripts/validate_docs.py`,
  `scripts/check_source_hierarchy.py`, and
  `scripts/tests/test_validate_docs.py`.
- Updated `scripts/README.md` to reflect the current `plan/00..132` scaffold.
- Updated `README.md` and `Documentation.md` to include the new LAB dry-run
  memory while preserving non-claims.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` so the dry-run
  is no longer only a candidate and the next candidate moves to a
  requested-status options matrix.
- Created this report.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `docs/reports/2217-g1-status-evidence-readiness-dry-run.md`

## Commands run

Context and inspection:

```bash
git status --short --branch
rg -n "plan/132|G1 status evidence|evidence readiness|dry-run|dry run" README.md Documentation.md progress.md tasks.md samples_progress.md plan/00-index.md plan/90-source-traceability.md scripts/README.md scripts/validate_docs.py scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py
sed -n '1,220p' progress.md
sed -n '1,220p' samples_progress.md
sed -n '1,220p' tasks.md
sed -n '640,700p' tasks.md
sed -n '1,240p' plan/132-g1-status-evidence-readiness-dry-run.md
date '+%Y-%m-%d %H:%M %Z'
```

Evidence dry-run:

```bash
lean --version && elan --version && lake --version
lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
rg -n "^\s*(axiom|constant|theorem)\b|\bsorry\b|:=\s*(by\s+)?trivial\b|:=\s*(\(\s*)?True(\s*\))?\b" samples/lean/lab-statements/obl001/THM001StatementDraft.lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
```

Validation after edits:

```bash
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py --format json
git diff --check
```

Secret / endpoint checks:

```bash
git diff --cached --name-only
git diff --name-only
WEBHOOK_SECRET=<configured webhook URL> TOKEN_PREFIX=<configured token prefix> bash -lc '...scan changed tracked files and tracked repository for those literal secret values...'
```

Commit / push:

```bash
git status --short --branch
git add ...
git commit --no-gpg-sign -m "Add G1 status evidence readiness dry-run"
git push
git rev-parse HEAD
git rev-parse origin/main
```

## Evidence / outputs / test results

Dry-run evidence:

- Lean toolchain available: Lean 4.29.1, elan 4.2.3, Lake 5.0.0-src.
- `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean`: pass.
- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`: pass.
- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`:
  pass.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`: pass,
  21 tests.
- Targeted admitted-stub / placeholder scan over the three OBL drafts:
  no matches.

Post-edit validation evidence:

- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`: pass,
  21 tests.
- `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean`: pass.
- `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`: pass.
- `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`:
  pass.
- `python3 -m unittest scripts.tests.test_validate_docs`: pass, 37 tests.
- `python3 scripts/validate_docs.py`: pass, documentation scaffold complete,
  1369 numbered reports found.
- `python3 scripts/check_source_hierarchy.py --format json`: pass,
  `status: ok`, `required_count: 672`, `present_count: 672`,
  `missing_count: 0`.
- `git diff --check`: pass.
- Secret scan: no full Discord webhook URL or webhook token prefix found in
  changed or tracked files.

## What changed in understanding

The repository now has a narrower checkpoint between `plan/131` and any future
status proposal: current OBL-001 / OBL-020 / OBL-021 LAB statement drafts have
fresh compile-check, sync-guard, and no-admitted-stub evidence that a proposal
packet can cite.

This does not make the drafts accepted canon objects. The remaining hard choices
are still human/canon decisions: requested status vocabulary, ledger mapping,
LAB namespace vs canon-facing wrapper, OBL-020 scope, OBL-021 equality /
diagnostic abstraction, and OPEN-014 handling.

## Open questions

- Should OBL-001 / OBL-020 / OBL-021 later request `stated`, `lean-stated`, or
  another canon vocabulary?
- Does OBL-020 need a full statement target or a G1-supporting slice target for
  the first ledger movement?
- Does OBL-021 need an explicit canon-facing equality / diagnostic abstraction
  wrapper before status movement?
- Can OPEN-014 be deferred in the first G1 packet, or does it need a prior canon
  decision?
- Should future ledger-facing Lean artifacts stay under the current
  `MirCore.Lab...` namespace or receive canon-facing wrapper names?

## Suggested next prompt

Prepare a docs-only G1 requested-status options matrix for OBL-001 / OBL-020 /
OBL-021. Compare `stated` and `lean-stated` per OBL using `plan/130..132`, keep
the recommendation advisory, and do not edit canon or move the ledger.

## Plan update status

Updated.

- Added `plan/132-g1-status-evidence-readiness-dry-run.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

Updated.

`Documentation.md` now mentions the G1 status evidence readiness dry-run and
keeps canon edit, status movement, proof, conformance, G1 exit, and sample
status relabel out of scope.

## progress.md update status

Updated.

`progress.md` now records `plan/132`, updates the Macro 5 and LAB Lean
statement draft rows, and appends a timestamped recent-log entry.

## tasks.md update status

Updated.

`tasks.md` now moves the dry-run into current holding state and changes the next
candidate package to a docs-only requested-status options matrix.

## samples_progress.md update status

Updated.

`samples_progress.md` now records the Lean statement dry-run as evidence-only
compile-check support and adds a recent validation log entry without changing
workflow status.

## Reviewer findings and follow-up

No sub-agent or Oracle reviewer was used for this narrow dry-run package.
Follow-up is local validation plus diff inspection.

## Skipped validations and reasons

`python3 scripts/current_l2_lean_sample_sync.py` was not run because it rewrites
`samples/lean/manifest.json` and generated clean-near-end stubs. This package's
target was direct evidence readiness for the three OBL statement drafts and
their sync guards, not manifest refresh or generated-stub regeneration.

No broad Cargo or runtime sample suite was run because this package changed
documentation, validator scaffolds, and Lean statement evidence references only;
it did not change Rust code, executable samples, runtime behavior, or helper
logic.

## Commit / push status

Pending at initial report creation.

## Sub-agent session close status

No sub-agent session was opened for this narrow package.
