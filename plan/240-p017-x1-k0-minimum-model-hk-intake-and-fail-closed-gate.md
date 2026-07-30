# Plan 240: P017 X1 K0 Minimum-Model H_K Intake and Receipt-Endpoint Reopen Gate

## Role and authority

This LAB ordinary-design intake follows the completed Plan 233 per-cell basis
inventory. Canon remains normative. Its purpose is to determine whether one
complete, reversible `C + H_K + D_K` model can be preregistered for the bounded
P017 X1 K0 scope without adding a reserved surface.

The result is **`INTAKE-REOPEN`**. It is not a rejection of X1, K0, V1, R1, or
M1. Canon alone does not provide a successful requester receipt endpoint, so a
complete candidate cannot be read directly from C. That absence does not itself
exclude an explicit, reversible `H_K` account using an existing Theory 04
receive role. No `working/WRK-*` record, model, relation schema, transition,
occurrence kind, causal generator, failure row, Config/SaveObject placement,
source/runtime/API, theorem/OBL, Gate, Phase, sample, or implementation is
selected by this plan.

## Authority cut and question

DIRECT constraints are theory/01, theory/02, theory/04, theory/05, P012 V1/R1,
P013 M1, P017 X1, and ADR-0014. Plans 227, 230--232, and 233--239 are LAB
design memory. The temporary Oracle consultation
`p017-theory04-causality-intake` was an advisory challenge review; its relevant
finding is restated and source-checked here rather than treated as authority.

The question is deliberately narrow:

> Can one K0 candidate close every P017 R/B/T/U/C/L integration row using only
> existing Canon occurrences and Theory 04 generators, while keeping no required
> row `OPEN` and selecting no reserved surface?

The scope is one V1/R1 cross-locus read anchored by a request occurrence `q` in
one current or admissibly restored history. K0's external rejected-delivery
treatment remains only a candidate-local comparison direction: a raw rejected
delivery is outside the semantic exchange and changes no semantic receipt,
owner-result availability, accepted-use budget, failure state, occurrence, or
restore frontier. It is not adopted as Canon semantics.

## Admission rule for a complete intake

`INTAKE-COMPLETE` would require exactly one active candidate presentation and
an explicit C/H_K/D_K/OPEN/Canon-gap classification for every fact, bridge,
order, and restore claim. In particular it requires:

1. a semantic residence and exactly-one non-shared requester pending binding;
2. positive bases for owner outstanding, typed terminal success/failure, typed
   result, consulted-validation provenance, and result provenance;
3. a semantic receipt and restricted-use account distinct from owner service;
4. an exact existing Theory 04 generator for every relied-on order;
5. a live-fact and causal-or-channel closure account for load; and
6. a decisive falsifier for each claimed row, with no hidden schema, identity,
   authority, lifecycle, transition, or observation surface.

If any required row remains `OPEN` or reaches a reserved surface, this intake
must stop. It may not add a carrier, event, field, transition, or causal edge
after observing the gap.

## What the source already fixes

| Item | Classification | Exact scope |
| --- | --- | --- |
| `q` is an existing request occurrence emitted by `[E-REQ]` | C | theory/01 |
| a successful `[E-SERVE]` appends a served occurrence `s` with causal order `q prec s` | C | theory/01 |
| a failed `[E-SERVE]` appends an explicit row-contained failure occurrence `f` with no store mutation | C | theory/01, theory/02 |
| causal order is the transitive closure of Theory 04's fixed generating family | C | theory/04 |
| claims and recorded provenance are not authority | C | P013, P017, theory/05 |
| owner result, requester receipt, receipt acceptance, and restricted use are distinct | C | P012 R1, P017 X1 |
| every claimed order must name an existing Theory 04 generator or identify a future amendment | C | P017 X1 |

Plans 234--239 identify conditional positive-basis forms for the owner facts,
but none is adopted here. Their basis inventory therefore does not supply an
active B coordinate by itself.

## Candidate-local causal reading and its limit

For the successful owner branch only, a future candidate could use the
candidate-local hypothesis below:

```text
H_K-sr: the existing pair (q, s) instantiates Theory 04's send -> receive
        generator for this bounded X1 request/service interaction.
```

The labels `request-send(q)` and `service-receive(s)` would be erasable `D_K`
role names only; the assertion that the pair instantiates a particular
generator is load-bearing `H_K`, not C. `q prec s` alone cannot identify the
generator because Theory 04 defines order as transitive closure, and the
enqueue/dequeue state path is also a possible interpretation. The candidate
must not claim that Canon already types every request occurrence as a send or
every served occurrence as a receive.

This limited hypothesis also leaves the owner-failure branch, lineage-to-service
roles, result/reply-to-receipt, acceptance-to-use, and use-to-dependent-occurrence
mappings unresolved. It cannot close coordinate C by itself.

The following separate candidate route remains to be screened, not adopted:

```text
H_K-rs: s has a reply-send projection; a distinct requester occurrence r has
        a generic receive projection; s -> r instantiates Theory 04 send ->
        receive; semantic receipt is associated with r.
```

