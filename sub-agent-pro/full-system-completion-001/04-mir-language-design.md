# 04 — Mir Language Design

## Design target

Mir must become a practical scripting/compiled language.

Short-term target:

- safe C-like baseline

Long-term target:

- Rust-level practical expressiveness

Non-target for now:

- Haskell-level typeclass ecosystem
- F* full dependent proof language
- full TypeScript structural universe
- theorem prover surface language

## Textual grammar MVP

The initial textual grammar may be alpha-only and not final, but must be real enough to author examples.

### Example: pure add_one

```mir
module Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
```

### Example: host boundary + Mir transform

```mir
module Computational.HostIoAddOne

import Computational.AddOne

effect read_int {
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  failure AdapterUnavailable
}

transition main at ComputationalHostPlace {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
    ensure y = x + 1
}
```

### Example: record / transform

```mir
module Pose.Basic

record Vec3 {
  x: Float64,
  y: Float64,
  z: Float64,
}

record Quat {
  x: Float64,
  y: Float64,
  z: Float64,
  w: Float64,
}

record Transform {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
  pose_version: UInt64,
}
```

### Example: array and bounds

```mir
module Computational.Arrays

fn sum3(xs: [Int64; 3]) -> Int64 {
  return xs[0] + xs[1] + xs[2]
}
```

## Required parser behavior

Parser must emit:

- AST
- spans
- diagnostics
- import refs
- syntax category

Do not make final keyword promises prematurely, but keep alpha grammar consistent.

## Typed IR minimum

IR nodes:

- Module
- Import
- Function
- Block
- Let
- Assign
- If
- Match minimal
- While/For minimal
- Return
- Call
- RecordConstruct
- FieldAccess
- ArrayConstruct
- Index
- Perform
- Publish
- Observe
- Witness
- Handoff
- AtomicCut

## Failure row explicitness

Every effectful operation has effect row `ε` and failure row `ρ`.

Do not hide failure in generic error.

## Built-in vs user-defined

Built-in:

- primitive types
- module/import/fn/let/mut/control syntax
- effect row/failure row machinery
- Place/effect/contract/core vocabulary

User-defined:

- specific labels
- authorities
- capabilities
- domain object types
- game rules
- avatar runtime names
- provider policies

## Roadmap

Phase 1:

- parser for computational samples
- typed IR
- interpreter

Phase 2:

- effectful operations
- runtime session integration

Phase 3:

- richer records/enums/modules
- limited generics/interfaces

Phase 4:

- compiler/projection backend
