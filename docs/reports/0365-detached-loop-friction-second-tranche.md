# Report 0365 — detached loop friction second tranche

- Date: 2026-04-09T04:18:24.006627Z
- Author / agent: Codex
- Scope: detached validation loop friction reduction の second tranche
- Decision levels touched: L2

## 1. Objective

- `tasks.md` の最優先 package である detached validation loop friction reduction を、first tranche の先へ narrow に 1 段進める。
- noisy な `smoke-fixture` aggregate contrast と、fixture-to-fixture aggregate compare を分離し、継続的 validation loop の triage を短くする。
- `tasks.md` の rough estimate table に phase 情報を持たせる user rule を repository maintenance policy に反映する。

## 2. Scope and assumptions

- current L2 core semantics、parser grammar、failure family、machine-check policy は変えない。
- detached loop helper の non-production 改善だけを扱い、production exporter API や final path policy は固定しない。
- task 開始時の dirty state は clean だった。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `AGENTS.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `docs/reports/0363-detached-loop-friction-first-tranche.md`

## 4. Actions taken

- first tranche の residual friction を確認し、aggregate compare noise と reference update flow のうち、先に `single-fixture aggregate 同士の direct compare convenience` を切るのが自然だと判断した。
- TDD で `scripts/tests/test_current_l2_detached_loop.py` に failing test を追加し、`compare-fixture-aggregates` が
  - fixture stem shorthand を使えること
  - `<fixture-stem>-single` default run label を導出すること
  - `.host-plan.json` sidecar を temporary single-fixture directory に複製すること
  を RED で確認した。
- `scripts/current_l2_detached_loop.py` に minimal implementation を追加した。
  - `default_single_fixture_aggregate_run_label`
  - `emit_single_fixture_aggregate`
  - `command_compare_fixture_aggregates`
  - parser subcommand `compare-fixture-aggregates`
- `specs/examples/26` と `specs/examples/28` に current second tranche の helper cut を mirror した。
- `plan/07`、`plan/09`、`plan/90` に helper stack / provenance の mirror を追加した。
- `AGENTS.md` と `plan/91` に、`tasks.md` の rough estimate table へ phase 情報を短く添える維持ルールを追加した。
- `tasks.md` と `progress.md` に current detached loop baseline を反映した。

## 5. Files changed

- `AGENTS.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `progress.md`
- `tasks.md`
- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `docs/reports/0365-detached-loop-friction-second-tranche.md`

## 6. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-09 12:59 JST`
  - `2026-04-09 13:17 JST`
- `df -h .`
  - `/dev/vda2 99G / 90G used / 5.0G avail / 95%`
- `free -h`
  - `Mem: 960Mi total / 777Mi used / 83Mi free / 182Mi available`
- `python3 -m unittest scripts.tests.test_current_l2_detached_loop.DetachedLoopPathTests.test_compare_fixture_aggregates_derives_single_labels_and_copies_sidecars`
  - RED: `AttributeError: module 'current_l2_detached_loop' has no attribute 'command_compare_fixture_aggregates'`
  - GREEN 後: `Ran 1 test ... OK`
- `python3 -m unittest scripts.tests.test_current_l2_detached_loop`
  - `Ran 14 tests ... OK`
- `python3 scripts/current_l2_detached_loop.py compare-fixture-aggregates e3-option-admit-chain e6-write-after-expiry --overwrite`
  - left aggregate artifact:
    - `.../aggregates/e3-option-admit-chain-single/batch-summary.detached.json`
  - right aggregate artifact:
    - `.../aggregates/e6-write-after-expiry-single/batch-summary.detached.json`
  - `summary_core: typed aggregate core matched`
  - reference-only difference は `aggregate_context.directory_path` だけ
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 366 numbered report(s).`
- `git diff --check`
  - no output

## 7. Evidence / outputs / test results

- detached loop の second tranche で、`smoke-fixture` の full-vs-single contrast は残しつつ、fixture-to-fixture aggregate compare は `compare-fixture-aggregates` に分離できた。
- actual shorthand smoke でも `e3` と `e6` の single-fixture aggregate `summary_core` は一致し、difference の大半が `aggregate_context` provenance に留まることが確認できた。
- これにより、aggregate compare noise のうち「single fixture 同士を比べたいのに full directory が混ざる」経路は thin helper で切れた。
- current remaining friction は
  - reference update / bless 相当を helper にどこまで入れるか
  - compare output の longer triage をどこまで短くするか
  に寄っている。

## 8. What changed in understanding

- detached validation loop の next friction は aggregate compare そのものより、`reference update` と `longer compare triage` に寄っている。
- single-fixture aggregate compare は full-vs-single contrast から切り離しても helper boundary を壊さず、production aggregate API の既成事実化にもつながらない。
- `tasks.md` の rough estimate table は phase 情報があると current mainline と reserve path の読み違いを減らせる。

## 9. Open questions

- `reference update / bless` 相当を detached loop helper に薄く入れるか、それとも compare-only を維持するか。
- current compare helper の output triage を、summary-first / reference-only folded display に寄せるか。
- `compare-fixture-aggregates` の run label policy を `<stem>-single` からさらに一般化するか。

## 10. Suggested next prompt

detached validation loop friction reduction の次 tranche として、reference update flow と longer compare triage のどちらを先に薄く改善するかを narrow comparison してください。
