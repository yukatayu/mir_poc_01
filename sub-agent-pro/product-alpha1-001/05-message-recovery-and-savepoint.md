# Message failure, recovery, and savepoint semantics

## Why this matters

The user explicitly asked whether one can save a state where:

> all messages before the cut have arrived, and no messages after the cut have been sent.

That is stronger than ordinary consistent cut.

## Cut taxonomy

### R0 LocalSavePoint

Single runtime/session local state.

### R1 ConsistentSavePoint

Causally closed cut. In-flight messages may exist and must be saved as channel state.

### R2 QuiescentSavePoint

Causally closed cut plus:

```text
NoInFlight
AllPlacesSealed
NoPostCutSend
```

This is the user's stronger savepoint.

### R3 DurableQuiescentSavePoint

R2 plus durable persistence.

### R4 DistributedDurableReplaySavePoint

R3 plus durable outbox/inbox/replay/dedup protocol.

## Product α-1 requirement

Implement R0 and bounded R2 in controlled local/Docker scope.

Do not implement or claim R3/R4 unless truly done.

## MessageState

Define and use:

```text
Created
Queued
Sent
InFlight
Delivered
Observed
Acked
TimedOut
Dropped
Rejected
Retried
Compensated
```

## TransportContract

Define at least:

```text
BestEffort
AtMostOnce
OrderedPerSender
TimeoutBounded
DurableOutbox   # likely later
DurableInbox    # likely later
```

For product α-1, use explicit `BestEffort + TimeoutBounded` or local reliable queue for bounded demo.

## RecoveryPolicy

Define:

```text
Reject
Retry(max, backoff)
Fallback(target)
Compensate(action)
RequireAck
DurableOutbox
IdempotentReceive(key)
```

Product demo must show at least one negative or failure case:

- timeout/drop -> explicit reject or fallback;
- stale membership -> reject before mutation;
- missing witness/capability -> reject before mutation.

## Modal operational indices

Use as documentation/checker carriers, not full proof language:

```text
○ A  = later / after transition
□ A  = stable invariant / sealed or guaranteed condition
```

Examples:

```text
send(m) : ○ InFlight(m)
seal(place) : □ NoPostCutSend(place, epoch)
drain(channel) : □ NoInFlight(channel, epoch)
quiescent_save : □AllPlacesSealed ∧ □NoInFlight -> R2SavePoint
```

## Quiescent save protocol

Product α-1 controlled protocol:

1. BeginSave(epoch)
2. all participating places enter `Sealing`
3. application sends are rejected/deferred after seal
4. in-flight local/Docker messages are drained or explicitly failed
5. all places report SaveReady(epoch)
6. runtime checks NoInFlight
7. savepoint emitted
8. runtime resumes or remains paused

## Required report fields

```json
{
  "savepoint_class": "R2_Quiescent",
  "all_places_sealed": true,
  "no_inflight": true,
  "no_post_cut_send": true,
  "drained_messages": [],
  "failed_messages": [],
  "retained_frontier": "...",
  "non_claims": ["not durable distributed save/load"]
}
```

## Samples

- `SAVE-A1-01`: local save/load R0
- `SAVE-A1-02`: stale membership non-resurrection
- `SAVE-A1-03`: invalid distributed cut preflight reject
- `SAVE-A1-04`: quiescent save success R2 in local session
- `SAVE-A1-05`: post-seal send rejected/deferred
- `SAVE-A1-06`: in-flight not drained => quiescent save rejected
- `MSG-A1-01`: timeout/reject explicit recovery
- `MSG-A1-02`: retry or fallback explicit recovery

## Non-claims

- not durable distributed save/load;
- not WAN partition recovery;
- not exactly-once transport;
- not arbitrary crash recovery.
