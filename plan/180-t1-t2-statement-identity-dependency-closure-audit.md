# 180 - T1/T2 statement identity and dependency closure audit

## Role and authority

This is LAB repository memory for a read-only audit at source cut
`b9320fa7a57baa8327caf93787268444ea818f09`. Canon remains normative. This
document does not amend an OBL, theorem, carrier, contract, SCN, Gate, Phase,
or proof status. It is not a new `WRK-####` record and does not narrow
ADR-0014's standing eligibility predicate.

## Purpose and audit rule

T1/T2 require statement identity before proof work can be meaningful. For each
listed exit-critical OBL row, this audit records the Canon statement domain, necessary
relation, current LAB evidence, and the first missing boundary. A Lean compile,
countermodel, or sample trace is classified only as evidence of its stated
artifact. It is never treated as a Canon proof or a ledger update.

The audit stops at an owner/canon boundary rather than selecting a new Core
primitive, relation, grammar, diagnostic ABI, or theorem wording. A later
research record may be opened only under ADR-0014 with its own pre-registration
and non-duplicative consumer/falsifier.

## Cross-cutting findings

- The dependency metadata between `theory/01` and `theory/02` is a permitted
  knowledge cycle, not an invalid prerequisite graph: Canon style-guide allows
  mutual `depends_on` references.
- The Canon target for OBL-020 is global preservation over every step rule.
  A familywise conditional lemma remains useful evidence but cannot replace the
  global target without an explicit coverage relation.
- OBL-021 pairwise coherence and exclusivity do not establish outcome
  production. The placement of a totality condition remains PROPOSAL-008's
  owner/canon question.
- No currently active Lean artifact provides a Canon correspondence proof.
  Foundations are small proof fragments; LAB statements are compile-check-only;
  clean-suite files are generated `True` stubs.

## T1 statement rows

| Row | Canonical domain and necessary relation | Current evidence and first missing boundary | Classification / next dependency |
| --- | --- | --- | --- |
| G1 / OBL-001 | A successful Surface **assignment** elaboration; every write in Core `c` is owner-local or an explicit owner-directed request with the stated obligation, failure, edge, and span properties. | `THM001StatementDraft.lean` is over opaque `Result`/`GeneratedWrite`; its retained countermodel shows that this does not enumerate Core writes. The draft also does not establish coverage of every Canon assignment form. | Proof-facing Core/write identity is not selected. The existing LAB preference for direct `c` is not a Canon choice. |
| G1 / OBL-020 | Every transition of `Sigma_r` preserves all named well-formedness clauses: DAG, grant lineage, observe provenance, state epoch/tombstone, and chain monotonicity. | `StepWFStatementDraft.lean` uses opaque `Config`, `Step`, and `WellFormed`; `CanonStepFamily` is not the actual transition relation. Familywise evidence exposes the missing coverage premise. | No complete formal transition family or frame/freshness relation is selected. Preserve the direct global target. |
| G1 / OBL-021 | Elaboration is a function of its complete input tuple, producing either the required outcome form or a Diagnostic under the selected boundary. | The current draft demonstrates only pairwise coherence/exclusion. Retained countermodels expose projection vacuity, carrier variance, and absence of outcome production. | PROPOSAL-008 controls totality placement. Do not infer totality from coherence. |
| G2 / OBL-005 | Canonical flattening laws for the chain syntax/normal form. | Canon defines the normalisation law; historical evidence has only a bounded reassociation/output-word model, not a Canon-aligned formal/Lean chain carrier. | Needs a selected formal chain representation after OPEN-005/Surface reconciliation. |
| G2 / OBL-006 | Uniqueness/confluence for the same chain normalisation relation. | No selected rewrite/equivalence relation; OBL-005's bounded model is insufficient. | Depends on OBL-005 statement identity and its relation. |
| G2 / OBL-007 | A trace-level chain position never re-promotes after degradation except by an explicit new lineage. | Canon states trace, selection, expiry, and reacquisition semantics; historical evidence is only a restricted lineage model, not a Canon-aligned formal/Lean relation. | Needs the same chain/lifetime model without elevating experiment-local state to Core. |
| G2 / OBL-008 | Proof of the OBL-007 statement. | No dedicated proof artifact. | Depends on a Canon-aligned OBL-007 statement. |
| G3 / OBL-015 | Every owner mutation occurrence has either validated grant lineage for the use or an owner-local declared transition. | Historical audit finds no mutation-to-use or owner-local transition association. | Needs an occurrence/history and lineage relation; role, transport identity, and helper booleans remain non-authority. |

