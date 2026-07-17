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
Lean interpretation, or artifact identity was added. At this preflight close,
no successor was selected; the later direct-theory resumption reading below
governs unrelated eligible research units.

**Separate decision-ready blocker:** a bridge-specific owner disposition would
be needed before any separately scoped concrete-evidence bridge design work,
such as comparing a read-only JSON-to-`Pred` interpretation, carrier exposure
for `C`/`O` and capability/witness references, or a reviewed artifact binding.
Such a disposition does not by itself authorize a committed bridge artifact or
waive the pre-T1 moratorium. The current LAB recommendation is to defer the
bridge until an OBL-001 proof-facing package actually needs it. This is an
evidence-route decision, not a request to change canon authority semantics,
Gate/Phase state, or proof status.

## Work-selection resumption reading (2026-07-17)

The owner's later, direct objective to autonomously solidify the theory while
preserving its purpose and rejecting proof laundering authorizes unrelated
existing-lane LAB research selection. It does not name the OBL-001 bridge, so
it is not a bridge-specific owner disposition and must not be recorded as an
explicit defer.

For work selection only, the bridge remains absent, dormant, and non-blocking:
do not design it, create an artifact, or treat OBL-001 itself as deferred;
reopen it only when an OBL-001 proof-facing package shows that a concrete
interpretation is necessary. This supersedes the former broad queue stop for
other eligible research units without altering the bridge bundle's provenance
condition, canon, Gate/Phase state, or proof status.

## T-RESEARCH-005: `[E-DEGRADE]` / `[E-REACQ]` restricted lineage kernel (research-complete)

**Question:** can a smallest, explicitly conditional model make the project
axis "monotone degradation; explicit fresh recovery" mechanically precise
without silently choosing the canonical representation of lineage, history,
or the full MirCore step relation?

**Source cut:** `mirrorea_canon/theory/01-mircore-v0.md` (the `L` carrier,
the chain-position well-formedness wording, `[E-DEGRADE]`, `[E-REACQ]`, and
OBL-020), `theory/04-ordering-and-cuts.md` (degrade-to-later-access causal
family), `theory/06-existence-fallback.md` (THM-002), ADR-0004, and SCN-08.
The cited canonical THM-002, not a scratch `history maximum` predicate, is the
source of the nondecrease direction.

**Experiment:** two disposable Lean files under
`/tmp/mirrorea-t-research-005/` define only an experiment-local `ChainState`:
lineage keys are `(witness, epoch)` pairs; positions are an `Option Nat` map;
`seen` is a support list; and `SupportInvariant` means only
`defined-position -> seen`. The two scratch constructors are a target-only
nondecreasing update and a reacquisition that inserts an absent pair while
framing older entries. `Run` is their reflexive-transitive closure only.

**Positive result:** conditional on that representation and its constructor
premises, every map entry defined at the start of a restricted run remains
defined and pointwise nondecreasing. A named two-step restricted run degrades
one old entry and initializes a distinct experiment-local pair at a lower
index without changing prior entries. This is neither a selected/active
lineage theorem nor the "only via explicit reacquire occurrence" half of
THM-002.

**Negative evidence:** a direct `2 -> 1` update falsifies the pointwise
property; an arbitrary unrelated `4 -> 0` mutation shows that some non-target
preservation condition is needed for this global property (not that
target-only framing is necessary); a deliberately weakened reacquire relation
can reset an existing key and violates the property; pair inequality is shown
strictly weaker than separate witness and epoch inequality; and two states
with a state-local `position <= maximum` invariant can still regress. The last
model is not canon's history maximum; it only rules out substituting separate
state-local inequalities for a transition monotonicity claim.

**Binding and non-claims:** `SupportInvariant` is not MirCore
well-formedness; `seen`, `Option Nat`, target-only framing, indefinite old
entry persistence, lineage-as-pair, pair absence, and the two-rule `Run` are
all experiment-local sufficient conditions. The monotonicity proof uses pair
absence and framing; it does not establish the necessity, global freshness, or
canonical meaning of the constructor's witness/epoch inequalities. No full
MirCore `Config`, lease/admissibility, chain access, occurrence DAG,
rollback/cut/load, other-step frame, canonical history-max, THM-002, OBL-007,
OBL-008, OBL-020, SCN conformance, or proof-status claim follows.

**Reproducibility:** on Lean 4.29.1, both scratch files compiled with
`lean --trust=0`; a named-source scan found no `sorry`, `admit`, declared
axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`. `#print axioms`
reported the sole dependency `propext` for the two audited positive theorems.
The exact commands, exit results, source hashes, and independent-review
boundary are recorded in Report 2258. The scratch files remain disposable and
untracked.

**Review and classification:** the first Oracle review found scope and
negative-evidence wording defects; the scratch model was corrected and a
second exact-file review returned PASS. The review is advisory only. The result
is `research-complete`, not `decision-ready`: no canonical semantic choice is
needed to preserve this conditional result, but any attempt to adopt its
representation or constructor equations as canon requires a separate stop and
decision bundle.

