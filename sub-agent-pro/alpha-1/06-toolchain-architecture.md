# 06 — Practical Toolchain Architecture

## 1. Desired alpha-1 toolchain

```text
alpha source / package manifest
  -> parser / loader
  -> alpha IR
  -> checker
  -> runtime plan
  -> local runtime / Docker runtime
  -> event + telemetry export
  -> devtools viewer
```

## 2. Components

### 2.1 Source / package loader

- Reads alpha `.mir` or `.mir.json` package files.
- Alpha grammar is limited and non-final.
- Package contains world, places, transitions, effects, layers, runtime packages, tests.

### 2.2 IR

IR should include:

- places
- principals / participants
- effects
- contracts
- capabilities
- fallback chains
- layers
- hot-plug requests
- message envelopes
- save/load markers
- devtools labels

### 2.3 Checker

Checker outputs:

- verdict
- diagnostics
- accepted obligations
- rejected rows
- canonical fallback chains
- contract comparison report
- cut validity report
- package admission report

### 2.4 Runtime

Runtime exposes reusable API:

- create world/runtime
- register place
- register participant
- enqueue message
- dispatch
- attach layer/package
- export event DAG
- save/load local

### 2.5 Transport

Transport implementations:

- in-process queue
- local TCP / Docker

Transport must keep lanes separate:

- transport
- auth evidence
- membership epoch
- capability requirements
- witness refs
- route trace

### 2.6 Package / hot-plug

Package manifest:

- package id/version
- kind: object/layer/runtime/avatar/adapter
- required/provided capabilities
- effect row
- failure row
- observation labels
- native policy
- signature/provenance fields

### 2.7 Devtools

Devtools exports:

- event DAG
- Place graph
- route trace
- membership timeline
- hot-plug lifecycle
- fallback degradation
- redacted observer view

## 3. Crate/module suggestions

Do not break current workspace unnecessarily. Add incrementally.

Potential new crates later:

- `mir-ir`
- `mir-checker`
- `mir-cli`
- `mirrorea-runtime`
- `mirrorea-transport`
- `mirrorea-devtools`
- `mirrorea-package`

If adding crates is too disruptive, create modules first under existing `mir-runtime` / `mirrorea-core`, but keep module boundaries clear.

## 4. CLI suggestions

Alpha CLI names may be non-final.

Possible commands:

```bash
mirrorea-alpha check <package>
mirrorea-alpha run-local <package>
mirrorea-alpha run-docker <package>
mirrorea-alpha attach <package> <layer-or-object>
mirrorea-alpha export-devtools <run-id>
mirrorea-alpha save <run-id>
mirrorea-alpha load <savepoint>
```

If no CLI crate exists yet, use examples/scripts but document them as alpha toolchain commands.
