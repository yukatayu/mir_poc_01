# Mir Computational Core Rebaseline

## summary

Product Alpha-1 is valuable as a runtime/product workflow floor: `mirrorea-alpha`, versioned `package.mir.json`, local/Docker runtime, devtools, R0/R2 save evidence, native host launch bundle, release checks, and the operational product suite are preserved.

The drift is that this floor can be misread as proof that Mir already owns general computation. It does not. The current `AddOne` lane proves a typed external host-boundary path, not Mir-owned arithmetic / variable / array / record / control-flow semantics.

`P-COMP-01` actualized the scaffold for that correction, `P-COMP-02` promoted the first direct executable row, `P-COMP-03` widened the first floor, and `P-COMP-04` actualized the bounded host read/write effect-boundary row:

- `samples/product-alpha1/computational/`
- `samples/product-alpha1/computational/matrix.json`
- `scripts/mir_computational_samples.py`

The helper can now list rows, validate the matrix, execute `run comp-02-pure-add-one`, execute positive/negative `comp-03` rows, execute `run comp-04-host-io-internal-transform-positive`, execute three expected `check` rejections for missing declarations, and prove the bounded event order `host_input_received -> mir_compute_step -> host_output_emitted` for the direct runtime rows. This is useful because the proof point is machine-readable without rewriting the old adapter-owned `typed_host_io.add_one` lane.

`P-POSE-01` now does the same for PoseGraph:

- `samples/product-alpha1/posegraph/`
- `samples/product-alpha1/posegraph/matrix.json`
- `scripts/posegraph_samples.py`

That helper keeps `no-split-frame` as a docs/spec boundary and rejects `run pose-04-no-split-frame-positive` / `run pose-05-split-frame-negative` as `planned_only` until `P-POSE-02`.

`P-PROJ-01` and `P-ENG-01` complete the same front-half pattern for projection/backend and engine/provider inventory:

- `samples/product-alpha1/projection/`
- `scripts/projection_boundary_samples.py`
- `samples/product-alpha1/engine-adapter/`
- `scripts/engine_adapter_boundary_samples.py`

These helpers keep projection code generation, server/client binary split, provider admission, native execution, and WASM execution as explicit non-claims while making the current boundary inventory machine-readable.

## new promoted docs/spec line

The next docs/spec line is:

```text
P-COMP-00 recognition rebaseline
P-COMP-01 Mir computational core spec and sample scaffold
P-POSE-01 Transform / PoseGraph spec and sample scaffold
P-PROJ-01 projection boundary and packet/FFI schema inventory
P-ENG-01 engine/backend adapter boundary spec
P-COMP-02 pure AddOne in Mir
P-COMP-03 variables / arrays / records / control-flow first floor
P-COMP-04 effect boundary around internal computation
P-POSE-02 avatar head + anchored object no-split-frame sample
```

This is additive. It does not roll back the Product Alpha-1 operational floor.

## theoretical correction

Pure computation and effectful boundary calls are separated.

Pure fragment:

```text
Σ ; Γ ⊢ e : A ⇝ C
```

Effectful fragment:

```text
Σ ; Ψ ; Γ ; Δ ⊢ t : A @ μ ! ε ? ρ ⇝ C ; O
```

The explicit failure row `ρ` matters. Bounds errors, stale membership, capability failures, and host-boundary rejects must be declared or discharged; they are not hidden magic behavior.

## practical target

The first meaningful computational proof point is not another adapter demo. It is:

```text
host input
  -> typed external read boundary
  -> Mir-owned add_one function
  -> typed external write boundary
  -> observer-safe evidence
```

After that, variables, arrays, records, control-flow, imports, and host-boundary wrapping can be widened.

Current computational validation commands:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
python3 scripts/mir_computational_samples.py run comp-03-control-flow-positive --format json
python3 scripts/mir_computational_samples.py run comp-03-variables-scope-negative --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json
```

These commands now prove `P-COMP-02`, `P-COMP-03`, and `P-COMP-04`. The `P-COMP-04` positive row proves declared boundary carriage through checker/runtime-plan/session evidence; it does not mean `required_capabilities` / `failure_tag` are already a broad effectful runtime semantics. Final grammar and backend realization remain open.

## PoseGraph target

Virtual-space state must be Mir / Mirrorea-owned:

- avatar head transform
- object anchor
- pose version
- fallback chain
- observation snapshot
- save/load frontier
- capability / witness / membership freshness

No-split-frame is same-client same-observation-snapshot coherence, not global simultaneous coordinates.

## non-claims

This rebaseline does not claim final textual grammar, final ABI/SDK, direct LLVM/native backend, final server/client binary split, arbitrary native/WASM execution, Unity/VRM compatibility, WAN/federation, or active PoseGraph runtime samples.