`H_K-rs` does not make `r`, the projection, or its typed receipt meaning C. It
must preserve existing occurrence kinds and the zero-or-one rule per step, keep
the reply projection co-located with `s`, and introduce no internal order in
`s`. `r` is a distinct later candidate occurrence instance, not an implicit
subevent. The route is invalid if it needs a new receipt constructor, Core primitive,
history schema, operational rule, or binding reinterpretation of OPEN-011.

## C-level receipt boundary and candidate reopen

P017 requires a requester-side **semantic receipt transition distinct from
owner service**. Its causality row requires a selected owner-result or
reply/send-side fact to be mapped to semantic receipt through an existing
Theory 04 generator. P012 R1 likewise directs a later package to model owner
service result and requester receipt as separate typed causal steps.

The current Canon source supplies no successful requester receipt occurrence:

- `[E-REQ]` supplies the request occurrence `q`.
- successful `[E-SERVE]` supplies the owner-side served occurrence `s` and says
  only `read+reply`; it does not define a requester receipt occurrence or its
  carrier.
- failed service's requester receive remains OPEN-010; it is not a successful
  R1 receipt model.
- OPEN-011 leaves the exact successful read reply/receipt carrier unresolved.

Consequently, the required owner-result/reply-to-semantic-receipt mapping has
no current pair of **C-level** Canon endpoints. The following shortcuts fail:

| Attempt | Why it does not close the row |
| --- | --- |
| treat owner-side `s` as the requester receipt | violates P012/P017's required service/receipt distinction and introduces no cross-locus endpoint |
| make semantic receipt zero-occurrence administrative state | it cannot be the receive endpoint of Theory 04 `send -> receive`; predicate implication is not a causal generator |
| introduce an implicit fresh reply/receipt subevent in `[E-SERVE]` | violates the per-step zero-or-one discipline and hides an occurrence/accounting choice |
| promote a new `ReadReceipt` constructor, history schema, or operational rule | selects a reserved Canon surface |

Neither relation membership, a `q` anchor, result provenance, acceptance, nor
restore correspondence can replace a C-level causal endpoint. However, the
absence of C does not prove that a future candidate must add a new occurrence
kind. A distinct later `r` may be screened as a candidate-local instance of
Theory 04's existing receive role, with `s` only a candidate-local reply-send
projection. This is not a hidden same-step subevent: it is a separate proposed
zero-or-one-occurrence step whose status, matching, typing, and causal closure
must be frozen and falsified before any reliance.

The required screen asks whether `r` can remain that generic `H_K` occurrence
account with its typed semantic receipt fact in X1 relation state. It stops for
ordinary Canon process only if the account necessarily needs a new constructor,
primitive transition/rule, history schema, or binding L0/L1 interpretation.

## A-Sigma and B-Pi are not used as a shortcut

This stop does **not** conclude that an A-Sigma relation fiber or B-Pi
factorization is impossible. It intentionally does not choose either
presentation, a relation signature, or a predicate inventory. A later ordinary
design package may expose candidate-local facts only when it classifies every
one as a primitive hypothesis or a genuinely erasable definition and does not
turn a fiber, pending binding, result, provenance bundle, or restore slice into
a hidden exchange identity, field layout, tagged lifecycle, or persistence key.

The causal receipt boundary is independent of that presentation question. Even
a fully explicit relation cannot make a non-occurrence into a Theory 04 receive
endpoint without a separately stated occurrence account.

## Disposition

```text
INTAKE-REOPEN

No complete P017 X1 K0 H_K tuple is frozen by this intake. C alone lacks the
successful requester receipt endpoint, but H_K-rs is a source-permitted
candidate route that must be screened before declaring an owner/Canon stop.
No owner-fact basis, relation presentation, or one-shot/restore hypothesis is
adopted to paper over the remaining rows.
```

This is a positive result about the boundary of the current cut, not a proof
that no future X1 model exists. It neither changes P017's owner disposition nor
claims that a later ordinary Canon amendment must use a particular carrier.

## Next autonomous preparation and owner boundary

The next self-driven task is a narrow `H_K-rs` occurrence-accounting preflight.
It must compare, without deciding:

1. an existing-C endpoint mapping, if one can be literally sourced;
2. `H_K-rs`: a reply-send projection of `s` and a later generic receive
   occurrence `r` using the existing `send -> receive` generator; and
3. deferral if neither account survives the no-new-primitive screen.

The preflight must separately account for owner failure, no-mutation, row
containment, M1 authority grounds, redaction, cut/channel closure, matching,
receipt typing, and the zero-occurrence status of consumption. It must not turn
the owner-side served occurrence into an implicit requester receipt, import
transport/fairness/retry, or choose a result wire format, runtime, schema, or
public API.

Normative selection or amendment of an occurrence/history mapping remains an
owner/Canon boundary. Reversible candidate-local role/instance accounting is
not automatically such a boundary under ADR-0014. No user decision is requested
by this intake alone.

## Non-effects

This plan changes no Canon text/status, working record, Core, Config,
SaveObject, relation schema, identifier, request/result/receipt carrier,
transition, occurrence kind, causal family, failure row, authority rule,
observation surface, source grammar/elaboration, runtime, adapter, wire/API,
theorem/OBL, scenario, Gate, Phase, sample, implementation, or public behavior.
