# Report 0155 — static-only fixture authoring tranche

- Date: 2026-04-05T21:20:00+09:00
- Author / agent: Codex
- Scope: current L2 parser-free PoC の detached validation loop を前提に、static-only fixture corpus へ stable static-gate cluster を 2 本追加し、scaffold helper・display-only `suggest-checked-reasons`・static gate detached loop・bundle/aggregate smoke までを実地に通す
- Decision levels touched: L2

## 1. Objective

- lineage-only だった static-only malformed / underdeclared corpus を、actual static gate が既に持つ stable cluster へ広げる。
- `expected_static.checked_reasons` の narrow adoption を `e4` / `e5` から `declared target missing` / `capability strengthens` まで拡張し、fixture authoring 実務の friction を実地で確認する。
- helper boundary や runtime semantics は変えず、fixture / tests / docs / plan / progress だけで detached validation loop を一段進める。

## 2. Scope and assumptions

- current L2 runtime semantics、parser grammar、failure family、machine-check policyは変えない。
- `checked_reasons` は additive optional bridge のままとし、fixture JSON の auto-fill は行わない。
- `host_plan_coverage_failure`、aggregate typed carrier、public exporter API は今回触らない。
- scaffold helper は boilerplate の operational evidence にのみ使い、repo tracked fixture の正本は hand-written JSON に置く。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `scripts/current_l2_scaffold_fixture.py`
- `scripts/current_l2_detached_loop.py`

## 4. Actions taken

1. current static-only corpus と actual static gate reason families を棚卸しし、未カバーの stable cluster 候補を比較した。
2. code mapper subagent `Helmholtz` を dispatch し、`declared target missing` と `capability strengthens` が最小 tranche に最も向くこと、および blast radius が fixture JSON / tests / plan mirror に主に限られることを確認した。
3. `current_l2_minimal_interpreter.rs` と `current_l2_static_gate_support.rs` に failing test を先に追加し、新 fixture 不在で RED を確認した。
4. `scripts/current_l2_scaffold_fixture.py` を static-only mode で実行し、`e12-underdeclared-target-missing` と `e13-malformed-capability-strengthening` の skeleton を `target/current-l2-fixture-scaffolds/` 下へ生成して current authoring flow を実地確認した。
5. repo tracked fixture として `e12-underdeclared-target-missing.json` と `e13-malformed-capability-strengthening.json` を hand-write で追加し、`checked_reasons` と explanatory `reasons` を分けた current cut に合わせた。
6. `suggest-checked-reasons` を両 fixture へ実行し、fixture 側 `checked_reasons` が actual static gate wording と既に一致していることを確認した。
7. `smoke-static-gate` と `smoke-fixture` を新 fixture に対して実行し、static gate artifact / bundle artifact / aggregate summary の detached loop が static-only tranche でも回ることを確認した。
8. fixture catalog / template / checked-reasons docs / roadmap / progress を更新し、lineage-only だった actual corpus の理解を stable cluster 側へ広げた。

## 5. Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json`
- `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0155-static-only-fixture-authoring-tranche.md`

## 6. Commands run and exact outputs

```text
$ cargo test -p mir-semantics --test current_l2_minimal_interpreter static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.63s
     Running tests/current_l2_minimal_interpreter.rs (...)

running 1 test
test static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording ... FAILED

thread '...' panicked ... fixture should load: Io(... No such file or directory)
```

```text
$ cargo test -p mir-semantics --test current_l2_static_gate_support static_gate_artifact_emits_reason_codes_for_target_and_capability_clusters -- --exact
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.04s
     Running tests/current_l2_static_gate_support.rs (...)

running 1 test
test static_gate_artifact_emits_reason_codes_for_target_and_capability_clusters ... FAILED

thread '...' panicked ... called `Result::unwrap()` on an `Err` value: Io(... No such file or directory)
```

```text
$ python3 scripts/current_l2_scaffold_fixture.py e12-underdeclared-target-missing --kind static-only --source-example-id E12
static-only scaffold follow-up: after first authoring pass, if this fixture becomes malformed/underdeclared, run `python3 scripts/current_l2_detached_loop.py suggest-checked-reasons target/current-l2-fixture-scaffolds/e12-underdeclared-target-missing.json --run-label TODO --overwrite` to inspect actual static gate wording.
target/current-l2-fixture-scaffolds/e12-underdeclared-target-missing.json
```

