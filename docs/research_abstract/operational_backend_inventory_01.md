# Operational Backend Inventory 01 Summary

`P-OPS-08` は、operational product sample suite に対する
**host launch bundle / WASM / LLVM comparison inventory** を docs-first で固定する package です。

## Current reading

- actualized:
  `native host launch bundle`
- planned inventory only:
  WASM client host
  LLVM/native projection backend

## What is concrete now

- `mirrorea-cli build-native-bundle`
- operational `sugoroku-world` bundle build / `run.sh check` / `run.sh view`
- schema-backed projection target / packet / FFI inventory

## What remains non-claim

- direct Mir-to-machine-code
- emitted server/client binaries
- final placement optimizer
- final public client/server ABI
- arbitrary native package execution

## Why this split matters

The current repo already has an executable alpha path for packaging and runtime replay, but that path is a host launch bundle around the compiled Rust CLI. Treating that as LLVM codegen or as a finished client/server backend would overclaim both runtime and projection semantics.

## Entry points

- hands-on: `../hands_on/operational_backend_inventory_01.md`
- storage/backend guardrail: `compiler_backend_llvm_preparation_01.md`
- normative boundary: `../../specs/26-operational-product-sample-suite.md`
- roadmap memory: `../../plan/23-compiler-backend-llvm-guardrail-roadmap.md`, `../../plan/51-operational-product-sample-roadmap.md`
