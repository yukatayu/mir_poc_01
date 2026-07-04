# plan/141 - G1 status packet shell with unresolved slots

## Purpose

This file is LAB repository memory.

It defines a non-applied G1 OBL status packet shell for future human/canon
review. The shell references the OBL-001, OBL-020, and OBL-021 artifact annex
templates now available in `plan/138`, `plan/136`, and `plan/140`, while
leaving requested status, ledger delta, artifact identity acceptance, wrapper
need, OPEN-014 handling, OBL-020 scope, OBL-021 abstraction boundary, proof,
conformance, runtime, and G1 exit decisions explicitly unresolved.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001 / OBL-020 / OBL-021, does not
prove OBL-002 / OBL-020 / OBL-021, does not create a proof skeleton, does not
create Lean wrapper files, does not claim conformance, does not add an
executable row, does not refine a Lean predicate, and does not change runtime,
transport, Core IR, public API, grammar, diagnostic / repair ABI, equality
relation, diagnostic equivalence contract, projection-totality, or sample
status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is a packet shell,
not a submitted proposal and not a status authority.

## Inputs

Canon authority:

- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`

LAB status-prep memory:

- `plan/129-g1-acceptance-packet-preflight.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/131-g1-status-proposal-packet-outline.md`
- `plan/132-g1-status-evidence-readiness-dry-run.md`
- `plan/133-g1-requested-status-options-matrix.md`

Artifact identity / annex memory:

- `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md`
- `plan/138-g1-obl001-artifact-annex-template.md`
- `plan/134-g1-obl020-scope-clarification-packet.md`
- `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`
- `plan/136-g1-obl020-artifact-annex-template.md`
- `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`
- `plan/140-g1-obl021-artifact-annex-template.md`

Lean statement artifacts:

- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`

## Shell status

The packet defined here is a fillable shell.

It is stricter than the outline in `plan/131` because it names the current
artifact annex templates and unresolved decision slots. It is still not a
draft proposal because it deliberately leaves requested statuses, ledger delta
text, human/canon decision requests, and final packet validation results
unfilled.

| State | Meaning | Current file status |
|---|---|---|
| Outline | Packet section structure exists. | Already supplied by `plan/131`. |
| Shell | Section structure plus current annex references and unresolved slots exist. | This file. |
| Draft proposal | Requested statuses, evidence values, ledger delta proposal text, and decision requests are filled. | Not created here. |
| Submitted proposal | Human/canon review is asked to decide. | Not created here. |
| Accepted / applied | Canon ledger or gate status changes. | Not created here. |

## Packet shell: cover sheet

A future draft packet may start from this cover sheet, but must not treat the
unresolved slots as accepted values:

```text
Packet ID:
  [UNRESOLVED]

Packet state:
  shell / draft proposal / submitted proposal
  [Current file state: shell only]

Requested review:
  [UNRESOLVED: OBL-001 only / OBL-001+020 / OBL-001+020+021 /
  staged asymmetric request / no status request yet]

Canon phase position:
  T0/G0 rebaseline unless mirrorea_canon/plan/01-phases.md changes.

Gate target:
  G1 ordinary assignment.

Canon status authority:
  mirrorea_canon/theory/11-metatheory-ledger.md

Requested status vocabulary:
  [UNRESOLVED per OBL: stated / lean-stated / defer /
  another canon-allowed status]

Ledger delta:
  [UNRESOLVED and non-applied]

Decision requested:
  [UNRESOLVED: review shell only / review requested statuses /
  review artifact identities / review abstraction boundaries / other]

Non-claim:
  This shell does not itself move the ledger, complete any OBL, prove any OBL,
  claim conformance, claim runtime readiness, or exit G1.
```

## Packet shell: canon-state section

The future packet must fill this section from canon at the time of submission:

| Slot | Required citation | Current shell value |
|---|---|---|
| Phase authority | `mirrorea_canon/plan/01-phases.md` | Current canon position remains T0. |
| Gate authority | `mirrorea_canon/plan/00-gates.md` | G1 exit requires human decision plus ADR / ledger update and the G1 ordinary-assignment criteria. |
| Status authority | `mirrorea_canon/theory/11-metatheory-ledger.md` | Ledger is the only proof/status authority. |
| Current OBL-001 status | `theory/11-metatheory-ledger.md` row | `open` unless canon changes. |
| Current OBL-020 status | `theory/11-metatheory-ledger.md` row | `open` unless canon changes. |
| Current OBL-021 status | `theory/11-metatheory-ledger.md` row | `open` unless canon changes. |

The future packet must not cite LAB `progress.md`, `tasks.md`, or `plan/` as
status authority.

## Packet shell: requested-status matrix

The future packet must fill one row per OBL. This shell leaves every requested
status unresolved.

