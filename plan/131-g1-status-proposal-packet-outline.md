# plan/131 - G1 status proposal packet outline

## Purpose

This file is LAB repository memory.

It drafts the outline of a future G1 OBL-001 / OBL-020 / OBL-021 status
proposal packet. The packet would use `plan/130` as its criteria matrix and
`plan/129` as its acceptance preflight, but this file does not itself propose,
accept, or apply any status movement.

This file does not edit canon, does not close G0 or G1, does not move
metatheory ledger status, does not complete OBL-001 / OBL-020 / OBL-021, does
not prove OBL-002, does not claim conformance, does not add an executable row,
does not refine a Lean predicate, and does not change runtime, transport,
diagnostic, repair, Core IR, public API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is an outline for
a later proposal packet, not the proposal packet itself.

## Why this outline exists

`plan/129` says a future G1 acceptance packet needs a statement/status annex.
`plan/130` then defines the criteria that a later status proposal would have to
satisfy before asking the canon metatheory ledger to move any OBL row.

This file turns that criteria matrix into a packet structure so later work can
fill evidence slots without silently turning LAB support into canon status.

## Packet state taxonomy

| State | Meaning | Allowed after this file? |
|---|---|---|
| `outline` | Section structure and required evidence slots exist. | Yes; this file creates it. |
| `draft proposal` | The slots are filled with exact evidence, requested status vocabulary, and proposed ledger delta text. | No; later package only. |
| `submitted proposal` | The human/canon process is asked to review the draft. | No; requires explicit promotion. |
| `accepted proposal` | Canon accepts a status movement and updates the status authority. | No; only canon/human process can do this. |
| `applied ledger update` | `mirrorea_canon/theory/11-metatheory-ledger.md` changes. | No; not without explicit canon-edit promotion. |

## Packet cover sheet

A future packet should begin with a compact cover sheet:

| Field | Required content | P78 value |
|---|---|---|
| Packet ID | Stable identifier for the proposal package. | Unassigned. |
| Requested review | Which OBL rows are being proposed for status movement. | OBL-001 / OBL-020 / OBL-021 candidate set only. |
| Requested status vocabulary | `stated`, `lean-stated`, or other allowed ledger status for each OBL. | Slot only; P78 does not choose. |
| Canon authority | Files whose acceptance would be changed or cited. | `plan/00-gates`, `theory/11-metatheory-ledger`, and relevant theory chapters. |
| LAB evidence root | LAB plans, Lean draft paths, guard tests, and validation commands. | To be filled by later draft proposal. |
| Explicit non-claims | Proof, runtime, conformance, ABI, gate, and sample exclusions. | Must be included before any submission. |
| Decision requested | Exact human/canon decision requested. | Unassigned; later proposal work. |

## Canon-state summary section

A future packet should quote or cite these canon facts before presenting LAB
evidence:

| Canon fact | Required packet wording |
|---|---|
| Current phase authority | Canon still places implementation state in T0/G0 unless canon says otherwise. |
| Gate authority | G1 exit requires human decision plus ADR / ledger update; this packet cannot move the gate by itself. |
| G1 criteria | G1 ordinary assignment requires `theory/01` and `theory/03` to explain SCN-01 / SCN-02, OBL-001 Lean statement completion, and OBL-020 / OBL-021 completion. |
| Status authority | `mirrorea_canon/theory/11-metatheory-ledger.md` is the only THM / OBL status authority. |
| Allowed status vocabulary | Status is one of `open`, `stated`, `lean-stated`, `lean-proved`, or `external` unless canon changes that vocabulary. |

The packet must not quote LAB `progress.md`, `tasks.md`, or `plan/` text as
status authority.

## Requested-status matrix

The future packet should use one row per OBL:

| OBL | Ledger target | Requested status slot | Rationale slot | Blocking decision slot |
|---|---|---|---|---|
| OBL-001 | `MirCore.Elab.Soundness (stmt)` | Unchosen. | Must explain whether THM-001 Lean statement completion means `lean-stated` or some other status. | Human/canon must accept the statement target identity. |
| OBL-020 | `MirCore.Step.WF` | Unchosen. | Must explain whether the current abstract WF preservation statement is enough, or whether concrete step / WF clauses must bind first. | Human/canon must choose full OBL-020 vs G1-supporting status scope. |
| OBL-021 | `MirCore.Elab.Det` | Unchosen. | Must explain whether abstract result / diagnostic equivalence is enough, or whether final equality / diagnostic equivalence must bind first. | Human/canon must accept the abstraction boundary. |

P78 deliberately leaves the requested status slots unfilled. Filling them is
the first act of a later draft proposal, not an outline task.

## Artifact identity annex

For each OBL, the future packet must identify the exact artifact that would be
accepted if the requested status is `lean-stated`.

| OBL | Current LAB artifact candidate | Required future check | Open artifact question |
|---|---|---|---|
| OBL-001 | `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` / `THM001StatementDraft` | Fresh Lean compile, no admitted stubs, body-link guard, vacuity guard, and statement-to-ledger mapping. | Whether a LAB namespace is acceptable as evidence or a canon-facing wrapper is required. |
| OBL-020 | `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` / `OBL020StatementDraft` | Fresh Lean compile, no admitted stubs, WF-preservation body-link guard, and statement-to-ledger mapping. | Whether abstract `WellFormed` / `Step` / `PreservesWF` is enough for status. |
| OBL-021 | `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` / `OBL021StatementDraft` | Fresh Lean compile, no admitted stubs, determinism body-link guard, and statement-to-ledger mapping. | Whether abstract equivalence predicates are enough for status. |

