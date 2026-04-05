# plan/00 — plan インデックス

## 目的

`plan/` は、このリポジトリの長期参照用の repository memory である。
役割は、`specs/` に散っている規範文書、`docs/reports/` に散っている経緯、`crates/` にある PoC 実装実体を、人間が将来の task で再読しやすい形へ構造化して保持することにある。

ここで重要なのは次の 2 点である。

- `specs/` は意味論・境界・判断の規範文書であり続ける。
- `plan/` はそれらを置き換えず、**現況・計画・責務境界・未決事項・作業順序**を長期参照しやすく整理する。

## 先に読む順序

1. `plan/00-index.md`
2. `plan/01-status-at-a-glance.md`
3. `plan/02-system-overview-and-positioning.md`
4. `plan/03-decision-strengths-and-boundaries.md`
5. current L2 / PoC を見るなら `plan/04` から `plan/09`
6. 今後の進め方を見るなら `plan/10` から `plan/13`
7. fixture を足す実務テンプレートが必要なら `plan/15-current-l2-fixture-authoring-template.md`
8. 用語と source 根拠を引くときは `plan/14`、`plan/90`、`plan/91`

## status legend

この `plan/` では、判断の強さと性質を次のように分ける。

| ラベル | 意味 |
|---|---|
| `L0/L1 settled` | リポジトリ全体を拘束する強い判断。勝手に変えない |
| `current L2 settled` | current L2 としていま採っている判断。将来見直し余地はあるが、現タスクでは正本として扱う |
| `current parser-free PoC reading` | parser-free PoC 基盤で採っている実装・検証上の最小読み。理論本体ではなく companion / helper 境界に属する |
| `OPEN` | 未決。仮説を事実化してはいけない |
| `FUTURE WORKSTREAM` | 重く、まだ本格着手していない将来 workstream |
| `HISTORICAL / COMPARISON` | 比較対象、採らなかった案、経緯整理 |

## current repo の短い要約

- 主眼は **Mir** の意味論基盤であり、Mirrorea / Typed-Effect Wiring Platform / PrismCascade は分離された関連層である。
- current L2 では fallback は outer-longer-lifetime wrapper ではなく **guarded option chain** として読む。
- same-lineage chain は **left-to-right monotone degradation** を採り、**earlier option への再昇格は禁止**している。
- parser-free PoC 基盤は、AST fixture から named profile catalog まで一通り揃っており、current L2 representative fixtures を machine-check できる。
- detached exporter chain については、non-production の bundle-first emitter、aggregate emitter、payload-core diff helper、tiny loop wrapper、fixture authoring template があり、継続的 validation loop の入口が見え始めている。
- notation では compact syntax を比較したが、現時点では **explicit edge-row form** を暫定 companion notation として維持している。
- final parser grammar、machine-readable catalog externalization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` などは未決のまま残している。

## 各ファイルの役割

| ファイル | 役割 |
|---|---|
| `plan/01-status-at-a-glance.md` | 1〜2 画面で把握する現況サマリ |
| `plan/02-system-overview-and-positioning.md` | Mir / Mirrorea / PrismCascade / typed-effects の大局的関係 |
| `plan/03-decision-strengths-and-boundaries.md` | L0〜L3 と current L2 / PoC / future の見分け方 |
| `plan/04-core-semantics-current-l2.md` | Mir current L2 核心の整理 |
| `plan/05-fallback-lease-and-chain-semantics.md` | fallback / `lease` / monotone degradation の核心整理 |
| `plan/06-surface-notation-status.md` | companion notation と syntax 候補の現在地 |
| `plan/07-parser-free-poc-stack.md` | parser-free PoC stack の構造と責務境界 |
| `plan/08-representative-programs-and-fixtures.md` | representative programs / fixtures のカタログ |
| `plan/09-helper-stack-and-responsibility-map.md` | helper stack と docs/tests/code boundary |
| `plan/10-roadmap-overall.md` | 全体ロードマップ |
| `plan/11-roadmap-near-term.md` | 近い数 task の進め方と rough estimate |
| `plan/12-open-problems-and-risks.md` | unresolved / risk register |
| `plan/13-heavy-future-workstreams.md` | 型・静的解析・定理証明・決定可能性などの重い将来 workstream |
| `plan/14-glossary-and-boundary-rules.md` | 用語集と混同防止 |
| `plan/15-current-l2-fixture-authoring-template.md` | fixture authoring / elaboration を追加し、detached validation loop へ接続するときの実務テンプレート |
| `plan/90-source-traceability.md` | どの plan がどの source に依拠するか |
| `plan/91-maintenance-rules.md` | 今後 `plan/` をどう維持するか |

## `plan/` と `specs/` / `docs/reports/` / `AGENTS.md` の関係

- `specs/`
  - 規範文書。意味論、境界、decision を定義する正本。
- `docs/reports/`
  - 時系列の作業記録。なぜ判断が追加・修正されたかの根拠。
- `AGENTS.md`
  - repo 内作業ルール。今後は `plan/` 維持ルールもここに含める。
- `plan/`
  - 上の 3 つを横断して、**現況・計画・責務分担・未決事項・更新運用**を長期参照可能にした人間向け正本。

`plan/` は scratchpad ではない。
推測で埋めず、決まっていること・未決のこと・比較対象・履歴を分けて書く。
