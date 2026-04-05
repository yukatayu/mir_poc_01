# Report 0251 — current L2 stage 1 parser spike malformed-source first tranche

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

actualized 済みの stage 1 parser spike first tranche に対して、
stage 1 helper 自身が malformed-source boundary をどこまで fail-closed で持つべきかを比較し、
最小の malformed-source smoke pair を TDD で actualize する。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free AST fixture schema、runtime semantics は変更しない。
- success-side structural smoke (`e4` / `e7` + guard-slot retention) は維持する。
- malformed-source smoke は helper-local substring compare に留め、typed parser error carrier や public diagnostics API は導入しない。
- stage 1 accepted cluster を超える request / `perform` spillover 全体はまだ扱わず、first tranche は 2 件だけに留める。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
- `specs/examples/81-current-l2-stage1-parser-spike-malformed-source-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`

## 4. Actions taken

1. `specs/examples/81-current-l2-stage1-parser-spike-malformed-source-comparison.md` を追加し、current next narrow step を comparison として固定した。
2. `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` に malformed-source smoke 2 本を追加した。
3. RED として `cargo test -p mir-ast --test current_l2_stage1_parser_spike` を回し、generic `unsupported ...` wording では落ちることを確認した。
4. `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` の helper-local error wording を 2 件だけ narrow に整えた。
5. `specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md` を追加し、actualized malformed-source pair と non-goal を記録した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/81-current-l2-stage1-parser-spike-malformed-source-comparison.md`
- Added `specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md`
- Updated `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- Updated `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0251-current-l2-stage1-parser-spike-malformed-source-first-tranche.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
cargo test -p mir-ast --test current_l2_stage1_parser_spike
```

### RED

- `stage1_parser_spike_rejects_missing_edge_local_lineage_metadata`
  - got `line 5: unsupported fallback row \`fallback mirror\``
- `stage1_parser_spike_rejects_option_local_admit_spillover`
  - got `line 2: unsupported option declaration \`option owner_writer on profile_doc capability write lease live admit owner_is(session_user)\``

### GREEN

- `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

## 7. What changed in understanding

- stage 1 parser spike の malformed-source smoke は、accepted-cluster malformed 1 件と later-stage spillover 1 件の pair が最小であることを code 付きで確認できた。
- option-local `admit` spillover は request / `perform` line より declaration boundary に近く、stage 1 non-goal を最小に示す材料として適している。
- helper-local wording fragment を 2 件だけ exact working set に固定しても、public diagnostics API や typed carrier を既成事実化せずに済む。

## 8. Open questions

- `perform on` / `perform via` や request-local `require` / `ensure` spillover を stage 1 helper にどこまで持たせるか。
- current malformed-source wording を exact working set のまま維持する閾値。
- `e3` を stage 2 parser spike へ送る timing。

plan/ 更新済み:
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md` を前提に、次は stage 1 parser spike に `perform` / request-local clause spillover をどこまで持たせるべきかを narrow に比較し、declaration-boundary malformed pair と混線しない最小 cut を整理してください。
