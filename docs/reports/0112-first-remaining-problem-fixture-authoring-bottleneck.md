# 0112 — first remaining problem fixture authoring bottleneck

## Objective

current repo の near-term mainline に残っている大きな問題群を 5 本程度に畳み、そのうち最優先の 1 本として

- fixture authoring / elaboration bottleneck

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
13. `plan/05-fallback-lease-and-chain-semantics.md`
14. `plan/07-parser-free-poc-stack.md`
15. `plan/09-helper-stack-and-responsibility-map.md`
16. `plan/11-roadmap-near-term.md`
17. `plan/12-open-problems-and-risks.md`
18. `plan/13-heavy-future-workstreams.md`
19. `plan/15-current-l2-fixture-authoring-template.md`
20. `progress.md`

## Actions taken

1. current repo の near-term mainline に残っている問題を再確認した。
2. 近い論点を 5 本へ畳んだ。
3. 最優先として fixture authoring / elaboration bottleneck を選び、比較軸を整理した。
4. 具体例として、new fixture を 1 本足すときに必要な carrier / evidence を確認した。

## Evidence / outputs / test results

- docs-only report 追加のみ。code / tests / fixtures は未変更。
- local reasoning の根拠:
  - `progress.md` の「バリデーションループ前の残課題」
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/11-roadmap-near-term.md`

## What changed in understanding

- current repo の mainline で最も近い残問題は、type system や theorem prover そのものより、fixture authoring / elaboration の実務コストである。
- これは detached validation loop を「1 回動く」から「継続的に回る」へ移すときの friction source であり、parser finalization や richer host interface より前に狭く潰す価値が高い。

## Open questions

- 手書き fixture をどこまで維持するか
- parser を入れずに elaboration helper をどこまで許すか
- fixture 追加時の profile / batch / detached export smoke をどこまで自動化するか

## plan/ progress updates

- `plan/ 更新不要`
- `progress.md 更新不要`

## Suggested next prompt

`plan/15-current-l2-fixture-authoring-template.md` を前提に、new fixture を 1 本足す最小実地 task を行ってください。特に hand-written fixture と tiny elaboration helper のどちらを先に narrow に試すべきか、detached artifact export / compare まで含めて source-backed に比較してください。
