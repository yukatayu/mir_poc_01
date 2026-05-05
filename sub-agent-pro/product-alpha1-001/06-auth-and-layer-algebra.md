# Auth and layer algebra

## Contract transformer model

A layer is a contract transformer:

```text
Layer : Contract -> Contract
```

Contract:

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
  cost_policy
}
```

Transparent overlay condition:

```text
input: contravariant
output: covariant
precondition: not strengthened
postcondition: not weakened
effect row: not widened unless declared
failure row: not widened unless declared
capabilities: not strengthened
provided surface: not shrunk
observation: not widened
redaction: not weakened
retention: not widened
```

## Auth layer

Auth is usually **not transparent**. It often strengthens admission or widens failure row with `AuthFailed`.

Therefore product α-1 must model auth layer as:

```text
explicit contract update + activation cut
```

unless base contract already declares the auth failure/requirement.

## Standard alpha policy catalog

Include minimally:

- `AuthNoneDev` — development only; explicit non-production marker.
- `MembershipAuth` — epoch/incarnation freshness.
- `CapabilityAuth` — required capability.
- `WitnessAuth` — required witness refs.
- `PackageProvenanceAuth` — signature/provenance only; not semantic safety.
- `DebugAuthorityAuth` — controls debug/devtools/admin view.
- `RateLimitPolicy` — declared failure row required.
- `RedactionPolicy`.
- `RetentionPolicy`.
- `NativeExecutionPolicy`.

## Composition

Default auth stack is `all_of`.

```text
AuthStack = all_of(MembershipAuth, CapabilityAuth, WitnessAuth, ...)
```

`any_of` is forbidden unless explicitly declared because it weakens admission.

## Required product α-1 samples

- `AUTH-A1-01`: debug layer attach accepted with DebugAuthorityAuth.
- `AUTH-A1-02`: non-admin debug attach rejected.
- `AUTH-A1-03`: auth layer accepted only via explicit contract update.
- `AUTH-A1-04`: rate-limit accepted with declared `RateLimited` failure.
- `AUTH-A1-05`: undeclared rate-limit failure rejected.
- `AUTH-A1-06`: package provenance accepted but native execution still disabled.
- `AUTH-A1-07`: over-capability package rejected.

## Required report fields

```json
{
  "auth_stack": ["MembershipAuth", "CapabilityAuth"],
  "contract_update": true,
  "activation_cut_ref": "...",
  "failure_row": ["AuthFailed", "RateLimited"],
  "capability_decisions": [...],
  "redaction_policy": "...",
  "retention_policy": "..."
}
```

## Non-claims

- no production auth protocol;
- no OAuth/WebAuthn/mTLS final policy unless actually implemented;
- signature is not safety proof;
- no arbitrary native execution.
