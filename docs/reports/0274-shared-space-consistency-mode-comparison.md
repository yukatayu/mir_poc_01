# 0274 — shared-space consistency mode comparison

## Objective

shared-space / membership line の次段として、consistency mode catalog を

- `authoritative serial transition`
- `append-friendly room`
- `relaxed merge-friendly room`

の 3 案で比較し、authority placement / ownership model とどう噛み合うかを docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- final consistency mode catalog は固定しない。
- authoritative room と append-heavy room の first practical catalog を示すが、merge-friendly branch は future comparison に残す。
- consensus family や merge algorithm 名は固定しない。

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
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0272-shared-space-authority-placement-comparison.md`

## Actions taken

1. `plan/16` に consistency mode catalog の候補比較 section を追加した。
2. `authoritative serial transition` / `append-friendly room` / `relaxed merge-friendly room` を、
   - 読み
   - 利点
   - 欠点
   - 向く room
   で書き分けた。
3. authoritative room と append-heavy room の current working judgment を追記した。
4. `plan/12` の shared-space authority / resource ownership risk row に、consistency mode の first practical catalog を反映した。
5. `progress.md` を更新し、shared-space line の current first choice を補正した。

## Files changed

- `docs/reports/0274-shared-space-consistency-mode-comparison.md`
- `docs/reports/0275-review-shared-space-consistency-mode-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:32 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 275 numbered report(s).

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
  - `plan/90` の traceability mirror に `0274` / `0275` を追加する必要
  - report の command trace / final validation evidence を close out する必要
- いずれも task 内で反映済み

## What changed in understanding

- authoritative room の current first choice は `authoritative serial transition` が最も自然だと整理できた。
- append-heavy room では `append-friendly room` を first practical catalog に置くのが自然で、membership activation と data-plane append visibility を同じ rule に縛らない方がよい。
- `relaxed merge-friendly room` は上位 collaboration space に向くが、merge policy / moderation / projection authority を別に詰める必要があり、current phase では future comparison に残すのが妥当である。

## Open questions

- append-friendly room の append order / moderation policy をどこで切るか。
- `relaxed merge-friendly room` を first practical catalog に上げる threshold は何か。
- consistency mode と RNG / fairness trust model をどこで接続するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space RNG / fairness source placement について、authority_rng / delegated_rng_service / distributed_randomness_provider の 3 案を、authoritative room と append-heavy room の対比つきで narrow に比較してください。`
