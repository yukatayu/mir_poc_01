---
id: meta/proposal-017
status: L3-open
maturity: draft
depends_on: [theory/01-mircore-v0, theory/02-types-effects-failures, theory/03-elaboration, theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, adr/ADR-0012, adr/ADR-0014, meta/proposal-012, meta/proposal-013]
summary: V1/R1/M1 が必要とする C2-B/C3 の相関・receipt・one-shot/use・save/load 境界について、relation-state envelope を採択候補として起票する。Core、runtime、OBL、公開契約は変更しない。
open_items: []
---

# PROPOSAL-017 - C2-B/C3 relation-state envelope

> Decision-request artifact. It presents the smallest candidate identified on
> the current Canon reading and supporting LAB evidence for the already recorded
> V1/R1/M1 directions. It records no owner disposition
> at creation. Acceptance authorizes only the bounded integration design
> package described below; it does not itself amend theory, select a concrete
> carrier schema, or authorize implementation.

## Target and authority boundary

`PROPOSAL-012` records V1 (restricted result binding) and R1 (separate typed
owner result and requester receipt); `PROPOSAL-013` records M1
request-associated validation context. Neither record selects the semantic
residence that connects one dynamic request to validation, owner outcome, reply
availability, receipt acceptance, one-shot use, observation, and load.

This is a core/Config/failure/authority boundary and therefore remains an
ordinary human-owner decision under ADR-0012 and ADR-0014. The proposal is
limited to cross-locus read requests in the V1/R1 scope, including their typed
failure paths and M1 request-local validation context. Owner-local reads, other
result-returning operation families, successful-write acknowledgements,
admission, fallback, a general continuation, protocol, and source grammar are
outside X1.

## Candidate disposition X

### X1 - relation-state envelope (recommended)

A later integration package may define and test an explicit relation-valued
exchange component proposed for a later ordinary Config amendment. Its semantic
anchor is an existing request occurrence `q` in the current history, including
a configuration restored by an admissible `SaveObject`. `q` is not a
source-visible identifier, transport/session key, public token, or globally
equal occurrence across independent loads.

The later package must define the dynamic domain of the relation and an
unambiguous association from every in-scope request occurrence `q` to exactly
one requester-side pending administrative binding. Distinct request occurrences
may not share effective pending, result, receipt, or consumption state merely
because their source spans, payloads, M1 claims, queue positions, or transport
metadata agree. The pending binding, its requester locus, and its held
`Gamma`/`Delta` state may not reside only in an evaluator side table or proof
relation.

The component must provide a named semantic residence for these facts, but this
proposal does not prescribe its final arity, field layout, constructor names,
or storage encoding:

1. request-associated M1 claims bound to `q` no later than semantic request
   emission/enqueue, plus immutable validation-provenance references to the
   membership, capability-lineage, witness, admission, visibility, and history
   grounds actually consulted at owner validation. Neither claims nor recorded
   provenance is authority or a substitute for live `M`, `G`, `W`, or history;
2. an explicit service-outcome state with an outstanding disposition and at
   most one terminal owner success or typed owner-service failure. Outstanding
   need not terminate, owner failure has no owner mutation, and owner success
   is terminal only for owner service, not for the whole exchange;
3. typed owner-result and owner-service-failure facts with result and
   validation provenance, plus a separate requester pending/receipt/use state
   after success. X1 does not select requester-side failure-receipt identity or
   close OPEN-010;
4. a requester-side **semantic** receipt transition distinct from owner service,
   including a declared acceptance predicate;
5. one-shot restricted-consumption semantics under which at most one accepted
   consumption is possible for a branch over an execution and every admissible
   restored continuation. The later package selects the representation of
   consumption, preserves theory/01's zero-or-one-occurrence discipline, and
   states the exact `Gamma`/`Delta` disposition on success, owner failure,
   receipt rejection, and load. X1 selects no `pure`, zero-occurrence, Core,
   evaluator, or occurrence presentation;
6. no implicit observability of raw result, failure, receipt, M1, or
   validation-provenance state. If a later package exposes an exchange fact to
   telemetry, audit, devtools, or another observer surface, it must define a
   separate typed theory/07 projection with a declared subject occurrence or
   telemetry effect, observer authority, view label, monotone redaction,
   explicit retention/export surface, proof-or-reason references, and source
   span linkage; and
7. a restore correspondence and complete `SaveObject`/causal-or-channel closure
   for every exchange fact relied on after load.

Raw adapter-delivery candidates are not semantic receipts and their multiplicity
is unconstrained by X1. For each successful branch, at most one semantic receipt
transition may change the branch to accepted. A rejected candidate must be
treated in exactly one of two ways by the later package: either it is outside
the semantic exchange transition system and leaves receipt-pending, result
availability, and the one-shot budget unchanged; or it is a separately typed
requester-side transition in the request's declared dynamic failure row, with
an explicitly stated terminal or nonterminal effect on receipt-pending state.
A requester-side rejection is not owner-service failure. If the selected
treatment needs a new failure member or closes OPEN-010, it needs a separate
ordinary Canon amendment.

