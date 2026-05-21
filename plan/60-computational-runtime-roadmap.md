# plan/60 — computational runtime roadmap

## purpose

This document is repository memory for `specs/35-mir-typed-ir-and-interpreter.md`.

## current state

Current computational evidence includes:

- one direct Mir-owned `add_one` runtime row.
- helper-executable variables / arrays / records / control-flow / imports rows.
- one direct host read/write transform row.
- effect / failure / capability check rejections.

This is first-floor evidence. It is not Rust-level language completion.

## package sequence

| Package | Goal | Close condition |
|---|---|---|
| `P-MIR-02` | typed IR and checker | types, scope, imports, effect rows, failure rows, and capability rows are checked |
| `P-MIR-03` | safe C-like interpreter | source-derived typed IR executes positive/negative computational samples |
| `P-MIR-04` | effectful runtime integration | host boundary, publish/observe, witness/handoff, fallback, and cut rows are explicit, with rollback-across-cut, stale-state non-resurrection, and violated `R2` precondition negatives where cut/save behavior is executable |
| later | Rust-like widening | ownership/borrow-like discipline, limited traits/interfaces, generics, async/effects |

## interpreter evidence requirements

Each behavior claim requires:

- positive row.
- negative row.
- compute trace.
- observer-safe report.
- static vs runtime rejection classification.

## validation target

Planned validation:

```bash
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
python3 scripts/full_system_v1_samples.py check-all --format json
```

Existing anchors remain:

```bash
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## stop lines

- Do not claim broad effectful semantics before `P-MIR-04`.
- Do not claim Rust-level expressiveness from the safe C-like subset.
- Do not introduce stdio as Mir core primitive.
- Do not start LLVM/native codegen before typed IR and projection boundaries are stable.
