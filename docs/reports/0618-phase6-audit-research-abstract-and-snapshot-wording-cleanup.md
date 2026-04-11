# Report 0618 — phase6 audit research abstract and snapshot wording cleanup

- Date: 2026-04-11T17:25:34.770787Z
- Author / agent: Codex
- Scope: current promoted line / historical handoff wordingの混線を research abstract 群で監査し、stale wording を cleanup する。
- Decision levels touched: docs mirror / research abstract wording hygiene

## 1. Objective

- `docs/research_abstract/` に残っている stale な current-line wording を洗い出す。
- phase close 時点の handoff と current snapshot を区別し、snapshot 文書との衝突をなくす。
- current promoted line / reserve path / historical handoff の使い分けを repo 全体で揃える。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0617-phase6-reserve-formal-tool-binding-inventory-package.md`

## 3. Actions taken

1. `docs/research_abstract/` を `current mainline` / `current promoted line` / `next line` で横断し、historical handoff と current snapshot が混ざっている箇所を抽出した。
2. Phase 2 / Phase 4 abstract では、dynamic な `current mainline` 表現を closeout 時点の handoff 表現へ置き換えた。
3. Phase 5 abstract では、Phase 6 側の進捗を `specs/examples/305...310` まで反映し、historical handoff と current mainline を分離した。
4. reviewer の指摘に基づき、`plan/01`、`plan/10`、`plan/11`、`plan/17`、`docs/research_abstract/phase6...` の near-term order を `sequencing -> actualization -> fixed-subset sample/program corpus staging` に揃えた。
5. `plan/90` の Task 3 addendum は、今回 package の主根拠だけに絞り、parser-first-tranche 由来の過剰 trace を削った。
6. `progress.md` / `tasks.md` は Task 3 時点で current snapshot に整合していたため、この audit では **progress.md 更新不要**、**tasks.md 更新不要** と判断した。

## 4. Files changed

- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0618-phase6-audit-research-abstract-and-snapshot-wording-cleanup.md`

## 5. Commands run and exact outputs

```bash
python3 scripts/new_report.py --slug phase6-audit-research-abstract-and-snapshot-wording-cleanup
rg -n "current mainline|current promoted line|next line|next promoted line" docs/research_abstract
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## 6. Evidence / findings

- stale wording は `docs/research_abstract/phase2...` と `phase4...` の handoff paragraph、`phase5...` の current line paragraphに集中していた。
- reviewer 監査で、snapshot / plan 側にも `sequencing -> actualization` を飛ばして sample/program corpus staging を second line に昇格させている drift が見つかったため、その order を tasks/progress/spec に合わせて修正した。
- `plan/90` addendum には Task 3 report と対応しない parser-first-tranche roots が混入しており、今回 package の主根拠だけに絞る必要があった。

## 7. Changes in understanding

- phase abstract では、closeout 時点の handoff と repo の current snapshot を混ぜない方が再読しやすい。
- `progress.md` / `tasks.md` が snapshot 正本で、`docs/research_abstract/` は condensed summary であるという役割分担を wording でも守る必要がある。

## 8. Open questions

- fixed-subset sample/program corpus staging をどの abstract / plan に first-class に載せるか
- parser-side follow-up package sequencing 後に actual sample corpus line をどこで reopen するか

## 9. Suggested next prompt

```text
Phase 6 parser-side follow-up package sequencing を進め、shared single attachment frame を current package に含めるかを narrow に決めてください。
```
