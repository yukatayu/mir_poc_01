# 2218 - G1 requested-status options matrix

## Objective

Create a LAB advisory matrix comparing `stated` and `lean-stated` as future
requested-status options for OBL-001 / OBL-020 / OBL-021, without submitting a
proposal or moving the canon metatheory ledger.

## Scope and assumptions

Scope:

- add `plan/133-g1-requested-status-options-matrix.md`;
- compare `stated` and `lean-stated` for OBL-001 / OBL-020 / OBL-021;
- use `plan/130..132` and canon status vocabulary as the immediate input;
- consult a sub-agent reviewer and ChatGPT Pro Oracle as advisory reviewers;
- synchronize root docs, `plan/` indexes, `progress.md`, `tasks.md`, and docs
  validator scaffolds.

Assumptions:

- `mirrorea_canon/` remains the normative source;
- `mirrorea_canon/theory/11-metatheory-ledger.md` remains the only proof/status
  authority;
- legacy `specs/`, `plan/`, samples, tests, reports, Rust code, and Lean drafts
  outside `mirrorea_canon/` remain LAB evidence / repository memory;
- advisory recommendations are not accepted status.

Out of scope:

- canon edit;
- G0 exit;
- T0 -> T1 transition;
- G1 exit;
- G2..G7 exit;
- requested status acceptance;
- status proposal submission;
- ledger movement;
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
- start `HEAD`: `9ba51399 Record G1 status evidence dry-run commit`;
- `main` matched `origin/main`;
- worktree was clean before this package's edits.

During this package, only P80 docs / scaffold / report files were modified.

## Documents consulted

- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/129-g1-acceptance-packet-preflight.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added `plan/133-g1-requested-status-options-matrix.md`.
- Compared `stated` and `lean-stated` for OBL-001 / OBL-020 / OBL-021.
- Recorded the advisory reading:
  - OBL-001: strongest later `lean-stated` candidate after artifact identity /
    wrapper acceptance and OPEN-014 deferral wording;
  - OBL-020: full-row status deferred; conditional `lean-stated` only after
    full-vs-G1-supporting scope acceptance;
  - OBL-021: conditional `lean-stated` if abstract result / diagnostic
    equivalence is accepted as the statement boundary; otherwise defer.
- Added hidden failure modes for status laundering, LAB namespace promotion,
  OBL-020 scope overclaim, OBL-021 equality freeze, weak `stated` fallback,
  paper/Lean divergence, OPEN-014 leakage, proof/conformance collapse, and G1
  exit by implication.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Registered `plan/133` in `scripts/check_source_hierarchy.py`,
  `scripts/validate_docs.py`, and `scripts/tests/test_validate_docs.py`.
- Updated `scripts/README.md` to reflect `plan/00..133`.
- Updated `README.md`, `Documentation.md`, `progress.md`, and `tasks.md`.
- Left `samples_progress.md` unchanged because no runnable sample, validation
  command, or sample workflow status changed in P80.
- Created this report.

## Files changed

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `docs/reports/2218-g1-requested-status-options-matrix.md`

## Commands run

Context / advisory setup:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
sed -n '1,260p' .docs/oracle-chatgpt-pro-operations.md
git status --short --branch
ask-chatgpt-pro -p "<advisory prompt>" --file mirrorea_canon/theory/11-metatheory-ledger.md --file mirrorea_canon/plan/01-phases.md --file plan/130-g1-obl-statement-status-completion-criteria-inventory.md --file plan/131-g1-status-proposal-packet-outline.md --file plan/132-g1-status-evidence-readiness-dry-run.md --file tasks.md
oracle status --hours 24 --limit 5
```

Local reading / inspection:

```bash
sed -n '1,260p' mirrorea_canon/theory/11-metatheory-ledger.md
sed -n '1,240p' mirrorea_canon/plan/01-phases.md
sed -n '1,260p' plan/130-g1-obl-statement-status-completion-criteria-inventory.md
sed -n '1,260p' plan/131-g1-status-proposal-packet-outline.md
sed -n '1,260p' plan/132-g1-status-evidence-readiness-dry-run.md
sed -n '1,240p' mirrorea_canon/plan/00-gates.md
sed -n '1,220p' plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md
sed -n '1,220p' plan/117-g1-obl001-020-021-statement-guard-hardening.md
sed -n '1,220p' plan/129-g1-acceptance-packet-preflight.md
sed -n '1,140p' mirrorea_canon/theory/03-elaboration.md
sed -n '1,120p' mirrorea_canon/theory/01-mircore-v0.md
sed -n '1,220p' samples/lean/lab-statements/obl001/THM001StatementDraft.lean
sed -n '1,220p' samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
sed -n '1,240p' samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
date '+%Y-%m-%d %H:%M %Z'
```

Post-edit validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py --format json
git diff --check
```

Secret / endpoint checks:

```bash
WEBHOOK_SECRET=<configured webhook URL> TOKEN_PREFIX=<configured token prefix> bash -lc '...scan changed tracked files and tracked repository for those literal secret values...'
```

Commit / push:

```bash
git status --short --branch
git add ...
git commit --no-gpg-sign -m "Add G1 requested status options matrix"
git push
git rev-parse HEAD
git rev-parse origin/main
```

