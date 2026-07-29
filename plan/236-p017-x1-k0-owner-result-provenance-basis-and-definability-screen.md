# Plan 236: P017 X1 K0 Owner-Result Provenance Basis and Definability Screen

## Role and authority

This LAB ordinary source-conformance and definability card follows Plan 235.
Canon remains normative. It compares only whether owner-side result provenance
has a minimal candidate-native positive basis before a complete X1 candidate
selects an exchange model. It does not adopt a basis, alter any fact status, or
amend P012, P013, or P017.

This card selects no relation schema, predicate signature, field, record,
tuple, result/provenance object, ground identity, carrier, occurrence,
transition, causal generator, validation or computation algorithm,
authority rule, persistence placement, restore function, receipt mechanism,
source/runtime/API surface, theorem/OBL, scenario, Gate, Phase, sample,
implementation, or public behavior.

## Non-duplication and direct constraints

Plans 208--210 already retain the full cross-locus alpha-trace obligations:
`ResultOf(reply-role, requester-visible-result, result-provenance)` remains
coupled there to request/reply/receipt/pending correlation and save/load
frontiers. Plan 220 separately records that a selected relation-state design
must distinguish a result/provenance record from any observer-facing redacted
projection. This card does not reopen those relation signatures, select a reply
role, or restate their complete matrix.

The narrower new question follows from Plan 235: if the same native
`owner-terminal-success(q, v : tau)` membership is explicitly declared to have
the P017 typed owner-result role, can the **owner-result provenance** part of
P017 item 3 be compared at the K0 owner side without covertly selecting the
reply/receipt carrier that Plans 208--210 leave open?

The direct constraints are:

- P012 V1 requires a later typed correlation/result/failure carrier and
  cut/save/load account. R1 keeps owner result distinct from requester receipt
  and requires result provenance, correlation, redaction, and failure
  accounting.
- P013 M1 supplies request-associated, non-authoritative **validation-input**
  claims and references to grounds actually consulted. It does not identify
  result-producing grounds.
- P017 X1 item 3 requires typed owner-result and owner-service-failure facts
  with result and validation provenance, while leaving residence arity,
  transitions, carrier, and persistence realization open. Its causality and
  save/load rows remain later integration obligations.
- theory/04 permits only its stated causal generators and requires complete
  cut/channel closure for facts relied on after load. theory/05 keeps claims
  and provenance references distinct from authority.
- Plan 233 has four B fact roles, none named result provenance; all eight of
  its cells remain `OPEN`. Plan 234 is advisory only, and Plan 235 makes an
  owner-result role conditional on an explicit candidate-local declaration.

`q` remains an existing request occurrence in one current or admissibly
restored history. It is not a source spelling, transport/session key, public
token, or global equality across independent loads. `g` below is metanotation
for an abstract ground, not a new object, identifier, occurrence, or fixed
ground domain.

## Status quo and local adequacy premise

Let `S(q, v : tau)` denote a candidate-native typed terminal-success
membership which a later candidate explicitly declares to be the typed
owner-result fact as permitted by Plan 235. That role declaration supplies no
result provenance. No current Canon clause defines the subject, ground domain,
adequacy, completeness, causal meaning, or persistence of result provenance.

The following **RP-min** is therefore a LAB-local comparison premise, not a
Canon reading, model hypothesis, or adopted `H_K`:

```text
For S(q, v : tau), result provenance is a q-scoped, ground-sensitive
association with abstract semantic grounds that the owner-side result judgment
declares as contributing to production of v.

It is role-distinct from M1 validation provenance. It neither asserts
authority or validation success nor entails causal precedence, persistence,
receipt matching, or observability. It is not exhausted by q, v : tau,
owner terminality, or static typing.
```

Ground-sensitive means only that two otherwise matching admissible
interpretations whose result-producing grounds differ must be distinguishable
by the provenance association. It fixes no cardinality, ground kind, selected
service computation, or requirement that provenance be non-empty. If this
minimum adequacy premise cannot be retained even as LAB comparison vocabulary,
the only valid disposition is `Canon gap / OPEN`; a vacuous reading such as
"provenance is q" is not a meaningful A/B comparison.

## Candidate A: direct positive provenance incidence

Candidate A adds only this candidate-local positive basis, separately for each
retained K0 presentation:

```text
A-Sigma: the q-scoped whole relation contains a positive provenance incidence
         from S(q, v : tau) to abstract ground g.

B-Pi:    a native q-anchored relation contains a positive provenance incidence
         from S(q, v : tau) to abstract ground g.
```

The incidence is a non-exhaustive semantic role, not a link object, result
record, ground collection, field, relation signature, or causal edge. It may
not give `g` an identity, choose its source classes, or state when the
association is installed. Its only allowed immutability reading is extensional:
within the same candidate semantic account, a recorded owner-result fact is
not retargeted to different grounds. This does not assert durability, storage,
or cross-load equality.

Under RP-min, A is conditionally compatible as a possible positive primitive
basis. The exact immediate consumer is P017 item 3 and P012 R1's
result-provenance accounting for the owner result. The deferred consumer is a
later owner-success/receipt-pending L review, which may list the incidence as a
closure input. A supplies neither that closure nor any restore correspondence.