The following scope exclusions are part of X1: delivery or fairness guarantees,
retry, timeout, cancellation, global exactly-once, cross-load global identity,
result, cache, or snapshot freshness beyond M1's required authority
epoch/incarnation and lineage checks, atomic read-modify-write, wire/API
format, and implementation storage representation. No proof, scenario, or
runtime claim may depend on an excluded property without a successor proposal.

### XD - defer

Leave the carrier boundary unresolved. A later package may not model C2-B/C3
as though the current history, queue position, source span, payload equality,
transport metadata, evaluator side table, or proof relation already supplies
the missing exchange facts.

## Why X1 is recommended

The current Canon declares no history-only projection supplying an outstanding
branch, receipt acceptance, restricted consumption, redacted observation, or
restore correspondence. Silently treating helper-derived state as semantic
would violate the no-hidden-edge/no-hidden-authority discipline. A fresh nominal
identity can also locate them, but introduces freshness, equality, non-reuse,
replay, and persistence commitments not yet needed by the observed obligations.

X1 therefore makes the necessary state explicit while reusing the existing
request occurrence only as an in-history anchor. `q` is not a public or
universal token, but every fact used to establish request/result/receipt/use
correspondence must connect through the selected relation to that occurrence.

## Required integration package after X1

Before any theory/spec/ADR amendment, the bounded package must define only the
minimum operations and then test all of the following:

These rows are design-package review conditions and falsifier requirements only.
They are not THM or OBL statements, do not receive OBL status, and discharge no
existing well-formedness, save/load, authority, or observation theorem.

| Boundary | Required result |
| --- | --- |
| M1 and authority | all P013 M1 and theory/05 lineage/visibility components are checked. Copied/replayed claims, stale epoch/incarnation, wrong principal/role/target, missing capability/witness, grant-policy mismatch, severed provenance, visibility denial, and two active principals at one source locus fail closed with no owner mutation. Transport, the relation record, and validation provenance are not authority |
| branch and type | terminal branches are exclusive; result and every dynamic failure are typed and row-contained; static ambiguity is Diagnostic |
| causality | name and map every claimed order to an existing theory/04 generator or an identified future amendment: request-to-owner-service; grant/witness/authentication-evidence/membership lineage to validated service; selected owner-result or reply/send-side fact to semantic receipt; receipt acceptance to restricted-consumption enabling; and consumption/dependency to each later dependent occurrence. A zero-occurrence administrative consumption is not an `H` node. All selected projections are acyclic, owner-serial, and zero-or-one-occurrence disciplined |
| one-shot use | only accepted success enables at most one V1 consumption over an execution and every admissible restored continuation. Load cannot reset the budget or re-enable a consumed branch; raw delivery multiplicity, service exactly-once, and global exactly-once do not follow |
| observation | storage in exchange state, SaveObject, or history grants no observability. Any export is a separate typed theory/07 projection with authority, label, monotone redaction, explicit retention/export, reason references, and source-span linkage; no untyped debug view leaks raw exchange state |
| save/load | distinguish and test emitted/service-pending, terminal owner-service failure, owner success/receipt-pending, accepted/unconsumed, consumed, and every selected semantic receipt-rejection state. Identify SaveObject or explicit proposed-widening placement for each live exchange fact, pending binding, and one-shot state; preserve consistent-cut/causal-or-channel closure; use a restore correspondence that neither merges nor duplicates branches; do not revalidate a recorded owner result against different grounds, revive stale authority, reset consumption, or infer occurrence equality across independent loads |
| source boundary | X1 authorizes no source syntax, omission, inferred dynamic correlation, or extension of the unified elaboration judgment. A separately approved future proposal must meet theory/03 determinism, no-hidden-edge, span-preservation, row-containment, and authority-obligation clauses; it may elaborate declared static syntax/context to one explicit administrative form or Diagnostic, but may not infer dynamic correlation from span, payload, queue, history adjacency, or transport metadata. A semantic receipt is not an existing `G_e` row |

The package must state which exchange facts are stored and which are derived,
and record a decisive falsifier and either a bounded proof argument,
countermodel, or explicit stop result for each row. It must stop if a row
requires an unannounced Core primitive, source identity, transport carrier,
public contract, causal generator, occurrence kind, or `G_e` row. The relation
itself records state only when an explicitly selected transition updates it; it
does not itself generate communication.

## Requested owner output

Record one of:

```text
X = X1 relation-state envelope | XD defer | return for clarification
```

`X1` authorizes only the required integration package. It does not change
`Config`, `SaveObject`, `Core`, a transition rule, any OPEN item, an OBL, a
scenario, a Gate, a Phase, or implementation/public readiness. A later precise
integration proposal must undergo the ordinary amendment procedure before any
of those effects occur.

## Non-effects

This proposal does not:

- add an `ExchangeId`, attempt/future/session object, queue schema, result
  field, receipt constructor, continuation, evaluator, or runtime transition;
- change request syntax, Core grammar, Config, SaveObject, occurrence schema,
  causal family, authority semantics, failure universe, redaction lattice,
  diagnostics, fallback, admission, or write service;
- authorize source-level inferred correlation, administrative omission, a new
  `G_e` row, or any source grammar change;
- establish a theorem, OBL status, scenario conformance, Gate/Phase movement,
  implementation authorization, wire/API contract, or public behavior; or
- supersede PROPOSAL-012 or PROPOSAL-013.
