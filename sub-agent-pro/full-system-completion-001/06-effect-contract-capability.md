# 06 — Effect / Contract / Capability

## Core principle

Effectful behavior must be explicit.

No operation may silently:

- perform external I/O
- send network messages
- mutate remote state
- read private data
- strengthen preconditions
- widen failure row
- leak observer-unsafe data

## Contract model

```text
Contract C = {
  input_type,
  output_type,
  precondition,
  postcondition,
  effect_row,
  failure_row,
  required_capabilities,
  observation_policy,
  redaction_policy,
  retention_policy,
}
```

## Subtyping / layer insertion

Transparent overlay is allowed only when:

- input type is contravariant
- output type is covariant
- precondition is not strengthened
- postcondition is not weakened
- effect row is not widened without declaration
- failure row is not widened without declaration
- capability requirement is not strengthened
- observation is not widened
- redaction is not weakened
- retention is not widened

Auth and rate-limit are usually not transparent overlays. They require explicit contract update or predeclared failure rows.

## Capability discipline

Capabilities are not just booleans.

They should carry:

- principal
- scope
- resource/Place
- epoch/incarnation if applicable
- provenance
- expiration/lease if applicable

## Standard alpha library

Initial standard policy set:

- membership_auth
- capability_auth
- witness_auth
- package_provenance_check
- debug_authority_check
- rate_limit_policy
- redaction_policy
- retention_policy

## Samples

- debug layer accepted
- non-admin debug rejected
- auth layer contract update
- rate-limit declared failure accepted
- rate-limit undeclared failure rejected
- object/avatar package deferred
- provider over-capability rejected
