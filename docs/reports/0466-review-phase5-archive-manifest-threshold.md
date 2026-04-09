# 0466 — review phase5 archive manifest threshold

## Objective

`0465-phase5-archive-manifest-threshold.md` と対応する spec / mirror 更新について、
current theorem-line archive-manifest threshold の整理が既存 repo memory と矛盾していないかを review する。

## Scope and assumptions

- Phase 5 theorem-line archive bundle family の次段だけを見る。
- implementation 追加はなく、docs / plan / progress / tasks の整合だけを対象にする。

## Documents consulted

- `specs/examples/175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md`
- `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. reviewer に current package を渡し、completion まで長めに待つ。
2. finding があれば該当 mirror を補正する。
3. final status をこの review record に残す。

## Files changed

- `docs/reports/0466-review-phase5-archive-manifest-threshold.md`

## Commands run

なし（reviewer completion と local diff inspection を使用）

## Evidence / outputs / test results

- reviewer completion を取得。
- finding は 3 件だった。
  1. High: `specs/examples/176...` の `decided` / `not decided` wording が self-contradictory
  2. Medium: `plan/17...` の immediate execution order が theorem-line を `173` までで止めていた
  3. Low: `progress.md` の作業ログに `175` package の 1 行が欠けていた
- 上記 3 件はすべて patch 済み。

## What changed in understanding

- archive-manifest-ready retained bridge の semantic core 自体は妥当で、remaining issue は wording / mirror hygiene だった。
- next later reopen は actual archive bundle member family comparison と読むので十分であり、manifest family と member family を同じ reopen に束ねる必要は current package ではない。

## Open questions

- actual archive bundle member family の最小 shape
- archive bundle member family を ref 1 本に留めるか、archive member body compare へ further split するか
- retained archive payload / bless-update policy との接続をどの threshold まで後段に残すか

## Suggested next prompt

`Phase5 の archive manifest threshold review 結果を反映したうえで、actual archive bundle member family comparison を next later reopen candidate として整理してください。`
