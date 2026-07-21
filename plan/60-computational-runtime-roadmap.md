# plan/60 — computational runtime roadmap

## purpose

This document is repository memory for `specs/35-mir-typed-ir-and-interpreter.md`.

## current state

Current computational evidence includes:

- one direct Mir-owned `add_one` runtime row.
- helper-executable variables / arrays / records / control-flow / imports rows.
- one direct host read/write transform row.
- effect / failure / capability check rejections.
- source-first textual parser, typed checker, and bounded runtime rows under `samples/full-system-v1/computational/`.

This is first-floor evidence. It is not Rust-level language completion.

`P-MIR-02` actualized crate-local typed IR lowering and checker rows for types, lexical scope, imports, imported-module semantic closure, ambiguous import rejection, fixed arrays, effect declarations, failure rows, and capability declarations through:

- `crates/mir-semantics::full_system_v1`
- `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- `samples/full-system-v1/computational/typed-ir-matrix.json`

`P-MIR-03` actualized source-derived pure interpreter rows for AddOne, lexical scope, arrays, records, control-flow, imports, static rejection, and dynamic runtime rejection through:

- `crates/mir-semantics::full_system_v1`
- `crates/mir-runtime::full_system_v1_session`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `samples/full-system-v1/computational/runtime-matrix.json`
- `expected/run.json`
- `python3 scripts/full_system_v1_samples.py runtime-matrix --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`

`P-MIR-04` actualized bounded source-first effectful runtime rows for host read/write, publish/observe, witness/handoff, and local atomic-cut rejection evidence through:

- `crates/mir-semantics::full_system_v1`
- `crates/mir-runtime::full_system_v1_session`
- `samples/full-system-v1/computational/runtime-matrix.json`
- `expected/run.json`
- `python3 scripts/full_system_v1_samples.py runtime-matrix --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`

The checker/runtime share a private exact-pair host-adapter policy for the
existing `read_int@host_input` and `write_int@host_output` examples. The
checker verifies the adapter signature and operation-specific minimum
capability, carries that minimum in the typed perform row, and rejects adapter
use without an ambient transition capability row. It also rejects duplicate
record construction fields and non-scalar equality before evaluation. The
active checker corpus is 3 positive and 18 negative rows. This is a bounded
LAB safeguard: it does not authenticate an executing principal, define a
public adapter ABI, infer capability inheritance through functions, or decide
record/array equality semantics. The final bounded release workflow accepted
all 29 planned commands with the 21/17/12 = 50 source-matrix partition.

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

- Do not claim broad or distributed effectful semantics beyond the bounded local `P-MIR-04` lane.
- Do not claim Rust-level expressiveness from the safe C-like subset.
- Do not introduce stdio as Mir core primitive.
- Do not start LLVM/native codegen before typed IR and projection boundaries are stable.
