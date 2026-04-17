# Report 0726 — order/handoff corrected prototype third tranche

- Date: 2026-04-17T05:26:00Z
- Author / agent: Codex
- Scope: late-join visibility and stale-reconnect refresh corrected prototype actualization, snapshot synchronization
- Decision levels touched: L2 current design proposal, L3 helper-local prototype convenience

## 1. Objective

order/handoff adequacy corpus のうち late join visibility と stale reconnect refresh を current L2 corrected prototype に落とし、publication / handoff / rollback / observation の差を runnable に比較できるようにする。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/prototype/README.md`
- `faq_005.md`
- `docs/reports/0722-sample-verification-preview-and-prototype-second-tranche.md`
- `docs/reports/0725-typed-theorem-model-check-corrected-prototype-tranche.md`

## 3. Actions taken

1. `samples/prototype/current-l2-order-handoff/` に `p07` / `p08` と sidecar host-plan を追加した。
2. runner / CLI test を先に追加し、missing file failure を確認した。
3. `p08` host-plan は first draft で invalid verdict kind (`failure`) を使って落ちたため、current helper schema に合わせて `explicit-failure` へ narrow fix した。
4. `specs/examples/457` を追加し、third tranche の current judgment を docs-first に固定した。
5. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`faq_005.md`、`samples/prototype/README.md` を prototype octet と self-driven queue `0 package` に同期した。

## 4. Files changed

- Added:
  - `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt`
  - `samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.host-plan.json`
  - `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt`
  - `samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.host-plan.json`
  - `specs/examples/457-current-l2-order-handoff-corrected-prototype-third-tranche.md`
- Modified:
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `samples/prototype/README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `faq_005.md`
  - `specs/00-document-map.md`

## 5. Commands run and exact outputs

- RED:
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_order_handoff_third_tranche_paths -- --exact --nocapture`
    - `FAILED`
    - `No such file or directory`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact --nocapture`
    - `FAILED`
    - `source sample not found`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact --nocapture`
    - `FAILED`
    - `source sample not found`
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_order_handoff_third_tranche_paths -- --exact --nocapture`
    - `1 passed`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact --nocapture`
    - `1 passed`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact --nocapture`
    - `1 passed`

## 6. Evidence / findings

- `p07` は publication -> `atomic_cut` -> handoff -> `atomic_cut` -> late join observation の 5-event trace を current L2 で runnable にできる。
- `p07` では `observer_debug_text_output` に late join visibility を helper-local preview として残せる。
- `p08` は stale reconnect failure -> rollback -> refresh success の 3-event trace を current L2 で runnable にできる。
- `p08` では stale reconnect の理由語彙を final protocol contract にせず、current helper schema では `explicit-failure` + refresh debug record に留めるのが自然だった。
- tranche close 後、promoted self-driven queue は `0 package` に戻せる。

## 7. Changes in understanding

- late join / reconnect family の全体理論は still mixed gate だが、corrected runnable sample としては current L2 subset へ十分落とせる。
- stale reconnect example は host-plan schema の範囲で見ると「failure reason を rich enum 化する」より「explicit-failure + fallback outcome」を先に比較する方が current cut に合う。

## 8. Open questions

- shared-space fairness / replay operational profile は mixed gate に残る。
- late join / reconnect semantics を shared-space final contract としてどこまで上げるかは未決である。
- snapshot-only cut / durable-cut public naming も未決である。

## 9. Suggested next prompt

remaining queue は mixed gate だけなので、stronger typed surface / theorem discharge transport / model-check property language / shared-space fairness profile / packaging success criteria のどれを先に reopen するかを boundary-prep として整理してください。
