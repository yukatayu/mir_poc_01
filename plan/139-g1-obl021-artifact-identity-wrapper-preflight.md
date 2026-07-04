# plan/139 - G1 OBL-021 artifact identity / wrapper preflight

## Purpose

This file is LAB repository memory.

It clarifies how the current OBL-021 LAB Lean artifact may be cited before any
future requested-status packet, and whether a future packet should prepare a
canon-facing wrapper or artifact-identity annex before asking for conditional
`lean-stated` review.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-021, does not prove OBL-021, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not change runtime, transport, Core IR, public API,
grammar, equality relation, diagnostic equivalence contract, diagnostic /
repair ABI, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is an
artifact-identity / wrapper preflight only; the canon metatheory ledger remains
the only proof/status authority.

## Artifact under review

Current LAB artifact:

```text
samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft
```

Canon ledger target:

```text
OBL-021 / Elaboration determinism / MirCore.Elab.Det
```

Current status:

- The LAB artifact compile-checks in prior LAB evidence.
- It is registered in the LAB Lean manifest.
- Sync guards check result-equivalence families, diagnostic equivalence,
  success/reject mutual exclusion, `ElabDeterministicPost`, and obvious vacuity
  / drift.
- It deliberately lives under `MirCore.Lab...`.
- The canon ledger has not accepted the LAB path, namespace, or constant as
  the OBL-021 target.

## Problem statement

`plan/133` identifies OBL-021 as a conditional later `lean-stated` candidate if
human/canon review accepts the abstraction boundary. The current LAB artifact
is a useful Lean statement-shape draft, but it deliberately leaves final result
equality, diagnostic equivalence, parser/checker implementation proof, and
runtime scheduling determinism outside the statement.

The remaining risk is artifact-identity and abstraction-boundary laundering:

- If a future packet cites the LAB constant directly, readers may infer that
  `MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft` is already the canon
  `MirCore.Elab.Det` target.
- If a future package creates a canon-facing wrapper too early, readers may
  infer that final equality and diagnostic equivalence have already been
  accepted, or that `lean-stated` status has already moved.

P86 therefore distinguishes evidence citation from requested-status artifact
identity and keeps the abstraction boundary explicit.

## Citation modes

| Mode | Meaning | Current reading |
|---|---|---|
| LAB evidence citation | Cite the current LAB Lean artifact as evidence that elaboration determinism can be expressed as an abstract Lean `Prop` shape. | Allowed now, if every citation says LAB evidence only and no status movement. |
| Proposal artifact identity | Name the exact artifact a future proposal asks human/canon review to accept for conditional `lean-stated` status. | Not allowed silently. Requires an artifact-identity annex and human/canon acceptance path. |
| Canon-facing wrapper | A future wrapper or renamed statement target that maps the LAB statement shape toward `MirCore.Elab.Det` without pretending canon already accepted it. | Possible later, but P86 should not create it. First decide wrapper need and abstraction-boundary non-claims. |
| Canon ledger target | The authoritative OBL-021 target in `mirrorea_canon/theory/11-metatheory-ledger.md`. | Unchanged and open. |

## Recommendation

The next OBL-021 status-prep path should use a two-level posture:

1. Directly cite the existing LAB artifact only as **LAB evidence** for
   G1-supporting OBL-021 statement/status discussion.
2. Before any later `lean-stated` request, prepare an **artifact-identity
   annex** that asks human/canon review to choose one of:
   - accept the LAB path / namespace / constant as the requested-status
     artifact and accept the abstract equivalence boundary;
   - require a canon-facing wrapper;
   - defer artifact identity until final equality / diagnostic equivalence /
     projection-totality boundaries are chosen.

OBL-021 is a plausible future `lean-stated` candidate only after the abstraction
boundary is accepted. The default recommendation is **annex-first and
wrapper-ready, not wrapper creation now**.

Creating a Lean wrapper now would risk freezing naming, final equality,
diagnostic equivalence, and status implications before the human/canon process
has accepted the artifact identity.

## Why direct citation is insufficient for status request

Direct citation is useful for evidence trace, but insufficient for any
requested-status packet because:

- the namespace is `MirCore.Lab...`, deliberately outside canon;
- the canon target is `MirCore.Elab.Det`, not `OBL021StatementDraft`;
- `plan/130` says artifact identity remains a human/canon decision;
- `plan/133` makes OBL-021 `lean-stated` conditional on abstraction-boundary
  acceptance;
- `plan/126` says final equality selection, projection-totality mechanics, and
  diagnostic ABI remain outside the current statement draft;
- the file does not prove that the implementation is deterministic, prove
  parser/checker correctness, select final equality, or claim C-static
  conformance.

## Why wrapper creation is premature

A wrapper may become useful later, but creating it now would introduce these
risks:

- readers may treat a `MirCore.Elab.Det`-like namespace as canon acceptance;
- the wrapper may freeze a public theorem name before the ledger target mapping
  is accepted;
- a thin alias may hide that equivalence predicates remain abstract;
- a stronger wrapper may accidentally select final equality, diagnostic
  equivalence, projection-totality, parser/checker implementation proof, or
  conformance boundaries;
- the project may create two statement identities that can drift.

Therefore P86 should not add a Lean wrapper file. It should define what a later
wrapper package would have to preserve.

## Wrapper preflight requirements

If a future package prepares a wrapper, it must satisfy all of these
requirements:

