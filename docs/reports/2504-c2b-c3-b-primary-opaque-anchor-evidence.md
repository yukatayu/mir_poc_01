# Report 2504 - C2-B/C3 B-primary opaque-anchor finite evidence

**Identifier:** `LAB-REPORT-2504`
**Date:** 2026-07-28 18:41 JST
**Status:** local evidence and independent review passed; evidence commit pending

## Objective

Execute the committed and pushed WRK-0037 procedure as a finite L3 experiment,
then determine whether its result remains artifact-local and whether every
registered falsifier is actually covered.

## Scope and assumptions

This is LAB evidence only. `mirrorea_canon/` remains normative. The experiment
does not select Family A/B/C, a Mir request/occurrence carrier or equality,
Core/Config/history/SaveObject state, a persistence rule, authority, source
inference, runtime behavior, OBL, Gate, Phase, conformance, or public API.

The finite model has exactly two locally distinct request atoms with equal
listed incidental observations. It tests only the explicit finite functions
and extension tables in its one fenced Lean block.

## Start state / dirty state

Started after the WRK-0037 registration was committed and pushed as
`2c9d5f3adf197fbf96d6db31488c49f3f6411db6`, with local `HEAD` equal to
`origin/main` and no user-authored dirty change. The task began by materializing
one new untracked LAB plan artifact only.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, ADR-0014, and `working/WRK-0037`
- P012, P013, theory/01, theory/04, and theory/05
- Plans 199, 200, 208, 209, 210, and 211
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and the existing WRK-0033--0036 artifact patterns
- Oracle operating guidance and two temporary GPT-5.6 Sol Pro advisory reviews

## Actions taken

1. Confirmed the registration had been pushed before materializing any source
   and rechecked every pinned input and digest.
2. Wrote a restore test that failed when the load view omitted the registered
   renaming; then implemented the explicit local reindexing and all-frontier
   round-trip check.
3. Replaced theorem proofs that accidentally depended on `propext` with explicit
   finite case analysis, leaving every retained theorem axiom-free.
4. Ran an independent review, which found that the first table lacked a
   sequential one-shot relation, failure mutation observable, and grounded
   dependency.
5. Wrote a second failing test that incorrectly accepted the same receipt at a
   later frontier, then added finite receipt/resume extension tables, direct
   grounds, and mutation observation.
6. Re-ran Lean, the placeholder scan, the registered source/digest checks, and
   diff validation. A second independent review found formal coverage complete
   and required only model-local prose narrowing.
7. Updated LAB plan memory and current snapshots without changing an active
   sample, implementation, Canon statement, or lifecycle status.

## Files changed

- `plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Focused Canon/LAB reads, working-record-history validator inspection, resource
  check, status/diff review, and exact registered source/digest/duplicate scans
- Two test-first extracted Lean runs that intentionally failed: omitted restore
  renaming, then duplicate receipt acceptance
- Repeated `lean --trust=0` extraction checks and `#print axioms` inspection
- `rg` scan for placeholders, unsafe/classical/choice/quotient/axiom tokens
- `git diff --check`
- `make docs`
- Two `ask-chatgpt-pro-temp` independent reviews with the WRK, selection plan,
  and evolving artifact attached

## Evidence / outputs / test results

Every WRK-0037 pinned Canon/LAB input was nonempty and matched its registered
SHA-256 digest. The final fenced source extracted to a disposable file with
SHA-256 `f80ece6b9b74985120e9016567a5543914c55006f5cae1ec01ade4d5c416bd5a`.
Lean 4.29.1 accepted it at `--trust=0`. `#print axioms` reported no axioms for
the no-left-inverse, distinction, injective/involutive restore, all-frontier
view round-trip, receipt/resume uniqueness, rejected-receipt, failure exclusion,
and authority-separation theorems. The prohibited-token scan and `git diff
--check` passed.

The first Oracle review completed in 14m28s; its response SHA-256 is
`37e2dd3ea5a4b1232a623fe0dbabd41a80ceb52b38f0220e4139a2efd0f99e21`.
It correctly identified the missing sequential receipt/resume, mutation, and
ground obligations. The revised review completed in 4m33s; its response
SHA-256 is `63be7cd478457aa9d3c37964fb6bc4fbbc424f8be8172d4d49d068ea66277513`.
It found the revised finite coverage complete and required prose narrowing
only. Both reviews are advisory inputs, not repository authority.

## What changed in understanding

An explicit request-indexed lookup table is not enough to demonstrate a scoped
one-shot branch: the finite candidate also needs an explicit relation from
receipt to subsequent resume, an observable failure-no-mutation condition, and
dependency/resume records that carry the same local result/provenance ground.

The retained non-inference fact is deliberately narrow. In this two-atom table,
the listed equal incidental record has no total left inverse that recovers both
atoms. This neither rules out recovery with other context nor establishes any
general carrier, recovery, or ergonomic-inference rule.

## Open questions

- Does an owner/Canon design select Family A, Family B, or another presentation
  for actual request correlation, pending state, receipt, and restore behavior?
- If Family B is later considered, which non-artifact semantic carrier and
  persistence scope make its identity and load behavior meaningful?
- The finite evidence does not advance the separate T0/T2/I1 lifecycle blockers.

## Suggested next prompt

Append the exact evidence-commit metadata to WRK-0037, validate the committed
history, then continue only with an ADR-0014-eligible research package or an
owner/Canon design decision. Do not promote this finite table into a carrier.

## Plan update status

更新済み: the new artifact, Plan index, and Plans 199/200 distinguish the
executed finite evidence from a semantic selection. Plan 211 remains unchanged
because it is a digest-pinned WRK input.

## Documentation.md update status

更新不要: WRK-0037's declared evidence surface permits `plan/`, direct reports,
and control files only. The reader-facing current state is synchronized through
`docs/project-status.md`; no general documentation claim changed.

## docs/project-status.md update status

更新済み: the status view records executed finite L3 evidence only and preserves
the owner/Canon C2-B/C3 decision boundary.

## progress.md update status

更新済み: the logical-specification row and recent log record the bounded result
without moving OBL, Gate, Phase, implementation, or public status.

## tasks.md update status

更新済み: the B2-OPAQUE autonomous package is marked as executed evidence, while
the carrier-selection decision remains owner/Canon work.

## samples_progress.md update status

更新不要: no active sample root, runnable workflow, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

The first temporary Oracle review returned `REVISE`, identifying three real
finite-coverage defects and two wording errors. The model was extended and the
second temporary review returned `REVISE — prose only`; that prose was narrowed
to the exact two-atom no-left-inverse statement. No callable sub-agent session
was available or opened.

## Skipped validations and reasons

No runtime, transport, parser, sample, or end-to-end command applies to this
artifact-local theorem table. `make docs` passed before the evidence commit:
Canon index, source hierarchy, report structure, and its secret scan all passed.
Committed WRK-history validation remains pending because the evidence commit has
not yet been created.

## Commit / push status

Evidence commit pending. It will be made with `--no-gpg-sign`, pushed to
`origin/main`, followed by a fetch and `HEAD == origin/main` check. A separate
metadata-only commit will then append the exact evidence commit and artifact
digest to WRK-0037.

## Sub-agent session close status

No callable sub-agent session was opened.
