# plan/59 — textual Mir roadmap

## purpose

This document is repository memory for `specs/34-textual-mir-alpha-grammar.md`.

## current state

Current `.mir` files under `samples/product-alpha1/` are explanatory sketches. Current executable input is usually `package.mir.json`.

Full System V1 must introduce a real textual Mir alpha source entrypoint without claiming final public grammar.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-MIR-01` | textual parser alpha | representative computational source files parse to AST with spans and diagnostics |
| `P-MIR-02` | typed IR lowering | parsed AST lowers to typed IR and checker can accept/reject rows |
| `P-MIR-03` | interpreter source bridge | accepted typed IR executes safe C-like samples |
| `P-MIR-04` | effectful source bridge | `perform`, publish/observe, witness/handoff, fallback, and cut enter runtime explicitly |

## planned sample family

Planned first source samples:

- `samples/full-system-v1/computational/add_one.mir`
- `samples/full-system-v1/computational/variables_scope.mir`
- `samples/full-system-v1/computational/arrays_bounds.mir`
- `samples/full-system-v1/computational/records_vec3.mir`
- `samples/full-system-v1/computational/control_flow.mir`
- `samples/full-system-v1/computational/imports_functions.mir`
- `samples/full-system-v1/world-core/src/world-core.mir`
- `samples/full-system-v1/membership-chat/src/membership-chat.mir`
- `samples/full-system-v1/sugoroku-world/src/sugoroku-world.mir`

These paths are planned until a package creates them.

## validation target

Planned validation:

```bash
cargo test -p mir-ast --test textual_mir_alpha -- --nocapture
python3 scripts/textual_mir_samples.py check-all --format json
```

## stop lines

- Do not freeze final grammar.
- Do not treat parser-free representative `.mir` sketches as executable source.
- Do not make `package.mir.json` the final language.
