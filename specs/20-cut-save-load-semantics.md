# 20 — Cut / Save / Load Semantics

## role

この文書は、`atomic_cut` / consistent cut / save-load / checkpoint / Z-cycle を、
α-0.5 / α-0.8 / α-0.9 operational readiness のために
**event-DAG-centered semantics** として固定する。

- `atomic_cut` の place-local boundary を保つ
- local-only save/load と distributed durable save/load を分ける
- stale membership / witness / lease resurrection を禁止する

## decision level

- `L1`
  - event history は DAG discipline を保つ
  - `atomic_cut` は place-local rollback frontier である
  - consistent cut は causal-order prefix closure を要する
  - load admissibility は stale fact resurrection を許さない
- `L2`
  - save object minimum carrier
  - activation cut / membership / witness / auth frontier closure
  - local-only / distributed durable split

## event DAG and causal order

execution history は event DAG として読む。

```text
E : set of events
≺ : strict partial order on E
```

minimum event family:

- local transition
- queue enqueue / dequeue
- send / receive of `MessageEnvelope`
- publish / observe
- witness create / witness use
- capability grant / capability use
- auth evidence create / auth evidence use
- membership update
- fallback degrade
- attach request / attach verdict / activation cut
- `atomic_cut`
- save request / load request
- external effect / compensation

minimum causal family:

- `program_order`
- `state_dependency_order`
- `send -> receive`
- `publish -> observe`
- `witness_create -> witness_use`
- `capability_grant -> capability_use`
- `auth_evidence_create -> auth_evidence_use`
- `membership_update -> dependent dispatch`
- `attach_request -> attach_verdict -> activation_cut`
- `fallback_degrade -> later access on same lineage`
- `atomic_cut(place) -> later local transition on same place`

## `atomic_cut` retained boundary

`atomic_cut(place)` は
current `place` の rollback frontier を確定する。

それは次ではない。

- distributed commit
- durable checkpoint
- global synchronization point
- low-level memory fence principal

minimum property:

```text
If atomic_cut(p) is in history,
rollback at p cannot remove events causally before that cut within p.
```

## consistent cut

cut `K ⊆ E` is consistent iff `K` is prefix-closed under causal order.

```text
Consistent(K) := ∀e ∈ K. ∀e'. e' ≺ e => e' ∈ K
```

minimum consequences:

- `receive ∈ K` implies `send ∈ K` or explicit channel state carries it
- `observe ∈ K` implies `publish ∈ K`
- `witness_use ∈ K` implies `witness_create ∈ K`
- `capability_use ∈ K` implies capability grant / admission frontier is in `K`
- `auth_evidence_use ∈ K` implies auth-evidence create / provenance frontier is in `K`
- `activation_cut ∈ K` implies attach request and verdict are in `K`
- membership-dependent dispatch in `K` implies relevant membership frontier is in `K`

## save object

save は byte copy ではなく cut-backed object である。

minimum carrier:

```text
SaveObject = {
  cut,
  causal_frontier,
  place_states,
  queues,
  in_flight_messages,
  membership_registry,
  place_catalog,
  capability_state,
  auth_evidence_store,
  witness_store,
  lease_store,
  fallback_positions,
  hotplug_lifecycle_state,
  package_versions,
  adapter_state,
  external_effect_obligations,
  provenance
}
```

local-only α-0.5 floor は subset 実装でよいが、
full shape は spec 上で visible に保つ。

## load admissibility

load may succeed only if:

```text
Consistent(cut)
No rollback across atomic_cut
No stale membership resurrection
No stale witness resurrection
No expired lease resurrection
Capability/auth provenance remains connected
Package versions are compatible
External irreversible effects are compensated or isolated
```

current local-only floor が証明してよいのは subset に限る。

## no stale membership / witness / lease resurrection

load / rollback は次を hidden に revive しない。

- expired lease
- stale witness
- stale membership epoch / participant incarnation
- severed capability provenance
- severed auth evidence provenance
- degraded fallback position

reacquire が必要なら、
new event / new epoch / new evidence として explicit に扱う。

## Z-cycle / useless checkpoint boundary

distributed checkpoint graph には
recoverable global cut を作れない useless checkpoint がありうる。

current rule:

```text
ZCycle(c) => checkpoint c is inadmissible for recoverable global cut
```

`CUT-11`-style checker evidence は structural guard であり、
distributed recovery protocol completion ではない。

## activation cut and checkpoint boundary

hot-plug activation は cut semantics の一部だが、
durable migration や distributed activation ordering と同一ではない。

- activation cut が cut に入るなら request / verdict / relevant membership frontier も必要
- deferred detach boundary は accepted detach execution と同一視しない
- runtime package version/admission state は save object で explicit に持つ

## memory-order profile boundary

weak-memory / `memory_order_*` profile は
model-check / profile line に属する。

- `atomic_cut` を low-level memory fence と再解釈しない
- source semantics principal は high-level causal relation である
- memory-order profile と save/load admissibility を混同しない

## local-only save/load vs distributed durable save/load

```text
local-only save/load
  = one or bounded local frontier, explicit local carrier, no global durability claim

distributed durable save/load
  = consistent distributed cut, channel/in-flight representation,
    persistence contract, replay / recovery story, activation/version closure
```

current α-0.5 / practical first floor で admissible なのは前者である。
後者は kept later である。

## soundness targets

- accepted local save/load restores a valid local frontier
- load does not resurrect stale membership
- future rows must prove no stale witness / lease resurrection
- invalid distributed cut is rejected before runtime checkpointing
- rollback does not cross `atomic_cut`

## required anchors

current freeze で visible であるべき row family:

- `CUT-04`
- `CUT-11`
- `CUT-17`
- `SL-A1-01`
- `SL-A1-02`
- `SL-A1-03`
- `VIS-A1-03`
- `PE2E-06`

future required row family:

- stale witness non-resurrection
- stale lease non-resurrection
- communication-induced checkpoint repair
- session-bound save/load timeline export

## deferred

- production distributed snapshot algorithm
- durable cut implementation
- durable replay service
- consensus mechanism
- distributed migration / activation ordering protocol
- final storage backend choice

## stop line

- `atomic_cut` を durable/distributed commit と書かない
- local save/load を distributed durable save/load と書かない
- Z-cycle structural reject を distributed recovery completion と書かない
- memory-order profile と source semantics principal を混同しない
- exact preflight reject row を runtime checkpoint execution と書かない
