# plan/107 - G1 ELAB-04 mixed visibility payload-model preflight

## Purpose

This file records a LAB-only, docs-only preflight for any future `ELAB-04`
mixed visibility payload decision. In this file, "payload-model preflight"
means a conceptual documentation model for future choices, not an adopted
Diagnostic ABI, JSON shape, emitted payload, or repair object.

Conclusion: `ELAB-04` remains no-repair in executable output. This package
does not widen `suggested_repair[]`, does not edit Rust emission logic, does
not edit expected JSON, does not add `ELAB-04` mixed set-insertion or general
set-insertion support, does not add bundle support, does not add
visibility-repair ranking, does not edit canon, does not freeze a Diagnostic
or repair ABI, does not prove OBL-024/025, does not claim conformance, and
does not claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon source hierarchy:
  `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and
  `mirrorea_canon/meta/source-hierarchy.md`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon static semantics:
  `mirrorea_canon/spec/03-static-semantics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon scenario pressure:
  `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`,
  `mirrorea_canon/scenarios/SCN-02-attack.md`, and
  `mirrorea_canon/scenarios/SCN-05-portal.md`
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB set-insertion / bundle payload inventory:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB exact `ELAB-07` set path:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md` through
  `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `scripts/surface_mir_samples.py`, and
  `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/`
- Advisory reviews:
  sub-agent mapping and ChatGPT Pro Oracle consult for this package, recorded
  in the package report

If this LAB preflight conflicts with canon, canon wins.

## Current executable facts

`ELAB-04` is a read-side row-containment rejection for a visible indexed-state
field. The current LAB detail is:

