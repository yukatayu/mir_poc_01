# Report 2251 -- P112 T0/G0 governance-profile adoption and evaluation

- Date: 2026-07-15
- Author / agent: Codex with advisory ChatGPT Pro Oracle review
- Scope: G0-D1/D2/D4 canonical transcription, one-off T0 profile evaluation,
  and LAB status synchronization. G0-D3 remains deferred.
- Decision levels touched: L1 through the owner-authorized ADR-0013 and
  `plan/01-phases` profile definition; no L0 change and no Gate/Phase exit.

## Objective

Apply the owner's accepted G0-D1, adopted T0 profile route, one-off artifact
route, D4 waiver, and D3 defer without creating a reusable tool or claiming
G0 exit/T1 entry.

## Scope and assumptions

The owner authorized the small protocol details needed to realize the accepted
route, provided they preserve the stated Mir/core and authority intent. Canon
remains normative; LAB files are evidence and operational memory. The profile
must be separate from SCN conformance and its artifact must be one-off.

## Start state / dirty state

Started clean on `6f96ce17` after P111. The canonical adoption was separately
committed and pushed as `edc2a36c`; the remaining worktree contains only this
package's one-off artifact, LAB memory/status updates, and this report.

## Documents consulted

- `mirrorea_canon/README.md`, `MAP.md`, `plan/00-gates.md`,
  `plan/01-phases.md`, `plan/02-operating-model.md`
- `meta/agent-instructions.md`, `meta/source-hierarchy.md`,
  `meta/style-guide.md`, `spec/06-conformance.md`, `arch/03-toolchain.md`
- `plan/153-g0-closeout-evidence-and-exit-decision-packet.md`, `plan/155`,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`
- `samples_progress.md`, report template, and the repo-local Oracle operations
  note

## Actions taken

1. Added ADR-0013, adopted PROPOSAL-002, defined the exact T0 profile, kept
   `spec/06-conformance` SCN-only, regenerated canon `INDEX.json`, and pushed
   the canonical adoption commit.
2. Incorporated Oracle review findings: phase-specific JSON semantics, direct
   adoption-parent revision binding, exact object shape/order, RFC 8785 digest,
   Git-blob hash sources, and byte-pinned LAB controls.
3. Ran one ephemeral evaluation to generate the single derived JSON artifact.
4. Verified the artifact against its exact field, order, Git-blob hash,
   repeated-evidence, derived-result, non-claim, and digest rules.
5. Updated LAB packet/routing memory and reader-facing status snapshots.

## Files changed

- `mirrorea_canon/` adoption files, including `adr/ADR-0013.md`,
  `plan/01-phases.md`, `spec/06-conformance.md`, and `INDEX.json`
- `plan/153-g0-closeout-evidence-and-exit-decision-packet.md`
- `plan/155-t0-g0-governance-profile-proposal.md`
- `plan/155-t0-g0-governance-profile-evaluation.json`
- `plan/00-index.md`, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`
- `docs/project-status.md`
- this report

## Commands run

- Canon index build/check, source-hierarchy guard, documentation validator,
  focused highlighter tests, and `make check`.
- Git-blob SHA-256 checks for all accepted-cut evidence and five LAB controls.
- One ephemeral Python evaluation generator and a separate ephemeral Python
  artifact validator.
- `git diff --check`, staged-diff inspection, commit/push, and clean-state
  checks for the canonical adoption commit.

## Evidence / outputs / test results

- Canon adoption commit: `edc2a36ce5145a897bf0e7a0e8788cf69216b2c3`.
- One-off profile result: all three checks `pass`; root result `pass`.
- Artifact digest: `b7a7c98cab610bf242b3e868dfe47e52953c19332ba82ee7fc24b9c473436cd4`.
- `meta/build-index.py --check`: `ok: 72 files indexed`.
- Source-hierarchy guard: 703/703 required paths present.
- Focused highlighter tests: 6 tests passed.
- Documentation scaffold validator: 1,405 numbered reports after this report.
- `make check`: Cargo check passed.

## What changed in understanding

A profile `pass` is usable only when its evidence protocol is reproducible.
The evaluation must bind to the immediately preceding canonical adoption commit,
not to the commit that records the artifact. That distinction prevents both
self-referential commit identities and accidental conversion of a profile result
into Gate authority.

## Open questions

G0-D3 remains explicitly deferred. A later owner decision must either continue
that defer or accept this exact artifact digest and identify the canonical G0
exit record. A full semantic/historical LAB-demotion audit remains outside the
waived checkpoint scope unless concrete drift appears.

## Suggested next prompt

Decide whether to continue deferring G0 exit or to accept the pinned P112
artifact digest and create the separate canonical G0-exit record.

## Plan update status

`plan/` 更新済み: `plan/153`, `plan/155`, and `plan/00-index` now distinguish
adoption/evaluation from G0 exit and record the remaining G0-D3 stop line.

## Documentation.md update status

`Documentation.md` 更新済み: current-reader summary now states adopted profile,
one-off `pass`, and G0-D3 defer without a conformance or exit claim.

## docs/project-status.md update status

更新済み: G0-D1/D2/D4, the evaluation artifact, and G0-D3 defer are reflected
in the concise owner-facing status table.

## progress.md update status

`progress.md` 更新済み: current milestone and dated recent log reflect P112 and
retain T0/G0 non-exit status.

## tasks.md update status

`tasks.md` 更新済み: P112 is closed; no autonomous successor is promoted while
G0-D3 is deferred.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or sample blocker changed in this documentation/governance package.

## Reviewer findings and follow-up

The first Oracle review found eight blockers in the initial canonical draft;
the material findings were incorporated before `edc2a36c`. A browser follow-up
failed before returning. The allowed independent retry then found six remaining
precision issues: adoption-parent binding, concrete object/cardinality shape,
hash equations, tri-state classification, and byte-pinned LAB controls. Those
were incorporated and locally checked. Oracle advice remained advisory; the
owner's recorded decisions are the authority.

## Skipped validations and reasons

No full runnable sample/product suite was rerun because this package changes no
executable source, runner, sample, or build configuration; `make check` and the
affected documentation/source-hierarchy/highlighter validations were run.
No reusable JSON validator was committed because the T1 moratorium permits only
the authorized one-off evaluation route.

## Commit / push status

Canonical adoption committed and pushed: `edc2a36c` (`Adopt T0 governance
profile`). The artifact/LAB synchronization commit is pending final validation
at report write.

## Sub-agent session close status

No local coding sub-agent was available. ChatGPT Pro Oracle completed the first
review and one independent retry; its browser follow-up ended without output.
No Oracle session remains required for this package.