| OBL | Canon ledger target | Current LAB artifact / annex | Requested status slot | Blocking decision slot | Ledger delta slot |
|---|---|---|---|---|---|
| OBL-001 | `MirCore.Elab.Soundness (stmt)` | `plan/138` / `THM001StatementDraft.lean` | `[UNRESOLVED: lean-stated candidate / stated / defer / other]` | Artifact identity or wrapper acceptance; OPEN-014 deferral; simple-assignment scope acceptance | `[UNRESOLVED / non-applied]` |
| OBL-020 | `MirCore.Step.WF` | `plan/136` / `StepWFStatementDraft.lean` | `[UNRESOLVED: scoped lean-stated candidate / stated / defer / other]` | Full-row vs G1-supporting scope; abstract WF vocabulary acceptance | `[UNRESOLVED / non-applied]` |
| OBL-021 | `MirCore.Elab.Det` | `plan/140` / `ElabDeterminismStatementDraft.lean` | `[UNRESOLVED: conditional lean-stated candidate / stated / defer / other]` | Artifact identity or wrapper acceptance; abstraction-boundary acceptance; final equality / Diagnostic ABI / projection-totality decision | `[UNRESOLVED / non-applied]` |

Allowed use of advisory readings:

- OBL-001 may be marked `lean-stated candidate`, not accepted `lean-stated`.
- OBL-020 may be marked scoped / conditional, not full-row accepted.
- OBL-021 may be marked conditional, not accepted, until abstraction-boundary
  acceptance is explicit.

## Packet shell: artifact annex index

The future packet may include these annex references:

| Annex | Role in future packet | Required unresolved slots |
|---|---|---|
| `plan/138` OBL-001 artifact annex template | Names current OBL-001 LAB artifact and validation slots. | Requested status, artifact identity, wrapper need, OPEN-014, assignment scope, OBL-002 proof, conformance, G1 exit. |
| `plan/136` OBL-020 artifact annex template | Names current OBL-020 LAB artifact and validation slots. | Requested status, artifact identity, wrapper need, full-row vs G1-supporting scope, concrete WF / Step definitions, proof, conformance, G1 exit. |
| `plan/140` OBL-021 artifact annex template | Names current OBL-021 LAB artifact and validation slots. | Requested status, artifact identity, wrapper need, abstraction-boundary acceptance, final equality, diagnostic equivalence, Diagnostic ABI, projection-totality, proof, conformance, G1 exit. |

The future packet must not copy these annexes as proof that their slots are
already accepted.

## Packet shell: fresh validation table

A future draft proposal must fill fresh results in the same work package that
submits it. Historical results may be cited only as background.

| Check | Command / evidence slot | Current shell value |
|---|---|---|
| OBL-001 Lean compile-check | `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | `[UNRESOLVED fresh result]` |
| OBL-020 Lean compile-check | `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | `[UNRESOLVED fresh result]` |
| OBL-021 Lean compile-check | `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | `[UNRESOLVED fresh result]` |
| LAB statement sync guard | `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` | `[UNRESOLVED fresh result]` |
| Admitted-stub scan | Packet-local scan for `axiom`, `constant`, `theorem`, `admit`, `sorry`, and placeholder bodies across OBL-001/020/021 artifacts | `[UNRESOLVED fresh result]` |
| Docs/source hierarchy validation | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py --format json` | `[UNRESOLVED fresh result]` |
| Secret scan | Tracked Discord webhook full URL / token-prefix scan excluding `.codex-discord` | `[UNRESOLVED fresh result]` |

This shell does not fill these results. Filling them is part of a future draft
proposal or evidence-readiness package.

## Packet shell: evidence trace index

The future packet should fill the trace rows below. This shell names the rows
but does not assert sufficiency.

| Evidence family | Canon anchor | LAB support candidates | Current shell status |
|---|---|---|---|
| SCN-01 static ordinary assignment | `SCN-01`, `theory/01`, `theory/03`, `spec/03` | `plan/121..124`, `ELAB-11`, `ELAB-17` | `[UNRESOLVED sufficiency]` |
| SCN-02 static ordinary assignment | `SCN-02`, `theory/01`, `theory/03`, `spec/03` | `plan/121`, `plan/122`, `plan/124`, `plan/125`, `ELAB-12`, structural `ELAB-02` / `IDX-05` | `[UNRESOLVED sufficiency]` |
| OBL-001 statement identity | `theory/03`, `theory/11` | `plan/73`, `plan/74`, `plan/117`, `plan/124`, `plan/137`, `plan/138` | `[UNRESOLVED artifact/status decision]` |
| OBL-020 statement identity | `theory/01`, `theory/11` | `plan/76`, `plan/78`, `plan/117`, `plan/126`, `plan/134..136` | `[UNRESOLVED scope/status decision]` |
| OBL-021 statement identity | `theory/03`, `theory/11` | `plan/76`, `plan/77`, `plan/117`, `plan/126`, `plan/139`, `plan/140` | `[UNRESOLVED abstraction/status decision]` |
| Boundary / non-claims | `plan/00`, `plan/01`, `spec/06` | `plan/127`, `plan/128`, `plan/129`, `plan/130`, `plan/133` | `[UNRESOLVED acceptance]` |

## Packet shell: open / deferral section

The future packet must choose or explicitly defer each item:

