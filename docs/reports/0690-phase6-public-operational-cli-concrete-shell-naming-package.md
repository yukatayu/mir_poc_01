# Report 0690 — phase6 public operational CLI concrete shell naming package

- Date: 2026-04-13T21:41:00+0900
- Author / agent: Codex
- Scope: `specs/examples/399...400` による public operational CLI concrete shell naming comparison close と、その mirror update
- Decision levels touched: L2

## 1. Objective

runtime-led thin facade first cut、Rust-side operational wrapper second gate、runtime skeleton later support cutを巻き戻さずに、public operational CLI の concrete shell naming をどの narrow shell concern に留めるかを固定する。

## 2. Scope and assumptions

- current package は docs-only naming comparison に留める。
- current public shell concern は `sample selector + explicit host-plan path + pretty/json output` に限ると仮定する。
- final `mir` top-level hierarchy、actual CLI binary、repo-local helper verb naming は still later とみなす。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/371...372`
- `specs/examples/389...390`
- `specs/examples/393...394`
- `specs/examples/395...396`
- `specs/examples/397...398`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`

## 4. Actions taken

1. current public shell concern を final public API / final host-input contract / repo-local helper verb naming から切り離して整理した。
2. current-L2 scoped shell `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` を docs-only first cut に置く comparison / threshold pair `399...400` を追加した。
3. snapshot / plan / abstract / FAQ / sample docs を current line `stable malformed capability second source-backed widening actualization comparison` に合わせて更新した。
4. `plan/90-source-traceability.md` に今回 package の addendum を追加した。

## 5. Evidence / outputs / test results

- current shell naming judgment は次の通り。
  - current shell family は `mir-current-l2`
  - current operational verb は `run-source-sample`
  - current shell concern は `<sample>`、`--host-plan <path>`、`--format pretty|json`
  - final `mir` hierarchy、inventory/regression verb、runtime-skeleton / lowering support verb、repo-local Python helper / cargo example naming は excluded bucket に残す
- Commands run:
  - `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
  - `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## 6. What changed in understanding

- public operational CLI の concrete shell naming は final `mir` hierarchy や actual binary binding を先に決めなくても、current-L2 scoped shell と single operational verb だけを docs-only first cut に置けば drift を十分抑えられる。
- inventory / regression helper と support-only verb は current public shell concern から明確に分離した方が、Rust-side operational wrapper second gate の判断と整合する。

## 7. Open questions

- current-L2 scoped shell naming を later actualization でどう executable 名へ落とすか
- `--format pretty|json` を later shell で enum flag のまま保つか
- public operational CLI concrete shell actualization をどの later support / public API gate の後に reopen するか

## 8. Suggested next prompt

`tasks.md` 先頭の `stable malformed capability second source-backed widening actualization comparison` を、そのまま次の package として自走してください。
