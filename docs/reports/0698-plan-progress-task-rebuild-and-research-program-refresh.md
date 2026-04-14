# 0698 — plan / progress / task rebuild and research program refresh

## Objective

- `plan/`、`progress.md`、`tasks.md` を current snapshot に合わせて再構成する。
- snapshot 文書を薄くし、detail-side の研究計画を `plan/` へ寄せる。
- 型 / 定理証明 / モデル検査 / ordering の見通しを、mainline と分離した detailed research program として明文化する。

## Scope and assumptions

- 規範判断の正本は `specs/` に残す。
- current promoted line は `Macro 4 / stable malformed capability second source-backed widening actualization` のまま維持する。
- current reserve line は `Macro 7 / public operational CLI concrete shell actualization` のまま維持する。
- external `answer_001...016` は参考意見として扱い、repo memory への反映は既存 spec / plan / report chain と整合する範囲に限る。
- 開始時点の workspace は inherited summary 上 clean と読め、今回 task では current edits のみを追加した。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/91-maintenance-rules.md`
- `.docs/progress-task-axes.md`
- `faq_003.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0697-external-answer-integration-audit-and-plan-hardening.md`

## Actions taken

1. current snapshot 文書の役割を再整理し、`progress.md` と `tasks.md` から長い fixed-chain 履歴を外した。
2. `plan/00`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/17` を薄い snapshot / roadmap / risk / autonomy 文書として再構成した。
3. `plan/18-type-proof-modelcheck-and-ordering-research-program.md` を新設し、typed-core / theorem / model-check / ordering の detailed research program を分離した。
4. `Documentation.md` に `plan/18` への導線を追加し、`faq_003.md` を current reading に合わせて更新した。
5. `plan/90-source-traceability.md` に `plan/18` と今回の rebuild addendum を追加した。
6. reviewer の指摘を受け、`tasks.md` の thin snapshot 化をさらに進め、Macro 6/7 の自走可否、`plan/18` traceability、FAQ / Documentation wording drift を修正した。
7. narrow re-review と最終 validation を通し、rebuild 後の doc-consistency drift が残っていないことを確認した。

## Files changed

- `Documentation.md`
- `faq_003.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0698-plan-progress-task-rebuild-and-research-program-refresh.md`

## Commands run

```bash
df -h .
free -h
sed -n '1,240p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,260p' plan/01-status-at-a-glance.md
sed -n '1,260p' plan/10-roadmap-overall.md
sed -n '1,260p' plan/11-roadmap-near-term.md
sed -n '1,260p' plan/17-research-phases-and-autonomy-gates.md
sed -n '1,260p' plan/00-index.md
sed -n '1,220p' Documentation.md
sed -n '1,260p' plan/12-open-problems-and-risks.md
sed -n '1,260p' plan/13-heavy-future-workstreams.md
sed -n '1,260p' plan/16-shared-space-membership-and-example-boundary.md
sed -n '1,260p' faq_003.md
date '+%Y-%m-%d %H:%M %Z'
rg -n "plan/1[078]|progress.md|tasks.md|macro phase|current line|public operational CLI concrete shell actualization|stable malformed capability" Documentation.md plan/01-status-at-a-glance.md plan/10-roadmap-overall.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md faq_003.md
sed -n '1,220p' plan/91-maintenance-rules.md
sed -n '1,260p' plan/90-source-traceability.md
sed -n '1,180p' specs/00-document-map.md
git diff --check
python3 scripts/validate_docs.py
git status --short
nl -ba Documentation.md | sed -n '92,102p'
sed -n '1,220p' progress.md
sed -n '1,220p' tasks.md
sed -n '1,220p' plan/18-type-proof-modelcheck-and-ordering-research-program.md
sed -n '1,220p' plan/90-source-traceability.md
```

## Evidence / outputs / test results

- `df -h .`
  - `/dev/vda2` 残り `19G`
- `free -h`
  - memory は逼迫気味だが swap に余裕あり
- `git diff --check`
  - 無出力
- reviewer
  - `Documentation.md` / `progress.md` / `tasks.md` / `faq_003.md` / `plan/90-source-traceability.md` の drift 5 件を指摘
- re-review 対応
  - `tasks.md` をさらに薄くし、Macro 6/7 の boundary-only 自走可否と traceability / wording を修正した
- narrow re-review
  - `No findings.`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 697 numbered report(s).`
- `git status --short`
  - current editsは本 task の docs update のみ

## What changed in understanding

- current repo の問題は「計画が無い」ことではなく、snapshot 文書へ detail が戻りすぎていたことだと再確認した。
- typed / theorem / model-check line は full theory には早いが、boundary と pilot plan は進められる段階にある。
- ordering / `memory_order` 再解釈 line も implementation には早いが、theory-first inventory は進められる段階にある。
- shared-space と host-facing I/O は docs-first boundary を保ったまま reserve hardening を進めるのが自然である。

## Open questions

- typed first attachment candidate を semantic carrier / checker boundary / source-visible surface のどこへ置くか。
- theorem pilot の first lemma family をどこまで narrow に切るか。
- model-check の first property family を row-local / small-cluster のどちらへ置くか。
- ordering line の authority-serial / witness-aware handoff split をどこまで detail-side で固定するか。

## Suggested next prompt

- `tasks.md` 先頭の 2 package を閉じたあと、`plan/18` 先頭の typed-core / theorem / model-check reserve planning package を順に進めてください。
