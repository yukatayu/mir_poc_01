# 0116 — fifth remaining problem static analysis and proof boundary

## Objective

current repo の near-term mainline に残っている大きな問題群のうち、第 5 の問題として

- static analysis / type system / theorem prover boundary

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
13. `plan/05-fallback-lease-and-chain-semantics.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/13-heavy-future-workstreams.md`
17. `progress.md`

## Actions taken

1. heavy workstream と current mainline の境界を再確認した。
2. static analysis / type system / theorem prover 境界を、第 5 の残問題として位置づけ直した。
3. 主な比較軸を
   - 強い language core へ寄せるか
   - thin core + external verifier に寄せるか
   - hybrid staged approach を採るか
   に分けて整理した。

## Evidence / outputs / test results

- docs-only report 追加のみ。code / tests / fixtures は未変更。
- local reasoning の根拠:
  - `plan/13-heavy-future-workstreams.md`
  - `specs/10-open-questions.md`
  - `plan/05-fallback-lease-and-chain-semantics.md`
  - `plan/11-roadmap-near-term.md`

## What changed in understanding

- この問題の本質は「型システムを入れるかどうか」ではなく、`どの性質を current language core の decidable / local judgment に乗せ、どの性質を external verifier / theorem prover / model checker に送るか` である。
- current repo では、fallback chain の static evidence floor、underdeclared rejection、capability strengthening の否定のような local / structural property は language core 側に寄せやすい。
- 一方で、global invariant、liveness、scheduler、shared-space membership churn、resourceful continuation の一般論は external verification 側に残す方が自然である。

## Open questions

- first type/core に何を含めるか
- theorem prover 向け relation を semantic core からどう切り出すか
- model checker を protocol / scheduler / shared-space 側へどう接続するか

## plan/ progress updates

- `plan/ 更新不要`
- `progress.md 更新不要`

## Suggested next prompt

`plan/13-heavy-future-workstreams.md` を前提に、current L2 で language core に先に入れる static/type judgment の最小 subset を narrow に比較してください。特に fallback same-lineage / capability floor / underdeclared rejection / option-local `admit` / `try` rollback locality のうち、どれを first checker cut に含めるかを source-backed に整理してください。
