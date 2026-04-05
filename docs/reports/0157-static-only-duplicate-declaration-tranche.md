# 0157 — static-only duplicate declaration tranche

## Objective

duplicate declaration cluster を current L2 static-only corpus に actualize し、

- `e14-malformed-duplicate-option-declaration`
- `e15-malformed-duplicate-chain-declaration`

を parser-free PoC / detached validation loop / fixture catalog に接続する。

同時に、current helper cut を維持し、

- duplicate cluster は `checked_reasons` に上げない
- duplicate cluster は detached static gate artifact の `reason_codes` にも上げない
- actual wording は `checker_core.reasons` と focused smoke / test で見る

という境界が崩れないことを確認する。

## Scope and assumptions

- current L2 core semantics は変更しない。
- runtime semantics、failure family、parser grammar、richer host interface には広げない。
- duplicate cluster は current docs の判断どおり static-only malformed cluster として actual corpus に入れる。
- duplicate cluster は declaration index visibility / grouping に寄るため、current helper cut では stable typed cluster として扱わない。

## Documents consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
11. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
12. `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
13. `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
14. `plan/01-status-at-a-glance.md`
15. `plan/08-representative-programs-and-fixtures.md`
16. `plan/11-roadmap-near-term.md`
17. `plan/15-current-l2-fixture-authoring-template.md`
18. `plan/90-source-traceability.md`
19. `crates/mir-semantics/src/lib.rs`
20. `crates/mir-semantics/src/harness.rs`
21. `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
22. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
23. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
24. `crates/mir-ast/tests/fixtures/current-l2/`

## Actions taken

1. duplicate declaration reason の actual source を `DeclarationIndex::collect_statements()` と `static_gate_detailed()` で再確認した。
2. TDD の RED として、fixture discovery count・static-only corpus count・selection list・static gate detached mirror omission を先に test 側へ追加した。
3. `e14-malformed-duplicate-option-declaration.json` と `e15-malformed-duplicate-chain-declaration.json` を hand-written fixture として追加した。
4. `current_l2_static_gate_support.rs` の focused test を追加し、duplicate cluster では `detached_noncore` が absent のままであることを固定した。
5. `specs/examples/00` と `specs/examples/02` に E14 / E15 と fixture catalog 行を追加した。
6. `plan/01`、`plan/08`、`plan/11`、`plan/15`、`progress.md` を更新し、duplicate cluster の actualization と non-promotion cut を mirror した。
7. `plan/90-source-traceability.md` を更新し、この tranche の report chain を plan root へ接続した。
8. detached loop smoke と full cargo test を回し、static-only / bundle-first loop の両側で duplicate cluster が current cut のまま流れることを確認した。

## Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json`
- `crates/mir-ast/tests/fixtures/current-l2/e15-malformed-duplicate-chain-declaration.json`
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

## Commands run

```bash
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
cargo test -p mir-semantics discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only -- --exact
cargo test -p mir-semantics static_gate_artifact_omits_reason_codes_for_duplicate_declaration_clusters -- --exact
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label e14-duplicate-static --overwrite
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e15-malformed-duplicate-chain-declaration.json --run-label e15-duplicate-static --overwrite
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label e14-duplicate-bundle --overwrite
cargo test -p mir-semantics
```

## Evidence / outputs / test results

- RED:
  - `cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact`
  - fixture 不在により `No such file or directory` で fail した。
- GREEN:
  - `cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact` が pass
  - `cargo test -p mir-semantics discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only -- --exact` が pass
  - `cargo test -p mir-semantics static_gate_artifact_omits_reason_codes_for_duplicate_declaration_clusters -- --exact` が pass
  - `cargo test -p mir-semantics` は unit 2 / integration 40 / static-gate-support 7 / doc-tests 0 がすべて pass
- smoke:
  - `smoke-static-gate e14` は static gate artifact を `target/current-l2-detached/static-gates/e14-duplicate-static/...` に保存した
  - `smoke-static-gate e15` は static gate artifact を `target/current-l2-detached/static-gates/e15-duplicate-static/...` に保存した
  - `smoke-fixture e14` は bundle artifact と aggregate artifact を保存し、aggregate diff で `summary_core.static_only_bundles: left=6 right=1` を含む coarse difference を報告した

## What changed in understanding

- duplicate declaration cluster は current static-only corpus に actualize してよい。
- ただし duplicate cluster は current helper cut では stable `checked_reasons` cluster ではない。
- 同様に、duplicate cluster は detached static gate artifact の `reason_codes` にも current では上げない方が自然である。
- したがって current operational boundary は、
  - static verdict は bundle machine-check core
  - duplicate actual wording は `checker_core.reasons`
  - duplicate cluster の typed / mirrored code 化は後段
 という 3 層のままでよい。

## Open questions

- duplicate declaration cluster を将来 typed reason code に昇格させるなら、scope / visibility grouping をどこまで parameter slot に出すか。
- next fixture-authoring tranche を duplicate nested-scope / `try-fallback` branch duplicate へ進めるか、それとも missing chain head / missing predecessor / missing successor cluster へ進めるか。
- duplicate cluster の actual wording を `checked_reasons` に narrow transfer する価値があるか、それとも current の display-only actual wording cut を維持する方が自然か。

## Suggested next prompt

```text
current L2 parser-free PoC 基盤を前提に、static-only fixture authoring の次 tranche として、
duplicate declaration cluster の nested-scope / try-fallback branch variant を actual corpus に入れる価値があるか、
それとも missing chain head / missing predecessor / missing successor cluster を先に actualize すべきかを narrow scope で比較してください。
current helper cut として、duplicate cluster は引き続き checked_reasons / detached reason_codes に昇格させない前提で整理してください。
```
