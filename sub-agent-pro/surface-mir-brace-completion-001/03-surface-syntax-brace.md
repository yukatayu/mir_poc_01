# 03 — Surface Syntax With Braces

## 1. Canonical syntax

Place / role / location block:

```mir
S {
  ...
}
```

Role instance block:

```mir
Participant[self] {
  ...
}
```

Indexed state access:

```mir
player[self]
array[i]
pose[frame]
```

No place scope with `[]`.

## 2. Example: server-owned per-participant player state

```mir
module Game.Attack

role Participant
place S

record Player {
  hp: Int64,
  atk: Int64,
}

S {
  state player[p: Participant]: Player
    init Player { hp: 100, atk: 10 }
}

Participant[self] {
  when attack(target: Participant) {
    S {
      player[target].hp -= player[self].atk
    }
  }
}
```

## 3. Meaning

- `S { ... }` sets current evaluation / declaration locus to S.
- `state player[p: Participant]: Player` declares S-owned indexed state.
- `Participant[self] { ... }` declares behavior at participant role instance `self`.
- Cross-place action inside `S { ... }` is elaborated into message/effect request.

## 4. Syntax groups

### Module

```mir
module Path.Name
import Other.Module
```

### Role

```mir
role Participant

role BrowserClient {
  supports renderer.pose_v1
  supports devtools.observer_safe
}
```

### Principal

```mir
principal self
```

### Place

```mir
place S
place WorldAdmission
```

### Record

```mir
record Player {
  hp: Int64,
  atk: Int64,
}
```

### Record literal

```mir
Player { hp: 100, atk: 10 }
```

### State

```mir
S {
  state player[p: Participant]: Player
    init Player { hp: 100, atk: 10 }
}
```

### Function

```mir
fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
```

### Event behavior

```mir
Participant[self] {
  when attack(target: Participant) {
    S {
      player[target].hp -= player[self].atk
    }
  }
}
```

### Effect boundary

Explicit expert syntax remains allowed:

```mir
draw <- perform roll_dice via authority_rng
```

But ordinary surface programs should not have to write every communication edge.

## 5. Grammar principles

- `[]` is for indexing only.
- `{}` is used for blocks and record literals.
- Place block is resolved by namespace/context.
- Type names and place names cannot collide in the same scope in alpha.
- If ambiguous, produce diagnostic.

## 6. Diagnostics

If user writes:

```mir
S[
  ...
]
```

produce:

```text
surface_place_scope_bracket_not_supported:
  use `S { ... }`; `[]` is reserved for indexing.
```

If user writes ambiguous:

```mir
S { hp: 100 }
```

and `S` is both place and type or unresolved, reject with:

```text
ambiguous_brace_construct
```
