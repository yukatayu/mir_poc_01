# alpha sample family — Network / Docker

- Status: first Rust network runtime floor + Docker Compose runner
- Phase: Phase 5
- Stage: Stage C
- This family is still not an active runnable root.
- Helper-local `scripts/network_transport_samples.py` remains a separate canary family and does not validate these Alpha-0 sample IDs by name.

## Current rows

| ID | File | Status | Kind | Expected |
|---|---|---|---|---|
| `NET-01` | `net-01-local_queue_parity.mir` | planned mirror only | positive | local queue parity remains sourced from `LR-01` until a dedicated row runner is added |
| `NET-02` | `net-02-docker_two_process_envelope.mir` | actualized | positive E2E | accepted over the narrow TCP / Docker bridge |
| `NET-03` | `net-03-stale_membership_rejected.mir` | actualized | negative E2E | `Reject(membership_freshness)` |
| `NET-04` | `net-04-missing_capability_rejected.mir` | actualized | negative E2E | `Reject(capability)` |
| `NET-05` | `net-05-missing_witness_rejected.mir` | actualized | negative E2E | `Reject(witness)` |
| `NET-06` | `net-06-route_rebinding_no_shadow.mir` | planned | positive | valid if route rebinding keeps interface/shadow discipline |
| `NET-07` | `net-07-observer_safe_route_trace.mir` | actualized | positive | observer-safe redacted route trace; no raw auth/capability/witness payloads |
| `NET-08` | `net-08-network_partition_explicit_failure.mir` | planned | negative/positive | `Reject(NetworkUnavailable)` or `Deferred` |
| `NET-09` | `net-09-auth_evidence_lane_preserved.mir` | actualized | positive | auth evidence lane preserved separately from transport |
| `NET-10` | `net-10-transport_medium_change_preserves_contract.mir` | planned | positive | valid if contract unchanged across medium change |

## Policy

- `.mir` files here remain source-ish anchors; the current runner is sample-ID keyed and non-public.
- `NET-02/03/04/05/07/09` are actualized by the dedicated `mir-runtime` Stage-C floor plus Docker Compose runner.
- `NET-01/06/08/10` remain planned and must not be promoted by similarity to helper-local canaries.
- `scripts/network_transport_samples.py` continues to prove the separate helper-local `samples/clean-near-end/network-transport/` family only.

## Validation anchor for the current package

```bash
cargo test -p mir-runtime --test alpha_network_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout
python3 scripts/alpha_network_docker_e2e.py check-all --format json
```

## Stop line

- do not call this family production transport, WAN federation, durable replay, or final public transport ABI
- do not treat current Docker Compose evidence as continuous shared runtime state across worlds
- do not claim `NET-06`, `NET-08`, or `NET-10` are implemented in this package
- do not collapse auth, membership, capability, or witness lanes into transport success
