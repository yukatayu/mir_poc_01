# Report 0150 — checked static reasons authoring assist

- Date: 2026-04-05T11:05:03.710829Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC における `expected_static.checked_reasons` の display-only authoring assist を docs/code/helper/plan/progress へ narrow に追加する
- Decision levels touched: L2

## 1. Objective

- `checked_reasons` の narrow adoption を補助する最小 helper を追加する。
- helper は fixture JSON を auto-fill / rewrite しない。
- detached static gate artifact の actual `checker_core.reasons` を source とする display-only assist に留める。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_scaffold_fixture.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_scaffold_fixture.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `checked_reasons` assist の候補を比較し、auto-fill ではなく display-only を current cut と判断した。
2. `scripts/current_l2_checked_reasons_assist.py` を追加し、fixture JSON と static gate artifact を読み、copyable な `checked_reasons` snippet を表示する helper を実装した。
3. `scripts/current_l2_detached_loop.py` に `suggest-checked-reasons` subcommand を追加し、static gate artifact emit 後に上記 assist を呼ぶ thin wrapper を追加した。
4. Python tests を追加し、assist 単体と wrapper delegation を RED/GREEN で固定した。
5. `specs/examples/36-current-l2-checked-reasons-authoring-assist.md` を新設し、display-only / no-rewrite policy を docs-first で明示した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。
7. reviewer は 1 回だけ回す前提で準備したが、この作業環境では completion を受け取れなかったため、local diff inspection と fresh validation evidence を report に残す fallback で閉じることにした。

## 4. Files changed

- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `scripts/current_l2_checked_reasons_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_checked_reasons_assist.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 5. Commands run and exact outputs

```text
$ python3 -m unittest scripts.tests.test_current_l2_checked_reasons_assist scripts.tests.test_current_l2_static_gate_loop
... initial run failed:
- ModuleNotFoundError: No module named 'current_l2_checked_reasons_assist'
- AttributeError: module 'current_l2_detached_loop' has no attribute 'checked_reasons_assist'
```

```text
$ python3 -m unittest scripts.tests.test_current_l2_checked_reasons_assist scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_scaffold_fixture
Ran 10 tests in 0.007s
OK
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --run-label suggest-e4 --overwrite
static gate artifact: .../target/current-l2-detached/static-gates/suggest-e4/e4-malformed-lineage.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json
artifact: .../e4-malformed-lineage.static-gate.json
current expected_static.checked_reasons: ["lineage assertion does not describe primary -> mirror"]
current expected_static.checked_reasons already matches actual static gate reasons
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label suggest-e3 --overwrite
static gate artifact: .../target/current-l2-detached/static-gates/suggest-e3/e3-option-admit-chain.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json
artifact: .../e3-option-admit-chain.static-gate.json
current expected_static.checked_reasons: absent
actual static gate reasons are empty; no checked_reasons suggestion
```

```text
$ cargo test -p mir-semantics
test result: ok. 49 passed; 0 failed
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 150 numbered report(s).
```

```text
$ git diff --check
(no output)
```

## 6. Evidence / findings

- `checked_reasons` assist は auto-fill ではなく display-only に留めても、actual static gate wording の adoption を十分補助できる。
- `e4` では current fixture の `checked_reasons` が actual wording と既に一致していることを smoke で確認できた。
- `e3` では actual static gate `reasons` が空であり、no-op status に留める方が current non-adoption policy と整合する。
- assist を detached static gate artifact の `checker_core.reasons` に寄せることで、fixture-side `reasons` explanation や helper-local `reason_codes` mirror を machine-check source と混同せずに済む。

## 7. Changes in understanding

- `checked_reasons` の narrow adoption は、scaffold helper に auto-fill を足さなくても進められる。
- authoring assist は detached validation loop の static gate path に載せるのが自然であり、scaffold helper へ統合する必要は current cut ではない。
- valid fixture 群への `checked_reasons = []` 非採用方針と、malformed / underdeclared fixture への narrow adoption は両立する。

## 8. Open questions

- `suggest-checked-reasons` を detached validation loop の default smoke flow に入れるか。
- `checked_reasons` の次段で fixture-side typed row をいつ導入するか。
- helper-local `detached_noncore.reason_codes` を assist 表示へどこまで使うか。
- `plan/` 更新不要 ではない。`plan/07`、`plan/09`、`plan/11`、`plan/15`、`plan/90` を同 task で更新した。

## 9. Suggested next prompt

- `checked_reasons` assist の次段として、static-only malformed / underdeclared fixture の scaffold / authoring で actual static gate wording をどこまで template に取り込んでよいかを、display-only / copy-paste helper / partial scaffold note の 3 案で narrow comparison してください。
