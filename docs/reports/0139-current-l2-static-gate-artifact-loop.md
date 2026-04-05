# Report 0139 — current L2 static gate artifact loop

## 1. Title and identifier

- Report 0139
- current L2 static gate artifact loop

## 2. Objective

current L2 parser-free PoC の first checker cut entry criteria を前提に、
static gate verdict / reasons を detached validation loop に接続する
**non-production static gate artifact helper**
を追加する。

今回の goal は、

- runtime bundle artifact と統合しない
- static gate の `static_verdict` / `reasons` を detached compare に載せる
- malformed / underdeclared / static-only fixture を 1 command で smoke できる

状態まで narrow に進めることである。

## 3. Scope and assumptions

- current L2 の runtime semantics、parser grammar、failure family、machine-check policy は変更しない。
- final checker API、final type system、theorem prover relation は固定しない。
- current step は non-production helper cut に留め、`lib.rs` / `harness.rs` の public API は増やさない。
- static gate artifact は `fixture_context` / `checker_core` の最小 shape に留める。
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
9. `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
10. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
11. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
12. `plan/07-parser-free-poc-stack.md`
13. `plan/09-helper-stack-and-responsibility-map.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/15-current-l2-fixture-authoring-template.md`
17. `plan/90-source-traceability.md`
18. `docs/reports/0140-review-current-l2-static-gate-artifact-loop.md`

## 5. Actions taken

1. static gate と first checker cut の current docs-only boundary を読み直し、runtime artifact と分けた static gate artifact helper が自然かを再確認した。
2. `crates/mir-semantics/tests/current_l2_static_gate_support.rs` を先に追加し、missing support module で RED を確認した。
3. `scripts/tests/test_current_l2_static_gate_loop.py` を先に追加し、wrapper 側に `static_gate_artifact_path` / `emit_static_gate` / `smoke-static-gate` が無い RED を確認した。
4. `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs` を追加し、`CurrentL2Fixture + StaticGateResult -> static gate artifact` の pure transform を shared support helper として実装した。
5. `crates/mir-semantics/examples/current_l2_emit_static_gate.rs` を追加し、fixture 1 本から static gate artifact を JSON へ出す emitter sketch を実装した。
6. `scripts/current_l2_diff_static_gate_artifacts.py` を追加し、`checker_core` だけを exact-compare する minimal diff helper を実装した。
7. `scripts/current_l2_detached_loop.py` に
   - `emit-static-gate`
   - `compare-static-gates`
   - `smoke-static-gate`
   を追加し、static gate artifact loop を detached wrapper に接続した。
8. `scripts/tests/test_current_l2_diff_static_gate_artifacts.py` を追加し、checker_core compare helper の focused test を追加した。
9. `specs/examples/32-current-l2-static-gate-artifact-loop.md` を追加し、artifact shape・compare contract・current path candidate を docs-only judgment として明文化した。
10. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`progress.md` を更新し、current understanding を mirror した。
11. reviewer report `0140` を読み、`checker_core.reasons` の nondeterministic order と document-map implementation anchor omission を確認した。
12. `crates/mir-semantics/tests/current_l2_static_gate_support.rs` に multi-reason fixture の deterministic order regression を追加して RED を確認し、`static_gate_detailed()` 側で `reasons.sort()` を行う fix を入れた。
13. `specs/00-document-map.md` に static gate emitter / support / diff helper の implementation anchor を追記し、`specs/examples/32-current-l2-static-gate-artifact-loop.md` に `reasons` lexical sort を明記した。
14. `plan/90-source-traceability.md` を更新し、spec 32 と reports 0139 / 0140、および static gate helper code anchor を traceability に追加した。

## 6. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `progress.md`
- `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_diff_static_gate_artifacts.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `scripts/tests/test_current_l2_diff_static_gate_artifacts.py`

## 7. Commands run and exact outputs

```text
cargo test -p mir-semantics current_l2_static_gate_support -- --nocapture
error: couldn't read `crates/mir-semantics/tests/../examples/support/current_l2_static_gate_support.rs`: No such file or directory (os error 2)
```

```text
python3 -m unittest scripts.tests.test_current_l2_static_gate_loop
AttributeError: module 'current_l2_detached_loop' has no attribute 'emit_static_gate'
AttributeError: module 'current_l2_detached_loop' has no attribute 'static_gate_artifact_path'
FAILED (errors=2)
```

