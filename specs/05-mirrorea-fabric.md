# 05 — Mirrorea Fabric

## Purpose

Mirrorea is the control-and-routing fabric that executes Mir-defined systems across real nodes and allows safe evolution under those semantics.

## Main responsibilities

1. **Logical names and routing**
   - Clients talk to logical names.
   - The physical route behind a logical name may change under controlled rules.

2. **Overlay insertion**
   - A service may be wrapped by compatible overlays.
   - Overlays may inspect, transform, log, rate-limit, authenticate, or reject, but they may not shadow a previously defined API.

3. **Patch application**
   - The preferred model is downstream addition.
   - Patch activation should happen at explicit cuts.

4. **Path proof and audit**
   - It should be possible to prove that a request passed through required overlays.
   - Audit traces should explain route changes and execution history.

5. **Scaling and reconfiguration**
   - Dynamic scaling and route rebinding should fit the same semantic discipline.

## Already-discussed principles

### No shadowing
A route or overlay may not make a previously reachable interface simply disappear as an untyped "no service" error.
If a request is rejected, rejection must be representable inside the original or explicitly versioned contract space.

### Compatibility-preserving overlays
Current direction:
- do not strengthen preconditions,
- do not weaken guarantees,
- do not worsen time/resource budgets without explicit contract change,
- keep failure behavior inside an agreed failure space.

### Existing-system integration
Mirrorea should be able to wrap legacy systems using tunnels/proxies/adapters.
However, whether that is theoretically "inside" Mir or merely operational wrapping must be stated clearly in each case.

## Important unresolved issues

- How strongly route changes are tied to a single consensus mechanism (they should not be specified in that way).
- How path proofs are represented and validated.
- How in-flight coroutine/task state behaves across route change or patch activation.
