# Report 1169 — P-A1-11 Product-Preview Avatar Companion Widening

- Date: 2026-05-04
- Author / agent: Codex
- Scope: practical alpha-1 product-preview widening over exact avatar preview reports
- Decision levels touched: `L1`, `L2`

## Objective

Close `P-A1-11` by widening the practical product-preview floor so that `PE2E-08/09` can consume exact `AV-A1-02/03` avatar preview reports as thin companion bundles, without collapsing those rows into native execution, same-session runtime attachment, unsupported-runtime execution success, or final product/runtime ABI claims.

## 日本語要約

`P-A1-11` では、`PE2E-08/09` を追加して、product-preview lane が `AV-A1-02/03` exact avatar preview reports を thin companion bundle として consume できるようにした。これは `PE2E-*` を widened する作業であり、custom-avatar companion preview と unsupported-runtime visible fallback companion preview を actualize するが、native execution、same-session runtime attach/detach execution、unsupported-runtime execution success、full product prototype completion は claim しない。

## Scope and assumptions

- `PE2E-08` と `PE2E-09` だけを新規 actualize する。
- `AV-A1-01/02/03` は source authority の distinct avatar-preview companion floor のまま維持する。
- `HP-A1-06` placeholder object preview seam と `PE2E-04` narrow 読みは変えない。
- `samples/practical-alpha1/` は active runnable root に昇格しない。
- native execution、same-session runtime attach/detach execution、unsupported-runtime execution success、full practical product prototype completion、final public CLI / viewer / transport / save-load / package-avatar ABI は scope 外とする。
- sub-agent delegation は current tool policy 上 user explicit request が無いため使わず、local focused review で閉じる。

## Start state / dirty state

- Start state was dirty.
- Existing in-flight implementation changes for `P-A1-11` were already present in:
  `scripts/practical_alpha1_product_preview.py`,
  `scripts/tests/test_practical_alpha1_product_preview.py`,
  `README.md`,
  `Documentation.md`,
  `specs/18-practical-alpha1-scope.md`,
  `plan/01-status-at-a-glance.md`,
  `plan/44-practical-alpha1-roadmap.md`,
  `samples/README.md`,
  `samples/practical-alpha1/previews/README.md`,
  `samples/practical-alpha1/expected/README.md`,
  `scripts/README.md`,
  plus new untracked preview/expected files for `PE2E-08/09`.
- Snapshot docs sync, dashboard sync, and this report were completed in the same task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/previews/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`
- `docs/reports/1167-p-a1-08-first-practical-product-preview-floor.md`
- `docs/reports/1168-p-a1-10-practical-avatar-preview-floor.md`

## Actions taken

- Widened `scripts/practical_alpha1_product_preview.py` so the product-preview lane can read exact avatar preview reports as source carriers.
- Added bundle kinds and source-report checks for:
  - `PE2E-08` custom-avatar companion preview over `AV-A1-02`
  - `PE2E-09` unsupported-runtime visible fallback companion preview over `AV-A1-03`
- Added preview manifests and exact expected bundles for `PE2E-08/09`.
- Expanded unit tests so `PE2E-08/09` are checked for exact avatar report consumption, rejected-source fallback retention, and `deferred_avatar_semantics = []`.
- Updated `specs/18`, `plan/44`, `plan/01`, `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample READMEs, and `scripts/README.md` so the widened product-preview floor and its non-claims are explicit.
- Re-ran the focused practical alpha-1 Python floor and the repo guardrails after snapshot/report updates.

## Files changed

- `scripts/practical_alpha1_product_preview.py`
- `scripts/tests/test_practical_alpha1_product_preview.py`
- `samples/practical-alpha1/previews/pe2e-a1-08-custom-avatar-companion-preview/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-09-unsupported-runtime-visible-fallback-preview/preview.json`
- `samples/practical-alpha1/expected/pe2e-a1-08-custom-avatar-companion-preview.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-09-unsupported-runtime-visible-fallback-preview.expected.json`
- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/previews/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1169-p-a1-11-product-preview-avatar-companion-widening.md`

## Commands run

- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_validate_docs`
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py closeout --format json`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
- `python3 scripts/practical_alpha1_avatar.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `cargo fmt --check`
- `git diff --check`
- `git status --short`

## Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M %Z'` returned `2026-05-04 14:33 JST` and was used for snapshot timestamps.
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_validate_docs` passed `26` tests.
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json` passed `9/9` preview rows with:
  - `actualized_rows = [PE2E-01..09]`
  - `product_preview_first_floor_complete: true`
  - `stage_pa1_8_complete: false`
  - `deferred_avatar_semantics: []`
  - `viewer_html_available: true`
- `python3 scripts/practical_alpha1_product_preview.py closeout --format json` reported:
  - `implemented_rows = [PE2E-01..09]`
  - `stop_lines` explicitly forbid native execution, same-session runtime completion, unsupported-runtime execution success, and final public surface claims
  - `limitations` explicitly keep the carrier split thin over preview manifests, exact reports, exact devtools bundles, and exact avatar preview reports
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` passed `RUN-01/02`, with `local_runtime_first_floor_complete: true`.
- `python3 scripts/practical_alpha1_avatar.py check-all --format json` passed `AV-A1-01/02/03`, with `native_execution_claimed: false` and `final_avatar_abi_frozen: false`.
- `python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json` returned a non-final static HTML bundle over exact devtools export payloads, confirming `viewer_html_available: true`.
- `python3 scripts/check_source_hierarchy.py` passed with `73/73` required paths present.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1170 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` passed `11` tests after the new report was added.
- `cargo fmt --check` and `git diff --check` were clean on the closeout tree.

## What changed in understanding

- Practical product-preview widening can honestly consume avatar semantics only through exact `AV-A1-*` reports, not by reinterpreting companion preview evidence as runtime execution.
- `PE2E-09` is safe only if the source avatar lane remains rejected and the fallback stays visible and monotone in the widened bundle.
- After `P-A1-11`, the natural reopen point shifts away from avatar-consumption design and toward remaining exact-evidence widenings such as `VIS-A1-03/05/07` or broader save/load work.

## Open questions

- Which of `VIS-A1-03/05/07` is the narrowest honest next widening over already-actualized exact practical carriers.
- Whether broader save/load widening can be promoted before any additional devtools widening without overclaiming stale witness / stale lease / distributed durable save/load semantics.
- Whether any later product-preview widening can be justified before same-session runtime semantics themselves are actualized.

## Suggested next prompt

Promote the narrowest remaining exact-evidence widening for practical alpha-1, preferring a `VIS-A1-*` package if one can be closed over already-actualized practical carriers without synthetic overclaim.

## Plan update status

`plan/` 更新済み:

- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み:

- practical alpha-1 summary now names `P-A1-11` as the latest closeout and widens the product-preview row set to `PE2E-01..09`.

## progress.md update status

`progress.md` 更新済み:

- practical stage map now advances `PA1-8` to `80%`, records `P-A1-11` as the latest closeout, and adds the `2026-05-04 14:33 JST` recent log entry.

## tasks.md update status

`tasks.md` 更新済み:

- current task map now records `P-A1-11` as closed, shifts the next reopen point to remaining devtools/save-load widening, and rewrites the avatar compatibility blocker as a later runtime-semantics question.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- practical sample dashboard now advances `PA1-8` to `80%`, widens `PE2E-01..09`, adds the `P-A1-11` validation row, and updates the repo-memory row to include report `1169`.

## Reviewer findings and follow-up

- No sub-agents were used in this task.
- Follow-up was local focused review only, because current tool policy for this turn did not authorize delegation without explicit user request.
- Local review focus:
  - carrier separation between avatar preview report and widened product-preview bundle
  - exact retention of rejected-source fallback semantics for `PE2E-09`
  - non-claim wording around native execution / same-session runtime attach/detach execution / unsupported-runtime execution success
  - snapshot and dashboard consistency after widening

## Skipped validations and reasons

- Did not rerun Rust/Cargo behavior tests beyond `cargo fmt --check`, because `P-A1-11` touched only Python helper logic, expected JSON, preview manifests, and documentation; no Rust source changed in this package.
- Did not rerun broader practical transport/save-load/devtools Cargo floors, because the widened bundle consumes already-actualized exact source carriers and the focused `run_local` / `avatar` / `product_preview` script floors were rerun directly.
- Did not use sub-agent review, because the current tool policy for this turn did not authorize delegation without an explicit user request.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent sessions opened in this task.
