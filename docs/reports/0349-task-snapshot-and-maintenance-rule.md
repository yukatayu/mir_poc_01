# Report 0349 — task snapshot and maintenance rule

- Date: 2026-04-09 09:44 JST
- Author / agent: Codex
- Scope: `tasks.md` の新設、維持ルールの追加、top-level 導線の最小更新
- Decision levels touched: L1 mirror / maintenance policy, L2 current task map presentation

## 1. Objective

`progress.md` だけでは「自走可能な task package」と「方針決定が必要な current blocker」の切り分けが読み取りづらくなってきたため、非規範の current task map として `tasks.md` を導入し、その維持ルールと top-level 導線を repo 全体で揃える。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`

## 3. Actions taken

1. `tasks.md` を追加し、current snapshot として次を整理した。
   - 自走可能な task package
   - 方針決定が必要で current research の障害になっている blocker / open question
   - 各 blocker の概要、影響範囲、主要な選択肢、current recommendation
2. `AGENTS.md` に `tasks.md 維持ルール` を追加し、append ではなく全体 rewrite で snapshot を保つ運用を明文化した。
3. `plan/91-maintenance-rules.md` に `tasks.md` の扱いを mirror した。
4. `README.md`、`Documentation.md`、`specs/00-document-map.md` に `tasks.md` への導線を追加した。
5. `plan/90-source-traceability.md` に provenance addendum を追加し、今回の top-level mirror 更新の根拠が `tasks.md` と本 report にあることを明示した。
6. `Documentation.md` の番号ずれも同 task で補正した。

## 4. Files changed

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `tasks.md`
- `docs/reports/0349-task-snapshot-and-maintenance-rule.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 09:44 JST

$ python3 scripts/new_report.py --slug task-snapshot-and-maintenance-rule
Created docs/reports/0349-task-snapshot-and-maintenance-rule.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 351 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- `progress.md` は phase / progress / autonomy gate の rough snapshot として有用だが、自走 task と blocker の切り分けを同じ文書で詳細化し続けると肥大化しやすい。
- `tasks.md` を non-normative snapshot として分離することで、以下を読みやすく保てる。
  - 直近で agent がまとめて進められる task package
  - 研究を止めている policy decision の所在
  - 各 blocker でいま何と何を比較しているか
- ただし `tasks.md` が規範文書に見えてしまうと repository memory の二重化になるため、root は常に `specs/` と `plan/` に残す必要がある。

## 7. Changes in understanding

- current repo では、`progress.md` と `tasks.md` を役割分離した方が保守しやすい。
  - `progress.md` = rough progress snapshot / phase / autonomy gate / work log
  - `tasks.md` = current self-driven task package / blockers / current recommendation
- `tasks.md` は append ではなく全体 rewrite の snapshot とする方が、古い blocker を引きずらず current state に整合しやすい。

## 8. Open questions

- `tasks.md` の section 粒度を今後どこまで固定するか。
- 将来、各 task package に rough step estimate や validation evidence への direct link を追加するか。
- `tasks.md` を phase end 以外の checkpoint でも更新する頻度をどこまで高めるか。
- `progress.md 更新不要`
  - 今回は phase 読みや rough progress 自体は変えず、task map の presentation layer を切り出しただけである。

## 9. Suggested next prompt

`tasks.md` にある最優先 self-driven task package のうち 1 本を選び、source-backed な narrow comparison と PoC evidence を伴って次の current checkpoint まで進めてください。