```text
cargo test -p mir-semantics --test current_l2_static_gate_support -- --nocapture
running 2 tests
test static_gate_support_preserves_fixture_context_and_malformed_reasons ... ok
test static_gate_support_keeps_valid_fixture_reason_list_empty ... ok
test result: ok. 2 passed; 0 failed
```

```text
cargo test -p mir-semantics --test current_l2_static_gate_support static_gate_reasons_are_deterministic_for_multi_reason_fixture -- --nocapture
thread 'static_gate_reasons_are_deterministic_for_multi_reason_fixture' panicked
left: ["missing lineage assertion for primary -> mirror", "lineage assertion does not describe primary -> archive"]
right: ["lineage assertion does not describe primary -> archive", "missing lineage assertion for primary -> mirror"]
FAILED
```

```text
cargo test -p mir-semantics --test current_l2_static_gate_support -- --nocapture
running 3 tests
test static_gate_reasons_are_deterministic_for_multi_reason_fixture ... ok
test static_gate_support_keeps_valid_fixture_reason_list_empty ... ok
test static_gate_support_preserves_fixture_context_and_malformed_reasons ... ok
test result: ok. 3 passed; 0 failed
```

```text
python3 -m unittest scripts.tests.test_current_l2_detached_loop scripts.tests.test_current_l2_diff_detached_aggregates scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_diff_static_gate_artifacts scripts.tests.test_current_l2_scaffold_fixture
Ran 17 tests in 0.012s
OK
```

```text
cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json --artifact-root target/current-l2-detached --run-label static-gate-e4 --reference-label static-gate-e5 --overwrite
static gate artifact: target/current-l2-detached/static-gates/static-gate-e4/e4-malformed-lineage.static-gate.json
reference static gate artifact: target/current-l2-detached/static-gates/static-gate-e5/e5-underdeclared-lineage.static-gate.json
=== current L2 static gate artifact diff ===
...
note: static gate artifact は runtime trace / must_explain を含めない
```

```text
python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --artifact-root target/current-l2-detached --run-label post-static-e3 --reference-label post-static-e6 --overwrite
fixture artifact: target/current-l2-detached/bundles/post-static-e3/e3-option-admit-chain.detached.json
reference artifact: target/current-l2-detached/bundles/post-static-e6/e6-write-after-expiry.detached.json
=== current L2 detached artifact diff ===
...
=== current L2 detached aggregate diff ===
...
```

## 8. Evidence / outputs / test results

- static gate helper は malformed / underdeclared / valid fixture を `checker_core` だけの detached artifact に落とせる。
- `smoke-static-gate` により、runtime bundle artifact を経由せずに static-only fixture compare を detached loop へ接続できる。
- existing `smoke-fixture` / aggregate compare path は regression せず、wrapper 拡張後も通った。
- current path candidate は `target/current-l2-detached/static-gates/<run-label>/<fixture-stem>.static-gate.json` で smoke できた。
- reviewer 指摘のとおり multi-reason fixture では `checker_core.reasons` が不安定だったが、`static_gate_detailed()` 側の lexical sort と regression test で deterministic order に修正した。
- `specs/00-document-map.md` の implementation anchor も static gate helper 群まで含めて補完し、repo bootstrap traceability を戻した。

## 9. What changed in understanding

- first checker cut の local / structural floor を detached validation loop に接続するときは、runtime bundle artifact と無理に統合するより static gate artifact を別立てにする方が自然である。
- `checker_core.static_verdict` / `checker_core.reasons` を exact-compare core に留めることで、future checker API を凍らせずに fixture authoring の static compare を一段軽くできる。
- static gate artifact loop を runtime loop の wrapper に薄く接続しても、helper boundary を public API 側へ漏らさずに済む。
- static gate artifact で string list を exact-compare core に含めるなら、reason order は emitter helper ではなく source relation 側で deterministic にしておく方が、future checker API や compare helper が分岐せずに済む。

## 10. Open questions

- static gate `reasons` を将来 typed reason code へ進めるべきか。
- static gate artifact を future checker API の first cut に昇格させるか、script/example helper のまましばらく留めるか。
- parser cut と checker cut の actual API 接続をどこで固定するか。

## 11. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、first checker cut の local / structural floor を actual machine-check にもう一段寄せるなら、expected_static.reasons comparison を bundle/harness 側へどこまで入れてよいかを source-backed に比較してください。`
