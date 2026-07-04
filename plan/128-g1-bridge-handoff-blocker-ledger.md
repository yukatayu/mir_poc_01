# plan/128 - G1 bridge handoff / blocker ledger

## Purpose

This file is LAB repository memory.

It turns `plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md`
into a compact handoff ledger. The goal is to classify remaining G1
ordinary-assignment bridge items by the authority that must handle them next:

- human / canon acceptance;
- future statement / proof-package work;
- canon-open / deferral decision;
- static LAB support-only evidence;
- later runtime / conformance / product work;
- reserve triggers that should not run by default.

This file does not edit canon, does not close G0 or G1, does not move
metatheory ledger status, does not complete OBL-001 / OBL-020 / OBL-021, does
not prove OBL-002, does not claim conformance, and does not change runtime,
transport, diagnostic, repair, Core IR, public API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This ledger is a routing
document for remaining work, not a source of new normative decisions.

## Classification rule

Use the categories below precisely:

| Category | Meaning | Authority |
|---|---|---|
| `human/canon acceptance` | A gate, phase, ADR, glossary, scenario, or ledger decision that LAB evidence cannot self-approve. | Human/canon process. |
| `future statement / proof-package work` | Lean statement/status completion for G1, or later proof-package work. Keep G1 statement/status separate from T2 proof skeleton / proof discharge. | Future statement/proof package plus canon ledger process. |
| `canon-open / deferral decision` | A canon OPEN item or policy that must either be resolved or explicitly deferred without pretending static LAB support closed it. | Human/canon process before implementation/conformance reliance. |
| `static LAB support-only` | Current parse / check / elaborate / statement-boundary evidence that supports G1 bridge routing but does not change canon status. | LAB plans, samples, helpers, tests, reports. |
| `later runtime / conformance / product` | Runtime serving, store mutation, occurrence ordering, admission lifecycle, transport, C-static / C-runtime / C-distributed suites, final ABI, packaging, or public product behavior. | Later phase or explicitly promoted package. |
| `reserve trigger` | A narrow event that would justify reopening a package otherwise kept closed. | Triggered only by concrete blocker, not by perceived progress. |

Do not invent a sixth "almost complete" status. If an item is not canon-accepted
or ledger-moved, keep it in LAB support or blocker form.

## Handoff ledger

| Item | Current LAB support | Category | Immediate blocker? | Future trigger / next owner | Forbidden claim |
|---|---|---|---|---|---|
| Project phase | LAB docs and reports are evidence only; canon phase authority says T0. | `human/canon acceptance` | Yes for any T0 -> T1 statement. | Human/canon process decides any transition. | No T0 -> T1 transition. |
| G0 exit | `plan/127` stays inside T0/G0 rebaseline support. | `human/canon acceptance` | Yes for G0 closeout. | Canon process and human acceptance. | No G0 exit. |
| G1 exit | `plan/121..127` route supporting evidence; no gate decision made. | `human/canon acceptance` | Yes for G1 closeout. | Human/canon process must accept theory explanation and OBL status. | No G1 exit. |
| theory/01 / theory/03 SCN-01/02 explanation | `plan/121` static spine, `plan/122` manifest, `plan/123..127` audits/maps. | `static LAB support-only` plus `human/canon acceptance` | Yes for "complete explanation"; no for continuing LAB support. | Human/canon acceptance of completeness. | No full canon closure or C-static pass. |
| OBL-001 statement/status | LAB `THM001StatementDraft.lean` compiles and `plan/124` found no immediate predicate refinement. | `static LAB support-only` now; `future statement / proof-package work` for G1 status | Yes for G1 exit; no for continuing LAB support. | Statement/status package defines completion and any ledger movement. Proof discharge is separate later work. | No OBL-001 completion or ledger movement. |
| OBL-002 proof | No proof package in current bridge. | `future statement / proof-package work` | No for G1 exit if statement/status criteria are separated; yes for proof claims and T2. | T2 proof package after statement acceptance. | No proof discharge or proof skeleton completion. |
| OBL-020 statement/status | LAB `StepWFStatementDraft.lean` compiles and `plan/126` found no immediate predicate refinement. | `static LAB support-only` now; `future statement / proof-package work` for G1 status | Yes for G1 exit; no for continuing LAB support. | Statement/status package and later proof package choose concrete configs, step labels, WF clauses, and lemmas. | No OBL-020 completion or WF proof. |
| OBL-021 statement/status | LAB `ElabDeterminismStatementDraft.lean` compiles and `plan/126` found no immediate predicate refinement. | `static LAB support-only` now; `future statement / proof-package work` for G1 status | Yes for G1 exit; no for continuing LAB support. | Statement/status package and later proof package choose final equality, diagnostic equivalence, projection-totality, and proof relation. | No OBL-021 completion or determinism proof. |
| OBL-004 corollary | Mentioned by THM-001 as later no-undeclared-communication corollary. | `future statement / proof-package work` | No for current G1 bridge support. | Keep after THM-001 statement/proof path. | No corollary completion. |
| `ELAB-11` | Exact current static SCN-01 positive evidence center. | `static LAB support-only` | No. | Cite as static evidence only. | No runtime roll behavior, conformance, or proof. |
| `ELAB-12` | Exact current static SCN-02 positive evidence center. | `static LAB support-only` | No. | Cite as static evidence only. | No runtime attack behavior, conformance, or proof. |
| `ELAB-17` | Exact current static SCN-01 `VisibilityDenied` negative evidence, rejected-row failure-containment pressure. | `static LAB support-only` | No. | Cite for rejected-row pressure only; keep diagnostic / repair payload with OBL-024/025 support. | No successful THM-001 instance; no final Diagnostic / repair ABI. |
| `ELAB-02/05/07/09/10` and `IDX-05` | Structural support for request shape, spans, E-ROW pressure, visible publish / observe, and non-ambient-authority reading. | `static LAB support-only` | No. | Cite only as structural support unless a later package actualizes a narrower row. | No exact SCN pass, conformance, or proof. |
| SCN-02 direct-local-write negative (b) | Non-blocking after `plan/125`; only structural support exists. | `reserve trigger`; later `runtime / conformance` only if promoted | No. | Add an exact negative row only if the claim changes or a conformance / second-elaborator trigger appears. | No exact executable negative evidence claim. |
| OPEN-014 read materialization | Dependency rows are explicit; materialization policy remains open. | `canon-open / deferral decision` | Yes for read-materialization claims; no for dependency-recording LAB support. | Decide or explicitly defer policy in canon/theory before runtime or projection commitments. | No OPEN-014 resolution. |
| Authority / capability obligations | Current bridge carries obligation representation only. | `static LAB support-only`; later G3 statement/proof/runtime | No for G1 static support; yes for authority claims. | G3 / THM-004 and production admission remain later. | No authority soundness or production auth claim. |
| Visible publish / observe | Current rows show static consequence pressure. | `static LAB support-only`; later `runtime / conformance` | No for static bridge; yes for runtime/ABI claims. | Runtime dispatch and telemetry/viewer ABI belong later. | No runtime `MessageEnvelope` dispatch or final viewer/telemetry ABI. |
| Diagnostic / repair projection in rejected rows | Useful OBL-024 / OBL-025 pressure. | `static LAB support-only`; later `future statement / proof-package work` | No for G1 bridge; yes for diagnostic/repair proof claims. | Keep outside OBL-001; future packages handle OBL-024/025. | No final Diagnostic / repair ABI or OBL-024/025 proof. |
| C-static conformance | Not claimed by exact LAB rows. | `later runtime / conformance / product` | Yes for conformance claims; no for docs-only bridge support. | Create only under an explicitly promoted conformance package. | No C-static pass. |
| Runtime request serving / store mutation / ordering | Explicitly out of scope for the static bridge. | `later runtime / conformance / product` | Yes for runtime claims; no for static bridge. | Later I-phase or promoted runtime package. | No runtime dispatch or store behavior. |
| Admission lifecycle / stale membership runtime failure | Out of current static bridge scope. | `later runtime / conformance / product` | Yes for runtime/admission claims. | Later runtime/admission package. | No production membership or stale-runtime behavior. |
| Transport / distributed execution | Out of G1 static bridge and forbidden as a G1 claim. | `later runtime / conformance / product` | Yes for distributed runtime claims. | Later I3+ or explicitly promoted distributed package. | No distributed runtime completion. |
| Final grammar / Core IR / Diagnostic / repair / projection / public API ABI | Not frozen by current evidence. | `later runtime / conformance / product` | Yes for final-public claims. | Later public-boundary package. | No final ABI freeze. |

