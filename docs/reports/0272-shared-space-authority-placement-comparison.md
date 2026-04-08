# 0272 — shared-space authority placement comparison

## Objective

shared-space / membership line の次段として、authority placement を

- `single room authority`
- `replicated authority`
- `relaxed projection authority`

の 3 案で比較し、authoritative room と append-friendly room でどの placement が current phase に最も自然かを docs-first に整理する。

## Scope and assumptions

- 今回は docs-only の comparison に留める。
- final consensus family や replication algorithm は固定しない。
- `authority` は room rule / membership activation / resource owner handoff を commit する operational role として読む。
- participant carrier、resource owner slot、delegated capability、consistency mode、RNG provider placement は同一 carrier に潰さない。

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
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0269-review-shared-space-authority-doc-change.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0271-review-shared-space-activation-rule-comparison.md`

## Actions taken

1. `plan/16` に authority placement の候補比較 section を追加した。
2. `single room authority` / `replicated authority` / `relaxed projection authority` を、
   - 読み
   - 利点
   - 欠点
   - 向く room
   で書き分けた。
3. authoritative game room と append-friendly room に対する current working judgment を追記した。
4. `plan/12` の shared-space authority / resource ownership risk row に、current phase の minimal candidate を反映した。
5. `progress.md` を更新し、shared-space line の current first choice を補正した。
6. `plan/90` を更新し、`plan/12` / `plan/16` の今回変更が `0272` / `0273` に依拠することを traceability に反映した。

## Files changed

- `docs/reports/0272-shared-space-authority-placement-comparison.md`
- `docs/reports/0273-review-shared-space-authority-placement-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:16 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 273 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Timestamp used for progress update:

Reviewer:

- reviewer 1 回実施
- substantive semantic finding なし
- hygiene finding 2 件
  - `progress.md` の scope が authoritative room 限定だと読み取れない
  - report に `Commands run` と final validation evidence が欠けていた
- いずれも task 内で反映済み

## What changed in understanding

- authoritative room の current first choice は `single room authority` が最も自然だと整理できた。
- `replicated authority` は single authority の reasoning を大きく壊さず failover を持てるため、operational realization 側の next candidate として自然である。
- `relaxed projection authority` は append-heavy room では有望だが、winner / reset / exclusive lock を含む authoritative game room には不向きで、current phase では future comparison に残すのが妥当である。
- authority placement は participant carrier や resource owner slot と別軸に保ち、deployment topology や algorithm family に早く commit しない方が current repo の layer separation に合う。

## Open questions

- `replicated authority` を docs-only comparison 以上に上げる threshold は何か。
- append-friendly room で `relaxed projection authority` を current candidate に上げるには、どの consistency mode / moderation policy が必要か。
- authority placement と RNG / fairness trust model をどこで接続するか。
- authority handoff event と membership activation event を同じ audit carrier に入れるか分けるか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space consistency mode catalog について、authoritative serial transition / append-friendly room / relaxed merge-friendly room の 3 案を、authority placement と ownership model の相性つきで narrow に比較してください。`
