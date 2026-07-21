# plan/53 — Mir computational core roadmap

## purpose

This document is repository memory for the Mir Computational Core rebaseline defined by `specs/28-mir-computational-core.md`.

It preserves the Product Alpha-1 runtime / package / devtools / save-load / native host launch bundle work, while correcting the over-read that typed external `AddOne` proves Mir-owned computation.

## current recognition

Current alpha evidence includes a typed external host-I/O lane. That is useful and must be preserved.

It now shows bounded separation between pure computation and effectful host boundary calls through one accepted direct host read/write row and three expected `check` rejections.

It does not yet show:

- broad publish / observe / witness / handoff effectful semantics.
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

`P-COMP-01` actualized the scaffold, `P-COMP-02` promoted the first direct executable row, and `P-COMP-03` widened the first floor. The current roots are:

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
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/host-io-internal-transform/positive --format json
```

These commands validate matrix classification, prove one bounded Mir-owned `add_one` runtime execution, prove helper-executable positive / runtime-rejection first-floor rows, and prove the bounded host read/write boundary package with accepted/check-rejection rows. They do not yet prove broad effectful semantics beyond that boundary.

### Execution-route boundary (2026-07-22 audit)

The matrix's `executable` label is intentionally broader than direct Product
Alpha runtime execution. `comp-02-pure-add-one` and positive `comp-04` enter
`mirrorea-cli run-local`, the Product Alpha session runtime, and the registered
Rust semantic evaluator. The five positive and five negative `P-COMP-03` rows
are evaluated by the existing Python helper's module-ID dispatcher; their
`package.mir.json` manifests and explanatory `.mir` files are not sent to the
Product Alpha Rust runtime through their fixture path. Separately, the existing
Rust runtime tests construct valid Product Alpha packages and directly execute
the five positive closed-registry modules and directly reject the five negative
modules; semantic tests also typecheck/evaluate the registered module forms.
This is useful first-floor evidence, but it is not the same claim as direct
package-runtime sample-fixture evidence.

The helper's `runtime_rejection` label is a matrix comparison category, not a
rejection-phase carrier. In the closed Rust registry, four P-COMP-03 negatives
reject during typechecking and only the array-bounds negative reaches evaluator
execution and yields `OutOfBounds`. Product Alpha currently wraps all five as
`MirCompute`; neither that wrapper nor the helper fixture path exposes the
static-versus-evaluation split. This does not select a future public diagnostic
or failure carrier. See `plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`.

Direct textual `.mir` input remains an explicit Product Alpha `check` /
`run-local` non-goal; this line uses versioned `package.mir.json` inputs. A new
direct fixture can be researched in the existing lane only while it remains a
non-production artifact and introduces no new helper, schema, CI/Make surface,
or production runtime implementation. Any such reserved expansion requires
owner/canon action. See
`plan/166-mir-computational-baseline-directness-audit.md`.

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

`P-COMP-01`、`P-COMP-02`、`P-COMP-03`、`P-COMP-04` are now closed. The next promoted implementation package is `P-POSE-02`, while broader distribution / final shared-space catalog decisions and broader computational effectful widening remain user-spec-required or later gates that do not block the current implementation queue.
