# Minimal Alpha-1 Pattern Verification

## summary

`P-PAT-01` adds a narrow verification helper and guide for the minimal practical alpha-1 pattern set:

- `scripts/minimal_alpha1_patterns.py`
- `docs/hands_on/minimal_alpha1_patterns_01.md`

It does not add a new runtime semantics layer. It consolidates existing executable and inventory-backed rows into one strict sample verifier so drift in row counts, expected rejections, or non-claim boundaries is caught quickly.

## why this exists

The repo already has product alpha release-candidate workflow, installed-binary adoption probe, operational suite, Mir computational helper rows, PoseGraph helper rows, projection inventory, and engine-adapter inventory. The risk after all-up closeout is that a reader must open too many documents to see the smallest useful pattern set.

The new helper answers this smaller question:

```text
What is the minimal alpha-1 pattern set that is runnable or intentionally rejected today,
and how do we prove that it has not drifted?
```

## current strict checks

Default strict check:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

The check fixes exact expectations:

- computational: 15 rows, 7 accepted, 5 expected runtime rejections, 3 expected check rejections.
- PoseGraph: 9 rows, 1 accepted, 1 `violation_export`, 7 planned.
- projection: 4 planned rows plus accepted/rejected compatibility rows.
- engine-adapter: 8 provider rows, `NativeExecutionPolicy = Disabled`, `WasmExecutionPolicy = InventoryOnly`, semantic owner `mir_mirrorea`.

This is intentionally stricter than a smoke test. It treats row drift as a failure even if the underlying helper still exits successfully.

## practical reading

Minimal positive rows:

- `mir-compute-add-one`
- `mir-compute-host-io-transform`
- `posegraph-no-split-frame`

Minimal negative / boundary rows:

- `mir-compute-missing-effect-reject`
- `posegraph-split-frame-violation`
- `projection-inventory-boundary`
- `engine-adapter-wasm-inventory`

Workflow anchors:

- `product-alpha1-release-candidate`
- `operational-sugoroku-workflow`

Workflow anchors are not run by default because they are heavier. Use:

```bash
python3 scripts/minimal_alpha1_patterns.py check-all --include-workflows --out /tmp/mirrorea-minimal-alpha1-patterns --format json
```

## theoretical reading

This package preserves the current theory boundaries:

- standard I/O is not a Mir core primitive.
- host I/O is typed external adapter boundary.
- pure computation and effectful host boundary calls are separated.
- effect, failure, capability, witness, and redaction rows stay explicit.
- Place remains execution locus, not participant identity.
- PoseGraph state belongs to Mir / Mirrorea semantics, not renderer hidden state.
- projection / engine-adapter rows are inventories until a later package actualizes codegen or provider admission.

## non-claims

`P-PAT-01` does not claim final textual grammar, final public product, final API/SDK, direct LLVM/native backend, final server/client binary split, provider admission, arbitrary native/WASM execution, full PoseGraph runtime/save-load/devtools completion, WAN/federation, or distributed durable save/load.
