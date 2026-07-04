# plan/136 - G1 OBL-020 artifact annex template

## Purpose

This file is LAB repository memory.

It defines a non-applied artifact annex template for a later OBL-020
requested-status packet. The template maps the canon OBL-020 ledger target to
the current LAB Lean statement-shape artifact, while preserving the
`plan/134` scope clarification and the `plan/135` artifact-identity / wrapper
preflight boundary.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-020, does not prove OBL-020, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not change runtime, transport, Core IR, public API,
grammar, or sample status.

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
OBL-020 / Well-formedness preservation of step rules / MirCore.Step.WF
```

Current LAB artifact candidate:

```text
samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft
```

Required prior LAB memory:

- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/134-g1-obl020-scope-clarification-packet.md`
- `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`

## Annex status

The annex defined here is a fillable template.

It may be copied into a later draft packet only if that packet also records
fresh validation evidence and leaves unresolved decision slots explicit. A
later packet may leave slots unfilled, but it must not treat unfilled slots as
implicit acceptance.

## Template: cover note

A later packet should start the OBL-020 artifact annex with this cover note,
filled with fresh command results and packet identifiers:

```text
OBL-020 artifact annex for requested-status review.

Canon target:
  OBL-020 / MirCore.Step.WF

LAB artifact cited:
  samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
  MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft

Scope:
  G1-supporting statement-shape evidence only, unless human/canon review
  explicitly accepts a broader OBL-020 status scope.

Current canon ledger status:
  open, unless mirrorea_canon/theory/11-metatheory-ledger.md says otherwise.

Requested status:
  [UNRESOLVED: stated / lean-stated / another canon-allowed status]

Fresh validation evidence:
  [commands, timestamps, and results]

Decision requested:
  [direct LAB artifact accepted / wrapper required / artifact identity deferred]

Non-claim:
  This annex does not itself move the ledger, prove OBL-020, complete OBL-020,
  claim conformance, or exit G1.
```

The `Current canon ledger status` line must cite the canon ledger directly at
the time of packet submission. The `Requested status` line must use only status
vocabulary accepted by canon. This file does not change that vocabulary and
does not treat LAB `plan/` mirrors as status authority.

## Template: artifact identity table

