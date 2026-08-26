---
id: theory/13-evaluation-materialization
status: L1-fixed
maturity: reviewed
depends_on: [theory/01-mircore-v0, theory/02-types-effects-failures, theory/03-elaboration, theory/05-authority, adr/ADR-0018, adr/ADR-0027, adr/ADR-0028, adr/ADR-0029]
summary: 評価場所・clock・authority origin・materializationを分離し、owner RMW、explicit receipt、designated evaluator、SYS-1 lifecycleとSYS-2 bounded execution refinementを定義する有限calculus。
open_items: []
---

# 13 — Evaluation and materialization calculus

This chapter defines the M3 extension of the shared Core universe. It is
syntax-independent: it specifies what elaboration records, not the final M6
Surface spelling.

## 1. Evaluation plan

Let declared loci, principals, providers, policies, fields, and frontiers be
finite. An evaluation plan is:

```text
EP = ⟨ key, form, site, trigger, authority, materialization, policy,
       observation-policy, frontier? ⟩
form ::= value | state | relation | computation
site ::= owner(O) | locus(L) | designated(E) | consumer(C) | provider(P)
trigger ::= on-request | on-event | on-change | logical-tick
          | frontier-advance | presentation-frame | explicit
authority ::= caller(π) | owner-transition(O) | admitted-evaluator(E)
            | admitted-provider(P)
materialization ::= canonical finite target set over
                    { local-only, store, publish-value, publish-relation,
                      adapter-stream, persist }
```

The five displayed axes are independent. `policy` and `observation-policy` are
members of declared finite policy sets. The target set is nonempty and has canonical order/no
duplicates; `local-only` is exclusive; there is at most one `store`; `publish-value` and
`publish-relation` are mutually exclusive; and `adapter-stream`/`persist` are
exclusive in M3. `key` is a deterministic operation-origin key in the
parser-free reference and is replaced by an M6 source span relation when a
Surface origin exists. `frontier?` is required exactly for `designated(E)` that
materializes a value. It is a canonical finite **set** of exact producer
occurrence references: producer order is not semantic. It is not a scalar
global frontier, distributed snapshot, or transaction boundary. Residency mode
(`local | remote(O)`) remains a separate answer to
where a value is anchored; it is not an alias for `site`.

## 2. Core, configuration, and well-formedness delta

The M3 Core extension has exactly one evaluation operation:

```text
op ::= eval(key, body, EP)
body ::= argument(i) | read-local(O, field) | receipt(r) | pure(body)

request/result trace ::= remote-result(r, O → T, type, span)
                       | receipt(r, T, O, producer, frontier, label, value | failure)
                       | decided(key, frontier, version, value,
                                 validated-authority, policy, observation-policy)
                       | consume-result(key, frontier, version, consumer)
```

`Config` extends theory/01's `Σᵣ` with a finite pending-request/receipt store
`R` and a designated-result store `D`. `R` maps a request identity to its
owner-labelled request and then one receipt containing the request, serve,
reply, and receive occurrences; producer, exact input frontier,
visibility/redaction label, operation-origin key, and either a typed value or a
typed failure. A receipt is single-assignment, is inserted only by its
request/serve/reply/receive transition, and does not transfer authority,
ownership, lease, or a state version. `D` maps `(evaluator, key, frontier)` to
one decided result, its stamp, and one explicit semantic-consumption state.
Both are occurrence-backed. A well-formed M3
configuration additionally requires:

1. every owner-store `eval` has `site = owner(O)`, all direct mutable
   reads/write at O, `trigger = on-request`, and a target set containing
   `store`;
2. every transition request has a caller/admitted origin with validated
   capability and witness lineage; an internal owner-initiated transition is
   explicitly `owner-transition(O)`, never inferred from its site;
3. a `remote-result` receipt has the causal chain `request ≺ serve ≺ reply ≺
   receive ≺ consuming-eval`, has a target matching its consuming owner, and
   is admitted at its producer for the exact `(caller, producer, target,
   label)` release tuple; its visibility/redaction label is therefore neither
   caller-chosen nor an authority transfer. Its consumption is explicit and
   does not authorize an O write by itself;
4. a designated `eval` has `site = designated(E)`, a frontier, and declared
   evaluation/observation policies; its result key is unique for `(E, k, frontier)`; and
5. consumer/provider sites have no M3 semantic-store mutation rule.

## 3. Deterministic elaboration and defaults

For a finite well-scoped input, elaboration selects the first applicable rule
in this priority order; a rule with unmet premises returns its named
Diagnostic rather than falling through to a weaker rule.

