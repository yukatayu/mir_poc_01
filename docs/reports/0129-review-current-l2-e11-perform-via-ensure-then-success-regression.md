# Report 0129 — review current L2 e11 perform-via ensure then success regression

## 1. Title and identifier

- Report 0129
- review current L2 e11 perform-via ensure then success regression

## 2. Objective

Report 0128 とその差分について reviewer を 1 回だけ依頼し、
via-chain の request-local `ensure` failure continuation regression が

- current L2 semantics と整合しているか
- focused machine-check が earlier preview suppression と later success commit を十分に押さえているか
- spec / plan / progress mirror の説明が evidence と一致しているか

を確認する。

## 3. Scope and assumptions

- 対象は `e11-perform-via-ensure-then-success` fixture、その sidecar、interpreter test、spec / plan / progress mirror に限る。
- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- via-chain の request-local `ensure` failure から later same-lineage success へ継続する branch だけを narrow regression として review する。
- `plan/` は Report 0128 時点で relevant mirror 更新済みのため、この review record では追加更新しない。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/02-current-l2-ast-fixture-schema.md`
9. `specs/examples/04-current-l2-step-semantics.md`
10. `specs/examples/06-current-l2-interpreter-skeleton.md`
11. `plan/08-representative-programs-and-fixtures.md`
12. `plan/90-source-traceability.md`
13. `progress.md`
14. `docs/reports/0128-current-l2-e11-perform-via-ensure-then-success-regression.md`
15. `crates/mir-semantics/src/lib.rs`
16. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
17. `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`
18. `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.host-plan.json`

## 5. Actions taken

1. reviewer subagent を 1 回だけ起動し、`e11` regression の semantic correctness、test coverage、mirror consistency を確認するよう依頼した。
2. reviewer completion を 600000ms で 2 回待機したが、どちらも timeout し completion は取得できなかった。
3. 規約どおり local evidence fallback に切り替え、次を確認した。
   - `e11` fixture が via-chain の earlier option `ensure` unsatisfied と later same-lineage success を最小で表していること
   - host plan sidecar が delegated / backup の `request-require` / `request-ensure` / success-side carrier を narrow に支えていること
   - focused regression が final `terminal_outcome = success` と final `place_store = {"profile_doc": ["write_profile@backup_writer"]}` を押さえていること
   - bundle / aggregate smoke、docs validation、`git diff --check` が green であること
4. local fallback の結果、current task は close 可能と判断した。

## 6. Evidence / outputs / test results

### reviewer wait result

```text
wait_agent(timeout=600000) -> timed_out
wait_agent(timeout=600000) -> timed_out
```

### local fallback evidence

```text
cargo test -p mir-semantics
...
running 36 tests
...
test perform_via_ensure_failure_can_continue_to_later_success ... ok
...
test result: ok. 36 passed; 0 failed
```

```text
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json --left-label e3-via-smoke --right-label e11-via-ensure-smoke --overwrite
...
payload_core differences:
- payload_core.event_kinds: left=["perform-success"] right=["perform-failure", "perform-success"]
- payload_core.non_admissible_metadata: left=[{"option_ref": "owner_writer", "subreason": "admit-miss"}] right=[]
```

```text
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-e11-full agg-e11-only
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
Found 128 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- via-chain の request-local `ensure` failure continuation は、direct `PerformOn` ensure failure と同じく tentative commit suppression を machine-check しておく方が安全である。
- reviewer completion が取れない環境でも、long wait を 2 回行った上で local evidence fallback を残せば、task close の根拠を保持できる。

## 8. Open questions

- via-chain ensure continuation variant を representative prose examples へ昇格するか。
- aggregate export の actual narrow API cut と fixture authoring helper のどちらを next に優先するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を operational に 1 段寄せるか、fixture authoring / elaboration の next narrow helper を追加するかを source-backed に比較してください。`
