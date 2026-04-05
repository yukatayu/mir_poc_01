# 0202 — try-rollback AST helper detached-loop insertion

## Objective

future dedicated AST structural helper を detached validation loop のどこへ差し込むのが最小かを、bundle-first runtime path、static gate artifact loop、generic checker-side shared entry の 3 案で source-backed に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual subcommand 追加、actual helper 実装、fixture schema actualization は行わない。
- `plan/08` は representative fixture catalog 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_detached_loop.py`
- `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`

## Actions taken

1. detached validation loop への差し込み位置を、bundle-first runtime path、static gate artifact loop、generic checker-side shared entry の 3 案で比較した。
2. `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md` を追加し、future dedicated helper は bundle-first runtime path へ混ぜず、static gate artifact loop の helper-local smoke family に留める current judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0202-try-rollback-ast-helper-detached-loop-insertion.md`
- `docs/reports/0203-review-try-rollback-ast-helper-detached-loop-insertion.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 203 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local inspection で確認した code anchor
  - `scripts/current_l2_detached_loop.py` では `smoke-fixture` / `smoke-try-rollback-locality` が bundle-first runtime path、`smoke-static-gate` が static gate path、`smoke-same-lineage-checker` / `smoke-missing-option-checker` / `smoke-capability-checker` が static-gate-side checker smoke family である
  - `current_l2_emit_static_gate.rs` と `current_l2_static_gate_support.rs` は `CurrentL2Fixture + StaticGateResult -> detached static gate artifact` の narrow path であり、AST structural helper を runtime event path へ混ぜなくて済む

## What changed in understanding

- dedicated AST structural helper を detached validation loop に載せるとしても、bundle-first runtime path に混ぜるより、static gate artifact emit のあとに helper-local compare を回す family に置く方が current split と整合する。
- これにより、runtime representative `smoke-try-rollback-locality` は引き続き runtime evidence の contrast pair に専念できる。

## Open questions

- structural verdict carrier / name をどこで切るか。
- actual subcommand 名をいつ決めるか。
- malformed static family を actual corpus に増やす必要が本当にあるか。

## Suggested next prompt

future dedicated AST structural helper を detached validation loop の static-gate-side family に載せる current judgmentを前提に、その helper の structural verdict carrier / name をどこまで narrow に切るかを docs-only で比較してください。shared detached carrier や actual subcommand 追加にはまだ進まないでください。

