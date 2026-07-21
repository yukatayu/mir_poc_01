# plan/157 - Delegated theory research governance and adoption plan

## Purpose

This LAB plan turns the owner's 2026-07-21 authorization into an implementable
canon-amendment package. Its goal is to make theory research continuously
productive without allowing a reversible research result to masquerade as a
settled language, system, conformance, or proof decision.

The intended result is a two-layer research route:

- the relevant canon theory file holds the concise **current** L3 or L2 working
  position; and
- LAB holds candidates, alternatives, experiments, command logs, reviews,
  supersession history, and reproducibility evidence.

Until the canon package described below is effective, this file is planning
memory only. It does not itself delegate authority, change a Gate or Phase,
move an OBL, or close a proof.

## Design constraints

The route must preserve the North Star and these non-negotiable boundaries:

1. Owner authority remains required for L0/L1 direction, core primitives,
   external contracts, SCN/Gate/Phase criteria, ADR effectivity, final proof or
   OBL discharge, and public completion claims.
2. Research authority is scoped by an explicit target ID and allowed operation,
   not inferred from a document's L2/L3 label.
3. A passing Lean artifact over an agent-chosen carrier, equality, trace, or
   axiomatisation is LAB evidence unless a separately owner-approved canonical
   statement binds it to the ledger.
4. `research-complete` is a LAB evidence classification, never a Gate, Phase,
   SCN, OBL, readiness, or public-capability state.
5. No current frozen SCN expectation, conformance classification, expected
   result, public API, or lifecycle label can change through the delegated
   route.
6. Candidate history is forward-recorded. A falsifier may demote a working
   position, but it never erases prior evidence.

## Target operating model

### Roles

| Role | Authority and duty |
| --- | --- |
| Owner | L0/L1, reserved boundaries, ADR effectivity, SCN/Gate/Phase actions, and final proof/public discharge. |
| Research author | Classify authority, create and compare LAB candidates, run countermodels and permitted experiments, propose or maintain scoped L3/L2 working state, and record rollback evidence. |
| Independent reviewer | Check authority classification, hidden semantic delta, evidence quality, non-effects, and exact canon wording. The reviewer cannot enlarge author authority. |
| Canon steward | Serialize integration, rebase the evidence cut, update shared snapshots, validate, commit, and push. The steward may be the author but not the independent reviewer for the same working-state update. |

### Candidate lifecycle

LAB lifecycle labels are distinct from canon document statuses:

```text
proposed -> compared -> provisionally-selected -> superseded
                                       |-> falsified
                                       |-> escalated (owner action required)
```

Every candidate begins with an immutable authority cut containing:

- owner authorization / governing ADR;
- target canon IDs and pinned canon revision;
- pinned LAB inputs;
- allowed operations and explicitly forbidden surfaces;
- result class: reproduction, literal transcription, countermodel,
  conditional lemma, or existing-lane experiment;
- expected falsifier, rollback condition, and integration owner; and
- explicit `semantic_delta` classification.

The authority cut invalidates for integration when a cited canon blob changes.
The author must then re-evaluate the candidate against the new cut.

### L3 to L2 provisional selection

A candidate may become a current `L2-working` position only after all of the
following are recorded and independently reviewed:

1. the exact target ID and a non-reserved authority classification;
2. the status quo and at least one alternative interpretation or falsifier;
3. positive and negative evidence with reproducible commands, tool versions,
   and hashes where artifacts are retained;
4. an impact read over dependent canon IDs and frozen SCNs, without changing
   their expectations;
5. explicit non-effects for core primitives, external contracts, Gates,
   proof discharge, runtime, and public API;
6. a concise canon working statement with scope, assumptions, LAB evidence
   reference, reviewer identity, and concrete rollback trigger; and
7. an independent review with no unresolved authority or soundness finding.

The canon statement contains only the present hypothesis. Alternatives,
rejected candidates, command output, and experiment source remain in LAB.

### Rollback and escalation

A reproducible falsifier may immediately demote a current L2 position to L3
to prevent new dependent work from assuming it. The author then records the
falsifier and the LAB lifecycle transition; a replacement L2 position still
needs independent review.

Stop and escalate to owner action when a candidate would:

- reinterpret an L0/L1 statement or change a core, authority, ownership,
  effect, failure, or judgment primitive;
- change source, public, wire, serialization, provider, transport, artifact,
  or compatibility contract;
- change an SCN expectation, conformance classification, Gate/Phase criterion,
  lifecycle state, or `theory/11` final proof status;
- assert `lean-proved`, `external`, OBL discharge, or a public completion
  claim;
- require a new evidence lane, helper family, schema, CI surface, or Make
  target during the current moratorium; or
- conflict with canon, leave the authority boundary ambiguous, or break a
  settled invariant.

## Whole-project visibility model

The project becomes navigable through five linked views, each with one job:

| View | Question answered | Source of truth |
| --- | --- | --- |
| `mirrorea_canon/MAP.md` | Which system and theory layers exist? | canon topology |
| `docs/diagrams/layer-stack.mmd` | Where do Mir, runtime, adapters, and applications sit? | derived structural map |
| `docs/diagrams/workflow.mmd` | How does research move from candidate to owner decision or working state? | derived process map |
| `docs/project-status.md` | What is current, runnable, open, and owner-reserved? | concise LAB control view |
| `progress.md`, `tasks.md`, and this plan | What is next, why, and under which evidence boundary? | LAB snapshots and repository memory |

