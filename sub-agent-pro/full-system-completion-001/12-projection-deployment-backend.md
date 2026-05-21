# 12 — Projection / Deployment / Backend

## Goal

A system-wide Mir source should not force humans to separately write server code, client code, packet protocol, and FFI glue.

Mir should project into deployment artifacts.

## Projection pipeline

```text
Mir source
  -> typed IR
  -> system graph
  -> projection IR
  -> deployment plan
  -> server artifact
  -> client/browser artifact
  -> adapter artifact
  -> packet schema
  -> FFI schema
```

## First projection target

No LLVM required first.

First target can be:

- interpreted typed IR
- Rust host runtime wrapper
- native host launch bundle
- server/client process split through same binary with different role

## Projection manifest

Must contain:

- source package
- target count
- target roles
- Place mapping
- packet boundary names
- FFI boundary names
- effect/failure rows per boundary
- required capabilities
- preservation checks

## Server/client split

Server target:

- authoritative world state
- membership
- capability/witness validation
- game rules
- save/load authority

Client/browser target:

- local view
- input capture
- pose observation
- prediction if declared
- renderer adapter
- observer-safe devtools

Adapter target:

- rendering
- host I/O
- native/WASM provider
- asset decoding

## Correctness checks

- no hidden client write to authoritative state
- all packets have declared schema
- all FFI calls have effect/failure/capability rows
- server publications match client observations
- projection does not widen failure/effect rows
- packet boundary preserves type schema

## LLVM/backend gate

Only attempt LLVM/native codegen after projection IR and boundary schemas are stable.

Do not conflate native host launch bundle with codegen.