**Next selection point:** choose a further rule x invariant clause x falsifier
only under this plan's selection rule. Do not generalize this restricted run to
OBL-020 or use it to pull G2 statement work ahead of the foundational OBL-020
and OBL-021 boundary.

## T-RESEARCH-006: OBL-020 selected-transition source-adequacy audit (research-complete)

**Question:** does the frozen canon source cut provide enough formal premises to
derive any selected `E-*` transition x named well-formedness-clause preservation
case without importing an unstated update, frame, freshness, graph, or record
invariant assumption?

**Source cut and method:** the audit fixes the 13 selected operational cases in
`theory/01` (`E-WRITE`, `E-REQ`, three `E-SERVE` outcomes, `E-PUB`, `E-OBS`,
`E-ADMIT`, `E-CUT`, `E-DEGRADE`, `E-REACQ`, and two `E-PATCH` outcomes) against
the five named clauses of `theory/01` well-formedness. It follows one-hop
normative references through `theory/04` to `theory/08`; the proof ledger is
read as a status boundary, not as a source of premises. A cell is `direct` only
when the source text alone gives a derivation-complete preservation argument;
an unproved theorem statement cannot be used as a premise. The resulting
disposable CSV is exactly 65 cells and rejects missing cases/cells, duplicate
cells, and merging `[E-SERVE]/fail` into a pass branch.

**Result:** all **65 / 65** cells are `missing`; none is `direct` or
`delegated`. This does not mean the high-level direction is wrong. It means the
current L1-fixed/draft prose has rule sketches and theorem
statements rather than the complete transition, history-extension, frame,
freshness, and record equations that an OBL-020 proof would consume.

**Normalized missing-premise groups:** the audit replaces a misleading generic
"global frame" bucket with five exact groups: `H_EXTENSION` (33 cells: fresh
event identity, retained old nodes/edges, exact added edges and orientation),
`COMPONENT_FRAME` (18: unchanged named configuration fields),
`STATE_MEMBERSHIP_COHERENCE` (4: indexed key/tombstone/recorded-epoch relation
across `S` and `M`), `AUTHORITY_RECORD_BINDING` (6: request/use/validation/
grant/witness/frontier lineage records), and `CHAIN_TRANSITION_DEFINITION` (4:
lease update, history maximum, lineage, and cut interaction). The cells retain
a specific detail field; this grouping is a LAB audit taxonomy, not new canon
vocabulary.

**Adversarial evidence:** a disposable two-state Lean model proves that an
Active-key predicate is preserved only if both its store and membership-epoch
inputs frame. It contains a state with the same stored epoch but a changed live
epoch that violates the predicate. Thus `[E-SERVE]/fail`'s prose `no store
change` is useful partial evidence but cannot by itself establish
`WF-ACTIVE-KEY`. Existing T-RESEARCH-002/003/005 negative evidence remains
local support for the store-key, history-extension, and chain rows respectively.

**Important source readings:** `[E-PUB]` and `[E-OBS]` do state the new event
kind and the new observation's publish-ancestor condition, but do not define
the required preservation of old history/ancestry; their blocker is therefore
`H_EXTENSION`, not a missing new-observation condition. Likewise,
`theory/08` says a deferred patch mutates lifecycle rows only, but THM-006 is
OBL-019 in the open ledger and cannot be imported as an OBL-020 premise. Its
five cells remain missing.

**Reproducibility:** under `/tmp/mirrorea-t-research-006/`,
`python3 validate_matrix.py obl020_source_adequacy.csv` reported 65 cells and
`0 direct / 0 delegated / 65 missing`; `python3 mutation_validation.py`
rejected four structural mutations. `lean --trust=0 ServeFailureFrame.lean`
passed; `#print axioms` reported no axioms for its positive frame lemma. Exact
commands and hashes are in Report 2259. All scratch files are disposable and
untracked.

**Review and classification:** the first exact-file Oracle run ended before an
answer because its browser disconnected. The one permitted retry also ended
with a browser failure, but its saved transcript contained an independent
review of the source cut. It confirmed `0 direct / 65 missing`, rejected using
unproved THM-006 as a premise, required the five-group taxonomy above, and
approved `research-complete` only as a frozen LAB source-adequacy result. The
advice is advisory and model-picker selection was not verified.

**Binding and non-claims:** this audit neither defines MirCore history append,
field frames, Active/tombstone semantics, authority records, chain transition
semantics, nor a proof interface. It does not complete OBL-020, alter a canon
rule, alter `theory/11`, change Gate/Phase status, create a new mainline
evidence lane, or establish conformance/runtime correctness.

**Next selection point:** a later source-grounded OBL-020 statement/premise
research unit may compare existing LAB statement drafts against these five
missing-premise groups. It must stop as `decision-ready` before proposing any
canonical transition equation, frame convention, or carrier definition.

## T-RESEARCH-007: OBL-020 formalization-boundary bundle (decision-ready)

**Question:** after T-RESEARCH-006 established that no selected OBL-020 cell
has derivation-complete canon premises, what is the smallest owner decision that
organizes later proof-facing formalization without choosing a hidden runtime
representation or language feature?

