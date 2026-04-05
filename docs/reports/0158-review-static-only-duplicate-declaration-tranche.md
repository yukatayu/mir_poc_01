# 0158 — review static-only duplicate declaration tranche

## Objective

`e14` / `e15` を追加した static-only duplicate declaration tranche について、

- semantic boundary が壊れていないか
- duplicate cluster を `checked_reasons` と detached `reason_codes` に premature に昇格させていないか
- docs / plan / progress / traceability が一致しているか

を最終 review する。

## Scope and assumptions

- review task のため、normative statement は変更しない。
- review 観点は current L2 static gate boundary と fixture authoring boundary に限定する。
- `plan/` 更新不要。
- `progress.md` 更新不要。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `progress.md`
5. `plan/00-index.md`
6. `specs/00-document-map.md`
7. `specs/01-charter-and-decision-levels.md`
8. `specs/02-system-overview.md`
9. `specs/03-layer-model.md`
10. `specs/09-invariants-and-constraints.md`
11. `specs/examples/00-representative-mir-programs.md`
12. `specs/examples/02-current-l2-ast-fixture-schema.md`
13. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
14. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
15. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
16. `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
17. `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
18. `plan/01-status-at-a-glance.md`
19. `plan/08-representative-programs-and-fixtures.md`
20. `plan/11-roadmap-near-term.md`
21. `plan/15-current-l2-fixture-authoring-template.md`
22. `plan/90-source-traceability.md`
23. `crates/mir-semantics/src/lib.rs`
24. `crates/mir-semantics/src/harness.rs`
25. `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
26. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
27. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
28. `crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json`
29. `crates/mir-ast/tests/fixtures/current-l2/e15-malformed-duplicate-chain-declaration.json`
30. `docs/reports/0157-static-only-duplicate-declaration-tranche.md`

## Actions taken

1. baseline docs と current L2 static gate boundary docs を読み直した。
2. duplicate declaration cluster の actual source を `DeclarationIndex::collect_statements()` と `static_gate_detailed()` で再確認した。
3. changed files を diff inspection した。
4. targeted test と full `cargo test -p mir-semantics` を再実行した。
5. detached static-gate smoke を `e14` / `e15` で、bundle smoke を `e14` で再実行した。
6. docs / plan / progress / report の mirror trail を照合した。

## Evidence / outputs / test results

実行コマンド:

```bash
git diff -- <requested files>
rg -n "duplicate option declaration|duplicate chain declaration|reason_code_from_reason|checked_reasons" crates/mir-semantics/src crates/mir-semantics/examples crates/mir-semantics/tests crates/mir-ast -g '!target'
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
cargo test -p mir-semantics static_gate_artifact_omits_reason_codes_for_duplicate_declaration_clusters -- --exact
cargo test -p mir-semantics discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only -- --exact
cargo test -p mir-semantics
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label review-e14-duplicate-static --overwrite
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e15-malformed-duplicate-chain-declaration.json --run-label review-e15-duplicate-static --overwrite
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label review-e14-duplicate-bundle --overwrite
git diff --check
```

要点:

- `cargo test -p mir-semantics` は unit 2 / integration 40 / static-gate-support 7 / doc-tests 0 が pass
- `smoke-static-gate e14` は static gate artifact を `target/current-l2-detached/static-gates/review-e14-duplicate-static/` に保存
- `smoke-static-gate e15` は static gate artifact を `target/current-l2-detached/static-gates/review-e15-duplicate-static/` に保存
- `smoke-fixture e14` は bundle / aggregate artifact を保存し、aggregate compare は coarse summary 差分だけを報告
- `git diff --check` は無出力

## What changed in understanding

- duplicate declaration cluster は current corpus に actualize してよいが、current helper cut では `checked_reasons` と detached `reason_codes` に上げない方が自然である。
- `e14` / `e15` は static-only malformed corpus expansion として整合しており、reason taxonomy expansion にはなっていない。
- docs / plan / progress / traceability の trail は、この boundary を崩さずに揃っている。

## Open questions

- next tranche を nested-scope / `try` / `fallback` branch duplicate variant に進めるか。
- それとも missing chain head / predecessor / successor cluster を先に actualize するか。
- duplicate declaration wording を長期的にも display-only に留めるか、将来 parameter slot が固まった時点で typed carrier に進めるか。

## Suggested next prompt

```text
current L2 parser-free PoC 基盤を前提に、static-only fixture authoring の次 tranche として、
duplicate declaration cluster の nested-scope / try-fallback branch variant を actual corpus に入れる価値があるか、
それとも missing chain head / missing predecessor / missing successor cluster を先に actualize すべきかを narrow scope で比較してください。
current helper cut として、duplicate cluster は引き続き checked_reasons / detached reason_codes に昇格させない前提で整理してください。
```