| Field | Required value / slot | Notes |
|---|---|---|
| Canon row | `OBL-020` | From `mirrorea_canon/theory/11-metatheory-ledger.md`. |
| Canon target | `MirCore.Step.WF` | Ledger target remains open unless canon changes it. |
| Canon source anchor | `mirrorea_canon/theory/11-metatheory-ledger.md` | Later packet should cite the exact row. |
| LAB path | `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | Evidence path only until accepted. |
| LAB namespace | `MirCore.Lab.OBL020.StatementDraft` | LAB namespace is not canon by itself. |
| LAB constant | `OBL020StatementDraft` | Full name: `MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft`. |
| Statement shape | `forall before label after, PreservesWF P before label after` | Abstract statement shape only. |
| Body links | `WellFormed`, `Step`, `PreservesWF` | Required sync-guard links. |
| Family helper | `FamilyStepPreservesWF` | Supporting helper; not the final step-family proof boundary. |
| Requested status | `[UNRESOLVED]` | Must not be silently filled by the template. |
| Artifact decision | `[UNRESOLVED]` | Direct LAB artifact / wrapper required / deferred. |
| Scope decision | `[UNRESOLVED]` | G1-supporting scope vs full-row status movement. |

## Template: validation evidence table

A later packet should fill this table with fresh results from the same work
package that submits the packet:

| Check | Command / evidence slot | Required result |
|---|---|---|
| Lean compile-check | `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | Pass. |
| LAB statement sync guard | `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` | Pass. |
| OBL-020 body-link guard | Sync guard evidence for `WellFormed`, `Step`, and `PreservesWF` links | Pass. |
| Vacuity guard | Sync guard evidence that the body is not bare `True` / placeholder | Pass. |
| Admitted-stub scan | Packet-local scan for `axiom`, `constant`, `theorem`, `admit`, `sorry`, and placeholder theorem bodies in the cited artifact | No matches that affect the requested artifact. |
| Docs/source hierarchy validation | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py --format json` | Pass. |
| Secret scan | Tracked-file Discord webhook full URL / token-prefix scan excluding `.codex-discord` | Pass. |

Historical passes from `plan/132` may be cited as background, but they are not
fresh validation for a new packet.

## Template: sufficiency / non-sufficiency matrix

| Evidence | Supports | Does not support |
|---|---|---|
| LAB Lean compile-check | The abstract OBL-020 statement shape is expressible as Lean `Prop`. | Proof, concrete runtime WF, conformance, or ledger movement. |
| Body-link guard | The draft still mentions the intended abstract preservation vocabulary. | Final concrete `WellFormed`, `Step`, scheduler, or step-family semantics. |
| `plan/134` scope clarification | A G1-supporting statement-scope review is separable from full-row OBL-020 movement. | Full OBL-020 status acceptance. |
| `plan/135` wrapper preflight | Direct citation may remain LAB evidence while artifact identity is reviewed. | Canon acceptance of the LAB namespace or any wrapper. |

## Template: decision slots

A later packet must ask human/canon review to choose one artifact-identity
answer:

| Option | Meaning | Consequence |
|---|---|---|
| Direct LAB artifact accepted | The LAB path / namespace / constant may be cited as the requested-status artifact for the scoped packet. | The packet may request status using the current LAB artifact, with explicit non-claims. |
| Wrapper required | A canon-facing non-applied wrapper or renamed statement target is required before status request. | Open a wrapper package with `plan/135` constraints before requesting status. |
| Artifact identity deferred | Neither direct LAB artifact nor wrapper should be used until concrete definitions bind. | Defer OBL-020 requested-status work to a later proof / concrete-boundary package. |

It must also ask whether the scope is:

| Scope | Meaning | Consequence |
|---|---|---|
| G1-supporting statement scope | Accept only that the abstract preservation statement shape is suitable evidence for G1 discussion. | No full-row OBL-020 ledger movement. |
| Full-row OBL-020 status movement | Treat the artifact as sufficient for the OBL-020 ledger row requested status. | Requires stronger canon acceptance and should remain deferred by default. |
| Proof-package fallback | Defer any status request until a later proof-oriented package binds concrete definitions. | No status request in the current packet. |

## Template: unresolved items

The annex must preserve these unresolved items unless a later canon decision
explicitly resolves them:

| Item | Current template reading |
|---|---|
| Concrete `Config` type | UNRESOLVED. |
| Concrete `StepLabel` type | UNRESOLVED. |
| Concrete `StepFamily` type | UNRESOLVED. |
| Concrete `WellFormed` clauses | UNRESOLVED. |
| Concrete `Step` relation / scheduler semantics | UNRESOLVED. |
| Per-step-family preservation lemmas | UNRESOLVED. |
| Proof skeleton / proof discharge | UNRESOLVED. |
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
- No full-row OBL-020 completion unless canon explicitly accepts it.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file unless a separate wrapper package creates one.
- No Lean predicate refinement unless a separate refinement package creates one.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.

## Drift checks for later use

Before a later packet copies this annex, it should recheck:

1. the canon ledger row still names `OBL-020 / MirCore.Step.WF`;
2. the LAB file path, namespace, and constant still exist;
3. the statement body still links `WellFormed`, `Step`, and `PreservesWF`;
4. `plan/134` still keeps full-row OBL-020 status movement deferred;
5. `plan/135` has not been superseded by an accepted wrapper decision;
6. docs validators still register the relevant plan / report files;
7. no fresh canon decision has changed the allowed status vocabulary.

If any of these checks fail, the later packet must update the annex instead of
copying it unchanged.

## How to use this template

Use this file as a checklist and copy source for a later proposal draft.

Do not treat the existence of this template as:

- artifact identity acceptance;
- a request to move OBL-020 status;
- evidence that a wrapper is unnecessary;
- evidence that a wrapper is required;
- evidence that full-row OBL-020 movement is now safe.

A later packet may fill only the G1-supporting statement-scope version. If it
does, it should explicitly say full-row OBL-020 movement remains deferred.

## Next allowed moves

Reasonable next packages are:

1. prepare OBL-001 artifact identity / wrapper preflight, because OBL-001
   remains the strongest later `lean-stated` candidate;
2. prepare a draft G1 status packet shell that uses this annex template but
   leaves requested statuses and ledger deltas unresolved;
3. open a wrapper package only if human/canon review says direct LAB artifact
   citation is not acceptable for the scoped requested-status artifact.
