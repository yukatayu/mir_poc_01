# Plan 208 - C2-B/C3 value-flow design preparation

## Role and authority

This is a **LAB ordinary-design preparation packet**. `mirrorea_canon/` remains normative. The packet compares presentations authorized for later design work by P012 `V1`/`R1` and P013 `M1`; it does not choose a Canon Core constructor, occurrence schema, request field, identity rule, Config field, SaveObject field, runtime, wire format, API, OBL, Gate, or Phase.

The recorded owner dispositions permit a compatibility-reviewed design package but not a rule change or implementation. Any recommendation below remains a proposal until the normal owner/Canon process selects its carrier and contract.

## Authority cut and scope

Preparation cut: `4d699c407c9d5a51ecb455905906fab615d92db2`.

| Settled bounded direction | Required consequence for this packet | Still unselected |
| --- | --- | --- |
| P012 V1 | one-shot, locus-bound result use; syntactically delimited administrative binding; held `Gamma`/`Delta`; no dependent write after failure | Core spelling, general bind, continuation, final evaluator |
| P012 R1 | typed owner reply and separate requester receipt; matching receipt before resume | correlation carrier, result storage, occurrence projection, retry/delivery semantics |
| P013 M1 | request-associated validation claims compared with authoritative membership/lineage/witness/history facts | request field, queue/wire envelope, request-instance identity, authority representation |
| theory/01/03/04/05 | `read` is a dependency; steps append zero or one occurrence; DAG and authority lineage remain explicit; cuts/save-load preserve safe frontiers | exact response/failure mapping, occurrence equality after restore, pending carrier |

An advisory temporary Oracle review with response digest `e1fc575e8981ad53f1603ea27f44b095ce99d95c4959d7a6966d0a10d87b3ac4` independently challenged this structure. It is advisory only; requirements below are grounded in the pinned Canon directions and LAB evidence.

## C2B/C3-alpha comparison cut

The smallest coherent comparison is one typed cross-locus read from requester `Lr` to owner `Lo`, with M1 context, one owner success reply or failure, one requester receipt, a requester-local restricted pending binding, first-order pure computation, and the first later dependent occurrence. It deliberately does not model service semantics for that later occurrence.

```text
request emission -> owner validation and typed reply/failure -> requester receipt -> zero-occurrence restricted pure resume -> later dependent ordinary occurrence
```

The roles are not Canon occurrence kinds or Config fields. Every successful candidate must retain direct order `request-role < reply-role < receipt-role < dependent-occurrence`, but ordering alone is insufficient: a typed correlation relation must identify which request, reply, receipt, and pending binding belong together. Payload, claims, source locus, queue position, transport session, and causal ancestry alone are not permitted replacements.

## Family-neutral roles and obligations

```text
CtxOf(request-role, validation-context)
ResultOf(reply-role, result, result-provenance)
Corr(request-role, reply-role, receipt-role, pending)
Accepted(receipt-role, pending)
ResumeOnce(pending, receipt-role, result)
Failed(pending, failure)
DepOf(later-occurrence, pending)
```

This four-ended `Corr` is an initial compact shorthand only. It is not
prefix-local before reply/receipt exists and cannot by itself describe the
failure branch. Plan 209 replaces it, for active comparison audit purposes,
with staged carrier-neutral obligations (`PendingFor`, `ReplyFor`,
`ReceiptFor`, and `FailureFor`) plus an owner-validation/outcome relation.
Those names are audit vocabulary, not selected Canon primitives or fields.

Every family must meet Plan 209's active prefix-local conditions: applicable
staged relations are functional; within one admissible restored-prefix
extension there is at most one accepted success receipt and one resume per
pending; an owner-service failure leaves the owner store unchanged and cannot
produce a matching success continuation or dependent occurrence derived from
that pending; M1 claims are inputs to authoritative
membership/lineage/witness/admission/visibility/history validation and confer
no authority; held `Gamma`/`Delta` ownership is explicit and linear evidence
is neither duplicated nor silently dropped; and correlation, pending, result
provenance, validation grounds, and post-consumption/failed status are present
in or uniquely reconstructible from the restored admissible configuration.
Result value, history metadata, provenance, and redaction remain distinct,
though provenance may refer to admissible history.

