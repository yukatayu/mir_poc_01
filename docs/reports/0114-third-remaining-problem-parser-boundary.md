# 0114 — third remaining problem parser boundary

## Objective

current repo の near-term mainline に残っている大きな問題群のうち、第 3 の問題として

- final parser syntax と companion notation の切り分け

を選び、何と何で迷っているか、各案でどの性質が保証されるかを簡潔に整理する。

## Scope and assumptions

- current L2 semantics、fallback law、parser-free PoC の既決事項は変更しない。
- 今回は conversational な整理であり、docs / plan の規範判断を更新しない。
- `plan/` と `progress.md` は current understanding の範囲内とみなし、更新不要とする。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/09-invariants-and-constraints.md`
10. `specs/10-open-questions.md`
11. `specs/11-roadmap-and-workstreams.md`
12. `specs/12-decision-register.md`
13. `plan/06-surface-notation-status.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/13-heavy-future-workstreams.md`
17. `progress.md`

## Actions taken

1. current companion notation の位置づけと final parser grammar 未固定の範囲を再確認した。
2. parser boundary を near-term の第 3 問題として位置づけ直した。
3. 主な比較軸を
   - companion notation を docs-only に長く留めるか
   - minimal parser subset を narrow に固定するか
   - sugar を含む広い grammar へ早く進むか
   に分けて整理した。

## Evidence / outputs / test results

- docs-only report 追加のみ。code / tests / fixtures は未変更。
- local reasoning の根拠:
  - `plan/06-surface-notation-status.md`
  - `specs/10-open-questions.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/13-heavy-future-workstreams.md`

## What changed in understanding

- parser 問題の本質は「syntax を実装するか」ではなく、「どの boundary までなら syntax decision が型・証明・runtime を早く拘束しないか」である。
- current repo では、type system / theorem prover / static analysis workstream の entry criteria として parser 境界の最小 shape が見えることを要求しているため、parser は単独タスクではなく heavy workstream への前提整備でもある。

## Open questions

- `perform` / `fallback successor` / `@ lineage(...)` / `require` / `ensure` / `admit` のうち、どこまでを fixed token にするか
- A2 rendering を parser grammar の本体にするか、examples companion notation に留めるか
- parser 導入時に static-only elaboration helper をどう縮退させるか

## plan/ progress updates

- `plan/ 更新不要`
- `progress.md 更新不要`

## Suggested next prompt

`plan/06-surface-notation-status.md` と `specs/10-open-questions.md` を前提に、current L2 で parser に昇格させる最小 subset を narrow に比較してください。特に A2 edge-row family、`perform on` / `perform via`、`try { ... } fallback { ... }`、`require` / `ensure`、option-local `admit` のうち、どれを first parser cut に含め、どれを companion notation に残すかを source-backed に整理してください。
