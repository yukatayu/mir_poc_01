# 2441 - Fixed-control drift scoped audit

- Date: 2026-07-28
- Author / agent: Codex
- Scope: Audit the four fixed controls that made the sole T0/G0 v2 artifact a
  valid `fail`; do not re-pin, regenerate, or reinterpret the artifact.
- Decision levels touched: none. The audit is LAB evidence and current-view
  synchronization only.

## Objective

Determine whether the fixed-control drift is a substantive change to the
accepted G0 evidence or a bounded documentation/governance change, before any
future owner decision could consider a normal Canon rebase proposal.

## Scope and assumptions

- `mirrorea_canon/` remains normative. `plan/198` and this report are LAB
  evidence; neither changes the T0/G0 profile result.
- The owner's acceptance of the recommended first action authorizes this
  scoped audit only. It does not authorize re-pinning, a second artifact, G0-D3
  acceptance, G0 exit, T1 entry, or I1 authorization.
- The comparison is from accepted evidence cut
  `6f96ce17e74173ca5d86ed76cee3db75d60dcbfe` to the v2 source cut
  `0ee3fdec553de31252a37478fc4a31f507221258` for exactly the four failed
  controls.

## Start state / dirty state

The task began clean and synchronized at `61e912a2`. The v2 artifact remained
the only authorized artifact and reported `pass` / `fail` / `pass`, with root
result `fail`. Disk had 61 GiB available; memory had 7.0 GiB available.

## Documents consulted

- Canon entry/process: `CANON.md`, `mirrorea_canon/README.md`,
  `mirrorea_canon/MAP.md`, ADR-0013, ADR-0014, `plan/01-phases`, and
  `meta/source-hierarchy`.
- v2 decision/evidence: PROPOSAL-014, `plan/198`, and
  `plan/198-t0-g0-governance-profile-v2-evaluation.json`.
- Current LAB views: `Documentation.md`, `docs/project-status.md`,
  `progress.md`, and `tasks.md`.

## Actions taken

1. Compared all four current-control blobs with their accepted-cut versions
   and listed their intervening commits.
2. Classified each diff by whether it changes source hierarchy, project
   semantics, scenario expectation, proof ledger, Gate, or Phase.
3. Confirmed that the changes clarify ADR-0014 research governance or reader/
   agent guidance and do not change MirCore, SCN, OBL, Gate, or Phase meaning.
4. Recorded the audit result in `plan/198` and synchronized current LAB views.

## Files changed

- `plan/198-t0-g0-governance-profile-v2.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2441-fixed-control-drift-scoped-audit.md`

## Commands run

- Discord task baseline.
- `df -h .`, `free -h`, `du -sh .`, `git status --short`, and Git history
  checks.
- `git diff`, `git log`, `jq`, and `rg` over the pinned/current controls and
  v2 artifact.
- `git diff --check` was attempted once with an invalid revision-range form;
  it produced no evidence and was not treated as a successful validation.
- A subsequent working-tree `git diff --check`, `python3
  scripts/validate_docs.py`, and `python3 scripts/check_source_hierarchy.py`
  completed successfully.

## Evidence / outputs / test results

The artifact's failed check names exactly four mismatches:
`mirrorea_canon/meta/source-hierarchy.md`, `CANON.md`, `README.md`, and
`AGENTS.md`. Their deltas are limited to bounded research-governance wording,
entry-point instructions, and LAB reader/agent operational guidance.

No comparison found a change to the accepted substantive ADR blobs, GLOSSARY,
LAB demotion evidence, MirCore, an SCN, an OBL, a Gate, or a Phase. The audit
therefore supports the description **governance/readability drift**, not a
semantic discrepancy. It does not alter the mechanically derived valid `fail`.
The final working-tree diff had no whitespace errors; document validation and
the source-hierarchy check passed (`748` required paths present, `0` missing).

## What changed in understanding

The fixed control set is stricter than a semantic-input set: it detects changes
to source-hierarchy and agent-facing anti-drift controls even when they improve
the clarity of the original policy. A future rebase is therefore a lifecycle
governance decision about what evidence cut to certify, not a repair to Mir
theory.

## Open questions

- Whether the owner wishes to retain the historical pins indefinitely or start
  a normal Canon rebase proposal for a future valid `pass` route.
- G0-D3 remains unavailable unless a separately authorized valid `pass` route
  and exact-digest acceptance exist.

## Suggested next prompt

Continue the accepted semantic decision integration: record the selected
proposal dispositions, then build the bounded shared-model and ergonomic
inference research package without changing a runtime or public contract.

## Plan update status

更新済み: `plan/198` に scoped audit の authority boundary、control-by-control
classification、non-effect を追記した。

## Documentation.md update status

更新済み: T0 row now distinguishes audited governance-only drift from an
authorized rebase/retry.

## docs/project-status.md update status

更新済み: the human control view now records the scoped audit and keeps normal
rebase as a separate owner/Canon action.

## progress.md update status

更新済み: current blockers and the dated recent log now record the audit result.

## tasks.md update status

更新済み: the task map marks the audit complete and retains only the
retain/defer versus normal-rebase owner decision.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or
sample-evidence classification changed.

## Reviewer findings and follow-up

The independent Oracle semantic-package review was started in parallel and was
still running when this audit report was drafted. Its result is not used here.
No controllable sub-agent tool was available in this session.

## Skipped validations and reasons

- No Lean, runtime, sample, or distributed command was run: this audit changes
  no executable/theory source in those lanes.
- No new v2 artifact was generated: ADR-0013 and PROPOSAL-014 permit exactly
  one, which has already been consumed.

## Commit / push status

Pending at report write. This package will be locally validated, committed with
`--no-gpg-sign`, pushed to `origin/main`, and checked for remote parity.

## Sub-agent session close status

No sub-agent session was opened. Oracle session
`theory-boundary-review-20260728` remains running asynchronously and is
advisory only.
