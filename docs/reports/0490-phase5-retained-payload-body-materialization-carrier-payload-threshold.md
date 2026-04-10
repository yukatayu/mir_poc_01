# Report 0490 — phase5 retained payload body materialization carrier payload threshold

- Date: 2026-04-10T05:06:16.238360Z
- Author / agent: Codex
- Scope: Phase 5 theorem-line の current promoted line として、`retained_payload_body_materialization_carrier_payload_ref` をどこまで current first choice に入れてよいかを docs-first で整理する。
- Decision levels touched: L1 / L2

## 1. Objective

`specs/examples/186-current-l2-theorem-line-retained-payload-body-materialization-payload-ready-retained-payload-body-materialization-carrier-detail-threshold.md`
までを前提に、theorem-side retained bridge の次段として

- actual retained payload body materialization carrier payload
- actual bless-side row / update-side row split

のどこまでを current first choice に寄せるかを narrow に比較し、current promoted line と mirror snapshot を揃える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`
- `specs/examples/181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`
- `specs/examples/182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md`
- `specs/examples/183-current-l2-theorem-line-retained-archive-payload-body-family-ready-retained-payload-materialization-family-threshold.md`
- `specs/examples/184-current-l2-theorem-line-retained-payload-materialization-family-ready-retained-payload-body-materialization-detail-threshold.md`
- `specs/examples/185-current-l2-theorem-line-retained-payload-body-materialization-detail-ready-retained-payload-body-materialization-payload-threshold.md`
- `specs/examples/186-current-l2-theorem-line-retained-payload-body-materialization-payload-ready-retained-payload-body-materialization-carrier-detail-threshold.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. 直近 theorem-line package `180...186` を読み直し、`retained_payload_body_materialization_carrier_detail_ref` の次段として何を current first choice に入れてよいかを確認した。
2. `specs/examples/187-current-l2-theorem-line-retained-payload-body-materialization-carrier-detail-ready-retained-payload-body-materialization-carrier-payload-threshold.md` を追加し、`retained_payload_body_materialization_carrier_payload_ref` だけを足す cut を current judgment に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を 187 package に追随させた。
4. local validation (`validate_docs.py` / `git diff --check`) を実行した。
5. reviewer を 1 回だけ起動し、completion を待つ運用に入る。

## 4. Files changed

- `specs/examples/187-current-l2-theorem-line-retained-payload-body-materialization-carrier-detail-ready-retained-payload-body-materialization-carrier-payload-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0490-phase5-retained-payload-body-materialization-carrier-payload-threshold.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 14:06 JST

$ python3 scripts/new_report.py --slug phase5-retained-payload-body-materialization-carrier-payload-threshold
/home/yukatayu/dev/mir_poc_01/docs/reports/0490-phase5-retained-payload-body-materialization-carrier-payload-threshold.md
```

## 6. Evidence / findings

- current first choice は `retained_payload_body_materialization_carrier_payload_ref` までを symbolic retained bridge に足すところまでで十分である。
- `actual bless-side / update-side row split` は still 後段に残してよく、carrier payload と同じ reopen で混ぜない方が narrow progression を保ちやすい。
- snapshot mirror では、Phase 5 current package close を `126...187` に更新し、next promoted line を `actual bless-side / update-side row split comparison` に切り替えるのが自然である。
- reviewer completion では `plan/90` の missing review trail と `progress.md` recent log ordering の 2 点が指摘され、[0491](/home/yukatayu/dev/mir_poc_01/docs/reports/0491-review-phase5-retained-payload-body-materialization-carrier-payload-threshold.md) で反映済みである。

## 7. Changes in understanding

- `retained_payload_body_materialization_carrier_detail_ref` は theorem-line retained bridge の current terminal cut ではなく、carrier payload への 1 段前の stop にすぎない、という理解がさらに明確になった。
- ただし carrier payload まで current first choice に入れても、bless/update split までは current core に上げない方がよい、という cut が source-backed になった。

## 8. Open questions

- actual bless-side row と update-side row の最小 split shapeをどう切るか
- split を carrier payload line の直下で切るか、still bridge 外の policy row として切るか
- typed symbolic ref family を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
- `proof_assistant_adapter` consumer pressure を second practical candidate のまま維持する条件がいつ崩れるか

`plan/ 更新済み`

`progress.md 更新済み`

`tasks.md 更新済み`

## 9. Suggested next prompt

`actual bless-side / update-side row split comparison` を Phase 5 theorem-line の next promoted line として進め、carrier payload 以降をどこまで symbolic retained bridge に残せるかを比較してください。
