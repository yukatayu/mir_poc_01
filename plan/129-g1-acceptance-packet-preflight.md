# plan/129 - G1 acceptance-packet preflight

## Purpose

This file is LAB repository memory.

It prepares, but does not execute, a future G1 ordinary-assignment acceptance
packet. The goal is to list the exact canon files a human/canon review would
need to accept or update, the exact LAB evidence files that can support each
acceptance point, the statement/status blockers that prevent OBL ledger
movement, and the later proof / runtime / conformance / product exclusions that
must stay outside the packet.

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

If LAB evidence conflicts with canon, canon wins. This file is a preflight
checklist for a later acceptance process, not a source of new normative
decisions.

## Preflight reading

The canon phase plan still places the project in T0. The G1 ordinary-assignment
line is the nearest theory-facing line because `mirrorea_canon/plan/00-gates.md`
defines G1 exit as ordinary assignment elaboration theory: `theory/01` and
`theory/03` must completely explain SCN-01 / SCN-02, OBL-001 Lean statement
must be complete, and OBL-020 / OBL-021 must be complete.

This preflight finds that the repo is ready to assemble a review packet, but it
is not ready to claim G1 acceptance or gate movement. The remaining blocking
work is mostly acceptance / ledger-status work, not another default executable
row or default Lean predicate refinement.

## Canon files in the acceptance surface

| Canon file | Why it is in scope | Default action in this preflight |
|---|---|---|
| `mirrorea_canon/plan/00-gates.md` | Defines G1 exit criteria and states that exit requires human decision plus ADR / ledger update. | Read-only citation. Do not change gate text. |
| `mirrorea_canon/plan/01-phases.md` | Defines T0/T1/T2 and says the current implementation state authority is T0. | Read-only citation. Do not claim T0 -> T1 transition. |
| `mirrorea_canon/theory/01-mircore-v0.md` | Defines assignment, Core `read` / `write` / `request`, runtime configuration shape, OBL-020, and nested locus block non-ambient-authority behavior. | Read-only citation. Do not import runtime step completion into G1 static support. |
| `mirrorea_canon/theory/03-elaboration.md` | Defines BND-001, THM-001, OBL-001 / OBL-002 status pointer, determinism pressure, and OPEN-014. | Read-only citation. Treat OPEN-014 as open or explicitly deferred later. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | The only canon proof/status authority for THM / OBL entries. All entries remain open at v0.1.0 unless this file changes. | Read-only citation. Do not move status here. |
| `mirrorea_canon/spec/03-static-semantics.md` | Surface-visible static obligations for declaration, indexed state, failure-row containment, visibility, cross-locus access, and nested non-ambient authority. | Read-only citation. |
| `mirrorea_canon/spec/06-conformance.md` | Defines C-static / C-runtime / C-distributed and prevents LAB rows from being mistaken for conformance. | Read-only citation. |
| `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md` | L0-frozen ordinary-assignment roll scenario: request edge, dependency row, visible publish, spans, capability obligation, runtime expectations, and `VisibilityDenied` negative. | Read-only citation. Static bullets only are relevant to this preflight. |
| `mirrorea_canon/scenarios/SCN-02-attack.md` | L0-frozen canonical attack scenario: request edge, target/self dependency rows, failure containment, nested non-ambient authority, runtime expectations, and negative variants. | Read-only citation. Static bullets only are relevant to this preflight. |

## Acceptance packet map

