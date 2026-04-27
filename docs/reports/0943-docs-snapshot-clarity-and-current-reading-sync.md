# 0943 Docs Snapshot Clarity And Current Reading Sync

## Objective

front-door docs と snapshot docs の stale current-line を解消し、`README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md` が **現在地・次の package・残 gate** を短く正確に読める状態へ揃える。

## Scope and assumptions

- code / sample behavior 自体は変えない
- 規範判断の正本は `specs/`、repository memory は `plan/` のまま維持する
- この task では snapshot / reader-facing docs の freshness と brevity を優先する
- worktree に残っていた runtime 側 4 ファイルの未commit差分は、今回の docs package と無関係な pre-existing formatting diff とみなし、混ぜない

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- reviewer subagent audit result `019dcf4a-4968-7542-839e-4187f17bd60f`

## Actions taken

1. reviewer subagent の read-only audit を待ち、stale wording / verbosity drift / snapshot-history 混同の指摘を受け取った。
2. `README.md` と `Documentation.md` の stale current date と stale next-package wording を 2026-04-27 current reading に更新した。
3. `tasks.md` を snapshot 文書として再構成し、closed package chain と command catalog を削って、current floor・next package・reserve reopen queue・blocker を先頭で読める形へ圧縮した。
4. `progress.md` を圧縮し、current snapshot、3 軸 progress、macro phase map、feature family snapshot、着手可否、recent log だけを残した。
5. `samples_progress.md` を dashboard として維持しつつ、append-only validation ledger を recent validation view に圧縮し、phase 0〜16 representation と current active package を残した。
6. pre-existing runtime 4 ファイルの diff を確認し、formatting-only だが今回の task と無関係なので commit 対象から除外する判断を記録した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Evidence / outputs / test results

### Reviewer findings incorporated

- front-door docs の stale wording:
  `README.md` / `Documentation.md` の current date と next package を更新
- snapshot-history 混同:
  `samples_progress.md` の recent validation を短い snapshot view に圧縮
- task-map の冗長さ:
  `tasks.md` の closed-chain memory と長い helper command list を削除
- progress snapshot の密度過多:
  `progress.md` の summary / recent log を圧縮

### Commands run

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
git diff --check
git diff -- crates/mir-runtime/src/bin/mir-clean-near-end.rs crates/mir-runtime/src/current_l2_cli.rs crates/mir-runtime/tests/current_l2_operational_cli.rs crates/mir-runtime/tests/current_l2_source_sample_runner.rs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

### Results

- `check_source_hierarchy.py`: pass
  - required `23`
  - present `23`
  - missing `0`
- `validate_docs.py`: pass
  - content-sync run:
    - `Documentation scaffold looks complete.`
    - `Found 940 numbered report(s).`
  - final rerun after adding `0943`:
    - `Documentation scaffold looks complete.`
    - `Found 941 numbered report(s).`
- `typed_external_boundary_samples.py closeout --format json`: pass
  - current reading は helper self-consistency + provider-boundary / local-queue anchor comparison
- `sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`: pass
  - `projection_view` は helper-local preview floor のまま
- runtime `05_delegated_rng_service`: pass
  - `cross_place_projection` inventory は report-local preview floor と一致
- `git diff --check`: pass
- final docs-only rerun after adding `0943`:
  - `check_source_hierarchy.py`: pass
  - `validate_docs.py`: pass
  - `git diff --check`: pass

### Uncommitted runtime diff inspection

- 対象:
  - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- 読み:
  - formatting / import-order / line-wrap 中心
- 判断:
  - 今回の docs package と無関係
  - local validation の対象 package でもない
  - 安全な意図の切り分けを優先して **未commitのまま残す**

## What changed in understanding

- source hierarchy 自体は壊れていなかった
- 問題は docs hierarchy collapse ではなく、**front-door freshness と snapshot brevity drift** だった
- `tasks.md` / `progress.md` / `samples_progress.md` は current snapshot に徹させ、closed-package memory と full validation ledger は report 側へ戻すのが正しい

## Open questions

- `Typed external boundary residual planned family review` で、`EXT-01` / `EXT-02` / `EXT-05` をどこまで host schema pressure へ触れずに整理できるか
- `Projection / placement residual emitted-program gate` で、preview floor と emitted artifact family の reopen criterion をどこまで docs-first で詰めるか
- pre-existing runtime formatting diff を別 package で commit する必要があるか

## Suggested next prompt

`Typed external boundary residual planned family review` を進めてください。`EXT-01` / `EXT-02` / `EXT-05` を synthetic preview helper subset `EXT-03` / `EXT-04` と混同しない reopen criterion に整理し、host schema / visualization / provider boundary の stop line を report・snapshot docs・必要最小限の reader-facing docs に反映し、validation / commit / push まで閉じてください。
