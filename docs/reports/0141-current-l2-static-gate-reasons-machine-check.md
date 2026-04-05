# Report 0141 — current L2 static gate reasons machine-check boundary

- Date: 2026-04-05T09:29:59.281718Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の `expected_static.reasons` を `run_bundle()` の actual machine-check に昇格できるかを source-backed に確認し、必要なら current boundary を docs / plan に反映する
- Decision levels touched: L2

## 1. Objective

current L2 の first checker cut と static gate artifact loop を前提に、

- `expected_static.reasons` をそのまま `run_bundle()` の machine-check core に上げられるか
- 上げられないなら、current carrier boundary をどう明記すべきか

を確認する。

## 2. Inputs consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/07-current-l2-host-stub-harness.md`
9. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
10. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
11. `plan/07-parser-free-poc-stack.md`
12. `plan/11-roadmap-near-term.md`
13. `plan/12-open-problems-and-risks.md`
14. `plan/15-current-l2-fixture-authoring-template.md`
15. `plan/90-source-traceability.md`
16. `progress.md`
17. `crates/mir-semantics/src/lib.rs`
18. `crates/mir-semantics/src/harness.rs`
19. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
20. `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. `run_bundle()` が現在 `expected_static.verdict` しか見ていないことを確認した。
2. `e5-underdeclared-lineage` を clone して `expected_static.reasons` を壊した bundle が `run_bundle()` を通ってしまう RED test を追加し、current mismatch を固定した。
3. `run_bundle()` で `expected_static.reasons` を fail-closed 比較する narrow fix を一度入れた。
4. そのうえで `valid` fixture 側の explanatory `expected_static.reasons` を壊した bundle も `run_bundle()` で落ちる RED test を追加し、さらに full `cargo test -p mir-semantics` を実行した。
5. full suite の failure を調べ、current fixture corpus では
   - `malformed` / `underdeclared` fixture の `expected_static.reasons`
   - `valid` fixture の `expected_static.reasons`
   のどちらにも human-facing explanation が混ざっており、`static_gate_detailed().reasons` と 1 対 1 対応しないことを確認した。
6. そのため actual code change は revert し、current judgment を docs / plan / progress に反映した。
7. `plan/90-source-traceability.md` に report 0141 を追加した。

## 4. Files changed

- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0141-current-l2-static-gate-reasons-machine-check.md`

## 5. Commands run and exact outputs

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_rejects_static_reason_mismatch -- --nocapture
thread 'run_bundle_rejects_static_reason_mismatch' panicked
called `Result::unwrap_err()` on an `Ok` value: BundleRunReport { ... static_verdict: Underdeclared ... }
FAILED
```

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_keeps_valid_fixture_static_reasons_as_explanatory_only -- --nocapture
thread 'run_bundle_keeps_valid_fixture_static_reasons_as_explanatory_only' panicked
called `Result::unwrap()` on an `Err` value: InvalidProgram("bundle static reasons mismatch for e3_option_admit_chain: expected [\"explanatory static prose should not fail runtime bundle\"], got []")
FAILED
```

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --nocapture
failures:
    run_bundle_checks_static_runtime_and_trace_expectations
    run_directory_profiled_static_only_includes_profile_name_in_summary
    run_directory_profiled_static_single_fixture_runs_one_static_bundle
    run_directory_returns_summary_for_current_l2_fixture_dir
...
InvalidProgram("bundle static reasons mismatch for e5_underdeclared_lineage: expected [\"declared access target alone is not enough\", \"edge-local lineage evidence is missing\"], got [\"missing lineage assertion for primary -> mirror\"]")
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 141 numbered report(s).
```

```text
git diff --check
[no output]
```

```text
cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed
test result: ok. 2 passed; 0 failed
test result: ok. 36 passed; 0 failed
test result: ok. 3 passed; 0 failed
test result: ok. 0 passed; 0 failed
```

```text
python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json --artifact-root target/current-l2-detached --run-label reasons-boundary-e4 --reference-label reasons-boundary-e5 --overwrite
static gate artifact: target/current-l2-detached/static-gates/reasons-boundary-e4/e4-malformed-lineage.static-gate.json
reference static gate artifact: target/current-l2-detached/static-gates/reasons-boundary-e5/e5-underdeclared-lineage.static-gate.json
=== current L2 static gate artifact diff ===
checker_core differences:
- checker_core.static_verdict: left="malformed" right="underdeclared"
- checker_core.reasons: left=["lineage assertion does not describe primary -> mirror"] right=["missing lineage assertion for primary -> mirror"]
```

## 6. Evidence / findings

- `expected_static.reasons` は current fixture corpus で一貫した machine-check carrier ではない。
- `valid` fixture では、`expected_static.reasons` は static gate の actual result ではなく explanatory note として使われている。
- `underdeclared` fixture でも、`expected_static.reasons` は `static_gate_detailed().reasons` の exact mirror ではなく、より user-facing な wording を持つ。
- したがって、`expected_static.reasons` をそのまま `run_bundle()` の exact machine-check core に昇格させると、current fixture corpus の意図と衝突する。
- actual `reasons` compare を current L2 で行う場所としては、`specs/examples/32-current-l2-static-gate-artifact-loop.md` の detached static gate artifact helper 側に残すのが自然である。
- revert 後の full `cargo test -p mir-semantics` は green に戻り、static gate actual `reasons` compare 自体は `smoke-static-gate` の detached loop で引き続き取れる。

## 7. Changes in understanding

- `expected_static.verdict` と `expected_static.reasons` は current schema の中で同じ強度の carrier ではない。
- `verdict` は current harness core に置いてよいが、`reasons` は explanatory note と machine-check 候補の二重用途になっている。
- したがって、future checker API で static reason compare を core に上げたいなら、
  - typed reason code
  - dedicated checked reason carrier
  のどちらかを別立てにする必要がある。
- current repo の narrow judgmentとしては、
  - harness / bundle machine-check: `expected_static.verdict`
  - detached static gate artifact compare: actual `checker_core.reasons`
  と切り分けるのが最も自然である。

## 8. Open questions

- future checker API で static reason compare を core に上げるなら、
  - `expected_static.checked_reasons`
  - `expected_static.reason_codes`
  - detached-only compare 維持
  のどれが最小か。
- `expected_static.reasons` を explanatory note に寄せ切るか、それとも将来 migration で分離するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、expected_static.reasons の dual-use carrier を future checker API でどう分離するのが最小かを、checked_reasons / typed reason code / detached-only 維持の 3 案で source-backed に比較してください。`
