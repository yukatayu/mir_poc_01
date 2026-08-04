---
id: theory/14-maintained-relation-projection
status: L1-fixed
maturity: reviewed
depends_on: [theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, theory/06-existence-fallback, theory/07-observation, theory/09-two-layer-time, theory/13-evaluation-materialization, adr/ADR-0019]
summary: M4 maintained relation、owner-held binding、semantic/presentation fallback 分離、consumer-local late projection の有限 calculus。
open_items: [OPEN-033]
---

# 14 — Maintained relation and late projection

This chapter defines the M4 extension of the shared Core universe. It is
syntax-independent. `RelationDef`, `BindingState`, and projection are
domain-neutral semantic carriers; application vocabulary appears only in
SCN-12.

## 1. Relation and binding state

For declared finite reference keys, a maintained relation is:

```text
RelationDef = ⟨relation-ref, owner, subject-ref,
               primary-anchor-ref, fallback-anchor-ref, relative-transform,
               relation-policy, relation-label⟩
BindingState = ⟨relation-ref, selected-option, lineage, binding-epoch,
                witness-epoch, activation-frontier, primary-anchor-epoch,
                fallback-anchor-epoch⟩
```

For compact M4 notation, `G[rid]` names this semantic relation/binding entry
held in theory/01's new `J` component; it is not the pre-existing capability
store `G`. `G[rid]` is owner-held state. The finite selector is exactly one
top-level guarded `primary > fallback` choice over this finite pure acyclic
relation graph. Nested selectors, relation rewrites, and arbitrary DAG
composition are deferred by OPEN-033.

`RelationDef.owner` alone mutates `BindingState`. The relation dependency
graph is acyclic. A binding is live only when its owner authority, lineage,
witness, selected-anchor existence/membership/lease, and activation frontier
are live. A consumer-side request to mutate relation definition or binding is
rejected; receiving a relation is neither a capability nor owner authority.
Every owner-side semantic binding mutation validates the current owner
capability and witness lineage for the exact relation and binding epoch before
mutation; a missing, stale, or mismatched validation rejects with no binding
state change.

The relation's semantic fallback options are primary then fallback. Their
guards use theory/06's same-lineage floor. `degrade` is an owner occurrence
that advances the selected option at its recorded lineage/frontier. It is
causally before later accesses on that lineage. `reacquire` is an explicit
owner occurrence requiring fresh witness and binding epoch; it begins a new
lineage. No semantic step re-selects an earlier option in the same lineage.

## 2. Relation publication and local evaluation

The owner may materialize an admitted `ProjectedRelation` with M3
`publish-relation`, never `publish-value` or `adapter-stream` for the derived
absolute value:

```text
ProjectedRelation = ⟨relation-ref, selected-option, selected-anchor-ref,
                     selected-anchor-epoch, required-anchor refs/epochs,
                     relative-transform,
                     binding-epoch, activation-frontier, relation-label⟩
```

The consumer's M3 plan is `relation / consumer(C) / presentation-frame /
caller(C) / {local-only}`. It evaluates a projected relation only with:

```text
PresentationContext = ⟨consumer, frontier,
                       admitted primary/fallback anchor samples,
                       sample anchor-epochs, local presentation policy⟩
```

Each sample in `PC` needs its own release/admission for C, including its
anchor, epoch, frontier, and label, before C may evaluate the relation. A
sample admitted for one anchor/context is not admission for another.

`Coherent(R,B,PC)` requires the relation to be acyclic, `PC.frontier` to equal
`B.activation-frontier`, every required sample to name that frontier, and each
sample anchor/epoch to match B. Thus one local interpretation never combines
the relation's anchors from distinct frontiers. On coherence,
`project then evaluate ≃ evaluate then project` for the finite exact relative
transform fragment. This is a relation preservation law, not authorization to
send an absolute derived value to C.

The finite relative-transform domain is checked before evaluation. A transform
or derived coordinate outside that declared finite domain, including numeric
overflow, rejects; it does not wrap, saturate, or trigger fallback.

`B.activation-frontier` is the relation binding's causal/lineage anchor. It is
not M3 designated-result `EP.frontier?`: it does not name a designated
evaluator's canonical producer set, result version, or semantic consumption.
Likewise let `Q_C` denote C's local `PresentationContext`. `Q_C` is not
`G[rid]`: it is an ephemeral read-side frame input, cannot mutate relation
state, and is not an occurrence, authority carrier, or persistence side state.

## 3. Two fallback domains

Semantic invalidation is limited to selected-anchor existence, membership or
incarnation, lease, authority, witness, or relation-lineage loss. It produces
the owner-recorded `degrade` occurrence and moves only primary → fallback.
Fresh explicit reacquire is the only route to a new primary binding.

Presentation invalidation is a missing sample, stale epoch, sample-loss,
latency-budget exhaustion, interpolation/prediction limitation, LOD, or split
frame. It produces a consumer-local no-frame/renderer response and optional
typed audit observation. It does **not** mutate `BindingState`, create a
semantic fallback occurrence, change authority/lineage, or make a stale sample
semantically admissible.

## 4. Restriction propagation and rejects

For the M4 finite profile, restrictions are the decidable total order:

```text
public < restricted < private
```

`release-label(R, PC)` for an evaluated/observed derived result is the greatest
restriction of the projected relation label and every admitted input sample
label. In this total finite order, that greatest restriction is the finite join
over the relation label and **every** admitted anchor input. A requested
consumer release must be at least that restrictive; a private-to-public request
rejects. This finite chain does not select the general label lattice deferred
by OPEN-020.

The following reject before an admitted relation evaluation or release:

```text
relation dependency cycle
missing or stale selected-anchor sample/epoch
split-frame presentation context
consumer-originated RelationDef or BindingState mutation
requested label weaker than release-label
relative-transform or derived-coordinate overflow
```

These rejects are M4 semantic categories, not frozen M6 diagnostic spellings.

## 5. Configuration, causality, and persistence boundary

Theory/01 `Config` holds the owner relation/binding store. Its occurrence DAG
adds `relation-publish → consumer projection`, `binding-degrade → later same-
lineage access`, and `reacquire → new-lineage access`; the graph remains
acyclic. Theory/04 SaveObject provenance includes every live relation/binding
record, its activation frontier, epochs, witness/authority provenance, and
selected fallback position. Presentation samples and a temporary presentation
gap are not semantic binding state and are not a claim of per-sample save/load.
This field-presence rule is not a general M4 save/load theorem or implementation.

## 6. Proof boundary

The finite obligations in theory/11 target: relation projection coherence,
semantic fallback/no automatic re-promotion/fresh lineage, presentation-gap
nonmutation and context coherence, greatest-restriction propagation, and the
finite rejection set. OBL-005--008 and OBL-022--023 remain the general
fallback/two-layer-time obligations and are intentionally deferred.

OPEN-033: the finite profile has two ordered anchors. Arbitrary acyclic
relation-DAG composition, approximation, and a general label lattice remain a
later shared-model question.
