# 19 — Codex Package Sequence

## Documentation package

### P-FS-00 docs rebaseline

- add specs/33..38 and plan/58..63
- replace progress.md/tasks.md completely
- update README/Documentation/samples_progress
- no runtime changes

## Language packages

### P-MIR-01 textual grammar alpha

- textual parser for computational samples
- AST + diagnostics
- representative .mir samples

### P-MIR-02 typed IR and checker

- lower AST to typed IR
- check types/scope/effects/failures
- negative diagnostics

### P-MIR-03 computational interpreter

- execute pure / C-like subset
- AddOne / variables / arrays / records / control-flow / imports

### P-MIR-04 effectful integration

- perform read/write boundary
- publish/observe/witness/handoff limited integration

## Runtime / PoseGraph packages

### P-POSE-03 runtime PoseGraph

- Transform / PoseVersion / Anchor runtime
- no-split-frame in runtime session

### P-POSE-04 pose save/devtools

- pose-aware save/load
- devtools PoseGraph panels

## Projection/backend packages

### P-PROJ-02 projection IR realization

- source -> projection manifest
- server/client/adapter target roles

### P-PROJ-03 boundary schemas

- packet schema
- FFI schema
- type preservation checks

### P-PROJ-04 server/client local split

- local/Docker server and client roles from projection manifest

## Provider packages

### P-ENG-02 provider admission

- provider manifests
- disabled native default
- WASM inventory only
- over-capability rejects

### P-ENG-03 renderer pose backend demo

- renderer receives pose snapshot
- renderer is not semantic owner

## Full sample packages

### P-FSV1-01 source operational suite

- WorldCore -> MembershipChat -> Sugoroku source-first samples

### P-FSV1-02 portal/shard source samples

- PortalWorldLink / TwoShard / Gradient source-first samples

### P-FSV1-03 full V1 release check

- all commands
- viewer
- bundle
- reports

## Completion package

### P-FSV1-99 final audit

- full validation
- docs/report cleanup
- claim/non-claim audit
- release bundle
