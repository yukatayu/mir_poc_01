# plan/156 - T0-T2 research autonomy envelope

## Purpose

This LAB operating plan records the owner's instruction to advance the
research portion of T0-T2 carefully and autonomously. It turns that instruction
into a bounded work-selection and stop protocol. It does not amend canon,
promote a canon package, change a Gate/Phase, change an OBL status, or create
implementation authority.

The controlling authority remains `mirrorea_canon/plan/02-operating-model.md`:
the owner decides L0/L1, Gate exits, and ADR effectivity; `theory/11` alone
states proof status. This file is a LAB operational interpretation of the
owner's current authorization, not a general replacement for that rule.

## Terms

| Term | Meaning in this plan |
| --- | --- |
| LAB research work unit | A bounded, reproducible investigation attached to an existing canon T0-T2 objective. It is not a fifth canon package type. |
| `research-complete` | The stated investigation has reproducible evidence and bounded conclusions. It is not a canon package close. |
| `decision-ready` | The investigation reached a choice requiring a canonical act or owner acceptance. The agent stops rather than declaring close. |
| dormant deferred item | A deferred item that is not selected unless its recorded reopen trigger occurs. This is a LAB selection guard, not a newly inferred canon rule. |

## Autonomous selection rule

The agent may select a work unit only when all of the following hold:

1. it is connected to an existing T0-T2 Gate, Phase, SCN, THM, OBL, BND, or
   OPEN item;
2. it is not mainly a dormant deferred decision;
3. it can use an existing LAB lane, runner, Lean statement location, or
   documentation route;
4. it does not preempt runtime, conformance, product, final ABI, or later
   implementation work; and
5. its plan names the exact criterion or obligation it pressures and the
   evidence that can falsify its working hypothesis.

Permitted work includes theory reading, adversarial counterexample search,
Lean statement transcription or exploratory proof skeletons, SCN-to-LAB
evidence mapping, reference-scenario candidate design, existing-runner
reproduction, and decision-bundle preparation. A spike remains disposable and
must not merge code into `main`.

## Exclusions before T1 exit

Do not create a new evidence lane, helper family, report series, generator,
schema, CI surface, Make target, production implementation, conformance claim,
or final/public API. Do not claim a Gate/Phase exit, OBL completion, proof
discharge, `mir-conform` result, or runtime/product readiness from LAB evidence.

An ephemeral countermodel or proof experiment is allowed only as a spike or
scratch artifact; its reusable conclusion is recorded in the existing report
and plan route, not promoted by keeping new mainline code.

## Mandatory stop triggers

Stop at `decision-ready` and prepare a decision bundle when any of these occur:

- a semantic choice cannot be derived from existing canon;
- an SCN expectation, canon wording, ADR, or `theory/11` status must change;
- canon and LAB evidence conflict;
- reproducible evidence is missing or a counterexample breaks the intended
  theorem;
- a new helper/evidence/CI/schema surface is needed;
- the work would require runtime, conformance, implementation, or later-phase
  scope; or
- a deferred item's reopen trigger is absent or ambiguous.

Every bundle must state the question, authority cut, affected Gate/Phase/SCN/
THM/OBL/BND/OPEN ids, alternatives, semantic delta, positive and negative
evidence, counterexample/Lean evidence, reproducibility, evidence level,
assumptions/non-claims, deferral trigger, requested canonical act, and
independent review result.

## Evidence-first order

| Order | Work | Boundary |
| --- | --- | --- |
| 0 | authority cut and dormant-decision audit | no status movement |
| 1 | criterion-to-evidence matrix and reproduction of existing anchors | existing lanes only |
| 2 | counterexample-first audit of OBL-020, OBL-021, and OBL-001 statement drafts | scratch/Lean only |
| 3 | concrete T1 statement research: OBL-020/021/001, then G2 OBL-005..008 and G3 OBL-015 | decision-ready if semantics is missing |
| 4 | future G1 acceptance preflight and adversarial review | owner/canon review required |
| 5 | T2 proof-skeleton research in OBL-020 -> OBL-021 -> OBL-002 order and G5 statements | `theory/11` controls proof status |
| 6 | decision bundle and independent review | no inferred adoption |

