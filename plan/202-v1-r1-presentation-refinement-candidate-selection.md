# Plan 202 - V1/R1 presentation-refinement candidate selection

## Role and authority

This is LAB repository memory for a possible next bounded research package. It
does not amend `mirrorea_canon/`, select a Core representation, or reopen C3
proper. Canon remains normative; P012's recorded V1 and R1 directions remain
the only relevant direction-level input.

`C3-VR-PRE` is deliberately smaller than Plan 199's C3 pending-control design.
It asks whether an already selected *restricted* result-binding contract can be
shown in two explicitly related **administrative presentations**. It does not
name a Mir pending object, request, attempt, occurrence, transport message, or
semantic correlation identity. A later `working/WRK-0033` record must pin the
authority and LAB input cut before any model or Lean command is run.

## Inputs and local review

| Input | Bounded reading | Consequence |
| --- | --- | --- |
| ADR-0014 | an L3 conditional lemma in existing `plan/` and `docs/reports/` lanes may be investigated only after pre-registration, with an alternative/falsifier, non-effects, and rollback | this candidate can be screened, but it cannot introduce a new Lean lane, helper, schema, or contract |
| P012 V1 | the restricted administrative binding is the reference presentation; an evaluation-frame or machine-state presentation may be investigated only as an explicit equivalent presentation | a presentation comparison is permitted; a general continuation is not |
| P012 R1 | a typed reply and receipt are explicit, and resumption requires the matching receipt | the comparison must account for matching, one resumption, and failure exclusion without defining the matching relation itself |
| Plan 187 | any V1 machine presentation needs trace equivalence, local decomposition, ownership/no-copying, success/failure resumption locus, and no hidden communication | these are comparison obligations, not a selected machine semantics |
| Plan 199 / Plan 200 | C3 proper remains blocked by pending unit, correlation, held `Delta`, failure/resume, and persistence choices | the candidate must stop before any of those choices |
| Plan 193 / WRK-0026 | adversarial information-loss inventory already covers copy/replay, same-locus, leave/rejoin, and save/load stress; WRK-0026 is frozen on a command falsifier | a new M1 information-loss package would duplicate or incorrectly repair frozen evidence |

Local source queries found no retained working record for a V1/R1
administrative-binding versus one-slot machine-state comparison. The result is
only a non-duplication observation for this exact question; it is not a claim
that no related evidence exists elsewhere.

An Oracle temporary review was used as advisory challenge input. Its first
session reached the documented one-hour zombie error without a model answer.
One smaller, non-duplicative retry completed. The usable advice agreed that a
narrow V1/R1 presentation comparison is the safest remaining candidate, while
M1 information-loss and SW1 interleaving should not be re-opened here. This
Plan records the distilled conclusion only; the external transcript is not
repository state or authority.

## Candidate disposition

| Candidate | Result | Reason |
| --- | --- | --- |
| C3-VR-PRE: V1/R1 presentation refinement | selected for L3 pre-registration | compares two explicitly bounded presentations and can fail without assigning a semantic carrier to Mir |
| M1 information-loss replay family | not selected | Plan 193 already inventories the relevant adversarial classes; re-running frozen WRK-0026 is prohibited |
| SW1 stale interleaving family | not selected | it would choose a validation/mutation boundary and served-write semantics, which is C4 proper |
| C3 proper, C4, C5, C0-D, C1, C2-B, C6, C7 | deferred | each needs an unresolved semantic choice, a new non-duplicate screen, or both |

This is a priority and standing-eligibility preflight only. It does not make
C3 a dependency of C4/C5, or make the selected local presentation a shared
model.

## Proposed C3-VR-PRE pre-registration

### Narrow question

Under an explicitly finite LAB model, can the following two presentations have
the same visible classification under the stated assumptions?

1. an administrative binding that is waiting, resumes successfully once after a
   matching reply, or terminates with failure; and
