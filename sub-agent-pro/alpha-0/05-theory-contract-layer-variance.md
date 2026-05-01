# 05 — Theory freeze: contract subtyping, layer insertion, and variance

Mirror normative content into `specs/14-contract-subtyping-layer-compatibility.md`.

## 1. Core problem

Mirrorea must support inserting layers into a running system: debug, telemetry, rate-limit, authentication, authorization, redaction, adapter, visualization, and policy layers. These layers must not silently break existing contracts.

The core issue is substitutability. If a layer wraps a service/Place/effect/route, existing clients should remain safe unless the layer is introduced through an explicit versioned contract update or activation cut that changes the contract.

## 2. Contract shape

For alpha purposes, model a contract as:

```text
Contract C = {
  input_type: I,
  output_type: O,
  precondition: P,
  postcondition: Q,
  effect_row: E,
  failure_row: F,
  required_capabilities: A,
  provided_capabilities: B,
  cost_bound: R,
  observation_policy: L,
  retention_policy: T
}
```

Exact representation may differ, but checker and samples must cover each dimension.

## 3. Substitutability rules

A transparent overlay/layer `C'` may replace/wrap `C` only if:

### 3.1 Input type contravariance

The layer must accept at least the inputs the base contract accepted.

```text
I_base <: I_layer
```

### 3.2 Output type covariance

The layer must return outputs acceptable to existing clients.

```text
O_layer <: O_base
```

### 3.3 Precondition weakening

The layer must not require more from callers.

```text
P_base => P_layer
```

If base requires `member(room)` and layer requires `member(room) AND admin`, it is not a transparent overlay.

### 3.4 Postcondition strengthening

The layer must not weaken guarantees.

```text
Q_layer => Q_base
```

If base guarantees `delivered(message)` and layer only guarantees `maybe_delivered(message)`, it is not a transparent overlay.

### 3.5 Effect row containment

The implementation/layer must only perform effects allowed by the declared contract.

```text
E_layer ⊆ E_base_allowed
```

If a debug layer writes to an external telemetry service but the contract only allows local observation, the layer is invalid or requires a contract update.

### 3.6 Failure row containment

The layer must only return failures allowed by the declared failure row.

```text
F_layer ⊆ F_base_allowed
```

Adding `RateLimited`, `AuthFailed`, or `MissingCapability` is not transparent unless those outcomes were already in the failure row or are introduced by explicit contract update.

### 3.7 Capability monotonicity

The layer must not require stronger caller capabilities for the same contract. It may consume its own administrative attach capability at installation time, but it must not turn ordinary calls into admin-only calls without contract update.

### 3.8 Cost / latency monotonicity

The layer must not worsen declared cost/latency/resource bounds unless the contract explicitly allows degraded outcomes such as `Approximate`, `Deferred`, or `RateLimited`.

### 3.9 Observation noninterference

A layer may not leak data above the observer's label/authority. Debug and telemetry layers must carry observation label, redaction, retention, and authority rules.

### 3.10 Retention monotonicity

A layer must not store telemetry longer than the declared retention policy allows.

## 4. Variance for reference/capability types

The checker must avoid Java-like mutable variance problems.

Recommended rules:

```text
Read<T>        covariant in T, subject to labels/contracts
Observe<T>     covariant in T, subject to labels/contracts
Write<T>       contravariant or restricted; do not casually expose
ReadWrite<T>   invariant
MutableRef<T>  invariant
Linear<T>      invariant
Capability<T>  depends on operation; split read/write/observe/publish/consume
```

Do not model a mutable container or read-write remote reference as covariant.

## 5. Layer categories

Alpha should model at least these layer kinds:

```text
Debug
Telemetry
Auth
Authorization
RateLimit
Redaction
Adapter
Policy
Projection
```

## 6. Attach points

Alpha should model at least these attach points:

```text
MessageDispatch
EffectCall
PlaceBoundary
StateTransition
PublishObserve
WitnessUse
Handoff
HotPlugLifecycle
ExternalAdapter
VisualizationStream
```

## 7. Auth/rate-limit layers

Auth and rate-limit layers are especially tricky.

Invalid transparent insertion:

```text
base failure row: {Reject(NetworkFailure)}
layer failure row: {Reject(NetworkFailure), Reject(AuthFailed), Reject(RateLimited)}
```

This widens the failure row and is not transparent.

Valid approaches:

1. Base contract already included `AuthFailed` / `RateLimited`.
2. Insert layer through explicit contract version update and activation cut.
3. Layer only observes/logs and does not change admission/outcome.

## 8. Debug layer rules

A debug layer must declare:

- what it observes
- who may attach it
- who may read its output
- observation labels
- redaction rules
- retention scope
- sampling/on-demand behavior
- whether it can affect runtime behavior

Observer-safe debug output must not include raw witness refs, raw auth evidence, secrets, or high-label values.

## 9. Required proof obligations

1. Overlay substitutability.
2. Precondition weakening.
3. Postcondition strengthening.
4. Effect-row containment.
5. Failure-row containment.
6. Mutable invariance.
7. Read-only covariance soundness.
8. Observation noninterference.
9. Retention compliance.
10. No silent API shadowing.

## 10. Required samples

See `08-sample-matrix.md`. Minimum set:

- valid logging layer
- invalid precondition strengthening
- invalid postcondition weakening
- valid output covariance
- invalid mutable covariance
- valid read-only covariance
- invalid failure row widening
- valid rate-limit with declared failure
- invalid effect row widening
- invalid cost degradation
- valid redaction layer
- debug layer requires authority

## 11. Extension points

Deferred:

- full dependent refinement subtyping
- theorem-prover-backed contract implication
- exact cost/latency algebra
- exact label lattice finalization
- final public API for layer attachment
- production auth schema
- production telemetry service
