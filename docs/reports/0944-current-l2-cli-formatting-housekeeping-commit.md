# 0944 Current L2 CLI Formatting Housekeeping Commit

## Objective

worktree に残っていた `mir-runtime` current-L2 CLI 周辺の未commit差分を確認し、必要なら独立 commit として切り出す。

## Scope and assumptions

- 対象は次の 4 ファイルに限る
  - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- 今回の差分は docs package と混ぜない
- 意味変更が無いなら formatting-only housekeeping commit として扱う

## Documents consulted

- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0943-docs-snapshot-clarity-and-current-reading-sync.md`

## Actions taken

1. 対象 4 ファイルの `git diff` を確認した。
2. import order、line wrap、function signature wrap のみで、意味変更が無いことを確認した。
3. CLI / source-sample runner / binary compile の focused validation を実行した。
4. housekeeping commit 用 report を追加した。

## Files changed

- `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `docs/reports/0944-current-l2-cli-formatting-housekeeping-commit.md`

## Evidence / outputs / test results

### Diff classification

- `mir-clean-near-end.rs`
  - import order の並び替えのみ
- `current_l2_cli.rs`
  - line wrap / formatting のみ
- `current_l2_operational_cli.rs`
  - assertion line wrap のみ
- `current_l2_source_sample_runner.rs`
  - import wrap / long line wrap のみ

### Commands run

```bash
git diff -- crates/mir-runtime/src/bin/mir-clean-near-end.rs crates/mir-runtime/src/current_l2_cli.rs crates/mir-runtime/tests/current_l2_operational_cli.rs crates/mir-runtime/tests/current_l2_source_sample_runner.rs
cargo test -p mir-runtime --test current_l2_operational_cli
cargo test -p mir-runtime --test current_l2_source_sample_runner
cargo test -p mir-runtime --bin mir-clean-near-end --no-run
git diff --check
```

### Results

- `cargo test -p mir-runtime --test current_l2_operational_cli`: pass
  - 4 tests passed
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`: pass
  - 2 tests passed
- `cargo test -p mir-runtime --bin mir-clean-near-end --no-run`: pass
  - binary compile succeeded
- `git diff --check`: pass

## What changed in understanding

- 残差分は意図不明な作業途中変更ではなく、current-L2 CLI 周辺の formatting-only housekeeping diff だった
- revert よりも独立 commit に切り出す方が安全で説明可能

## Open questions

- `progress.md` / `tasks.md` 更新不要
- この種の formatting-only diff を後続 package の途中で残さないため、将来は package close 前に早めに独立 commit する方がよい

## Suggested next prompt

`Typed external boundary residual planned family review` を進めてください。phase 9 residual planned family `EXT-01` / `EXT-02` / `EXT-05` の reopen criterion を、synthetic preview helper subset `EXT-03` / `EXT-04` と final public host-facing contract を混同しない形で docs / plan / snapshot に反映し、validation / commit / push まで閉じてください。