2. a one-slot machine presentation that is waiting, consumes the one matching
   reply exactly once to resume, or terminates with failure.

The model may use opaque labels such as `LAB$Correlation`; they do **not**
denote a Mir request, receipt, attempt, occurrence, queue item, identity, or
wire token. Its explicit assumptions are: one waiting slot; matching reply
only; single consumption; success and terminal failure are disjoint; no
save/load, authority, redaction, or multi-slot behavior. The model must expose
the facts and basis needed to reconstruct the comparison. Thus it tests a
necessary precondition for future ergonomic omission: a fact may be inferred
only when the elaborated form reconstructs the fact and the unique basis.

### Alternative and expected falsifiers

The alternative is that one of the listed assumptions is weakened. The finite
model must exhibit a differing outcome for at least one of: a swapped reply, a
duplicate reply, or failure followed by an attempted success. Those
counterexamples demonstrate that an unrecorded matching/single-use/failure
fact cannot be silently inferred.

Freeze or decline registration if an identical current-cut presentation result
is discovered; the model needs a Mir carrier, Core grammar/rule, source
elaboration correspondence, pending queue, request/attempt/occurrence identity,
result payload, failure-family design, save/load schema, authority/redaction
policy, new helper/validator/CI/Make surface, or a new evidence lane; or a
counterexample cannot be described without selecting one of those. A passed
finite check is not enough to generalize beyond the stated assumptions.

### Non-effects and rollback

The permitted result is a conditional lemma or finite counterexample record in
ordinary Markdown. It cannot define or choose V1/R1's final syntax, type,
correlation, identity, payload, continuation, evaluation context, pending
carrier, failure semantics, persistence, history, transport, security, API,
Core, SCN, OBL, Gate, Phase, proof status, runtime, or public contract.

On a falsifier, mark the working record `Reliance status: frozen`, retain only
the reproducible procedure evidence, and do not repair or rerun it. A changed
Canon/LAB cut or an actual design need is a forward successor or ordinary Canon
proposal, never a reinterpretation of the finite model.

## Evidence route and execution order

1. Commit this selection and synchronize the LAB snapshots.
2. Create and push `WRK-0033` with the standing predicate, one frozen cut,
   exact source/LAB digests, alternative, falsifiers, non-effects, rollback,
   and registered commands. No model source is written or run first.
3. After the registration is pushed, retain the exact finite Lean source as a
   fenced block inside `plan/wrk-0033-v1r1-presentation-refinement.md`.
   Registered commands may materialize that block into a unique disposable
   `/tmp` file for `lean`; no repository `.lean` file, Lake manifest, helper,
   sample, or new lane is created.
4. Run the registered searches, extraction, Lean check, and diff/doc checks;
   retain the exact result and any countermodel in the existing `plan/` lane.
5. Re-open C3 proper only if a later ordinary Canon design selects the missing
   correlation, pending-control, failure, persistence, and source-elaboration
   boundaries.

## Execution outcome

The selected route was committed at `ddabd97bb3e13df51ede3ba00ead626600e1011a`,
pre-registered as WRK-0033 at `32e7d9a8e7ec4db526812bec650e54d766b0abc6`,
and retained at `37d2fd00a01aa5cf302f0293f0b6be51a337b217`; its Canon metadata
link is `0cccb94373284b7659659ba203ab78a0af1c8072`. The fenced 133-line Lean
source passed `lean --trust=0` and retains only the finite observation equality
under matching/single-use/failure-exclusion assumptions plus the three
registered adverse distinctions. It neither supplies a Mir correlation or
pending carrier nor authorizes source omission, full trace equivalence, C3
proper, or implementation.

## Non-claims

This selection does not claim a completed formalization, a valid C3 design, a
Mir implementation, inference/desugaring approval, source-level convenience,
compatibility, proof discharge, or an official T0/T1/T2/I1 movement. In
particular, it does not make an opaque LAB correlation label a user-visible
language feature.
