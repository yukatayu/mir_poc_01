# Plan 237: P017 X1 K0 Owner-Outstanding Positive-Basis and Pending-Nonconflation Card

## Role and authority

This LAB ordinary-design comparison follows Plans 233--236. It compares exactly
one Plan 233 fact role, owner outstanding, for one K0 V1/R1 cross-locus read.
It retains A-Sigma and B-Pi as separate presentations. Canon remains
normative. This card neither introduces nor accepts an outstanding fact,
amends P017, nor changes Plan 233's all-`OPEN` ledger.

The question is only the smallest candidate-native positive basis by which a
future ordinary candidate could classify this one fact-status cell. It is not
a lifecycle, branch, request/response, or pending-control design. A later
candidate must state and accept its own `H_K` before Plan 233 can be rerun with
a non-`OPEN` outcome.

This card selects no relation arity, carrier, field, record, tuple, relation
family or role index, identity/equality rule, owner-result/failure/receipt
object, failure member/row, validation or mutation algorithm, transition,
lifecycle, occurrence, causal generator, `Config`/`SaveObject` placement,
restore function, source/elaboration form, runtime, theorem/OBL, conformance,
Gate, Phase, sample, implementation, wire/API, or public behavior.

## Source cut and exact consumer

P017 X1 item 2 requires an explicit service-outcome state with an outstanding
disposition and at most one terminal owner success or typed owner-service
failure. Outstanding need not terminate; owner success is terminal only for
owner service. P012 V1/R1 separately requires requester pending control and a
matching receipt before requester resumption. P013 M1 is request-local
validation-input provenance, not service status. theory/04 supplies no
closed-world ``no terminal occurrence therefore outstanding'' rule, and
theory/05 forbids treating claims or authority facts as a service state.

The immediate consumers are only these two Plan 233 cells:

```text
A-Sigma / owner outstanding
B-Pi    / owner outstanding
```

The sole normative requirement consumer is P017 item 2's outstanding
disposition. The card deliberately does not discharge item 2's terminal
exclusivity, at-most-one outcome, no-mutation, or later load obligations.
Plans 208--210 and 220 retain the full request/result/receipt/pending,
provenance, causality, linearity, observation, and save/load audit. They are
not immediate consumers here.

## Pending nonconflation

`PendingFor` names a requester-side administrative binding. It can exist while
owner service is outstanding, after owner success before semantic receipt, or
on an owner-failure branch. It therefore cannot be a positive or derived basis
for owner outstanding. The same exclusion applies to held `Gamma`/`Delta`,
absence of requester receipt/resumption, raw delivery, queue/in-flight state,
transport/session metadata, source span, payload equality, and elapsed time.

Likewise, M1 claims, their provenance references, current membership,
capability, witness, admission, visibility, history, principal, role, locus,
and endpoint are not service-outstanding facts. A request occurrence `q`, a
request-to-service causal edge, or the absence of a terminal fact in the
current prefix also does not establish outstanding status. These exclusions do
not deny any future fact; they keep this K0 source screen from making a hidden
lifecycle or closed-world assumption.

## Candidate A: direct positive owner-outstanding membership

### A-Sigma

Candidate A uses only a positive non-exhaustive native view of the whole
`q`-scoped restriction:

```text
Sigma_q positively contains owner-outstanding(q)
```

The role is intrinsic to that one membership. It states neither introduction
nor termination, incompatibility with a terminal fact, eventual service,
no-owner-mutation, receipt behavior, nor a transition path. It is invalid if
it needs a tag, field, tuple/record position, constructor, fixed role, or
complete view inventory.

### B-Pi

Candidate A independently uses one positive native `q`-anchored membership:

```text
Pi positively contains, at q, owner-outstanding(q)
```

It names no carrying-family identifier, role index, shared key, common record,
membership witness, factor role, or packaged branch object. Static coherence
may not manufacture the fact's residence.

### Conditional classification

If a future ordinary candidate explicitly adopts the corresponding statement
as `H_K`, Plan 233 classifies that candidate's owner-outstanding cell as
`primitive`. This is a conditional ledger result only. Neither membership is
currently adopted by this card or by P017.

## Candidate B: exact erasable owner-service-pending view

Candidate B is legal only if an independently motivated positive
owner-service-pending fact already exists. It is expressly not requester
`PendingFor`.

```text
positive owner-service-pending fact at q
+ q-local constitutive clause making owner-outstanding an exact erasable view
```

For A-Sigma, the source must already be a native whole-slice membership. For
B-Pi, it must already be a native `q`-anchored membership; coherence may
constrain compatibility but cannot supply semantic residence. The source fact
must have an independent consumer, not be introduced merely as a synonym for
the target. The clause must be local, positive, non-circular, choice-free, and
independent of a terminal complement or lifecycle reachability.

If such premises are adopted and the uniqueness test below passes, Plan 233
would classify the affected cell as `uniquely derived`. No independently
primitive owner-service-pending fact is present at this cut, so B is only a
comparison form, not a present derivation.

## Candidate C: retain OPEN

Candidate C adds no positive basis. Both owner-outstanding cells remain
`OPEN`. It is the operative repository result until a later ordinary candidate
adopts and passes A or B. `OPEN` does not assert that owner service is not
outstanding.

## Uniqueness tests, falsifiers, and stop line

For B, hold fixed every permitted positive owner-service-pending membership
and static constraint. If two admissible interpretations still disagree on
owner outstanding, the derivation is not unique. B also fails if its source is
requester pending, is introduced solely to rename the target, uses terminal
absence, invokes lifecycle/reachability/eventuality, turns B-Pi coherence into
residence, or needs a family index/common witness/branch package.

Candidate A fails when its positive fact requires any excluded source from the
pending-nonconflation section, terminal absence/exhaustiveness, a lifecycle
enum/state-machine node/transition, a relation schema or complete role
inventory, a selected occurrence or causal predecessor, a failure row,
validation/mutation enforcement, persistence realization, receipt/acceptance/
resumption/one-shot behavior, result/provenance fact, observation surface,
runtime/API, theorem/OBL, or public behavior. Stop the affected presentation
and retain `OPEN` rather than repairing it with hidden structure.

## Comparative result and classification

| Alternative | Positive assumptions for this one cell | Result if later adopted |
| --- | --- | --- |
| A | one native owner-outstanding membership | `primitive` |
| B | independently useful owner-service-pending membership plus local exact view | `uniquely derived` |
| C | no positive basis | `OPEN` |

For this local Plan 233 consumer, A is the smallest conditional basis in both
presentations. It directly names the required role and avoids B's additional
pending-service fact and constitutive clause. This **A/A advisory
recommendation is not adoption**. Accordingly, every Plan 233 row, including
the two examined here, remains repository-wide `OPEN`.

This is ordinary LAB source-conformance/design work rather than an L3
experiment. It defines no minimum model, transition system, fixture, theorem,
or runnable behavior; a new `working/` record would encode an unselected
declaration rather than test an independent hypothesis.

## Non-effects

This card changes no Canon text or status, `working/` record, relation model,
schema, carrier, identity, failure row, authority or validation/mutation rule,
transition, lifecycle, occurrence/causal rule, observation surface,
`Config`/`SaveObject` placement, restore behavior, source grammar,
elaboration, runtime, adapter, wire/API, theorem/OBL, scenario, Gate, Phase,
sample, implementation, or public behavior.
