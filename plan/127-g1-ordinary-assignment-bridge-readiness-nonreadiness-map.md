# plan/127 - G1 ordinary-assignment bridge readiness / non-readiness map

## Purpose

This file is LAB repository memory.

It records the post-`plan/126` readiness / non-readiness map for the current
G1 ordinary-assignment bridge. "Readiness" here means only that the next narrow
LAB support package can continue without adding a new executable row, Lean
predicate refinement, or canon wording proposal by default.

It does not mean G1 exit readiness, T1 readiness, conformance readiness,
runtime readiness, proof readiness, or product readiness.

This file does not edit canon, does not close G0 or G1, does not move
metatheory ledger status, does not complete OBL-001, OBL-020, or OBL-021, does
not prove OBL-002, does not claim C-static conformance, and does not change
runtime, transport, diagnostic, repair, Core IR, public API, grammar, or sample
status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is a bridge map,
not a source of new normative decisions.

## Current phase reading

The canon phase plan still places the project in T0. G1 ordinary assignment is
the nearest theory-facing line inside that T0/G0 rebaseline work, but G1 exit
requires the canon process described by `mirrorea_canon/plan/00-gates.md` and
`mirrorea_canon/plan/01-phases.md`.

The useful planning reading after `plan/126` is:

- full project phase: T0 of T0, T1, T2, I1, I2, I3, I4, I5, I6;
- current local line: late T0/G0 rebaseline support for G1 ordinary
  assignment;
- Macro axis pressure: theorem / model-check / verifier bridge, especially
  OBL-001 / OBL-020 / OBL-021 statement-boundary support.

This is a rough management reading only. It is not a percentage completion
claim, gate movement, or implementation-state claim.

## Bridge map

| Canon G1 criterion or pressure | Current LAB support | Bridge reading | Remaining blocker | Forbidden claim |
|---|---|---|---|---|
| `theory/01` / `theory/03` explain SCN-01 and SCN-02 ordinary assignment | `plan/121` defines the static assignment spine; `plan/122` maps SCN-01/02 static bullets; `ELAB-11` and `ELAB-12` are exact current static evidence centers; `ELAB-17` covers the exact SCN-01 `VisibilityDenied` negative; `plan/125` keeps SCN-02 direct-local-write negative (b) non-blocking. | Good enough to continue a LAB bridge without broadening runtime or product scope. | Human/canon gate review has not accepted G1 exit. Runtime SCN bullets and C-static conformance remain outside this map. | Do not claim SCN-01/02 are fully canon-closed, C-static conformant, runtime-complete, or G1-exited. |
| OBL-001 Lean statement | `THM001StatementDraft.lean` compiles as a LAB `Prop` shape; `plan/117` guards body-level assignment postcondition links; `plan/124` audits `ELAB-11`, `ELAB-12`, and `ELAB-17` and finds no Lean predicate refinement needed now. | Statement boundary is sufficient for the current static bridge. | Canon ledger still records all entries as open at v0.1.0. OBL-001 completion is not declared in canon. OBL-002 proof is later. | Do not claim OBL-001 completion, OBL-002 proof, proof skeleton completion, or canon ledger movement. |
| OBL-020 well-formedness preservation | `StepWFStatementDraft.lean` compiles as a LAB `Prop` shape; `plan/126` audits the boundary and finds no Lean predicate refinement needed for the current bridge. | No new OBL-020 predicate or test is needed before the next support package. | Concrete `Config`, `StepLabel`, `StepFamily`, `WellFormed` clauses, and per-step preservation lemmas are future proof-package work. | Do not claim WF preservation proof, OBL-020 completion, or runtime implementation proof. |
| OBL-021 elaboration determinism | `ElabDeterminismStatementDraft.lean` compiles as a LAB `Prop` shape; `plan/126` keeps final equality / projection-totality / diagnostic ABI outside the draft and hardens sync tests against bare `:= True` and comment-only body links. | No new OBL-021 predicate refinement is needed before the next support package. | Final result equality, diagnostic equivalence contract, projection totality, and proof relation are not chosen. | Do not claim elaboration determinism proof, OBL-021 completion, final equality selection, or runtime scheduling determinism. |
| RHS dependency and OPEN-014 | `ELAB-11` records the SCN-01 same-field RHS dependency; `ELAB-12` records target/self RHS dependencies for SCN-02; `plan/75` and `plan/122` keep dependency rows explicit. | Dependency-recording evidence is enough for the static bridge. | OPEN-014 read materialization, cache, freshness, reply, and transport policy remain unresolved. | Do not claim read materialization policy, cross-locus read transport, cache semantics, or OPEN-014 resolution. |
| Authority / capability pressure | `plan/121`, `plan/122`, `plan/124`, and `ELAB-11/12` carry authority-obligation representation and owner-directed request shape. | Obligation-carrier support is present for G1 static assignment. | G3 / THM-004 authority soundness, grant-lineage proof, production auth, membership, and witness freshness remain later. | Do not claim authority theorem proof, production authentication, or membership/capability implementation completion. |
| SCN-02 direct-local-write negative (b) | `plan/125` reviews the gap and keeps it non-blocking using exact positive `ELAB-12` plus structural `ELAB-02` / `IDX-05` support. | No new executable negative row is needed for this bridge checkpoint. | A dedicated negative row is still needed later if the project wants exact negative evidence or a C-static negative suite. | Do not claim exact executable negative evidence for SCN-02 negative (b). |
| Runtime behavior | None in this map. `plan/122` explicitly marks request serving, store mutation, ordering, admission lifecycle, stale-membership runtime failure, and distributed transport as out of scope. | Runtime work should not be imported into the static bridge. | Later runtime / conformance / implementation phases. | Do not claim runtime dispatch, request serving, store mutation, occurrence ordering, distributed transport, or product behavior. |