| Requirement | Meaning |
|---|---|
| Non-applied status | The wrapper is proposal evidence only; the ledger stays unchanged. |
| Exact source link | The wrapper names the current LAB artifact and the exact constant it wraps or re-exports. |
| Scope label | The wrapper says OBL-021 elaboration determinism statement identity, not proof, conformance, runtime scheduling determinism, or G1 exit. |
| Abstraction-boundary label | The wrapper must say whether abstract `SameElabResult`, `SameDiagnostic`, and component equivalence predicates are accepted or still under review. |
| No strengthening by accident | Any wrapper must not add final equality, final Diagnostic ABI, projection-totality, parser/checker implementation proof, or proof premises without a separate refinement package. |
| No weakening by accident | Any wrapper must not erase success-success result equivalence, diagnostic-diagnostic equivalence, success/reject mutual exclusion, or component-equivalence links. |
| Fresh validation | The wrapper package must rerun Lean compile-check, sync guards, no-admitted-stub scan, docs validation, and secret scan. |
| Open annex | The wrapper package must repeat that proof, conformance, runtime scheduling, final equality, final diagnostic ABI, and G1 exit remain open. |

## Candidate future wrapper shapes

These are naming sketches only. P86 does not accept any of them.

| Candidate | Shape | Risk |
|---|---|---|
| Wrapper alias | A canon-facing file imports or references the LAB statement and defines an alias-like proposition. | May look like canon acceptance even if non-applied. |
| Wrapper theorem statement | A theorem statement with no proof attempt, still as `Prop` shape. | May be mistaken for proof skeleton or `lean-stated` request. |
| Artifact annex only | No new Lean file; a proposal annex names the LAB artifact and asks whether it is acceptable. | Safest for now, but leaves wrapper decision unresolved. |

Current preference:

- Use **artifact annex only** for the next OBL-021 proposal-prep step, if the
  project wants symmetric OBL-001/020/021 packet material.
- Create a wrapper only after human/canon review says the LAB namespace cannot
  be cited directly for the requested status.

## Required wording for later packets

Any later packet citing the current OBL-021 LAB artifact should say:

- "This citation is LAB evidence, not canon artifact acceptance."
- "The cited artifact is
  `MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft`; the canon ledger
  target remains `MirCore.Elab.Det` and remains open."
- "Direct citation is acceptable for evidence trace only. Requested-status
  artifact identity still requires human/canon acceptance."
- "A wrapper, if later created, is non-applied proposal evidence until canon
  explicitly accepts it."
- "No OBL-021 completion, proof discharge, parser/checker implementation
  proof, final equality selection, final Diagnostic ABI, conformance, runtime
  scheduling determinism, or G1 exit is claimed."

## Hidden failure modes

| Failure mode | Trigger | Avoidance rule |
|---|---|---|
| LAB-to-canon namespace laundering | Directly citing `MirCore.Lab...` as if it were `MirCore.Elab.Det`. | Always label it LAB evidence unless artifact identity is accepted. |
| Wrapper status laundering | Adding a canon-facing wrapper and readers infer ledger movement. | Do not create a wrapper in P86; require non-applied wrapper wording later. |
| Abstraction-boundary laundering | A packet cites the LAB artifact while readers infer final equality / diagnostic equivalence is accepted. | State that OBL-021 `lean-stated` is conditional on accepting the abstract equivalence boundary. |
| Alias drift | A wrapper aliases the LAB statement, then one side evolves. | Require exact source link and drift guards in any wrapper package. |
| Accidental strengthening | A wrapper adds final equality, projection-totality, Diagnostic ABI, or parser/checker proof. | Keep wrapper shape-preserving unless a separate refinement package opens. |
| Accidental weakening | A wrapper hides result-equivalence, diagnostic-equivalence, or success/reject exclusion links. | Require body-link guards and no-vacuity checks. |
| Premature naming freeze | A wrapper path or namespace becomes de facto public API. | Mark candidate names unresolved and non-public. |
| G1 shortcut | Artifact identity acceptance is read as G1 exit. | Keep gate movement separate from artifact review. |

## Decision boundary for later work

This packet makes the next human/canon-facing artifact decision smaller:

```text
For OBL-021 elaboration determinism review, should the future packet cite the
existing LAB artifact directly as the requested conditional `lean-stated`
artifact, require a canon-facing wrapper, or defer artifact identity until final
equality / diagnostic equivalence / projection-totality decisions are made?
```

Consequences:

| Answer | Consequence |
|---|---|
| Direct LAB artifact accepted | A later proposal may cite the LAB path / namespace / constant as the requested artifact, still without applying ledger movement. |
| Wrapper required | A later wrapper package may be opened with strict non-applied, shape-preserving constraints. |
| Defer artifact identity | OBL-021 requested-status work should wait for equality, diagnostic equivalence, projection-totality, or proof-boundary choices. |

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted.
- No status proposal submission.
- No metatheory ledger movement.
- No OBL-021 completion.
- No proof skeleton completion.
- No proof discharge.
- No parser/checker implementation proof.
- No final equality relation.
- No final diagnostic equivalence contract.
- No final Diagnostic ABI.
- No projection-totality proof.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new Lean wrapper file.
- No new executable row.
- No Lean predicate refinement.
- No runtime scheduling determinism claim.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, or distributed
  transport claim.
- No final Core IR, repair, runtime, transport, projection, telemetry, public
  API, grammar ABI, or assignment taxonomy freeze.
- No sample status relabel.
- No G1 exit by implication from artifact review.

## Next allowed move

The next autonomous package can choose one of these follow-ups:

1. prepare an OBL-021 artifact annex template for a later draft proposal,
   without creating a wrapper or applying ledger movement;
2. prepare a G1 status packet shell that references OBL-001 / OBL-020 /
   OBL-021 artifact decisions but leaves requested statuses and ledger deltas
   unresolved;
3. create a wrapper package only if human/canon review requires one.
