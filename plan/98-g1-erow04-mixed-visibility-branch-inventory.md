# plan/98 - G1 ELAB-04 mixed visibility branch inventory

## Purpose

This file records the LAB-only branch inventory for `ELAB-04`, the mixed
visibility / non-visibility generated failure-row omission.

Conclusion: `ELAB-04` remains no-repair in executable output. This package does
not widen `suggested_repair[]`, does not add set-insertion support, does not
add bundle semantics, does not add visibility-repair ranking, does not edit
canon, does not freeze a Diagnostic or repair ABI, does not prove OBL-024/025,
does not claim conformance, and does not claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB visibility singleton prototype:
  `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB set-insertion / bundle payload inventory:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-07` gate review:
  `plan/97-g1-erow07-set-insertion-gate-review.md`
- LAB `ELAB-07` executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `scripts/surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/`
- Advisory review:
  sub-agent inventory from 2026-07-04 and Oracle consult
  `retry-after-attachment-timeout-advisory` completed / advisory, recorded in
  the package report

If this LAB inventory conflicts with canon, canon wins.

## Current `ELAB-04` facts

`ELAB-04` is a read-side row-containment rejection for a visible indexed-state
field:

```text
S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability {
    seen_hp = player[self].hp
  }
}
```

Current expected diagnostic detail:

