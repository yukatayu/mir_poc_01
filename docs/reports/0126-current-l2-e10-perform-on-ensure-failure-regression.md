# Report 0126 — current L2 e10 perform-on ensure failure regression

## 1. Title and identifier

- Report 0126
- current L2 e10 perform-on ensure failure regression

## 2. Objective

current L2 parser-free PoC 基盤で、direct `PerformOn` の request-local `ensure` unsatisfied を
runtime `explicit_failure` として固定する narrow regression fixture を追加する。

今回の goal は、

- success-side carrier を返す effect があっても
- request-local `ensure` が unsatisfied なら tentative commit を適用せず
- `Reject` や non-admissible metadata を fabricated しない

ことを machine-check と detached validation loop の両方で確認することである。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- `PerformOn` の direct target request contract 読みだけを追加 regression で固定する。
- fallback / lease / chain semantics には広げない。
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
10. `specs/examples/04-current-l2-step-semantics.md`
11. `specs/examples/06-current-l2-interpreter-skeleton.md`
12. `plan/08-representative-programs-and-fixtures.md`
13. `plan/90-source-traceability.md`
14. `docs/reports/0121-current-l2-e9-monotone-degradation-success-regression.md`
15. `crates/mir-semantics/src/lib.rs`
16. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
17. `scripts/current_l2_detached_loop.py`
18. `scripts/current_l2_diff_detached_artifacts.py`
19. `scripts/current_l2_diff_detached_aggregates.py`

## 5. Actions taken

1. `PerformOn` の request-local `ensure` unsatisfied branch を current code anchor で再確認し、new fixture の scope を
   - direct target
   - `ensure` unsatisfied
   - effect verdict は success-side carrier
   - final outcome は `explicit_failure`
   に限定した。
2. `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json` と `.host-plan.json` を追加した。
3. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` を更新し、
   - broad runtime loops
   - directory / selection / profile summary counts
   - focused regression test
   を `e10` 前提に更新した。
4. `specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/06-current-l2-interpreter-skeleton.md`、`plan/08-representative-programs-and-fixtures.md`、`progress.md` を更新し、fixture catalog と current status を mirror した。
5. detached loop smoke として
   - `e1` vs `e10` の bundle compare
   - full fixture directory vs `e10` 単独 directory の aggregate compare
   を実行した。

## 6. Evidence / outputs / test results

### machine-check

```text
cargo test -p mir-semantics
...
running 35 tests
...
test perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata ... ok
...
test result: ok. 35 passed; 0 failed
```

### bundle detached compare smoke

```text
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json --left-label e1-ensure-smoke --right-label e10-ensure-smoke --overwrite
...
payload_core differences:
- payload_core.event_kinds: left=["perform-success", "atomic-cut", "perform-failure"] right=["perform-failure"]
```

### aggregate detached compare smoke

```text
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label agg-e10-full --overwrite
python3 scripts/current_l2_detached_loop.py emit-aggregate <tmp-e10-only-dir> --run-label agg-e10-only --overwrite
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-e10-full agg-e10-only
...
summary_core differences:
- summary_core.total_bundles: left=10 right=1
- summary_core.runtime_bundles: left=8 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=10 right=1
```

### docs / diff validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 126 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- direct `PerformOn` の request-local `ensure` unsatisfied は current L2 では non-admissible skip でも `Reject` でもなく、request contract failure として `explicit_failure` に落ちる。
- success-side carrier preview があっても、unsatisfied `ensure` があれば commit を適用しないという読みを narrow regression として固定できた。
- detached loop では bundle compare と aggregate compare の両方で `e10` を観測でき、validation loop 入口の実地反復がもう 1 本増えた。

## 8. Open questions

- `PerformVia` 側の request-local `ensure` unsatisfied variant を同じ粒度で追加するか。
- direct target ensure failure を representative prose examples へ昇格するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を docs-only から 1 段だけ operational に寄せるか、あるいは `PerformVia` 側の request-local ensure regression を追加するかを source-backed に比較してください。`
