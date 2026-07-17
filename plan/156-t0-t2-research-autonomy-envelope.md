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

## T-RESEARCH-003: OBL-020 [E-OBS] append kernel (research-complete)

**Question:** Can one narrow `[E-OBS]` graph case preserve two named
well-formedness conjuncts: occurrence-DAG acyclicity and kind-level publication
ancestry for observes? The exact canon source cut is
`mirrorea_canon/theory/01-mircore-v0.md` (the `H` well-formedness clauses,
one-occurrence append, and `[E-OBS]`), together with
`mirrorea_canon/theory/04-ordering-and-cuts.md` (causal generating-family
direction `publish -> observe` and transitive closure).

**Method:** disposable Lean files under `/tmp/mirrorea-t-research-003/` use a
generating relation and `Relation.TransGen`, not a MirCore `Config`. The kernel
extends an arbitrary old event carrier with `OldEvent + Unit`: old generators
are retained, the fresh node is an observe, and every new generator has an old
source and the fresh observe as target. It proves acyclicity preservation and
kind-level publication-ancestry preservation when the old graph satisfies the
same two properties and a publication is a direct incoming predecessor. A
separate finite model keeps an acyclic old graph and direct publication edge,
then adds a fresh-to-old edge to form a two-edge cycle.

**Result:** the incoming-only construction is a sufficient conditional graph
kernel. The weak premise package (acyclic old history plus a direct publication
edge while fresh-to-old edges remain allowed) is insufficient for acyclicity.
This does not prove that incoming-only is necessary, minimal, unique, or
canonically required; other acyclicity-preserving designs were not studied.

**Interpretation boundary:** freshness, unchanged old generators, and
incoming-only extension are an experiment-local reading of "appends one
occurrence" and the canonical causal direction. Canon does not provide an
extensional graph-update equation for `H + occurrence`. Therefore this result
does not identify `PostGen` with canonical `[E-OBS]`, choose a final occurrence
representation, or define final publication matching. The direct predecessor
is a stronger local premise than arbitrary transitive publication ancestry and
does not check resource, field, version, visibility, principal, authority, or
redaction matching.

**Review and disposition:** an Oracle follow-up independently reviewed the
actual scratch files and agreed that this is `research-complete`, not
`decision-ready`. Escalation is required before a later package treats the
experiment's graph update as the canonical definition of `[E-OBS]`, forbids
new-to-old edges normatively, or selects publication matching, authority,
redaction, or complete runtime premises. It also advised against selecting a
second autonomous calculus experiment from this result alone.

**Reopen trigger:** an explicit owner/canon request to formalize graph append,
or a different rule-local OBL/SCN candidate satisfying the selection rule in
this plan. Do not reopen merely to search for a mathematically weakest graph
condition.

## T-RESEARCH-004 candidate preflight: not selected

**Candidate:** a literal-RHS, private-field foreign-locus write inversion audit
for the `[WRITE-CROSS]` / `[LOCUS-BLOCK]` portion of THM-001 / OBL-001. The
source cut deliberately excluded RHS indexed reads, visible fields, and
publication/observation consequences so that it would not select OPEN-014 or
visibility semantics.

**Bounded preflight evidence:** an untracked `/tmp` source differed from the
existing `ELAB-07` negative only by completing its `fails` row. The existing
Surface elaborator accepted that positive source; the committed counterpart was
rejected with `generated_failure_not_declared`. Both reported one
`BrowserClient -> S` remote write request, no dependency/publication/observation
rows, and the same six source-span entity kinds. This is reproducible source
evidence only, not a tracked fixture or a new runner lane.

**Existing-lane falsifier:** the current elaborator output contains no
structured capability/witness-obligation carrier, and no existing interpretation
maps that output into the OBL-001 abstract `Pred` fields or the canonical `C ∪
O` obligation boundary. The preflight therefore cannot evaluate the candidate's
required authority-carrier clause. The absence is a limitation of the current
LAB evidence projection; it is not evidence that canon obligations are absent
or that the request is unauthorized.

**Disposition:** `T-RESEARCH-004` is **not selected** and is not
`research-complete` or `decision-ready`. No fixture, helper, schema, wrapper,
Lean interpretation, or artifact identity was added. The next autonomous LAB
research unit remains unselected.

**Separate decision-ready blocker:** a bridge-specific owner disposition would
be needed before any separately scoped concrete-evidence bridge design work,
such as comparing a read-only JSON-to-`Pred` interpretation, carrier exposure
for `C`/`O` and capability/witness references, or a reviewed artifact binding.
Such a disposition does not by itself authorize a committed bridge artifact or
waive the pre-T1 moratorium. The current LAB recommendation is to defer the
bridge until an OBL-001 proof-facing package actually needs it. This is an
evidence-route decision, not a request to change canon authority semantics,
Gate/Phase state, or proof status.

## OBL-001 concrete-evidence bridge decision bundle (owner action pending)

This section completes the presentation required by the mandatory stop rule.
It does not select T-RESEARCH-004, record an owner decision, create a bridge,
or promote another research unit. A broad instruction to continue autonomous
work authorizes work up to this stop; it does not by itself choose this bridge's
disposition.

**Question:** should the owner explicitly defer the concrete bridge until an
OBL-001 proof-facing package needs it, or authorize a separately scoped
bridge-design decision package? The question is about a LAB evidence route,
not whether authority/capability/witness obligations exist in canon.