```text
[INF-MUTATE]       same-owner mutable RMW → owner(O)/on-request/caller/{store}
[INF-PRIVATE]      owner-private computation → owner(O)/on-request/caller/local-only
[INF-DESIGNATED]   declared authoritative decision → designated(E)/declared clock/
                   admitted-evaluator(E)/{publish-value}, with frontier
[INF-RELATION]     safe pure relation → consumer(C)/presentation-frame/
                   caller/{local-only} plus M4 residual obligation
[INF-PROVIDER]     declared external computation → provider(P)/declared clock/
                   admitted-provider(P)/{adapter-stream}
[INF-LOCAL]        uniquely declared pure local computation → locus(L)/explicit/
                   caller/local-only
```

`[INF-MUTATE]` dominates the other rules. A cross-owner operand in its body is
not a direct read: an explicit `remote-result` annotation selects
`[ELAB-REMOTE]`, inserts a receipt dependency, and requires the receipt before
the owner transition becomes serviceable. No remote request, wait, callback,
provider call, or receipt acquisition occurs inside that transition. Without
the annotation, `E-EVAL-CROSS-OWNER` results. If two remaining plans are valid
and no priority rule selects one, `E-EVAL-AMBIGUOUS` names the differing
coordinate and requests only that annotation. Thus elaboration is a partial
deterministic function, never a placement search.

## 4. Steps and trace rows

The deterministic reference profile serves each owner FIFO. The following are
the M3 step schemas; each appended row includes its `EP` and origin reference.
When an M6 Surface origin exists that reference is its source span; the
parser-free M3 reference uses only a deterministic operation key and does not
claim a source-span implementation before M6.

```text
[E-OWNER-TRANSITION]
  dequeue r at O; validate caller/capability/witness/membership;
  read all O-owned direct dependencies; evaluate pure body once; write O-owned target;
  append request, O-local dependency, serve, evaluation, and write rows.

[E-OWNER-FAIL]
  failed validation appends request and failure rows; it changes neither S, R, nor D.

[E-REMOTE-RESULT]
  The reference first records r's request at the target, then T validates and
  admits the exact release tuple `(caller, T, O, label)`, then appends serve,
  reply, and receive/receipt rows before inserting R[r]. A later O evaluation
  may name only that stored typed receipt; no rule combines it with a T/O
  snapshot.

[E-DESIGNATED]
  if D[E,k,F] is absent, E validates, evaluates at F, assigns the next version,
  stores D[E,k,F], and appends evaluation/publish rows. If present, append at
  most a duplicate-publication observation: it does not decide or consume again.
  Failure appends a failure row and creates no decided success value.

[E-CONSUME]
  an explicitly named consumer records the one bounded consumption of an
  existing decided version. Repeating the same consumption returns that decided
  value without a second semantic-consumption row; a competing consumer is a
  typed conflict in this finite one-consumer profile. Presentation-only
  interpolation may follow, but never a semantic evaluation of expr.
```

SYS-3 maps the provisional internal Surface-v0 clause
`designated consume E.result at C` to this normative rule through a distinct
source/Core edge. The mapping adds no new multi-consumer semantics and no
runtime delivery claim; it exists so projection cannot invent the named
consumer from topology or schedule.

The rule is not a claim about the current M8 direct-consume API. That legacy
runtime rejects a repeated delivery id with `AlreadyConsumed` and admits a
different delivery id; M10 retains the same-delivery rejection as accepted
regression behavior. Therefore `ReturnExistingNoNewConsumption` is a new SYS-4
endpoint refinement obligation derived from `[E-CONSUME]`. SYS-3 records only
the source/Core-bound semantic-consumption identity and static requirement.
SYS-4 must interpose a carrier-side idempotent return or compatible wrapper so
the accepted first semantic consumption reaches M8 exactly once, while the
same named consumer's retry returns the decided result without a second M8
consume. This is not network exactly-once, hidden retry, or SYS-3 runtime
evidence.

All value-bearing trace rows are observations: requester-visible rows redact
owner-private operands unless an independent observation policy admits them.
The direct attack consequence is therefore a serial service trace
`100 → 90 → 80`, not two requester reads of `100`. `Caller(self)` remains on
both transition plans while `owner(S)` remains their evaluation site.

## 5. Named diagnostics and boundaries

`E-EVAL-CROSS-OWNER` rejects an unannotated other-owner operand;
`E-EVAL-AMBIGUOUS` rejects a non-unique plan; `E-EVAL-FRONTIER` rejects a
designated materialization without frontier; `E-EVAL-PROVIDER-MUTATION`
rejects a provider plan that writes semantic owner state; and
`E-EVAL-RECEIPT` rejects a missing, causally incomplete, failed, stale,
mismatched, or visibility-denied explicit receipt. Dynamic validation failures
remain the existing typed failure row and never mutate state. A consumer may
only use `{local-only}` and a provider only `{local-only}` or
`{adapter-stream}`; either site yields `E-EVAL-PROVIDER-MUTATION` (or its
consumer counterpart) when it targets semantic owner state or an authoritative
publication.

