# Plan 238: P017 X1 K0 Typed Terminal Owner-Service Failure Positive-Basis and Failure-Nonconflation Card

## Role and authority

This LAB ordinary-design card follows Plans 233--237. It compares only Plan
233's typed-terminal-owner-service-failure cells for one K0 V1/R1 cross-locus
read, separately for retained A-Sigma and B-Pi. Canon remains normative. It
does not introduce or adopt a failure fact, alter P017, or change the
repository-wide all-`OPEN` ledger.

It selects no relation schema, carrier, field, record, role index, identity,
failure member/row, lifecycle, transition, occurrence, causal generator,
validation/mutation algorithm, `Config`/`SaveObject` placement, restore rule,
receipt/use mechanism, source/elaboration form, runtime, OBL, Gate, Phase,
sample, API, or public behavior.

## Source cut and immediate consumer

P017 item 2 requires at most one terminal owner success or typed owner-service
failure, with outstanding not required to terminate. Item 3 separately requires
a typed owner-service-failure fact. theory/02 permits abstract dynamic-failure
typing and row containment without selecting a member or row. P012 V1/R1 and
Plans 208--210/220 retain later failure continuation, `FailureFor`, branch,
correlation, linearity, causality, observation, and load work.

The sole status consumers are:

```text
Plan 233 / A-Sigma / typed terminal failure
Plan 233 / B-Pi    / typed terminal failure
```

The card does not discharge terminal exclusivity, at-most-one outcome,
no-owner-mutation, validation provenance, requester failure receipt, or load.

## Failure nonconflation

Typed owner-service failure is not static Diagnostic, malformed/underdeclared
row, owner validation itself, M1 claim/provenance, current authority state,
receipt rejection, no delivery, raw adapter rejection, queue loss, terminal
absence, or a requester-side `FailureFor` relation. It is not established by a
DAG path, a request occurrence, a causal predecessor, or a terminal-success/
owner-outstanding advisory basis. A membership also does not prove no owner
mutation or validation failure. These exclusions preserve theory/02's
Diagnostic/dynamic boundary, P013's non-authoritative M1 boundary, theory/04's
closed-world prohibition, and P017's owner/requester separation.

## Candidate A: direct typed terminal-failure membership

For A-Sigma, Candidate A is a non-exhaustive native whole-slice view:

```text
Sigma_q positively contains typed-terminal-owner-service-failure(q)
```

For B-Pi, it is independently a native q-anchored view:

```text
Pi positively contains, at q, typed-terminal-owner-service-failure(q)
```

Typing, abstract row containment, owner-service role, and terminality are
intrinsic to that one fact. It supplies neither branch exclusivity,
no-mutation, provenance, receipt behavior, persistence, nor a transition.
A-Sigma remains non-exhaustive; B-Pi requires no carrying family, role index,
branch/failure object, shared key, witness, factor role, or coherence-as-
residence. If a future ordinary candidate adopts it as `H_K`, its cell is
conditionally `primitive`. No adoption occurs here.

## Candidate B: typed failure plus erasable terminality

Candidate B requires an independently useful positive typed owner-service-
failure view plus one q-local constitutive clause:

```text
positive typed owner-service-failure view at q
+ each such view denotes terminal completion of owner service for q
```

The source has an independent consumer in P017 item 3 and later P012 V1 typed
failure accounting; it is not introduced only to rename the target. The clause
is static, local, non-circular, erasable, and independent of success,
outstanding, complement, lifecycle/reachability, occurrence, transition,
receipt absence, delivery, and concrete row/member selection. B-Pi coherence
may constrain an existing positive membership but cannot create it, pair it
through a witness, or supply terminality. Adopted premises passing the
uniqueness test would be conditionally `uniquely derived`; none is adopted.

## Candidate C, tests, and result

Candidate C adds no basis; both cells remain `OPEN`. For B, hold fixed every
permitted positive typed-failure membership and local clause. If two admissible
interpretations still disagree about terminal owner-service failure, B is not
unique. Retain `OPEN` if A needs a concrete row/member, lifecycle inventory,
complement, validation/mutation rule, occurrence/causal rule, failure receipt,
persistence, provenance, or shared B-Pi package; or if B lacks its independent
item-3 consumer.

| Alternative | Positive assumptions | Conditional status |
| --- | --- | --- |
| A | one native typed terminal owner-service-failure view | `primitive` |
| B | independently useful typed failure view plus local terminality | `uniquely derived` |
| C | none | `OPEN` |

A is the smallest conditional basis for both presentations. This A/A result is
advisory and **not adoption**: all Plan 233 rows remain `OPEN`.

## Duplicate control, stop, and classification

This is not Plan 234 with a word substitution. Its only failure-specific
content is theory/02 typing/row containment, Diagnostic separation,
owner-service versus requester/adapter failure separation, no-mutation
non-entailment, M1 non-entailment, and B's independent item-3 consumer. Stop
instead of repeating Plans 208--210/220's branch, correlation, linearity,
causality, observation, or save/load matrices, or Plan 236's provenance work.

This is ordinary LAB source-conformance/design work, not L3: it defines no
minimum model, executable semantics, theorem, or fixture. A `working/` record
would encode an unadopted fact declaration.

## Non-effects

This card changes no Canon text/status, `working/` record, schema, carrier,
identity, failure row, authority/validation/mutation rule, transition,
lifecycle, occurrence/causal rule, observation, save/load, grammar,
elaboration, runtime, adapter, wire/API, theorem/OBL, scenario, Gate, Phase,
sample, implementation, or public behavior.
