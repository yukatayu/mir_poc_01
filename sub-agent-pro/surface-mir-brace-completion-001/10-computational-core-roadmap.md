# 10 — Computational Core Roadmap

## 1. Target expression power

First target: C-like safe subset.

Must have:

- Bool / Int64 / UInt64 / Float64 / Text / Unit
- let / mut
- lexical scope
- records
- arrays / vectors
- indexing with bounds
- functions
- imports
- if / else
- match
- while
- for
- return
- arithmetic / comparison / boolean ops

Not first target:

- raw pointer
- pointer arithmetic
- goto
- unsafe memory
- full Rust borrow checker
- traits
- generics
- async
- F* style dependent programming

## 2. Current state

Existing computational rows prove first-floor Mir-owned computation, but not full language completion.

Need next:

- Surface `.mir` source examples.
- Parser support.
- Typed IR lowering.
- Interpreter execution.
- Diagnostics.
- Effectful integration.

## 3. Key samples

```text
COMP-SURF-01 pure add_one.mir
COMP-SURF-02 variables/scope
COMP-SURF-03 arrays bounds positive/negative
COMP-SURF-04 records Vec3
COMP-SURF-05 if/match/while/for
COMP-SURF-06 imports/functions
COMP-SURF-07 host read -> Mir compute -> host write
COMP-SURF-08 publish/observe integration
COMP-SURF-09 witness/handoff integration
COMP-SURF-10 state-indexed computation
```

## 4. Mathematical safety

Preservation target:

```text
If Γ ⊢ e : T and e -> e', then Γ ⊢ e' : T.
```

Progress target for pure subset:

```text
If ∅ ⊢ e : T, then e is value or e can step or yields declared runtime rejection.
```

Bounds safety:

```text
Array indexing either statically in range, or runtime rejects with declared failure.
```

Effectful separation:

```text
Pure expressions cannot perform effects.
Effectful statements carry effect/failure rows.
```
