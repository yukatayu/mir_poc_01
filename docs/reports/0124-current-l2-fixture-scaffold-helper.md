# Report 0124 — current L2 fixture scaffold helper

## 1. Title and identifier

- Report 0124
- current L2 fixture scaffold helper

## 2. Objective

current L2 parser-free PoC の fixture authoring / elaboration bottleneck のうち、
boilerplate 作成だけを non-production helper へ切り出す。

今回の goal は、hand-written fixture を正本に保ったまま、

- runtime / static-only の minimal skeleton を作る
- runtime fixture なら empty `.host-plan.json` sidecar 骨格を作る
- output を `target/current-l2-fixture-scaffolds/` 下へ fail-closed default で置く

ための thin scaffold helper と、その docs / plan mirror を追加することである。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- helper は semantics inference を行わない。
- helper は completed `program` や completed expectation を作らない。
- hand-written fixture と reviewed expectation を引き続き正本に保つ。
- `plan/` は relevant mirror を更新する。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/07-current-l2-host-stub-harness.md`
11. `specs/examples/08-current-l2-host-plan-schema.md`
12. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
13. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
14. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
15. `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
16. `plan/07-parser-free-poc-stack.md`
17. `plan/09-helper-stack-and-responsibility-map.md`
18. `plan/11-roadmap-near-term.md`
19. `plan/12-open-problems-and-risks.md`
20. `plan/15-current-l2-fixture-authoring-template.md`
21. `plan/90-source-traceability.md`
22. `docs/reports/0106-detached-exporter-consolidation-sprint.md`
23. `docs/reports/0108-detached-validation-loop-sprint.md`
24. `docs/reports/0122-current-l2-detached-aggregate-compare-helper.md`
25. `scripts/current_l2_detached_loop.py`
26. `crates/mir-ast/tests/fixtures/current-l2/`

## 5. Actions taken

1. fixture authoring bottleneck のうち、repo tracked fixture directory に直接書き込まない最小 cut を再確認した。
2. `scripts/tests/test_current_l2_scaffold_fixture.py` を追加し、
   - runtime scaffold が fixture JSON と host plan sidecar を作ること
   - static-only scaffold が fixture JSON のみを作ること
   - `--overwrite` 無しでは既存 target を fail-closed に弾くこと
   - `--fixture-id` override が効くこと
   を確認する unit test を用意した。
3. `scripts/current_l2_scaffold_fixture.py` を追加し、`target/current-l2-fixture-scaffolds/` を default candidate とする non-production scaffold helper を実装した。
4. `specs/examples/27-current-l2-fixture-scaffold-helper.md` を追加し、helper boundary と OPEN を docs-only で整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`progress.md` を更新し、scaffold helper と current progress を mirror した。
6. smoke として runtime / static-only scaffold を `target/current-l2-fixture-scaffolds/` 下へ 1 本ずつ生成し、default candidate と fail-closed overwrite policy が使えることを確認した。

## 6. Evidence / outputs / test results

### unit tests

```text
python3 -m unittest scripts/tests/test_current_l2_scaffold_fixture.py
....
OK
```

### smoke

```text
python3 scripts/current_l2_scaffold_fixture.py e14-smoke-runtime --kind runtime --output-dir target/current-l2-fixture-scaffolds --overwrite
target/current-l2-fixture-scaffolds/e14-smoke-runtime.json
target/current-l2-fixture-scaffolds/e14-smoke-runtime.host-plan.json
```

```text
python3 scripts/current_l2_scaffold_fixture.py e15-smoke-static --kind static-only --output-dir target/current-l2-fixture-scaffolds --overwrite
target/current-l2-fixture-scaffolds/e15-smoke-static.json
```

### docs / diff validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 124 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- fixture authoring bottleneck は、completed expectation を自動化しなくても boilerplate cut だけで実務負担をかなり下げられる。
- `target/current-l2-fixture-scaffolds/` を default candidate にすると、incomplete placeholder を repo tracked fixture catalog と混同しにくい。
- runtime / static-only の別と empty sidecar skeleton までは helper が持っても hidden acceptance になりにくいが、それ以上を埋め始めると semantics inference と authoring review が混線しやすい。

## 8. Open questions

- scaffold helper を detached loop wrapper へ統合するべきか、それとも別 helper のまま保つべきか。
- scaffold placeholder naming を長期固定するか。
- `target/current-l2-fixture-scaffolds/` の cleanup / retention policy をどこまで formalize するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、scaffold helper を使って runtime regression 候補を 1 本起こし、hand-written fixture として完成させたうえで bundle artifact / aggregate artifact を保存・比較する detached validation-loop 実地反復を行ってください。`
