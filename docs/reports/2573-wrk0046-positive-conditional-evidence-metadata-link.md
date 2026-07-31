# Report 2573 — WRK-0046 positive conditional evidence metadata link

- Date: 2026-08-01
- Author / agent: Codex
- Scope: Link the already pushed WRK-0046 source/evidence into append-only
  Canon Results, MAP, and generated index metadata without rewriting the L3
  preregistration or promoting its result.
- Decision levels touched: L3 evidence metadata only; no L0/L1/L2 decision,
  theorem/OBL, Gate, Phase, implementation contract, or public claim changes.

## Objective

Make the exact bounded positive evidence from the immutable source/evidence
commit durable in WRK-0046 while retaining `L3-open` and `not-promoted`.

## Scope and assumptions

The evidence commit `7e4b01eb6bc431be044a6343ec686a3b8d7d2a96` changed exactly
the declared `plan/` source and its direct report. The source artifact SHA-256
is `37753dbde1290c0b5e1602a60e1159a830cc31cc9fb99c9e4ff6f34e64eab7c1`;
the direct report SHA-256 is
`2005bfe0c4aabbeabd98369f01e17992503bfde2ae0bbe76df5e2595e267f57d`.
The embedded sole Lean block has SHA-256
`07538caaf5e1c369e4baf1a1f3b3dac1a957b8f3947998c6bb2d2304d3349efd`.

This package changes only the append-only Results/reliance metadata, MAP,
generated index, and this report. It does not revise the question, status quo,
candidate ledger, alternative, falsifiers, rollback rule, method, non-claims,
or reliance ceiling.

## Start state / dirty state

`HEAD` and `origin/main` were equal and clean at
`7e4b01eb6bc431be044a6343ec686a3b8d7d2a96`. The immutable evidence source
had passed `lean --trust=0`, 53 axiom checks, static boundary scans, source
hierarchy `795/795`, and documentation validation. WRK-0046 was still
`L3-open`, `not-promoted`, and unlinked in its Results section; MAP still
called it `unexecuted`.

## Documents consulted

- Canon: `mirrorea_canon/README.md`, `MAP.md`, ADR-0014, P012, P013, P017,
  theory/01, theory/02, theory/04, theory/05, theory/07, `working/README.md`,
  and WRK-0046.
- LAB: the immutable source and Report 2572, Plans 230, 231, and 245, the
  WRK-0045 evidence-link precedent, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Operations: Canon index generator, source-hierarchy and documentation
  validators, report template, and repository-local planner operating rules.

## Actions taken

1. Verified that the evidence commit contains exactly its declared source and
   direct report, and recomputed both immutable artifact digests.
2. Freshly extracted the sole Lean block, re-ran Lean 4.29.1 with `--trust=0`,
   confirmed all 53 `#print axioms` declarations are axiom-free, and re-ran the
   prohibited-surface scan.
3. Obtained a Canon-first planner review. It fixed the exact four-file
   allowlist, required byte stability outside Results, and confirmed that
   `not-promoted` must remain after linkage.
4. Replaced only WRK-0046 Results evidence fields, updated its MAP row to
   remove `unexecuted`, and regenerated `INDEX.json`.

## Files changed

- `mirrorea_canon/working/WRK-0046-p017-x1-k0-qf-ul-lift.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2573-wrk0046-positive-conditional-evidence-metadata-link.md`

## Commands run

- Evidence commit two-path allowlist, local/remote-head, and immutable SHA-256
  checks.
- Fresh source extraction, `lean --trust=0`, 53-declaration axiom check, and
  static prohibited-surface scan.
- Canon index generation and check, source hierarchy, and diff checks.
- After the local commit, run documentation and authoritative-working-annex
  validation from a clean worktree; then push and verify the remote head.

## Evidence / outputs / test results

The exact evidence commit and artifact digests above identify the source and
direct report unambiguously. The source retains one 434-line Lean block; it
passes under `--trust=0`, and every printed declaration has no axiom dependency.
Its closed A0 fixture inhabits the registered premises and its A1 omission
control produces the required two-consume trace without `PreservesSpentAt`.

The result is positive conditional evidence only: for one supplied finite
experimental line, ordinary-edge preservation and supplied restore-edge
preservation exclude two counted consumes. It does not establish a P017 model,
actual restore semantics, every restored continuation, global exactly-once, or
the final semantic residence of `Spent`.

## What changed in understanding

The source evidence is now durable and attributable without strengthening its
claim. The A1 control demonstrates why local restore preservation is
load-bearing for this candidate-local line, but it is not an A0 falsifier and
does not determine the eventual representation of the fact.

## Open questions

`Spent` remains OPEN as primitive-versus-uniquely-derived and as an eventual
semantic residence. Actual admissible-load closure, no-merge/no-duplicate,
receipt/matching/authority/failure semantics, persistence, runtime behavior,
and all Gate/Phase or public consequences remain outside this evidence.

## Suggested next prompt

Refresh reader/status snapshots in a separate package so that they cite the
durable non-promoted evidence link. Then perform a Canon-first frontier
re-screen; open no subsequent research package without a fresh eligible
source, consumer, and falsifier.

## Plan update status

`plan/` 更新不要: the immutable source was already created in evidence commit
`7e4b01eb`; this metadata package only references it.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing index wording is synchronized only
after this durable Canon metadata link, in a separate snapshot package.

## docs/project-status.md update status

更新不要: the compact reader view is deliberately updated only after this
Canon evidence link becomes durable.

## progress.md update status

`progress.md` 更新不要: the following reader snapshot records the durable
non-promoted evidence and its bounded consequence.

## tasks.md update status

`tasks.md` 更新不要: no next research task is selected before reader snapshots
are synchronized with the durable result.

## samples_progress.md update status

`samples_progress.md` 更新不要: this L3 source evidence changes no runnable Mir
sample, runner, debug surface, or sample dashboard row.

## Reviewer findings and follow-up

The Canon-first planner approved the metadata-link shape after requiring that
the immutable source digest and embedded Lean digest remain distinct, that
`not-promoted` remain unchanged, and that status snapshots wait for the next
package. Its final review found and corrected three report-only issues: no
pre-commit command is reported as completed, no unrecorded build claim remains,
and the next research package is not preselected. The corrected four-file diff
was approved with no residual issue.

## Skipped validations and reasons

No parser, runtime, transport, or sample command applies because no
implementation artifact changes. Reader-status validation is intentionally
deferred until the next package after this metadata link is durable.

## Commit / push status

Pending at report write. This metadata package will be committed, validated
from a clean worktree, pushed, and verified against `origin/main` before any
reader snapshot is changed.

## Sub-agent session close status

Planner `Locke` completed the pre-edit and final focused reviews, then was
closed after approval. No other sub-agent session is open for this package.
