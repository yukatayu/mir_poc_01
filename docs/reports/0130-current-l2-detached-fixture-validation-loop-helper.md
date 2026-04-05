# Report 0130 — current L2 detached fixture validation-loop helper

## 1. Title and identifier

- Report 0130
- current L2 detached fixture validation-loop helper

## 2. Objective

current L2 parser-free PoC の detached validation loop で、
1 fixture を追加した直後の common path を 1 command で回せる non-production helper を追加する。

今回の goal は、

- target fixture の bundle artifact を保存する
- optional な reference fixture compare を payload core だけで行う
- full directory aggregate と single-fixture aggregate の smoke compare を行う

までを thin wrapper にまとめつつ、
production exporter API や final path policy を既成事実化しないことである。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- helper は `scripts/current_l2_detached_loop.py` の non-production subcommand として追加する。
- bundle / aggregate emitter、bundle / aggregate diff helper の current boundary を再利用する。
- compare helper の exit code `1` は informational difference として扱い、helper failure だけを non-zero で返す。
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
9. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
10. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
11. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
12. `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
13. `specs/examples/27-current-l2-fixture-scaffold-helper.md`
14. `plan/00-index.md`
15. `plan/07-parser-free-poc-stack.md`
16. `plan/09-helper-stack-and-responsibility-map.md`
17. `plan/11-roadmap-near-term.md`
18. `plan/12-open-problems-and-risks.md`
19. `plan/15-current-l2-fixture-authoring-template.md`
20. `plan/90-source-traceability.md`
21. `docs/reports/0108-detached-validation-loop-sprint.md`
22. `docs/reports/0122-current-l2-detached-aggregate-compare-helper.md`
23. `docs/reports/0124-current-l2-fixture-scaffold-helper.md`
24. `scripts/current_l2_detached_loop.py`
25. `scripts/tests/test_current_l2_detached_loop.py`

## 5. Actions taken

1. detached validation loop の current wrapper を再確認し、existing `emit-fixture` / `compare-fixtures` / `emit-aggregate` / `compare-aggregates` を束ねる common path だけを切り出す方針を選んだ。
2. `scripts/current_l2_detached_loop.py` に `smoke-fixture` subcommand を追加した。
   - target fixture の bundle artifact を emit
   - optional reference fixture があれば bundle compare
   - parent directory 全体の aggregate artifact を emit
   - target fixture だけを一時 directory へ複製した single-fixture aggregate artifact を emit
   - aggregate compare helper を実行
   - compare helper の exit code `1` は informational difference として許容
3. `scripts/tests/test_current_l2_detached_loop.py` を更新し、
   - `smoke-fixture` が bundle / aggregate path を正しく導出すること
   - compare helper が `1` を返しても helper 全体は `0` を返すこと
   - compare helper の `2` 以上は helper failure として伝播すること
   を unit test で固定した。
4. `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md` を追加し、helper boundary と exit code policy を docs-only で整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/00`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`progress.md` を更新し、current helper boundary と near-term understanding を mirror した。
6. smoke として `e11` を target fixture、`e3` を reference fixture にして `smoke-fixture` を実行し、1 command で bundle compare と aggregate smoke が回ることを確認した。

## 6. Evidence / outputs / test results

### unit tests

```text
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
.....
OK
```

### CLI surface

```text
python3 scripts/current_l2_detached_loop.py --help
...
{emit-fixture,emit-aggregate,compare-artifacts,compare-aggregates,compare-fixtures,smoke-fixture}
...
smoke-fixture       1 fixture を bundle-first detached loop に載せ、必要なら
                    reference fixture compare と single-fixture aggregate
                    smoke までまとめて回す
```

### smoke

```text
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label e11-smoke --reference-label e3-reference --overwrite
...
fixture artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/e11-smoke/e11-perform-via-ensure-then-success.detached.json
reference artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/e3-reference/e3-option-admit-chain.detached.json
...
payload_core differences:
- payload_core.event_kinds: left=["perform-failure", "perform-success"] right=["perform-success"]
- payload_core.non_admissible_metadata: left=[] right=[{"option_ref": "owner_writer", "subreason": "admit-miss"}]
...
aggregate artifact (full)  : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e11-smoke-full/batch-summary.detached.json
aggregate artifact (single): /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e11-smoke-single/batch-summary.detached.json
...
summary_core differences:
- summary_core.total_bundles: left=11 right=1
- summary_core.runtime_bundles: left=9 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=11 right=1
```

## 7. What changed in understanding

- fixture authoring bottleneck は scaffold helper だけでも下がるが、detached loop の common path を 1 command にまとめると「追加して回す」反復の認知コストをさらに下げられる。
- compare helper の exit code `1` を informational difference として扱う cut は、new fixture を足した直後の smoke loop に自然である。
- それでも wrapper は production CLI ではなく、final path policy や actual exporter API finalization は引き続き OPEN に残すべきである。

## 8. Open questions

- `smoke-fixture` を既存 wrapper の subcommand に留めるか、別 helper に分けるか。
- aggregate smoke の default compare partner を full directory 以外にも広げるか。
- final storage / retention policy をどこで固定するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を 1 段だけ operational に寄せるか、あるいは parser 前 inventory を narrow に作り始めるかを source-backed に比較してください。`
