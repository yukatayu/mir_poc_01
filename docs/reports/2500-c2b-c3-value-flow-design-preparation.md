# Report 2500 - C2-B/C3 value-flow design preparation

**Identifier:** `LAB-REPORT-2500`
**Date:** 2026-07-28 16:28 JST
**Status:** design-preparation package prepared; commit/push pending

## Objective

Prepare the first non-normative C2-B/C3 design comparison after Plan 207 found no remaining autonomous L3 theorem candidate.

## Scope and assumptions

The packet uses the already recorded P012 V1/R1 and P013 M1 directions. It compares semantic presentations but selects no Core carrier, occurrence identity, request field, pending state, reply/receipt representation, runtime, grammar, wire protocol, OBL, Gate, Phase, or public behavior.

## Start state / dirty state

Start point was clean `main` at `4d699c407c9d5a51ecb455905906fab615d92db2`, pushed and equal to `origin/main`, after Plan 207 recorded the no-candidate disposition.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, P012, P013, and theory/01/03/04/05
- Plans 187, 192, 193, 199, 200, 207, and current status snapshots
- a temporary GPT-5.6 Sol Pro advisory review, distilled into Plan 208 and not treated as authority

## Actions taken

1. Defined a minimal C2B/C3-alpha trace: request, authoritative validation and reply/failure, receipt, zero-occurrence pure resume, then a later dependent occurrence.
2. Separated correlation, authority validation, result provenance, pending control, and linear held context into family-neutral comparison obligations.
3. Compared relation-first, request-occurrence anchored, and nominal-attempt presentations, including failure and save/load stopping conditions.
4. Recorded a two-layer recommendation: Family A as reference, Family B as the first instantiation, and Family C only after a concrete A/B failure.
5. Synchronized plan index, validation registries, reader guidance, project status, progress, and tasks.

## Files changed

- `plan/208-c2b-c3-value-flow-design-preparation.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Focused Canon/LAB source reads and relation-boundary searches
- Temporary Oracle review with ten attached Canon/LAB documents
- Canon index check, source hierarchy check, documentation validation, and `git diff --check`

## Evidence / outputs / test results

The advisory response digest is `e1fc575e8981ad53f1603ea27f44b095ce99d95c4959d7a6966d0a10d87b3ac4`. It supports the local conclusion that C2-B and C3 form one design cut, not two independent theorem lines. It recommends a relation-first reference and request-occurrence anchor comparison, retaining nominal attempt identity only as a bounded alternative. No executable artifact or semantic result was claimed.

## What changed in understanding

P012 V1/R1 and P013 M1 make the next question concrete without answering it: a successful reply must reach exactly one matching requester-side pending binding, while request-local claims remain non-authoritative and are checked against authoritative lineage. This establishes the minimum comparison obligations and makes clear which later inference would be justified only after a selected artifact retains the fact, grounds, full observation, and cumulative representation.

## Open questions

- What semantic foundation identifies a request/pending/reply/receipt exchange: relation, request occurrence, or nominal attempt?
- What equality and save/load stability apply to that foundation?
- Where are pending, result provenance, M1 context, held `Delta`, spent/failed state, and requester-side failure represented?
- Whether receipt and pure resume remain separate as recommended, and what freshness/revalidation rule applies before receipt/resume.

## Suggested next prompt

Review Plan 208 and choose whether the first Canon design package should adopt the relation-first plus request-occurrence anchored direction, retain nominal attempt as fallback, and keep receipt separate from zero-occurrence pure resume. The owner decision must also bound restore identity, pending/held-context ownership, freshness timing, and failure mapping.

## Plan update status

更新済み: Plan 208 is indexed and Plans 199/200 identify its non-normative comparison role.

## Documentation.md update status

更新済み: reader guidance links Plan 208 and states that it does not select a carrier or source syntax.

## docs/project-status.md update status

更新済み: semantic-kernel status and evidence references identify Plan 208 as preparation awaiting owner/Canon selection.

## progress.md update status

更新済み: logical status, blocker detail, research row, timestamp, and recent log distinguish prepared alternatives from adopted semantics.

## tasks.md update status

更新済み: current task map now identifies the C2-B/C3 owner/Canon selection needed before compatibility design proceeds.

## samples_progress.md update status

更新不要: no active sample root, runnable workflow, command, or debug surface changed.

## Reviewer findings and follow-up

The advisory review required explicit correlation rather than DAG ancestry alone, branch-scoped one-shot wording, a failure/no-resume boundary, and save/load frontier coverage. It rejected transport, queue, and payload equality as implicit identity. No callable sub-agent session was available.

## Skipped validations and reasons

No Lean, runtime, sample, transport, or end-to-end validation applies because the package intentionally creates no executable or selected semantic artifact. No Canon edit is attempted because the unresolved choices are reserved.

## Commit / push status

Pending design-preparation commit, push, fetch, and `HEAD == origin/main` verification.

## Sub-agent session close status

No callable sub-agent session was opened.