**Finding:** the existing abstract `WellFormed` / `Step` LAB statement should
not be refined now. `plan/126` already establishes that this abstraction is
appropriate until concrete carriers and per-step proof obligations exist; the
five T-RESEARCH-006 groups explain why adding fields now would launder a choice
of history, frame, authority-record, or chain semantics into LAB.

**Prepared bundle:** `mirrorea_canon/meta/proposals/PROPOSAL-003-obl020-formalization-boundary-review.md`
asks the owner to select one organizational posture only: A, a shared
five-heading LAB-derived review checklist (advisory recommendation); B, no
required shared checklist and package-local organization; or C, defer the
organizational choice. The headings are not canon predicates or fixed Lean
premises. The proposal explicitly does not choose any concrete `Config`, history
append, field frame, record schema, rule equation, scheduler, Lean artifact
identity, OBL status, Gate, or Phase change.

**Authority cut:** this is an L1 organizational choice for the proof-facing
calculus. `plan/02` assigns L0/L1 decisions to the owner, and
`meta/agent-instructions` permits only proposal preparation, not adoption.
The proposal is a non-self-executing L3-open decision request; its index entry
was regenerated, but no ADR, changelog record, or normative theorem wording was
added.

**Classification:** `decision-ready` for the OBL-020 formalization boundary.
It does not block unrelated existing-lane theory research. OBL-021 or other
bounded counterexample/statement work may continue under the selection rule;
no OBL-020 proof-facing definition may be adopted until an owner disposition is
recorded through the canon process.

## T-RESEARCH-008: OBL-021 BND-001 postcondition source-adequacy audit (research-complete)

**Question:** excluding OBL-021 itself as a premise, do existing canon sources
give a derivation-complete basis for each conjunct of the abstract
`ElabDeterministicPost` statement?

**Result:** all three rows are canonically motivated but not derivation-complete:
successful-result equivalence lacks result/projection interpretation and
coherence; rejection equivalence lacks rejecting-branch interpretation and
diagnostic equivalence laws; success/reject exclusion lacks shared-outcome
coherence or an explicit disjointness rule. The frozen audit is `0 direct / 0
delegated / 3 missing`.

**Falsification:** disposable trusted Lean models preserve success functionality
while allowing inequivalent projections, preserve rejection functionality while
using non-reflexive diagnostic comparison, and preserve both branch
functionalities while allowing overlap. Targeted mutations repair only the
respective defect. They are not canon countermodels.

**Boundary:** the abstract statement remains unchanged. No equality, diagnostic
equivalence, totality, outcome carrier, proof interface, OBL status, Gate, or
Phase is selected. The audit is `research-complete`; a concrete proof package
must stop `decision-ready` before choosing any missing contract.

## T-RESEARCH-009: OBL-005 structural-flattening kernel (research-complete)

**Question:** do the two settled equations in `theory/06` — singleton for a
lone option and left-to-right append for `fallback` — suffice for a smallest
algebraic output-equality result under one syntactic reassociation, without
choosing a MirCore source AST, canonical carrier, validity predicate,
evaluation, or source-level empty fallback term?

**Method:** a disposable trusted Lean file under
`/tmp/mirrorea-t-research-009/CanonicalFlatten.lean` uses an opaque leaf
carrier, an experiment-local finite binary shape, and `List` only as a free
ordered-word output model. Its `flattenShape` fold is a structural witness for
the two equations; it is deliberately not named or treated as a total
canonicalization judgment. It proves output equality for the one root
reassociation and the identity of the empty one-hole **meta-context**. The
source shape has only `leaf` and `fallback`; the hole is not a source term.

**Positive result:** the root reassociation theorem follows from append
associativity. The empty-hole context plugs to its argument by definition. The
experiment does not prove arbitrary-context reassociation, source-shape
equality, validity preservation, lineage-annotation preservation,
admissibility, denotation, evaluation, normalization, confluence, uniqueness,
or a leftmost-selection theorem.

**Negative evidence:** an order-reversing fold still satisfies the same root
reassociation equation, so associativity alone does not test the settled
left-to-right direction. A two-distinct-leaf orientation oracle separates it
from `flattenShape`. A second disposable mutation begins with no empty source
constructor (so a source-unit theorem fails to typecheck), then adds one and
proves left/right source units. That demonstrates that a source-level empty
chain is a signature change, not a consequence of the settled equations.

**Review and classification:** a completed Oracle review accepted this as a
bounded LAB algebraic kernel after requiring: structural-output rather than
canonicalization wording; a one-hole context identity instead of list/source
unit wording; an explicit no-validity boundary because edge-local lineage
annotations are not reassociation-neutral; and the order-reversal mutation.
All requested corrections were applied. This is `research-complete`, not
`decision-ready`: no canonical choice is needed for the stated result. It does
not discharge OBL-005 or select what the ledger's shorthand `assoc/unit` would
mean for any later source-level empty-chain interpretation.

