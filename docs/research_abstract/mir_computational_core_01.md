# Mir Computational Core Rebaseline

## summary

Product Alpha-1 is valuable as a runtime/product workflow floor: `mirrorea-alpha`, versioned `package.mir.json`, local/Docker runtime, devtools, R0/R2 save evidence, native host launch bundle, release checks, and the operational product suite are preserved.

The drift is that this floor can be misread as proof that Mir already owns general computation. It does not. The current `AddOne` lane proves a typed external host-boundary path, not Mir-owned arithmetic / variable / array / record / control-flow semantics.

## new promoted docs/spec line

The next docs/spec line is:

```text
P-COMP-00 recognition rebaseline
P-COMP-01 Mir computational core spec and sample scaffold
P-COMP-02 pure AddOne in Mir
P-COMP-03 variables / arrays / records / control-flow first floor
P-COMP-04 effect boundary around internal computation
P-POSE-01 Transform / PoseGraph spec and sample scaffold
P-POSE-02 avatar head + anchored object no-split-frame sample
P-PROJ-01 projection boundary and packet/FFI schema inventory
P-ENG-01 engine/backend adapter boundary spec
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

