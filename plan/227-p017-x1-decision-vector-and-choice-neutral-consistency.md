# Plan 227: P017 X1 Owner-Decision Vector and Choice-Neutral Consistency Matrix

## Role and authority

This is a LAB ordinary-design decision-preparation record following the owner
disposition in `mirrorea_canon/meta/proposals/PROPOSAL-017-c2b-c3-relation-state-envelope.md`
(P017 X1). Canon remains normative. This plan does not choose a relation
schema, a `Config` or `SaveObject` field, a request/result/receipt
carrier, an occurrence kind, a transition, a failure member, a Core form,
source grammar, runtime operation, wire/API contract, theorem, OBL, Gate,
Phase, or implementation.

Its only positive conclusion is procedural: a future X1-bounded design package
can state the minimum coherent decision tuple and its review conditions without
pretending that the tuple has already been selected or implemented. X1 already
selects an explicit relation-valued exchange component as the semantic family;
the vector only names questions within that family, not semantic objects or a
shared interface.

## Authority cut and source ledger

Review cut: `e8be4a5b90cce433d0d3270a0d25dac412e28071`.

### DIRECT

| Source | SHA-256 | Exact role in this plan |
| --- | --- | --- |
| P017 X1 relation-state envelope | `65f847f3d57cbbc5dd1f86540964fd5d9a7b6e3fcf13387c2776a08edf8254e3` | Bounds the cross-locus V1/R1 read scope, fixes X1 requirements/exclusions, and authorizes only a later integration design package. |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | Keeps theory/spec/contract/OBL/runtime amendments outside this LAB preparation record. |

### DIRECT-PIN-REQUIRED

These sources are direct constraints whenever a later ordinary proposal claims
to close a row. The current plan cites them as read-only anchors and does not
derive a new rule from them.