```text
$ python3 scripts/current_l2_scaffold_fixture.py e13-malformed-capability-strengthening --kind static-only --source-example-id E13
static-only scaffold follow-up: after first authoring pass, if this fixture becomes malformed/underdeclared, run `python3 scripts/current_l2_detached_loop.py suggest-checked-reasons target/current-l2-fixture-scaffolds/e13-malformed-capability-strengthening.json --run-label TODO --overwrite` to inspect actual static gate wording.
target/current-l2-fixture-scaffolds/e13-malformed-capability-strengthening.json
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json --run-label e12-authoring --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e12-authoring/e12-underdeclared-target-missing.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json
artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e12-authoring/e12-underdeclared-target-missing.static-gate.json
current expected_static.checked_reasons: ["declared access target is missing for primary -> mirror"]
current expected_static.checked_reasons already matches actual static gate reasons
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label e13-authoring --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e13-authoring/e13-malformed-capability-strengthening.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json
artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e13-authoring/e13-malformed-capability-strengthening.static-gate.json
current expected_static.checked_reasons: ["capability strengthens from read to write"]
current expected_static.checked_reasons already matches actual static gate reasons
```

```text
$ python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json --run-label e12-static-loop --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e12-static-loop/e12-underdeclared-target-missing.static-gate.json
```

```text
$ python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label e13-static-loop --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/e13-static-loop/e13-malformed-capability-strengthening.static-gate.json
```

```text
$ python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json --run-label e12-bundle-loop --overwrite
fixture artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/e12-bundle-loop/e12-underdeclared-target-missing.detached.json
aggregate artifact (full)  : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e12-bundle-loop-full/batch-summary.detached.json
aggregate artifact (single): /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e12-bundle-loop-single/batch-summary.detached.json
=== current L2 detached aggregate diff ===
summary_core differences:
- summary_core.total_bundles: left=13 right=1
- summary_core.runtime_bundles: left=9 right=0
- summary_core.static_only_bundles: left=4 right=1
- summary_core.passed: left=13 right=1
```

```text
$ python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label e13-bundle-loop --overwrite
fixture artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/e13-bundle-loop/e13-malformed-capability-strengthening.detached.json
aggregate artifact (full)  : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e13-bundle-loop-full/batch-summary.detached.json
aggregate artifact (single): /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/aggregates/e13-bundle-loop-single/batch-summary.detached.json
=== current L2 detached aggregate diff ===
summary_core differences:
- summary_core.total_bundles: left=13 right=1
- summary_core.runtime_bundles: left=9 right=0
- summary_core.static_only_bundles: left=4 right=1
- summary_core.passed: left=13 right=1
```

## 7. Evidence / findings

- `declared target missing` と `capability strengthens` は、actual static gate が既に出す stable reason family であり、new semantics を追加せずに static-only corpus を広げられる。
- `checked_reasons` は両 fixture で actual static gate wording と一致し、display-only assist を authoring 後段の confirmation として使える。
- detached static gate artifact 側の helper-local `reason_codes` mirror も、new cluster をそのまま分類できる。
- bundle / aggregate smoke を static-only fixture にも流せるため、fixture authoring 実務の next friction は helper boundary そのものよりも fixture 数増加に伴う catalog / count / profile maintenance 側へ移り始めている。
- reviewer completion では、helper-boundary regression や semantic mismatch は見つからず、残っていた問題は `plan/01` の corpus summary drift、`E12` / `E13` source example の prose 不足、`progress.md` task-close log 欠落の 3 点だった。これらは `docs/reports/0156-review-static-only-fixture-authoring-tranche.md` で follow-up 済みである。
- follow-up 後の fresh verification では `cargo test -p mir-semantics` が全 green、`python3 scripts/validate_docs.py` が `Found 156 numbered report(s).`、`git diff --check` が無出力、`smoke-static-gate` / `smoke-fixture` が `e12` / `e13` の両方で再度通った。

## 8. What changed in understanding

- `checked_reasons` の narrow adoption は lineage-only corpus に留める必要がなく、stable cluster であれば declared target / capability monotonicity 側にも広げられる。
- scaffold helper は production authoring API ではないが、static-only fixture の first pass を detached validation loop へ接続する operational evidence として十分役立つ。
- next bottleneck は `checked_reasons` assist 自体より、fixture corpus 増加時に representative catalog / profile / selection count をどこまで mirror すべきかの discipline である。

## 9. Open questions

- stable cluster をどこまで actual corpus に広げると、checked-reason maintenance cost が benefit を上回るか。
- duplicate declaration 系の static reason familiesにも actual corpus を広げるか、それとも detached reason-code mirror と docs-only inventory に留めるか。
- static-only fixture 増加を named profile / representative example prose へどこまで反映するのが current mainline に適切か。

## 10. Suggested next prompt

- current L2 parser-free PoC 基盤を前提に、duplicate declaration 系を actual corpus へ入れる前に、static-only fixture 増加が named profile / representative examples / aggregate summary の運用コストにどう効くかを narrow に比較してください。
