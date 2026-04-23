# plan/01 — 現況サマリ

## repo 全体の主眼

- 主眼は Mir current-L2 の semantics / runner / verification floor にある
- current active sample suite は `samples/clean-near-end/`
- Sugoroku world vertical slice は `samples/clean-near-end/sugoroku-world/`
  と `scripts/sugoroku_world_samples.py` で別 helper として runnable
- old prototype / rough-stimulus / old Lean stub corpus は archive に移した
- Mirrorea / PrismCascade / Typed-Effect Wiring Platform は separable track として扱う

## current executable floor

- `samples/current-l2/` base corpus は維持
- `samples/clean-near-end/` active suite 16 本は runnable
- `samples/clean-near-end/sugoroku-world/` vertical slice 10 本は runnable
- `crates/mir-runtime/src/clean_near_end.rs` が finite-index typing / order-handoff / model-check / modal current layer を持つ
- `scripts/sugoroku_world_samples.py` が logical multi-place runtime attachment emulator を持つ
- representative smoke は
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  で再確認できる

## twin peaks の current state

### Problem 1

- current first line:
  finite-index first strong typing layer、Lean-first proof skeleton、model-check second-line handoff
- active sample:
  clean typing 5 本
- repo-local evidence:
  `run typing`
  `matrix`
  `closeout`
  `samples/lean/foundations/`
  `samples/lean/clean-near-end/`
- still later:
  final typed source principal、final theorem result object、final public checker/verifier contract

### Problem 2

- current first line:
  order / handoff relation decomposition、witness/publication discipline、model-check second-line split
- active sample:
  clean order-handoff 6 本 + model-check 3 本
- repo-local evidence:
  `run order-handoff`
  `run model-check`
  `closeout`
- still later:
  final source wording、final emitted-artifact/public contract、exhaustive shared-space catalog

### Sugoroku world / Mirrorea vertical slice

- current first line:
  empty world server、runtime attach、membership epoch/incarnation、publish/witness/handoff、late join、leave、owner leave、reset model-check
- active sample:
  `samples/clean-near-end/sugoroku-world/00...09`
- repo-local evidence:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py model-check`
  `python3 scripts/sugoroku_world_samples.py closeout`
- still later:
  real network、multi-server consensus、durable distributed commit、detach lifecycle implementation、final public API

## Lean の current reading

- `samples/lean/foundations/`
  - actual small proof fragment
- `samples/lean/clean-near-end/`
  - active sample 由来の generated stub
- `samples/lean/old/2026-04-22-pre-clean-near-end/`
  - historical archive

## current stop line

- final public parser grammar
- final public parser / checker / runtime / verifier API
- full dependent type theory
- final public theorem / model-check / witness-provider contract
- low-level `memory_order` exact surface
- packaging / installed binary / FFI / engine adapter

## current recommendation

- active current layer を読むときは `samples/clean-near-end/` を正本にする
- old `p..` chain は historical comparison としてのみ扱う
- low-level `memory_order` exact wording や concrete tool brand はまだ public line に上げない
