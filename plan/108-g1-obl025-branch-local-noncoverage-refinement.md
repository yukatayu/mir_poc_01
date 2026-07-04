# plan/108 - G1 OBL-025 branch-local non-coverage refinement

## Purpose

This file records a LAB-only refinement of the OBL-025 Lean statement-shape
draft after the `ELAB-04` mixed visibility preflight.

The refinement adds abstract branch-local non-coverage vocabulary to
`samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
It makes explicit that branch-local repair witnesses or branch-local guidance
do not satisfy OBL-025 whole-gap coverage for a mixed row unless a separate
whole-gap relation covers every missing failure for the associated request.

This package does not edit canon, does not move
`mirrorea_canon/theory/11-metatheory-ledger.md`, does not prove OBL-025, does
not claim OBL-025 completion, does not widen executable repair output, does
not add `ELAB-04` payloads, does not add general set-insertion support, does
not add bundle semantics, does not add partial-guidance output, does not
freeze branch IDs / JSON keys / Diagnostic ABI / repair ABI, and does not
claim conformance or G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB OBL-025 Lean draft:
  `plan/87-g1-obl025-lean-statement-draft.md`
- LAB mixed / multi repair decomposition:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB set-insertion / bundle payload inventory:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB exact `ELAB-07` set-insertion evidence:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
  through `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`
- LAB `ELAB-04` mixed visibility preflight:
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- LAB statement artifact:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- LAB explanation:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- LAB repair completeness guard hardening:
  `plan/116-g1-obl025-repair-completeness-guard-hardening.md`
- LAB manifest:
  `samples/lean/manifest.json`

If this LAB refinement conflicts with canon, canon wins.

## What changed in the Lean draft

The refinement adds one abstract carrier:

```text
RepairBranch
```

and these abstract predicates:

```text
BranchOfRejectedGap
BranchLocalRepairWitness
SuggestedRepairBranchLocalGuidance
```

It also adds two helper relations:

```text
BranchLocalRepairNonCoverage
BranchLocalSuggestionNonCoverage
```

These helpers are intentionally negative / boundary relations. They make it
possible to say:

- a branch belongs to the rejected gap;
- a witness or suggestion is local to that branch;
- the witness / suggestion does not cover the whole rejected gap;
- therefore it must not satisfy current OBL-025 complete repair coverage.

The core `RepairCompletenessForRejection` relation still quantifies over
`EligibleSingleEditRepair` and `SuggestionCoversWitness`. Those still require
whole rejected-gap coverage. Branch-local helpers do not widen the coverage
relation.

## Relation to `ELAB-04`

`ELAB-04` remains executable no-repair evidence. `plan/107` records its
conceptual mixed wrapper, base remote-request branch, and visibility branch
without emitting branch diagnostics, branch IDs, repair items, or ranking.

This OBL-025 refinement mirrors that reading:

- the base branch may classify / account for `MissingWitness`,
  `RouteUnavailable`, and `StaleMembership`;
- the visibility branch may classify / account for `VisibilityDenied`;
- either branch alone is branch-local and not complete for the whole row;
- no branch-local suggestion may count as OBL-025 coverage unless a later
  whole-gap relation also covers every missing failure for the associated
  request.

`RepairBranch` is therefore an abstract proof-shape carrier only. It is not a
final branch ID, JSON key, diagnostic field, or public ABI.

## Relation to `ELAB-07`

Exact `ELAB-07` continues to use the later `plan/102` set-insertion payload
under the guard chain in `plan/103..106`.

This refinement does not reclassify `ELAB-07`. The existing set-insertion
reading remains:

- a set-insertion witness can enter current OBL-025-shaped coverage only when
  it is admitted as `EligibleSingleEditRepair`;
- it must cover the whole rejected gap;
- it must not be a child singleton alternative, bundle field, partial
  guidance, or textual-only guidance.

## Relation to grouped multi-edit and partial guidance

The existing Lean draft already separates:

- `CompleteGroupedMultiEditRepair`, which names complete grouped witnesses but
  does not add them to current OBL-025 coverage; and
- `PartialGuidanceNonCoverage`, which names partial guidance and excludes it
  from `SuggestionCoversWitness`.

This package adds the analogous mixed-row guard:

- `BranchLocalRepairNonCoverage` for branch-local witnesses; and
- `BranchLocalSuggestionNonCoverage` for branch-local guidance.

These are not final semantics for bundles, partial guidance, branch IDs, or
repair ranking.
`plan/116` adds a sync-test guard that keeps this reading from drifting into
placeholder non-empty repair lists, repair ranking, all-repairs / minimality
vocabulary, final repair ABI names, or branch-local guidance as whole-gap
coverage.

## Current coverage reading

Current narrow OBL-025-shaped evidence remains:

- singleton repair-bearing rows `ELAB-10` and `ELAB-13..16`;
- exact `ELAB-07` set-insertion as LAB evidence for a candidate single-source
  edit under its exact assumption and guards.

Current non-coverage / pressure evidence remains:

- mixed `ELAB-04` branch-local pressure, with no executable repair output;
- grouped multi-edit / bundle shapes unless a later obligation admits them;
- partial guidance unless a later field and relation make its non-coverage
  explicit.

## Open questions

- Should a future OBL-025 extension admit grouped multi-edit whole-gap
  coverage, or should grouped repair completeness remain a separate
  obligation?
- Should branch-local guidance ever live in `suggested_repair[]`, or should it
  use a separate guidance field?
- If mixed rows later emit separate associated diagnostics, what association
  key prevents double-counting one generated request?
- Which future visibility repair family, if any, should cover
  `VisibilityDenied` in mixed rows?
- How should branch-local guidance be ordered or ranked relative to complete
  whole-row repairs?

## Suggested next packages

1. Keep `ELAB-04` no-repair until a mixed wrapper or associated diagnostics
   model is explicitly accepted.
2. Draft OBL-024 only after diagnostic replay / association vocabulary can
   distinguish one mixed request from two independent diagnostics.
3. Revisit OBL-025 only if a later package wants grouped multi-edit coverage
   or branch-local guidance in executable output.

## Non-claims

- No canon edit.
- No proof-status movement.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No final branch ID or branch JSON key.
- No repair generation widening.
- No `ELAB-04` executable payload.
- No general set-insertion support.
- No bundle semantics support.
- No partial-guidance output support.
- No visibility-repair ranking.
- No repair ranking.
- No multi-edit support.
- No OBL-024 proof.
- No OBL-025 proof.
- No OBL-025 completion.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
- No whole-program success after repair claim.
