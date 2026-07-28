# Plan 203 - V1/R1 finite-sequence presentation candidate selection

## Role and authority

This is LAB repository memory for a possible bounded successor to WRK-0033.
It does not amend `mirrorea_canon/`, select a Core representation, or reopen
C3 proper. Canon remains normative. The selected P012 V1/R1 directions are
the only direction-level inputs; ADR-0014 remains the authority boundary for
any later L3 record.

The candidate is deliberately about closure of the already retained **finite
LAB presentation**, not about a Mir execution trace. It names no request,
attempt, occurrence, transport, scheduler, history, save/load, payload,
authority, or semantic correlation identity.

## Inputs and local novelty check

| Input | Bounded reading | Consequence |
| --- | --- | --- |
| ADR-0014 | a pre-registered conditional lemma may use an existing LAB lane when it changes no reserved boundary or helper surface | a successor may only reuse the existing fenced Lean evidence route |
| P012 V1/R1 | an administrative binding is the reference presentation; a machine presentation is allowed only as an explicit equivalent presentation with matching typed receipt and one-shot/failure boundaries | the candidate cannot change the state, reply, matching, or failure assumptions |
| Plan 187 | a real V1 machine presentation later needs trace equivalence and several additional relations | a finite list-fold result must not be called trace equivalence |
| WRK-0033 / retained artifact | equality is proved for every finite *one-step* state/reply pair, plus three weakened-assumption distinctions | the only potential increment is whether the fixed translation is preserved over arbitrary finite lists |
| Plan 199 / Plan 200 | C3 proper, C0-D, C1, C2-B, C4, C5 proper, C6, and full C7 remain deferred | the candidate stops before any carrier, source-elaboration, or semantic-selection question |

Focused repository searches found `presentation_refinement` only as the
one-step theorem in `plan/wrk-0033-v1r1-presentation-refinement.md`; no
retained `List.foldl`, finite-sequence, or arbitrary-list preservation theorem
for this presentation exists at the pinned cut. The search result is a
novelty check, not a claim about the whole historical repository or the Canon
semantics.

## Frontier comparison

| Candidate | Disposition | Reason |
| --- | --- | --- |
| C0-D outcome-totality restatement | not selected | at an abstract level it repeats P008 and existing outcome-totality evidence; making it exact chooses a domain, statement/OBL identity, equality, or Diagnostic boundary |
| C1, C6, C2-B | not selected | any useful positive comparison selects snapshot/evaluation, scalar/terminal resolution, or semantic identity/edge/persistence relations |
| C3 inference/desugaring | not selected | it requires an already selected normative source/elaboration basis and reconstructible authority/failure/history evidence; it belongs to deferred C7 |
| `C3-VR-SEQ-PRE`: fixed finite-sequence presentation closure | selected for pre-registration only | it can reuse every WRK-0033 finite type, transition, translation, observation, and assumption while testing the specific one-step-to-list boundary |
| no-candidate disposition | fallback | use it if an identical theorem is found at the frozen cut or the proposed proof changes the registered finite model |

The selection was challenged by a temporary Oracle review. That advisory review
agreed with the local comparison: the finite-sequence route is eligible only
under the exact non-change condition above, and the no-candidate disposition is
required if that condition fails. Its conclusion is not Canon authority.

## Proposed `C3-VR-SEQ-PRE` pre-registration

### Narrow question

With the exact WRK-0033 `AdminState`, `MachineState`, `LabReply`,
`adminStep`, `machineStep`, `toMachine`, local observations, and explicit
matching/single-use/failure-exclusion assumptions unchanged, does `toMachine`
commute with every step and therefore preserve the final local observation
after an arbitrary finite list of opaque LAB replies?

The intended result class is a conditional lemma in the current fenced Lean
route: a translation-preservation one-step lemma and an induction over an
ordinary finite list. It is not a new state machine, a model of message
delivery, a reachability theorem, or a semantic trace relation.

### Alternative, falsifier, and stop line

The alternative is that equality of one-step observations is insufficient:
some fixed state/reply pair may fail to preserve the translation, allowing a
later finite list to diverge. A reproducible counterexample is retained as the
result; the finite model must not be repaired to force a refinement theorem.

Stop and use the no-candidate disposition when any of the following occurs:

1. an equivalent arbitrary-finite-list theorem is already retained at the
   pinned authority/evidence cut;
2. the proof requires changing a state, reply, transition, observation,
   translation, matching rule, single-use rule, or failure-exclusion rule;
3. it requires multiple slots, payload/provenance, persistence, authority,
   redaction, history, scheduling, transport, reachability, request/attempt/
   occurrence identity, or a Mir semantic correlation; or
4. it is described as full trace equivalence, C3 completion, source inference,
   grammar, implementation, conformance, or public behavior.

### Non-effects and standing eligibility

The successor changes no Canon theory/spec/scenario/plan/ledger text and does
not choose a primitive, contract, source or wire form, API, helper, schema,
validator, CI/Make surface, sample, runtime, or production implementation. It
is confined to the existing `plan/`, `docs/reports/`, `working/`, and
disposable-fenced-Lean evidence route. It therefore remains a prospective
ADR-0014 L3 conditional lemma, not an L2 promotion or a request for an owner
decision.

## Evidence route and execution order

1. Commit this selection and synchronize LAB snapshots. No new Lean source is
   written or run in this selection package.
2. Create and push a new `working/WRK-0034` pre-registration that pins the
   exact WRK-0033 source/evidence cut, all input digests, the alternative,
   falsifier, non-effects, rollback trigger, and registered extraction/search/
   Lean commands.
3. Only after that registration is committed and pushed, append the exact
   finite source to the existing LAB evidence artifact, run the registered
   commands, and retain either the finite result or the falsifier.
4. Re-screen the remaining frontier. Do not extend the result into C3 proper
   or C7 without an ordinary Canon design package.

## Execution outcome

The selection was committed at `1553bcc8fd140ad5ca98f5d7294fd802f776c7f1`,
pre-registered as WRK-0034 at `384a94bb3882da7acab393a38d663cf8994c59b4`,
retained at `dc66f08237acd11e4de722cd67a42fae0b26e1eb`, and linked in Canon at
`c1af9c5007eb0a16ca6224d4742fd59883027321`. The 182-line Lean source passed
`--trust=0`; it retains only fixed one-step translation preservation and final
local-observation equality after every finite `List.foldl` of opaque replies.
Its copied 133-line predecessor prefix is byte-identical. This does not supply
a Mir trace, carrier, source inference, C3 proper, or implementation.

## Non-claims

This selection does not establish full trace equivalence, a read-result
carrier, correlation, pending control, receipt delivery, failure semantics,
source inference, syntax, Core rule, OBL, Gate/Phase movement, implementation
readiness, or public completion. A finite list of opaque replies is only a
mathematical input to the existing LAB comparison; it has no hidden transport
or runtime interpretation.