| Open / deferral item | Current shell value | Must not be inferred |
|---|---|---|
| OPEN-014 read materialization | `[UNRESOLVED: defer / resolve separately / blocker]` | No cache, freshness, transport, projection, or read-reply policy. |
| OBL-001 artifact identity | `[UNRESOLVED: direct LAB artifact / wrapper / defer]` | No LAB namespace promotion. |
| OBL-020 scope | `[UNRESOLVED: G1-supporting statement scope / full-row movement / proof fallback]` | No full step-rule WF completion. |
| OBL-021 abstraction boundary | `[UNRESOLVED: abstract equivalence accepted / final equality required / deferred]` | No final equality or Diagnostic ABI. |
| LAB namespace vs wrapper | `[UNRESOLVED per OBL]` | No wrapper requirement or wrapper waiver by default. |
| G3 authority theorem boundary | `[DEFERRED unless separately promoted]` | No THM-004 / production auth proof. |
| C-static / runtime / distributed conformance | `[DEFERRED]` | No conformance by status shell. |

## Packet shell: ledger delta placeholder

This shell reserves space for a future ledger delta but does not provide one.

```text
Ledger delta proposal:
  [UNRESOLVED / omitted in shell]

Target rows:
  OBL-001: [UNRESOLVED proposed status]
  OBL-020: [UNRESOLVED proposed status]
  OBL-021: [UNRESOLVED proposed status]

Patch text:
  [UNRESOLVED and non-applied]

Human/canon decision:
  pending / not submitted

Application status:
  not applied by this shell
```

Any future ledger delta text must remain proposal text until human/canon review
accepts it and a canon-edit package is explicitly promoted.

## Packet shell: submission guard

Before this shell can become a submitted proposal, all of these must be true:

| Guard | Required value before submission |
|---|---|
| Requested status chosen | Each target OBL has exact requested status vocabulary. |
| Artifact identity chosen or deferred | Each target OBL says direct LAB artifact / wrapper required / deferred. |
| OBL-020 scope chosen | Full-row vs G1-supporting scope is explicit. |
| OBL-021 abstraction boundary chosen | Abstract equivalence acceptance vs final relation requirement is explicit. |
| OPEN-014 handled | Deferred or resolved separately without runtime leakage. |
| Fresh validations run | Compile / sync / admitted-stub / docs / secret checks are current to the packet. |
| Ledger delta text either omitted or proposal-only | No canon file is edited by the packet shell. |
| Non-claims preserved | Proof, conformance, runtime, ABI, gate, and sample exclusions are copied or deliberately updated. |

If any guard is missing, the packet remains shell/draft material only.

## Required non-claims

Any future packet based on this shell must include these non-claims unless a
human/canon decision explicitly changes them:

- No canon edit by the shell itself.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted by LAB evidence alone.
- No status proposal submission by this shell alone.
- No metatheory ledger movement.
- No OBL-001 / OBL-020 / OBL-021 completion unless canon explicitly accepts it.
- No OBL-002 / OBL-020 / OBL-021 proof skeleton completion.
- No proof discharge.
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
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution unless separately promoted.
- No G3 / THM-004 authority proof or production auth claim.

## Drift checks for later use

Before a later package promotes this shell to a draft proposal, it should
recheck:

1. `mirrorea_canon/plan/00-gates.md` still defines G1 in the same way;
2. `mirrorea_canon/plan/01-phases.md` still places the project in T0 unless
   canon changed it;
3. `mirrorea_canon/theory/11-metatheory-ledger.md` still has the same OBL rows
   and allowed status vocabulary;
4. OBL-001 / OBL-020 / OBL-021 LAB artifact paths, namespaces, and constants
   still exist;
5. `plan/136`, `plan/138`, and `plan/140` have not been superseded by accepted
   wrapper or artifact-identity decisions;
6. OBL-020 scope and OBL-021 abstraction-boundary decisions remain explicit;
7. docs validators still register the relevant plan / report files;
8. no fresh canon decision has converted any UNRESOLVED shell slot into an
   accepted value.

If any check fails, the later package must update this shell rather than copy
it unchanged.

## How to use this shell

Use this file as a controlled starting point for a later draft proposal.

Do not treat the existence of this shell as:

- status proposal submission;
- requested status acceptance;
- artifact identity acceptance;
- wrapper waiver or wrapper requirement;
- ledger movement;
- OBL completion;
- proof or conformance evidence;
- G1 exit readiness;
- runtime readiness.

A later package may either:

1. fill the shell into a draft proposal with all slots explicit;
2. first run a fresh evidence-readiness package over the exact shell commands;
3. first open a narrower decision packet for OBL-020 scope or OBL-021
   abstraction boundary.

## Next allowed moves

Reasonable next packages are:

1. run a fresh G1 status packet shell evidence dry-run for the exact commands
   named above, still without choosing requested statuses;
2. prepare an OBL-021 equality / diagnostic abstraction decision packet if the
   project wants to resolve the largest OBL-021 blocker before status drafting;
3. prepare a draft proposal only after the user explicitly promotes proposal
   work and agrees to fill requested-status / decision slots.