## Candidate B: erasable static derivation

Candidate B tries to define provenance only from `S(q, v : tau)` and the facts
already permitted at this cut:

```text
Fstatic = q scope, typed owner-result role, intrinsic terminality,
          and static compatibility/non-sharing constraints

ProvB = D(S(q, v : tau), Fstatic)
```

`D` must be erasable, non-circular, and introduce no positive ground-bearing
membership, subject, choice, identity, package, history lookup, dynamic
service rule, validation operation, causal reachability, persistence premise,
or receipt fact.

Under RP-min, B has a decisive definability countermodel. Consider two
admissible interpretations `M0` and `M1` that agree on `q`, `S(q, v : tau)`,
the returned value/type, loci, payload/spans, M1 claims, validation-provenance
references, and all `Fstatic`, but differ in their abstract result-producing
grounds. The permitted static input to `D` is identical in both
interpretations, so `D` returns the same view. It cannot be ground-sensitive
in both. Thus B does not uniquely derive result provenance.

Treating `S` as secretly carrying grounds merely renames A or introduces a
hidden carrier. Obtaining grounds from owner state, service evaluation,
validated M1 facts, history, causal predecessors, or restored state selects a
forbidden dynamic surface. B is therefore a countermodel result, not a
positive basis.

## Candidate C and the operative disposition

Candidate C adds neither A nor a valid derivation. Result provenance remains
`OPEN` for the current repository state. C is not a claim that provenance is
absent; it is the only state that does not rely on an unadopted candidate
assumption.

| Alternative | Result under RP-min | Exact falsifier or stop |
| --- | --- | --- |
| A | conditionally compatible candidate-native positive basis | its consumer requires a selected occurrence, transition, carrier, identity, schema, algorithm, persistence realization, or receipt mechanism |
| B | countermodel; no erasable ground-sensitive derivation | `M0` and `M1` agree on all permitted input but have different result grounds |
| C | operative `OPEN` | only an adopted, no-smuggling positive basis or genuine unique derivation can displace it |

No Plan 233 cell changes: it has no result-provenance cell, and Plan 236 does
not adopt A as `H_K`. A later candidate may rely on A only after its own
ADR-0014 standing-eligibility pre-registration and the complete P017
minimum-model process. A standalone L3 experiment for this one incidence is
premature because any reliance immediately couples B to C and L coordinates.

## No-smuggling screen

| Forbidden surface | A stops when... | B stops when... |
| --- | --- | --- |
| occurrence or transition | the incidence, result recording, or ground is made an occurrence, or its creation/freeze is specified | a producing event, terminal transition, reachability, or absence of failure is searched |
| causality | incidence is read as a theory/04 predecessor edge | the view traverses causal history or assumes a new generator |
| carrier or schema | it packages grounds in a record/object/list, fixes arity, roles, fields, or cardinality | it relies on a hidden field, fiber, factorization, or closed-world inventory |
| identity or choice | it creates result/service/link/ground identity or treats q as cross-load global identity | it selects among grounds by equality, adjacency, queue position, span, payload, or implicit choice |
| result-production semantics | it fixes an owner-state snapshot, computation derivation, service act, or deterministic rule | it calculates provenance from any such selected semantics |
| validation or authority | it identifies result grounds with M1 consultation, current M/G/W, or successful authorization | it uses claims, live authority, or validation outcome to infer production grounds |
| persistence | immutability becomes a Config/SaveObject field, serialization, durability, or restore equality | it reconstructs after load from current state/authority or assumes q equality across loads |
| receipt/correlation | delivery, receipt, acceptance, or resume confirms/identifies grounds | receipt or raw delivery is used as a selection witness or q completes R1 correlation |
| observation | provenance storage exposes raw result/grounds | the derived view is exported without P017's separate theory/07 gate |

M1 validation provenance and result provenance might later share a semantic
ground, but neither overlap nor disjointness follows here. Neither is
authority, and neither becomes observable merely by residence or storage.

## Process boundary, stop, and next work

This is ordinary LAB card work because it only compares an unadopted positive
basis, a definability countermodel, and `OPEN` under an explicit local premise.
It is not a proof, model, implementation, or source proposal.

Stop this card and enter the ordinary Canon process if making A meaningful
requires any listed forbidden surface. Stop at `Canon gap / OPEN` if RP-min
cannot be used even as a labelled LAB adequacy premise. Do not create a
provenance field or helper merely to avoid either stop.

The next autonomous boundary is another independent Plan 233 B fact role, not
an attempt to promote A or to fill the reply/receipt/save/load gap. The next
card must first perform the same non-duplication and source-consumer screen.

## Non-effects

This plan changes no Canon text or status, `working/` record, Plan 233 status,
relation model, carrier, identity, branch/failure/validation/mutation rule,
transition, occurrence/causal rule, observer surface, Config/SaveObject
placement, restore behavior, source grammar, elaboration, runtime, adapter,
wire/API, theorem/OBL, scenario, Gate, Phase, sample, implementation, or
public behavior.
