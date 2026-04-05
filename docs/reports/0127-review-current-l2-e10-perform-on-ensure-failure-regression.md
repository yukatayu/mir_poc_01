# Report 0127 — review current L2 e10 perform-on ensure failure regression

## 1. Title and identifier

- Report 0127
- review current L2 e10 perform-on ensure failure regression

## 2. Objective

Report 0126 とその差分について reviewer を 1 回だけ依頼し、
completion が取得できない環境では local evidence fallback で task close 可否を判断する。

## 3. Scope and assumptions

- 対象は `e10-perform-on-ensure-failure` fixture、その sidecar、interpreter test、spec / plan / progress mirror に限る。
- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- direct `PerformOn` の request-local `ensure` unsatisfied 読みだけを narrow regression として review する。
- `plan/` は Report 0126 時点で relevant mirror 更新済みのため、この review record では追加更新しない。

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
14. `docs/reports/0126-current-l2-e10-perform-on-ensure-failure-regression.md`
15. `crates/mir-semantics/src/lib.rs`
16. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
17. `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
18. `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.host-plan.json`

## 5. Actions taken

1. reviewer subagent に対し、`e10` regression の semantic correctness、test coverage、mirror consistency、progress accuracy を確認するよう依頼した。
2. ただし current environment では reviewer completion を待つ専用 tool が使えないため、依頼後は local evidence fallback に切り替えた。
3. local fallback として次を確認した。
   - `e10` fixture が direct `PerformOn` の `request-ensure` unsatisfied を `explicit_failure` として表現していること
   - host plan sidecar が `request-require=satisfied`、`request-ensure=unsatisfied`、effect `success` という branch を最小で支えていること
   - interpreter tests が broad runtime loops と focused regression の両方で `e10` を拾っていること
   - detached bundle / aggregate smoke が current loop で通ること
   - docs validation と `git diff --check` が green であること
4. local fallback の結果、current task は close 可能と判断した。

## 6. Evidence / outputs / test results

### reviewer fallback note

```text
reviewer was requested once, but this session did not expose a completion-wait tool.
task close therefore used local evidence fallback.
```

### local fallback evidence

```text
cargo test -p mir-semantics
...
running 35 tests
...
test perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata ... ok
...
test result: ok. 35 passed; 0 failed
```

```text
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json --left-label e1-ensure-smoke --right-label e10-ensure-smoke --overwrite
...
payload_core differences:
- payload_core.event_kinds: left=["perform-success", "atomic-cut", "perform-failure"] right=["perform-failure"]
```

```text
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-e10-full agg-e10-only
...
summary_core differences:
- summary_core.total_bundles: left=10 right=1
- summary_core.runtime_bundles: left=8 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=10 right=1
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 127 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- direct `PerformOn` の `ensure` failure は current L2 で request contract failure として narrow に固定でき、fallback / non-admissible / `Reject` family と混線させずに扱える。
- detached validation loop は、new runtime regression を追加したあとでも bundle / aggregate compare をそのまま再利用できる。

## 8. Open questions

- `PerformVia` 側の request-local `ensure` unsatisfied variant をいつ追加するか。
- direct target ensure failure を representative prose examples へ昇格させるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を 1 段だけ operational に寄せるか、あるいは PerformVia 側の request-local ensure regression を追加するかを source-backed に比較してください。`
