# Report 2446 — WRK-0025 command falsifier

- Date: 2026-07-28 05:52 JST
- Author / agent: Codex
- Scope: Record the first registered WRK-0025 falsifier and freeze the L3
  record without drawing a C0 semantic conclusion.
- Decision levels touched: L3 reliance status only. No Canon semantics changed.

## Objective

Preserve the failed registered command exactly enough to prevent a repaired
rerun from being mistaken for the original C0 inventory.

## Scope and assumptions

The failure concerns a token precondition in the registration command, not
whether any Surface form has a Core/Diagnostic outcome. No inventory row is
accepted as evidence.

## Start state / dirty state

Started from clean, pushed WRK-0025 registration `1bbd8529442a1ecd0188f8030c540d3332cd4427`.
The registered `rg` source audit ran; its following Python assertion failed.

## Documents consulted

- WRK-0025, ADR-0014, working annex rules, spec/02, P004, P008, P015, Plan 199.

## Actions taken

1. Stopped the command chain at its assertion failure.
2. Immediately changed only WRK-0025's reliance status to `frozen` and recorded
   the exact cause; regenerated Canon index metadata.
3. Committed/pushed the freeze before preparing this LAB report and snapshots.

## Files changed

- `mirrorea_canon/working/WRK-0025-surface-totality-domain-inventory.md`
- `mirrorea_canon/INDEX.json`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2446-wrk0025-command-falsifier.md`

## Commands run

- Registered `rg` source audit and required-token Python assertion.
- `git rev-parse`, Canon index generation/check, `git diff --check`,
  document/source-hierarchy validation.

## Evidence / outputs / test results

- The Python assertion required `CallArgs` in `spec/02-surface-grammar.md`.
- The displayed grammar uses prose `postfix (index, field, call)` and does not
  define `CallArgs`; P004's candidate EBNF does define that nonterminal.
- The assertion raised `AssertionError`. This is the first registered
  falsifier. The frozen record provides no C0 inventory evidence.

## What changed in understanding

The distinction between displayed grammar and a prospective P004 candidate
grammar must itself be explicit in C0 evidence. A successor audit must check
each source independently and pre-register its mapping method; it cannot reuse
or repair WRK-0025.

## Open questions

- C0 exact-domain and total-diagnostic inventory remains open.
- C2 request/replay and C6 scalar terminal are unaffected and remain available
  for separate pre-registration.

## Suggested next prompt

Proceed with C2 or C6 while preparing a separately pre-registered C0 successor
whose displayed-vs-candidate grammar inventories are distinct.

## Plan update status

更新済み: Plan 199 distinguishes the frozen command record from a C0 result.

## Documentation.md update status

更新不要: reader orientation has not changed.

## docs/project-status.md update status

更新不要: project-level semantic status has not changed; no C0 result exists.

## progress.md update status

更新済み: the current blocker and recent log distinguish the frozen command
from a semantic conclusion.

## tasks.md update status

更新済み: current work now requires a C0 successor rather than repair of
WRK-0025, while C2/C6 can proceed.

## samples_progress.md update status

更新不要: no sample/workflow changed.

## Reviewer findings and follow-up

No additional Oracle consultation was needed for this procedural falsifier.
The relevant action follows ADR-0014/working annex freeze rules directly.

## Skipped validations and reasons

No further source inventory or executable validation was run: the frozen
pre-registration forbids repairing or rerunning its command. No runtime/sample
artifact changed.

## Commit / push status

The immediate freeze was committed/pushed as `a5400a494156d0a038fde359684b39166fdef062`.
This report/snapshot package is pending commit/push at report write.

## Sub-agent session close status

No callable sub-agent session was available. No advisory review is needed to
classify a registered-command falsifier.
