# plan/138 - G1 OBL-001 artifact annex template

## Purpose

This file is LAB repository memory.

It defines a non-applied artifact annex template for a later OBL-001 /
THM-001 requested-status packet. The template maps the canon OBL-001 ledger
target to the current LAB Lean statement-shape artifact, while preserving the
`plan/137` artifact-identity / wrapper preflight boundary.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001, does not prove OBL-002, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, does not resolve OPEN-014, and does not change runtime, transport,
Core IR, public API, grammar, diagnostic / repair ABI, or sample status.

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
OBL-001 / THM-001 Lean statement / MirCore.Elab.Soundness (stmt)
```

Current LAB artifact candidate:

```text
samples/lean/lab-statements/obl001/THM001StatementDraft.lean
MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft
```

Required prior LAB memory:

- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `plan/124-g1-obl001-boundary-audit.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md`

## Annex status

The annex defined here is a fillable template.

It may be copied into a later draft packet only if that packet also records
fresh validation evidence and leaves unresolved decision slots explicit. A
later packet may leave slots unfilled, but it must not treat unfilled slots as
implicit acceptance.

## Template: cover note

A later packet should start the OBL-001 artifact annex with this cover note,
filled with fresh command results and packet identifiers:

```text
OBL-001 artifact annex for requested-status review.

Canon target:
  OBL-001 / THM-001 Lean statement / MirCore.Elab.Soundness (stmt)

LAB artifact cited:
  samples/lean/lab-statements/obl001/THM001StatementDraft.lean
  MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft

Scope:
  THM-001 / OBL-001 statement identity for ordinary-assignment elaboration
  soundness. No OBL-002 proof, runtime conformance, OPEN-014 resolution, or G1
  exit is claimed.

Current canon ledger status:
  open, unless mirrorea_canon/theory/11-metatheory-ledger.md says otherwise.

Requested status:
  [UNRESOLVED: lean-stated is the advisory first candidate / stated /
  another canon-allowed status]

Fresh validation evidence:
  [commands, timestamps, and results]

Decision requested:
  [direct LAB artifact accepted / wrapper required / artifact identity deferred]

Non-claim:
  This annex does not itself move the ledger, prove OBL-002, complete OBL-001,
  claim conformance, resolve OPEN-014, or exit G1.
```

The `Current canon ledger status` line must cite the canon ledger directly at
the time of packet submission. The `Requested status` line must use only status
vocabulary accepted by canon. This file does not change that vocabulary and
does not treat LAB `plan/` mirrors as status authority.

## Template: artifact identity table

| Field | Required value / slot | Notes |
|---|---|---|
| Canon row | `OBL-001` | From `mirrorea_canon/theory/11-metatheory-ledger.md`. |
| Canon theorem | `THM-001` / assignment elaboration soundness | Ledger theorem row remains open unless canon changes it. |
| Canon target | `MirCore.Elab.Soundness (stmt)` | Ledger target remains open unless canon changes it. |
| Canon source anchor | `mirrorea_canon/theory/11-metatheory-ledger.md` | Later packet should cite the exact row. |
| LAB path | `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | Evidence path only until accepted. |
| LAB namespace | `MirCore.Lab.OBL001.StatementDraft` | LAB namespace is not canon by itself. |
| LAB constant | `THM001StatementDraft` | Full name: `MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft`. |
| Statement shape | successful simple assignment elaboration implies `AssignmentElabSoundnessPost` | Abstract statement shape only. |
| Body links | `RequestEvidenceSound`, `GeneratedWriteSound`, `AssignmentElabSoundnessPost`, `THM001StatementDraft` | Required sync-guard links. |
| Postcondition links | generated writes, RHS reads, failure containment, authority obligations, source spans, visible consequences, nested-locus non-authority | Required semantic hook coverage. |
| Requested status | `[UNRESOLVED]` | `lean-stated` is the advisory first candidate, not accepted status. |
| Artifact decision | `[UNRESOLVED]` | Direct LAB artifact / wrapper required / deferred. |
| OPEN-014 decision | `[UNRESOLVED]` | Defer or separately resolve read materialization. |
| Assignment-scope decision | `[UNRESOLVED]` | Simple ordinary assignment now; compound assignment handling remains explicit. |

## Template: validation evidence table

A later packet should fill this table with fresh results from the same work
package that submits the packet:

| Check | Command / evidence slot | Required result |
|---|---|---|
| Lean compile-check | `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | Pass. |
| LAB statement sync guard | `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` | Pass. |
| OBL-001 body-link guard | Sync guard evidence for `RequestEvidenceSound`, `GeneratedWriteSound`, `AssignmentElabSoundnessPost`, and `THM001StatementDraft` links | Pass. |
| Postcondition coverage guard | Sync guard evidence for request evidence, generated-write coverage, RHS dependency recording, generated-failure containment, authority obligations, source-span evidence, visible consequences, and nested-locus non-authority | Pass. |
| Vacuity guard | Sync guard evidence that the body is not bare `True` / placeholder | Pass. |
| Admitted-stub scan | Packet-local scan for `axiom`, `constant`, `theorem`, `admit`, `sorry`, and placeholder theorem bodies in the cited artifact | No matches that affect the requested artifact. |
| Docs/source hierarchy validation | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py --format json` | Pass. |
| Secret scan | Tracked-file Discord webhook full URL / token-prefix scan excluding `.codex-discord` | Pass. |

Historical passes from `plan/132` may be cited as background, but they are not
fresh validation for a new packet.

