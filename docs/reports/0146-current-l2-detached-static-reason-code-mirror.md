# Report 0146 — current L2 detached static reason code mirror

- Date: 2026-04-05T12:20:00Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の detached static gate artifact に helper-local / reference-only `reason_codes` mirror を actualize し、その boundary を spec / plan / progress に揃える
- Decision levels touched: L2

## 1. Objective

current L2 では `checked_reasons` を fixture-side bridge として導入済みであり、
その次段として typed reason code の entry criteria も docs-only で整理できた。

一方で code 側には、detached static gate artifact の `detached_noncore.reason_codes`
mirror が先に actualize されていた。

本 task の目的は、

- この mirror を helper-local / reference-only に正確に位置づけ
- `checker_core` や `checked_reasons` と混同しない
- docs / plan / progress / traceability を current code anchor に揃える

ことである。

## 2. Scope and assumptions

- current L2 runtime semantics は変えない
- `checker_core.static_verdict` / `checker_core.reasons` は exact-compare core のまま維持する
- `expected_static.checked_reasons` は fixture-side bridge のまま維持する
- `detached_noncore.reason_codes` は first-class typed source に昇格させない
- `reason_codes` difference は compare helper 上でも reference-only に留める

## 3. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
9. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
10. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
11. `plan/07-parser-free-poc-stack.md`
12. `plan/09-helper-stack-and-responsibility-map.md`
13. `plan/11-roadmap-near-term.md`
14. `plan/12-open-problems-and-risks.md`
15. `plan/15-current-l2-fixture-authoring-template.md`
16. `plan/90-source-traceability.md`
17. `progress.md`
18. `docs/reports/0144-current-l2-static-reason-code-entry-criteria.md`
19. `docs/reports/0145-review-current-l2-static-reason-code-entry-criteria.md`
20. `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
21. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
22. `scripts/current_l2_diff_static_gate_artifacts.py`
23. `scripts/tests/test_current_l2_diff_static_gate_artifacts.py`

## 4. Actions taken

1. detached static gate artifact support codeを読み、`detached_noncore.reason_codes` が
   stable cluster だけを `filter_map` で best-effort 変換していることを確認した。
2. `specs/examples/32-current-l2-static-gate-artifact-loop.md` を更新し、
   artifact shape に optional `detached_noncore` を追加した。
3. `specs/examples/33-current-l2-checked-static-reasons-carrier.md` を更新し、
   detached-side `reason_codes` mirror は `checked_reasons` を置き換える typed carrier ではないと明記した。
4. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md` を更新し、
   current detached mirror と future first-class typed source を区別した。
5. 新規 `specs/examples/35-current-l2-detached-static-reason-code-mirror.md` を追加し、
   current code anchor、artifact shape、reference-only compare contract、OPEN を 1 箇所へ整理した。
6. `plan/07` / `plan/09` / `plan/11` / `plan/12` / `plan/15` / `plan/90`、`Documentation.md`、
   `specs/00-document-map.md`、`progress.md` を current understanding に合わせて更新した。
7. support code に短い comment を入れ、stable cluster だけを mirror する helper-local cut だと明示した。
8. focused regression として、
   - stable cluster が `detached_noncore.reason_codes` に出ること
   - unclassified reason text では `detached_noncore` が省略されること
   - diff helper が `detached_noncore` difference を reference-only に留めること
   を test で固定した。

## 5. Files changed

- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_diff_static_gate_artifacts.py`
- `scripts/tests/test_current_l2_diff_static_gate_artifacts.py`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0145-review-current-l2-static-reason-code-entry-criteria.md`
- `docs/reports/0146-current-l2-detached-static-reason-code-mirror.md`

## 6. Evidence / outputs / test results

```text
cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed
test result: ok. 2 passed; 0 failed
test result: ok. 36 passed; 0 failed
test result: ok. 4 passed; 0 failed
test result: ok. 0 passed; 0 failed
```

```text
python3 -m unittest scripts.tests.test_current_l2_diff_static_gate_artifacts
.
----------------------------------------------------------------------
Ran 3 tests in ...

OK
```

```text
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json --artifact-root target/current-l2-detached --run-label reason-code-mirror-e4 --reference-label reason-code-mirror-e5 --overwrite
static gate artifact: target/current-l2-detached/static-gates/reason-code-mirror-e4/e4-malformed-lineage.static-gate.json
reference static gate artifact: target/current-l2-detached/static-gates/reason-code-mirror-e5/e5-underdeclared-lineage.static-gate.json
=== current L2 static gate artifact diff ===
checker_core differences:
- checker_core.static_verdict: left="malformed" right="underdeclared"
- checker_core.reasons: left=[...] right=[...]

reference-only differences:
- detached_noncore: left={...} right={...}
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 146 numbered report(s).
```

```text
git diff --check
[no output]
```

## 7. What changed in understanding

- detached-side `reason_codes` mirror を持つこと自体は、typed checker carrier を確定したことにはならない。
- current helper cut で実際に言えるのは、
  - `checker_core` は exact-compare core
  - `checked_reasons` は fixture-side bridge
  - `detached_noncore.reason_codes` は helper-local / reference-only mirror
  という 3-way split である。
- stable cluster inventory と detached-side mirror actualization は別 anchor に分ける方が drift を抑えやすい。

## 8. Open questions

- `detached_noncore.reason_codes` を first-class typed source に昇格させるか。
- fixture-side `checked_reasons` の次段として typed row を導入するか。
- duplicate reason cluster を別 namespace / separate carrier に切る必要があるか。
- detached mirror を theorem prover / checker の final serialization contract とどこで接続するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached static gate artifact の helper-local reason_codes mirror を fixture-side typed carrier と接続する価値が本当にあるかを source-backed に比較し、checked_reasons を広げるか、reason_codes を昇格させるか、helper-local のまま維持するかを narrow に判断してください。`