| Source | SHA-256 | Constraint that a later candidate must pin |
| --- | --- | --- |
| `theory/01-mircore-v0` | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` | Request occurrence, zero-or-one occurrence discipline, owner seriality, OPEN-010/011 boundary. |
| `theory/02-types-effects-failures` | `40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257` | Typed result/failure, declared dynamic rows, static Diagnostic boundary. |
| `theory/03-elaboration` | `2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641` | No hidden communication, authority, or semantic carrier; source convenience remains later. |
| `theory/04-ordering-and-cuts` | `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` | Existing causal family, acyclicity, admissible save/load, complete cut/channel closure. |
| `theory/05-authority` | `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` | Claims are not authority; lineage validation and stale-resurrection prohibition. |
| `theory/06-existence-fallback` | `3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8` | If fallback enters scope, lineage remains monotone and reacquisition is fresh. |
| `theory/07-observation` | `3b0ed16c0506550e33f25f2d71839cef14e545fb9f51bd7a117e2a9b41f8d239` | Any exchange observation is a typed authority/label/redaction/retention projection. |
| P012 V1/R1 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` | Restricted result use and owner result/requester receipt are separate directions, not final carriers. |
| P013 M1 | `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | Validation context is request-associated and non-authoritative. |
| `spec/04-core-ir` / `spec/05-runtime-semantics` | `50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950` / `25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c` | Present L2 exchange/runtime shapes are not a selected X1 carrier or lifecycle. |

### LAB-MEMORY

Plans 215--221 supply the pre-P017 comparison history and P017 preparation;
Plans 225--226 close only their respective duplicate fixture/restore
countermodel lines. They do not replace the DIRECT ledger. WRK-0040--0043
remain finite supplied-fixture detector evidence only. The temporary Oracle
scope review (SHA-256
`67aa38d2893f186c21bc56d542b167948f697f130b020af0a9158324aa67b574`)
was advisory input; this plan is justified by the pinned repository sources.

## Fixed X1 invariant envelope

Every later candidate in this scope must preserve the following P017 facts.
This is a constraint list, not a common state machine.

1. The scope is a V1/R1 cross-locus read. An owner-local read stays the
   existing dependency-only path and receives no X1 lifecycle.
2. The semantic anchor is an existing request occurrence in one current or
   admissibly restored history. It is neither source spelling, public token,
   transport/session key, nor global equality across independent loads.
3. The X1 relation component has a dynamic domain and gives each in-scope
   request occurrence exactly one requester-side pending administrative binding.
   Incidental equality of span, payload, M1 claims, queue position, or
   transport metadata cannot merge effective state.
4. M1 claims are bound to the request occurrence no later than semantic request
   emission/enqueue. Immutable provenance references name the membership,
   capability-lineage, witness, admission, visibility, and history grounds
   actually consulted; claims and provenance remain separate,
   non-authoritative facts.
5. Owner service can remain outstanding. At most one terminal owner success or
   typed owner-service failure is associated with the selected branch; owner
   failure has no owner mutation.
6. Owner result availability, requester receipt, receipt acceptance, and
   restricted consumption are distinct. For each successful branch, at most one
   semantic receipt transition may change the branch to accepted. Raw adapter
   delivery is not a semantic receipt and has no X1 multiplicity guarantee.
7. At most one accepted consumption is possible for the branch over an
   execution and every admissibly restored continuation. Storage is not
   observation, and no raw exchange state becomes an untyped debug leak.
8. Any fact used after load has complete `SaveObject` / causal-or-channel
   closure and a declared restore correspondence. Load cannot merge or
   duplicate branches, reset use, revalidate a recorded service result against
   different grounds, resurrect stale authority, or imply independent-load
   equality.

## The owner-decision vector

The following six metanames make interdependent choices inspectable. A future
proposal chooses a coherent candidate-native answer for each relevant
coordinate. This plan ranks none of their concrete alternatives.

### R — residence and reference scope

**Question.** Where do selected exchange facts live semantically, what counts
as the request occurrence's scope within a current/restored history, and which
facts are primitive versus uniquely derived?

The answer must account for the X1 relation's dynamic domain, the pending
binding, requester locus and held
`Gamma`/`Delta` state, M1 claims/grounds, owner outcome, result
availability, receipt/use facts, and restored reference scope. It must name a
relation-valued semantic residence rather than relocate facts into an evaluator
table, proof relation, queue convention, source span, or transport metadata.
It must not select a field layout, key, public identity, or global cross-load
equality.

### B — owner branch, typing, and authority provenance

**Question.** How are outstanding and terminal owner-service alternatives
distinguished, typed, and tied to the authoritative grounds actually checked?

The answer preserves at-most-one terminal owner branch, typed success/failure,
no owner mutation on failure, and the difference between M1 claims and
validated authority provenance. It does not choose a failure-universe member,
validation algorithm, mutation rule, result payload carrier, or transition.

### T — requester receipt and rejection

**Question.** What makes a requester-side receipt semantic, how does it match
the selected owner result, and how are candidate rejection and multiplicity
treated?

The answer keeps owner service, result availability, receipt, acceptance, and
consumption distinct, and permits at most one semantic receipt transition to
change a successful branch to accepted. P017 permits either a rejected
candidate outside the semantic exchange transition system, leaving receipt-
pending, result availability, and the one-shot budget unchanged, or a
separately typed requester-side transition in the request's declared dynamic
failure row. In the second case the candidate states whether its effect on
receipt-pending is
terminal or nonterminal. It does not infer delivery, timeout, retry, fairness,
global exactly-once, or a wire protocol.

### U — restricted use and linear disposition

**Question.** How does an accepted success make one restricted V1 use
available, and what are the exact `Gamma`/`Delta` dispositions on
success, owner failure, receipt rejection, and load?

The answer prevents re-consumption/re-resumption after an admissible load. It
does not preselect `pure`, zero occurrence, Core presentation,
evaluator mechanism, or continuation form; it only respects theory/01's
zero-or-one-occurrence discipline.

### C — occurrence and causal integration

**Question.** Which selected facts are occurrences, zero-occurrence
administrative transitions, or derived projections, and which existing
theory/04 causal generator justifies every relied-on order?

At minimum, the candidate accounts for request-to-owner service;
grant/witness/authentication-evidence/membership lineage to validation; owner
result or selected reply/send-side fact to semantic receipt; accepted receipt
to restricted-consumption enabling; and consumption/dependency to each later
dependent occurrence. An order without an existing generator is a future
amendment, not an implicit edge. This plan creates no causal family or
occurrence kind.

### L — semantic persistence and restore

**Question.** What is placed in configuration, `SaveObject`, or selected
channel/in-flight closure, and how does an admissible load correspond to the
pre-load exchange facts without merging, duplicating, resetting, or
revalidating them?

The answer distinguishes emitted/service-pending, terminal owner failure,
owner success/receipt-pending, accepted/unconsumed, consumed, and each selected
receipt-rejection state. It states all live facts and required predecessor
closure. These are required test/restore frontiers, not a reusable shared
lifecycle enumeration. It does not select serialization, global identity,
restore function, or persistence implementation.

### Observation gate, not a seventh lifecycle coordinate

Observation has no selected X1 projection. A candidate that claims telemetry,
audit, devtools, or another observer surface adds an explicit conditional gate
between C and L: subject occurrence or telemetry effect, observer authority,
view label, monotone redaction, retention/export surface, reason references,
and source-span linkage. Otherwise it makes no observation claim. Storage
never passes this gate by itself.

## Dependency and coupling graph

```
R ──> B ──> T ──> U
          │     │
          └────> C
