# Report 0127 — review current L2 e10 perform-on ensure failure regression

## 1. Title and identifier

- Report 0127
- review current L2 e10 perform-on ensure failure regression

## 2. Objective

Report 0126 とその差分について reviewer を 1 回だけ依頼し、
direct `PerformOn` の request-local `ensure` unsatisfied regression が

- semantic boundary を壊していないか
- tentative commit suppression を十分に machine-check できているか
- mirror / progress の説明が evidence と一致しているか

を確認する。

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

1. reviewer subagent を 1 回だけ起動し、`e10` regression の semantic correctness、test coverage、mirror consistency、progress wording を確認するよう依頼した。
2. reviewer completion が返るまで長めに待ち、completion から次の 2 finding を得た。
   - **medium**: focused regression が `explicit_failure` / event / metadata しか確認しておらず、success-side carrier preview が final `place_store` に commit されていないことを machine-check していない。
   - **low**: Report 0126 と `progress.md` が commit suppression まで確認済みと読める wording だが、当時の evidence ではそこまで言えていなかった。
3. finding への対応として、
   - `DirectStyleEvaluator` と tiny predicate / effect oracle helper を使う focused assertion を追加し、final `place_store` が空のままであることを確認するよう修正した。
   - Report 0126 の machine-check / docs validation evidence を current suite counts に合わせて更新した。
4. 修正後の diff を local で再確認し、task close 可能と判断した。

## 6. Evidence / outputs / test results

### reviewer completion 要約

```text
finding 1 (medium):
the regression does not actually prove that the previewed success-side commit is discarded.
it only proves explicit_failure, events, and empty metadata/narrative.

finding 2 (low):
report/progress wording overstates the machine-check evidence because commit suppression is not asserted.
```

### fix summary

```text
focused regression now:
- reruns e10 with DirectStyleEvaluator
- asserts terminal_outcome == explicit_failure
- asserts final place_store.is_empty()
```

### local validation after fix

```text
cargo test -p mir-semantics
...
running 36 tests
...
test perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata ... ok
...
test result: ok. 36 passed; 0 failed
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

- direct `PerformOn` の `ensure` failure を current L2 の request contract failure として固定するには、terminal outcome だけでなく tentative commit suppression まで machine-check する方が安全である。
- detached validation loop の smoke evidence は useful だが、success-side preview suppression のような細い state claim は focused evaluator assertion で補うべきである。

## 8. Open questions

- `PerformVia` 側の request-local `ensure` unsatisfied variant をいつ追加するか。
- direct target ensure failure を representative prose examples へ昇格させるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、PerformVia 側の request-local ensure failure が later same-lineage success へ継続できる branch を narrow regression fixture として追加し、direct target ensure failure との境界を比較してください。`
