# 0929 Network Transport Loopback Preview

## Objective

phase 13 `Network transport` の最小 executable widening として、helper-local `NET-01` loopback preview を actualize し、same-process emulator のまま attach / envelope / reject parity を検証可能にする。

## Scope and assumptions

- current scope は helper-local preview である。
- real socket / broker / reconnect / typed transport failure / route-trace retention は scope 外に残す。
- runtime side では fake parity を作らず、existing `provider_boundary` / `audit_trace_boundary` inventory は維持する。
- `loopback_socket` は final public transport ABI ではない。
- worktree には unrelated current-L2 CLI diff が残っているため、この package でも stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/not_implemented/network-transport/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/22-network-transport-roadmap.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0926-network-transport-plan.md`
- `docs/reports/0928-hands-on-docs-closeout.md`

## Actions taken

1. `scripts/sugoroku_world_samples.py` に helper-local transport seam parameter を追加し、`run` / `check-all` で `--transport loopback_socket` preview を選べるようにした。
2. `MessageEnvelope.transport` を hard-coded `local_queue` から選択 transport seam に widen し、`transport_seam` field と loopback preview note を sample result に追加した。
3. `scripts/tests/test_sugoroku_world_samples.py` に attach parity、handoff parity、reject parity、loopback `check-all` の unit/regression test を追加した。
4. runtime side では fake attach/loopback carrier を増やさず、`crates/mir-runtime/tests/clean_near_end_samples.rs` で reserved loopback seam inventory を明示的に確認する assertion を追加した。
5. `plan/22`、reader-facing docs、`samples_progress.md`、`progress.md`、`tasks.md`、report を同期し、`NET-01` actualization と `NET-02..05` residual queue を current snapshot に反映した。

## Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/not_implemented/network-transport/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/22-network-transport-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `README.md`
- `Documentation.md`
- `docs/reports/0929-network-transport-loopback-preview.md`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --transport loopback_socket --debug envelopes --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --transport loopback_socket --debug envelopes --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 04_non_owner_roll_rejected --transport loopback_socket --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass (`Found 927 numbered report(s).`)
- `git diff --check`
  - pass

## What changed in understanding

- `NET-01` は new sample source を増やさなくても actualize できる。current Sugoroku sample family に transport seam parameter を通すだけで、attach / handoff / reject parity を helper-local canary として確認できる。
- runtime side に fake loopback carrier を作るより、`provider_boundary` / `audit_trace_boundary` をそのまま保ち、reserved loopback seam を inventory として明示する方が current repo の layer boundary に合っている。

## Open questions

- `NET-02` を existing Sugoroku helper extension として続けるか、separate helper / process harness に切るか。
- reconnect / epoch guard を Sugoroku helper で先にやるか、hot-plug lifecycle widening とまとめるか。
- route trace / observer-safe redaction を network lane で先に actualize するか、visualization lane と同時に reopen するか。

## Suggested next prompt

`Avatar fairy follow representative slice` package を進めてください。`samples/not_implemented/avatar-fairy-follow/README.md` の `FAIRY-01..06` から最小 canary を選び、active helper / debug surface / report / `samples_progress.md` / `progress.md` / `tasks.md` を同一 task で更新してください。