## Current default

Default action after this ledger:

```text
continue docs-only handoff / acceptance preparation
do not add rows
do not refine Lean predicates
do not edit canon
do not claim gate movement
```

This default changes only if a concrete blocker matches a reserve trigger.

## Reserve triggers

| Reserve package | Trigger | Do not run for |
|---|---|---|
| SCN-02 exact direct-local-write negative row | Need to cite exact negative evidence, start C-static negative suite, add a second frontend/elaborator, introduce owner-local write Core IR artifact, or strengthen the claim to exclude co-emitted direct local artifacts. | General progress, cosmetic symmetry, or "nice to have" coverage. |
| OBL-001 predicate refinement | Found missing abstraction that cannot carry `ELAB-11/12/17`, proof-package blocker, or sync-guard drift. | Desire to mirror helper JSON names or scenario vocabulary. |
| OBL-020 predicate refinement | Future proof package introduces concrete step family / WF clauses and finds the aggregate shape insufficient. | Static bridge support work. |
| OBL-021 predicate refinement | Future proof package chooses final equality / diagnostic equivalence and finds the abstract shape insufficient. | Static bridge support work. |
| Canon clarification proposal | Real canon wording gap around ordinary assignment, dependency recording, nested-locus non-authority, or LAB/canon citation rules. | Merely summarizing existing canon and LAB evidence. |
| Runtime / conformance package | Explicit promotion into a runtime / conformance phase or package. | G1 static bridge support. |

## Misclassification guards

- Human/canon acceptance items must not be recorded as closed by LAB plans.
- Future proof-package items must not be downgraded to docs-only support just
  because Lean drafts compile.
- Static LAB support-only rows must not be promoted to C-static conformance.
- Runtime / conformance / product work must not be pulled into G1 static bridge
  just because a scenario includes runtime expectations.
- Reserve triggers must stay event-based. They are not backlog tasks to run in
  sequence by default.

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

The next safe package can prepare a G1 acceptance-packet preflight, still
docs-only:

- list the exact canon files a human would need to accept or update;
- list the exact LAB evidence files that support each acceptance point;
- list the exact statement/status blockers that prevent G1 OBL ledger movement;
- list the later proof-package blockers that belong to T2 or later;
- keep runtime / conformance / product work out of the acceptance packet.

This should remain a preflight unless the user explicitly asks to start a
canon edit or proof package.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is ledger-only: no canon edit, no gate exit, no proof, no
conformance claim, no implementation change, and no runnable sample status
change.
