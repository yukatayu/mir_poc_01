# Report 2502 - C2-B/C3 Family A/B instantiation audit

**Identifier:** `LAB-REPORT-2502`
**Date:** 2026-07-28 17:26 JST
**Status:** local validation passed; content commit pending

## Objective

Compare the relation-first Family A and request-occurrence Family B
presentations against Plan 209's staged C2-B/C3 obligations, without selecting
a semantic carrier or treating ergonomic omission as semantic inference.

## Scope and assumptions

This is LAB ordinary-design preparation only. Canon remains normative. The
audit does not select a Core constructor, occurrence equality rule, Config or
SaveObject field, source syntax/elaboration rule, runtime, queue/wire format,
API, OBL, Gate, Phase, or public behavior. It preserves P012 V1/R1 and P013
M1 as bounded directions and leaves OPEN-010/011 unresolved.

## Start state / dirty state

Started from clean `main` at
`0240956f732f9246bc1630d6a4288d1f76ce38e5`, equal to `origin/main`, after
the Plan 209 package. No user-authored dirty change was present. The Plan 210
draft was the task-local uncommitted change before documentation synchronization.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, ADR-0014, P012, P013, and theory/01/04/05/11
- architecture/04 and runtime-semantics/05 for the L2 carrier boundary
- Plans 199, 200, 207, 208, and 209
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
- a temporary GPT-5.6 Sol Pro advisory review with the Plan 210 draft and
  relevant Canon/LAB sources attached

## Actions taken

1. Compared both presentations at every staged obligation: validation context
   and outcome, pending, reply/result, receipt, failure, one-shot resume,
   later dependency, save/load, and ergonomic projection.
2. Made explicit that the DAG supplies ordering/prefix constraints only after
   a semantics identifies the relevant roles and edges; it is not correlation
   or post-load occurrence equality.
3. Preserved M1's distinction between non-authoritative request claims and
   authoritative owner validation. Audit judgments need not become fields or
   relation objects.
4. Recorded symmetric viability conditions: A needs a semantic configuration
   locus and load behavior; B needs selected equality/load scope plus
   non-circular staged projections stronger than ancestry or incidental data.
5. Recorded the minimum future design questions while leaving Family C as a
   reserve comparison only, not a selected nominal identity.
6. Synchronized plan indexes, validator inventories, reader guidance, current
   status, progress snapshot, and task map.

## Files changed

- `plan/210-c2b-c3-family-a-b-instantiation-audit.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Focused Canon/LAB source reads, path inventory, relation-boundary searches,
  working-tree checks, and diff reviews
- `ask-chatgpt-pro-temp` temporary advisory review with nine attached sources
- `git diff --check`
- `make docs`

## Evidence / outputs / test results

The temporary Oracle review completed under GPT-5.6 Sol Pro in 9m28s. Its
response SHA-256 is
`7e4299b8ef5f5602acce3b22a01fee8bd5fc7426ef5398b78486f4b9ee003402`.
It identified the need to keep `CtxOf` observational under M1, avoid claiming
that an existing served occurrence is a typed reply, preserve OPEN-010 for the
requester-side failure mapping, prevent a B bias from source spans or DAG
ancestry, and scope one-shot behavior to restored-prefix execution rather than
global transport exactly-once. Plan 210 incorporates these corrections.

No Lean, runtime, sample, transport, or end-to-end result is claimed by this
documentation-only audit. `make docs` passed: Canon index reported 123 indexed
files; source hierarchy reported 760 required and 760 present paths; documentation
validation reported a complete scaffold and 1,656 numbered reports.

## What changed in understanding

Family A and Family B are not competing completed implementations. A is only
a reference vocabulary until a semantic configuration and restore behavior are
selected. B has an existing request-occurrence anchor but cannot derive pending,
reply, receipt, consumption, or equality from existence, order, span, payload,
or transport data. The design choice must therefore select the smallest
semantic state/projection model that realizes every staged obligation.

This retains a practical ergonomic route: after that selection, an elaborator
may hide or generate self-evident administrative spelling only when it keeps an
inspectable elaborated artifact and diagnostics for every selected semantic fact.

## Open questions

- Is Family A's staged relation or Family B's request-occurrence anchor
  definitional; if both are present, which is derived?
- Where do pending, outcome, reply/receipt, provenance, terminal status, and
  held-context disposition live in the semantic configuration?
- What equality/reconstruction and one-shot scope applies after admissible
  save/load without claiming global exactly-once?
- What exact failure mapping, success/failure `Gamma`/`Delta` disposition, and
  post-resume dependency account are selected without resolving OPEN-010/011 early?

## Suggested next prompt

Review the Family A/B decision packet and decide which presentation is
definitional for the ordinary Canon design package. The current LAB
recommendation is to avoid Family C unless a concrete A or B instantiation
fails its Plan 209 obligations under a stated load scope.

## Plan update status

更新済み: `plan/210-c2b-c3-family-a-b-instantiation-audit.md`, Plans 199/200,
and `plan/00-index.md` record the comparison without selecting a carrier.

## Documentation.md update status

更新済み: `Documentation.md` links Plan 210 and explains its conditional A/B boundary.

## docs/project-status.md update status

更新済み: `docs/project-status.md` records that A/B are conditional and no theory or implementation status advanced.

## progress.md update status

更新済み: `progress.md` records the current decision boundary and the timestamped Plan 210 work log.

## tasks.md update status

更新済み: `tasks.md` records Family A/B as the immediate C2-B/C3 selection options.

## samples_progress.md update status

更新不要: no active sample root, runnable workflow, validation command, debug surface, or blocker changed.

## Reviewer findings and follow-up

The temporary Oracle review was advisory only. Its material findings were
applied and checked against Canon: M1 observational status, reply/receipt and
failure distinctions, DAG/equality non-inference, prefix-scoped one-shot
behavior, and non-selecting ergonomics. No callable sub-agent session was
available.

## Skipped validations and reasons

No Lean, runtime, sample, transport, or end-to-end validation applies because
the package creates no executable or selected semantic artifact. No Canon edit
is attempted because the A/B carrier/state choice remains an ordinary design
selection.

## Commit / push status

Pending: the content package and a report-status closeout will each be
committed with `--no-gpg-sign`, pushed to `origin/main`, and verified by
fetching `origin/main` after successful documentation validation.

## Sub-agent session close status

No callable sub-agent session was opened.
