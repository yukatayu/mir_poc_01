# 0792 — witness/provider emitted-contract trace-alignment bridge

## Objective

Package 47 として、witness/provider route actual adoption と emitted-contract coupled-later gate の representative trace alignment bridge を `p07 / p08 / p09` reached、`p05` guard-only で actualize する。

## Scope and assumptions

- final public witness/provider schema や final emitted-handoff contract は採らない。
- representative trace alignment は repo-local emitted artifact refs anchored pair match に留める。
- representative corpus は `p07 / p08 / p09` と `p05` guard-only で止める。

## Documents consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `specs/examples/502-current-l2-witness-provider-route-actual-adoption.md`
- `specs/examples/510-current-l2-theorem-lean-stub-representative-trace-alignment-bridge.md`

## Actions taken

1. witness/provider emitted-contract coupled-later manifest に `repo_local_emitted_artifact_refs` を carry するよう runtime support を更新した。
2. `build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge` を追加した。
3. `current_l2_witness_provider_emitted_contract_trace_alignment_bridge.rs` を追加し、representative corpus の pair alignment を focused test 化した。
4. `specs/examples/512` に current trace-alignment bridge を文書化した。

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test current_l2_witness_provider_emitted_contract_coupled_later_gate --test current_l2_witness_provider_emitted_contract_trace_alignment_bridge`
  - PASS
- `cargo test -p mir-runtime --test current_l2_witness_provider_route_actual_adoption --test current_l2_witness_provider_public_schema_coupled_later_gate`
  - PASS

## What changed in understanding

- witness/provider emitted-contract line は coupled-later gate で止めず、representative trace alignment bridge まで helper-local に actualize してよい。
- route actual adoption と coupled-later gate の relation は repo-local emitted artifact refs anchored pair match で十分 narrow に固定できる。
- これにより final public witness/provider contract 群を採らずに trace-alignment evidence を積める。

## Open questions

- final public witness schema / provider receipt schema / combined contract をどの reopen matrix で詰めるか。
- final emitted-handoff contract をどの段で別 package として reopen するか。
- representative corpus を beyond `p07 / p08 / p09` に広げるか。

## Suggested next prompt

`specs/examples/512` を anchor に、order-handoff / witness-provider final public seam compression package を進めてください。