This annex is not a claim that these artifacts are already accepted ledger
targets.

## Evidence trace annex

The future packet should include a trace table with these columns:

| Column | Meaning |
|---|---|
| Canon anchor | Exact canon file / heading / row. |
| LAB support | Plan, Lean file, sample row, test, or report that supports the statement shape. |
| Evidence class | `current LAB support`, `proposal criterion`, `human/canon decision`, or `later proof/runtime`, matching `plan/130`. |
| Fresh validation command | Command that was run for the packet, not stale historical evidence. |
| Sufficiency note | Why the evidence supports the proposal without claiming proof or conformance. |
| Remaining decision | What the human/canon process must still accept or reject. |

Minimum trace requirements:

| Area | Required trace |
|---|---|
| SCN-01 static ordinary assignment | `plan/121..124`, `ELAB-11`, `ELAB-17`, and canon SCN-01 static bullets. |
| SCN-02 static ordinary assignment | `plan/121`, `plan/122`, `plan/124`, `plan/125`, `ELAB-12`, structural `ELAB-02` / `IDX-05`, and canon SCN-02 static bullets. |
| OBL-001 statement shape | `plan/73`, `plan/74`, `plan/117`, `plan/124`, `plan/130`, and the OBL-001 Lean draft. |
| OBL-020 statement shape | `plan/76`, `plan/78`, `plan/117`, `plan/126`, `plan/130`, and the OBL-020 Lean draft. |
| OBL-021 statement shape | `plan/76`, `plan/77`, `plan/117`, `plan/126`, `plan/130`, and the OBL-021 Lean draft. |
| Boundary / nonclaim trace | `plan/127`, `plan/128`, `plan/129`, and `plan/130`. |

## Open / deferral annex

The future packet must include a dedicated OPEN / deferral annex. It should not
hide unresolved choices inside positive evidence tables.

| Open item | Packet requirement |
|---|---|
| OPEN-014 read materialization | Either explicitly defer as non-blocking for static G1 statement/status or promote a separate canon decision before relying on runtime/materialization behavior. |
| OBL-020 abstraction boundary | Decide whether abstract WF preservation vocabulary is acceptable for status or list concrete binding work as a blocker. |
| OBL-021 equivalence boundary | Decide whether abstract result / diagnostic equivalence is acceptable for status or list final equality / diagnostic ABI as a blocker. |
| LAB namespace vs canon-facing wrapper | Decide whether LAB Lean statement constants can be cited as status artifacts or whether wrapper statements are needed. |
| G3 authority theorem boundary | State that authority obligations are represented only; THM-004 / OBL-015 / OBL-016 remain later. |

## Non-claim appendix

The future packet must preserve these non-claims:

- No canon edit by the outline itself.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No OBL status movement by LAB evidence alone.
- No OBL completion unless canon accepts the proposal.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new executable row.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution unless separately promoted.
- No G3 / THM-004 authority proof or production auth claim.

## Ledger delta placeholder

A future draft proposal may include a proposed ledger delta, but only as a
review artifact. The outline should require this shape:

| Field | Required content |
|---|---|
| Target row | OBL ID and current ledger row. |
| Current status | Current ledger status before proposal. |
| Proposed status | Requested new status, if any. |
| Evidence bundle | Exact evidence trace rows that justify the request. |
| Human/canon decision | `pending` until accepted. |
| Non-applied note | A statement that no ledger file is changed by the draft proposal unless explicitly promoted. |

This file does not include actual ledger patch text.

## Submission checklist

Before any future proposal packet is submitted for human/canon review, it
should pass this checklist:

| Check | Required result |
|---|---|
| Status vocabulary chosen | Each target OBL has an explicit requested status and rationale. |
| Artifact identity fixed | Lean path / namespace / constant or paper statement identity is exact. |
| Fresh compile evidence | Lean compile and sync guard commands are current to the packet. |
| No admitted stubs | `axiom`, `constant`, `sorry`, and placeholder bodies are scanned or otherwise excluded. |
| Evidence trace complete | Every positive claim maps to canon anchor plus LAB support. |
| Deferrals explicit | OPEN-014, OBL-020 abstraction, OBL-021 equivalence, LAB namespace, and G3 authority boundaries are explicit. |
| Non-claims copied | The non-claim appendix is included verbatim or updated deliberately. |
| Ledger delta is not applied | Any ledger change remains proposal text until human/canon acceptance. |

## Next allowed move

The next autonomous package can perform a docs/validation-only evidence
readiness dry-run for this outline:

- run the Lean statement draft compile / sync guard commands that a future
  packet would cite;
- scan the three OBL-001 / OBL-020 / OBL-021 Lean statement drafts for admitted
  stubs and placeholder bodies;
- record gaps in a LAB plan/report without editing canon or moving status.

If that dry-run finds a concrete blocker, the default fallback is a narrow
OBL-specific criteria refinement or guard hardening package, not canon ledger
movement.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is outline-only: no canon edit, no gate exit, no OBL status
movement, no proof, no conformance claim, no implementation change, and no
runnable sample status change.
