# plan/135 - G1 OBL-020 artifact identity / wrapper preflight

## Purpose

This file is LAB repository memory.

It clarifies how the current OBL-020 LAB Lean artifact may be cited after the
scope clarification in `plan/134`, and whether a future status proposal should
prepare a canon-facing wrapper or artifact-identity annex before requesting any
ledger status movement.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not move metatheory ledger status, does not complete
OBL-020, does not prove OBL-020, does not create a proof skeleton, does not add
a Lean wrapper file, does not claim conformance, does not add an executable
row, does not refine a Lean predicate, and does not change runtime, transport,
Core IR, public API, grammar, or sample status.

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
samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft
```

Canon ledger target:

```text
OBL-020 / MirCore.Step.WF
```

Current status:

- The LAB artifact compile-checks.
- It is registered in the LAB Lean manifest.
- Sync guards check the OBL-020 body links and obvious vacuity / drift.
- It deliberately lives under `MirCore.Lab...`.
- The canon ledger has not accepted the LAB path, namespace, or constant as
  the OBL-020 target.

## Problem statement

`plan/134` narrows the safe near-term posture to a G1-supporting OBL-020
statement-scope candidate, while full-row OBL-020 status movement remains
deferred.

The next risk is artifact-identity laundering:

- If a future packet cites the LAB constant directly, readers may infer that
  `MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft` is already the canon
  `MirCore.Step.WF` target.
- If a future package creates a canon-facing wrapper too early, readers may
  infer that the wrapper has already been accepted as canon or that full-row
  status movement has begun.

P82 therefore distinguishes evidence citation from requested-status artifact
identity.

## Citation modes

| Mode | Meaning | Current reading |
|---|---|---|
| LAB evidence citation | Cite the current LAB Lean artifact as evidence that the abstract OBL-020 statement shape exists and compile-checks. | Allowed now, if every citation says LAB evidence only and no status movement. |
| Proposal artifact identity | Name the exact artifact a future proposal asks human/canon review to accept for a requested status. | Not allowed silently. Requires an artifact-identity annex and human/canon acceptance path. |
| Canon-facing wrapper | A future wrapper or renamed statement target that maps the LAB statement shape toward `MirCore.Step.WF` without pretending canon already accepted it. | Possible later, but P82 should not create it. First decide wrapper need and non-claims. |
| Canon ledger target | The authoritative OBL-020 target in `mirrorea_canon/theory/11-metatheory-ledger.md`. | Unchanged and open. |

## Recommendation

The next status-prep path should use a two-level posture:

1. Directly cite the existing LAB artifact only as **LAB evidence** for
   G1-supporting statement-scope discussion.
2. Before any later `lean-stated` request, prepare an **artifact-identity annex**
   that asks human/canon review to choose one of:
   - accept the LAB path / namespace / constant as the requested-status
     artifact;
   - require a canon-facing wrapper;
   - reject both until concrete `Config`, `Step`, and `WellFormed` definitions
     are chosen.

The default recommendation is **wrapper-preflight, not wrapper creation**.
Creating a Lean wrapper now would risk freezing naming and status implications
before the human/canon process has accepted the scope boundary.

## Why direct citation is insufficient for status request

Direct citation is useful for evidence trace, but insufficient for any
requested-status packet because:

- the namespace is `MirCore.Lab...`, deliberately outside canon;
- `plan/130` says artifact identity remains a human/canon decision;
- `plan/133` says OBL-020 conditional `lean-stated` is blocked until scope
  acceptance;
- `plan/134` says full-row OBL-020 remains deferred and the current artifact is
  only a G1-supporting statement-scope candidate;
- the file does not instantiate final `Config`, `StepLabel`, `StepFamily`,
  `WellFormed`, `Step`, scheduler semantics, or per-step proof boundaries.

## Why wrapper creation is premature

A wrapper may become useful later, but creating it now would introduce these
risks:

- readers may treat the wrapper namespace as canon acceptance;
- the wrapper may freeze a name before the ledger target mapping is accepted;
- a thin alias may hide the fact that abstract predicates remain abstract;
- a stronger wrapper may accidentally refine the statement without a separate
  proof-boundary decision;
- the project may create two statement identities that can drift.

Therefore P82 should not add a Lean wrapper file. It should define what a later
wrapper package would have to preserve.

## Wrapper preflight requirements

If a future package prepares a wrapper, it must satisfy all of these
requirements:

| Requirement | Meaning |
|---|---|
| Non-applied status | The wrapper is proposal evidence only; the ledger stays unchanged. |
| Exact source link | The wrapper names the current LAB artifact and the exact constant it wraps or re-exports. |
| Scope label | The wrapper says G1-supporting statement scope, not full-row OBL-020 completion. |
| No strengthening by accident | Any wrapper must not add concrete WF clauses, step families, or proof premises without a separate refinement package. |
| No weakening by accident | Any wrapper must not erase `WellFormed`, `Step`, or `PreservesWF` body links. |
| Fresh validation | The wrapper package must rerun Lean compile-check, sync guards, no-admitted-stub scan, docs validation, and secret scan. |
| Open annex | The wrapper package must repeat that concrete `Config`, `Step`, `WellFormed`, step-family coverage, proof, conformance, and G1 exit remain open. |

## Candidate future wrapper shapes

These are naming sketches only. P82 does not accept any of them.

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

Any later packet citing the current OBL-020 LAB artifact should say:

- "This citation is LAB evidence, not canon artifact acceptance."
- "The cited artifact is
  `MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft`; the canon ledger
  target remains `MirCore.Step.WF` and remains open."
- "Direct citation is acceptable for evidence trace only. Requested-status
  artifact identity still requires human/canon acceptance."
- "A wrapper, if later created, is non-applied proposal evidence until canon
  explicitly accepts it."
- "No full-row OBL-020 completion, proof skeleton, proof discharge,
  conformance, runtime readiness, or G1 exit is claimed."

## Hidden failure modes

| Failure mode | Trigger | Avoidance rule |
|---|---|---|
| LAB-to-canon namespace laundering | Directly citing `MirCore.Lab...` as if it were `MirCore.Step.WF`. | Always label it LAB evidence unless artifact identity is accepted. |
| Wrapper status laundering | Adding a canon-facing wrapper and readers infer ledger movement. | Do not create a wrapper in P82; require non-applied wrapper wording later. |
| Alias drift | A wrapper aliases the LAB statement, then one side evolves. | Require exact source link and drift guards in any wrapper package. |
| Accidental strengthening | A wrapper adds concrete clauses that were not accepted. | Keep wrapper shape-preserving unless a separate refinement package opens. |
| Accidental weakening | A wrapper hides `PreservesWF` / `Step` / `WellFormed` links. | Require body-link guards and no-vacuity checks. |
| Premature naming freeze | A wrapper path or namespace becomes de facto public API. | Mark candidate names unresolved and non-public. |
| G1 shortcut | Artifact identity acceptance is read as G1 exit. | Keep gate movement separate from artifact review. |

## Decision boundary for later work

This packet makes the next human/canon-facing artifact decision smaller:

```text
For G1-supporting OBL-020 statement-scope review, should the future packet cite
the existing LAB artifact directly as the requested-status artifact, require a
canon-facing wrapper, or defer artifact identity until concrete step/WF
definitions are chosen?
```

Consequences:

| Answer | Consequence |
|---|---|
| Direct LAB artifact accepted | A later proposal may cite the LAB path / namespace / constant as the requested artifact, still without applying ledger movement. |
| Wrapper required | A later wrapper package may be opened with strict non-applied, shape-preserving constraints. |
| Defer artifact identity | OBL-020 requested-status work should wait for concrete runtime / proof-boundary choices. |

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted.
- No status proposal submission.
- No metatheory ledger movement.
- No OBL completion.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new Lean wrapper file.
- No new executable row.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.
- No G1 exit by implication from artifact review.

## Next allowed move

The next autonomous package can choose one of these follow-ups:

1. prepare an OBL-020 artifact annex template for a later draft proposal,
   without creating a wrapper or applying ledger movement;
2. prepare an OBL-001 canon-facing wrapper preflight, because OBL-001 remains
   the strongest later `lean-stated` candidate;
3. prepare an OBL-021 equality / diagnostic abstraction decision packet;
4. draft a non-applied G1 status proposal skeleton only if OBL-020 scope and
   artifact identity remain explicit unresolved slots.

The default should not be wrapper creation until the wrapper need is accepted.

Later-state note, 2026-07-05: `plan/136` now supplies the OBL-020 artifact
annex template, `plan/137` / `plan/138` now supply the OBL-001 artifact
preflight and annex template, `plan/143` now supplies the OBL-021 abstraction
decision packet, `plan/144` prevents duplicate OBL-020 scope work, and
`plan/145` prevents duplicate OBL-001 artifact-decision work. Do not read the
list above as current permission to duplicate those packets or create a wrapper
without explicit human/canon promotion.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is artifact-identity-preflight-only: no canon edit, no gate
exit, no wrapper file, no status proposal submission, no requested status
acceptance, no OBL status movement, no proof, no conformance claim, no
implementation change, and no runnable sample status change.
