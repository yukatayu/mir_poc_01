# plan/95 - G1 E-ROW mixed / multi repair decomposition inventory

## Purpose

This file records the LAB-only decomposition inventory for mixed and
multi-missing E-ROW failure-row containment cases.

At the time, it kept `ELAB-04` and `ELAB-07` as no-repair evidence. `plan/102`
later implemented one exact non-final `ELAB-07` set payload. `ELAB-04` remains
no-repair. This inventory still does not edit canon, freeze a Diagnostic or
repair ABI, prove OBL-024/025, claim explanation soundness or completeness,
claim repair ranking or multi-edit support, claim conformance, or claim G1
exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- LAB singleton repair gate and prototype:
  `plan/93-g1-erow001-singleton-repair-assumption.md`
  and `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB set-insertion / bundle payload vocabulary:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-07` set-insertion gate review:
  `plan/97-g1-erow07-set-insertion-gate-review.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`
- Advisory review:
  ChatGPT Pro Oracle consult `we-are-working-in-a`

If this LAB inventory conflicts with canon, canon wins.

## Current executable state

The current repair-bearing executable classes are:

| Sample | Shape | Repair output |
|---|---|---|
| `ELAB-10` | `E-ROW-002` / `VisibilityDenied` singleton | one LAB-only `add-to-fails-row` item |
| `ELAB-13..16` | `E-ROW-001` non-visibility singleton | one LAB-only `add-to-fails-row` item per row |
| `ELAB-07` | exact non-visibility multi-missing `E-ROW-001` | one non-final LAB-only `set_insertion` item under `plan/102` |

The current mixed / unresolved multi-missing fence remains:

