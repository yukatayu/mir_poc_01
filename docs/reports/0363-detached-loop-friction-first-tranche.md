# Report 0363 — detached loop friction first tranche

- Date: 2026-04-09T04:00:00Z
- Author / agent: Codex
- Scope: detached validation loop friction reduction の first tranche
- Decision levels touched: L2

## 1. Objective

- `tasks.md` の最優先 package である detached validation loop friction reduction を narrow に 1 段進める。
- current helper を production API にせず、`scripts/` の薄い改善だけで loop throughput を上げる。

## 2. Scope and assumptions

- current L2 core semantics、parser grammar、failure family、machine-check policy は変えない。
- 触るのは detached validation loop helper とその tests、docs mirror に限る。
- current tranche では、初歩的 friction に絞る。
  - generic run label を毎回手で付ける必要
  - fixture / fixture directory typo が helper ではなく深い subprocess / IO error に見えること

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
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_diff_detached_artifacts.py`
- `scripts/tests/test_current_l2_detached_loop.py`

## 4. Actions taken

- current helper を実地 smoke で回し、`smoke-fixture` が label 指定無しだと generic naming に寄りやすいこと、reference fixture typo が helper 内の意図とずれたエラーになりやすいことを確認した。
- `scripts/tests/test_current_l2_detached_loop.py` に次の failing tests を追加した。
  - missing fixture を subprocess 前に reject すること
  - `compare-fixtures` が label omitted 時に fixture stem から run label を導出すること
  - `smoke-fixture` が label omitted 時に fixture stem から run label を導出すること
- `scripts/current_l2_detached_loop.py` に minimal implementation を入れた。
  - `default_run_label_for_fixture`
  - `effective_run_label`
  - `emit_fixture` / `emit_aggregate` / `emit_static_gate` の fail-fast input validation
  - `compare-fixtures` / `smoke-fixture` / `emit-fixture` / `emit-static-gate` の label omitted 時 stem 由来 fallback
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md` に、この helper convenience の current cut を追記した。
- `tasks.md` と `progress.md` に、Task A の current baseline がここまで進んだことを反映した。

## 5. Files changed

- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `tasks.md`
- `progress.md`
- `docs/reports/0363-detached-loop-friction-first-tranche.md`

## 6. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-09 12:59 JST`
- `df -h .`
  - `/dev/vda2 99G / 90G used / 5.0G avail / 95%`
- `free -h`
  - `Mem: 960Mi total / 777Mi used / 83Mi free / 182Mi available`
- `python3 -m unittest scripts.tests.test_current_l2_detached_loop.DetachedLoopPathTests.test_emit_fixture_rejects_missing_fixture_before_spawning_subprocess scripts.tests.test_current_l2_detached_loop.DetachedLoopPathTests.test_compare_fixtures_derives_labels_from_fixture_stems_when_omitted scripts.tests.test_current_l2_detached_loop.DetachedLoopPathTests.test_smoke_fixture_derives_labels_from_fixture_stems_when_omitted`
  - RED: `FAIL/ERROR` を確認
  - GREEN 後: `Ran 3 tests ... OK`
- `python3 -m unittest scripts.tests.test_current_l2_detached_loop`
  - `Ran 13 tests ... OK`
- `python3 scripts/current_l2_detached_loop.py smoke-fixture e3-option-admit-chain --reference-fixture e6-write-after-expiry --overwrite`
  - shorthand fixture argument で bundle / aggregate smoke を完走
  - bundle artifact path:
    - `.../bundles/e3-option-admit-chain/e3-option-admit-chain.detached.json`
    - `.../bundles/e6-write-after-expiry/e6-write-after-expiry.detached.json`
  - aggregate artifact path:
    - `.../aggregates/e3-option-admit-chain-full/batch-summary.detached.json`
    - `.../aggregates/e3-option-admit-chain-single/batch-summary.detached.json`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 364 numbered report(s).`
- `git diff --check`
  - no output

## 7. Evidence / outputs / test results

- current helper baseline で、`smoke-fixture` / `compare-fixtures` の run label を毎回手で決めなくても fixture stem ベースで回せるようになった。
- missing fixture / missing fixture directory は helper 側で fail-closed になった。
- 実地 smoke でも shorthand fixture argument と stem-derived label の組み合わせで current loop を完走できた。
- current tranche では aggregate compare の noisy diff 自体は残しており、next friction は
  - full directory aggregate と single-fixture aggregate の差分ノイズ
  - reference update / compare triage の長さ
  に寄っている。

## 8. What changed in understanding

- detached validation loop の first friction は、artifact schema そのものよりも **naming / path / typo triage** の operational costだった。
- ここを先に潰すと、次の tranche で aggregate-noise や reference update flow を見やすくなる。

## 9. Open questions

- `smoke-fixture` の aggregate compare noise を suppress するのか、informational compare として残すのか。
- reference artifact の update / bless 相当を helper に入れるか、それとも compare-only のまま保つか。
- run label derivation を aggregate / static gate 側でもさらに統一するか。
- plan/ 更新不要

## 10. Suggested next prompt

detached validation loop friction reduction の次 tranche として、aggregate compare noise と reference update flow のどちらを先に薄く改善するかを narrow comparison してください。
