# plan/74 - G1 OBL-001 repo-local Lean statement draft

## Purpose

This file records the first repo-local Lean-checked statement-shape draft for
THM-001 / OBL-001 ordinary assignment elaboration soundness.

This is LAB repository memory. It does not change canon, does not edit
`mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim OBL-001
completion, proof discharge, G1 exit, conformance, runtime dispatch, or final
public API / grammar freeze.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB statement artifact:
  `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- LAB explanation:
  `samples/lean/lab-statements/obl001/THM001StatementDraft.md`
- LAB manifest:
  `samples/lean/manifest.json`
- LAB guard hardening:
  `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- Planning predecessor: `plan/73-g1-obl001-lean-statement-inventory.md`

If this LAB statement conflicts with canon, canon wins.

## What was added

`samples/lean/lab-statements/obl001/THM001StatementDraft.lean` introduces a
LAB-only namespace:

```text
MirCore.Lab.OBL001.StatementDraft
```

The file defines abstract carriers and predicates:

- `Vocab`: abstract types for environment, context, locus, assignment, result,
  write, and request;
- `Pred`: abstract proposition fields for Surface assignment, simple
  assignment, elaboration, generated writes, owner-local write, owner-directed
  request evidence, RHS read dependency recording, failure containment,
  authority-obligation representation, span preservation, visible publish /
  observe consequence, and nested-locus non-authority;
- `RequestEvidenceSound`, `GeneratedWriteSound`,
  `AllGeneratedWritesSound`, and `AssignmentElabSoundnessPost`;
- `THM001StatementDraft`, a `Prop` definition tying successful simple
  assignment elaboration to the postcondition.

## Lean reading

`THM001StatementDraft` is a `Prop` definition. It is intentionally not a proved
`theorem`.

This keeps the statement shape machine-checked while avoiding all of the
following:

- `axiom`;
- `constant`;
- `sorry`;
- a false proof claim;
- a final `MirCore.Elab.Soundness` namespace claim;
- an accidental canon OBL status movement.

The elaboration judgment and semantic vocabulary are carried by abstract
predicate fields. The LAB file does not define final Surface-to-Core semantics.

## Predicate boundary

| Predicate group | Current statement reading | Explicit non-claim |
|---|---|---|
| generated writes | every generated write is owner-local or backed by owner-directed request evidence | no request-serving runtime semantics |
| RHS reads | RHS read consequences are represented abstractly | no final OPEN-014 materialization policy |
| failures | generated failures are contained in declared failures | no final diagnostic ID contract |
| authority | required authority obligations are represented as carriers | no grant-lineage proof |
| spans | source spans are preserved as proposition-level evidence | no final JSON source-map ABI |
| visible consequences | visible writes require explicit publish / observe consequence evidence | no telemetry/viewer ABI |
| nested locus | nested foreign locus syntax must not create ambient owner-local authority | no full authority theorem |

## SCN pressure carried forward

The statement draft is designed to carry the pressure identified in `plan/72`
and `plan/73`:

- SCN-01 owner-directed write, same-field RHS dependency, visible publish /
  observe consequence, failure containment, capability-obligation carrier, and
  source span preservation;
- SCN-02 owner-directed write, target/self RHS dependency, failure containment,
  and nested-locus non-authority.

The current Lean file keeps those requirements abstract. It does not add exact
LAB expected JSON rows for SCN-01 same-field RHS dependency or SCN-02 two-read
RHS dependency.

## Status

- Lean file exists and compiles locally.
- `samples/lean/manifest.json` records the new `statement_drafts` entry and
  successful verification.
- `scripts/current_l2_lean_sample_sync.py` now includes a separate
  `statement_drafts` category, rather than overloading `foundations` or
  `clean_near_end`.
- `scripts/tests/test_current_l2_lean_sample_sync.py` checks that the OBL-001
  LAB draft remains registered with its explanation file.
- `plan/117` now hardens the sync unit guard so `RequestEvidenceSound`,
  `GeneratedWriteSound`, `AssignmentElabSoundnessPost`, and
  `THM001StatementDraft` retain their body-level links. This is still a
  compile-check-only statement guard, not a proof skeleton or runtime dispatch.

## Open questions

- Should the next package add exact LAB evidence rows for the SCN-01 / SCN-02
  RHS dependency gaps before tightening the Lean statement further?
- Should OBL-020 well-formedness preservation and OBL-021 determinism be stated
  as separate Lean `Prop` drafts before any proof-oriented OBL-002 work?
- Should the authority predicates later split capability and witness
  obligations into separate carriers, or is a single owner-directed request
  evidence group sufficient for the first OBL-001 statement iteration?

## Next safe packages

The first two items below were actualized as LAB evidence/memory in
`plan/75-g1-scn-rhs-dependency-gap-evidence.md` and
`plan/76-g1-obl020-021-dependency-inventory.md`.

1. OBL-020/021 dependency inventory and possible Lean statement-shape drafts,
   kept separate from OBL-001 proof work.
2. Narrow refinement of `THM001StatementDraft.lean` if review finds a
   statement-shape overfit or missing predicate, still without canon status
   movement.
3. Separate OBL-020 / OBL-021 LAB Lean statement-shape drafts, only if useful.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No OBL status movement in canon.
- No Lean proof completion.
- No theorem discharge.
- No OBL-020 / OBL-021 completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No final grammar, final Core IR JSON, public API, runtime, transport,
  projection, devtools, telemetry, provider, or product completion.
