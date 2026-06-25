# 07 — Auto Communication / Publish / Observe

## 1. Goal

User writes system-wide meaning.
The compiler generates communication.

Example:

```mir
Participant[self] {
  when attack(target: Participant) {
    S {
      player[target].hp -= player[self].atk
    }
  }
}
```

Generated:

```text
Participant attack event
MessageEnvelope to S
S-side state update
optional publish hp_changed
observer observe hp_changed
```

## 2. Auto communication is not hidden

It must appear in:

- Core IR.
- event DAG.
- MessageEnvelope trace.
- devtools source-link panel.

## 3. Read generation

If current locus differs from owner:

```text
generate read request / observe edge.
```

## 4. Write generation

If current locus differs from owner:

```text
generate write/effect request to owner.
```

If no authority, reject.

## 5. Auto publish

State declaration may mark fields visible.

```mir
S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}
```

Only visible fields are auto-published.

## 6. No implicit data leak

Private fields are never auto-published.

Bad:

```mir
visible observer_safe fields { secret_key }
```

should require stronger explicit policy or reject.

## 7. Failure rows

Auto communication can introduce failures:

```text
StaleMembership
RouteUnavailable
MissingCapability
VisibilityDenied
MissingWitness
```

They must be declared or rejected.

## 8. Interest / observers

Alpha can use simple observer set.
Later, interest management and spatial observation can refine this.

## 9. Publish/witness relation

Auto publish may produce devtools/audit event.
Contract-level witness requires explicit annotation or explicit policy.

## 10. Examples

Surface:

```mir
S {
  state score[p: Participant]: Int64
    visible observer_safe
}

Participant[self] {
  when gain(points: Int64) {
    S {
      score[self] += points
    }
  }
}
```

Generated Core concept:

```text
GainRequest(self, points) -> S
S.score[self] update
publish score_changed(self)
observe for authorized observers
```
