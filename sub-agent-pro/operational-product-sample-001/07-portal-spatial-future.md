# 07 — portal and spatial future boundary

## 1. Portal first

Portal / WorldLink is the near-term future layer. It is analogous to a WWW hyperlink.

Portal is discrete:

```text
World A --PortalRef--> World B
```

This is easier and safer than continuous spatial federation.

## 2. Portal model

```text
Portal {
  portal_id,
  from_world,
  from_anchor,
  to_world,
  to_anchor,
  required_capability,
  membership_policy,
  handoff_contract,
  fallback_target
}
```

Portal transition:

```text
source: request_leave_or_handoff
source: publish portal_handoff_offer
source: create witness
transport: route portal admission request
destination: validate membership/capability/witness
destination: admit participant or reject
devtools: portal route trace
```

## 3. Portal sample boundary

In P-OPS-01, portal can be planned/skeleton only.

Create:

```text
samples/product-alpha1/operational/future/portal-worldlink/README.md
samples/product-alpha1/operational/future/portal-worldlink.package.mir.json
```

The README should say:

- portal is future/near-term
- not implemented as continuous federation
- no WAN claim
- no final portal ABI
- uses same MessageEnvelope / membership / capability / witness lanes when implemented

## 4. Spatial shard future

Continuous infinite world is possible as a Mirrorea upper layer, but not current implementation.

Recommended sequence:

```text
S1: Portal / WorldLink
S2: finite two-shard hard authority boundary
S3: gradient observation boundary
S4: ghost / replica handoff
S5: optional CRDT / dotted-vector replication profile
S6: WAN federation / distributed durability
```

## 5. Spatial model

```text
Shard_i = {
  shard_id,
  authority_region,
  observation_region,
  boundary_region,
  config_epoch,
  membership_frontier
}
```

Distinguish:

- authority region = who can write / own / simulate
- observation region = who can see / receive approximated state

Boundary may look smooth, but authority should be hard at first.

## 6. Object ownership

Use single-owner first.

```text
ObjectStamp = {
  owner_shard,
  owner_epoch,
  sequence
}
```

Do not use global participant vector clock by default.

Membership remains:

```text
membership_epoch
member_incarnation
```

Hot-plug / shard config uses:

```text
config_epoch
activation_cut_ref
```

## 7. Handoff protocol

First finite shard protocol:

```text
HandoffOffer(object, from=A, to=B, owner_epoch=e)
HandoffPrepare(object, B)
HandoffCommit(object, owner=B)
HandoffAck(object, A)
```

Static/model-check obligations:

- no double owner
- old owner write rejected after commit
- missing handoff witness rejected
- stale config epoch rejected
- observer-safe ghost state does not grant write capability

## 8. Replication profiles

Default profile:

```text
SingleOwnerSequence
```

Next profile:

```text
OwnerEpochSequence
```

Optional future:

```text
CRDTJoinSemilattice
DottedVersionVector
IntervalTreeClock
```

Do not implement optional profiles in P-OPS-01.

## 9. Static verification

Checker should eventually validate:

- object write requires ShardAuthority(region)
- observer only receives allowed labels
- handoff requires witness / capability / config freshness
- replication profile declared
- failure row includes StaleShardConfig / RouteUnavailable / MissingHandoffWitness

## 10. Model-check samples to plan

- 2 shards, one object, boundary crossing, no double owner
- handoff offer lost, no duplicate owner
- stale config during crossing, old handoff rejected
- observer near boundary sees ghost but cannot write

## 11. Stop line

P-OPS-01 must not claim:

- continuous infinite federation implemented
- WAN / federation implemented
- distributed durable save implemented
- CRDT/vector-clock replication implemented
- spatial shard runtime completed