**Reproducibility:** Lean 4.29.1 compiled both disposable files with
`lean --trust=0`. `#print axioms` reported only Lean's `propext` for the
append-associativity results and no axioms for the hole-context identity. A
named scan found no `sorry`, `admit`, declared `axiom`, `opaque`, `unsafe`,
`partial`, or `implemented_by`. Exact commands, the failed-before-defined
checks, output, and source hash are recorded in Report 2262. Scratch artifacts
remain disposable and untracked.

**Binding and non-claims:** opaque leaves do not choose option fields,
equality, guard, contract, capability, lease, or admission representation.
`List` is an experiment-local free-word carrier, not the selected canonical
form. The work does not alter `theory/06`, `theory/11`, a Gate, a Phase, a
chain notation, a parser, a runtime, or proof status.

**Next selection point:** a later OBL-005 package must make its exact
source-level `unit` interpretation explicit before claiming a full obligation
discharge. Any OBL-006 work must first name an existing source-grounded rewrite
or equivalence relation; do not infer it from this one output-equality lemma.

## T-RESEARCH-010: OBL-006 relation-boundary source audit (research-complete)

**Question:** do the settled fallback equations and same-order denotation
sentence define enough of a term domain and relation to derive a canonical
uniqueness or confluence theorem?

**Result:** `0 direct / 0 delegated / 1 missing`: the source cut motivates
OBL-006 but does not fix its **formalization boundary**. It leaves open the
related objects, guarded validity domain, equality/denotation, meaning of
`uniqueness / confluence`, and any reduction or equivalence relation with its
orientation, closure, reachability, and joinability target. This is not a list
of mandatory theorem mechanisms: termination, a normalizer, and equivalence
closure depend on the eventual theorem architecture.

**Evidence:** a disposable trusted Lean model gives three test shapes with the
same ordered leaves. Its two proper steps preserve that word but lead to
distinct irreducible branches with no join. Thus singleton/append equations,
same ordered output, and per-step output preservation do not determine
confluence. The test syntax and reachability relation are not Surface/Core
syntax or a canonical relation.

**Classification:** an Oracle review passed this bounded audit and required
the missing item to be named an OBL-006 formalization boundary rather than an
isolated premise. No owner decision is needed to record the audit. A decision
bundle is required before a proof-facing OBL-006 statement selects a domain,
guarded validity condition, equality/denotation, and relation, or interprets
the ledger slash as one theorem, two properties, or alternatives.

**Non-claims:** this audit does not change `theory/06`, the ledger, the
Surface/Core grammar, canonical carrier, rewrite relation, OBL status, Gate,
or Phase. Exact evidence is in Report 2263.

## T-RESEARCH-011: THM-002 / OBL-007 trace-formalization boundary audit (research-complete)

**Question:** does the existing THM-002 source cut directly determine a
proof-facing Lean statement for monotone fallback selection and explicit fresh
reacquisition?

**Source reading:** `theory/06`, ADR-0004, and SCN-08 directly fix the
normative policy: selection is non-decreasing on one lineage; an earlier option
may be selected again only through an explicit reacquire that starts a new
lineage with a fresh witness and epoch. `theory/01` names the lease/chain store,
well-formedness condition, and `[E-DEGRADE]` / `[E-REACQ]` rule sketches; the
ordering chapter gives one degrade-to-later-access causal generator. This audit
does not weaken or question that policy.

**Result:** `0 direct / 0 delegated / 1 missing` **THM-002 formalization
boundary** for a complete Lean statement. The sources do not define a trace
carrier or admissibility predicate, a non-circular later/alongs relation, a
chain-instance identity, the selected-option observation and its bridge to
`L.position`, lineage continuity/creation, a recognizable reacquire occurrence,
the interval/causal meaning of "only via", separately bound witness and epoch
freshness, an `[E-DEGRADE]` transition binding, other-step framing, or the
history-maximum interpretation. These are a coupled formalization boundary,
not silently adopted theorem premises.

**Adversarial evidence:** one disposable trusted Lean model admits a `2 -> 1`
selection because an unconstrained trace schema lacks transition and lineage
constraints. A second has ordered accesses `a0 < a1 < a2`, keeps `a0/a1` on
one nondecreasing lineage, gives `a2` a distinct fresh-witness/fresh-epoch
lineage at index `0`, and declares no reacquire occurrence. Thus same-lineage
monotonicity plus new-lineage freshness does not establish the "only via
explicit reacquire" clause without a lineage-origin/reacquire bridge. The
finite carriers and predicates are experiment-local, not MirCore.

**Classification:** no owner decision is needed to record this bounded audit.
Before a proof-facing OBL-007 statement selects any member of the boundary, it
must stop `decision-ready` for the relevant canon/owner formalization act. The
scope is one complete THM-002 statement boundary; it neither completes OBL-007
or OBL-008 nor blocks unrelated eligible research.

**Non-claims:** this audit does not define canonical traces, selection,
lineages, histories, events, freshness, `L.position`, `[E-DEGRADE]`,
`[E-REACQ]`, the theorem interface, or a proof. It does not change `theory/01`,
`theory/04`, `theory/06`, ADR-0004, SCN-08, the ledger, a Gate, or a Phase.
Exact evidence is in Report 2264.

