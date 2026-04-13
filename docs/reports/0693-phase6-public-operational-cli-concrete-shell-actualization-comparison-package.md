# Report 0693 — phase6 public operational CLI concrete shell actualization comparison package

- Date: 2026-04-13T23:56:08+0900
- Author / agent: Codex
- Scope: `specs/examples/403...404` による public operational CLI concrete shell actualization comparison close と、その mirror update
- Decision levels touched: L2

## 1. Objective

current-L2 scoped docs-only shell naming を維持したまま、public operational CLI concrete shell actualization の current first cut をどこに置くかを fixed する。

## 2. Scope and assumptions

- current package は docs-only actualization comparison に留める。
- `run_current_l2_source_sample` + `CurrentL2SourceSampleRunReport` の runtime-led thin facade first cut は巻き戻さない。
- `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` は later support cut に残し、`lower_current_l2_fixed_source_text` と repo-local helper 群は public shell concern に含めない。
- `plan/09-helper-stack-and-responsibility-map.md` は current public/support split の明文化が変わるため更新対象とする。

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
- `specs/examples/393...396`
- `specs/examples/399...402`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_runtime_skeleton.rs`

## 4. Actions taken

1. thin facade first cut、Rust-side operational wrapper second gate、runtime skeleton later support cut、docs-only shell naming の current split を再確認した。
2. current-L2 scoped shell naming を actual shell concern へ reopen する comparison / threshold pair `403...404` を追加した。
3. current first choice を `mir-current-l2 run-source-sample` を包む current-L2 scoped Rust concrete shell over thin facade に固定し、support / excluded / kept-later bucket を明記した。
4. snapshot / plan / abstract / FAQ / sample docs を current line `stable malformed capability second source-backed widening actualization` に合わせて更新した。
5. `plan/90-source-traceability.md` に今回 package の addendum を追加した。

## 5. Evidence / outputs / test results

- current judgment は次の通り。
  - current actual shell concern は `mir-current-l2 run-source-sample`
  - delegated entry / report は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport`
  - operational shell concern は `<sample>`、`--host-plan <path>`、`--format pretty|json`
  - `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` は support refs に留める
  - `lower_current_l2_fixed_source_text`、path resolver、accepted-set hard-coding、repo-local inventory / regression helper、repo-local Python helper、cargo example emitter / support moduleは excluded bucket に残す
  - installed binary packaging、final `mir` hierarchy、final public parser/checker/runtime API、final host/input contract は kept-later に残す
- Commands run:
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `cargo test -p mir-runtime --test current_l2_runtime_skeleton`
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## 6. What changed in understanding

- current-L2 scoped shell naming は docs-only label に留まらず、runtime-led thin facade を包む current-L2 scoped Rust shell concern として narrow に actualization comparison へ送れる。
- current public operational shell concern を narrow に保つには、runtime skeleton later support と repo-local maintenance helper を threshold 側で明示的に外す必要がある。

## 7. Open questions

- current-L2 scoped shell を later actual binary へどう packaging するか
- `--host-plan` を later public contract で path flag のまま維持するか
- maintenance helper shell と public shell を later でどう分離命名するか

## 8. Suggested next prompt

`tasks.md` 先頭の `stable malformed capability second source-backed widening actualization` を、そのまま次の package として自走してください。
