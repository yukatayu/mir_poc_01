# Report 0723 — sample artifact preview third tranche

- Date: 2026-04-17T03:44:32.360176Z
- Author / agent: Codex
- Scope: helper-local sample artifact preview の追加、underdeclared source-form gap の preservation bucket 整理、snapshot / plan mirror 同期
- Decision levels touched: L2, L3

## 1. Objective

current runnable sample / prototype を使った研究で、theorem / model-check bridge の current floor をもう一段 sample-visible にしつつ、underdeclared family の source-form omission gap を current parser / lowerer convenience cut の範囲で明示する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples/prototype/README.md`
- `samples/not_implemented/README.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `specs/00-document-map.md`
- `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`
- `specs/examples/452-current-l2-debug-output-preview-helper-cut.md`
- `specs/examples/453-current-l2-sample-verification-preview-and-prototype-second-tranche.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json`

## 3. Actions taken

1. `current_l2_operational_cli` の test expectation を先に拡張し、`artifact_preview` が無い状態で RED を確認した。
2. operational CLI summary に helper-local `artifact_preview` を追加し、runtime/static/guarded の 3 状態で derived preview row を出せるようにした。
3. `samples/not_implemented/current-l2-underdeclared/` を新設し、missing lineage assertion / missing declared target の source-form omission stimulus を preservation した。
4. `specs/examples/454` を追加し、artifact preview cut と underdeclared source-form gap の current judgment を docs-first に固定した。
5. `Documentation.md`、`progress.md`、`tasks.md`、`plan/08`、`plan/09`、`specs/00-document-map.md`、`faq_005.md`、`plan/90-source-traceability.md` を current snapshot に同期した。

## 4. Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `samples/prototype/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/current-l2-underdeclared/u01-missing-lineage-assertion.txt`
- `samples/not_implemented/current-l2-underdeclared/u02-missing-declared-target.txt`
- `specs/examples/454-current-l2-artifact-preview-and-underdeclared-source-gap-note.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `faq_005.md`
- `docs/reports/0723-sample-artifact-preview-third-tranche.md`

## 5. Commands run and exact outputs

```bash
$ cargo test -p mir-runtime --test current_l2_operational_cli -- --nocapture
```

RED phase:

- `operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted` failed because `artifact_preview:` was absent.
- `operational_cli_json_reports_static_verification_preview_for_prototype_sample` failed because `artifact_preview.proof_notebook_review_units[0].obligation_kind` was `Null`.
- `operational_cli_json_reports_guarded_verification_preview_for_prototype_sample` failed because `artifact_preview.proof_notebook_review_units` was `Null` instead of `[]`.

GREEN phase:

- `7 passed; 0 failed`

```bash
$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.txt --format pretty
```

- `terminal_outcome: success`
- `artifact_preview:` が表示され、`rollback_cut_non_interference` の goal/evidence preview が出た。

```bash
$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p04-dice-owner-duplicate-declaration.txt --format pretty
```

- `static_gate: malformed`
- `artifact_preview:` が表示され、`canonical_normalization_law` と `no_re_promotion` の preview が出た。

```bash
$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p05-dice-owner-guarded-chain.txt --format pretty
```

- `verification_preview.formal_hook_status: guarded_not_reached`
- `artifact_preview.proof_notebook_review_units: []`
- `artifact_preview.model_check_concrete_carriers: []`

```bash
$ python3 scripts/validate_docs.py
```

- `Documentation scaffold looks complete.`
- `Found 722 numbered report(s).`

```bash
$ git diff --check
```

- no output

## 6. Evidence / findings

- helper-local `artifact_preview` は runtime/static/guarded の 3 状態で一貫して出せる。
- underdeclared family は fixture-level では source-backed だが、source text omission は current parser / lowerer convenience cut でまだ扱えない。
- したがって current repo では、underdeclared omission stimulus を `samples/not_implemented/current-l2-underdeclared/` に preservation するのが最小である。

## 7. Changes in understanding

- theorem / model-check bridge の sample-visible floor は `verification_preview` だけでなく、derived row-level `artifact_preview` まで helper-local に深めてよい。
- underdeclared family は「理論未整備」ではなく、「fixture-level floor はあるが source-form omission を current parser cut がまだ受けない」という gap として読む方が正確である。

## 8. Open questions

- underdeclared source-form omission を current parser convenience cut に入れるかどうか。
- helper-local `artifact_preview` を emitted artifact helper example へ寄せるか、それとも CLI summary にだけ留めるか。
- prototype third tranche を typed / theorem / model-check mixed-gate topic へどこまで広げるか。

## 9. Suggested next prompt

- `samples/prototype/` の third tranche として、typed / theorem / model-check mixed-gate topic を corrected prototype と helper preview でどこまで sample-visible にできるか、comparison package と small executable helper を進めてください。