| Acceptance point | Canon files | LAB evidence supporting review | Current blocker | Next owner | Forbidden claim |
|---|---|---|---|---|---|
| Current phase / gate authority | `plan/00-gates`, `plan/01-phases` | `plan/127`, `plan/128`, `progress.md`, `tasks.md` | Human/canon acceptance has not happened. Canon still says T0. | Human/canon process. | No G0 exit, G1 exit, or T0 -> T1 transition. |
| SCN-01 static ordinary assignment explanation | `theory/01`, `theory/03`, `spec/03-static-semantics`, `SCN-01` | `plan/121`, `plan/122`, `plan/123`, `plan/124`; exact LAB evidence center `ELAB-11`; exact SCN-01 `VisibilityDenied` negative `ELAB-17` | Human/canon review must decide whether this is sufficient explanation for the static G1 portion. Runtime roll behavior is excluded. | Future acceptance packet / human review. | No C-static pass, runtime roll behavior, store mutation, or conformance claim. |
| SCN-02 static ordinary assignment explanation | `theory/01`, `theory/03`, `spec/03-static-semantics`, `SCN-02` | `plan/121`, `plan/122`, `plan/124`, `plan/125`; exact positive evidence center `ELAB-12`; structural non-ambient support from `ELAB-02` / `IDX-05` | Human/canon review must accept the static explanation. SCN-02 direct-local-write negative (b) remains non-blocking structural support, not exact executable negative evidence. | Future acceptance packet / human review. | No exact executable negative evidence claim for SCN-02 negative (b); no runtime attack behavior or conformance claim. |
| Failure-row containment and generated visible failure pressure | `theory/03`, `spec/03-static-semantics`, `SCN-01`, `SCN-02` | `plan/122`, `plan/123`, `plan/124`; `ELAB-17` as rejected-row pressure; `ELAB-10` and `ELAB-13..16` only as LAB diagnostic / repair support | Acceptance must keep diagnostic / repair payload details outside OBL-001. | Future acceptance packet plus later OBL-024/025 packages. | No final Diagnostic / repair ABI, OBL-024/025 proof, or successful THM-001 instance from rejected rows. |
| RHS dependency and read-consequence recording | `theory/01`, `theory/03`, `SCN-01`, `SCN-02` | `plan/75`, `plan/121`, `plan/122`, `plan/127`; `ELAB-11` same-field dependency and `ELAB-12` target/self dependencies | OPEN-014 read materialization remains open; static dependency rows do not choose cache / freshness / transport policy. | Canon-open / deferral decision. | No OPEN-014 resolution, read materialization policy, cache semantics, or transport claim. |
| Nested locus block is not ambient authority | `theory/01`, `theory/03`, `spec/03-static-semantics`, `SCN-02` | `plan/121`, `plan/122`, `plan/124`, `plan/125`; exact positive `ELAB-12`; structural `ELAB-02` / `IDX-05` | Acceptance must avoid overreading support as exact negative (b) evidence. | Future acceptance packet; reserve executable row only if triggered. | No exact direct-local-write negative evidence unless a later row is explicitly added. |
| Authority-obligation carrier support | `theory/01`, `theory/03`, `spec/03-static-semantics`, `SCN-01`, `SCN-02`; later `theory/05` for authority theorem | `plan/121`, `plan/122`, `plan/124`, `plan/127`, `plan/128`; `ELAB-11` / `ELAB-12` obligation-carrier evidence | This is G1 static obligation representation only. G3 / THM-004 authority soundness remains later. | Future acceptance packet for G1 static support; later G3 package for authority. | No production auth, grant-lineage proof, membership implementation, or THM-004 proof claim. |
| OBL-001 statement/status | `plan/00-gates`, `theory/03`, `theory/11-metatheory-ledger` | `plan/73`, `plan/74`, `plan/117`, `plan/124`, `plan/127`, `plan/128`; `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | Canon ledger still records entries as open. A future statement/status package must define completion and any ledger movement. | Future statement/status package plus canon ledger process. | No OBL-001 completion, OBL-002 proof, proof skeleton completion, or ledger movement. |
| OBL-020 statement/status | `plan/00-gates`, `theory/01`, `theory/11-metatheory-ledger` | `plan/76`, `plan/78`, `plan/117`, `plan/126`, `plan/127`, `plan/128`; `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | Concrete config / step label / step family / WellFormed clauses and lemma boundaries are future statement/status and later proof-package work. | Future statement/status package plus canon ledger process. | No OBL-020 completion, WF preservation proof, or runtime implementation proof. |
| OBL-021 statement/status | `plan/00-gates`, `theory/03`, `theory/11-metatheory-ledger` | `plan/76`, `plan/77`, `plan/117`, `plan/126`, `plan/127`, `plan/128`; `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | Final result equality, diagnostic equivalence contract, projection totality, and proof relation are not canon-chosen. | Future statement/status package plus canon ledger process. | No OBL-021 completion, determinism proof, final equality selection, or runtime scheduling determinism. |
| OBL-002 / OBL-004 later theorem work | `theory/03`, `theory/11-metatheory-ledger`, `plan/01-phases` | `plan/127`, `plan/128` only as blocker routing | Proof discharge is not a G1 gate-exit requirement under `plan/00`, but proof skeletons belong to T2. | Later proof package / T2. | No proof discharge, proof skeleton completion, or no-undeclared-communication corollary completion. |
| C-static / runtime / distributed behavior | `spec/06-conformance`, `SCN-01`, `SCN-02`, `plan/01-phases` | `plan/122`, `plan/127`, `plan/128` as exclusions | Conformance and runtime are not part of the G1 static bridge acceptance packet. | Later explicitly promoted conformance / runtime package. | No C-static, C-runtime, C-distributed, runtime request serving, store mutation, occurrence ordering, admission lifecycle, stale-membership runtime failure, or distributed transport claim. |

## Statement/status blockers

Before any G1 OBL status movement, a future package must answer at least these
questions in writing:

- What exact criterion makes the LAB OBL-001 Lean statement complete enough for
  canon ledger movement?
- What exact criterion makes the LAB OBL-020 statement complete enough for
  G1, while leaving per-step proof discharge to later T2 work?
- What exact criterion makes the LAB OBL-021 statement complete enough for G1,
  while leaving final equality / diagnostic equivalence / projection-totality
  proof relations unresolved or abstract where appropriate?
- Does OBL-020 / OBL-021 "complete" in G1 mean `lean-stated`, `stated`, or a
  different canon-status movement? The canon ledger currently defines allowed
  status vocabulary, but this preflight does not choose a new status.
- What, if anything, must be mirrored into `mirrorea_canon/theory/11-metatheory-ledger.md`
  when status movement is eventually accepted?

## Canon-open / deferral blockers

OPEN-014 is the main canon-open item in this packet. Dependency rows and
read-request / observe consequences are explicit enough for current static LAB
support, but materialization, caching, freshness, reply carrier, and transport
policy are not selected.

A later acceptance packet should either:

- explicitly defer OPEN-014 as non-blocking for G1 static acceptance; or
- resolve the relevant materialization policy in canon before relying on it for
  runtime, projection, or conformance claims.

This preflight does neither.

## Packet components for a later human review

A future G1 acceptance packet should be split into these parts:

| Component | Contents | Must not include |
|---|---|---|
| Canon-state summary | Current T0 phase, G1 criteria, status authority, and no-conformance boundary. | Any gate movement by implication. |
| Static explanation annex | SCN-01 / SCN-02 static bullets, the corresponding canon clauses, and LAB evidence links. | Runtime SCN bullets as if already implemented. |
| Statement/status annex | OBL-001 / OBL-020 / OBL-021 current LAB draft paths, guard evidence, and proposed status criteria. | Proof discharge or final proof relation choices unless explicitly promoted. |
| OPEN / deferral annex | OPEN-014 decision or deferral wording, and why static dependency evidence remains sufficient. | Cache / transport / projection policy by accident. |
| Non-claim appendix | Forbidden claims and later-phase work. | Product, runtime, conformance, or final ABI promises. |

## Reserve-only work

Do not run these by default after this preflight:

- executable SCN-02 direct-local-write negative row;
- OBL-001 predicate refinement;
- OBL-020 predicate refinement;
- OBL-021 predicate refinement;
- canon clarification proposal;
- conformance or runtime package.

They should trigger only under the reserve conditions in `plan/127` and
`plan/128`, or under an explicit user/canon promotion.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No OBL status movement.
- No OBL completion.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new executable row.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, or distributed
  transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, or grammar ABI freeze.
- No sample status relabel.
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof or production auth claim.

## Next allowed move

The next narrow autonomous package can remain docs-only and define a G1
OBL-001 / OBL-020 / OBL-021 statement/status completion-criteria inventory.
That package should not move the canon ledger. It should only prepare the exact
criteria that a later human/canon acceptance packet would need before ledger
status movement.

If the user explicitly promotes acceptance work, the next package should draft
a review packet rather than editing canon directly: the packet should quote the
canon-state summary, evidence annex, statement/status annex, OPEN / deferral
annex, and non-claim appendix above.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is preflight-only: no canon edit, no gate exit, no proof, no
conformance claim, no implementation change, and no runnable sample status
change.