The order is a research priority, not canon roadmap advancement. Current T0
does not authorize an official T1 entry; it permits bounded research only.

## T-RESEARCH-001: statement-shape countermodel audit (research-complete)

**Question:** Do the existing LAB OBL-020, OBL-021, and OBL-001 statement
drafts express standalone theorems, or only postcondition shapes requiring a
concrete calculus and rule hypotheses?

**Sources:** `mirrorea_canon/theory/01-mircore-v0.md`,
`mirrorea_canon/theory/03-elaboration.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/121-g1-minimal-vertical-slice-candidate-map.md`,
`plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`, and
the existing `samples/lean/lab-statements/obl001`, `obl020`, and `obl021`
drafts.

**Method:** reproduce the current Lean compile checks and the Surface static
anchor; construct finite countermodels in a disposable scratch location; map
the precise missing premise for each draft; record only a bounded conclusion.

**Evidence and result:** the three source drafts compile with `lean --trust=0`;
their existing sync suite has 21 passing tests; and the existing Surface static
anchor has 53 accepted expected rows and no failures. Disposable scratch files
under `/tmp/mirrorea-t-research-001/` construct finite models that prove the
negation of each draft when its `Pred` fields are unconstrained.

| Draft | What the countermodel establishes | Concrete semantic force still needed |
| --- | --- | --- |
| OBL-020 | `WellFormed before` and `Step before label after` alone do not force `WellFormed after`. | Canonical step rules, per-rule preservation/frame reasoning, and case exhaustiveness. |
| OBL-021 | One well-scoped input can elaborate to inequivalent results if `Elaborates` is unconstrained. | Successful and rejecting elaboration functionality, success/reject exclusion, result-projection adequacy, and selected equivalence laws. |
| OBL-001 | A generated write need not be locally justified or request-justified if the predicates are unconstrained. | Rule inversion from successful elaboration to local/cross-write origin and the remaining edge/failure/authority/span invariants. |

This establishes **parametric non-validity** (`exists V, P` for which a draft
does not hold), not a counterexample to canonical MirCore rules and not a proof
that the drafts are unusable. Concrete rules can supply the force definitionally
instead of as new final theorem arguments. It is therefore `research-complete`,
not `decision-ready`.

An independent Oracle review agreed with this reading, noted that the OBL-021
projection clauses may otherwise be vacuous without adequacy/totality, and
confirmed that `plan/126` remains consistent when its word "sufficient" is read
as current-bridge/compile-boundary sufficiency only. The initial browser run
lost its connection; the single permitted retry produced the advisory result.

## T-RESEARCH-002: OBL-020 [E-WRITE] store-key clause (research-complete)

**Question:** Does the `[E-WRITE]` description in
`mirrorea_canon/theory/01-mircore-v0.md` supply a non-circular minimum premise
bundle for one well-formedness conjunct: every store entry key is Active at its
recorded epoch or tombstoned?

**Method:** a disposable Lean model uses a `Store` of entries with epochs,
separate Active/Tombstoned membership predicates, and the value-update reading
of `S[ell][x][key := value]`: update the target value while retaining its key
and epoch. It proves that pre-step key well-formedness plus the canonical
active-key premise preserves this one clause. A second finite theorem changes
the target epoch and proves that the clause can fail, exposing why the
value-only/frame reading matters.

**Result:** the narrow case is feasible without a new canon choice. The canon
store notation and `[E-WRITE]` active-key premise support this bounded reading,
but the experiment is not an exact runtime-configuration formalization and
does not cover other well-formedness clauses or step rules. The frame condition
is necessary evidence for a future proof package, not a newly adopted semantic
requirement.

**Next selection point:** select a further concrete rule/clause only when it
has a similarly explicit canon source cut and falsification criterion. Do not
infer a general OBL-020 theorem from this one case.

## Current non-claims

This plan does not assert that G0 exits, that T1 begins, that G1 is ready, that
any OBL is `stated` or `lean-stated`, that an exploratory Lean proof proves a
canon theorem, or that existing runnable samples satisfy conformance.
