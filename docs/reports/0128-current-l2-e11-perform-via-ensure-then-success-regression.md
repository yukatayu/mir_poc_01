# Report 0128 — current L2 e11 perform-via ensure then success regression

## 1. Title and identifier

- Report 0128
- current L2 e11 perform-via ensure then success regression

## 2. Objective

current L2 parser-free PoC 基盤で、via-chain の earlier option が request-local `ensure` で失敗しても、
later same-lineage option が残っていれば runtime `success` へ継続できる branch を narrow regression fixture として追加する。

今回の goal は、

- earlier option の success-side carrier preview を commit しない
- earlier `ensure` failure で formal non-admissible metadata を fabricated しない
- later same-lineage option が `require` と `ensure` を満たせば final outcome は `success` になる

ことを machine-check と detached validation loop の両方で確認することである。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- `PerformVia` の via-chain request contract 読みだけを追加 regression で固定する。
- fallback / lease / chain semantics の一般法則は広げず、request-local `ensure` branch に限る。
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
14. `docs/reports/0126-current-l2-e10-perform-on-ensure-failure-regression.md`
15. `docs/reports/0127-review-current-l2-e10-perform-on-ensure-failure-regression.md`
16. `crates/mir-semantics/src/lib.rs`
17. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
18. `scripts/current_l2_detached_loop.py`
19. `scripts/current_l2_diff_detached_artifacts.py`
20. `scripts/current_l2_diff_detached_aggregates.py`

## 5. Actions taken

1. via-chain の request-local `ensure` unsatisfied branch を current code anchor で再確認し、new fixture の scope を
   - `PerformVia`
   - earlier option の `ensure` unsatisfied
   - effect verdict は success-side carrier
   - tentative commit を破棄して later same-lineage option へ継続
   - final outcome は `success`
   に限定した。
2. `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json` と `.host-plan.json` を追加した。
3. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` を更新し、
   - broad runtime loops
   - directory / selection / profile summary counts
   - focused regression test
   を `e11` 前提に更新した。
4. focused regression では `run_bundle` の結果だけでなく、`DirectStyleEvaluator` を host plan で再実行し、
   - final `terminal_outcome = success`
   - final `place_store` に `backup_writer` の commit だけが残る
   ことを確認する assertion を追加した。
5. `specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/06-current-l2-interpreter-skeleton.md`、`plan/08-representative-programs-and-fixtures.md`、`progress.md` を更新し、fixture catalog と current status を mirror した。
6. detached loop smoke として
   - `e3` vs `e11` の bundle compare
   - full fixture directory vs `e11` 単独 directory の aggregate compare
   を実行した。

## 6. Evidence / outputs / test results

### machine-check

```text
cargo test -p mir-semantics
...
running 36 tests
...
test perform_via_ensure_failure_can_continue_to_later_success ... ok
...
test result: ok. 36 passed; 0 failed
```

focused regression では、`terminal_outcome = success`、`perform-failure` → `perform-success`、formal metadata 空だけでなく、
`DirectStyleEvaluator` の final `place_store` が
`profile_doc -> ["write_profile@backup_writer"]`
だけを保持することも確認した。

### bundle detached compare smoke

```text
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json --left-label e3-via-smoke --right-label e11-via-ensure-smoke --overwrite
...
payload_core differences:
- payload_core.event_kinds: left=["perform-success"] right=["perform-failure", "perform-success"]
- payload_core.non_admissible_metadata: left=[{"option_ref": "owner_writer", "subreason": "admit-miss"}] right=[]
```

### aggregate detached compare smoke

```text
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label agg-e11-full --overwrite
python3 scripts/current_l2_detached_loop.py emit-aggregate <tmp-e11-only-dir> --run-label agg-e11-only --overwrite
python3 scripts/current_l2_detached_loop.py compare-aggregates agg-e11-full agg-e11-only
...
summary_core differences:
- summary_core.total_bundles: left=11 right=1
- summary_core.runtime_bundles: left=9 right=1
- summary_core.static_only_bundles: left=2 right=0
- summary_core.passed: left=11 right=1
```

## 7. What changed in understanding

- via-chain の earlier option が request-local `ensure` で失敗しても、current L2 では直ちに `Reject` へ行かず、later same-lineage candidate が残っていれば evaluation を継続できる。
- earlier `ensure` failure は request contract failure として `perform-failure` を記録するが、formal non-admissible metadata にはしない。
- success-side carrier preview suppression は direct target だけでなく via-chain continuation でも machine-check しておく方が安全である。

## 8. Open questions

- via-chain request-local `ensure` failure を representative prose examples へ昇格するか。
- aggregate export の actual narrow API cut をこの regression 追加後にどう operational に寄せるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached aggregate export の actual narrow API cut を 1 段だけ operational に寄せるか、あるいは fixture authoring / elaboration の next narrow helper を追加するかを source-backed に比較してください。`