## Readiness statement

The current LAB bridge is ready to continue in the narrow sense that no
immediate new executable row, Lean predicate refinement, or canon clarification
proposal is justified by `plan/121..126`.

The current project is not ready to claim G1 exit. G1 exit remains blocked by
canon acceptance and by the fact that OBL-001, OBL-020, and OBL-021 are not
completed in the canon metatheory ledger.

## Non-readiness blockers

These are the blockers that must remain visible:

- canon phase status remains T0;
- G1 exit requires human/canon decision, not only LAB evidence;
- `mirrorea_canon/theory/11-metatheory-ledger.md` is the only proof-status
  authority and still lists entries as open at v0.1.0;
- OBL-001 / OBL-020 / OBL-021 LAB Lean drafts are compile-check-only;
- OBL-002 proof is not discharged;
- OPEN-014 remains unresolved;
- SCN runtime behavior is out of scope;
- SCN-02 direct-local-write negative (b) is not exact executable negative
  evidence;
- authority evidence is obligation-carrier support only, not G3 / THM-004
  authority soundness.

## Why not add more now

No new executable row is the smallest move because the only remaining SCN-02
direct-local-write negative gap is explicitly non-blocking for the current
bridge after `plan/125`.

No Lean predicate refinement is the smallest move because `plan/124` and
`plan/126` both found that the current OBL-001 / OBL-020 / OBL-021 statement
boundaries are sufficiently abstract for this checkpoint.

No canon wording proposal is the smallest move because no concrete canon
wording gap has been found for ordinary assignment. Current canon BND-001,
static semantics, scenario text, and ledger boundaries are enough to keep the
LAB bridge honest.

## Trigger matrix

