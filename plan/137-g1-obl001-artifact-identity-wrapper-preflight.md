# plan/137 - G1 OBL-001 artifact identity / wrapper preflight

## Purpose

This file is LAB repository memory.

It clarifies how the current OBL-001 / THM-001 LAB Lean artifact may be cited
before any future requested-status packet, and whether a future packet should
prepare a canon-facing wrapper or artifact-identity annex before asking for
`lean-stated` review.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001, does not prove OBL-002, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not change runtime, transport, Core IR, public API,
grammar, diagnostic / repair ABI, or sample status.

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
samples/lean/lab-statements/obl001/THM001StatementDraft.lean
MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft
```

Canon ledger target:

```text
OBL-001 / THM-001 Lean statement / MirCore.Elab.Soundness (stmt)
```

Current status:

- The LAB artifact compile-checks in prior LAB evidence.
- It is registered in the LAB Lean manifest.
- Sync guards check request evidence, generated-write coverage, RHS
  dependency recording, generated-failure containment, authority obligations,
  source-span evidence, visible consequences, nested-locus non-authority, and
  obvious vacuity / drift.
- It deliberately lives under `MirCore.Lab...`.
- The canon ledger has not accepted the LAB path, namespace, or constant as
  the OBL-001 target.

## Problem statement

`plan/133` identifies OBL-001 as the strongest later `lean-stated` candidate
among OBL-001 / OBL-020 / OBL-021, because the canon row explicitly asks for
the THM-001 Lean statement and the current LAB artifact is already a
compile-checked Lean `Prop` shape.

The remaining risk is artifact-identity laundering:

- If a future packet cites the LAB constant directly, readers may infer that
  `MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft` is already the
  canon `MirCore.Elab.Soundness (stmt)` target.
- If a future package creates a canon-facing wrapper too early, readers may
  infer that the wrapper has already been accepted as canon or that
  `lean-stated` status has already moved.

P84 therefore distinguishes evidence citation from requested-status artifact
identity.

## Citation modes

| Mode | Meaning | Current reading |
|---|---|---|
| LAB evidence citation | Cite the current LAB Lean artifact as evidence that the THM-001 assignment-elaboration statement shape exists and compile-checks. | Allowed now, if every citation says LAB evidence only and no status movement. |
| Proposal artifact identity | Name the exact artifact a future proposal asks human/canon review to accept for `lean-stated` status. | Not allowed silently. Requires an artifact-identity annex and human/canon acceptance path. |
| Canon-facing wrapper | A future wrapper or renamed statement target that maps the LAB statement shape toward `MirCore.Elab.Soundness (stmt)` without pretending canon already accepted it. | Possible later, but P84 should not create it. First decide wrapper need and non-claims. |
| Canon ledger target | The authoritative OBL-001 target in `mirrorea_canon/theory/11-metatheory-ledger.md`. | Unchanged and open. |

## Recommendation

The next OBL-001 status-prep path should use a two-level posture:

1. Directly cite the existing LAB artifact only as **LAB evidence** for
   G1-supporting OBL-001 statement/status discussion.
2. Before any later `lean-stated` request, prepare an **artifact-identity
   annex** that asks human/canon review to choose one of:
   - accept the LAB path / namespace / constant as the requested-status
     artifact;
   - require a canon-facing wrapper;
   - defer artifact identity until OPEN-014 and assignment-scope acceptance
     are resolved.

OBL-001 is a stronger future `lean-stated` candidate than OBL-020 because the
canon row is itself a Lean statement obligation. That does not remove the
artifact-identity decision. The default recommendation is **annex-first and
wrapper-ready, not wrapper creation now**.

Creating a Lean wrapper now would risk freezing naming, simple-assignment
scope, OPEN-014 deferral posture, and status implications before the
human/canon process has accepted the artifact identity.

## Why direct citation is insufficient for status request

Direct citation is useful for evidence trace, but insufficient for any
requested-status packet because:

- the namespace is `MirCore.Lab...`, deliberately outside canon;
- the canon target is `MirCore.Elab.Soundness (stmt)`, not
  `THM001StatementDraft`;
- `plan/130` says artifact identity remains a human/canon decision;
- `plan/133` says OBL-001 is a natural later `lean-stated` candidate only if
  artifact identity or wrapper acceptance is resolved;
- OPEN-014 read materialization remains deferred and must not be resolved by a
  statement-status request;
- the file does not prove that the implementation satisfies the predicates,
  discharge OBL-002, prove OBL-020/021, or claim C-static conformance.

## Why wrapper creation is premature

A wrapper may become useful later, but creating it now would introduce these
risks:

- readers may treat a `MirCore.Elab.Soundness`-like namespace as canon
  acceptance;
- the wrapper may freeze a public theorem name before the ledger target mapping
  is accepted;
- a thin alias may hide that predicates remain abstract and implementation
  satisfaction is unproved;
- a stronger wrapper may accidentally decide OPEN-014, compound assignment
  scope, authority-soundness scope, diagnostic / repair ABI, or conformance
  boundaries;
- the project may create two statement identities that can drift.

Therefore P84 should not add a Lean wrapper file. It should define what a later
wrapper package would have to preserve.

## Wrapper preflight requirements

If a future package prepares a wrapper, it must satisfy all of these
requirements:

| Requirement | Meaning |
|---|---|
| Non-applied status | The wrapper is proposal evidence only; the ledger stays unchanged. |
| Exact source link | The wrapper names the current LAB artifact and the exact constant it wraps or re-exports. |
| Scope label | The wrapper says THM-001 / OBL-001 statement identity, not OBL-002 proof, conformance, runtime, or G1 exit. |
| OPEN-014 deferral | The wrapper must not choose read materialization, cache, freshness, transport, projection, or observe-vs-read-request policy. |
| No strengthening by accident | Any wrapper must not add concrete implementation satisfaction, authority proof, diagnostic/repair ABI, compound-assignment coverage, or proof premises without a separate refinement package. |
| No weakening by accident | Any wrapper must not erase generated-write soundness, RHS dependency recording, failure containment, authority-obligation representation, source-span preservation, visible consequences, or nested-locus non-authority links. |
| Fresh validation | The wrapper package must rerun Lean compile-check, sync guards, no-admitted-stub scan, docs validation, and secret scan. |
| Open annex | The wrapper package must repeat that proof, conformance, runtime, OPEN-014, final ABI, and G1 exit remain open. |

## Candidate future wrapper shapes

These are naming sketches only. P84 does not accept any of them.

| Candidate | Shape | Risk |
|---|---|---|
| Wrapper alias | A canon-facing file imports or references the LAB statement and defines an alias-like proposition. | May look like canon acceptance even if non-applied. |
| Wrapper theorem statement | A theorem statement with no proof attempt, still as `Prop` shape. | May be mistaken for proof skeleton or `lean-stated` request. |
| Artifact annex only | No new Lean file; a proposal annex names the LAB artifact and asks whether it is acceptable. | Safest for now, but leaves wrapper decision unresolved. |

Current preference:

- Use **artifact annex only** for the next proposal-prep step.
- Create a wrapper only after human/canon review says the LAB namespace cannot
  be cited directly for the requested status.

## Required wording for later packets

Any later packet citing the current OBL-001 LAB artifact should say:

- "This citation is LAB evidence, not canon artifact acceptance."
- "The cited artifact is
  `MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft`; the canon ledger
  target remains `MirCore.Elab.Soundness (stmt)` and remains open."
- "Direct citation is acceptable for evidence trace only. Requested-status
  artifact identity still requires human/canon acceptance."
- "A wrapper, if later created, is non-applied proposal evidence until canon
  explicitly accepts it."
- "No OBL-001 completion, OBL-002 proof discharge, OBL-020/021 completion,
  conformance, runtime readiness, or G1 exit is claimed."
- "OPEN-014 remains deferred unless a separate canon decision resolves it."

## Hidden failure modes

| Failure mode | Trigger | Avoidance rule |
|---|---|---|
| LAB-to-canon namespace laundering | Directly citing `MirCore.Lab...` as if it were `MirCore.Elab.Soundness`. | Always label it LAB evidence unless artifact identity is accepted. |
| Wrapper status laundering | Adding a canon-facing wrapper and readers infer ledger movement. | Do not create a wrapper in P84; require non-applied wrapper wording later. |
| Alias drift | A wrapper aliases the LAB statement, then one side evolves. | Require exact source link and drift guards in any wrapper package. |
| Accidental strengthening | A wrapper adds implementation satisfaction, OPEN-014, compound assignment, authority proof, or conformance clauses. | Keep wrapper shape-preserving unless a separate refinement package opens. |
| Accidental weakening | A wrapper hides request, dependency, failure, authority-obligation, span, visible-consequence, or nested-locus links. | Require body-link guards and no-vacuity checks. |
| Premature naming freeze | A wrapper path or namespace becomes de facto public API. | Mark candidate names unresolved and non-public. |
| G1 shortcut | Artifact identity acceptance is read as G1 exit. | Keep gate movement separate from artifact review. |

## Decision boundary for later work

This packet makes the next human/canon-facing artifact decision smaller:

```text
For OBL-001 / THM-001 statement review, should the future packet cite the
existing LAB artifact directly as the requested `lean-stated` artifact, require
a canon-facing wrapper, or defer artifact identity until OPEN-014 and
assignment-scope decisions are resolved?
```

Consequences:

| Answer | Consequence |
|---|---|
| Direct LAB artifact accepted | A later proposal may cite the LAB path / namespace / constant as the requested artifact, still without applying ledger movement. |
| Wrapper required | A later wrapper package may be opened with strict non-applied, shape-preserving constraints. |
| Defer artifact identity | OBL-001 requested-status work should wait for OPEN-014, assignment-scope, or proof-boundary choices. |

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted.
- No status proposal submission.
- No metatheory ledger movement.
- No OBL-001 completion.
- No OBL-002 proof skeleton completion.
- No OBL-002 proof discharge.
- No OBL-020 / OBL-021 completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new Lean wrapper file.
- No new executable row.
- No Lean predicate refinement.
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, or assignment taxonomy freeze.
- No sample status relabel.
- No G1 exit by implication from artifact review.

## Next allowed move

The next autonomous package can choose one of these follow-ups:

1. prepare an OBL-001 artifact annex template for a later draft proposal,
   without creating a wrapper or applying ledger movement;
2. prepare a G1 status packet shell that references OBL-001 / OBL-020 artifact
   annex needs but leaves requested statuses and ledger deltas unresolved;
3. create a wrapper package only if human/canon review requires one.