## T-RESEARCH-012: THM-004 / OBL-015 mutation-origin boundary audit (research-complete)

**Question:** excluding THM-004 and OBL-015 as premises, does the source cut
derive a complete proof-facing bridge from every owner-state mutation to either
an owner-local declared transition or a specific validating capability use?

**Source reading:** `theory/05` and ADR-0005 fix the authority policy: delegated
or capability-mediated authority is carried only by grant lineage, validated on
verdict, principal, role, target, epoch, incarnation, required witness, and
policy version. THM-004 separately permits owner-local mutation under the
owner's declared transitions. `theory/04` gives the `capability_grant ->
capability_use` causal generator. `theory/01` says every `use` has a matching
grant ancestor, sketches `[E-SERVE]` validation followed by an operation, names
owner-local writes, and lets `[E-ADMIT]` issue grants/witnesses. This audit does
not weaken any of those policy statements.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled
mutation-origin/authorization formalization boundary**. The sources do not
define an occurrence-level association from a mutation to a particular
validating use/request/capref, a complete owner-local classifier or declared
transition relation, a complete trace/step construction linking validation,
use, service, and mutation, or their required frame facts. The single missing
item is the coupled boundary, not an adopted Boolean premise.

**Adversarial evidence:** a disposable trusted three-event model has
`grant < use < mutation`. The `use < mutation` edge is an explicit favorable
experiment-local strengthening, not a canonical causal generator. Grant and
use match every listed lineage coordinate, the history is acyclic, and the
mutation is not owner-local; nevertheless an everywhere-false
`MutationUses(mutation, use)` makes the delegated theorem shape false. Its twin
changes only that experiment-local relation and satisfies the finite delegated
shape. The evidence isolates semantic association despite favorable causal
precedence; neither carrier is MirCore nor a legal canonical trace.

**Classification:** no owner decision is needed to record this bounded audit.
Before a proof-facing OBL-015 statement selects the trace, occurrence,
owner-local, validation, or association interface, work must stop
`decision-ready`. This result neither completes OBL-015/016 nor blocks
unrelated eligible research.

**Non-claims:** this audit does not cover the owner-local/declared-transition
branch; define canonical traces, occurrences, state mutation, requests,
caprefs, lineage equality, validation, causal closure, or `MutationUses`;
show that the binary bridge is necessary, minimal, unique, or sufficient for a
canonical statement; refute THM-004; establish anti-spoofing, runtime,
conformance, lifecycle, revocation, load/rollback correctness; or change
`theory/05`, ADR-0005, the ledger, a Gate, or a Phase. Exact evidence is in
Report 2265.

## T-RESEARCH-013: THM-005 / OBL-017 observer-safe export boundary audit (research-complete)

**Audit row:** the complete proof-facing THM-005 / OBL-017 statement
interpretation, excluding THM-005 and OBL-017 themselves as premises.

**Question:** does the existing observation source cut determine a complete
Lean statement for observer-safe noninterference, including configuration
low-equivalence and observer-safe export equality?

**Source reading:** `theory/07` directly fixes the observer-safe
noninterference policy, observation-event vocabulary, typed observation effect,
occurrence/telemetry provenance requirement, retention and redaction
constraints, and forbidden observer-safe contents. SCN-07 directly supplies a
position-only observer-safe expectation, session-local retention, occurrence
provenance, and absence of raw witness/auth material. `theory/02` permits a
finite label lattice but does not select a final lattice; OPEN-020 remains
open. Existing `CurrentL2IfcSecretExamples.lean` is adjacent LAB evidence: its
two-point label lattice and declassification predicates are neither a delegated
OBL-017 formalization nor imported by this audit.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled THM-005
formalization boundary**. This count describes the audit row, not absence of
direct policy facts. Excluding the theorem and obligation themselves as
premises, the source cut does not determine one proof-facing interpretation
coupling: canonical configuration low-equivalence and permitted high/raw
variation; observer context and observer-safe export shape; and output equality
or equivalence, including treatment of event/reference identity, order, and
multiplicity. The single missing item is this coupled boundary, not a list of
adopted Boolean premises.

**Adversarial evidence:** a disposable trusted two-configuration model gives
both configurations the same modeled low position and different designated
high-state and aggregate raw-payload projections. Both opaque exports satisfy
the same selected, stipulated experiment-local side predicates: observer-safe,
occurrence-derived, session-local, free of the aggregate forbidden payload, and
equal visible position. Constructor identity makes the finite
noninterference shape false; equality of the visible-position projection makes
the same shape true. These are explicit experiment-local relations, not a
canonical row, export-collection, or observation ABI. The aggregate raw axis
does not independently model raw witness and raw authority material.

**Classification:** recording this boundary needs no owner decision and is
`research-complete`, not `decision-ready`. Before a proof-facing OBL-017
package selects a configuration carrier or low-equivalence; label domain,
order, declassification, or redaction algebra; observation/event/export ABI;
identity/renaming/order/multiplicity semantics; or turns stipulated predicates
into pipeline semantics, work must stop `decision-ready`. It also stops before
using the adjacent two-point lattice as canonical/delegated evidence or adding
a persistent helper, evidence lane, schema, CI surface, or formal artifact.

**Non-claims:** this audit does not counterexample THM-005 or deny its
canonical policy; define the OBL-017 statement, discharge OBL-017/018, define
the explicit-flow or declassification theorem, select a final lattice/flow/
redaction relationship, identify the toy `Config` with a MirCore
configuration, identify toy exports with an `ObservationEvent` or export ABI,
select any equality/quotient/trace/renaming/order/multiplicity semantics,
derive observer safety/provenance/redaction/retention from a pipeline, cover
SCN-07, conformance, runtime, or viewer behavior, prove necessity/minimality/
uniqueness/sufficiency of a future interface, or change canon, ledger, Gate,
Phase, or proof status. Exact evidence is in Report 2266.

## T-RESEARCH-014: THM-003 / OBL-009 successful-load restoration boundary audit (research-complete)

**Audit row:** the complete proof-facing THM-003 / OBL-009 successful-load
restoration interpretation, excluding THM-003 and OBL-009 themselves as
premises.

**Question:** do the theory/04 successful-load conditions plus the theory/01
runtime vocabulary determine a relation from a SaveObject to the restored
configuration and restored history prefix on which the THM-003 result-side
safety properties are evaluated?

**Source reading:** theory/04 directly supplies one grouped SaveObject schema
anchor and eight individually named necessary conditions for a successful load:
consistent saved cut; no rollback across atomic cut; no stale membership,
witness, or lease resurrection; connected capability/auth provenance;
compatible package versions; and compensated or isolated external irreversible
effects. It separately states THM-003's target and the explicit-reacquire
policy. Theory/01 supplies the runtime `Config` and `WellFormed` vocabulary,
including acyclic history, grant/use lineage, observation ancestry, indexed-key,
and chain-position categories. That is a chapter-local vocabulary delegation,
not delegated proof evidence. The ledger keeps OBL-009 through OBL-014 and
OBL-027 open.

**Source-anchor inventory:** `1` direct grouped SaveObject schema anchor + `8`
direct successful-load condition anchors + `1` delegated `Config` / `WellFormed`
vocabulary-family anchor. This accounting is not the source-adequacy result:
the theorem sentence is the audited target, not a proof premise, and no anchor
is a complete Lean interpretation.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled successful-load
restoration formalization boundary**. Excluding THM-003 and OBL-009 as
premises, neither chapter determines a derivation-complete interpretation of
successful-load recognition, SaveObject-to-restored-Config association,
restored-prefix projection, the meanings of result-side liveness predicates,
or the bridge from necessary no-resurrection/admissibility conditions to
`WellFormed` and no-live postconditions. The one missing item is this coupled
boundary, not a request to adopt individual Boolean premises.

**Adversarial evidence:** a disposable trusted model has one SaveObject and
two Config values. Both candidate `LoadResult` relations respect the same eight
selected successful-load condition tags. Saved cut and restored prefix use the
same experiment-local `Consistent` predicate through distinct projections; all
are empty in the toy model, so the consistency facts are deliberately vacuous.
Both outputs satisfy five stipulated no-live predicates and four modeled
well-formedness categories. The good result satisfies the selected THM-003
shape. Changing only the experiment-local result relation to return the bad
configuration, whose `StoreKeyWF` tag is false, falsifies that shape. This does
not construct a legal MirCore load or prove that every THM-003 conjunct is
independently underdetermined.

**Classification:** recording the boundary is `research-complete`, not
`decision-ready`. Work must stop `decision-ready` before selecting a canonical
Load relation/result discipline, SaveObject or restored-Config interface,
saved-cut/restored-prefix relation, liveness/resurrection/provenance bridge,
rule that makes the eight necessary conditions sufficient, or a persistent
theorem/helper/schema/evidence/CI surface.

**Non-claims:** this audit does not counterexample THM-003, define a canonical
load or serialization algorithm, full Config, causal history, atomic-cut
behavior, consistency checker, resurrection/liveness/provenance semantics,
package compatibility, compensation, or local/distributed durable save/load;
identify no-resurrection with no-live; prove OBL-009 through OBL-014, OBL-020,
or OBL-027; establish necessity/minimality/uniqueness/sufficiency of a future
interface; or change theory/01, theory/04, ledger, Gate, Phase, or proof
status. Exact evidence is in Report 2267.

## T-RESEARCH-015: OBL-026 transparent-overlay composition boundary (research-complete)

**Question:** does theory/02's transparent-overlay prose itself determine the
proof-facing composition statement for a stack of transparent layers?

**Source reading:** theory/02 directly fixes ten directions: input, output,
precondition, postcondition, combined effect/failure, ordinary-path capability,
provided surface, observation, redaction, and retention. It does not define
their orders, a Contract carrier, layer composition, or equality.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled transparent-overlay
composition formalization boundary**. A disposable ten-component Nat-preorder
kernel proves pairwise transparency composes when every component has the
chosen transitive order. A three-contract opaque-label model has two pairwise
transparent labels but not their composite. The latter is not a canon
counterexample; it isolates the missing component orders and composition law.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical contract directions, `all_of`/`any_of`,
ContractUpdate, a final ABI, OBL-026 statement/proof, or cost algebra. Stop
`decision-ready` before selecting any of those interfaces. Exact evidence is
in Report 2268.

## T-RESEARCH-016: OBL-028 revocation-monotonicity boundary (research-complete)

**Selection:** OBL-003 has finite-fragment direction but no derivation-ready
Line-1 judgment or grammar interface. OBL-019 would substantially repeat the
existing E-PATCH transition/frame gap in T-RESEARCH-006. OBL-028 has direct
authority-lifecycle anchors and is independent of T-RESEARCH-012, whose
non-claims expressly exclude revocation.

**Question:** does theory/05's statement that revocation is monotone unless a
new epoch/evidence is issued itself determine a proof-facing OBL-028
revocation/reissue statement?

**Source reading:** theory/05 fixes epoch/incarnation lifecycle, tombstoning,
old grant/witness non-revival, and the monotone-revocation policy. theory/01
names the membership, capability, and witness stores; theory/04 and SCN-03/04
add causal/load and stale-capref expectations. They do not define revocation,
reissue, state identity, or trace transitions.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled revocation-
monotonicity formalization boundary**. A disposable action model proves that a
revoked snapshot remains revoked when its two explicitly chosen reissue actions
are absent. An unstructured-label model permits a revoked-to-active label pair
while labels named no-reissue and transition also hold. The latter is not a
canon counterexample; it isolates the missing semantic link.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical revocation, epochs, evidence, grants, witnesses,
caprefs, state/trace relations, load/rollback behavior, runtime ABI, OBL-028
statement/proof, or status. Stop `decision-ready` before selecting any of those
interfaces. Exact evidence is in Report 2269.

## T-RESEARCH-017: OBL-022 stream read-side boundary (research-complete)

**Selection:** OBL-022 has direct two-layer-time and typed-adapter anchors.
OBL-027 remains closely coupled to the successful-load/cut source family
already audited in T-RESEARCH-014, while OBL-023 explicitly has a pending
formal statement and open clock/latency model. The OBL-022 question is
independent of the earlier theorem-statement boundaries.

**Question:** does theory/09's rule that samples cannot influence discrete
state except via declared adapter effects itself determine a proof-facing
read-side OBL-022 statement?

**Source reading:** theory/09 fixes samples as non-occurrences outside `H`,
not saved per-sample, anchor/frontier admissibility, drop-not-buffer behavior,
and the read-side policy. BND-007 prevents View/Provider semantic ownership
and confines their connection to typed adapters. It does not define samples,
discrete state, adapter effects, or a transition relation.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled stream read-side
formalization boundary**. A disposable action model proves a sample-only branch
preserves an experiment-local discrete field while its distinct adapter and
discrete branches may change it. An unstructured-label model gives a sample
both no-adapter and discrete-change labels. The latter is not a canon
counterexample; it isolates the missing frame/transition link.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical streams, samples, anchors, discrete Config,
adapter effects/ABI, effect declarations, clocks, transition/trace relations,
frame equality, OBL-022/023 statement/proof, or status. Stop `decision-ready`
before selecting any of those interfaces. Exact evidence is in Report 2270.

## T-RESEARCH-018: OBL-027 atomic-cut rollback boundary (research-complete)

**Selection:** T-RESEARCH-014 expressly leaves OBL-027 separately open. The
diagnostic obligations already have detailed LAB statement-shape inventories,
and OBL-019 substantially overlaps the E-PATCH transition/frame gap in
T-RESEARCH-006. OBL-027 has direct local-cut policy anchors without selecting
the load/restoration interface excluded by T-RESEARCH-014.

**Question:** does theory/04's rule that local rollback cannot remove
occurrences causally before `cut(ell)` itself determine a proof-facing OBL-027
statement?

**Source reading:** theory/04 fixes a locus-local rollback frontier and the
causal-before property; theory/01 fixes the `cut(ell)` / `[E-CUT]` reading.
theory/06 and ADR-0004 say rollback/atomic_cut do not rewind degradation;
ADR-0007 says it is not a memory fence. They do not define occurrences,
causality, locus membership, cut projection, or rollback.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled atomic-cut rollback
formalization boundary**. A disposable Nat-frontier model proves a target at or
after an explicitly chosen cut retains that cut. An unstructured-label model
gives a cut both local-rollback and causal-containment labels while targeting a
pre-cut point. The latter is not a canon counterexample; it isolates the
missing order/result relation.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical history, occurrence identity, causal order,
locus relation, cut/frontier projection, rollback semantics, load/persistence
ABI, OBL-027 statement/proof, or status. Stop `decision-ready` before selecting
any of those interfaces. Exact evidence is in Report 2271.

## T-RESEARCH-019: OBL-023 temporal-coherence boundary (research-complete)

**Selection:** theory/09 expressly leaves OBL-023's formal statement pending
and its clock/latency model open. T-RESEARCH-017 deliberately separated this
per-consumer coherence question from OBL-022's read-side boundary. OBL-023
therefore has a distinct source cut without selecting the stream/adapter
transition interface excluded by T-RESEARCH-017.

**Question:** do theory/09's frontier-admissibility and no-split-frame policy
itself determine a proof-facing OBL-023 temporal-coherence statement?

**Source reading:** theory/09 fixes two-layer time, consumer-frontier
admissibility, epoch match, dropping inadmissible samples, and a no-split-frame
working law for discrete atomic groups. BND-007 fixes non-owning
View/Provider placement. Theory/04 and ADR-0007 constrain the high-level
frontier/order reading. They do not define consumers, atomic grouping,
interpretation, coherence, or clock/latency semantics.

**Result:** `0 direct / 0 delegated / 1 missing` **coupled per-consumer
temporal-coherence formalization boundary**. A disposable shared-frontier
model proves that two epochs admissible at one chosen frame are equal. An
unstructured-label model gives two labels called admissibility and no-split
while retaining distinct frontiers. The latter is not a canon counterexample;
it isolates the missing binding relation.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical consumers, samples, anchors, atomic groups,
frontiers, interpretation, temporal coherence, equality/observation, clocks,
latency, provider/transport ABI, OBL-023 statement/proof, or status. Stop
`decision-ready` before selecting any of those interfaces. Exact evidence is
in Report 2272.

## T-RESEARCH-020: OBL-010 consistent-cut checker kernel (research-complete)

**Selection:** theory/04 directly gives both the generating-family transitive
closure and the `Consistent(Kc)` prefix-closure definition. This is separate
from T-RESEARCH-014's load-restoration question and T-RESEARCH-018's
local-rollback question. The source can therefore pressure the direct closure
lemma without choosing a load, rollback, or persistence interface.

**Question:** what part of OBL-010 checker soundness follows before a canonical
finite checker is selected?

**Source reading:** theory/04 identifies causal order as the transitive closure
of every named direct generating edge and defines a consistent cut by prefix
closure. Theory/01 supplies the occurrence-DAG role; BND-002 fixes the
decidable-checker direction; ADR-0007 keeps the relation high-level. None
defines a finite carrier, enumeration, decider, checker result, or diagnostics.

**Result:** **one direct conditional mathematical kernel** plus **one remaining
full-checker formalization boundary**. A disposable generic Lean theorem proves
that closure under each direct predecessor implies closure under the transitive
closure. A finite two-edge model shows a checker that validates only
`send -> receive` can accept an observe-only cut while omitted
`publish -> observe` makes it inconsistent. The negative model is not a canon
counterexample; it isolates complete-family coverage as required evidence.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical events, history, cuts, generators, finite
enumeration, checker algorithm/result, diagnostics, SaveObject, Z-cycle,
persistence ABI, OBL-010 statement/proof, or status. Stop `decision-ready`
before selecting any of those interfaces. Exact evidence is in Report 2273.

## T-RESEARCH-021: OBL-004 no-undeclared-communication kernel (research-complete)

**Selection:** theory/03 directly requires every cross-locus consequence to
appear in `G_e` and prohibits other communication generation. LAB plan/73 and
plan/76 expressly keep the whole-program OBL-004 corollary outside the OBL-001
statement inventory, so this source cut is limited to composition algebra and
does not preempt THM-001, runtime, or transport work.

**Question:** what no-undeclared-communication composition fact follows before
a program/elaboration/runtime relation is selected?

**Source reading:** theory/03 fixes no hidden edges and names request, publish,
observe, and witness rows; theory/01 fixes `G_e` in the unified judgment;
spec/03 fixes cross-locus static duties; BND-001 and BND-004 delimit
elaboration and verdict-approved execution. They do not define program
composition, edge equality, declared-edge mapping, or runtime communication.

**Result:** **one direct conditional composition kernel** plus **one remaining
full-corollary formalization boundary**. A disposable generic Lean theorem
proves itemwise generated-edge containment composes over an experiment-local
binary sequence. A finite two-edge model shows a checker that validates only a
request accepts a sequential program containing an undeclared publish. The
negative model is not a canon counterexample; it isolates complete category
coverage as required evidence.

**Classification and non-claims:** `research-complete`, not `decision-ready`.
This does not define canonical programs, handlers, branches, `G_e`,
declarations, generated-edge equality, elaboration, effect/failure/authority
relations, runtime, transport ABI, THM-001/OBL-004 statement/proof, or status.
Stop `decision-ready` before selecting any of those interfaces. Exact evidence
is in Report 2274.

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

**Current bridge-specific disposition:** waiting for the owner to record one
of the two presently admissible dispositions. The later direct theory objective
permits unrelated existing-lane research selection around this dormant item;
it does not record the bridge as explicitly deferred or authorize its design.

## Current non-claims

This plan does not assert that G0 exits, that T1 begins, that G1 is ready, that
any OBL is `stated` or `lean-stated`, that an exploratory Lean proof proves a
canon theorem, or that existing runnable samples satisfy conformance.