| Future action | Trigger | Still forbidden |
|---|---|---|
| Add an executable row | Only if the project wants exact SCN-02 direct-local-write negative evidence, a C-static negative suite begins, a second frontend / elaborator appears, Core IR grows an owner-local write artifact that could collide with request artifacts, or the claim changes from "request edge exists" to "no direct local write artifact can also be emitted". | Do not add rows only to improve perceived progress. Do not claim conformance from a LAB row. |
| Refine OBL-001 / OBL-020 / OBL-021 Lean predicates | Only if a concrete missing abstraction, overfit, proof-package blocker, or sync-guard drift appears. | Do not refine because a docs map feels incomplete. Do not import helper JSON, scenario names, final ABI fields, or runtime scheduler vocabulary. |
| Draft a canon clarification proposal | Only if a real canon wording gap is found around ordinary assignment, dependency recording, nested-locus non-authority, or bridge/canon citation rules. | Do not edit canon or imply gate movement from LAB evidence alone. |
| Start runtime / conformance work | Only after the relevant phase / gate path is explicitly promoted. | Do not fold runtime dispatch, store mutation, admission, stale behavior, or transport into G1 static bridge work. |

## Review inputs

A read-only sidecar reviewer for this package agreed with the smallest safe
shape: write a docs-only readiness / non-readiness map. The reviewer highlighted
four overclaim risks:

- "G1 readiness audit" can be misread as G1 exit or T1 readiness;
- exact current executable evidence remains LAB-static evidence, not
  conformance, proof, runtime behavior, or final ABI;
- `plan/126` hardens sync guards only and does not complete OBL-020 / OBL-021;
- SCN-02 direct-local-write remains structural support only, not exact negative
  evidence.

The same review recommended no executable row, no Lean refinement, and no canon
proposal by default.

Oracle consult `you-are-advising-on-a-2` independently reached the same
decision: use bridge-readiness as readiness-to-continue LAB support work, not
G1-ready status; make blocker and trigger tables explicit; keep executable
rows, Lean refinements, and canon proposals reserve-only.

## Hidden failure modes

| Hidden failure mode | Guard |
|---|---|
| "Bridge readiness" reads as G1 exit. | Say readiness-to-continue LAB support work, not G1-ready. |
| "Exact executable evidence" reads as C-static conformance. | Say exact only for the current LAB static manifest. |
| `ELAB-17` reads as a successful THM-001 instance. | Say rejected-row failure-containment pressure only. |
| Diagnostic / repair payloads get pulled into OBL-001. | Keep them under OBL-024 / OBL-025 pressure only. |
| `ELAB-12` plus structural rows reads as exact SCN-02 direct-local-write negative evidence. | Say non-blocking structural support only. |
| Dependency rows read as OPEN-014 resolution. | Say dependency / read consequence is explicit; materialization policy remains open. |
| Owner-directed request evidence reads as G3 authority proof. | Say obligation representation only, no grant-lineage soundness. |
| Publish / observe rows read as runtime `MessageEnvelope` dispatch. | Say static consequence only, no runtime dispatch or final ABI. |
| Lean compile-check reads as proof or ledger movement. | Cite the canon ledger as authority and keep status unchanged. |
| Docs package reads as canon edit. | Use proposal-only trigger language for any future canon clarification. |
| Scenario terms read as Mir core vocabulary. | Keep `World`, `BrowserClient`, `S`, `player`, `hp`, and `position` as scenario / sample vocabulary only. |
| Package label reads as a promoted Surface package. | Call it a LAB map / report, not a promoted implementation package. |

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

The next safe package should stay docs-only unless a concrete blocker appears.

The narrow autonomous follow-up is a G1 bridge handoff / blocker ledger: turn
this map into a compact list of remaining canon-facing closeout checks and
explicitly separate:

- items that need human/canon acceptance;
- items that need future proof-package work;
- items that are static LAB support only;
- items that are later runtime / conformance / product work.

Executable rows, Lean statement refinements, and canon wording proposals should
remain reserve-only until that handoff finds a concrete missing artifact.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is map-only: no canon edit, no gate exit, no proof, no
conformance claim, no implementation change, and no runnable sample status
change.
