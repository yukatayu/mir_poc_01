# 0278 — shared-space authoritative game room profile comparison

## Objective

shared-space / membership research line の次段として、authoritative game room を practical example として取り上げ、

- activation rule
- authority placement
- consistency mode
- RNG / fairness source

の 4 軸を 1 つの concrete profile に束ねるとき、どの bundle が current phase に最も自然かを docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- example は途中参加可能なすごろく風 room を使う。
- identity / auth / reconnect / actual consensus algorithm は finalization しない。
- practical pseudocode は explanatory companion であり、final syntax ではない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- blog `https://blog.yukatayu.tech/blog/sync_language_01/`
- blog `https://blog.yukatayu.tech/blog/sync_language_02/`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0272-shared-space-authority-placement-comparison.md`
- `docs/reports/0274-shared-space-consistency-mode-comparison.md`
- `docs/reports/0276-shared-space-rng-fairness-source-placement.md`

## Actions taken

1. `plan/16` に authoritative game room の concrete profile candidates section を追加した。
2. minimal trusted authoritative room、replaceable RNG provider room、future high-availability / stronger-fairness candidate を比較した。
3. すごろく風 room の practical pseudocode を追加し、join / roll / pending / goal / reset / leave を一連の authoritative transition として読める形にした。
4. append-friendly notice / presence room を contrast として添え、shared-space 全体の既定と誤読しないようにした。
5. `plan/12` と `progress.md` に current minimal bundle と next practical bundle を mirror した。
6. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`
- `docs/reports/0279-review-shared-space-authoritative-game-room-profile-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:54 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 279 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer 1 回実施
- substantive semantic finding なし
- hygiene finding 2 件
  - `plan/90` の traceability mirror に `0278` / `0279` を追加する必要
  - report の command trace / final validation evidence を close out する必要
- いずれも task 内で反映済み

## What changed in understanding

- authoritative game room の current minimal bundle は、4 軸を別 carrier のまま束ねると
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  が最も素直である。
- RNG だけを `delegated_rng_service` に差し替える bundle は、replaceability / HW hook / debug provider の観点で next practical candidate になる。
- 同じ participant carrier を使っても append-friendly room では data-plane rule が変わるため、game room bundle を shared-space 全体の default にしてはいけない。
- blog 起点の「木構造で見たい」直感は derived snapshot / explanatory view として活かせるが、source of truth は依然 registry / authority / capability の分離で保つ方が理論的にきれいである。

## Open questions

- reconnect / late leave / in-flight roll を minimal bundle の外部 policy に残すか、room profile 内へ一部昇格させるか。
- fairness trust model を `authority_rng` のままどこまで許すか。
- identity / auth を room profile とどこで接続するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space room profile について、reconnect / late leave / in-flight action policy を room profile 本体に入れるべきか、外部 policy layer に残すべきかを narrow に比較してください。`
