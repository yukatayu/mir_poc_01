# WRK-0032 C5-PRE - ordinary-admission issuance guard

## Role and evidence boundary

This is a **LAB** result artifact for `working/WRK-0032`. Canon remains
normative. It is a source-local literal matrix over the five spans registered
there, not an admission model, occurrence/history mapping, or proof of
atomicity.

“No distinct marker in the named span” means only that the registered query and
the cited passage do not literally name a distinct ordinary-admission issuance
rule, transition, state, issuance-specific failure, scheduling point, or
independent observation point. It is not a global absence claim, a prohibition,
or proof that a later design cannot expose one. Patch admission is outside this
ordinary-admission corpus.

## Pinned source-local observations

| Source-owned span | Literal source-local observation | Registered interpretation and non-inference |
| --- | --- | --- |
| `meta/proposal-012` owner disposition and Question A | The recorded A2 direction is conditional on verdict-time membership plus a finite named grant/witness set. Its stop line says that separately failing, observable, or schedulable issuance in a later design needs A1 assessment. | This is the **guard-direction match**, not a current ordinary-admission phase. It does not establish that a separate phase exists, choose A1 or A2, or make the condition true. |
| `theory/01` Core/Config, `[JOIN]`, `[E-ADMIT]`, and scheduling | Core lists `admitreq` and `verdict`; Config names `M`, `G`, and `W`; `[E-ADMIT]` says one admission-verdict rule updates M and issues grants/witnesses. The scheduling paragraph is generic over enabled steps and owner queues. | The named `[E-ADMIT]` wording does **not** literally name a distinct issuance rule, transition, state, issuance-specific failure, scheduling point, or independent observation. General `grant`/`witness` terms and generic scheduling are not linked here as a separately observable ordinary-admission phase. No atomicity or occurrence identity follows. |
| `theory/04` causal family and consistent-cut consequences | The family lists `admit_request -> verdict -> activation_cut`, `capability_grant -> capability_use`, `witness_create -> witness_use`, and `membership_update -> dependent dispatch`. | The named edges establish no issuer phase or mapping from the admission verdict to separate grant/witness/membership occurrences. They do not make distinct causal vocabulary a distinct issuance phase. |
| `theory/05` admission carriers, lineage, and lifecycle | `AdmissionVerdict` includes epoch/incarnation, granted capabilities, an admission witness reference, and failure/reason; post-admission messages and capability use require lineage facts. | This names authority facts and later use validation, but the named span has no distinct ordinary-admission issuance transition/state, issuance-specific failure, scheduler, or observation. It does not choose field or occurrence identity. |
| `spec/05` request lifecycle, membership, observation, and conformance schedule | The request lifecycle is generic emitted/enqueued/validated/served-or-failed behavior; membership and observation have separate bullets; the deterministic profile schedules owner request service. Patch verdict visibility is expressly a different bullet. | The named span has no literal ordinary-admission issuance phase. Generic request failure, FIFO service, observation export, and patch pipeline text are not evidence for a separately failing, schedulable, or observable ordinary-admission issuance phase. |

The registered query therefore has one direct conditional guard direction in
P012 and no separate-phase marker in each of the four named ordinary-admission
theory/spec spans. This is a span-local reading only.

## Retained result

At the WRK-0032 authority cut, P012 remains the sole registered source in this
matrix that expressly states the conditional-A2 stop rule. The current
ordinary-admission source spans examined here describe admission and its
membership/grant/witness consequences, but do not literally expose the distinct
issuance phase that would activate that conditional guard.

This does **not** establish that `[E-ADMIT]` is atomic, that its effects are one
occurrence, that A2 is compatible or adopted as current theory, or that a
future implementation/design lacks a distinct phase. It supplies no reason to
skip the required ordinary Canon/A1-successor assessment if a later design
introduces an independently failing, observable, or schedulable issuance
phase.

## Falsifier audit

The registered absence marker passed before this artifact was created. All
seven registered Canon inputs were nonempty and their SHA-256 values matched
the WRK-0032 authority cut. The literal query returned the admission, verdict,
membership, grant, witness, failure, queue, scheduling, and observation wording
needed to classify only the named spans. `git diff --check` passed before this
artifact was written.

No command output required a same/different occurrence decision, cardinality,
operational trace, patch-admission classification, source-wide absence claim,
or reserved-surface change. Any such use freezes this record under its
registered rollback route. A later Canon cut requires a successor rather than
revision of this artifact.

## Consequences and non-effects

The result makes the next design boundary more precise: an A1 successor is not
selected by current source wording alone, but a design that makes issuance
separately fail, observable, or schedulable must stop and propose that ordinary
Canon assessment explicitly. C3 pending control and C4 served-write identity
remain independent design boundaries.

No fact is eligible for ergonomic inference from this matrix. In particular,
an omitted admission-phase fact is neither uniquely semantically determined nor
reconstructible from an elaborated artifact. Inference can be considered only
after those two conditions are separately established by a future design and
its evidence.

No Core grammar/judgment, authority or membership rule, history carrier,
failure behavior, scheduler, OBL/theory status, SCN, Gate, Phase, parser,
checker, runtime, wire, serialization, API, public behavior, or implementation
readiness changes.
