# Report 0617 — phase6 reserve formal tool binding inventory package

- Date: 2026-04-11T17:14:07.671919Z
- Author / agent: Codex
- Scope: Phase 6 parser second tranche first package fixed 後の reserve formal tool binding inventory を docs-first に固定し、current promoted line を parser-side follow-up package sequencing へ返す。
- Decision levels touched: current L2 / L2 docs-first reserve path inventory

## 1. Objective

- theorem-first / model-check concrete tool binding を current mainline へ戻さず、reserve path として narrow に整理する。
- tool-neutral formal hook を current entry criteria に維持したまま、formal-side next reopen friction を減らす。
- parser-side follow-up package sequencing を current promoted line に切り替える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md`
- `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
- `specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`
- `specs/examples/297-current-l2-phase4-shared-space-self-driven-closeout-ready-phase5-proof-protocol-runtime-policy-handoff-closeout-comparison.md`
- `specs/examples/298-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-minimal-phase5-proof-protocol-runtime-policy-handoff-closeout-threshold.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
- `specs/examples/306-current-l2-phase6-next-reopen-sequencing-ready-minimal-phase6-next-reopen-sequencing-threshold.md`
- `specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
- `specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`

## 3. Actions taken

1. Phase 5 closeout / Phase 6 formal hook / next reopen sequencing / parser second tranche first package の current fixed line を再確認し、formal-side current task は actual binding ではなく reserve inventory wording で十分かを整理した。
2. theorem-side consumer pressure / notebook pressure ladder と model-check side inventory の source-backed strengthを比較し、theorem-first を first reserve、model-check を second reserve に置くのが current minimum だと判断した。
3. `specs/examples/309...310` を追加し、reserve formal tool binding inventory の comparison / threshold を docs-first に固定した。
4. `Documentation.md`、`progress.md`、`tasks.md`、`specs/00-document-map.md`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`docs/research_abstract/phase6...` を current promoted line に合わせて更新した。

## 4. Files changed

- `specs/examples/309-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-phase6-reserve-formal-tool-binding-inventory-comparison.md`
- `specs/examples/310-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-minimal-phase6-reserve-formal-tool-binding-inventory-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0617-phase6-reserve-formal-tool-binding-inventory-package.md`

## 5. Commands run and exact outputs

```bash
python3 scripts/new_report.py --slug phase6-reserve-formal-tool-binding-inventory-package
date '+%Y-%m-%d %H:%M JST'
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## 6. Evidence / findings

- theorem-side は Phase 5 の existing consumer pressure / notebook pressure ladder があり、reserve wording を source-backed に組みやすい。
- model-check side は boundary inventory と checker cut boundary までは強いが、concrete tool first cut は theorem-side より薄い。
- current line では tool-neutral formal hook を entry criteria に維持し、parser-side follow-up sequencing を mainline に戻すのが最小である。

## 7. Changes in understanding

- formal-side の immediate task は concrete tool choice そのものではなく、reserve inventory priority を narrow に mirror することだった。
- theorem-first を first reserve に置くことで、model-check side を消さずに current mainline と分離できる。
- parser-side follow-up package sequencing と formal reserve inventory は separate package にした方が drift suppression しやすい。

## 8. Open questions

- theorem-first reserve line を notebook pressure 以降のどこまで snapshot に残すか
- parser-side follow-up package で shared single attachment frame を immediate line に含めるか
- fixed-subset sample/program corpus をどの phase gate で current mainline に接続するか

## 9. Suggested next prompt

```text
Phase 6 parser-side follow-up package sequencing を進め、shared single attachment frame を current package に含めるかを narrow に決めてください。
```
