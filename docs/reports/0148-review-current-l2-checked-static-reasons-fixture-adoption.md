# Report 0148 — review current L2 checked static reasons fixture adoption

- Date: 2026-04-05T10:41:00Z
- Author / agent: Codex
- Scope: report 0147 の fixture-side `checked_reasons` adoption が current helper boundary と actual corpus に対して過不足ないかを確認する
- Decision levels touched: L2

## 1. Objective

`e4` / `e5` への `checked_reasons` actual adoption が、

- explanatory `reasons` を壊していないか
- detached-side `reason_codes` mirror と役割衝突していないか
- plan / progress / traceability を stale にしていないか

を確認する。

## 2. Scope and assumptions

- runtime semantics は不変
- `checked_reasons` は fixture-side bridge のまま扱う
- detached-side `reason_codes` は helper-local / reference-only のまま扱う

## 3. Actions taken

1. local diff inspection で fixture / test / spec / plan / progress の変更面を読み直した。
2. fresh verification として full `cargo test -p mir-semantics`、`python3 scripts/validate_docs.py`、`git diff --check` を取り直した。
3. `e4` / `e5` の explanatory `reasons` が維持され、actual wording は `checked_reasons` にだけ載っていることを確認した。
4. reviewer subagent に相当する completion handle は current tool surface で取得できなかったため、本 report では local evidence fallback を明記する。

## 4. Files changed

- `docs/reports/0148-review-current-l2-checked-static-reasons-fixture-adoption.md`

## 5. Commands run and exact outputs

```text
cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed
test result: ok. 2 passed; 0 failed
test result: ok. 40 passed; 0 failed
test result: ok. 5 passed; 0 failed
test result: ok. 0 passed; 0 failed
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 148 numbered report(s).
```

```text
git diff --check
[no output]
```

## 6. Evidence / findings

- `e4` / `e5` の `expected_static.reasons` は explanatory prose のまま残っている。
- actual wording は `checked_reasons` にだけ入り、`run_bundle()` の fail-closed compare path と素直につながる。
- `e3` のような valid fixture には `checked_reasons` を足しておらず、explanatory prose と machine-check bridge の境界は保たれている。
- plan 更新は `plan/08` / `plan/11` / `plan/15` / `plan/90` に留まり、helper-local detached mirror actualization とも矛盾しない。

## 7. What changed in understanding

- `checked_reasons` は current corpus に入れ始めても、狭い static-only fixture から始めれば explanation split を崩さない。
- detached-side mirror を先に昇格させなくても、fixture-side bridge adoption だけで validation loop の実務価値は上がる。

## 8. Open questions

- `checked_reasons` の採用を他 fixture に広げる閾値をどこに置くか。
- detached-side `reason_codes` mirror と fixture-side typed carrier のどちらを先に強化するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、checked_reasons の current corpus adoption を valid fixture へ広げる価値があるかを source-backed に比較し、explanatory prose と conflict しない narrow candidate だけを actual adoption してください。`
