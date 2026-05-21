# 28 — Mir Computational Core

## role

この文書は、Product Alpha-1 で得た runtime / package / devtools / save-load / native host launch bundle の運用床を保ったまま、次に直すべき中核を **Mir-owned computation** として定義する。

current alpha line の typed external `AddOne` は有用な host-boundary evidence である。しかし、それは Mir が arithmetic / variable / array / record / control-flow computation を source / typed IR / runtime / compiler target として所有している証拠ではない。この文書はその drift を修正する。

## decision level

- `L1`
  - Mir は最終的に Rust-level expressive な実用言語へ向かう。
  - current next promoted docs/spec line は broader distribution ではなく Mir computational core rebaseline である。
  - stdio / engine / native provider は core computation owner ではない。
- `L2`
  - first computational floor は C-like baseline から始める。
  - Rust-like ownership / borrowing / traits / generics / async は段階的に後続で扱う。
  - final textual `.mir` grammar はまだ固定しない。

## relationship to existing layer model

`specs/03-layer-model.md` の global layer numbering は置き換えない。handoff で整理された次の line は、既存 layer を横断する **realization strata** として読む。

```text
S0 External substrate
S1 Mir Computational Core
S2 Mir Effect / Contract Core
S3 Verification Layer
S4 Mirrorea Runtime / Fabric
S5 Projection / Deployment / Compiler Boundary
S6 Host / Client / Backend
S7 Application / Spatial World
```

`S1` は既存 `L1 — Mir Core` の computational sublayer であり、`S4` は既存 `L2 — Mirrorea Fabric` に対応する。`S5` 以後は projection / backend / application boundary を説明するための realization view であり、global architecture の renumbering ではない。

## first computational floor

最初の floor は、raw pointer / unchecked memory / hidden global mutable state を持たない C-like subset とする。

必要な source / typed IR vocabulary:

- primitives:
  `Bool`, `Int64`, `UInt64`, `Float64`, `Text`, `Unit`
- variables:
  `let`, `mut`, lexical scope
- control:
  `if/else`, `match`, `while`, `for`, `return`, block
- functions and modules:
  `fn`, effectful `fn`, module, import
- compound data:
  tuple, record/struct, enum/variant
- arrays:
  fixed array, vector, indexing, length, iteration
- computation:
  arithmetic, comparisons, boolean operators, conversions
- Mir effect surface:
  `perform`, `require`, `ensure`, `publish`, `observe`, `witness`, `handoff`
- resource discipline:
  no raw pointer by default, no unchecked pointer arithmetic, no hidden global mutable state

## pure and effectful split

Pure computation and effectful boundary calls are separate.

Pure fragment:

```text
Σ ; Γ ⊢ e : A ⇝ C
```

Effectful fragment:

```text
Σ ; Ψ ; Γ ; Δ ⊢ t : A @ μ ! ε ? ρ ⇝ C ; O
```

where:

- `Σ` is type / module / package environment
- `Ψ` is place / observation / projection environment
- `Γ` is lexical variable environment
- `Δ` is capability / resource / effect precondition environment
- `μ` is execution locus / mode context
- `ε` is effect row
- `ρ` is failure / reject row
- `C` is contract or residual obligation carrier
- `O` is observation / export obligation carrier

The handoff's earlier judgment sketch omitted an explicit failure row. This document fixes that by keeping `ρ` explicit. Preservation/progress claims for the full effectful fragment remain future proof obligations until `ε` and `ρ` are implemented and checked together.

`perform`, `publish`, `observe`, `witness`, `handoff`, host I/O, and `atomic_cut` are not pure-expression constructs. They enter only through the effectful layer and must carry declared effect / failure / capability obligations.

## corrected AddOne reading

Current alpha `AddOne` means:

```text
host input
  -> typed external adapter
  -> adapter-owned transform
  -> typed receipt / output
```

Future Mir-owned `AddOne` must mean:

```mir
fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}

transition main at HostPlace {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
}
```

The computation `x + 1` must be represented, typed, executed, and later compiled as Mir-owned computation. The external adapter may provide only `read_int` / `write_int` boundary behavior.

## arrays and bounds

Array and vector access must be safe.

- fixed-size arrays with constant in-range indices may be statically discharged in the finite Line 1 checker.
- dynamic vector or loop-dependent indexing must either carry a runtime checked reject path in `ρ` or emit a residual proof obligation.
- `IndexOutOfBounds` is not silently builtin; it is a declared failure-row member when the fragment uses that name.

## save/load and cut interaction

Computational state is not side state outside cut semantics.

Locals that survive into runtime state, arrays, records, module state, and effect boundary state must be carried through the existing `SaveObject` / consistent cut model from `specs/20-cut-save-load-semantics.md`.

The core rules remain:

- `atomic_cut` is place-local rollback frontier.
- R0 local save and R2 quiescent-save remain current actualized floors.
- R2 requires `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.
- R3/R4 distributed durable save/load remain later.
- load must not resurrect stale membership, stale witness, stale lease, stale fallback, or stale computational state.

## future Rust-like widening

The following are desired but staged:

- ownership / borrowing-like discipline
- traits / interfaces / typeclass-like capability, limited and explicit
- richer pattern matching
- bounded / staged generics
- modules and package visibility
- async / effect handling
- FFI with explicit schema, effect row, failure row, capability policy, observation policy, sandbox/native policy

## too-rich-for-now stop line

Mir is not currently being turned into:

- full Haskell typeclass ecosystem
- TypeScript structural universe
- F* dependent language
- arbitrary dependent source terms
- proof assistant surface language

Dependent proof obligations may exist in verifier / proof side layers, but they do not become arbitrary user-facing source terms in this package line.

## continuation boundary

Continuation support is `UNRESOLVED`.

Any future one-shot or multi-shot continuation must not enable replay, duplication, hidden rollback, or state resurrection across an already finalized `atomic_cut`. Multi-shot continuation is especially constrained: it may only capture unrestricted / copyable / replay-safe context, and must not capture linear resource, mutable state, one-shot continuation, open transport, irreversible external effect, or state past cut.

## package line and completion

`P-COMP-00` is the recognition rebaseline: this document, `plan/53`, snapshot docs, and non-claims are synchronized. It is docs/spec work only.

`P-COMP-01` closes only when:

- `specs/28` and `plan/53` exist and are indexed.
- sample matrix names exist and are classified as planned vs executable.
- current alpha `AddOne` is explicitly documented as host-boundary evidence only.
- no runtime completion or final grammar claim is made.

`P-COMP-02` closes only when:

- one `AddOne` path is demonstrably Mir-owned.
- host input, Mir compute, and host output are separate observable events.
- the adapter only carries typed boundary I/O.

`P-COMP-03` closes only when variables, arrays, records, and control flow each have positive and negative machine-readable rows.

`P-COMP-04` closes only when pure computation and effectful boundary calls are separated in checker / runtime evidence, and undeclared effect / failure / capability rows are rejectable.

## non-claims

This document does not claim:

- final textual `.mir` grammar
- Rust-level expressive power completed
- direct Mir-to-machine-code or LLVM backend
- final server/client binary split
- arbitrary native or WASM execution
- Unity / Unreal / renderer ownership of world semantics
- that current alpha `AddOne` proves Mir computational-core completion

