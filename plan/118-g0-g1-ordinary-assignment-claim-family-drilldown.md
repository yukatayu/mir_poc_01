# plan/118 - G0/G1 ordinary assignment claim-family drilldown

## Purpose

This file is LAB repository memory.

It drills down the `plan/70-lab-to-canon-reconciliation-ledger.md` claim-family
row for ordinary Surface assignment. The goal is traceability: identify exactly
which parts of the old LAB ordinary-assignment story are supported by canon
anchors, which parts are only LAB evidence, and which parts remain non-claims.

This file does not edit canon, does not close G0 or G1, does not move
`mirrorea_canon/theory/11-metatheory-ledger.md`, does not discharge any proof
obligation, and does not promote a new implementation package.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB claim-family ledger:
  `plan/70-lab-to-canon-reconciliation-ledger.md`
- LAB ordinary-assignment memory:
  `plan/71-g1-ordinary-assignment-target.md`
- LAB SCN / OBL follow-through:
  `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` through
  `plan/78-g1-obl020-lean-statement-draft.md`, plus
  `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- LAB runnable/sample evidence:
  `samples/full-system-v1-surface/elaboration/`,
  `samples/lean/lab-statements/`, `crates/mir-semantics/`, and related scripts

If any LAB evidence conflicts with canon, canon wins. Cite LAB files as
`LAB:path` unless a canon ADR, theory/spec file, conformance scenario, gate, or
phase file already mirrors the point.

## Selected claim-family row

The row drilled down here is:

```text
LAB source family: specs/39, plan/64, progress.md, Surface P-SURF rows
Claim summary: Ordinary Surface assignment should elaborate to explicit
owner-directed Core consequences.
Canon disposition: OPEN
Canon anchor: theory/03, theory/11, spec/02, spec/03, spec/04, plan/00 G1
```

This is the right first `plan/70` row to drill down because it is the current
G1 pressure case, and because `plan/71..78` plus `plan/117` already form a
bounded LAB evidence chain around SCN-01 / SCN-02 and OBL-001 / OBL-020 /
OBL-021. Drilling it down improves traceability without widening runtime scope.

## Canon anchors

| Canon source | Reading for this drilldown |
|---|---|
| `mirrorea_canon/theory/00-overview.md` | S0 stays ordinary while S1-S4 make hidden consequences explicit and diagnosable. Reads are dependencies; writes are occurrences. |
| `mirrorea_canon/adr/ADR-0002.md` | Read/write/occurrence vocabulary must not collapse ordinary source code into event machinery. |
| `mirrorea_canon/adr/ADR-0003.md` | Source spans remain part of diagnostic / generated-consequence traceability. |
| `mirrorea_canon/plan/00-gates.md` | G1 is ordinary assignment. G1 exit requires SCN-01/02 explanation plus OBL-001, OBL-020, and OBL-021 completion. This file closes none of them. |
| `mirrorea_canon/plan/01-phases.md` | Canon implementation position remains T0. LAB files and runnable samples are evidence, not canon implementation-state completion. |
| `mirrorea_canon/theory/01-mircore-v0.md` | Assignment, read, owner-local write, owner-directed request, locus block, handler, publish / observe, and generated edges remain separate semantic families. |
| `mirrorea_canon/theory/02-types-effects-failures.md` | Generated effects and generated failures must be explicit and row-contained; static failure-row violations are not generic runtime rejects. |
| `mirrorea_canon/theory/03-elaboration.md` | BND-001 and THM-001 define the assignment-elaboration target shape: no hidden cross-locus edges, source-span preservation, row containment, authority obligations, determinism, and no authority creation. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | OBL-001, OBL-020, and OBL-021 remain separate obligations; LAB statement drafts and guards are not proof discharge. |
| `mirrorea_canon/spec/02-surface-grammar.md` | Surface assignment and `S { ... }` locus-block syntax are source-side references; `S[ ... ]` is not the canon syntax. |
| `mirrorea_canon/spec/03-static-semantics.md` | Cross-locus access, role / state declarations, and failure-row containment stay static-boundary issues where canon says so. |
| `mirrorea_canon/spec/04-core-ir.md` | Core IR shape is working exchange evidence; do not freeze final JSON field names from LAB helpers. |
| `mirrorea_canon/spec/06-conformance.md` | SCN-01 and SCN-02 are conformance anchors, but this drilldown is not a C-static pass claim. |
| `mirrorea_canon/architecture/02-boundary-contracts.md` | BND-001 is the immediate boundary; runtime, transport, projection, provider, and devtools boundaries remain later. |
| `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md` | Sugoroku roll pressures owner-directed write, same-field RHS dependency, visible publish / observe, failure containment, spans, and authority-obligation carrier. |
| `mirrorea_canon/scenarios/SCN-02-attack.md` | Attack pressures owner-directed write, target/self RHS dependencies, failure containment, nested-locus non-authority, and later runtime stale/capability behavior. |

## Claim decomposition

| Subclaim | Canon reading | Current LAB support | Boundary / non-claim |
|---|---|---|---|
| Source-side ordinary assignment exists | Canon grammar and theory name assignment as an ordinary Surface item. | `LAB:plan/71` scopes the first G1 target to simple assignment before compound assignment. | No final public grammar/API freeze. |
| Nested `S { ... }` does not grant ambient authority | Canon locus-block rule distinguishes checking under a locus from authorization source. | `LAB:plan/72` records SCN-02 nested-locus non-authority; ELAB-02 / ELAB-08 show requester remains actor-side in LAB evidence. | No THM-004 / authority theorem discharge. |
| Owner-local write is distinct from owner-directed request | Canon write rules distinguish local write occurrence and remote request. | `LAB:plan/71`, `LAB:plan/72`, and ELAB-02 / ELAB-09 / ELAB-11 / ELAB-12 support the split. | No runtime request serving or direct remote store correctness claim. |
| RHS reads are explicit dependencies | Canon treats reads as dependencies and THM-001 requires dependency recording. | `LAB:plan/75` records `rhs_indexed_read` evidence for SCN-01 same-field and SCN-02 target/self reads. | OPEN-014 read materialization remains open; no cache / reply / observe policy freeze. |
| Visible write consequences are explicit | Canon observation theory keeps publish / observe as explicit information effects. | `LAB:plan/72` and ELAB-09 / ELAB-11 show visible write publish / observe evidence. | LAB `MessageEnvelope` is helper evidence, not canon vocabulary or runtime dispatch. |
| Generated failures are row-contained | Canon BND-001 / static semantics require generated failure containment. | `LAB:plan/72`, `LAB:plan/79`, and E-ROW rows show underdeclared generated failure pressure. | Current LAB diagnostic strings are not final canon diagnostic IDs or ABI. |
| Authority/capability/witness obligations are represented | Canon requires authority obligations, but authority validity belongs to a later theorem family. | `LAB:plan/73` / `plan/74` model authority-obligation representation in the OBL-001 statement draft. | No grant-lineage proof, production identity, membership proof, or G3 completion. |
| Source spans are preserved | Canon BND-001 requires generated consequences to map back to source spans. | `LAB:plan/72`, ELAB-05 / ELAB-09, and later E-ROW projection work preserve source-span evidence. | No final JSON `source_map` ABI. |
| OBL-001 statement shape exists as LAB evidence | Canon ledger still requires an accepted OBL-001 statement. | `LAB:plan/74` adds `THM001StatementDraft.lean`; `LAB:plan/117` guards body-level OBL-001 links. | Compile-check-only `Prop` is not canon `lean-stated`, proof, or OBL completion. |
| OBL-020 statement shape exists as LAB evidence | Canon ledger still requires OBL-020 completion for G1. | `LAB:plan/78` adds `StepWFStatementDraft.lean`; `LAB:plan/117` guards the WF preservation aggregate shape. | No runtime step proof, proof skeleton, or WF preservation completion. |
| OBL-021 statement shape exists as LAB evidence | Canon ledger still requires OBL-021 completion for G1. | `LAB:plan/77` adds `ElabDeterminismStatementDraft.lean`; `LAB:plan/117` guards result / diagnostic equivalence links. | No final equality relation, diagnostic ABI, or elaboration determinism proof. |
| SCN-01/02 are explained enough for future G1 work | Canon requires theory/spec to explain SCN-01/02 for G1 exit. | `LAB:plan/72` maps SCN-01/02 static consequences; `LAB:plan/75` closes the immediate RHS dependency evidence gap. | No C-static conformance pass and no G1 exit. |

## LAB citation map

| LAB citation | Current reading | Unsafe reading blocked |
|---|---|---|
| `LAB:specs/39-surface-mir-placement-elaboration.md` | Historical Surface-placement elaboration design memory for generated edges, nested place blocks, cross-locus reads/writes, and alpha helper expectations. | Not canon; not final Surface grammar; not final Core IR / runtime behavior. |
| `LAB:plan/64-surface-mir-placement-roadmap.md` | Historical P-SURF-03 / P-SURF-04 roadmap and actualized ELAB-row memory. | Not gate status; not final implementation completion. |
| `LAB:plan/70-lab-to-canon-reconciliation-ledger.md` | Claim-family ledger that marks ordinary assignment as `OPEN` and points to G1-safe drilldown. | Not shadow canon and not G0 exit. |
| `LAB:plan/71-g1-ordinary-assignment-target.md` | Target/proof-boundary split for simple ordinary assignment. | Not G1 exit, theorem discharge, runtime dispatch, or API freeze. |
| `LAB:plan/72-g1-scn01-scn02-static-consequence-drilldown.md` | SCN-01/02 static consequence map and immediate LAB gaps / boundaries. | Not C-static conformance or runtime behavior. |
| `LAB:plan/73-g1-obl001-lean-statement-inventory.md` | Pre-statement vocabulary / predicate / overfit inventory for OBL-001. | Not Lean statement, proof, or OBL status movement. |
| `LAB:plan/74-g1-obl001-lean-statement-draft.md` | Compile-check-only `Prop` statement-shape draft for OBL-001. | Not canon `lean-stated`, THM-001 proof, or G1 exit. |
| `LAB:plan/75-g1-scn-rhs-dependency-gap-evidence.md` | LAB evidence for SCN-01 same-field RHS and SCN-02 target/self RHS dependency rows. | Not OPEN-014 resolution, read-materialization policy, or conformance pass. |
| `LAB:plan/76-g1-obl020-021-dependency-inventory.md` | Separation inventory for OBL-020 well-formedness preservation and OBL-021 elaboration determinism. | Not proof skeleton, OBL completion, or runtime scheduling determinism. |
| `LAB:plan/77-g1-obl021-lean-statement-draft.md` | Compile-check-only `Prop` statement-shape draft for OBL-021. | Not final equality relation, diagnostic ABI, or proof. |
| `LAB:plan/78-g1-obl020-lean-statement-draft.md` | Compile-check-only `Prop` statement-shape draft for OBL-020. | Not step-family taxonomy freeze or WF preservation proof. |
| `LAB:plan/117-g1-obl001-020-021-statement-guard-hardening.md` | Sync-test guard against silent body-level drift in OBL-001/020/021 drafts. | Not proof discharge or canon ledger movement. |
| `LAB:samples/full-system-v1-surface/elaboration/` | Runnable/current helper evidence for ELAB rows, including ELAB-11/12 dependency rows. | Not final runtime workflow, final Core IR ABI, or SCN conformance. |
| `LAB:crates/mir-semantics/src/surface_to_core_elaboration.rs` and `LAB:crates/mir-semantics/tests/surface_to_core_elaboration.rs` | Current implementation and test evidence for Surface-to-Core elaboration. | Not canon implementation-state completion. |
| `LAB:samples/lean/lab-statements/obl001/`, `obl020/`, `obl021/` | Compile-check-only Lean statement-shape evidence. | Not theorem proof, proof skeleton completion, or OBL completion. |

## Required non-claims

- No canon edit.
- No canon L0/L1 source change.
- No ADR creation or canon `OPEN` resolution.
- No G0 exit.
- No G1 exit.
- No T1 or T2 transition.
- No OBL-001 / OBL-002 / OBL-020 / OBL-021 proof discharge.
- No proof skeleton completion.
- No canon `theory/11` status movement.
- No C-static, C-runtime, or C-distributed conformance claim.
- No final grammar, Core IR JSON, public API, runtime API, diagnostic ABI, or
  repair ABI freeze.
- No runtime `MessageEnvelope` dispatch claim.
- No direct remote store.
- No ambient authority from nested locus syntax.
- No authority from role names, keys, providers, transports, package artifacts,
  or helper row IDs.
- No OPEN-014 read-materialization decision.
- No implementation-state completion.
- No promotion of legacy `specs/`, `plan/`, report, helper, sample, or Lean
  compile-check output to canon.

## Hidden failure modes to keep out of future packages

| Failure mode | Guard |
|---|---|
| Treating `plan/70` or this drilldown as a second canon source | Keep both files LAB-only and cite canon anchors first. |
| Treating helper output as G1 completion | State command/sample results as evidence only; never as gate status. |
| Treating Lean compile-check as theorem discharge | Keep statement drafts under LAB namespace and keep `theory/11` unchanged. |
| Smuggling OBL-020/021 into OBL-001 | Keep OBL-020 WF preservation and OBL-021 determinism separate in summaries and tests. |
| Freezing helper JSON / row IDs as final Core IR | Use semantic labels in planning text; treat JSON as current LAB carrier only. |
| Collapsing dependency rows into runtime communication | Preserve OPEN-014 and phrase G1 as explicit dependency/read consequence. |
| Collapsing publish / observe into debug output | Keep observation as an authority/redaction-aware information effect boundary. |
| Promoting `World`, `S`, `Player`, `hp`, `position`, or `atk` into Mir core | Treat them as scenario/domain vocabulary. |
| Using `S { ... }` as authority | Keep nested-locus non-authority explicit. |

## Follow-up reading

For ordinary assignment, the current traceability chain is now:

1. `plan/70` for the claim-family disposition.
2. `plan/118` for the line-level ordinary-assignment drilldown.
3. `plan/71` for the target/proof-boundary split.
4. `plan/72` for SCN-01/02 static consequence mapping.
5. `plan/73` / `plan/74` for OBL-001 inventory and LAB statement draft.
6. `plan/75` for SCN RHS dependency evidence.
7. `plan/76` / `plan/77` / `plan/78` for OBL-020/021 separation and LAB
   statement drafts.
8. `plan/117` for body-level drift guards on OBL-001/020/021 statement drafts.

## Next safe packages

1. Focused OBL-001 / OBL-020 / OBL-021 statement refinement only if review
   identifies a concrete missing predicate, overfit, or drift risk.
2. Remaining `plan/70` claim-family drilldowns for non-ordinary-assignment rows
   only when needed for a future G0 close decision.
3. Canon mental-model clarification proposal only if a real canon wording gap is
   found; do not edit canon directly from this LAB file.
4. Continue E-ROW / OBL-024 / OBL-025 work separately; do not mix diagnostic /
   repair ABI questions into ordinary-assignment G1 status.

## Close condition

This file is closed when the drilldown, `plan/70`, `plan/00-index.md`,
`plan/90-source-traceability.md`, snapshot docs, report, and validators are
synchronized.

Close condition is traceability-only: no canon edit, no gate exit, no proof, no
conformance claim, no runtime implementation claim, and no public API freeze.