## Template: sufficiency / non-sufficiency matrix

| Evidence | Supports | Does not support |
|---|---|---|
| LAB Lean compile-check | The abstract THM-001 / OBL-001 statement shape is expressible as Lean `Prop`. | Proof, implementation satisfaction, conformance, or ledger movement. |
| Body-link guard | The draft still mentions the intended abstract request / write / postcondition vocabulary. | Final implementation proof, runtime dispatch, or public ABI. |
| `plan/124` boundary audit | Existing abstract predicates can carry `ELAB-11`, `ELAB-12`, and `ELAB-17` pressure without Lean refinement. | OBL-001 completion, diagnostic / repair proof, or conformance. |
| `plan/133` requested-status matrix | OBL-001 is the strongest later `lean-stated` candidate. | Accepted status, ledger movement, or G1 exit. |
| `plan/137` wrapper preflight | Direct citation may remain LAB evidence while artifact identity is reviewed. | Canon acceptance of the LAB namespace or any wrapper. |

## Template: decision slots

A later packet must ask human/canon review to choose one artifact-identity
answer:

| Option | Meaning | Consequence |
|---|---|---|
| Direct LAB artifact accepted | The LAB path / namespace / constant may be cited as the requested-status artifact for OBL-001. | The packet may request `lean-stated` using the current LAB artifact, with explicit non-claims. |
| Wrapper required | A canon-facing non-applied wrapper or renamed statement target is required before status request. | Open a wrapper package with `plan/137` constraints before requesting status. |
| Artifact identity deferred | Neither direct LAB artifact nor wrapper should be used until OPEN-014, assignment scope, or proof-boundary choices bind. | Defer OBL-001 requested-status work to a later boundary package. |

It must also ask how OPEN-014 and assignment scope are handled:

| Decision | Meaning | Consequence |
|---|---|---|
| OPEN-014 deferred | Read materialization, cache, freshness, transport, projection, and observe-vs-read-request policy remain out of the status request. | Safe default for static OBL-001 statement identity. |
| OPEN-014 promoted | A separate canon decision resolves read materialization before relying on it. | Requires separate canon work before status request. |
| Simple assignment accepted | Current `SimpleAssign` scope is sufficient for OBL-001 statement identity at this checkpoint. | Compound assignment remains deferred. |
| Compound assignment required | The statement must include or explicitly factor compound assignment before status request. | Open a separate statement refinement package. |

## Template: unresolved items

The annex must preserve these unresolved items unless a later canon decision
explicitly resolves them:

| Item | Current template reading |
|---|---|
| Accepted artifact identity | UNRESOLVED. |
| Wrapper requirement | UNRESOLVED. |
| OPEN-014 read materialization | UNRESOLVED / deferred by default. |
| Compound assignment coverage | UNRESOLVED / deferred by default. |
| Concrete implementation satisfaction proof | UNRESOLVED and not part of OBL-001 statement identity. |
| OBL-002 proof skeleton / proof discharge | UNRESOLVED. |
| OBL-020 well-formedness preservation | UNRESOLVED and separate. |
| OBL-021 determinism | UNRESOLVED and separate. |
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
- No OBL-001 completion unless canon explicitly accepts it.
- No OBL-002 proof skeleton completion.
- No OBL-002 proof discharge.
- No OBL-020 / OBL-021 completion.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file unless a separate wrapper package creates one.
- No Lean predicate refinement unless a separate refinement package creates one.
- No OPEN-014 resolution unless separately promoted.
- No G3 / THM-004 authority proof.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, or assignment taxonomy freeze.
- No sample status relabel.

## Drift checks for later use

Before a later packet copies this annex, it should recheck:

1. the canon ledger row still names `OBL-001 / MirCore.Elab.Soundness (stmt)`;
2. the LAB file path, namespace, and constant still exist;
3. the statement body still links `RequestEvidenceSound`,
   `GeneratedWriteSound`, `AssignmentElabSoundnessPost`, and
   `THM001StatementDraft`;
4. the postcondition still includes generated-write soundness, RHS dependency
   recording, failure containment, authority-obligation representation,
   source-span preservation, visible consequences, and nested-locus
   non-authority;
5. `plan/137` has not been superseded by an accepted wrapper decision;
6. OPEN-014 has not been silently resolved elsewhere;
7. docs validators still register the relevant plan / report files;
8. no fresh canon decision has changed the allowed status vocabulary.

If any of these checks fail, the later packet must update the annex instead of
copying it unchanged.

## How to use this template

Use this file as a checklist and copy source for a later proposal draft.

Do not treat the existence of this template as:

- artifact identity acceptance;
- a request to move OBL-001 status;
- evidence that a wrapper is unnecessary;
- evidence that a wrapper is required;
- evidence that OBL-001 is already complete;
- evidence that OBL-002 proof work has started;
- evidence that OPEN-014 or compound assignment scope is resolved.

A later packet may fill only the `lean-stated` candidate path. If it does, it
must explicitly say that `lean-stated` is requested status, not accepted status,
until the canon process accepts it.

## Next allowed moves

Reasonable next packages are:

1. prepare a draft G1 status packet shell that uses the OBL-001 and OBL-020
   annex templates but leaves requested statuses and ledger deltas unresolved;
2. prepare an OBL-021 artifact identity / wrapper preflight if the project
   wants symmetric treatment for all OBL-001/020/021 status artifacts;
3. create a wrapper package only if human/canon review says direct LAB artifact
   citation is not acceptable for the scoped requested-status artifact.
