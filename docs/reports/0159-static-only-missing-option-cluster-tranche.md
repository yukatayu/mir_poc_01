# 0159 — static-only missing-option cluster tranche

## Objective

current L2 parser-free PoC の static-only fixture corpus に、
stable malformed cluster として扱ってよい missing-option 系 3 本
`e16` / `e17` / `e18`
を追加し、

- `expected_static.checked_reasons`
- detached static gate artifact の helper-local `reason_codes` mirror
- detached validation loop の smoke evidence

までを一貫して揃える。

## Scope and assumptions

- current L2 semantics は変更しない。
- duplicate declaration cluster と違い、missing chain head / predecessor / successor option cluster は
  stable malformed wording として narrow actualization してよい、という既存 docs-only judgmentを前提にした。
- `detached_noncore.reason_codes` は今回も helper-local / reference-only mirror に留め、
  exact-compare core へは上げない。
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

1. 先行して修正済みだった test expectation に対し、targeted RED を実行して missing fixture file による failure を確認した。
2. 新規 fixture として次を追加した。
   - `crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json`
   - `crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json`
   - `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
3. 3 fixture すべてで `expected_static.checked_reasons` を actual wording に合わせて埋めた。
4. 代表例 / schema / plan mirror を更新し、missing-option cluster が
   stable malformed cluster であることを人間向け文書にも反映した。
5. `progress.md` を更新し、rough progress と task log に今回の tranche を反映する準備を行った。
6. detached validation loop の smoke と full cargo test を回し、bundle / static gate の両方で挙動を確認した。

## Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `progress.md`
- `docs/reports/0159-static-only-missing-option-cluster-tranche.md`

## Commands run

```bash
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
cargo test -p mir-semantics static_gate_artifact_emits_reason_codes_for_missing_option_clusters -- --exact
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json --run-label smoke-static-e16 --reference-label smoke-static-e17 --overwrite
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json --run-label smoke-fixture-e16 --overwrite
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- RED:
  - `static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording`
    は `fixture should load: Io(... No such file or directory)` で失敗し、fixture 未追加状態を確認できた。
- targeted GREEN:
  - `static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording` pass
  - `static_gate_artifact_emits_reason_codes_for_missing_option_clusters` pass
- smoke:
  - `smoke-static-gate` では `e16` と `e17` の `checker_core.reasons` 差分と、reference-only `reason_codes` 差分を確認した。
  - `smoke-fixture` では `e16` の bundle artifact 保存と single-fixture aggregate smoke が通った。
- full verify:
  - `cargo test -p mir-semantics` pass
  - `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 160 numbered report(s).`
  - `git diff --check` → 無出力

## What changed in understanding

- missing chain head / predecessor / successor option cluster は、docs-only inventory だけでなく
  actual corpus fixture と detached validation loop の両方へ安全に actualize してよいことが確認できた。
- duplicate declaration cluster と違い、今回の 3 cluster は
  `checked_reasons` と detached `reason_codes` mirror を同時に持たせても helper boundary を壊さない。
- static-only stable cluster の actual corpus 化は、
  typed reason code の finalization ではなく fixture authoring / detached validation loop の operational gain として進められる。

## Open questions

- `declared_target_mismatch` cluster を同じ方針で actual corpus に入れるか。
- helper-local `reason_codes` mirror を first-class typed carrier に昇格させる前に、stable cluster 実地反復を何本まで増やすべきか。
- duplicate declaration cluster を今後も display-only wording に留める cut をどこで見直すか。

## Suggested next prompt

```text
current L2 parser-free PoC を前提に、static-only stable cluster の actual corpus expansion を続けてください。
次は declared-target-mismatch など、specs/examples/34 と 35 で stable cluster 候補に入っているが fixture 化されていないものが残っているかを inventory し、
追加してよいものは RED→fixture→detached smoke→docs/plan/progress→review→push まで進めてください。
```
