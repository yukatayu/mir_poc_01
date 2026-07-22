# Report 2366 - WRK-0015 stale-grant-fence selection

- Date: 2026-07-22 20:23 JST
- Author / agent: Codex
- Scope: select a bounded P-SURF-05 source-local L3 candidate; no registration or outcome
- Decision levels touched: none; LAB candidate selection only

## Objective

Determine whether the untested second-admission branch after `StaleGrantFence`
is a distinct, standing-eligible ADR-0014 research candidate.

## Scope and assumptions

The candidate is a source-local checker experiment only. Canon authority,
revocation, epoch/incarnation, rejoin, OBL-028, contracts, and lifecycle state
are excluded. The transformed input must remain disposable.

## Start state / dirty state

`main...origin/main` was clean at `02761382`. The completed remaining-ledger
screen explicitly kept this behavior as a possible operational reserve, not a
theory bridge.

## Documents consulted

Read ADR-0014, working-annex rules, Canon authority/runtime texts, the
P-SURF-05 role-admission README and matrix, role-admission checker/test source,
the current snapshots, and the preceding remaining-ledger revalidation memo.

## Actions taken

Inspected the existing source/test route and obtained an independent read-only
eligibility screen. Compared the source-local experiment with an OBL-028
bridge, a runtime repair, and no further work. Selected only the disposable
source-local experiment for a future L3 pre-registration.

## Files changed

- `plan/wrk-0015-stale-grant-fence-selection.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- targeted `rg` source/test/plan screens
- `git` clean/upstream checks and source hashing
- `df -h`, `free -h`, and `du` resource audit before any future Cargo command
- independent source/eligibility screen, including one preliminary disposable
  Cargo command whose output is excluded from WRK evidence

## Evidence / outputs / test results

Static source reading shows that `StaleGrantFence` is keyed by principal and
target, stale-message handling inserts it, later report-level admission does
not statically remove it, and the write check reads it. Existing tests do not
cover a second admission before the write. The independent screen reproduced
the disposable command before registration. Its output is excluded and is not
relied on by this selection or any future WRK record. Only a fresh
post-registration command may become WRK evidence.

## What changed in understanding

The remaining-ledger no-candidate result was correctly scoped. A distinct
existing-lane operational experiment remains available without turning the
checker into a Canon theorem model.

## Open questions

- Does the post-registration command retain the stale fence after a second
  admission?
- Can direct report evidence be retained without a new fixture/helper or
  validation-surface change?

## Suggested next prompt

Register the selected source-local experiment as a reversible ADR-0014 L3
record, then execute only its committed disposable-source command.

## Plan update status

`plan/` 更新済み: the selection memo records scope, alternatives, eligibility,
branches, and the registration boundary.

## Documentation.md update status

更新済み: the research-reading map now links the selected, not-yet-registered
source-local candidate.

## docs/project-status.md update status

更新済み: the reader view distinguishes the selected operational candidate
from a registered WRK or Canon authority conclusion.

## progress.md update status

更新済み: the logical/macro/task snapshots record selection only, with no
registered post-registration execution/evidence, lifecycle, or proof movement.

## tasks.md update status

更新済み: task 39 records selection and makes registration the next bounded
package.

## samples_progress.md update status

`samples_progress.md` 更新不要: no committed sample, runnable workflow,
validation command, debug surface, or dashboard row changed.

## Reviewer findings and follow-up

The independent screen found the source-local branch distinct from the current
test but rejected every OBL-028 or repair interpretation. Final review found
that its preliminary command was described inconsistently with the snapshots;
it is now explicitly excluded, and a narrow re-review found one remaining
generic "no execution" phrase, also corrected to the registered
post-registration boundary. The next package must pre-register the exact
command and falsifier before a fresh evidence run.

## Skipped validations and reasons

The independent screen did run one preliminary disposable Cargo command before
registration. It is explicitly excluded from WRK evidence, so a fresh
post-registration command remains mandatory. No broad suite was run.
Documentation and metadata validation run after the selection-document edits.

## Commit / push status

Pending at report write.

## Sub-agent session close status

The source/eligibility explorer completed without edits and was closed.