| Field | Current value |
|---|---|
| `canon_id` | `E-ROW-001` |
| `request_kind` | `read` |
| `generated_from` | `cross_locus_read_expression` |
| `target_kind` | `when_fails_row` |
| `required_failures` | `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` |
| `declared_failures` | `MissingCapability` |
| `missing_failures` | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` |
| `local_premise` | `generated_failures_subset_declared_fails` |
| current repair output | no `suggested_repair` field |

The current implementation classifies the mixed missing set as `E-ROW-001`
because `E-ROW-002` is reserved in LAB output for the singleton
`VisibilityDenied` case. It also omits `suggested_repair` because the current
repair emitter only handles singleton missing-failure rows.

This `E-ROW-001` classifier result is lossy for branch ownership. It must not
be read as evidence that `ELAB-04` is base-only. The independent branch fact is
that `VisibilityDenied` remains in the computed missing set.

The source field is marked visible, but current required-failure computation
still includes `VisibilityDenied`. Do not replace the generated failure-row
fact with the shortcut `visible source read means no visibility failure branch`.

## Branch split

`ELAB-04` has at least two conceptual branches.

| Branch | Missing failures | Current executable status | Future pressure |
|---|---|---|---|
| base remote-request branch | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | no repair | same set-insertion / bundle / partial-guidance questions as `ELAB-07` |
| visibility branch | `VisibilityDenied` | no repair in mixed row | `E-ROW-002`-like row addition or later visibility / observe-authority repair family |

This split is conceptual repository memory only. The current executable output
does not emit separate branch diagnostics, child repairs, branch summaries, or
ranking metadata.

## Why `ELAB-04` is not just `ELAB-07` plus one atom

`VisibilityDenied` is not merely a fifth base remote-request failure. It names
a visibility / observation-policy failure surface and has a separate singleton
prototype in `ELAB-10`.

Unsafe readings:

- treating the whole mixed row as ordinary `E-ROW-001` set insertion and
  hiding the visibility branch;
- treating `VisibilityDenied` as ordinary non-visibility failure evidence;
- emitting the `ELAB-10` singleton repair as a visible child while the base
  branch remains unrepaired;
- emitting several child items without `all_required` semantics;
- ranking visibility and base-branch repairs before the project has a ranking
  policy.

The current safe reading is no-repair executable output with explicit branch
pressure recorded in LAB memory.

## Current executable guard

Current code gates repair emission as follows:

```text
emit repair only when missing_failures.len() == 1
```

Then it chooses one of two singleton families:

- `E-ROW-002` plus `VisibilityDenied`;
- `E-ROW-001` plus one base remote-request failure.

`ELAB-04` fails both singleton gates: its missing set has four failures, and
it mixes the visibility branch with base remote-request failures.

This guard should remain until a later package defines diagnostic association,
branch grouping, whole-gap coverage, and ranking / ordering semantics.

## Status classification

Use separate statuses for `ELAB-04`:

| Status | Meaning | `ELAB-04` current reading |
|---|---|---|
| `no_repair_executable` | diagnostics omit `suggested_repair` | yes |
| `mixed_branch_pressure` | row requires base / visibility branch separation before widening | yes |
| `candidate_branch_bundle_gate` | a future grouped payload may cover both branches under explicit semantics | candidate only, not adopted here |
| `repair_bearing_evidence` | expected JSON emits a complete local repair item | no |

## Minimum future executable widening package

A later executable mixed-row widening package must update all of the following
in one task:

- diagnostic ownership policy: one mixed diagnostic, separate E-ROW-001 /
  E-ROW-002 diagnostics, or one top-level wrapper;
- branch vocabulary: base branch, visibility branch, child branch coverage,
  and associated request id;
- payload model, because current `missing_failure: String` cannot express
  branch groups or mixed whole-gap coverage;
- local premise coverage relation for the whole rejected row, not just one
  branch;
- ordering / ranking policy when base and visibility branch repairs are both
  present;
- Rust emission logic and tests;
- Python helper tests;
- expected JSON for affected rows;
- sample README and matrix wording if status changes;
- `plan/95`, `plan/96`, this file, and any OBL-025 statement note affected by
  branch coverage;
- `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md`;
- a new report under `docs/reports/`.

Before executable widening, the LAB predicates should be explicit enough to
distinguish at least:

- `missing_failures = required_failures - declared_failures`;
- base-missing failures versus the `VisibilityDenied` component;
- visibility singleton rows;
- base singleton rows;
- base composite rows;
- mixed base / visibility rows;
- complete repair coverage for every missing failure in the associated row;
- partial guidance that may be diagnostic information but is not OBL-025
  coverage.

The payload must include or otherwise prove:

- one associated generated request;
- one concrete `when_fails_row` target or explicit multi-diagnostic target
  association;
- clear separation of base remote-request failures from `VisibilityDenied`;
- a whole rejected-gap coverage relation if it is presented as complete repair;
- `declared_failures_after` or equivalent branch effects containing every
  required failure needed for the associated request;
- explicit local-premise discharge for the whole row;
- no runtime success, authorization, visibility authorization, capability /
  witness / route / membership availability, or whole-program acceptance claim;
- LAB / non-final flags.

## Hidden failure modes

- A top-level `E-ROW-001` repair can hide that part of the row is
  `E-ROW-002`-like.
- Treating `E-ROW-001` as a branch label can hide that the classifier is lossy
  for mixed visibility rows.
- A visible source field can be misread as proving the generated row has no
  `VisibilityDenied` component.
- A child `VisibilityDenied` repair can look complete while the base branch
  remains underdeclared.
- A base set insertion can look complete while `VisibilityDenied` remains
  underdeclared.
- A list of child repairs can be misread as alternatives when all are required.
- Branch ranking can imply a preferred repair without a ranking policy.
- A visibility repair can be overread as granting observation authority or
  making runtime access safe.
- A row repair can be overread as supplying capability, witness, route, or
  membership evidence.
- Separate E-ROW-001 and E-ROW-002 diagnostics can double-count one generated
  request unless diagnostic association is explicit.
- A future `suggested_repair: []` on mixed rows can accidentally standardize
  empty-list semantics before the LAB carrier is ready.
- Any-overlap coverage can accidentally count a repair as complete merely
  because one missing failure has singleton repair guidance.
- A future set-handling refactor can make `ELAB-07` gain `suggested_repair`
  before its own set-insertion or bundle gate is closed.

## Relation to OBL-025

OBL-025 remains a Line-1 single-edit explanation-completeness target in canon.
Current LAB repair-bearing evidence is singleton-only:

- `ELAB-10`: `E-ROW-002` / `VisibilityDenied` singleton;
- `ELAB-13..16`: `E-ROW-001` non-visibility singleton rows.

`ELAB-04` is not current coverage evidence. It is pressure evidence for:

- branch association;
- mixed whole-gap coverage;
- ordering / ranking;
- partial-guidance non-coverage;
- possible future grouped multi-edit or set-insertion relations.

No current row proves OBL-025 or moves canon ledger status.

## Suggested next packages

1. Keep executable output unchanged and periodically validate that `ELAB-04`
   still omits `suggested_repair`.
2. Treat `plan/99..102` as the completed exact `ELAB-07` preflight,
   assumption, payload design, and executable set prototype sequence before
   mixed-row widening.
3. Draft a mixed-branch payload model only after diagnostic ownership and
   ordering / ranking are explicit.
4. Draft OBL-024 only after diagnostic replay / association vocabulary is
   stable enough to discuss mixed rows without importing repair ranking.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No set-insertion support.
- No bundle semantics support.
- No visibility-repair ranking.
- No partial-guidance output support.
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
