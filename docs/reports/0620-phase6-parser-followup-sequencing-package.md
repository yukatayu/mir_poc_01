# Report 0620 — phase6 parser followup sequencing package

- Date: 2026-04-12T01:53:36.327636Z
- Author / agent: Codex
- Scope: Phase 6 parser-side follow-up package sequencing fixed、snapshot / roadmap / traceability mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

- `tasks.md` 先頭 task だった Phase 6 parser-side follow-up package sequencing を source-backed comparison / threshold として閉じる。
- shared single attachment frame を next package に置くか、request clause suite / perform head / source-sample path とどう切り分けるかを narrow に固定する。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、research abstract、traceability mirror を current promoted line に揃える。

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
- `specs/examples/95-current-l2-stage3-fragment-vs-attachment-next-step-sequencing.md`
- `specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
- `specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`
- `specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
- `specs/examples/310-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-minimal-phase6-reserve-formal-tool-binding-inventory-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`

## 3. Actions taken

- disk / memory 状況を再確認し、current snapshot と parser-side code/test evidence を読み直した。
- explorer subagent 3 本で
  - sequencing judgment の候補比較
  - shared multiline extraction helper の actual cut
  - source-sample corpus layout の current repo 状況
  を並列に調べ、shared single attachment frame が next narrow package として最も自然であることを確認した。
- `specs/examples/311...312` を追加し、parser-side follow-up package sequencing を
  - fixed entry criteria
  - selected next package
  - deferred reopen line
  - guard line
  を持つ docs-first threshold として固定した。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、Phase 6 abstract、`specs/00-document-map.md` を更新し、current promoted line を `Phase 6 parser-side follow-up package actualization` に切り替えた。

## 4. Files changed

- `Documentation.md`
- `docs/reports/0620-phase6-parser-followup-sequencing-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
- `specs/examples/312-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-minimal-phase6-parser-side-follow-up-package-sequencing-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `df -h .`
  - `Filesystem Size Used Avail Use% Mounted on`
  - `/dev/vda2 99G 76G 19G 80% /`
- `free -h`
  - `Mem: 960Mi total / 563Mi used / 107Mi free / 396Mi available`
- `python3 scripts/new_report.py --slug phase6-parser-followup-sequencing-package`
  - `docs/reports/0620-phase6-parser-followup-sequencing-package.md`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 10:54 JST`

## 6. Evidence / findings

- current repo の actual parser-side stop lineは `attached-slot + minimal predicate fragment` までであり、shared single attachment frame は helper-local / test-only multiline extraction bridge として既に actual evidence がある。
- request clause suite は shared single attachment frame より 1 段深い layer であり、current sequencing line では同一 package に混ぜる必要がない。
- source-sample path へ進む前に shared single attachment frame を next package として固定しておく方が、later lowering / runner / verification ladder への逆流を抑えやすい。
- `plan/90` traceability も、今回の snapshot 更新に対して new example / report を根拠として追加したため **更新済み** とした。

## 7. Changes in understanding

- parser-side follow-up sequencing の本質は「shared single attachment frame を actualize するかどうか」ではなく、「request clause suite / perform head / source-sample path と混線させず、next package を 1 本だけ選ぶこと」にある。
- source-sample path は current mainline だが、parser-side follow-up actualization を 1 本挟んでから入る方が task boundary と guard が明確になる。

## 8. Open questions

- shared single attachment frame actualization の current code anchor を `mir_ast::current_l2` へどこまで昇格するか
- request clause suite helper 側の duplicated extraction logic をどの timing で寄せるか
- source-sample corpus scope / file layout を repo root / crate-local / docs-local のどれに置くか

## 9. Suggested next prompt

- `tasks.md` の先頭 task である `Phase 6 parser-side follow-up package actualization` を、TDD と focused cargo test を通して source-backed に閉じてください。その後、`fixed-subset source-sample corpus scope / file layout` を docs-first に固定し、representative prose / fixture corpus / source sample の 3 層を layout と ID policy まで整理してください。