## T2 proof and cut rows

| Row | Canonical domain and necessary relation | Current evidence and first missing boundary | Classification / next dependency |
| --- | --- | --- | --- |
| T2 / OBL-002 | Proof skeleton for the exact OBL-001 statement. | No dedicated proof artifact. | Cannot start a proof-facing proof skeleton until the OBL-001 Core/write relation is fixed. |
| T2 / OBL-020 | Proof skeleton for global step preservation. | Abstract compile-check statement and conditional familywise models only. | Requires a complete transition family and explicit preservation premises; must not hide them inside opaque predicates. |
| T2 / OBL-021 | Proof skeleton for deterministic elaboration. | Abstract coherence statement and countermodels only. | Requires the selected outcome/projection/equality boundary; totality remains separate unless canon says otherwise. |
| G5 / OBL-009 | Successful load restores a well-formed configuration with a consistent prefix and no stale resurrection. | Canon defines `Load` admissibility and SaveObject semantics; historical evidence is only a restoration twin, not a Canon-aligned formal/Lean load/restored/live-state relation. | Needs a shared history/save/load model. |
| G5 / OBL-010 | Soundness of the `Consistent(K)` checker. | Canon defines `Consistent(K)` over causal precedence; historical evidence is only a bounded prefix-check kernel, not a Canon-aligned formal/Lean checker relation. | Depends on explicit occurrence/precedence and checker input relations. |
| G5 / OBL-011 | No stale membership epoch/incarnation resurrection after load. | No standalone formal artifact. | Depends on OBL-009's load/restored/live relation. |
| G5 / OBL-012 | No stale witness resurrection after load. | No standalone formal artifact. | Depends on OBL-009's load/restored/live relation. |
| G5 / OBL-013 | No expired lease resurrection after load. | No standalone formal artifact. | Depends on OBL-009's load/restored/live relation. |
| G5 / OBL-014 | Equivalence between Z-cycle rejection and the named useless-checkpoint characterization. | Current `CUT-11` evidence checks synthetic reason codes, not checkpoint graphs or the equivalence. | Needs a checkpoint-graph and stated external characterization boundary. |

## Excluded rows and surfaced risks

Non-OBL G1/G2/G3 criteria, including the required theory-to-SCN explanations
and finalized SCN expectations, are not assessed at per-scenario level here.
OBL-003/004, G4, G6, and G7 are likewise outside this audit. OBL-003's
finite-fragment decision procedure still lacks a full carrier/enumeration
boundary; the frozen local-predicate routes do not settle it. Hot-plug's
multi-Place frontier and observation's occurrence-versus-declared-telemetry
origin need later owner/canon interpretation if they block their respective
G7/G4 statement work. This audit records neither as a contradiction or a
selected repair.

## Ordered path and stop line

1. Owner/canon action remains needed for G0-D3 before official T1 entry.
2. The first proof-facing choices are OBL-001's direct-Core versus explicit
   enumeration bridge and PROPOSAL-008's outcome-totality placement.
3. After those boundaries and the required phase profile are canonically
   resolved, a shared model may formalize only existing `Core`, `Config`,
   transitions, elaboration outcomes, and named invariants.
4. T1 integrates G1/G2/G3 statements after Canon/ledger/phase action; this is
   not proof completion.
5. T2 follows the ledger order `020 -> 021 -> 001/002 -> 005-008 -> 009-014`,
   with later statement and proof work over the same
   history/membership/witness/lease model.

Until a boundary is selected through its proper route, this is a dependency map,
not permission to fill it with a toy model. The audit establishes no Gate exit,
T1/T2 entry, OBL status movement, implementation readiness, or public claim.
