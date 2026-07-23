# Report 2400 - OBL-001 owner decision interface packet

- Date: 2026-07-23 21:26 JST
- Author / agent: Codex
- Scope: owner-facing OBL-001 Core/write statement-interface decision packet
- Decision levels touched: no settled semantic decision; new L3-open Canon proposal only

## Objective

Create the missing concise owner decision surface for the OBL-001 proof-facing
Core/write correspondence boundary without selecting a theorem interface or
promoting LAB evidence to Canon.

## Scope and assumptions

Canon remains normative. The proposal is a decision request, not an accepted
direction. It treats `theory/03` as the existing statement authority and
WRK-0007 plus LAB plans as statement-shape evidence only.

## Start state / dirty state

Started at pushed `1630dd32` with no tracked worktree diff and no local/remote
divergence. Ignored local webhook and helper state remains intentionally
preserved and is not part of evidence.

## Documents consulted

Read Canon README, MAP, theory/01, theory/03, theory/11, ADR-0014, the style
guide, CHANGELOG, PROPOSAL-003, PROPOSAL-008, and WRK-0007. Read LAB plan 180,
reports 2391 and 2399, and the current status snapshots. An independent planner
and reviewer checked non-duplication and statement-boundary risks. A temporary
external advisory review was compared with the local sources.

## Actions taken

1. Confirmed that no existing proposal covers OBL-001 Core/write statement
   interface selection.
2. Distinguished the owner choice of final statement boundary from proof-local
   implementation technique.
3. Added PROPOSAL-009 with direct-Core, explicit-bridge, and defer options,
   including coverage, static/runtime separation, independent totality, and
   stop conditions.
4. Updated the human control view and LAB snapshots to point to the new owner
   decision surface without claiming an answer.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-009-obl001-core-write-correspondence.md`
- `mirrorea_canon/INDEX.json`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- targeted Canon/LAB source reads and decision-artifact searches
- initial root-relative `python3 mirrorea_canon/meta/build-index.py` (rejected:
  Canon root not found; no file change)
- `cd mirrorea_canon && python3 meta/build-index.py`
- `cd mirrorea_canon && python3 meta/build-index.py --check`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `make check`
- `git diff --check`

## Evidence / outputs / test results

The existing Canon theorem already ranges over every write in Core `c`, while
WRK-0007 establishes only that the current experiment-local Result predicate
does not cover that domain. Planner, reviewer, and advisory review agreed that
the missing item is an owner decision packet, not a new toy proof model.
PROPOSAL-009 preserves direct-Core and bridge routes as statement alternatives,
requires an inspectable bridge if selected, and keeps runtime mutation,
OBL-021 outcome totality, and all ledger status outside the decision.

The first `make check` rejected only backtick-wrapped `更新済み:` text in this
report's project-status update section. The report was corrected to the exact
template form; the subsequent index, hierarchy, documentation, diff, and full
`make check` validations passed, including `cargo check`.

## What changed in understanding

Direct-Core and a bridge are not mutually exclusive proof techniques. The
actual owner choice is whether a later OBL-001 package states the existing
Core-write property directly or exposes an output-level view plus a sufficient
correspondence. This distinction prevents an opaque Result predicate from
silently weakening the Canon theorem.

## Open questions

- Does the owner accept A, B, or C in PROPOSAL-009?
- If B is selected, which later design package should define its exact
  proof-local carrier and correspondence relation?
- PROPOSAL-003, PROPOSAL-008, G0-D3, and LANE-CATALOG remain independent owner
  boundaries.

## Suggested next prompt

Review and record the A/B/C disposition in PROPOSAL-009. No proof-facing
OBL-001 package should begin from the current LAB Result predicate before that
decision.

## Plan update status

`plan/` 更新不要: plan 180 already preserves the detailed statement-identity
audit. Duplicating it would not improve repository memory.

## Documentation.md update status

`更新済み:` the concise reader map now identifies PROPOSAL-009 as the OBL-001
owner decision surface.

## docs/project-status.md update status

更新済み: the owner decision table now points to PROPOSAL-009 and its exact
non-semantic scope.

## progress.md update status

`更新済み:` the current logical boundary and recent log now record the new
decision surface without changing readiness or status.

## tasks.md update status

`更新済み:` the current task map now names PROPOSAL-009 rather than an
unmaterialized OBL-001 route choice.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, command, runnable workflow, or
evidence classification changed.

## Reviewer findings and follow-up

The planner found no duplicate packet and recommended a thin A/B/C owner
request. The reviewer required that the packet distinguish final statement
boundary from implementation technique; retain static-versus-runtime
separation; require bridge coverage, duplicate/extra-item handling, and all
THM-001 properties; and leave PROPOSAL-008 independent. Those requirements
were incorporated before validation. A re-review corrected the local/request
disjunction, defer wording, stale control-view sentence, and reproducible index
command; its final report wording check was also incorporated. The advisory
review independently agreed that no fourth option or preselected carrier is
appropriate.

## Skipped validations and reasons

No targeted Lean, runtime, or generated-artifact validation is required for
this documentation and decision-request package. The package changes no
executable source or proof artifact; `make check` nevertheless ran the
repository-required `cargo check` alongside documentation and hierarchy checks.

## Commit / push status

Pending at report write. This package will be committed with `--no-gpg-sign`
and pushed after validation and focused review.

## Sub-agent session close status

The planner and reviewer completed read-only work without edits. Their sessions
will be closed after the package review.
