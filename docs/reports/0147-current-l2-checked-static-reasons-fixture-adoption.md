# Report 0147 — current L2 checked static reasons fixture adoption

- Date: 2026-04-05T10:40:00Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC で `expected_static.checked_reasons` を current fixture corpus に narrow adoption し、static-only fixture の explanatory prose と actual static gate wording の split を actual corpus で回す
- Decision levels touched: L2

## 1. Objective

current L2 では `checked_reasons` が additive optional bridge として入っているが、
current fixture corpus にはまだ actual adoption がなかった。

本 task の目的は、

- `checked_reasons` をどの fixture に入れてよいかを narrow に判断し
- explanatory `reasons` を壊さず
- actual static gate wording と bundle machine-check bridge を actual corpus で接続する

ことである。

## 2. Scope and assumptions

- runtime semantics は変更しない
- `expected_static.reasons` は explanation に残す
- `checked_reasons` は actual static gate wording の fail-closed bridge に限る
- detached-side `reason_codes` mirror は helper-local / reference-only のまま維持する
- current task では stable actual wording を持つ static-only malformed / underdeclared fixture だけを対象にする

## 3. Inputs consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/02-current-l2-ast-fixture-schema.md`
9. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
10. `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
11. `plan/08-representative-programs-and-fixtures.md`
12. `plan/11-roadmap-near-term.md`
13. `plan/15-current-l2-fixture-authoring-template.md`
14. `plan/90-source-traceability.md`
15. `progress.md`
16. `crates/mir-semantics/src/harness.rs`
17. `crates/mir-semantics/src/lib.rs`
18. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
19. `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
20. `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
21. `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`

## 4. Actions taken

1. current actual compare path を再確認し、`run_bundle()` は `checked_reasons` が present のときだけ `static_gate_detailed().reasons` を fail-closed compare することを確認した。
2. current fixture corpus を見直し、`e4-malformed-lineage` と `e5-underdeclared-lineage` が
   - static-only fixture であり
   - explanatory `reasons` を残したい一方
   - actual static gate wording が 1 本に安定している
   ため、最初の adoption candidate と判断した。
3. TDD として `static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording` を追加し、fixture corpus がまだ `checked_reasons` を持たない状態で RED を確認した。
4. `e4` / `e5` fixture に `checked_reasons` を追加し、explanatory `reasons` はそのまま維持した。
5. `specs/examples/33-current-l2-checked-static-reasons-carrier.md` に initial adoption candidate を追記した。
6. `plan/08` / `plan/11` / `plan/15` / `plan/90` / `progress.md` を current understanding に合わせて更新した。

## 5. Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0147-current-l2-checked-static-reasons-fixture-adoption.md`

## 6. Commands run and exact outputs

```text
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
...
test static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording ... FAILED
...
left: None
right: Some(["lineage assertion does not describe primary -> mirror"])
```

```text
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
...
test static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording ... ok
```

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
Found 147 numbered report(s).
```

```text
git diff --check
[no output]
```

## 7. Evidence / findings

- `e4-malformed-lineage`
  - fixture-side explanation:
    - `lineage assertion does not describe the decorated edge`
  - actual static gate wording:
    - `lineage assertion does not describe primary -> mirror`
  - したがって explanation と machine-check bridge を分ける価値が高い。
- `e5-underdeclared-lineage`
  - fixture-side explanation:
    - `declared access target alone is not enough`
    - `edge-local lineage evidence is missing`
  - actual static gate wording:
    - `missing lineage assertion for primary -> mirror`
  - ここでも explanation と machine-check bridge の split が自然に成立する。
- `e3-option-admit-chain`
  - `expected_static.reasons` は explanatory prose であり、current では `checked_reasons` を持たないままが自然。
- このため、current corpus の最初の adoption candidate は `e4` / `e5` に限るのが最小である。

## 8. Changes in understanding

- `checked_reasons` は docs-only bridge ではなく、actual corpus に narrow adoption してはじめて fixture authoring guide と detached loop の運用面に効く。
- ただし adoption 範囲は広げすぎない方がよく、現時点では stable actual wording を持つ static-only malformed / underdeclared fixture に限るのが自然である。
- detached-side `reason_codes` mirror を先に昇格させるより、まず `checked_reasons` の current corpus adoption を少数 fixture で回す方が drift を抑えやすい。

## 9. Open questions

- `checked_reasons` の adoption を `e4` / `e5` から他の fixture へどこまで広げるか。
- detached-side `reason_codes` mirror を first-class typed source に昇格させる価値が本当にあるか。
- duplicate / multi-row static reason cluster を fixture-side carrier へ入れる必要が出るか。

## 10. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、checked_reasons の current corpus adoption を e4/e5 からさらに広げる価値があるかを source-backed に比較し、valid fixture の explanatory prose と conflict しない範囲だけ narrow に actual adoption してください。`
