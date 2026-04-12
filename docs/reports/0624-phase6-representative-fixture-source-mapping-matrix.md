# Report 0624 — phase6 representative fixture source mapping matrix

- Date: 2026-04-12T03:10:22.461395Z
- Author / agent: Codex
- Scope: Phase 6 representative / fixture / source mapping matrix package、source corpus matrix mirror actualization、snapshot / roadmap / traceability 更新
- Decision levels touched: L1 / L2

## 1. Objective

- `tasks.md` 先頭 task だった Phase 6 representative / fixture / source mapping matrix を source-backed に閉じる。
- initial cluster `e1` / `e2` / `e3` / `e4` / `e21` / `e23` を representative prose / fixture corpus / source target path の 3 層 row へ束ねる。
- `e3` variant と `e23` unresolved representative anchor を silent repair せず mirror し、current mainline を lowering first cut へ進める。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
- `specs/examples/316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json`
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`

## 3. Actions taken

- `specs/examples/317...318` を追加し、representative / fixture / source mapping matrix の comparison / threshold を fixed した。
- `samples/current-l2/README.md` に current mapping matrix table を追記し、initial cluster 6 本の representative anchor / fixture id / source target / expected verdict を repo-tracked path に mirror した。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/08`、`plan/10`、`plan/11`、`plan/12`、`plan/15`、`plan/17`、`plan/90`、Phase 6 abstract、`specs/00-document-map.md` を更新し、current mainline を `actual parser-to-Program lowering first cut` に切り替えた。
- `e3` は fixture-side `source_example_id = E3-variant` を direct に mirror し、`e23` は representative prose row を invent せず unresolved anchor として残した。

## 4. Files changed

- `Documentation.md`
- `docs/reports/0624-phase6-representative-fixture-source-mapping-matrix.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/317-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-representative-fixture-source-mapping-matrix-comparison.md`
- `specs/examples/318-current-l2-representative-fixture-source-mapping-matrix-ready-minimal-representative-fixture-source-mapping-matrix-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `git status --short`
  - `?? docs/reports/0624-phase6-representative-fixture-source-mapping-matrix.md`
- `find specs/examples -maxdepth 1 -type f | sed 's#specs/examples/##' | sort -V | tail -n 20`
  - `...`
  - `315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
  - `316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 12:17 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 623 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- current initial cluster を 1 表に束ねるとき、minimum row には representative layer と fixture layer と source target layer を分けて持たせる方が自然である。
- `e3-option-admit-chain` は plain `E3` ではなく fixture-side `source_example_id = E3-variant` を mirror しないと、admit-carrying variant であることが失われる。
- `e23-malformed-try-fallback-missing-fallback-body` は fixture-side `source_example_id = E23` を already 持つが、current representative prose row は無い。したがって representative anchor を invent せず `unresolved` に留める方が fail-closed である。
- current matrix task は source target path ref までで閉じられ、actual sample file content、reached stage、bless policyをまだ要求しない。

## 7. Changes in understanding

- mapping matrix task の本質は source file を増やすことではなく、3 層の identity drift を先に止めることである。
- `source_example_id` と representative prose anchor は同一 field に潰さない方が later drift を抑えやすい。
- `e23` のような unresolved representative anchor を current row に残せるようになったため、later prose actualization を still later に押し戻しても source-sample path は進められる。

## 8. Open questions

- lowering first cut で actual sample file content をどの cluster から authoring するか
- `e23` representative anchor を later prose row に actualize するか
- reached stage inventory を Task 3 でどの shape にするか

## 9. Suggested next prompt

- `tasks.md` 先頭 task の `actual parser-to-Program lowering first cut` を進めてください。fixed subset source sample を semantic `Program` と parser bridge evidence へ fail-closed に落とし、initial cluster のうち static-only / runtime fixture の最小 pair を actual source text から通してください。
