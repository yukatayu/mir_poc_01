# Plan 187 - MirCore value-flow and occurrence decision packet

## Role and authority

This is LAB decision support. `mirrorea_canon/` remains normative.
`PROPOSAL-012` is the corresponding Canon decision-request artifact only; it
records no answer. This plan does not amend a Core constructor, result carrier,
transition rule, event kind, OBL, Gate, Phase, conformance rule, runtime, or
public contract.

Plan 186 established the literal source boundary. This plan records the
minimal question decomposition, primary-source comparison, and stop line for
the ordinary owner/canon process. It is not a new WRK: the current admitted LAB
lanes have no shared executable semantics for either disputed interface.

## Settled inputs

- `read` is a dependency, never an occurrence; `write` is an occurrence.
- Cross-locus effects are explicit S1 requests/rows; ordinary S0 source does
  not acquire event machinery or ambient authority.
- A step appends zero or one S2 occurrence. The occurrence history is acyclic.
- Owner queues serialize mutation. Capability/witness lineage, epoch, and
  incarnation are checked fail-closed at owner service.
- `G_e` dependency membership is closed by current Canon wording. Its full row
  schema is not selected here.
- `PROPOSAL-008` (BND-001 outcome totality) and `PROPOSAL-009` (static
  THM-001 Core/write statement interface) are separate owner boundaries.

## Source-grounded gaps

| Gap | Literal Canon evidence | What is not supplied | Consequence |
| --- | --- | --- | --- |
| Read result to computed write | Core has `read`, `write(..., vprime)`, `seq`, `cond`, and `pure`; SCN-02 uses a computed `vprime`; OPEN-011 leaves reply/receipt open. | A result carrier, computation-binding relation, evaluation context, result locus, failure path, and no-duplication rule. | A formal model must not silently invent value flow. |
| Successful remote service | `[E-SERVE]` gives `request -> serve`, validation, write or read+reply, and one occurrence per step. | Whether service/mutation/result/reply/receipt are one node or several and the exact causal projection. | A preservation or trace proof cannot assume a service event identity. |
| Admission effects | `[E-ADMIT]` updates membership and issues grants/witnesses; causal family names verdict, grants, witnesses, and membership-dependent dispatch. | Whether those are individual occurrence nodes or projections of one admission node. | A proof cannot assume grant/witness/membership history mapping. |

## Primary-literature comparison

| Mechanism or model | Established point used here | Mir-specific conclusion not justified |
| --- | --- | --- |
| Moggi, *Notions of Computation and Monads* (1991) | A monadic `bind` is a standard way to sequence a computation that produces a value into a dependent computation. | That Mir must use a monad, choose a type constructor, or expose a general `bind`. |
| Flanagan et al., *The Essence of Compiling with Continuations* (1993) | Administrative normal form and CPS make intermediate results/evaluation order explicit. | That Mir should add first-class or migratory continuations; they are outside v0. |
| Felleisen and Hieb, *The Revised Report on the Syntactic Theories of Sequential Control and State* (1992) | Evaluation contexts can specify direct-style operational sequencing through an explicit formal context relation. | That an unstated meta-level evaluator is sufficient, or which Mir evaluation strategy is correct. |
| Fournet and Gonthier, *The Join Calculus: a Language for Distributed Mobile Programming* (2000) | Distributed operational semantics can make asynchronous send/receive and synchronization phases explicit. | That Mir needs its protocol calculus, queue/delivery/fairness model, or transport realization now. |
| Abadi, Fournet, and Gonthier, *Authentication Primitives and their Compilation* (2000) | Authentication facts may be modeled semantically and related to lower-level realizations by a security/refinement argument. | That Mir may collapse capability/witness/membership into transport, or which proof relation it needs. |

The following are lower-bound inferences from the current syntax plus the
sources, not claimed published minimality theorems for Mir:

1. A non-fused read that returns a value for a dependent write needs either a
   syntax-level value-flow form or a formal machine/context relation. Plain
   `seq : Core x Core -> Core` orders terms but cannot name the result.