| Sample | Shape | Missing failures | Repair output |
|---|---|---|---|
| `ELAB-04` | mixed visibility / non-visibility multi-missing | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` | no `suggested_repair` field |

The implementation preserves the singleton guard for singleton repair output
and adds a separate exact `ELAB-07` set path. It does not make mixed rows or
arbitrary multi-missing rows repair-bearing.

For `ELAB-04`, the carrier should continue to omit `suggested_repair`. This
package does not standardize empty `suggested_repair: []` semantics.

## Policy for `ELAB-07`

This package originally kept `ELAB-07` no-repair. After `plan/100..102`, the
current policy is exact-only set insertion for the one known `ELAB-07` fact
pattern, not general multi-missing support.

Reason: each missing base failure is individually singleton-shaped, but adding
only one of the missing failures does not discharge the reported local premise:

```text
generated_failures_subset_declared_fails
```

Emitting three independent singleton repairs would therefore be misleading
unless the payload language can say that they are a conjunctive repair bundle
or unless the system separately defines each item as a partial repair. Neither
semantics exists today.

Before `plan/102`, widening for `ELAB-07` needed an explicit decision between:

1. one set-insertion repair item that adds all missing base failures and is
   treated as one source edit;
2. a conjunctive bundle of several atom-addition edits that must all be
   applied together;
3. several independent suggestions marked as partial repairs, not
   `RepairDischargesLocalPremise` witnesses;
4. staying no-repair until multi-edit support exists.

Only options 1 or 2 could plausibly serve as a local premise-discharging repair
for OBL-025-shaped coverage. `plan/102` took option 1 for the exact current
row only. Option 3 is useful guidance but not a complete repair witness for the
row-containment premise.

## Policy for `ELAB-04`

`ELAB-04` should remain no-repair today.

Reason: it is not just multi-missing. It mixes base remote-request failures
with `VisibilityDenied`, whose singleton path is classified as `E-ROW-002` and
has separate visibility / observe-authority alternatives. Treating
`VisibilityDenied` as merely another base failure would collapse the
visibility / authority distinction.

Future widening for `ELAB-04` needs an explicit decision between:

1. one mixed set-insertion repair item that adds all missing failures to the
   same `when ... fails` row;
2. separate non-visibility and visibility repair families with ordering /
   ranking;
3. a conjunctive bundle that contains a base-failure set repair plus a
   visibility repair;
4. no repair until alternative visibility repairs and ranking are defined.

The current safe reading is option 4.

## Minimum future-widening axes

Before any mixed / multi-missing row emits `suggested_repair[]`, a later
package must document at least the following axes.

| Axis | Required decision |
|---|---|
| Edit atom | Is adding several failures to one `fails` row one source edit, several edits, or a separate set-insertion family? |
| Repair completeness | Does each suggested item independently discharge the local premise, or can a suggestion be a partial item? |
| Bundle semantics | If multiple items are required together, how does the payload mark `all_of` / conjunctive repair rather than alternatives? |
| Ordering and ranking | If several repairs or families are possible, are they ordered, ranked, or left unordered? |
| Visibility split | Does `VisibilityDenied` stay `E-ROW-002`-like inside mixed rows, or is there a mixed-family wrapper? |
| Alternative visibility repairs | Can a visibility omission be repaired by declaration / observe authority instead of adding `VisibilityDenied` to `fails`? |
| Diagnostic association | Which diagnostic owns a repair when one generated request has mixed missing failures or when a source item creates multiple failing requests? |
| Target policy | Is there exactly one concrete `when_fails_row` target, and what happens if the target is ambiguous or non-row? |
| Local effect | What is the exact `declared_failures_after` for set or bundled repairs, including duplicate handling and ordering? |
| Non-goal wording | The payload must still deny runtime success, authorization, capability / witness / route / membership availability, and whole-program acceptance. |
| OBL-025 relation | Which shapes are single-edit repair witnesses, multi-edit witnesses, or guidance-only payloads? |
| Empty-list semantics | Should no-repair rows omit `suggested_repair`, or does an empty array carry standardized meaning? |
| Coincident diagnostics | Can a row repair be emitted when independent non-row diagnostics are present, or would that imply the whole rejection is repaired? |

## Hidden failure modes

- Emitting one item per missing failure can produce partial repairs that do not
  make `required_failures` a subset of `declared_failures_after`.
- Calling a partial item a `RepairDischargesLocalPremise` witness would
  overstate OBL-025 coverage.
- Treating a list of repairs as alternatives when all must be applied together
  would make the payload semantically false.
- Treating `VisibilityDenied` as a base remote-request failure would erase the
  distinction between visibility policy and capability / witness / route /
  membership failure surfaces.
- Emitting a mixed repair under only `E-ROW-001` can hide the `E-ROW-002`
  visibility-specific branch; emitting both without association can create
  duplicate or conflicting diagnostics.
- Standardizing `suggested_repair: []` accidentally can create a public ABI
  signal before the LAB carrier is ready.
- Emitting row repairs while independent non-row diagnostics are also present
  can imply that the whole rejection is solved by the row edit.
- Adding failures to `fails` only declares explicit failure behavior. It does
  not authorize communication, supply capability / witness / route /
  membership evidence, or make the program accepted.
- A future final ABI may need target spans, use spans, and edit scripts rather
  than the current LAB-local `target_ref`.

## Current no-repair guard

The current guard is intentionally simple:

```text
emit repair only when missing_failures.len() == 1
```

This guard is not a final theorem. It is a LAB safety rule that prevents
placeholder or partial repair payloads from satisfying OBL-025-shaped checks.
`plan/96` inventories candidate vocabulary for set insertion, conjunctive
bundles, and partial guidance. `plan/97` reviews the `ELAB-07` gate and keeps
that row no-repair until set-insertion atomicity or bundle semantics are
explicit.

The guard should remain until a later package provides:

- a set-insertion or bundle vocabulary;
- tests proving `ELAB-04/07` still reject placeholder payloads;
- expected JSON that makes bundle / ranking / partiality explicit;
- repository memory explaining why the new payload realizes a local repair
  witness rather than only human guidance.

## Relation to OBL-025

`plan/87` and `plan/94` provide executable singleton evidence for a narrow
candidate `CoveredLine1RepairCase`. This inventory says that mixed /
multi-missing cases are not yet covered by that singleton case.

Safe reading:

- `ELAB-10` and `ELAB-13..16` are singleton repair-bearing evidence.
- `ELAB-07` is pressure evidence for set insertion / bundle semantics.
- `ELAB-04` is pressure evidence for mixed-family decomposition and visibility
  repair alternatives.
- Neither `ELAB-04` nor `ELAB-07` is evidence of OBL-025 completion.

## Relation to `plan/96`

`plan/96-g1-erow-set-insertion-bundle-payload-inventory.md` provides a
docs-only candidate vocabulary for the future choices listed here:

- set insertion as one grouped row edit;
- conjunctive repair bundles with `all_required` semantics;
- partial guidance that does not discharge the local premise;
- mixed visibility / non-visibility branch separation.

Later packages changed the `ELAB-07` executable policy only for the exact
`plan/102` set path. `ELAB-04` still omits `suggested_repair`, and singleton
repair output still requires exactly one missing failure.

`plan/97..102` record the `ELAB-07` gate, source-locus edit assumption,
payload design, and exact executable set path. `plan/98` separately records
that `ELAB-04` remains no-repair because the row is both multi-missing and
mixed across base remote-request failures plus a `VisibilityDenied` branch;
future widening needs diagnostic ownership, branch association, and ordering /
ranking before any payload change.

## Suggested next packages

1. Keep `ELAB-04` no-repair and validate that it still omits
   `suggested_repair`; keep `ELAB-07` restricted to the exact `plan/102` set
   path.
2. If widening is desired later, promote a narrow assumption from `plan/96`
   before editing Rust output.
3. Refine OBL-025 only after deciding whether multi-missing repair witnesses
   are single-edit, multi-edit, or outside the first coverage fragment.
4. Draft OBL-024 explanation-soundness only if replay / association vocabulary
   is stable enough to avoid pulling in repair ranking or edit semantics.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No set-insertion support.
- No bundle semantics.
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
