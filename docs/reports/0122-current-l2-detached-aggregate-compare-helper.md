# Report 0122 — current L2 detached aggregate compare helper

## 1. Title and identifier

- Report 0122
- current L2 detached aggregate compare helper

## 2. Objective

current L2 parser-free PoC の detached validation loop で、aggregate detached artifact 2 本を
`summary_core` だけ exact-compare し、`aggregate_context` と `detached_noncore` は reference-only に留める
non-production helper と wrapper を narrow に追加する。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- aggregate detached artifact の actual narrow cut は `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md` に従う。
- `bundle_failure_kind_counts_scope = "migrated-kinds-only"` を current partial histogram の scope marker として exact-compare core に残す。
- production compare API、profile-aware aggregate compare、final path policy は固定しない。
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
9. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
10. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
11. `plan/07-parser-free-poc-stack.md`
12. `plan/09-helper-stack-and-responsibility-map.md`
13. `plan/11-roadmap-near-term.md`
14. `plan/12-open-problems-and-risks.md`
15. `plan/15-current-l2-fixture-authoring-template.md`
16. `plan/90-source-traceability.md`
17. `docs/reports/0118-detached-validation-loop-actualization-and-progress-refresh.md`
18. `docs/reports/0119-review-detached-validation-loop-actualization-and-progress-refresh.md`
19. `scripts/current_l2_detached_loop.py`
20. `scripts/current_l2_diff_detached_artifacts.py`
21. `scripts/tests/test_current_l2_detached_loop.py`
22. `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`

## 5. Actions taken

1. current aggregate emitter sketch と review `0119` を読み直し、actual next step を
   - `summary_core` only compare
   - `aggregate_context` / `detached_noncore` reference-only
   - run-label convenience wrapper
   に限定した。
2. `scripts/tests/test_current_l2_diff_detached_aggregates.py` を追加し、
   - typed count difference を検出すること
   - reference-only section を compare core に含めないこと
   - `summary_core` 欠落時に fail すること
   - identical aggregate artifact なら exit code 0 を返すこと
   を red から確認した。
3. `scripts/tests/test_current_l2_detached_loop.py` に `compare-aggregates` wrapper の run-label path 導出 test を追加して red を作った。
4. `scripts/current_l2_diff_detached_aggregates.py` を追加し、`summary_core` compare と reference-only print を実装した。
5. `scripts/current_l2_detached_loop.py` に `compare-aggregates` subcommand を追加し、`aggregate_artifact_path()` を再利用して run label から artifact path を導出する薄い wrapper を実装した。
6. `specs/examples/26-current-l2-detached-aggregate-compare-helper.md` を追加し、aggregate compare helper の docs-only boundary を 1 箇所に整理した。
7. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`progress.md` を更新し、aggregate compare helper と current remaining tasks を mirror した。

## 6. Evidence / outputs / test results

### failing tests first

```text
python3 -m unittest scripts/tests/test_current_l2_diff_detached_aggregates.py
...
ModuleNotFoundError: No module named 'current_l2_diff_detached_aggregates'
```

```text
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
...
AttributeError: module 'current_l2_detached_loop' has no attribute 'command_compare_aggregates'
```

### green tests

```text
python3 -m unittest scripts/tests/test_current_l2_diff_detached_aggregates.py
....
OK
```

```text
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
...
OK
```

```text
cargo test -p mir-semantics --example current_l2_emit_detached_aggregate
...
test result: ok. 2 passed; 0 failed
```

### aggregate compare smoke

same-run-content compare:

```text
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label agg-left --overwrite
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label agg-right --overwrite
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-left agg-right
...
summary_core: typed aggregate core matched
```

coverage-failure diff compare:

```text
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-left agg-coverage
...
summary_core differences:
- summary_core.total_bundles: left=9 right=1
- summary_core.runtime_bundles: left=7 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=9 right=0
- summary_core.failed: left=0 right=1
- summary_core.bundle_failure_kind_counts: left=[] right=[{"failure_kind": "host-plan-coverage-failure", "count": 1}]
```

### docs / diff validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 122 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- aggregate compare は current phase では `summary_core` だけを exact-compare すれば十分であり、`aggregate_context` / `detached_noncore` を core に上げる必要はない。
- `bundle_failure_kind_counts_scope = "migrated-kinds-only"` は current aggregate helper で full histogram 誤読を防ぐ重要な compare field である。
- run-label から aggregate artifact path を導出する convenience は、current detached validation loop の non-production phase では許容できるが、まだ production compare API ではない。

## 8. Open questions

- aggregate compare helper を `harness.rs` / `lib.rs` の public behavior に上げるか。
- aggregate compare を profile / named profile summary へ広げる timingをどう切るか。
- `bundle_failure_kind_counts` を row list のまま比較し続けるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、fixture authoring / elaboration template を使って runtime regression をもう 1 本追加し、bundle artifact と aggregate artifact を保存して compare する detached validation-loop 実地反復を行ってください。progress.md と relevant plan mirror も更新してください。`