`one-shot` is branch-scoped. It does not claim transport exactly-once, global uniqueness across two loads of the same SaveObject, or durable exactly-once.

## Presentation families

| Family | Comparison presentation | Strength | Decisive risk / stop line |
| --- | --- | --- | --- |
| A: relational administrative reference | typed `Waits`, `Answers`, `Delivers`, and `Resumes` relations connect pending, request, reply, receipt, result, provenance, and held contexts | exposes required relations before choosing nominal identity | relation must not be an implicit meta-level correspondence; its semantic locus and restore behavior need selection |
| B: request-occurrence anchored | one existing request occurrence role anchors pending, M1 context, reply, receipt, and provenance relations | minimizes a new identity type and uses request-to-service order | occurrence equality/stability across cut/save-load must be explicit; ancestry alone cannot identify correlation |
| C: nominal attempt/exchange | fresh semantic attempt identity maps injectively to request roles and has pending/reply/receipt status | makes retry, stale reply, and future attempt distinction expressible | selects freshness, equality, retirement, persistence, and branch behavior; defer unless A/B cannot close the boundary |

An abstract machine or evaluation-frame may be used only as a trace-observation-preserving presentation of V1, not as a fourth owner alternative or a runtime. Receipt is one role/occurrence, pure resume appends no occurrence, and the dependent action is later.

## Comparative recommendation

Use Family A as the reference signature and test whether Family B instantiates every relation without hidden identity. If request-occurrence anchoring cannot preserve correlation and one-shot status through save/load/replay, this is concrete grounds to compare Family C. This does not choose A or B as a Canon carrier. Family C is not a convenience ID: a wire correlation ID, session ID, or queue position may not be relabeled as Family C.

## Required adverse matrix

| Case | Required rejection or distinction |
| --- | --- |
| identical payload/claims, two request emissions | no payload-based aliasing of `Corr` or pending |
| two active principals at one locus | no source-locus or transport-based identity/authority recovery |
| duplicate or late reply | at most one matching receipt; no resume after spent/failed state |
| wrong-locus receipt | semantic requester/locus relation is checked; transport endpoint is insufficient |
| copied/replayed complete-looking M1 claims | claims do not substitute for authoritative lineage validation |
| leave/rejoin, revocation, or one lineage mismatch | stale/severed use fails closed without owner mutation |
| failure before receipt | no success reply/resume/dependent occurrence; held context is accounted for |
| save/load at request, reply, receipt, resume, failure, dependent frontiers | correlation/provenance/pending/linearity are explicit or uniquely reconstructed |
| owner-side interleaving before later write | no read-modify-write atomicity is inferred |

## Explicitly deferred

This cut excludes SW1 service/mutation identity, A2 admission projections, successful-write acknowledgement, source `bind`/`let`/ANF, general futures/continuations, retry/timeout/cancellation/fairness/delivery, queue/wire representation, C1 snapshot/fusion/atomic read-modify-write, pending migration, durable exactly-once, concrete Config/SaveObject/IR/`G_e` fields, and every Lean/OBL/Gate/Phase claim. SCN-02 is only a compatibility trace: read reply, receipt, pure computation, then a separately authorized later write request. The receipt is neither authority nor a freshness/atomicity guarantee for that write.

## Owner questions and autonomous preparation

The following need owner/Canon selection before a semantic model or implementation relies on them: identity foundation and equality/reuse scope; occurrence identity across save/load; pending/correlation locus including result/provenance/spent/failed status; receipt/resume granularity; result freshness/revalidation timing and requester-side failure mapping; held `Delta` ownership and one-shot scope across load; and the exact artifact retaining M1 context, source span, failure row, and dependency grounds.

Before that selection, autonomous preparation may define relation signatures,
map families A/B/C to common obligations, build the finite adverse matrix,
audit save/load frontiers, and state machine-presentation equivalence
requirements. Plan 209 performs the first such prefix-local audit. It may not
create a new WRK theorem, runtime helper, schema, queue, or source syntax.

## Reopen and non-effects

Reopen when an owner selects a semantic choice or a source-backed discriminator shows that a family cannot meet the common obligations. This packet does not adopt a family, alter P012/P013, change authority/transport boundaries, make a fact inferable, authorize a source omission, or advance proof, implementation, or public status.
