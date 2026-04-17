# Report 0727 — faq006 current knowns open questions and tradeoffs

- Date: 2026-04-17T09:28:06.622191Z
- Author / agent: Codex
- Decision levels touched: L0/L1/L2 の既存判断を説明したが、新しい規範判断は追加していない

## 1. Objective

- 現状で何が分かっていて、何が論点で、何と何で悩んでいるのかを、current repo state に即して平易に整理する。
- `e*` / `p*` sample を具体例にして、source-backed floor と OPEN / mixed gate を混同しない FAQ を追加する。

## 2. Scope and assumptions

- scope は `faq_006.md` の新規作成と、FAQ 導線としての `Documentation.md` 最小同期に限る。
- 規範判断の正本は `specs/`、current status の薄い snapshot は `progress.md` / `tasks.md`、detail-side memory は `plan/` とする。
- この task では current roadmap や macro phase 読み自体は変えない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `faq_005.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## 4. Actions taken

- current snapshot から、次を分離して整理した。
  - source-backed に分かっていること
  - 論点になっていること
  - 比較で悩んでいる軸
  - sample / prototype が示している具体例
- `faq_006.md` を新規作成した。
- `Documentation.md` の FAQ 導線を `faq_005.md` 単独から複数 FAQ 表記へ更新した。
- 実行した主なコマンド:
  - `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
    - `Task baseline recorded.`
  - `python3 scripts/new_report.py --slug faq006-current-knowns-open-questions-and-tradeoffs`
    - `/home/yukatayu/dev/mir_poc_01/docs/reports/0727-faq006-current-knowns-open-questions-and-tradeoffs.md`
  - `date '+%Y-%m-%d %H:%M %Z'`
    - `2026-04-17 18:27 JST`

## 5. Evidence / outputs / test results

- current repo は「何も分かっていない research memo」ではなく、
  - authored sixteen
  - runnable prototype octet
  - helper-local `debug_outputs`
  - helper-local `verification_preview`
  - helper-local `artifact_preview`
  を持つ段階である。
- いまの主要な悩みは、存在しない機能を発明することではなく、
  - stronger typed surface をどこで昇格させるか
  - theorem discharge / model-check property language をどこで public contract に近づけるか
  - `memory_order` exact family と relation decomposition のどちらを principal wording にするか
  を決めることである。
- `Documentation.md` の FAQ 導線は `faq_005.md` だけだと stale になるため、複数 FAQ 表記へ更新する必要があった。

## 6. What changed in understanding

- 2026-04-17 時点の current reading では、難所は「表現不能」から「昇格 threshold の選別」へ移っている。
- `p06` / `p07` / `p08` と `e5` / `e12` を並べると、repo の current cut が
  - runtime success
  - static stop
  - typed/theorem/model-check bridge preview
  - order/handoff corrected comparison
  をすでに持っていることを、FAQ として説明しやすい形で示せる。

## 7. Open questions

- stronger typed surface をどこで helper-local / prototype から public-adjacent へ昇格させるか
- theorem discharge transport / public-contract concretization をどの時点で mixed gate から出すか
- model-check property language / tool seam をどの concrete family で固定するか
- `memory_order` 参照 family と snake_case relation family の最終 public balance をどう取るか
- shared-space fairness / replay final profile をどこで operational line に引き上げるか

## 8. Suggested next prompt

- `faq_006.md` の内容を踏まえて、mixed gate のうちどれを先に reopen するかを 1 本選び、その comparison package と corrected prototype を進めてください。

## 9. Maintenance notes

- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
