# Report 0131 — review current L2 detached fixture validation-loop helper

## 1. Title and identifier

- Report 0131
- review current L2 detached fixture validation-loop helper

## 2. Objective

Report 0130 とその差分について review を行い、`smoke-fixture` helper が

- current helper boundary を壊していないか
- compare exit code policy を過剰に既成事実化していないか
- fixture authoring / exporter / batch responsibility を混線させていないか

を確認する。

## 3. Scope and assumptions

- 対象は `scripts/current_l2_detached_loop.py` の `smoke-fixture` subcommand、関連 unit test、spec / plan / progress mirror に限る。
- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- helper は non-production convenience に留める。
- `plan/` は relevant mirror と traceability を同 task で更新する。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
9. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
10. `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
11. `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
12. `plan/07-parser-free-poc-stack.md`
13. `plan/09-helper-stack-and-responsibility-map.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/15-current-l2-fixture-authoring-template.md`
17. `plan/90-source-traceability.md`
18. `progress.md`
19. `docs/reports/0130-current-l2-detached-fixture-validation-loop-helper.md`
20. `scripts/current_l2_detached_loop.py`
21. `scripts/tests/test_current_l2_detached_loop.py`

## 5. Actions taken

1. current session の tool surface を確認したが、reviewer dispatch / completion retrieval tool は exposed されていなかった。
2. そのため local diff review に切り替え、次を重点的に確認した。
   - `smoke-fixture` が existing emitter / diff helper を再利用しているだけで、新しい semantics を導入していないこと
   - compare exit code `1` を informational difference として扱い、`2` 以上だけを helper failure として返していること
   - aggregate smoke が full directory と single-fixture temporary directory の比較に留まり、final path policy を既成事実化していないこと
   - docs / plan mirror が production API と誤読しない wording になっていること
3. local review の結果、blocking issue は見つからなかった。

## 6. Evidence / outputs / test results

```text
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
.....
OK
```

```text
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label e11-smoke --reference-label e3-reference --overwrite
...
payload_core differences:
- payload_core.event_kinds: left=["perform-failure", "perform-success"] right=["perform-success"]
- payload_core.non_admissible_metadata: left=[] right=[{"option_ref": "owner_writer", "subreason": "admit-miss"}]
...
summary_core differences:
- summary_core.total_bundles: left=11 right=1
- summary_core.runtime_bundles: left=9 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=11 right=1
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 131 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- fixture smoke helper は detached validation loop の common path を減らすが、existing bundle / aggregate compare contract を再利用する限り helper boundary を大きく壊さない。
- compare exit code `1` を informational difference として扱う cut は、new fixture 追加直後の smoke loop に自然であり、exact-compare core 自体の定義を崩さない。

## 8. Open questions

- reviewer tool surface が戻った session では、この helper を reviewer subagent でも再確認する価値がある。
- `smoke-fixture` を別 script に分けるかどうかは、subcommand 数がさらに増えるまで OPEN に残してよい。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を 1 段だけ operational に寄せるか、あるいは parser 前 inventory を narrow に作り始めるかを source-backed に比較してください。`
