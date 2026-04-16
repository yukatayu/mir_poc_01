# plan/00 — plan インデックス

## 目的

`plan/` は、この repo の長期参照用 repository memory である。
`specs/` の規範判断、`docs/reports/` の経緯、code anchor の現物を横断しつつ、

- 現況
- 計画
- 責務境界
- 未決事項
- 自走順序

を人間が再読しやすい形へ整理する。

重要なのは次の 2 点である。

- 規範判断の正本は常に `specs/` に残る。
- `plan/` は snapshot 文書ではなく、detail-side の repository memory として保つ。

## 先に読む順序

1. `plan/00-index.md`
2. `plan/01-status-at-a-glance.md`
3. `plan/02-system-overview-and-positioning.md`
4. `plan/03-decision-strengths-and-boundaries.md`
5. current L2 / PoC を見るなら `plan/04` から `plan/09`
6. 全体計画を見るなら `plan/10-roadmap-overall.md`
7. 直近の順序を見るなら `plan/11-roadmap-near-term.md`
8. risk / heavy line / detailed research program を見るなら `plan/12`、`plan/13`、`plan/18`
9. shared-space boundary を見るなら `plan/16-shared-space-membership-and-example-boundary.md`
10. phase / autonomy gate を見るなら `plan/17-research-phases-and-autonomy-gates.md`
11. 用語 / traceability / maintenance を見るなら `plan/14`、`plan/90`、`plan/91`

## status legend

| ラベル | 意味 |
|---|---|
| `L0/L1 settled` | repo 全体を拘束する強い判断 |
| `current L2 settled` | current L2 では固定しているが、将来再比較余地はある |
| `current parser-free PoC reading` | parser-free / helper-local の最小読み |
| `OPEN` | 未決。仮説を事実化しない |
| `CURRENT RESEARCH PROGRAM` | 今まさに detail-side で進める研究線 |
| `HEAVY FUTURE WORKSTREAM` | まだ mainline に昇格させない重い将来線 |
| `HISTORICAL / COMPARISON` | 比較対象、履歴、採らなかった案 |

## current repo の短い要約

- 主眼は **Mir** の意味論基盤である。
- current repo は architecture-first だが、parser-free PoC、compile-ready minimal actualization、fixed-subset source sample authored dozen を already 持つ。
- 近接運用は 3 lane で読むのが自然である。
  - execution lane: `Macro 4 / malformed duplicate-cluster source-sample widening comparison with try-rollback malformed-static kept-later inventory`
  - theory-lab lane: `Macro 5` typed / theorem / model-check reserve follow-up と `Macro 5/6` order / memory / syntax / modality reserve follow-up
  - reserve integration lane: `Macro 6/7 / public operational CLI packaging reserve note と shared-space fairness/replay strengthening reserve note`
- 型 / 定理証明 / モデル検査 / ordering は mainline に無理に混ぜず、detail-side research program として扱う。
- shared-space と host-facing I/O は docs-first boundary までは自走可能だが、final operational catalog や concrete target binding は later / mixed gate に残す。

## 各ファイルの役割

| ファイル | 役割 |
|---|---|
| `plan/01-status-at-a-glance.md` | 1〜2 画面で把握する current status |
| `plan/02-system-overview-and-positioning.md` | Mir / Mirrorea / Prism / Typed-Effect の大局関係 |
| `plan/03-decision-strengths-and-boundaries.md` | L0〜L3 と current L2 / future の見分け方 |
| `plan/04-core-semantics-current-l2.md` | Mir current L2 核心 |
| `plan/05-fallback-lease-and-chain-semantics.md` | fallback / `lease` / monotone degradation |
| `plan/06-surface-notation-status.md` | companion notation と syntax 候補 |
| `plan/07-parser-free-poc-stack.md` | parser-free PoC stack の責務境界 |
| `plan/08-representative-programs-and-fixtures.md` | representative programs / fixtures の catalog |
| `plan/09-helper-stack-and-responsibility-map.md` | Rust / Python helper stack と public/shell/support 境界 |
| `plan/10-roadmap-overall.md` | macro roadmap と research split |
| `plan/11-roadmap-near-term.md` | 近接 package の順序と completion signal |
| `plan/12-open-problems-and-risks.md` | compact risk register |
| `plan/13-heavy-future-workstreams.md` | current mainline に上げない heavy workstream |
| `plan/14-glossary-and-boundary-rules.md` | 用語と混同防止 |
| `plan/15-current-l2-fixture-authoring-template.md` | fixture authoring 実務テンプレート |
| `plan/16-shared-space-membership-and-example-boundary.md` | shared-space / membership / example boundary |
| `plan/17-research-phases-and-autonomy-gates.md` | macro phase と autonomy gate |
| `plan/18-type-proof-modelcheck-and-ordering-research-program.md` | 型 / 定理証明 / モデル検査 / ordering / syntax / modality の detailed research program |
| `plan/90-source-traceability.md` | 各 plan の主要根拠 source |
| `plan/91-maintenance-rules.md` | `plan/` の維持規則 |

## `plan/` と他文書の関係

- `specs/`
  - 規範判断の正本
- `docs/reports/`
  - 作業証跡の正本
- `progress.md`
  - repo 全体の薄い進捗 snapshot
- `tasks.md`
  - current task map の薄い snapshot
- `plan/`
  - それらを横断して detail-side の repository memory を保つ文書群

`plan/` は scratchpad ではない。
決定、未決、仮説、履歴 / comparison を分けて書く。