| Field | Current value |
|---|---|
| `legacy_code` | `generated_failure_not_declared` |
| `canon_id` | `E-ROW-001` |
| `request_id` | `req-0001` |
| `request_kind` | `read` |
| `generated_from` | `cross_locus_read_expression` |
| `requester_locus` | `role:BrowserClient` |
| `owner_locus` | `S` |
| `state_name` / `field_name` | `player` / `hp` |
| `target_kind` | `when_fails_row` |
| `target_ref` | `when_fails_row|locus=role:BrowserClient|event=render` |
| `required_failures` | `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` |
| `declared_failures` | `MissingCapability` |
| `missing_failures` | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` |
| current repair output | no `suggested_repair` field |

The top-level `E-ROW-001` classifier is a current LAB carrier choice. It is not
a final branch-ownership model and must not be read as saying that the
`VisibilityDenied` component is a base remote-request failure.

## Preflight branch ownership model

Record three conceptual association layers, without emitting them in JSON
today. The top-level conceptual wrapper accounts for the current failed
row-containment premise; child branches only classify / account for subsets of
missing evidence.

| Layer | Scope | Current status | Future use |
|---|---|---|---|
| mixed wrapper | the associated generated request and the whole failed row-containment premise | conceptual only | the only layer that may claim whole-row local-premise discharge if every branch is covered |
| base remote-request branch | classifies / accounts for missing evidence `MissingWitness`, `RouteUnavailable`, `StaleMembership` | conceptual only | may later use a base set-insertion or conjunctive bundle shape, but only as branch coverage unless the visibility branch is also covered |
| visibility branch | classifies / accounts for missing evidence `VisibilityDenied` | conceptual only | may later use an `E-ROW-002`-like row addition or a separate visibility / observe-authority family |

Recommended first future payload shape, if any, is a top-level mixed wrapper
with branch children. That avoids double-counting one generated request while
still preserving the base / visibility split. Separate E-ROW-001 and E-ROW-002
diagnostics remain possible later, but only if they share an explicit
association key and cannot be mistaken for two independent generated requests.

This preflight does not adopt either shape. The child branches are not emitted
diagnostics, not repair items, not branch IDs, and not independent failed
premises.

## Conceptual wrapper invariants

The docs-only association model is safe only under these invariants:

- canon in `mirrorea_canon/` remains normative;
- current `ELAB-04` emits one rejected generated cross-locus read request, one
  `E-ROW-001` LAB diagnostic detail, and no `suggested_repair` field;
- the conceptual wrapper is associated with the current single request
  `req-0001` and the current concrete `when_fails_row` target;
- the top-level wrapper / current diagnostic owns the whole failed premise
  `generated_failures_subset_declared_fails`;
- child branches classify missing evidence only; they are not emitted
  diagnostics, repair items, branch IDs, independent premises, or independent
  actionable repair objects;
- branch partition is exact for the current fact pattern:
  `{MissingWitness, RouteUnavailable, StaleMembership}` union
  `{VisibilityDenied}` equals current `missing_failures`, and the intersection
  is empty;
- `MissingCapability` is a required failure but already declared, so it is not
  branch-missing evidence;
- no complete repair may be claimed unless all missing failures for the
  associated request are covered;
- `VisibilityDenied` must not be collapsed into base remote-request failures;
- current `canon_id = E-ROW-001` is lossy for branch ownership and does not
  prove that `ELAB-04` is base-only;
- source-level visibility must not be used to erase the computed
  `VisibilityDenied` missing evidence;
- branch order in prose or tables is editorial only and has no ranking meaning;
- no-repair rows continue to omit `suggested_repair`; this package assigns no
  final semantics to `suggested_repair: []`.

## Safe association vocabulary

The following names are repository-memory vocabulary only, not final JSON keys:

| Vocabulary | Meaning | Current evidence |
|---|---|---|
| `associated_request_id` | the generated request whose row-containment premise failed | current `request_context.request_id` |
| `target_row_ref` | the concrete failure-row target | current `failure_row_context.target_ref` |
| `mixed_group_id` | future id tying all branch payloads for one generated request and one target row | candidate only |
| `branch_id` | stable name for a conceptual branch, such as `base_remote_request` or `visibility` | candidate only |
| `branch_missing_failures` | missing failures owned by one branch | derivable from current `missing_failures` |
| `branch_coverage_scope` | whether a branch item is complete for only the branch or for the whole row | candidate only |
| `whole_gap_coverage` | whether every missing failure for the associated request is covered | candidate only |
| `local_premise_after_edit` | whether the whole row-containment premise is discharged after the edit/group | current set path uses this for exact `ELAB-07`; `ELAB-04` candidate only |

Important distinction:

- `complete_for_branch_not_whole_row` is not OBL-025 single-edit coverage for
  the current whole row.
- `complete_missing_set_for_associated_request` must be reserved for a wrapper
  or item whose effect includes both base and visibility components.

## Ordering and ranking policy

Current policy:

```text
ELAB-04 emits no suggested_repair and therefore has no ordering or ranking.
```

Future invariant:

- any prose / table ordering of branches is editorial only and must not imply
  ranking or preference;
- branch children must not be serialized as alternatives if all are required;
- a base branch repair must not look complete while `VisibilityDenied` remains
  undeclared;
- a visibility branch repair must not look complete while base failures remain
  undeclared;
- any ranking between row addition, visibility declaration, observe-authority,
  and grouped branch plans must say whether it ranks complete whole-row
  repairs, branch-local guidance, or human-facing alternatives;
- no ranked item may claim runtime success, visibility authorization,
  capability, witness, route, membership, or whole-program acceptance.

Explicitly deferred:

- final repair ranking;
- visibility-repair ranking;
- whether visibility repair belongs in `suggested_repair[]` or a separate
  guidance field;
- whether separate diagnostics or a mixed wrapper is the final public ABI;
- whether OBL-025 should stay single-edit only or gain a grouped multi-edit /
  whole-gap relation.

## Candidate future shape matrix

This matrix is only a preflight. It does not authorize executable output.

| Shape | Complete whole-row repair? | Main risk |
|---|---:|---|
| mixed set insertion adding all missing failures to one row | yes if accepted as one source edit and whole-gap coverage | collapses `VisibilityDenied` into base failures unless branch metadata remains explicit |
| mixed wrapper with base set child plus visibility child | yes only at wrapper level | child list can be misread as alternatives instead of `all_required` |
| base branch set insertion only | no | hides the still-missing visibility branch |
| visibility singleton only | no | hides the still-missing base branch |
| partial textual guidance | no | can be overread as OBL-025 repair evidence |
| separate E-ROW-001 / E-ROW-002 diagnostics | possible later | double-counts one generated request unless association is explicit |

## Hidden failure modes

- Treating the current top-level `E-ROW-001` classifier as final ownership for
  the whole mixed row.
- Letting the docs-only wrapper become a de facto Diagnostic ABI, JSON schema,
  emitted payload, branch-id namespace, or repair-object shape.
- Describing child branches as independent failed premises rather than
  classification partitions of the current missing-evidence set.
- Treating `VisibilityDenied` as a base remote-request failure atom.
- Treating a visible source field as proof that no visibility failure branch
  exists in the generated failure set.
- Emitting branch children as independent alternatives when all are required.
- Calling a branch-local repair a whole-row local-premise witness.
- Letting exact `ELAB-07` set-insertion vocabulary leak into `ELAB-04` without
  visibility-branch metadata.
- Ranking branch guidance before defining whether the ranking is over complete
  repairs, partial guidance, or human-facing alternatives.
- Standardizing `suggested_repair: []` semantics for no-repair rows.
- Emitting separate diagnostics without a shared association key for the one
  generated request.
- Treating a branch list or table order as ranking.
- Generalizing this single-request / single-row-target note to multi-request
  rows, multiple failure-row targets, or ambiguous non-row targets.
- Calling current no-repair omission final canon-format behavior. It is a
  non-final LAB carrier behavior.

## Relation to existing executable output

The executable state remains:

- `ELAB-04`: no `suggested_repair`;
- `ELAB-10`: one `E-ROW-002` / `VisibilityDenied` singleton item;
- `ELAB-13..16`: one `E-ROW-001` non-visibility singleton item per row;
- exact `ELAB-07`: one non-final `set_insertion` item under `plan/102`,
  guarded by `plan/103..106`.

Singleton repair emission remains guarded by singleton missing-failure cases.
The only current non-singleton exception is the exact `ELAB-07`
set-insertion path. `ELAB-04` remains outside both paths.

This package adds no sample row and changes no expected JSON.

## Acceptance criteria for this package

- Add this docs-only preflight as LAB repository memory.
- Update `plan/00-index.md` and `plan/90-source-traceability.md`.
- Update `README.md`, `Documentation.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, the Surface research abstract, and the elaboration
  sample README to say that `ELAB-04` has branch ownership / association /
  ranking preflight evidence while remaining no-repair.
- Validate that `ELAB-04` still omits `suggested_repair` through the Surface
  helper / tests.
- Keep sample row count at 52.
- Write a new report under `docs/reports/`.

## Suggested next packages

1. Keep `ELAB-04` no-repair unless a later package explicitly accepts a mixed
   wrapper or separate associated diagnostics model.
2. If executable widening is desired, first choose whether the public LAB
   payload uses a mixed wrapper with branch children or separate associated
   diagnostics.
3. If proof-side work comes first, refine OBL-025 around whole-gap coverage,
   branch-local guidance non-coverage, and grouped multi-edit relation without
   importing final ranking. `plan/108` later adds branch-local non-coverage
   helper predicates to the LAB Lean statement draft while preserving
   no-repair executable output.
4. Draft OBL-024 only after diagnostic replay / association vocabulary can
   distinguish one mixed request from two independent diagnostics.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No `ELAB-04` executable payload.
- No `ELAB-04` mixed set-insertion support.
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
