# Report 2278 - Remaining-ledger research closure map

## Objective

Map the open metatheory ledger after T-RESEARCH-024 so that autonomous work
selects only independent source-grounded investigations and does not relabel a
missing parent interface as a new proof task.

## Scope and assumptions

Canon remains normative. This is a LAB management and source-dependency map,
not a theorem proof, a status proposal, a statement selection, or an
implementation plan. Existing LAB Lean statement drafts are evidence of
expressibility only and remain unpromoted.

## Start state / dirty state

The worktree was clean at `15d71f8e`. T-RESEARCH-025 recorded its Discord task
baseline before ledger reading. No tracked source had changed before this map.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, theory/10, and theory/11
- Canon plan/00--02 and architecture/02
- LAB `plan/130`, `plan/149`, `plan/156`, `plan/109`, `plan/87`,
  `.docs/progress-task-axes.md`, `tasks.md`, `progress.md`, and
  `docs/project-status.md`
- Existing OBL-024/025 LAB statement drafts and their surrounding plans/reports
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared every ledger obligation against the completed T-RESEARCH source
  cuts, the direct kernels, and the current LAB-only OBL-024/025 statement
  drafts.
- Separated parent statement boundaries from dependent proof/lemma work. OBL-002,
  OBL-008, and OBL-016 require their parent statements; OBL-011..013 require
  the Load/restored-state/live-after-load bridge that T-RESEARCH-014 found
  missing.
- Retained the bounded direct-kernel classification for OBL-003, OBL-004,
  OBL-010, and OBL-018. None has a complete checker, corollary, or proof
  interface.
- Retained OBL-019 as unselected because its E-PATCH transition/frame gap is
  explicitly overlapping T-RESEARCH-006 rather than an independent source cut.
- Identified theory/10 OBL-024/025 as the remaining independent family:
  canonical target directions exist, but their existing LAB drafts do not
  establish a canon statement, proof, or final diagnostic/repair ABI.
- Did not retry Oracle: the concrete pre-submit browser model-picker failure
  remains unchanged. The repository operating note now defaults new questions
  to temporary chats; no external advice was needed for this local inventory.

## Files changed

- `docs/reports/2278-remaining-ledger-research-closure-map.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- focused ledger and source searches with `rg`, `sed`, and `find`
- `df -h .` and `free -h` before broad validation
- `python3 scripts/current_l2_lean_sample_sync.py`
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Parent-boundary class: OBL-001/020/021, OBL-005..007, OBL-009, OBL-014,
  OBL-015, OBL-017, OBL-022, OBL-023, OBL-026..028 each retain their recorded
  source boundary or formalization stop. This map makes no new boundary claim.
- Dependent class: OBL-002/008/016 are proofs after their parent statements;
  OBL-011..013 are load-safety lemmas after OBL-009 supplies the relation on
  which "restored" and "live" are evaluated.
- Direct-fragment class: OBL-003/004/010/018 have bounded mechanical kernels
  only. They cannot be escalated to their ledger obligation without choosing
  their remaining full interfaces.
- Explicit-overlap class: OBL-019 remains tied to the E-PATCH history/frame
  gap already found across the OBL-020 source-adequacy matrix.
- Independent next family: theory/10's OBL-024/025. Their current LAB Lean
  drafts show non-final abstract statement shapes but do not supply canon
  diagnostics, replay, repair, or proof status.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.4 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `python3 scripts/current_l2_lean_sample_sync.py` completed successfully and
  reported the current Lean manifest without changing it. `make check` passed:
  source hierarchy `704 / 704`, documentation validation passed with 1,432
  numbered reports, and `cargo check` finished successfully. `python3 -m
  unittest scripts.tests.test_current_l2_lean_sample_sync` passed all 21 tests.
  `git diff --check` passed.

## What changed in understanding

The theory research stream is no longer best represented as a linear list of
all open ledger IDs. Most remaining entries are blocked by a small set of
named formalization interfaces. Treating their proofs or sublemmas as fresh
independent work would encourage accidental interface selection. The next
useful autonomous work is the diagnostics family, where canon gives a separate
target direction and the LAB evidence can be audited without promotion.

## Open questions

- Which canonical formalization process will choose the parent statement
  interfaces already isolated by the completed source audits?
- For OBL-024, what replay and diagnostic-to-rejection association relation is
  canonical rather than report-local LAB evidence?
- For OBL-025, what Line-1 coverage, single-edit semantics, and repair witness
  relation is canonical rather than a compile-check-only abstraction?
- When a parent statement is selected, which dependent proof/lemma should be
  reopened first and with what proof boundary?

## Suggested next prompt

Audit theory/10 explanation soundness and completeness as the next independent
source family, retaining the existing OBL-024/025 LAB drafts as non-normative
evidence only.

## Plan update status

Updated: plan/156 now classifies the remaining ledger into parent boundaries,
dependents, bounded kernels, overlap, and the next independent source family.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now gives the reason proof dependents and OBL-019 are
not selected, and names theory/10 as the next independent family.

## progress.md update status

Updated: the research summary and dated recent log include T-RESEARCH-025.

## tasks.md update status

Updated: the ordered packages and discovery rows distinguish dependency work
from the unselected theory/10 source cut.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested. The repeated browser
model-picker failure is concrete and unchanged, and this ledger inventory
relies on local canon/LAB source hierarchy. No local sub-agent service was
available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation-only dependency map. No individual theorem was rerun or
claimed; existing Lean draft synchronization is the relevant mechanization
check.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