`docs/project-status.md` must show three separate dimensions: logical
specification, user-facing specification, and implementation/operations. It
must separately show the current research lifecycle and the owner-reserved
decision boundary. It remains a derived view and cannot make a normative claim.

## Adoption sequence

### Task 1: Record the owner-approved governance amendment

**Files:**

- Create: `mirrorea_canon/meta/proposals/PROPOSAL-005-l2-l3-theory-research-delegation.md`
- Create: `mirrorea_canon/adr/ADR-0014-l2-l3-theory-research-delegation.md`
- Modify: `mirrorea_canon/adr/README.md`, `mirrorea_canon/CHANGELOG.md`, and `mirrorea_canon/INDEX.json`

- [ ] State the owner-approved delegation exactly, including the two-layer
  placement and every reserved boundary.
- [ ] Partially supersede ADR-0012 only for independently reviewed, reversible
  L2/L3 working-theory maintenance.
- [ ] State that no Gate, Phase, SCN, proof, implementation, or public status
  changes as an effect of the governance amendment.
- [ ] Regenerate and validate the canon index.

### Task 2: Reconcile canon operating rules

**Files:**

- Modify: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`
- Modify: `mirrorea_canon/plan/02-operating-model.md`
- Modify: `mirrorea_canon/meta/agent-instructions.md`, `mirrorea_canon/meta/style-guide.md`, and `mirrorea_canon/meta/source-hierarchy.md`
- Modify: `mirrorea_canon/theory/11-metatheory-ledger.md` and `mirrorea_canon/plan/03-risks.md`

- [ ] Replace blanket agent prohibitions with the target-ID and operation-based
  delegated route.
- [ ] Distinguish conditional Lean evidence from owner-controlled final proof
  states without changing any existing entry state.
- [ ] Add the mandatory authority cut, review, integration serialization,
  rollback, and escalation rules.
- [ ] Preserve the current moratorium for new lanes, helpers, schemas, CI, and
  production implementation.

### Task 3: Recut LAB operation and reports

**Files:**

- Modify: `AGENTS.md`, `CANON.md`, and `plan/156-t0-t2-research-autonomy-envelope.md`
- Modify: `plan/00-index.md`, `Documentation.md`, `progress.md`, `tasks.md`, and `docs/project-status.md`
- Modify: `docs/diagrams/workflow.mmd`
- Create: `docs/reports/2287-delegated-theory-research-governance.md`

- [ ] Mark `plan/156` as historical pre-delegation research evidence; retain
  its findings and do not rewrite its investigation history.
- [ ] Make this plan the current lifecycle, candidate-ledger, and next-work
  reference.
- [ ] Update the human-facing control view and workflow diagram so that current
  theory, research evidence, owner decisions, and runnable LAB are visually
  distinct.
- [ ] Record every status-view update decision in the required new report.

### Task 4: Verify and integrate

**Files:**

- Modify only files made necessary by review findings.

- [ ] Search for stale blanket restrictions and distinguish historical text
  from active operating rules.
- [ ] Run `python3 meta/build-index.py` and `python3 meta/build-index.py --check`
  from `mirrorea_canon/`.
- [ ] Run `git diff --check`, `python3 scripts/validate_docs.py`, and
  `make check`.
- [ ] Obtain one final independent review of the complete diff, address valid
  findings, and rerun affected checks.
- [ ] Commit each coherent package with `git commit --no-gpg-sign`, push it,
  and confirm `main...origin/main` is clean.

### Task 5: Pilot the research ratchet

**Files:**

- Create: a new report under `docs/reports/` for the selected candidate.
- Modify: only the relevant existing LAB lane and target canon theory file
  after the authority-cut and independent-review conditions hold.

- [ ] Select one non-reserved target that does not require a new lane or an
  L1 decision. `PROPOSAL-003` and `PROPOSAL-004` are excluded because they are
  owner-controlled L1 questions.
- [ ] Execute the full candidate lifecycle, including a counterexample or
  alternative comparison and explicit rollback trigger.
- [ ] Treat a failed pilot as evidence for revising the governance process, not
  as a reason to invent an implementation or change an SCN.

## Validation and acceptance criteria

The adoption package is accepted only when all of the following hold:

1. Active policy text authorizes reversible L2/L3 working-theory work while
   retaining the owner-reserved surfaces above.
2. Every proposed working-state change has a defined evidence cut, result
   class, independent reviewer, non-effects, and rollback rule.
3. A conditional Lean theorem cannot be represented as a canon proof merely
   because it compiles.
4. Two agents cannot independently integrate the same target without a single
   canon steward reconciling their authority and evidence cuts.
5. The whole-project views link to their sources and do not introduce a new
   normative status or hide a pending owner decision.
6. Canon index, documentation validation, source hierarchy, and `make check`
   pass without an SCN, Gate, Phase, OBL, conformance, runnable-sample, or
   product-readiness overclaim.

## Non-claims

This adoption plan does not make T0 complete, enter T1, discharge an OBL,
prove a theorem, select the OBL-020 formalization organization, close Surface
v0 grammar, create a parser/runtime, promote any sample, or change a public
contract. It establishes research governance and project observability only.
