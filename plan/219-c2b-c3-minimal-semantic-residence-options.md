# Plan 219: C2-B/C3 Minimal Semantic-Residence Options

## Role and authority

This is a **LAB ordinary-design decision-preparation packet**. It narrows the
carrier gap recorded by Plan 218 into explicit alternatives and one LAB
recommendation. It is not a Canon proposal, owner decision, Core amendment,
implementation contract, or authorization to create a `working/WRK-####`
record. `mirrorea_canon/` remains normative.

The recommendation is deliberately about the smallest semantic residence for
one cross-locus, result-returning request. It neither turns `World` or `Game`
into core vocabulary nor introduces a future, session, public wire token,
queue API, general continuation, retry protocol, cancellation, timeout,
fairness, freshness, distributed transaction, or durable exactly-once claim.

## Authority cut and fixed constraints

Review cut: `86c40f952ca74a82f7f339432549a011dd66a2a9`.

| Anchor | Digest / source | Constraint used here |
| --- | --- | --- |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | Core, authority, failure, and contract primitives require owner/Canon action; this packet cannot select them. |
| theory/01 | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` | `request` is an occurrence; `[E-REQ]` and `[E-SERVE]` preserve zero-or-one-occurrence steps, serial owner service, fail-closed validation, and an acyclic history. `OPEN-010`/`OPEN-011` leave the receive/reply carrier open. |
| theory/02 | `40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257` | A result and every dynamic failure remain typed and row-contained; static ambiguity is a Diagnostic. |
| theory/03 | `2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641` | Elaboration cannot add hidden communication, authority, or unrecorded semantic facts. |
| theory/04 | `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` | Successful load restores a complete admissible state and causal prefix; it cannot silently recreate stale facts. |
| theory/05 | `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` | Request claims are checked against authority lineage; transport/session/locus identity is not authority. |
| P012 / P013 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` / `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | V1/R1 permits a restricted one-shot result use and distinct reply/receipt direction; M1 is request-associated. None chooses identity, carrier, encoding, or replay policy. |
| Plans 215--218 | `96255bc1f30c8a3add4a7cb40066958fe4d58454b7782378cc79e76f327e4297` / `00649b49b2e051bca966230c25c4da07f519f8c33b51aa48f03bf1f654da00c8` / `e115fa5c24024de7c641b69fde76b690581ac3a310c482dce24f466d6aa80e5e` / `ac5fd5af5706b94334325c52ec93cc22e6f55a9ae207313552bab7878211e14b` | Candidate comparisons must remain carrier-neutral until an ordinary decision; the first existing-request card has a `CARRIER-GAP`. |

Within a single running or successfully restored configuration, an occurrence
node `q` denotes one dynamic request emission in `H`. This is not an
assumption of a source-visible identifier, a wire identity, or equality of
occurrence nodes across independently restored configurations.

## The decision to make

The missing choice is **where the following facts live semantically and how
they remain linked after admissible load**:

```text
request emission q
  -> owner validation and success/failure outcome
  -> reply availability
  -> requester receipt and acceptance
  -> at-most-one restricted result consumption
