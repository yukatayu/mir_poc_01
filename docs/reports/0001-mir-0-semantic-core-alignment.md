# Report 0001 — mir 0 semantic core alignment

- Date: 2026-03-27T03:56:43.199923Z
- Author / agent: Codex
- Scope: Clarify the minimum semantic kernel of Mir-0, separate it from later Mir and Mirrorea concerns, and record the resulting decisions and unresolved points without moving into implementation work.
- Decision levels touched: L1 scoping clarification for Mir-0, with no intentional change to L0 graph / failure / ownership invariants and no change to subsystem separability.

## 1. Objective

Align the specification so that Mir-0 is a clearly bounded minimum semantic core rather than a mixed summary of both settled kernel semantics and later unresolved features.

## 2. Scope and assumptions

- Scope was limited to specification alignment and minimal document restructuring.
- Repository documents were treated as authoritative; prior conversation context was not used as a source of truth.
- Unresolved matters were left explicitly unresolved rather than silently decided.
- PrismCascade, Mirrorea, and the Typed-Effect Wiring Platform were consulted only far enough to keep boundaries consistent.
- No build-oriented work was assumed, because the repository explicitly warns that `cargo` may be unavailable in the current environment.

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- Boundary checks only:
  - `specs/05-mirrorea-fabric.md`
  - `specs/06-prismcascade-positioning.md`

## 4. Actions taken

1. Read the repository entry documents and normative spec chain in the required order to recover the current architecture and invariants.
2. Dispatched `code_mapper` to summarize repository structure and overlap zones around Mir semantics before editing normative files.
3. Reworked `specs/04-mir-core.md` so it now explicitly distinguishes broader Mir from the Mir-0 minimum semantic core.
4. Fixed the Mir-0 kernel in `specs/04-mir-core.md` to include:
   - event DAG / causality,
   - `place`,
   - `perform` / effect / contract,
   - the minimum structured failure space,
   - primitive fallback,
   - local `try` / rollback,
   - `atomic_cut`,
   - linear resources and monotone lifetime.
5. Moved later or unresolved topics out of the Mir-0 kernel in `specs/04-mir-core.md` and marked them explicitly as `UNRESOLVED` or `OPEN QUESTION`.
6. Followed up on reviewer feedback by:
   - weakening "source of truth" language so `specs/04` does not appear to supersede `specs/01` to `specs/03` or `specs/09`,
   - explicitly classifying `barrier` as deferred beyond Mir-0 instead of silently dropping it,
   - clarifying `atomic_cut` as a place-local Mir-0 cut primitive,
   - clarifying that `perform` is working notation for the minimum effect request operation rather than a silently fixed final keyword.
7. Adjusted `specs/09-invariants-and-constraints.md` so that:
   - `atomic_cut` is the minimum Mir-0 cut primitive,
   - any later cut vocabulary such as `durable_cut` still inherits the same no-hidden-backtracking discipline,
   - full fallback normalization is stated as an invariant for the later formalization, not as something already fixed inside Mir-0.
8. Updated `specs/10-open-questions.md` to enumerate the topics deferred beyond Mir-0, especially `barrier`, `durable_cut`, `emit`, coroutine semantics, overlay alias details, and the exact surface rules for `place`.
9. Added decision `D-011` to `specs/12-decision-register.md` to record the Mir-0 boundary set in this pass.
10. Created this new report under `docs/reports/`.
11. Kept the separate review artifact `docs/reports/0002-mir-0-review-findings.md` created during the reviewer pass so the review trail remains explicit.

Files changed:

- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0001-mir-0-semantic-core-alignment.md`
- `docs/reports/0002-mir-0-review-findings.md` (review artifact created by the reviewer pass)

Commands run:

- `sed -n '1,240p' AGENTS.md`
- `sed -n '1,260p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' specs/00-document-map.md`
- `sed -n '1,260p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,320p' specs/09-invariants-and-constraints.md`
- `sed -n '1,420p' specs/04-mir-core.md`
- `sed -n '1,320p' specs/08-cross-system-relations.md`
- `sed -n '1,320p' specs/10-open-questions.md`
- `sed -n '1,320p' specs/11-roadmap-and-workstreams.md`
- `sed -n '1,360p' specs/12-decision-register.md`
- `sed -n '1,240p' specs/05-mirrorea-fabric.md`
- `sed -n '1,220p' specs/06-prismcascade-positioning.md`
- `rg -n "Mir-0|mir-0|Mir 0|mir 0|fallback|atomic_cut|durable_cut|coroutine|emit|failure lattice|linear resources" specs docs README.md Documentation.md`
- `python scripts/new_report.py --slug mir-0-semantic-core-alignment`
- `python3 scripts/new_report.py --slug mir-0-semantic-core-alignment`
- `git diff --check`
- `python3 scripts/validate_docs.py`
- `git diff --stat`
- `git status --short`
- reviewer subagent on the changed specs

## 5. Evidence / outputs / test results

- `python scripts/new_report.py --slug mir-0-semantic-core-alignment`
  - Output: `/usr/bin/bash: line 1: python: command not found`
- `python3 scripts/new_report.py --slug mir-0-semantic-core-alignment`
  - Output: `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0001-mir-0-semantic-core-alignment.md`
- `git diff --check`
  - Output: no output, exit code 0
- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.`
  - Output: `Found 3 numbered report(s).`
