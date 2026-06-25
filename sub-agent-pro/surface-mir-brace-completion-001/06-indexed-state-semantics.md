# 06 — Indexed State Semantics

## 1. Canonical declaration

```mir
S {
  state player[p: Participant]: Player
    init Player { hp: 100, atk: 10 }
}
```

## 2. Mathematical reading

```text
player_S : Active(Participant, epoch) ⇀ Player
```

This is a partial map owned by S.

## 3. Components

```text
owner_locus = S
state_name = player
keyspace = Participant
key = p
value_type = Player
```

## 4. Key is not owner

`player[Alice]` is stored at S.
Alice is only the key.

Therefore:

```text
Alice being key does not grant write authority.
```

## 5. Access rules

Read:

```text
requires key active or historical read mode
requires observer permission if current locus != owner
```

Write:

```text
requires current locus == owner OR write capability
requires active key
```

## 6. Join lifecycle

```text
join(p):
  membership_epoch += 1
  incarnation[p] becomes current
  active[p] = true
  allocate/init indexed entries for p
```

## 7. Leave lifecycle

```text
leave(p):
  membership_epoch += 1
  active[p] = false
  mark indexed entries retired / tombstoned
```

Do not immediately drop them.

## 8. Compaction rule

May drop entry only if:

```text
no in-flight message references p
no live witness references p
no live lease/fallback references p
no retained savepoint references p
audit retention policy permits deletion
```

## 9. Dynamic keyspace constraints

Alpha supports:

```text
role keyspace: Participant
object keyspace: Object, later
avatar keyspace: Avatar, later
```

Alpha does not support arbitrary unconstrained maps as storage-owner indexed state.

## 10. DAG / dependency graph

Indexed state dependencies are checked at schema level.

Allowed:

```text
score[p] depends on player[p]
ranking depends on score[*]
```

Danger:

```text
score[p] depends on ranking
ranking depends on score[*]
```

This can create cycle and must be rejected or residual model/proof obligation.

## 11. Devtools

Devtools must show:

- owner.
- keyspace.
- active keys.
- retired/tombstoned keys.
- access source spans.
- generated communication for cross-locus access.
