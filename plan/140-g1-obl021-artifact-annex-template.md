# plan/140 - G1 OBL-021 artifact annex template

## Purpose

This file is LAB repository memory.

It defines a non-applied artifact annex template for a later OBL-021
conditional `lean-stated` requested-status packet. The template maps the canon
OBL-021 ledger target to the current LAB Lean statement-shape artifact, while
preserving the `plan/139` artifact-identity / wrapper preflight boundary and
the OBL-021 abstraction-boundary blocker from `plan/126`, `plan/130`, and
`plan/133`.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-021, does not prove OBL-021, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not change runtime, transport, Core IR, public API,
grammar, final equality, final diagnostic equivalence, final Diagnostic ABI,
projection-totality, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is a template for
future human/canon review, not a status authority.

## Inputs

Canon target:

```text
mirrorea_canon/theory/11-metatheory-ledger.md
OBL-021 / Elaboration determinism / MirCore.Elab.Det
```

Current LAB artifact candidate:

```text
samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft
```

Required prior LAB memory:

- `plan/76-g1-obl020-021-dependency-inventory.md`
- `plan/77-g1-obl021-lean-statement-draft.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`

## Annex status

The annex defined here is a fillable template.

It may be copied into a later draft packet only if that packet also records
fresh validation evidence and leaves unresolved decision slots explicit. A
later packet may leave slots unfilled, but it must not treat unfilled slots as
implicit acceptance.

For OBL-021, the central unfilled slot is not compile quality. The central
slot is whether human/canon review accepts the current abstract equivalence
boundary for statement status.

## Template: cover note

A later packet should start the OBL-021 artifact annex with this cover note,
filled with fresh command results and packet identifiers:

```text
OBL-021 artifact annex for requested-status review.

Canon target:
  OBL-021 / MirCore.Elab.Det

LAB artifact cited:
  samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
  MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft

Scope:
  Elaboration determinism statement identity, conditional on human/canon
  acceptance of the abstract result / diagnostic equivalence boundary. No proof,
  parser/checker implementation proof, runtime scheduling determinism,
  conformance, final equality selection, final Diagnostic ABI, projection-
  totality proof, or G1 exit is claimed.

Current canon ledger status:
  open, unless mirrorea_canon/theory/11-metatheory-ledger.md says otherwise.

Requested status:
  [UNRESOLVED: lean-stated is conditional on abstraction-boundary acceptance /
  stated / defer / another canon-allowed status]

Fresh validation evidence:
  [commands, timestamps, and results]

Decision requested:
  [direct LAB artifact accepted / wrapper required / artifact identity deferred]

Abstraction-boundary decision:
  [abstract equivalence boundary accepted / final equality and diagnostic
  equivalence required first / deferred]

Non-claim:
  This annex does not itself move the ledger, prove OBL-021, complete OBL-021,
  select final equality, freeze Diagnostic ABI, claim conformance, claim runtime
  scheduling determinism, or exit G1.
```

The `Current canon ledger status` line must cite the canon ledger directly at
the time of packet submission. The `Requested status` line must use only status
vocabulary accepted by canon. This file does not change that vocabulary and
does not treat LAB `plan/` mirrors as status authority.

## Template: artifact identity table

