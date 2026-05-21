# samples/product-alpha1/engine-adapter

This root is the planned-only engine / WASM / FFI adapter inventory scaffold for `P-ENG-01`.

## Current Status

- The root exists as inventory actualization only.
- All provider rows are `planned_only`.
- Mir / Mirrorea remain the semantic owners of world state, authority, witness, fallback, observation, and save/load admissibility.
- Packet and FFI seams are explicit and remain separate from transport.
- Default execution gating stays `NativeExecutionPolicy = Disabled` and `WasmExecutionPolicy = InventoryOnly`.

## Current Validation Anchor

Use the helper inventory commands:

```bash
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/engine_adapter_boundary_samples.py matrix --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py run wasm-sandbox --format json
```

`run wasm-sandbox` must reject as `planned_only` until a later bounded provider-admission package exists.

## Provider Rows

- `renderer`
- `input-device`
- `asset-loader`
- `physics-spatial-query`
- `host-runtime-bridge`
- `wasm-sandbox`
- `native-library-bridge`
- `viewer-diagnostic-exporter`

Representative `*.contract.json` files are inventory-only contract sketches. They are not final ABI, not executable runtime admission, and not evidence that providers own semantics.
