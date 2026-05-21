# 09 — Membership / Clock / Compaction

## Default design

Do not use global participant vector clocks as default membership freshness mechanism.

Use:

```text
membership_epoch
member_incarnation
config_epoch
owner_epoch + sequence for objects when needed
```

## Join

```text
Join(p):
  membership_epoch += 1
  incarnation[p] = current or next
  active[p] = true
  emit admission witness
```

## Leave

```text
Leave(p):
  membership_epoch += 1
  active[p] = false
  tombstone(p, incarnation, retired_epoch)
```

## Historical mention

A message mentioning `C` must not resurrect C.

Only an explicit join/admission witness can introduce active C.

## Can forget rule

```text
CanForget(p, incarnation) iff
  no in-flight message references it
  no live witness references it
  no live lease/fallback references it
  no retained savepoint references it
  audit retention policy permits deletion
  all relevant frontiers passed the leave event
```

## Vector-clock optional profile

Vector clocks, dotted version vectors, interval tree clocks, or CRDT clocks are optional replication profiles for future multi-writer replicated state.

They are not default participant membership mechanism.

## Samples

- stale membership reject
- historical mention does not join
- join requires admission witness
- leave tombstone retained until retention frontier
- stale config reject in shard handoff
