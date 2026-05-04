# 05 — cut / save-load theory

## event DAG

実行は event DAG として扱う。

```text
E : set of events
≺ : causal order, strict partial order on E
```

event kinds:

```text
local_transition(place)
send(envelope)
receive(envelope)
publish(value)
observe(value)
create_witness(w)
use_witness(w)
fallback_degrade(ref, from, to)
attach_request(pkg)
attach_verdict(pkg)
activation_cut(pkg)
atomic_cut(place)
save_request
load_request
external_effect
compensation
```

causal order includes:

```text
program_order
send -> receive
publish -> observe
create_witness -> use_witness
attach_request -> attach_verdict -> activation_cut
membership_update -> dependent dispatch
fallback_degrade -> later access on same lineage
atomic_cut -> later local transition in same place
```

## atomic cut

`atomic_cut(place)` is a place-local rollback frontier.

It is not:

- distributed commit
- durable checkpoint
- global sync point
- memory fence

Property:

```text
If atomic_cut(p) is in history,
rollback at p cannot remove events causally before that cut in p.
```

## consistent cut

A cut `K ⊆ E` is consistent iff it is prefix-closed under causal order.

```text
Consistent(K) := ∀ e ∈ K. ∀ e'. e' ≺ e => e' ∈ K
```

Examples:

- `receive ∈ K` implies `send ∈ K` or channel state carries message
- `observe ∈ K` implies `publish ∈ K`
- `use_witness ∈ K` implies `create_witness ∈ K`
- `activation_cut ∈ K` implies attach request and verdict are in K

## save object

Save is not byte copy. It is a cut-backed object.

```text
SaveObject = {
  cut,
  place_states,
  queues,
  in_flight_messages,
  membership_registry,
  capability_state,
  witness_store,
  lease_store,
  hotplug_lifecycle_state,
  external_effect_obligations,
  package_versions,
  provenance
}
```

α-0.5 may implement a local-only subset, but the full shape must remain visible to avoid ad-hoc expansion later.

## load admissibility

Load may succeed only if:

```text
Consistent(cut)
No rollback across atomic_cut
No stale membership resurrection
No stale witness resurrection
No expired lease resurrection
External irreversible effects are compensated or isolated
Package versions are compatible
```

Current local-only floor can prove only a subset.

## Z-cycle / useless checkpoint

Distributed checkpoint graph must reject checkpoints trapped in Z-cycle.

```text
ZCycle(c) => checkpoint c is inadmissible for recoverable global cut
```

`CUT-11`-style checker evidence is a structural guard. It is not a distributed recovery protocol.

## memory-order profile boundary

`memory_order_*` profiles belong to model-check / profile line. They are not the principal source syntax for Mir core.

`atomic_cut` must not be reinterpreted as a low-level memory fence.

## soundness targets

- accepted local save/load restores a valid local frontier
- load does not resurrect stale membership
- future rows must prove no stale witness / lease resurrection
- invalid distributed cut is rejected before runtime checkpointing
- rollback does not cross `atomic_cut`

## required samples

- local save/load resume
- stale membership non-resurrection
- invalid distributed cut reject
- stale witness non-resurrection future row
- expired lease non-resurrection future row
- communication-induced checkpoint repair future row

## stop line

- local save/load = distributed save/load と書かない
- `atomic_cut` = durable cut と書かない
- Z-cycle structural rejection = distributed recovery completion と書かない

