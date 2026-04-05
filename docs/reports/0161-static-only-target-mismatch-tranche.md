# 0161 — static-only target-mismatch tranche

## Objective

current L2 parser-free PoC の static-only fixture corpus に、
stable malformed cluster の残件である `declared_target_mismatch`
を `e19` として追加し、

- `expected_static.checked_reasons`
- detached static gate artifact の helper-local `reason_codes` mirror
- detached validation loop の smoke
- count / selection / profile summary

までを一貫して揃える。

## Scope and assumptions

- current L2 semantics は変更しない。
- `declared_target_mismatch` は `specs/examples/34` と `35` で stable cluster として列挙済みであり、
  actual corpus に narrow actualization してよいと仮定した。
- `detached_noncore.reason_codes` は今回も helper-local / reference-only mirror に留める。
- `plan/` は更新対象とした。

## Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/00-representative-mir-programs.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
11. `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
12. `plan/01-status-at-a-glance.md`
13. `plan/08-representative-programs-and-fixtures.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/15-current-l2-fixture-authoring-template.md`
16. `plan/90-source-traceability.md`
17. `progress.md`
18. `crates/mir-semantics/src/lib.rs`
19. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
20. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`

## Actions taken

1. 先に tests を更新し、`e19` fixture 未存在で RED になることを確認した。
2. 新規 fixture `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json` を hand-written で追加した。
3. `expected_static.checked_reasons` に actual wording
   `declared access target mismatch between primary and mirror`
   を入れた。
4. representative example / fixture schema / plan mirror / progress を更新し、current docs で stable と列挙している same-lineage static cluster が actual corpus に一巡したことを反映した。
5. detached validation loop で `e19` の static gate smoke と bundle smoke を回した。
6. full `cargo test -p mir-semantics` 中に discovery count の expectation を 1 箇所だけ誤って 10 に上げていたことを見つけ、runtime bundle count は 9 のままだと root cause を確認して修正した。

## Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0161-static-only-target-mismatch-tranche.md`

## Commands run

```bash
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
cargo test -p mir-semantics static_gate_artifact_emits_reason_codes_for_target_and_capability_clusters -- --exact
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json --run-label smoke-static-e19 --reference-label smoke-static-e12 --overwrite
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json --run-label smoke-fixture-e19 --overwrite
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- RED:
  - `static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording`
    は `fixture should load: Io(... No such file or directory)` で失敗した。
  - `static_gate_artifact_emits_reason_codes_for_target_and_capability_clusters`
    も `e19` fixture 不在で失敗した。
- targeted GREEN:
  - `static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording` pass
  - `static_gate_artifact_emits_reason_codes_for_target_and_capability_clusters` pass
- smoke:
  - `smoke-static-gate` では `e19` と `e12` を比較し、`malformed` vs `underdeclared`、actual wording、reference-only `DeclaredTargetMismatch` vs `DeclaredTargetMissing` を確認した。
  - `smoke-fixture` では `e19` の bundle artifact 保存と single-fixture aggregate smoke が通った。
- full verify:
  - `cargo test -p mir-semantics` pass
  - `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 162 numbered report(s).`
  - `git diff --check` → 無出力

## What changed in understanding

- `declared_target_mismatch` も `e12` / `e13` / `e16` / `e17` / `e18` と同じく、
  stable malformed cluster として actual corpus に安全に載せてよいことが確認できた。
- current docs で stable と列挙している same-lineage static cluster は、duplicate declaration cluster を除いて actual corpus / `checked_reasons` / detached `reason_codes` mirror に一巡した。
- 次の判断は、fixture actualization をさらに増やすより、typed carrier の first-class actualization を narrow に始めるかどうかへ寄る。

## Open questions

- static gate の helper-local `reason_codes` mirror を first-class typed carrier に昇格させる前に、どの migration cut を最小とみなすか。
- duplicate declaration cluster を display-only wording のままどこまで維持するか。
- current stable cluster 一巡後の次 task は、typed carrier 側 actualization と fixture authoring 実地反復のどちらを優先するか。

## Suggested next prompt

```text
current L2 parser-free PoC を前提に、static gate helper-local `reason_codes` mirror を first-class typed carrier に昇格させる最小 migration cut を narrow に actualize してください。
current exact-compare core と `checked_reasons` bridge を壊さず、non-production helper / detached artifact / tests / docs / plan / progress を一緒に更新し、review と push まで進めてください。
```
