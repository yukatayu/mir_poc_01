# Report 0683 — Phase 6 final public parser / checker / runtime first later gate actualization comparison package

- Date: 2026-04-13T18:51:00Z
- Author / agent: Codex
- Scope: stable malformed missing-option reopen comparison close 後の次段として、library-side final public parser / checker / runtime first later gate の symbol-level first cut を docs-only で固定し、repo-level current line を malformed-side source-backed widening へ進める。
- Decision levels touched: L2

## 1. Objective

- final public parser / checker / runtime first later gate の current first actualization cut を narrow に固定する。
- `run_current_l2_source_sample` current gate と library-before-CLI later ordering を崩さず、support-only tranche と excluded bucket を分ける。
- repo-level current line を stable malformed missing-option source-backed widening へ handoff する。

## 2. Scope and assumptions

- current package は docs-only actualization comparison / threshold close に留める。
- runtime-led thin library facade を first later cut に置き、standalone parser/checker support entry や public operational CLI actual shape は still later に残す。
- `run_current_l2_source_sample` の public/report shape は変えない。

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
- `specs/examples/355...356`
- `specs/examples/363...364`
- `specs/examples/371...372`
- `specs/examples/385...386`
- `specs/examples/387...388`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
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
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-ast/src/current_l2.rs`

## 4. Actions taken

- `specs/examples/389...390` を追加し、library-side final public parser / checker / runtime first later gate の current first choice を runtime-led thin library facade に固定した。
- public entry は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` に置き、parser/checker/runtime detail は nested report carrier に留めると整理した。
- `run_current_l2_runtime_skeleton`、`lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floor は support-only bucket、`resolve_current_l2_source_sample_path` と accepted-set hard-coding、repo-local helper / example surface は excluded bucket に残すと整理した。
- `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、Phase 6 abstract、FAQ、sample docs、`.docs` を current snapshot に同期し、repo-level current line を `stable malformed missing-option first source-backed widening actualization` に進めた。
- `plan/08-representative-programs-and-fixtures.md` は sample corpus そのものを変えないため、この package では更新不要と判断した。

## 5. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0683-phase6-final-public-parser-checker-runtime-first-later-gate-actualization-comparison-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/389-current-l2-stable-malformed-missing-option-first-reopen-actualization-ready-final-public-parser-checker-runtime-first-later-gate-actualization-comparison.md`
- `specs/examples/390-current-l2-final-public-parser-checker-runtime-first-later-gate-actualization-ready-minimal-final-public-parser-checker-runtime-first-later-gate-threshold.md`
- `tasks.md`

## 6. Evidence / outputs / test results

- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `10 passed`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `current L2 fixed-subset authored inventory` から始まる authored octet table を表示し、`e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` がすべて `present` で一致した。
- `date -u '+%Y-%m-%dT%H:%M:%SZ'`
  - `2026-04-13T18:51:00Z`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-13 18:51 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 682 numbered report(s).`
- `git diff --check`
  - 出力なし

## 7. What changed in understanding

- final public parser / checker / runtime later gate の first cut は、parser/checker/runtime を 3 本の standalone public entry に分けるより、`run_current_l2_source_sample` entry と nested report carrier を中心にした runtime-led thin facade の方が current guards と整合する。
- `pub` visibility がある symbol でも、path resolver、accepted-set hard-coding、repo-local helper は final public contract 候補に混ぜない方が drift が少ない。
- malformed-side widening reserve と CLI second gate を kept-later refs に並べることで、Macro 4 と Macro 7 の近接 line を同時に見失わずに済む。

## 8. Open questions

- runtime-led thin facade の次段で standalone parser/checker support entry を later public support として切る必要があるか。
- explicit `FixtureHostPlan` coupling を library-side final contract からどう薄くするか。
- public operational CLI second gate を runtime-led thin facade とどう分けて actualize するか。

## 9. Suggested next prompt

- `tasks.md` の current line どおり、stable malformed missing-option first source-backed widening actualization を進めてください。helper-local compare を entry evidence に再利用しつつ、`e16/e18` first cut と `e17` staged guard を current source sample / runner / ladder / regression line にどう落とすかを fixed してください。
