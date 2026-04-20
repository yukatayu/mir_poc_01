# plan/01 — 現況サマリ

## repo 全体の主眼

- 主眼は Mir current-L2 の semantics / runner / verification floor にある
- Mirrorea / PrismCascade / Typed-Effect Wiring Platform は separable track として扱う
- current repo は architecture-first だが、docs-only skeleton ではない

## current executable floor

- authored sixteen と corrected prototype set `p01 ... p16` は runnable
- current-L2 Rust path
  - `mir-ast`
  - `mir-semantics`
  - `mir-runtime`
  は helper / CLI / sample runner までつながっている
- representative bundle は `python3 scripts/current_l2_guided_samples.py smoke-all --format json` で再確認できる

## twin peaks の current state

### Problem 1

- current first line:
  checker-adjacent first strong typing layer、notebook-first theorem line、row-local model-check carrier first
- representative sample:
  `p06`
- supporting sample:
  `p10 / p11 / p12 / p15 / p16`
- repo-local evidence:
  - `emit-theorem problem1`
  - `emit-reserve model-check-second-line`
  - Lean foundation proof fragment
  - generated Lean stub acceptance
- still later:
  final public theorem result object、payload public contract、concrete theorem/model-check tool binding、final public verifier contract

### Problem 2

- current first line:
  relation decomposition principal、authoritative-room first default、reserve strengthening split、negative static-stop pair
- representative sample:
  `p07 / p08`
- reserve / negative:
  `p09 / p13 / p14`
- repo-local evidence:
  - `emit-scenario problem2`
  - `emit-reserve auditable-authority-witness`
  - `emit-reserve delegated-rng-service`
- still later:
  low-level `memory_order` exact source surface、final source wording、final emitted schema、final witness/provider public contract、exhaustive shared-space catalog

## Lean の current reading

- `samples/lean/foundations/`
  - actual small proof fragment
  - Lean でそのまま通る
- `samples/lean/current-l2/`
  - representative sample 由来の generated stub corpus
  - Lean は受理するが `sorry` を含む
  - したがって、artifact alignment と theorem discharge は分けて読む

## current stop line

- final public parser grammar
- final public parser / checker / runtime API
- stronger typed source principal
- final public theorem / model-check contract
- final public verifier contract
- low-level `memory_order` exact surface
- final public witness/provider/artifact contract
- packaging / installed binary / FFI / engine adapter

## current recommendation

- representative bundle と reserve summary index を入口に current state を読む
- sample corpus を増やすより、existing bundle / reserve / lane helper で residual gate を整理する
- low-level `memory_order` exact wording や concrete tool brand はまだ public line に上げない

## next reopen map

- Problem 1:
  `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
- Problem 2:
  `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
- parser-side residual:
  `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
- reserve integration:
  `python3 scripts/current_l2_guided_samples.py reserve`
- true user-spec residual:
  `python3 scripts/current_l2_guided_samples.py hold-line`

## current capability summary

| 項目 | 読み |
|---|---|
| runner / CLI | runnable |
| typed rejection pair | runnable |
| theorem-first emitted artifact loop | repo-local actualized |
| model-check second-line reserve summary | repo-local actualized |
| order / handoff representative pair | runnable |
| negative static-stop pair | runnable |
| Lean foundations | actual small proof |
| generated Lean stubs | accepted with `sorry` |
| final public contract family | still later |