This chapter classifies but does not define M4 maintained relation DAGs,
projection composition, semantic/presentation fallback, or derived
observation labels. It defines no transport protocol, cache, retry,
multi-owner transaction, final grammar, or public schema.

## 6. Proof and executable boundaries

The relevant statements are OBL-001/003/004/020/021 in theory/11. M3 finite
evidence proves only its declared finite model; it is not a proof of arbitrary
topology, relation DAG, save/load, or patch behavior. Theory/15 supplies the
one concrete finite shared `EP`, `Config`, `Step`, `WellFormed`, `Trace`, and
Diagnostic universe for the accepted M3/M4 profile; it does not generalize
this M3 boundary.

## 7. SYS-1 internal carrier refinement

ADR-0027 realizes the previously abstract pending/receipt relation `R` for
two bounded I2-internal operation families without choosing a public schema:

```text
owner request r:
  pre-admit(r) ; request(r) ; serve(r) ; reply(r, outcome) ;
  receive(receipt(r, outcome))

designated remote input d:
  pre-admit(d) ; request(d) ; serve-at-source-owner(d) ;
  reply(d, outcome) ; receive(receipt(d, outcome)) ;
  consume-at-designated-evaluator(d)

outcome ::= typed-success(v) | declared-failure(f)
```

`pre-admit` checks the exact checked operation/dependency and source/Core
origin, target owner or producer/evaluator pair, origin principal/locus,
membership epoch/incarnation, capability/witness lineage, visibility/
redaction, effect/failure rows, and applicable input frontier/release tuple.
A failed `pre-admit` returns a typed diagnostic before `request(r)` exists; it
adds no semantic occurrence, enqueues no owner work, and changes neither
semantic state nor authority state. `[E-OWNER-FAIL]` continues to describe a
request that has passed this boundary and later obtains a declared dynamic
failure. These cases are not conflated.

For each admitted request, the kernel assigns a stable request identity
independent of queue position and concrete occurrences satisfying:

```text
request(r) ≺ serve(r) ≺ reply(r) ≺ receive(r)
receive(d) ≺ consume-at-designated-evaluator(d)
```

The owner receipt preserves the exact operation identity and origin/owner
lineage. The designated-input receipt additionally preserves the input
frontier, producer release tuple, receipt identity, and explicit consumption
key/state. Receipt construction is single-assignment. Duplicate or mismatched
reply/receipt attempts reject, but this implies neither hidden retry nor a
transport-level exactly-once guarantee.

The remote-input serve is a typed read effect at the checked source owner. Its
producer release capability is necessary and cannot be replaced by the
designated evaluator's decision capability. Receipt arrival communicates a
typed result/failure and causal provenance only; it does not grant authority,
transfer ownership, or authorize any semantic write. Evaluator consumption is
therefore an explicit later step, not an implicit callback or a multi-owner
transaction.

The SYS-1 production carrier initially consumes an immutable M9 generation.
ADR-0028 supplies its bounded live successor and ST/OW1 ordering refinement as
specified below. This section defines no generic provider registry, public
API/ABI/wire, transport, retry, or general memory model.

## 8. SYS-2 bounded execution refinement

For ST and OW1, an admitted owner transition evaluates its same-owner reads
and performs its successful write at the owner runtime. The successful write's
actual owner-store trace node is the linearization point; the read records the
initial or immediately preceding admitted owner version. A declared or
authority failure produces neither a write linearization nor a version
advance.

For designated remote input, `serve-at-source-owner(d)` includes an
acknowledged read from source-owner state. `reply(d, outcome)` is derived from
that read. A caller-provided divergent value is not a valid effect result and
rejects before reply/receipt/mutation. The remote operation continues to keep
producer release authority, evaluator decision authority, and receipt
causality separate.

An M9 successor generation becomes current only after the owner runtime has
acknowledged its translated inventory. After this publication, an earlier-
generation queued owner use rejects without mutation. A transition that
linearized before publication is not undone; its later result carrier remains
non-authority. The selected OW1 profile has one combined owner/source-owner
locus and one worker-exclusive store. Multi-owner placement and actual
generated dispatch remain SYS-3/4 work.

OBL-058 and OBL-059 classify the exact finite model and executable evidence.
They add no Lean theorem, general scheduler/memory/data-race result, fairness,
or ordinary-Surface memory-order vocabulary.
