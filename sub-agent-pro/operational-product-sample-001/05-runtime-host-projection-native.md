# 05 — runtime host, projection, and native boundary

## 1. The user's desired long-term shape

Long-term desirable pipeline:

```text
Mir source / package
  -> checked IR
  -> system projection
  -> server target
  -> client target
  -> packet / FFI boundary schema
  -> optimized native code, potentially via LLVM
```

This direction is valid.

## 2. Current product alpha boundary

Current product alpha does not do direct Mir-to-machine-code.

Current native output is:

```text
native host launch bundle
  bin/mirrorea-alpha
  packages/
  devtools/
  reports/
  manifest.json
  launch.json
  provenance.json
  run.sh
  README.md
```

This is useful but not direct codegen.

## 3. Immediate P-OPS-01 requirement

P-OPS-01 must not implement LLVM backend.

It should add:

- deployment profile
- projection profile
- server/client target intent
- packet boundary inventory
- FFI boundary inventory
- future backend stop line

## 4. Runtime host model

Do not assume a monolithic host.

Use this conceptual split:

```text
Mirrorea Host
  loads package
  checks package
  grants capabilities
  provides effect adapters
  provides transport
  provides save/load
  provides devtools
  enforces native policy
```

Host variants:

- server host / daemon-like
- client host / browser-like
- headless participant host
- local CLI host
- Docker fixture host
- future engine host

## 5. Projection profile

Add `deployments/projection/projection.profile.json` with intent such as:

```json
{
  "projection_profile_version": "ops-product-projection-v0",
  "source_package": "sugoroku-world",
  "non_final": true,
  "targets": [
    {
      "target_id": "server",
      "target_kind": "server_host",
      "places": ["WorldServerPlace", "ChatPlace", "SugorokuGamePlace"],
      "emitted_binary_claimed": false
    },
    {
      "target_id": "participant-client",
      "target_kind": "client_host",
      "places": ["ParticipantPlace[*]", "ClientViewPlace"],
      "emitted_binary_claimed": false
    }
  ],
  "boundaries": {
    "packet_schema_required": true,
    "ffi_schema_required": true,
    "typed_effect_boundary_required": true
  },
  "backend": {
    "direct_mir_to_machine_code": false,
    "llvm_backend_claimed": false,
    "native_host_bundle_only": true
  }
}
```

## 6. Packet boundary

For server/client split, boundary schema must include:

- message id
- payload schema
- source / destination place
- transport lane
- auth lane
- membership epoch / member incarnation
- capability requirements
- witness refs
- failure row
- redaction policy for observer trace

Do not erase boundary types.
Internal optimized representation may erase more, but boundary schemas stay typed.

## 7. FFI boundary

If native / engine / host adapter appears, FFI boundary must include:

- input schema
- output schema
- effect row
- failure row
- resource budget
- timeout / cancellation policy
- auth/capability requirement
- provenance / signature metadata
- sandbox policy
- audit events

Signature is provenance only, not safety.

## 8. Future LLVM backend requirements

If later opened, LLVM/codegen line must prove or validate:

- source/package check passed
- projection preserves package contracts
- emitted target communicates only via declared packet/FFI boundaries
- native policy respected
- direct codegen does not bypass auth/capability/witness lanes
- boundary schemas remain observable in devtools
- optimization does not remove required audit/debug events

P-OPS-01 records this future boundary; it does not implement it.
