# plan/97 - G1 ELAB-07 set-insertion gate review

## Purpose

This file records the LAB-only gate review for `ELAB-07` after the
set-insertion / bundle vocabulary inventory in `plan/96` and the OBL-025 Lean
statement refinement in `plan/87`.

Conclusion: `ELAB-07` remains no-repair in executable output. This package does
not widen `suggested_repair[]`, does not add set-insertion support, does not
add bundle semantics, does not edit canon, does not freeze a Diagnostic or
repair ABI, does not prove OBL-024/025, does not claim conformance, and does
not claim G1 exit.

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
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB set-insertion / bundle payload inventory:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB `ELAB-07` executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB `ELAB-07` assumption acceptance:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB `ELAB-07` payload-model design:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB singleton repair prototype:
  `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB OBL-025 statement draft:
  `plan/87-g1-obl025-lean-statement-draft.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/`
- Advisory review:
  sub-agent inventory from 2026-07-04 and Oracle consult
  `we-are-continuing-autonomous-g1`

If this LAB gate review conflicts with canon, canon wins.

## Current `ELAB-07` facts

`ELAB-07` is a non-visibility multi-missing `E-ROW-001` row-containment case.

Source:

```text
BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
```

Current expected diagnostic detail:

| Field | Current value |
|---|---|
| `canon_id` | `E-ROW-001` |
| `request_kind` | `write` |
| `target_kind` | `when_fails_row` |
| `required_failures` | `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `declared_failures` | `MissingCapability` |
| `missing_failures` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `local_premise` | `generated_failures_subset_declared_fails` |
| current repair output | no `suggested_repair` field |

The row is different from `ELAB-13..16`: each singleton row misses exactly one
base remote-request failure and now carries one LAB-only `add-to-fails-row`
repair item. `ELAB-07` misses three failures, so one ordinary singleton repair
would be a partial repair and would not discharge the local row-containment
premise.

## Current executable guard

The Rust payload shape is singleton-oriented:

- `SurfaceLabSuggestedRepair` has `missing_failure: String`;
- `local_effect.declared_failures_after` is computed by appending that one
  `missing_failure`;
- `erow_singleton_row_addition_suggested_repair` returns `None` unless
  `failure_row_context.missing_failures.len() == 1`.

Current Rust and Python tests assert that `ELAB-07` has missing failures
`MissingWitness`, `RouteUnavailable`, `StaleMembership` and omits
`suggested_repair`.

This guard is intentionally narrow. It prevents a one-child or one-atom repair
from being misread as whole rejected-gap coverage.

## Gate decision

`ELAB-07` should stay no-repair until the project makes one of the following
explicit:

1. **Set insertion is one source edit.** Adding all missing base failures to one
   concrete `when ... fails` row is accepted as a single source edit in the
   declared fragment.
2. **Conjunctive bundle semantics exist.** A grouped repair item with
   `all_required` semantics can state that all child edits must be applied
   together, and only the group discharges the local premise.
3. **Partial guidance is separated.** Human guidance that mentions missing
   failures is clearly marked as non-coverage and is not counted by
   `SuggestionCoversWitness` / OBL-025-shaped completeness.
4. **No repair remains the explicit policy.** The row continues to omit
   `suggested_repair` until a later proof / payload / edit-script boundary is
   ready.

This gate-review package chose option 4 at the time. `plan/100` later accepts
option 1 only as a narrow `ELAB-07` LAB source-locus edit assumption, but the
current LAB implementation still has no set payload. Option 2 is a different
witness class from the current OBL-025 single-edit coverage. Option 3 may be
useful later, but it is not a repair witness.

## Status classification

Use three statuses for `ELAB-07`, not a binary covered / uncovered label:

| Status | Meaning | `ELAB-07` current reading |
|---|---|---|
| `no_repair_executable` | diagnostics omit `suggested_repair` | yes |
| `candidate_set_insertion_gate` | this shape may become one grouped row edit under explicit LAB predicates | candidate only, not adopted here |
| `repair_bearing_evidence` | expected JSON emits a complete local repair item | no |