- `git diff --stat`
  - Output: `specs/04-mir-core.md                   | 118 +++++++++++++++++++++++----------`
  - Output: `specs/09-invariants-and-constraints.md |   6 +-`
  - Output: `specs/10-open-questions.md             |  44 ++++++------`
  - Output: `specs/12-decision-register.md          |   3 +-`
  - Output: `4 files changed, 112 insertions(+), 59 deletions(-)`
  - Note: `git diff --stat` reports only tracked modifications, so the untracked reports are not counted in this total.
- `git status --short`
  - Output: ` M specs/04-mir-core.md`
  - Output: ` M specs/09-invariants-and-constraints.md`
  - Output: ` M specs/10-open-questions.md`
  - Output: ` M specs/12-decision-register.md`
  - Output: `?? docs/reports/0001-mir-0-semantic-core-alignment.md`
  - Output: `?? docs/reports/0002-mir-0-review-findings.md`
- reviewer subagent on the changed specs
  - Output summary: the first review pass flagged two substantive issues in the then-current draft: overly strong "source of truth" wording in `specs/04` and silent removal of `barrier`.
  - Resolution: both were addressed in a follow-up edit by weakening the authority wording and classifying `barrier` as deferred beyond Mir-0.

Normative change note:

- This pass intentionally changed the normative reading of `specs/04-mir-core.md` by fixing a narrower Mir-0 kernel and moving `barrier`, `durable_cut`, full fallback normalization, `emit`, coroutine semantics, and overlay alias details out of Mir-0.
- This pass intentionally changed the normative reading of `specs/04-mir-core.md` by fixing a narrower Mir-0 kernel, treating `atomic_cut` as a place-local Mir-0 cut primitive, and using `perform` only as working notation for the minimum effect request operation.
- This was recorded explicitly in `specs/10-open-questions.md` and `specs/12-decision-register.md` rather than left implicit.

## 6. What changed in understanding

- The strongest stable core already present across the specs was smaller than the prior `specs/04-mir-core.md` presentation suggested.
- Safe evolution remains a foundational Mir concern, but the complete operational semantics of overlays, route rebinding, and patch activation do not need to be inside Mir-0.
- `atomic_cut` can be fixed as the minimum Mir-0 cut primitive without forcing an early decision about `durable_cut`.
- `atomic_cut` needs an explicit minimum scope in Mir-0; place-local finalization is the narrowest reading that stays consistent with local rollback and leaves distributed durability to later work.
- `barrier` was present in the earlier Mir summary but not yet stable enough to remain in Mir-0 without a clearer relation to `atomic_cut` and `durable_cut`.
- A minimum notion of `place` is needed in Mir-0 to make rollback scope, ownership transfer, and same-place reasoning meaningful, but routing and distribution policy should remain outside Mir-0.
- `perform` can be used as working notation in this pass, but the repo does not yet justify treating that exact keyword as a final settled part of Mir syntax.

## 7. Open questions

- **OPEN QUESTION**: whether `barrier` remains necessary as distinct vocabulary once `atomic_cut` is fixed as the minimum Mir-0 cut primitive.
- **OPEN QUESTION**: whether `durable_cut` belongs to Mir-1 or to a persistence / distributed-finalization layer beyond Mir-0.
- **OPEN QUESTION**: whether `perform` remains the final Mir-0 keyword for the minimum effect request operation or is replaced by different surface notation.
- **UNRESOLVED**: full normalization of fallback / preference chains.
- **UNRESOLVED**: exact relationship between `emit`, effect handlers, and structured event routing.
- **UNRESOLVED**: coroutine semantics, especially suspension restrictions and interaction with cuts, rollback, and patching.
- **UNRESOLVED**: exact overlay alias and route rebinding details.
- **UNRESOLVED**: exact surface syntax and static rules for `place` introduction and cross-place transfer.

## 8. Suggested next prompt

`Read the updated Mir-0 boundary in specs/04, specs/09, specs/10, and specs/12, then refine the remaining cut semantics by deciding whether durable_cut belongs to Mir-1 or to a persistence/distributed-finalization layer, without broadening Mir-0.`
