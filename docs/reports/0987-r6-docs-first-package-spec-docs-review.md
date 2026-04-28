# Report 0987 — R6 docs-first package spec/docs review

- Date: 2026-04-28 23:53:40 JST
- Author / agent: Codex
- Scope: current uncommitted R6 docs-first package の spec/docs correctness review（code semantics review ではない）
- Decision levels touched: none（review only）

## 1. Objective

未コミットの R6 docs-first package について、仕様・文書の整合性だけを review する。焦点は以下に置く。

- `R6` は engine actualization ではなく queue/admission cut か
- first admissible Rust-side hot-plug-specific family は engine-neutral request/verdict carrier のみか
- helper-local `hotplug_lifecycle` / sample-grounded IDs / attach-detach view-telemetry IDs は preview ownership のままか
- promoted next は `P19`、reopen next は `P20` か
- stale active reference / overclaim が残っていないか

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples_progress.md`
- `tasks.md`
- `git diff` / `git status --short`

## 3. Actions taken

- repository front-door / normative docs を AGENTS.md 指示順に読んだ
- uncommitted diff から code file 2 件を除外して docs-only review 対象を確認した
- `R6` / `P19` / `P20` / `request / verdict` / `engine actualization` / `0986` を横断 grep した
- changed / new docs の relevant section を line-number 付きで確認した
- report trail と referenced evidence file 実在性を `docs/reports/` で確認した

## 4. Files changed

- `docs/reports/0987-r6-docs-first-package-spec-docs-review.md` を新規追加
- `plan/` 更新不要
- `progress.md` 更新不要（review task のため修正は未実施）
- `samples_progress.md` 更新不要（review task のため修正は未実施）
- `tasks.md` 更新不要（review task のため修正は未実施）

## 5. Commands run and exact outputs

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ git status --short
 M Documentation.md
 M README.md
 M crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs
 M crates/mir-ast/src/current_l2.rs
 M docs/hands_on/README.md
 M docs/hands_on/current_phase_closeout_01.md
 M docs/research_abstract/README.md
 M docs/research_abstract/mirrorea_future_axis_01.md
 M plan/00-index.md
 M plan/01-status-at-a-glance.md
 M plan/11-roadmap-near-term.md
 M plan/21-hotplug-attachpoint-roadmap.md
 M plan/33-runtime-crate-hotplug-engine-ownership-cut.md
 M plan/90-source-traceability.md
 M progress.md
 M samples_progress.md
 M specs/00-document-map.md
 M specs/10-open-questions.md
 M specs/11-roadmap-and-workstreams.md
 M tasks.md
?? docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md
?? docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md
?? plan/34-runtime-crate-hotplug-carrier-admission-cut.md

$ ls docs/reports | tail -n 20
0967-p13-helper-local-process-boundary-closeout.md
0968-p14-hotplug-package-surface-map.md
0969-p14-hotplug-package-manager-first-cut-closeout.md
0970-p15-projection-codegen-generated-bridge-first-cut-closeout.md
0971-p16-visual-debugger-viewer-first-public-prototype-closeout.md
0972-p17-storage-llvm-backend-preparation-first-cut-closeout.md
0973-p18-public-api-parser-gate-first-cut-closeout.md
0974-u1-post-p18-true-user-spec-hold-option-matrix-closeout.md
0975-r1-verification-layer-widening-threshold-inventory-first-cut.md
0976-r1-verification-threshold-closeout-and-r2-promotion.md
0977-r2-attachpoint-detach-minimal-contract-closeout.md
0978-r3-fairy05-visibility-return-carrier-bundling-closeout.md
0979-r4-hotplug-real-migration-rollback-boundary-closeout.md
0980-r4-docs-first-package-review.md
0981-r4-followup-narrow-rereview-no-findings.md
0982-r5-runtime-crate-hotplug-engine-ownership-cut-closeout.md
0983-post-r5-next-package-decomposition-recommendation.md
0984-r5-docs-first-package-review.md
0985-r5-followup-local-rereview.md
TEMPLATE.md

$ ls docs/reports/0986*
ls: cannot access 'docs/reports/0986*': No such file or directory
```

## 6. Evidence / findings

1. `R6` closeout evidence trail is broken.
   `plan/01-status-at-a-glance.md`、`plan/11-roadmap-near-term.md`、`plan/90-source-traceability.md`、`samples_progress.md` が `docs/reports/0986-*` を R6 closeout evidence として参照しているが、`docs/reports/` に `0986` report が存在しない。R6 closeout を source-backed に読めるという package claim が証跡上成立していない。
2. `docs/research_abstract/README.md` still points the promoted next line at `R6`.
   当該 front-door summary は `R6` を close 済み memory ではなく current promoted next line と書いており、`P19` promoted next / `P20` reopen next へ recut された current queue と衝突する。reader-facing entrypoint として stale active reference が残っている。
3. `progress.md` recent log does not record the R6 closeout it claims in the snapshot.
   文書冒頭の snapshot は R6 close と `P19` / `P20` split を current repo に mirror 済みと書くが、recent log 最新行は `R5` follow-up と `R5` closeoutのままで、R6 closeout の日時付き entry がない。snapshot と trail の同期が取れていない。

## 7. Changes in understanding

- `R6` の substantive wording自体は大半の changed docs で揃っている
- 主な欠陥は semantics overclaim というより snapshot / evidence / front-door queue pointer の drift にある

## 8. Open questions

- `docs/reports/0986-*` は未作成なのか、別 identifier で誤参照しているのか
- `docs/research_abstract/README.md` の next-line drift を `P19` / `P20` へ直すだけで十分か、同時に `R6` closeout memory 参照列も追記するか
- `progress.md` recent log には `R6` closeout 1 行を追記すべきか、`R5` follow-up row を圧縮再編して trail を並べ替えるべきか

## 9. Suggested next prompt

Fix the R6 docs drift by adding the missing closeout report/evidence reference, updating `docs/research_abstract/README.md` to `P19` promoted next and `P20` reopen next, and syncing `progress.md` recent log with the actual R6 closeout trail.
