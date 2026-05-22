# plan/59 — textual Mir roadmap

## purpose

This document is repository memory for `specs/34-textual-mir-alpha-grammar.md`.

## current state

Current `.mir` files under `samples/product-alpha1/` remain explanatory sketches. Current executable runtime input is still usually `package.mir.json`.

`P-MIR-01` actualized the first real textual Mir alpha source entrypoint:

- `crates/mir-ast::textual_alpha`
- `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`
- `scripts/textual_mir_samples.py check-all --format json`
- `samples/full-system-v1/computational/`

`P-MIR-02` now consumes that parser output into crate-local typed IR lowering and checker rows through `crates/mir-semantics::full_system_v1`, but the textual line itself remains alpha grammar evidence rather than final grammar freeze.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-MIR-01` | textual parser alpha | representative computational source files parse to AST with spans/diagnostics and the alpha negative matrix is covered |
| `P-MIR-02` | typed IR lowering | parsed AST lowers to typed IR and checker can accept/reject rows |
| `P-MIR-03` | interpreter source bridge | accepted typed IR executes safe C-like samples |
| `P-MIR-04` | effectful source bridge | `perform`, publish/observe, witness/handoff, fallback, and cut enter runtime explicitly |

## planned sample family

Current parser-floor source samples:

- `samples/full-system-v1/computational/add-one-positive/src/add-one.mir`
- `samples/full-system-v1/computational/host-boundary-positive/src/host-boundary-add-one.mir`
- `samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir`
- `samples/full-system-v1/computational/malformed-function-negative/src/malformed-function.mir`
- `samples/full-system-v1/computational/missing-type-annotation-negative/src/missing-type-annotation.mir`
- `samples/full-system-v1/computational/malformed-record-negative/src/malformed-record.mir`
- `samples/full-system-v1/computational/malformed-perform-negative/src/malformed-perform.mir`
- `samples/full-system-v1/computational/malformed-transition-negative/src/malformed-transition.mir`
- `samples/full-system-v1/computational/malformed-capability-negative/src/malformed-capability.mir`
- `samples/full-system-v1/computational/contract-clause-position-negative/src/contract-clause-position.mir`

Current widened source rows after `P-MIR-02`:

- `samples/full-system-v1/computational/record-field-positive/src/record-field.mir`
- `samples/full-system-v1/computational/imported-semantic-negative/src/imported-semantic-negative.mir`
- `samples/full-system-v1/computational/duplicate-module-path-negative/src/duplicate-module-path.mir`
- `samples/full-system-v1/computational/type-mismatch-negative/src/type-mismatch.mir`
- `samples/full-system-v1/computational/scope-unbound-negative/src/scope-unbound.mir`
- `samples/full-system-v1/computational/static-array-bounds-negative/src/static-array-bounds.mir`
- `samples/full-system-v1/computational/undeclared-effect-negative/src/undeclared-effect.mir`
- `samples/full-system-v1/computational/effect-failure-missing-negative/src/effect-failure-missing.mir`
- `samples/full-system-v1/computational/undeclared-capability-negative/src/undeclared-capability.mir`

Later widening:

- variables / scope
- arrays / bounds
- records / Vec3
- control-flow
- `samples/full-system-v1/world-core/src/world-core.mir`
- `samples/full-system-v1/membership-chat/src/membership-chat.mir`
- `samples/full-system-v1/sugoroku-world/src/sugoroku-world.mir`

## validation target

Current validation:

```bash
cargo test -p mir-ast --test textual_mir_alpha -- --nocapture
python3 scripts/textual_mir_samples.py check-all --format json
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
python3 scripts/full_system_v1_samples.py check-all --format json
```

## stop lines

- Do not freeze final grammar.
- Do not treat parser-free representative `.mir` sketches as executable source.
- Do not make `package.mir.json` the final language.