R ─────────────────┐
B ─────────────────┤
T ─────────────────┼──> L
U ─────────────────┤
C ─────────────────┘

R <──────────────────── L
```

The feedback arrow is deliberate: R needs the scope in which a restored
reference is meaningful, while L needs the chosen semantic facts to preserve.
It is a design-review dependency, not a causal cycle or permission to reason
from a post-load result back into a request.

The coordinates are one tuple when a candidate relies on their links. The plan
does not impose an implementation sequence, object layout, or common
identifier. A candidate may leave a coordinate open only when no claimed row
depends on it.

## Alternative domains and escalation boundary

Within accepted X1, a candidate defines the explicit relation-valued exchange
component and its dynamic domain. It may compare only candidate-native,
erasable **presentations or definitions of that X1 relation**; a direct
projection is admissible only when it does not replace the relation's semantic
residence. It may not reopen history-only or nominal-identity families, use a
Plan 227 label as a shared predicate/data structure, or call another carrier
form an equal X1 candidate.

Escalate instead of completing a candidate card when it requires:

- a new Core primitive, `Config`/`SaveObject` schema, failure
  member, occurrence kind, causal generator, source/elaboration form, or
  public/wire/serialization contract;
- a change to authority semantics, an algorithm that treats claims as
  authority, a new public observer surface outside P017's conditional
  theory/07 projection gate, SCN/Gate/Phase, or theory/11; or
- a final proof/OBL/conformance/runtime claim, liveness, fairness, retry,
  timeout, cancellation, cache/result/snapshot freshness beyond the mandatory
  M1 epoch/incarnation/lineage checks, atomic read-modify-write, or global
  exactly-once claim.

These are not negative results. They identify the ordinary Canon amendment
surface that a concrete candidate must enter. A bounded proof argument,
countermodel, or explicit stop result for an X1 package row is permitted and
required by P017; it is not an escalation by itself. Likewise, a candidate
claiming observation must define P017's conditional typed theory/07 projection
instead of treating storage as observation.

## Choice-neutral adversarial-case matrix

The matrix is a review checklist over source-backed distinctions. It neither
predicts a transition nor proves an implementation.

| Adversarial case | Required distinction or forbidden conclusion | Coordinates that must close it |
| --- | --- | --- |
| Equal incidental data for two requests | payload, span, M1, queue, and transport equality do not share effective pending/result/receipt/consumption state | R, B, T, U |
| One source site executes twice | dynamic emissions remain distinct; span is diagnostic provenance only | R, C, L |
| Two principals share requester locus | neither locus nor session/transport collapses authority or correlation | R, B |
| Copied/replayed/stale/wrong/missing request | validation fails closed with no owner mutation and no matching success continuation | B, C, L |
| Competing owner success and owner failure | at most one terminal owner branch | B, C, L |
| Owner success but no accepted receipt | no consumption is enabled, including after save/load | T, U, L |
| Competing semantic receipt acceptances | at most one transition changes a successful branch to accepted | T, U, L |
| Accepted receipt saved before consumption | exactly one remaining consumption budget survives an admissible load | U, L |
| Consumed branch saved | no re-consumption or re-resumption follows | U, L |
| Duplicate/late/stale/wrong-locus delivery | no second accepted use; rejection semantics stays explicit or open | T, U, L |
| Similar failure outcomes | owner failure, no delivery, no receipt, requester rejection, static Diagnostic, and load refusal remain distinct | B, T, L |
| Claimed causal edge has no source | map it to theory/04 or escalate a new generator; no causal cycle is admitted | C |
| Incomplete saved closure | load cannot lose, duplicate, merge, revalidate, or stale-resurrect a branch | R, B, T, U, C, L |
| Stored exchange facts | storage alone grants no observation | observation gate |
| Owner interleaving before later use | no freshness, snapshot, or fused read-modify-write claim is inferred | C, scope exclusions |
| Owner-local read | no X1 exchange lifecycle is introduced | R, C |

## Blank candidate-native card contract

A later comparison card has this shape:

```text
C + H_K + D_K