```

The owner need not decide a storage encoding, source spelling, or network
protocol now. The owner does need to select one semantic stratum and its
minimal transition account, because a proof relation, evaluator side table,
queue position, session identifier, payload equality, source span, or transport
metadata cannot carry these facts invisibly.

## Compared alternatives

### A. Explicit relation-first exchange state - recommended

Add one semantic configuration component, written `X` only as a placeholder in
this packet. `X` is a relation-valued exchange state, indexed by the existing
request occurrence `q` *within the current history*. It is not a source type,
user-visible object, capability, public protocol token, or fresh nominal
identity. Its rows are allowed to be relation-valued rather than functions.

The ordinary proposal would define enough rows to observe:

| Observation | Minimum intended meaning |
| --- | --- |
| request context | `q` is paired with the complete M1 context used for validation and the separate authoritative grounds actually checked. |
| owner outcome | exactly one selected success or explicit typed failure branch for `q`; failure has no owner-store mutation. |
| reply availability | a success value/provenance or explicit failure is available for a requester-side delivery step, without implying that delivery happened. |
| requester receipt | a requester-side receipt occurrence is causally linked to the selected available branch; receipt availability, acceptance, and consumption are distinct observations. |
| one-shot use | only an accepted success enables the V1 restricted, zero-occurrence pure use; the linked linear disposition makes a second use impossible. |
| restore | `X`, held linear state, and their causal predecessors are in the complete `SaveObject`; post-load reasoning uses a declared restore correspondence, not global occurrence equality. |

The smallest associated operational refinement is a requester-side receipt
step, provisionally called `[E-RECEIVE]`. It appends at most one receipt
occurrence and has a causal predecessor in the owner outcome/reply-availability
row. `[E-SERVE]` remains the owner validation/outcome step. The proposal must
state the exact success and failure occurrence placement and the policy for an
invalid or duplicate receipt: audit-only exclusion, or a declared dynamic
failure contained in the request row. This packet does not choose that policy.

The relation is explicit semantic state: it cannot be reconstructed from
transport metadata or `Q`. A transport implementation may refine a selected
receipt relation later, but cannot become its semantic definition. A raw
duplicate delivery need not be a second semantic receipt; if it has an
observable semantic effect, the chosen policy must say so.

**Why this is recommended.** It uses the already necessary dynamic request
occurrence without adding a second identity sort; it keeps success, failure,
receipt, acceptance, and consumption separately observable; and it gives
save/load an explicit object to preserve. It also avoids assuming that every
link is a function or factors through one hidden token. Its only new semantic
residence is one relation-valued configuration component plus the required
receipt transition, which is smaller than a public attempt object or a general
asynchronous abstraction.

### B. History-only request-occurrence projection - not recommended

Treat `q` and existing causal edges in `H` as the whole carrier. Owner outcome,
reply, receipt, and consumption would have to be derived solely from occurrence
ancestry, existing queue contents, and existing linear contexts; no dedicated
`X` relation is introduced.

This looks smaller, but it cannot currently distinguish reply availability from
receipt, retain a zero-occurrence consumption after load, or give an M1/result
link without adding exactly the missing projections and persistence state.
Queue position, ancestry, and span are explicitly insufficient correlation
facts. Once it adds the required branch and restore facts, it becomes an
unlocated form of A and violates theory/03's no-hidden-carrier boundary.

It is therefore a useful falsifier for accidental over-minimization, not a
viable first proposal at the present Canon cut.

### C. Fresh nominal exchange identity - reserve alternative

Create a new semantic `ExchangeId` at request emission, carry it through
owner outcome and receipt, and persist its lifecycle. It makes correlation
direct, but requires a new identity sort, equality, freshness/non-reuse,
serialization/restore scope, replay story, and a rule separating it from every
transport or session identifier.

This is viable only if later evidence shows that A's relation/restore
correspondence cannot express a required property. No such need is currently
demonstrated. Selecting it now would enlarge the core and make an otherwise
unnecessary identity contract prematurely rigid.

## Comparison and recommendation

| Criterion | A: explicit relation state | B: history-only | C: nominal identity |
| --- | --- | --- | --- |
| semantic residence is explicit | yes, `X` | no at this cut | yes |
| fresh identity sort | no | no | yes |
| reply versus receipt | distinct rows/events | not derivable | direct but larger |
| zero-occurrence consumption after load | explicit linear/state disposition | missing | explicit lifecycle required |
| restore requirement | relation correspondence in complete `SaveObject` | missing | identity equality/reconstruction and non-reuse |
| accidental transport coupling | excluded by the relation's meaning | likely | must be separately prohibited |
| new public/source surface | none required | none, but incomplete | none required initially, yet future pressure is high |

**LAB recommendation:** use A as the basis of a normal Canon proposal, with
the request occurrence as an in-history anchor, an explicit relation-valued
configuration component, a distinct requester receipt step, and restore
correspondence rather than global cross-load occurrence equality. Treat C as a
reserve, and reject B as insufficient unless it is reformulated into an
explicit semantic relation.

This recommendation is not a claim that the selected relation has a final name,
schema, arity, wire representation, or source syntax. It is a minimality claim
relative to the facts that Plans 215--218 show must be represented.

## Proposed owner/Canon decision boundary

An ordinary proposal based on A should ask for only the following coupled
decisions:

1. **Semantic stratum:** add an explicit relation-valued exchange component to
   the abstract configuration and `SaveObject`, anchored by an existing request
   occurrence only within the restored history.
2. **Transition boundary:** introduce a requester receipt transition distinct
   from owner service; success/failure availability, receipt, acceptance, and
   zero-occurrence restricted consumption have separately stated meanings.
3. **Safety scope:** require branch exclusivity and at-most-one accepted
   consumption in one trace extending an admissibly restored configuration;
   do not claim delivery, retry, global exactly-once, freshness, or atomic
   read-modify-write behavior.

The exact representation of M1 claims, reply payload, provenance, failure
receipt, `Gamma`/`Delta` disposition, invalid receipt policy, and fallback
interaction must be specified by that ordinary proposal only to the extent
needed by these three choices. They must not be invented as source syntax or
runtime API here.

## Future ergonomic inference boundary

The user-facing language should not force authors to spell an administrative
exchange relation or a correlation discriminator when the selected semantics
can generate it uniquely. This is a future elaboration question, not a reason
to leave the semantic relation implicit.

After a selected A-like model exists, a separate convenience proposal may
permit a cross-locus read form to elaborate an administrative `q`/`X` binding
when all of the following are proved for its admitted source fragment:

1. exactly one complete explicit semantic record or one Diagnostic results;
2. generated request, receipt, effect/failure rows, authority obligations,
   M1 claims, provenance, dependency, linear disposition, and save/load facts
   are preserved and inspectable;
3. no identity, authority, validation grounds, result, or restore fact is
   guessed from payload, principal, locus, span, session, queue position, or
   transport metadata; and
4. an ambiguity or missing premise remains a static Diagnostic rather than a
   hidden runtime branch.

This permits ergonomic omission of bookkeeping, not inference of semantically
meaningful facts. It also keeps a future simple `.mir` surface compatible with
an explicit, inspectable core model.

## What can proceed after the decision

Once an owner/Canon decision supplies this boundary, autonomous work can
reopen in existing lanes: a candidate-native adverse-case model, a
pre-registered L3 countermodel or conditional lemma, a small formal
configuration/restore model, and later an elaboration-convenience proof. Each
remains subject to ADR-0014 and cannot move OBL/Gate/Phase status. Before that
decision, Plans 214 and 218 still rule out a non-duplicate L3 candidate that
would test C2-B/C3 proper without choosing a carrier.

## Non-effects and reopen trigger

This packet changes no Canon text, `working/` record, Core term, Config,
SaveObject, failure universe, authority rule, grammar, elaboration rule,
runtime, adapter, wire/API contract, theorem/OBL, scenario, Gate, Phase,
sample, implementation status, or public claim.

Reopen this packet when an owner/Canon response accepts, rejects, or revises
the A boundary; when a source change supplies a different literal carrier; or
when a concrete falsifier shows that A cannot meet one of the stated safety
requirements without C's nominal identity. Do not create a new helper or
implementation merely from this comparison.
