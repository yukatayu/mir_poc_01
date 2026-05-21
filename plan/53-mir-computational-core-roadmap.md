# plan/53 — Mir computational core roadmap

## purpose

This document is repository memory for the Mir Computational Core rebaseline defined by `specs/28-mir-computational-core.md`.

It preserves the Product Alpha-1 runtime / package / devtools / save-load / native host launch bundle work, while correcting the over-read that typed external `AddOne` proves Mir-owned computation.

## current recognition

Current alpha evidence includes a typed external host-I/O lane. That is useful and must be preserved.

It does not yet show:

- Mir-owned arithmetic expression typing and execution.
- variables and lexical scope in Mir-owned source / typed IR.
- arrays, records, and control-flow with positive and negative rows.
- separation between pure computation and effectful host boundary calls in checker / runtime evidence.
- compiler/backend readiness.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-COMP-00` | recognition rebaseline | `specs/28`, `plan/53`, snapshot docs, non-claims, and report are synchronized; no implementation claim |
| `P-COMP-01` | spec and sample scaffold | sample matrix exists, rows are classified as planned vs executable, current AddOne non-claim is explicit |
| `P-COMP-02` | pure AddOne in Mir | host input / Mir compute / host output are separate observable events |
| `P-COMP-03` | variables / arrays / records / control-flow first floor | each feature has positive and negative machine-readable rows |
| `P-COMP-04` | effect boundary around internal computation | pure/effect split and declared effect / failure / capability rejection are checked |

## current sample matrix

`P-COMP-01` actualized the scaffold, and `P-COMP-02` promoted the first executable row. The current roots are:

- `samples/product-alpha1/computational/add-one-pure-mir/`
- `samples/product-alpha1/computational/variables-scope/`
- `samples/product-alpha1/computational/arrays-bounds/`
- `samples/product-alpha1/computational/records-vec3/`
- `samples/product-alpha1/computational/control-flow/`
- `samples/product-alpha1/computational/imports-functions/`
- `samples/product-alpha1/computational/host-io-internal-transform/`

Current helper/runtime anchors:

- `scripts/mir_computational_samples.py`
- `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json`
- `samples/product-alpha1/computational/expected/add-one-pure-mir.expected.json`

Current validation anchors:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
```

These commands validate matrix classification and prove one bounded Mir-owned runtime execution. They do not yet prove variables / arrays / records / control-flow or the effect-boundary packages.

## typing and failure-row roadmap

The pure fragment should first use a narrow typing judgment:

```text
Σ ; Γ ⊢ e : A ⇝ C
```

The effectful fragment must include both effect and failure rows:

```text
Σ ; Ψ ; Γ ; Δ ⊢ t : A @ μ ! ε ? ρ ⇝ C ; O
```

The old sketch without `ρ` is treated as incomplete. Future proof obligations must not state full preservation/progress for the effectful fragment until `ε` and `ρ` are implemented and checked together.

## feature floor

First floor:

- `Bool`, `Int64`, `UInt64`, `Float64`, `Text`, `Unit`
- `let`, `mut`, lexical scope
- `if/else`, `match`, `while`, `for`, `return`, block
- `fn`, effectful `fn`, module, import
- tuple, record/struct, enum/variant
- fixed array, vector, indexing, length, iteration
- arithmetic, comparisons, boolean operators, conversions

Safety floor:

- no raw pointer by default
- no unchecked pointer arithmetic
- no hidden global mutable state
- bounds checked arrays
- declared failure rows for runtime rejects

## future widening

Later, after first-floor evidence:

- ownership / borrowing-like discipline
- trait / interface / typeclass-like capability, limited and explicit
- richer pattern matching
- bounded generics
- async / effect handling
- explicit schema-backed FFI

## stop lines

- Do not claim final textual `.mir` grammar.
- Do not claim Rust-level expressive power is complete.
- Do not claim direct LLVM/native backend.
- Do not claim final server/client split.
- Do not claim current alpha `AddOne` proves Mir-owned computation.

## current recommendation

`P-COMP-01` and `P-COMP-02` are now closed. The next computational implementation package is `P-COMP-03`, while broader distribution / final shared-space catalog decisions remain user-spec-required gates and do not block the current implementation queue.
