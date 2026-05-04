# 06 — auth and layer algebra

## layer as contract transformer

A layer is a contract transformer.

```text
Layer : Contract -> Contract
```

Contract shape:

```text
Contract = {
  input_type,
  output_type,
  precondition,
  postcondition,
  effect_row,
  failure_row,
  required_capabilities,
  provided_surface,
  observation_policy,
  redaction_policy,
  retention_policy,
  cost_bound
}
```

## transparent overlay condition

Transparent overlay is admissible only if substitutability holds.

```text
I_base <: I_layer            # input contravariance
O_layer <: O_base            # output covariance
P_base => P_layer            # no precondition strengthening
Q_layer => Q_base            # no postcondition weakening
E_layer ⊆ E_base_allowed     # no undeclared effect widening
F_layer ⊆ F_base_allowed     # no undeclared failure widening
R_layer <= R_base            # no required capability strengthening
S_base ⊆ S_layer             # no provided surface shrink
Obs_layer ⊆ Obs_base_allowed # no observation widening
Redact_layer >= Redact_base  # no redaction weakening
Retain_layer ⊆ Retain_base   # no retention widening
```

Auth and rate-limit layers often violate transparency. They should usually go through explicit contract update and activation cut.

## auth provider interface

```text
AuthProvider = {
  provider_id,
  input_evidence_schema,
  output_claim_schema,
  granted_capabilities,
  failure_kinds,
  audit_event_shape,
  revocation_semantics,
  observation_policy
}
```

Checker obligations:

```text
granted_capabilities <= declared_capabilities
failure_kinds ⊆ declared_failure_row
audit events emitted
observation policy not widened
revocation is monotone unless new epoch/evidence is issued
```

## auth stack composition

### all_of

```text
all_of(A, B)
```

Admission requires both A and B. This is safe if failure rows and audit events are declared.

### any_of

```text
any_of(A, B)
```

Dangerous because weaker provider may bypass stronger provider. Allowed only if explicitly declared in contract.

### grants

Capability grants are not automatic. Provider output claims may justify a grant only if the package/layer contract declares it.

## standard-library-like initial set

Initial standard set should be small.

Required auth/policy components:

```text
AuthNoneDev
MembershipAuth
CapabilityAuth
WitnessAuth
PackageProvenanceAuth
LocalTokenAuth
DebugAuthorityAuth
RateLimitPolicy
RedactionPolicy
RetentionPolicy
PackageAdmissionPolicy
NativeExecutionPolicy
```

### AuthNoneDev

Development only. Must be labelled non-production.

### MembershipAuth

Checks membership epoch / participant incarnation freshness.

### CapabilityAuth

Checks required capabilities.

### WitnessAuth

Checks witness refs and scopes.

### PackageProvenanceAuth

Checks signature/provenance. Does not prove semantic safety.

### NativeExecutionPolicy

Native package requires explicit sandbox/trust policy. Signature alone is insufficient.

## debug layer authority

Debug layer must declare:

- who may attach
- who may read
- labels / redaction
- retention
- whether it can affect behavior
- on-demand vs always-on

## rate limit

Rate-limit layer is transparent only if failure row already includes `RateLimited` or the layer is observation-only.

Otherwise it requires explicit contract update.

## stop line

- auth/rate-limit layer を無条件 transparent overlay としない
- signature = safety としない
- debug layer を label-less leak としない
- failure row widening を hidden にしない

