# 0222 — try-rollback AST helper first tranche actualization

## Objective

`TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche を、current docs-only chain (`specs/examples/65`〜`67`) に従って actualize し、

- helper-local compare helper
- fixture-side additive expected field
- minimal malformed static family (`e23` / `e24`)
- static gate artifact loop の family-specific smoke path

までを current repo に入れる。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- current task は helper-local first tranche actualization だけを扱う。
- runtime semantics、parser grammar、failure family、public checker API は変更しない。
- `must_explain` は machine-check に上げない。
- shared detached carrier、generic structural checker family、public checker API は今回も外に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md`
- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `specs/examples/61-current-l2-try-rollback-ast-helper-subcommand-and-wrapper-family.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `scripts/current_l2_detached_loop.py`

## Actions taken

1. TDD で first tranche actualization の write set を切った。
   - Python 側では dedicated helper script と wrapper subcommand の failing test を先に書いた。
   - Rust 側では `e23` / `e24` と typed expected field decode の failing test を先に足した。
2. RED を確認した。
   - dedicated helper script / wrapper が存在しない状態で Python tests が落ちることを確認した。
   - `static_gate_rejects_malformed_and_underdeclared_fixtures` に `e23` / `e24` を追加した状態で、static gate がまだ `Valid` を返して落ちることを確認した。
   - typed expected field decode test も field 未定義で落ちることを確認した。
3. minimal implementation を追加した。
   - `scripts/current_l2_try_rollback_structural_checker.py`
   - `scripts/current_l2_detached_loop.py` の `smoke-try-rollback-structural-checker`
   - `TryRollbackStructuralVerdict`
   - `TryRollbackStructuralSubjectKind`
   - `TryRollbackStructuralFindingKind`
   - `TryRollbackStructuralFindingRow`
   - `ExpectedStatic.checked_try_rollback_structural_verdict`
   - `ExpectedStatic.checked_try_rollback_structural_findings`
   - `static_gate_detailed()` の first-tranche structural malformed check
   - dedicated helper は fixture AST の再計算ではなく emitted static gate artifact の `checker_core.static_verdict` / `checker_core.reasons` を helper-local row family へ写して compare する cut に揃えた。
4. minimal malformed static family として `e23` / `e24` を追加した。
5. negative coverage を追加した。
   - artifact から `checker_core.static_verdict` が欠ける場合
   - unrelated reason wording しか乗っていない場合
   の dedicated helper failure を Python test で固定した。
6. docs / plan / progress / report mirror を current code anchor に揃えた。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `scripts/tests/test_current_l2_try_rollback_structural_checker.py`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`
- `docs/reports/0220-try-rollback-malformed-pattern-slot-selection.md`
- `docs/reports/0221-review-try-rollback-malformed-pattern-slot-selection.md`
- `docs/reports/0222-try-rollback-ast-helper-first-tranche-actualization.md`
- `docs/reports/0223-review-try-rollback-ast-helper-first-tranche-actualization.md`

## Commands run

```bash
rm -rf scripts/__pycache__ scripts/tests/__pycache__
python3 -m unittest scripts.tests.test_current_l2_try_rollback_structural_checker scripts.tests.test_current_l2_static_gate_loop
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json --run-label try-rollback-e23 --overwrite
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json --run-label try-rollback-e24 --overwrite
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- Python targeted tests

```text
....
----------------------------------------------------------------------
Ran 4 tests in ...

OK
```

- `smoke-try-rollback-structural-checker` for `e23`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- `smoke-try-rollback-structural-checker` for `e24`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- negative helper coverage

```text
python3 -m unittest scripts.tests.test_current_l2_try_rollback_structural_checker
.....
----------------------------------------------------------------------
Ran 5 tests in ...

OK
```

- `cargo test -p mir-semantics`

```text
test result: ok.
```

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
```

- `git diff --check`

```text
<no output>
```

## What changed in understanding

- `TryFallback` / `AtomicCut` dedicated AST structural helper は、docs-only threshold を越え、helper-local first tranche として actualize 済みになった。
- ただし current actualization は public checker core ではなく、static gate artifact loop にだけ差し込まれた dedicated smoke family に留まる。
- current dedicated helper は fixture AST を再計算して green を作るのではなく、emitted artifact contract を narrow に検査する helper-local compare として維持する。
- `missing_fallback_body` と `disallowed_fallback_placement` は current first tranche の working family として十分 source-backed になったが、shared carrier や generic family へ上げるにはまだ追加比較が必要である。

## Open questions

- second malformed static tranche を追加する threshold をどこに置くか。
- first-tranche wording / `finding_kind` family を何 task くらい helper-local のまま維持するか。
- saved artifact compare need が shared carrier threshold を本当に満たしているか。

## Suggested next prompt

`TryFallback` / `AtomicCut` dedicated AST structural helper first tranche の current actualization を前提に、second malformed static tranche を足すべきか、それとも helper-local wording / finding family を first tranche のまま数回反復してから shared carrier / public checker comparison へ進むべきかを narrow に比較してください。