2. If service, validation, result, reply, receipt, or admission sub-actions
   must be independently observable or interleavable, they cannot be hidden in
   a genuinely atomic one-node model without an explicit abstraction/refinement
   boundary.

## Decision decomposition

`PROPOSAL-012` asks the owner for four independently recordable dispositions:

| ID | Alternatives | Dependency | Why it is separate |
| --- | --- | --- | --- |
| V | V1 restricted result-binding contract; VD defer | First for read-returning service | An evaluation frame may be an equivalent presentation of V1, not a no-semantics alternative. |
| R | R1 explicit typed reply/receipt; R2 explicit abstract-receipt refinement; RD defer | After V and before read-returning service | It decides how one requester can resume without a hidden callback. |
| S | SW1 atomic served-write with typed facets; SW2 decomposed service/write; SWD defer | Independent of R for write-only service | It decides successful owner mutation identity, not admission lifecycle. |
| A | A1 decomposed admission occurrences; A2 composite admission/verdict occurrence with explicit projections; AD defer | Independent | It decides membership/grant/witness identity, not read computation. |

The candidate families deliberately exclude a new fused read-modify-write
primitive, general continuations, generic I/O, source syntax for S1
requests/publications or S2 occurrences, transport-defined authority, and
domain vocabulary. S5 domain events remain distinct and may appear in source.
Those alternatives would
either prematurely specialize the Core or contradict existing L0/L1 boundaries.

## Required later proof obligations

No alternative is a design by itself. A non-defer owner disposition must be
followed by a separate design package that makes at least the following
obligations inspectable:

| Family | Required obligations |
| --- | --- |
| V1 | typed result/failure carrier; lexical/local administrative scope; single evaluation; no dependent write after failure; capability/witness conservation; deterministic elaboration relation. |
| V1 machine presentation, if later compared | trace equivalence to the V1 restricted binding contract; unique local decomposition where applicable; context ownership/no copying; result/failure resumption locus; no hidden communication; and relation to source spans and `G_e`. |
| R1 | typed request/result correlation; requester receipt and one-shot resumption; result provenance/redaction; duplicate/stale/wrong-locus rejection; explicit failure behavior; and cut/save/load treatment. |
| R2 | explicit abstraction/refinement relation showing why no separate receipt is observable or interleavable; a unique requester-side pending-control transition; and the same failure/cut/save/load safeguards. |
| SW1 | explicit projections to service and state-mutation roles; the same node satisfies the state-mutation predicate; relation to request; and auditability of authority and failure. |
| SW2 | pending authorized-operation state; primitive-step ordering, intermediate-state observability, request/service/mutation causal edges, owner seriality, revocation/epoch protection, and failure/no-mutation. |
| A1 | causal order and partial-failure behavior for verdict/membership/grant/witness; no dependent dispatch before required facts exist. |
| A2 | projection references usable by grant/use, witness/use, and membership/dispatch edges; rejection no-mutation; epoch/incarnation and load/rollback lineage. |

All families must preserve SCN-02 intent, DAG acyclicity, owner-serial writes,
linear/non-duplicating capability use, explicit failure rows, and the boundary
between semantic carriers and replaceable security/transport realization.

## Recommendation and stop line

The recommended next action is to record the four owner dispositions in
`PROPOSAL-012`, not to create an executable model. An executable model would
have to choose the value and occurrence semantics that the packet is asking the
owner to choose.

Current recommendations, not decisions, are V1, R1, SW1, and conditional A2.
They minimize hidden cross-locus state while preserving owner-serial atomic
mutation and typed grant/witness lineage. Their exact carriers, syntax, and
proof laws remain later design work.

Until a disposition and a subsequent design package exist, autonomous work may
continue on unrelated standing-eligible L3 research. It must not add a Core
binding/context/event carrier, reinterpret `[E-SERVE]` or `[E-ADMIT]`, claim a
proof bridge, or make LAB code a Canon tie-breaker.

## Non-claims

This plan does not claim that the Canon is inconsistent; that any option is
accepted; that a restricted binding, an evaluation context, a composite event,
or a decomposed event is final; or that a parser, runtime, distributed protocol,
or formal proof is ready. It does not change samples or their workflow status.
