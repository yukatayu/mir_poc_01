# plan/130 - G1 OBL statement/status completion criteria inventory

## Purpose

This file is LAB repository memory.

It records a docs-only inventory of what would have to be true before
OBL-001 / OBL-020 / OBL-021 status movement could be proposed to the canon
metatheory ledger for G1 ordinary assignment.

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

If LAB evidence conflicts with canon, canon wins. This file prepares criteria
for a later proposal; it is not itself a status proposal.

## Why this inventory exists

`mirrorea_canon/plan/00-gates.md` says G1 exit requires:

- `theory/01` and `theory/03` completely explaining SCN-01 / SCN-02;
- OBL-001 Lean statement completion;
- OBL-020 / OBL-021 completion.

`mirrorea_canon/theory/11-metatheory-ledger.md` is the only proof/status
authority and currently says all entries are open at v0.1.0 unless noted.
`plan/129` therefore leaves actual acceptance and ledger movement to a later
human/canon process.

This file defines the checklist that such a later process would need before it
can even propose status movement.

## Criteria status legend

This inventory separates criteria from achievement:

| Label | Meaning |
|---|---|
| `current LAB support` | Evidence exists in LAB plans, Lean drafts, tests, or samples and can support a later proposal. |
| `proposal criterion` | A condition a later status proposal must explicitly satisfy or argue for. |
| `human/canon decision` | A criterion that only the human/canon process can accept. |
| `later proof/runtime` | Work that must stay outside G1 statement/status movement. |

No row in this file means a criterion is already canon-accepted.

## Criteria categories

| Category | What a later packet must contain | Current reading |
|---|---|---|
| Authority / status criteria | Canon files cited, requested ledger status named, proposed ledger delta described, and human/canon acceptance path named. | Not satisfied; this inventory only prepares the shape. |
| Common Lean statement criteria | Exact Lean draft identity, successful compile-check evidence, no admitted stubs such as `axiom`, `constant`, or `sorry`, body-link / vacuity guards, and mapping to the ledger target. | Partially supported by LAB drafts and guards; not canon-accepted. |
| Canon traceability criteria | Every required predicate maps to `theory/01`, `theory/03`, SCN-01 / SCN-02 static pressure, and the correct OBL row. | Supported by `plan/73`, `plan/76`, `plan/121..129`; human/canon sufficiency remains open. |
| Open / deferral criteria | OPEN-014, equality / diagnostic equivalence, and WF abstraction choices are either explicitly deferred or listed as blockers. | Not satisfied; this is future packet work. |
| Acceptance trigger criteria | The packet names what new evidence or human decision would allow ledger movement to be proposed. | Not satisfied; no trigger has been accepted. |

## Common criteria before any status proposal

| Criterion | Required before status proposal | Current LAB support | Still open |
|---|---|---|---|
| Canon scope alignment | The proposal must name the canon statement being moved and quote the relevant canon anchors. | `plan/73`, `plan/76`, `plan/127..129` already cite `plan/00`, `plan/01`, `theory/01`, `theory/03`, and `theory/11`. | Human/canon review must accept that the named statement matches canon intent. |
| Status target | The proposal must say whether the requested ledger status is `stated`, `lean-stated`, or another allowed ledger status. | The ledger's allowed vocabulary is known. | Current LAB files do not choose a ledger status. |
| Artifact identity | The proposal must name the exact Lean file, namespace, and statement constant if `lean-stated` is proposed. | Current files live under `samples/lean/lab-statements/obl001`, `obl020`, and `obl021`. | Canon has not accepted these LAB paths as ledger targets. |
| Compile-check evidence | If `lean-stated` is proposed, the file must compile under the repo Lean toolchain and be registered in the LAB Lean manifest. | Current drafts compile in prior LAB checks and are covered by sync tests. | P76 does not rerun Lean or promote the result. |
| No admitted stubs | If `lean-stated` is proposed, the statement file must not rely on `axiom`, `constant`, `sorry`, or a placeholder theorem body to get accepted. | Current LAB guards check selected vacuity patterns and body links. | A future packet must run and cite the exact Lean / guard checks used for this criterion. |
| Non-vacuity / drift guard | The statement body must be protected from obvious vacuity and body-link drift. | `plan/117` and `plan/126` harden body-level sync guards. | Sync guards are LAB evidence, not proof and not canon acceptance. |
| Ledger target mapping | The artifact identity must map back to the ledger target namespace in `theory/11`, for example `MirCore.Elab.Soundness`, `MirCore.Step.WF`, or `MirCore.Elab.Det`. | Current LAB namespaces are deliberately `MirCore.Lab...`, so they avoid implying canon status. | A future packet must decide whether LAB namespaces are acceptable as evidence or whether canon-facing wrapper names are required. |
| Evidence trace | The proposal must map the statement to SCN-01 / SCN-02 static pressure and to the relevant LAB rows without claiming conformance. | `plan/121..129` provide the trace. | Human/canon review must accept sufficiency. |
| Boundary statement | The proposal must list proof, runtime, conformance, diagnostic / repair ABI, authority theorem, and OPEN-014 exclusions. | `plan/127..129` list non-claims and blockers. | The future packet must preserve them verbatim or update them deliberately. |