The next docs-first package may adopt a LAB-local sentence such as:

```text
Adding a duplicate-free set of base non-visibility failure atoms to one
existing concrete when_fails_row is one source edit for the ELAB-07
candidate gate.
```

This package did not adopt that sentence as executable policy. `plan/100`
later accepts the sentence only as a LAB source-locus edit assumption for the
exact `ELAB-07` candidate gate. Executable output still remains no-repair
until a separate set payload package implements and tests it.

## Minimum future executable widening package

A later executable `ELAB-07` widening package must update all of the following
in one task:

- Rust payload model, because current `missing_failure: String` cannot express
  a set insertion or bundle group;
- Rust emission logic for multi-missing base failures only, explicitly
  excluding `VisibilityDenied`;
- Rust tests proving whole rejected-gap coverage, no placeholder values, and no
  alternative-singleton semantics;
- Python helper tests proving the same actual sample behavior;
- expected JSON for `ELAB-07`;
- sample README and matrix stage / status wording;
- `plan/95`, `plan/96`, and this file;
- `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md`;
- a new report under `docs/reports/`.

The payload must include or otherwise prove:

- one associated generated request;
- a concrete `when_fails_row` target reference;
- base remote-request failures only;
- a whole rejected-gap coverage relation;
- `declared_failures_after` containing every `required_failure`;
- explicit local-premise discharge;
- no runtime success, authorization, capability / witness / route /
  membership availability, or whole-program acceptance claim;
- LAB / non-final flags.

For the first future widening, prefer exactly one top-level set-insertion item
over visible child repairs. Child repairs require bundle semantics first.

## Hidden failure modes

- Emitting three singleton-looking items can make required child edits look like
  alternatives even though all are required.
- Emitting one child item can make a partial repair look like OBL-025 coverage.
- Calling set insertion a single edit before the project decides edit
  granularity can silently widen the proof target.
- A set payload without whole-gap coverage can repeat the atom-level
  existential leak that the OBL-025 draft now guards against.
- Reusing singleton field names such as `missing_failure` for set insertion can
  freeze an accidental ABI or hide ordering / duplicate handling.
- Adding failures to `fails` declares possible failure behavior only. It does
  not supply capability, witness, route, membership, authorization, or runtime
  success.

## Relation to OBL-025

OBL-025 remains a single-edit Line-1 explanation-completeness target in canon.
The LAB OBL-025 draft now has abstract whole rejected-gap and set-insertion /
grouped multi-edit / partial-guidance predicates, but it does not admit
`ELAB-07` as covered executable evidence.

Safe reading:

- `ELAB-10` and `ELAB-13..16` are current singleton repair-bearing evidence.
- `ELAB-07` is pressure evidence for set insertion or grouped repair semantics.
- `ELAB-04` is separate pressure evidence for mixed visibility and
  non-visibility branch decomposition; `plan/98` keeps it no-repair until
  diagnostic ownership, branch association, and ordering / ranking are
  explicit.
- No current row proves OBL-025 or moves canon ledger status.

## Suggested next packages

1. Keep executable output unchanged and periodically validate that `ELAB-07`
   still omits `suggested_repair`.
2. If code widening is promoted, first write a narrow payload-model design for a
   single set-insertion item and its tests, then implement it in a separate
   package. `plan/99` records that payload-model preflight and `plan/100`
   accepts the narrow `ELAB-07` source-locus edit assumption. `plan/101`
   designs the first non-final set payload roles and test matrix, but
   executable output still remains no-repair.
3. Keep `ELAB-04` no-repair until visibility branch alternatives and ranking /
   association are explicit.
4. Draft OBL-024 only after diagnostic replay / association vocabulary is
   stable enough to avoid pulling in set/bundle edit semantics prematurely.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No set-insertion support.
- No bundle semantics support.
- No partial-guidance output support.
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