| Field | Required value / slot | Notes |
|---|---|---|
| Canon row | `OBL-021` | From `mirrorea_canon/theory/11-metatheory-ledger.md`. |
| Canon target | `MirCore.Elab.Det` | Ledger target remains open unless canon changes it. |
| Canon source anchor | `mirrorea_canon/theory/11-metatheory-ledger.md` | Later packet should cite the exact row. |
| LAB path | `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | Evidence path only until accepted. |
| LAB namespace | `MirCore.Lab.OBL021.StatementDraft` | LAB namespace is not canon by itself. |
| LAB constant | `OBL021StatementDraft` | Full name: `MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft`. |
| Statement shape | well-scoped fixed input implies `ElabDeterministicPost` | Abstract statement shape only. |
| Body links | `SameElabResult`, `SameDiagnostic`, `ElabDeterministicPost`, `OBL021StatementDraft` | Required sync-guard links. |
| Result-equivalence components | Core term, type, mode, effect row, failure row, constraints, obligations, generated edges, source spans | Abstract component equivalence only. |
| Diagnostic equivalence | `SameDiagnostic` backed by `EquivalentDiagnostic` | No final Diagnostic ABI or diagnostic equivalence contract. |
| Success/reject exclusion | Same fixed input cannot both successfully elaborate and reject | Statement-shape hook only; not implementation proof. |
| Requested status | `[UNRESOLVED]` | `lean-stated` is conditional and advisory, not accepted status. |
| Artifact decision | `[UNRESOLVED]` | Direct LAB artifact / wrapper required / deferred. |
| Abstraction-boundary decision | `[UNRESOLVED]` | Abstract equivalence accepted / final relations required / deferred. |
| Projection-totality decision | `[UNRESOLVED]` | The current draft does not prove projection-totality. |

## Template: validation evidence table

A later packet should fill this table with fresh results from the same work
package that submits the packet:

| Check | Command / evidence slot | Required result |
|---|---|---|
| Lean compile-check | `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | Pass. |
| LAB statement sync guard | `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` | Pass. |
| OBL-021 body-link guard | Sync guard evidence for `SameElabResult`, `SameDiagnostic`, `ElabDeterministicPost`, and `OBL021StatementDraft` links | Pass. |
| Component-equivalence guard | Sync guard evidence that result families, diagnostic equivalence, and success/reject exclusion remain linked | Pass. |
| Vacuity guard | Sync guard evidence that the body is not bare `True` / placeholder | Pass. |
| Admitted-stub scan | Packet-local scan for `axiom`, `constant`, `theorem`, `admit`, `sorry`, and placeholder theorem bodies in the cited artifact | No matches that affect the requested artifact. |
| Docs/source hierarchy validation | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py --format json` | Pass. |
| Secret scan | Tracked-file Discord webhook full URL / token-prefix scan excluding `.codex-discord` | Pass. |

Historical passes from `plan/132` may be cited as background, but they are not
fresh validation for a new packet.

## Template: sufficiency / non-sufficiency matrix

| Evidence | Supports | Does not support |
|---|---|---|
| LAB Lean compile-check | The abstract OBL-021 statement shape is expressible as Lean `Prop`. | Proof, implementation determinism, conformance, final equality, or ledger movement. |
| Body-link guard | The draft still mentions the intended determinism postcondition vocabulary. | Final implementation proof, parser/checker correctness, runtime scheduling determinism, or public ABI. |
| Result-equivalence component coverage | The statement names the result dimensions that must be compared for same-input success-success outcomes. | Final equality relation, projection-totality proof, or Core IR / JSON / API freeze. |
| Diagnostic equivalence hook | The statement has a place for same-input reject-reject diagnostic equivalence. | Final Diagnostic ABI, diagnostic equivalence contract, replay relation, or explanation soundness. |
| Success/reject exclusion | The statement includes mutual exclusion for same fixed input. | Proof that the real elaborator cannot both succeed and reject. |
| `plan/126` boundary audit | Current abstract boundary is sufficient for the current G1 bridge. | OBL-021 completion, proof, final equality, Diagnostic ABI, or runtime scheduling determinism. |
| `plan/133` requested-status matrix | OBL-021 is a conditional later `lean-stated` candidate if abstraction boundary is accepted. | Accepted status, ledger movement, or G1 exit. |
| `plan/139` wrapper preflight | Direct citation may remain LAB evidence while artifact identity and wrapper need are reviewed. | Canon acceptance of the LAB namespace or any wrapper. |

## Template: decision slots

A later packet must ask human/canon review to choose one artifact-identity
answer:

| Option | Meaning | Consequence |
|---|---|---|
| Direct LAB artifact accepted | The LAB path / namespace / constant may be cited as the requested-status artifact for OBL-021. | The packet may request conditional `lean-stated` using the current LAB artifact, with explicit non-claims. |
| Wrapper required | A canon-facing non-applied wrapper or renamed statement target is required before status request. | Open a wrapper package with `plan/139` constraints before requesting status. |
| Artifact identity deferred | Neither direct LAB artifact nor wrapper should be used until final equality, diagnostic equivalence, projection-totality, or proof-boundary choices bind. | Defer OBL-021 requested-status work to a later boundary package. |

It must also ask whether the abstraction boundary is accepted:

| Boundary decision | Meaning | Consequence |
|---|---|---|
| Abstract equivalence accepted | `SameElabResult`, `SameDiagnostic`, and component-equivalence predicates are acceptable statement-status vocabulary at this checkpoint. | Conditional `lean-stated` may be requested, still without proof or conformance. |
| Final equality required first | The status packet must choose concrete equality / equivalence relations before status request. | Open a statement-refinement package before requesting OBL-021 status. |
| Diagnostic equivalence required first | The status packet must choose a final Diagnostic ABI or diagnostic equivalence contract before status request. | Defer OBL-021 status request until diagnostic boundary work binds. |
| Projection-totality required first | The packet must prove or state totality/uniqueness for result projections before status request. | Defer OBL-021 status request or refine statement shape. |
| Boundary deferred | Human/canon review declines to accept or reject the abstraction boundary now. | Keep OBL-021 status movement deferred. |

## Template: unresolved items

The annex must preserve these unresolved items unless a later canon decision
explicitly resolves them:

| Item | Current template reading |
|---|---|
| Accepted artifact identity | UNRESOLVED. |
| Wrapper requirement | UNRESOLVED. |
| Abstract equivalence boundary acceptance | UNRESOLVED. |
| Final result equality relation | UNRESOLVED. |
| Final diagnostic equivalence contract | UNRESOLVED. |
| Final Diagnostic ABI | UNRESOLVED. |
| Projection-totality / projection uniqueness | UNRESOLVED. |
| Parser/checker implementation proof | UNRESOLVED. |
| Runtime scheduling determinism | Out of scope for OBL-021 statement status. |
| Concrete implementation satisfaction proof | UNRESOLVED. |
| OBL-021 proof skeleton / proof discharge | UNRESOLVED. |
| OBL-001 / OBL-002 soundness proof | UNRESOLVED and separate. |
| OBL-020 well-formedness preservation | UNRESOLVED and separate. |
| OBL-024 / OBL-025 diagnostic / repair proof | UNRESOLVED and separate. |
| C-static / C-runtime / C-distributed conformance | UNRESOLVED. |
| G1 exit | UNRESOLVED and separate from artifact review. |

## Required non-claims

Any later packet using this annex must include these non-claims:

- No canon edit by the annex itself.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted by LAB evidence alone.
- No proposal submission by this template alone.
- No metatheory ledger movement.
- No OBL-021 completion unless canon explicitly accepts it.
- No OBL-021 proof skeleton completion.
- No OBL-021 proof discharge.
- No OBL-001 / OBL-002 proof claim.
- No OBL-020 proof claim.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No final equality relation.
- No final diagnostic equivalence contract.
- No final Diagnostic ABI.
- No projection-totality proof.
- No parser/checker implementation proof.
- No runtime scheduling determinism claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file unless a separate wrapper package creates one.
- No Lean predicate refinement unless a separate refinement package creates one.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, or assignment taxonomy freeze.
- No sample status relabel.

## Drift checks for later use

Before a later packet copies this annex, it should recheck:

1. the canon ledger row still names `OBL-021 / MirCore.Elab.Det`;
2. the LAB file path, namespace, and constant still exist;
3. the statement body still links `SameElabResult`, `SameDiagnostic`,
   `ElabDeterministicPost`, and `OBL021StatementDraft`;
4. result component equivalence still covers Core term, type, mode, effect row,
   failure row, constraints, obligations, generated edges, and source spans;
5. diagnostic equivalence and success/reject exclusion remain explicit;
6. `plan/126` has not been superseded by a statement refinement that chooses
   final equality, projection-totality, or diagnostic equivalence;
7. `plan/139` has not been superseded by an accepted wrapper decision;
8. docs validators still register the relevant plan / report files;
9. no fresh canon decision has changed the allowed status vocabulary.

If any of these checks fail, the later packet must update the annex instead of
copying it unchanged.

## How to use this template

Use this file as a checklist and copy source for a later proposal draft.

Do not treat the existence of this template as:

- artifact identity acceptance;
- a request to move OBL-021 status;
- evidence that a wrapper is unnecessary;
- evidence that a wrapper is required;
- evidence that OBL-021 is already complete;
- evidence that OBL-021 proof work has started;
- evidence that final equality, diagnostic equivalence, Diagnostic ABI, or
  projection-totality is resolved;
- evidence that runtime scheduling determinism or implementation determinism is
  proved.

A later packet may fill only the conditional `lean-stated` candidate path. If
it does, it must explicitly say that `lean-stated` is requested status, not
accepted status, until the canon process accepts it.

## Next allowed moves

Reasonable next packages are:

1. prepare a draft G1 status packet shell that references the OBL-001 /
   OBL-020 annex templates and this OBL-021 annex template, while leaving
   requested statuses and ledger deltas unresolved;
2. prepare an OBL-021 equality / diagnostic abstraction decision packet if the
   project wants to ask human/canon review about the central blocker before any
   status shell;
3. create a wrapper package only if human/canon review says direct LAB artifact
   citation is not acceptable for the scoped requested-status artifact.
