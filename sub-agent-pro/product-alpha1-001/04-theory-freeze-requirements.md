# Theory freeze requirements

Product α-1 must not be ad-hoc. It needs enough formal theory to justify the implemented subset.

## Verification stratification

Keep three lines:

```text
Line 1: decidable static checker
Line 2: model-check second line
Line 3: proof / residual obligation side line
```

### Line 1

Checker must decide the finite fragment used by product demo:

- package schema well-formedness;
- effect row containment;
- failure row containment;
- capability requirement;
- witness requirement;
- fallback/lifetime selected cases;
- contract variance for layers;
- observability redaction/retention;
- savepoint class admissibility for implemented modes;
- message recovery policy coverage.

### Line 2

Model-check second line handles finite transition systems:

- stale membership interleaving;
- hot-plug attach/reject/defer order;
- save/load or quiescent-save barriers;
- message failure/retry/drop where finite.

### Line 3

Residual obligations are explicit:

- no implicit proof-by-absence-of-negative;
- obligations have family, source, expected discharge target;
- proof stubs are not proof completion.

## Indexed / dependent type boundary

Allowed in product α-1:

```text
Ref<T, region, cap, label>
Message<epoch, incarnation, recovery_policy>
Package<effects, failures, capabilities>
Layer<pre, post, effects, failures>
SavePoint<class>
```

Not allowed as product α-1 completion claim:

- full dependent type theory;
- Isabelle-like integrated proof language in Mir source;
- arbitrary proof terms in runtime;
- final theorem prover binding.

## Required invariants

### Runtime

- Event graph is a DAG.
- Dispatch mutation occurs only after membership/capability/witness checks.
- Fallback degradation is observable.
- Layer attach accepted produces activation cut.
- Rejected attach does not mutate state.
- Deferred detach is not accepted detach execution.

### Save/load

- Local save/load does not resurrect stale membership.
- Quiescent save requires sealed/no-in-flight evidence.
- Distributed durable save/load is not claimed.

### Auth/layer

- Auth/rate-limit do not silently strengthen transparent contract.
- Contract update is explicit.
- Failure row widening must be declared.

### Observability

- Observer-safe view is redacted.
- Admin/debug view is separate or explicitly later.
- Retention policy is enforced or non-claimed.

## Completeness claims

For product α-1, use bounded completeness:

> All declared product-demo checks in the declared finite fragment are checked.

Do not claim:

> All Mir programs are completely checked.
