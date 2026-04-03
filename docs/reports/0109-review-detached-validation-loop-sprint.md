# Report 0109 — detached validation-loop sprint review fallback

- Date: 2026-04-03
- Author / agent: Codex
- Scope: detached validation-loop sprint の review fallback 記録
- Decision levels touched: current parser-free PoC reading の確認

## 1. Objective

detached validation-loop sprint の closing にあたり、reviewer completion が返らない場合の local evidence fallback を明示的に残す。

## 2. Inputs consulted

- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `scripts/current_l2_diff_detached_artifacts.py`
- `scripts/current_l2_detached_loop.py`

## 3. Actions taken

1. sprint 全体の変更を reviewer へ渡し、storage/path policy、aggregate API cut、tiny wrapper、plan mirror の整合を確認するよう依頼した。
2. long wait を行ったが、session から reviewer completion payload を取得できなかった。
3. そのため、local evidence fallback として次を確認した。
   - `host_plan_coverage_failure` を success-side payload core に入れていない
   - tiny wrapper を `scripts/` に置き、`harness.rs` に public API を追加していない
   - `target/current-l2-detached/` を current non-production candidate とする docs-only judgment が `.gitignore` と整合する
   - smoke 実行と `cargo test -p mir-semantics`、`python3 scripts/validate_docs.py`、`git diff --check` が通る

## 4. Files changed

- `docs/reports/0109-review-detached-validation-loop-sprint.md`

## 5. Commands run and exact outputs

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 109 numbered report(s).

$ git diff --check
<no output>

$ cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests mir_semantics
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 6. Evidence / findings

- blocking finding は local evidence からは見つからなかった。
- residual risk は 2 つ残る。
  1. `target/current-l2-detached/` を current non-production default candidate とした判断は妥当だが、final path policy と誤読されないよう今後も docs で OPEN を維持する必要がある。
  2. `scripts/current_l2_detached_loop.py` は現時点で十分に薄いが、将来 aggregate export まで抱え込ませると helper boundary が崩れやすい。

## 7. Changes in understanding

- detached validation-loop sprint は、現段階では helper boundary を壊さずに運用入口まで進められている。
- ただし review automation が返らない場合の close 手順は、今後も report へ残す必要がある。

## 8. Open questions

- reviewer completion が返らない session で retry / fallback をどう標準化するか
- aggregate exporter sketch を wrapper 側へ持たせるか、別 helper に分けるか

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、必ずリポジトリ内の文書とコードを正本として扱ってください。

今回は、detached validation-loop sprint 後の next narrow step として、
BatchRunSummary を起点にした non-production aggregate exporter sketch を追加し、
target/current-l2-detached/aggregates/<run-label>/batch-summary.detached.json
までを current helper boundary を壊さずに通してください。
```
