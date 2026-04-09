# Report 0367 — detached loop friction third tranche

## 1. Title and identifier

- Report ID: 0367
- Title: detached loop friction third tranche

## 2. Objective

- `tasks.md` の Task A で残っていた `longer compare triage` を narrow に進める。
- bundle / aggregate / static gate の diff helper で、reference-only section の whole-section blob 表示を shallow per-field summary に揃える。
- detached validation loop の self-driven friction reduction を current checkpoint で close できるかを判断する。

## 3. Scope and assumptions

- Scope は detached validation loop の non-production helper に限定する。
- exact-compare core は変えない。
- `reference update / bless` は今回扱わず、path policy / retention policy と接続する later candidate に残す。
- `plan/` と `progress.md` と `tasks.md` は current snapshot に合わせて更新する。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/reports/0365-detached-loop-friction-second-tranche.md`
- `docs/reports/0366-review-detached-loop-friction-second-tranche.md`

## 5. Actions taken

- `scripts/tests/test_current_l2_diff_detached_aggregates.py` に、aggregate helper が reference-only section を field 単位で出す failing test を追加した。
- `scripts/tests/test_current_l2_diff_static_gate_artifacts.py` の expectation を、whole-section blob ではなく `detached_noncore.reason_codes` 単位の shallow summary に更新した。
- `scripts/tests/test_current_l2_diff_detached_artifacts.py` を新規追加し、bundle helper の reference-only section が field 単位で出ることを red から確認した。
- `scripts/current_l2_diff_detached_aggregates.py`
  - `aggregate_context` / `detached_noncore` の reference-only section を shallow per-field summary で出すようにした。
- `scripts/current_l2_diff_detached_artifacts.py`
  - `bundle_context` / `detached_noncore` の reference-only section を shallow per-field summary で出すようにした。
  - `main(argv=None)` へ揃えて unit test しやすい形にした。
- `scripts/current_l2_diff_static_gate_artifacts.py`
  - `fixture_context` / `detached_noncore` の reference-only section を shallow per-field summary で出すようにした。
- detached loop の docs / plan / progress / task snapshot を、third tranche と Task A checkpoint close に合わせて更新した。

## 6. Evidence / outputs / test results

### Targeted RED evidence

- `python3 -m unittest scripts.tests.test_current_l2_diff_detached_aggregates.DetachedAggregateDiffTests.test_main_prints_reference_only_differences_per_field`
  - 実装前は `aggregate_context` whole-section blob を出して FAIL
- `python3 -m unittest scripts.tests.test_current_l2_diff_detached_artifacts.DetachedArtifactDiffTests.test_main_prints_reference_only_differences_per_field`
  - 実装前は `main()` の argv 非対応で ERROR
- `python3 -m unittest scripts.tests.test_current_l2_diff_static_gate_artifacts.StaticGateDiffTests.test_main_keeps_detached_noncore_reason_code_diff_reference_only`
  - 実装前は `detached_noncore` whole-section blob を出して FAIL

### GREEN / regression evidence

- `python3 -m unittest scripts.tests.test_current_l2_diff_detached_aggregates scripts.tests.test_current_l2_diff_detached_artifacts scripts.tests.test_current_l2_diff_static_gate_artifacts scripts.tests.test_current_l2_detached_loop`
  - `Ran 23 tests ... OK`
- `python3 scripts/current_l2_detached_loop.py compare-fixture-aggregates e3-option-admit-chain e6-write-after-expiry --overwrite`
  - `summary_core: typed aggregate core matched`
  - reference-only は `aggregate_context.directory_path` の shallow summary だけを表示
- `python3 scripts/current_l2_detached_loop.py smoke-fixture e3-option-admit-chain --reference-fixture e6-write-after-expiry --overwrite`
  - bundle compare では `bundle_context.fixture_id` / `fixture_path` / `host_plan_path` / `steps_executed` を shallow summary として表示
  - aggregate compare では full-vs-single contrast を informational difference として維持
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 366 numbered report(s).`
- `git diff --check`
  - 無出力

## 7. What changed in understanding

- detached validation loop の `longer compare triage` は、exact-compare core を変えずに shallow reference-only summary を入れるだけでかなり短くできる。
- これにより Task A の self-driven portion は
  - fixture shorthand / fail-fast
  - default run labels
  - `compare-fixture-aggregates`
  - shallow reference-only triage
  までで checkpoint close と見なしてよい。
- current residual は `reference update / bless` だけに絞られ、これは path policy / retention policy と接続する別判断に残す方が boundary-safe である。

## 8. Open questions

- `reference update / bless` を detached loop helper と同じ layer に入れるか。
- overwrite / retention / path policy をどの timing で detached loop maintenance residual から actual API 候補へ上げるか。
- full-vs-single aggregate contrast を今後さらに薄くする必要があるか、それとも authoritative room baseline 側へ主線を移す方が良いか。

## 9. Suggested next prompt

`tasks.md` の current order に従い、Phase 4 前半の authoritative room baseline を docs-first で精密化してください。特に `authority-ack` / `single room authority` / `authoritative serial transition` / `authority_rng` の minimal practical bundle を、すごろく room のような concrete example で narrow comparison してください。