## Evidence / outputs / test results

Advisory evidence:

- Sub-agent reviewer `019f2d08-f593-7ba0-b8fc-305170ae8516` completed and was
  closed. It recommended OBL-001 as the natural later `lean-stated` candidate,
  OBL-020 as defer, and OBL-021 as defer unless the abstraction boundary is
  accepted.
- Oracle session `we-need-an-advisory-review` completed in Pro Extended mode.
  It recommended OBL-001 advisory `lean-stated`, OBL-020 full-row defer with
  conditional `lean-stated` after scope acceptance, and OBL-021 conditional
  advisory `lean-stated` pending abstraction-boundary acceptance. It also
  warned against both premature `lean-stated` status laundering and all-`stated`
  underuse / paper-Lean divergence.

Post-edit validation evidence:

- `python3 -m unittest scripts.tests.test_validate_docs`: pass, 37 tests.
- `python3 scripts/validate_docs.py`: pass, documentation scaffold complete,
  1370 numbered reports found.
- `python3 scripts/check_source_hierarchy.py --format json`: pass,
  `status: ok`, `required_count: 673`, `present_count: 673`,
  `missing_count: 0`.
- `git diff --check`: pass.
- Secret scan: no full Discord webhook URL or webhook token prefix found in
  changed or tracked files.

## What changed in understanding

The requested-status matrix should not treat `stated` as automatically safer.
For OBL-001, `stated` underclaims the Lean-statement nature of the obligation.
For OBL-020 / OBL-021, the blocker is not lack of Lean syntax; it is human/canon
acceptance of scope and abstraction boundaries.

The best current advisory posture is asymmetric:

- OBL-001: later `lean-stated` candidate;
- OBL-020: full-row defer, conditional `lean-stated` after scope acceptance;
- OBL-021: conditional `lean-stated` after abstraction-boundary acceptance,
  otherwise defer.

## Open questions

- Should OBL-020 status movement target the full canon WF-preservation row or a
  G1-supporting statement scope first?
- Is the current abstract OBL-020 `WellFormed` / `Step` / `PreservesWF`
  vocabulary acceptable as a ledger-facing statement identity?
- Is the current OBL-021 abstract result / diagnostic equivalence boundary
  acceptable for ledger-facing statement identity?
- Should OBL-001 / OBL-020 / OBL-021 receive canon-facing wrapper names before
  any later proposal?
- How should a later proposal phrase OPEN-014 deferral so it is not read as
  runtime read-materialization policy?

## Suggested next prompt

Prepare a docs-only OBL-020 scope clarification packet. Decide whether a later
OBL-020 status proposal should target the full canon WF-preservation obligation
or a G1-supporting statement scope, without editing canon or moving the ledger.

## Plan update status

Updated.

- Added `plan/133-g1-requested-status-options-matrix.md`.
- Updated `plan/00-index.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

Updated.

`Documentation.md` now mentions the G1 requested-status options matrix and
preserves non-claims around canon edit, requested status acceptance, ledger
movement, proof, conformance, G1 exit, and sample status relabel.

## progress.md update status

Updated.

`progress.md` now records `plan/133`, updates the Macro 5 and LAB Lean
statement rows, and appends a timestamped recent-log entry.

## tasks.md update status

Updated.

`tasks.md` now records `plan/133` in current holding state and moves the next
candidate package to `OBL-020 scope clarification packet`.

## samples_progress.md update status

`samples_progress.md 更新不要`.

Reason: P80 changed advisory status-planning memory only. It did not change
runnable sample status, validation commands, Lean compile-check evidence, or
workflow readiness beyond the already recorded P79 evidence-readiness dry-run.

## Reviewer findings and follow-up

Sub-agent reviewer findings:

- OBL-001 should be the natural later `lean-stated` candidate, subject to
  artifact / wrapper acceptance.
- OBL-020 should defer because full-vs-G1-supporting scope is unresolved.
- OBL-021 should not freeze abstract equality / diagnostic boundaries.
- `stated` also needs exact mathematical statement identity, not an English
  summary.

Oracle findings:

- OBL-001 should be advisory `lean-stated`.
- OBL-020 should defer full-row status and keep conditional `lean-stated` only
  after scope acceptance.
- OBL-021 may be conditional advisory `lean-stated` if abstraction-boundary
  acceptance is explicit; otherwise defer.
- All-`stated` is not automatically safer because it can underuse Lean evidence
  and create paper/Lean divergence.

Follow-up:

- Incorporated the Oracle refinement into `plan/133` after the first local
  draft.
- Closed the sub-agent session.

## Skipped validations and reasons

No Lean compile-checks, sync guards, Cargo tests, or runtime sample commands
were run in P80 because this package did not change Lean source, Rust source,
runtime helper logic, executable samples, or sample status. P79 already recorded
the fresh OBL-001/020/021 Lean evidence-readiness dry-run that P80 cites.

## Commit / push status

Pending at initial report creation.

## Sub-agent session close status

Sub-agent reviewer `019f2d08-f593-7ba0-b8fc-305170ae8516` completed and was
closed.
