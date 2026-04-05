# 0113 — second remaining problem detached exporter api finalization

## Objective

current repo の near-term mainline に残っている大きな問題群のうち、第 2 の問題として

- detached exporter の actual API / storage policy finalization

を選び、何と何で迷っているか、各案でどの性質が保証されるかを簡潔に整理する。

## Scope and assumptions

- current L2 semantics、parser-free PoC、detached validation loop の既決事項は変更しない。
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
13. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
14. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
15. `plan/07-parser-free-poc-stack.md`
16. `plan/09-helper-stack-and-responsibility-map.md`
17. `plan/11-roadmap-near-term.md`
18. `plan/12-open-problems-and-risks.md`
19. `plan/15-current-l2-fixture-authoring-template.md`
20. `progress.md`
21. `scripts/current_l2_detached_loop.py`

## Actions taken

1. current detached validation loop の docs-only judgment と tiny helper の現状を再確認した。
2. detached exporter の actual API / storage policy finalization を near-term の第 2 問題として位置づけ直した。
3. 主な比較軸を
   - harness 本体へ寄せるか
   - scripts / examples の non-production helper に留めるか
   - storage/path policy をどこまで formalize するか
   に分けて整理した。

## Evidence / outputs / test results

- docs-only report 追加のみ。code / tests / fixtures は未変更。
- local reasoning の根拠:
  - `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `scripts/current_l2_detached_loop.py`

## What changed in understanding

- current repo は detached validation loop の入口までは来ているが、actual API を `harness.rs` / `lib.rs` に切るか、どこまで scripts/examples helper に留めるかがまだ未決である。
- この論点は、fixture authoring bottleneck の次に近い operational bottleneck であり、parser finalization や richer host interface より先に狭く決める価値が高い。

## Open questions

- actual exporter API を library helper として切るか、それとも non-production wrapper を維持するか
- `target/current-l2-detached/` をどこまで current default と見なすか
- convenience discovery をどこまで formalize するか

## plan/ progress updates

- `plan/ 更新不要`
- `progress.md 更新不要`

## Suggested next prompt

`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` を前提に、actual exporter API を `harness.rs` に薄く入れる案と、non-production scripts/examples helper に留める案を narrow に比較してください。特に helper boundary、テスト責務、artifact path policy の固定圧の差を source-backed に整理してください。
