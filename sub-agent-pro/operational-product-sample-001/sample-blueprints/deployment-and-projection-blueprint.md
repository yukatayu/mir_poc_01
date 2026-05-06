# deployment and projection blueprint

## local deployment profile

```json
{
  "deployment_profile_version": "ops-local-v0",
  "non_final": true,
  "package": "ops.sugoroku-world",
  "host_kind": "local_native_process",
  "session_id": "session#operational-sugoroku",
  "places": [
    "WorldServerPlace",
    "ChatPlace",
    "SugorokuGamePlace",
    "ParticipantPlace[Alice]",
    "ParticipantPlace[Bob]",
    "HostAdapterPlace"
  ],
  "transport": {
    "mode": "local_loopback_tcp",
    "wan_federation_claimed": false
  },
  "save_load": {
    "supported": ["R0_Local", "R2_Quiescent"],
    "distributed_durable_claimed": false
  }
}
```

## docker deployment profile

```yaml
services:
  world:
    image: mirrorea-product-alpha1-host
    command: ["mirrorea-alpha", "transport-fixture", "world"]
  participant:
    image: mirrorea-product-alpha1-host
    command: ["mirrorea-alpha", "transport-fixture", "participant"]
```

Use current product alpha Docker model if available.
Do not claim production WAN.

## projection profile

```json
{
  "projection_profile_version": "ops-product-projection-v0",
  "non_final": true,
  "source_package": "ops.sugoroku-world",
  "targets": [
    {
      "target_id": "server",
      "target_kind": "server_host",
      "places": ["WorldServerPlace", "ChatPlace", "SugorokuGamePlace"],
      "outputs": {
        "native_binary_emitted": false,
        "host_launch_bundle_part": true
      }
    },
    {
      "target_id": "participant-client",
      "target_kind": "client_host",
      "places": ["ParticipantPlace[*]", "ClientViewPlace"],
      "outputs": {
        "native_binary_emitted": false,
        "host_launch_bundle_part": true
      }
    }
  ],
  "packet_boundaries": [
    {
      "name": "roll_request_packet",
      "fields": ["message_id", "payload", "membership_epoch", "member_incarnation", "capability_requirements", "witness_refs"]
    },
    {
      "name": "chat_message_packet",
      "fields": ["message_id", "payload", "membership_epoch", "redaction_policy"]
    }
  ],
  "ffi_boundaries": [
    {
      "name": "host_io_adapter",
      "input_schema": "typed_payload",
      "output_schema": "typed_payload",
      "effect_row": ["HostIO"],
      "failure_row": ["HostAdapterUnavailable", "Timeout"]
    }
  ],
  "backend": {
    "llvm_codegen_claimed": false,
    "direct_mir_to_machine_code_claimed": false,
    "future_backend_requirements_documented": true
  }
}
```

## projection correctness obligations

Future proof/model-check obligations:

- server/client projection preserves source contracts
- all cross-target communication uses declared packet boundaries
- client cannot write server-authoritative state without capability
- FFI adapter cannot emit undeclared effects/failures
- optimization must preserve required audit/devtools events at boundaries
