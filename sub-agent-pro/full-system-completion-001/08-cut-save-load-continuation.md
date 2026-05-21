# 08 — Cut / Save / Load / Continuation

## Atomic cut

`atomic_cut` is place-local rollback frontier.

It is not:

- distributed commit
- durable checkpoint
- global barrier
- memory fence

## Savepoint classes

```text
R0 LocalSavePoint
R1 ConsistentSavePoint
R2 QuiescentSavePoint
R3 DurableQuiescentSavePoint
R4 DistributedDurableReplaySavePoint
```

Full System V1 requires:

- R0 local save/load
- bounded R2 for controlled local/Docker demo
- explicit non-goal for R3/R4 unless implemented

## Consistent cut

```text
Consistent(K) := for all e in K, all causal predecessors of e are in K
```

Consequences:

- receive implies send
- observe implies publish
- witness use implies witness create
- activation cut implies request and verdict

## Quiescent save

R2 requires:

- Consistent subset implemented by carrier
- NoInFlight
- AllPlacesSealed
- NoPostCutSend

## Continuation issue

Multi-shot continuation is unresolved and dangerous.

It may duplicate:

- linear resource
- mutable state
- one-shot witness
- open transport
- external irreversible effect
- post-cut state

## Required rule if continuations are introduced

```text
MultiShotAllowed(k) iff
  captured_linear_context = empty
  captured_mutable_state = empty
  captured_effects subset replay_safe
  no one-shot witness
  no open transport obligation
  no irreversible external effect
  no state past atomic_cut
```

One-shot continuations may capture affine resources but must be consumed once.

Do not implement first-class continuation until this spec is written and checked.