## Status vocabulary reading

The canon ledger defines status vocabulary, but the current G1 docs do not
state exactly which status movement should count as "completion" for each G1
OBL. A future packet must decide this explicitly.

| Candidate status | Meaning in a future packet | P76 recommendation |
|---|---|---|
| `stated` | Canon accepts a precise mathematical statement, even if not in Lean. | Usable only if the project wants paper statement acceptance before Lean path acceptance. |
| `lean-stated` | Canon accepts a Lean statement artifact as the statement identity. | Most natural candidate for OBL-001 because `plan/00` says "Lean statement"; plausible for OBL-020/021 only if canon accepts the current abstract Lean statement shapes. |
| `lean-proved` | Statement has a Lean proof. | Out of scope for G1 status-prep and later than this inventory. |
| `external` | Accepted outside Lean. | Not currently justified for OBL-001/020/021. |

P76 does not choose one. The next status-movement proposal must.

## Acceptance trigger criteria

Before a later packet can propose ledger movement, it should identify the exact
trigger that made proposal reasonable:

| Trigger | What it would allow | What it still would not allow |
|---|---|---|
| Human/canon accepts the current abstract statement shape | A status proposal can point at the accepted abstraction and proposed ledger delta. | Proof discharge, conformance, runtime behavior, or G1 exit by itself. |
| Human/canon requires a canon-facing wrapper statement | A docs/Lean package can prepare a wrapper around current LAB predicates. | Silent namespace promotion or ledger movement. |
| A proof package finds a missing abstraction | A narrow OBL-specific predicate refinement may be opened. | Broad rewrite of OBL-001/020/021 or runtime import by default. |
| A conformance package is explicitly promoted | LAB rows can be reclassified under a conformance workflow. | Retroactive C-static claim for existing LAB rows. |

Absent one of these triggers, the current OBL-001/020/021 drafts remain
current LAB support only.

## OBL-001 criteria

OBL-001 is the THM-001 Lean statement for assignment elaboration soundness.

| Criterion | Required before proposing OBL-001 status movement | Current support | Still open |
|---|---|---|---|
| Statement target identity | Name the accepted statement target for THM-001 / assignment elaboration soundness. | `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` defines `THM001StatementDraft`. | Canon has not accepted that file / namespace / constant as OBL-001. |
| Assignment scope | State whether the accepted statement is simple assignment only and how compound assignment is deferred. | `plan/73` and `THM001StatementDraft.lean` keep `SurfaceAssignment` and `SimpleAssign`. | Human/canon review must accept this as sufficient for G1 ordinary assignment. |
| Successful elaboration postcondition | Generated writes are owner-local or backed by owner-directed requests carrying authority, failure, dependency, and span evidence. | `GeneratedWriteSound`, `RequestEvidenceSound`, and guard links in `plan/117`. | No proof that any implementation satisfies the predicates. |
| RHS dependency coverage | RHS reads are recorded at result level and request level. | `AllRhsReadsRecorded`, `RequestCarriesDependencyEvidence`, `plan/75`, `ELAB-11`, `ELAB-12`. | OPEN-014 materialization remains unresolved. |
| Failure containment | Generated failures are contained in declared rows. | `GeneratedFailuresContained`, `RequestCarriesFailureContainment`, `plan/123`, `plan/124`. | Rejected diagnostic details stay outside OBL-001. |
| Authority obligations | Obligations are represented, not proved. | `AuthorityObligationsRepresented`, `RequestCarriesAuthorityObligations`. | G3 / THM-004 authority soundness remains later. |
| Source spans | Assignment-caused consequences preserve source-span evidence. | `SourceSpansPreserved`, `RequestCarriesSpanEvidence`. | No final source map ABI is frozen. |
| Visible consequences | Visible writes make publish / observe consequences explicit. | `VisibleWriteConsequencesExplicit`, `ELAB-11`, `ELAB-17` context. | No final viewer / telemetry / publish ABI. |
| Nested locus non-authority | Nested locus block does not mint ambient authority. | `NoAmbientAuthorityFromNestedLocus`, `plan/125`, `ELAB-12`, structural `ELAB-02` / `IDX-05`. | SCN-02 direct-local-write negative (b) remains structural support only. |

OBL-001 status movement must not include OBL-002 proof, OBL-004 corollary,
OBL-020 proof, OBL-021 proof, diagnostic / repair payload proof, runtime
dispatch, or C-static conformance.

## OBL-020 criteria

OBL-020 is well-formedness preservation of step rules. It is broader than the
ordinary-assignment static bridge, so a future packet must be explicit about
whether it is proposing full OBL-020 statement status or a G1-supporting
statement-status slice.