**Authority cut:** `mirrorea_canon/plan/02-operating-model.md` reserves
roadmap advancement and L0/L1 decisions to the owner; the canon moratorium and
`meta/agent-instructions.md` prohibit a new evidence lane or helper family
before T1 exit. This plan may prepare the decision bundle but cannot treat a
recommendation, a generic continuation request, or Oracle advice as the owner
record.

**Directly affected identifiers:**

| Kind | Identifier | Effect of this bundle |
| --- | --- | --- |
| Gate / Phase | GATE-1 / G1, T0/T1 | No exit or entry; `GATE-1` is the theory/ledger spelling and `G1` is the gates-table label. |
| theorem / obligation | THM-001, OBL-001 | No statement status or artifact identity moves. |
| boundary | BND-001 | No elaboration contract or authority obligation changes. |
| scenario | SCN-02 | Existing static attack evidence remains evidence only; no scenario expectation changes. |

`OBL-002`, `SCN-01`, and `OPEN-014` are contextual but not directly affected:
this bundle neither changes the THM-001 proof boundary, selects the roll
scenario, nor enters the transparent-read materialization question.

**Alternatives and semantic delta:**

| Owner record (two presently admissible dispositions) | Permitted immediate effect | Semantic / lifecycle delta |
| --- | --- | --- |
| Explicitly defer (current LAB recommendation) | Keep the bridge absent until a proof-facing OBL-001 package names why it needs a concrete interpretation; later research selection may use another existing lane only if the selection rule is met. | None. No canon change, implementation, wrapper, or ledger movement. |
| Authorize a bounded bridge-design decision package | The owner record must name the existing route and permitted persistence. Its scope is comparison only: no committed bridge artifact and no new evidence/helper/schema/runner surface. | None by itself. It is not approval of JSON fields, Lean predicates, helper/schema surfaces, a moratorium exception, or a canon status. |

**Out-of-scope future escalation:** a committed bridge artifact is not a third
current disposition. It would encounter the pre-T1 new-lane/helper moratorium
and requires an independent canon-compatible owner/canon route; an owner record
in this bundle cannot waive that restriction.

**Positive evidence:** the disposable literal-RHS source that differs from
`ELAB-07` only by completing its `fails` row is accepted by the existing Surface
elaborator. The committed negative is rejected with
`generated_failure_not_declared`. Both project one `BrowserClient -> S` remote
write, no dependency/publication/observation rows, and the same six source-span
entity kinds.

**Negative evidence:** no structured capability/witness carrier appears in the
current elaborator JSON, and no existing interpretation maps its output to the
abstract OBL-001 `Pred` hooks or the canonical `C ∪ O` obligation boundary.
This falsifies the proposed existing-lane investigation, not THM-001 or the
existence of canon authority obligations.

**Counterexample / Lean evidence:** T-RESEARCH-001 established that the
unconstrained OBL-001 predicate shape is not a standalone soundness theorem.
`samples/lean/lab-statements/obl001/THM001StatementDraft.lean` contains the
abstract `RequestCarriesAuthorityObligations` hook, but not an interpretation
from the concrete elaborator output. This bundle adds neither a counterexample
to canon nor a Lean artifact.

**Reproducibility and evidence level:** reproduce the committed negative with
`python3 scripts/surface_mir_samples.py --format json run ELAB-07`; reproduce
the positive by making the documented one-line `fails`-row change in disposable
scratch and running the existing `surface_to_core_elaborate` example. This is
LAB static source/evidence-projection data, below a theorem interpretation,
proof, conformance result, or runtime observation.

**Assumptions and non-claims:** the source pair intentionally excludes RHS
indexed reads, visible fields, publication/observation consequences, transport,
admission, and runtime serving. It does not resolve OPEN-014, alter BND-001,
choose artifact identity, complete OBL-001/002, or authorize a bridge.

**Disposition condition:** only an owner record that names the OBL-001 bridge
and selects one of the two presently admissible dispositions is operative. A
design authorization must state its existing route and permitted persistence;
without both, the bridge remains unselected. A generic continuation instruction
never meets this condition.

**Reopen trigger after explicit defer:** an OBL-001 proof-facing package names
the concrete interpretation as necessary. That trigger returns the item to
owner/selection review; it does not authorize a bridge artifact or waive the
moratorium.

**Requested canonical act:** none at this point. The immediate requested act
is an owner-recorded LAB disposition. A committed bridge artifact, a canon
semantic change, a ledger update, or a Gate/Phase change would each require its
own appropriate canonical process.

**Independent review:** Oracle session
`t0-bridge-authority-and-next` reviewed this source cut and concluded that a
broad autonomous-continuation instruction is not an operative defer/authorize
record. The advice is advisory; its conclusion was checked against the local
canon and this plan. The wrapper did not independently verify the model picker,
so no model-selection claim is made. Its exact-file review then found six
scope-clarity defects; the route/persistence condition, generic-instruction
guard, two-disposition limit, identifier cut, non-sufficiency wording, and
reopen separation were corrected. A final re-review returned PASS with no new
scope or authority defect.

**Current disposition:** waiting for the owner to record one of the two
presently admissible dispositions. No autonomous successor research unit is
selected in the meantime.

## Current non-claims

This plan does not assert that G0 exits, that T1 begins, that G1 is ready, that
any OBL is `stated` or `lean-stated`, that an exploratory Lean proof proves a
canon theorem, or that existing runnable samples satisfy conformance.
