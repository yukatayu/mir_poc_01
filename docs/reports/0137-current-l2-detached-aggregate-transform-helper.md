# Report 0137 — current L2 detached aggregate transform helper

## 1. Title and identifier

- Report 0137
- current L2 detached aggregate transform helper

## 2. Objective

current L2 parser-free PoC の detached aggregate export について、
`BatchRunSummary -> detached aggregate artifact` の transform を
example 内 private code から **repo 内 callable boundary** へ narrow に切り出す。

今回の goal は、

- public exporter API を増やさず
- current aggregate artifact schema judgment を維持し
- bundle / aggregate smoke loop と compare helper を壊さず
- integration test から直接検証できる shared helper を置く

ことである。

## 3. Scope and assumptions

- current L2 の runtime semantics、parser grammar、failure family、machine-check policy は変更しない。
- detached aggregate artifact の current split
  - `aggregate_context`
  - `summary_core`
  - `detached_noncore`
  を維持する。
- `host_plan_coverage_failure` は success-side payload core に上げない。
- `bundle_failure_kind_counts` と current list anchor の additive coexistence は維持する。
- current step は non-production helper cut に留め、`lib.rs` / `harness.rs` の public API は増やさない。
- relevant `plan/` mirror は同 task で更新する。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
10. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
11. `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
12. `plan/07-parser-free-poc-stack.md`
13. `plan/09-helper-stack-and-responsibility-map.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/90-source-traceability.md`
17. `docs/reports/0108-detached-validation-loop-sprint.md`
18. `docs/reports/0109-review-detached-validation-loop-sprint.md`
19. `docs/reports/0113-second-remaining-problem-detached-exporter-api-finalization.md`

## 5. Actions taken

1. current aggregate emitter の private transform と aggregate compare / wrapper / path policy の境界を再確認した。
2. `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs` を追加し、
   `BatchRunSummary -> detached aggregate artifact` の pure transform と carrier struct を shared helper として切り出した。
3. `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs` を shared helper 利用へ切り替え、example 内 duplicate carrier / transform を削除した。
4. `crates/mir-semantics/tests/current_l2_detached_aggregate_support.rs` を追加し、
   - typed count row が出ること
   - current list anchor が残ること
   - coverage failure が 0 のとき typed row が空であること
   を integration test で machine-check 化した。
5. `specs/examples/31-current-l2-detached-aggregate-transform-helper.md` を追加し、current cut を docs-only judgment として明文化した。
6. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/25`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新し、shared helper cut と current understanding を mirror した。

## 6. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
- `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_aggregate_support.rs`

## 7. Commands run and exact outputs

```text
cargo test -p mir-semantics
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/mir_semantics-a4dc6a96e218b890)

running 2 tests
test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok
test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/current_l2_detached_aggregate_support.rs (target/debug/deps/current_l2_detached_aggregate_support-252a016353db5ca0)

running 2 tests
test detached_aggregate_support_keeps_anchor_refs_and_typed_count ... ok
test detached_aggregate_support_omits_typed_rows_when_no_coverage_failure_exists ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/current_l2_minimal_interpreter.rs (target/debug/deps/current_l2_minimal_interpreter-9f0ddd6b093a3dae)

running 36 tests
...
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

   Doc-tests mir_semantics

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 137 numbered report(s).
```

```text
git diff --check
```

無出力。

```text
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --artifact-root target/current-l2-detached --run-label aggregate-helper-cut --overwrite
target/current-l2-detached/aggregates/aggregate-helper-cut/batch-summary.detached.json
```

```text
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --artifact-root target/current-l2-detached --run-label aggregate-helper-e3 --reference-label aggregate-helper-e6 --overwrite
fixture artifact: target/current-l2-detached/bundles/aggregate-helper-e3/e3-option-admit-chain.detached.json
reference artifact: target/current-l2-detached/bundles/aggregate-helper-e6/e6-write-after-expiry.detached.json
=== current L2 detached artifact diff ===
...
note: must_explain と long-form explanation は比較対象に含めない
aggregate artifact (full)  : target/current-l2-detached/aggregates/aggregate-helper-e3-full/batch-summary.detached.json
aggregate artifact (single): target/current-l2-detached/aggregates/aggregate-helper-e3-single/batch-summary.detached.json
=== current L2 detached aggregate diff ===
...
note: current comparison keeps bundle_failure_kind_counts_scope = "migrated-kinds-only" as part of the typed aggregate core
```

## 8. Evidence / outputs / test results

- shared helper cut により、`BatchRunSummary -> detached aggregate artifact` transform を example-private code ではなく repo 内 callable boundary から直接 test できるようになった。
- integration test は typed row あり / typed row なしの両方を確認し、current bool/list anchor と typed count coexistence を machine-check に残した。
- detached loop smoke では bundle compare と aggregate compare の両方が通り、helper cut が wrapper / compare contract を壊していないことを確認した。
- reviewer は code-level boundary leak を指摘せず、report structure と traceability の補正だけを返した。

## 9. What changed in understanding

- aggregate export の actual narrow cut は、example-private transform のままではなく、repo 内 shared helper へ narrow に落とした方が loop 実装と schema judgment の接続点として自然である。
- それでも `src/` / `lib.rs` / `harness.rs` 側へ昇格しないことで、public exporter API の premature freezing を避けられる。
- aggregate emitter / wrapper / compare helper / path policy の責務分離を保ったまま、`BatchRunSummary -> detached aggregate artifact` だけを callable boundary にできる。

## 10. Open questions

- shared helper を actual exporter API の first cut へいつ昇格させるか。
- aggregate compare helper が typed count field の additive coexistence をどこまで exact-compare core とみなすか。
- detached aggregate artifact の final path policy と compare input discovery をどこで固定するか。

## 11. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、shared detached aggregate transform helper を actual narrow exporter API 候補へ寄せる前に、aggregate compare contract と storage/path policy の current candidate を smoke evidence でどこまで固めるべきかを source-backed に比較してください。`