| Criterion | Required before proposing OBL-020 status movement | Current support | Still open |
|---|---|---|---|
| Scope choice | Decide whether the proposal covers full canon OBL-020 or only a G1-supporting statement identity. | `plan/76` warns that OBL-020 ranges over the step-rule family, not only G1. | No status movement should happen until this is explicit. |
| Statement target identity | Name the accepted statement target. | `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` defines `OBL020StatementDraft`. | Canon has not accepted that file / namespace / constant as OBL-020. |
| WF abstraction acceptance | State whether abstract `WellFormed`, `Step`, and `PreservesWF` are sufficient for statement status. | `plan/126` says this abstraction is sufficient for the current bridge. | Human/canon review must decide if it is sufficient for ledger movement. |
| Step-family handling | Either enumerate accepted step-family coverage or explicitly defer concrete step taxonomy to proof/T2 work. | `FamilyStepPreservesWF` threads `CanonStepFamily` and `StepHasFamily`. | Concrete `Config`, `StepLabel`, `StepFamily`, WF clauses, and per-step lemmas are not chosen. |
| Relation to G1 | Explain why this statement-status movement is needed for G1 while not importing runtime proof. | `plan/128` separates G1 statement/status from later proof discharge. | The future packet must not turn runtime behavior into a G1 claim. |
| Non-vacuity guard | Show body-level WF preservation links are guarded. | `plan/117` / `plan/126` guard `PreservesWF`, `FamilyStepPreservesWF`, and `OBL020StatementDraft`. | Guard evidence is not proof. |

OBL-020 status movement must not claim WF preservation proof, per-step proof
decomposition, scheduler semantics, runtime implementation proof, C-runtime
conformance, or final step-family taxonomy.

## OBL-021 criteria

OBL-021 is elaboration determinism. It is not runtime scheduling determinism.

| Criterion | Required before proposing OBL-021 status movement | Current support | Still open |
|---|---|---|---|
| Statement target identity | Name the accepted statement target. | `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` defines `OBL021StatementDraft`. | Canon has not accepted that file / namespace / constant as OBL-021. |
| Input boundary | State the accepted well-scoped input boundary. | `WellScopedInput` gates `OBL021StatementDraft`. | Final checker / parser completeness is not proved. |
| Success/success equivalence | Same input successful elaborations produce equivalent outputs. | `SameElabResult` covers core term, type, mode, effect row, failure row, constraints, obligations, generated edges, and source spans. | Final equality relation is not selected. |
| Reject/reject equivalence | Same input rejections produce equivalent diagnostics. | `SameDiagnostic` / `EquivalentDiagnostic`. | Final Diagnostic ABI and diagnostic equivalence contract are not selected. |
| Success/reject exclusion | Same input cannot both succeed and reject. | `ElabDeterministicPost` includes mutual exclusion. | No proof or implementation determinism claim. |
| Projection/equality boundary | Explain why abstract equivalence predicates are sufficient for statement status, or choose final equivalence relations. | `plan/126` keeps final equality, projection-totality, and diagnostic ABI outside current draft. | Human/canon review must accept abstraction or defer status movement. |
| Non-vacuity guard | Show determinism body links are guarded. | `plan/117` / `plan/126` guard result families, diagnostic equivalence, and success/reject exclusion. | Guard evidence is not proof. |

OBL-021 status movement must not claim elaboration determinism proof, runtime
scheduling determinism, final equality selection, final Diagnostic ABI,
projection-totality proof, parser/checker implementation proof, or C-static
conformance.

## Status proposal packet shape

A future status proposal should include, for each of OBL-001 / OBL-020 /
OBL-021:

| Packet part | Required contents |
|---|---|
| Canon anchor | Exact canon rows and chapter text that define the obligation. |
| Requested status | `stated`, `lean-stated`, or other allowed status, with rationale. |
| Artifact identity | Lean path / namespace / constant if `lean-stated`; paper section if `stated`. |
| Scope statement | What is included and what is explicitly deferred. |
| Evidence trace | LAB plans, rows, and guards that support the statement shape. |
| Non-claim appendix | Proof, conformance, runtime, ABI, OPEN, and later-gate exclusions. |
| Ledger patch proposal | Proposed change to `theory/11-metatheory-ledger.md`, if any, left for human/canon acceptance. |

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
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof or production auth claim.

## Next allowed move

The next autonomous package can stay docs-only and prepare a G1 status proposal
packet outline, but only as a proposal draft. It should not edit
`mirrorea_canon/theory/11-metatheory-ledger.md` unless the user explicitly
promotes canon-edit work.

If a concrete blocker appears while drafting that outline, the default fallback
is a narrower OBL-specific criteria refinement, not a Lean predicate change.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is criteria-inventory-only: no canon edit, no gate exit, no
OBL status movement, no proof, no conformance claim, no implementation change,
and no runnable sample status change.