C   = exact pinned Canon clauses
H_K = candidate-local hypothesis delta, labelled Canon-native, LAB hypothesis,
      OPEN, or carrier gap
D_K = erasable definitions over C + H_K only
```

The card defines its own roles and linkage formulae. It does not inherit a
shared `Interaction`, `Pending`, attempt, request key,
identifier, lifecycle enum, common `q/p/r/t` factorization, or restore
function from this plan. A comparison may conclude only `conditionally
compatible`, `countermodel`, `open`, `Canon gap`,
or `out of scope`. It must not say that a candidate “satisfies Canon”
without an ordinary amendment and review.

For every claimed matrix row, the card records semantic strata, typed
success/failure observations, authority grounds, occurrence accounting, causal
basis, one-shot premise, claimed load frontier, restore correspondence, and
fallback scope. It also records its own authority cut, permitted existing LAB
locations, narrow question, status quo, alternative or expected falsifier,
decisive per-row falsifier, card-specific non-effects, rollback/reopen trigger,
and one of a bounded proof argument, countermodel, or explicit stop result for
every row on which it relies. Missing material is `open`, not a
convenient default.

## Completion, stop, and reopen conditions

This plan is complete when the source ledger, vector, dependency graph,
alternative boundary, adversarial matrix, and blank candidate contract are
present. No L3 record, Lean theorem, schema, sample, runtime run, or Canon
change follows from completion.

Before relying on a positive model, first register an ADR-0014-eligible L3
record in an existing LAB lane with the card-specific controls above. Its
bounded result may then support an ordinary Canon proposal; an amendment is
still required before normative adoption. Reopen this preparation record only
when P017 or a direct source changes, an independent reviewer finds a matrix
omission or hidden shared carrier, a concrete proposal supplies a new
source-backed requirement, or a reproducible defect is found in the cited
record. Do not reopen it to enumerate fixture permutations or relabel existing
restore evidence.

## Non-effects

This plan changes no Canon text, working record, Core, configuration,
`SaveObject`, history relation, request/result/receipt identity,
authority rule, failure row, causal family, observation surface, source grammar,
elaboration, runtime, adapter, wire/API, theorem/OBL, scenario, Gate, Phase,
sample, implementation, or public behavior.
