# 0468 — review phase5 archive member-family threshold

## Objective

`0467-phase5-archive-member-family-threshold.md` と対応する spec / mirror 更新について、
current theorem-line archive-member-family threshold の整理が既存 repo memory と矛盾していないかを review する。

## Scope and assumptions

- Phase 5 theorem-line archive manifest family の次段だけを見る。
- implementation 追加はなく、docs / plan / progress / tasks の整合だけを対象にする。

## Documents consulted

- `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
- `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`
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

- `docs/reports/0468-review-phase5-archive-member-family-threshold.md`

## Commands run

なし

## Evidence / outputs / test results

- reviewer completion を取得。詳細 review note は `docs/reports/0469-review-phase5-archive-member-family-package.md`。
- finding は 2 件だった。
  1. `plan/11-roadmap-near-term.md` に `176` 止まり / actual archive bundle member family comparison の stale reopen wording が 2 箇所残っていた
  2. `docs/reports/0467-phase5-archive-member-family-threshold.md` の evidence section が `PENDING` のままだった
- 上記 2 件はすべて patch 済み。

## What changed in understanding

- archive-member-family-ready retained bridge の semantic core 自体は妥当で、remaining issue は roadmap mirror drift と report evidence hygiene だった。
- next later reopen は actual archive member body compare comparison と読むので十分であり、member family と member body compare を同じ reopen に束ねる必要は current package ではない。

## Open questions

- actual archive member body compare の最小 shape
- member body compare を member-body ref 1 本に留めるか、bless / update policy まで further split するか
- retained archive payload / bless-update policy との接続をどの threshold まで後段に残すか

## Suggested next prompt

`Phase 5 の archive member-family threshold review 結果を反映したうえで、actual archive member body compare comparison を next later reopen candidate として整理してください。`
